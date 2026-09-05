use std::collections::{BTreeMap, BTreeSet, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CELL: usize = 30;
const AU_M: f64 = 1.495978707e11;
const NEAR_ALPHA_DEG: f64 = 60.0;
const ANTI_ALPHA_DEG: f64 = 120.0;
const OUT: &str = "reports/galileo_mode1_strength_split.txt";

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
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

    let mut days: Vec<i32> = Vec::new();
    let mut stas: Vec<i32> = Vec::new();
    let mut resids: Vec<f64> = Vec::new();
    let mut strengths: Vec<f64> = Vec::new();
    let mut n_mode1 = 0usize;
    let mut n_lock = 0usize;
    let mut n_zero = 0usize;
    for r in &recs {
        if r[3] as i64 != 1 {
            continue;
        }
        n_mode1 += 1;
        let resid = r[1];
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        let st = r[7];
        days.push((r[0] / DAY_S).floor() as i32);
        stas.push(r[2] as i32);
        resids.push(resid);
        strengths.push(st);
        if st == 0.0 {
            n_zero += 1;
        }
    }
    drop(recs);
    let n = days.len();
    if n == 0 {
        eprintln!("galileo: no mode 1 samples");
        return;
    }

    let mut out: Vec<String> = Vec::new();
    out.push("galileo mode-1 strength split — the signal-strength fingerprint".to_string());
    out.push(
        "binding: mode 1 (one-way), lock transitions (|resid| > 1000 Hz) excluded before noise"
            .to_string(),
    );
    out.push(
        "noise metric: per-day RMS (root-mean-square about the cell mean), median across day cells"
            .to_string(),
    );
    out.push("strength bins: sample-level quartiles of signal_strength (slot 7, non-zero); strength == 0 carried separately".to_string());
    out.push(format!("day-cell minimum sample count: {MIN_CELL}"));
    out.push("geometry: Sun-Earth-probe angle (deg) and heliocentric distance (AU) per day from galileo_daily + earth ephemerides".to_string());

    let day_set: BTreeSet<i32> = days.iter().copied().collect();
    let day_first = days.iter().min().copied();
    let day_last = days.iter().max().copied();
    out.push(String::new());
    out.push("overview".to_string());
    out.push(format!("  mode 1 samples incl lock: {n_mode1}"));
    out.push(format!("  mode 1 lock transitions: {n_lock}"));
    out.push(format!("  mode 1 samples after lock exclusion: {n}"));
    out.push(format!("  days after lock exclusion: {}", day_set.len()));
    match (day_first, day_last) {
        (Some(a), Some(b)) => out.push(format!(
            "  span: {} .. {}",
            jd_date(a as f64 * DAY_S),
            jd_date(b as f64 * DAY_S)
        )),
        _ => {}
    }
    let per_day: Vec<f64> = {
        let mut m: BTreeMap<i32, usize> = BTreeMap::new();
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

    let mut station_hist: BTreeMap<i32, usize> = BTreeMap::new();
    let mut station_days: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
    for (i, s) in stas.iter().enumerate() {
        *station_hist.entry(*s).or_insert(0) += 1;
        station_days.entry(*s).or_default().insert(days[i]);
    }
    let mut st_line = String::from("  samples by station: ");
    for (s, c) in &station_hist {
        st_line.push_str(&format!(
            "{s} {c} ({} d), ",
            station_days.get(s).map(|x| x.len()).unwrap_or(0)
        ));
    }
    out.push(st_line.trim_end_matches(", ").to_string());

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

    let mut alpha_of: BTreeMap<i32, (f64, f64)> = BTreeMap::new();
    let mut no_geom: BTreeSet<i32> = BTreeSet::new();
    for d in &day_set {
        let t = *d as f64 * DAY_S;
        match (
            body_barycenter_position("galileo_daily", t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) {
            (Some(p), Some(e)) => {
                let rp = norm(p);
                let re = norm(e);
                let cos_alpha = dot(p, e) / (rp * re).max(1e-30);
                let alpha_deg = cos_alpha.clamp(-1.0, 1.0).acos().to_degrees();
                alpha_of.insert(*d, (alpha_deg, rp / AU_M));
            }
            _ => {
                no_geom.insert(*d);
            }
        }
    }
    out.push(format!(
        "  days without ephemeris geometry: {}",
        no_geom.len()
    ));

    let near_days: BTreeSet<i32> = alpha_of
        .iter()
        .filter(|(_, (s, _))| *s < NEAR_ALPHA_DEG)
        .map(|(d, _)| *d)
        .collect();
    let anti_days: BTreeSet<i32> = alpha_of
        .iter()
        .filter(|(_, (s, _))| *s >= ANTI_ALPHA_DEG)
        .map(|(d, _)| *d)
        .collect();
    out.push(format!(
        "  days alpha < {NEAR_ALPHA_DEG:.0} deg (near, angle at Sun): {}; days alpha >= {ANTI_ALPHA_DEG:.0} deg (anti): {}",
        near_days.len(),
        anti_days.len()
    ));

    let mut agg_day: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut agg_q: BTreeMap<(i32, u8), Vec<f64>> = BTreeMap::new();
    let mut agg_stq: BTreeMap<(i32, i32, u8), Vec<f64>> = BTreeMap::new();
    let mut str_q: Vec<Vec<f64>> = vec![Vec::new(); 5];
    let mut qc_near = [0usize; 5];
    let mut qc_anti = [0usize; 5];
    let mut near_day_str: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    for i in 0..n {
        let d = days[i];
        let st = strengths[i];
        let q = q_of(st, &bounds);
        agg_day.entry(d).or_default().push(resids[i]);
        agg_q.entry((d, q)).or_default().push(resids[i]);
        agg_stq.entry((d, stas[i], q)).or_default().push(resids[i]);
        str_q[q as usize].push(st);
        if near_days.contains(&d) {
            qc_near[q as usize] += 1;
            near_day_str.entry(d).or_default().push(st);
        } else if anti_days.contains(&d) {
            qc_anti[q as usize] += 1;
        }
    }
    drop(days);
    drop(stas);
    drop(resids);
    drop(strengths);

    let daycells: Vec<(i32, f64, usize)> = agg_day
        .iter()
        .filter(|(_, v)| v.len() >= MIN_CELL)
        .map(|(d, v)| (*d, rms(v), v.len()))
        .filter(|c| c.1.is_finite())
        .collect();
    let qcells: Vec<(i32, u8, f64, usize)> = agg_q
        .iter()
        .filter(|(_, v)| v.len() >= MIN_CELL)
        .map(|((d, q), v)| (*d, *q, rms(v), v.len()))
        .filter(|c| c.2.is_finite())
        .collect();
    let stqcells: Vec<(i32, i32, u8, f64, usize)> = agg_stq
        .iter()
        .filter(|(_, v)| v.len() >= MIN_CELL)
        .map(|((d, s, q), v)| (*d, *s, *q, rms(v), v.len()))
        .filter(|c| c.3.is_finite())
        .collect();

    let q_label = |q: u8| -> String {
        if q == 0 {
            "s=0".to_string()
        } else {
            format!("Q{q}")
        }
    };

    out.push(String::new());
    out.push("A. resid noise vs strength bin (mode 1, median per-day RMS)".to_string());
    out.push(format!(
        "   col: samples (samples in bin), days (distinct days carrying the bin), cells (day cells >= {MIN_CELL} samples), med_day_rms"
    ));
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
            .collect::<BTreeSet<i32>>()
            .len();
        let rms_list: Vec<f64> = qcells.iter().filter(|c| c.1 == q).map(|c| c.2).collect();
        let str_list = str_q[q as usize].clone();
        out.push(format!(
            "   {}  samples {}  days {}  cells {}  med_day_rms {} Hz  med_strength {}",
            q_label(q),
            samples_q,
            days_q,
            rms_list.len(),
            fmt_o(median(&rms_list)),
            fmt_o(median(&str_list))
        ));
    }
    {
        let rms_all: Vec<f64> = daycells.iter().map(|c| c.1).collect();
        out.push(format!(
            "   all (all bins merged)  cells {}  days {}  med_day_rms {} Hz",
            daycells.len(),
            daycells
                .iter()
                .map(|c| c.0)
                .collect::<BTreeSet<i32>>()
                .len(),
            fmt_o(median(&rms_all))
        ));
    }

    out.push(String::new());
    out.push("B. alpha split per strength bin (day cells >= MIN_CELL)".to_string());
    out.push(format!(
        "   near = alpha < {NEAR_ALPHA_DEG:.0} deg, anti = alpha >= {ANTI_ALPHA_DEG:.0} deg; ratio = near med / anti med"
    ));
    for q in 0..=4u8 {
        let mut near_rms: Vec<f64> = Vec::new();
        let mut anti_rms: Vec<f64> = Vec::new();
        let mut near_days_seen: BTreeSet<i32> = BTreeSet::new();
        let mut anti_days_seen: BTreeSet<i32> = BTreeSet::new();
        for c in qcells.iter().filter(|c| c.1 == q) {
            match alpha_of.get(&c.0) {
                Some((s, _)) if *s < NEAR_ALPHA_DEG => {
                    near_rms.push(c.2);
                    near_days_seen.insert(c.0);
                }
                Some((s, _)) if *s >= ANTI_ALPHA_DEG => {
                    anti_rms.push(c.2);
                    anti_days_seen.insert(c.0);
                }
                _ => {}
            }
        }
        let nm = median(&near_rms);
        let am = median(&anti_rms);
        let ratio = match (nm, am) {
            (Some(a), Some(b)) if b > 0.0 => format!("{:.2}", a / b),
            _ => "-".to_string(),
        };
        out.push(format!(
            "   {}  near {} Hz ({} cells, {} days)  anti {} Hz ({} cells, {} days)  ratio {}",
            q_label(q),
            fmt_o(nm),
            near_rms.len(),
            near_days_seen.len(),
            fmt_o(am),
            anti_rms.len(),
            anti_days_seen.len(),
            ratio
        ));
    }
    {
        out.push("   Rausch-Kurve reproduction (all strengths, per-day RMS):".to_string());
        for (lo, hi, tag) in [
            (0.0f64, 30.0f64, "alpha 0-30"),
            (0.0f64, NEAR_ALPHA_DEG, "alpha 0-60"),
            (ANTI_ALPHA_DEG, 180.0f64, "alpha 120-180"),
            (150.0f64, 180.0f64, "alpha 150-180"),
        ] {
            let band: Vec<f64> = daycells
                .iter()
                .filter(|c| match alpha_of.get(&c.0) {
                    Some((s, _)) => *s >= lo && *s <= hi,
                    None => false,
                })
                .map(|c| c.1)
                .collect();
            let days_b: usize = daycells
                .iter()
                .filter(|c| match alpha_of.get(&c.0) {
                    Some((s, _)) => *s >= lo && *s <= hi,
                    None => false,
                })
                .map(|c| c.0)
                .collect::<BTreeSet<i32>>()
                .len();
            out.push(format!(
                "   {tag}: med_day_rms {} Hz ({} cells, {} days)",
                fmt_o(median(&band)),
                band.len(),
                days_b
            ));
        }
    }

    out.push(String::new());
    out.push("C. station split 14/43/63 (cells (day, station, q) >= MIN_CELL)".to_string());
    for st in [14i32, 43, 63] {
        if !station_hist.contains_key(&st) {
            continue;
        }
        for q in 0..=4u8 {
            let list: Vec<f64> = stqcells
                .iter()
                .filter(|c| c.1 == st && c.2 == q)
                .map(|c| c.3)
                .collect();
            let cells: Vec<usize> = stqcells
                .iter()
                .filter(|c| c.1 == st && c.2 == q)
                .map(|c| c.4)
                .collect();
            out.push(format!(
                "   station {st} {}  cells {}  samples {}  med_day_rms {} Hz",
                q_label(q),
                cells.len(),
                cells.iter().sum::<usize>(),
                fmt_o(median(&list))
            ));
        }
    }

    out.push(String::new());
    out.push("D. low-alpha day ledger (alpha < 60 deg at Sun, mode 1, all strengths)".to_string());
    for d in near_days.iter() {
        let Some((sep, au)) = alpha_of.get(d) else {
            continue;
        };
        let Some(v) = agg_day.get(d) else {
            continue;
        };
        if v.len() < MIN_CELL {
            out.push(format!(
                "   {}  sep {:.1} deg  dist {:.2} AU  samples {} (< {MIN_CELL})  day_rms {} Hz  med_str {}",
                jd_date(*d as f64 * DAY_S),
                sep,
                au,
                v.len(),
                fmt_o(Some(rms(v))),
                fmt_o(median(near_day_str.get(d).map(|x| x.as_slice()).unwrap_or(&[])))
            ));
        } else {
            out.push(format!(
                "   {}  sep {:.1} deg  dist {:.2} AU  samples {}  day_rms {} Hz  med_str {}",
                jd_date(*d as f64 * DAY_S),
                sep,
                au,
                v.len(),
                fmt_o(Some(rms(v))),
                fmt_o(median(
                    near_day_str.get(d).map(|x| x.as_slice()).unwrap_or(&[])
                ))
            ));
        }
    }

    out.push(String::new());
    out.push("E. near vs anti strength composition (non-lock mode 1 samples)".to_string());
    let fmt_qc = |qc: &[usize; 5]| -> String {
        let mut s = String::from("[");
        for (q, c) in qc.iter().enumerate() {
            s.push_str(&format!("{}:{}, ", q_label(q as u8), c));
        }
        s.trim_end_matches(", ").to_string() + "]"
    };
    out.push(format!(
        "   near (alpha < 60): samples per bin {}",
        fmt_qc(&qc_near)
    ));
    out.push(format!(
        "   anti (alpha >= 120): samples per bin {}",
        fmt_qc(&qc_anti)
    ));

    std::fs::create_dir_all("reports").ok();
    let body = out.join("\n") + "\n";
    match std::fs::write(OUT, &body) {
        Ok(()) => {
            eprintln!("galileo: mode 1 strength split report written to {OUT}");
        }
        Err(_) => {
            eprintln!("galileo: write {OUT} void");
        }
    }
}
