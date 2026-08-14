use omegaflow::cdn::upload_asset;
use omegaflow::kepler::elements_to_icrs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const ASTEROID_RECORD_BYTES: u64 = 835;
const CATALOG_STRIDE: usize = 92;
const J2000_JD: f64 = 2451545.0;

struct AsteroidRecord {
    number: u32,
    epoch_jd: f64,
    a_au: f64,
    e: f64,
    incl_deg: f64,
    node_deg: f64,
    peri_deg: f64,
    ma_deg: f64,
    h: f32,
    g: f32,
    albedo: f32,
    rot_period_h: f32,
    radius_km: f32,
    gm_km3_s2: f32,
    sptype: [u8; 5],
}

fn f32_at(buf: &[u8], off: usize) -> f32 {
    f32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

fn f64_at(buf: &[u8], off: usize) -> f64 {
    f64::from_le_bytes(buf[off..off + 8].try_into().unwrap())
}

fn parse_record(buf: &[u8; 835]) -> AsteroidRecord {
    let mut sptype = [0u8; 5];
    sptype.copy_from_slice(&buf[646..651]);
    AsteroidRecord {
        number: i32::from_le_bytes(buf[0..4].try_into().unwrap()) as u32,
        epoch_jd: f64_at(buf, 16),
        a_au: f64_at(buf, 72),
        e: f64_at(buf, 64),
        incl_deg: f64_at(buf, 56),
        node_deg: f64_at(buf, 48),
        peri_deg: f64_at(buf, 40),
        ma_deg: f64_at(buf, 32),
        h: f32_at(buf, 492),
        g: f32_at(buf, 496),
        albedo: f32_at(buf, 588),
        rot_period_h: f32_at(buf, 560),
        radius_km: f32_at(buf, 568),
        gm_km3_s2: f32_at(buf, 564),
        sptype,
    }
}

fn encode_catalog(rec: &AsteroidRecord, out: &mut Vec<u8>) {
    out.extend_from_slice(&rec.number.to_le_bytes());
    out.extend_from_slice(&rec.epoch_jd.to_le_bytes());
    out.extend_from_slice(&rec.a_au.to_le_bytes());
    out.extend_from_slice(&rec.e.to_le_bytes());
    out.extend_from_slice(&rec.incl_deg.to_le_bytes());
    out.extend_from_slice(&rec.node_deg.to_le_bytes());
    out.extend_from_slice(&rec.peri_deg.to_le_bytes());
    out.extend_from_slice(&rec.ma_deg.to_le_bytes());
    out.extend_from_slice(&rec.h.to_le_bytes());
    out.extend_from_slice(&rec.g.to_le_bytes());
    out.extend_from_slice(&rec.albedo.to_le_bytes());
    out.extend_from_slice(&rec.rot_period_h.to_le_bytes());
    out.extend_from_slice(&rec.radius_km.to_le_bytes());
    out.extend_from_slice(&rec.gm_km3_s2.to_le_bytes());
    out.extend_from_slice(&rec.sptype);
    out.extend_from_slice(&[0u8; 3]);
}

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
    let mut buf = Vec::with_capacity(1_600_000 * CATALOG_STRIDE);
    let mut record_buf = [0u8; 835];
    let mut written = 0usize;
    let mut skipped = 0usize;
    loop {
        match file.read_exact(&mut record_buf) {
            Ok(()) => {}
            Err(_) => break,
        }
        let rec = parse_record(&record_buf);
        if rec.number == 0 || rec.a_au <= 0.0 || rec.e >= 1.0 {
            skipped += 1;
            continue;
        }
        encode_catalog(&rec, &mut buf);
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
        let rec = parse_record(&record_buf);
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
    let r = elements_to_icrs(
        rec.a_au,
        rec.e,
        rec.incl_deg,
        rec.node_deg,
        rec.peri_deg,
        rec.ma_deg,
        rec.epoch_jd,
        t,
    );
    match r {
        Some(p) => eprintln!(
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
    if ci_mode {
        let _ = upload_asset(&out_path);
    }
}
