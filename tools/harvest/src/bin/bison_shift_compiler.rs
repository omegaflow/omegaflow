use omegaflow::bison_shift::{parse_bin, write_bin};
use omegaflow::cdn::upload_asset;
use std::process::Command;

const SHIFT_URL: &str = "https://edata.bham.ac.uk/1572/1/bison_7day_2025paper.txt";
const JD_UNIX_OFFSET: f64 = 2440587.5;

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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "bison_shift.bin".to_string());

    let body = match arg_value(&args, "--file") {
        Some(p) => std::fs::read(&p).ok(),
        None => fetch(SHIFT_URL),
    };
    let Some(body) = body else {
        eprintln!("bison_7day_2025paper.txt fetch void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let text = String::from_utf8_lossy(&body);
    let mut records: Vec<(i64, f64, f64)> = Vec::new();
    let mut skipped = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 5 {
            skipped += 1;
            continue;
        }
        let (Ok(jd), Ok(shift_uhz), Ok(err_uhz)) = (
            f[0].parse::<f64>(),
            f[3].parse::<f64>(),
            f[4].parse::<f64>(),
        ) else {
            skipped += 1;
            continue;
        };
        if !jd.is_finite() || !shift_uhz.is_finite() || !err_uhz.is_finite() {
            skipped += 1;
            continue;
        }
        if err_uhz <= 0.0 {
            skipped += 1;
            continue;
        }
        let days = (jd - JD_UNIX_OFFSET).round() as i64;
        records.push((days, shift_uhz * 1e-6, err_uhz * 1e-6));
    }
    records.sort_by(|a, b| a.0.cmp(&b.0));
    records.dedup_by(|a, b| a.0 == b.0);
    eprintln!(
        "BiSON-Shift: {} records, {} skipped (void/non-finite/err<=0)",
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
