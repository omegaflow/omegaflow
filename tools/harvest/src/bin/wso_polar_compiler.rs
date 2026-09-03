use omegaflow::cdn::upload_asset;
use omegaflow::lsk::days_from_civil;
use omegaflow::wso_polar::{parse_bin, write_bin};
use std::process::Command;

const POLAR_URL: &str = "http://wso.stanford.edu/Polar.html";

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
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn parse_date(s: &str) -> Option<i64> {
    let date = s.split('_').next()?;
    let mut parts = date.split(':');
    let year: i64 = parts.next()?.parse().ok()?;
    let month: i64 = parts.next()?.parse().ok()?;
    let day: i64 = parts.next()?.parse().ok()?;
    days_from_civil(year, month, day)
}

fn parse_mt(s: &str) -> Option<f64> {
    let end = s
        .bytes()
        .position(|b| !(b.is_ascii_digit() || b == b'-' || b == b'+' || b == b'.'))
        .unwrap_or(s.len());
    let num = &s[..end];
    if num.is_empty() || num == "-" || num.contains('X') {
        return None;
    }
    num.parse().ok()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "wso_polar.bin".to_string());

    let Some(body) = fetch(POLAR_URL) else {
        eprintln!("Polar.html fetch void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let text = String::from_utf8_lossy(&body);
    let mut records: Vec<(i64, f64, f64, f64)> = Vec::new();
    let mut skipped = 0usize;
    for line in text.lines() {
        let line = line.trim_start();
        if !line.starts_with(|c: char| c.is_ascii_digit()) {
            continue;
        }
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 9 {
            continue;
        }
        let Some(days) = parse_date(f[0]) else {
            skipped += 1;
            continue;
        };
        let (Some(north), Some(south), Some(avg)) =
            (parse_mt(f[1]), parse_mt(f[2]), parse_mt(f[3]))
        else {
            skipped += 1;
            continue;
        };
        if !north.is_finite() || !south.is_finite() || !avg.is_finite() {
            skipped += 1;
            continue;
        }
        records.push((days, north * 1e-6, south * 1e-6, avg * 1e-6));
    }
    records.sort_by(|a, b| a.0.cmp(&b.0));
    records.dedup_by(|a, b| a.0 == b.0);
    eprintln!(
        "WSO polar field: {} records, {} skipped (XXX/void)",
        records.len(),
        skipped
    );
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
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
