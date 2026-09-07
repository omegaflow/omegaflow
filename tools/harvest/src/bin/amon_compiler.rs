use omegaflow::amon::{
    decode_rec, encode_rec, parse_header, write_header, AmonRecord, NoticeClass, HEADER_LEN,
    PRES_ENERGY, PRES_ERR50, PRES_ERR90, PRES_FAR, PRES_REVISION, PRES_SIGNALNESS, PRES_SOD,
    PRES_TJD, REC_BYTES,
};
use omegaflow::cdn::upload_asset;
use omegaflow::zeuge::{magic_identity, FeldIdentitaet, ZeugeArt};
use std::collections::BTreeSet;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn line_field<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    text.lines()
        .find_map(|l| l.strip_prefix(name).map(str::trim))
}

fn first_f64(s: &str) -> Option<f64> {
    s.split_whitespace()
        .next()?
        .trim_end_matches(['d', ','])
        .parse()
        .ok()
}

fn first_u32(s: &str) -> Option<u32> {
    s.split_whitespace().next()?.parse().ok()
}

enum Skip {
    NoType,
    BadRun,
    BadEvent,
    BadRa,
    BadDec,
}

struct Parsed {
    run: u32,
    event: u32,
    class: NoticeClass,
    ra_deg: f64,
    dec_deg: f64,
    err90_arcmin: Option<f64>,
    err50_arcmin: Option<f64>,
    energy_tev: Option<f64>,
    signalness: Option<f64>,
    far_per_yr: Option<f64>,
    revision: Option<u32>,
    tjd: Option<u32>,
    sod_s: Option<f64>,
}

fn parse_notice(text: &str) -> Result<Parsed, Skip> {
    let notice_type = line_field(text, "NOTICE_TYPE:").ok_or(Skip::NoType)?;
    let lower = notice_type.to_lowercase();
    let class = if lower.contains("gold") {
        NoticeClass::Gold
    } else if lower.contains("bronze") {
        NoticeClass::Bronze
    } else {
        return Err(Skip::NoType);
    };
    let run = first_u32(line_field(text, "RUN_NUM:").ok_or(Skip::BadRun)?).ok_or(Skip::BadRun)?;
    let event =
        first_u32(line_field(text, "EVENT_NUM:").ok_or(Skip::BadEvent)?).ok_or(Skip::BadEvent)?;
    let ra_deg = first_f64(line_field(text, "SRC_RA:").ok_or(Skip::BadRa)?).ok_or(Skip::BadRa)?;
    let dec_deg =
        first_f64(line_field(text, "SRC_DEC:").ok_or(Skip::BadDec)?).ok_or(Skip::BadDec)?;
    let opt = |name: &str| line_field(text, name).and_then(first_f64);
    Ok(Parsed {
        run,
        event,
        class,
        ra_deg,
        dec_deg,
        err90_arcmin: opt("SRC_ERROR:"),
        err50_arcmin: opt("SRC_ERROR50:"),
        energy_tev: opt("ENERGY:"),
        signalness: opt("SIGNALNESS:"),
        far_per_yr: opt("FAR:"),
        revision: line_field(text, "REVISION:").and_then(first_u32),
        tjd: line_field(text, "DISCOVERY_DATE:").and_then(first_u32),
        sod_s: opt("DISCOVERY_TIME:"),
    })
}

fn witness_s2_direction_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FeldIdentitaet::Zeuge(ZeugeArt::S2Richtung)) => {
            eprintln!(
                "{} reads as an s2-direction witness record",
                String::from_utf8_lossy(&magic)
            );
            Ok(())
        }
        Some(other) => Err(format!(
            "{} reads {:?}, not s2-direction — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    witness_s2_direction_identity(omegaflow::amon::MAGIC)?;
    let Some(input) = arg_value(args, "--input") else {
        return Err(
            "usage: amon_compiler --input <dir-of-.amon> --out <map.amn1> [--ci-mode] — refused"
                .into(),
        );
    };
    let Some(out_path) = arg_value(args, "--out") else {
        return Err("--out <map.amn1>: the asset path is never silent — refused".into());
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let dir =
        std::fs::read_dir(&input).map_err(|e| format!("read_dir {input} returned void: {e}"))?;
    let mut names = BTreeSet::new();
    for entry in dir {
        let entry = entry.map_err(|e| format!("read_dir {input} returned void: {e}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".amon") {
            names.insert(name);
        }
    }

    let mut records: Vec<AmonRecord> = Vec::new();
    let mut census_skip = [0u64; 5];
    let mut unplaceable = 0u64;
    let mut missing_optional = 0u64;
    let mut multi_run = std::collections::BTreeMap::<(u32, u32, u8), u64>::new();
    for name in &names {
        let text = std::fs::read_to_string(format!("{input}/{name}"))
            .map_err(|e| format!("read {input}/{name} returned void: {e}"))?;
        let p = match parse_notice(&text) {
            Ok(p) => p,
            Err(Skip::NoType) => {
                census_skip[0] += 1;
                continue;
            }
            Err(Skip::BadRun) => {
                census_skip[1] += 1;
                continue;
            }
            Err(Skip::BadEvent) => {
                census_skip[2] += 1;
                continue;
            }
            Err(Skip::BadRa) => {
                census_skip[3] += 1;
                continue;
            }
            Err(Skip::BadDec) => {
                census_skip[4] += 1;
                continue;
            }
        };
        let (order, ipix) = match AmonRecord::pixel_of(p.ra_deg, p.dec_deg) {
            Some(x) => x,
            None => {
                unplaceable += 1;
                continue;
            }
        };
        let class_bit = p.class as u8;
        *multi_run.entry((p.run, p.event, class_bit)).or_insert(0) += 1;
        if p.err90_arcmin.is_none()
            || p.err50_arcmin.is_none()
            || p.energy_tev.is_none()
            || p.signalness.is_none()
            || p.far_per_yr.is_none()
            || p.tjd.is_none()
            || p.sod_s.is_none()
        {
            missing_optional += 1;
        }
        let finite = |v: Option<f64>| match v {
            Some(x) => x.is_finite(),
            None => true,
        };
        if !(finite(p.err90_arcmin)
            && finite(p.err50_arcmin)
            && finite(p.energy_tev)
            && finite(p.signalness)
            && finite(p.far_per_yr)
            && finite(p.sod_s))
        {
            missing_optional += 1;
        }
        let present = if p.err90_arcmin.is_some() {
            PRES_ERR90
        } else {
            0
        } | if p.err50_arcmin.is_some() {
            PRES_ERR50
        } else {
            0
        } | if p.energy_tev.is_some() {
            PRES_ENERGY
        } else {
            0
        } | if p.signalness.is_some() {
            PRES_SIGNALNESS
        } else {
            0
        } | if p.far_per_yr.is_some() { PRES_FAR } else { 0 }
            | if p.revision.is_some() {
                PRES_REVISION
            } else {
                0
            }
            | if p.tjd.is_some() { PRES_TJD } else { 0 }
            | if p.sod_s.is_some() { PRES_SOD } else { 0 };
        let f32_present = |v: Option<f64>| -> f32 {
            match v {
                Some(x) => x as f32,
                None => 0.0,
            }
        };
        let u32_present = |v: Option<u32>| -> u32 {
            match v {
                Some(x) => x,
                None => 0,
            }
        };
        let r = AmonRecord {
            order,
            class: p.class,
            ipix,
            present,
            run: p.run,
            event: p.event,
            ra_deg: p.ra_deg as f32,
            dec_deg: p.dec_deg as f32,
            err90_arcmin: f32_present(p.err90_arcmin),
            err50_arcmin: f32_present(p.err50_arcmin),
            energy_tev: f32_present(p.energy_tev),
            signalness: f32_present(p.signalness),
            far_per_yr: f32_present(p.far_per_yr),
            revision: u32_present(p.revision),
            tjd: u32_present(p.tjd),
            sod_s: f32_present(p.sod_s),
        };
        records.push(r);
    }

    if records.is_empty() {
        return Err("no decoded notice — the asset stays unwritten (0 honored)".into());
    }

    let mut out = BufWriter::with_capacity(
        1 << 20,
        std::fs::File::create(&out_path)
            .map_err(|e| format!("create {out_path} returned void: {e}"))?,
    );
    let mut hbuf = Vec::with_capacity(HEADER_LEN);
    write_header(&mut hbuf, records.len() as u64);
    out.write_all(&hbuf)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    let mut rec = [0u8; REC_BYTES];
    for r in &records {
        encode_rec(&mut rec, r);
        out.write_all(&rec)
            .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    }
    let _ = out.flush();
    let _ = out.into_inner();

    let expect = HEADER_LEN as u64 + records.len() as u64 * REC_BYTES as u64;
    let actual = std::fs::metadata(&out_path)
        .map_err(|e| format!("stat {out_path} returned void: {e}"))?
        .len();
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} bytes written, {expected} expected — the asset stays unwritten",
            expected = expect
        ));
    }
    let mut vf = std::fs::File::open(&out_path)
        .map_err(|e| format!("open {out_path} returned void: {e}"))?;
    let mut head = [0u8; HEADER_LEN];
    vf.read_exact(&mut head)
        .map_err(|e| format!("read {out_path} header returned void: {e}"))?;
    let n_rows =
        parse_header(&head).ok_or_else(|| format!("{out_path}: the header stays unread"))?;
    let last_off = HEADER_LEN as u64 + (records.len() as u64 - 1) * REC_BYTES as u64;
    vf.seek(SeekFrom::Start(last_off))
        .map_err(|e| format!("seek {out_path} returned void: {e}"))?;
    let mut tail = vec![0u8; REC_BYTES];
    vf.read_exact(&mut tail)
        .map_err(|e| format!("read {out_path} tail returned void: {e}"))?;
    let last =
        decode_rec(&tail).ok_or_else(|| format!("{out_path}: the last record stays unread"))?;

    eprintln!(
        "{out_path}: {} notices from {} .amon files, header {} rows — roundtrip verified",
        records.len(),
        names.len(),
        n_rows
    );
    eprintln!(
        "last notice: run {} event {} {} ra {:.4} dec {:.4} err90 {:.2} arcmin energy {:.3} TeV signalness {:.3} FAR {:.3} yr^-1",
        last.run,
        last.event,
        match last.class {
            NoticeClass::Gold => "Gold",
            NoticeClass::Bronze => "Bronze",
        },
        last.ra_deg,
        last.dec_deg,
        last.err90_arcmin,
        last.energy_tev,
        last.signalness,
        last.far_per_yr
    );
    eprintln!(
        "skipped: {} no-notice, {} bad-run, {} bad-event, {} bad-ra, {} bad-dec, {} unplaceable",
        census_skip[0], census_skip[1], census_skip[2], census_skip[3], census_skip[4], unplaceable
    );
    eprintln!("notices missing at least one optional column: {missing_optional}");
    for ((run, event, class_bit), n) in &multi_run {
        if *n > 1 {
            eprintln!(
                "duplicate run/event {} {} class {} count {}",
                run,
                event,
                if *class_bit == 1 { "Gold" } else { "Bronze" },
                n
            );
        }
    }
    if ci_mode && !upload_asset(&out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("amon_compiler: {msg}");
        std::process::exit(1);
    }
}
