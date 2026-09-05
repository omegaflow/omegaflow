use std::collections::{BTreeMap, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};

const DAY_S: f64 = 86400.0;
const AU: f64 = 1.495978707e11;
const LOCK_HZ: f64 = 1.0e3;

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

fn rms(vals: &[f64]) -> f64 {
    if vals.is_empty() {
        return f64::NAN;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    (vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64).sqrt()
}

fn median(vals: &[f64]) -> f64 {
    if vals.is_empty() {
        return f64::NAN;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
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
    let recs = omegaflow::atdf::parse_resid_bin(&bytes).unwrap_or_default();
    if recs.is_empty() {
        eprintln!("galileo: resid bin empty");
        return;
    }

    // resid slots: [0]=tdb [1]=resid_hz [2]=station [3]=mode [4]=dtype [5]=ref [6]=sampler [7]=strength
    let mut per: BTreeMap<(i64, i64), (Vec<f64>, usize, usize)> = BTreeMap::new();
    let mut per_station: BTreeMap<i64, (usize, usize)> = BTreeMap::new();
    let mut per_station_day: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    for r in &recs {
        let mode = r[3] as i64;
        let station = r[2] as i64;
        let day = (r[0] / DAY_S).floor() as i64;
        let resid = r[1];
        let e = per.entry((mode, day)).or_insert_with(|| (Vec::new(), 0, 0));
        e.2 += 1;
        let se = per_station.entry(station).or_insert((0, 0));
        se.0 += 1;
        if resid.abs() > LOCK_HZ {
            e.1 += 1;
            se.1 += 1;
        } else {
            e.0.push(resid);
            per_station_day
                .entry((station, day))
                .or_default()
                .push(resid);
        }
    }

    let mut dist_bands: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    let mut alpha_bands: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    let mut elong_bands: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    let mut mode_days: BTreeMap<i64, usize> = BTreeMap::new();
    let mut mode_nlock: BTreeMap<i64, usize> = BTreeMap::new();
    let mut mode_nsamp: BTreeMap<i64, usize> = BTreeMap::new();
    for ((mode, day), (vals, n_lock, n_total)) in &per {
        let t = *day as f64 * DAY_S;
        let (Some(p_pos), Some(e_pos)) = (
            body_barycenter_position("galileo_daily", t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) else {
            continue;
        };
        let sun = [0.0, 0.0, 0.0];
        let r_probe = norm(sub(p_pos, sun));
        let r_earth = norm(sub(e_pos, sun));
        let e_to_p = sub(p_pos, e_pos);
        let r_e_p = norm(e_to_p);
        // alpha = angle at the Sun between the Earth and probe vectors
        let alpha_deg = (dot(sub(e_pos, sun), sub(p_pos, sun)) / (r_earth * r_probe).max(1e-30))
            .clamp(-1.0, 1.0)
            .acos()
            .to_degrees();
        // epsilon = solar elongation = angle at the Earth between Sun and probe
        let elong_deg = (dot(sub(sun, e_pos), e_to_p) / (r_earth * r_e_p).max(1e-30))
            .clamp(-1.0, 1.0)
            .acos()
            .to_degrees();
        let au = r_probe / AU;
        let r = rms(vals);
        let db = au.floor() as i64;
        let ab = (alpha_deg / 30.0).floor() as i64;
        let eb = (elong_deg / 30.0).floor() as i64;
        dist_bands.entry((*mode, db)).or_default().push(r);
        alpha_bands.entry((*mode, ab)).or_default().push(r);
        elong_bands.entry((*mode, eb)).or_default().push(r);
        *mode_days.entry(*mode).or_default() += 1;
        *mode_nlock.entry(*mode).or_default() += n_lock;
        *mode_nsamp.entry(*mode).or_default() += n_total;
    }

    eprintln!(
        "galileo: {} resid samples, {} (mode, day) cells",
        recs.len(),
        per.len()
    );
    for mode in mode_days.keys() {
        eprintln!(
            "  mode {mode}: {} days, {} samples, {n_lock} lock transitions (|resid| > {LOCK_HZ:.0} Hz)",
            mode_days.get(mode).copied().unwrap_or(0),
            mode_nsamp.get(mode).copied().unwrap_or(0),
            n_lock = mode_nlock.get(mode).copied().unwrap_or(0),
        );
    }
    eprintln!("galileo: per-station (14/43/63 = die 20-s-Bande-Stationen):");
    for (station, (nsamp, nlock)) in &per_station {
        let day_rms: Vec<f64> = per_station_day
            .iter()
            .filter(|((st, _), _)| st == station)
            .map(|(_, v)| rms(v))
            .filter(|r| r.is_finite())
            .collect();
        eprintln!(
            "  station {station}: {nsamp} samples, {nlock} lock transitions, median per-day RMS {med:.1} Hz ({n} days)",
            med = median(&day_rms),
            n = day_rms.len(),
        );
    }
    eprintln!(
        "galileo: days per mode per heliocentric-distance band (AU) — n first, then the curve:"
    );
    for ((mode, band), v) in &dist_bands {
        eprintln!(
            "  mode {mode} dist {band}-{} AU: {n} days, median resid-RMS {med:.1} Hz",
            band + 1,
            n = v.len(),
            med = median(v),
        );
    }
    eprintln!(
        "galileo: per-day resid-RMS by alpha band (angle at Sun, deg), mode-split (n >= 10):"
    );
    for ((mode, band), v) in &alpha_bands {
        if v.len() < 10 {
            continue;
        }
        eprintln!(
            "  mode {mode} alpha {lo}-{hi} deg: median resid-RMS {med:.1} Hz ({n} days)",
            lo = band * 30,
            hi = (band + 1) * 30,
            med = median(v),
            n = v.len(),
        );
    }
    eprintln!("galileo: per-day resid-RMS by solar elongation band (angle at Earth, deg), mode-split (n >= 10):");
    for ((mode, band), v) in &elong_bands {
        if v.len() < 10 {
            continue;
        }
        eprintln!(
            "  mode {mode} elong {lo}-{hi} deg: median resid-RMS {med:.1} Hz ({n} days)",
            lo = band * 30,
            hi = (band + 1) * 30,
            med = median(v),
            n = v.len(),
        );
    }
}
