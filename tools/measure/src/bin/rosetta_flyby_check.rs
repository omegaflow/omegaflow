use std::path::{Path, PathBuf};

use omegaflow::archivar::geoid::{WGS84_A_M, WGS84_F};
use omegaflow::archivar::{
    aem, aux, body_barycenter_position, embedded_lsk, icrs_to_body_geodetic, oem,
    parse_ephemeris_binary, tdm,
};

const REFERENCE_ALTITUDE_KM: f64 = 1956.0;
const REFERENCE_MASS_KG: f64 = 2895.21;
const REFERENCE_UNIX: &str = "2005-03-04T22:09:14";

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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut package = PathBuf::from(".");
    let mut earth_bin = PathBuf::new();
    let mut body = String::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--package"
            && let Some(p) = args.get(i + 1)
        {
            package = PathBuf::from(p);
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
        "rosetta-check: perigee |r| {radius_km:.3} km at {epoch} TDB, latitude {latitude_deg:.3} deg"
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
    for (file, index, label, reference) in [
        ("mass.ascii", 0usize, "mass kg", Some(REFERENCE_MASS_KG)),
        ("srp.ascii", 0usize, "srp km/s2", None),
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
        if let Some((low, high)) = aux::column_range(&parsed, index) {
            match reference {
                Some(r) => println!(
                    "rosetta-check: {label} {low:.2} .. {high:.2} — reference {r:.2}, delta {:+.2}",
                    low - r
                ),
                None => println!("rosetta-check: {label} {low:.3e} .. {high:.3e}"),
            }
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
