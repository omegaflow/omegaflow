use std::collections::{BTreeMap, BTreeSet, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CELL: usize = 30;
const AU_M: f64 = 1.495978707e11;
const PASS_GAP_S: f64 = 600.0;
const TARGET_YEAR: u32 = 1996;
const OUT: &str = "state/reports/galileo_1996_residual.txt";

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn ang(a: [f64; 3], b: [f64; 3]) -> f64 {
    let na = norm(a);
    let nb = norm(b);
    if na <= 0.0 || nb <= 0.0 {
        return f64::NAN;
    }
    (dot(a, b) / (na * nb)).clamp(-1.0, 1.0).acos().to_degrees()
}
fn rms(vals: &[f64]) -> f64 {
    if vals.is_empty() {
        return f64::NAN;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    (vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64).sqrt()
}
fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}
fn rms_drop_k(vals: &[f64], k: usize) -> Option<f64> {
    if vals.is_empty() || k >= vals.len() {
        return None;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    let mut idx: Vec<usize> = (0..vals.len()).collect();
    idx.sort_by(|a, b| (vals[*a] - m).abs().total_cmp(&(vals[*b] - m).abs()));
    let kept: Vec<f64> = idx[..vals.len() - k].iter().map(|i| vals[*i]).collect();
    let mm = kept.iter().sum::<f64>() / kept.len() as f64;
    let rr = (kept.iter().map(|v| (v - mm) * (v - mm)).sum::<f64>() / kept.len() as f64).sqrt();
    if rr.is_finite() {
        Some(rr)
    } else {
        None
    }
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
fn load(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let p = format!("data/ssd.jpl.nasa.gov/ephemeris_{name}.bin");
    std::fs::read(&p)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| eph.insert(name.to_string(), e))
        .is_some()
}
fn fmt_o(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.2}"),
        _ => "-".to_string(),
    }
}
fn day_of(t: f64) -> i64 {
    (t / DAY_S).floor() as i64
}
fn year_of_day(day: i64) -> Option<u32> {
    let jd = 2451545.0 + day as f64;
    let unix_day = (jd - 2440587.5).round() as i64;
    civil_from_days(unix_day).map(|(y, _, _)| y)
}
fn date_of_day(day: i64) -> String {
    let jd = 2451545.0 + day as f64;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day {day}"),
    }
}
fn dt_str(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / DAY_S;
    let day_f = jd - 2440587.5;
    let unix_day = day_f.floor() as i64;
    let frac = day_f - unix_day as f64;
    let secs = (frac * DAY_S).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!(
            "{y:04}-{m:02}-{d:02} {:02}:{:02}:{:02}",
            secs / 3600,
            (secs % 3600) / 60,
            secs % 60
        ),
        None => format!("tdb {tdb_s:.0}"),
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

struct PassRow {
    mode: i64,
    station: i64,
    t0: f64,
    t1: f64,
    n_xs: usize,
    rms: Option<f64>,
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

    let mut sel: Vec<[f64; 8]> = Vec::new();
    let mut per_mode_day: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    for r in &recs {
        let mode = r[3] as i64;
        if mode < 1 || mode > 3 {
            continue;
        }
        let day = day_of(r[0]);
        let Some(y) = year_of_day(day) else {
            continue;
        };
        if y != TARGET_YEAR {
            continue;
        }
        sel.push(*r);
        *per_mode_day.entry((mode, day)).or_insert(0) += 1;
    }
    let total_recs = recs.len();
    drop(recs);
    if sel.is_empty() {
        eprintln!("galileo: no {TARGET_YEAR} mode 1-3 samples");
        return;
    }

    let mut days: BTreeSet<i64> = BTreeSet::new();
    let mut n_lock = 0usize;
    for r in &sel {
        days.insert(day_of(r[0]));
        if r[1].abs() > LOCK_HZ {
            n_lock += 1;
        }
    }

    let mut geo: BTreeMap<i64, (f64, f64, f64)> = BTreeMap::new();
    let mut no_geom = 0usize;
    for d in &days {
        let t = *d as f64 * DAY_S;
        let (Some(p), Some(e)) = (
            body_barycenter_position("galileo_daily", t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) else {
            no_geom += 1;
            continue;
        };
        let eps = ang(sub([0.0; 3], e), sub(p, e));
        let alpha = ang(p, e);
        let au = norm(p) / AU_M;
        if !eps.is_finite() || !alpha.is_finite() || !au.is_finite() {
            no_geom += 1;
            continue;
        }
        geo.insert(*d, (eps, alpha, au));
    }
    drop(eph);

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

    sel.sort_by(|a, b| {
        a[3].total_cmp(&b[3])
            .then(a[2].total_cmp(&b[2]))
            .then(a[0].total_cmp(&b[0]))
    });

    let mut stday: BTreeMap<(i64, i64, i64), Vec<f64>> = BTreeMap::new();
    let mut stday_n: BTreeMap<(i64, i64, i64), usize> = BTreeMap::new();
    let mut day_pool: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    let mut window_passes: Vec<PassRow> = Vec::new();
    let mut all_pass_count: BTreeMap<(i64, i64), usize> = BTreeMap::new();

    let n = sel.len();
    let mut i = 0usize;
    while i < n {
        let mode = sel[i][3] as i64;
        let station = sel[i][2] as i64;
        let mut j = i;
        while j + 1 < n {
            let next = &sel[j + 1];
            if next[3] as i64 != mode || next[2] as i64 != station {
                break;
            }
            if next[0] - sel[j][0] > PASS_GAP_S {
                break;
            }
            j += 1;
        }
        let mut xs: Vec<f64> = Vec::new();
        for k in i..=j {
            let r = &sel[k];
            let day = day_of(r[0]);
            *stday_n.entry((mode, station, day)).or_insert(0) += 1;
            if r[1].abs() <= LOCK_HZ {
                xs.push(r[1]);
                stday.entry((mode, station, day)).or_default().push(r[1]);
                day_pool.entry((mode, day)).or_default().push(r[1]);
            }
        }
        *all_pass_count.entry((mode, station)).or_insert(0) += 1;
        let pass_rms = if xs.len() >= MIN_CELL {
            let rr = rms(&xs);
            if rr.is_finite() {
                Some(rr)
            } else {
                None
            }
        } else {
            None
        };
        let in_window = (day_of(sel[i][0])..=day_of(sel[j][0])).any(|d| opp_days.contains(&d));
        if in_window {
            window_passes.push(PassRow {
                mode,
                station,
                t0: sel[i][0],
                t1: sel[j][0],
                n_xs: xs.len(),
                rms: pass_rms,
            });
        }
        i = j + 1;
    }
    drop(sel);

    let n_nonlock: usize = stday.values().map(|v| v.len()).sum();
    let n_cells = stday.len();
    let mut out: Vec<String> = Vec::new();
    out.push("galileo 1996 residual probe — the same-era opposition/conjunction contrast (<=2.4x): real small geometry, station effect, or outlier load?".to_string());
    out.push(format!(
        "binding: year {TARGET_YEAR} only, modes 1-3; lock transitions (|resid| > {LOCK_HZ:.0} Hz) excluded before noise; noise = RMS about the cell mean; cell minimum {MIN_CELL} non-lock samples for cell medians; pooled day cells count any non-lock day (era-cycle reproduction); pass = contiguous (station, mode) tracking arc, boundary = tdb gap > {PASS_GAP_S:.0} s"
    ));
    out.push("geometry: eps = solar elongation at the Earth (Sun-Earth-probe), alpha = angle at the Sun, AU heliocentric, at the TDB day start".to_string());
    out.push(String::new());
    out.push("overview".to_string());
    out.push(format!("  resid samples total: {total_recs}"));
    out.push(format!(
        "  {TARGET_YEAR} mode 1-3 samples: {} ({} lock transitions excluded)",
        n_nonlock + n_lock,
        n_lock
    ));
    out.push(format!(
        "  {TARGET_YEAR} non-lock samples: {n_nonlock}; (mode, station, day) cells: {n_cells}"
    ));
    out.push(format!(
        "  distinct days with records: {}; geometry resolved: {} days, {} without ephemeris geometry",
        days.len(),
        geo.len(),
        no_geom
    ));
    out.push(format!(
        "  region day counts (any record): OPP (eps>=150) {}, CONJ (eps<=30) {}, MID {}",
        opp_days.len(),
        conj_days.len(),
        geo.len() - opp_days.len() - conj_days.len()
    ));
    let mut per_mode_records: BTreeMap<i64, usize> = BTreeMap::new();
    for ((mode, _), v) in &day_pool {
        *per_mode_records.entry(*mode).or_insert(0) += v.len();
    }
    for mode in [1i64, 2, 3] {
        out.push(format!(
            "  mode {mode}: {} non-lock samples, {} day cells, {} (mode, station, day) cells, {} passes",
            per_mode_records.get(&mode).copied().map_or(0, |c| c),
            day_pool.iter().filter(|((m, _), _)| *m == mode).count(),
            stday.iter().filter(|((m, _, _), _)| *m == mode).count(),
            all_pass_count.iter().filter(|((m, _), _)| *m == mode).map(|(_, c)| c).sum::<usize>()
        ));
    }
    let mut stations: BTreeSet<i64> = BTreeSet::new();
    for ((_, st, _), _) in &stday {
        stations.insert(*st);
    }
    out.push(format!(
        "  stations in {TARGET_YEAR}: {}",
        stations
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    ));
    out.push(format!(
        "  opposition window days: {}",
        opp_days
            .iter()
            .map(|d| date_of_day(*d))
            .collect::<Vec<String>>()
            .join(", ")
    ));

    out.push(String::new());
    out.push("A. pooled day-cell median reproduction, 1996 (era-cycle metric: median of per-(mode, day) RMS, any non-lock n)".to_string());
    for mode in [1i64, 2, 3] {
        let mut regs: BTreeMap<&'static str, Vec<f64>> = BTreeMap::new();
        for ((m, day), v) in &day_pool {
            if *m != mode {
                continue;
            }
            let (Some((eps, _, _)), Some(rr)) = (geo.get(day), Some(rms(v))) else {
                continue;
            };
            if !rr.is_finite() {
                continue;
            }
            regs.entry(region(*eps)).or_default().push(rr);
        }
        let med_opp = median(regs.get("OPP").map(|v| v.as_slice()).unwrap_or(&[]));
        let med_conj = median(regs.get("CONJ").map(|v| v.as_slice()).unwrap_or(&[]));
        let mut line = format!("  mode {mode}:");
        for rl in ["CONJ", "MID", "OPP"] {
            let v = regs.get(rl);
            line.push_str(&format!(
                "  {rl} n {:>2} med {:6.2} Hz",
                v.map_or(0, |x| x.len()),
                median(v.map(|x| x.as_slice()).unwrap_or(&[])).unwrap_or(f64::NAN)
            ));
        }
        out.push(line);
        let rat = match (med_opp, med_conj) {
            (Some(o), Some(c)) if c > 0.0 => {
                format!("  mode {mode} ratio OPP/CONJ = {:.2}x", o / c)
            }
            _ => "  ratio: one side absent".to_string(),
        };
        out.push(rat);
    }

    out.push(String::new());
    out.push("B. opposition window day cells, per (mode, day, station) — n first; '*' marks a cell below the 30-sample minimum".to_string());
    for mode in [1i64, 2, 3] {
        let opp_rows: Vec<(i64, f64, f64, f64)> = opp_days
            .iter()
            .filter_map(|d| geo.get(d).map(|g| (*d, g.0, g.1, g.2)))
            .collect();
        if opp_rows.is_empty() {
            continue;
        }
        out.push(format!("  mode {mode}:"));
        for (day, eps, alpha, au) in &opp_rows {
            let pooled: Vec<f64> = match day_pool.get(&(mode, *day)) {
                Some(v) => v.clone(),
                None => Vec::new(),
            };
            let pr = rms(&pooled);
            out.push(format!(
                "    {date}  eps {eps:5.1}  alpha {alpha:5.1}  {au:4.2} AU  pooled {pr:8.2} Hz (n {n})",
                date = date_of_day(*day),
                n = pooled.len()
            ));
            for ((m, st, d), v) in &stday {
                if *m != mode || *d != *day {
                    continue;
                }
                let rr = rms(v);
                let star = if v.len() < MIN_CELL { "*" } else { "" };
                out.push(format!(
                    "      st {st:>2}: rms {rr:8.2} Hz (n {}{star})",
                    v.len()
                ));
            }
        }
    }

    out.push(String::new());
    out.push(
        "C. per-station 1996 medians over (mode, station, day) cell RMS (cells >= 30 samples)"
            .to_string(),
    );
    out.push("   st  opp_med(n)  conj_med(n)  nonopp_floor_med(n)  total_day_cells".to_string());
    for mode in [1i64, 2, 3] {
        for st in &stations {
            let mut opp: Vec<(f64, usize)> = Vec::new();
            let mut conj: Vec<(f64, usize)> = Vec::new();
            let mut floor: Vec<(f64, usize)> = Vec::new();
            let mut total_cells = 0usize;
            let mut below_min = 0usize;
            for ((m, s, day), v) in &stday {
                if *m != mode || *s != *st {
                    continue;
                }
                total_cells += 1;
                if v.len() < MIN_CELL {
                    below_min += 1;
                    continue;
                }
                let (Some((eps, _, _)), Some(rr)) = (geo.get(day), Some(rms(v))) else {
                    continue;
                };
                if !rr.is_finite() {
                    continue;
                }
                match region(*eps) {
                    "OPP" => opp.push((rr, v.len())),
                    "CONJ" => conj.push((rr, v.len())),
                    _ => floor.push((rr, v.len())),
                }
            }
            let mks = |l: &[(f64, usize)]| -> (String, String) {
                let vals: Vec<f64> = l.iter().map(|x| x.0).collect();
                (fmt_o(median(&vals)), format!("n{}", l.len()))
            };
            let (om, on) = mks(&opp);
            let (cm, cn) = mks(&conj);
            let (fm, fn_) = mks(&floor);
            out.push(format!(
                "  m{mode} st {st:>2}: opp {om} ({on})  conj {cm} ({cn})  floor {fm} ({fn_})  cells {total_cells} (below-min {below_min})"
            ));
        }
    }

    out.push(String::new());
    out.push(
        "D. bimodality of the opposition window (pooled per-(mode, day) RMS, era-cycle metric)"
            .to_string(),
    );
    for mode in [1i64, 2, 3] {
        let mut opp_vals: Vec<(String, f64, usize)> = Vec::new();
        for d in &opp_days {
            let v = day_pool.get(&(mode, *d));
            if let Some(x) = v {
                if !x.is_empty() {
                    opp_vals.push((date_of_day(*d), rms(x), x.len()));
                }
            }
        }
        let mut conj_vals: Vec<f64> = Vec::new();
        for d in &conj_days {
            if let Some(x) = day_pool.get(&(mode, *d)) {
                if !x.is_empty() {
                    conj_vals.push(rms(x));
                }
            }
        }
        if opp_vals.is_empty() {
            continue;
        }
        out.push(format!("  mode {mode}: OPP days sorted by day RMS:"));
        let mut sv: Vec<&(String, f64, usize)> = opp_vals.iter().collect();
        sv.sort_by(|a, b| a.1.total_cmp(&b.1));
        for (date, rr, nn) in sv {
            out.push(format!("      {date}  rms {rr:8.2} Hz  n {nn}"));
        }
        let rms_list: Vec<f64> = opp_vals.iter().map(|x| x.1).collect();
        let conj_med = median(&conj_vals).unwrap_or(f64::NAN);
        out.push(format!(
            "    OPP n {}  med {:.2}  med-minus-1-loudest {:.2}  med-minus-2-loudest {:.2}  max {:.2} Hz",
            rms_list.len(),
            median(&rms_list).unwrap_or(f64::NAN),
            median_trim_top(&rms_list, 1).unwrap_or(f64::NAN),
            median_trim_top(&rms_list, 2).unwrap_or(f64::NAN),
            rms_list.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
        ));
        out.push(format!(
            "    CONJ pooled n {}  med {:.2} Hz  (era-cycle reference med OPP/CONJ from A)",
            conj_vals.len(),
            conj_med
        ));
    }

    out.push(String::new());
    out.push(
        "E. outlier load inside loud opposition cells (rms >= 3 Hz, cells >= 30 samples)"
            .to_string(),
    );
    out.push("   mode st day: n, rms, rms-drop-1-sample, rms-drop-3-samples, max|resid|, count |resid| > 3*rms".to_string());
    for mode in [2i64, 3] {
        for ((m, st, day), v) in &stday {
            if *m != mode || !opp_days.contains(day) || v.len() < MIN_CELL {
                continue;
            }
            let rr = rms(v);
            if !rr.is_finite() || rr < 3.0 {
                continue;
            }
            let mabs = v.iter().map(|x| x.abs()).fold(f64::NEG_INFINITY, f64::max);
            let over = v.iter().filter(|x| x.abs() > 3.0 * rr).count();
            out.push(format!(
                "   m{m} st {st:>2} {date}: n {n:>5}  rms {rr:9.2}  drop1 {drop1}  drop3 {drop3}  max {mabs:9.2}  >3rms {over}",
                date = date_of_day(*day),
                n = v.len(),
                drop1 = fmt_o(rms_drop_k(v, 1)),
                drop3 = fmt_o(rms_drop_k(v, 3))
            ));
        }
    }

    out.push(String::new());
    out.push("F. pass resolution over the opposition window (whole pass RMS, n >= 30)".to_string());
    window_passes.sort_by(|a, b| {
        a.mode
            .cmp(&b.mode)
            .then(a.station.cmp(&b.station))
            .then(a.t0.total_cmp(&b.t0))
    });
    for p in &window_passes {
        let dur_min = (p.t1 - p.t0) / 60.0;
        out.push(format!(
            "   m{mode} st {station:>2}: {t0} .. {t1}  dur {dur_min:7.1} min  n {n:>6}  pass_rms {pr} Hz",
            mode = p.mode,
            station = p.station,
            t0 = dt_str(p.t0),
            t1 = dt_str(p.t1),
            n = p.n_xs,
            pr = fmt_o(p.rms)
        ));
    }

    out.push(String::new());
    out.push("G. direction consistency per station (opp_med vs conj_med, cells >= 30): the coherent-case direction is loud-opposition".to_string());
    for mode in [1i64, 2, 3] {
        let mut same_dir = 0usize;
        let mut opp_dir = 0usize;
        let mut none = 0usize;
        let mut detail: Vec<String> = Vec::new();
        for st in &stations {
            let mut opp: Vec<f64> = Vec::new();
            let mut conj: Vec<f64> = Vec::new();
            for ((m, s, day), v) in &stday {
                if *m != mode || *s != *st || v.len() < MIN_CELL {
                    continue;
                }
                let (Some((eps, _, _)), Some(rr)) = (geo.get(day), Some(rms(v))) else {
                    continue;
                };
                if !rr.is_finite() {
                    continue;
                }
                match region(*eps) {
                    "OPP" => opp.push(rr),
                    "CONJ" => conj.push(rr),
                    _ => {}
                }
            }
            let (om, cm) = (median(&opp), median(&conj));
            match (om, cm) {
                (Some(o), Some(c)) => {
                    if o > c {
                        same_dir += 1;
                        detail.push(format!(
                            "     st {st:>2}: opp {o:.2} > conj {c:.2} ({:.1}x)  [loud-opposition]",
                            o / c
                        ));
                    } else if c > o {
                        opp_dir += 1;
                        detail.push(format!("     st {st:>2}: conj {c:.2} > opp {o:.2} ({:.1}x)  [loud-conjunction]", c / o));
                    } else {
                        detail.push(format!("     st {st:>2}: opp = conj = {o:.2}"));
                    }
                }
                _ => {
                    none += 1;
                    detail.push(format!(
                        "     st {st:>2}: one side absent (opp n {}, conj n {})",
                        opp.len(),
                        conj.len()
                    ));
                }
            }
        }
        out.push(format!(
            "   mode {mode}: stations with loud-opposition {same_dir}, loud-conjunction {opp_dir}, one side absent {none}"
        ));
        for d in detail {
            out.push(d);
        }
    }

    std::fs::create_dir_all("reports").ok();
    let body = out.join("\n") + "\n";
    match std::fs::write(OUT, &body) {
        Ok(()) => eprintln!("galileo: 1996 residual probe report written to {OUT}"),
        Err(_) => eprintln!("galileo: write {OUT} void"),
    }
}
