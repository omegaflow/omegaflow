#![allow(mixed_script_confusables)]

use std::collections::HashMap;
use std::path::Path;

use omegaflow::archivar::{
    body_barycenter_position, body_fixed_to_icrs, embedded_lsk, fetch_raw_bytes,
    icrs_to_body_surface, light_time_worldline, parse_ephemeris_binary, BodyEphemeris, J2000_EPOCH,
};
use omegaflow::cdn::CDN_BASE;
use omegaflow::odp::{dsn_station, EARTH};

const DAY_S: f64 = 86400.0;
const RAD_DEG: f64 = 180.0 / std::f64::consts::PI;
const EARTH_MEAN_RADIUS_M: f64 = 6371.0e3;
const BIN_TTL_S: u64 = 604800;
const SWEEP_SAMPLES: usize = 9;

const ANCHOR_UNIX: f64 = 1503273600.0 + 18.0 * 3600.0 + 26.0 * 60.0 + 40.0;

struct LineSpec {
    word: &'static str,
    netloc: &'static str,
    sun_asset: &'static str,
    earth_asset: &'static str,
    mars_asset: Option<&'static str>,
}

const LINES: [LineSpec; 5] = [
    LineSpec {
        word: "de441",
        netloc: "ssd.jpl.nasa.gov",
        sun_asset: "ephemeris_sun.bin",
        earth_asset: "ephemeris_earth.bin",
        mars_asset: Some("ephemeris_mars.bin"),
    },
    LineSpec {
        word: "de440",
        netloc: "ssd.jpl.nasa.gov",
        sun_asset: "ephemeris_de440_sun.bin",
        earth_asset: "ephemeris_de440_earth.bin",
        mars_asset: None,
    },
    LineSpec {
        word: "de442",
        netloc: "ssd.jpl.nasa.gov",
        sun_asset: "ephemeris_de442_sun.bin",
        earth_asset: "ephemeris_de442_earth.bin",
        mars_asset: None,
    },
    LineSpec {
        word: "inpop19a",
        netloc: "ftp.imcce.fr",
        sun_asset: "ephemeris_inpop_sun.bin",
        earth_asset: "ephemeris_inpop_earth.bin",
        mars_asset: None,
    },
    LineSpec {
        word: "epm2021",
        netloc: "ftp.iaaras.ru",
        sun_asset: "ephemeris_epm_sun.bin",
        earth_asset: "ephemeris_epm_earth.bin",
        mars_asset: None,
    },
];

struct Line {
    map: HashMap<String, BodyEphemeris>,
    map_analytic: HashMap<String, BodyEphemeris>,
    bodies: Vec<String>,
    earth_size: Option<u64>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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

fn bin_path(eph_dir: &str, netloc: &str, asset: &str) -> String {
    format!("{eph_dir}/{netloc}/{asset}")
}

fn ensure_bin(path: &str, netloc: &str, asset: &str, ttl: u64) -> Option<Vec<u8>> {
    if let Ok(bytes) = std::fs::read(path) {
        return Some(bytes);
    }
    if !path.starts_with("data/") {
        return None;
    }
    let url = format!("{CDN_BASE}/{netloc}/{asset}");
    let bytes = fetch_raw_bytes(&url, ttl)?;
    if let Some(parent) = Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, &bytes);
    Some(bytes)
}

fn load_body(eph_dir: &str, netloc: &str, asset: &str) -> Result<BodyEphemeris, String> {
    let path = bin_path(eph_dir, netloc, asset);
    let bytes = ensure_bin(&path, netloc, asset, BIN_TTL_S).ok_or_else(|| {
        format!("{path} void — absent on disk and the CDN fetch returned non-200")
    })?;
    parse_ephemeris_binary(&bytes)
        .ok_or_else(|| format!("{path} reads but does not parse to a BodyEphemeris"))
}

fn analytic_eph(eph: &BodyEphemeris) -> BodyEphemeris {
    let mut c = eph.clone();
    c.rotation_matrices = Vec::new();
    c
}

fn load_line(spec: &LineSpec, eph_dir: &str) -> Result<Line, String> {
    let earth = load_body(eph_dir, spec.netloc, spec.earth_asset)?;
    let sun = load_body(eph_dir, spec.netloc, spec.sun_asset)?;
    if earth.rotation_matrices.is_empty() || earth.props.is_none() {
        return Err(format!(
            "{} carries {} rotation matrices and props {} — the body is absent for the orientation measure",
            spec.earth_asset,
            earth.rotation_matrices.len(),
            if earth.props.is_some() {
                "present"
            } else {
                "absent"
            },
        ));
    }
    let earth_size = std::fs::metadata(bin_path(eph_dir, spec.netloc, spec.earth_asset))
        .ok()
        .map(|m| m.len());
    let mut map = HashMap::new();
    map.insert(EARTH.to_string(), earth);
    map.insert("sun".to_string(), sun.clone());
    let mut map_analytic = HashMap::new();
    map_analytic.insert(EARTH.to_string(), analytic_eph(&map[EARTH]));
    map_analytic.insert("sun".to_string(), sun);
    Ok(Line {
        map,
        map_analytic,
        bodies: vec![EARTH.to_string()],
        earth_size,
    })
}

fn subsolar(body: &str, t: f64, map: &HashMap<String, BodyEphemeris>) -> Option<(f64, f64)> {
    let center = body_barycenter_position(body, t, map)?;
    let sun = light_time_worldline(center, t, &|s| body_barycenter_position("sun", s, map))?.0;
    icrs_to_body_surface(sun[0], sun[1], sun[2], t, body, map)
}

fn arc_deg(a_lat: f64, a_lon: f64, b_lat: f64, b_lon: f64) -> f64 {
    let dlat = (a_lat - b_lat).to_radians();
    let dlon = (a_lon - b_lon).to_radians();
    let la = a_lat.to_radians();
    let lb = b_lat.to_radians();
    let h = (dlat / 2.0).sin() * (dlat / 2.0).sin()
        + la.cos() * lb.cos() * (dlon / 2.0).sin() * (dlon / 2.0).sin();
    2.0 * h.sqrt().min(1.0).asin() * RAD_DEG
}

fn arc_km(a_lat: f64, a_lon: f64, b_lat: f64, b_lon: f64) -> f64 {
    arc_deg(a_lat, a_lon, b_lat, b_lon) * std::f64::consts::PI * EARTH_MEAN_RADIUS_M
        / 180.0
        / 1000.0
}

fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}

fn granule_span(eph: &BodyEphemeris) -> Option<(f64, f64)> {
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
    if lo.is_finite() && hi.is_finite() && hi > lo {
        Some((lo, hi))
    } else {
        None
    }
}

fn elevation_via(station: [f64; 3], center: [f64; 3], sun: [f64; 3]) -> Option<f64> {
    let up = sub(station, center);
    let los = sub(sun, station);
    let nu = norm(up);
    let nl = norm(los);
    if nu > 0.0 && nl > 0.0 && nu.is_finite() && nl.is_finite() {
        Some(
            (dot(los, up) / (nu * nl))
                .clamp(-1.0, 1.0)
                .asin()
                .to_degrees(),
        )
    } else {
        None
    }
}

fn station_sun_elevation(
    body: &str,
    lat: f64,
    lon: f64,
    alt: f64,
    t: f64,
    map: &HashMap<String, BodyEphemeris>,
) -> Option<f64> {
    let station = body_fixed_to_icrs(body, lat, lon, alt, t, map)?;
    let center = body_barycenter_position(body, t, map)?;
    let sun = light_time_worldline(station, t, &|s| body_barycenter_position("sun", s, map))?.0;
    elevation_via(station, center, sun)
}

fn textbook_sun_elevation(
    body: &str,
    lat: f64,
    lon: f64,
    t: f64,
    map: &HashMap<String, BodyEphemeris>,
) -> Option<f64> {
    let center = body_barycenter_position(body, t, map)?;
    let sun = light_time_worldline(center, t, &|s| body_barycenter_position("sun", s, map))?.0;
    let v = sub(sun, center);
    let r = norm(v);
    if !(r > 0.0 && r.is_finite()) {
        return None;
    }
    let dec = (v[2] / r).clamp(-1.0, 1.0).asin().to_degrees();
    let mut ra = v[1].atan2(v[0]).to_degrees();
    if ra < 0.0 {
        ra += 360.0;
    }
    let jd = J2000_EPOCH + t / DAY_S;
    let gmst = (280.46061837 + 360.98564736629 * (jd - 2451545.0)).rem_euclid(360.0);
    let lst = (gmst + lon).rem_euclid(360.0);
    let mut ha = (lst - ra).rem_euclid(360.0);
    if ha > 180.0 {
        ha -= 360.0;
    }
    let sinel = lat.to_radians().sin() * dec.to_radians().sin()
        + lat.to_radians().cos() * dec.to_radians().cos() * ha.to_radians().cos();
    Some(sinel.clamp(-1.0, 1.0).asin().to_degrees())
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.3}"),
        None => "-".to_string(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let eph_dir = match arg_value(&args, "--eph-dir") {
        Some(d) => d,
        None => "data".to_string(),
    };
    let Some(lsk) = embedded_lsk() else {
        eprintln!("orientation: the embedded LSK carries no naif0012 table — the TDB axis stays unconverted");
        return;
    };
    let Some(t_anchor) = lsk.unix_to_tdb(ANCHOR_UNIX) else {
        eprintln!("orientation: the anchor date reads void on the leap table");
        return;
    };
    println!("=== orientation — sub-solar point on the matrix path against the analytic IAU path, real bins ===");
    println!("anchor 2017-08-21T18:26:40Z (tdb {t_anchor:.3} s past J2000)");

    for spec in &LINES {
        let mut line = match load_line(spec, &eph_dir) {
            Ok(l) => l,
            Err(e) => {
                println!("orientation {}: {e}", spec.word);
                continue;
            }
        };
        if let Some(ma) = spec.mars_asset {
            match load_body(&eph_dir, spec.netloc, ma) {
                Ok(mars) => {
                    if mars.rotation_matrices.is_empty() || mars.props.is_none() {
                        println!(
                            "orientation {}: {ma} carries no rotation matrices / props — mars absent",
                            spec.word
                        );
                    } else {
                        line.map.insert("mars".to_string(), mars.clone());
                        line.map_analytic
                            .insert("mars".to_string(), analytic_eph(&mars));
                        line.bodies.push("mars".to_string());
                    }
                }
                Err(e) => println!("orientation {}: {e}", spec.word),
            }
        }
        let earth = &line.map[EARTH];
        let (lo, hi) = match granule_span(earth) {
            Some(s) => s,
            None => {
                println!("orientation {}: no granule coverage", spec.word);
                continue;
            }
        };
        let size = match line.earth_size {
            Some(s) => s.to_string(),
            None => "size-absent".to_string(),
        };
        println!(
            "orientation {}: earth bin {} B, {} granules, {} rotation matrices, span jd [{lo:.3}..{hi:.3}]; bodies {}",
            spec.word,
            size,
            earth.granules.len(),
            earth.rotation_matrices.len(),
            line.bodies.join(" "),
        );
        for body in &line.bodies {
            let mut deltas: Vec<f64> = Vec::new();
            let mut absent = 0usize;
            for k in 0..SWEEP_SAMPLES {
                let frac = k as f64 / (SWEEP_SAMPLES - 1) as f64;
                let jd = lo + frac * (hi - lo);
                let t = (jd - J2000_EPOCH) * DAY_S;
                match (
                    subsolar(body, t, &line.map),
                    subsolar(body, t, &line.map_analytic),
                ) {
                    (Some((mlat, mlon)), Some((alat, alon))) => {
                        deltas.push(arc_km(mlat, mlon, alat, alon));
                    }
                    _ => absent += 1,
                }
            }
            let present = deltas.len();
            if present == 0 {
                println!(
                    "orientation {} {body}: sweep 0/{SWEEP_SAMPLES} present — the sub-solar stays absent over the span",
                    spec.word,
                );
            } else {
                let max = deltas.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let med = median(&deltas).unwrap();
                println!(
                    "orientation {} {body}: sweep {present}/{SWEEP_SAMPLES} present, delta max {max:.1} km median {med:.1} km ({absent} absent)",
                    spec.word,
                );
            }
            match (
                subsolar(body, t_anchor, &line.map),
                subsolar(body, t_anchor, &line.map_analytic),
            ) {
                (Some((mlat, mlon)), Some((alat, alon))) => println!(
                    "orientation {} {body} anchor: matrix ({mlat:.4} {mlon:.4}) analytic ({alat:.4} {alon:.4}) delta {:.1} km",
                    spec.word,
                    arc_km(mlat, mlon, alat, alon),
                ),
                _ => println!(
                    "orientation {} {body} anchor: absent at tdb {t_anchor:.3}",
                    spec.word,
                ),
            }
        }
        if line.bodies.iter().any(|b| b == EARTH) {
            let (lat, lon, alt) = dsn_station(43).unwrap();
            let el_analytic =
                station_sun_elevation(EARTH, lat, lon, alt, t_anchor, &line.map_analytic);
            let el_matrix = station_sun_elevation(EARTH, lat, lon, alt, t_anchor, &line.map);
            let el_textbook = textbook_sun_elevation(EARTH, lat, lon, t_anchor, &line.map);
            println!(
                "orientation {} DSS43 anchor: sun elevation analytic {} matrix {} textbook {} deg",
                spec.word,
                fmt_opt(el_analytic),
                fmt_opt(el_matrix),
                fmt_opt(el_textbook),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::{BodyProperties, ChebyshevGranule, CHEBYSHEV_N};
    use omegaflow::ephemeris::rotation_matrix_from_angles;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Arc;

    const GATE_ELEVATION_DEG: f64 = 1.0;

    fn mars_props() -> BodyProperties {
        BodyProperties {
            α0_deg: 317.68143,
            dα0_dt_deg_per_century: 0.0,
            δ0_deg: 52.88650,
            dδ0_dt_deg_per_century: 0.0,
            w0_deg: 176.630,
            dw_dt_deg_per_day: 350.89198226,
            radius_m: 3389500.0,
            flattening: Some(0.00589),
            gaussian_inverse_square: 0.0,
            gaussian_inverse: 0.0,
            erfc: 0.0,
            patch_levy: 0.0,
            exponential_decay: 0.0,
            gm: None,
            j2: None,
            j4: None,
            radii_b: None,
            radii_c: None,
            nut_ra: None,
            nut_dec: None,
            nutation: None,
            omega_g: None,
        }
    }

    fn synthetic_eph(
        props: Option<BodyProperties>,
        pos: [f64; 3],
        matrix: Option<(f64, [f64; 9])>,
    ) -> BodyEphemeris {
        let mut cx = [0.0f64; CHEBYSHEV_N];
        cx[0] = pos[0];
        let mut cy = [0.0f64; CHEBYSHEV_N];
        cy[0] = pos[1];
        let mut cz = [0.0f64; CHEBYSHEV_N];
        cz[0] = pos[2];
        let granule = ChebyshevGranule {
            t0_jd: J2000_EPOCH,
            dt_jd: 32.0,
            cx,
            cy,
            cz,
        };
        BodyEphemeris {
            granules: vec![granule],
            rotation_matrices: match matrix {
                Some(m) => vec![m],
                None => Vec::new(),
            },
            props,
            orbit: None,
            granule_hint: Arc::new(AtomicUsize::new(0)),
        }
    }

    #[test]
    fn subsolar_matrix_matches_analytic_on_a_synthetic_body() {
        let props = mars_props();
        let jd0 = J2000_EPOCH + 5.0;
        let w_mt = props.w0_deg + props.dw_dt_deg_per_day * (jd0 - J2000_EPOCH);
        let m = rotation_matrix_from_angles(props.α0_deg, props.δ0_deg, w_mt);
        let body_matrix = synthetic_eph(Some(props.clone()), [1.5e9, 0.0, 0.0], Some((jd0, m)));
        let body_analytic = synthetic_eph(Some(props), [1.5e9, 0.0, 0.0], None);
        let sun = synthetic_eph(None, [2.0e11, 3.0e10, 1.0e10], None);
        let mut map_matrix = HashMap::new();
        map_matrix.insert("body".to_string(), body_matrix);
        map_matrix.insert("sun".to_string(), sun.clone());
        let mut map_analytic = HashMap::new();
        map_analytic.insert("body".to_string(), body_analytic);
        map_analytic.insert("sun".to_string(), sun);
        for off in [0.0, 0.5, 1.0] {
            let jd = jd0 + off;
            let t = (jd - J2000_EPOCH) * DAY_S;
            let (mlat, mlon) = subsolar("body", t, &map_matrix).expect("matrix subsolar");
            let (alat, alon) = subsolar("body", t, &map_analytic).expect("analytic subsolar");
            let d = arc_deg(mlat, mlon, alat, alon);
            assert!(
                d < 1e-4,
                "off {off}: matrix ({mlat},{mlon}) vs analytic ({alat},{alon}) delta {d} deg"
            );
        }
    }

    fn local_bins_present(eph_dir: &str, netloc: &str, assets: &[&str]) -> bool {
        assets
            .iter()
            .all(|a| std::path::Path::new(&bin_path(eph_dir, netloc, a)).exists())
    }

    #[test]
    fn lehrbuch_gate_de441_anchor() {
        let spec = &LINES[0];
        if !local_bins_present("data", spec.netloc, &[spec.sun_asset, spec.earth_asset]) {
            return;
        }
        let Ok(line) = load_line(spec, "data") else {
            return;
        };
        let Some(lsk) = embedded_lsk() else {
            return;
        };
        let Some(t) = lsk.unix_to_tdb(ANCHOR_UNIX) else {
            return;
        };
        let (lat, lon, alt) = dsn_station(43).unwrap();
        let el_analytic = station_sun_elevation(EARTH, lat, lon, alt, t, &line.map_analytic)
            .expect("lehrbuch gate: the analytic-path DSS43 sun elevation reads absent");
        let el_textbook = textbook_sun_elevation(EARTH, lat, lon, t, &line.map)
            .expect("lehrbuch gate: the textbook sun elevation reads absent");
        assert!(
            (el_analytic - el_textbook).abs() < GATE_ELEVATION_DEG,
            "lehrbuch gate: analytic-path DSS43 sun elevation {el_analytic:.3} deg drifts from the textbook {el_textbook:.3} deg beyond {GATE_ELEVATION_DEG} deg"
        );
    }
}
