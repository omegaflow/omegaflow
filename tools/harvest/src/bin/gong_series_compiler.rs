use omegaflow::cdn::upload_asset;
use omegaflow::gong_series::{parse_bin, write_bin};
use omegaflow::lsk::days_from_civil;
use omegaflow::lzw::uncompress_z;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const TSERIES: &str = "ftp://gong2.nso.edu/TSERIES";

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
        .arg("--retry-delay")
        .arg("3")
        .arg("--retry-all-errors")
        .arg("--max-time")
        .arg("240")
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

fn fetch_with_patience(url: &str) -> Option<Vec<u8>> {
    for attempt in 0..4 {
        if let Some(b) = fetch(url) {
            return Some(b);
        }
        if attempt < 3 {
            std::thread::sleep(std::time::Duration::from_secs(30 * (attempt + 1) as u64));
        }
    }
    None
}

fn fetch_listing(url: &str) -> Option<Vec<String>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-l")
        .arg("--retry")
        .arg("2")
        .arg("--retry-delay")
        .arg("3")
        .arg("--retry-all-errors")
        .arg("--max-time")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        let text = String::from_utf8_lossy(&out.stdout);
        Some(
            text.lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect(),
        )
    } else {
        eprintln!(
            "list {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn is_month(entry: &str) -> bool {
    entry.len() == 6 && entry.bytes().all(|b| b.is_ascii_digit())
}

fn month_dirs() -> Option<Vec<String>> {
    let names = fetch_listing(&format!("{TSERIES}/v1y/"))?;
    let mut months: Vec<String> = names.into_iter().filter(|n| is_month(n)).collect();
    months.sort();
    months.dedup();
    Some(months)
}

fn run_dir(month: &str) -> Option<(String, i64)> {
    let names = fetch_listing(&format!("{TSERIES}/v1y/{month}/"))?;
    let name = names.into_iter().find(|n| n.starts_with("mrv1y"))?;
    let yymmdd = name.strip_prefix("mrv1y")?;
    if yymmdd.len() != 6 || !yymmdd.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let yy: i64 = yymmdd[0..2].parse().ok()?;
    let mm: i64 = yymmdd[2..4].parse().ok()?;
    let dd: i64 = yymmdd[4..6].parse().ok()?;
    let year = if yy >= 50 { 1900 + yy } else { 2000 + yy };
    let days = days_from_civil(year, mm, dd)?;
    Some((name, days))
}

fn compile_run(month: &str, run: &str, days: i64, lmax: u32) -> Vec<(u32, i32, i64, f64)> {
    let url = format!("{TSERIES}/v1y/{month}/{run}/{run}.txt.Z");
    let Some(compressed) = fetch_with_patience(&url) else {
        eprintln!("{month}/{run}: fetch void");
        return Vec::new();
    };
    let Some(text) = uncompress_z(&compressed) else {
        eprintln!("{month}/{run}: decompress void");
        return Vec::new();
    };
    let text = String::from_utf8_lossy(&text);
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 4 {
            continue;
        }
        let Ok(n) = f[0].parse::<i32>() else { continue };
        let Ok(l) = f[1].parse::<u32>() else { continue };
        if l > lmax {
            continue;
        }
        let Ok(nu) = f[2].parse::<f64>() else {
            continue;
        };
        if !nu.is_finite() || nu <= 0.0 {
            continue;
        }
        out.push((l, n, days, nu * 1e-6));
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "gong_series.bin".to_string());
    let lmax: u32 = arg_value(&args, "--lmax")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);

    let months: Vec<String> = if let Some(month) = arg_value(&args, "--month") {
        vec![month]
    } else {
        match month_dirs() {
            Some(m) => m,
            None => {
                eprintln!("v1y index void");
                std::process::exit(1);
            }
        }
    };
    if months.is_empty() {
        eprintln!("no months — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }

    let collected: Arc<Mutex<Vec<(u32, i32, i64, f64)>>> = Arc::new(Mutex::new(Vec::new()));
    let next: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let n_runs = Arc::new(AtomicUsize::new(0));
    let n_months = months.len();
    let mut workers = Vec::new();
    for _ in 0..4 {
        let collected = Arc::clone(&collected);
        let next = Arc::clone(&next);
        let n_runs = Arc::clone(&n_runs);
        let months = months.clone();
        workers.push(std::thread::spawn(move || {
            loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= months.len() {
                    break;
                }
                let month = &months[i];
                match run_dir(month) {
                    Some((run, days)) => {
                        let modes = compile_run(month, &run, days, lmax);
                        if !modes.is_empty() {
                            n_runs.fetch_add(1, Ordering::SeqCst);
                        }
                        if let Ok(mut g) = collected.lock() {
                            g.extend(modes);
                        }
                    }
                    None => eprintln!("{month}: run void — the month stays unharvested"),
                }
                if i % 20 == 0 {
                    eprintln!("{}/{} months harvested", i + 1, n_months);
                }
            }
        }));
    }
    for w in workers {
        let _ = w.join();
    }
    let mut modes = Arc::try_unwrap(collected)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    modes.sort_by(|a, b| (a.0, a.1, a.2).cmp(&(b.0, b.1, b.2)));
    let n_runs = n_runs.load(Ordering::SeqCst);
    eprintln!(
        "v1y {} runs over {} months: {} records (l 0..{lmax}, freq in Hz)",
        n_runs,
        n_months,
        modes.len()
    );
    if modes.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&modes);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} records, roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
