use std::collections::{BTreeMap, BTreeSet, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CELL: usize = 30;
const CONJ_ELONG_DEG: f64 = 30.0;
const OUT: &str = "reports/galileo_mode2_strength_split.txt";

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
        .map(|e| {
            eph.insert(name.to_string(), e);
        })
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

fn value_at_fraction(counts: &BTreeMap<i64, usize>, total: usize, pct: usize) -> Option<i64> {
    let mut cum = 0usize;
    for (v, &c) in counts {
        cum += c;
        if cum * 100 >= total * pct {
            return Some(*v);
        }
    }
    None
}

fn quartile_boundaries(counts: &BTreeMap<i64, usize>) -> [Option<i64>; 3] {
    let total: usize = counts.values().sum();
    let mut out = [None; 3];
    if total == 0 {
        return out;
    }
    let mut cum = 0usize;
    for (v, &c) in counts {
        cum += c;
        for (k, slot) in out.iter_mut().enumerate() {
            if slot.is_none() && cum * 4 >= total * (k + 1) {
                *slot = Some(*v);
            }
        }
        if out.iter().all(|s| s.is_some()) {
            break;
        }
    }
    out
}

fn q_of(s: f64, b: &[Option<i64>; 3]) -> u8 {
    if s == 0.0 {
        return 0;
    }
    let sv = s as i64;
    let mut q = 4u8;
    for (k, cut) in b.iter().enumerate() {
        if let Some(c) = cut {
            if sv <= *c {
                q = (k + 1) as u8;
                break;
            }
        }
    }
    q
}

fn fmt_o(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.3}"),
        _ => "-".to_string(),
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
    drop(bytes);

    let mut days: Vec<i64> = Vec::new();
    let mut stas: Vec<i64> = Vec::new();
    let mut resids: Vec<f64> = Vec::new();
    let mut strengths: Vec<f64> = Vec::new();
    let mut station_hist: BTreeMap<i64, (usize, BTreeSet<i64>)> = BTreeMap::new();
    let mut n_mode2 = 0usize;
    let mut n_lock = 0usize;
    let mut n_zero = 0usize;
    for r in &recs {
        if r[3] as i64 != 2 {
            continue;
        }
        n_mode2 += 1;
        let resid = r[1];
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        let d = (r[0] / DAY_S).floor() as i64;
        let st = r[2] as i64;
        days.push(d);
        stas.push(st);
        resids.push(resid);
        let s = r[7];
        strengths.push(s);
        if s == 0.0 {
            n_zero += 1;
        }
        let e = station_hist.entry(st).or_default();
        e.0 += 1;
        e.1.insert(d);
    }
    drop(recs);
    let n = days.len();
    if n == 0 {
        eprintln!("galileo: no mode 2 samples");
        return;
    }

    let day_set: BTreeSet<i64> = days.iter().copied().collect();

    let mut out: Vec<String> = Vec::new();
    out.push("galileo mode-2 strength split — the two-way signal-strength fingerprint".to_string());
    out.push("binding: mode 2 (two-way) records only; lock transitions (|resid| > 1000 Hz) excluded before noise".to_string());
    out.push("noise cell = per (day) and per (day, station, strength-quartile); cell RMS about the cell mean; median across cells".to_string());
    out.push("strength bins: sample-level quartiles of signal_strength (slot 7, non-zero); strength == 0 carried separately".to_string());
    out.push(format!("cell minimum sample count: {MIN_CELL}"));
    out.push("geometry: elong = angle at Earth (Sun/probe), from galileo_daily + earth barycenters; conjunction = elong < 30 deg".to_string());

    out.push(String::new());
    out.push("overview".to_string());
    out.push(format!("  mode 2 samples incl lock: {n_mode2}"));
    out.push(format!("  mode 2 lock transitions: {n_lock}"));
    out.push(format!("  mode 2 samples after lock exclusion: {n}"));
    out.push(format!("  days after lock exclusion: {}", day_set.len()));
    let dl = day_set.iter().min().copied();
    let dh = day_set.iter().max().copied();
    match (dl, dh) {
        (Some(a), Some(b)) => out.push(format!(
            "  span: {} .. {}",
            jd_date(a as f64 * DAY_S),
            jd_date(b as f64 * DAY_S)
        )),
        _ => {}
    }
    let per_day: Vec<f64> = {
        let mut m: BTreeMap<i64, usize> = BTreeMap::new();
        for d in &days {
            *m.entry(*d).or_insert(0) += 1;
        }
        m.values().map(|c| *c as f64).collect()
    };
    out.push(format!(
        "  samples per day: median {}, min {}, max {}",
        fmt_o(median(&per_day)),
        per_day.iter().cloned().fold(f64::INFINITY, f64::min),
        per_day.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    ));
    for (s, (c, ds)) in &station_hist {
        out.push(format!("  station {s}: {c} samples, {} days", ds.len()));
    }

    let mut counts: BTreeMap<i64, usize> = BTreeMap::new();
    for s in strengths.iter().filter(|s| **s != 0.0) {
        *counts.entry(*s as i64).or_insert(0) += 1;
    }
    let nonzero: usize = counts.values().sum();
    let bounds = quartile_boundaries(&counts);
    let cut = |k: usize| -> String {
        match bounds[k] {
            Some(v) => v.to_string(),
            None => "-".to_string(),
        }
    };
    out.push(String::new());
    out.push("strength scale (signal_strength, slot 7)".to_string());
    out.push(format!(
        "  non-zero samples: {nonzero}, strength == 0 samples: {n_zero}, unique non-zero values: {}",
        counts.len()
    ));
    let mut dec = String::from("  sample-strength percentiles: ");
    for (i, pct) in [10usize, 20, 30, 40, 50, 60, 70, 80, 90].iter().enumerate() {
        dec.push_str(&format!(
            "p{}={}, ",
            (i + 1) * 10,
            value_at_fraction(&counts, nonzero, *pct)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string())
        ));
    }
    out.push(dec.trim_end_matches(", ").to_string());
    out.push(format!(
        "  quartile cuts: b1 = {}, b2 = {}, b3 = {}",
        cut(0),
        cut(1),
        cut(2)
    ));
    let minv = counts.iter().min_by_key(|(v, _)| **v).map(|(v, c)| (*v, *c));
    let maxc = counts.iter().max_by_key(|(_, c)| **c).map(|(v, c)| (*v, *c));
    match minv {
        Some((v, c)) => out.push(format!(
            "  weakest non-zero strength: {v} ({c} samples, {:.2} % of non-zero)",
            100.0 * c as f64 / nonzero as f64
        )),
        None => {}
    }
    match maxc {
        Some((v, c)) => out.push(format!(
            "  most frequent non-zero strength: {v} ({c} samples, {:.2} % of non-zero)",
            100.0 * c as f64 / nonzero as f64
        )),
        None => {}
    }

    let mut agg_day: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    let mut agg_q: BTreeMap<(i64, u8), Vec<f64>> = BTreeMap::new();
    let mut agg_stq: BTreeMap<(i64, i64, u8), Vec<f64>> = BTreeMap::new();
    let mut agg_stday: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    let mut str_q: Vec<Vec<f64>> = vec![Vec::new(); 5];
    for i in 0..n {
        let d = days[i];
        let q = q_of(strengths[i], &bounds);
        agg_day.entry(d).or_default().push(resids[i]);
        agg_q.entry((d, q)).or_default().push(resids[i]);
        agg_stq.entry((d, stas[i], q)).or_default().push(resids[i]);
        agg_stday.entry((d, stas[i])).or_default().push(resids[i]);
        str_q[q as usize].push(strengths[i]);
    }
    drop(days);
    drop(stas);
    drop(resids);
    drop(strengths);

    let mut elong_of: BTreeMap<i64, f64> = BTreeMap::new();
    let mut no_geom = 0usize;
    if geom_ok {
        for d in &day_set {
            let t = *d as f64 * DAY_S;
            match (
                body_barycenter_position("galileo_daily", t, &eph),
                body_barycenter_position("earth", t, &eph),
            ) {
                (Some(p), Some(e)) => {
                    let el = ang(sub([0.0, 0.0, 0.0], e), sub(p, e));
                    if el.is_finite() {
                        elong_of.insert(*d, el);
                    } else {
                        no_geom += 1;
                    }
                }
                _ => no_geom += 1,
            }
        }
    } else {
        no_geom = day_set.len();
    }
    let conj_days: BTreeSet<i64> = elong_of
        .iter()
        .filter(|(_, el)| **el < CONJ_ELONG_DEG)
        .map(|(d, _)| *d)
        .collect();
    out.push(String::new());
    out.push(format!(
        "geometry: {} days resolved, {} without ephemeris geometry",
        elong_of.len(),
        no_geom
    ));
    out.push(format!(
        "  conjunction (elong < 30 deg): {} of the {} resolved days carry non-lock mode 2 samples",
        conj_days.len(),
        elong_of.len()
    ));

    let daycells: Vec<(i64, f64, usize)> = agg_day
        .iter()
        .filter(|(_, v)| v.len() >= MIN_CELL)
        .map(|(d, v)| (*d, rms(v), v.len()))
        .filter(|c| c.1.is_finite())
        .collect();
    let qcells: Vec<(i64, u8, f64, usize)> = agg_q
        .iter()
        .filter(|(_, v)| v.len() >= MIN_CELL)
        .map(|((d, q), v)| (*d, *q, rms(v), v.len()))
        .filter(|c| c.2.is_finite())
        .collect();
    let stqcells: Vec<(i64, i64, u8, f64, usize)> = agg_stq
        .iter()
        .filter(|(_, v)| v.len() >= MIN_CELL)
        .map(|((d, s, q), v)| (*d, *s, *q, rms(v), v.len()))
        .filter(|c| c.3.is_finite())
        .collect();
    let stdaycells: Vec<(i64, i64, f64, usize)> = agg_stday
        .iter()
        .filter(|(_, v)| v.len() >= MIN_CELL)
        .map(|((d, s), v)| (*d, *s, rms(v), v.len()))
        .filter(|c| c.2.is_finite())
        .collect();

    let q_label = |q: u8| -> String {
        if q == 0 {
            "s=0".to_string()
        } else {
            format!("Q{q}")
        }
    };

    out.push(String::new());
    out.push("A. resid noise vs strength bin (mode 2, median per-day RMS, day cells merged over stations)".to_string());
    out.push("   col: samples (samples in bin), days (distinct days), cells (day cells >= MIN_CELL), med_day_rms, med_strength, conj_cells, conj_med (cells whose day has elong < 30)".to_string());
    for q in 0..=4u8 {
        let samples_q: usize = agg_q
            .iter()
            .filter(|((_, qq), _)| *qq == q)
            .map(|(_, v)| v.len())
            .sum();
        let days_q: usize = agg_q
            .keys()
            .filter(|(_, qq)| *qq == q)
            .map(|(d, _)| *d)
            .collect::<BTreeSet<i64>>()
            .len();
        let rms_list: Vec<f64> = qcells.iter().filter(|c| c.1 == q).map(|c| c.2).collect();
        let conj_list: Vec<f64> = qcells
            .iter()
            .filter(|c| c.1 == q && conj_days.contains(&c.0))
            .map(|c| c.2)
            .collect();
        let str_list = str_q[q as usize].clone();
        out.push(format!(
            "   {}  samples {}  days {}  cells {}  med_day_rms {} Hz  med_strength {}  conj_cells {}  conj_med {} Hz",
            q_label(q),
            samples_q,
            days_q,
            rms_list.len(),
            fmt_o(median(&rms_list)),
            fmt_o(median(&str_list)),
            conj_list.len(),
            fmt_o(median(&conj_list))
        ));
    }
    {
        let rms_all: Vec<f64> = daycells.iter().map(|c| c.1).collect();
        let conj_all: Vec<f64> = daycells
            .iter()
            .filter(|c| conj_days.contains(&c.0))
            .map(|c| c.1)
            .collect();
        out.push(format!(
            "   all (all bins merged)  cells {}  days {}  med_day_rms {} Hz  conj_cells {}  conj_med {} Hz",
            daycells.len(),
            daycells
                .iter()
                .map(|c| c.0)
                .collect::<BTreeSet<i64>>()
                .len(),
            fmt_o(median(&rms_all)),
            conj_all.len(),
            fmt_o(median(&conj_all))
        ));
    }

    out.push(String::new());
    out.push(format!("B. station split, cells (day, station, q) >= {MIN_CELL} samples"));
    out.push("   row per station per bin: cells, samples, days, med cell RMS; conj subset = cells whose day has elong < 30".to_string());
    let stations: BTreeSet<i64> = agg_stq.keys().map(|k| k.1).collect();
    for st in stations {
        for q in 0..=4u8 {
            let cells: Vec<&(i64, i64, u8, f64, usize)> =
                stqcells.iter().filter(|c| c.1 == st && c.2 == q).collect();
            let cell_n: Vec<f64> = cells.iter().map(|c| c.4 as f64).collect();
            let rms_list: Vec<f64> = cells.iter().map(|c| c.3).collect();
            let days_seen: usize = cells
                .iter()
                .map(|c| c.0)
                .collect::<BTreeSet<i64>>()
                .len();
            let conj: Vec<f64> = cells
                .iter()
                .filter(|c| conj_days.contains(&c.0))
                .map(|c| c.3)
                .collect();
            out.push(format!(
                "   station {st} {}  cells {}  samples {}  days {}  med {} Hz (cell n med/min/max {}/{}/{})  conj_cells {}  conj_med {} Hz",
                q_label(q),
                cells.len(),
                cells.iter().map(|c| c.4).sum::<usize>(),
                days_seen,
                fmt_o(median(&rms_list)),
                fmt_o(median(&cell_n)),
                cell_n.iter().cloned().fold(f64::INFINITY, f64::min),
                cell_n.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                conj.len(),
                fmt_o(median(&conj))
            ));
        }
        let cells: Vec<&(i64, i64, f64, usize)> =
            stdaycells.iter().filter(|c| c.1 == st).collect();
        let rms_list: Vec<f64> = cells.iter().map(|c| c.2).collect();
        let conj: Vec<f64> = cells
            .iter()
            .filter(|c| conj_days.contains(&c.0))
            .map(|c| c.2)
            .collect();
        out.push(format!(
            "   station {st} all bins (day,station cells)  cells {}  samples {}  med {} Hz  conj_cells {}  conj_med {} Hz",
            cells.len(),
            cells.iter().map(|c| c.3).sum::<usize>(),
            fmt_o(median(&rms_list)),
            conj.len(),
            fmt_o(median(&conj))
        ));
    }

    std::fs::create_dir_all("reports").ok();
    let body = out.join("\n") + "\n";
    match std::fs::write(OUT, &body) {
        Ok(()) => eprintln!("galileo: mode 2 strength split report written to {OUT}"),
        Err(_) => eprintln!("galileo: write {OUT} void"),
    }
}
