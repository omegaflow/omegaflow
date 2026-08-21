use omegaflow::archivar::goes::{parse_bin, write_bin, COMP_XRSA, COMP_XRSB};
use omegaflow::cdn::upload_asset;
use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use std::process::Command;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str =
    "https://www.ncei.noaa.gov/data/goes-space-environment-monitor/access/science/xrs";
const SATS: [&str; 6] = ["goes08", "goes10", "goes12", "goes13", "goes14", "goes15"];
const EPOCH_UNIX: f64 = 946728000.0;
const FILL: f64 = -9999.0;
const VALID_MIN: f64 = 1e-9;
const INDEX_MARKER: &str = "sci_xrsf-l2-avg1m_";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn parse_days(s: &str) -> Option<i64> {
    let (y, rest) = s.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    days_from_civil(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?)
}

fn median(vals: &mut Vec<f64>) -> f64 {
    vals.sort_by(|a, b| a.total_cmp(b));
    let n = vals.len();
    if n % 2 == 0 {
        (vals[n / 2 - 1] + vals[n / 2]) * 0.5
    } else {
        vals[n / 2]
    }
}

fn index_files(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(pos) = rest.find(INDEX_MARKER) {
        let tail = &rest[pos..];
        let name: String = tail
            .chars()
            .take_while(|&c| c != '"' && c != '\'' && c != '<')
            .collect();
        if name.ends_with(".nc") && !out.contains(&name) {
            out.push(name);
        }
        rest = &tail[1..];
    }
    out
}

fn filename_day(name: &str) -> Option<i64> {
    let pos = name.find("_d")?;
    if pos + 10 > name.len() {
        return None;
    }
    let digits = &name[pos + 2..pos + 10];
    if digits.len() != 8 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let y: i64 = digits[0..4].parse().ok()?;
    let m: i64 = digits[4..6].parse().ok()?;
    let d: i64 = digits[6..8].parse().ok()?;
    days_from_civil(y, m, d)
}

fn parse_nc(
    path: &str,
    decimate_s: f64,
    start_day: i64,
    end_day: i64,
    buckets: &Mutex<std::collections::HashMap<(u32, u64), Vec<f64>>>,
) -> Option<(usize, usize, usize)> {
    let bytes = std::fs::read(path).ok()?;
    let file = Hdf5File::parse(&bytes).ok()?;
    let time_raw = file.read_dataset("time").ok()?;
    let flux_a = file.read_dataset("xrsa_flux").ok()?;
    let flux_b = file.read_dataset("xrsb_flux").ok()?;
    let flag_a = file
        .read_dataset("xrsa_flag")
        .or_else(|_| file.read_dataset("xrsa_flags"))
        .ok()?;
    let flag_b = file
        .read_dataset("xrsb_flag")
        .or_else(|_| file.read_dataset("xrsb_flags"))
        .ok()?;
    let n = time_raw.len() / 8;
    if flux_a.len() != n * 4
        || flux_b.len() != n * 4
        || flag_a.len() != n * 2
        || flag_b.len() != n * 2
    {
        eprintln!("{}: dataset shapes carry no common row count", path);
        return None;
    }
    let mut kept_a = 0usize;
    let mut kept_b = 0usize;
    let mut guard = buckets.lock().unwrap_or_else(|e| e.into_inner());
    for i in 0..n {
        let t = match decode_f64(&time_raw, i * 8, Endian::Le) {
            Some(v) => v,
            None => continue,
        };
        if t == FILL || !t.is_finite() {
            continue;
        }
        let day_1970 = (t / 86400.0).floor() as i64;
        let day_2000 = ((t + EPOCH_UNIX) / 86400.0).floor() as i64;
        let file_day = std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(filename_day);
        let t_unix = match file_day {
            Some(fd) if (day_1970 - fd).abs() <= (day_2000 - fd).abs() => t,
            Some(_) => t + EPOCH_UNIX,
            None => t + EPOCH_UNIX,
        };
        let day = (t_unix / 86400.0).floor() as i64;
        if day < start_day || day > end_day {
            continue;
        }
        let bucket = (t_unix / decimate_s).floor() as u64;
        let fa = u16::from_le_bytes([flag_a[i * 2], flag_a[i * 2 + 1]]);
        if fa == 0 {
            let va = decode_f32(&flux_a, i * 4, Endian::Le).unwrap_or(f32::NAN) as f64;
            if va.is_finite() && va != FILL && va >= VALID_MIN {
                guard.entry((COMP_XRSA, bucket)).or_default().push(va);
                kept_a += 1;
            }
        }
        let fb = u16::from_le_bytes([flag_b[i * 2], flag_b[i * 2 + 1]]);
        if fb == 0 {
            let vb = decode_f32(&flux_b, i * 4, Endian::Le).unwrap_or(f32::NAN) as f64;
            if vb.is_finite() && vb != FILL && vb >= VALID_MIN {
                guard.entry((COMP_XRSB, bucket)).or_default().push(vb);
                kept_b += 1;
            }
        }
    }
    Some((n, kept_a, kept_b))
}

fn harvest_unit(
    sat: &str,
    year: i64,
    month: i64,
    cache_dir: &str,
    start_day: i64,
    end_day: i64,
    decimate_s: f64,
    buckets: &Arc<Mutex<std::collections::HashMap<(u32, u64), Vec<f64>>>>,
) -> (usize, usize) {
    let index_url = format!("{BASE}/{sat}/xrsf-l2-avg1m_science/{year}/{month:02}/");
    let Some(html) = fetch(&index_url) else {
        eprintln!(
            "index {} {}-{:02}: fetch void — the month stays unharvested",
            sat, year, month
        );
        return (0, 1);
    };
    let files = index_files(&String::from_utf8_lossy(&html));
    let mut rows = 0usize;
    let mut voids = 0usize;
    for name in files {
        let Some(day) = filename_day(&name) else {
            continue;
        };
        if day < start_day || day > end_day {
            continue;
        }
        let cache_path = format!("{}/{}_{}", cache_dir, sat, name);
        if std::fs::metadata(&cache_path).is_err() {
            let url = format!("{BASE}/{sat}/xrsf-l2-avg1m_science/{year}/{month:02}/{name}");
            let Some(bytes) = fetch(&url) else {
                eprintln!("file {}: fetch void — the day stays unharvested", name);
                voids += 1;
                continue;
            };
            if std::fs::write(&cache_path, &bytes).is_err() {
                eprintln!("file {}: cache write void", name);
                voids += 1;
                continue;
            }
        }
        match parse_nc(&cache_path, decimate_s, start_day, end_day, buckets) {
            Some((n, ka, kb)) => {
                rows += n;
                eprintln!("{}: {} rows, xrsa {} / xrsb {} kept", name, n, ka, kb);
            }
            None => {
                eprintln!("{}: parses void — the day stays unharvested", name);
                voids += 1;
            }
        }
    }
    (rows, voids)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "goes_xrs.bin".to_string());
    let decimate_min: f64 = arg_value(&args, "--decimate-min")
        .and_then(|v| v.parse().ok())
        .unwrap_or(60.0);
    let decimate_s = decimate_min * 60.0;
    if !(decimate_s > 0.0) || !decimate_s.is_finite() {
        eprintln!(
            "--decimate-min {} carries no positive bucket width",
            decimate_min
        );
        std::process::exit(1);
    }
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(4);
    let cache_dir = arg_value(&args, "--cache-dir")
        .unwrap_or_else(|| "/tmp/omegaflow_goes_xrs_cache".to_string());
    if std::fs::create_dir_all(&cache_dir).is_err() {
        eprintln!("{}: cache dir stays uncreatable", cache_dir);
        std::process::exit(1);
    }
    let start_day = match arg_value(&args, "--window-start")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => parse_days("1995-01-01").unwrap_or_else(|| {
            eprintln!("--window-start undeclared and the dataset start parses void");
            std::process::exit(1);
        }),
    };
    let end_day = match arg_value(&args, "--window-end")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => parse_days("2020-12-31").unwrap_or_else(|| {
            eprintln!("--window-end undeclared and the dataset end parses void");
            std::process::exit(1);
        }),
    };
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(&lsk_text) {
        Some(l) => l,
        None => {
            eprintln!("--lsk parses void — the leap-second table stays unread");
            std::process::exit(1);
        }
    };
    let buckets: Arc<Mutex<std::collections::HashMap<(u32, u64), Vec<f64>>>> =
        Arc::new(Mutex::new(std::collections::HashMap::new()));
    if let Some(path) = arg_value(&args, "--file") {
        match parse_nc(&path, decimate_s, start_day, end_day, &buckets) {
            Some((n, ka, kb)) => eprintln!("{}: {} rows, xrsa {} / xrsb {} kept", path, n, ka, kb),
            None => {
                eprintln!("{}: parses void — no records", path);
                std::process::exit(1);
            }
        }
    } else {
        let (sy, sm, _) = civil_from_days(start_day);
        let (ey, em, _) = civil_from_days(end_day);
        let mut units: Vec<(String, i64, i64)> = Vec::new();
        let mut y = sy;
        let mut m = sm;
        loop {
            if y > ey || (y == ey && m > em) {
                break;
            }
            for sat in SATS {
                units.push((sat.to_string(), y, m));
            }
            m += 1;
            if m == 13 {
                m = 1;
                y += 1;
            }
        }
        let units = Arc::new(units);
        let next = Arc::new(AtomicI64::new(0));
        let total_rows = Arc::new(AtomicI64::new(0));
        let total_voids = Arc::new(AtomicI64::new(0));
        let mut workers = Vec::new();
        for _ in 0..jobs {
            let units = Arc::clone(&units);
            let buckets = Arc::clone(&buckets);
            let next = Arc::clone(&next);
            let total_rows = Arc::clone(&total_rows);
            let total_voids = Arc::clone(&total_voids);
            let cache_dir = cache_dir.clone();
            workers.push(std::thread::spawn(move || loop {
                let idx = next.fetch_add(1, Ordering::SeqCst) as usize;
                let Some((sat, y, m)) = units.get(idx).cloned() else {
                    break;
                };
                let (rows, voids) = harvest_unit(
                    &sat, y, m, &cache_dir, start_day, end_day, decimate_s, &buckets,
                );
                total_rows.fetch_add(rows as i64, Ordering::SeqCst);
                total_voids.fetch_add(voids as i64, Ordering::SeqCst);
            }));
        }
        for w in workers {
            let _ = w.join();
        }
        eprintln!(
            "harvest: {} units, {} rows, {} void days",
            units.len(),
            total_rows.load(Ordering::SeqCst),
            total_voids.load(Ordering::SeqCst)
        );
    }
    let buckets_guard = Arc::try_unwrap(buckets)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let mut raw: Vec<(f64, f64, u32)> = Vec::new();
    for ((comp, bucket), mut vals) in buckets_guard {
        if vals.is_empty() {
            continue;
        }
        let mid_unix = (bucket as f64 + 0.5) * decimate_s;
        let Some(tdb) = lsk.unix_to_tdb(mid_unix) else {
            continue;
        };
        raw.push((tdb, median(&mut vals), comp));
    }
    raw.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    if raw.is_empty() {
        eprintln!("{}: no records — the bin stays unwritten (0 honored)", out);
        std::process::exit(1);
    }
    eprintln!(
        "{}: {} bucket medians ({} min buckets), {} B",
        out,
        raw.len(),
        decimate_min,
        raw.len() * 20 + 8
    );
    let bytes = write_bin(&raw);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!("{}: {} records, roundtrip parses", out, parsed.len());
        }
        None => {
            eprintln!("{}: roundtrip parse void — the bin stays unverified", out);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
