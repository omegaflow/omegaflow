use omegaflow::auger::{
    decode_rec, encode_rec, parse_header, write_header, AugerRecord, HEADER_LEN, REC_BYTES,
};
use omegaflow::json::{jnum, parse_json};
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
    witness_s2_direction_identity(omegaflow::auger::MAGIC)?;
    let Some(input) = arg_value(args, "--input") else {
        return Err(
            "usage: auger_compiler --input <dir-of-PAO*.json> --out <map.pao1> [--ci-mode] — refused"
                .into(),
        );
    };
    let Some(out_path) = arg_value(args, "--out") else {
        return Err("--out <map.pao1>: the asset path is never silent — refused".into());
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let dir =
        std::fs::read_dir(&input).map_err(|e| format!("read_dir {input} returned void: {e}"))?;
    let mut names = BTreeSet::new();
    for entry in dir {
        let entry = entry.map_err(|e| format!("read_dir {input} returned void: {e}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with("PAO") && name.ends_with(".json") {
            names.insert(name);
        }
    }

    let mut records: Vec<AugerRecord> = Vec::new();
    let mut census_skip = [0u64; 6];
    let mut id_census = std::collections::BTreeMap::<String, u64>::new();
    for name in &names {
        let text = std::fs::read_to_string(format!("{input}/{name}"))
            .map_err(|e| format!("read {input}/{name} returned void: {e}"))?;
        let parsed = match parse_json(&text) {
            Some(j) => j,
            None => {
                census_skip[0] += 1;
                continue;
            }
        };
        let ra = match jnum(&parsed, "sdrec.ra") {
            Some(v) if v.is_finite() => v,
            _ => {
                census_skip[1] += 1;
                continue;
            }
        };
        let dec = match jnum(&parsed, "sdrec.dec") {
            Some(v) if v.is_finite() && (-90.0..=90.0).contains(&v) => v,
            _ => {
                census_skip[2] += 1;
                continue;
            }
        };
        let energy = match jnum(&parsed, "sdrec.energy") {
            Some(v) if v.is_finite() && v > 0.0 => v,
            _ => {
                census_skip[3] += 1;
                continue;
            }
        };
        let gpstime = match jnum(&parsed, "info.gpstime") {
            Some(v) if v.is_finite() && v > 0.0 => v as i64,
            _ => {
                census_skip[4] += 1;
                continue;
            }
        };
        let (order, ipix) = match AugerRecord::pixel_of(ra, dec) {
            Some(x) => x,
            None => {
                census_skip[5] += 1;
                continue;
            }
        };
        let id = name.trim_end_matches(".json").to_string();
        *id_census.entry(id).or_insert(0) += 1;
        records.push(AugerRecord {
            order,
            ipix,
            ra_deg: ra as f32,
            dec_deg: dec as f32,
            energy_eef: energy as f32,
            gpstime,
        });
    }
    if records.is_empty() {
        return Err("no decoded event — the asset stays unwritten (0 honored)".into());
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
        "{out_path}: {} events from {} PAO*.json files, header {} — roundtrip verified",
        records.len(),
        names.len(),
        n_rows
    );
    eprintln!(
        "last event: ra {:.4} dec {:.4} energy {:.3} EeV gpstime {}",
        last.ra_deg, last.dec_deg, last.energy_eef, last.gpstime
    );
    let mut max_energy = 0.0f32;
    for r in &records {
        if r.energy_eef > max_energy {
            max_energy = r.energy_eef;
        }
    }
    eprintln!(
        "skipped: {} bad-json, {} bad-ra, {} bad-dec, {} no-energy, {} no-time, {} unplaceable",
        census_skip[0],
        census_skip[1],
        census_skip[2],
        census_skip[3],
        census_skip[4],
        census_skip[5]
    );
    eprintln!("highest recorded energy {max_energy:.2} EeV");
    if id_census.len() != names.len() {
        eprintln!(
            "event ids seen {}, PAO files {} — duplicate ids held",
            id_census.len(),
            names.len()
        );
    }
    if ci_mode && !upload_asset(&out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("auger_compiler: {msg}");
        std::process::exit(1);
    }
}
