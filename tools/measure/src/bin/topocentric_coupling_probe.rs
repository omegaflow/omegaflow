use std::collections::HashMap;

use omegaflow::archivar::odp::dsn_station;
use omegaflow::archivar::{
    body_barycenter_position, fetch_raw_bytes, parse_ephemeris_binary, BodyEphemeris, Motion,
    C_LIGHT,
};
use omegaflow::cdn::{CDN_BASE, CDN_RELEASE};

const DSN_DEFAULT_S1: i64 = 63;
const DSN_DEFAULT_S2: i64 = 43;

#[derive(Clone)]
struct Station {
    label: String,
    body: String,
    lat_deg: f64,
    lon_deg: f64,
    alt_m: f64,
}

impl Station {
    fn icrs_at(&self, tdb: f64, eph: &HashMap<String, BodyEphemeris>) -> Option<[f64; 3]> {
        let motion = Motion::Surface {
            body_name: self.body.clone(),
            lat: self.lat_deg,
            lon: self.lon_deg,
            alt: self.alt_m,
        };
        motion.at(tdb, tdb, eph)
    }
}

enum Worldline<'e> {
    Body {
        name: String,
        eph: &'e HashMap<String, BodyEphemeris>,
    },
    Resting {
        unit: [f64; 3],
    },
}

impl Worldline<'_> {
    fn at(&self, tdb: f64) -> Option<[f64; 3]> {
        match self {
            Worldline::Body { name, eph } => body_barycenter_position(name, tdb, eph),
            Worldline::Resting { .. } => None,
        }
    }

    fn resting_unit(&self) -> Option<[f64; 3]> {
        match self {
            Worldline::Resting { unit } => Some(*unit),
            _ => None,
        }
    }
}

struct Fold {
    t_light_s: f64,
    emitted_tdb: f64,
    distance_m: f64,
    unit: [f64; 3],
}

struct StationOutcome {
    unit: Option<[f64; 3]>,
    fold: Option<Fold>,
}

const BIN_TTL_S: u64 = 604800;

fn ensure_bin(path: &str, netloc: &str, asset: &str, ttl: u64) -> Option<Vec<u8>> {
    if let Ok(bytes) = std::fs::read(path) {
        return Some(bytes);
    }
    if !path.starts_with("data/") {
        return None;
    }
    let url = format!("{}/{}/{}", CDN_BASE, netloc, asset);
    let bytes = fetch_raw_bytes(&url, ttl)?;
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(path, &bytes).is_err() {
        return None;
    }
    Some(bytes)
}

fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vec_len(v: [f64; 3]) -> Option<f64> {
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if n.is_finite() {
        Some(n)
    } else {
        None
    }
}

fn toward_unit(from: [f64; 3], to: [f64; 3]) -> Option<[f64; 3]> {
    let d = vec_sub(to, from);
    let n = vec_len(d)?;
    if n <= 0.0 {
        return None;
    }
    Some([d[0] / n, d[1] / n, d[2] / n])
}

fn icrs_unit(ra_deg: f64, dec_deg: f64) -> [f64; 3] {
    let ra = ra_deg.to_radians();
    let dec = dec_deg.to_radians();
    let cd = dec.cos();
    [cd * ra.cos(), cd * ra.sin(), dec.sin()]
}

fn unit_to_icrs_deg(u: [f64; 3]) -> Option<(f64, f64)> {
    if !(u[0].is_finite() && u[1].is_finite() && u[2].is_finite()) {
        return None;
    }
    let p = (u[0] * u[0] + u[1] * u[1]).sqrt();
    if !p.is_finite() {
        return None;
    }
    let ra = u[1].atan2(u[0]).to_degrees().rem_euclid(360.0);
    let dec = u[2].atan2(p).to_degrees();
    Some((ra, dec))
}

fn separation_rad(a: [f64; 3], b: [f64; 3]) -> Option<f64> {
    let na = vec_len(a)?;
    let nb = vec_len(b)?;
    if na <= 0.0 || nb <= 0.0 {
        return None;
    }
    let c = (a[0] * b[0] + a[1] * b[1] + a[2] * b[2]) / (na * nb);
    let s = c.clamp(-1.0, 1.0).acos();
    if s.is_finite() {
        Some(s)
    } else {
        None
    }
}

fn roemer_fold(
    station: [f64; 3],
    tdb: f64,
    worldline: &dyn Fn(f64) -> Option<[f64; 3]>,
) -> Option<Fold> {
    let mut emitted = tdb;
    for _ in 0..12 {
        let apparent = worldline(emitted)?;
        let d = vec_len(vec_sub(apparent, station))?;
        if !(d > 0.0) {
            return None;
        }
        let next = tdb - d / C_LIGHT;
        if !next.is_finite() {
            return None;
        }
        if (next - emitted).abs() < 1e-9 {
            emitted = next;
            break;
        }
        emitted = next;
    }
    let apparent = worldline(emitted)?;
    let distance_m = vec_len(vec_sub(apparent, station))?;
    if !(distance_m > 0.0) {
        return None;
    }
    let unit = toward_unit(station, apparent)?;
    let t_light_s = tdb - emitted;
    if !(t_light_s.is_finite() && t_light_s > 0.0) {
        return None;
    }
    Some(Fold {
        t_light_s,
        emitted_tdb: emitted,
        distance_m,
        unit,
    })
}

fn station_outcome(icrs: [f64; 3], tdb: f64, wl: &Worldline) -> StationOutcome {
    if let Some(unit) = wl.resting_unit() {
        return StationOutcome {
            unit: Some(unit),
            fold: None,
        };
    }
    match roemer_fold(icrs, tdb, &|t| wl.at(t)) {
        Some(f) => StationOutcome {
            unit: Some(f.unit),
            fold: Some(f),
        },
        None => StationOutcome {
            unit: None,
            fold: None,
        },
    }
}

fn plausible_geodetic(lat: f64, lon: f64, alt: f64) -> bool {
    lat.is_finite()
        && lon.is_finite()
        && alt.is_finite()
        && (-90.0..=90.0).contains(&lat)
        && (-360.0..=360.0).contains(&lon)
}

fn load(name: &str) -> Option<BodyEphemeris> {
    let path = format!("data/ssd.jpl.nasa.gov/ephemeris_{name}.bin");
    let asset = format!("ephemeris_{name}.bin");
    ensure_bin(&path, CDN_RELEASE, &asset, BIN_TTL_S)
        .and_then(|bytes| parse_ephemeris_binary(&bytes))
}

fn arg_has(args: &[String], key: &str) -> bool {
    args.iter().any(|a| a == key)
}

fn arg_token(args: &[String], key: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == key)?;
    args.get(pos + 1).cloned()
}

fn arg_f64(args: &[String], key: &str) -> Option<f64> {
    let token = arg_token(args, key)?;
    let v = token.parse::<f64>().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn resolve_station(
    args: &[String],
    body: String,
    lat_key: &str,
    lon_key: &str,
    alt_key: &str,
    default_dsn: Option<i64>,
) -> Result<Station, String> {
    let lat = arg_f64(args, lat_key);
    let lon = arg_f64(args, lon_key);
    let alt = arg_f64(args, alt_key);
    let any = lat.is_some() || lon.is_some() || alt.is_some();
    if any {
        let (Some(lat), Some(lon), Some(alt)) = (lat, lon, alt) else {
            return Err(format!(
                "the {body} geodetic triple is incomplete — {lat_key} {lon_key} {alt_key} arrive together"
            ));
        };
        if !plausible_geodetic(lat, lon, alt) {
            return Err(format!(
                "the {body} geodetic triple {lat}°, {lon}°, {alt} m lies outside its plausibility box"
            ));
        }
        return Ok(Station {
            label: "surface point".to_string(),
            body,
            lat_deg: lat,
            lon_deg: lon,
            alt_m: alt,
        });
    }
    if body == "earth" {
        if let Some(id) = default_dsn {
            if let Some((la, lo, al)) = dsn_station(id) {
                return Ok(Station {
                    label: format!("DSN {id}"),
                    body,
                    lat_deg: la,
                    lon_deg: lo,
                    alt_m: al,
                });
            }
        }
        return Err(format!(
            "no {body} station coordinates arrive — give {lat_key} {lon_key} {alt_key} or a named station"
        ));
    }
    Err(format!(
        "a {body} station carries no geodetic point — give {lat_key} {lon_key} {alt_key} (the named earth stations do not host {body})"
    ))
}

fn print_station(tag: &str, geo: &Station, icrs: Option<[f64; 3]>, tdb: f64) {
    println!(
        "  {} {}: body {}, geodetic lat {:.6}° lon {:.6}° alt {:.1} m",
        tag, geo.label, geo.body, geo.lat_deg, geo.lon_deg, geo.alt_m
    );
    match icrs {
        Some(p) => println!(
            "    ICRS position at the epoch (TDB {tdb:.3} s): x {:.6e} m  y {:.6e} m  z {:.6e} m",
            p[0], p[1], p[2]
        ),
        None => println!(
            "    ICRS position at the epoch: absent — the {} ephemeris does not cover the epoch or carries no surface point",
            geo.body
        ),
    }
}

fn report_target(tdb: f64, label: &str, wl: &Worldline, icrs_list: &[Option<[f64; 3]>]) {
    let resting = wl.resting_unit().is_some();
    println!("  target: {label}");
    let mut outcomes: Vec<StationOutcome> = Vec::with_capacity(icrs_list.len());
    for (i, icrs) in icrs_list.iter().enumerate() {
        match icrs {
            None => {
                println!(
                    "  sightline {}: absent — the station carries no ICRS point at the epoch",
                    i + 1
                );
                outcomes.push(StationOutcome {
                    unit: None,
                    fold: None,
                });
            }
            Some(p) => {
                let out = station_outcome(*p, tdb, wl);
                match (&out.unit, &out.fold) {
                    (Some(u), None) => {
                        if let Some((ra, dec)) = unit_to_icrs_deg(*u) {
                            println!(
                                "  sightline {}: topocentric ICRS direction ra {:.6}° dec {:.6}°  (unit [{:.9}, {:.9}, {:.9}])",
                                i + 1, ra, dec, u[0], u[1], u[2]
                            );
                        }
                        println!(
                            "    the direction rests on the sphere — distance and Rømer light-time are absent (0 honored)"
                        );
                    }
                    (Some(u), Some(f)) => {
                        if let Some((ra, dec)) = unit_to_icrs_deg(*u) {
                            println!(
                                "  sightline {}: topocentric ICRS direction ra {:.6}° dec {:.6}°  (unit [{:.9}, {:.9}, {:.9}])",
                                i + 1, ra, dec, u[0], u[1], u[2]
                            );
                        }
                        println!(
                            "    distance {:.3e} m; Rømer light-time (station point → target) {:.6} s; light left the target at TDB {:.3} s",
                            f.distance_m, f.t_light_s, f.emitted_tdb
                        );
                    }
                    _ => {
                        println!(
                            "  sightline {}: absent — the target worldline carries no point along the Rømer fold",
                            i + 1
                        );
                    }
                }
                outcomes.push(out);
            }
        }
    }
    if icrs_list.len() >= 2 {
        if let (Some(u1), Some(u2)) = (
            outcomes.first().and_then(|o| o.unit),
            outcomes.get(1).and_then(|o| o.unit),
        ) {
            match separation_rad(u1, u2) {
                Some(s) => {
                    let deg = s.to_degrees();
                    let asec = deg * 3600.0;
                    if resting {
                        println!(
                            "  station parallax: the two sightlines open by {:.9}° ({:.3}″) — the direction rests on the sphere, so the offset is the measured zero of parallel sightlines (0 honored)",
                            deg, asec
                        );
                    } else {
                        println!(
                            "  station parallax: the two sightlines to this target open by {:.9}° ({:.3}″)",
                            deg, asec
                        );
                    }
                }
                None => println!("  station parallax: absent — a sightline unit is degenerate"),
            }
        } else {
            println!("  station parallax: absent — every station needs an ICRS point at the epoch");
        }
        if let (Some(fa), Some(fb)) = (
            outcomes.first().and_then(|o| o.fold.as_ref()),
            outcomes.get(1).and_then(|o| o.fold.as_ref()),
        ) {
            println!(
                "  Rømer light-time difference between the sightlines (station 1 − station 2): {:.6} s",
                fa.t_light_s - fb.t_light_s
            );
        }
    }
}

fn usage() {
    println!(
        "Topocentric coupling probe — the station sees the sky from its own worldline.\n\
         Everything is emitted in one frame and one clock: ICRS coordinates, TDB.\n\n\
         cargo run -p omegaflow-measure --bin topocentric_coupling_probe -- \\\n\
         \x20   --tdb <TDB seconds since J2000> \\\n\
         \x20   [--body <name>] [--lat <deg> --lon <deg> --alt <m>] \\\n\
         \x20   [--target-body <name> | --ra <deg> --dec <deg>] \\\n\
         \x20   [--station2 [--body2 <name>] [--lat2 <deg> --lon2 <deg> --alt2 <m>]]\n\n\
         Default station 1: DSN 63 (Madrid) on earth; a declared second station defaults to\n\
         DSN 43 (Canberra) on earth. Both stations measure the same epoch. Ephemeris data come\n\
         from the local data/ssd.jpl.nasa.gov/ephemeris_<name>.bin; a bin absent on disk is fetched\n\
         from the CDN into data/ssd.jpl.nasa.gov, and one the CDN does not serve prints bin void and\n\
         its worldline stays absent."
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!(
        "Topocentric coupling probe — the station sees the sky from its own worldline (ICRS, TDB)."
    );
    let Some(tdb) = arg_f64(&args, "--tdb") else {
        println!(
            "  epoch: absent — --tdb <TDB seconds since J2000> is the one clock; without it no station worldline point is measurable (0 honored)"
        );
        usage();
        return;
    };
    println!("  epoch: TDB {tdb:.3} s (J2000-relative)");

    let s1_body = match arg_token(&args, "--body") {
        Some(b) => b,
        None => "earth".to_string(),
    };
    let s1 = match resolve_station(
        &args,
        s1_body.clone(),
        "--lat",
        "--lon",
        "--alt",
        Some(DSN_DEFAULT_S1),
    ) {
        Ok(s) => Some(s),
        Err(reason) => {
            println!("  station 1: absent — {reason}");
            None
        }
    };

    let s2_enabled = arg_has(&args, "--station2")
        || arg_has(&args, "--lat2")
        || arg_has(&args, "--lon2")
        || arg_has(&args, "--alt2")
        || arg_has(&args, "--body2");
    let s2 = if s2_enabled {
        let s2_body = match arg_token(&args, "--body2") {
            Some(b) => b,
            None => s1_body.clone(),
        };
        match resolve_station(
            &args,
            s2_body,
            "--lat2",
            "--lon2",
            "--alt2",
            Some(DSN_DEFAULT_S2),
        ) {
            Ok(s) => Some(s),
            Err(reason) => {
                println!("  station 2: absent — {reason}");
                None
            }
        }
    } else {
        None
    };

    let mut needed: Vec<String> = Vec::new();
    for body in [&s1, &s2]
        .iter()
        .filter_map(|s| s.as_ref().map(|g| &g.body))
    {
        if !needed.contains(body) {
            needed.push(body.clone());
        }
    }
    if let Some(body) = arg_token(&args, "--target-body") {
        if !needed.contains(&body) {
            needed.push(body.clone());
        }
    }
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for name in &needed {
        match load(name) {
            Some(e) => {
                eph.insert(name.clone(), e);
            }
            None => println!(
                "  {name} ephemeris bin void (data/ssd.jpl.nasa.gov/ephemeris_{name}.bin) — absent on disk with the CDN fetch non-200 or the bin reads unparsed — its worldline carries no point"
            ),
        }
    }

    let stations = [s1, s2];
    let icrs: Vec<Option<[f64; 3]>> = stations
        .iter()
        .map(|s| s.as_ref().and_then(|g| g.icrs_at(tdb, &eph)))
        .collect();
    for (i, (geo_opt, p_opt)) in stations.iter().zip(icrs.iter()).enumerate() {
        if let Some(geo) = geo_opt {
            print_station(&format!("station {}", i + 1), geo, *p_opt, tdb);
        }
    }
    let active = stations.iter().filter(|s| s.is_some()).count();
    let icrs_active: Vec<Option<[f64; 3]>> = icrs[..active].to_vec();

    if active >= 2 {
        match (icrs_active[0], icrs_active[1]) {
            (Some(a), Some(b)) => match vec_len(vec_sub(a, b)) {
                Some(base) => {
                    println!("  station-pair baseline at the epoch: {:.3e} m", base);
                    println!("    ({:.1} km)", base / 1000.0);
                }
                None => {
                    println!("  station-pair baseline: absent — the baseline vector is not finite")
                }
            },
            _ => println!(
                "  station-pair baseline: absent — both stations need an ICRS point at the epoch"
            ),
        }
    }

    let mut targets: Vec<(String, Worldline)> = Vec::new();
    if let Some(body) = arg_token(&args, "--target-body") {
        targets.push((
            format!("body {body} (barycenter worldline)"),
            Worldline::Body {
                name: body,
                eph: &eph,
            },
        ));
    }
    let ra = arg_f64(&args, "--ra");
    let dec = arg_f64(&args, "--dec");
    if ra.is_some() != dec.is_some() {
        println!(
            "  direction target: absent — --ra and --dec arrive together (a lone coordinate is a partial input)"
        );
    } else if let (Some(ra), Some(dec)) = (ra, dec) {
        if (-90.0..=90.0).contains(&dec) {
            targets.push((
                format!("direction ra {ra}° dec {dec}°"),
                Worldline::Resting {
                    unit: icrs_unit(ra, dec),
                },
            ));
        } else {
            println!(
                "  direction target: absent — ra {ra}° dec {dec}° lies outside its plausibility box"
            );
        }
    }
    if targets.is_empty() {
        println!(
            "  target: absent — give --target-body <name> or --ra <deg> --dec <deg> (0 honored)"
        );
        return;
    }
    for (label, wl) in &targets {
        report_target(tdb, label, wl, &icrs_active);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOON_M: f64 = 384_400_000.0;

    #[test]
    fn icrs_unit_maps_celestial_poles_and_equator() {
        let n = icrs_unit(0.0, 90.0);
        assert!(n[0].abs() < 1e-15);
        assert!(n[1].abs() < 1e-15);
        assert!((n[2] - 1.0).abs() < 1e-15);
        let e = icrs_unit(90.0, 0.0);
        assert!(e[0].abs() < 1e-15);
        assert!((e[1] - 1.0).abs() < 1e-15);
        assert!(e[2].abs() < 1e-15);
        let ra = 37.284397f64;
        let dec = 9.258595f64;
        let u = icrs_unit(ra, dec);
        let (ra_back, dec_back) = unit_to_icrs_deg(u).unwrap();
        assert!((ra_back - ra).abs() < 1e-9);
        assert!((dec_back - dec).abs() < 1e-9);
    }

    #[test]
    fn separation_measures_zero_and_right_angle() {
        let x = [1.0, 0.0, 0.0];
        let y = [0.0, 1.0, 0.0];
        assert!(separation_rad(x, x).unwrap().abs() < 1e-15);
        assert!((separation_rad(x, y).unwrap() - std::f64::consts::FRAC_PI_2).abs() < 1e-15);
        assert_eq!(separation_rad([0.0; 3], x), None);
    }

    #[test]
    fn toward_unit_normalizes_and_refuses_a_zero_arm() {
        let u = toward_unit([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]).unwrap();
        assert!((u[0] - 0.6).abs() < 1e-15);
        assert!((u[1] - 0.8).abs() < 1e-15);
        assert!(u[2].abs() < 1e-15);
        assert_eq!(toward_unit([1.0, 1.0, 1.0], [1.0, 1.0, 1.0]), None);
    }

    #[test]
    fn roemer_fold_on_a_static_target_is_distance_over_c() {
        let tdb = 8.0e8;
        let target = |_t: f64| Some([MOON_M, 0.0, 0.0]);
        let f = roemer_fold([0.0, 0.0, 0.0], tdb, &target).unwrap();
        assert!((f.t_light_s - MOON_M / C_LIGHT).abs() < 1e-6);
        assert!((f.distance_m - MOON_M).abs() < 1e-6);
        assert!((f.emitted_tdb - (tdb - MOON_M / C_LIGHT)).abs() < 1e-6);
        assert!((f.unit[0] - 1.0).abs() < 1e-12);
        assert!(f.unit[1].abs() < 1e-12);
        assert!(f.unit[2].abs() < 1e-12);
    }

    #[test]
    fn two_stations_measure_the_station_parallax_of_one_target() {
        let tdb = 8.0e8;
        let b = 10_000_000.0;
        let target = |_t: f64| Some([MOON_M, 0.0, 0.0]);
        let f1 = roemer_fold([0.0, 0.0, 0.0], tdb, &target).unwrap();
        let f2 = roemer_fold([0.0, b, 0.0], tdb, &target).unwrap();
        let sep = separation_rad(f1.unit, f2.unit).unwrap();
        let expect = (b / MOON_M).atan();
        assert!((sep - expect).abs() < 1e-12);
        assert!(f2.t_light_s > f1.t_light_s);
        let spread = (f2.t_light_s - f1.t_light_s).abs();
        let dist_spread = ((f2.distance_m - f1.distance_m) / C_LIGHT).abs();
        assert!((spread - dist_spread) < 1e-3);
    }

    #[test]
    fn a_resting_direction_yields_parallel_sightlines() {
        let u1 = icrs_unit(120.0, -30.0);
        let wl = Worldline::Resting { unit: u1 };
        let out = station_outcome([1.0, 0.0, 0.0], 8.0e8, &wl);
        assert!(out.fold.is_none());
        let u = out.unit.unwrap();
        assert!((u[0] - u1[0]).abs() < 1e-15);
        assert!(separation_rad(u1, u).unwrap().abs() < 1e-15);
    }

    #[test]
    fn non_finite_and_zero_arms_stay_absent() {
        assert!(vec_len([f64::NAN, 0.0, 0.0]).is_none());
        assert_eq!(toward_unit([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]), None);
        assert_eq!(separation_rad([1.0, 0.0, 0.0], [0.0, 0.0, 0.0]), None);
        assert!(unit_to_icrs_deg([f64::INFINITY, 0.0, 0.0]).is_none());
    }

    #[test]
    fn geodetic_plausibility_box_refuses_pole_and_nan_crossings() {
        assert!(plausible_geodetic(40.4, -4.2, 865.0));
        assert!(!plausible_geodetic(90.1, 0.0, 0.0));
        assert!(!plausible_geodetic(0.0, -361.0, 0.0));
        assert!(!plausible_geodetic(0.0, 0.0, f64::NAN));
    }
}
