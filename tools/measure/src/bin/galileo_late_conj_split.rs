use std::collections::{BTreeMap, BTreeSet, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CELL: usize = 30;
const CONJ_ELONG_DEG: f64 = 30.0;
const FLOOR_STRENGTH: f64 = -2560.0;
const PLATEAU_STRENGTH: f64 = -1900.0;
const STUDY_YEAR: i32 = 1997;
const STATIONS: [i64; 3] = [14, 43, 63];

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
    std::fs::read(format!("data/ephemeris_{name}.bin"))
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| {
            eph.insert(name.to_string(), e);
        })
        .is_some()
}
fn ymd(tdb_day: i64) -> (i32, u32, u32) {
    let jd = 2451545.0 + tdb_day as f64;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => (y as i32, m, d),
        None => (0, 0, 0),
    }
}
fn date_str(tdb_day: i64) -> String {
    let (y, m, d) = ymd(tdb_day);
    format!("{y:04}-{m:02}-{d:02}")
}
fn window_day(day: i64, year_of: &BTreeMap<i64, i32>, elong_of: &BTreeMap<i64, f64>) -> bool {
    match (year_of.get(&day), elong_of.get(&day)) {
        (Some(&y), Some(&el)) => y == STUDY_YEAR && el < CONJ_ELONG_DEG,
        _ => false,
    }
}
fn fmt_o(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.3}"),
        _ => "-".to_string(),
    }
}
fn bucket_tag(b: u8) -> &'static str {
    match b {
        0 => "floor",
        1 => "strong",
        _ => "trans",
    }
}

fn main() {
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in ["galileo_daily", "earth"] {
        if !load(b, &mut eph) {
            eprintln!("galileo: {b} ephemeris bin void");
        }
    }
    let geom_ok = eph.contains_key("galileo_daily") && eph.contains_key("earth");

    let Ok(bytes) = std::fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };

    let mut cells: BTreeMap<(i64, i64, i64, u8), Vec<f64>> = BTreeMap::new();
    let mut merged: BTreeMap<(i64, i64, i64), Vec<f64>> = BTreeMap::new();
    let mut locks: BTreeMap<i64, usize> = BTreeMap::new();
    let mut pad_zero = 0usize;
    for r in &recs {
        let mode = r[3] as i64;
        if !(1..=3).contains(&mode) {
            continue;
        }
        let resid = r[1];
        if resid.abs() > LOCK_HZ {
            *locks.entry(mode).or_insert(0) += 1;
            continue;
        }
        let day = (r[0] / DAY_S).floor() as i64;
        let st = r[2] as i64;
        let s = r[7];
        merged.entry((mode, day, st)).or_default().push(resid);
        if s == 0.0 {
            pad_zero += 1;
            continue;
        }
        let bucket = if s <= FLOOR_STRENGTH {
            0
        } else if s >= PLATEAU_STRENGTH {
            1
        } else {
            2
        };
        cells
            .entry((mode, day, st, bucket))
            .or_default()
            .push(resid);
    }
    drop(recs);
    drop(bytes);

    let day_set: BTreeSet<i64> = merged.keys().map(|k| k.1).collect();

    let mut year_of: BTreeMap<i64, i32> = BTreeMap::new();
    let mut elong_of: BTreeMap<i64, f64> = BTreeMap::new();
    for &day in &day_set {
        let (y, _, _) = ymd(day);
        year_of.insert(day, y);
        if geom_ok {
            let t = day as f64 * DAY_S;
            if let (Some(p), Some(e)) = (
                body_barycenter_position("galileo_daily", t, &eph),
                body_barycenter_position("earth", t, &eph),
            ) {
                let el = ang(sub([0.0, 0.0, 0.0], e), sub(p, e));
                if el.is_finite() {
                    elong_of.insert(day, el);
                }
            }
        }
    }

    let mut out: Vec<String> = Vec::new();
    out.push("galileo late-conjunction (1997, elong < 30 deg) station x strength split".to_string());
    out.push(format!(
        "binding: modes 1/2/3; lock transitions (|resid| > {:.0} Hz) excluded before noise; s = signal_strength slot 7",
        LOCK_HZ
    ));
    out.push(format!(
        "states: floor = s <= {FLOOR_STRENGTH:.0} (AGC clamp); strong plateau = s >= {PLATEAU_STRENGTH:.0}; s == 0 pad and in-between s excluded"
    ));
    out.push(format!(
        "noise cell = per (day, station, state), cell RMS about cell mean, cells >= {MIN_CELL} samples; median across day cells; window = calendar year {STUDY_YEAR} with elong < {CONJ_ELONG_DEG:.0} deg"
    ));
    out.push(format!(
        "stations {STATIONS:?}; total non-lock mode 1/2/3 samples {}, s == 0 pad samples excluded {}",
        merged.values().map(|v| v.len()).sum::<usize>(),
        pad_zero
    ));
    for m in 1..=3i64 {
        out.push(format!(
            "mode {m}: non-lock samples {}, lock transitions {}",
            merged
                .iter()
                .filter(|((mm, _, _), _)| *mm == m)
                .map(|(_, v)| v.len())
                .sum::<usize>(),
            locks.get(&m).copied().unwrap_or(0)
        ));
    }

    let mut by_mode: BTreeMap<i64, Vec<(i64, i64, u8, f64, usize)>> = BTreeMap::new();
    for ((m, day, st, bk), v) in &cells {
        if !STATIONS.contains(st)
            || v.len() < MIN_CELL
            || !window_day(*day, &year_of, &elong_of)
        {
            continue;
        }
        by_mode.entry(*m).or_default().push((*day, *st, *bk, rms(v), v.len()));
    }
    let mut all_by_mode: BTreeMap<i64, Vec<(i64, i64, f64, usize)>> = BTreeMap::new();
    for ((m, day, st), v) in &merged {
        if !STATIONS.contains(st)
            || v.len() < MIN_CELL
            || !window_day(*day, &year_of, &elong_of)
        {
            continue;
        }
        all_by_mode.entry(*m).or_default().push((*day, *st, rms(v), v.len()));
    }

    for m in 1..=3i64 {
        let wdays: BTreeSet<i64> = merged
            .keys()
            .filter(|(mm, day, st)| {
                *mm == m
                    && STATIONS.contains(st)
                    && window_day(*day, &year_of, &elong_of)
            })
            .map(|k| k.1)
            .collect();
        out.push(String::new());
        out.push(format!(
            "== mode {m}: 1997 quiet-conjunction window, {} day(s) at stations 14/43/63 with non-lock recording ==",
            wdays.len()
        ));
        if wdays.is_empty() {
            out.push("  no window days (0 honored)".to_string());
            continue;
        }
        let win_rows = by_mode.get(&m).cloned().unwrap_or_default();
        let win_all = all_by_mode.get(&m).cloned().unwrap_or_default();

        for st in STATIONS {
            let samp: usize = merged
                .iter()
                .filter(|((mm, day, ss), _)| {
                    *mm == m
                        && *ss == st
                        && window_day(*day, &year_of, &elong_of)
                })
                .map(|(_, v)| v.len())
                .sum();
            let mut parts = Vec::new();
            for bk in [0u8, 1] {
                let cell_rms: Vec<f64> = win_rows
                    .iter()
                    .filter(|r| r.1 == st && r.2 == bk)
                    .map(|r| r.3)
                    .collect();
                parts.push(format!(
                    "{} n_cells {} med {} Hz",
                    bucket_tag(bk),
                    cell_rms.len(),
                    fmt_o(median(&cell_rms))
                ));
            }
            let all_rms: Vec<f64> = win_all.iter().filter(|r| r.1 == st).map(|r| r.2).collect();
            if samp > 0 {
                out.push(format!(
                    "  station {st}: window samples {samp} | {} | all-states n_cells {} med {} Hz",
                    parts.join(" | "),
                    all_rms.len(),
                    fmt_o(median(&all_rms))
                ));
            }
        }

        for bk in [0u8, 1] {
            let cell_rms: Vec<f64> = win_rows.iter().filter(|r| r.2 == bk).map(|r| r.3).collect();
            out.push(format!(
                "  pooled {}: n_cells {} med {} Hz",
                bucket_tag(bk),
                cell_rms.len(),
                fmt_o(median(&cell_rms))
            ));
        }
        let all_rms: Vec<f64> = win_all.iter().map(|r| r.2).collect();
        out.push(format!(
            "  pooled all-states: n_cells {} med {} Hz",
            all_rms.len(),
            fmt_o(median(&all_rms))
        ));

        if m == 2 {
            out.push("  per-day detail (mode 2): date eps | st14 floor/strong | st43 floor/strong | st63 floor/strong as med_hz(n_cell)".to_string());
            for day in &wdays {
                let el = elong_of.get(day).copied().unwrap_or(f64::NAN);
                let mut daypart = Vec::new();
                for st in STATIONS {
                    let mut sp = Vec::new();
                    for bk in [0u8, 1] {
                        let hit: Vec<&(i64, i64, u8, f64, usize)> = win_rows
                            .iter()
                            .filter(|r| r.0 == *day && r.1 == st && r.2 == bk)
                            .collect();
                        if hit.is_empty() {
                            sp.push("-".to_string());
                        } else {
                            sp.push(format!("{:.3}({})", hit[0].3, hit[0].4));
                        }
                    }
                    daypart.push(sp.join("/"));
                }
                out.push(format!(
                    "    {} eps {el:4.1} | {} | {} | {}",
                    date_str(*day),
                    daypart[0],
                    daypart[1],
                    daypart[2]
                ));
            }
        }
    }

    println!("{}", out.join("\n"));
}
