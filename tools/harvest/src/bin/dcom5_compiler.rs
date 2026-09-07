use omegaflow::cdn::upload_asset;
use omegaflow::dastcom::{comet_state_at, parse_comet_record, COMET_RECORD_BYTES};
use omegaflow::kepler::AU_M;
use std::io::Write;

struct CometRow {
    name: String,
    ra_deg: f64,
    dec_deg: f64,
    dist_au: f64,
    h: Option<f32>,
    m1: Option<f32>,
}

fn trim(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

fn row_name(rec: &omegaflow::dastcom::CometRec) -> String {
    let sb = trim(&rec.sbnam);
    if !sb.is_empty() {
        return sb;
    }
    trim(&rec.desig)
}

fn evaluate(rec: &omegaflow::dastcom::CometRec) -> Option<CometRow> {
    if rec.ec >= 1.0 || rec.a_au <= 0.0 || rec.qr_au <= 0.0 {
        return None;
    }
    let (p, _) = comet_state_at(rec, rec.epoch_jd)?;
    let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
    if !r.is_finite() || r <= 0.0 {
        return None;
    }
    let ra_deg = p[1].atan2(p[0]).to_degrees().rem_euclid(360.0);
    let dec_deg = (p[2] / r).asin().to_degrees();
    Some(CometRow {
        name: row_name(rec),
        ra_deg,
        dec_deg,
        dist_au: r / AU_M,
        h: (rec.h.is_finite() && rec.h < 90.0).then_some(rec.h),
        m1: (rec.m1.is_finite() && rec.m1 < 900.0).then_some(rec.m1),
    })
}

fn json_string(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            _ => o.push(c),
        }
    }
    o.push('"');
    o
}

fn open_records(input: &str) -> Option<(std::fs::File, Vec<omegaflow::dastcom::CometRec>)> {
    let mut file = std::fs::File::open(input).ok()?;
    let mut header = [0u8; COMET_RECORD_BYTES];
    std::io::Read::read_exact(&mut file, &mut header).ok()?;
    let ftyp = header[79];
    if ftyp != b'5' {
        eprintln!(
            "header FTYP byte 80 = {} (not '5' — not a DASTCOM5 comet file)",
            ftyp as char
        );
        return None;
    }
    let mut recs = Vec::new();
    let mut buf = [0u8; COMET_RECORD_BYTES];
    loop {
        match std::io::Read::read_exact(&mut file, &mut buf) {
            Ok(()) => match parse_comet_record(&buf) {
                Some(r) => recs.push(r),
                None => {}
            },
            Err(_) => break,
        }
    }
    Some((file, recs))
}

fn read_records(input: &str) -> Option<Vec<omegaflow::dastcom::CometRec>> {
    open_records(input).map(|(_, recs)| recs)
}

fn primary_key(rec: &omegaflow::dastcom::CometRec) -> String {
    let desig = trim(&rec.desig);
    if !desig.is_empty() {
        return desig;
    }
    trim(&rec.comnam)
}

fn write_comet_catalog(input: &str, out_path: &str) -> Option<usize> {
    let mut file = std::fs::File::open(input).ok()?;
    let mut header = [0u8; COMET_RECORD_BYTES];
    std::io::Read::read_exact(&mut file, &mut header).ok()?;
    if header[79] != b'5' {
        return None;
    }
    let mut slots: Vec<Vec<u8>> = Vec::new();
    let mut slot_of: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut buf = [0u8; COMET_RECORD_BYTES];
    loop {
        match std::io::Read::read_exact(&mut file, &mut buf) {
            Ok(()) => {}
            Err(_) => break,
        }
        let Some(rec) = parse_comet_record(&buf) else {
            continue;
        };
        if rec.ec >= 1.0 || !(rec.a_au > 0.0) {
            continue;
        }
        let key = primary_key(&rec);
        let raw = buf.to_vec();
        match slot_of.get(&key).copied() {
            Some(idx) => {
                let Some(held) = parse_comet_record(&slots[idx]) else {
                    continue;
                };
                if rec.epoch_jd > held.epoch_jd {
                    slots[idx] = raw;
                }
            }
            None => {
                slot_of.insert(key, slots.len());
                slots.push(raw);
            }
        }
    }
    let mut buf = Vec::with_capacity(slots.len() * COMET_RECORD_BYTES);
    for s in &slots {
        buf.extend_from_slice(s);
    }
    std::fs::write(out_path, &buf).ok()?;
    eprintln!(
        "dcom5: comet catalog {} records (current solution per primary designation, elliptic), {} B → {}",
        slots.len(),
        buf.len(),
        out_path
    );
    Some(slots.len())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input: Option<String> = None;
    let mut out: Option<String> = None;
    let mut catalog: Option<String> = None;
    let mut ci_mode = false;
    let mut probe: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--catalog" => {
                catalog = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--probe" => {
                probe = args.get(i + 1).cloned();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let input = match input {
        Some(p) => p,
        None => {
            eprintln!("--input absent");
            std::process::exit(1);
        }
    };
    let Some(recs) = read_records(&input) else {
        eprintln!("read {} returned void", input);
        std::process::exit(1);
    };
    eprintln!("dcom5: {} records", recs.len());
    if let Some(pat) = &probe {
        let lower = pat.to_lowercase();
        let mut shown = 0usize;
        for rec in &recs {
            let id = format!(
                "{}|{}|{}",
                trim(&rec.sbnam),
                trim(&rec.desig),
                trim(&rec.comnam)
            )
            .to_lowercase();
            if id.contains(&lower) {
                match evaluate(rec) {
                    Some(r) => eprintln!(
                        "probe {}: e={} q={} au a={} au epoch={:.5} tp={:.5} ma={} → ra={:.6} dec={:.6} dist={:.6} au H={} M1={} G={} rad={} km albedo={} nobs={}",
                        r.name,
                        rec.ec,
                        rec.qr_au,
                        rec.a_au,
                        rec.epoch_jd,
                        rec.tp_jd,
                        rec.ma_deg,
                        r.ra_deg,
                        r.dec_deg,
                        r.dist_au,
                        rec.h,
                        rec.m1,
                        rec.g,
                        rec.rad_km,
                        rec.albedo,
                        rec.nobs
                    ),
                    None => eprintln!(
                        "probe {}: e={} outside Kepler domain (0 honored)",
                        trim(&rec.sbnam),
                        rec.ec
                    ),
                }
                shown += 1;
                if shown >= 32 {
                    break;
                }
            }
        }
        if shown == 0 {
            eprintln!("probe: {} not present", pat);
        }
        return;
    }
    if let Some(catalog_path) = &catalog {
        match write_comet_catalog(&input, catalog_path) {
            Some(_) => {}
            None => {
                eprintln!("catalog {} returned void", catalog_path);
                std::process::exit(1);
            }
        }
        if ci_mode && !upload_asset(catalog_path) {
            eprintln!("upload: {} did not reach the CDN", catalog_path);
            std::process::exit(1);
        }
        return;
    }
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    let mut rows: Vec<CometRow> = Vec::new();
    let mut skipped = 0usize;
    for rec in &recs {
        match evaluate(rec) {
            Some(r) => rows.push(r),
            None => skipped += 1,
        }
    }
    let mut buf = String::from("[");
    for (k, r) in rows.iter().enumerate() {
        if k > 0 {
            buf.push(',');
        }
        buf.push_str(&format!(
            "{{\"name\":{},\"ra\":{},\"dec\":{},\"dist_au\":{},\"H\":{},\"M1\":{}}}",
            json_string(&r.name),
            r.ra_deg,
            r.dec_deg,
            r.dist_au,
            match r.h {
                Some(v) => v.to_string(),
                None => "null".to_string(),
            },
            match r.m1 {
                Some(v) => v.to_string(),
                None => "null".to_string(),
            }
        ));
    }
    buf.push_str("]\n");
    match std::fs::File::create(&out_path) {
        Ok(mut f) => {
            if let Err(err) = f.write_all(buf.as_bytes()) {
                eprintln!("write {}: {}", out_path, err);
                std::process::exit(1);
            }
        }
        Err(_) => {
            eprintln!("write {} returned void", out_path);
            std::process::exit(1);
        }
    }
    eprintln!(
        "dcom5: {} records written, {} skipped (e>=1 or void fields), {} B → {}",
        rows.len(),
        skipped,
        buf.len(),
        out_path
    );
    if ci_mode && !upload_asset(&out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}
