use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use omegaflow::atdf::parse_resid_bin;
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR_AGC: f64 = -2560.0;
const LOUD_HZ: f64 = 1.0;
const LOUD_SAMPLE_HZ: f64 = 1.0;
const ROBUST_N: usize = 30;
const PASS_GAP_S: f64 = 600.0;
const STATIONS: [i64; 3] = [14, 43, 63];
const MODES: [i64; 3] = [1, 2, 3];
const N_SURR: usize = 200;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

fn unix_day_float(tdb: f64) -> f64 {
    tdb / DAY_S + 10957.5
}

fn day_key(tdb: f64) -> i64 {
    unix_day_float(tdb).round() as i64
}

fn hour_of_day(tdb: f64) -> f64 {
    unix_day_float(tdb).rem_euclid(1.0) * 24.0
}

fn civil_date(day: i64) -> String {
    match civil_from_days(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day{day}"),
    }
}

fn rms_of(sum: f64, sum2: f64, n: usize) -> f64 {
    if n == 0 {
        return f64::NAN;
    }
    let m = sum / n as f64;
    let v = (sum2 / n as f64 - m * m).max(0.0);
    v.sqrt()
}

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn ls_power(times: &[f64], vals: &[f64], f: f64) -> f64 {
    let m = times.len() as f64;
    if m < 5.0 {
        return 0.0;
    }
    let vmean = vals.iter().sum::<f64>() / m;
    let mut s = 0.0;
    let mut c = 0.0;
    for &t in times {
        let ph = std::f64::consts::TAU * f * t;
        s += ph.sin();
        c += ph.cos();
    }
    s /= m;
    c /= m;
    let mut ss = 0.0;
    let mut cc = 0.0;
    let mut sc = 0.0;
    let mut sy = 0.0;
    let mut cy = 0.0;
    for (i, &t) in times.iter().enumerate() {
        let ph = std::f64::consts::TAU * f * t;
        let ds = ph.sin() - s;
        let dc = ph.cos() - c;
        let dv = vals[i] - vmean;
        ss += ds * ds;
        cc += dc * dc;
        sc += ds * dc;
        sy += ds * dv;
        cy += dc * dv;
    }
    let det = ss * cc - sc * sc;
    if det.abs() <= 1e-300 {
        return 0.0;
    }
    let a = (sy * cc - cy * sc) / det;
    let b = (cy * ss - sy * sc) / det;
    let ss_reg = a * sy + b * cy;
    let tss = vals.iter().map(|v| (v - vmean) * (v - vmean)).sum::<f64>();
    if tss <= 0.0 {
        return 0.0;
    }
    (ss_reg / tss).clamp(0.0, 1.0)
}

fn ls_best(times: &[f64], vals: &[f64], flo: f64, fhi: f64, step: f64) -> (f64, f64) {
    let mut best = (f64::NAN, 0.0f64);
    let mut f = flo;
    while f <= fhi + step * 0.5 {
        let p = ls_power(times, vals, f);
        if p > best.1 {
            best = (f, p);
        }
        f += step;
    }
    best
}

fn shuffle(v: &mut [f64], rng: &mut u64) {
    for i in (1..v.len()).rev() {
        let j = ((next_rng(rng) * (i as f64 + 1.0)) as usize).min(i);
        v.swap(i, j);
    }
}

fn circ_stats(hours: &[f64]) -> (usize, f64, f64, f64) {
    let n = hours.len();
    if n == 0 {
        return (0, f64::NAN, f64::NAN, f64::NAN);
    }
    let mut sx = 0.0;
    let mut sy = 0.0;
    for &h in hours {
        let ph = std::f64::consts::TAU * h / 24.0;
        sx += ph.cos();
        sy += ph.sin();
    }
    let r = (sx * sx + sy * sy).sqrt() / n as f64;
    let mean = sy.atan2(sx) * 24.0 / std::f64::consts::TAU;
    let mean = if mean < 0.0 { mean + 24.0 } else { mean };
    let rayleigh_p = (-(n as f64) * r * r).exp();
    (n, mean, r, rayleigh_p)
}

fn bin24(hours: &[f64]) -> [usize; 24] {
    let mut b = [0usize; 24];
    for &h in hours {
        let k = ((h / 24.0) * 24.0).floor() as usize;
        let k = if k >= 24 { 23 } else { k };
        b[k] += 1;
    }
    b
}

fn fmt_bins(b: &[usize; 24]) -> String {
    (0..24)
        .map(|i| format!("{i}:{0}", b[i]))
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "/tmp/opencode/galileo_floor_subday_clock_report.txt".to_string(),
    };
    let Ok(bytes) = fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    drop(bytes);

    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String| {
        println!("{s}");
        out.push(s);
    };

    push(format!(
        "galileo floor sub-day clock probe: {} resid samples",
        recs.len()
    ));
    push("cell = (station, mode, day-key); floor = strength -2560; lock (|resid|>1000) excluded; robust day = n>=30; loud = day RMS>=1 Hz; loud sample = |resid - daymean| >= 1 Hz (named gate); pass = contiguous floor samples, gap<=600 s".to_string());

    let mut cell: BTreeMap<(i64, i64, i64), (f64, f64, usize)> = BTreeMap::new();
    let mut n_lock = 0usize;
    for r in &recs {
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        let st = r[2] as i64;
        let mo = r[3] as i64;
        if !STATIONS.contains(&st) || !MODES.contains(&mo) {
            continue;
        }
        if r[7] != FLOOR_AGC {
            continue;
        }
        let day = day_key(r[0]);
        let e = cell.entry((st, mo, day)).or_insert((0.0, 0.0, 0));
        e.0 += resid;
        e.1 += resid * resid;
        e.2 += 1;
    }
    push(format!(
        "lock/non-finite excluded {n_lock}; floor cells {}",
        cell.len()
    ));

    let mut rows: Vec<(i64, i64, i64, f64, f64, usize)> = Vec::new();
    for (&(st, mo, day), &(sum, sum2, n)) in &cell {
        let rms = rms_of(sum, sum2, n);
        if !rms.is_finite() || n == 0 {
            continue;
        }
        rows.push((st, mo, day, rms, sum / n as f64, n));
    }
    rows.sort_by_key(|r| (r.1, r.0, r.2));

    push("\n== 0. reproduction: per (mode, station) robust day series, loud cells, flips ==".to_string());
    let mut n_robust_all = 0usize;
    let mut n_loud_all = 0usize;
    let mut n_flip_all = 0usize;
    let mut series_map: BTreeMap<(i64, i64), Vec<(i64, f64)>> = BTreeMap::new();
    for mo in MODES {
        for st in STATIONS {
            let srows: Vec<&(i64, i64, i64, f64, f64, usize)> = rows
                .iter()
                .filter(|r| r.1 == mo && r.0 == st && r.5 >= ROBUST_N)
                .collect();
            let loud = srows.iter().filter(|r| r.3 >= LOUD_HZ).count();
            let quiet = srows.len() - loud;
            let mut flips = 0usize;
            for w in srows.windows(2) {
                if w[1].2 - w[0].2 == 1 {
                    let la = w[0].3 >= LOUD_HZ;
                    let lb = w[1].3 >= LOUD_HZ;
                    if la != lb {
                        flips += 1;
                    }
                }
            }
            n_robust_all += srows.len();
            n_loud_all += loud;
            n_flip_all += flips;
            push(format!(
                "mode {mo} st{st}: robust {} (loud {loud}, quiet {quiet}) flips {flips}",
                srows.len()
            ));
            series_map.insert(
                (mo, st),
                srows.iter().map(|r| (r.2, r.3)).collect::<Vec<(i64, f64)>>(),
            );
        }
    }
    push(format!(
        "TOTAL robust {n_robust_all}; loud cells {n_loud_all}; flips {n_flip_all}"
    ));

    let loud_cells: Vec<(i64, i64, i64, f64, f64, usize)> = rows
        .iter()
        .filter(|r| r.5 >= ROBUST_N && r.3 >= LOUD_HZ)
        .copied()
        .collect();
    push(format!(
        "loud day-cell set for sub-day analysis: n = {}",
        loud_cells.len()
    ));

    let loud_keys: BTreeSet<(i64, i64, i64)> =
        loud_cells.iter().map(|r| (r.0, r.1, r.2)).collect();
    let quiet_cells: Vec<(i64, i64, i64)> = rows
        .iter()
        .filter(|r| r.5 >= ROBUST_N && r.3 < LOUD_HZ)
        .map(|r| (r.0, r.1, r.2))
        .collect();
    let quiet_keys: BTreeSet<(i64, i64, i64)> = quiet_cells.iter().copied().collect();

    let mut loud_samples: BTreeMap<(i64, i64, i64), Vec<(f64, f64)>> = BTreeMap::new();
    let mut quiet_first: BTreeMap<(i64, i64, i64), f64> = BTreeMap::new();
    for r in &recs {
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            continue;
        }
        let st = r[2] as i64;
        let mo = r[3] as i64;
        if !STATIONS.contains(&st) || !MODES.contains(&mo) {
            continue;
        }
        if r[7] != FLOOR_AGC {
            continue;
        }
        let key = (st, mo, day_key(r[0]));
        if loud_keys.contains(&key) {
            loud_samples.entry(key).or_default().push((r[0], resid));
        } else if quiet_keys.contains(&key) {
            let e = quiet_first.entry(key).or_insert(f64::INFINITY);
            if r[0] < *e {
                *e = r[0];
            }
        }
    }
    drop(recs);
    for v in loud_samples.values_mut() {
        v.sort_by(|a, b| a.0.total_cmp(&b.0));
    }

    push("\n== A. sub-day pass structure per loud day-cell ==".to_string());
    push("per loud cell: date | mX stY | dayRMS | n | passes (startUTC- endUTC dur-min nsamp nloud) | loud-windowUTC".to_string());

    let mut run_start_hours: Vec<f64> = Vec::new();
    let mut loud_onset_hours: Vec<f64> = Vec::new();
    let mut loud_onset_min_after_start: Vec<f64> = Vec::new();
    let mut cell_first_hours: Vec<f64> = Vec::new();
    let mut per_series_onset: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    let mut n_cells_with_loud = 0usize;
    let mut n_runs_total = 0usize;

    for mo in MODES {
        for st in STATIONS {
            let cells_of: Vec<&(i64, i64, i64, f64, f64, usize)> = loud_cells
                .iter()
                .filter(|r| r.1 == mo && r.0 == st)
                .collect();
            for rc in cells_of {
                let key = (rc.0, rc.1, rc.2);
                let Some(samples) = loud_samples.get(&key) else {
                    continue;
                };
                if samples.is_empty() {
                    continue;
                }
                let daymean = rc.4;
                let mut runs: Vec<(usize, usize)> = Vec::new();
                let mut i = 0usize;
                while i < samples.len() {
                    let mut j = i + 1;
                    while j < samples.len() && samples[j].0 - samples[j - 1].0 <= PASS_GAP_S {
                        j += 1;
                    }
                    runs.push((i, j));
                    i = j;
                }
                let mut loud_in_day = 0usize;
                let mut first_loud_tdb: Option<f64> = None;
                let mut last_loud_tdb: Option<f64> = None;
                for s in samples {
                    if (s.1 - daymean).abs() >= LOUD_SAMPLE_HZ {
                        loud_in_day += 1;
                        if first_loud_tdb.is_none() {
                            first_loud_tdb = Some(s.0);
                        }
                        last_loud_tdb = Some(s.0);
                    }
                }
                let mut run_parts: Vec<String> = Vec::new();
                for (a, b) in &runs {
                    n_runs_total += 1;
                    let t0 = samples[*a].0;
                    let t1 = samples[b - 1].0;
                    let dur_min = (t1 - t0) / 60.0;
                    let h0 = hour_of_day(t0);
                    let h1 = hour_of_day(t1);
                    run_start_hours.push(h0);
                    let mut nl = 0usize;
                    for s in &samples[*a..*b] {
                        if (s.1 - daymean).abs() >= LOUD_SAMPLE_HZ {
                            nl += 1;
                        }
                    }
                    run_parts.push(format!(
                        "{h0:4.1}-{h1:4.1}h/{dur_min:4.0}m/{}/{}",
                        b - a,
                        nl
                    ));
                }
                if let Some(tl) = first_loud_tdb {
                    loud_onset_hours.push(hour_of_day(tl));
                    n_cells_with_loud += 1;
                    let st_h = samples[0].0;
                    loud_onset_min_after_start.push((tl - st_h) / 60.0);
                    per_series_onset.entry((mo, st)).or_default().push(hour_of_day(tl));
                }
                let cell_start = samples[0].0;
                cell_first_hours.push(hour_of_day(cell_start));
                let loud_w = match (first_loud_tdb, last_loud_tdb) {
                    (Some(t0), Some(t1)) => format!("{:.2}..{:.2}", hour_of_day(t0), hour_of_day(t1)),
                    _ => "none".to_string(),
                };
                push(format!(
                    "{} | m{mo} st{st} | rms {:.3} | n {} | passes {} | nloud {} | window {}",
                    civil_date(rc.2),
                    rc.3,
                    rc.5,
                    run_parts.join(" ; "),
                    loud_in_day,
                    loud_w
                ));
            }
        }
    }
    let mut n_frac_ge90 = 0usize;
    let mut n_frac_10_90 = 0usize;
    let mut n_frac_lt10 = 0usize;
    let mut n_single_pass = 0usize;
    let mut n_multi_pass = 0usize;
    for (mo, st) in MODES.iter().flat_map(|m| STATIONS.iter().map(move |s| (*m, *s))) {
        for rc in loud_cells.iter().filter(|r| r.1 == mo && r.0 == st) {
            let key = (rc.0, rc.1, rc.2);
            let Some(samples) = loud_samples.get(&key) else {
                continue;
            };
            if samples.is_empty() {
                continue;
            }
            let daymean = rc.4;
            let nl = samples
                .iter()
                .filter(|s| (s.1 - daymean).abs() >= LOUD_SAMPLE_HZ)
                .count();
            let frac = nl as f64 / samples.len() as f64;
            if frac >= 0.9 {
                n_frac_ge90 += 1;
            } else if frac >= 0.1 {
                n_frac_10_90 += 1;
            } else {
                n_frac_lt10 += 1;
            }
            let mut runs = 0usize;
            let mut i = 0usize;
            while i < samples.len() {
                let mut j = i + 1;
                while j < samples.len() && samples[j].0 - samples[j - 1].0 <= PASS_GAP_S {
                    j += 1;
                }
                runs += 1;
                i = j;
            }
            if runs == 1 {
                n_single_pass += 1;
            } else {
                n_multi_pass += 1;
            }
        }
    }
    push(format!(
        "loud day-cells n {} ({} with >=1 loud sample); sample runs total {n_runs_total}; loud-sample fraction of day >=0.9: {n_frac_ge90}, 0.1..0.9: {n_frac_10_90}, <0.1: {n_frac_lt10}; single-pass cells {n_single_pass}, multi-pass {n_multi_pass}",
        loud_cells.len(),
        n_cells_with_loud
    ));

    push("\n== B. time-of-day folding of the loud episodes (0..24 h) ==".to_string());
    let (n1, m1, r1, p1) = circ_stats(&loud_onset_hours);
    push(format!(
        "loud-onset UTC hour (first loud sample per loud cell): n {n1} mean {m1:.2} R {r1:.3} rayleigh p {p1:.3e}"
    ));
    push(format!("  1h bins (n per bin): {}", fmt_bins(&bin24(&loud_onset_hours))));
    let (n2, m2, r2, p2) = circ_stats(&run_start_hours);
    push(format!(
        "pass-run start UTC hour (all runs of loud cells): n {n2} mean {m2:.2} R {r2:.3} rayleigh p {p2:.3e}"
    ));
    push(format!("  1h bins: {}", fmt_bins(&bin24(&run_start_hours))));
    let (n3, m3, r3, p3) = circ_stats(&cell_first_hours);
    push(format!(
        "cell first floor-sample hour (all loud cells): n {n3} mean {m3:.2} R {r3:.3} rayleigh p {p3:.3e}"
    ));
    push("per (mode, station) loud-onset hour: n mean R".to_string());
    for mo in MODES {
        for st in STATIONS {
            let Some(hs) = per_series_onset.get(&(mo, st)) else {
                continue;
            };
            let (n, mean, r, _) = circ_stats(hs);
            push(format!("  m{mo} st{st}: n {n} mean {mean:.2} R {r:.3}"));
        }
    }
    push("per (mode, station) quiet-cell first floor-sample hour (control): n mean R".to_string());
    for mo in MODES {
        for st in STATIONS {
            let qh: Vec<f64> = quiet_first
                .iter()
                .filter(|((s, m, _), _)| *s == st && *m == mo)
                .map(|(_, t)| hour_of_day(*t))
                .collect();
            let (n, mean, r, _) = circ_stats(&qh);
            push(format!("  m{mo} st{st}: n {n} mean {mean:.2} R {r:.3}"));
        }
    }
    let mut run_onset_hours: Vec<f64> = Vec::new();
    for mo in MODES {
        for st in STATIONS {
            let Some(series) = series_map.get(&(mo, st)) else {
                continue;
            };
            let mut i = 0usize;
            while i < series.len() {
                if series[i].1 < LOUD_HZ {
                    i += 1;
                    continue;
                }
                let mut j = i;
                while j + 1 < series.len()
                    && series[j + 1].1 >= LOUD_HZ
                    && series[j + 1].0 - series[j].0 == 1
                {
                    j += 1;
                }
                let start_day = series[i].0;
                if let Some(daymean) = loud_cells
                    .iter()
                    .find(|c| c.0 == st && c.1 == mo && c.2 == start_day)
                    .map(|c| c.4)
                {
                    if let Some(samples) = loud_samples.get(&(st, mo, start_day)) {
                        if let Some(tl) = samples
                            .iter()
                            .find(|s| (s.1 - daymean).abs() >= LOUD_SAMPLE_HZ)
                        {
                            run_onset_hours.push(hour_of_day(tl.0));
                        }
                    }
                }
                i = j + 1;
            }
        }
    }
    let (no, moo, ro, po) = circ_stats(&run_onset_hours);
    push(format!(
        "episode (loud-run) onset UTC hour: n {no} mean {moo:.2} R {ro:.3} rayleigh p {po:.3e}"
    ));
    push(format!("  1h bins: {}", fmt_bins(&bin24(&run_onset_hours))));

    let mut onset_dist: Vec<f64> = loud_onset_min_after_start.clone();
    onset_dist.sort_by(f64::total_cmp);
    if !onset_dist.is_empty() {
        let med = onset_dist[onset_dist.len() / 2];
        let p90 = onset_dist[((onset_dist.len() as f64) * 0.9) as usize];
        let mx = onset_dist[onset_dist.len() - 1];
        push(format!(
            "loud onset minutes after cell first floor sample: n {} median {med:.1} p90 {p90:.1} max {mx:.1}",
            onset_dist.len()
        ));
    }

    let quiet_first_hours: Vec<f64> = quiet_first.values().map(|t| hour_of_day(*t)).collect();
    let (nq, mq, rq, pq) = circ_stats(&quiet_first_hours);
    push(format!(
        "control: quiet robust cell first floor-sample hour: n {nq} mean {mq:.2} R {rq:.3} rayleigh p {pq:.3e}"
    ));
    push(format!("  1h bins: {}", fmt_bins(&bin24(&quiet_first_hours))));

    push("\n== C. inter-episode intervals and Lomb-Scargle over the robust day series ==".to_string());
    for mo in MODES {
        for st in STATIONS {
            let Some(series) = series_map.get(&(mo, st)) else {
                continue;
            };
            let loud_days: Vec<i64> = series
                .iter()
                .filter(|(_, r)| *r >= LOUD_HZ)
                .map(|(d, _)| *d)
                .collect();
            let intervals: Vec<i64> = loud_days.windows(2).map(|w| w[1] - w[0]).collect();
            let int_str = if intervals.is_empty() {
                "none".to_string()
            } else {
                let mut v = intervals.clone();
                v.sort_unstable();
                format!(
                    "n {} median {} max {}",
                    v.len(),
                    v[v.len() / 2],
                    v[v.len() - 1]
                )
            };
            push(format!(
                "m{mo} st{st}: loud days n {}; consecutive loud-day gaps (d): {int_str}; list {:?}",
                loud_days.len(),
                intervals
            ));

            if series.len() >= 8 {
                let t0 = series[0].0 as f64;
                let t1 = series[series.len() - 1].0 as f64;
                let span = t1 - t0;
                let times: Vec<f64> = series.iter().map(|(d, _)| *d as f64).collect();
                let vals: Vec<f64> = series
                    .iter()
                    .map(|(_, r)| if *r > 0.0 { r.log10() } else { -4.0 })
                    .collect();
                let pmin = 2.5f64;
                let pmax = (span / 2.0).min(90.0);
                if pmax > pmin {
                    let fhi = 1.0 / pmin;
                    let flo = 1.0 / pmax;
                    let step = 1.0 / (4.0 * span);
                    let (bf, bp) = ls_best(&times, &vals, flo, fhi, step);
                    let mut rng = SEED ^ ((mo as u64) << 8) ^ (st as u64);
                    let mut null_max = Vec::with_capacity(N_SURR);
                    for _ in 0..N_SURR {
                        let mut sv = vals.clone();
                        shuffle(&mut sv, &mut rng);
                        let (_, p) = ls_best(&times, &sv, flo, fhi, step);
                        null_max.push(p);
                    }
                    null_max.sort_by(f64::total_cmp);
                    let p95 = null_max[(N_SURR as f64 * 0.95) as usize];
                    let pmax = null_max[N_SURR - 1];
                    let surv = bp > p95;
                    push(format!(
                        "  LS log10(dayRMS): span {span:.0} d, best period {:.2} d (f {bf:.5} 1/d), var-explained {bp:.3}, surrogate-p95 {p95:.3} p100 {pmax:.3}, above p95 {surv}",
                        1.0 / bf
                    ));
                } else {
                    push(format!("  LS: span {span:.0} d too short for period search (pmax {pmax:.1} <= pmin {pmin})"));
                }
            }
        }
    }

    push("\n== D. loud-run durations, bursts, station transitions ==".to_string());
    let mut burst_lens: Vec<usize> = Vec::new();
    let mut run_start_days: Vec<i64> = Vec::new();
    for mo in MODES {
        for st in STATIONS {
            let Some(series) = series_map.get(&(mo, st)) else {
                continue;
            };
            let mut i = 0usize;
            while i < series.len() {
                if series[i].1 < LOUD_HZ {
                    i += 1;
                    continue;
                }
                let mut j = i;
                while j + 1 < series.len()
                    && series[j + 1].1 >= LOUD_HZ
                    && series[j + 1].0 - series[j].0 == 1
                {
                    j += 1;
                }
                burst_lens.push(j - i + 1);
                run_start_days.push(series[i].0);
                i = j + 1;
            }
        }
    }
    let mut bl = burst_lens.clone();
    bl.sort_unstable();
    let mut counts = BTreeMap::new();
    for b in &bl {
        *counts.entry(*b).or_insert(0usize) += 1;
    }
    let count_str: Vec<String> = counts.iter().map(|(k, v)| format!("len{k}:{v}")).collect();
    push(format!(
        "loud runs (episodes) n {} over all series; length dist: {}",
        bl.len(),
        count_str.join(" ")
    ));
    if run_start_days.len() >= 2 {
        let mut sd = run_start_days.clone();
        sd.sort_unstable();
        let gaps: Vec<i64> = sd.windows(2).map(|w| w[1] - w[0]).collect();
        let mut gmed = gaps.clone();
        gmed.sort_unstable();
        push(format!(
            "loud-run onset gaps (d) across series: n {} median {} max {}",
            gaps.len(),
            gmed[gmed.len() / 2],
            *gmed.last().unwrap()
        ));
    }
    for mo in MODES {
        let mut trans: [[usize; 3]; 3] = [[0; 3]; 3];
        let mut days: BTreeMap<i64, (f64, i64)> = BTreeMap::new();
        for st in STATIONS {
            let Some(series) = series_map.get(&(mo, st)) else {
                continue;
            };
            for (d, r) in series {
                if *r >= LOUD_HZ {
                    let e = days.entry(*d).or_insert((f64::NEG_INFINITY, st));
                    if *r > e.0 {
                        *e = (*r, st);
                    }
                }
            }
        }
        let dlist: Vec<(i64, i64)> = days.iter().map(|(d, (_, s))| (*d, *s)).collect();
        let mut ntrans = 0usize;
        for w in dlist.windows(2) {
            if w[1].0 - w[0].0 == 1 {
                let a = STATIONS.iter().position(|x| *x == w[0].1).unwrap();
                let b = STATIONS.iter().position(|x| *x == w[1].1).unwrap();
                trans[a][b] += 1;
                ntrans += 1;
            }
        }
        let names = ["14", "43", "63"];
        let mut mstr = String::new();
        for a in 0..3 {
            let row: Vec<String> = (0..3)
                .map(|b| format!("{}->{}:{}", names[a], names[b], trans[a][b]))
                .collect();
            mstr.push_str(&row.join(" "));
            mstr.push_str(" | ");
        }
        push(format!("mode {mo}: loudest-station day transitions n {ntrans}: {mstr}"));
    }

    push("\n== E. summary ==".to_string());
    push(format!(
        "loud cells {n_loud_all}; flips {n_flip_all}; sub-day loud-cell set {}; loud-sample gate {LOUD_SAMPLE_HZ} Hz; pass gap {PASS_GAP_S} s",
        loud_cells.len()
    ));

    let _ = fs::write(&report_path, out.join("\n") + "\n");
    eprintln!("galileo: sub-day clock report written to {report_path}");
}
