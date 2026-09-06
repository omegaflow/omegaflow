use std::collections::{BTreeMap, HashMap};

use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;
use omegaflow::te::{
    conditional_te_stats, surrogate_stats_block, surrogate_stats_phase,
    transfer_entropy_conditional, transfer_entropy_lag,
};

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR_STRENGTH: i64 = -2560;
const STRONG_STRENGTH: i64 = -1750;
const MIN_CELL: usize = 30;
const MIN_N: usize = 30;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const N_SURR: usize = 20;
const BLOCK: usize = 5;
const LAGS: [usize; 4] = [1, 3, 5, 7];

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn ang(a: [f64; 3], b: [f64; 3]) -> Option<f64> {
    let na = norm(a);
    let nb = norm(b);
    if na <= 0.0 || nb <= 0.0 {
        return None;
    }
    Some((dot(a, b) / (na * nb)).clamp(-1.0, 1.0).acos().to_degrees())
}

fn j2000_day(tdb: f64) -> i64 {
    (tdb / DAY_S).floor() as i64
}

fn unix_day(dj: i64) -> i64 {
    (dj as f64 + 2451545.0 - 2440587.5).round() as i64
}

fn civil_of(dj: i64) -> Option<(u32, u32, u32)> {
    civil_from_days(unix_day(dj))
}

fn month_index(dj: i64) -> Option<i64> {
    let (y, m, _) = civil_of(dj)?;
    Some(y as i64 * 12 + m as i64)
}

fn median(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut w = v.to_vec();
    w.sort_by(f64::total_cmp);
    Some(w[w.len() / 2])
}

fn date_label(dj: i64) -> String {
    match civil_of(dj) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("jd {dj}"),
    }
}

fn fmt_o(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.4e}"),
        _ => "-".to_string(),
    }
}

fn star(te: Option<f64>, thr: Option<f64>) -> &'static str {
    match (te, thr) {
        (Some(t), Some(h)) if t > h => "*",
        _ => "",
    }
}

fn fmt_pair(te: Option<f64>, thr: Option<f64>) -> String {
    format!("{}{}", fmt_o(te), star(te, thr))
}

fn fmt_cell(te: Option<f64>, thr: Option<f64>) -> String {
    format!("{}{}", fmt_o(te), star(te, thr))
}

fn load_eph(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let p = format!("data/ssd.jpl.nasa.gov/ephemeris_{name}.bin");
    match std::fs::read(&p) {
        Ok(bytes) => parse_ephemeris_binary(&bytes).is_some_and(|e| {
            eph.insert(name.to_string(), e);
            true
        }),
        Err(_) => false,
    }
}

fn elongation_deg(dj: i64, eph: &HashMap<String, BodyEphemeris>) -> Option<f64> {
    let t = dj as f64 * DAY_S;
    match (
        body_barycenter_position("galileo_daily", t, eph),
        body_barycenter_position("earth", t, eph),
    ) {
        (Some(p), Some(e)) => ang(sub([0.0, 0.0, 0.0], e), sub(p, e)),
        _ => None,
    }
}

fn receiver_step(dj: i64) -> Option<f64> {
    let u = unix_day(dj);
    let bvr = days_from_civil(1995, 9, 18)?;
    let dgt = days_from_civil(1996, 5, 1)?;
    if u < bvr {
        Some(0.0)
    } else if u < dgt {
        Some(1.0)
    } else {
        Some(2.0)
    }
}

fn build_driver_arrays(
    days: &[i64],
    eph: &HashMap<String, BodyEphemeris>,
) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut elong = Vec::with_capacity(days.len());
    let mut step = Vec::with_capacity(days.len());
    let mut era = Vec::with_capacity(days.len());
    for &d in days {
        match elongation_deg(d, eph) {
            Some(e) if e.is_finite() => elong.push(e as f32),
            _ => elong.push(f32::NAN),
        }
        step.push(receiver_step(d).unwrap_or(f64::NAN) as f32);
        era.push(match month_index(d) {
            Some(m) => m as f32,
            None => f32::NAN,
        });
    }
    (elong, step, era)
}

fn run_unconditional(y: &[f32], xs: &[f32], dname: &str) -> Vec<String> {
    let mut out = Vec::new();
    if y.len() < MIN_N {
        out.push(format!("  {dname}: n {} < {MIN_N} -> no verdict", y.len()));
        return out;
    }
    out.push(format!("  {dname}: n {}", y.len()));
    for &lag in &LAGS {
        let te = transfer_entropy_lag(y, xs, lag);
        let thr_p = surrogate_stats_phase(y, xs, lag, SEED).map(|(_, _, t)| t);
        let thr_b = surrogate_stats_block(y, xs, lag, BLOCK, SEED).map(|(_, _, t)| t);
        let te_r = transfer_entropy_lag(xs, y, lag);
        out.push(format!(
            "    lag {lag:>2}  fwd TE {}  thrPh {}  thrBl {}  |  rev TE {}",
            fmt_pair(te, thr_p),
            fmt_o(thr_p),
            fmt_o(thr_b),
            fmt_o(te_r)
        ));
    }
    out
}

fn run_conditional(y: &[f32], xs: &[f32], c: &[f32], dname: &str, cname: &str) -> Vec<String> {
    let mut out = Vec::new();
    if y.len() < MIN_N || c.len() != y.len() {
        out.push(format!("  {dname}|{cname}: n {} -> no verdict", y.len()));
        return out;
    }
    out.push(format!("  {dname}|{cname}: n {}", y.len()));
    for &lag in &LAGS {
        let cte = transfer_entropy_conditional(y, xs, c, lag);
        let cthr = conditional_te_stats(y, xs, c, lag, SEED, N_SURR).map(|(_, _, t)| t);
        out.push(format!(
            "    lag {lag:>2}  cTE {}  cThr {}",
            fmt_cell(cte, cthr),
            fmt_o(cthr)
        ));
    }
    out
}

fn analyze(
    out: &mut Vec<String>,
    label: &str,
    days: &[i64],
    y: &[f32],
    eph: &HashMap<String, BodyEphemeris>,
    cond: bool,
) {
    if days.len() != y.len() {
        out.push(format!("== {label}: day/series mismatch"));
        return;
    }
    if days.len() < MIN_N {
        out.push(format!(
            "== {label}: n {} < {MIN_N} -> no verdict (measured limit)",
            days.len()
        ));
        return;
    }
    let (elong, step, era) = build_driver_arrays(days, eph);
    if !elong.iter().all(|v| v.is_finite()) {
        out.push(format!(
            "== {label}: elongation void on some days -> no verdict"
        ));
        return;
    }
    out.push(format!("== {label}: n {}", days.len()));
    let lo_d = days[0];
    let hi_d = *days.last().unwrap();
    let to_f64 = |v: &[f32]| -> Vec<f64> { v.iter().map(|&x| x as f64).collect() };
    let ef = to_f64(&elong);
    let sf = to_f64(&step);
    let ra = to_f64(&era);
    let elong_lo = ef.iter().cloned().fold(f64::INFINITY, f64::min);
    let elong_hi = ef.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let step_lo = sf.iter().cloned().fold(f64::INFINITY, f64::min);
    let step_hi = sf.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let era_lo = ra.iter().cloned().fold(f64::INFINITY, f64::min);
    let era_hi = ra.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    out.push(format!(
        "   span {} .. {} | elong {:.2}..{:.2} | step {:.0}..{:.0} | era {:.0}..{:.0}",
        date_label(lo_d),
        date_label(hi_d),
        elong_lo,
        elong_hi,
        step_lo,
        step_hi,
        era_lo,
        era_hi
    ));
    let mut gaps: Vec<i64> = days.windows(2).map(|w| w[1] - w[0]).collect();
    gaps.sort_unstable();
    let mgap = gaps.iter().cloned().fold(0i64, i64::max);
    let med_gap = median(&gaps.iter().map(|&g| g as f64).collect::<Vec<_>>())
        .map(|m| m as i64)
        .unwrap_or(-1);
    out.push(format!("   day gaps: max {mgap}  median {med_gap}"));
    out.extend(run_unconditional(y, &elong, "D1 elong -> y"));
    out.extend(run_unconditional(y, &step, "D2 step  -> y"));
    out.extend(run_unconditional(y, &era, "D3 era   -> y"));
    if cond {
        out.push("   conditional (driver | conditioning driver):".to_string());
        out.extend(run_conditional(y, &elong, &era, "D1 elong", "D3 era"));
        out.extend(run_conditional(y, &elong, &step, "D1 elong", "D2 step"));
        out.extend(run_conditional(y, &step, &era, "D2 step", "D3 era"));
        out.extend(run_conditional(y, &step, &elong, "D2 step", "D1 elong"));
        out.extend(run_conditional(y, &era, &elong, "D3 era", "D1 elong"));
        out.extend(run_conditional(y, &era, &step, "D3 era", "D2 step"));
    }
    out.push(String::new());
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "tmp/galileo_floor_external_te.txt".to_string(),
    };

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let geom_ok = load_eph("galileo_daily", &mut eph) && load_eph("earth", &mut eph);

    let Ok(bytes) = std::fs::read("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };

    let lo = days_from_civil(1994, 12, 1).unwrap();
    let hi = days_from_civil(1997, 2, 28).unwrap();

    let mut n_mode1 = 0usize;
    let mut n_lock = 0usize;
    let mut n_floor = 0usize;
    let mut n_strong = 0usize;
    let mut cells: BTreeMap<(i64, i64, usize), Vec<f64>> = BTreeMap::new();
    for r in &recs {
        if r[3] as i64 != 1 {
            continue;
        }
        n_mode1 += 1;
        if r[1].abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        let st = r[2] as i64;
        if st != 14 && st != 43 && st != 63 {
            continue;
        }
        let dj = j2000_day(r[0]);
        let u = unix_day(dj);
        if u < lo || u > hi {
            continue;
        }
        let s = r[7] as i64;
        let class = if s == FLOOR_STRENGTH {
            0usize
        } else if s >= STRONG_STRENGTH {
            1usize
        } else {
            continue;
        };
        if class == 0 {
            n_floor += 1;
        } else {
            n_strong += 1;
        }
        cells.entry((dj, st, class)).or_default().push(r[1].abs());
    }
    drop(recs);

    let mut day_set: Vec<i64> = cells.keys().map(|&(d, _, _)| d).collect();
    day_set.sort_unstable();
    day_set.dedup();

    let mut out: Vec<String> = Vec::new();
    out.push("galileo mode-1 floor/strong noise vs external researched drivers (TE)".to_string());
    out.push(format!(
        "mode1 samples {n_mode1}, lock {n_lock}; floor == -2560 {n_floor}, strong >= -1750 {n_strong} (window, stations 14/43/63)"
    ));
    out.push(format!(
        "geometry ephemerides: {}",
        if geom_ok { "loaded" } else { "void" }
    ));
    out.push(format!(
        "window {} .. {} ({} distinct j2000 days carrying floor/strong cells)",
        date_label(day_set[0]),
        date_label(*day_set.last().unwrap()),
        day_set.len()
    ));
    out.push("TE convention: transfer_entropy(target = day level, source = driver) reports driver-directed coupling".to_string());
    out.push("nulls: phase-randomized (10) and block-bootstrap block=5 (10); conditional residual surrogates (20). * = TE above the null threshold".to_string());
    out.push(format!(
        "cell minimum {MIN_CELL} samples per (day, station, population); series minimum {MIN_N} days; lags {LAGS:?} days"
    ));
    out.push(String::new());

    for (class, cname) in [(0usize, "floor"), (1usize, "strong")] {
        for &st in &[0i64, 14, 43, 63] {
            let label = if st == 0 {
                format!("network | {cname}")
            } else {
                format!("st{st} | {cname}")
            };
            let mut cells_of: Vec<(i64, f64)> = Vec::new();
            for ((d, s, c), v) in &cells {
                if *c != class {
                    continue;
                }
                if st != 0 && s != &st {
                    continue;
                }
                if v.len() < MIN_CELL {
                    continue;
                }
                if let Some(m) = median(v) {
                    if m.is_finite() {
                        cells_of.push((*d, m));
                    }
                }
            }
            cells_of.sort_by_key(|&(d, _)| d);
            if cells_of.is_empty() {
                out.push(format!(
                    "== {label}: no day-cells >= {MIN_CELL} -> no verdict (measured absence)\n"
                ));
                continue;
            }
            let days: Vec<i64> = cells_of.iter().map(|&(d, _)| d).collect();
            let ys: Vec<f32> = cells_of.iter().map(|&(_, m)| m as f32).collect();
            let cond = st == 0;
            analyze(&mut out, &label, &days, &ys, &eph, cond);
        }
    }

    out.push("day-cell counts per (month, population) over all stations:".to_string());
    let mut month_counts: BTreeMap<(i64, usize), usize> = BTreeMap::new();
    for ((d, _, class), v) in &cells {
        let Some(mi) = month_index(*d) else {
            continue;
        };
        *month_counts.entry((mi, *class)).or_insert(0) += v.len();
    }
    for ((mi, class), n) in &month_counts {
        let (y, m) = (mi / 12, mi % 12);
        let cname = if *class == 0 { "floor" } else { "strong" };
        out.push(format!("  {y:04}-{m:02} {cname}: {n} samples"));
    }

    let body = out.join("\n") + "\n";
    if let Some(parent) = std::path::Path::new(&report_path).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    match std::fs::write(&report_path, &body) {
        Ok(()) => println!("report written: {report_path}"),
        Err(e) => println!("write {report_path} void: {e}"),
    }
}
