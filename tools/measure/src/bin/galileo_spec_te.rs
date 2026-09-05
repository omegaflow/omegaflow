use std::collections::BTreeMap;

use omegaflow::te::{
    conditional_te_stats, surrogate_stats_block, surrogate_stats_phase, transfer_entropy_conditional,
    transfer_entropy_lag,
};

const LOCK_HZ: f64 = 1.0e3;
const MIN_SAMP: usize = 30;
const MIN_DAYS: usize = 14;
const MIN_MODE_DAYS: usize = 4;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const N_SURR: usize = 20;
const BLOCK: usize = 5;

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / 86400.0;
    (jd - 2440587.5).round() as i64
}

fn month_index(unix_day: i64) -> Option<i64> {
    let (y, m, _) = omegaflow::spectral::civil_from_days(unix_day)?;
    Some(y as i64 * 12 + m as i64)
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

fn fmt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.4e}"),
        None => "-".to_string(),
    }
}

fn series_var(v: &[f32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    Some(v.iter().map(|&x| (x as f64 - mean) * (x as f64 - mean)).sum::<f64>() / n)
}

fn distinct_count(v: &[f32], tol: f64) -> usize {
    let mut w: Vec<f64> = v.iter().map(|&x| (x as f64 / tol).round()).collect();
    w.sort_by(f64::total_cmp);
    w.dedup();
    w.len()
}

#[derive(Default)]
struct Agg {
    n: usize,
    n_non1: usize,
    vals: Vec<f32>,
    refs: Vec<f64>,
}

struct DayRow {
    day: i64,
    mode: f64,
    domfrac: f64,
    noise_rms: f64,
    noise_med: f64,
    ref_med: f64,
    era: f64,
    frac_non1: f64,
}

fn te_tables(label: &str, drv: &[f32], tgt: &[f32], era: &[f32]) {
    let n = drv.len();
    let (Some(vd), Some(vt)) = (series_var(drv), series_var(tgt)) else {
        println!("== {label} | n = {n} | degenerate series -> no directed TE");
        return;
    };
    if n < 8 {
        println!("== {label} | n = {n} < 8 -> no verdict (measured limit)");
        return;
    }
    println!(
        "== {label} | n = {n} | drvVar {vd:.3e} tgtVar {vt:.3e} | drvLvl {} | eraLv {}",
        distinct_count(drv, 1.0),
        distinct_count(era, 1.0)
    );
    println!("    dir | lag |      TE |   thrPh |   thrBl |     cTE |    cThr");
    for &lag in &[1usize, 2, 3, 5] {
        let f_te = transfer_entropy_lag(tgt, drv, lag);
        let f_ph = surrogate_stats_phase(tgt, drv, lag, SEED).map(|(_, _, t)| t);
        let f_bl = surrogate_stats_block(tgt, drv, lag, BLOCK, SEED).map(|(_, _, t)| t);
        let f_ct = transfer_entropy_conditional(tgt, drv, era, lag);
        let f_cthr = conditional_te_stats(tgt, drv, era, lag, SEED, N_SURR).map(|(_, _, t)| t);
        println!(
            "    D->T | {:>2} | {} | {} | {} | {} | {}",
            lag,
            fmt(f_te),
            fmt(f_ph),
            fmt(f_bl),
            fmt(f_ct),
            fmt(f_cthr)
        );
        let r_te = transfer_entropy_lag(drv, tgt, lag);
        let r_ph = surrogate_stats_phase(drv, tgt, lag, SEED).map(|(_, _, t)| t);
        let r_bl = surrogate_stats_block(drv, tgt, lag, BLOCK, SEED).map(|(_, _, t)| t);
        let r_ct = transfer_entropy_conditional(drv, tgt, era, lag);
        let r_cthr = conditional_te_stats(drv, tgt, era, lag, SEED, N_SURR).map(|(_, _, t)| t);
        println!(
            "    T->D | {:>2} | {} | {} | {} | {} | {}",
            lag,
            fmt(r_te),
            fmt(r_ph),
            fmt(r_bl),
            fmt(r_ct),
            fmt(r_cthr)
        );
    }
    println!();
}

fn main() {
    let path = std::env::args()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .unwrap_or_else(|| "data/galileo_resid.bin".to_string());
    let bytes = std::fs::read(&path).expect("resid bin read");
    if bytes.len() < 8 || &bytes[0..4] != b"GASR" {
        println!("no GASR header");
        return;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().expect("count")) as usize;
    if bytes.len() != 8 + count * 64 {
        println!("length mismatch");
        return;
    }
    let mut agg: BTreeMap<(i64, i64, i64), Agg> = BTreeMap::new();
    let mut total = 0usize;
    let mut cleaned = 0usize;
    let mut n_lock = 0usize;
    let mut n_zero = 0usize;
    let mut mode_sampler: [[usize; 3]; 3] = [[0; 3]; 3];
    let mut mode_total = [0usize; 3];
    for i in 0..count {
        let base = 8 + i * 64;
        let rd = |k: usize| {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&bytes[base + k * 8..base + k * 8 + 8]);
            f64::from_le_bytes(buf)
        };
        let rec: [f64; 8] = [rd(0), rd(1), rd(2), rd(3), rd(4), rd(5), rd(6), rd(7)];
        total += 1;
        let mi = rec[3] as i64 - 1;
        let si = if (rec[6] * 10.0).round() as i64 == 10 {
            0usize
        } else if (rec[6] * 10.0).round() as i64 == 600 {
            1usize
        } else {
            2usize
        };
        if (0..3).contains(&mi) {
            mode_total[mi as usize] += 1;
            mode_sampler[mi as usize][si] += 1;
        }
        if !rec[1].is_finite() {
            continue;
        }
        if rec[1].abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        if rec[7] == 0.0 {
            n_zero += 1;
            continue;
        }
        let day = unix_day(rec[0]);
        let key = (day, rec[2] as i64, rec[3] as i64);
        let a = agg.entry(key).or_default();
        a.n += 1;
        if (rec[6] * 10.0).round() as i64 != 10 {
            a.n_non1 += 1;
        }
        a.vals.push(rec[1].abs() as f32);
        a.refs.push(rec[5]);
        cleaned += 1;
    }
    println!(
        "GASR {path}: {total} records, cleaned {cleaned} (|resid|<={LOCK_HZ:.0} Hz, strength != 0, finite), lock {n_lock}, zero-strength {n_zero}"
    );
    println!("mode_total {mode_total:?}");
    for m in 0..3 {
        let t = mode_total[m] as f64;
        if t > 0.0 {
            println!(
                "  mode {} frac60s {:.4} frac1s {:.4} fracOther {:.4}",
                m + 1,
                mode_sampler[m][1] as f64 / t,
                mode_sampler[m][0] as f64 / t,
                mode_sampler[m][2] as f64 / t
            );
        }
    }
    let mut daymap: BTreeMap<(i64, i64), Vec<(i64, &Agg)>> = BTreeMap::new();
    for ((d, st, mo), a) in &agg {
        if a.n >= MIN_SAMP {
            daymap.entry((*d, *st)).or_default().push((*mo, a));
        }
    }
    let mut c1s = 0usize;
    let mut cnon1 = 0usize;
    for (_, v) in &daymap {
        for (_, a) in v {
            c1s += a.n - a.n_non1;
            cnon1 += a.n_non1;
        }
    }
    println!(
        "cleaned cadence field: {c1s} records at 1 s, {cnon1} at other cadence (frac1s {:.6})",
        c1s as f64 / (c1s + cnon1).max(1) as f64
    );
    let mut rows: BTreeMap<i64, Vec<DayRow>> = BTreeMap::new();
    for ((d, st), bins) in &daymap {
        let tot = bins.iter().map(|(_, a)| a.n).sum::<usize>();
        let (dm, da) = bins
            .iter()
            .max_by(|(_, a), (_, b)| a.n.cmp(&b.n))
            .map(|(m, a)| (*m, a))
            .expect("nonempty");
        let mut av: Vec<f64> = da.vals.iter().map(|&x| x as f64).collect();
        let med = median(&mut av).expect("vals");
        let Some(rr) = rms(&av) else {
            continue;
        };
        let rm = median(&mut da.refs.clone()).expect("refs");
        let Some(era) = month_index(*d) else {
            continue;
        };
        rows.entry(*st).or_default().push(DayRow {
            day: *d,
            mode: dm as f64,
            domfrac: da.n as f64 / tot as f64,
            noise_rms: rr,
            noise_med: med,
            ref_med: rm,
            era: era as f64,
            frac_non1: da.n_non1 as f64 / da.n as f64,
        });
    }
    let sel: Vec<i64> = vec![14, 43, 63, 12, 42, 61];
    for st in sel {
        let Some(rl) = rows.get(&st) else {
            continue;
        };
        if rl.len() < MIN_DAYS {
            continue;
        }
        println!("\n##### station {st}: {} qualifying days", rl.len());
        let mut runs: Vec<(usize, usize)> = Vec::new();
        let mut a = 0usize;
        for i in 1..=rl.len() {
            if i == rl.len() || rl[i].day != rl[i - 1].day + 1 {
                runs.push((a, i - 1));
                a = i;
            }
        }
        for (lo, hi) in runs {
            let nd = hi - lo + 1;
            if nd < MIN_DAYS {
                continue;
            }
            let d0 = rl[lo].day;
            let d1 = rl[hi].day;
            let d: Vec<&DayRow> = rl[lo..=hi].iter().collect();
            let mut mode_days: BTreeMap<i64, usize> = BTreeMap::new();
            for r in &d {
                *mode_days.entry(r.mode as i64).or_insert(0usize) += 1;
            }
            let mean_df = d.iter().map(|r| r.domfrac).sum::<f64>() / nd as f64;
            let mut same = 0usize;
            let mut adj = 0usize;
            for i in 1..d.len() {
                adj += 1;
                if (d[i].mode as i64) == (d[i - 1].mode as i64) {
                    same += 1;
                }
            }
            let mut mode_logrms: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
            for r in &d {
                mode_logrms
                    .entry(r.mode as i64)
                    .or_default()
                    .push(r.noise_rms.max(1e-12).log10());
            }
            let mut ml: Vec<String> = Vec::new();
            for (m, v) in &mode_logrms {
                let mut c = v.clone();
                ml.push(format!(
                    "m{m}:n{}:medlog10rms {:.2}",
                    c.len(),
                    median(&mut c).unwrap_or(0.0)
                ));
            }
            println!(
                "  run {d0}-{d1}: {nd} days | domfrac mean {mean_df:.2} | mode-day counts {:?} | mode persist {same}/{adj} | {}",
                mode_days,
                ml.join(" ")
            );
            let era: Vec<f32> = d.iter().map(|r| r.era as f32).collect();
            let tgt_rms: Vec<f32> = d.iter().map(|r| r.noise_rms as f32).collect();
            let tgt_med: Vec<f32> = d.iter().map(|r| r.noise_med as f32).collect();
            let l_base = format!("st{st} {d0}-{d1}");
            let drv_ref: Vec<f32> = d.iter().map(|r| r.ref_med as f32).collect();
            te_tables(&format!("S1 ref->rms | {l_base}"), &drv_ref, &tgt_rms, &era);
            te_tables(&format!("S1 ref->med | {l_base}"), &drv_ref, &tgt_med, &era);
            let (mj_mode, _) = mode_days
                .iter()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(m, c)| (*m, *c))
                .expect("mode_days");
            let mj_rows: Vec<&DayRow> = d.iter().copied().filter(|r| (r.mode as i64) == mj_mode).collect();
            if mj_rows.len() >= MIN_DAYS {
                let drv_ref_m: Vec<f32> = mj_rows.iter().map(|r| r.ref_med as f32).collect();
                let tgt_rms_m: Vec<f32> = mj_rows.iter().map(|r| r.noise_rms as f32).collect();
                let tgt_med_m: Vec<f32> = mj_rows.iter().map(|r| r.noise_med as f32).collect();
                let era_m: Vec<f32> = mj_rows.iter().map(|r| r.era as f32).collect();
                te_tables(
                    &format!("S1 ref->rms mode{mj_mode} | {l_base}"),
                    &drv_ref_m,
                    &tgt_rms_m,
                    &era_m,
                );
                te_tables(
                    &format!("S1 ref->med mode{mj_mode} | {l_base}"),
                    &drv_ref_m,
                    &tgt_med_m,
                    &era_m,
                );
            } else {
                println!(
                    "    S1 mode{mj_mode}-controlled skip: only {} dominant-mode days in run",
                    mj_rows.len()
                );
            }
            let drv_mode: Vec<f32> = d.iter().map(|r| r.mode as f32).collect();
            let nmodes = distinct_count(&drv_mode, 1.0);
            if nmodes >= 2 {
                te_tables(&format!("S0 ctl mode->ref | {l_base}"), &drv_mode, &drv_ref, &era);
            }
            let small_mode = mode_days.values().min().cloned().unwrap_or(0);
            if nmodes >= 2 && small_mode >= MIN_MODE_DAYS {
                te_tables(&format!("S2 mode->rms | {l_base}"), &drv_mode, &tgt_rms, &era);
                te_tables(&format!("S2 mode->med | {l_base}"), &drv_mode, &tgt_med, &era);
            } else {
                println!(
                    "    S2 skip: {nmodes} mode levels, smallest mode-day count {small_mode} < {MIN_MODE_DAYS}"
                );
            }
            let non1: Vec<f32> = d.iter().map(|r| r.frac_non1 as f32).collect();
            let Some(vn) = series_var(&non1) else {
                continue;
            };
            if vn > 0.0 {
                te_tables(&format!("S3 cad->rms | {l_base}"), &non1, &tgt_rms, &era);
            } else {
                println!(
                    "    S3 skip: day-level cadence fracNon1 var {vn:.3e} -> cadence axis constant in run"
                );
            }
        }
    }
}
