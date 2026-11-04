use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const USAGE: &str = "usage: series_manifest --root <dir> [--root ...] --from <iso> --to <iso> [--bin <seconds>] [--max-bytes <n>] [--limit <n>]";

#[derive(Clone, Debug)]
pub struct Spec {
    pub roots: Vec<String>,
    pub from: f64,
    pub to: f64,
    pub bin: i64,
    pub max_bytes: u64,
    pub limit: usize,
}

pub fn parse_time(token: &str) -> Option<f64> {
    omegaflow::archivar::parse_iso_unix(token)
}

pub fn row(line: &str) -> Option<(f64, f64)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let cells: Vec<&str> = trimmed
        .split([' ', '\t', ';', '|'])
        .filter(|c| !c.is_empty())
        .collect();
    let mut time: Option<f64> = None;
    let mut value: Option<f64> = None;
    for cell in &cells {
        if time.is_none() {
            if let Some(t) = parse_time(cell) {
                time = Some(t);
                continue;
            }
        }
        if value.is_none() {
            if let Ok(v) = cell.parse::<f64>() {
                if v.is_finite() {
                    value = Some(v);
                }
            }
        }
    }
    match (time, value) {
        (Some(t), Some(v)) => Some((t, v)),
        _ => None,
    }
}

#[derive(Clone, Debug)]
pub struct Sample {
    pub path: PathBuf,
    pub first: Option<f64>,
    pub last: Option<f64>,
    pub in_window: usize,
    pub total: usize,
    pub median_gap: f64,
}

pub fn scan_file(path: &Path, spec: &Spec) -> Option<Sample> {
    let meta = fs::metadata(path).ok()?;
    if meta.len() > spec.max_bytes {
        return None;
    }
    let text = fs::read_to_string(path).ok()?;
    let mut times: Vec<f64> = Vec::new();
    let mut in_window = 0usize;
    let mut total = 0usize;
    for line in text.lines() {
        if let Some((t, _)) = row(line) {
            total += 1;
            if t >= spec.from && t <= spec.to {
                in_window += 1;
                times.push(t);
            }
        }
    }
    if total == 0 {
        return None;
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mut gaps: Vec<f64> = times.windows(2).map(|w| w[1] - w[0]).collect();
    gaps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median_gap = if gaps.is_empty() {
        0.0
    } else {
        gaps[gaps.len() / 2]
    };
    Some(Sample {
        path: path.to_path_buf(),
        first: times.first().copied(),
        last: times.last().copied(),
        in_window,
        total,
        median_gap,
    })
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>, depth: usize) {
    if depth > 4 {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out, depth + 1);
        } else {
            out.push(path);
        }
    }
}

fn format_time(unix: f64) -> String {
    let days = (unix / 86_400.0).floor() as i64;
    let rest = unix - days as f64 * 86_400.0;
    match omegaflow::spectral::civil_from_days(days) {
        Some((year, month, day)) => {
            let hour = (rest / 3600.0).floor() as i64;
            let minute = ((rest - hour as f64 * 3600.0) / 60.0).floor() as i64;
            let second = (rest - hour as f64 * 3600.0 - minute as f64 * 60.0).floor() as i64;
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
        }
        None => "absent".to_string(),
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut spec = Spec {
        roots: Vec::new(),
        from: 0.0,
        to: f64::INFINITY,
        bin: 3600,
        max_bytes: 64 * 1024 * 1024,
        limit: 0,
    };
    let mut from_given = false;
    let mut to_given = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--root" => {
                spec.roots.push(
                    args.get(index + 1)
                        .ok_or_else(|| USAGE.to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--from" => {
                let token = args.get(index + 1).ok_or_else(|| USAGE.to_string())?;
                spec.from =
                    parse_time(token).ok_or_else(|| format!("--from {token} does not parse"))?;
                from_given = true;
                index += 2;
            }
            "--to" => {
                let token = args.get(index + 1).ok_or_else(|| USAGE.to_string())?;
                spec.to =
                    parse_time(token).ok_or_else(|| format!("--to {token} does not parse"))?;
                to_given = true;
                index += 2;
            }
            "--bin" => {
                spec.bin = args
                    .get(index + 1)
                    .ok_or_else(|| USAGE.to_string())?
                    .parse()
                    .map_err(|_| "bin carries no number".to_string())?;
                index += 2;
            }
            "--max-bytes" => {
                spec.max_bytes = args
                    .get(index + 1)
                    .ok_or_else(|| USAGE.to_string())?
                    .parse()
                    .map_err(|_| "max-bytes carries no number".to_string())?;
                index += 2;
            }
            "--limit" => {
                spec.limit = args
                    .get(index + 1)
                    .ok_or_else(|| USAGE.to_string())?
                    .parse()
                    .map_err(|_| "limit carries no number".to_string())?;
                index += 2;
            }
            other => return Err(format!("{other} names no option; {USAGE}")),
        }
    }
    if spec.roots.is_empty() || !from_given || !to_given {
        return Err(USAGE.to_string());
    }
    let mut files: Vec<PathBuf> = Vec::new();
    for root in &spec.roots {
        walk(Path::new(root), &mut files, 0);
    }
    files.sort();
    println!(
        "series_manifest: {} file(s) under {} root(s), window {} .. {}, bin {} s",
        files.len(),
        spec.roots.len(),
        format_time(spec.from),
        format_time(spec.to),
        spec.bin
    );
    println!("path\tfirst\tlast\tin_window\ttotal\tmedian_gap_s");
    let mut with_data = 0usize;
    let mut in_window_files = 0usize;
    let mut reported = 0usize;
    for path in &files {
        if let Some(sample) = scan_file(path, &spec) {
            with_data += 1;
            if sample.in_window > 0 {
                in_window_files += 1;
                let first = match sample.first {
                    Some(value) => format_time(value),
                    None => "absent".to_string(),
                };
                let last = match sample.last {
                    Some(value) => format_time(value),
                    None => "absent".to_string(),
                };
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{:.3}",
                    sample.path.display(),
                    first,
                    last,
                    sample.in_window,
                    sample.total,
                    sample.median_gap
                );
                reported += 1;
                if spec.limit > 0 && reported >= spec.limit {
                    break;
                }
            }
        }
    }
    println!(
        "series_manifest: {with_data} file(s) carry an ISO time and a number; {in_window_files} carry samples in the window; {reported} reported"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_row_with_time_and_value_parses() {
        let got = row("2005-03-04T22:10:18 12.5").expect("a row");
        assert!((got.0 - 1_109_974_218.0).abs() < 1.0);
        assert!((got.1 - 12.5).abs() < 1e-12);
    }

    #[test]
    fn a_row_without_time_is_absent() {
        assert!(row("12.5 13.5").is_none());
    }

    #[test]
    fn a_comment_carries_nothing() {
        assert!(row("# 2005-03-04T22:10:18 12.5").is_none());
    }

    #[test]
    fn a_tab_separated_row_parses() {
        let got = row("2005-03-04T22:10:18\t12.5").expect("a row");
        assert!((got.1 - 12.5).abs() < 1e-12);
    }
}
