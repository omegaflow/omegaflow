use std::collections::BTreeMap;
use std::env;
use std::fs;

use omegaflow::archivar::f107;
use omegaflow::spectral::civil_from_days;

fn monthly_from_bin(path: &str) -> Result<BTreeMap<(i32, u32), f64>, String> {
    let bytes = fs::read(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    let rows = f107::parse_bin(&bytes).ok_or_else(|| format!("{path} does not parse as F107"))?;
    let mut sums: BTreeMap<(i32, u32), (f64, u32)> = BTreeMap::new();
    for (day, value) in rows {
        let Some((year, month, _)) = civil_from_days(day) else {
            continue;
        };
        let entry = sums.entry((year as i32, month)).or_insert((0.0, 0));
        entry.0 += value * 1e22;
        entry.1 += 1;
    }
    Ok(sums
        .into_iter()
        .map(|(k, (s, n))| (k, s / n as f64))
        .collect())
}

fn monthly_from_text(
    path: &str,
    year_col: usize,
    month_col: usize,
    value_col: usize,
) -> Result<BTreeMap<(i32, u32), f64>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    let mut sums: BTreeMap<(i32, u32), (f64, u32)> = BTreeMap::new();
    for line in text.lines().skip(1) {
        let cells: Vec<&str> = line.split_whitespace().collect();
        if cells.len() <= value_col.max(year_col).max(month_col) {
            continue;
        }
        let year = cells[year_col].parse::<i32>();
        let month = cells[month_col].parse::<u32>();
        let value = cells[value_col].parse::<f64>();
        if let (Ok(year), Ok(month), Ok(value)) = (year, month, value) {
            if !(1..=12).contains(&month) || !value.is_finite() {
                continue;
            }
            let entry = sums.entry((year, month)).or_insert((0.0, 0));
            entry.0 += value;
            entry.1 += 1;
        }
    }
    Ok(sums
        .into_iter()
        .map(|(k, (s, n))| (k, s / n as f64))
        .collect())
}

fn slope(rows: &[(f64, f64)]) -> Option<(f64, usize)> {
    let n = rows.len();
    if n < 6 {
        return None;
    }
    let sx: f64 = rows.iter().map(|r| r.0).sum();
    let sy: f64 = rows.iter().map(|r| r.1).sum();
    let sxx: f64 = rows.iter().map(|r| r.0 * r.0).sum();
    let sxy: f64 = rows.iter().map(|r| r.0 * r.1).sum();
    let den = n as f64 * sxx - sx * sx;
    if den.abs() < f64::EPSILON {
        return None;
    }
    Some(((n as f64 * sxy - sx * sy) / den, n))
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = values.len();
    if n == 0 {
        return f64::NAN;
    }
    if n % 2 == 1 {
        values[n / 2]
    } else {
        0.5 * (values[n / 2 - 1] + values[n / 2])
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let usage = "usage: ratio_jump --bin <f107.bin> --text <series.txt> --cols <year,month,value> [--window <months>]";
    let mut bin: Option<&String> = None;
    let mut text: Option<&String> = None;
    let mut cols: Option<(usize, usize, usize)> = None;
    let mut window = 24usize;
    let mut floor: Option<f64> = None;
    let mut at: Option<(i32, u32)> = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--bin" => {
                bin = args.get(index + 1);
                index += 2;
            }
            "--text" => {
                text = args.get(index + 1);
                index += 2;
            }
            "--cols" => {
                let raw = args.get(index + 1).ok_or_else(|| usage.to_string())?;
                let parts: Vec<usize> = raw
                    .split(',')
                    .map(|p| p.trim().parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| format!("--cols {raw} carries no three column numbers"))?;
                if parts.len() != 3 {
                    return Err(format!(
                        "--cols {raw} carries {} numbers, not three",
                        parts.len()
                    ));
                }
                cols = Some((
                    parts[0].saturating_sub(1),
                    parts[1].saturating_sub(1),
                    parts[2].saturating_sub(1),
                ));
                index += 2;
            }
            "--at" => {
                let raw = args.get(index + 1).ok_or_else(|| usage.to_string())?;
                let normalised = raw.replace('-', ",");
                let parts: Vec<&str> = normalised.split(',').collect();
                if parts.len() != 2 {
                    return Err(format!("--at {raw} carries no year,month"));
                }
                let year = parts[0]
                    .parse::<i32>()
                    .map_err(|_| format!("--at {raw}: no year"))?;
                let month = parts[1]
                    .parse::<u32>()
                    .map_err(|_| format!("--at {raw}: no month"))?;
                at = Some((year, month));
                index += 2;
            }
            "--floor" => {
                floor = Some(
                    args.get(index + 1)
                        .ok_or_else(|| usage.to_string())?
                        .parse::<f64>()
                        .map_err(|_| "the floor carries no number".to_string())?,
                );
                index += 2;
            }
            "--window" => {
                window = args
                    .get(index + 1)
                    .ok_or_else(|| usage.to_string())?
                    .parse::<usize>()
                    .map_err(|_| "the window carries no number".to_string())?;
                index += 2;
            }
            other => return Err(format!("{other}: not a flag of this tool")),
        }
    }
    let bin_path = bin.ok_or_else(|| usage.to_string())?;
    let text_path = text.ok_or_else(|| usage.to_string())?;
    let (year_col, month_col, value_col) = cols.ok_or_else(|| usage.to_string())?;
    let floor = floor.ok_or_else(|| usage.to_string())?;

    let a = monthly_from_bin(bin_path)?;
    let b = monthly_from_text(text_path, year_col, month_col, value_col)?;

    let keys: Vec<(i32, u32)> = a
        .keys()
        .filter(|k| b.get(k).is_some_and(|v| *v >= floor) && a.get(k).is_some_and(|v| *v > 0.0))
        .cloned()
        .collect();
    if keys.len() < 2 * window + 1 {
        return Err(format!(
            "{} common months carry no window of {}",
            keys.len(),
            window
        ));
    }
    let pairs: Vec<(f64, f64)> = keys.iter().map(|k| (b[k], a[k])).collect();

    let mut found: Vec<(f64, usize, f64)> = Vec::new();
    for split in 12..pairs.len().saturating_sub(12) {
        let (Some((before, nb)), Some((after, na))) =
            (slope(&pairs[..split]), slope(&pairs[split..]))
        else {
            continue;
        };
        if !(before > 0.0) {
            continue;
        }
        let ratio = after / before;
        let weight = (nb.min(na) as f64).sqrt();
        let score = (ratio.ln()).abs() * weight;
        found.push((score, split, ratio));
        let _ = na;
    }
    found.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut kept: Vec<(f64, usize, f64)> = Vec::new();
    for entry in found {
        if kept
            .iter()
            .all(|k| (k.1 as i64 - entry.1 as i64).abs() > 24)
        {
            kept.push(entry);
        }
        if kept.len() == 5 {
            break;
        }
    }
    println!("months {}  top changepoints:", pairs.len());
    for (score, split, ratio) in &kept {
        let (year, month) = keys[*split];
        println!(
            "  {:04}-{:02}  slope ratio {:.3}  ({:+.1} %)  score {:.1}",
            year,
            month,
            ratio,
            100.0 * (ratio - 1.0),
            score
        );
    }
    if let Some((year, month)) = at {
        if let Some(split) = keys.iter().position(|k| *k == (year, month)) {
            if let (Some((before, nb)), Some((after, na))) =
                (slope(&pairs[..split]), slope(&pairs[split..]))
            {
                println!(
                    "at {:04}-{:02}: slope before {:.4} (n={nb})  after {:.4} (n={na})  ratio {:.4} ({:+.2} %)",
                    year,
                    month,
                    before,
                    after,
                    after / before,
                    100.0 * (after / before - 1.0)
                );
            }
        } else {
            println!(
                "at {:04}-{:02}: that month carries no common data",
                year, month
            );
        }
    }
    let split = kept.first().map(|k| k.1).unwrap_or(pairs.len() / 2);
    let ratio = kept.first().map(|k| k.2).unwrap_or(1.0);
    if let (Some((before, nb)), Some((after, na))) =
        (slope(&pairs[..split]), slope(&pairs[split..]))
    {
        println!(
            "slope before {:.4} (n={nb})  after {:.4} (n={na})",
            before, after
        );
    }
    if let (Some((first, nf)), Some((last, nl))) = (
        slope(&pairs[..pairs.len() / 3]),
        slope(&pairs[2 * pairs.len() / 3..]),
    ) {
        println!(
            "first third slope {:.4} (n={nf})  last third {:.4} (n={nl})  ratio {:.3}",
            first,
            last,
            last / first
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_two_regime_pair_yields_the_slope_ratio() {
        let mut rows = Vec::new();
        for i in 0..40 {
            rows.push((i as f64, 2.0 * i as f64 + 5.0));
        }
        for i in 40..80 {
            rows.push((i as f64, 3.0 * i as f64 + 5.0));
        }
        let (before, _) = slope(&rows[..40]).expect("before");
        let (after, _) = slope(&rows[40..]).expect("after");
        assert!((before - 2.0).abs() < 1e-9, "before {before}");
        assert!((after - 3.0).abs() < 1e-9, "after {after}");
        assert!(((after / before) - 1.5).abs() < 1e-9);
    }

    #[test]
    fn a_monthly_text_series_reads_its_named_columns() {
        let path = env::temp_dir().join("ratio_jump_months.txt");
        fs::write(
            &path,
            "y m dec value\n1980 01 1980.0 120.0\n1980 02 1980.1 130.0\n",
        )
        .expect("write");
        let map = monthly_from_text(&path.to_string_lossy(), 0, 1, 3).expect("read");
        assert_eq!(map.get(&(1980, 1)).copied(), Some(120.0));
        assert_eq!(map.get(&(1980, 2)).copied(), Some(130.0));
        let _ = fs::remove_file(&path);
    }
}
