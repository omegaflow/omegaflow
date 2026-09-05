use std::collections::{BTreeMap, BTreeSet, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;

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

fn sorted_nan_last(vals: &[f64]) -> Vec<f64> {
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    s
}

fn median_upper(s: &[f64]) -> Option<f64> {
    if s.is_empty() {
        return None;
    }
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

    let mut day_total: BTreeMap<i64, usize> = BTreeMap::new();
    let mut day_nl: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for r in &recs {
        if r[3] as i64 != 2 {
            continue;
        }
        let day = (r[0] / DAY_S).floor() as i64;
        *day_total.entry(day).or_insert(0) += 1;
        if r[1].abs() <= LOCK_HZ {
            day_nl.entry(day).or_default().push(r[1]);
        }
    }
    drop(recs);

    let mut geo: BTreeMap<i64, (f64, f64)> = BTreeMap::new();
    let all_days: BTreeSet<i64> = day_total.keys().copied().collect();
    for d in &all_days {
        let t = *d as f64 * DAY_S;
        let (Some(p), Some(e)) = (
            body_barycenter_position("galileo_daily", t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) else {
            continue;
        };
        let alpha_deg = ang(p, e);
        let elong_deg = ang(sub([0.0, 0.0, 0.0], e), sub(p, e));
        if !alpha_deg.is_finite() || !elong_deg.is_finite() {
            continue;
        }
        geo.insert(*d, (alpha_deg, elong_deg));
    }
    drop(eph);

    let band = |el: f64| -> i64 { (el / 30.0).floor() as i64 };

    let quiet_any: BTreeSet<i64> = geo
        .iter()
        .filter(|(d, (_, el))| band(*el) == 0 && day_total.contains_key(d))
        .map(|(d, _)| *d)
        .collect();
    let quiet_real: BTreeSet<i64> = quiet_any.iter().filter(|d| day_nl.contains_key(d)).copied().collect();
    let all_lock: Vec<i64> = quiet_any.difference(&quiet_real).copied().collect();

    let band1_any: BTreeSet<i64> = geo
        .iter()
        .filter(|(d, (_, el))| band(*el) == 1 && day_total.contains_key(d))
        .map(|(d, _)| *d)
        .collect();
    let band1_real: BTreeSet<i64> = band1_any.iter().filter(|d| day_nl.contains_key(d)).copied().collect();

    let mut out: Vec<String> = Vec::new();
    out.push("galileo mode-2 quiet-window day-set reconciliation (1.5 vs 0.65 Hz)".to_string());
    out.push("binding: mode 2 records only; lock transitions |resid|>1000 Hz excluded from noise; day RMS about the day mean;".to_string());
    out.push("  epsilon = solar elongation at Earth at the TDB day start, band = floor(el/30); both probe conventions identical".to_string());
    out.push(format!("mode 2 days with any record: {}", day_total.len()));
    out.push(format!("  with >= 1 non-lock sample: {}", day_nl.len()));
    out.push(String::new());

    out.push("quiet window (elong band 0 = el < 30 deg)".to_string());
    out.push(format!("  days with any mode-2 record (the rausch/eps-curve count): {}", quiet_any.len()));
    out.push(format!("  days with >= 1 non-lock sample (the pass-seg / real-cell count): {}", quiet_real.len()));
    out.push(format!(
        "  all-lock days (any record, 0 non-lock samples): {}",
        quiet_any.len() - quiet_real.len()
    ));
    for d in &all_lock {
        let (al, el) = geo[d];
        out.push(format!(
            "    {}: elong {el:.1} deg, alpha {al:.1} deg, mode-2 records {}",
            jd_date(*d as f64 * DAY_S),
            day_total[d]
        ));
    }
    out.push(format!("  band1 (30 <= el < 60): days any record {}, days with non-lock {}", band1_any.len(), band1_real.len()));

    out.push(String::new());
    out.push("set arithmetic between the two probe conventions".to_string());
    let overlap_b1 = band1_any.intersection(&quiet_any).count();
    out.push(format!("  quiet_any (rausch) ∩ band1_any (30-60): {} days (the 8 loud days must be disjoint if both eps-curve and pass-seg used the same classification)", overlap_b1));
    out.push(format!("  quiet_any − quiet_real = {} all-lock days", quiet_any.difference(&quiet_real).count()));
    out.push(format!("  band1_real ∩ quiet_real: {} days", band1_real.intersection(&quiet_real).count()));

    let ref_list: Vec<f64> = quiet_any
        .iter()
        .map(|d| match day_nl.get(d) {
            Some(v) => rms(v),
            None => f64::NAN,
        })
        .collect();
    let ref_sorted = sorted_nan_last(&ref_list);
    out.push(String::new());
    out.push("reference reproduction (rausch noise_geo convention: all-lock day = NaN cell sorted last)".to_string());
    out.push(format!(
        "  n = {}, median index n/2 = {} -> {:.3} Hz",
        ref_sorted.len(),
        ref_sorted.len() / 2,
        ref_sorted[ref_sorted.len() / 2]
    ));

    let real: Vec<(i64, f64)> = quiet_real
        .iter()
        .filter_map(|d| {
            let v = day_nl.get(d)?;
            let r = rms(v);
            if r.is_finite() { Some((*d, r)) } else { None }
        })
        .collect();
    let mut real_sorted = real.clone();
    real_sorted.sort_by(|a, b| a.1.total_cmp(&b.1));
    let n = real_sorted.len();
    out.push(format!(
        "  real day cells (non-lock, the pass-seg/bound set): n = {}, median index n/2 = {} -> {:.3} Hz",
        n,
        n / 2,
        real_sorted[n / 2].1
    ));
    for idx in [35usize, 36, 37, 38, 39, 40].into_iter().filter(|i| *i < n) {
        let (d, r) = real_sorted[idx];
        let (al, el) = geo[&d];
        out.push(format!(
            "  sorted real index {idx}: {:.3} Hz  {}  elong {el:.1} alpha {al:.1} n_nonlock {}",
            r,
            jd_date(d as f64 * DAY_S),
            day_nl[&d].len()
        ));
    }

    out.push(String::new());
    out.push("the 75 quiet real day cells sorted by day RMS (index: date, elong, alpha, day_rms, n_nonlock, lobe)".to_string());
    let lobe_boundary = real_sorted[n / 2].1;
    for (idx, (d, r)) in real_sorted.iter().enumerate() {
        let (al, el) = geo[d];
        let mark = if *r > lobe_boundary { "UP" } else { "lo" };
        out.push(format!(
            "{idx:3} {} elong {el:6.2} alpha {al:6.2} rms {r:9.3} n {} {mark}",
            jd_date(*d as f64 * DAY_S),
            day_nl[d].len()
        ));
    }

    out.push(String::new());
    let n_lo = real_sorted.iter().filter(|(_, r)| *r <= lobe_boundary).count();
    let up: Vec<&(i64, f64)> = real_sorted.iter().filter(|(_, r)| *r > lobe_boundary).collect();
    out.push(format!("  bimodal split at the upper median {lobe_boundary:.3} Hz: {} cells in the quiet lobe, {} cells in the upper lobe", n_lo, up.len()));
    let up_max = up.last().unwrap().1;
    out.push(format!("  upper lobe range: {:.3} .. {:.3} Hz ({} days)", up.first().unwrap().1, up_max, up.len()));
    let loud1: Vec<&(i64, f64)> = real_sorted.iter().filter(|(_, r)| *r > 1.0).collect();
    out.push(format!("  loud days (day RMS > 1.0 Hz): {}", loud1.len()));
    for (d, r) in &loud1 {
        let (al, el) = geo[d];
        out.push(format!(
            "    {}: elong {el:.1} alpha {al:.1} rms {r:.3} Hz",
            jd_date(*d as f64 * DAY_S)
        ));
    }
    let loud_deep = loud1.iter().filter(|(d, _)| geo[d].1 < 10.0).count();
    out.push(format!("  of the loud (>1 Hz) days, those with elong < 10 deg: {} / {}", loud_deep, loud1.len()));

    out.push(String::new());
    out.push("band1 (30 <= el < 60) mode-2 days with any record, sorted by rms (the pass-seg 8-day 16.7 Hz window)".to_string());
    let mut b1: Vec<(i64, f64)> = band1_any
        .iter()
        .map(|d| {
            let r = match day_nl.get(d) {
                Some(v) => rms(v),
                None => f64::NAN,
            };
            (*d, r)
        })
        .collect();
    b1.sort_by(|a, b| a.1.total_cmp(&b.1));
    for (d, r) in &b1 {
        let (al, el) = geo[d];
        let nn = day_nl.get(d).map(|v| v.len()).unwrap_or(0);
        out.push(format!(
            "  {} elong {el:6.2} alpha {al:6.2} rms {r:9.3} n_nonlock {}",
            jd_date(*d as f64 * DAY_S),
            nn
        ));
    }
    let b1_real_rms: Vec<f64> = b1.iter().filter(|(_, r)| r.is_finite()).map(|(_, r)| *r).collect();
    if !b1_real_rms.is_empty() {
        let s = sorted_nan_last(&b1_real_rms);
        out.push(format!(
            "  band1 median (real cells, n = {}): {:.3} Hz",
            b1_real_rms.len(),
            median_upper(&s).unwrap_or(f64::NAN)
        ));
    }

    out.push(String::new());
    out.push("edge sensitivity: quiet days whose elong is within 1.0 deg of the 30-deg band edge".to_string());
    for (d, (al, el)) in &geo {
        if !quiet_any.contains(d) {
            continue;
        }
        if *el >= 29.0 && *el < 30.0 {
            let r = match day_nl.get(d) {
                Some(v) => rms(v),
                None => f64::NAN,
            };
            out.push(format!(
                "  {}: elong {el:.2} alpha {al:.1} rms {r:.3}",
                jd_date(*d as f64 * DAY_S)
            ));
        }
    }

    let body = out.join("\n") + "\n";
    println!("{body}");
}
