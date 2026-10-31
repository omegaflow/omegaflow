use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use omegaflow::archivar::ae_ua::{FILL, day_start_unix, parse};

fn iso(unix: f64) -> String {
    let secs = unix as i64;
    let days = secs.div_euclid(86400);
    let sod = secs.rem_euclid(86400);
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    format!(
        "{year:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60
    )
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dir = match args.iter().position(|a| a == "--dir") {
        Some(i) => match args.get(i + 1) {
            Some(v) => v.clone(),
            None => {
                println!("usage: ae_ua_series --dir <directory> [--out <path>]");
                return;
            }
        },
        None => {
            println!("usage: ae_ua_series --dir <directory> [--out <path>]");
            return;
        }
    };
    let out = args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .cloned();

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
            println!("ae_ua_series: {dir}: {e}");
            return;
        }
    }
    files.sort();

    let mut rows: BTreeMap<i64, [Option<f64>; 3]> = BTreeMap::new();
    let mut days = 0usize;
    for path in &files {
        let text = match fs::read_to_string(path) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let day = match parse(&text) {
            Some(d) => d,
            None => continue,
        };
        let start = match day_start_unix(day.year_day) {
            Some(v) => v,
            None => continue,
        };
        days += 1;
        for block in &day.blocks {
            let slot = match block.ivar {
                46 => 0usize,
                47 => 1,
                41 => 2,
                _ => continue,
            };
            for (index, value) in block.values.iter().enumerate() {
                if (*value - FILL).abs() < 1.0 {
                    continue;
                }
                let t = start as i64 + (block.ib + index) as i64 * 15;
                rows.entry(t).or_insert([None, None, None])[slot] = Some(*value);
            }
        }
    }

    let mut lines = String::from("ut\tunix\tua_wind_ms\tua_wind_vertical_ms\tneutral_temp_k\n");
    let mut wind = 0usize;
    let mut vertical = 0usize;
    let mut temp = 0usize;
    let mut first = 0i64;
    let mut last = 0i64;
    let mut seen = false;
    for (t, values) in &rows {
        if values[0].is_none() && values[1].is_none() && values[2].is_none() {
            continue;
        }
        if !seen {
            first = *t;
            seen = true;
        }
        last = *t;
        if values[0].is_some() {
            wind += 1;
        }
        if values[1].is_some() {
            vertical += 1;
        }
        if values[2].is_some() {
            temp += 1;
        }
        let cell = |v: &Option<f64>| match v {
            Some(x) => format!("{x:.3}"),
            None => String::new(),
        };
        lines.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            iso(*t as f64),
            t,
            cell(&values[0]),
            cell(&values[1]),
            cell(&values[2])
        ));
    }
    if let Some(path) = out {
        match fs::write(&path, &lines) {
            Ok(()) => {}
            Err(e) => println!("ae_ua_series: write {path}: {e}"),
        }
    }
    println!(
        "ae_ua_series: {days} days from {} files, {} intervals, wind {wind}, vertical {vertical}, temperature {temp}",
        files.len(),
        rows.len()
    );
    if seen {
        println!(
            "ae_ua_series: {} .. {}",
            iso(first as f64),
            iso(last as f64)
        );
    }
}
