use std::collections::{BTreeMap, BTreeSet};
use std::fs;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const ERA0: (i64, i64, i64) = (1995, 11, 23);
const ERA1: (i64, i64, i64) = (1997, 2, 28);
const STATIONS: [i64; 3] = [14, 43, 63];
const N_PERM: usize = 1999;
const SEED_BASE: u64 = 0x9E37_79B9_7F4A_7C15;

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

fn month_key(day: i64) -> Option<(i64, i64)> {
    let (y, m, _) = omegaflow::spectral::civil_from_days(day)?;
    Some((y as i64, m as i64))
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

fn pct(vals: &[f64], q: usize) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    let k = (s.len() - 1) * q / 4;
    Some(s[k])
}

fn gammln(xx: f64) -> f64 {
    let cof = [
        76.18009172947146,
        -86.50532032941677,
        24.01409824083091,
        -1.231739572450155,
        0.1208650973866179e-2,
        -0.5395239384953e-5,
    ];
    let x = xx;
    let mut y = xx;
    let mut tmp = x + 5.5;
    tmp -= (x + 0.5) * tmp.ln();
    let mut ser = 1.000000000190015;
    for j in 0..6 {
        y += 1.0;
        ser += cof[j] / y;
    }
    -tmp + (2.5066282746310005 * ser / x).ln()
}

fn betacf(a: f64, b: f64, x: f64) -> f64 {
    const MAXIT: usize = 400;
    const EPS: f64 = 1.0e-14;
    const FPMIN: f64 = 1.0e-300;
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < FPMIN {
        d = FPMIN;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..=MAXIT {
        let m2 = 2 * m;
        let mut aa = m as f64 * (b - m as f64) * x / ((qam + m2 as f64) * (a + m2 as f64));
        d = 1.0 + aa * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + aa / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        h *= d * c;
        let m2 = 2 * m + 1;
        aa = -(a + m as f64) * (qab + m as f64) * x / ((a + m2 as f64) * (qap + m2 as f64));
        d = 1.0 + aa * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + aa / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < EPS {
            break;
        }
    }
    h
}

fn betai(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let bt = (gammln(a + b) - gammln(a) - gammln(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        bt * betacf(a, b, x) / a
    } else {
        1.0 - bt * betacf(b, a, 1.0 - x) / b
    }
}

fn t_two_p(t: f64, df: f64) -> f64 {
    betai(df / 2.0, 0.5, df / (df + t * t))
}

fn shuffle(v: &mut [f64], rng: &mut u64) {
    for i in (1..v.len()).rev() {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = ((*rng >> 33) as usize) % (i + 1);
        v.swap(i, j);
    }
}

fn ols_slope(xs: &[f64], ys: &[f64]) -> Option<(f64, f64, f64)> {
    let n = xs.len();
    if n < 3 {
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
    let a = my - b * mx;
    let mut rss = 0.0;
    for (x, y) in xs.iter().zip(ys.iter()) {
        let r = y - (a + b * x);
        rss += r * r;
    }
    let s2 = rss / (n - 2) as f64;
    let se = (s2 / sxx).sqrt();
    Some((b, se, b / se))
}

fn perm_slope_p(xs: &[f64], ys: &[f64], seed: u64) -> f64 {
    let Some((b0, _, _)) = ols_slope(xs, ys) else {
        return f64::NAN;
    };
    let mut rng = seed;
    let mut yp = ys.to_vec();
    let mut cnt = 0usize;
    for _ in 0..N_PERM {
        shuffle(&mut yp, &mut rng);
        if let Some((bp, _, _)) = ols_slope(xs, &yp) {
            if bp.abs() >= b0.abs() {
                cnt += 1;
            }
        }
    }
    (cnt as f64 + 1.0) / (N_PERM as f64 + 1.0)
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

#[derive(Clone, Copy)]
struct DayRow {
    day: i64,
    n: usize,
    mean: f64,
    med: f64,
    rms: f64,
}

fn seed_for(mode: i64, st: i64, salt: u64) -> u64 {
    SEED_BASE
        .wrapping_add((mode as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15))
        .wrapping_add((st as u64).wrapping_mul(0x0F0F_0F0F_0F0F_0F0F))
        .wrapping_add(salt)
}

fn fmt_p(p: f64) -> String {
    if p < 0.00005 {
        format!("{p:.1e}")
    } else {
        format!("{p:.4}")
    }
}

fn fmt_stat(vals: &[f64]) -> String {
    if vals.is_empty() {
        return "-".to_string();
    }
    let fmt = |v: Option<f64>| match v {
        Some(x) if x.is_finite() => format!("{x:.4}"),
        _ => "-".to_string(),
    };
    format!(
        "mean {} sd {} med {} p25 {} p75 {} min {:.4} max {:.4}",
        fmt(mean(vals)),
        fmt(sd_sample(vals)),
        fmt(median(vals)),
        fmt(pct(vals, 1)),
        fmt(pct(vals, 3)),
        vals.iter().cloned().fold(f64::INFINITY, f64::min),
        vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    )
}

fn rec(out: &mut Vec<String>, s: String) {
    out.push(s);
}

fn quiet_level_fit(out: &mut Vec<String>, rows: &[&DayRow], mode: i64, st: i64, era0: i64) {
    let days: Vec<i64> = rows.iter().map(|r| r.day).collect();
    if days.len() < 3 {
        rec(
            out,
            format!(
                "M{mode} st{st} quiet level: n days {} < 3 — no trend fit (0 honored)",
                days.len()
            ),
        );
        return;
    }
    let xs: Vec<f64> = days.iter().map(|d| (*d - era0) as f64).collect();
    let span = days[days.len() - 1] - days[0];
    let months: BTreeSet<(i64, i64)> = days.iter().filter_map(|d| month_key(*d)).collect();
    for kind in ["daily-median", "daily-mean"] {
        let vals: Vec<f64> = rows
            .iter()
            .map(|r| {
                if kind == "daily-median" {
                    r.med
                } else {
                    r.mean
                }
            })
            .collect();
        let salt = if kind == "daily-median" {
            0x11u64
        } else {
            0x12u64
        };
        let (b, se, t) = ols_slope(&xs, &vals).expect("level OLS present");
        let p_t = t_two_p(t, (xs.len() - 2) as f64);
        let p_perm = perm_slope_p(&xs, &vals, seed_for(mode, st, salt));
        rec(
            out,
            format!(
                "M{mode} st{st} quiet {kind}: n days {} | months {} | span {span} d | slope {b:.3e} Hz/day | SE {se:.3e} | t {t:.2} | p_t {} | p_perm {} | drift over span {:.3e} Hz | {}..{}",
                xs.len(),
                months.len(),
                fmt_p(p_t),
                fmt_p(p_perm),
                b * span as f64,
                civil_str(days[0]),
                civil_str(days[days.len() - 1])
            ),
        );
    }
}

fn quiet_floor_fit(out: &mut Vec<String>, rows: &[&DayRow], mode: i64, st: i64, era0: i64) {
    let days: Vec<i64> = rows.iter().map(|r| r.day).collect();
    if days.len() < 3 {
        rec(
            out,
            format!(
                "M{mode} st{st} quiet floor: n days {} < 3 — no trend fit (0 honored)",
                days.len()
            ),
        );
        return;
    }
    let xs: Vec<f64> = days.iter().map(|d| (*d - era0) as f64).collect();
    let span = (days[days.len() - 1] - days[0]) as f64;
    let rms: Vec<f64> = rows.iter().map(|r| r.rms).collect();
    let pos: Vec<&DayRow> = rows.iter().filter(|r| r.rms > 0.0).copied().collect();
    let (b, se, t) = ols_slope(&xs, &rms).expect("rms OLS present");
    let p_t = t_two_p(t, (xs.len() - 2) as f64);
    let p_perm = perm_slope_p(&xs, &rms, seed_for(mode, st, 0x22));
    let log_tag = if pos.len() >= 3 {
        let xsp: Vec<f64> = pos.iter().map(|r| (r.day - era0) as f64).collect();
        let lrms: Vec<f64> = pos.iter().map(|r| r.rms.ln()).collect();
        let (lb, _, _) = ols_slope(&xsp, &lrms).expect("log-rms OLS present");
        format!(
            "log10-RMS slope {:.3e} /day (n {})",
            lb / std::f64::consts::LN_10,
            pos.len()
        )
    } else {
        format!(
            "log10-RMS slope absent (rms>0 on {} of {} days)",
            pos.len(),
            rms.len()
        )
    };
    rec(
        out,
        format!(
            "M{mode} st{st} quiet floor (day-RMS): n days {} | slope {b:.3e} Hz/day (SE {se:.3e}) | p_t {} | p_perm {} | drift over span {:.3e} Hz | {log_tag} | rms med {}",
            xs.len(),
            fmt_p(p_t),
            fmt_p(p_perm),
            b * span,
            median(&rms).map_or("-".to_string(), |v| format!("{v:.4}"))
        ),
    );
}

fn quiet_month_blocks(out: &mut Vec<String>, rows: &[&DayRow], mode: i64, st: i64) {
    let mut by_month: BTreeMap<(i64, i64), Vec<&DayRow>> = BTreeMap::new();
    for r in rows {
        if let Some(k) = month_key(r.day) {
            by_month.entry(k).or_default().push(r);
        }
    }
    for ((y, m), sub) in by_month {
        let meds: Vec<f64> = sub.iter().map(|r| r.med).collect();
        let means: Vec<f64> = sub.iter().map(|r| r.mean).collect();
        let rms: Vec<f64> = sub.iter().map(|r| r.rms).collect();
        let d0 = days_from_civil(y, m, 1);
        rec(
            out,
            format!(
                "M{mode} st{st} | {} | quiet days n {} | med-level {} | mean-level {} | rms-med {}",
                civil_str(d0),
                sub.len(),
                median(&meds).map_or("-".to_string(), |v| format!("{v:.4}")),
                median(&means).map_or("-".to_string(), |v| format!("{v:.4}")),
                median(&rms).map_or("-".to_string(), |v| format!("{v:.4}"))
            ),
        );
    }
}

fn subset_median_level_fit(
    out: &mut Vec<String>,
    rows: &[&DayRow],
    mode: i64,
    st: i64,
    era0: i64,
    from_day: i64,
    label: &str,
) {
    let sub: Vec<&DayRow> = rows.iter().copied().filter(|r| r.day >= from_day).collect();
    if sub.len() < 3 {
        rec(
            out,
            format!(
                "M{mode} st{st} quiet daily-median {label}: n days {} < 3 — no trend fit (0 honored)",
                sub.len()
            ),
        );
        return;
    }
    let days: Vec<i64> = sub.iter().map(|r| r.day).collect();
    let xs: Vec<f64> = days.iter().map(|d| (*d - era0) as f64).collect();
    let vals: Vec<f64> = sub.iter().map(|r| r.med).collect();
    let (b, se, t) = ols_slope(&xs, &vals).expect("subset OLS present");
    let p_t = t_two_p(t, (xs.len() - 2) as f64);
    let p_perm = perm_slope_p(&xs, &vals, seed_for(mode, st, 0x44));
    let span = (days[days.len() - 1] - days[0]) as f64;
    let months: BTreeSet<(i64, i64)> = days.iter().filter_map(|d| month_key(*d)).collect();
    rec(
        out,
        format!(
            "M{mode} st{st} quiet daily-median {label}: n days {} | months {} | span {:.0} d | slope {b:.3e} Hz/day | SE {se:.3e} | t {t:.2} | p_t {} | p_perm {} | drift over span {:.3e} Hz",
            xs.len(),
            months.len(),
            span,
            fmt_p(p_t),
            fmt_p(p_perm),
            b * span
        ),
    );
}

fn main() {
    let mut report = "/tmp/opencode/galileo_floor_basis_drift_report.txt".to_string();
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

    let era0 = days_from_civil(ERA0.0, ERA0.1, ERA0.2);
    let era1 = days_from_civil(ERA1.0, ERA1.1, ERA1.2);

    let mut cells: BTreeMap<(i64, i64, i64), Cell> = BTreeMap::new();
    let mut n_lock = 0usize;
    let mut n_floor_sample = 0usize;
    let mut month_presence: BTreeSet<(i64, i64)> = BTreeSet::new();
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
        if let Some((y, m)) = month_key(day) {
            month_presence.insert((y, m));
        }
        let key = (mode, day, st);
        let c = cells.entry(key).or_insert_with(Cell::new);
        c.n += 1;
        c.sum += resid;
        c.sum2 += resid * resid;
        c.vals.push(resid);
    }
    drop(recs);

    let present: Vec<(i64, i64)> = month_presence.iter().copied().collect();
    let holes: Vec<String> = present
        .windows(2)
        .filter(|w| {
            let k1 = w[0].0 * 12 + w[0].1;
            let k2 = w[1].0 * 12 + w[1].1;
            k2 - k1 > 1
        })
        .map(|w| {
            let k1 = w[0].0 * 12 + w[0].1 + 1;
            let k2 = w[1].0 * 12 + w[1].1 - 1;
            let y1 = k1 / 12;
            let m1 = k1 % 12;
            let y2 = k2 / 12;
            let m2 = k2 % 12;
            format!(
                "{} to {}",
                civil_str(days_from_civil(y1, m1, 1)),
                civil_str(days_from_civil(y2, m2, 1))
            )
        })
        .collect();

    let mut series: Vec<(i64, i64, Vec<DayRow>, usize, usize)> = Vec::new();
    for &mode in &[1i64, 2, 3] {
        for &st in &STATIONS {
            let mut rows: Vec<DayRow> = Vec::new();
            let mut thin = 0usize;
            let mut total_samp = 0usize;
            for ((m, d, s), c) in &cells {
                if *m != mode || *s != st {
                    continue;
                }
                total_samp += c.n;
                if c.n < MIN_CELL {
                    thin += 1;
                    continue;
                }
                let mval = c.sum / c.n as f64;
                let v = (c.sum2 / c.n as f64 - mval * mval).max(0.0);
                let mut sv = c.vals.clone();
                sv.sort_by(f64::total_cmp);
                let med = sv[sv.len() / 2];
                rows.push(DayRow {
                    day: *d,
                    n: c.n,
                    mean: mval,
                    med,
                    rms: v.sqrt(),
                });
            }
            rows.sort_by_key(|r| r.day);
            series.push((mode, st, rows, thin, total_samp));
        }
    }

    let mut out: Vec<String> = Vec::new();
    rec(
        &mut out,
        format!("galileo floor quiet-basis drift probe — {path}"),
    );
    rec(&mut out, format!(
        "floor era {} .. {} (daycells {}..{}); floor = strength == {FLOOR} (AGC clamp); floor cell = (mode, station, day) over in-track (finite, |resid| <= {LOCK_HZ:.0} Hz) floor samples; robust = cell n >= {MIN_CELL}; loud = cell RMS >= {LOUD_HZ:.0} Hz, quiet = RMS < {LOUD_HZ:.0} Hz; day = round-of-tdb civil day (register convention); day-level resid value = daily mean / daily median of the cell resid (level, not scatter); day-level RMS = scatter about the cell mean; trend fit: OLS of the day value vs days since era0, SE from the residual variance, p from Student-t (two-sided, n-2 df) and from 1999 y-permutations; x spans the calendar gaps as empty days (0 honored)",
        civil_str(era0),
        civil_str(era1),
        era0,
        era1
    ));

    rec(&mut out, String::new());
    rec(&mut out, format!(
        "sample census: floor in-track samples (st 14/43/63, mode 1..3, era) {n_floor_sample}; lock samples excluded {n_lock}; months with any floor sample {}; empty month spans: {}",
        month_presence.len(),
        if holes.is_empty() { "none".to_string() } else { holes.join("; ") }
    ));

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== 0. register reproduction on the robust floor cells (n >= 30) ==".to_string(),
    );
    let mut tot_rob = 0usize;
    let mut tot_loud = 0usize;
    for (mode, st, rows, thin, tsamp) in &series {
        let loud = rows.iter().filter(|c| c.rms >= LOUD_HZ).count();
        let quiet = rows.len() - loud;
        tot_rob += rows.len();
        tot_loud += loud;
        rec(
            &mut out,
            format!(
                "M{mode} st{st}: robust days {} (loud {} quiet {}) | thin days (<{MIN_CELL}) {} | floor samples of the (mode,st) cells {tsamp}",
                rows.len(),
                loud,
                quiet,
                thin
            ),
        );
    }
    rec(
        &mut out,
        format!(
            "total robust days {tot_rob}, loud {tot_loud}, quiet {} (register reference: 400 robust, 207 loud, 193 quiet on the n >= 30 floor cells)",
            tot_rob - tot_loud
        ),
    );

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== 1. day-level resid series (level, not RMS) — offset and spread ==".to_string(),
    );
    rec(
        &mut out,
        "series | class | n days | member samples | daily-median-level {mean sd med p25 p75 min max} | daily-mean-level {mean sd med p25 p75 min max} | member-level (n-weighted daily mean) Hz | daily-RMS med (quiet floor) Hz".to_string(),
    );
    for (mode, st, rows, _, _) in &series {
        if rows.is_empty() {
            rec(
                &mut out,
                format!("M{mode} st{st}: no robust floor day (0 honored)"),
            );
            continue;
        }
        let quiet: Vec<&DayRow> = rows.iter().filter(|r| r.rms < LOUD_HZ).collect();
        let allr: Vec<&DayRow> = rows.iter().collect();
        let nq: usize = quiet.iter().map(|r| r.n).sum();
        let na: usize = allr.iter().map(|r| r.n).sum();
        let quiet_meds: Vec<f64> = quiet.iter().map(|r| r.med).collect();
        let quiet_means: Vec<f64> = quiet.iter().map(|r| r.mean).collect();
        let all_meds: Vec<f64> = allr.iter().map(|r| r.med).collect();
        let all_means: Vec<f64> = allr.iter().map(|r| r.mean).collect();
        let quiet_rms: Vec<f64> = quiet.iter().map(|r| r.rms).collect();
        let all_rms: Vec<f64> = allr.iter().map(|r| r.rms).collect();
        let member_level = |rows: &[&DayRow]| -> String {
            if rows.is_empty() {
                return "-".to_string();
            }
            let n: usize = rows.iter().map(|r| r.n).sum();
            let s: f64 = rows.iter().map(|r| r.mean * r.n as f64).sum();
            format!("{:.4}", s / n as f64)
        };
        rec(
            &mut out,
            format!(
                "M{mode} st{st} quiet  | n days {:<3} | samples {:<8} | daily-med {} | daily-mean {} | member-level {} | rms-med {}",
                quiet.len(),
                nq,
                fmt_stat(&quiet_meds),
                fmt_stat(&quiet_means),
                member_level(&quiet),
                median(&quiet_rms).map_or("-".to_string(), |v| format!("{v:.4}"))
            ),
        );
        rec(
            &mut out,
            format!(
                "M{mode} st{st} all    | n days {:<3} | samples {:<8} | daily-med {} | daily-mean {} | member-level {} | rms-med {}",
                allr.len(),
                na,
                fmt_stat(&all_meds),
                fmt_stat(&all_means),
                member_level(&allr),
                median(&all_rms).map_or("-".to_string(), |v| format!("{v:.4}"))
            ),
        );
        let n_co = quiet.iter().filter(|r| r.med.abs() >= 1.0).count();
        if n_co > 0 {
            rec(
                &mut out,
                format!("M{mode} st{st} quiet: coherent-offset days (|daily-median| >= 1 Hz, day-RMS < 1 Hz) {n_co}"),
            );
        }
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== 2. linear trend of the quiet day-level resid (level, Hz) vs time ==".to_string(),
    );
    for (mode, st, rows, _, _) in &series {
        let quiet: Vec<&DayRow> = rows.iter().filter(|r| r.rms < LOUD_HZ).collect();
        quiet_level_fit(&mut out, &quiet, *mode, *st, era0);
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== 3. trend of the quiet day-level RMS (the quiet floor, Hz) vs time ==".to_string(),
    );
    for (mode, st, rows, _, _) in &series {
        let quiet: Vec<&DayRow> = rows.iter().filter(|r| r.rms < LOUD_HZ).collect();
        quiet_floor_fit(&mut out, &quiet, *mode, *st, era0);
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== 4. whole-floor day-level series (all robust days incl loud) — context, named as not the quiet basis ==".to_string(),
    );
    for (mode, st, rows, _, _) in &series {
        let allr: Vec<&DayRow> = rows.iter().collect();
        let days: Vec<i64> = allr.iter().map(|r| r.day).collect();
        if days.len() < 3 {
            rec(
                &mut out,
                format!(
                    "M{mode} st{st} all: n days {} < 3 — no trend fit (0 honored)",
                    days.len()
                ),
            );
            continue;
        }
        let xs: Vec<f64> = days.iter().map(|d| (*d - era0) as f64).collect();
        let vals: Vec<f64> = allr.iter().map(|r| r.mean).collect();
        let vmed: Vec<f64> = allr.iter().map(|r| r.med).collect();
        let (b, se, t) = ols_slope(&xs, &vals).expect("all-mean OLS present");
        let p_t = t_two_p(t, (xs.len() - 2) as f64);
        let p_perm = perm_slope_p(&xs, &vals, seed_for(*mode, *st, 0x33));
        let (bm, _, _) = ols_slope(&xs, &vmed).expect("all-med OLS present");
        let span = (days[days.len() - 1] - days[0]) as f64;
        rec(
            &mut out,
            format!(
                "M{mode} st{st} all: n days {} | mean-level slope {b:.3e} Hz/day (SE {se:.3e}) | p_t {} | p_perm {} | med-level slope {bm:.3e} | drift over span {:.3e} Hz",
                xs.len(),
                fmt_p(p_t),
                fmt_p(p_perm),
                b * span
            ),
        );
    }

    let _ = fs::write(&report, out.join("\n") + "\n");
    rec(&mut out, String::new());
    rec(
        &mut out,
        "== 5. quiet-day level structure by calendar month (step vs ramp check) ==".to_string(),
    );
    rec(
        &mut out,
        "series | month | quiet days n | med-of-daily-median level Hz | med-of-daily-mean level Hz | rms-med Hz".to_string(),
    );
    for (mode, st, rows, _, _) in &series {
        let quiet: Vec<&DayRow> = rows.iter().filter(|r| r.rms < LOUD_HZ).collect();
        if quiet.is_empty() {
            rec(
                &mut out,
                format!("M{mode} st{st}: no quiet day (0 honored)"),
            );
            continue;
        }
        quiet_month_blocks(&mut out, &quiet, *mode, *st);
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== 6. quiet daily-median level trend on the late body only (drift within the 1996-97 body, past the 1995 cluster) ==".to_string(),
    );
    let cut_a = days_from_civil(1996, 1, 1);
    let cut_b = days_from_civil(1996, 6, 1);
    for (mode, st, rows, _, _) in &series {
        let quiet: Vec<&DayRow> = rows.iter().filter(|r| r.rms < LOUD_HZ).collect();
        subset_median_level_fit(
            &mut out,
            &quiet,
            *mode,
            *st,
            era0,
            cut_a,
            "body>=1996-01-01",
        );
        subset_median_level_fit(
            &mut out,
            &quiet,
            *mode,
            *st,
            era0,
            cut_b,
            "body>=1996-06-01",
        );
    }

    let _ = fs::write(&report, out.join("\n") + "\n");
    println!("report written to {report}");
    for (mode, st, rows, _, _) in &series {
        println!("series M{mode} st{st}: robust {} days", rows.len());
    }
}
