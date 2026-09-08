use std::collections::HashMap;

use omegaflow::archivar::{
    body_barycenter_position, body_fixed_to_icrs, parse_ephemeris_binary, BodyEphemeris,
    J2000_EPOCH,
};

const LAT_DEG: f64 = -22.534444444;
const LON_DEG: f64 = -45.5825;
const ALT_M: f64 = 1810.7;
const SAMPLES: usize = 120;

fn load(path: &str) -> Option<BodyEphemeris> {
    let bytes = std::fs::read(path).ok()?;
    parse_ephemeris_binary(&bytes)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut bins: Vec<(String, String)> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--bin" => {
                let word = match args.get(i + 1) {
                    Some(w) => w.clone(),
                    None => String::new(),
                };
                let path = match args.get(i + 2) {
                    Some(p) => p.clone(),
                    None => String::new(),
                };
                if !word.is_empty() && !path.is_empty() {
                    bins.push((word, path));
                }
                i += 2;
            }
            _ => {}
        }
        i += 1;
    }
    if bins.is_empty() {
        bins.push((
            "de441".to_string(),
            "data/ssd.jpl.nasa.gov/ephemeris_earth.bin".to_string(),
        ));
        bins.push((
            "inpop19a".to_string(),
            "data/ftp.imcce.fr/ephemeris_inpop_earth.bin".to_string(),
        ));
        bins.push((
            "epm2021".to_string(),
            "data/ftp.iaaras.ru/ephemeris_epm_earth.bin".to_string(),
        ));
    }
    for (word, path) in &bins {
        let Some(eph) = load(path) else {
            println!("{word}: {} reads unparsed", path);
            continue;
        };
        let mut lo = f64::MAX;
        let mut hi = f64::MIN;
        for g in &eph.granules {
            let a = g.t0_jd - g.dt_jd;
            let b = g.t0_jd + g.dt_jd;
            if a < lo {
                lo = a;
            }
            if b > hi {
                hi = b;
            }
        }
        let props = eph.props.as_ref();
        match props {
            Some(p) => println!(
                "{word}: {} granules, {} rotation matrices, radius_m {:.1} m, flattening {:?}, radii_b {:?} m, radii_c {:?} m, nutation {} records",
                eph.granules.len(),
                eph.rotation_matrices.len(),
                p.radius_m,
                p.flattening,
                p.radii_b,
                p.radii_c,
                p.nutation.as_ref().map_or(0, |n| n.len()),
            ),
            None => println!(
                "{word}: {} granules, {} rotation matrices, no props",
                eph.granules.len(),
                eph.rotation_matrices.len()
            ),
        }
        if !lo.is_finite() || !hi.is_finite() || hi <= lo {
            println!("{word}: no granule coverage");
            continue;
        }
        let mut map: HashMap<String, BodyEphemeris> = HashMap::new();
        map.insert("earth".to_string(), eph);
        let mut own = 0usize;
        let mut absent = 0usize;
        let mut geo_absent = 0usize;
        for k in 0..SAMPLES {
            let frac = k as f64 / (SAMPLES - 1) as f64;
            let jd = lo + frac * (hi - lo);
            let tdb = (jd - J2000_EPOCH) * 86400.0;
            if body_barycenter_position("earth", tdb, &map).is_none() {
                geo_absent += 1;
                continue;
            }
            if body_fixed_to_icrs("earth", LAT_DEG, LON_DEG, ALT_M, tdb, &map).is_some() {
                own += 1;
            } else {
                absent += 1;
            }
        }
        println!(
            "{word}: {own}/{SAMPLES} body-fixed present, {absent} absent, {geo_absent} geocenter void over jd [{lo:.3}, {hi:.3}]",
        );
    }
}
