use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;

use omegaflow::archivar::{BodyEphemeris, body_barycenter_position, parse_ephemeris_binary};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const STRONG_MIN: i64 = -1750;
const LOUD_HZ: f64 = 1.0;
const AU_M: f64 = 1.495978707e11;
const MIN_WIN_N: usize = 2;
const ANCHOR_WIN_D: i64 = 120;
const ADJ_D: i64 = 1;
const CONJ_EPS: f64 = 30.0;

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn angle(a: [f64; 3], b: [f64; 3]) -> Option<f64> {
    let na = norm(a);
    let nb = norm(b);
    if na > 0.0 && nb > 0.0 {
        Some((dot(a, b) / (na * nb)).clamp(-1.0, 1.0).acos().to_degrees())
    } else {
        None
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
    if vals.is_empty() {
        return None;
    }
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

fn spearman(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len();
    if n < 8 || n != y.len() {
        return None;
    }
    let mut xi: Vec<usize> = (0..n).collect();
    let mut yi: Vec<usize> = (0..n).collect();
    xi.sort_by(|a, b| x[*a].total_cmp(&x[*b]));
    yi.sort_by(|a, b| y[*a].total_cmp(&y[*b]));
    let mut rx = vec![0.0f64; n];
    let mut ry = vec![0.0f64; n];
    let mut i = 0usize;
    while i < n {
        let mut j = i + 1;
        while j < n && x[xi[j]] == x[xi[i]] {
            j += 1;
        }
        let avg = ((i + j - 1) as f64) / 2.0;
        for k in xi[i..j].iter() {
            rx[*k] = avg;
        }
        i = j;
    }
    i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && y[yi[j]] == y[yi[i]] {
            j += 1;
        }
        let avg = ((i + j - 1) as f64) / 2.0;
        for k in yi[i..j].iter() {
            ry[*k] = avg;
        }
        i = j;
    }
    let mx = rx.iter().sum::<f64>() / n as f64;
    let my = ry.iter().sum::<f64>() / n as f64;
    let mut num = 0.0;
    let mut dx2 = 0.0;
    let mut dy2 = 0.0;
    for k in 0..n {
        let a = rx[k] - mx;
        let b = ry[k] - my;
        num += a * b;
        dx2 += a * a;
        dy2 += b * b;
    }
    if dx2 > 0.0 && dy2 > 0.0 {
        Some(num / (dx2 * dy2).sqrt())
    } else {
        None
    }
}

fn load(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    std::fs::read(format!("data/ephemeris_{name}.bin"))
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| eph.insert(name.to_string(), e))
        .is_some()
}

fn unix_day(tdb: f64) -> i64 {
    (tdb / DAY_S + 10957.5).round() as i64
}

fn date_of(tdb: f64) -> (i64, i64, i64) {
    match civil_from_days(unix_day(tdb)) {
        Some((y, m, d)) => (y as i64, m as i64, d as i64),
        None => (0, 0, 0),
    }
}

fn fmt_date(tdb: f64) -> String {
    let (y, m, d) = date_of(tdb);
    format!("{y:04}-{m:02}-{d:02}")
}

fn fmt_daycell(day: i64) -> String {
    fmt_date(day as f64 * DAY_S)
}

fn fmt_opt_t0(t: Option<f64>) -> String {
    match t {
        Some(x) => fmt_date(x),
        None => "-".to_string(),
    }
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn anchor_daycell(y: i64, m: i64, d: i64) -> i64 {
    days_from_civil(y, m, d) - 10958
}

#[derive(Clone, Copy)]
struct Cell {
    mode: i64,
    day: i64,
    st: i64,
    floor: bool,
    n: usize,
    rms: f64,
    t0: f64,
}

fn rms_of(sum: f64, sum2: f64, n: usize) -> f64 {
    if n == 0 {
        return f64::NAN;
    }
    let m = sum / n as f64;
    let v = (sum2 / n as f64 - m * m).max(0.0);
    v.sqrt()
}

fn fmt_med(vals: &[f64]) -> String {
    match median(vals) {
        Some(m) => format!("{m:.4}"),
        None => "-".to_string(),
    }
}

fn loud_frac(vals: &[f64]) -> String {
    if vals.is_empty() {
        return "-".to_string();
    }
    let loud = vals.iter().filter(|v| **v >= LOUD_HZ).count();
    format!("{loud}/{}", vals.len())
}

struct Geo {
    r: f64,
    eps: f64,
    alpha: f64,
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "/tmp/opencode/galileo_ops_era_station_step_probe_report.txt".to_string(),
    };

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in ["galileo_daily", "earth"] {
        if !load(b, &mut eph) {
            eprintln!("galileo: {b} ephemeris bin void");
        }
    }
    let geom_ok = eph.contains_key("galileo_daily") && eph.contains_key("earth");
    let Ok(bytes) = fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    drop(bytes);

    let mut cell: BTreeMap<(i64, i64, i64, u8), (f64, f64, usize, f64)> = BTreeMap::new();
    let mut n_lock = 0usize;
    let mut n_mode = 0usize;
    let mut n_zero = 0usize;
    let mut n_uncl = 0usize;
    let mut n_inf = 0usize;
    let mut t0_all = f64::INFINITY;
    let mut t1_all = f64::NEG_INFINITY;
    for r in &recs {
        let tdb = r[0];
        t0_all = t0_all.min(tdb);
        t1_all = t1_all.max(tdb);
        let mode = r[3] as i64;
        if mode != 1 && mode != 2 {
            n_mode += 1;
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() {
            n_inf += 1;
            continue;
        }
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        let s = r[7] as i64;
        if s == 0 {
            n_zero += 1;
            continue;
        }
        let cls: u8 = if s == FLOOR {
            0
        } else if s >= STRONG_MIN {
            1
        } else {
            n_uncl += 1;
            continue;
        };
        let day = (tdb / DAY_S).floor() as i64;
        let st = r[2] as i64;
        let e = cell
            .entry((mode, day, st, cls))
            .or_insert_with(|| (0.0, 0.0, 0, tdb));
        e.0 += resid;
        e.1 += resid * resid;
        e.2 += 1;
        if tdb < e.3 {
            e.3 = tdb;
        }
    }
    drop(recs);

    let mut rows: Vec<Cell> = Vec::new();
    for (&(mode, day, st, cls), &(sum, sum2, n, t0)) in &cell {
        if n == 0 {
            continue;
        }
        rows.push(Cell {
            mode,
            day,
            st,
            floor: cls == 0,
            n,
            rms: rms_of(sum, sum2, n),
            t0,
        });
    }
    rows.sort_by_key(|c| (c.mode, c.st, c.day, c.floor));

    let mut geo: BTreeMap<i64, Geo> = BTreeMap::new();
    if geom_ok {
        let days: BTreeSet<i64> = rows.iter().map(|c| c.day).collect();
        for &d in &days {
            let t = d as f64 * DAY_S;
            if let (Some(p), Some(e)) = (
                body_barycenter_position("galileo_daily", t, &eph),
                body_barycenter_position("earth", t, &eph),
            ) {
                if let (Some(ep), Some(al)) = (angle(sub([0.0; 3], e), sub(p, e)), angle(e, p)) {
                    geo.insert(d, Geo {
                        r: norm(p) / AU_M,
                        eps: ep,
                        alpha: al,
                    });
                }
            }
        }
    }

    let anchors: Vec<(&str, i64)> = vec![
        (
            "1995-09-18 BVR network standard mode (Beyer 42-125)",
            anchor_daycell(1995, 9, 18),
        ),
        (
            "1995-12-05 suppressed(90)->residual(58)-carrier switch (Beyer 42-125)",
            anchor_daycell(1995, 12, 5),
        ),
        (
            "1996-05-15 DGT phase-2 uplink window (Beyer 42-125, May 1996)",
            anchor_daycell(1996, 5, 15),
        ),
        (
            "1996-11-01 full array mode routine (Beyer 42-133)",
            anchor_daycell(1996, 11, 1),
        ),
        (
            "1995-04-15 DSS-14 maintenance window (Apr 1995, ~6 weeks)",
            anchor_daycell(1995, 4, 15),
        ),
        (
            "1995-05-15 DSS-43 ultracone window (May 1995, 7 days)",
            anchor_daycell(1995, 5, 15),
        ),
        (
            "1995-08-15 DSS-63 subreflector window (Aug 1995, ~4 weeks)",
            anchor_daycell(1995, 8, 15),
        ),
        (
            "1995-09-01 DSS-43 maintenance window (Sep 1995)",
            anchor_daycell(1995, 9, 1),
        ),
    ];

    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String| {
        println!("{s}");
        out.push(s);
    };

    push("galileo ops-era station floor step probe".to_string());
    push(format!(
        "span {} .. {}; modes 1 and 2; lock (|resid| > {LOCK_HZ:.0} Hz) excluded before noise; non-finite excluded; strength 0 separated, never classed",
        fmt_date(t0_all),
        fmt_date(t1_all)
    ));
    push(format!(
        "classes: floor = strength == {FLOOR} (AGC clamp); strong = strength >= {STRONG_MIN}; loud = cell RMS >= {LOUD_HZ} Hz; cell = (mode, tdb day, station, class)"
    ));
    push(format!(
        "excluded: lock {n_lock}, non-mode {n_mode}, strength 0 {n_zero}, strength unclassed {n_uncl}, non-finite {n_inf}"
    ));
    push("ops anchors (external, TDA Progress Report 42-125 / 42-133, Beyer et al.):".to_string());
    for (name, d) in &anchors {
        push(format!("  {name} -> daycell {d} ({})", fmt_daycell(*d)));
    }
    push("geometry at TDB day start: r heliocentric AU, eps elongation at the Earth, alpha angle at the Sun".to_string());

    for mode in [1i64, 2] {
        for floor in [true, false] {
            let sub: Vec<&Cell> = rows
                .iter()
                .filter(|c| c.mode == mode && c.floor == floor)
                .collect();
            let name = if floor { "floor" } else { "strong" };
            if sub.is_empty() {
                continue;
            }
            let n_samp: usize = sub.iter().map(|c| c.n).sum();
            let days: BTreeSet<i64> = sub.iter().map(|c| c.day).collect();
            let rms_v: Vec<f64> = sub.iter().map(|c| c.rms).collect();
            let loud = sub.iter().filter(|c| c.rms >= LOUD_HZ).count();
            push(String::new());
            push(format!(
                "== mode {mode} {name}: {n_samp} samples, {} cells, {} distinct days, {}-{}, loud {loud}, median cell RMS {:.4} Hz ==",
                sub.len(),
                days.len(),
                fmt_opt_t0(sub.iter().map(|c| c.t0).min_by(f64::total_cmp)),
                fmt_opt_t0(sub.iter().map(|c| c.t0).max_by(f64::total_cmp)),
                median(&rms_v).unwrap_or(f64::NAN)
            ));
            let stations: BTreeSet<i64> = sub.iter().map(|c| c.st).collect();
            for st in &stations {
                let ssub: Vec<&&Cell> = sub.iter().filter(|c| c.st == *st).collect();
                let srms: Vec<f64> = ssub.iter().map(|c| c.rms).collect();
                let sdays: BTreeSet<i64> = ssub.iter().map(|c| c.day).collect();
                let sloud = ssub.iter().filter(|c| c.rms >= LOUD_HZ).count();
                let s_samp: usize = ssub.iter().map(|c| c.n).sum();
                push(format!(
                    "  st{st}: {} samples, {} cells, {} days, {}-{}, loud {sloud}, median {:.4} Hz, span med n {}",
                    s_samp,
                    ssub.len(),
                    sdays.len(),
                    fmt_opt_t0(ssub.iter().map(|c| c.t0).min_by(f64::total_cmp)),
                    fmt_opt_t0(ssub.iter().map(|c| c.t0).max_by(f64::total_cmp)),
                    median(&srms).unwrap_or(f64::NAN),
                    median(&ssub.iter().map(|c| c.n as f64).collect::<Vec<f64>>()).unwrap_or(f64::NAN) as usize
                ));
            }
        }
    }

    for mode in [1i64, 2] {
        push(String::new());
        push(format!("== mode {mode} floor day cells by sample-count bucket, per station =="));
        push("  bucket n<30 / 30..<100 / 100..<1000 / >=1000: cells, loud cells; thin cells carry a RMS over few samples".to_string());
        let stations: BTreeSet<i64> = rows
            .iter()
            .filter(|c| c.mode == mode && c.floor)
            .map(|c| c.st)
            .collect();
        for st in &stations {
            let sub: Vec<&Cell> = rows
                .iter()
                .filter(|c| c.mode == mode && c.floor && c.st == *st)
                .collect();
            let mut line = format!("  st{st}:");
            for (lo, hi) in [(0usize, 30usize), (30, 100), (100, 1000), (1000, usize::MAX)] {
                let cells: Vec<&&Cell> = sub.iter().filter(|c| c.n >= lo && c.n < hi).collect();
                let loud = cells.iter().filter(|c| c.rms >= LOUD_HZ).count();
                line.push_str(&format!(
                    " n{lo}..<{hi} {}c/{}l |",
                    cells.len(),
                    loud
                ));
            }
            let n30: Vec<&Cell> = sub.iter().filter(|c| c.n >= 30).copied().collect();
            let loud30 = n30.iter().filter(|c| c.rms >= LOUD_HZ).count();
            let n30_rms: Vec<f64> = n30.iter().map(|c| c.rms).collect();
            line.push_str(&format!(
                " n>=30 {}c/{}l/med {:.4} Hz",
                n30.len(),
                loud30,
                median(&n30_rms).unwrap_or(f64::NAN)
            ));
            push(line);
        }
    }

    for mode in [1i64, 2] {
        push(String::new());
        push(format!("== mode {mode} floor day rows, per station chronological =="));
        let stations: BTreeSet<i64> = rows
            .iter()
            .filter(|c| c.mode == mode && c.floor)
            .map(|c| c.st)
            .collect();
        for st in &stations {
            push(format!("  --- st{st} floor (loud = RMS >= {LOUD_HZ} Hz) ---"));
            let srows: Vec<&Cell> = rows
                .iter()
                .filter(|c| c.mode == mode && c.floor && c.st == *st)
                .collect();
            for c in &srows {
                let g = match geo.get(&c.day) {
                    Some(gg) => format!("r {:.3} eps {:6.1} alpha {:6.1}", gg.r, gg.eps, gg.alpha),
                    None => "r --  eps --  alpha --".to_string(),
                };
                push(format!(
                    "  {} day {} n {:<7} RMS {:10.4} Hz {}  {}",
                    fmt_daycell(c.day),
                    c.day,
                    c.n,
                    c.rms,
                    if c.rms >= LOUD_HZ { "LOUD" } else { "quiet" },
                    g
                ));
            }
        }
    }

    let trio = [14i64, 43, 63];
    for mode in [1i64, 2] {
        push(String::new());
        push(format!("== mode {mode} floor calendar runs, stations 14/43/63 =="));
        for st in trio {
            let srows: Vec<&Cell> = rows
                .iter()
                .filter(|c| c.mode == mode && c.floor && c.st == st)
                .collect();
            if srows.is_empty() {
                push(format!("  st{st}: no floor cells"));
                continue;
            }
            let mut runs: Vec<Vec<&Cell>> = Vec::new();
            let mut cur: Vec<&Cell> = Vec::new();
            for c in &srows {
                if let Some(last) = cur.last() {
                    if c.day - last.day > 1 {
                        runs.push(std::mem::take(&mut cur));
                    }
                }
                cur.push(c);
            }
            if !cur.is_empty() {
                runs.push(cur);
            }
            push(format!("  st{st}: {} runs", runs.len()));
            for (k, run) in runs.iter().enumerate() {
                let rms_v: Vec<f64> = run.iter().map(|c| c.rms).collect();
                let loud = run.iter().filter(|c| c.rms >= LOUD_HZ).count();
                let d0 = run.first().unwrap().day;
                let d1 = run.last().unwrap().day;
                let run_len = d1 - d0 + 1;
                push(format!(
                    "    run {k}: {}-{} ({} calendar d, {} floor d, loud {loud}/{}) med {:.4} Hz",
                    fmt_daycell(d0),
                    fmt_daycell(d1),
                    run_len,
                    run.len(),
                    run.len(),
                    median(&rms_v).unwrap_or(f64::NAN)
                ));
                if k + 1 < runs.len() {
                    let nd = runs[k + 1].first().unwrap().day;
                    let gap = nd - d1 - 1;
                    push(format!(
                        "      gap to next run: {} days ({} absent)",
                        gap,
                        gap
                    ));
                }
            }
        }
    }

    push(String::new());
    push(format!("== ops-anchor segmentation, floor, stations 14/43/63, window +/- {ANCHOR_WIN_D} d =="));
    push(format!("  each anchor: floor day cells before (A-W .. A) and after (A .. A+W); med RMS, mean log10 RMS, loud frac; a side below {MIN_WIN_N} cells stays absent"));
    for (name, a) in &anchors {
        for mode in [1i64, 2] {
            for st in trio {
                let sub: Vec<&Cell> = rows
                    .iter()
                    .filter(|c| c.mode == mode && c.floor && c.st == st)
                    .collect();
                let mut pre: Vec<f64> = Vec::new();
                let mut post: Vec<f64> = Vec::new();
                for c in &sub {
                    if c.day >= a - ANCHOR_WIN_D && c.day < *a {
                        pre.push(c.rms);
                    } else if c.day > *a && c.day <= a + ANCHOR_WIN_D {
                        post.push(c.rms);
                    }
                }
                let pre_s = if pre.len() >= MIN_WIN_N {
                    format!(
                        "n {} med {} dB {}",
                        pre.len(),
                        fmt_med(&pre),
                        mean_log10(&pre).map(|m| format!("{m:+.2}")).unwrap_or("-".to_string())
                    )
                } else {
                    format!("n {} absent", pre.len())
                };
                let post_s = if post.len() >= MIN_WIN_N {
                    format!(
                        "n {} med {} dB {}",
                        post.len(),
                        fmt_med(&post),
                        mean_log10(&post).map(|m| format!("{m:+.2}")).unwrap_or("-".to_string())
                    )
                } else {
                    format!("n {} absent", post.len())
                };
                push(format!(
                    "  [{name}] mode {mode} st{st}: pre {pre_s} | post {post_s}"
                ));
            }
        }
    }

    push(String::new());
    push("== per-station floor-state flips between calendar-adjacent floor days (gap 1), stations 14/43/63 ==".to_string());
    for mode in [1i64, 2] {
        for st in trio {
            let srows: Vec<&Cell> = rows
                .iter()
                .filter(|c| c.mode == mode && c.floor && c.st == st)
                .collect();
            let mut flips = 0usize;
            let mut lines: Vec<String> = Vec::new();
            for w in srows.windows(2) {
                let a = w[0];
                let b = w[1];
                if b.day - a.day != ADJ_D {
                    continue;
                }
                let la = a.rms >= LOUD_HZ;
                let lb = b.rms >= LOUD_HZ;
                if la != lb {
                    flips += 1;
                    let (from, to) = if la { ("LOUD", "quiet") } else { ("quiet", "LOUD") };
                    let nearest = nearest_anchor(a.day, b.day, &anchors);
                    let ga = geo.get(&a.day);
                    let gb = geo.get(&b.day);
                    let geo_s = match (ga, gb) {
                        (Some(x), Some(y)) => format!(
                            "eps {:.2}->{:.2} r {:.3}->{:.3}",
                            x.eps,
                            y.eps,
                            x.r,
                            y.r
                        ),
                        _ => "geometry --".to_string(),
                    };
                    lines.push(format!(
                        "    {}-{}: {:.4} Hz {from} -> {:.4} Hz {to} | {geo_s} | nearest anchor {nearest}",
                        fmt_daycell(a.day),
                        fmt_daycell(b.day),
                        a.rms,
                        b.rms
                    ));
                }
            }
            push(format!(
                "  mode {mode} st{st}: {flips} adjacent-day loud/quiet flips"
            ));
            for l in &lines {
                push(l.clone());
            }
        }
    }

    push(String::new());
    push("== data-driven best single split (max |mean log10 RMS| gap, >= 2 floor days per side), stations 14/43/63 ==".to_string());
    for mode in [1i64, 2] {
        for st in trio {
            let srows: Vec<&Cell> = rows
                .iter()
                .filter(|c| c.mode == mode && c.floor && c.st == st)
                .collect();
            if srows.len() < 4 {
                push(format!("  mode {mode} st{st}: only {} floor cells, no split scan", srows.len()));
                continue;
            }
            let mut best: Option<(f64, usize, &Cell)> = None;
            for k in 2..srows.len() - 1 {
                let left: Vec<f64> = srows[..k].iter().map(|c| c.rms).collect();
                let right: Vec<f64> = srows[k..].iter().map(|c| c.rms).collect();
                if left.len() < MIN_WIN_N || right.len() < MIN_WIN_N {
                    continue;
                }
                let (Some(ml), Some(mr)) = (mean_log10(&left), mean_log10(&right)) else {
                    continue;
                };
                let gap = (ml - mr).abs();
                if best.as_ref().map(|(bg, _, _)| gap > *bg).unwrap_or(true) {
                    best = Some((gap, k, &srows[k]));
                }
            }
            match best {
                Some((gap, k, split)) => {
                    let left: Vec<f64> = srows[..k].iter().map(|c| c.rms).collect();
                    let right: Vec<f64> = srows[k..].iter().map(|c| c.rms).collect();
                    let d_left = srows[k - 1].day;
                    let nearest = nearest_anchor(d_left, split.day, &anchors);
                    push(format!(
                        "  mode {mode} st{st}: best split after {} (day {}) — left n {} med {} loudfrac {} | right n {} med {} loudfrac {} | gap {gap:.2} dB | nearest anchor {nearest}",
                        fmt_daycell(d_left),
                        d_left,
                        left.len(),
                        fmt_med(&left),
                        loud_frac(&left),
                        right.len(),
                        fmt_med(&right),
                        loud_frac(&right)
                    ));
                }
                None => push(format!("  mode {mode} st{st}: no valid split")),
            }
        }
    }

    push(String::new());
    push("== anchor-day decomposition 1995-11-24 and 1996-06-26, all (mode, station, class) cells ==".to_string());
    for anchor_civil in [(1995i64, 11i64, 24i64), (1996, 6, 26)] {
        let a = anchor_daycell(anchor_civil.0, anchor_civil.1, anchor_civil.2);
        push(format!(
            "  == {} (daycell {a}) ==",
            fmt_daycell(a)
        ));
        let same: Vec<&Cell> = rows.iter().filter(|c| c.day == a).collect();
        if same.is_empty() {
            push("    no classed cells on this day (0 honored)".to_string());
            continue;
        }
        for c in &same {
            let g = match geo.get(&c.day) {
                Some(gg) => format!("r {:.3} eps {:6.1} alpha {:6.1}", gg.r, gg.eps, gg.alpha),
                None => "r --".to_string(),
            };
            push(format!(
                "    mode {} st{} {} n {:<7} RMS {:10.4} Hz {}  {}",
                c.mode,
                c.st,
                if c.floor { "floor " } else { "strong" },
                c.n,
                c.rms,
                if c.rms >= LOUD_HZ { "LOUD" } else { "quiet" },
                g
            ));
        }
    }

    push(String::new());
    push("== geometry control: within-floor-era, per (mode, station) floor Spearman of log10 RMS vs eps/alpha/r/day ==".to_string());
    for mode in [1i64, 2] {
        for st in trio {
            let srows: Vec<&Cell> = rows
                .iter()
                .filter(|c| c.mode == mode && c.floor && c.st == st)
                .collect();
            if srows.is_empty() {
                continue;
            }
            let mut es = Vec::new();
            let mut al = Vec::new();
            let mut rs = Vec::new();
            let mut lr = Vec::new();
            let mut dy = Vec::new();
            for c in &srows {
                if c.rms > 0.0 {
                    lr.push(c.rms.log10());
                    dy.push(c.day as f64);
                    if let Some(g) = geo.get(&c.day) {
                        es.push(g.eps);
                        al.push(g.alpha);
                        rs.push(g.r);
                    }
                }
            }
            let rho = |x: &Vec<f64>, y: &Vec<f64>| -> String {
                if x.len() == y.len() && x.len() >= 8 {
                    spearman(x, y).map(|v| format!("{v:+.2}")).unwrap_or("-".to_string())
                } else if x.len() == y.len() {
                    format!("n{}<8", x.len())
                } else {
                    "-".to_string()
                }
            };
            push(format!(
                "  mode {mode} st{st}: n {lrlen} | rho(logRMS,eps) {a} | rho(logRMS,alpha) {b} | rho(logRMS,r) {c} | rho(logRMS,day) {d}",
                lrlen = lr.len(),
                a = rho(&es, &lr),
                b = rho(&al, &lr),
                c = rho(&rs, &lr),
                d = rho(&dy, &lr)
            ));
        }
    }

    push(String::new());
    push(format!("== geometry control: floor at eps < {CONJ_EPS:.0} deg across the two conjunctions (1995-12 vs 1997-01), stations 14/43/63 =="));
    for mode in [1i64, 2] {
        for st in trio {
            let mut c95: Vec<f64> = Vec::new();
            let mut c97: Vec<f64> = Vec::new();
            let a95 = anchor_daycell(1995, 11, 1);
            let b95 = anchor_daycell(1996, 1, 31);
            let a97 = anchor_daycell(1997, 1, 1);
            let b97 = anchor_daycell(1997, 2, 28);
            for c in rows.iter().filter(|c| c.mode == mode && c.floor && c.st == st) {
                let g = match geo.get(&c.day) {
                    Some(g) => g,
                    None => continue,
                };
                if g.eps < CONJ_EPS {
                    if c.day >= a95 && c.day <= b95 {
                        c95.push(c.rms);
                    } else if c.day >= a97 && c.day <= b97 {
                        c97.push(c.rms);
                    }
                }
            }
            push(format!(
                "  mode {mode} st{st}: conj1995 n {} med {} loudfrac {} | conj1997 n {} med {} loudfrac {}",
                c95.len(),
                fmt_med(&c95),
                loud_frac(&c95),
                c97.len(),
                fmt_med(&c97),
                loud_frac(&c97)
            ));
        }
    }

    let _ = fs::write(&report_path, out.join("\n") + "\n");
    eprintln!("galileo: ops-era station step report written to {report_path}");
}

fn nearest_anchor(d0: i64, d1: i64, anchors: &[(&str, i64)]) -> String {
    let mut best: Option<(i64, &str)> = None;
    for (name, a) in anchors {
        for d in [d0, d1] {
            let dist = (d - a).abs();
            if best.map(|(bd, _)| dist < bd).unwrap_or(true) {
                best = Some((dist, name));
            }
        }
    }
    match best {
        Some((dist, name)) => format!("{name} ({dist} d)"),
        None => "-".to_string(),
    }
}
