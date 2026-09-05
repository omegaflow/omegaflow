use std::collections::{BTreeMap, BTreeSet, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CELL: usize = 30;
const AU_M: f64 = 1.495978707e11;
const OUT: &str = "reports/galileo_mode2_station_split.txt";

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

fn load(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let p = format!("data/ephemeris_{name}.bin");
    std::fs::read(&p)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| eph.insert(name.to_string(), e))
        .is_some()
}

fn jd_date(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb_s:.0}"),
    }
}

fn fmt_o(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.2}"),
        _ => "-".to_string(),
    }
}

fn main() {
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in ["galileo_daily", "earth"] {
        if !load(b, &mut eph) {
            eprintln!("galileo: {b} ephemeris bin void");
            return;
        }
    }
    let Ok(bytes) = std::fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    drop(bytes);

    let mut day_cells: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    let mut day_total: BTreeMap<i64, usize> = BTreeMap::new();
    let mut stday_cells: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    let mut n_mode2 = 0usize;
    let mut n_lock = 0usize;
    for r in &recs {
        if r[3] as i64 != 2 {
            continue;
        }
        n_mode2 += 1;
        let day = (r[0] / DAY_S).floor() as i64;
        *day_total.entry(day).or_insert(0) += 1;
        let resid = r[1];
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        let station = r[2] as i64;
        day_cells.entry(day).or_default().push(resid);
        stday_cells.entry((day, station)).or_default().push(resid);
    }
    drop(recs);
    if day_cells.is_empty() {
        eprintln!("galileo: no mode 2 samples");
        return;
    }

    let mut st_hist: BTreeMap<i64, (usize, BTreeSet<i64>)> = BTreeMap::new();
    for ((day, station), v) in &stday_cells {
        let e = st_hist.entry(*station).or_default();
        e.0 += v.len();
        e.1.insert(*day);
    }

    let mut out: Vec<String> = Vec::new();
    out.push("galileo mode-2 station split — the quiet-window pooling test".to_string());
    out.push("binding: mode 2 records only; lock transitions (|resid| > 1000 Hz) excluded before noise".to_string());
    out.push("noise cell = per (day) and per (day, station); cell RMS about the cell mean; median across cells".to_string());
    out.push(format!("day-cell minimum sample count: {MIN_CELL}"));
    out.push("geometry: alpha = angle at the Sun (Earth/probe), elong = angle at Earth (Sun/probe), from galileo_daily + earth barycenters".to_string());

    out.push(String::new());
    out.push("overview".to_string());
    out.push(format!("  mode 2 samples incl lock: {n_mode2}"));
    out.push(format!("  mode 2 lock transitions: {n_lock}"));
    let n: usize = day_cells.values().map(|v| v.len()).sum();
    out.push(format!("  mode 2 samples after lock exclusion: {n}"));
    out.push(format!("  days with any mode-2 record: {}", day_total.len()));
    out.push(format!("  days after lock exclusion (>= 1 non-lock sample): {}", day_cells.len()));
    out.push(format!(
        "  days whose mode-2 records are all lock transitions: {}",
        day_total
            .keys()
            .filter(|d| !day_cells.contains_key(d))
            .count()
    ));
    let dl = day_cells.keys().min().copied();
    let dh = day_cells.keys().max().copied();
    match (dl, dh) {
        (Some(a), Some(b)) => out.push(format!(
            "  span: {} .. {}",
            jd_date(a as f64 * DAY_S),
            jd_date(b as f64 * DAY_S)
        )),
        _ => {}
    }
    let spd: Vec<f64> = day_cells.values().map(|v| v.len() as f64).collect();
    out.push(format!(
        "  samples per day (non-lock): median {}, min {}, max {}",
        fmt_o(median(&spd)),
        spd.iter().cloned().fold(f64::INFINITY, f64::min),
        spd.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    ));
    for (s, (c, days)) in &st_hist {
        out.push(format!("  station {s}: {c} samples, {} days", days.len()));
    }

    let mut geo: BTreeMap<i64, (f64, f64, f64)> = BTreeMap::new();
    let mut no_geom = 0usize;
    let geom_days: BTreeSet<i64> = day_total.keys().copied().chain(day_cells.keys().copied()).collect();
    for d in &geom_days {
        let t = *d as f64 * DAY_S;
        let (Some(p), Some(e)) = (
            body_barycenter_position("galileo_daily", t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) else {
            no_geom += 1;
            continue;
        };
        let alpha_deg = ang(p, e);
        let elong_deg = ang(sub([0.0, 0.0, 0.0], e), sub(p, e));
        if !alpha_deg.is_finite() || !elong_deg.is_finite() {
            no_geom += 1;
            continue;
        }
        geo.insert(*d, (alpha_deg, elong_deg, norm(p) / AU_M));
    }
    out.push(String::new());
    out.push(format!(
        "geometry: {} days resolved, {} without ephemeris geometry",
        geo.len(),
        no_geom
    ));
    let quiet_elong: BTreeSet<i64> = geo
        .iter()
        .filter(|(_, (_, el, _))| *el < 30.0)
        .map(|(d, _)| *d)
        .collect();
    let quiet_alpha: BTreeSet<i64> = geo
        .iter()
        .filter(|(_, (al, _, _))| *al >= 150.0)
        .map(|(d, _)| *d)
        .collect();
    out.push(format!(
        "  conjunction (elong < 30 deg): {} days with any record ({} with non-lock); alpha >= 150 deg: {} with any record",
        quiet_elong.len(),
        quiet_elong.iter().filter(|d| day_cells.contains_key(d)).count(),
        quiet_alpha.len()
    ));

    let daycell = |v: &Vec<f64>| -> Option<f64> {
        if v.len() < MIN_CELL {
            return None;
        }
        let r = rms(v);
        if r.is_finite() {
            Some(r)
        } else {
            None
        }
    };

    let pooled_all: Vec<(i64, f64)> = day_cells
        .iter()
        .filter_map(|(d, v)| daycell(v).map(|r| (*d, r)))
        .collect();
    out.push(String::new());
    out.push(format!("A. pooled per-day RMS, mode 2 (cells >= {MIN_CELL} samples)"));
    let mut fmt_pool = |tag: &str, dayset: &BTreeSet<i64>| {
        let list: Vec<f64> = pooled_all
            .iter()
            .filter(|(d, _)| dayset.is_empty() || dayset.contains(d))
            .map(|(_, r)| *r)
            .collect();
        let nd: usize = pooled_all
            .iter()
            .filter(|(d, _)| dayset.is_empty() || dayset.contains(d))
            .count();
        out.push(format!(
            "  {tag}: med_day_rms {} Hz ({} day cells)",
            fmt_o(median(&list)),
            nd
        ));
    };
    fmt_pool("all mode 2 days", &BTreeSet::new());
    fmt_pool("conjunction elong < 30", &quiet_elong);
    fmt_pool("alpha >= 150", &quiet_alpha);
    {
        let list: Vec<f64> = quiet_elong
            .iter()
            .filter_map(|d| day_cells.get(d).and_then(daycell))
            .collect();
        out.push(format!(
            "  (no-min cells, any n, conjunction elong < 30): med {} Hz ({} day cells)",
            fmt_o(median(&list)),
            list.len()
        ));
    }

    out.push(String::new());
    out.push("A2. reference metric reproduction — median per-day RMS over all (mode-2, day) cells incl days whose mode-2 records are all lock (Rausch-Kurve metric, no cell minimum)".to_string());
    let mut ref_band = |tag: &str, dayset: &BTreeSet<i64>| {
        let mut list: Vec<f64> = Vec::new();
        for d in dayset {
            let r = match day_cells.get(d) {
                Some(v) => rms(v),
                None => f64::NAN,
            };
            list.push(r);
        }
        list.sort_by(f64::total_cmp);
        out.push(format!(
            "  {tag}: med {} Hz ({} day cells)",
            fmt_o(list.get(list.len() / 2).copied()),
            list.len()
        ));
    };
    ref_band("all mode-2 day cells (incl all-lock days)", &geo.keys().copied().collect());
    ref_band("elong < 30", &quiet_elong);
    ref_band("alpha >= 150", &quiet_alpha);
    {
        let mut sorted: Vec<(String, f64, f64, f64, usize)> = Vec::new();
        for d in &quiet_elong {
            let v = day_cells.get(d);
            let r = match v {
                Some(x) => rms(x),
                None => f64::NAN,
            };
            let (al, el, _au) = geo.get(d).copied().unwrap_or((f64::NAN, f64::NAN, f64::NAN));
            sorted.push((
                jd_date(*d as f64 * DAY_S),
                al,
                el,
                r,
                v.map(|x| x.len()).unwrap_or(0),
            ));
        }
        sorted.sort_by(|a, b| a.3.total_cmp(&b.3));
        out.push("  elong < 30 day cells sorted by day RMS (date, alpha, elong, day_rms, n_nonlock):".to_string());
        for (date, al, el, r, nn) in &sorted {
            out.push(format!("    {date}  alpha {al:.1}  elong {el:.1}  rms {r:.3} Hz  n {nn}"));
        }
    }

    out.push(String::new());
    out.push(format!("B. per-(day, station) RMS, mode 2 (cells >= {MIN_CELL} samples)"));
    out.push("   station: med_all = median over all station-day cells; med_conj = median over cells whose day has elong < 30".to_string());
    for (s, (_, _)) in &st_hist {
        let cells_all: Vec<((i64, i64), f64, usize)> = stday_cells
            .iter()
            .filter(|((_, st), _)| st == s)
            .filter(|(_, v)| v.len() >= MIN_CELL)
            .filter_map(|((d, st), v)| {
                let r = rms(v);
                if r.is_finite() {
                    Some(((*d, *st), r, v.len()))
                } else {
                    None
                }
            })
            .collect();
        let rms_all: Vec<f64> = cells_all.iter().map(|c| c.1).collect();
        let ncell_all: Vec<f64> = cells_all.iter().map(|c| c.2 as f64).collect();
        let ndays_all: usize = cells_all.iter().map(|c| c.0 .0).collect::<BTreeSet<i64>>().len();
        let cells_conj: Vec<((i64, i64), f64, usize)> = cells_all
            .iter()
            .filter(|c| quiet_elong.contains(&c.0 .0))
            .copied()
            .collect();
        let rms_conj: Vec<f64> = cells_conj.iter().map(|c| c.1).collect();
        let ndays_conj: usize = cells_conj.iter().map(|c| c.0 .0).collect::<BTreeSet<i64>>().len();
        out.push(format!(
            "  station {s}: med_all {} Hz ({} cells, {} days, cell n med/min/max {}/{}/{})   med_conj {} Hz ({} cells, {} days)",
            fmt_o(median(&rms_all)),
            cells_all.len(),
            ndays_all,
            fmt_o(median(&ncell_all)),
            ncell_all.iter().cloned().fold(f64::INFINITY, f64::min),
            ncell_all.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            fmt_o(median(&rms_conj)),
            cells_conj.len(),
            ndays_conj
        ));
    }

    std::fs::create_dir_all("reports").ok();
    let body = out.join("\n") + "\n";
    match std::fs::write(OUT, &body) {
        Ok(()) => eprintln!("galileo: mode 2 station split report written to {OUT}"),
        Err(_) => eprintln!("galileo: write {OUT} void"),
    }
}
