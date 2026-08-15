use omegaflow::cdn::upload_asset;
use omegaflow::dastcom::{encode_record, parse_db_record, state_at, RECORD_STRIDE};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const ASTEROID_RECORD_BYTES: u64 = 835;
const J2000_JD: f64 = 2451545.0;

fn compile_catalog(input: &str, out_path: &str) -> usize {
    let mut file = File::open(input).expect("open dast5_le.dat");
    let mut header = [0u8; 835];
    file.read_exact(&mut header).expect("header record");
    let ibias1 = i32::from_le_bytes(header[0..4].try_into().unwrap());
    let caldate = String::from_utf8_lossy(&header[52..71]).to_string();
    let ftyp = header[79] as char;
    eprintln!(
        "dast5: ibias1_raw={} caldate={} ftyp={}",
        ibias1, caldate, ftyp
    );
    let mut buf = Vec::with_capacity(1_600_000 * RECORD_STRIDE);
    let mut record_buf = [0u8; 835];
    let mut written = 0usize;
    let mut skipped = 0usize;
    loop {
        match file.read_exact(&mut record_buf) {
            Ok(()) => {}
            Err(_) => break,
        }
        let rec = match parse_db_record(&record_buf) {
            Some(r) => r,
            None => {
                skipped += 1;
                continue;
            }
        };
        if rec.number == 0 || rec.a_au <= 0.0 || rec.e >= 1.0 {
            skipped += 1;
            continue;
        }
        encode_record(&rec, &mut buf);
        written += 1;
    }
    std::fs::write(out_path, &buf).expect("write catalog");
    eprintln!(
        "dastcom: {} records, {} skipped, {} B → {}",
        written,
        skipped,
        buf.len(),
        out_path
    );
    written
}

fn probe_record(file: &mut File, number: u32, t_jd: f64) {
    file.seek(SeekFrom::Start(ASTEROID_RECORD_BYTES))
        .expect("seek past header");
    let mut record_buf = [0u8; 835];
    let mut offset: u64 = ASTEROID_RECORD_BYTES;
    let rec = loop {
        match file.read_exact(&mut record_buf) {
            Ok(()) => {}
            Err(_) => {
                eprintln!("probe: number {} not present in dast5_le.dat", number);
                return;
            }
        }
        let rec = match parse_db_record(&record_buf) {
            Some(r) => r,
            None => {
                eprintln!("probe: number {} not present in dast5_le.dat", number);
                return;
            }
        };
        offset += ASTEROID_RECORD_BYTES;
        if rec.number == number {
            break rec;
        }
    };
    let t = if t_jd == 0.0 { rec.epoch_jd } else { t_jd };
    eprintln!(
        "probe: number={} at offset {}, epoch={:.5}",
        rec.number,
        offset - ASTEROID_RECORD_BYTES,
        rec.epoch_jd
    );
    match state_at(&rec, t) {
        Some((p, _)) => eprintln!(
            "probe: t={:.5} icrs_km x={:.6} y={:.6} z={:.6} | a={} e={} incl={} node={} peri={} ma={} h={} albedo={} rot={} rad={} gm={}",
            t, p[0] / 1000.0, p[1] / 1000.0, p[2] / 1000.0,
            rec.a_au, rec.e, rec.incl_deg, rec.node_deg, rec.peri_deg, rec.ma_deg,
            rec.h, rec.albedo, rec.rot_period_h, rec.radius_km, rec.gm_km3_s2
        ),
        None => eprintln!("probe: elements outside Kepler domain"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: dastcom_compiler --input <dast5_le.dat> --out <dastcom_asteroids.bin> [--ci-mode] [--probe <no> <jd>]");
        std::process::exit(1);
    }
    let mut input: Option<String> = None;
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut probe: Option<(u32, f64)> = None;
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
            "--ci-mode" => ci_mode = true,
            "--probe" => {
                let no = args.get(i + 1).and_then(|n| n.parse().ok()).unwrap_or(0);
                let jd = args
                    .get(i + 2)
                    .and_then(|j| j.parse().ok())
                    .unwrap_or(J2000_JD);
                probe = Some((no, jd));
                i += 2;
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
    if let Some((no, jd)) = probe {
        let mut file = File::open(&input).expect("open dast5_le.dat");
        probe_record(&mut file, no, jd);
        return;
    }
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    compile_catalog(&input, &out_path);
    if ci_mode && !upload_asset(&out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}
