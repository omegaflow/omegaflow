use omegaflow::cdn::upload_asset;
use omegaflow::dastcom::{encode_record, parse_db_record, state_at, RECORD_STRIDE};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const ASTEROID_RECORD_BYTES: u64 = 835;
const J2000_JD: f64 = 2451545.0;

const GM_CATALOG_DEFAULT: &str = "phi/pipeline/katalog/asteroid_gm_inpop25c.φ";
const DIAMETER_CATALOGS_DEFAULT: &str =
    "phi/pipeline/katalog/asteroid_diameters_neowise.φ,phi/pipeline/katalog/asteroid_diameters_akari.φ";

fn read_gm_catalog(path: &str) -> HashMap<u32, f64> {
    let mut map = HashMap::new();
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("gm catalog {} absent", path);
            return map;
        }
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut cells = line.split('|');
        let number = cells
            .next()
            .and_then(|c| c.trim().parse().ok())
            .unwrap_or(0u32);
        let gm_m3s2 = cells
            .next()
            .and_then(|c| c.trim().parse().ok())
            .unwrap_or(0.0f64);
        if number == 0 || !(gm_m3s2 > 0.0) {
            continue;
        }
        map.insert(number, gm_m3s2 / 1.0e9);
    }
    map
}

fn read_diameter_catalog(path: &str, map: &mut HashMap<u32, f32>) {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("diameter catalog {} absent", path);
            return;
        }
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut cells = line.split('|');
        let number = cells
            .next()
            .and_then(|c| c.trim().parse().ok())
            .unwrap_or(0u32);
        let diam_km = cells
            .next()
            .and_then(|c| c.trim().parse().ok())
            .unwrap_or(0.0f32);
        if number == 0 || !(diam_km > 0.0) {
            continue;
        }
        map.entry(number).or_insert(diam_km);
    }
}

fn compile_catalog(
    input: &str,
    out_path: &str,
    gm: &HashMap<u32, f64>,
    diam: &HashMap<u32, f32>,
) -> usize {
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
    let mut gm_filled = 0usize;
    let mut radius_filled = 0usize;
    loop {
        match file.read_exact(&mut record_buf) {
            Ok(()) => {}
            Err(_) => break,
        }
        let mut rec = match parse_db_record(&record_buf) {
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
        if rec.gm_km3_s2 <= 0.0 {
            if let Some(&g) = gm.get(&rec.number) {
                rec.gm_km3_s2 = g as f32;
                gm_filled += 1;
            }
        }
        if rec.radius_km <= 0.0 {
            if let Some(&d) = diam.get(&rec.number) {
                rec.radius_km = d * 0.5;
                radius_filled += 1;
            }
        }
        encode_record(&rec, &mut buf);
        written += 1;
    }
    std::fs::write(out_path, &buf).expect("write catalog");
    eprintln!(
        "dastcom: {} records, {} skipped, {} B → {} (gm {} diam {})",
        written,
        skipped,
        buf.len(),
        out_path,
        gm_filled,
        radius_filled
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
        eprintln!("usage: dastcom_compiler --input <dast5_le.dat> --out <dastcom_asteroids.bin> [--gm <φ>] [--diameters <φ,φ>] [--ci-mode] [--probe <no> <jd>]");
        std::process::exit(1);
    }
    let mut input: Option<String> = None;
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut probe: Option<(u32, f64)> = None;
    let mut gm_path: Option<String> = None;
    let mut diameter_paths: Option<String> = None;
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
            "--gm" => {
                gm_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--diameters" => {
                diameter_paths = args.get(i + 1).cloned();
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
    let gm = read_gm_catalog(gm_path.as_deref().unwrap_or(GM_CATALOG_DEFAULT));
    let mut diam: HashMap<u32, f32> = HashMap::new();
    let diam_list = diameter_paths
        .as_deref()
        .unwrap_or(DIAMETER_CATALOGS_DEFAULT);
    for path in diam_list.split(',') {
        read_diameter_catalog(path.trim(), &mut diam);
    }
    eprintln!("dastcom join: {} gm, {} diameters", gm.len(), diam.len());
    compile_catalog(&input, &out_path, &gm, &diam);
    if ci_mode && !upload_asset(&out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_tmp(name: &str, content: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(name);
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        path
    }

    #[test]
    fn gm_catalog_parses_to_km3s2() {
        let path = write_tmp(
            "omegaflow_test_gm.φ",
            "# header\n45 | 4.239e8 | 6.73e7\n87 | 1.219e9 | 1.57e8\n",
        );
        let map = read_gm_catalog(path.to_str().unwrap());
        assert!((map[&45] - 0.4239).abs() < 1e-6);
        assert!((map[&87] - 1.219).abs() < 1e-6);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn diameter_catalog_first_source_wins() {
        let p1 = write_tmp("omegaflow_test_d1.φ", "# n\n2 | 544 | 0.1417\n");
        let p2 = write_tmp(
            "omegaflow_test_d2.φ",
            "# n\n2 | 512 | 0.15\n3 | 100 | 0.2\n",
        );
        let mut map = HashMap::new();
        read_diameter_catalog(p1.to_str().unwrap(), &mut map);
        read_diameter_catalog(p2.to_str().unwrap(), &mut map);
        assert_eq!(map[&2], 544.0);
        assert_eq!(map[&3], 100.0);
        let _ = std::fs::remove_file(&p1);
        let _ = std::fs::remove_file(&p2);
    }
}
