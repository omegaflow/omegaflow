use std::fs;
use std::path::PathBuf;

use omegaflow::archivar::ae_ua::{FILL, day_start_unix, parse};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dir = match args.iter().position(|a| a == "--dir") {
        Some(i) => match args.get(i + 1) {
            Some(v) => v.clone(),
            None => {
                println!("usage: ae_ua_probe --dir <directory> [--ivar 46]");
                return;
            }
        },
        None => {
            println!("usage: ae_ua_probe --dir <directory> [--ivar 46]");
            return;
        }
    };
    let ivar: i32 = match args.iter().position(|a| a == "--ivar") {
        Some(i) => match args.get(i + 1).and_then(|v| v.parse::<i32>().ok()) {
            Some(v) => v,
            None => 46,
        },
        None => 46,
    };

    let mut files: Vec<PathBuf> = Vec::new();
    match fs::read_dir(&dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "dat").unwrap_or(false) {
                    files.push(path);
                }
            }
        }
        Err(e) => {
            println!("ae_ua_probe: {dir}: {e}");
            return;
        }
    }
    files.sort();

    let mut days = 0usize;
    let mut blocks = 0usize;
    let mut rows = 0usize;
    let mut filled = 0usize;
    let mut first = f64::NAN;
    let mut last = f64::NAN;
    for path in &files {
        let text = match fs::read_to_string(path) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let day = match parse(&text) {
            Some(d) => d,
            None => {
                println!("ae_ua_probe: {} does not parse", path.display());
                continue;
            }
        };
        days += 1;
        let start = match day_start_unix(day.year_day) {
            Some(v) => v,
            None => continue,
        };
        for block in &day.blocks {
            blocks += 1;
            if block.ivar != ivar {
                continue;
            }
            for (index, value) in block.values.iter().enumerate() {
                let t = start + (block.ib + index) as f64 * 15.0;
                if first.is_nan() || t < first {
                    first = t;
                }
                if last.is_nan() || t > last {
                    last = t;
                }
                rows += 1;
                if (*value - FILL).abs() < 1.0 {
                    filled += 1;
                }
            }
        }
    }
    println!(
        "ae_ua_probe: {days} days from {} files, {blocks} blocks, ivar {ivar}: {rows} values, {filled} fill",
        files.len()
    );
    if rows > 0 {
        println!(
            "ae_ua_probe: ivar {ivar} spans unix {first:.0} .. {last:.0} ({:.2} days)",
            (last - first) / 86400.0
        );
    }
}
