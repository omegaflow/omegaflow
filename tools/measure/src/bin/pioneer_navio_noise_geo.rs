use std::collections::HashMap;

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};

const DAY_S: f64 = 86400.0;
const AU: f64 = 1.495978707e11;

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
    // Per-day noise (residuum RMS) from the navio residuum
    let p = format!("data/{probe}_navio_residuum.bin");
    let Ok(bytes) = std::fs::read(&p) else {
        eprintln!("{probe}: residuum void");
        return;
    };
    let recs = omegaflow::odf::parse_p11r_bin(&bytes).unwrap_or_default();
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
    // Compute SEP + heliocentric distance per day, pair with noise
    let mut rows: Vec<(i64, f64, f64, f64)> = Vec::new();
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
        // SEP = angle at Sun between Earth-probe (r_earth) and probe (r_probe)
        let cos_sep = dot(sub(e_pos, sun), sub(p_pos, sun)) / (r_earth * r_probe).max(1e-30);
        let sep_deg = cos_sep.clamp(-1.0, 1.0).acos().to_degrees();
        // noise RMS
        let m = vals.iter().sum::<f64>() / vals.len() as f64;
        let rms = (vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64).sqrt();
        rows.push((*day, r_probe / AU, sep_deg, rms));
    }
    rows.sort_by_key(|r| r.0);
    // Average noise by SEP band and by heliocentric-distance band
    let mut sep_bands: std::collections::BTreeMap<i64, Vec<f64>> =
        std::collections::BTreeMap::new();
    let mut dist_bands: std::collections::BTreeMap<i64, Vec<f64>> =
        std::collections::BTreeMap::new();
    for (_, au, sep, rms) in &rows {
        sep_bands
            .entry((*sep as i64 / 10) * 10)
            .or_default()
            .push(*rms);
        dist_bands
            .entry((*au as i64 / 5) * 5)
            .or_default()
            .push(*rms);
    }
    eprintln!(
        "{probe}: {n} days. Mean per-day resid-RMS by SEP band (deg):",
        n = rows.len()
    );
    for (band, v) in &sep_bands {
        if v.len() < 10 {
            continue;
        }
        let med = {
            let mut s = v.clone();
            s.sort_by(f64::total_cmp);
            s[s.len() / 2]
        };
        eprintln!(
            "  SEP {band_lo}-{band_hi}+ deg: median resid-RMS {med:.0} Hz ({len} days)",
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
    // Payoff: emit the quiet-zone (far-out) days with their per-day medians so the drift
    // can be measured there alone. Print the median resid-RMS of the far-out half.
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
