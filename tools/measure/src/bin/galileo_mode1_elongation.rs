use std::collections::{BTreeMap, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CELL: usize = 30;
const AU_M: f64 = 1.495978707e11;

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

    let mut day_resid: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut day_str: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    for r in &recs {
        if r[3] as i64 != 1 {
            continue;
        }
        if r[1].abs() > LOCK_HZ {
            continue;
        }
        let d = (r[0] / DAY_S).floor() as i32;
        day_resid.entry(d).or_default().push(r[1]);
        day_str.entry(d).or_default().push(r[7]);
    }
    drop(recs);

    let mut out: Vec<String> = Vec::new();
    out.push("galileo mode-1 solar-elongation split (correct solar axis)".to_string());
    out.push("alpha = angle at the Sun (the Rausch-Kurve 'SEP'); elong = angle at Earth (the solar elongation)".to_string());
    out.push("noise = pooled per-day RMS (Rausch-Kurve metric); day cell >= 30 samples".to_string());

    let band: Vec<(f64, f64, &str)> = vec![
        (0.0, 10.0, "elong 0-10"),
        (10.0, 30.0, "elong 10-30"),
        (30.0, 60.0, "elong 30-60"),
        (60.0, 120.0, "elong 60-120"),
        (120.0, 150.0, "elong 120-150"),
        (150.0, 180.0, "elong 150-180"),
    ];
    let mut counts = vec![Vec::new(); band.len()];
    let mut ledger: Vec<String> = Vec::new();
    let mut geok: BTreeMap<i32, (f64, f64, f64)> = BTreeMap::new();
    for (d, v) in &day_resid {
        if v.len() < MIN_CELL {
            continue;
        }
        let t = *d as f64 * DAY_S;
        let (Some(p), Some(e)) = (
            body_barycenter_position("galileo_daily", t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) else {
            continue;
        };
        let alpha = ang(p, e);
        let elong = ang(sub([0.0, 0.0, 0.0], e), sub(p, e));
        geok.insert(*d, (alpha, elong, norm(p) / AU_M));
        let rr = rms(v);
        if !rr.is_finite() {
            continue;
        }
        let mut bi = 0usize;
        for (i, (lo, hi, _)) in band.iter().enumerate() {
            if elong >= *lo && elong < *hi {
                bi = i;
            }
        }
        if elong >= 180.0 {
            bi = 5;
        }
        counts[bi].push(rr);
        if elong < 60.0 || rr > 15.0 {
            ledger.push(format!(
                "  {}  alpha {alpha:6.2}  elong {elong:6.2}  dist {:.3} AU  n {}  day_rms {} Hz  med_str {}",
                jd_date(*d as f64 * DAY_S),
                norm(p) / AU_M,
                v.len(),
                fmt_o(Some(rr)),
                fmt_o(median(day_str.get(d).map(|x| x.as_slice()).unwrap_or(&[])))
            ));
        }
    }
    for (i, (_lo, _hi, tag)) in band.iter().enumerate() {
        out.push(format!(
            "  {tag}: days {}  med pooled day RMS {} Hz",
            counts[i].len(),
            fmt_o(median(&counts[i]))
        ));
    }
    out.push(String::new());
    out.push("ledger (days with elong < 60 or day_rms > 15 Hz):".to_string());
    out.extend(ledger);
    out.push(String::new());
    out.push(format!(
        "  geometry days resolved: {} of {} day cells",
        geok.len(),
        day_resid.len()
    ));

    std::fs::create_dir_all("reports").ok();
    let body = out.join("\n") + "\n";
    match std::fs::write("reports/galileo_mode1_elongation.txt", &body) {
        Ok(()) => {
            eprintln!("galileo: mode 1 elongation report written");
        }
        Err(_) => {
            eprintln!("galileo: write void");
        }
    }
}
