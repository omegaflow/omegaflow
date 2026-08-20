use omegaflow::cdn::upload_asset;
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use omegaflow::rpw::{parse_bin, write_bin, COMP_EY, COMP_EZ};
use std::process::Command;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str = "https://amda.irap.omp.eu/service/hapi";
const DATASET: &str = "solo-rpw-efield10s";
const FILL: f64 = -1.0e31;
const WINDOW_DAYS: i64 = 10;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
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

fn date_of(days: i64) -> String {
    let (y, m, d) = civil_from_days(days);
    format!("{y}-{m:02}-{d:02}")
}

fn parse_iso(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 19 {
        return None;
    }
    let year: i64 = s.get(0..4)?.parse().ok()?;
    let month: i64 = s.get(5..7)?.parse().ok()?;
    let day: i64 = s.get(8..10)?.parse().ok()?;
    let h: i64 = s.get(11..13)?.parse().ok()?;
    let mi: i64 = s.get(14..16)?.parse().ok()?;
    let sec: i64 = s.get(17..19)?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0 + sec as f64)
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

fn harvest_window(
    start_day: i64,
    end_day: i64,
    decimate_s: f64,
    records: &Mutex<Vec<(f64, f64, u32)>>,
    day_count: &Mutex<usize>,
) {
    let url = format!(
        "{}/data?id={}&time.min={}T00:00:00Z&time.max={}T23:59:59Z&format=csv",
        BASE,
        DATASET,
        date_of(start_day),
        date_of(end_day)
    );
    let Some(text) = fetch(&url) else {
        eprintln!(
            "window {}-{}: fetch void — the window stays unharvested",
            date_of(start_day),
            date_of(end_day)
        );
        return;
    };
    let mut buckets: std::collections::HashMap<u64, (Vec<f64>, Vec<f64>)> =
        std::collections::HashMap::new();
    let mut rows = 0usize;
    let mut fills = 0usize;
    for line in text.lines() {
        if line.is_empty() || !line.as_bytes()[0].is_ascii_digit() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 4 {
            continue;
        }
        let Some(t) = parse_iso(parts[0]) else {
            continue;
        };
        rows += 1;
        let bucket = (t / decimate_s).floor() as u64;
        let entry = buckets.entry(bucket).or_default();
        let mut taken = 0usize;
        if let Ok(ey) = parts[2].parse::<f64>() {
            if ey.is_finite() && ey != FILL {
                entry.0.push(ey);
                taken += 1;
            }
        }
        if let Ok(ez) = parts[3].parse::<f64>() {
            if ez.is_finite() && ez != FILL {
                entry.1.push(ez);
                taken += 1;
            }
        }
        if taken == 0 {
            fills += 1;
        }
    }
    let mut window_records: Vec<(f64, f64, u32)> = Vec::new();
    for (bucket, (ey, ez)) in &mut buckets {
        let t = *bucket as f64 * decimate_s + decimate_s * 0.5;
        if !ey.is_empty() {
            window_records.push((t, median(ey), COMP_EY));
        }
        if !ez.is_empty() {
            window_records.push((t, median(ez), COMP_EZ));
        }
    }
    window_records.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut day_count_guard = day_count.lock().unwrap_or_else(|e| e.into_inner());
    *day_count_guard += (end_day - start_day + 1) as usize;
    drop(day_count_guard);
    let mut guard = records.lock().unwrap_or_else(|e| e.into_inner());
    guard.extend(window_records);
    eprintln!(
        "window {}-{}: {} rows, {} fill-skipped, {} buckets",
        date_of(start_day),
        date_of(end_day),
        rows,
        fills,
        guard.len()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "rpw_efield.bin".to_string());
    let decimate_min: f64 = arg_value(&args, "--decimate-min")
        .and_then(|v| v.parse().ok())
        .unwrap_or(10.0);
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(4);
    let start_day = match arg_value(&args, "--window-start")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => match parse_days("2020-06-15") {
            Some(d) => d,
            None => {
                eprintln!("--window-start undeclared and the dataset start parses void");
                std::process::exit(1);
            }
        },
    };
    let end_day = match arg_value(&args, "--window-end")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => match parse_days("2022-12-01") {
            Some(d) => d,
            None => {
                eprintln!("--window-end undeclared and the dataset end parses void");
                std::process::exit(1);
            }
        },
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
    let records: Arc<Mutex<Vec<(f64, f64, u32)>>> = Arc::new(Mutex::new(Vec::new()));
    let day_count: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let next = Arc::new(AtomicI64::new(start_day));
    let mut workers = Vec::new();
    for _ in 0..jobs {
        let records = Arc::clone(&records);
        let day_count = Arc::clone(&day_count);
        let next = Arc::clone(&next);
        workers.push(std::thread::spawn(move || loop {
            let w_start = next.fetch_add(WINDOW_DAYS, Ordering::SeqCst);
            if w_start > end_day {
                break;
            }
            let w_end = (w_start + WINDOW_DAYS - 1).min(end_day);
            harvest_window(w_start, w_end, decimate_min * 60.0, &records, &day_count);
        }));
    }
    for w in workers {
        let _ = w.join();
    }
    let records_guard = Arc::try_unwrap(records)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let mut raw = records_guard;
    raw.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut records_tdb: Vec<(f64, f64, u32)> = Vec::with_capacity(raw.len());
    for (t, v, c) in raw {
        match lsk.unix_to_tdb(t) {
            Some(tdb) => records_tdb.push((tdb, v, c)),
            None => continue,
        }
    }
    let days_covered = Arc::try_unwrap(day_count)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    eprintln!(
        "{}: {} days, {} records, {} medians/min window ({} B)",
        DATASET,
        days_covered,
        records_tdb.len(),
        decimate_min,
        records_tdb.len() * 20 + 8
    );
    if records_tdb.is_empty() {
        eprintln!(
            "{}: no records — the bin stays unwritten (0 honored)",
            DATASET
        );
        std::process::exit(1);
    }
    if args.iter().any(|a| a == "--merge") {
        match std::fs::read(&out) {
            Ok(existing) => match parse_bin(&existing) {
                Some(mut old) => {
                    let main_len = records_tdb.len();
                    let old_len = old.len();
                    let mut merged = records_tdb;
                    merged.append(&mut old);
                    merged.sort_by(|a, b| a.0.total_cmp(&b.0));
                    merged.dedup_by(|a, b| a.0 == b.0 && a.2 == b.2);
                    eprintln!(
                        "{}: {} harvested + {} existing = {} merged ({} dups dropped)",
                        out,
                        main_len,
                        old_len,
                        merged.len(),
                        main_len + old_len - merged.len()
                    );
                    records_tdb = merged;
                }
                None => {
                    eprintln!(
                        "{}: existing bin parses void — the bin stays unwritten",
                        out
                    );
                    std::process::exit(1);
                }
            },
            Err(_) => {
                eprintln!("{}: no existing bin — the harvest stands alone", out);
            }
        }
    }
    let bytes = write_bin(&records_tdb);
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
