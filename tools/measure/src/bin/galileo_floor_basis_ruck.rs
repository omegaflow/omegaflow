use std::collections::BTreeMap;
use std::fs;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const ERA0_C: (i64, i64, i64) = (1995, 11, 23);
const ERA1_C: (i64, i64, i64) = (1997, 2, 28);
const STATIONS: [i64; 3] = [14, 43, 63];
const N_PERM: usize = 1999;
const SEED_BASE: u64 = 0x9E37_79B9_7F4A_7C15;
const MIN_SEG: usize = 3;

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / DAY_S;
    (jd - 2440587.5).round() as i64
}

fn civil_str(day: i64) -> String {
    match omegaflow::spectral::civil_from_days(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day {day}"),
    }
}

fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}

fn mean(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    Some(vals.iter().sum::<f64>() / vals.len() as f64)
}

fn sd_sample(vals: &[f64]) -> Option<f64> {
    if vals.len() < 2 {
        return None;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    let v = vals.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (vals.len() - 1) as f64;
    Some(v.sqrt())
}

fn next_rng(rng: &mut u64) -> u64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *rng >> 33
}

fn seed_for(mode: i64, st: i64, salt: u64) -> u64 {
    SEED_BASE
        .wrapping_add((mode as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15))
        .wrapping_add((st as u64).wrapping_mul(0x0F0F_0F0F_0F0F_0F0F))
        .wrapping_add(salt)
}

fn ols_line(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    let n = xs.len();
    if n < 2 {
        return None;
    }
    let nn = n as f64;
    let mx = xs.iter().sum::<f64>() / nn;
    let my = ys.iter().sum::<f64>() / nn;
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for (x, y) in xs.iter().zip(ys.iter()) {
        sxx += (x - mx) * (x - mx);
        sxy += (x - mx) * (y - my);
    }
    if sxx <= 0.0 {
        return None;
    }
    let b = sxy / sxx;
    Some((my - b * mx, b))
}

#[derive(Clone)]
struct Cell {
    n: usize,
    sum: f64,
    sum2: f64,
    vals: Vec<f64>,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            n: 0,
            sum: 0.0,
            sum2: 0.0,
            vals: Vec::new(),
        }
    }
}

struct Row {
    day: i64,
    n: usize,
    mean: f64,
    med: f64,
    rms: f64,
    run: usize,
}

struct BestCut {
    c: usize,
    nl: usize,
    nr: usize,
    ml: f64,
    mr: f64,
    sp: f64,
    t: f64,
}

fn best_cut(levels: &[f64]) -> Option<BestCut> {
    let n = levels.len();
    if n < 2 * MIN_SEG {
        return None;
    }
    let mut best: Option<BestCut> = None;
    let mut best_jump = -1.0f64;
    for c in MIN_SEG..=(n - MIN_SEG) {
        let l = &levels[..c];
        let r = &levels[c..];
        let (ml, mr) = match (mean(l), mean(r)) {
            (Some(a), Some(b)) => (a, b),
            _ => continue,
        };
        let j = (mr - ml).abs();
        if j > best_jump {
            let (sl, sr) = match (sd_sample(l), sd_sample(r)) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            };
            best_jump = j;
            let df = (l.len() + r.len() - 2) as f64;
            let sp = (((l.len() - 1) as f64 * sl * sl + (r.len() - 1) as f64 * sr * sr)
                / df)
            .sqrt();
            let denom = sp * (1.0 / l.len() as f64 + 1.0 / r.len() as f64).sqrt();
            let t = if denom > 0.0 { (mr - ml) / denom } else { f64::INFINITY };
            best = Some(BestCut {
                c,
                nl: l.len(),
                nr: r.len(),
                ml,
                mr,
                sp,
                t,
            });
        }
    }
    best
}

fn max_step(levels: &[f64]) -> f64 {
    let n = levels.len();
    if n < 2 * MIN_SEG {
        return 0.0;
    }
    let mut pref = vec![0.0f64; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + levels[i];
    }
    let mut best = 0.0f64;
    for c in MIN_SEG..=(n - MIN_SEG) {
        let ml = pref[c] / c as f64;
        let mr = (pref[n] - pref[c]) / (n - c) as f64;
        let d = (ml - mr).abs();
        if d > best {
            best = d;
        }
    }
    best
}

fn jump_at(levels: &[f64], c: usize) -> f64 {
    let (l, r) = levels.split_at(c);
    let ml = l.iter().sum::<f64>() / l.len() as f64;
    let mr = r.iter().sum::<f64>() / r.len() as f64;
    mr - ml
}

fn block_perm_p(runs: &[(usize, usize)], levels: &[f64], obs: f64, seed: u64) -> f64 {
    let nb = runs.len();
    let n = levels.len();
    if nb < 2 {
        return f64::NAN;
    }
    let mut rng = seed;
    let mut order: Vec<usize> = (0..nb).collect();
    let mut buf = vec![0.0f64; n];
    let mut cnt = 0usize;
    for _ in 0..N_PERM {
        for i in (1..nb).rev() {
            let j = (next_rng(&mut rng) as usize) % (i + 1);
            order.swap(i, j);
        }
        let mut k = 0usize;
        for &oi in &order {
            let (s, l) = runs[oi];
            buf[k..k + l].copy_from_slice(&levels[s..s + l]);
            k += l;
        }
        if max_step(&buf) >= obs {
            cnt += 1;
        }
    }
    (cnt as f64 + 1.0) / (N_PERM as f64 + 1.0)
}

fn fmt_p(p: f64) -> String {
    if p < 0.00005 {
        format!("{p:.1e}")
    } else {
        format!("{p:.4}")
    }
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.4}"),
        Some(x) => format!("{x}"),
        None => "-".to_string(),
    }
}

fn rec(out: &mut Vec<String>, s: String) {
    out.push(s);
}

fn runs_of(rows: &[Row]) -> Vec<(usize, usize)> {
    let n_runs = rows.iter().map(|r| r.run).max().map_or(0, |m| m + 1);
    let mut runs: Vec<(usize, usize)> = Vec::new();
    for rid in 0..n_runs {
        let start = rows.iter().position(|r| r.run == rid).expect("run present");
        let end = rows.iter().rposition(|r| r.run == rid).expect("run present");
        runs.push((start, end - start + 1));
    }
    runs
}

fn series_block(out: &mut Vec<String>, mode: i64, st: i64, rows: &[Row], era0: i64) -> Option<String> {
    let n = rows.len();
    if n < 6 {
        rec(
            out,
            format!("M{mode} st{st}: quiet robust days {n} < 6 — step scan void (0 honored)"),
        );
        return None;
    }
    let days: Vec<f64> = rows.iter().map(|r| (r.day - era0) as f64).collect();
    let meds: Vec<f64> = rows.iter().map(|r| r.med).collect();
    let means: Vec<f64> = rows.iter().map(|r| r.mean).collect();
    let runs = runs_of(rows);
    let n_runs = runs.len();

    let span_d = rows[n - 1].day - rows[0].day;
    let (g_a, g_b) = match ols_line(&days, &meds) {
        Some(v) => v,
        None => (f64::NAN, f64::NAN),
    };
    let slope_txt = if g_b.is_finite() {
        format!("global OLS slope {g_b:.3e} Hz/day")
    } else {
        "global OLS slope void (flat level series)".to_string()
    };

    rec(out, String::new());
    rec(
        out,
        format!(
            "M{mode} st{st} quiet level series: {n} days, {n_runs} runs, span {span_d} d | {slope_txt}",
        ),
    );

    rec(
        out,
        "day rows (date | cell-n | run | daily-med Hz | daily-mean Hz | daily-RMS Hz)".to_string(),
    );
    for r in rows {
        rec(
            out,
            format!(
                "{:10} | {:<6} | r{} | {:+.4} | {:+.4} | {:.4}",
                civil_str(r.day),
                r.n,
                r.run,
                r.med,
                r.mean,
                r.rms
            ),
        );
    }

    rec(
        out,
        "runs (plateaus of consecutive quiet days): start..end | days | med-level | mean-level | last-first med".to_string(),
    );
    for (s, l) in &runs {
        let sub = &rows[*s..*s + l];
        let medl: Vec<f64> = sub.iter().map(|r| r.med).collect();
        let meanl: Vec<f64> = sub.iter().map(|r| r.mean).collect();
        let d0 = sub[0].day;
        let d1 = sub[l - 1].day;
        let lastfirst = if *l > 1 {
            format!("{:+.4}", sub[l - 1].med - sub[0].med)
        } else {
            "-".to_string()
        };
        rec(
            out,
            format!(
                "run {}: {}..{} | {l} d | med {} | mean {} | last-first {}",
                *s,
                civil_str(d0),
                civil_str(d1),
                median(&medl).map_or("-".to_string(), |v| format!("{v:.4}")),
                median(&meanl).map_or("-".to_string(), |v| format!("{v:.4}")),
                lastfirst
            ),
        );
    }

    let mut adj: Vec<(f64, String)> = Vec::new();
    for (s, l) in &runs {
        for i in *s..*s + l - 1 {
            let d = (rows[i + 1].med - rows[i].med).abs();
            let label = format!(
                "{} -> {}",
                civil_str(rows[i].day),
                civil_str(rows[i + 1].day)
            );
            adj.push((d, label));
        }
    }
    if !adj.is_empty() {
        let mut ad: Vec<f64> = adj.iter().map(|x| x.0).collect();
        ad.sort_by(f64::total_cmp);
        let med_adj = median(&ad).expect("adjacent deltas present");
        let mut mxx = 0.0f64;
        let mut mxl = String::new();
        for (d, l) in &adj {
            if *d > mxx {
                mxx = *d;
                mxl = l.clone();
            }
        }
        rec(
            out,
            format!(
                "within-run day-to-day |level change| (daily-median): n {} | median {med_adj:.4} Hz | max {mxx:.4} Hz ({mxl})",
                ad.len()
            ),
        );
    } else {
        rec(
            out,
            "within-run day-to-day |level change| (daily-median): none — all runs single-day (0 honored)".to_string(),
        );
    }

    let bcut = match best_cut(&meds) {
        Some(bc) => bc,
        None => {
            rec(
                out,
                format!(
                    "single step scan: {n} days < 2*{MIN_SEG} — no cut feasible (0 honored)"
                ),
            );
            return None;
        }
    };
    let lday = rows[bcut.c - 1].day;
    let rday = rows[bcut.c].day;
    let gap = rday - lday;
    let jump_means = jump_at(&means, bcut.c);
    let detrended: Vec<f64> = rows
        .iter()
        .map(|r| r.med - (g_a + g_b * (r.day - era0) as f64))
        .collect();
    let jump_detr = jump_at(&detrended, bcut.c);
    let sl_l = ols_line(&days[..bcut.c], &meds[..bcut.c]).map(|(_, b)| b);
    let sl_r = ols_line(&days[bcut.c..], &meds[bcut.c..]).map(|(_, b)| b);
    let obs = (bcut.mr - bcut.ml).abs();
    let p_perm = block_perm_p(&runs, &meds, obs, seed_for(mode, st, 0x55));
    let jump = bcut.mr - bcut.ml;
    rec(
        out,
        format!(
            "single step scan (daily-median, min {MIN_SEG} days per segment): best cut {}<->{} (day gap {gap} d) | nL {} nR {} | levelL {:+.4} levelR {:+.4} | jump (R-L) {jump:+.4} Hz | pooled within-seg sd {:.4} | t {:.2} | run-block perm p {}",
            civil_str(lday),
            civil_str(rday),
            bcut.nl,
            bcut.nr,
            bcut.ml,
            bcut.mr,
            bcut.sp,
            bcut.t,
            fmt_p(p_perm)
        ),
    );
    rec(
        out,
        format!(
            "  same cut: jump on daily-mean levels {jump_means:+.4} Hz | jump on trend-detrended daily-median {jump_detr:+.4} Hz | inner OLS slope left {} right {} Hz/day",
            fmt_opt(sl_l),
            fmt_opt(sl_r)
        ),
    );

    let pre_meds: Vec<f64> = meds[..bcut.c].to_vec();
    let post_meds = &meds[bcut.c..];
    if let Some(premed) = median(&pre_meds) {
        let pre_max = pre_meds.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let pre_min = pre_meds.iter().cloned().fold(f64::INFINITY, f64::min);
        let post_max = post_meds.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let n_ret = post_meds.iter().filter(|v| **v >= premed).count();
        rec(
            out,
            format!(
                "  return-regime: pre-step n {} levels med {premed:+.4} (min {pre_min:+.4}, max {pre_max:+.4}) | post-step n {} max {post_max:+.4} | post days at/above pre-step median {n_ret} of {}",
                pre_meds.len(),
                post_meds.len(),
                post_meds.len()
            ),
        );
    }
    let offs: Vec<&Row> = rows.iter().filter(|r| r.med.abs() >= 1.0).collect();
    if offs.is_empty() {
        rec(out, "coherent-offset days (|daily-median| >= 1 Hz at quiet RMS): none".to_string());
    }
    for o in &offs {
        let idx = rows.iter().position(|r| r.day == o.day).expect("offset row found");
        let lo = idx.saturating_sub(4);
        let hi = (idx + 5).min(rows.len());
        rec(
            out,
            format!(
                "coherent-offset day {} | daily-med {:+.4} | daily-mean {:+.4} | rms {:.4} | cell-n {} | run {} | context:",
                civil_str(o.day),
                o.med,
                o.mean,
                o.rms,
                o.n,
                o.run
            ),
        );
        for k in lo..hi {
            let r = &rows[k];
            let tag = if k == idx {
                "this day".to_string()
            } else if r.run == o.run {
                format!("run {}", r.run)
            } else {
                format!("run {} ({} d after/before)", r.run, (r.day - o.day).abs())
            };
            rec(
                out,
                format!(
                    "   {} {:10} | med {:+.4} | mean {:+.4} | rms {:.4} | {tag}",
                    if k < idx { "prev" } else if k > idx { "next" } else { "===" },
                    civil_str(r.day),
                    r.med,
                    r.mean,
                    r.rms
                ),
            );
        }
    }

    Some(format!(
        "M{mode} st{st} | quiet n {n} | best step {}<->{} | nL {} nR {} | jump {jump:+.4} Hz | pooled-sd {:.4} | t {:.2} | p_perm {}",
        civil_str(lday),
        civil_str(rday),
        bcut.nl,
        bcut.nr,
        bcut.sp,
        bcut.t,
        fmt_p(p_perm)
    ))
}

fn main() {
    let mut report = "/tmp/opencode/galileo_floor_basis_ruck_report.txt".to_string();
    for a in std::env::args().skip(1) {
        if let Some(r) = a.strip_prefix("--report=") {
            report = r.to_string();
        }
    }
    let path = "data/pds-ppi.igpp.ucla.edu/galileo_resid.bin";
    let Ok(bytes) = fs::read(path) else {
        eprintln!("{path}: resid bin void");
        return;
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        eprintln!("{path}: resid bin parse void");
        return;
    };
    drop(bytes);

    let era0 = days_from_civil(ERA0_C.0, ERA0_C.1, ERA0_C.2);
    let era1 = days_from_civil(ERA1_C.0, ERA1_C.1, ERA1_C.2);

    let mut cells: BTreeMap<(i64, i64, i64), Cell> = BTreeMap::new();
    let mut n_lock = 0usize;
    let mut n_floor_sample = 0usize;
    for r in &recs {
        let mode = r[3] as i64;
        let st = r[2] as i64;
        if !(1..=3).contains(&mode) || !STATIONS.contains(&st) {
            continue;
        }
        let day = unix_day(r[0]);
        if day < era0 || day > era1 {
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() {
            continue;
        }
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        if r[7] as i64 != FLOOR {
            continue;
        }
        n_floor_sample += 1;
        let key = (mode, day, st);
        let c = cells.entry(key).or_insert_with(Cell::new);
        c.n += 1;
        c.sum += resid;
        c.sum2 += resid * resid;
        c.vals.push(resid);
    }
    drop(recs);

    let mut out: Vec<String> = Vec::new();
    rec(
        &mut out,
        format!("galileo floor quiet-basis step/ruck probe — {path}"),
    );
    rec(&mut out, format!(
        "floor era {} .. {} (daycells {era0}..{era1}); floor = strength == {FLOOR} (AGC clamp); robust cell = (mode, station, day) n >= {MIN_CELL} over in-track (finite, |resid| <= {LOCK_HZ:.0} Hz) floor samples; quiet = cell RMS < {LOUD_HZ:.0} Hz, loud = >= {LOUD_HZ:.0} Hz (out-of-lock ground noise, held separate); day = round-of-tdb civil day; day level = daily median / daily mean of the cell resid (level axis, not RMS); step scan = single-cut two-segment piecewise-constant model on the quiet-day level series, cut between sequence indices, min {MIN_SEG} days per segment; permutation null = the level blocks (runs of consecutive quiet days) are reshuffled in time (1999 draws), statistic = max |segment-mean jump|",
        civil_str(era0),
        civil_str(era1),
    ));

    let mut tot_rob = 0usize;
    let mut tot_loud = 0usize;
    for &mode in &[1i64, 2, 3] {
        for &st in &STATIONS {
            let mut rows: Vec<Row> = Vec::new();
            for ((m, d, s), c) in &cells {
                if *m == mode && *s == st {
                    let mval = c.sum / c.n as f64;
                    let v = (c.sum2 / c.n as f64 - mval * mval).max(0.0);
                    let mut sv = c.vals.clone();
                    sv.sort_by(f64::total_cmp);
                    let rms = v.sqrt();
                    if c.n >= MIN_CELL && rms < LOUD_HZ {
                        rows.push(Row {
                            day: *d,
                            n: c.n,
                            mean: mval,
                            med: sv[sv.len() / 2],
                            rms,
                            run: 0,
                        });
                    }
                }
            }
            rows.sort_by_key(|r| r.day);
            let mut run_id = 0usize;
            for i in 0..rows.len() {
                if i > 0 && rows[i].day > rows[i - 1].day + 1 {
                    run_id += 1;
                }
                rows[i].run = run_id;
            }
            let quiet = rows.len();
            let robust = cells
                .iter()
                .filter(|((m, _, s), c)| *m == mode && *s == st && c.n >= MIN_CELL)
                .count();
            let loud = robust - quiet;
            tot_rob += robust;
            tot_loud += loud;
            rec(
                &mut out,
                format!(
                    "M{mode} st{st}: robust {robust} cells (loud {loud}, quiet {quiet}) | {quiet} quiet-day rows feed the level analysis",
                ),
            );
        }
    }
    rec(
        &mut out,
        format!(
            "total robust cells {tot_rob} (loud {tot_loud}, quiet {}) — floor in-track samples {n_floor_sample}, lock samples excluded {n_lock}",
            tot_rob - tot_loud
        ),
    );

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== per-series quiet level series, runs and step scan ==".to_string(),
    );
    let mut summary: Vec<String> = Vec::new();
    for &mode in &[1i64, 2, 3] {
        for &st in &STATIONS {
            let mut rows: Vec<Row> = Vec::new();
            for ((m, d, s), c) in &cells {
                if *m == mode && *s == st {
                    let mval = c.sum / c.n as f64;
                    let v = (c.sum2 / c.n as f64 - mval * mval).max(0.0);
                    let mut sv = c.vals.clone();
                    sv.sort_by(f64::total_cmp);
                    let rms = v.sqrt();
                    if c.n >= MIN_CELL && rms < LOUD_HZ {
                        rows.push(Row {
                            day: *d,
                            n: c.n,
                            mean: mval,
                            med: sv[sv.len() / 2],
                            rms,
                            run: 0,
                        });
                    }
                }
            }
            rows.sort_by_key(|r| r.day);
            let mut run_id = 0usize;
            for i in 0..rows.len() {
                if i > 0 && rows[i].day > rows[i - 1].day + 1 {
                    run_id += 1;
                }
                rows[i].run = run_id;
            }
            if let Some(s) = series_block(&mut out, mode, st, &rows, era0) {
                summary.push(s);
            }
        }
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== summary: single best step per quiet series (min 3 days per segment) ==".to_string(),
    );
    for s in &summary {
        rec(&mut out, s.clone());
    }

    let _ = fs::write(&report, out.join("\n") + "\n");
    println!("report written to {report}");
}
