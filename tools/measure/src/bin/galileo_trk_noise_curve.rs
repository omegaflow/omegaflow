use std::collections::{BTreeMap, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::atdf::parse_resid_bin;
use omegaflow::odf::parse_podf_bin;

const DAY_S: f64 = 86400.0;
const AU: f64 = 1.495978707e11;
const EPS_BAND_DEG: f64 = 30.0;
const MIN_DAYS: usize = 3;

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

fn rms(vals: &[f64]) -> f64 {
    if vals.is_empty() {
        return f64::NAN;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    (vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64).sqrt()
}

fn diff_rms(vals: &[f64]) -> f64 {
    if vals.len() < 2 {
        return f64::NAN;
    }
    let mut d = Vec::with_capacity(vals.len() - 1);
    for w in vals.windows(2) {
        d.push(w[1] - w[0]);
    }
    rms(&d)
}

fn median(vals: &[f64]) -> f64 {
    if vals.is_empty() {
        return f64::NAN;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}

fn geom(t: f64, eph: &HashMap<String, BodyEphemeris>) -> Option<(f64, f64, f64)> {
    let p_pos = body_barycenter_position("galileo_daily", t, eph)?;
    let e_pos = body_barycenter_position("earth", t, eph)?;
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
    Some((r_probe / AU, alpha_deg, elong_deg))
}

struct Bands {
    eps: BTreeMap<i64, Vec<f64>>,
    dist: BTreeMap<i64, Vec<f64>>,
}

impl Bands {
    fn new() -> Bands {
        Bands {
            eps: BTreeMap::new(),
            dist: BTreeMap::new(),
        }
    }
    fn push(&mut self, day: i64, per_day_rms: f64, eph: &HashMap<String, BodyEphemeris>) {
        let Some((au, _, elong)) = geom(day as f64 * DAY_S, eph) else {
            return;
        };
        if per_day_rms.is_finite() {
            self.eps
                .entry((elong / EPS_BAND_DEG).floor() as i64)
                .or_default()
                .push(per_day_rms);
            self.dist
                .entry(au.floor() as i64)
                .or_default()
                .push(per_day_rms);
        }
    }
    fn report(&self, label: &str) {
        for (band, vals) in &self.eps {
            if vals.len() < MIN_DAYS {
                continue;
            }
            println!(
                "  {label} elong {lo}-{hi} deg: median per-day RMS {med:.3} Hz ({n} days)",
                lo = band * 30,
                hi = (band + 1) * 30,
                med = median(vals),
                n = vals.len(),
            );
        }
        for (band, vals) in &self.dist {
            if vals.len() < MIN_DAYS {
                continue;
            }
            println!(
                "  {label} heliocentric {lo}-{hi} AU: median per-day RMS {med:.3} Hz ({n} days)",
                lo = band,
                hi = band + 1,
                med = median(vals),
                n = vals.len(),
            );
        }
    }
}

fn main() {
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in ["galileo_daily", "earth"] {
        if !load(b, &mut eph) {
            eprintln!("{b} ephemeris bin void");
            return;
        }
    }

    println!("galileo TRK-2-25/2-18 empirical noise curve");
    println!("binding: per-day RMS of the TRK-2-25 doppler residuum (Hz) and per-day first-difference RMS of the TRK-2-18 ODF observable (Hz), binned by solar elongation eps (angle at the Earth, deg) and heliocentric distance (AU)");

    let Ok(resid_bytes) = std::fs::read("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = parse_resid_bin(&resid_bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    let mut per: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    for r in &recs {
        let mode = r[3] as i64;
        let day = (r[0] / DAY_S).floor() as i64;
        per.entry((mode, day)).or_default().push(r[1]);
    }
    let mut modes: Vec<i64> = per.keys().map(|(m, _)| *m).collect();
    modes.sort_unstable();
    modes.dedup();
    println!(
        "== TRK-2-25 residuum == {n} samples, {days} (mode, day) cells",
        n = recs.len(),
        days = per.len()
    );
    for mode in modes {
        let mut bands = Bands::new();
        let mut nsamp = 0usize;
        for ((m, day), vals) in &per {
            if *m != mode {
                continue;
            }
            nsamp += vals.len();
            bands.push(*day, rms(vals), &eph);
        }
        println!("mode {mode} ({nsamp} samples):");
        bands.report(&format!("mode {mode}"));
    }

    let Ok(odf_bytes) = std::fs::read("data/pds-ppi.igpp.ucla.edu/galileo_odf.bin") else {
        println!("== TRK-2-18 ODF == asset absent (0 honored)");
        return;
    };
    let Some(orecs) = parse_podf_bin(&odf_bytes) else {
        println!("== TRK-2-18 ODF == asset parse void");
        return;
    };
    let mut oper: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    for r in &orecs {
        let dt = r[5] as i64;
        let day = (r[0] / DAY_S).floor() as i64;
        oper.entry((dt, day)).or_default().push(r[1]);
    }
    let mut dts: Vec<i64> = oper.keys().map(|(d, _)| *d).collect();
    dts.sort_unstable();
    dts.dedup();
    println!(
        "== TRK-2-18 ODF observable == {n} samples, {days} (data_type, day) cells",
        n = orecs.len(),
        days = oper.len()
    );
    for dt in dts {
        let mut bands = Bands::new();
        let mut nsamp = 0usize;
        for ((d, day), vals) in &oper {
            if *d != dt {
                continue;
            }
            nsamp += vals.len();
            bands.push(*day, diff_rms(vals), &eph);
        }
        println!("data_type {dt} ({nsamp} samples):");
        bands.report(&format!("data_type {dt}"));
    }
}
