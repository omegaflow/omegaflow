use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str = "https://earthquake.usgs.gov/fdsnws/event/1/query";
const MIN_MAG: f64 = 4.5;
const PAGE_LIMIT: usize = 20_000;
const MAX_PAGES: usize = 8;
const SECONDS_PER_DAY: f64 = 86_400.0;

const MAGIC: [u8; 4] = *b"USC1";
const COMP_RATE: u32 = 1;
const COMP_MAX: u32 = 1;

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
    format!("{y:04}-{m:02}-{d:02}")
}

fn parse_days(s: &str) -> Option<i64> {
    let (y, rest) = s.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    days_from_civil(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?)
}

fn month_windows(start: i64, end: i64) -> Option<Vec<(i64, i64)>> {
    if start >= end {
        return None;
    }
    let (sy, sm, _) = civil_from_days(start);
    let mut out = Vec::new();
    let (mut y, mut m) = (sy, sm);
    loop {
        let first = days_from_civil(y, m, 1)?;
        if first >= end {
            break;
        }
        let (ny, nm) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
        let next = days_from_civil(ny, nm, 1)?;
        out.push((first.max(start), next));
        y = ny;
        m = nm;
    }
    Some(out)
}

fn count_month(start_day: i64, end_day: i64) -> Option<u64> {
    let mut offset = 1usize;
    let mut kept = 0u64;
    let mut pages = 0usize;
    loop {
        let url = format!(
            "{BASE}?format=csv&starttime={}&endtime={}&minmagnitude={}&orderby=time-asc&limit={}&offset={}",
            date_of(start_day),
            date_of(end_day),
            MIN_MAG,
            PAGE_LIMIT,
            offset
        );
        let Some(text) = fetch(&url) else {
            eprintln!(
                "month {}-{}: page offset {} fetch void — the month stays absent",
                date_of(start_day),
                date_of(end_day),
                offset
            );
            return None;
        };
        let mut page_rows = 0usize;
        for line in text.lines() {
            let b = line.as_bytes();
            if b.is_empty() || !b[0].is_ascii_digit() {
                continue;
            }
            page_rows += 1;
            let parts: Vec<&str> = line.split(',').collect();
            let mag = parts.get(4).and_then(|s| s.parse::<f64>().ok());
            if let Some(m) = mag {
                if m.is_finite() && m >= MIN_MAG {
                    kept += 1;
                }
            }
        }
        pages += 1;
        if page_rows < PAGE_LIMIT {
            break;
        }
        if pages >= MAX_PAGES {
            eprintln!(
                "month {}-{}: page count reaches {} at limit {} — the month stays absent",
                date_of(start_day),
                date_of(end_day),
                pages,
                PAGE_LIMIT
            );
            return None;
        }
        offset += PAGE_LIMIT;
    }
    Some(kept)
}

fn harvest_window(start_day: i64, end_day: i64, results: &Mutex<Vec<(i64, i64, u64)>>) {
    match count_month(start_day, end_day) {
        Some(count) if count > 0 => {
            let mut guard = match results.lock() {
                Ok(g) => g,
                Err(poisoned) => poisoned.into_inner(),
            };
            guard.push((start_day, end_day, count));
        }
        Some(_) => {
            eprintln!(
                "month {}-{}: 0 events — absent (0 honored)",
                date_of(start_day),
                date_of(end_day)
            );
        }
        None => {}
    }
}

fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, val, comp) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&comp.to_le_bytes());
    }
    buf
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / 20 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let val = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let comp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if !(COMP_RATE..=COMP_MAX).contains(&comp) {
            return None;
        }
        out.push((t, val, comp));
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => omegaflow::archivar::cache_root()
            .join("usgs_comcat_m45.bin")
            .to_string_lossy()
            .into_owned(),
    };
    let start_day = match arg_value(&args, "--window-start")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => match parse_days("1973-01-01") {
            Some(d) => d,
            None => {
                eprintln!("--window-start undeclared and the catalog start 1973-01-01 parses void");
                std::process::exit(1);
            }
        },
    };
    let end_day = match arg_value(&args, "--window-end")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => match parse_days("2026-08-01") {
            Some(d) => d,
            None => {
                eprintln!("--window-end undeclared and the default parses void");
                std::process::exit(1);
            }
        },
    };
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(8);
    if jobs == 0 {
        eprintln!("--jobs 0 carries no worker");
        std::process::exit(1);
    }
    let windows = match month_windows(start_day, end_day) {
        Some(w) if !w.is_empty() => w,
        _ => {
            eprintln!(
                "window {}..{} carries no month — the bin stays unwritten",
                date_of(start_day),
                date_of(end_day)
            );
            std::process::exit(1);
        }
    };
    eprintln!(
        "usgs_comcat: {} month windows {}..{}, min magnitude {}, {} workers",
        windows.len(),
        date_of(start_day),
        date_of(end_day),
        MIN_MAG,
        jobs
    );
    let results: Arc<Mutex<Vec<(i64, i64, u64)>>> = Arc::new(Mutex::new(Vec::new()));
    let windows = Arc::new(windows);
    let next = Arc::new(AtomicUsize::new(0));
    let mut workers = Vec::new();
    for _ in 0..jobs {
        let results = Arc::clone(&results);
        let windows = Arc::clone(&windows);
        let next = Arc::clone(&next);
        workers.push(std::thread::spawn(move || {
            loop {
                let idx = next.fetch_add(1, Ordering::SeqCst);
                let Some(&(ws, we)) = windows.get(idx) else {
                    break;
                };
                harvest_window(ws, we, &results);
            }
        }));
    }
    for w in workers {
        let _ = w.join();
    }
    let Some(harvested) = Arc::try_unwrap(results)
        .ok()
        .and_then(|m| m.into_inner().ok())
    else {
        eprintln!("the month results stay shared — the bin stays unwritten");
        std::process::exit(1);
    };
    let mut raw: Vec<(f64, f64, u32)> = Vec::new();
    for (first_day, next_day, count) in &harvested {
        let mid_days = *first_day as f64 + (*next_day - *first_day) as f64 * 0.5;
        raw.push((mid_days * SECONDS_PER_DAY, *count as f64, COMP_RATE));
    }
    raw.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    if raw.is_empty() {
        eprintln!("usgs_comcat: no month carries events — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    eprintln!(
        "usgs_comcat: {} month records of {} windows, epoch at month midpoint (UTC unix s), rate in events per month",
        raw.len(),
        windows.len()
    );
    let bytes = write_bin(&raw);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
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
    if ci_mode && !upload_release("earthquake.usgs.gov", &out) {
        std::process::exit(1);
    }
}
