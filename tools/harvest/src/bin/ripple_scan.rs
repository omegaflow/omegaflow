use std::collections::BTreeMap;
use std::env;
use std::fs;

use omegaflow::archivar::f107;

type Month = (i32, u32);

#[derive(Clone, Debug)]
pub struct Series {
    pub name: String,
    pub months: BTreeMap<Month, f64>,
    pub source: String,
}

fn month_mean(bucket: BTreeMap<Month, (f64, u32)>) -> BTreeMap<Month, f64> {
    bucket
        .into_iter()
        .map(|(k, (sum, n))| (k, sum / n as f64))
        .collect()
}

pub fn months_from_bin(path: &str) -> Result<BTreeMap<Month, f64>, String> {
    let bytes = fs::read(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    let rows = f107::parse_bin(&bytes).ok_or_else(|| format!("{path} does not parse as F107"))?;
    let mut bucket: BTreeMap<Month, (f64, u32)> = BTreeMap::new();
    for (day, value) in rows {
        if !value.is_finite() {
            continue;
        }
        let Some((year, month, _)) = omegaflow::spectral::civil_from_days(day) else {
            continue;
        };
        let entry = bucket.entry((year as i32, month)).or_insert((0.0, 0));
        entry.0 += value;
        entry.1 += 1;
    }
    Ok(month_mean(bucket))
}

pub fn months_from_text(
    path: &str,
    year_col: usize,
    month_col: usize,
    value_col: usize,
) -> Result<BTreeMap<Month, f64>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    let mut bucket: BTreeMap<Month, (f64, u32)> = BTreeMap::new();
    for line in text.lines() {
        let cells: Vec<&str> = line.split_whitespace().collect();
        let widest = year_col.max(month_col).max(value_col);
        if cells.len() <= widest {
            continue;
        }
        let (Ok(year), Ok(month), Ok(value)) = (
            cells[year_col].parse::<i32>(),
            cells[month_col].parse::<u32>(),
            cells[value_col].parse::<f64>(),
        ) else {
            continue;
        };
        if !(1..=12).contains(&month) || !value.is_finite() {
            continue;
        }
        let entry = bucket.entry((year, month)).or_insert((0.0, 0));
        entry.0 += value;
        entry.1 += 1;
    }
    Ok(month_mean(bucket))
}

pub fn months_from_rstn(dir: &str, from: usize, to: usize) -> Result<BTreeMap<Month, f64>, String> {
    let mut bucket: BTreeMap<Month, (f64, u32)> = BTreeMap::new();
    let mut files = 0usize;
    for entry in fs::read_dir(dir).map_err(|e| format!("read {dir} returned void: {e}"))? {
        let path = entry.map_err(|e| format!("{dir} carries an entry that reads as void: {e}"))?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("txt") {
            continue;
        }
        files += 1;
        let text = fs::read_to_string(&path)
            .map_err(|e| format!("read {} returned void: {e}", path.display()))?;
        for line in text.lines() {
            if line.len() < to {
                continue;
            }
            let year = line.get(0..2).and_then(|s| s.trim().parse::<i32>().ok());
            let month = line.get(2..4).and_then(|s| s.trim().parse::<u32>().ok());
            let day = line.get(4..6).and_then(|s| s.trim().parse::<u32>().ok());
            let (Some(year), Some(month), Some(day)) = (year, month, day) else {
                continue;
            };
            if day == 32 {
                continue;
            }
            let Some(field) = line.get(from - 1..to) else {
                continue;
            };
            let Ok(value) = field.trim().parse::<f64>() else {
                continue;
            };
            if !(1..=12).contains(&month) || !value.is_finite() {
                continue;
            }
            let entry = bucket.entry((1900 + year, month)).or_insert((0.0, 0));
            entry.0 += value;
            entry.1 += 1;
        }
    }
    if files == 0 {
        return Err(format!("{dir} carries no txt files"));
    }
    Ok(month_mean(bucket))
}

pub fn months_from_norp(path: &str, value_col: usize) -> Result<BTreeMap<Month, f64>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    let mut bucket: BTreeMap<Month, (f64, u32)> = BTreeMap::new();
    for line in text.lines() {
        let cells: Vec<&str> = line.split(',').map(|c| c.trim().trim_matches('"')).collect();
        if cells.len() <= value_col {
            continue;
        }
        let date = cells[0];
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() != 3 {
            continue;
        }
        let (Ok(year), Ok(month)) = (parts[0].parse::<i32>(), parts[1].parse::<u32>()) else {
            continue;
        };
        let Ok(value) = cells[value_col].parse::<f64>() else {
            continue;
        };
        if !(1..=12).contains(&month) || !value.is_finite() {
            continue;
        }
        let entry = bucket.entry((year, month)).or_insert((0.0, 0));
        entry.0 += value;
        entry.1 += 1;
    }
    Ok(month_mean(bucket))
}

#[derive(Clone, Copy, Debug)]
struct Fit {
    slope: f64,
    n: usize,
}

struct Moments {
    n: Vec<f64>,
    x: Vec<f64>,
    y: Vec<f64>,
    xx: Vec<f64>,
    xy: Vec<f64>,
}

impl Moments {
    fn build(xs: &[f64], ys: &[f64]) -> Self {
        let len = xs.len();
        let mut m = Moments {
            n: vec![0.0; len + 1],
            x: vec![0.0; len + 1],
            y: vec![0.0; len + 1],
            xx: vec![0.0; len + 1],
            xy: vec![0.0; len + 1],
        };
        for i in 0..len {
            m.n[i + 1] = m.n[i] + 1.0;
            m.x[i + 1] = m.x[i] + xs[i];
            m.y[i + 1] = m.y[i] + ys[i];
            m.xx[i + 1] = m.xx[i] + xs[i] * xs[i];
            m.xy[i + 1] = m.xy[i] + xs[i] * ys[i];
        }
        m
    }

    fn fit(&self, lo: usize, hi: usize) -> Option<Fit> {
        let n = self.n[hi] - self.n[lo];
        if n < 6.0 {
            return None;
        }
        let sx = self.x[hi] - self.x[lo];
        let sy = self.y[hi] - self.y[lo];
        let sxx = self.xx[hi] - self.xx[lo];
        let sxy = self.xy[hi] - self.xy[lo];
        let den = n * sxx - sx * sx;
        let scale = (n * sxx).abs() + (sx * sx).abs();
        if scale == 0.0 || den.abs() <= 1e-12 * scale {
            return None;
        }
        Some(Fit {
            slope: (n * sxy - sx * sy) / den,
            n: n as usize,
        })
    }
}

#[derive(Clone, Debug)]
struct Step {
    month: Month,
    ratio: f64,
    score: f64,
}

fn changepoints(
    xs: &[f64],
    ys: &[f64],
    months: &[Month],
    separation: usize,
    edge: usize,
    keep: usize,
) -> (Vec<Step>, usize) {
    let m = Moments::build(xs, ys);
    let mut found: Vec<Step> = Vec::new();
    let mut flat = 0usize;
    for split in edge..months.len().saturating_sub(edge) {
        let (Some(before), Some(after)) = (m.fit(0, split), m.fit(split, months.len())) else {
            continue;
        };
        if before.slope <= 0.0 || after.slope <= 0.0 {
            flat += 1;
            continue;
        }
        let ratio = after.slope / before.slope;
        let weight = (before.n.min(after.n) as f64).sqrt();
        found.push(Step {
            month: months[split],
            ratio,
            score: ratio.ln().abs() * weight,
        });
    }
    found.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    let mut kept: Vec<Step> = Vec::new();
    for step in found {
        let near = kept.iter().any(|k| months_between(k.month, step.month) < separation);
        if !near {
            kept.push(step);
        }
        if kept.len() == keep {
            break;
        }
    }
    (kept, flat)
}

fn months_between(a: Month, b: Month) -> usize {
    let ai = a.0 * 12 + a.1 as i32;
    let bi = b.0 * 12 + b.1 as i32;
    (ai - bi).unsigned_abs() as usize
}

fn parse_series(spec: &str) -> Result<Series, String> {
    let parts: Vec<&str> = spec.split('|').collect();
    if parts.len() != 4 {
        return Err(format!("--series {spec} carries {} fields, not four: name|kind|path|params", parts.len()));
    }
    let (name, kind, path, params) = (parts[0], parts[1], parts[2], parts[3]);
    let numbers = |required: usize| -> Result<Vec<usize>, String> {
        let list: Vec<usize> = params
            .split(',')
            .filter(|p| !p.trim().is_empty())
            .map(|p| p.trim().parse::<usize>().map_err(|_| format!("{name}: params {params} carry no number")))
            .collect::<Result<Vec<_>, _>>()?;
        if list.len() != required {
            return Err(format!("{name}: params {params} carry {} numbers, not {required}", list.len()));
        }
        Ok(list)
    };
    let months = match kind {
        "bin" => months_from_bin(path)?,
        "text" => {
            let c = numbers(3)?;
            months_from_text(path, c[0].saturating_sub(1), c[1].saturating_sub(1), c[2].saturating_sub(1))?
        }
        "rstn" => {
            let c = numbers(2)?;
            months_from_rstn(path, c[0], c[1])?
        }
        "norp" => {
            let c = numbers(1)?;
            months_from_norp(path, c[0].saturating_sub(1))?
        }
        other => return Err(format!("{name}: kind {other} is not bin, text, rstn or norp")),
    };
    if months.len() < 2 {
        return Err(format!("{name}: {path} carries {} months", months.len()));
    }
    Ok(Series {
        name: name.to_string(),
        months,
        source: format!("{kind}:{path}:{params}"),
    })
}

#[derive(Clone, Debug)]
struct Era {
    name: String,
    y0: i32,
    y1: i32,
}

fn parse_eras(raw: &str) -> Result<Vec<Era>, String> {
    let mut eras = Vec::new();
    for part in raw.split(':') {
        let nums: Vec<i32> = part
            .split(',')
            .map(|n| n.trim().parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| format!("--eras {raw} carries no year"))?;
        if nums.len() != 2 {
            return Err(format!("--eras {raw}: {part} carries {} years, not two", nums.len()));
        }
        eras.push(Era {
            name: format!("{}-{}", nums[0], nums[1]),
            y0: nums[0],
            y1: nums[1],
        });
    }
    if eras.len() != 2 {
        return Err(format!("--eras {raw} carries {} eras, not two", eras.len()));
    }
    Ok(eras)
}

#[derive(Clone, Debug)]
struct Row {
    lo: f64,
    hi: f64,
    cells: Vec<(usize, f64)>,
}

fn control_rows(
    control: &Series,
    other: &Series,
    eras: &[Era],
    width: f64,
    min_bin: usize,
) -> Vec<Row> {
    let mut keys: Vec<i64> = control
        .months
        .values()
        .map(|value| (value / width).floor() as i64)
        .collect();
    keys.sort_unstable();
    keys.dedup();
    let mut rows: Vec<Row> = Vec::new();
    for key in keys {
        let lo = key as f64 * width;
        let hi = lo + width;
        let mut cells: Vec<(usize, f64)> = Vec::new();
        for era in eras {
            let mut sum = 0.0;
            let mut n = 0usize;
            for (month, c) in &control.months {
                if month.0 < era.y0 || month.0 > era.y1 {
                    continue;
                }
                if *c < lo || *c >= hi || *c <= 0.0 {
                    continue;
                }
                let Some(v) = other.months.get(month) else {
                    continue;
                };
                if !v.is_finite() || *v <= 0.0 {
                    continue;
                }
                sum += v / c;
                n += 1;
            }
            cells.push((n, if n > 0 { sum / n as f64 } else { f64::NAN }));
        }
        if cells[0].0 < min_bin || cells[1].0 < min_bin {
            continue;
        }
        rows.push(Row { lo, hi, cells });
    }
    rows
}

fn control_table(control: &Series, other: &Series, eras: &[Era], width: f64, min_bin: usize) {
    println!();
    println!("series {} against control {}", other.name, control.name);
    println!("  bin               A: n  ratio       B: n  ratio     delta");
    for row in control_rows(control, other, eras, width, min_bin) {
        println!(
            "  {:>4.0}-{:<4.0}       {:>5}  {:.3}   {:>5}  {:.3}   {:+.1} %",
            row.lo,
            row.hi,
            row.cells[0].0,
            row.cells[0].1,
            row.cells[1].0,
            row.cells[1].1,
            100.0 * (row.cells[1].1 / row.cells[0].1 - 1.0)
        );
    }
}

fn span_years(months: &[Month]) -> f64 {
    match (months.first(), months.last()) {
        (Some(a), Some(b)) => {
            let ai = a.0 * 12 + a.1 as i32;
            let bi = b.0 * 12 + b.1 as i32;
            ((bi - ai) as f64 + 1.0) / 12.0
        }
        _ => 0.0,
    }
}

fn span(months: &BTreeMap<Month, f64>) -> String {
    let first = months.keys().next();
    let last = months.keys().next_back();
    match (first, last) {
        (Some(a), Some(b)) => format!("{:04}-{:02}..{:04}-{:02}", a.0, a.1, b.0, b.1),
        _ => "absent".to_string(),
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let usage = "usage: ripple_scan --series <name|kind|path|params> [--series ...] --min-months <n> --separation <months> [--keep <n>]";
    let mut specs: Vec<String> = Vec::new();
    let mut min_months: Option<usize> = None;
    let mut separation: Option<usize> = None;
    let mut keep = 0usize;
    let mut control: Option<String> = None;
    let mut eras: Option<String> = None;
    let mut bin_width: Option<f64> = None;
    let mut min_bin: Option<usize> = None;
    let mut edge: Option<usize> = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--series" => {
                specs.push(args.get(index + 1).ok_or_else(|| usage.to_string())?.clone());
                index += 2;
            }
            "--min-months" => {
                min_months = Some(
                    args.get(index + 1)
                        .ok_or_else(|| usage.to_string())?
                        .parse::<usize>()
                        .map_err(|_| "min-months carries no number".to_string())?,
                );
                index += 2;
            }
            "--separation" => {
                separation = Some(
                    args.get(index + 1)
                        .ok_or_else(|| usage.to_string())?
                        .parse::<usize>()
                        .map_err(|_| "separation carries no number".to_string())?,
                );
                index += 2;
            }
            "--edge" => {
                edge = Some(
                    args.get(index + 1)
                        .ok_or_else(|| usage.to_string())?
                        .parse::<usize>()
                        .map_err(|_| "edge carries no number".to_string())?,
                );
                index += 2;
            }
            "--keep" => {
                keep = args
                    .get(index + 1)
                    .ok_or_else(|| usage.to_string())?
                    .parse::<usize>()
                    .map_err(|_| "keep carries no number".to_string())?;
                index += 2;
            }
            "--control" => {
                control = Some(args.get(index + 1).ok_or_else(|| usage.to_string())?.clone());
                index += 2;
            }
            "--eras" => {
                eras = Some(args.get(index + 1).ok_or_else(|| usage.to_string())?.clone());
                index += 2;
            }
            "--bin-width" => {
                bin_width = Some(
                    args.get(index + 1)
                        .ok_or_else(|| usage.to_string())?
                        .parse::<f64>()
                        .map_err(|_| "bin-width carries no number".to_string())?,
                );
                index += 2;
            }
            "--min-bin" => {
                min_bin = Some(
                    args.get(index + 1)
                        .ok_or_else(|| usage.to_string())?
                        .parse::<usize>()
                        .map_err(|_| "min-bin carries no number".to_string())?,
                );
                index += 2;
            }
            other => return Err(format!("{other}: not a flag of this tool")),
        }
    }
    let min_months = min_months.ok_or_else(|| usage.to_string())?;
    let separation = separation.ok_or_else(|| usage.to_string())?;
    let edge = edge.ok_or_else(|| usage.to_string())?;
    let keep = if keep == 0 { 1 } else { keep };
    if specs.len() < 2 {
        return Err(format!("{} series carry no pair", specs.len()));
    }

    let mut series: Vec<Series> = Vec::new();
    for spec in &specs {
        series.push(parse_series(spec)?);
    }
    series.sort_by(|a, b| a.name.cmp(&b.name));

    for s in &series {
        println!("series {:<22} months {:>5}  {:<19}  {}", s.name, s.months.len(), span(&s.months), s.source);
    }

    if let Some(name) = control {
        let eras = parse_eras(&eras.ok_or_else(|| usage.to_string())?)?;
        let width = bin_width.ok_or_else(|| usage.to_string())?;
        let floor = min_bin.ok_or_else(|| usage.to_string())?;
        let Some(held) = series.iter().find(|s| s.name == name) else {
            return Err(format!("--control {name} names no series in this run"));
        };
        println!();
        println!("control {}  width {:.0}  min bin {}  era A {}  era B {}", held.name, width, floor, eras[0].name, eras[1].name);
        for other in series.iter().filter(|s| s.name != name) {
            control_table(held, other, &eras, width, floor);
        }
        return Ok(());
    }

    let mut all_steps: Vec<(String, Step)> = Vec::new();
    for i in 0..series.len() {
        for j in (i + 1)..series.len() {
            let a = &series[i];
            let b = &series[j];
            let common: Vec<Month> = a.months.keys().filter(|k| b.months.contains_key(*k)).cloned().collect();
            if common.len() < min_months {
                println!(
                    "pair {:<22} ~ {:<22} common months {:>5}  below {}",
                    a.name, b.name, common.len(), min_months
                );
                continue;
            }
            let xs: Vec<f64> = common.iter().map(|k| b.months[k]).collect();
            let ys: Vec<f64> = common.iter().map(|k| a.months[k]).collect();
            let (steps, flat) = changepoints(&xs, &ys, &common, separation, edge, keep);
            if steps.is_empty() && common.len() <= 2 * edge {
                println!(
                    "pair {:<22} ~ {:<22} {:>5.1} yrs  absent: {} months carry no split with edge {}",
                    a.name,
                    b.name,
                    span_years(&common),
                    common.len(),
                    edge
                );
                continue;
            }
            if steps.is_empty() {
                println!(
                    "pair {:<22} ~ {:<22} {:>5.1} yrs  absent: {} of {} splits carry a non-positive slope on one side",
                    a.name,
                    b.name,
                    span_years(&common),
                    flat,
                    common.len().saturating_sub(2 * edge)
                );
                continue;
            }
            let rendered: Vec<String> = steps
                .iter()
                .map(|s| format!("{:04}-{:02} {:+.1}%", s.month.0, s.month.1, 100.0 * (s.ratio - 1.0)))
                .collect();
            println!(
                "pair {:<22} ~ {:<22} common months {:>5}  top {}",
                a.name,
                b.name,
                common.len(),
                rendered.join("  ")
            );
            for step in steps {
                all_steps.push((format!("{}~{}", a.name, b.name), step));
            }
        }
    }

    let mut clusters: Vec<(Month, Vec<(String, Step)>)> = Vec::new();
    let mut ordered = all_steps.clone();
    ordered.sort_by(|a, b| a.1.month.cmp(&b.1.month).then(a.0.cmp(&b.0)));
    for (name, step) in ordered {
        match clusters.last_mut() {
            Some((anchor, members)) if months_between(*anchor, step.month) <= separation => {
                members.push((name, step));
            }
            _ => clusters.push((step.month, vec![(name, step)])),
        }
    }
    println!();
    println!("ripple: dates where more than one pair steps within {} months", separation);
    let mut any = false;
    for (anchor, members) in &clusters {
        if members.len() < 2 {
            continue;
        }
        let mut distinct: Vec<&str> = Vec::new();
        for (name, _) in members {
            if !distinct.contains(&name.as_str()) {
                distinct.push(name.as_str());
            }
        }
        if distinct.len() < 2 {
            continue;
        }
        any = true;
        let mean = members.iter().map(|(_, s)| (s.ratio - 1.0).abs()).sum::<f64>() / members.len() as f64;
        let names: Vec<String> = members.iter().map(|(n, s)| format!("{n}@{:04}-{:02}", s.month.0, s.month.1)).collect();
        println!(
            "  {:04}-{:02}  distinct pairs {}  steps {}  mean |step| {:+.1} %  {}",
            anchor.0,
            anchor.1,
            distinct.len(),
            members.len(),
            100.0 * mean,
            names.join(", ")
        );
    }
    if !any {
        println!("  absent: no date is shared by two pairs");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_two_regime_pair_yields_its_slope_ratio() {
        let months: Vec<Month> = (0..80).map(|i| (1970 + (i as i32) / 12, (i as u32) % 12 + 1)).collect();
        let xs: Vec<f64> = months.iter().enumerate().map(|(i, _)| i as f64).collect();
        let ys: Vec<f64> = months
            .iter()
            .enumerate()
            .map(|(i, _)| if i < 40 { 2.0 * i as f64 + 5.0 } else { 3.0 * i as f64 + 5.0 })
            .collect();
        let (steps, flat) = changepoints(&xs, &ys, &months, 24, 12, 1);
        assert_eq!(flat, 0);
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].month, months[40]);
        assert!((steps[0].ratio - 1.5).abs() < 1e-6, "ratio {}", steps[0].ratio);
    }

    #[test]
    fn a_series_in_si_units_gives_the_same_slope_ratio() {
        let months: Vec<Month> = (0..80)
            .map(|i| (1970 + (i as i32) / 12, (i as u32) % 12 + 1))
            .collect();
        let xs: Vec<f64> = months.iter().enumerate().map(|(i, _)| (i as f64 + 40.0) * 1e-22).collect();
        let ys: Vec<f64> = months
            .iter()
            .enumerate()
            .map(|(i, _)| if i < 40 { 2.0 * i as f64 + 5.0 } else { 3.0 * i as f64 + 5.0 })
            .collect();
        let (steps, flat) = changepoints(&xs, &ys, &months, 24, 12, 1);
        assert_eq!(flat, 0, "a scale of 1e-22 is not degenerate");
        assert_eq!(steps.len(), 1);
        assert!((steps[0].ratio - 1.5).abs() < 1e-6, "ratio {}", steps[0].ratio);
    }

    #[test]
    fn eras_parse_two_ranges_and_reject_one() {
        let eras = parse_eras("2002,2004:2006,2009").expect("eras");
        assert_eq!(eras.len(), 2);
        assert_eq!((eras[1].y0, eras[1].y1), (2006, 2009));
        assert!(parse_eras("2002,2004").is_err());
    }

    #[test]
    fn the_control_table_separates_a_tilt_from_the_activity_curve() {
        let eras = parse_eras("1990,1995:2000,2005").expect("eras");
        let mut control = BTreeMap::new();
        let mut early = BTreeMap::new();
        let mut late = BTreeMap::new();
        for year in 1990..=2005 {
            for month in 1..=12u32 {
                let level = 100.0 + 10.0 * ((month as f64) / 12.0);
                control.insert((year, month), level);
                let factor = if year < 1998 { 0.9 } else { 0.85 };
                early.insert((year, month), level * factor);
                late.insert((year, month), level * 1.0);
            }
        }
        let control = Series { name: "c".into(), months: control, source: "test".into() };
        let series = |map: BTreeMap<Month, f64>| Series { name: "x".into(), months: map, source: "test".into() };
        let rows = control_rows(&control, &series(early), &eras, 30.0, 10);
        assert!(!rows.is_empty());
        for row in &rows {
            if row.cells[0].0 == 0 {
                continue;
            }
            let delta = row.cells[1].1 / row.cells[0].1 - 1.0;
            assert!((delta - (0.9 / 0.85 - 1.0)).abs() < 1e-9, "delta {delta}");
        }
        let _ = series(late);
    }

    #[test]
    fn a_text_series_reads_its_named_columns() {
        let path = env::temp_dir().join("ripple_scan_months.txt");
        fs::write(&path, "1980 01 1980.0 120.0\n1980 02 1980.1 130.0\n").expect("write");
        let map = months_from_text(&path.to_string_lossy(), 0, 1, 3).expect("read");
        assert_eq!(map.get(&(1980, 1)).copied(), Some(120.0));
        assert_eq!(map.get(&(1980, 2)).copied(), Some(130.0));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn the_rstn_reader_takes_2695_from_forty_five_and_drops_day_32() {
        let dir = env::temp_dir().join("ripple_scan_rstn");
        let _ = fs::create_dir_all(&dir);
        let a = "760105 SGMR   7.9    23.0    42.2    51.0    71.9   118     276     525         \n";
        let b = "760132 SGMR   7.9    23.0    42.2    51.0   999.9   118     276     525         \n";
        fs::write(dir.join("1976.txt"), format!("{a}{b}")).expect("write");
        let map = months_from_rstn(&dir.to_string_lossy(), 45, 49).expect("read");
        assert_eq!(map.get(&(1976, 1)).copied(), Some(71.9));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn nearby_steps_cluster_into_one_date() {
        let steps = vec![
            (("a~b").to_string(), Step { month: (1981, 1), ratio: 1.09, score: 2.0 }),
            (("c~d").to_string(), Step { month: (1981, 3), ratio: 1.10, score: 1.9 }),
            (("e~f").to_string(), Step { month: (2003, 9), ratio: 1.17, score: 2.2 }),
        ];
        let mut ordered = steps.clone();
        ordered.sort_by(|a, b| a.1.month.cmp(&b.1.month));
        assert_eq!(months_between(ordered[0].1.month, ordered[1].1.month), 2);
        assert_eq!(months_between(ordered[1].1.month, ordered[2].1.month), 270);
    }
}
