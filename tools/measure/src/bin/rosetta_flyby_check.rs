use std::path::{Path, PathBuf};

use omegaflow::archivar::geoid::{WGS84_A_M, WGS84_F};
use omegaflow::archivar::{
    aem, aux, body_barycenter_position, embedded_lsk, gfc, icrs_to_body_geodetic, oem,
    parse_ephemeris_binary, parse_iso_tdb, srp, tdm,
};

const REFERENCE_ALTITUDE_KM: f64 = 1956.0;
const REFERENCE_MASS_KG: f64 = 2895.21;
const REFERENCE_UNIX: &str = "2005-03-04T22:09:14";
const REFERENCE_RADIUS_KM: f64 = 8331.7981;
const REFERENCE_VELOCITY_KM_S: f64 = 10.517049;
const REFERENCE_SEMI_MAJOR_KM: f64 = -26704.0701;
const REFERENCE_ECCENTRICITY: f64 = 1.3120048;
const REFERENCE_INCLINATION_DEG: f64 = 144.931532;
const REFERENCE_NODE_DEG: f64 = 170.17250;
const REFERENCE_ARGP_DEG: f64 = 143.09909;
const REFERENCE_VINF_KM_S: f64 = 3.8634932;
const REFERENCE_ARRAY_ANGLE_DEG: f64 = 14.5;
const REFERENCE_ALBEDO_M_S2: f64 = 1.5e-8;
const REFERENCE_MINUS_X_ENTER: &str = "2005-03-04T21:52:00";
const REFERENCE_MINUS_X_EXIT: &str = "2005-03-05T00:20:00";
const REFERENCE_MINUS_Z_ENTER: &str = "2005-03-04T22:17:00";
const REFERENCE_Y_ORTHOGONAL_DEG: f64 = 8.0;

fn ymd_to_unix(y: i64, m: u32, d: u32) -> Option<i64> {
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    let a = (14 - m as i64) / 12;
    let yy = y + 4800 - a;
    let mm = m as i64 + 12 * a - 3;
    let jdn = d as i64 + (153 * mm + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    Some((jdn - 2440588) * 86400)
}

fn civil_to_unix(s: &str) -> Option<f64> {
    let (date, time) = s.split_once('T').or_else(|| s.split_once(' '))?;
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: u32 = dp.next()?.parse().ok()?;
    let d: u32 = dp.next()?.parse().ok()?;
    let t = time.split(['.', 'Z', 'z']).next()?;
    let mut tp = t.split(':');
    let hh: f64 = tp.next()?.parse().ok()?;
    let mm: f64 = tp.next().unwrap_or("0").parse().ok()?;
    let ss: f64 = tp.next().unwrap_or("0").parse().ok()?;
    let mut secs = hh * 3600.0 + mm * 60.0 + ss;
    if let Some((_, fraction)) = time.split_once('.') {
        match format!("0.{fraction}").parse::<f64>() {
            Ok(v) => secs += v,
            Err(_) => return None,
        }
    }
    Some(ymd_to_unix(y, m, d)? as f64 + secs)
}

fn geodetic_altitude_km(x: f64, y: f64, z: f64) -> (f64, f64) {
    let a = WGS84_A_M / 1000.0;
    let f = WGS84_F;
    let e2 = 2.0 * f - f * f;
    let p = (x * x + y * y).sqrt();
    let mut lat = z.atan2(p * (1.0 - e2));
    let mut h = 0.0;
    for _ in 0..6 {
        let sl = lat.sin();
        let n = a / (1.0 - e2 * sl * sl).sqrt();
        h = p / lat.cos() - n;
        lat = z.atan2(p * (1.0 - e2 * n / (n + h)));
    }
    let sl = lat.sin();
    let n = a / (1.0 - e2 * sl * sl).sqrt();
    let rho = ((n * lat.cos()).powi(2) + (n * (1.0 - e2) * sl).powi(2)).sqrt();
    let _ = rho;
    (h, lat.to_degrees())
}

fn files_in(dir: &Path, suffix: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.to_string_lossy().ends_with(suffix) {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn plane_crossings(
    direction: &impl Fn(f64) -> Option<[f64; 3]>,
    axis: usize,
    from: f64,
    to: f64,
    step: f64,
) -> Vec<f64> {
    let mut out = Vec::new();
    let mut previous: Option<f64> = None;
    let mut t = from;
    while t <= to {
        if let Some(v) = direction(t) {
            let sign = v[axis];
            if let Some(p) = previous
                && p.signum() != sign.signum()
            {
                let mut low = t - step;
                let mut high = t;
                for _ in 0..60 {
                    let middle = 0.5 * (low + high);
                    match direction(middle) {
                        Some(vm) if vm[axis].signum() == p.signum() => low = middle,
                        Some(_) => high = middle,
                        None => break,
                    }
                }
                out.push(high);
            }
            previous = Some(sign);
        }
        t += step;
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut package = PathBuf::from(".");
    let mut earth_bin = PathBuf::new();
    let mut body = String::new();
    let mut gm_km3_s2: Option<f64> = None;
    let mut gm_source = String::new();
    let mut gravity_path = PathBuf::new();
    let mut flux_w_m2: Option<f64> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--package"
            && let Some(p) = args.get(i + 1)
        {
            package = PathBuf::from(p);
            i += 2;
        } else if args[i] == "--gm"
            && let Some(v) = args.get(i + 1).and_then(|v| v.parse::<f64>().ok())
        {
            gm_km3_s2 = Some(v);
            gm_source = "argument --gm".to_string();
            i += 2;
        } else if args[i] == "--flux"
            && let Some(v) = args.get(i + 1).and_then(|v| v.parse::<f64>().ok())
        {
            flux_w_m2 = Some(v);
            i += 2;
        } else if args[i] == "--gravity"
            && let Some(p) = args.get(i + 1)
        {
            gravity_path = PathBuf::from(p);
            i += 2;
        } else if args[i] == "--body"
            && let Some(b) = args.get(i + 1)
        {
            body = b.clone();
            i += 2;
        } else if args[i] == "--earth"
            && let Some(p) = args.get(i + 1)
        {
            earth_bin = PathBuf::from(p);
            i += 2;
        } else {
            i += 1;
        }
    }
    let Some(lsk) = embedded_lsk() else {
        println!("rosetta-check: leap seconds absent — the time fold has no leap seconds");
        return;
    };

    if !gravity_path.as_os_str().is_empty() {
        match std::fs::read(&gravity_path) {
            Ok(bytes) => match gfc::parse_head(&bytes) {
                Some((head, _)) => {
                    gm_km3_s2 = Some(head.gm_m3_s2 / 1e9);
                    gm_source = format!(
                        "{} head, model {} to degree {}",
                        gravity_path.display(),
                        head.modelname,
                        head.max_degree
                    );
                }
                None => println!(
                    "rosetta-check: {} carries no gravity head; the gm argument stands",
                    gravity_path.display()
                ),
            },
            Err(_) => println!("rosetta-check: {} absent", gravity_path.display()),
        }
    }

    let oem_path = package.join("ros.oem");
    let Ok(oem_text) = std::fs::read_to_string(&oem_path) else {
        println!("rosetta-check: {} absent", oem_path.display());
        return;
    };
    let Some(segments) = oem::parse_oem(&oem_text) else {
        println!(
            "rosetta-check: {} does not parse as OEM",
            oem_path.display()
        );
        return;
    };
    let mut perigee: Option<(f64, [f64; 3], String)> = None;
    for segment in &segments {
        if segment.center_name != "EARTH" {
            continue;
        }
        for record in &segment.records {
            let r = (record.position_km[0].powi(2)
                + record.position_km[1].powi(2)
                + record.position_km[2].powi(2))
            .sqrt();
            if perigee.as_ref().is_none_or(|(best, _, _)| r < *best) {
                perigee = Some((r, record.position_km, record.time.clone()));
            }
        }
    }
    let Some((radius_km, position, epoch)) = perigee else {
        println!("rosetta-check: no Earth-centred record in the OEM");
        return;
    };
    let (altitude_km, latitude_deg) = geodetic_altitude_km(position[0], position[1], position[2]);

    if !earth_bin.as_os_str().is_empty() {
        match std::fs::read(&earth_bin) {
            Ok(bytes) => match parse_ephemeris_binary(&bytes) {
                Some(earth) => {
                    let mut map = std::collections::HashMap::new();
                    map.insert(body.clone(), earth);
                    let seconds = civil_to_unix(&epoch).map(|u| u - 946_728_000.0);
                    match seconds {
                        Some(seconds) => match body_barycenter_position(&body, seconds, &map) {
                            Some(centre) => {
                                let magnitude = (centre[0] * centre[0]
                                    + centre[1] * centre[1]
                                    + centre[2] * centre[2])
                                    .sqrt();
                                println!(
                                    "rosetta-check: earth barycentric |r| {magnitude:.1} — unit check"
                                );
                                let polar = [centre[0], centre[1], centre[2] + WGS84_A_M];
                                match icrs_to_body_geodetic(
                                    polar[0], polar[1], polar[2], seconds, &body, &map,
                                ) {
                                    Some((lat, _lon, depth)) => println!(
                                        "rosetta-check: pole test — latitude {lat:.3}, altitude {:.3} km (a-b {:.3} expected)",
                                        -depth,
                                        (WGS84_A_M * WGS84_F) / 1000.0
                                    ),
                                    None => println!("rosetta-check: pole test returned absent"),
                                }
                                let point = [
                                    centre[0] + position[0] * 1000.0,
                                    centre[1] + position[1] * 1000.0,
                                    centre[2] + position[2] * 1000.0,
                                ];
                                match icrs_to_body_geodetic(
                                    point[0], point[1], point[2], seconds, &body, &map,
                                ) {
                                    Some((lat, lon, depth)) => println!(
                                        "rosetta-check: library path — latitude {lat:.4}, longitude {lon:.4}, altitude {:.3} km",
                                        -depth
                                    ),
                                    None => {
                                        println!("rosetta-check: library path returned absent")
                                    }
                                }
                            }
                            None => {
                                println!("rosetta-check: earth position absent at {seconds:.1}")
                            }
                        },
                        None => println!("rosetta-check: perigee epoch does not parse"),
                    }
                }
                None => println!("rosetta-check: {} is not an ephemeris", earth_bin.display()),
            },
            Err(_) => println!("rosetta-check: {} absent", earth_bin.display()),
        }
    }

    if let Some(gm) = gm_km3_s2 {
        let mut candidates: Vec<[f64; 3]> = Vec::new();
        for segment in &segments {
            if segment.center_name != "EARTH" {
                continue;
            }
            for record in &segment.records {
                if record.time == epoch && !candidates.contains(&record.velocity_km_s) {
                    candidates.push(record.velocity_km_s);
                }
            }
        }
        if candidates.is_empty() {
            println!("rosetta-check: no velocity record at the perigee epoch");
        }
        let mut semi_major: Vec<f64> = Vec::new();
        for v in &candidates {
            let vmag = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            let a = 1.0 / (2.0 / radius_km - vmag * vmag / gm);
            let h = [
                position[1] * v[2] - position[2] * v[1],
                position[2] * v[0] - position[0] * v[2],
                position[0] * v[1] - position[1] * v[0],
            ];
            let hmag = (h[0] * h[0] + h[1] * h[1] + h[2] * h[2]).sqrt();
            let rv = position[0] * v[0] + position[1] * v[1] + position[2] * v[2];
            let evec = [
                ((vmag * vmag - gm / radius_km) * position[0] - rv * v[0]) / gm,
                ((vmag * vmag - gm / radius_km) * position[1] - rv * v[1]) / gm,
                ((vmag * vmag - gm / radius_km) * position[2] - rv * v[2]) / gm,
            ];
            let e = (evec[0] * evec[0] + evec[1] * evec[1] + evec[2] * evec[2]).sqrt();
            let inc = (h[2] / hmag).acos().to_degrees();
            let n = [-h[1], h[0]];
            let nmag = (n[0] * n[0] + n[1] * n[1]).sqrt();
            let mut node_deg = n[1].atan2(n[0]).to_degrees();
            if node_deg < 0.0 {
                node_deg += 360.0;
            }
            let argp_raw = ((evec[0] * n[0] + evec[1] * n[1]) / (nmag * e))
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees();
            let argp_deg = if evec[2] < 0.0 {
                360.0 - argp_raw
            } else {
                argp_raw
            };
            let v_inf = if a < 0.0 {
                Some((gm / -a).sqrt())
            } else {
                None
            };
            semi_major.push(a);
            println!(
                "rosetta-check: elements with gm {gm:.6} from {gm_source} — v {vmag:.9} km/s (delta {:+.3} mm/s), a {a:.4} km (delta {:+.1} m), e {e:.7} (delta {:+.1e}), i {inc:.6} deg (delta {:+.6} deg)",
                (vmag - REFERENCE_VELOCITY_KM_S) * 1e6,
                (a - REFERENCE_SEMI_MAJOR_KM) * 1000.0,
                e - REFERENCE_ECCENTRICITY,
                inc - REFERENCE_INCLINATION_DEG
            );
            println!(
                "rosetta-check: angles — node {node_deg:.5} deg (delta {:+.5}), argp {argp_deg:.5} deg (delta {:+.5}), v_inf {} (reference {REFERENCE_VINF_KM_S:.7} km/s)",
                node_deg - REFERENCE_NODE_DEG,
                argp_deg - REFERENCE_ARGP_DEG,
                match v_inf {
                    Some(x) => format!("{x:.7} km/s (delta {:+.7})", x - REFERENCE_VINF_KM_S),
                    None => "absent — a is not negative".to_string(),
                }
            );
        }
        if semi_major.len() > 1 {
            let low = semi_major.iter().cloned().fold(f64::INFINITY, f64::min);
            let high = semi_major.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            println!(
                "rosetta-check: the perigee epoch sits on a segment seam — {} distinct velocities, a spread {:.1} m",
                semi_major.len(),
                (high - low).abs() * 1000.0
            );
        }
    }

    let reference_unix = match civil_to_unix(REFERENCE_UNIX) {
        Some(u) => u,
        None => {
            println!("rosetta-check: reference epoch does not parse");
            return;
        }
    };
    let reference_tdb = match lsk.unix_to_tdb(reference_unix) {
        Some(t) => t,
        None => {
            println!("rosetta-check: leap-second fold absent at the reference epoch");
            return;
        }
    };
    const J2000_UNIX: f64 = 946_728_000.0;
    let offset = reference_tdb + J2000_UNIX - reference_unix;
    let perigee_unix = civil_to_unix(&epoch).map(|u| u - offset);

    println!(
        "rosetta-check: OEM {} segment(s), {}/{} Earth-centred records",
        segments.len(),
        segments
            .iter()
            .filter(|s| s.center_name == "EARTH")
            .map(|s| s.records.len())
            .sum::<usize>(),
        oem::records(&segments)
    );
    println!(
        "rosetta-check: perigee |r| {radius_km:.4} km at {epoch} TDB, latitude {latitude_deg:.3} deg — reference {REFERENCE_RADIUS_KM:.4} km (arXiv 2609.23482), delta {:+.1} m",
        (radius_km - REFERENCE_RADIUS_KM) * 1000.0
    );
    println!(
        "rosetta-check: geodetic altitude {altitude_km:.2} km — reference {REFERENCE_ALTITUDE_KM:.1} km (arXiv 2609.23482), delta {:+.2} km",
        altitude_km - REFERENCE_ALTITUDE_KM
    );
    match perigee_unix {
        Some(u) => {
            let delta = u - reference_unix;
            println!(
                "rosetta-check: perigee {u:.2} unix — reference {reference_unix:.2} (ESA release 2005), delta {delta:+.2} s"
            );
        }
        None => println!("rosetta-check: perigee epoch does not parse"),
    }

    let aux_dir = package.join("AUXILIARY_data");
    let perigee_mjd2000 = civil_to_unix(&epoch).map(|u| (u - aux::MJD2000_ORIGIN_UNIX) / 86_400.0);
    match perigee_mjd2000 {
        Some(mjd) => println!(
            "rosetta-check: perigee MJD2000 {mjd:.6} — aux epochs count from 2000-01-01 00:00 TDB"
        ),
        None => println!("rosetta-check: perigee epoch does not parse as MJD2000"),
    }
    let mut mass_at_perigee: Option<f64> = None;
    let mut array_angle_rad: Option<f64> = None;
    let mut srp_main_at_perigee: Option<f64> = None;
    if let Some(mjd) = perigee_mjd2000 {
        for (file, label, reference, scale, unit) in [
            ("mass.ascii", "mass", Some(REFERENCE_MASS_KG), 1.0, "kg"),
            ("srp.ascii", "srp main component", None, 1.0, "km/s2"),
            (
                "sa.ascii",
                "array angle",
                Some(REFERENCE_ARRAY_ANGLE_DEG),
                1.0,
                "deg",
            ),
        ] {
            let path = aux_dir.join(file);
            let Ok(text) = std::fs::read_to_string(&path) else {
                println!("rosetta-check: {} absent", path.display());
                continue;
            };
            let Some(parsed) = aux::parse_aux(&text) else {
                println!("rosetta-check: {} does not parse", path.display());
                continue;
            };
            let Some((values, left, right)) = aux::interpolate_at(&parsed, mjd) else {
                println!(
                    "rosetta-check: {file} carries no bracket for MJD2000 {mjd:.6} — the epoch lies outside its span"
                );
                continue;
            };
            let Some(first) = values.first().copied() else {
                continue;
            };
            let value = if file == "sa.ascii" {
                first.to_degrees() * scale
            } else {
                first * scale
            };
            match file {
                "mass.ascii" => mass_at_perigee = Some(value),
                "sa.ascii" => array_angle_rad = Some(first),
                "srp.ascii" => srp_main_at_perigee = Some(first),
                _ => {}
            }
            match reference {
                Some(r) => println!(
                    "rosetta-check: {label} {value:.6} {unit} at perigee (bracket {left:.6} .. {right:.6}) — reference {r:.4}, delta {:+.6}",
                    value - r
                ),
                None => println!(
                    "rosetta-check: {label} {value:.6e} {unit} at perigee (bracket {left:.6} .. {right:.6})"
                ),
            }
        }
    }

    match (
        mass_at_perigee,
        array_angle_rad,
        srp_main_at_perigee,
        flux_w_m2,
    ) {
        (Some(mass), Some(angle), Some(main), Some(flux)) => {
            let normal = [angle.cos(), 0.0, angle.sin()];
            let faces = [
                srp::Face {
                    area_m2: 5.15,
                    normal: [1.0, 0.0, 0.0],
                    absorption: 0.909,
                    diffuse_ratio: 1.0,
                },
                srp::Face {
                    area_m2: 5.15,
                    normal: [-1.0, 0.0, 0.0],
                    absorption: 0.853,
                    diffuse_ratio: 0.925,
                },
                srp::Face {
                    area_m2: 5.4075,
                    normal: [0.0, 1.0, 0.0],
                    absorption: 0.795,
                    diffuse_ratio: 1.0,
                },
                srp::Face {
                    area_m2: 5.4075,
                    normal: [0.0, -1.0, 0.0],
                    absorption: 0.750,
                    diffuse_ratio: 1.0,
                },
                srp::Face {
                    area_m2: 4.2,
                    normal: [0.0, 0.0, 1.0],
                    absorption: 0.916,
                    diffuse_ratio: 1.0,
                },
                srp::Face {
                    area_m2: 4.2,
                    normal: [0.0, 0.0, -1.0],
                    absorption: 0.889,
                    diffuse_ratio: 0.721,
                },
                srp::Face {
                    area_m2: 32.31,
                    normal,
                    absorption: 0.840,
                    diffuse_ratio: 0.313,
                },
                srp::Face {
                    area_m2: 32.31,
                    normal,
                    absorption: 0.840,
                    diffuse_ratio: 0.313,
                },
            ];
            match srp::force_n(&faces, normal, flux) {
                Some(force) => {
                    let along = force[0] * normal[0] + force[1] * normal[1] + force[2] * normal[2];
                    let rebuilt = along / mass / 1000.0;
                    let implied_flux = flux * main.abs() / rebuilt;
                    println!(
                        "rosetta-check: srp rebuilt from the optical table at {flux:.2} W/m2 — {rebuilt:.4e} km/s2 along the sun line, file {main:.4e} (the file counts sunward negative), ratio {:.4}, implied flux {implied_flux:.1} W/m2",
                        rebuilt / main.abs()
                    );
                }
                None => println!("rosetta-check: the srp rebuild carries no force at this flux"),
            }
        }
        _ => println!(
            "rosetta-check: the srp rebuild needs the mass, the array angle, the file value and --flux"
        ),
    }

    let alb_path = aux_dir.join("alb.ascii");
    if let Ok(text) = std::fs::read_to_string(&alb_path)
        && let Some(parsed) = aux::parse_aux(&text)
    {
        match (aux::span(&parsed), aux::peak(&parsed, 3), perigee_mjd2000) {
            (Some((first, last)), Some((norm_magnitude, at, _)), Some(mjd)) => println!(
                "rosetta-check: albedo spans MJD2000 {first:.5} .. {last:.5} = {:+.2} .. {:+.2} h around perigee; peak {:.4e} km/s2 = {:.3e} m/s2 at {:.2} min after perigee (reference {:.1e}), delta {:+.1} %",
                (first - mjd) * 24.0,
                (last - mjd) * 24.0,
                norm_magnitude,
                norm_magnitude * 1000.0,
                (at - mjd) * 24.0 * 60.0,
                REFERENCE_ALBEDO_M_S2,
                (norm_magnitude * 1000.0 / REFERENCE_ALBEDO_M_S2 - 1.0) * 100.0
            ),
            _ => println!("rosetta-check: albedo carries no peak"),
        }
    }

    let aem_path = package.join("ros.aem");
    if let Ok(text) = std::fs::read_to_string(&aem_path)
        && let Some(attitude) = aem::parse_aem(&text)
    {
        let worst = aem::worst_norm_deviation(&attitude);
        println!(
            "rosetta-check: AEM {} segment(s), {} record(s), worst |q|-1 {:?}",
            attitude.len(),
            aem::records(&attitude),
            worst.map(|(d, _)| d)
        );
        let degree = match attitude.iter().find_map(|s| s.interpolation_degree) {
            Some(declared) => declared as usize,
            None => {
                println!(
                    "rosetta-check: the AEM declares no interpolation degree — the attitude check is absent"
                );
                return;
            }
        };
        let earth_direction = {
            let norm = radius_km;
            [
                -position[0] / norm,
                -position[1] / norm,
                -position[2] / norm,
            ]
        };
        for (label, civil) in [
            ("perigee", epoch.clone()),
            ("paper 22:17 TDB", "2005-03-04T22:17:00".to_string()),
        ] {
            let Some(seconds) = civil_to_unix(&civil).map(|u| u - J2000_UNIX) else {
                println!("rosetta-check: {label} does not parse");
                continue;
            };
            let Some(q) = aem::quaternion_at(&attitude, seconds, &lsk, degree) else {
                println!("rosetta-check: {label} carries no attitude interpolation");
                continue;
            };
            let body = aem::to_body_frame(q, earth_direction);
            let angle_minus_x = (-body[0]).clamp(-1.0, 1.0).acos().to_degrees();
            let angle_minus_z = (-body[2]).clamp(-1.0, 1.0).acos().to_degrees();
            println!(
                "rosetta-check: earth direction in body frame at {label} — x {:+.4}, y {:+.4}, z {:+.4}; angle to -X {angle_minus_x:.2} deg, to -Z {angle_minus_z:.2} deg",
                body[0], body[1], body[2]
            );
        }

        let points = oem::positions_in_center(&segments, "EARTH", &lsk);
        let direction = |seconds: f64| -> Option<[f64; 3]> {
            let p = oem::position_at(&points, seconds)?;
            let q = aem::quaternion_at(&attitude, seconds, &lsk, degree)?;
            let norm = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            if norm <= 0.0 {
                return None;
            }
            Some(aem::to_body_frame(
                q,
                [-p[0] / norm, -p[1] / norm, -p[2] / norm],
            ))
        };
        if let Some(base) = parse_iso_tdb(&epoch, &lsk) {
            let minus_x = plane_crossings(
                &direction,
                0,
                base - 4.0 * 3600.0,
                base + 6.0 * 3600.0,
                10.0,
            );
            let minus_z = plane_crossings(
                &direction,
                2,
                base - 4.0 * 3600.0,
                base + 6.0 * 3600.0,
                10.0,
            );
            let reference = |civil: &str| parse_iso_tdb(civil, &lsk);
            let enter = reference(REFERENCE_MINUS_X_ENTER);
            let exit = reference(REFERENCE_MINUS_X_EXIT);
            let z_enter = reference(REFERENCE_MINUS_Z_ENTER);
            let delta = |crossing: f64, printed: Option<f64>| match printed {
                Some(r) => format!("{:+.0} s", crossing - r),
                None => "the printed time does not parse".to_string(),
            };
            match (minus_x.first(), minus_x.get(1)) {
                (Some(first), Some(second)) => println!(
                    "rosetta-check: -X face visible from {:+.1} to {:+.1} min around perigee — paper 21:52 and 00:20, delta {} and {}",
                    (first - base) / 60.0,
                    (second - base) / 60.0,
                    delta(*first, enter),
                    delta(*second, exit)
                ),
                _ => println!("rosetta-check: -X visibility crossings absent"),
            }
            match minus_z.first() {
                Some(first) => println!(
                    "rosetta-check: -Z face enters visibility at {:+.1} min around perigee — paper 22:17, delta {}",
                    (first - base) / 60.0,
                    delta(*first, z_enter)
                ),
                None => println!("rosetta-check: -Z visibility crossing absent"),
            }
            let mut worst: f64 = 0.0;
            let mut worst_at = 0.0;
            let mut t = base - 2.0 * 3600.0;
            while t <= base + 2.0 * 3600.0 {
                if let Some(v) = direction(t) {
                    let deviation = v[1].abs().min(1.0).asin().to_degrees();
                    if deviation > worst {
                        worst = deviation;
                        worst_at = t;
                    }
                }
                t += 20.0;
            }
            println!(
                "rosetta-check: the body Y axis stays {worst:.2} deg from square to the earth direction within two hours (at {:+.0} min) — paper within {REFERENCE_Y_ORTHOGONAL_DEG:.0} deg",
                (worst_at - base) / 60.0
            );
        }
    }

    let dsn = files_in(&package.join("DSN_data"), ".tdm.txt");
    let estrack = files_in(&package.join("ESTRACK_data"), ".tdm");
    let mut observations = 0usize;
    let mut messages = 0usize;
    for path in dsn.iter().chain(estrack.iter()) {
        if let Ok(text) = std::fs::read_to_string(path)
            && let Some(parsed) = tdm::parse_tdm(&text)
        {
            messages += 1;
            observations += tdm::observations(&parsed).count();
        }
    }
    println!("rosetta-check: TDM {messages} message(s), {observations} observation(s)");
}
