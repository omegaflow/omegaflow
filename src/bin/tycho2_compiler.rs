// Tycho-2 (VizieR I/259: tyc2.dat.00-19.gz + suppl_1/2) + Hipparcos (I/239:
// hip_main.dat → Plx + Vmag) → Stern-Katalog-Bin (36-B-Records) für den
// runtime catalog_tycho-Kanal. Nur Sterne mit plx > 0 manifestieren — die
// übrigen sind absent (0 honored). Suppl-Positionen (J1991.25) werden auf
// J2000 propagiert. Byte-Layouts: CDS ReadMe I/259 und I/239.

use omegaflow::cdn::upload_asset;
use omegaflow::inflate::gunzip;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::process::Command;

const STAR_RECORD_STRIDE: usize = 44;

struct StarRow {
    hip: i32,
    ra_deg: f64,
    dec_deg: f64,
    pm_ra: f64,
    pm_de: f64,
    plx_mas: f64,
    mag: f64,
    rv: f64,
    from_suppl: bool,
}

#[derive(Clone, Copy)]
struct SupplRow {
    ra_deg: f64,
    dec_deg: f64,
    pm_ra: f64,
    pm_de: f64,
    mag: f64,
    hip: i32,
}

fn load_tyc1(path: &str, map: &mut HashMap<(i32, i32, i32), SupplRow>) -> usize {
    let Ok(data) = std::fs::read(path) else {
        return 0;
    };
    let text = String::from_utf8_lossy(&data);
    let mut n = 0;
    for line in text.lines() {
        let b = line.as_bytes();
        if b.len() < 236 {
            continue;
        }
        let (Some(ra), Some(dec)) = (num(b, 52, 63), num(b, 65, 76)) else {
            continue;
        };
        let Some(pm_ra) = num(b, 88, 95) else {
            continue;
        };
        let Some(pm_de) = num(b, 97, 104) else {
            continue;
        };
        let Some(vt) = num(b, 231, 236) else {
            continue;
        };
        let hip: i32 = field(b, 211, 216)
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0);
        let (Some(t1s), Some(t2s), Some(t3s)) = (field(b, 3, 6), field(b, 8, 11), field(b, 13, 14))
        else {
            continue;
        };
        let (Ok(t1), Ok(t2), Ok(t3)) = (
            t1s.trim().parse::<i32>(),
            t2s.trim().parse::<i32>(),
            t3s.trim().parse::<i32>(),
        ) else {
            continue;
        };
        map.insert(
            (t1, t2, t3),
            SupplRow {
                ra_deg: ra,
                dec_deg: dec,
                pm_ra,
                pm_de,
                mag: vt,
                hip,
            },
        );
        n += 1;
    }
    n
}

fn field(line: &[u8], lo: usize, hi: usize) -> Option<&str> {
    std::str::from_utf8(line.get(lo - 1..hi)?).ok()
}

fn num(line: &[u8], lo: usize, hi: usize) -> Option<f64> {
    field(line, lo, hi)?.trim().parse::<f64>().ok()
}

fn tyc_key(line: &str) -> Option<(i32, i32, i32)> {
    let b = line.as_bytes();
    let t1: i32 = field(b, 1, 4)?.trim().parse().ok()?;
    let t2: i32 = field(b, 6, 10)?.trim().parse().ok()?;
    let t3: i32 = field(b, 12, 12)?.trim().parse().ok()?;
    Some((t1, t2, t3))
}

fn load_suppl(path: &str, map: &mut HashMap<(i32, i32, i32), SupplRow>) -> usize {
    let Some(bytes) = std::fs::read(path).ok() else {
        return 0;
    };
    let Some(text) = gunzip(&bytes) else {
        return 0;
    };
    let mut n = 0;
    for line in String::from_utf8_lossy(&text).lines() {
        let b = line.as_bytes();
        let (Some(ra), Some(dec)) = (num(b, 16, 27), num(b, 29, 40)) else {
            continue;
        };
        let Some(pm_ra) = num(b, 42, 48) else {
            continue;
        };
        let Some(pm_de) = num(b, 50, 56) else {
            continue;
        };
        let Some(mag) = num(b, 97, 102) else {
            continue;
        };
        let hip: i32 = field(b, 116, 121)
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0);
        let Some(key) = tyc_key(line) else {
            continue;
        };
        map.insert(
            key,
            SupplRow {
                ra_deg: ra,
                dec_deg: dec,
                pm_ra,
                pm_de,
                mag,
                hip,
            },
        );
        n += 1;
    }
    n
}

fn propagate(ra: f64, dec: f64, pm_ra: f64, pm_de: f64, dt_yr: f64) -> (f64, f64) {
    let dec_rad = dec.to_radians();
    let ra_j = ra + pm_ra / (3.6e6 * dec_rad.cos().max(1e-6)) * dt_yr;
    let dec_j = dec + pm_de / 3.6e6 * dt_yr;
    (ra_j, dec_j)
}

fn propagate_j1991_25(ra: f64, dec: f64, pm_ra: f64, pm_de: f64) -> (f64, f64) {
    propagate(ra, dec, pm_ra, pm_de, 8.75)
}

fn parse_tgas_record(line: &str) -> Option<StarRow> {
    let f: Vec<&str> = line.split('|').collect();
    if f.len() < 54 {
        return None;
    }
    let hip: i32 = f[0].trim().parse().ok().unwrap_or(0);
    let ra = f[6].trim().parse::<f64>().ok()?;
    let dec = f[8].trim().parse::<f64>().ok()?;
    let plx = f[10].trim().parse::<f64>().ok()?;
    let pm_ra = f[12].trim().parse::<f64>().ok()?;
    let pm_de = f[14].trim().parse::<f64>().ok()?;
    let gmag = f[53].trim().parse::<f64>().ok()?;
    let (ra_j, dec_j) = propagate(ra, dec, pm_ra, pm_de, -15.0);
    if !ra_j.is_finite() || !dec_j.is_finite() || !gmag.is_finite() {
        return None;
    }
    Some(StarRow {
        hip,
        ra_deg: ra_j,
        dec_deg: dec_j,
        pm_ra,
        pm_de,
        plx_mas: plx,
        mag: gmag,
        rv: 0.0,
        from_suppl: false,
    })
}

fn parse_tyc2_record(
    line: &str,
    suppl: &HashMap<(i32, i32, i32), SupplRow>,
    tyc1: &HashMap<(i32, i32, i32), SupplRow>,
    used: &mut HashSet<(i32, i32, i32)>,
) -> Option<StarRow> {
    let b = line.as_bytes();
    let key = tyc_key(line)?;
    let mut ra = num(b, 16, 27);
    let mut dec = num(b, 29, 40);
    let mut pm_ra = num(b, 42, 48);
    let mut pm_de = num(b, 50, 56);
    let mut hip: i32 = field(b, 143, 148)
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    if ra.is_none() || dec.is_none() {
        let Some(s) = suppl.get(&key).or_else(|| tyc1.get(&key)) else {
            return None;
        };
        if suppl.contains_key(&key) {
            used.insert(key);
        }
        let (ra_j, dec_j) = propagate_j1991_25(s.ra_deg, s.dec_deg, s.pm_ra, s.pm_de);
        ra = Some(ra_j);
        dec = Some(dec_j);
        if pm_ra.is_none() {
            pm_ra = Some(s.pm_ra);
        }
        if pm_de.is_none() {
            pm_de = Some(s.pm_de);
        }
        if hip == 0 {
            hip = s.hip;
        }
    }
    let vt = num(b, 124, 129)?;
    let ra = ra?;
    let dec = dec?;
    let pm_ra = pm_ra?;
    let pm_de = pm_de?;
    if !ra.is_finite() || !dec.is_finite() || !vt.is_finite() {
        return None;
    }
    Some(StarRow {
        hip,
        ra_deg: ra,
        dec_deg: dec,
        pm_ra,
        pm_de,
        plx_mas: 0.0,
        mag: vt,
        rv: 0.0,
        from_suppl: false,
    })
}

fn load_hip(path: &str) -> Option<HashMap<i32, (f64, f64)>> {
    let data = std::fs::read(path).ok()?;
    let text = String::from_utf8_lossy(&data);
    let mut map = HashMap::new();
    for line in text.lines() {
        let b = line.as_bytes();
        if b.len() < 86 {
            continue;
        }
        let Some(hip_s) = field(b, 9, 14) else {
            continue;
        };
        let Ok(hip) = hip_s.trim().parse::<i32>() else {
            continue;
        };
        let plx = num(b, 80, 86).unwrap_or(0.0);
        if !plx.is_finite() || plx <= 0.0 {
            continue;
        }
        let Some(vmag) = num(b, 42, 46) else {
            continue;
        };
        map.insert(hip, (plx, vmag));
    }
    Some(map)
}

fn tap_query_csv(root: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-G")
        .arg("--max-time")
        .arg("120")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(root)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "crossmatch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn crossmatch_rv(hips: &[i32]) -> HashMap<i32, f64> {
    let mut out = HashMap::new();
    const BATCH: usize = 500;
    for chunk in hips.chunks(BATCH) {
        let ids = chunk
            .iter()
            .map(|h| format!("'{}'", h))
            .collect::<Vec<_>>()
            .join(",");
        let adql = format!(
            "SELECT t.original_ext_source_id, g.radial_velocity FROM gaiadr3.hipparcos2_best_neighbour AS t JOIN gaiadr3.gaia_source AS g ON t.source_id = g.source_id WHERE t.original_ext_source_id IN ({}) AND g.radial_velocity IS NOT NULL AND t.angular_distance < 1.5",
            ids
        );
        let Some(body) = tap_query_csv("https://gea.esac.esa.int/tap-server/tap/sync", &adql)
        else {
            continue;
        };
        for line in body.lines().skip(1) {
            let mut parts = line.split(',');
            let (Some(hs), Some(rs)) = (parts.next(), parts.next()) else {
                continue;
            };
            let (Ok(hip), Ok(rv)) = (hs.trim().parse::<i32>(), rs.trim().parse::<f64>()) else {
                continue;
            };
            if rv.is_finite() {
                out.insert(hip, rv * 1000.0);
            }
        }
    }
    out
}

fn encode(row: &StarRow, out: &mut Vec<u8>) {
    out.extend_from_slice(&row.ra_deg.to_le_bytes());
    out.extend_from_slice(&row.dec_deg.to_le_bytes());
    out.extend_from_slice(&(row.pm_ra as f32).to_le_bytes());
    out.extend_from_slice(&(row.pm_de as f32).to_le_bytes());
    out.extend_from_slice(&(row.plx_mas as f32).to_le_bytes());
    out.extend_from_slice(&(row.mag as f32).to_le_bytes());
    out.extend_from_slice(&(10.0f64.powf(-0.4 * row.mag) as f32).to_le_bytes());
    out.extend_from_slice(&0f32.to_le_bytes());
    out.extend_from_slice(&(row.rv as f32).to_le_bytes());
}

fn bv_to_bp_rp(bv: f64) -> f64 {
    if !bv.is_finite() {
        return 0.0;
    }
    let x = bv.clamp(-0.5, 3.5);
    0.06483 + 1.575 * x - 0.7815 * x.powi(2) + 0.5707 * x.powi(3) - 0.176 * x.powi(4)
        + 0.01916 * x.powi(5)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut source: String = "tycho2".to_string();
    let mut input: Option<String> = None;
    let mut input_dir: Option<String> = None;
    let mut hip: Option<String> = None;
    let mut tyc1_path: Option<String> = None;
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut probe: Option<i32> = None;
    let mut xmatch = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--source" => {
                source = args.get(i + 1).cloned().unwrap_or_default();
                i += 1;
            }
            "--input" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--input-dir" => {
                input_dir = args.get(i + 1).cloned();
                i += 1;
            }
            "--hip" => {
                hip = args.get(i + 1).cloned();
                i += 1;
            }
            "--tyc1" => {
                tyc1_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--xmatch" => xmatch = true,
            "--probe" => {
                probe = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    if source == "tgas" {
        let input = match input {
            Some(p) => p,
            None => {
                eprintln!("--input absent (tgas.dat.gz)");
                std::process::exit(1);
            }
        };
        let bytes = match std::fs::read(&input) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("read {} returned void", input);
                std::process::exit(1);
            }
        };
        let Some(text) = gunzip(&bytes) else {
            eprintln!("gunzip {} returned void", input);
            std::process::exit(1);
        };
        let mut rows: Vec<StarRow> = Vec::with_capacity(2_100_000);
        let mut skipped = 0usize;
        for line in String::from_utf8_lossy(&text).lines() {
            match parse_tgas_record(line) {
                Some(r) => rows.push(r),
                None => skipped += 1,
            }
        }
        let hip_map = match &hip {
            Some(p) => load_hip(p).unwrap_or_default(),
            None => HashMap::new(),
        };
        let mut recovered = 0usize;
        for row in rows.iter_mut() {
            if row.plx_mas <= 0.0 && row.hip > 0 {
                if let Some(&(plx, _)) = hip_map.get(&row.hip) {
                    row.plx_mas = plx;
                    recovered += 1;
                }
            }
        }
        eprintln!(
            "tgas: {} rows, {} ohne plx, davon {} via Hipparcos-Join wiederhergestellt",
            rows.len(),
            rows.iter().filter(|r| r.plx_mas <= 0.0).count() + 0,
            recovered
        );
        if let Some(p) = probe {
            for row in &rows {
                if row.hip != p {
                    continue;
                }
                eprintln!(
                    "probe HIP {}: ra={:.8} dec={:.8} pmRA={} pmDE={} mas/yr plx={} mas Gmag={} dist={:.1} pc",
                    row.hip,
                    row.ra_deg,
                    row.dec_deg,
                    row.pm_ra,
                    row.pm_de,
                    row.plx_mas,
                    row.mag,
                    1000.0 / row.plx_mas
                );
                return;
            }
            eprintln!("probe: HIP {} not present", p);
            return;
        }
        let out_path = match out {
            Some(p) => p,
            None => {
                eprintln!("--out absent");
                std::process::exit(1);
            }
        };
        let rv_map = if xmatch {
            let hips: Vec<i32> = rows.iter().map(|r| r.hip).filter(|h| *h > 0).collect();
            let m = crossmatch_rv(&hips);
            eprintln!(
                "crossmatch: {} of {} tgas stars with rv",
                m.len(),
                hips.len()
            );
            m
        } else {
            HashMap::new()
        };
        let mut buf = Vec::with_capacity(rows.len() * STAR_RECORD_STRIDE);
        let mut written = 0usize;
        let mut no_rv = 0usize;
        for mut row in rows {
            if !row.plx_mas.is_finite() || row.plx_mas <= 0.0 {
                continue;
            }
            let Some(rv) = rv_map.get(&row.hip).copied() else {
                no_rv += 1;
                continue;
            };
            row.rv = rv;
            encode(&row, &mut buf);
            written += 1;
        }
        if let Ok(mut f) = std::fs::File::create(&out_path) {
            let _ = f.write_all(&buf);
        } else {
            eprintln!("write {} returned void", out_path);
            std::process::exit(1);
        }
        eprintln!(
            "tgas: {} records written (plx>0, rv present), {} without rv, {} skipped, {} B → {}",
            written,
            no_rv,
            skipped,
            buf.len(),
            out_path
        );
        if ci_mode && !upload_asset(&out_path) {
            eprintln!("upload: {} did not reach the CDN", out_path);
            std::process::exit(1);
        }
        return;
    }
    if source == "bright" {
        let input = match input {
            Some(p) => p,
            None => {
                eprintln!("--input absent (hip_main.dat)");
                std::process::exit(1);
            }
        };
        let data = match std::fs::read(&input) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("read {} returned void", input);
                std::process::exit(1);
            }
        };
        let text = String::from_utf8_lossy(&data);
        let mut rows: Vec<(i32, String)> = Vec::new();
        for line in text.lines() {
            let b = line.as_bytes();
            if b.len() < 104 {
                continue;
            }
            let (Some(ra), Some(dec)) = (num(b, 52, 63), num(b, 65, 76)) else {
                continue;
            };
            let Some(plx) = num(b, 80, 86) else {
                continue;
            };
            let Some(vmag) = num(b, 42, 46) else {
                continue;
            };
            if !(plx > 0.0) || !(vmag < 1.94) {
                continue;
            }
            let hip: i32 = field(b, 9, 14)
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(0);
            let Some(pm_ra) = num(b, 88, 95) else {
                continue;
            };
            let Some(pm_de) = num(b, 97, 104) else {
                continue;
            };
            let bp_rp = num(b, 246, 251).map(bv_to_bp_rp);
            let (ra_j, dec_j) = propagate(ra, dec, pm_ra, pm_de, 8.75);
            rows.push((
                hip,
                format!(
                    "{{\"ra\":{},\"dec\":{},\"mag\":{},\"dist_pc\":{},\"pmra\":{},\"pmdec\":{},\"bp_rp\":{}}}",
                    ra_j,
                    dec_j,
                    vmag,
                    1000.0 / plx,
                    pm_ra,
                    pm_de,
                    match bp_rp {
                        Some(v) => v.to_string(),
                        None => "null".to_string(),
                    }
                ),
            ));
        }
        let rv_map = if xmatch {
            let hips: Vec<i32> = rows.iter().map(|(h, _)| *h).filter(|h| *h > 0).collect();
            let m = crossmatch_rv(&hips);
            eprintln!(
                "crossmatch: {} of {} bright stars with rv",
                m.len(),
                hips.len()
            );
            m
        } else {
            HashMap::new()
        };
        let rows_len = rows.len();
        let mut rows_out: Vec<String> = Vec::with_capacity(rows_len);
        let mut no_rv = 0usize;
        for (hip, mut r) in rows {
            let Some(rv) = rv_map.get(&hip).copied() else {
                no_rv += 1;
                continue;
            };
            r.pop();
            r.push_str(&format!(",\"rv\":{}}}", rv));
            rows_out.push(r);
        }
        let out_path = match out {
            Some(p) => p,
            None => {
                eprintln!("--out absent");
                std::process::exit(1);
            }
        };
        let mut buf = String::from("[");
        for (k, r) in rows_out.iter().enumerate() {
            if k > 0 {
                buf.push(',');
            }
            buf.push_str(r);
        }
        buf.push_str("]\n");
        if let Ok(mut f) = std::fs::File::create(&out_path) {
            let _ = f.write_all(buf.as_bytes());
        } else {
            eprintln!("write {} returned void", out_path);
            std::process::exit(1);
        }
        eprintln!(
            "bright: {} records (V<1.94, plx>0), {} without rv (0 honored) → {}",
            rows_len, no_rv, out_path
        );
        return;
    }
    let dir = match input_dir {
        Some(d) => d,
        None => {
            eprintln!("--input-dir absent");
            std::process::exit(1);
        }
    };
    let hip_path = match hip {
        Some(h) => h,
        None => {
            eprintln!("--hip absent");
            std::process::exit(1);
        }
    };
    let hip_map = match load_hip(&hip_path) {
        Some(m) => m,
        None => {
            eprintln!("hip_main parse returned void");
            std::process::exit(1);
        }
    };
    eprintln!("hip_main: {} records with plx > 0", hip_map.len());
    let mut tyc1: HashMap<(i32, i32, i32), SupplRow> = HashMap::new();
    let t1 = match &tyc1_path {
        Some(p) => load_tyc1(p, &mut tyc1),
        None => 0,
    };
    eprintln!("tyc1: {} records", t1);
    let mut suppl: HashMap<(i32, i32, i32), SupplRow> = HashMap::new();
    let s1 = load_suppl(&format!("{}/suppl_1.dat.gz", dir), &mut suppl);
    let s2 = load_suppl(&format!("{}/suppl_2.dat.gz", dir), &mut suppl);
    eprintln!("supplements: {} + {} records", s1, s2);
    let mut used: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut rows: Vec<StarRow> = Vec::with_capacity(2_600_000);
    let mut skipped = 0usize;
    let mut parts: Vec<String> = (0..20).map(|n| format!("tyc2.dat.{:02}.gz", n)).collect();
    parts.sort();
    for part in &parts {
        let path = format!("{}/{}", dir, part);
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("read {} returned void", path);
                continue;
            }
        };
        let Some(text) = gunzip(&bytes) else {
            eprintln!("gunzip {} returned void", path);
            continue;
        };
        for line in String::from_utf8_lossy(&text).lines() {
            match parse_tyc2_record(line, &suppl, &tyc1, &mut used) {
                Some(r) => rows.push(r),
                None => skipped += 1,
            }
        }
    }
    let mut suppl_only = 0usize;
    for (key, s) in &suppl {
        if used.contains(key) {
            continue;
        }
        let (ra_j, dec_j) = propagate_j1991_25(s.ra_deg, s.dec_deg, s.pm_ra, s.pm_de);
        rows.push(StarRow {
            hip: s.hip,
            ra_deg: ra_j,
            dec_deg: dec_j,
            pm_ra: s.pm_ra,
            pm_de: s.pm_de,
            plx_mas: 0.0,
            mag: s.mag,
            rv: 0.0,
            from_suppl: true,
        });
        suppl_only += 1;
    }
    eprintln!(
        "tyc2: {} main rows, {} suppl-only rows, {} unparseable",
        rows.len() - suppl_only,
        suppl_only,
        skipped
    );
    if let Some(p) = probe {
        for row in &rows {
            if row.hip != p {
                continue;
            }
            let (plx, vmag) = hip_map.get(&row.hip).copied().unwrap_or((0.0, row.mag));
            eprintln!(
                "probe HIP {}: ra={:.8} dec={:.8} pmRA={} pmDE={} mas/yr plx={} mas mag={} (hip V={}) dist={:.1} pc",
                row.hip,
                row.ra_deg,
                row.dec_deg,
                row.pm_ra,
                row.pm_de,
                plx,
                row.mag,
                vmag,
                1000.0 / plx
            );
            return;
        }
        eprintln!("probe: HIP {} not present", p);
        return;
    }
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    let mut buf = Vec::with_capacity(rows.len() * STAR_RECORD_STRIDE);
    let mut written = 0usize;
    let mut no_plx = 0usize;
    let mut no_rv = 0usize;
    let rv_map = if xmatch {
        let hips: Vec<i32> = rows
            .iter()
            .map(|r| r.hip)
            .filter(|h| *h > 0 && hip_map.contains_key(h))
            .collect();
        let m = crossmatch_rv(&hips);
        eprintln!(
            "crossmatch: {} of {} tycho2 stars with rv",
            m.len(),
            hips.len()
        );
        m
    } else {
        HashMap::new()
    };
    for mut row in rows {
        match hip_map.get(&row.hip) {
            Some(&(plx, _)) if row.hip > 0 => {
                row.plx_mas = plx;
            }
            _ => {
                no_plx += 1;
                continue;
            }
        }
        if row.from_suppl {
            if let Some(&(_, vmag)) = hip_map.get(&row.hip) {
                row.mag = vmag;
            }
        }
        let Some(rv) = rv_map.get(&row.hip).copied() else {
            no_rv += 1;
            continue;
        };
        row.rv = rv;
        encode(&row, &mut buf);
        written += 1;
    }
    if let Ok(mut f) = std::fs::File::create(&out_path) {
        let _ = f.write_all(&buf);
    } else {
        eprintln!("write {} returned void", out_path);
        std::process::exit(1);
    }
    eprintln!(
        "tycho2: {} records written (plx>0), {} without plx, {} without rv (0 honored), {} B → {}",
        written,
        no_plx,
        no_rv,
        buf.len(),
        out_path
    );
    if ci_mode && !upload_asset(&out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}
