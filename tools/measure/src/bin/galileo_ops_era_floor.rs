use std::collections::{BTreeMap, BTreeSet, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const STRONG_MIN: i64 = -1750;
const LOUD_HZ: f64 = 1.0;
const AU_M: f64 = 1.495978707e11;
const STEP_LOG: f64 = 0.7;
const BRACKET_DAYS: i64 = 14;
const NEAR_ANCHOR_DAYS: i64 = 10;

const ANCHORS: &[(&str, i32, u32, u32)] = &[
    (
        "dss14 maintenance: hydrostatic-bearing regrout, 6 weeks",
        1995,
        4,
        1,
    ),
    (
        "dss43 maintenance: ultracone s-band feed install, 7 days",
        1995,
        5,
        1,
    ),
    (
        "dss63 maintenance: subreflector drive replacement, 4 weeks",
        1995,
        8,
        1,
    ),
    (
        "BVR installed at all complexes, tests at dss14+dss43",
        1995,
        5,
        15,
    ),
    (
        "BVR first lock over dss63 on suppressed carrier",
        1995,
        9,
        11,
    ),
    ("BVR standard tracking across the 70-m network", 1995, 9, 18),
    (
        "modulation change suppressed-90deg to residual-58deg",
        1995,
        12,
        5,
    ),
    ("solar conjunction 1995", 1995, 12, 11),
    ("DGT subsystem added at all stations", 1996, 5, 1),
    ("array mode routine", 1996, 11, 1),
    ("solar conjunction 1997", 1997, 1, 11),
];

#[derive(Clone, Copy)]
struct Cell {
    mode: i64,
    day: i64,
    st: i64,
    floor: bool,
    n: usize,
    rms: f64,
    unix_day: i64,
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = (m as i64 + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn unix_day(tdb: f64) -> i64 {
    (tdb / DAY_S + 10957.5).round() as i64
}

fn civil_date_str(ud: i64) -> String {
    match civil_from_days(ud) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => "----".to_string(),
    }
}

fn month_str(ud: i64) -> String {
    match civil_from_days(ud) {
        Some((y, m, _)) => format!("{y:04}-{m:02}"),
        None => "----".to_string(),
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

fn load(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    std::fs::read(format!("data/ssd.jpl.nasa.gov/ephemeris_{name}.bin"))
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| eph.insert(name.to_string(), e))
        .is_some()
}

fn both(rep: &mut Vec<String>, s: String) {
    println!("{s}");
    rep.push(s);
}

fn file(rep: &mut Vec<String>, s: String) {
    rep.push(s);
}

fn fmt_rms(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.4}"),
        None => "-".to_string(),
    }
}

fn fmt_med(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.4}"),
        None => "-".to_string(),
    }
}

fn geo_str(geo: &BTreeMap<i64, (f64, f64, f64)>, day: i64) -> String {
    match geo.get(&day) {
        Some(&(r, ep, al)) => format!("r {r:.3} AU  eps {ep:5.1}  alpha {al:5.1}"),
        None => "r --  eps --  alpha --".to_string(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = match args.iter().position(|a| a == "--report") {
        Some(i) => match args.get(i + 1) {
            Some(p) => p.clone(),
            None => "state/reports/galileo_ops_era_floor.txt".to_string(),
        },
        None => "state/reports/galileo_ops_era_floor.txt".to_string(),
    };

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in ["galileo_daily", "earth"] {
        if !load(b, &mut eph) {
            eprintln!("galileo: {b} ephemeris bin void");
            return;
        }
    }

    let Ok(bytes) = std::fs::read("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin") else {
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
    let mut n_skip = 0usize;
    let mut n_zero = 0usize;
    let mut n_uncl = 0usize;
    for r in &recs {
        let mode = r[3] as i64;
        if mode != 1 && mode != 2 {
            n_skip += 1;
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
        let st = r[2] as i64;
        if st == 0 {
            n_zero += 1;
            continue;
        }
        let s = r[7] as i64;
        let cls: u8 = if s == FLOOR {
            0
        } else if s >= STRONG_MIN {
            1
        } else {
            n_uncl += 1;
            continue;
        };
        let day = (r[0] / DAY_S).floor() as i64;
        let e = cell
            .entry((mode, day, st, cls))
            .or_insert_with(|| (0.0, 0.0, 0, r[0]));
        e.0 += resid;
        e.1 += resid * resid;
        e.2 += 1;
        if r[0] < e.3 {
            e.3 = r[0];
        }
    }
    drop(recs);

    let mut rows: Vec<Cell> = Vec::new();
    for (&(mode, day, st, cls), &(sum, sum2, n, t0)) in &cell {
        if n == 0 {
            continue;
        }
        let m = sum / n as f64;
        let v = (sum2 / n as f64 - m * m).max(0.0);
        rows.push(Cell {
            mode,
            day,
            st,
            floor: cls == 0,
            n,
            rms: v.sqrt(),
            unix_day: unix_day(t0),
        });
    }
    rows.sort_by_key(|c| (c.mode, c.day, c.st));

    let days_all: BTreeSet<i64> = rows.iter().map(|c| c.day).collect();
    let mut geo: BTreeMap<i64, (f64, f64, f64)> = BTreeMap::new();
    for &d in &days_all {
        let t = d as f64 * DAY_S;
        if let (Some(p), Some(e)) = (
            body_barycenter_position("galileo_daily", t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) {
            let rp = norm(p) / AU_M;
            let re = norm(e);
            let e_to_p = sub(p, e);
            let rep_ = norm(e_to_p);
            let al = (dot(e, p) / (re * rp).max(1e-30))
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees();
            let ep = (dot(sub([0.0; 3], e), e_to_p) / (re * rep_).max(1e-30))
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees();
            geo.insert(d, (rp, ep, al));
        }
    }

    let mut rep: Vec<String> = Vec::new();
    both(
        &mut rep,
        "galileo ops-era floor probe: floor loudness per (mode, station, day) against the documented 1995/96 station operational anchors".to_string(),
    );
    both(&mut rep, "binding: modes 1 and 2; lock (|resid| > 1000 Hz) excluded before the cell; cell = (mode, day, station, class); floor = strength == -2560 (AGC clamp); strong = strength >= -1750; strength 0 separated, never classed".to_string());
    both(&mut rep, "geometry at the TDB day start from galileo_daily / earth barycentric ICRS (AU = 1.495978707e11 m): r heliocentric, eps elongation at the Earth, alpha at the Sun".to_string());
    both(
        &mut rep,
        "operational anchors (external DSN research, dated events; see the befund register):"
            .to_string(),
    );
    for (name, y, m, d) in ANCHORS {
        both(&mut rep, format!("  {y:04}-{m:02}-{d:02}  {name}"));
    }

    let classed: usize = cell.values().map(|c| c.2).sum();
    both(
        &mut rep,
        format!(
            "cells after lock exclusion: {classed} classed samples; excluded non-mode {n_skip}, lock {n_lock}, strength 0 {n_zero}, strength neither floor nor strong {n_uncl}"
        ),
    );

    for mode in [1i64, 2] {
        both(&mut rep, String::new());
        both(
            &mut rep,
            format!("== mode {mode} occurrence (per class) =="),
        );
        for floor in [true, false] {
            let srows: Vec<&Cell> = rows
                .iter()
                .filter(|c| c.mode == mode && c.floor == floor)
                .collect();
            let name = if floor { "floor" } else { "strong" };
            if srows.is_empty() {
                both(&mut rep, format!("  {name}: n = 0 cells (absent)"));
                continue;
            }
            let n_samp: usize = srows.iter().map(|c| c.n).sum();
            let days: BTreeSet<i64> = srows.iter().map(|c| c.day).collect();
            let loud = srows.iter().filter(|c| c.rms >= LOUD_HZ).count();
            let (lo_d, hi_d) = (
                civil_date_str(srows.iter().map(|c| c.unix_day).min().unwrap()),
                civil_date_str(srows.iter().map(|c| c.unix_day).max().unwrap()),
            );
            let rms_v: Vec<f64> = srows.iter().map(|c| c.rms).collect();
            both(
                &mut rep,
                format!(
                    "  {name}: {n_samp} samples, {} cells, {} distinct days, {lo_d} .. {hi_d}, loud (cell RMS >= 1 Hz) {loud}, median cell RMS {} Hz",
                    srows.len(),
                    days.len(),
                    fmt_med(median(&rms_v))
                ),
            );
        }
    }

    both(&mut rep, String::new());
    both(
        &mut rep,
        "== sample ledger per (mode, station, era window) — floor cells, 0 honored ==".to_string(),
    );
    both(
        &mut rep,
        "  windows: W0 1994-12-01..1995-05-31 (pre-install) | W1 1995-06-01..1995-09-30 (BVR install/tests) | W2 1995-10-01..1995-12-31 (post-BVR standard + 5.12.1995) | W3 1996-01-01..1996-04-30 (pre-DGT) | W4 1996-05-01..1996-10-31 (DGT) | W5 1996-11-01..1997-02-28 (array era)".to_string(),
    );
    let windows: [(&str, &str, &str); 6] = [
        ("W0", "1994-12-01", "1995-05-31"),
        ("W1", "1995-06-01", "1995-09-30"),
        ("W2", "1995-10-01", "1995-12-31"),
        ("W3", "1996-01-01", "1996-04-30"),
        ("W4", "1996-05-01", "1996-10-31"),
        ("W5", "1996-11-01", "1997-02-28"),
    ];
    for mode in [1i64, 2] {
        let stations: BTreeSet<i64> = rows
            .iter()
            .filter(|c| c.mode == mode && c.floor)
            .map(|c| c.st)
            .collect();
        for st in &stations {
            for (wname, w0, w1) in windows {
                let sub: Vec<&Cell> = rows
                    .iter()
                    .filter(|c| {
                        let dstr = civil_date_str(c.unix_day);
                        c.mode == mode
                            && c.floor
                            && c.st == *st
                            && dstr.as_str() >= w0
                            && dstr.as_str() <= w1
                    })
                    .collect();
                let days: BTreeSet<i64> = sub.iter().map(|c| c.day).collect();
                let loud = sub.iter().filter(|c| c.rms >= LOUD_HZ).count();
                if sub.is_empty() {
                    file(
                        &mut rep,
                        format!("  mode {mode} st{st} {wname}: n cells 0 (absent)"),
                    );
                } else {
                    let rms_v: Vec<f64> = sub.iter().map(|c| c.rms).collect();
                    both(
                        &mut rep,
                        format!(
                            "  mode {mode} st{st} {wname}: {} cells / {} days / med {} Hz / loud {loud}",
                            sub.len(),
                            days.len(),
                            fmt_med(median(&rms_v))
                        ),
                    );
                }
            }
        }
    }

    both(&mut rep, String::new());
    both(
        &mut rep,
        "== anchor-day decomposition (exact date labels, floor + strong, modes 1 and 2) =="
            .to_string(),
    );
    for anchor in ["1995-11-24", "1996-06-26"] {
        both(&mut rep, format!("--- anchor {anchor} ---"));
        for c in rows.iter().filter(|c| civil_date_str(c.unix_day) == anchor) {
            both(
                &mut rep,
                format!(
                    "  {} mode {} st{} {} n {:<8} RMS {:10.4} Hz  {}",
                    civil_date_str(c.unix_day),
                    c.mode,
                    c.st,
                    if c.floor { "floor " } else { "strong" },
                    c.n,
                    c.rms,
                    if c.rms >= LOUD_HZ { "LOUD" } else { "quiet" }
                ),
            );
        }
    }
    both(&mut rep, String::new());
    both(
        &mut rep,
        "  same-station floor comparison across the two anchors (mode, station, 1995-11-24 RMS vs 1996-06-26 RMS): a present station on both dates carries two numbers; one present date carries one; absent on both stays absent".to_string(),
    );
    for mode in [1i64, 2] {
        let a_rows: Vec<&Cell> = rows
            .iter()
            .filter(|c| c.mode == mode && c.floor && civil_date_str(c.unix_day) == "1995-11-24")
            .collect();
        let b_rows: Vec<&Cell> = rows
            .iter()
            .filter(|c| c.mode == mode && c.floor && civil_date_str(c.unix_day) == "1996-06-26")
            .collect();
        let sts: BTreeSet<i64> = a_rows
            .iter()
            .map(|c| c.st)
            .chain(b_rows.iter().map(|c| c.st))
            .collect();
        for st in &sts {
            let fa = a_rows.iter().find(|c| c.st == *st);
            let fb = b_rows.iter().find(|c| c.st == *st);
            let sfa = match fa {
                Some(c) => format!("{:10.4}", c.rms),
                None => "-".to_string(),
            };
            let sfb = match fb {
                Some(c) => format!("{:10.4}", c.rms),
                None => "-".to_string(),
            };
            let na = match fa {
                Some(c) => c.n.to_string(),
                None => "-".to_string(),
            };
            let nb = match fb {
                Some(c) => c.n.to_string(),
                None => "-".to_string(),
            };
            both(
                &mut rep,
                format!(
                    "  mode {mode} st{st}: 1995-11-24 {sfa} Hz | 1996-06-26 {sfb} Hz | n {na} / {nb}",
                ),
            );
        }
    }

    both(&mut rep, String::new());
    both(
        &mut rep,
        format!(
            "== bracket test: floor cell-RMS median in [{anchor_before} d before, anchor) vs (anchor, {anchor_after} d after], per (mode, station); sides with no cell are reported absent (0 honored)",
            anchor_before = BRACKET_DAYS,
            anchor_after = BRACKET_DAYS
        ),
    );
    for (name, y, m, d) in ANCHORS {
        let au = days_from_civil(*y as i64, *m, *d);
        for mode in [1i64, 2] {
            let stations: BTreeSet<i64> = rows
                .iter()
                .filter(|c| c.mode == mode && c.floor)
                .map(|c| c.st)
                .collect();
            for st in &stations {
                let pre: Vec<&Cell> = rows
                    .iter()
                    .filter(|c| {
                        c.mode == mode
                            && c.floor
                            && c.st == *st
                            && c.unix_day >= au - BRACKET_DAYS
                            && c.unix_day < au
                    })
                    .collect();
                let post: Vec<&Cell> = rows
                    .iter()
                    .filter(|c| {
                        c.mode == mode
                            && c.floor
                            && c.st == *st
                            && c.unix_day > au
                            && c.unix_day <= au + BRACKET_DAYS
                    })
                    .collect();
                if pre.is_empty() && post.is_empty() {
                    continue;
                }
                let pre_rms: Vec<f64> = pre.iter().map(|c| c.rms).collect();
                let post_rms: Vec<f64> = post.iter().map(|c| c.rms).collect();
                let (mp, mn) = (median(&pre_rms), median(&post_rms));
                let ratio = match (mp, mn) {
                    (Some(p), Some(q)) if p > 0.0 && q > 0.0 => format!("{:.2}x", q / p),
                    (None, Some(_)) | (Some(_), None) => "-".to_string(),
                    _ => "-".to_string(),
                };
                let mut tag = String::new();
                match (mp, mn) {
                    (Some(p), Some(q)) => {
                        if q >= 3.0 * p && p > 0.0 {
                            tag = "  STEP UP".to_string();
                        } else if p >= 3.0 * q && q > 0.0 {
                            tag = "  STEP DOWN".to_string();
                        }
                    }
                    _ => {}
                }
                let geom_pre = if pre.is_empty() {
                    "".to_string()
                } else {
                    format!("  {}", geo_str(&geo, pre[pre.len() / 2].day))
                };
                both(
                    &mut rep,
                    format!(
                        "  {name}  mode {mode} st{st}: pre n {} med {} Hz | post n {} med {} Hz | ratio {ratio}{tag}{geom_pre}",
                        pre.len(),
                        fmt_rms(mp),
                        post.len(),
                        fmt_rms(mn)
                    ),
                );
            }
        }
    }

    both(&mut rep, String::new());
    both(
        &mut rep,
        "== step search: day-to-day floor cell-RMS transitions with |dlog10| >= 0.7 (~5x) within one (mode, station); midpoint date and nearest anchor within +-10 days ==".to_string(),
    );
    let mut steps: Vec<(i64, i64, f64, f64, String)> = Vec::new();
    for mode in [1i64, 2] {
        let stations: BTreeSet<i64> = rows
            .iter()
            .filter(|c| c.mode == mode && c.floor)
            .map(|c| c.st)
            .collect();
        for st in &stations {
            let mut series: Vec<&Cell> = rows
                .iter()
                .filter(|c| c.mode == mode && c.floor && c.st == *st)
                .collect();
            series.sort_by_key(|c| c.day);
            for w in series.windows(2) {
                let (a, b) = (w[0], w[1]);
                if a.rms <= 0.0 || b.rms <= 0.0 {
                    continue;
                }
                let dl = (b.rms.log10() - a.rms.log10()).abs();
                if dl >= STEP_LOG {
                    let mid_ud = (a.unix_day + b.unix_day) / 2;
                    let mut near: Vec<String> = Vec::new();
                    for (name, y, m, d) in ANCHORS {
                        let au = days_from_civil(*y as i64, *m, *d);
                        let dist = (mid_ud - au).abs();
                        if dist <= NEAR_ANCHOR_DAYS {
                            near.push(format!("{name}@{}d", dist));
                        }
                    }
                    let near_s = if near.is_empty() {
                        "none".to_string()
                    } else {
                        near.join("; ")
                    };
                    let mid_d = civil_date_str(mid_ud);
                    steps.push((mode, *st, a.rms, b.rms, mid_d.clone()));
                    both(
                        &mut rep,
                        format!(
                            "  mode {mode} st{st}: {} -> {}  {:.4} -> {:.4} Hz (x{:.1})  near: {near_s}",
                            civil_date_str(a.unix_day),
                            civil_date_str(b.unix_day),
                            a.rms,
                            b.rms,
                            b.rms / a.rms
                        ),
                    );
                }
            }
        }
    }
    both(
        &mut rep,
        format!("  total step transitions: {}", steps.len()),
    );

    both(&mut rep, String::new());
    both(
        &mut rep,
        "== per (mode, station) floor month medians (chronological) ==".to_string(),
    );
    for mode in [1i64, 2] {
        let stations: BTreeSet<i64> = rows
            .iter()
            .filter(|c| c.mode == mode && c.floor)
            .map(|c| c.st)
            .collect();
        for st in &stations {
            let mut bym: BTreeMap<String, Vec<f64>> = BTreeMap::new();
            let mut loud_m: BTreeMap<String, usize> = BTreeMap::new();
            for c in rows
                .iter()
                .filter(|c| c.mode == mode && c.floor && c.st == *st)
            {
                let mk = month_str(c.unix_day);
                bym.entry(mk.clone()).or_default().push(c.rms);
                if c.rms >= LOUD_HZ {
                    *loud_m.entry(mk).or_insert(0) += 1;
                }
            }
            for (mk, v) in &bym {
                let loud = loud_m.get(mk).copied().map_or(0, |x| x);
                both(
                    &mut rep,
                    format!(
                        "  mode {mode} st{st} {mk}: n {:<3} med {} Hz / {} loud",
                        v.len(),
                        fmt_rms(median(v)),
                        loud
                    ),
                );
            }
        }
    }

    both(&mut rep, String::new());
    both(
        &mut rep,
        "== geometry counter across the era: floor cells carry r/eps/alpha per day; rows below list mode 1 floor cells with loud marker so a geometry coincidence with the loud states is visible ==".to_string(),
    );
    for c in rows.iter().filter(|c| c.mode == 1 && c.floor) {
        file(
            &mut rep,
            format!(
                "  {} st{} n {:<8} RMS {:10.4} Hz {}  {}",
                civil_date_str(c.unix_day),
                c.st,
                c.n,
                c.rms,
                if c.rms >= LOUD_HZ { "LOUD" } else { "quiet" },
                geo_str(&geo, c.day)
            ),
        );
    }
    both(
        &mut rep,
        format!(
            "  (the {n} mode-1 floor rows above are in the report file only)",
            n = rows.iter().filter(|c| c.mode == 1 && c.floor).count()
        ),
    );

    both(&mut rep, String::new());
    both(
        &mut rep,
        "== geometry at the anchor civil dates (ephemeris at the day start) ==".to_string(),
    );
    for (name, y, m, d) in ANCHORS {
        let ud = days_from_civil(*y as i64, *m, *d);
        let t = (ud as f64 - 10957.5) * DAY_S;
        let s_geo = match (
            body_barycenter_position("galileo_daily", t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) {
            (Some(p), Some(e)) => {
                let rp = norm(p) / AU_M;
                let re = norm(e);
                let e_to_p = sub(p, e);
                let rep_ = norm(e_to_p);
                let al = (dot(e, p) / (re * rp).max(1e-30))
                    .clamp(-1.0, 1.0)
                    .acos()
                    .to_degrees();
                let ep = (dot(sub([0.0; 3], e), e_to_p) / (re * rep_).max(1e-30))
                    .clamp(-1.0, 1.0)
                    .acos()
                    .to_degrees();
                format!("r {rp:.3} AU  eps {ep:5.1}  alpha {al:5.1}")
            }
            _ => "r --  eps --  alpha --".to_string(),
        };
        both(&mut rep, format!("  {name}: {s_geo}"));
    }

    let _ = std::fs::write(&report, rep.join("\n") + "\n");
    eprintln!("galileo: ops-era floor report written to {report}");
}
