use std::collections::HashMap;

use omegaflow::doppler::{parse_bin, write_bin};

const S_BAND_LO: f64 = 2.0e9;
const S_BAND_HI: f64 = 2.5e9;
const SPIKE_JUMP_HZ: f64 = 1.0e7;
const WINDOW: usize = 5;

fn median(mut v: Vec<f64>) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

fn percentile(mut v: Vec<f64>, p: f64) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(f64::total_cmp);
    let idx = ((v.len() as f64 - 1.0) * p).round() as usize;
    v[idx.min(v.len() - 1)]
}

fn clean(name: &str) {
    let raw_path = format!("data/{name}_doppler.bin");
    let Ok(bytes) = std::fs::read(&raw_path) else {
        eprintln!("{name}: doppler bin void ({raw_path})");
        return;
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("{name}: doppler bin parse void");
        return;
    };
    let n = records.len();

    let mut freq_map: HashMap<i64, usize> = HashMap::new();
    for r in &records {
        if r[2].is_finite() && r[2] > 0.0 {
            *freq_map.entry(r[2].round() as i64).or_insert(0) += 1;
        }
    }
    let mut band: Vec<i64> = freq_map
        .iter()
        .filter(|(v, _)| (**v as f64) >= S_BAND_LO && (**v as f64) <= S_BAND_HI)
        .map(|(v, _)| *v)
        .collect();
    band.sort_unstable();
    let band_span = band.last().copied().unwrap_or(0) - band.first().copied().unwrap_or(0);
    let distinct = freq_map.len();
    eprintln!(
        "{name}: FREQCY — {distinct} distinct values, {band_len} of them in the S-band window [2,0–2,5 GHz], S-band span {band_span} Hz",
        band_len = band.len()
    );

    let mut neg: Vec<f64> = Vec::new();
    let mut pos = 0usize;
    let mut zero = 0usize;
    let mut neg_count = 0usize;
    let mut neg_runs: Vec<usize> = Vec::new();
    let mut run_len = 0usize;
    for r in &records {
        let o = r[1];
        if o < 0.0 {
            neg.push(o);
            neg_count += 1;
            run_len += 1;
        } else if o == 0.0 {
            zero += 1;
            if run_len > 0 {
                neg_runs.push(run_len);
                run_len = 0;
            }
        } else {
            pos += 1;
            if run_len > 0 {
                neg_runs.push(run_len);
                run_len = 0;
            }
        }
    }
    if run_len > 0 {
        neg_runs.push(run_len);
    }
    neg_runs.sort_unstable();
    let all: Vec<f64> = records.iter().map(|r| r[1]).collect();
    eprintln!(
        "{name}: OBSVBL — positive {pos}, zero {zero}, negative {neg_count} ({:.1} %); negative median {:.3e} Hz, negative runs {} (longest {})",
        neg_count as f64 / n as f64 * 100.0,
        median(neg.clone()),
        neg_runs.len(),
        neg_runs.last().copied().unwrap_or(0)
    );
    eprintln!(
        "{name}: OBSVBL percentiles [0,1,50,90,99,100] = [{:.3e}, {:.3e}, {:.3e}, {:.3e}, {:.3e}, {:.3e}] Hz",
        percentile(all.clone(), 0.0),
        percentile(all.clone(), 0.01),
        percentile(all.clone(), 0.5),
        percentile(all.clone(), 0.9),
        percentile(all.clone(), 0.99),
        percentile(all.clone(), 1.0)
    );

    let mut clean_records: Vec<[f64; 6]> = Vec::with_capacity(n);
    for (i, r) in records.iter().enumerate() {
        let mut r = *r;
        let f = r[2];
        let plausible = (S_BAND_LO..=S_BAND_HI).contains(&f);
        if !plausible {
            let mut local_val = f64::NAN;
            for d in 1..=WINDOW {
                let lo = i.saturating_sub(d);
                let hi = (i + d).min(n - 1);
                for j in [lo, hi] {
                    if j != i {
                        let g = records[j][2];
                        if (S_BAND_LO..=S_BAND_HI).contains(&g) {
                            local_val = g;
                            break;
                        }
                    }
                }
                if local_val.is_finite() {
                    break;
                }
            }
            r[2] = if local_val.is_finite() {
                local_val
            } else {
                band[band.len() / 2] as f64
            };
        }
        clean_records.push(r);
    }
    let freq_corrected = clean_records
        .iter()
        .zip(records.iter())
        .filter(|(a, b)| a[2] != b[2])
        .count();
    eprintln!("{name}: FREQCY — {freq_corrected} out-of-line corrected to the local ramp value");

    let mut spike = vec![false; n];
    let mut spike_count = 0usize;
    for i in 1..n - 1 {
        let a = clean_records[i][1] - clean_records[i - 1][1];
        let b = clean_records[i + 1][1] - clean_records[i][1];
        if a.abs() > SPIKE_JUMP_HZ && b.abs() > SPIKE_JUMP_HZ && a * b < 0.0 {
            spike[i] = true;
            spike_count += 1;
        }
    }
    eprintln!("{name}: isolated corruption spikes discarded: {spike_count}");

    let mut local: Vec<f64> = Vec::with_capacity(n);
    for i in 0..n {
        let mut win: Vec<f64> = Vec::new();
        let lo = i.saturating_sub(WINDOW);
        let hi = (i + WINDOW + 1).min(n);
        for j in lo..hi {
            if j != i && !spike[j] {
                win.push(clean_records[j][1]);
            }
        }
        local.push(median(win));
    }
    let mut count_500k = 0usize;
    let mut count_1000 = 0usize;
    let mut count_misplaced = 0usize;
    for i in 0..n {
        if spike[i] || !local[i].is_finite() {
            continue;
        }
        let d = (clean_records[i][1] - local[i]).abs();
        if d > 7.5e3 && d < 9.2e3 {
            count_500k += 1;
        } else if d >= 500.0 && d < 1.0e5 && (d / 1000.0).round() * 1000.0 - d < 200.0 {
            count_1000 += 1;
        } else if d >= 1.0e5 && d <= 2.5e6 {
            count_misplaced += 1;
        }
    }
    eprintln!(
        "{name}: correction classes recognized — ±500k count (≈8,3 kHz): {count_500k}, 1000-Hz observable: {count_1000}, shifted counts (100 kHz–2 MHz): {count_misplaced} (named, correction open — needs the full model)"
    );

    let mut out: Vec<[f64; 6]> = Vec::with_capacity(n);
    let mut discarded = 0usize;
    for i in 0..n {
        if spike[i] {
            discarded += 1;
            continue;
        }
        out.push(clean_records[i]);
    }
    let out_path = format!("data/{name}_doppler_clean.bin");
    let bin = write_bin(&out);
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("{name}: write {out_path} void");
        return;
    }
    eprintln!(
        "{out_path}: {n} → {} records ({} discarded, {freq_corrected} FREQCY corrected)",
        out.len(),
        discarded
    );

    let raw_med = daily_medians(&records);
    let clean_med = daily_medians(&out);
    let mut lost: Vec<f64> = Vec::new();
    for (day, m) in &raw_med {
        if !clean_med.contains_key(day) {
            lost.push(*m);
        }
    }
    let mut delta: Vec<f64> = Vec::new();
    for (day, m) in &clean_med {
        if let Some(&r) = raw_med.get(day) {
            if (m - r).abs() > 1.0 {
                delta.push(m - r);
            }
        }
    }
    let dmed = median(delta.iter().map(|x| x.abs()).collect());
    let dmax = delta.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
    let lost_med = median(lost.clone());
    eprintln!(
        "{name}: daily-median audit — {r_days} → {c_days} days; {l} corruption-dominated days empty (median of their raw medians {lost_med:.3e} Hz), {d} days shifted (>1 Hz), median |Δ| {dmed:.3e} Hz, max |Δ| {dmax:.3e} Hz",
        r_days = raw_med.len(),
        c_days = clean_med.len(),
        l = lost.len(),
        d = delta.len(),
    );
}

fn daily_medians(records: &[[f64; 6]]) -> HashMap<i64, f64> {
    let mut groups: HashMap<i64, Vec<f64>> = HashMap::new();
    for r in records {
        let jd = 2451545.0 + r[0] / 86400.0;
        let day = (jd - 2440587.5).floor() as i64;
        if r[1].is_finite() && r[1] > 0.0 {
            groups.entry(day).or_default().push(r[1]);
        }
    }
    let mut out = HashMap::new();
    for (day, mut vals) in groups {
        vals.sort_by(f64::total_cmp);
        out.insert(day, vals[vals.len() / 2]);
    }
    out
}

fn main() {
    for name in ["pioneer10", "pioneer11"] {
        clean(name);
    }
}
