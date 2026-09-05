use std::collections::BTreeMap;

use omegaflow::te::{
    conditional_te_stats, surrogate_stats_block, surrogate_stats_phase, transfer_entropy_conditional,
    transfer_entropy_lag,
};

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_DAY: usize = 30;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const N_SURR: usize = 20;
const BLOCK: usize = 5;

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / DAY_S;
    (jd - 2440587.5).round() as i64
}

fn median(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    Some(v[v.len() / 2])
}

fn rms(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    let mean = v.iter().sum::<f64>() / n;
    Some((v.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / n).sqrt())
}

fn month_index(unix_day: i64) -> Option<i64> {
    let (y, m, _) = omegaflow::spectral::civil_from_days(unix_day)?;
    Some(y as i64 * 12 + m as i64)
}

fn load(path: &str) -> Option<Vec<[f64; 8]>> {
    let bytes = std::fs::read(path).ok()?;
    omegaflow::atdf::parse_resid_bin(&bytes)
}

fn fmt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.4e}"),
        None => "-".to_string(),
    }
}

fn block(label: &str, xs: &[f32], ys: &[f32], era: &[f32], metric: &str) {
    let n = xs.len();
    if n < 30 {
        println!("== {label} | {metric} | n = {n} < 30 -> no verdict (measured limit)\n");
        return;
    }
    println!("== {label} | {metric} | n = {n}");
    println!("    dir | lag |    TE  |  thrPh |  thrBl |  cTE|era |  cThr");
    for &lag in &[1usize, 2, 3, 5] {
        let te_sn = transfer_entropy_lag(ys, xs, lag);
        let thr_sn_p = surrogate_stats_phase(ys, xs, lag, SEED).map(|(_, _, t)| t);
        let thr_sn_b = surrogate_stats_block(ys, xs, lag, BLOCK, SEED).map(|(_, _, t)| t);
        let te_ns = transfer_entropy_lag(xs, ys, lag);
        let thr_ns_p = surrogate_stats_phase(xs, ys, lag, SEED).map(|(_, _, t)| t);
        let thr_ns_b = surrogate_stats_block(xs, ys, lag, BLOCK, SEED).map(|(_, _, t)| t);
        let cte_sn = transfer_entropy_conditional(ys, xs, era, lag);
        let cthr_sn = conditional_te_stats(ys, xs, era, lag, SEED, N_SURR).map(|(_, _, t)| t);
        let cte_ns = transfer_entropy_conditional(xs, ys, era, lag);
        let cthr_ns = conditional_te_stats(xs, ys, era, lag, SEED, N_SURR).map(|(_, _, t)| t);
        let mark = |te: Option<f64>, th: Option<f64>, tag: &str| match (te, th) {
            (Some(t), Some(h)) if t > h => format!(" {tag}*"),
            _ => String::new(),
        };
        println!(
            "    S->N | {:>2} | {} | {} | {} | {} | {}{}{}{}",
            lag,
            fmt(te_sn),
            fmt(thr_sn_p),
            fmt(thr_sn_b),
            fmt(cte_sn),
            fmt(cthr_sn),
            mark(te_sn, thr_sn_p, "Ph"),
            mark(te_sn, thr_sn_b, "Bl"),
            mark(cte_sn, cthr_sn, "cT")
        );
        println!(
            "    N->S | {:>2} | {} | {} | {} | {} | {}{}{}{}",
            lag,
            fmt(te_ns),
            fmt(thr_ns_p),
            fmt(thr_ns_b),
            fmt(cte_ns),
            fmt(cthr_ns),
            mark(te_ns, thr_ns_p, "Ph"),
            mark(te_ns, thr_ns_b, "Bl"),
            mark(cte_ns, cthr_ns, "cT")
        );
    }
    println!();
}

fn main() {
    let Some(recs) = load("data/galileo_resid.bin") else {
        println!("no resid bin");
        return;
    };
    let mut agg: BTreeMap<(i64, i64), Vec<[f64; 8]>> = BTreeMap::new();
    let mut n_mode1 = 0usize;
    let mut n_lock = 0usize;
    for r in &recs {
        if r[3] as i64 != 1 {
            continue;
        }
        n_mode1 += 1;
        if r[1].abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        if r[7] == 0.0 {
            continue;
        }
        let d = unix_day(r[0]);
        agg.entry((d, r[2] as i64)).or_default().push(*r);
    }
    println!("mode1 {n_mode1} lock {n_lock}\n");

    let windows = [
        (
            "island 1995-11-22..1996-01-14",
            omegaflow::archivar::lsk::days_from_civil(1995, 11, 22).unwrap(),
            omegaflow::archivar::lsk::days_from_civil(1996, 1, 14).unwrap(),
        ),
        (
            "island 1996-12-16..1997-02-14",
            omegaflow::archivar::lsk::days_from_civil(1996, 12, 16).unwrap(),
            omegaflow::archivar::lsk::days_from_civil(1997, 2, 14).unwrap(),
        ),
    ];

    for (wlabel, wlo, whi) in windows {
        for station in [0i64, 14, 43, 63] {
            let mut days: BTreeMap<i64, Vec<[f64; 8]>> = BTreeMap::new();
            for ((d, st), v) in &agg {
                if d < &wlo || d > &whi {
                    continue;
                }
                if station != 0 && st != &station {
                    continue;
                }
                let e = days.entry(*d).or_default();
                e.extend_from_slice(v);
            }
            let mut xs: Vec<f32> = Vec::new();
            let mut ys_m: Vec<f32> = Vec::new();
            let mut ys_r: Vec<f32> = Vec::new();
            let mut era: Vec<f32> = Vec::new();
            let mut daykeys: Vec<i64> = Vec::new();
            for (d, v) in &days {
                if v.len() < MIN_DAY {
                    continue;
                }
                let mut ss: Vec<f64> = v.iter().map(|r| r[7]).collect();
                let rr: Vec<f64> = v.iter().map(|r| r[1].abs()).collect();
                let (Some(sm), Some(rmm), Some(rrms)) =
                    (median(&mut ss), median(&mut rr.clone()), rms(&rr))
                else {
                    continue;
                };
                if !sm.is_finite() || !rmm.is_finite() || !rrms.is_finite() {
                    continue;
                }
                xs.push(sm as f32);
                ys_m.push(rmm as f32);
                ys_r.push(rrms as f32);
                era.push(month_index(*d).unwrap_or(0) as f32);
                daykeys.push(*d);
            }
            if xs.len() < 30 {
                continue;
            }
            let gaps: Vec<i64> = daykeys.windows(2).map(|w| w[1] - w[0]).collect();
            let maxgap = gaps.iter().cloned().fold(0, i64::max);
            let stn = if station == 0 {
                "pooled".to_string()
            } else {
                format!("st{station}")
            };
            let label = format!("{stn} | {wlabel} | maxdaygap {maxgap}");
            block(&label, &xs, &ys_m, &era, "median|r|");
            block(&label, &xs, &ys_r, &era, "rms(r)");
        }
    }
}
