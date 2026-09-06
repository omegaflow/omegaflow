use std::collections::{BTreeMap, BTreeSet, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CELL: usize = 30;
const MIN_PASS_QUART: usize = 120;
const PASS_GAP_S: f64 = 600.0;
const AU_M: f64 = 1.495978707e11;
const YEAR: i64 = 1996;
const NEAR_DAYS: i64 = 30;
const MIN_SPEAR: usize = 8;
const LOUD_HZ: f64 = 3.0;
const OUT: &str = "reports/galileo_1996_rest_split.txt";

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
    if na > 0.0 && nb > 0.0 {
        Some((dot(a, b) / (na * nb)).clamp(-1.0, 1.0).acos().to_degrees())
    } else {
        None
    }
}
fn rms(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let n = vals.len() as f64;
    let m = vals.iter().sum::<f64>() / n;
    let v = (vals.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / n).sqrt();
    if v.is_finite() {
        Some(v)
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
fn median_trim_top(vals: &[f64], k: usize) -> Option<f64> {
    if vals.len() <= k {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    let cut = &s[..s.len() - k];
    Some(cut[cut.len() / 2])
}
fn day_of(t: f64) -> i64 {
    (t / DAY_S).floor() as i64
}
fn civil(day: i64) -> Option<(i32, u32, u32)> {
    let jd = 2451545.0 + day as f64;
    let unix_day = (jd - 2440587.5).round() as i64;
    civil_from_days(unix_day).map(|(y, m, d)| (y as i32, m, d))
}
fn date_of_day(day: i64) -> String {
    match civil(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day {day}"),
    }
}
fn dt_str(t: f64) -> String {
    let day = (t / DAY_S).floor() as i64;
    let frac = t - day as f64 * DAY_S;
    let mins = (frac * 1440.0).round() as i64;
    match civil(day) {
        Some((y, mo, d)) => format!(
            "{y:04}-{mo:02}-{d:02} {:02}:{:02}",
            mins / 60,
            mins % 60
        ),
        None => format!("tdb {t:.0}"),
    }
}
fn region(eps: f64) -> &'static str {
    if !eps.is_finite() {
        "NOGEO"
    } else if eps <= 30.0 {
        "CONJ"
    } else if eps >= 150.0 {
        "OPP"
    } else {
        "MID"
    }
}
fn fmt_o(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.3}"),
        _ => "-".to_string(),
    }
}
fn fmt_o2(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.2}"),
        _ => "-".to_string(),
    }
}
fn load(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let p = format!("data/ssd.jpl.nasa.gov/ephemeris_{name}.bin");
    std::fs::read(&p)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| eph.insert(name.to_string(), e))
        .is_some()
}
fn rms_drop_k(vals: &[f64], k: usize) -> Option<f64> {
    if vals.is_empty() || k >= vals.len() {
        return None;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    let mut idx: Vec<usize> = (0..vals.len()).collect();
    idx.sort_by(|a, b| {
        (vals[*a] - m)
            .abs()
            .total_cmp(&(vals[*b] - m).abs())
            .then_with(|| a.cmp(b))
    });
    let kept: Vec<f64> = idx[..vals.len() - k].iter().map(|i| vals[*i]).collect();
    rms(&kept)
}
fn quartile_rms(vals: &[f64], q: usize) -> Option<f64> {
    let n = vals.len();
    let lo = q * n / 4;
    let hi = (q + 1) * n / 4;
    if hi <= lo {
        return None;
    }
    rms(&vals[lo..hi])
}
fn spearman(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len();
    if n < MIN_SPEAR || n != y.len() {
        return None;
    }
    let mut xi: Vec<usize> = (0..n).collect();
    let mut yi: Vec<usize> = (0..n).collect();
    xi.sort_by(|a, b| x[*a].total_cmp(&x[*b]));
    yi.sort_by(|a, b| y[*a].total_cmp(&y[*b]));
    let rx = rank(&xi, x);
    let ry = rank(&yi, y);
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
fn rank(ord: &[usize], v: &[f64]) -> Vec<f64> {
    let n = v.len();
    let mut r = vec![0.0f64; n];
    let mut i = 0usize;
    while i < n {
        let mut j = i + 1;
        while j < n && v[ord[j]] == v[ord[i]] {
            j += 1;
        }
        let avg = ((i + j - 1) as f64) / 2.0;
        for k in ord[i..j].iter() {
            r[*k] = avg;
        }
        i = j;
    }
    r
}

struct CellAgg {
    sum: f64,
    sum2: f64,
    n: usize,
    t0: f64,
}

struct CellRow {
    mode: i64,
    station: i64,
    day: i64,
    rms: f64,
}

struct PassRec {
    mode: i64,
    station: i64,
    t0: f64,
    t1: f64,
    xs: Vec<f64>,
}

fn main() {
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
    let n_total = recs.len();
    let n_lock_total = recs.iter().filter(|r| r[1].abs() > LOCK_HZ).count();

    let mut sel: Vec<[f64; 8]> = Vec::new();
    for r in &recs {
        let day = day_of(r[0]);
        let Some((y, _, _)) = civil(day) else {
            continue;
        };
        if y as i64 != YEAR {
            continue;
        }
        sel.push(*r);
    }
    drop(recs);
    let n_year = sel.len();
    let n_lock_year = sel.iter().filter(|r| r[1].abs() > LOCK_HZ).count();

    let days: BTreeSet<i64> = sel.iter().map(|r| day_of(r[0])).collect();

    let mut geo: BTreeMap<i64, (f64, f64, f64)> = BTreeMap::new();
    for d in &days {
        let t = *d as f64 * DAY_S;
        let (Some(p), Some(e)) = (
            body_barycenter_position("galileo_daily", t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) else {
            continue;
        };
        let Some(eps) = ang(sub([0.0; 3], e), sub(p, e)) else {
            continue;
        };
        let Some(alpha) = ang(p, e) else {
            continue;
        };
        let au = norm(p) / AU_M;
        if eps.is_finite() && alpha.is_finite() && au.is_finite() {
            geo.insert(*d, (eps, alpha, au));
        }
    }
    drop(eph);
    let n_geom = geo.len();
    let no_geom = days.len().saturating_sub(n_geom);

    let opp_days: BTreeSet<i64> = geo
        .iter()
        .filter(|(_, (eps, _, _))| *eps >= 150.0)
        .map(|(d, _)| *d)
        .collect();
    let conj_days: BTreeSet<i64> = geo
        .iter()
        .filter(|(_, (eps, _, _))| *eps <= 30.0)
        .map(|(d, _)| *d)
        .collect();

    let nearest_opp = |day: i64| -> Option<i64> {
        opp_days.iter().map(|d| (*d - day).abs()).min()
    };

    sel.sort_by(|a, b| {
        a[3]
            .total_cmp(&b[3])
            .then(a[2].total_cmp(&b[2]))
            .then(a[0].total_cmp(&b[0]))
    });

    let mut day_pool: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    let mut stday: BTreeMap<(i64, i64, i64), CellAgg> = BTreeMap::new();
    let mut stday_opp: BTreeMap<(i64, i64, i64), Vec<f64>> = BTreeMap::new();
    let mut pass_win: Vec<PassRec> = Vec::new();

    let n = sel.len();
    let mut i = 0usize;
    while i < n {
        let mode = sel[i][3] as i64;
        let station = sel[i][2] as i64;
        let mut j = i;
        while j + 1 < n {
            let nx = &sel[j + 1];
            if nx[3] as i64 != mode || nx[2] as i64 != station {
                break;
            }
            if nx[0] - sel[j][0] > PASS_GAP_S {
                break;
            }
            j += 1;
        }
        let mut xs: Vec<f64> = Vec::new();
        for k in i..=j {
            let r = &sel[k];
            let day = day_of(r[0]);
            if r[1].abs() <= LOCK_HZ {
                day_pool.entry((mode, day)).or_default().push(r[1]);
                let e = stday
                    .entry((mode, station, day))
                    .or_insert_with(|| CellAgg {
                        sum: 0.0,
                        sum2: 0.0,
                        n: 0,
                        t0: r[0],
                    });
                e.sum += r[1];
                e.sum2 += r[1] * r[1];
                e.n += 1;
                if r[0] < e.t0 {
                    e.t0 = r[0];
                }
                if opp_days.contains(&day) {
                    stday_opp.entry((mode, station, day)).or_default().push(r[1]);
                }
                xs.push(r[1]);
            }
        }
        let d0 = day_of(sel[i][0]);
        let d1 = day_of(sel[j][0]);
        let over_opp = (d0..=d1).any(|d| opp_days.contains(&d));
        if over_opp {
            pass_win.push(PassRec {
                mode,
                station,
                t0: sel[i][0],
                t1: sel[j][0],
                xs,
            });
        }
        i = j + 1;
    }
    drop(sel);

    let mut rows: Vec<CellRow> = Vec::new();
    for (&(mode, station, day), a) in &stday {
        if a.n < MIN_CELL {
            continue;
        }
        let m = a.sum / a.n as f64;
        let v = (a.sum2 / a.n as f64 - m * m).max(0.0);
        let rr = v.sqrt();
        if rr.is_finite() {
            rows.push(CellRow {
                mode,
                station,
                day,
                rms: rr,
            });
        }
    }
    rows.sort_by_key(|r| (r.mode, r.day, r.station));

    let mut out: Vec<String> = Vec::new();
    out.push("galileo 1996 rest-contrast split probe — the same-era (1996) opposition remainder (pooled <=1.9x/2.4x vs conjunction): real small geometry, station/day/pass state, or outlier load?".to_string());
    out.push(format!(
        "binding: calendar year {YEAR} only, modes 1-3; lock transitions (|resid| > {LOCK_HZ:.0} Hz) excluded before noise; cell RMS about the cell mean; (mode, station, day) cell minimum {MIN_CELL} non-lock samples for cell medians; pooled (mode, day) cells count any non-lock sample (era-cycle reproduction); pass = contiguous (station, mode) arc, boundary = tdb gap > {PASS_GAP_S:.0} s"
    ));
    out.push(
        "geometry at the TDB day start from galileo_daily / earth barycentric ICRS: eps = solar elongation at the Earth, alpha = angle at the Sun, AU = heliocentric |p|".to_string(),
    );
    out.push("region: OPP eps>=150, CONJ eps<=30, MID 30<eps<150".to_string());
    out.push(format!(
        "overview: {n_total} resid samples total ({n_lock_total} lock transitions excluded dataset-wide); {YEAR}: {n_year} samples ({n_lock_year} lock); {} distinct record days, geometry resolved on {n_geom} (no geometry on {no_geom})", days.len()
    ));
    let opp_dates: Vec<String> = opp_days.iter().map(|d| date_of_day(*d)).collect();
    let conj_dates: Vec<String> = conj_days.iter().map(|d| date_of_day(*d)).collect();
    out.push(format!(
        "  {YEAR} OPP record days (n {}): {}",
        opp_days.len(),
        opp_dates.join(", ")
    ));
    out.push(format!(
        "  {YEAR} CONJ record days (n {}): {}",
        conj_days.len(),
        conj_dates.join(", ")
    ));
    for mode in [1i64, 2, 3] {
        let sts: BTreeSet<i64> = rows.iter().filter(|r| r.mode == mode).map(|r| r.station).collect();
        let nc = rows.iter().filter(|r| r.mode == mode).count();
        let dset: BTreeSet<i64> = rows.iter().filter(|r| r.mode == mode).map(|r| r.day).collect();
        out.push(format!(
            "  mode {mode}: {nc} (mode, station, day) cells n>={MIN_CELL}, {} distinct days, stations {}",
            dset.len(),
            sts.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",")
        ));
    }

    out.push(String::new());
    out.push("A. pooled (mode, day) reproduction, 1996 — median of per-(mode, day) RMS per region (era-cycle metric)".to_string());
    for mode in [1i64, 2, 3] {
        let mut byreg: BTreeMap<&'static str, Vec<f64>> = BTreeMap::new();
        for ((m, day), v) in &day_pool {
            if *m != mode {
                continue;
            }
            let Some((eps, _, _)) = geo.get(day) else {
                continue;
            };
            let Some(rr) = rms(v) else {
                continue;
            };
            byreg.entry(region(*eps)).or_default().push(rr);
        }
        let mut line = format!("  mode {mode}:");
        for name in ["OPP", "MID", "CONJ"] {
            let med_s = match byreg.get(name).map(|v| v.as_slice()) {
                Some(v) => match median(v) {
                    Some(x) => format!("{x:.3}"),
                    None => "-".to_string(),
                },
                None => "-".to_string(),
            };
            let n_s = match byreg.get(name) {
                Some(v) => format!("{:>2}", v.len()),
                None => "-".to_string(),
            };
            line.push_str(&format!("  {name} n {n_s} med {med_s}"));
        }
        out.push(line);
        let opp = byreg.get("OPP").and_then(|v| median(v));
        let conj = byreg.get("CONJ").and_then(|v| median(v));
        let ratio = match (opp, conj) {
            (Some(o), Some(c)) if c > 0.0 => format!("  mode {mode} ratio OPP/CONJ med = {:.2}x", o / c),
            _ => "  ratio: one side without n".to_string(),
        };
        out.push(ratio);
    }

    out.push(String::new());
    out.push("B. per (mode, station, day) cells on OPP record days — full listing ('*' below the 30-sample cell minimum)".to_string());
    for mode in [1i64, 2, 3] {
        let has = rows.iter().any(|r| r.mode == mode && opp_days.contains(&r.day));
        if !has && !stday.iter().any(|((m, _, d), _)| *m == mode && opp_days.contains(d)) {
            out.push(format!("  mode {mode}: no OPP-day records in {YEAR}"));
            continue;
        }
        out.push(format!("  mode {mode}:"));
        for day in &opp_days {
            let pooled = day_pool.get(&(mode, *day));
            let pr = pooled.and_then(|v| rms(v));
            let Some((eps, alpha, au)) = geo.get(day).copied() else {
                continue;
            };
            let mut line = format!(
                "    {} eps {eps:5.1} alpha {alpha:5.1} {au:4.2} AU pooled {} Hz",
                date_of_day(*day),
                fmt_o(pr)
            );
            if let Some(v) = pooled {
                line.push_str(&format!(" (n {})", v.len()));
            } else {
                line.push_str(" (n 0)");
            }
            out.push(line);
            let mut any_cell = false;
            for (&(m, st, d), a) in &stday {
                if m != mode || d != *day {
                    continue;
                }
                any_cell = true;
                let mm = a.sum / a.n as f64;
                let vv = (a.sum2 / a.n as f64 - mm * mm).max(0.0).sqrt();
                let star = if a.n < MIN_CELL { "*" } else { "" };
                out.push(format!(
                    "      st {st:>2}: cell rms {vv:9.3} Hz (n {}{star})",
                    a.n
                ));
            }
            if !any_cell {
                out.push("      (no station cell)".to_string());
            }
        }
    }

    out.push(String::new());
    out.push("C. per (mode, station) 1996 medians over (mode, station, day) cell RMS (cells n >= 30); non-OPP floor = CONJ+MID cells of the same station; opp/conj and opp/nonopp ratios".to_string());
    out.push("   mode st: opp_med(nc/nd)  conj_med(nc/nd)  nonopp_med(nc/nd)  opp:conj  opp:nonopp".to_string());
    for mode in [1i64, 2, 3] {
        let mut sts: BTreeSet<i64> = rows.iter().filter(|r| r.mode == mode).map(|r| r.station).collect();
        for (&(m, st, _), _) in &stday {
            if m == mode {
                sts.insert(st);
            }
        }
        for st in &sts {
            let mut opp: Vec<f64> = Vec::new();
            let mut conj: Vec<f64> = Vec::new();
            let mut nonopp: Vec<f64> = Vec::new();
            let mut nd_opp: BTreeSet<i64> = BTreeSet::new();
            let mut nd_conj: BTreeSet<i64> = BTreeSet::new();
            for r in &rows {
                if r.mode != mode || r.station != *st {
                    continue;
                }
                let Some((eps, _, _)) = geo.get(&r.day) else {
                    continue;
                };
                match region(*eps) {
                    "OPP" => {
                        opp.push(r.rms);
                        nd_opp.insert(r.day);
                    }
                    "CONJ" => {
                        conj.push(r.rms);
                        nd_conj.insert(r.day);
                        nonopp.push(r.rms);
                    }
                    _ => nonopp.push(r.rms),
                }
            }
            let om = median(&opp);
            let cm = median(&conj);
            let nm = median(&nonopp);
            let roc = match (om, cm) {
                (Some(o), Some(c)) if c > 0.0 => format!("{:.2}x", o / c),
                _ => "-".to_string(),
            };
            let ron = match (om, nm) {
                (Some(o), Some(nn)) if nn > 0.0 => format!("{:.2}x", o / nn),
                _ => "-".to_string(),
            };
            out.push(format!(
                "   m{mode} st {st:>2}: opp {} (c{}/d{})  conj {} (c{}/d{})  nonopp {} (c{})  {roc}  {ron}",
                fmt_o(om),
                opp.len(),
                nd_opp.len(),
                fmt_o(cm),
                conj.len(),
                nd_conj.len(),
                fmt_o(nm),
                nonopp.len()
            ));
        }
    }

    out.push(String::new());
    out.push("D. opposition-window bimodality — pooled (mode, day) RMS sorted, median, median after dropping the loudest day(s), max, and the ratio to the 1996 CONJ pooled median".to_string());
    let conj_med_pooled = |mode: i64| -> Option<f64> {
        let mut v = Vec::new();
        for ((m, day), x) in &day_pool {
            if *m != mode {
                continue;
            }
            let Some((eps, _, _)) = geo.get(day) else {
                continue;
            };
            if region(*eps) == "CONJ" {
                if let Some(rr) = rms(x) {
                    v.push(rr);
                }
            }
        }
        median(&v)
    };
    for mode in [1i64, 2, 3] {
        let mut vals: Vec<(String, f64, usize)> = Vec::new();
        for d in &opp_days {
            let Some(v) = day_pool.get(&(mode, *d)) else {
                continue;
            };
            let Some(rr) = rms(v) else {
                continue;
            };
            vals.push((date_of_day(*d), rr, v.len()));
        }
        if vals.is_empty() {
            out.push(format!("  mode {mode}: no pooled OPP day cells"));
            continue;
        }
        vals.sort_by(|a, b| a.1.total_cmp(&b.1));
        let rl: Vec<f64> = vals.iter().map(|x| x.1).collect();
        let cm = conj_med_pooled(mode);
        let med = median(&rl);
        let m1 = median_trim_top(&rl, 1);
        let m2 = median_trim_top(&rl, 2);
        let m3 = median_trim_top(&rl, 3);
        let mx = rl.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        out.push(format!("  mode {mode}: OPP days sorted (date rms n):"));
        for (dt, rr, nn) in &vals {
            out.push(format!("      {dt}  {rr:8.2} Hz  n {nn}"));
        }
        let mut ratio = String::new();
        if let Some(c) = cm {
            if c > 0.0 {
                ratio = match (med, m1, m2) {
                    (Some(md), Some(a), Some(b)) => format!(
                        "  med/conj {:.2}x  med-minus-1/conj {:.2}x  med-minus-2/conj {:.2}x",
                        md / c,
                        a / c,
                        b / c
                    ),
                    _ => String::new(),
                };
            }
        }
        out.push(format!(
            "  mode {mode}: n {n}  med {}  med-minus-1 {}  med-minus-2 {}  med-minus-3 {}  max {mx:.2} Hz  conj_med {}",
            fmt_o(med),
            fmt_o(m1),
            fmt_o(m2),
            fmt_o(m3),
            fmt_o(cm),
            n = rl.len()
        ));
        out.push(format!("  mode {mode}:{ratio}"));
    }

    out.push(String::new());
    out.push("E. pass resolution over the OPP window (whole-pass non-lock RMS, n >= 30; quartile RMS needs n >= 120)".to_string());
    out.push("   mode st: t0 .. t1  dur_min  n  rms  q1 q2 q3 q4  drop1pct  max|resid|".to_string());
    pass_win.sort_by(|a, b| {
        a.mode
            .cmp(&b.mode)
            .then(a.station.cmp(&b.station))
            .then(a.t0.total_cmp(&b.t0))
    });
    for p in &pass_win {
        let nn = p.xs.len();
        let dur = (p.t1 - p.t0) / 60.0;
        if nn < MIN_CELL {
            continue;
        }
        let Some(rr) = rms(&p.xs) else {
            continue;
        };
        let qs: Vec<String> = (0..4)
            .map(|q| {
                if nn >= MIN_PASS_QUART {
                    fmt_o2(quartile_rms(&p.xs, q))
                } else {
                    "-".to_string()
                }
            })
            .collect();
        let drop1pct = fmt_o2(rms_drop_k(
            &p.xs,
            (nn as f64 * 0.01).ceil() as usize,
        ));
        let mabs = p.xs.iter().map(|x| x.abs()).fold(f64::NEG_INFINITY, f64::max);
        out.push(format!(
            "   m{m} st {st:>2}: {} .. {}  {dur:7.1}  n {nn:>6}  {rr:8.2}  {q}  {drop1pct}  {mabs:8.2}",
            dt_str(p.t0),
            dt_str(p.t1),
            m = p.mode,
            st = p.station,
            q = qs.join(" ")
        ));
    }

    out.push(String::new());
    out.push("F. outlier load inside loud OPP (mode, station, day) cells (cell RMS >= 3 Hz, n >= 30)".to_string());
    out.push("   mode st day: n  rms  drop1  drop3  drop1pct  max|dev|  max|dev|/rms  cnt|dev|>3rms".to_string());
    for (&(mode, station, day), v) in &stday_opp {
        if v.len() < MIN_CELL {
            continue;
        }
        let Some(rr) = rms(v) else {
            continue;
        };
        if rr < LOUD_HZ {
            continue;
        }
        let m = v.iter().sum::<f64>() / v.len() as f64;
        let maxdev = v.iter().map(|x| (x - m).abs()).fold(f64::NEG_INFINITY, f64::max);
        let cnt3 = v.iter().filter(|x| (*x - m).abs() > 3.0 * rr).count();
        out.push(format!(
            "   m{mode} st {station:>2} {date}: n {n:>5}  rms {rr:9.2}  drop1 {drop1}  drop3 {drop3}  drop1pct {drop1p}  maxdev {maxdev:9.2}  ratio {ratio:.1}  cnt3 {cnt3}",
            date = date_of_day(day),
            n = v.len(),
            drop1 = fmt_o(rms_drop_k(v, 1)),
            drop3 = fmt_o(rms_drop_k(v, 3)),
            drop1p = fmt_o(rms_drop_k(v, (v.len() as f64 * 0.01).ceil() as usize)),
            ratio = maxdev / rr
        ));
    }

    out.push(String::new());
    out.push("G. same-station same-era non-OPP floor vs the OPP window (0 honored where a bin has no n)".to_string());
    out.push(format!(
        "   (mode, station, day) cells n >= {MIN_CELL}; near = day within {NEAR_DAYS} days of an OPP day"
    ));
    for mode in [1i64, 2, 3] {
        let mut sts: BTreeSet<i64> = BTreeSet::new();
        for r in &rows {
            if r.mode == mode {
                sts.insert(r.station);
            }
        }
        for st in &sts {
            let mut opp: Vec<f64> = Vec::new();
            let mut near: Vec<f64> = Vec::new();
            let mut mid: Vec<f64> = Vec::new();
            let mut conj: Vec<f64> = Vec::new();
            for r in &rows {
                if r.mode != mode || r.station != *st {
                    continue;
                }
                let Some((eps, _, _)) = geo.get(&r.day) else {
                    continue;
                };
                match region(*eps) {
                    "OPP" => opp.push(r.rms),
                    "CONJ" => {
                        conj.push(r.rms);
                        if let Some(dd) = nearest_opp(r.day) {
                            if dd <= NEAR_DAYS {
                                near.push(r.rms);
                            }
                        }
                    }
                    _ => {
                        mid.push(r.rms);
                        if let Some(dd) = nearest_opp(r.day) {
                            if dd <= NEAR_DAYS {
                                near.push(r.rms);
                            }
                        }
                    }
                }
            }
            let min_conj_dist: Option<i64> = rows
                .iter()
                .filter(|r| r.mode == mode && r.station == *st)
                .filter(|r| {
                    geo.get(&r.day)
                        .map(|(eps, _, _)| region(*eps) == "CONJ")
                        .unwrap_or(false)
                })
                .filter_map(|r| nearest_opp(r.day))
                .min();
            out.push(format!(
                "   m{mode} st {st:>2}: OPP med {} (c{}), non-OPP-floor med {} (c{}), MID med {} (c{}), CONJ med {} (c{}), near-window non-OPP med {} (c{}), min CONJ->OPP day gap {:?}",
                fmt_o(median(&opp)),
                opp.len(),
                fmt_o(median(&conj.iter().chain(mid.iter()).copied().collect::<Vec<f64>>())),
                conj.len() + mid.len(),
                fmt_o(median(&mid)),
                mid.len(),
                fmt_o(median(&conj)),
                conj.len(),
                fmt_o(median(&near)),
                near.len(),
                min_conj_dist
            ));
        }
    }

    out.push(String::new());
    out.push("H. direction consistency of the rest contrast (cells n >= 30) — the coherent case ran loud-opposition".to_string());
    for mode in [1i64, 2, 3] {
        let sts: BTreeSet<i64> = rows.iter().filter(|r| r.mode == mode).map(|r| r.station).collect();
        for st in &sts {
            let mut opp: Vec<f64> = Vec::new();
            let mut nonopp: Vec<f64> = Vec::new();
            for r in &rows {
                if r.mode != mode || r.station != *st {
                    continue;
                }
                let Some((eps, _, _)) = geo.get(&r.day) else {
                    continue;
                };
                match region(*eps) {
                    "OPP" => opp.push(r.rms),
                    _ => nonopp.push(r.rms),
                }
            }
            let om = median(&opp);
            let nm = median(&nonopp);
            let dir = match (om, nm) {
                (Some(o), Some(nn)) if o > nn => "loud-opposition",
                (Some(o), Some(nn)) if nn > o => "loud-floor",
                (Some(_), Some(_)) => "opp=floor",
                _ => "one side without n",
            };
            out.push(format!(
                "   m{mode} st {st:>2}: opp_med {} (c{}), nonopp_med {} (c{})  -> {dir}",
                fmt_o(om),
                opp.len(),
                fmt_o(nm),
                nonopp.len()
            ));
        }
    }
    out.push("   spearman of log10 cell RMS vs eps over all 1996 (mode, station, day) cells n >= 30:".to_string());
    for mode in [1i64, 2, 3] {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for r in &rows {
            if r.mode != mode {
                continue;
            }
            let Some((eps, _, _)) = geo.get(&r.day) else {
                continue;
            };
            let lg = r.rms.log10();
            if lg.is_finite() {
                xs.push(*eps);
                ys.push(lg);
            }
        }
        out.push(format!(
            "   mode {mode}: rho {rho} (n cells {n})",
            rho = fmt_o(spearman(&xs, &ys)),
            n = xs.len()
        ));
    }

    out.push(String::new());
    out.push("I. how broad is the opposition loudness across stations per OPP day (cell count per mode per OPP day, n >= 30)".to_string());
    for mode in [1i64, 2, 3] {
        let mut per_day: Vec<(i64, usize)> = Vec::new();
        for d in &opp_days {
            let c = stday
                .iter()
                .filter(|((m, _, day), a)| *m == mode && *day == *d && a.n >= MIN_CELL)
                .count();
            if c > 0 {
                per_day.push((*d, c));
            }
        }
        if per_day.is_empty() {
            continue;
        }
        out.push(format!("  mode {mode}: {}", per_day.iter().map(|(d, c)| format!("{}: {} cells", date_of_day(*d), c)).collect::<Vec<String>>().join(" | ")));
    }

    let body = out.join("\n") + "\n";
    match std::fs::write(OUT, &body) {
        Ok(()) => eprintln!("galileo: 1996 rest split report written to {OUT} ({} lines)", out.len()),
        Err(_) => eprintln!("galileo: write {OUT} void"),
    }
}
