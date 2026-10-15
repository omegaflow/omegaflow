use omegaflow::intermagnet;
use omegaflow::substorm::{SubstormOnset, write_bin};

const DAY: f64 = 86400.0;
const HOUR: f64 = 3600.0;
const MINUTE: f64 = 60.0;
const MIN_SEPARATION_S: f64 = 1800.0;
const MAD_K: f64 = 6.0;
const ROBUST_SIGMA: f64 = 1.4826;

#[derive(Debug, PartialEq)]
struct Onset {
    t_unix: f64,
    peak: f64,
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn iso_utc(unix: f64) -> String {
    let total = (unix.max(0.0) / DAY).floor() as i64;
    let day_secs = unix.max(0.0) - total as f64 * DAY;
    let z = total + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    let hh = (day_secs / HOUR) as i64;
    let mm = ((day_secs - hh as f64 * HOUR) / MINUTE) as i64;
    let ss = (day_secs - hh as f64 * HOUR - mm as f64 * MINUTE) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn days_from_civil(y: i64, m: i64, d: i64) -> f64 {
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let jdn =
        d + (153 * (m + 12 * a - 3) + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    (jdn - 2440588) as f64
}

fn median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.total_cmp(b));
    let n = values.len();
    Some(if n % 2 == 1 {
        values[n / 2]
    } else {
        0.5 * (values[n / 2 - 1] + values[n / 2])
    })
}

fn median_mad(series: &[(f64, f64)]) -> Option<(f64, f64)> {
    let mut vals: Vec<f64> = series
        .iter()
        .map(|(_, v)| *v)
        .filter(|v| v.is_finite())
        .collect();
    let med = median(&mut vals)?;
    let mut dev: Vec<f64> = series
        .iter()
        .map(|(_, v)| (v - med).abs())
        .filter(|d| d.is_finite())
        .collect();
    let mad = median(&mut dev)?;
    Some((med, mad))
}

fn robust_threshold(series: &[(f64, f64)]) -> Option<f64> {
    let (med, mad) = median_mad(series)?;
    if !(mad > 0.0) {
        return None;
    }
    Some(med + MAD_K * ROBUST_SIGMA * mad)
}

fn detect_onsets(series: &[(f64, f64)], threshold: f64, min_separation_s: f64) -> Vec<Onset> {
    let mut onsets = Vec::new();
    let mut i = 0;
    while i < series.len() {
        if !(series[i].1.is_finite() && series[i].1 >= threshold) {
            i += 1;
            continue;
        }
        let mut peak = series[i].1;
        let mut peak_t = series[i].0;
        let mut last_above = i;
        let mut j = i + 1;
        while j < series.len() {
            if series[j].0 - series[last_above].0 >= min_separation_s {
                break;
            }
            if series[j].1.is_finite() && series[j].1 >= threshold {
                if series[j].1 > peak {
                    peak = series[j].1;
                    peak_t = series[j].0;
                }
                last_above = j;
            }
            j += 1;
        }
        onsets.push(Onset {
            t_unix: peak_t,
            peak,
        });
        i = j;
    }
    onsets
}

fn read_mcpherron(path: &str) -> Vec<f64> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("gic_onset_detector: {path} read void: {e}");
            std::process::exit(1);
        }
    };
    let mut out = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split_whitespace().take(6).collect();
        if cols.len() < 6 {
            continue;
        }
        let mut v = [0i64; 6];
        let mut ok = true;
        for (i, c) in cols.iter().enumerate() {
            match c.parse::<i64>() {
                Ok(x) => v[i] = x,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            out.push(
                days_from_civil(v[0], v[1], v[2]) * DAY
                    + v[3] as f64 * HOUR
                    + v[4] as f64 * MINUTE
                    + v[5] as f64,
            );
        }
    }
    out
}

fn nearest(sorted: &[f64], x: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let idx = sorted.partition_point(|&v| v < x);
    let mut best: Option<f64> = None;
    for k in [idx.saturating_sub(1), idx.min(sorted.len() - 1)] {
        let d = (sorted[k] - x).abs();
        let better = match best {
            Some(b) => d < b,
            None => true,
        };
        if better {
            best = Some(d);
        }
    }
    best
}

fn match_onsets(a: &[f64], b: &[f64], tol: f64) -> (usize, usize, Option<f64>) {
    let mut dts = Vec::new();
    let mut matched_a = 0usize;
    for &x in a {
        if let Some(dt) = nearest(b, x) {
            if dt <= tol {
                matched_a += 1;
                dts.push(dt);
            }
        }
    }
    let mut matched_b = 0usize;
    for &x in b {
        if let Some(dt) = nearest(a, x) {
            if dt <= tol {
                matched_b += 1;
            }
        }
    }
    (matched_a, matched_b, median(&mut dts))
}

fn ratio(num: usize, den: usize) -> String {
    if den == 0 {
        "n/a".to_string()
    } else {
        format!("{:.6}", num as f64 / den as f64)
    }
}

fn min_separation_of(args: &[String]) -> f64 {
    match arg_value(args, "--min-separation") {
        Some(s) => match s.parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => {
                eprintln!("--min-separation must be positive seconds");
                std::process::exit(2);
            }
        },
        None => MIN_SEPARATION_S,
    }
}

fn parse_day(s: &str) -> Option<f64> {
    let date = s.split('T').next()?;
    let mut it = date.split('-');
    let y: i64 = it.next()?.parse().ok()?;
    let m: i64 = it.next()?.parse().ok()?;
    let d: i64 = it.next()?.parse().ok()?;
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    Some(days_from_civil(y, m, d) * DAY)
}

fn window_of(args: &[String]) -> Option<(f64, f64)> {
    let lo = parse_day(&arg_value(args, "--window-start")?)?;
    let hi = parse_day(&arg_value(args, "--window-end")?)? + DAY - 1.0;
    if hi > lo { Some((lo, hi)) } else { None }
}

fn onsets_within(
    series: &[(f64, f64)],
    threshold: f64,
    min_sep: f64,
    lo: f64,
    hi: f64,
) -> Vec<Onset> {
    detect_onsets(series, threshold, min_sep)
        .into_iter()
        .filter(|o| o.t_unix >= lo && o.t_unix <= hi)
        .collect()
}

fn count_onsets(series: &[(f64, f64)], threshold: f64, min_sep: f64, lo: f64, hi: f64) -> usize {
    let mut count = 0usize;
    let mut i = 0usize;
    while i < series.len() {
        if !(series[i].1.is_finite() && series[i].1 >= threshold) {
            i += 1;
            continue;
        }
        let mut peak_idx = i;
        let mut peak_t = series[i].0;
        let mut last_above = i;
        let mut j = i + 1;
        while j < series.len() {
            if series[j].0 - series[last_above].0 >= min_sep {
                break;
            }
            if series[j].1.is_finite() && series[j].1 >= threshold {
                if series[j].1 > series[peak_idx].1 {
                    peak_idx = j;
                    peak_t = series[j].0;
                }
                last_above = j;
            }
            j += 1;
        }
        if peak_t >= lo && peak_t <= hi {
            count += 1;
        }
        i = j;
    }
    count
}

fn calibrate_threshold(
    series: &[(f64, f64)],
    lo: f64,
    hi: f64,
    min_sep: f64,
    target_per_day: f64,
) -> Option<f64> {
    let span_days = (hi - lo) / DAY;
    if !(span_days > 0.0) || !(target_per_day > 0.0) {
        return None;
    }
    let target = target_per_day * span_days;
    let mut vals: Vec<f64> = series
        .iter()
        .filter(|(t, v)| v.is_finite() && *t >= lo && *t <= hi)
        .map(|(_, v)| *v)
        .collect();
    if vals.is_empty() {
        return None;
    }
    vals.sort_by(|a, b| a.total_cmp(b));
    let step = vals.len() / 64;
    let mut best_t = vals[0];
    let mut best_d = f64::INFINITY;
    let last = vals.len() - 1;
    for (k, &t) in vals.iter().enumerate() {
        let sampled = step == 0 || k % step == 0;
        if !sampled && k != last {
            continue;
        }
        let n = count_onsets(series, t, min_sep, lo, hi) as f64;
        let d = (n - target).abs();
        if d < best_d {
            best_d = d;
            best_t = t;
        }
    }
    Some(best_t)
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

fn percentile(values: &mut [f64], q: f64) -> f64 {
    values.sort_by(|a, b| a.total_cmp(b));
    let idx = (((values.len() - 1) as f64) * q).round() as usize;
    values[idx]
}

fn shifted_within(sorted: &[f64], offset_s: f64, lo: f64, span: f64) -> Vec<f64> {
    let mut out: Vec<f64> = sorted
        .iter()
        .map(|&t| lo + (t - lo + offset_s).rem_euclid(span))
        .collect();
    out.sort_by(|a, b| a.total_cmp(b));
    out
}

fn run_null(mcp: &[f64], cand: &[f64], lo: f64, hi: f64, tols: &[i64], n: usize) {
    let span = hi - lo;
    if !(span > 0.0) {
        println!("null: the comparison window carries no extent — no surrogate (0 honored)");
        return;
    }
    if n == 0 {
        eprintln!("--null needs a positive surrogate count");
        std::process::exit(2);
    }
    if mcp.is_empty() || cand.is_empty() {
        println!("null: witness or detector set empty — no null (0 honored)");
        return;
    }
    println!("null surrogates={n} span_s={span:.3}");
    for &tol_min in tols {
        let tol = tol_min as f64 * MINUTE;
        let (_, hit_cand_real, _) = match_onsets(mcp, cand, tol);
        let (_, hit_mcp_real, _) = match_onsets(cand, mcp, tol);
        let real_prec = hit_cand_real as f64 / cand.len() as f64;
        let real_rec = hit_mcp_real as f64 / mcp.len() as f64;
        let mut precs: Vec<f64> = Vec::with_capacity(n);
        let mut recs: Vec<f64> = Vec::with_capacity(n);
        for i in 0..n {
            let offset = ((i as f64 + 0.5) / n as f64) * span;
            let shifted = shifted_within(cand, offset, lo, span);
            let (_, hit_cand, _) = match_onsets(mcp, &shifted, tol);
            let (_, hit_mcp, _) = match_onsets(&shifted, mcp, tol);
            precs.push(hit_cand as f64 / cand.len() as f64);
            recs.push(hit_mcp as f64 / mcp.len() as f64);
        }
        let prec_ge = precs.iter().filter(|p| **p >= real_prec).count();
        let rec_ge = recs.iter().filter(|r| **r >= real_rec).count();
        let prec_mean = mean(&precs);
        let rec_mean = mean(&recs);
        let prec_p95 = percentile(&mut precs, 0.95);
        let rec_p95 = percentile(&mut recs, 0.95);
        println!(
            "null tol_min={tol_min} real_prec={real_prec:.6} null_prec_mean={prec_mean:.6} null_prec_p95={prec_p95:.6} prec_ge={prec_ge} real_rec={real_rec:.6} null_rec_mean={rec_mean:.6} null_rec_p95={rec_p95:.6} rec_ge={rec_ge}"
        );
    }
}

fn run_sweep(series: &[(f64, f64)], station: &str, csv: &str, args: &[String]) {
    let Some((med, mad)) = median_mad(series) else {
        eprintln!("gic_onset_detector: series carries no finite sample");
        std::process::exit(1);
    };
    if !(mad > 0.0) {
        println!(
            "gic_onset_detector{station} sweep — the series carries no variation (0 honored: no onset)"
        );
        return;
    }
    let sigma = ROBUST_SIGMA * mad;
    let ks: Vec<f64> = csv
        .split(',')
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .filter(|k| *k > 0.0)
        .collect();
    if ks.is_empty() {
        eprintln!("--sweep-k needs a positive CSV of multipliers");
        std::process::exit(2);
    }
    let min_separation_s = min_separation_of(args);
    let lo = series[0].0;
    let hi = series[series.len() - 1].0;
    let mcp: Vec<f64> = match arg_value(args, "--mcpherron") {
        Some(p) => read_mcpherron(&p)
            .into_iter()
            .filter(|&t| t >= lo && t <= hi)
            .collect(),
        None => Vec::new(),
    };
    let tol = match arg_value(args, "--tol-min") {
        Some(csv) => {
            let mut t = 15.0 * MINUTE;
            if let Some(v) = csv
                .split(',')
                .filter_map(|s| s.trim().parse::<i64>().ok())
                .find(|v| *v > 0)
            {
                t = v as f64 * MINUTE;
            }
            t
        }
        None => 15.0 * MINUTE,
    };
    let n_surr: usize = match arg_value(args, "--null") {
        Some(s) => match s.parse::<usize>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("--null must be a non-negative surrogate count");
                std::process::exit(2);
            }
        },
        None => 0,
    };
    let span = hi - lo;
    println!(
        "sweep{station} samples={} median={med:.4} mad={mad:.4} min_separation={min_separation_s:.0}s",
        series.len()
    );
    for k in ks {
        let threshold = med + k * sigma;
        let onsets = detect_onsets(series, threshold, min_separation_s);
        let cand: Vec<f64> = onsets.iter().map(|o| o.t_unix).collect();
        if mcp.is_empty() {
            println!(
                "sweep k={k} threshold={threshold:.4} onsets={}",
                onsets.len()
            );
            continue;
        }
        let (hit_mcp, hit_cand, med_dt) = match_onsets(&mcp, &cand, tol);
        if n_surr == 0 {
            println!(
                "sweep k={k} threshold={threshold:.4} onsets={} prec_onset={} rec_mcp={} median_dt_s={}",
                onsets.len(),
                ratio(hit_cand, cand.len()),
                ratio(hit_mcp, mcp.len()),
                match med_dt {
                    Some(x) => format!("{x:.3}"),
                    None => "n/a".to_string(),
                }
            );
            continue;
        }
        let real_prec = if cand.is_empty() {
            0.0
        } else {
            hit_cand as f64 / cand.len() as f64
        };
        let real_rec = if mcp.is_empty() {
            0.0
        } else {
            hit_mcp as f64 / mcp.len() as f64
        };
        let mut precs: Vec<f64> = Vec::with_capacity(n_surr);
        let mut recs: Vec<f64> = Vec::with_capacity(n_surr);
        for i in 0..n_surr {
            let offset = ((i as f64 + 0.5) / n_surr as f64) * span;
            let shifted = shifted_within(&cand, offset, lo, span);
            let (sh_mcp, sh_cand, _) = match_onsets(&mcp, &shifted, tol);
            precs.push(if cand.is_empty() {
                0.0
            } else {
                sh_cand as f64 / cand.len() as f64
            });
            recs.push(if mcp.is_empty() {
                0.0
            } else {
                sh_mcp as f64 / mcp.len() as f64
            });
        }
        let prec_ge = precs.iter().filter(|p| **p >= real_prec).count();
        let rec_ge = recs.iter().filter(|r| **r >= real_rec).count();
        println!(
            "sweep k={k} threshold={threshold:.4} onsets={} prec_onset={} rec_mcp={} prec_ge={prec_ge} rec_ge={rec_ge}",
            onsets.len(),
            ratio(hit_cand, cand.len()),
            ratio(hit_mcp, mcp.len())
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = arg_value(&args, "--series") else {
        eprintln!("gic_onset_detector needs --series <IMDT bin> (per-bucket peak |dB/dt|, nT/min)");
        std::process::exit(2);
    };
    let mut station = String::new();
    if let Some(s) = arg_value(&args, "--station") {
        station = format!(" station={s}");
    }
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("gic_onset_detector: {path} read void: {e}");
            std::process::exit(1);
        }
    };
    let Some(parsed) = intermagnet::parse_bin(&bytes) else {
        eprintln!("gic_onset_detector: {path} is not an IMDT bin");
        std::process::exit(1);
    };
    let series: Vec<(f64, f64)> = parsed.into_iter().map(|(t, v, _)| (t, v)).collect();
    if series.is_empty() {
        eprintln!("gic_onset_detector: {path} carries no sample");
        std::process::exit(1);
    }
    if let Some(csv) = arg_value(&args, "--sweep-k") {
        run_sweep(&series, &station, &csv, &args);
        return;
    }
    let min_separation_s = min_separation_of(&args);
    let (lo, hi) = match window_of(&args) {
        Some(w) => w,
        None => (series[0].0, series[series.len() - 1].0),
    };
    let threshold = if let Some(s) = arg_value(&args, "--target-yield") {
        let per_day = match s.parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => {
                eprintln!("--target-yield must be a positive onsets-per-day count");
                std::process::exit(2);
            }
        };
        match calibrate_threshold(&series, lo, hi, min_separation_s, per_day) {
            Some(v) => {
                println!(
                    "gic_onset_detector{station} target_yield={per_day:.4}/day window={}..{} -> threshold={v:.4}",
                    iso_utc(lo),
                    iso_utc(hi)
                );
                v
            }
            None => {
                eprintln!("--target-yield needs a positive window span and a varying series");
                std::process::exit(2);
            }
        }
    } else {
        match arg_value(&args, "--threshold") {
            Some(t) => match t.parse::<f64>() {
                Ok(v) if v.is_finite() && v > 0.0 => v,
                _ => {
                    eprintln!("--threshold must be a positive number (nT/min)");
                    std::process::exit(2);
                }
            },
            None => match robust_threshold(&series) {
                Some(v) => v,
                None => {
                    println!(
                        "gic_onset_detector{station} samples={} threshold=none — the series carries no variation (0 honored: no onset)",
                        series.len()
                    );
                    return;
                }
            },
        }
    };
    let onsets = onsets_within(&series, threshold, min_separation_s, lo, hi);
    println!(
        "gic_onset_detector{station} samples={} threshold={threshold:.4} nT/min min_separation={min_separation_s:.0}s window={}..{} onsets={}",
        series.len(),
        iso_utc(lo),
        iso_utc(hi),
        onsets.len()
    );
    for o in &onsets {
        println!("onset {} peak {:.4} nT/min", iso_utc(o.t_unix), o.peak);
    }
    if let Some(out) = arg_value(&args, "--out") {
        let records: Vec<SubstormOnset> = onsets
            .iter()
            .map(|o| SubstormOnset {
                t_unix: o.t_unix,
                mlt: None,
                mlat: None,
                glon: None,
                glat: None,
            })
            .collect();
        match std::fs::write(&out, write_bin(&records)) {
            Ok(()) => println!("wrote {} onset(s) → {out}", records.len()),
            Err(e) => {
                eprintln!("gic_onset_detector: {out} write void: {e}");
                std::process::exit(1);
            }
        }
    }
    if let Some(mcp_path) = arg_value(&args, "--mcpherron") {
        let mcp: Vec<f64> = read_mcpherron(&mcp_path)
            .into_iter()
            .filter(|&t| t >= lo && t <= hi)
            .collect();
        let cand: Vec<f64> = onsets.iter().map(|o| o.t_unix).collect();
        let tols: Vec<i64> = match arg_value(&args, "--tol-min") {
            Some(csv) => csv
                .split(',')
                .filter_map(|s| s.trim().parse::<i64>().ok())
                .filter(|v| *v > 0)
                .collect(),
            None => vec![5, 15, 30],
        };
        println!(
            "compare window {} .. {}, mcpherron {} onsets, detector {} onsets",
            iso_utc(lo),
            iso_utc(hi),
            mcp.len(),
            cand.len()
        );
        for &tol_min in &tols {
            let tol = tol_min as f64 * MINUTE;
            let (hit_mcp, hit_cand, med) = match_onsets(&mcp, &cand, tol);
            println!(
                "compare tol_min={tol_min} n_mcp={} n_onset={} hit_mcp={} hit_onset={} prec_onset={} rec_mcp={} median_dt_s={}",
                mcp.len(),
                cand.len(),
                hit_mcp,
                hit_cand,
                ratio(hit_cand, cand.len()),
                ratio(hit_mcp, mcp.len()),
                match med {
                    Some(x) => format!("{x:.3}"),
                    None => "n/a".to_string(),
                }
            );
        }
        if let Some(s) = arg_value(&args, "--null") {
            let n = match s.parse::<usize>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("--null must be a non-negative surrogate count");
                    std::process::exit(2);
                }
            };
            run_null(&mcp, &cand, lo, hi, &tols, n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn series(values: &[(f64, f64)]) -> Vec<(f64, f64)> {
        values.to_vec()
    }

    #[test]
    fn a_single_crossing_yields_one_onset_at_its_first_bucket() {
        let s = series(&[(0.0, 0.1), (3600.0, 0.2), (7200.0, 5.0), (10800.0, 0.3)]);
        assert_eq!(
            detect_onsets(&s, 1.0, 1800.0),
            vec![Onset {
                t_unix: 7200.0,
                peak: 5.0
            }]
        );
    }

    #[test]
    fn a_run_stays_one_event_and_its_peak_is_the_maximum() {
        let s = series(&[(0.0, 5.0), (600.0, 9.0), (1200.0, 3.0), (1800.0, 7.0)]);
        let onsets = detect_onsets(&s, 1.0, 1800.0);
        assert_eq!(onsets.len(), 1);
        assert_eq!(onsets[0].t_unix, 600.0);
        assert_eq!(onsets[0].peak, 9.0);
    }

    #[test]
    fn a_quiet_gap_longer_than_the_refractory_window_splits_the_events() {
        let s = series(&[(0.0, 5.0), (7200.0, 5.0)]);
        let onsets = detect_onsets(&s, 1.0, 1800.0);
        assert_eq!(onsets.len(), 2);
        assert_eq!(onsets[1].t_unix, 7200.0);
    }

    #[test]
    fn a_sub_threshold_series_carries_no_onset() {
        let s = series(&[(0.0, 0.1), (3600.0, 0.2), (7200.0, 0.3)]);
        assert!(detect_onsets(&s, 1.0, 1800.0).is_empty());
    }

    #[test]
    fn a_flat_series_has_no_robust_threshold() {
        let s = series(&[(0.0, 1.0), (3600.0, 1.0), (7200.0, 1.0)]);
        assert_eq!(robust_threshold(&s), None);
    }

    #[test]
    fn the_robust_threshold_derives_from_the_live_series() {
        let s = series(&[(0.0, 1.0), (3600.0, 2.0), (7200.0, 100.0)]);
        let thr = robust_threshold(&s).expect("a varying series carries a threshold");
        assert!(thr > 2.0 && thr < 100.0);
    }

    #[test]
    fn the_epoch_formats_to_iso_utc() {
        assert_eq!(iso_utc(0.0), "1970-01-01T00:00:00Z");
        assert_eq!(iso_utc(1_700_000_000.0), "2023-11-14T22:13:20Z");
    }

    #[test]
    fn matching_counts_both_directions_and_medians_the_gaps() {
        let (hit_a, hit_b, med) = match_onsets(&[0.0, 100.0, 1000.0], &[10.0, 90.0, 5000.0], 60.0);
        assert_eq!(hit_a, 2);
        assert_eq!(hit_b, 2);
        assert_eq!(med, Some(10.0));
    }

    #[test]
    fn the_civil_epoch_matches_the_measured_reference() {
        assert_eq!(days_from_civil(1970, 1, 1) * DAY, 0.0);
        assert_eq!(days_from_civil(1966, 1, 1) * DAY, -126230400.0);
    }

    #[test]
    fn a_circular_shift_moves_onsets_and_wraps_inside_the_window() {
        let shifted = shifted_within(&[0.0, 100.0, 900.0], 200.0, 0.0, 1000.0);
        assert_eq!(shifted, vec![100.0, 200.0, 300.0]);
    }

    #[test]
    fn a_circular_shift_preserves_the_count_and_the_internal_gaps() {
        let cand = [10.0, 500.0, 1500.0, 4000.0];
        let shifted = shifted_within(&cand, 137.0, 0.0, 8000.0);
        assert_eq!(shifted.len(), cand.len());
        let gaps = |s: &[f64]| -> Vec<f64> { s.windows(2).map(|w| w[1] - w[0]).collect() };
        assert_eq!(gaps(&shifted), gaps(&cand));
    }

    #[test]
    fn the_null_mean_and_percentile_are_measured_from_the_surrogates() {
        assert_eq!(mean(&[0.0, 0.5, 1.0]), 0.5);
        let mut v = [0.0, 1.0, 2.0, 3.0, 4.0];
        assert_eq!(percentile(&mut v, 0.95), 4.0);
        let mut w = [10.0, 0.0, 5.0];
        assert_eq!(percentile(&mut w, 0.0), 0.0);
    }

    #[test]
    fn a_day_parses_to_the_civil_epoch() {
        assert_eq!(parse_day("1970-01-01"), Some(0.0));
        assert_eq!(parse_day("1970-01-02"), Some(DAY));
        assert_eq!(
            parse_day("2024-12-31T23:59:00Z"),
            Some(days_from_civil(2024, 12, 31) * DAY)
        );
        assert_eq!(parse_day("not-a-date"), None);
        assert_eq!(parse_day("2024-13-01"), None);
    }

    #[test]
    fn the_window_takes_the_start_day_and_covers_the_end_day() {
        let args = vec![
            "--window-start".to_string(),
            "2000-01-01".to_string(),
            "--window-end".to_string(),
            "2000-01-02".to_string(),
        ];
        let (lo, hi) = window_of(&args).expect("a well-formed window");
        assert_eq!(lo, days_from_civil(2000, 1, 1) * DAY);
        assert_eq!(hi, lo + 2.0 * DAY - 1.0);
        assert_eq!(window_of(&[]), None);
    }

    #[test]
    fn onsets_within_keeps_only_the_window() {
        let s = series(&[
            (0.0, 5.0),
            (2000.0, 0.0),
            (4000.0, 5.0),
            (6000.0, 0.0),
            (8000.0, 5.0),
        ]);
        let all = detect_onsets(&s, 1.0, 1800.0);
        assert_eq!(all.len(), 3);
        let windowed = onsets_within(&s, 1.0, 1800.0, 3000.0, 7000.0);
        assert_eq!(windowed.len(), 1);
        assert_eq!(windowed[0].t_unix, 4000.0);
        assert_eq!(count_onsets(&s, 1.0, 1800.0, 3000.0, 7000.0), 1);
    }

    #[test]
    fn the_calibrated_threshold_meets_the_yield() {
        let s = series(&[
            (0.0, 0.0),
            (3600.0, 10.0),
            (7200.0, 0.0),
            (86400.0, 0.0),
            (90000.0, 10.0),
            (93600.0, 0.0),
        ]);
        let (lo, hi) = (0.0, 2.0 * DAY);
        let thr = calibrate_threshold(&s, lo, hi, 1800.0, 1.0).expect("a calibratable series");
        let n = count_onsets(&s, thr, 1800.0, lo, hi);
        assert_eq!(n, 2);
    }
}
