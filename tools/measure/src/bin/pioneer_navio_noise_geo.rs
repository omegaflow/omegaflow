use std::collections::HashMap;

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};

const DAY_S: f64 = 86400.0;
const AU: f64 = 1.495978707e11;

fn load(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let p = format!("data/ssd.jpl.nasa.gov/ephemeris_{name}.bin");
    std::fs::read(&p)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| {
            eph.insert(name.to_string(), e);
        })
        .is_some()
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn run(probe: &str, sc_body: &str) {
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in [sc_body, "earth"] {
        if !load(b, &mut eph) {
            eprintln!("{probe}: {b} ephemeris bin void");
            return;
        }
    }
    let p = format!("data/spdf.gsfc.nasa.gov/{probe}_navio_residuum.bin");
    let Ok(bytes) = std::fs::read(&p) else {
        eprintln!("{probe}: residuum void");
        return;
    };
    let Some(recs) = omegaflow::odf::parse_p11r_bin(&bytes) else {
        eprintln!("{probe}: residuum parse void");
        return;
    };
    let mut day_noise: std::collections::BTreeMap<i64, Vec<f64>> =
        std::collections::BTreeMap::new();
    for r in &recs {
        if r[1].is_finite() {
            day_noise
                .entry((r[0] / DAY_S).floor() as i64)
                .or_default()
                .push(r[1]);
        }
    }
    let mut rows: Vec<(i64, f64, f64, f64, f64)> = Vec::new();
    for (day, vals) in &day_noise {
        if vals.len() < 30 {
            continue;
        }
        let t = *day as f64 * DAY_S;
        let (Some(p_pos), Some(e_pos)) = (
            body_barycenter_position(sc_body, t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) else {
            continue;
        };
        let sun = [0.0, 0.0, 0.0];
        let r_probe = norm(sub(p_pos, sun));
        let r_earth = norm(sub(e_pos, sun));
        let e_to_p = sub(p_pos, e_pos);
        let r_e_p = norm(e_to_p);
        let alpha_deg = (dot(sub(e_pos, sun), sub(p_pos, sun)) / (r_earth * r_probe).max(1e-30))
            .clamp(-1.0, 1.0)
            .acos()
            .to_degrees();
        let elong_deg = (dot(sub(sun, e_pos), e_to_p) / (r_earth * r_e_p).max(1e-30))
            .clamp(-1.0, 1.0)
            .acos()
            .to_degrees();
        let m = vals.iter().sum::<f64>() / vals.len() as f64;
        let rms = (vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64).sqrt();
        rows.push((*day, r_probe / AU, alpha_deg, elong_deg, rms));
    }
    rows.sort_by_key(|r| r.0);
    let mut alpha_bands: std::collections::BTreeMap<i64, Vec<f64>> =
        std::collections::BTreeMap::new();
    let mut elong_bands: std::collections::BTreeMap<i64, Vec<f64>> =
        std::collections::BTreeMap::new();
    let mut dist_bands: std::collections::BTreeMap<i64, Vec<f64>> =
        std::collections::BTreeMap::new();
    for (_, au, alpha, elong, rms) in &rows {
        alpha_bands
            .entry((*alpha as i64 / 10) * 10)
            .or_default()
            .push(*rms);
        elong_bands
            .entry((*elong as i64 / 10) * 10)
            .or_default()
            .push(*rms);
        dist_bands
            .entry((*au as i64 / 5) * 5)
            .or_default()
            .push(*rms);
    }
    eprintln!(
        "{probe}: {n} days. Mean per-day resid-RMS by alpha band (angle at Sun, deg):",
        n = rows.len()
    );
    for (band, v) in &alpha_bands {
        if v.len() < 10 {
            continue;
        }
        let med = {
            let mut s = v.clone();
            s.sort_by(f64::total_cmp);
            s[s.len() / 2]
        };
        eprintln!(
            "  alpha {band_lo}-{band_hi}+ deg: median resid-RMS {med:.0} Hz ({len} days)",
            band_lo = band,
            band_hi = band,
            len = v.len()
        );
    }
    eprintln!("{probe}: Mean per-day resid-RMS by solar elongation band (angle at Earth, deg):");
    for (band, v) in &elong_bands {
        if v.len() < 10 {
            continue;
        }
        let med = {
            let mut s = v.clone();
            s.sort_by(f64::total_cmp);
            s[s.len() / 2]
        };
        eprintln!(
            "  elong {band_lo}-{band_hi}+ deg: median resid-RMS {med:.0} Hz ({len} days)",
            band_lo = band,
            band_hi = band,
            len = v.len()
        );
    }
    eprintln!("{probe}: Mean per-day resid-RMS by heliocentric distance band (AU):");
    for (band, v) in &dist_bands {
        if v.len() < 10 {
            continue;
        }
        let med = {
            let mut s = v.clone();
            s.sort_by(f64::total_cmp);
            s[s.len() / 2]
        };
        eprintln!(
            "  dist {band_lo}-{band_hi}+ AU: median resid-RMS {med:.0} Hz ({len} days)",
            band_lo = band,
            band_hi = band,
            len = v.len()
        );
    }
    let far_med = {
        let mut s: Vec<f64> = dist_bands
            .values()
            .flat_map(|v| v.iter().copied())
            .collect();
        s.sort_by(f64::total_cmp);
        s[s.len() / 2]
    };
    eprintln!("{probe}: overall median resid-RMS {far_med:.0} Hz — the far-out (quiet, low-plasma) zone is the best floor for a drift-only measurement");
}

fn main() {
    for (probe, sc) in [
        ("pioneer10", "pioneer10_daily"),
        ("pioneer11", "pioneer11_daily"),
    ] {
        run(probe, sc);
    }
}
