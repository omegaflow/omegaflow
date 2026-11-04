use omegaflow::archivar::asset_path::compiled;
use omegaflow::archivar::embedded_lsk;
use omegaflow::lsk::days_from_civil;
use std::env;
use std::fs;

const NETLOC: &str = "lasp.colorado.edu";
const MAGIC: [u8; 4] = *b"TSE1";
const BASE: &str = "https://lasp.colorado.edu/data/timed_see/spwx";
const VALUE_COLUMNS: [usize; 8] = [5, 7, 9, 11, 13, 15, 17, 19];
const FILL_LIMIT: f64 = -0.5;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn positional_source(args: &[String]) -> Option<String> {
    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        if matches!(a.as_str(), "--source" | "--year" | "--out") {
            i += 2;
            continue;
        }
        if a.starts_with('-') {
            i += 1;
            continue;
        }
        return Some(a.clone());
    }
    None
}

fn read_source(source: &str) -> Result<Vec<u8>, String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        let out = std::process::Command::new("curl")
            .args(["-sSfL", "--retry", "3", "--max-time", "300", source])
            .output()
            .map_err(|e| format!("{source}: curl void ({e})"))?;
        if !out.status.success() {
            return Err(format!(
                "{source}: fetch void ({})",
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
        Ok(out.stdout)
    } else {
        fs::read(source).map_err(|e| format!("{source}: read void ({e})"))
    }
}

fn timestamp(year: i64, month: i64, day: i64, hhmmss: &str) -> Option<f64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hhmmss.len() != 6 {
        return None;
    }
    let hh: i64 = hhmmss.get(0..2)?.parse().ok()?;
    let mm: i64 = hhmmss.get(2..4)?.parse().ok()?;
    let ss: i64 = hhmmss.get(4..6)?.parse().ok()?;
    if !(0..24).contains(&hh) || !(0..60).contains(&mm) || !(0..61).contains(&ss) {
        return None;
    }
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86_400.0 + (hh * 3600 + mm * 60 + ss) as f64)
}

fn parse_obs(
    text: &str,
    lsk: &omegaflow::archivar::LeapSeconds,
) -> (Vec<(f64, f64, u32)>, usize, Option<String>) {
    let mut records = Vec::new();
    let mut rows = 0usize;
    let mut version = None;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with(';') {
            if let Some(v) = trimmed.strip_prefix("; Version:") {
                version = Some(v.trim().to_string());
            }
            continue;
        }
        let cols: Vec<&str> = trimmed.split_whitespace().collect();
        if cols.len() < 20 {
            continue;
        }
        let (Ok(year), Ok(month), Ok(day)) = (
            cols[0].parse::<i64>(),
            cols[1].parse::<i64>(),
            cols[2].parse::<i64>(),
        ) else {
            continue;
        };
        let Some(unix) = timestamp(year, month, day, cols[3]) else {
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        rows += 1;
        for (idx, col) in VALUE_COLUMNS.iter().enumerate() {
            let Ok(value) = cols[*col].parse::<f64>() else {
                continue;
            };
            if value <= FILL_LIMIT || !value.is_finite() {
                continue;
            }
            records.push((tdb, value, idx as u32));
        }
    }
    (records, rows, version)
}

fn run(args: &[String]) -> Result<(), String> {
    let explicit = arg_value(args, "--source").or_else(|| positional_source(args));
    let (source, year) = match explicit {
        Some(s) => {
            let year = arg_value(args, "--year").and_then(|v| v.parse::<i64>().ok());
            (s, year)
        }
        None => {
            let year = arg_value(args, "--year")
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(2026);
            (format!("{BASE}/{year}_TIMED_SEE_obs.dat"), Some(year))
        }
    };
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => {
            let name = match year {
                Some(y) => format!("timed_see_{y}.bin"),
                None => "timed_see.bin".to_string(),
            };
            let path = compiled(NETLOC, &name);
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            path.to_string_lossy().into_owned()
        }
    };
    let bytes = read_source(&source)?;
    let text = String::from_utf8_lossy(&bytes);
    let lsk = embedded_lsk().ok_or_else(|| "the embedded leap seconds stay unread".to_string())?;
    let (mut records, rows, version) = parse_obs(&text, &lsk);
    records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    records.dedup_by(|a, b| a.0 == b.0 && a.2 == b.2);
    if records.is_empty() {
        return Err(format!("{source}: no records — the bin stays unwritten"));
    }
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, idx) in &records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
        buf.extend_from_slice(&idx.to_le_bytes());
    }
    fs::write(&out, &buf).map_err(|e| format!("{out}: write void ({e})"))?;
    let (t0, _, _) = records[0];
    let (t1, _, _) = records[records.len() - 1];
    eprintln!(
        "{out}: {} records ({} B) from {} rows, version {}, epoch_tdb {:.0}..{:.0}",
        records.len(),
        buf.len(),
        rows,
        version.as_deref().unwrap_or("absent"),
        t0,
        t1
    );
    if let Some(year) = year {
        eprintln!("{source}: year {year}");
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "usage: timed_see_compiler <YYYY_TIMED_SEE_obs.dat> [--out <bin>] [--year 2026] [--source <path|url>]"
        );
        return;
    }
    if let Err(e) = run(&args) {
        eprintln!("timed_see: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::embedded_lsk;

    fn lsk() -> omegaflow::archivar::LeapSeconds {
        embedded_lsk().expect("leap seconds")
    }

    const EXCERPT: &str = "\
; Data_list: 2026_TIMED_SEE_obs.dat
; Version: 012
; UTC Date  Time  Julian 1_7 26_34 304 335 368 121 1335 145_165
; YR MO DA HHMMSS  Date  E U  E U  E U E U E U E U  E U   E U
2026 01 01 000000 2461041.500000  9.920E-05    1.6  5.507E-04  33.0  3.719E-04  48.2  2.285E-05  20.8  4.432E-05  15.2  6.755E-03  19.4  1.657E-04  19.9  2.886E-03   2.8
2026 01 01 000300 2461041.502083  1.000E-04    1.0  5.000E-04  30.0  3.000E-04  40.0  2.000E-05  20.0  4.000E-05  15.0  6.000E-03  19.0  1.500E-04  19.0  2.800E-03   2.8
2026 01 01 000600 2461041.504167 -1.000E+00  100.0 -1.000E+00 100.0 -1.000E+00 100.0 -1.000E+00 100.0 -1.000E+00 100.0 -1.000E+00 100.0 -1.000E+00 100.0 -1.000E+00 100.0
";

    #[test]
    fn header_is_skipped_fill_is_dropped_and_eight_bands_survive() {
        let (recs, rows, version) = parse_obs(EXCERPT, &lsk());
        assert_eq!(rows, 3);
        assert_eq!(version.as_deref(), Some("012"));
        assert_eq!(recs.len(), 16);
        assert_eq!(recs[0].2, 0);
        assert!((recs[0].1 - 9.920e-05).abs() < 1e-12);
        assert_eq!(recs[7].2, 7);
        assert!((recs[7].1 - 2.886e-03).abs() < 1e-12);
        assert!(recs[8].0 > recs[0].0);
    }

    #[test]
    fn a_wrong_width_line_is_not_half_parsed() {
        let (recs, rows, _) = parse_obs("2026 01 01 000000 2461041.5 1 2 3\n", &lsk());
        assert!(recs.is_empty());
        assert_eq!(rows, 0);
    }
}
