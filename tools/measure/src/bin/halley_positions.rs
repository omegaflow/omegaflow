use std::fs;
use std::path::{Path, PathBuf};

use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::archivar::parse_iso_unix;
use omegaflow::bsp_reader::spk::SpkFile;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn number(token: &str) -> Option<f64> {
    token.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let kernel = match arg_value(&args, "--kernel") {
        Some(v) => v,
        None => {
            println!(
                "usage: halley_positions --kernel <file.bsp> --dir <tab dir> [--target <id>] [--center <id>]"
            );
            return;
        }
    };
    let dir = match arg_value(&args, "--dir") {
        Some(v) => v,
        None => {
            println!("halley_positions: --dir missing");
            return;
        }
    };
    let target: i32 = match arg_value(&args, "--target") {
        Some(v) => match v.parse() {
            Ok(n) => n,
            Err(_) => -67,
        },
        None => -67,
    };
    let center: i32 = match arg_value(&args, "--center") {
        Some(v) => match v.parse() {
            Ok(n) => n,
            Err(_) => 1000036,
        },
        None => 1000036,
    };
    let spac = match SpkFile::open(&kernel) {
        Ok(s) => s,
        Err(e) => {
            println!("halley_positions: {kernel}: {e}");
            return;
        }
    };
    let lsk = match embedded_lsk() {
        Some(v) => v,
        None => {
            println!("halley_positions: the embedded leap second table is absent");
            return;
        }
    };

    let mut files: Vec<PathBuf> = Vec::new();
    match fs::read_dir(&dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "tab").unwrap_or(false) {
                    files.push(path);
                }
            }
        }
        Err(e) => {
            println!("halley_positions: {dir}: {e}");
            return;
        }
    }
    files.sort();

    println!(
        "file\tut\tet\tx_km\ty_km\tz_km\tr_km\tbx_nt\tby_nt\tbz_nt\tbt_nt\tbu_nt\tbux_nt\tflag"
    );
    let mut rows = 0usize;
    let mut outside = 0usize;
    let mut closest = f64::INFINITY;
    let mut closest_ut = String::new();
    for path in &files {
        let text = match fs::read_to_string(path) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let name = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => String::new(),
        };
        for line in text.lines() {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() < 8 {
                continue;
            }
            let ut = cols[0];
            if !ut.starts_with("19") {
                continue;
            }
            let unix = match parse_iso_unix(ut) {
                Some(v) => v,
                None => continue,
            };
            let tdb = match lsk.unix_to_tdb(unix) {
                Some(v) => v,
                None => continue,
            };
            let state = match spac.state(target, center, tdb) {
                Ok(s) => s,
                Err(_) => {
                    outside += 1;
                    continue;
                }
            };
            let r = (state[0] * state[0] + state[1] * state[1] + state[2] * state[2]).sqrt();
            if r < closest {
                closest = r;
                closest_ut = ut.to_string();
            }
            let mut values = [0.0_f64; 6];
            for (i, slot) in values.iter_mut().enumerate() {
                *slot = match number(cols.get(i + 1).copied().unwrap_or("")) {
                    Some(v) => v,
                    None => f64::NAN,
                };
            }
            println!(
                "{name}\t{ut}\t{tdb:.3}\t{:.3}\t{:.3}\t{:.3}\t{r:.1}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{}",
                state[0],
                state[1],
                state[2],
                values[0],
                values[1],
                values[2],
                values[3],
                values[4],
                values[5],
                cols[7]
            );
            rows += 1;
        }
    }
    println!(
        "halley_positions: {rows} rows from {} files, {outside} outside the kernel, closest {closest:.1} km at {closest_ut}",
        files.len()
    );
    let _ = Path::new(&dir);
}
