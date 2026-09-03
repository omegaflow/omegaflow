use omegaflow::archivar::goes::{parse_bin, write_bin, COMP_XRSA, COMP_XRSB};
use omegaflow::cdn::upload_asset;
use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use std::collections::HashMap;
use std::process::Command;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

const BASE_R: &str =
    "https://www.ncei.noaa.gov/data/goes-r-series-l2-operational-space-weather-products/access";
const SATS_R: [u16; 4] = [16, 17, 18, 19];
const EPOCH_UNIX: f64 = 946728000.0;
const FILL: f64 = -9999.0;
const VALID_MIN: f64 = 1e-9;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_days(s: &str) -> Option<i64> {
    let mut it = s.split('-');
    let y: i64 = it.next()?.parse().ok()?;
    let m: i64 = it.next()?.parse().ok()?;
    let d: i64 = it.next()?.parse().ok()?;
    days_from_civil(y, m, d)
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn parse_nc(bytes: &[u8], buckets: &Mutex<HashMap<(u32, u64), Vec<f64>>>) -> (usize, usize) {
    let Ok(file) = Hdf5File::parse(bytes) else {
        return (0, 0);
    };
    let Ok(time_raw) = file.read_dataset("time") else {
        return (0, 0);
    };
    let Ok(flux_a) = file.read_dataset("xrsa_flux") else {
        return (0, 0);
    };
    let Ok(flux_b) = file.read_dataset("xrsb_flux") else {
        return (0, 0);
    };
    let Ok(flag_a) = file.read_dataset("xrsa_flag") else {
        return (0, 0);
    };
    let Ok(flag_b) = file.read_dataset("xrsb_flag") else {
        return (0, 0);
    };
    let n = time_raw.len() / 8;
    if flux_a.len() != n * 4 || flux_b.len() != n * 4 || flag_a.len() != n || flag_b.len() != n {
        return (0, 0);
    }
    let mut kept = 0usize;
    let mut guard = buckets.lock().unwrap_or_else(|e| e.into_inner());
    for i in 0..n {
        let Some(t) = decode_f64(&time_raw, i * 8, Endian::Le) else {
            continue;
        };
        if t == FILL || !t.is_finite() {
            continue;
        }
        let t_unix = t + EPOCH_UNIX;
        let bucket = (t_unix / 60.0).floor() as u64;
        if flag_a[i] == 0 {
            let va = decode_f32(&flux_a, i * 4, Endian::Le).unwrap_or(f32::NAN) as f64;
            if va.is_finite() && va != FILL && va >= VALID_MIN {
                guard.entry((COMP_XRSA, bucket)).or_default().push(va);
                kept += 1;
            }
        }
        if flag_b[i] == 0 {
            let vb = decode_f32(&flux_b, i * 4, Endian::Le).unwrap_or(f32::NAN) as f64;
            if vb.is_finite() && vb != FILL && vb >= VALID_MIN {
                guard.entry((COMP_XRSB, bucket)).or_default().push(vb);
                kept += 1;
            }
        }
    }
    (n, kept)
}

fn median(vals: &mut Vec<f64>) -> f64 {
    if vals.is_empty() {
        return f64::NAN;
    }
    vals.sort_by(|a, b| a.total_cmp(b));
    let mid = vals.len() / 2;
    if vals.len() % 2 == 0 {
        (vals[mid - 1] + vals[mid]) / 2.0
    } else {
        vals[mid]
    }
}

fn harvest_day(
    sat: u16,
    y: i64,
    m: i64,
    d: i64,
    cache_dir: &str,
    buckets: &Arc<Mutex<HashMap<(u32, u64), Vec<f64>>>>,
) -> (usize, usize) {
    let day_url = format!("{BASE_R}/goes{sat}/exis/{y:04}/{m:02}/{d:02}/");
    let Some(html) = fetch(&day_url) else {
        eprintln!("goes{sat} {y:04}-{m:02}-{d:02}: day listing void");
        return (0, 0);
    };
    let text = String::from_utf8_lossy(&html);
    let mut rows = 0usize;
    let mut kept = 0usize;
    let mut found = 0usize;
    for hour in 0..24 {
        let prefix = format!("ops_xrsf-l2-avg1m_g{sat}_{y:04}{m:02}{d:02}T{hour:02}Z_");
        let Some(fname) = text
            .split('"')
            .find(|s| s.starts_with(&prefix) && s.ends_with(".tar.gz"))
            .map(|s| s.to_string())
        else {
            continue;
        };
        found += 1;
        let cache_path = format!("{cache_dir}/{fname}");
        if std::fs::metadata(&cache_path).is_err() {
            let Some(b) = fetch(&format!("{day_url}{fname}")) else {
                continue;
            };
            let _ = std::fs::write(&cache_path, &b);
        }
        let tmp = format!("{cache_dir}/x_{fname}");
        let _ = std::fs::create_dir_all(&tmp);
        let _ = Command::new("tar")
            .arg("xzf")
            .arg(&cache_path)
            .arg("-C")
            .arg(&tmp)
            .output();
        if let Ok(entries) = std::fs::read_dir(&tmp) {
            for e in entries.flatten() {
                let p = e.path();
                if p.extension().and_then(|s| s.to_str()) != Some("nc") {
                    continue;
                }
                if let Ok(nc) = std::fs::read(&p) {
                    let (n, k) = parse_nc(&nc, buckets);
                    rows += n;
                    kept += k;
                }
            }
        }
        let _ = std::fs::remove_dir_all(&tmp);
    }
    if found == 0 {
        eprintln!("goes{sat} {y:04}-{m:02}-{d:02}: no xrsf-tar.gz (absent)");
    }
    (rows, kept)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "goes_r_xrs.bin".to_string());
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(4);
    let cache_dir = arg_value(&args, "--cache-dir").unwrap_or_else(|| {
        omegaflow::archivar::cache_root()
            .join("omegaflow_goes_r_cache")
            .to_string_lossy()
            .into_owned()
    });
    if std::fs::create_dir_all(&cache_dir).is_err() {
        eprintln!("{cache_dir}: cache dir stays uncreatable");
        std::process::exit(1);
    }
    let start_day = match arg_value(&args, "--window-start")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => parse_days("2017-01-01").unwrap(),
    };
    let end_day = match arg_value(&args, "--window-end")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => parse_days("2025-12-31").unwrap(),
    };
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void");
            std::process::exit(1);
        }
    };
    let Some(lsk) = parse_lsk(&lsk_text) else {
        eprintln!("--lsk parses void");
        std::process::exit(1);
    };
    let buckets: Arc<Mutex<HashMap<(u32, u64), Vec<f64>>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut units: Vec<(u16, i64, i64, i64)> = Vec::new();
    let mut day = start_day;
    while day <= end_day {
        let (y, m, d) = civil_from_days(day);
        for sat in SATS_R {
            units.push((sat, y, m, d));
        }
        day += 1;
    }
    let units = Arc::new(units);
    let next = Arc::new(AtomicI64::new(0));
    let total_rows = Arc::new(AtomicI64::new(0));
    let total_kept = Arc::new(AtomicI64::new(0));
    let mut workers = Vec::new();
    for _ in 0..jobs {
        let units = Arc::clone(&units);
        let buckets = Arc::clone(&buckets);
        let next = Arc::clone(&next);
        let total_rows = Arc::clone(&total_rows);
        let total_kept = Arc::clone(&total_kept);
        let cache_dir = cache_dir.clone();
        workers.push(std::thread::spawn(move || loop {
            let idx = next.fetch_add(1, Ordering::SeqCst) as usize;
            let Some((sat, y, m, d)) = units.get(idx).copied() else {
                break;
            };
            let (rows, kept) = harvest_day(sat, y, m, d, &cache_dir, &buckets);
            total_rows.fetch_add(rows as i64, Ordering::SeqCst);
            total_kept.fetch_add(kept as i64, Ordering::SeqCst);
        }));
    }
    for w in workers {
        let _ = w.join();
    }
    eprintln!(
        "harvest: {} units (sat×day), {} rows, {} kept",
        units.len(),
        total_rows.load(Ordering::SeqCst),
        total_kept.load(Ordering::SeqCst)
    );
    let buckets_guard = Arc::try_unwrap(buckets)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let mut raw: Vec<(f64, f64, u32)> = Vec::new();
    for ((comp, bucket), mut vals) in buckets_guard {
        if vals.is_empty() {
            continue;
        }
        let mid_unix = (bucket as f64 + 0.5) * 60.0;
        let Some(tdb) = lsk.unix_to_tdb(mid_unix) else {
            continue;
        };
        raw.push((tdb, median(&mut vals), comp));
    }
    raw.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    if raw.is_empty() {
        eprintln!("{out}: no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    eprintln!(
        "{out}: {} bucket medians (1 min buckets), {} B",
        raw.len(),
        raw.len() * 20 + 8
    );
    let bytes = write_bin(&raw);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => eprintln!("{out}: {} records, roundtrip parses", parsed.len()),
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
