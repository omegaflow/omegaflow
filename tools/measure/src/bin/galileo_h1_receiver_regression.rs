use std::collections::{BTreeMap, BTreeSet};
use std::fs;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const MIN_SIDE: usize = 3;
const ERA0: (i64, i64, i64) = (1995, 11, 23);
const ERA1: (i64, i64, i64) = (1997, 2, 28);

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

fn month_key(day: i64) -> i64 {
    match omegaflow::spectral::civil_from_days(day) {
        Some((y, m, _)) => (y as i64) * 100 + m as i64,
        None => 0,
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

fn mean_log10(vals: &[f64]) -> Option<f64> {
    let mut acc = 0.0;
    let mut n = 0usize;
    for v in vals {
        if v.is_finite() && *v > 0.0 {
            acc += v.log10();
            n += 1;
        }
    }
    if n > 0 {
        Some(acc / n as f64)
    } else {
        None
    }
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
    let y = xx;
    let tmp = x + 5.5;
    let tmp = (x + 0.5) * tmp.ln() - tmp;
    let ser = 1.000000000190015
        + cof
            .iter()
            .enumerate()
            .fold(0.0, |s, (i, c)| s + c / (y + (i + 1) as f64));
    tmp + (2.5066282746310005 * ser / x).ln()
}

fn ln_choose(n: f64, k: f64) -> f64 {
    gammln(n + 1.0) - gammln(k + 1.0) - gammln(n - k + 1.0)
}

fn fisher_two_sided(a: usize, b: usize, c: usize, d: usize) -> Option<f64> {
    let r1 = a + b;
    let r2 = c + d;
    let c1 = a + c;
    let n = r1 + r2;
    if r1 == 0 || r2 == 0 || c1 == 0 || c1 == n {
        return None;
    }
    let p_obs = ln_choose(r1 as f64, a as f64) + ln_choose(r2 as f64, c as f64)
        - ln_choose(n as f64, c1 as f64);
    let lo = if c1 > r2 { c1 - r2 } else { 0 };
    let hi = if c1 < r1 { c1 } else { r1 };
    let mut sum = 0.0;
    for a2 in lo..=hi {
        let c2 = c1 - a2;
        let p = ln_choose(r1 as f64, a2 as f64) + ln_choose(r2 as f64, c2 as f64)
            - ln_choose(n as f64, c1 as f64);
        if p <= p_obs + 1.0e-9 {
            sum += p.exp();
        }
    }
    Some(sum.min(1.0))
}

#[derive(Clone, Copy)]
struct Cell {
    mode: i64,
    day: i64,
    st: i64,
    n: usize,
    rms: f64,
}

struct Milestone {
    name: &'static str,
    day: i64,
    note: &'static str,
}

fn fmt_p(p: Option<f64>) -> String {
    match p {
        Some(v) => format!("{v:.4}"),
        None => "-".to_string(),
    }
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "/tmp/opencode/galileo_h1_receiver_regression_report.txt".to_string(),
    };

    let era0 = days_from_civil(ERA0.0, ERA0.1, ERA0.2);
    let era1 = days_from_civil(ERA1.0, ERA1.1, ERA1.2);

    let milestones: Vec<Milestone> = vec![
        Milestone {
            name: "1995-09-18 suppressed-carrier (MI 90 deg) + BVR as the 70-m standard mode",
            day: days_from_civil(1995, 9, 18),
            note: "external, TDA 42-125 p.10",
        },
        Milestone {
            name: "1995-12-05 residual-carrier (MI 58 deg) switch + special configuration table at all 70-m sites",
            day: days_from_civil(1995, 12, 5),
            note: "external, TDA 42-125 p.10-11",
        },
        Milestone {
            name: "1996-05-23 DGT (DSN Galileo Telemetry) phase-2 packet telemetry routine",
            day: days_from_civil(1996, 5, 23),
            note: "external, TDA 42-125 p.15",
        },
        Milestone {
            name: "1996-11-01 full-arraying routine (Canberra 70-m array, C3 era)",
            day: days_from_civil(1996, 11, 1),
            note: "external, TDA 42-133 p.5-7",
        },
    ];

    let context_events: Vec<Milestone> = vec![
        Milestone {
            name: "1995-05-15 BVR installed at all three DSCCs (before the floor era)",
            day: days_from_civil(1995, 5, 15),
            note: "external, TDA 42-125 p.10; no floor cells of the era precede it",
        },
        Milestone {
            name: "1995-09-11 BVR test over DSS-63, BVR fully implemented in the 70-m network",
            day: days_from_civil(1995, 9, 11),
            note: "external, TDA 42-125 p.10",
        },
        Milestone {
            name: "1996-06-27 G1 closest approach: FSR channel + BVR -> FCD single-antenna Doppler chain",
            day: days_from_civil(1996, 6, 27),
            note: "external, TDA 42-133 p.6-7; falls at the floor resumption after the Feb-May 1996 void",
        },
        Milestone {
            name: "1997-08 BVR close-out / Block V receiver completion",
            day: days_from_civil(1997, 8, 1),
            note: "external; after the floor era end 1997-02-28, not testable here",
        },
    ];

    let Ok(bytes) = fs::read("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    drop(bytes);

    let mut cell: BTreeMap<(i64, i64, i64), (f64, f64, usize)> = BTreeMap::new();
    let mut n_lock = 0usize;
    let mut n_nonfin = 0usize;
    let mut n_outside = 0usize;
    let mut n_strong = 0usize;
    for r in &recs {
        let mode = r[3] as i64;
        let st = r[2] as i64;
        if mode < 1 || mode > 3 || st != 14 && st != 43 && st != 63 {
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() {
            n_nonfin += 1;
            continue;
        }
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        if r[7] as i64 != FLOOR {
            n_strong += 1;
            continue;
        }
        let day = unix_day(r[0]);
        if day < era0 || day > era1 {
            n_outside += 1;
            continue;
        }
        let e = cell.entry((mode, day, st)).or_insert((0.0, 0.0, 0));
        e.0 += resid;
        e.1 += resid * resid;
        e.2 += 1;
    }
    drop(recs);

    let mut rows: Vec<Cell> = Vec::new();
    for (&(mode, day, st), &(sum, sum2, n)) in &cell {
        if n == 0 {
            continue;
        }
        let m = sum / n as f64;
        let v = (sum2 / n as f64 - m * m).max(0.0);
        rows.push(Cell {
            mode,
            day,
            st,
            n,
            rms: v.sqrt(),
        });
    }
    let robust: Vec<Cell> = rows.into_iter().filter(|c| c.n >= MIN_CELL).collect();
    let mut cells_sorted = robust.clone();
    cells_sorted.sort_by_key(|c| (c.mode, c.st, c.day));

    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String| {
        println!("{s}");
        out.push(s);
    };

    push("galileo h1 receiver-milestone regression probe".to_string());
    push(format!(
        "floor era {} .. {} (daycells {} .. {}); floor = strength == {FLOOR} (AGC clamp); floor cell = (mode, station, day) RMS around the cell mean; lock (|resid| > {LOCK_HZ:.0} Hz) and non-finite excluded before the cell; robust = cell samples n >= {MIN_CELL}; loud = cell RMS >= {LOUD_HZ:.0} Hz",
        civil_str(era0),
        civil_str(era1),
        era0,
        era1
    ));
    push(format!(
        "excluded before the era window: lock {n_lock}, non-finite {n_nonfin}, non-floor strength {n_strong}, outside-floor-era {n_outside}"
    ));
    push("daycell = unix day of the sample (round of tdb; register convention), civil date via civil_from_days(day)".to_string());
    push("milestones are external research dates (TDA Progress Reports 42-125 / 42-133, Beyer et al.), not measured assets:".to_string());
    for ms in &milestones {
        push(format!(
            "  TEST {} (daycell {}) — {}",
            ms.name,
            ms.day,
            ms.note
        ));
    }
    for ms in &context_events {
        push(format!(
            "  CONTEXT {} (daycell {}) — {}",
            ms.name,
            ms.day,
            ms.note
        ));
    }

    push(String::new());
    push("== 0. register reproduction on the robust series (n >= 30) ==".to_string());
    let modes = [1i64, 2, 3];
    let trio = [14i64, 43, 63];
    let mut tot_rob = 0usize;
    let mut tot_loud = 0usize;
    let mut tot_flip = 0usize;
    push("series | robust days | loud | quiet | adjacent-day flips | loud frac".to_string());
    for mode in modes {
        for st in trio {
            let sub: Vec<&Cell> = cells_sorted.iter().filter(|c| c.mode == mode && c.st == st).collect();
            let loud = sub.iter().filter(|c| c.rms >= LOUD_HZ).count();
            let quiet = sub.len() - loud;
            let mut flips = 0usize;
            for w in sub.windows(2) {
                if w[1].day - w[0].day == 1 && (w[0].rms >= LOUD_HZ) != (w[1].rms >= LOUD_HZ) {
                    flips += 1;
                }
            }
            tot_rob += sub.len();
            tot_loud += loud;
            tot_flip += flips;
            push(format!(
                "M{mode} st{st} | {} | {} | {} | {} | {:.3}",
                sub.len(),
                loud,
                quiet,
                flips,
                if sub.is_empty() { 0.0 } else { loud as f64 / sub.len() as f64 }
            ));
        }
    }
    push(format!("total | {tot_rob} | {tot_loud} | {} | {tot_flip} |", tot_rob - tot_loud));
    push("register reference: 400 robust days, 207 loud, 193 quiet, 105 flips (floor register, n >= 30)".to_string());

    push(String::new());
    push("== 1. loud incidence per station over time (monthly bins) ==".to_string());
    push("row = station and calendar month of the floor era; cells = robust (mode, station, day) cells, l/t = loud/robust per mode; station-days = distinct days with >= 1 robust cell, loud day = >= 1 loud cell; 0 cells in a month is 0 honored (no floor tracking), printed not filled".to_string());
    let (mut y, mut m) = (1995i64, 11i64);
    while (y < 1997) || (y == 1997 && m <= 2) {
        for st in trio {
            let sub: Vec<&Cell> = cells_sorted
                .iter()
                .filter(|c| c.st == st && month_key(c.day) == y * 100 + m)
                .collect();
            if sub.is_empty() {
                push(format!("st{st} {y:04}-{m:02}: 0 cells (0 honored) | 0 station-days (0 honored)"));
                continue;
            }
            let mut modes_s: Vec<String> = Vec::new();
            for md in modes {
                let msub: Vec<&&Cell> = sub.iter().filter(|c| c.mode == md).collect();
                let ml = msub.iter().filter(|c| c.rms >= LOUD_HZ).count();
                modes_s.push(format!("M{md} {ml}/{}", msub.len()));
            }
            let loud = sub.iter().filter(|c| c.rms >= LOUD_HZ).count();
            let days: BTreeSet<i64> = sub.iter().map(|c| c.day).collect();
            let mut loud_days = 0usize;
            for d in &days {
                if sub.iter().any(|c| c.day == *d && c.rms >= LOUD_HZ) {
                    loud_days += 1;
                }
            }
            push(format!(
                "st{st} {y:04}-{m:02}: robust cells {} loud {} [{}] frac cells {:.3} | station-days {} loud-days {} frac days {:.3}",
                sub.len(),
                loud,
                modes_s.join(" "),
                if sub.is_empty() { 0.0 } else { loud as f64 / sub.len() as f64 },
                days.len(),
                loud_days,
                if days.is_empty() { 0.0 } else { loud_days as f64 / days.len() as f64 }
            ));
        }
        m += 1;
        if m > 12 {
            m = 1;
            y += 1;
        }
    }

    push(String::new());
    push("== 2. milestone before/after regression on the robust series ==".to_string());
    push(format!("per (mode, station) cell split at each milestone daycell: pre = day < milestone, post = day >= milestone; loud frac = loud cells / robust cells (a series has at most one robust cell per day, so the series cell loud-frac equals its loud-day fraction); fisher = exact two-sided test on (loud, quiet) x (pre, post); p printed only when both sides have >= {MIN_SIDE} robust cells, a smaller side is data-thin and named absent"));
    for ms in &milestones {
        push(String::new());
        push(format!(
            "--- TEST {} (daycell {}) — {}",
            ms.name,
            ms.day,
            ms.note
        ));
        for mode in modes {
            for st in trio {
                let sub: Vec<&Cell> = cells_sorted
                    .iter()
                    .filter(|c| c.mode == mode && c.st == st)
                    .collect();
                let pre: Vec<&&Cell> = sub.iter().filter(|c| c.day < ms.day).collect();
                let post: Vec<&&Cell> = sub.iter().filter(|c| c.day >= ms.day).collect();
                let pre_l = pre.iter().filter(|c| c.rms >= LOUD_HZ).count();
                let post_l = post.iter().filter(|c| c.rms >= LOUD_HZ).count();
                let pre_q = pre.len() - pre_l;
                let post_q = post.len() - post_l;
                let p = if pre.len() >= MIN_SIDE && post.len() >= MIN_SIDE {
                    match fisher_two_sided(pre_l, pre_q, post_l, post_q) { Some(v) => format!("{v:.4}"), None => "-".to_string() }
                } else {
                    "thin".to_string()
                };
                let frac = |l: usize, n: usize| -> f64 {
                    if n == 0 {
                        0.0
                    } else {
                        l as f64 / n as f64
                    }
                };
                push(format!(
                    "  M{mode} st{st}: pre n {} loud {pre_l} frac {:.3} | post n {} loud {post_l} frac {:.3} | diff {:+.3} | fisher p {p}",
                    pre.len(),
                    frac(pre_l, pre.len()),
                    post.len(),
                    frac(post_l, post.len()),
                    frac(post_l, post.len()) - frac(pre_l, pre.len())
                ));
            }
        }
        for st in trio {
            let sub: Vec<&Cell> = cells_sorted.iter().filter(|c| c.st == st).collect();
            let pre: Vec<&&Cell> = sub.iter().filter(|c| c.day < ms.day).collect();
            let post: Vec<&&Cell> = sub.iter().filter(|c| c.day >= ms.day).collect();
            let pre_l = pre.iter().filter(|c| c.rms >= LOUD_HZ).count();
            let post_l = post.iter().filter(|c| c.rms >= LOUD_HZ).count();
            let p = if pre.len() >= MIN_SIDE && post.len() >= MIN_SIDE {
                fmt_p(fisher_two_sided(pre_l, pre.len() - pre_l, post_l, post.len() - post_l))
            } else {
                "thin".to_string()
            };
            let fpre = if pre.is_empty() { 0.0 } else { pre_l as f64 / pre.len() as f64 };
            let fpost = if post.is_empty() { 0.0 } else { post_l as f64 / post.len() as f64 };
            push(format!(
                "  st{st} pooled modes (cell unit): pre n {} loud {pre_l} frac {:.3} | post n {} loud {post_l} frac {:.3} | diff {:+.3} | fisher p {p}",
                pre.len(),
                fpre,
                post.len(),
                fpost,
                fpost - fpre
            ));
        }
        let all: Vec<&Cell> = cells_sorted.iter().collect();
        let pre: Vec<&&Cell> = all.iter().filter(|c| c.day < ms.day).collect();
        let post: Vec<&&Cell> = all.iter().filter(|c| c.day >= ms.day).collect();
        let pre_l = pre.iter().filter(|c| c.rms >= LOUD_HZ).count();
        let post_l = post.iter().filter(|c| c.rms >= LOUD_HZ).count();
        let p = if pre.len() >= MIN_SIDE && post.len() >= MIN_SIDE {
            fmt_p(fisher_two_sided(pre_l, pre.len() - pre_l, post_l, post.len() - post_l))
        } else {
            "thin".to_string()
        };
        let fpre = if pre.is_empty() { 0.0 } else { pre_l as f64 / pre.len() as f64 };
        let fpost = if post.is_empty() { 0.0 } else { post_l as f64 / post.len() as f64 };
        push(format!(
            "  all stations pooled (cell unit): pre n {} loud {pre_l} frac {:.3} | post n {} loud {post_l} frac {:.3} | diff {:+.3} | fisher p {p}",
            pre.len(),
            fpre,
            post.len(),
            fpost,
            fpost - fpre
        ));
    }

    push(String::new());
    push("== 2b. loud / quiet day states of the late-1995 season (1995-11-23 .. 1995-12-07), the window around the 1995-12-05 milestone ==".to_string());
    push("robust cells only; L = loud cell, q = quiet cell, . = no robust floor cell that day (0 honored); the BVR test window per the external research ran Oct-Dec 1995 with staggered station rollout".to_string());
    let d0 = days_from_civil(1995, 11, 23);
    let d1 = days_from_civil(1995, 12, 7);
    for mode in modes {
        for st in trio {
            let sub: BTreeMap<i64, bool> = cells_sorted
                .iter()
                .filter(|c| c.mode == mode && c.st == st && c.day >= d0 && c.day <= d1)
                .map(|c| (c.day, c.rms >= LOUD_HZ))
                .collect();
            let mut line = format!("M{mode} st{st}: ");
            let mut d = d0;
            while d <= d1 {
                match sub.get(&d) {
                    Some(true) => line.push('L'),
                    Some(false) => line.push('q'),
                    None => line.push('.'),
                }
                d += 1;
            }
            push(line);
        }
    }

    push(String::new());
    push("== 2c. per-series data-driven best split inside the whole floor era, robust series ==".to_string());
    push(format!("split point between consecutive robust days, both sides >= {MIN_SIDE} cells; the gap = |loud frac right - loud frac left|; nearest tested milestone given"));
    for mode in modes {
        for st in trio {
            let sub: Vec<&Cell> = cells_sorted
                .iter()
                .filter(|c| c.mode == mode && c.st == st)
                .collect();
            if sub.len() < 2 * MIN_SIDE {
                push(format!("M{mode} st{st}: {} robust days, no split with >= {MIN_SIDE} per side", sub.len()));
                continue;
            }
            let mut best: Option<(f64, usize, i64)> = None;
            for k in MIN_SIDE..=sub.len() - MIN_SIDE {
                let left = sub[..k].iter().filter(|c| c.rms >= LOUD_HZ).count();
                let right = sub[k..].iter().filter(|c| c.rms >= LOUD_HZ).count();
                let fl = left as f64 / k as f64;
                let fr = right as f64 / (sub.len() - k) as f64;
                let gap = (fr - fl).abs();
                let better = match best.as_ref() {
                    Some((bg, _, _)) => gap > *bg,
                    None => true,
                };
                if better {
                    best = Some((gap, k, sub[k - 1].day));
                }
            }
            if let Some((gap, k, d_left)) = best {
                let left_l = sub[..k].iter().filter(|c| c.rms >= LOUD_HZ).count();
                let right_l = sub[k..].iter().filter(|c| c.rms >= LOUD_HZ).count();
                let fl = left_l as f64 / k as f64;
                let fr = right_l as f64 / (sub.len() - k) as f64;
                let nearest = nearest_milestone(d_left, sub[k].day, &milestones);
                push(format!(
                    "M{mode} st{st}: best split after {} (daycell {d_left}): left n {k} loud {left_l} frac {fl:.3} | right n {} loud {right_l} frac {fr:.3} | gap {gap:.3} | nearest milestone {nearest}",
                    civil_str(d_left),
                    sub.len() - k
                ));
            }
        }
    }

    push(String::new());
    push("== 3. station-day level probability (a station-day = distinct day with >= 1 robust cell; loud = >= 1 loud robust cell), milestone split ==".to_string());
    for ms in &milestones {
        push(format!(
            "--- TEST {} (daycell {})",
            ms.name,
            ms.day
        ));
        for st in trio {
            let days: BTreeSet<i64> = cells_sorted.iter().filter(|c| c.st == st).map(|c| c.day).collect();
            let pre_d: Vec<i64> = days.iter().filter(|d| **d < ms.day).copied().collect();
            let post_d: Vec<i64> = days.iter().filter(|d| **d >= ms.day).copied().collect();
            let loud_of = |d: &i64| -> bool {
                cells_sorted
                    .iter()
                    .any(|c| c.st == st && c.day == *d && c.rms >= LOUD_HZ)
            };
            let pre_l = pre_d.iter().filter(|d| loud_of(d)).count();
            let post_l = post_d.iter().filter(|d| loud_of(d)).count();
            let p = if pre_d.len() >= MIN_SIDE && post_d.len() >= MIN_SIDE {
                fmt_p(fisher_two_sided(pre_l, pre_d.len() - pre_l, post_l, post_d.len() - post_l))
            } else {
                "thin".to_string()
            };
            let fpre = if pre_d.is_empty() { 0.0 } else { pre_l as f64 / pre_d.len() as f64 };
            let fpost = if post_d.is_empty() { 0.0 } else { post_l as f64 / post_d.len() as f64 };
            push(format!(
                "  st{st}: pre days {} loud {pre_l} frac {:.3} | post days {} loud {post_l} frac {:.3} | diff {:+.3} | fisher p {p}",
                pre_d.len(),
                fpre,
                post_d.len(),
                fpost,
                fpost - fpre
            ));
        }
        let all_days: BTreeSet<i64> = cells_sorted.iter().map(|c| c.day).collect();
        let pre_d: Vec<i64> = all_days.iter().filter(|d| **d < ms.day).copied().collect();
        let post_d: Vec<i64> = all_days.iter().filter(|d| **d >= ms.day).copied().collect();
        let loud_of = |d: &i64| -> bool {
            cells_sorted.iter().any(|c| c.day == *d && c.rms >= LOUD_HZ)
        };
        let pre_l = pre_d.iter().filter(|d| loud_of(d)).count();
        let post_l = post_d.iter().filter(|d| loud_of(d)).count();
        let p = if pre_d.len() >= MIN_SIDE && post_d.len() >= MIN_SIDE {
            fmt_p(fisher_two_sided(pre_l, pre_d.len() - pre_l, post_l, post_d.len() - post_l))
        } else {
            "thin".to_string()
        };
        let fpre = if pre_d.is_empty() { 0.0 } else { pre_l as f64 / pre_d.len() as f64 };
        let fpost = if post_d.is_empty() { 0.0 } else { post_l as f64 / post_d.len() as f64 };
        push(format!(
            "  all stations (station-days over all modes): pre days {} loud {pre_l} frac {:.3} | post days {} loud {post_l} frac {:.3} | diff {:+.3} | fisher p {p}",
            pre_d.len(),
            fpre,
            post_d.len(),
            fpost,
            fpost - fpre
        ));
    }

    push(String::new());
    push("== 3b. station concordance at each milestone (station-day level, all modes) ==".to_string());
    push("station deltas of the section-3 day metric (post loud-day frac - pre loud-day frac); concordance counts how many stations move up at the milestone; H1 station-bound config steps put the change at the station whose equipment changed (staggered rollout), a global date step moves all stations alike".to_string());
    for ms in &milestones {
        let mut deltas: Vec<(i64, f64, usize, usize, usize, usize)> = Vec::new();
        for st in trio {
            let days: BTreeSet<i64> = cells_sorted.iter().filter(|c| c.st == st).map(|c| c.day).collect();
            let loud_of = |d: &i64| -> bool {
                cells_sorted.iter().any(|c| c.st == st && c.day == *d && c.rms >= LOUD_HZ)
            };
            let pre: Vec<i64> = days.iter().filter(|d| **d < ms.day).copied().collect();
            let post: Vec<i64> = days.iter().filter(|d| **d >= ms.day).copied().collect();
            let pre_l = pre.iter().filter(|d| loud_of(d)).count();
            let post_l = post.iter().filter(|d| loud_of(d)).count();
            let fp = if pre.is_empty() { 0.0 } else { pre_l as f64 / pre.len() as f64 };
            let fo = if post.is_empty() { 0.0 } else { post_l as f64 / post.len() as f64 };
            deltas.push((st, fo - fp, pre.len(), pre_l, post.len(), post_l));
        }
        let up = deltas.iter().filter(|(_, d, _, _, _, _)| *d > 0.001).count();
        let down = deltas.iter().filter(|(_, d, _, _, _, _)| *d < -0.001).count();
        let dstr: Vec<String> = deltas
            .iter()
            .map(|(st, d, pn, pl, on, ol)| {
                format!("st{st} {d:+.3} (pre {pl}/{pn} post {ol}/{on})")
            })
            .collect();
        push(format!(
            "--- TEST {} (daycell {}) | stations up {up} down {down} | {}",
            ms.name,
            ms.day,
            dstr.join(" ")
        ));
    }
    push(String::new());
    push("== 4. mean log10 RMS and median RMS per side of each tested milestone (cell unit, robust series) ==".to_string());
    for ms in &milestones {
        push(format!("--- TEST {} (daycell {})", ms.name, ms.day));
        for mode in modes {
            for st in trio {
                let sub: Vec<&Cell> = cells_sorted
                    .iter()
                    .filter(|c| c.mode == mode && c.st == st)
                    .collect();
                let pre: Vec<f64> = sub.iter().filter(|c| c.day < ms.day).map(|c| c.rms).collect();
                let post: Vec<f64> = sub.iter().filter(|c| c.day >= ms.day).map(|c| c.rms).collect();
                let fmt = |v: &[f64]| -> String {
                    if v.is_empty() {
                        "n 0".to_string()
                    } else {
                        match (median(v), mean_log10(v)) {
                            (Some(med), Some(mv)) => format!(
                                "n {} med {med:.3} Hz meanlog {mv:+.2}",
                                v.len()
                            ),
                            _ => format!("n {}", v.len()),
                        }
                    }
                };
                push(format!("  M{mode} st{st}: pre {} | post {}", fmt(&pre), fmt(&post)));
            }
        }
    }

    let _ = fs::write(&report_path, out.join("\n") + "\n");
    eprintln!("galileo: h1 receiver regression report written to {report_path}");
}

fn nearest_milestone(d0: i64, d1: i64, milestones: &[Milestone]) -> String {
    let mut best: Option<(i64, &str)> = None;
    for ms in milestones {
        for d in [d0, d1] {
            let dist = (d - ms.day).abs();
            let closer = match best {
                Some((bd, _)) => dist < bd,
                None => true,
            };
            if closer {
                best = Some((dist, ms.name));
            }
        }
    }
    match best {
        Some((dist, name)) => format!("{name} ({dist} d)"),
        None => "-".to_string(),
    }
}
