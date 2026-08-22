use omegaflow::archivar::omni2::{
    parse_bin, write_bin, COMP_BX, COMP_BY, COMP_BZ, COMP_N1800, COMP_PRESSURE, COMP_T1800,
    COMP_V1800,
};
use omegaflow::cdn::upload_asset;
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use std::process::Command;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str = "https://cdaweb.gsfc.nasa.gov/hapi";
const DATASET: &str = "OMNI2_H0_MRG1HR";
const PARAMS: &str = "BX_GSE1800,BY_GSM1800,BZ_GSM1800,T1800,N1800,V1800,Pressure1800";
const FILL_B: f64 = 999.9;
const FILL_T: f64 = 9_999_999.0;
const FILL_V: f64 = 9999.0;
const FILL_P: f64 = 99.99;
const RANGE_B: f64 = 1000.0;
const RANGE_T: f64 = 1.0e8;
const RANGE_N: f64 = 1000.0;
const RANGE_V: f64 = 5000.0;
const RANGE_P: f64 = 1000.0;

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
        .arg("300")
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

fn keep_b(v: f64) -> bool {
    v.is_finite() && v != FILL_B && v.abs() <= RANGE_B
}

fn keep_positive(v: f64, fill: f64, range: f64) -> bool {
    v.is_finite() && v != fill && v > 0.0 && v <= range
}

fn harvest_window(
    start_day: i64,
    end_day: i64,
    decimate_s: f64,
    buckets: &Mutex<std::collections::HashMap<(u32, i64), Vec<f64>>>,
    year_rows: &Mutex<usize>,
    year_fills: &Mutex<usize>,
) {
    let url = format!(
        "{}/data?id={}&time.min={}T00:00:00Z&time.max={}T23:59:59Z&parameters={}&format=csv",
        BASE,
        DATASET,
        date_of(start_day),
        date_of(end_day),
        PARAMS
    );
    let Some(text) = fetch(&url) else {
        eprintln!(
            "window {}-{}: fetch void — the year stays unharvested",
            date_of(start_day),
            date_of(end_day)
        );
        return;
    };
    let mut rows = 0usize;
    let mut fills = 0usize;
    let mut guard = buckets.lock().unwrap_or_else(|e| e.into_inner());
    for line in text.lines() {
        if line.is_empty() || !line.as_bytes()[0].is_ascii_digit() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 8 {
            continue;
        }
        let Some(t) = parse_iso(parts[0]) else {
            continue;
        };
        rows += 1;
        let bucket = (t / decimate_s).floor() as i64;
        let mut taken = 0usize;
        let keepers: [(usize, u32, fn(f64) -> bool); 7] = [
            (1usize, COMP_BX, keep_b),
            (2usize, COMP_BY, keep_b),
            (3usize, COMP_BZ, keep_b),
            (4usize, COMP_T1800, |v| keep_positive(v, FILL_T, RANGE_T)),
            (5usize, COMP_N1800, |v| keep_positive(v, FILL_B, RANGE_N)),
            (6usize, COMP_V1800, |v| keep_positive(v, FILL_V, RANGE_V)),
            (7usize, COMP_PRESSURE, |v| keep_positive(v, FILL_P, RANGE_P)),
        ];
        for (col, comp, keep) in keepers {
            if let Ok(v) = parts[col].parse::<f64>() {
                if keep(v) {
                    guard.entry((comp, bucket)).or_default().push(v);
                    taken += 1;
                }
            }
        }
        if taken == 0 {
            fills += 1;
        }
    }
    drop(guard);
    let mut yr = year_rows.lock().unwrap_or_else(|e| e.into_inner());
    *yr += rows;
    drop(yr);
    let mut yf = year_fills.lock().unwrap_or_else(|e| e.into_inner());
    *yf += fills;
    drop(yf);
    eprintln!(
        "window {}-{}: {} rows, {} fill-skipped",
        date_of(start_day),
        date_of(end_day),
        rows,
        fills
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "omni2_serie.bin".to_string());
    let decimate_min: f64 = arg_value(&args, "--decimate-min")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1440.0);
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
        .unwrap_or(8);
    let start_day = match arg_value(&args, "--window-start")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => parse_days("1963-01-01").unwrap_or_else(|| {
            eprintln!("--window-start undeclared and the dataset start parses void");
            std::process::exit(1);
        }),
    };
    let end_day = match arg_value(&args, "--window-end")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => parse_days("2026-08-06").unwrap_or_else(|| {
            eprintln!("--window-end undeclared and the dataset stop parses void");
            std::process::exit(1);
        }),
    };
    let (sy, _, _) = civil_from_days(start_day);
    let (ey, _, _) = civil_from_days(end_day);
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
    let buckets: Arc<Mutex<std::collections::HashMap<(u32, i64), Vec<f64>>>> =
        Arc::new(Mutex::new(std::collections::HashMap::new()));
    let year_rows: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let year_fills: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let next = Arc::new(AtomicI64::new(sy));
    let mut workers = Vec::new();
    for _ in 0..jobs {
        let buckets = Arc::clone(&buckets);
        let year_rows = Arc::clone(&year_rows);
        let year_fills = Arc::clone(&year_fills);
        let next = Arc::clone(&next);
        workers.push(std::thread::spawn(move || loop {
            let year = next.fetch_add(1, Ordering::SeqCst);
            if year > ey {
                break;
            }
            let w_start = match days_from_civil(year, 1, 1) {
                Some(d) => d,
                None => continue,
            };
            let w_end_raw = match days_from_civil(year, 12, 31) {
                Some(d) => d,
                None => continue,
            };
            let w_end = w_end_raw.min(end_day);
            if w_start > end_day {
                continue;
            }
            harvest_window(
                w_start,
                w_end,
                decimate_s,
                &buckets,
                &year_rows,
                &year_fills,
            );
        }));
    }
    for w in workers {
        let _ = w.join();
    }
    let rows = Arc::try_unwrap(year_rows)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let fills = Arc::try_unwrap(year_fills)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let buckets_guard = Arc::try_unwrap(buckets)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let mut raw: Vec<(f64, f64, u32)> = Vec::new();
    let mut pre_lsk_skip = 0usize;
    for ((comp, bucket), mut vals) in buckets_guard {
        if vals.is_empty() {
            continue;
        }
        let t_unix = (bucket as f64 + 0.5) * decimate_s;
        let Some(t) = lsk.unix_to_tdb(t_unix) else {
            pre_lsk_skip += 1;
            continue;
        };
        raw.push((t, median(&mut vals), comp));
    }
    raw.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    if raw.is_empty() {
        eprintln!(
            "{}: no records — the bin stays unwritten (0 honored)",
            DATASET
        );
        std::process::exit(1);
    }
    eprintln!(
        "{}: {} years, {} rows, {} fill-skipped rows, {} records ({} min buckets), {} buckets pre-1972 stay unharvested (leap table void, 0 honored) — epoch TDB via LSK",
        DATASET,
        ey - sy + 1,
        rows,
        fills,
        raw.len(),
        decimate_min,
        pre_lsk_skip
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
