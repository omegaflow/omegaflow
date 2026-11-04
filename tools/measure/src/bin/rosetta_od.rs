use std::collections::HashMap;
use std::path::{Path, PathBuf};

use omegaflow::archivar::ephemeris::body_of_ephemeris_asset;
use omegaflow::archivar::{
    BodyEphemeris, aux, body_barycenter_position, body_fixed_vector_to_icrs, csp, embedded_lsk,
    gfc, icrs_to_body_fixed, oem, parse_ephemeris_binary, parse_iso_tdb, tdm,
};

struct Station {
    body: String,
    x_m: f64,
    y_m: f64,
    z_m: f64,
    vx_m_per_year: f64,
    vy_m_per_year: f64,
    vz_m_per_year: f64,
    epoch_year: f64,
}

fn load_stations(path: &Path) -> Option<HashMap<String, Station>> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut out = HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split('\t');
        let Some(name) = parts.next() else {
            continue;
        };
        let Some(body) = parts.next() else {
            continue;
        };
        let numbers: Vec<f64> = (0..7)
            .map(|_| parts.next().and_then(|p| p.trim().parse::<f64>().ok()))
            .collect::<Option<Vec<f64>>>()?;
        out.insert(
            name.to_string(),
            Station {
                body: body.to_string(),
                x_m: numbers[0],
                y_m: numbers[1],
                z_m: numbers[2],
                vx_m_per_year: numbers[3],
                vy_m_per_year: numbers[4],
                vz_m_per_year: numbers[5],
                epoch_year: numbers[6],
            },
        );
    }
    (!out.is_empty()).then_some(out)
}

fn load_ephemeris(path: &Path) -> Option<HashMap<String, BodyEphemeris>> {
    let bytes = std::fs::read(path).ok()?;
    let ephemeris = parse_ephemeris_binary(&bytes)?;
    let name = body_of_ephemeris_asset(path.to_str()?)?;
    let mut map = HashMap::new();
    map.insert(name, ephemeris);
    Some(map)
}

fn station_position(
    station: &Station,
    tdb: f64,
    ephemerides: &HashMap<String, BodyEphemeris>,
) -> Option<[f64; 3]> {
    let years = tdb / 86400.0 / 365.25 + 2000.0;
    let dt = years - station.epoch_year;
    let itrf_m = [
        station.x_m + station.vx_m_per_year * dt,
        station.y_m + station.vy_m_per_year * dt,
        station.z_m + station.vz_m_per_year * dt,
    ];
    body_fixed_vector_to_icrs(&station.body, itrf_m, tdb, ephemerides)
}

fn target_state(
    points: &[oem::OemPoint],
    t: f64,
    body: &str,
    ephemerides: Option<&HashMap<String, BodyEphemeris>>,
    barycentric: bool,
) -> Option<([f64; 3], [f64; 3])> {
    let position_km = oem::position_at(points, t)?;
    let step = 1.0;
    let before = oem::position_at(points, t - step).unwrap_or(position_km);
    let after = oem::position_at(points, t + step).unwrap_or(position_km);
    let velocity_km_s = [
        (after[0] - before[0]) / (2.0 * step),
        (after[1] - before[1]) / (2.0 * step),
        (after[2] - before[2]) / (2.0 * step),
    ];
    if barycentric {
        return Some((position_km, velocity_km_s));
    }
    let map = ephemerides?;
    let offset_m = body_barycenter_position(body, t, map)?;
    let offset_km = [
        offset_m[0] / 1000.0,
        offset_m[1] / 1000.0,
        offset_m[2] / 1000.0,
    ];
    Some((
        [
            position_km[0] + offset_km[0],
            position_km[1] + offset_km[1],
            position_km[2] + offset_km[2],
        ],
        velocity_km_s,
    ))
}

fn tdm_files(package: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for directory in ["DSN_data", "ESTRACK_data"] {
        let Ok(entries) = std::fs::read_dir(package.join(directory)) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.contains("METEO") {
                continue;
            }
            if name.ends_with(".tdm") || name.ends_with(".tdm.txt") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn station_complex(name: &str) -> Option<&'static str> {
    match name {
        "DSS-15" | "DSS-24" | "DSS-25" => Some("C10"),
        "DSS-54" | "DSS-55" => Some("C60"),
        "NN11" => Some("74"),
        _ => None,
    }
}

fn load_tropo(package: &Path) -> Option<tdm::TropoSeries> {
    let entries = std::fs::read_dir(package.join("ESTRACK_data")).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.contains("METEO") || !name.ends_with(".tdm") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        if let Some(parsed) = tdm::parse_tdm(&text)
            && let Some(series) = tdm::TropoSeries::from_tdm(&parsed)
        {
            return Some(series);
        }
    }
    None
}

fn load_ionosphere(package: &Path) -> Vec<csp::CspAdjust> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(package.join("DSN_data")) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.extension().is_some_and(|ext| ext == "csp") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        if let Some(mut parsed) = csp::parse_csp(&text) {
            out.append(&mut parsed);
        }
    }
    out
}

fn iono_at(adjustments: &[csp::CspAdjust], complex: &str, unix: f64) -> Option<f64> {
    adjustments
        .iter()
        .filter(|adjustment| {
            adjustment.part.contains("CHPART") && adjustment.complex.trim() == complex
        })
        .find_map(|adjustment| adjustment.at(unix))
}

fn check_range(
    package: &Path,
    earth_path: Option<&Path>,
    stations_path: Option<&Path>,
    transponder_override: Option<f64>,
) -> Option<()> {
    let earth_path = earth_path?;
    let stations_path = stations_path?;
    let stations = load_stations(stations_path)?;
    let ephemerides = load_ephemeris(earth_path)?;
    let text = std::fs::read_to_string(package.join("ros.oem")).ok()?;
    let segments = oem::parse_oem(&text)?;
    let center = segments
        .first()
        .map(|segment| segment.center_name.clone())?;
    let barycentric = center.to_ascii_uppercase().contains("BARYCENTER");
    let center_body = center.to_ascii_lowercase();
    let points = oem::positions_in_center(&segments, &center);
    if points.is_empty() {
        return None;
    }
    let lsk = embedded_lsk()?;
    let c_km_s = omegaflow::archivar::C_LIGHT / 1000.0;
    let mut total = 0usize;
    let mut absent = 0usize;
    for path in tdm_files(package) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Some(parsed) = tdm::parse_tdm(&text) else {
            continue;
        };
        let file = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<unnamed>")
            .to_string();
        let mut count = 0usize;
        let mut wraps_low = i64::MAX;
        let mut wraps_high = i64::MIN;
        let mut residual_low = f64::INFINITY;
        let mut residual_high = f64::NEG_INFINITY;
        let carrier_hz = tdm::observations(&parsed)
            .find(|observation| observation.name == "TRANSMIT_FREQ")
            .map(|observation| observation.value);
        for block in &parsed.blocks {
            let Some(unit) = block.meta.range_unit() else {
                continue;
            };
            let unit_seconds = match unit {
                tdm::RangeUnit::Ru => block.meta.transmit_band.as_deref().and_then(|band| {
                    carrier_hz.and_then(|carrier| tdm::range_unit_seconds(band, carrier))
                }),
                tdm::RangeUnit::Seconds => Some(1.0),
            };
            let Some(unit_seconds) = unit_seconds else {
                absent += block
                    .observations
                    .iter()
                    .filter(|observation| observation.name == "RANGE")
                    .count();
                continue;
            };
            let Some(station_name) = block.meta.participant_1.clone() else {
                continue;
            };
            let Some(station) = stations.get(&station_name) else {
                continue;
            };
            let Some(reference) = block.meta.timetag_reference() else {
                continue;
            };
            let transponder = match transponder_override.or(block.meta.transponder_delay_s()) {
                Some(value) => value,
                None => {
                    absent += block
                        .observations
                        .iter()
                        .filter(|observation| observation.name == "RANGE")
                        .count();
                    continue;
                }
            };
            for observation in &block.observations {
                if observation.name != "RANGE" {
                    continue;
                }
                let Some(label) = parse_iso_tdb(&observation.time, &lsk) else {
                    absent += 1;
                    continue;
                };
                let station_at = |t: f64| {
                    station_position(station, t, &ephemerides).map(|position| {
                        [
                            position[0] / 1000.0,
                            position[1] / 1000.0,
                            position[2] / 1000.0,
                        ]
                    })
                };
                let target = |t: f64| {
                    target_state(&points, t, &center_body, Some(&ephemerides), barycentric)
                };
                let Some(solution) =
                    tdm::observation_light_time_station(label, reference, station_at, target)
                else {
                    absent += 1;
                    continue;
                };
                let reference_two_way_s = solution.receive_tdb - solution.transmit_tdb;
                let Some(reading) = tdm::unfold_range(
                    observation.value,
                    block.meta.range_modulus,
                    Some(unit_seconds),
                    reference_two_way_s,
                    transponder,
                ) else {
                    absent += 1;
                    continue;
                };
                let corrected = match block.meta.correction_range {
                    Some(value) => reading.unfolded_seconds + value,
                    None => reading.unfolded_seconds,
                };
                let station_delay = match (block.meta.transmit_delay_s, block.meta.receive_delay_s)
                {
                    (Some(transmit), Some(receive)) => Some(transmit + receive),
                    _ => None,
                };
                let (residual_m, delay_text) = match station_delay {
                    Some(delay) => (
                        (corrected - transponder - delay - reference_two_way_s) * c_km_s * 1000.0
                            / 2.0,
                        format!("{:.0} ns", delay * 1.0e9),
                    ),
                    None => (
                        (corrected - transponder - reference_two_way_s) * c_km_s * 1000.0 / 2.0,
                        "pending".to_string(),
                    ),
                };
                let unit_text = match unit {
                    tdm::RangeUnit::Ru => "RU",
                    tdm::RangeUnit::Seconds => "S",
                };
                let modulus_text = match (block.meta.range_modulus, reading.modulus_seconds) {
                    (Some(value), Some(seconds)) => format!("{value} {unit_text} ({seconds:.9} s)"),
                    _ => "pending".to_string(),
                };
                let transponder_text = format!("{:.0} ns", transponder * 1.0e9);
                let correction_text = match block.meta.correction_range {
                    Some(value) => format!("{value:+.9} s"),
                    None => "pending".to_string(),
                };
                println!(
                    "rosetta_od: range {file} {station_name} label {} — raw {} {unit_text} · modulus {modulus_text} · wraps {} · unfolded {:.9} s · oem {:.3} km · one-way {:.3} km · residual {:+.3} m · transponder {transponder_text} · delay {delay_text} · corr {correction_text}",
                    observation.time,
                    observation.value,
                    reading.wraps,
                    reading.unfolded_seconds,
                    reference_two_way_s * c_km_s / 2.0,
                    reading.one_way_km,
                    residual_m
                );
                count += 1;
                total += 1;
                wraps_low = wraps_low.min(reading.wraps);
                wraps_high = wraps_high.max(reading.wraps);
                residual_low = residual_low.min(residual_m);
                residual_high = residual_high.max(residual_m);
            }
        }
        if count > 0 {
            println!(
                "rosetta_od: range {file} {count} reading(s) — wraps {wraps_low}..{wraps_high} · residual {residual_low:+.3}..{residual_high:+.3} m"
            );
        }
    }
    println!(
        "rosetta_od: range {total} reading(s), {absent} absent; raw and unfolded are named per reading"
    );
    Some(())
}

fn check_media(package: &Path) -> Option<()> {
    let lsk = embedded_lsk()?;
    let tropo = load_tropo(package);
    let adjustments = load_ionosphere(package);
    if tropo.is_none() {
        println!("rosetta_od: media — no METEO TDM; dry and wet stay pending");
    }
    let mut observations = 0usize;
    let mut dry_present = 0usize;
    let mut wet_present = 0usize;
    let mut iono_present = 0usize;
    let mut iono_pending = 0usize;
    let mut station_absent = 0usize;
    for path in tdm_files(package) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Some(parsed) = tdm::parse_tdm(&text) else {
            continue;
        };
        let file = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<unnamed>")
            .to_string();
        for block in &parsed.blocks {
            let Some(station_name) = block.meta.participant_1.clone() else {
                station_absent += block.observations.len();
                continue;
            };
            let complex = station_complex(&station_name);
            for observation in &block.observations {
                observations += 1;
                let Some(label) = parse_iso_tdb(&observation.time, &lsk) else {
                    continue;
                };
                let terms = match &tropo {
                    Some(series) => series.at(label),
                    None => tdm::MediaTerms::default(),
                };
                let unix = lsk.tdb_to_unix(label);
                let iono = match (complex, unix) {
                    (Some(code), Some(unix)) => iono_at(&adjustments, code, unix),
                    _ => None,
                };
                if terms.dry_m.is_some() {
                    dry_present += 1;
                }
                if terms.wet_m.is_some() {
                    wet_present += 1;
                }
                match iono {
                    Some(_) => iono_present += 1,
                    None => iono_pending += 1,
                }
                let dry = match terms.dry_m {
                    Some(value) => format!("{value:.6} m"),
                    None => "pending".to_string(),
                };
                let wet = match terms.wet_m {
                    Some(value) => format!("{value:.6} m"),
                    None => "pending".to_string(),
                };
                let iono_text = match iono {
                    Some(value) => format!("{value:+.6} csp"),
                    None => match complex {
                        Some(code) => format!("pending (DSN {code})"),
                        None => "pending (complex unnamed)".to_string(),
                    },
                };
                println!(
                    "rosetta_od: media {file} {station_name} {}_{} label {} — dry {dry} · wet {wet} · iono {iono_text}",
                    observation.name, observation.link, observation.time
                );
            }
        }
    }
    println!(
        "rosetta_od: media {observations} observation(s) — dry {dry_present}, wet {wet_present}; iono {iono_present} in the CSP window, {iono_pending} pending; {station_absent} observation(s) without a station; the metre conversion of the ionosphere is absent (VTEC series)"
    );
    Some(())
}

fn check_forces(package: &Path, earth_path: &Path, gravity_path: &Path) -> Option<()> {
    let ephemerides = load_ephemeris(earth_path)?;
    let body_name = body_of_ephemeris_asset(earth_path.to_str()?)?;
    let gravity_bytes = std::fs::read(gravity_path).ok()?;
    let model = gfc::GfcModel::from_bytes(&gravity_bytes)?;
    let text = std::fs::read_to_string(package.join("ros.oem")).ok()?;
    let segments = oem::parse_oem(&text)?;
    let oem_center = body_name.to_ascii_uppercase();
    let points = oem::positions_in_center(&segments, &oem_center);
    let mut perigee: Option<(f64, f64, [f64; 3])> = None;
    for point in &points {
        let radius = (point.position_km[0] * point.position_km[0]
            + point.position_km[1] * point.position_km[1]
            + point.position_km[2] * point.position_km[2])
            .sqrt();
        if perigee.is_none_or(|(best, _, _)| radius < best) {
            perigee = Some((radius, point.seconds, point.position_km));
        }
    }
    let (radius_km, seconds, relative_km) = perigee?;
    let earth_m = body_barycenter_position(&body_name, seconds, &ephemerides)?;
    let barycentric_km = [
        relative_km[0] + earth_m[0] / 1000.0,
        relative_km[1] + earth_m[1] / 1000.0,
        relative_km[2] + earth_m[2] / 1000.0,
    ];
    let (fixed_x, fixed_y, fixed_z) = icrs_to_body_fixed(
        barycentric_km[0] * 1000.0,
        barycentric_km[1] * 1000.0,
        barycentric_km[2] * 1000.0,
        seconds,
        &body_name,
        &ephemerides,
    )?;
    let radius_m = (fixed_x * fixed_x + fixed_y * fixed_y + fixed_z * fixed_z).sqrt();
    let colatitude = (fixed_z / radius_m).clamp(-1.0, 1.0).acos().to_degrees();
    let longitude = fixed_y.atan2(fixed_x).to_degrees();
    let back = body_fixed_vector_to_icrs(
        &body_name,
        [fixed_x, fixed_y, fixed_z],
        seconds,
        &ephemerides,
    )?;
    let round_trip_m = ((back[0] - barycentric_km[0] * 1000.0).powi(2)
        + (back[1] - barycentric_km[1] * 1000.0).powi(2)
        + (back[2] - barycentric_km[2] * 1000.0).powi(2))
    .sqrt();
    let acceleration = model
        .acceleration(radius_m, colatitude, longitude)
        .map(|a| {
            let magnitude = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
            (magnitude, a)
        })?;
    let mjd2000 = seconds / 86_400.0 + 51_544.5;
    let srp = std::fs::read_to_string(package.join("AUXILIARY_data/srp.ascii"))
        .ok()
        .and_then(|text| aux::parse_aux(&text))
        .and_then(|file| aux::interpolate_at(&file, mjd2000))
        .map(|(values, _, _)| values);
    let albedo = std::fs::read_to_string(package.join("AUXILIARY_data/alb.ascii"))
        .ok()
        .and_then(|text| aux::parse_aux(&text))
        .and_then(|file| aux::interpolate_at(&file, mjd2000))
        .map(|(values, _, _)| values);
    println!(
        "rosetta_od: forces perigee {seconds:.3} s (MJD2000 {mjd2000:.5}) · radius {radius_km:.3} km · body-fixed r {:.3} km, colat {colatitude:.3}°, lon {longitude:.3}°",
        radius_m / 1000.0
    );
    println!(
        "rosetta_od: forces rotation round trip {round_trip_m:.6} m — body-fixed and back through the same chain"
    );
    println!(
        "rosetta_od: forces gravity {:.6e} m/s² (x {:.6e}, y {:.6e}, z {:.6e})",
        acceleration.0, acceleration.1[0], acceleration.1[1], acceleration.1[2]
    );
    match srp {
        Some(values) if values.len() >= 3 => {
            let magnitude =
                (values[0] * values[0] + values[1] * values[1] + values[2] * values[2]).sqrt();
            println!(
                "rosetta_od: forces srp {:.6e} m/s² (x {:.6e}, y {:.6e}, z {:.6e})",
                magnitude * 1000.0,
                values[0] * 1000.0,
                values[1] * 1000.0,
                values[2] * 1000.0
            );
        }
        _ => println!("rosetta_od: forces srp pending"),
    }
    match albedo {
        Some(values) if values.len() >= 3 => {
            let magnitude =
                (values[0] * values[0] + values[1] * values[1] + values[2] * values[2]).sqrt();
            println!(
                "rosetta_od: forces albedo {:.6e} m/s² (x {:.6e}, y {:.6e}, z {:.6e})",
                magnitude * 1000.0,
                values[0] * 1000.0,
                values[1] * 1000.0,
                values[2] * 1000.0
            );
        }
        _ => println!("rosetta_od: forces albedo pending"),
    }
    println!(
        "rosetta_od: forces thermal pending — the srp.ascii carries the SRP acceleration, not a separated thermal term"
    );
    Some(())
}

struct Tally {
    observations: usize,
    solved: usize,
    absent_time: usize,
    absent_station: usize,
    absent_state: usize,
    low: f64,
    high: f64,
}

impl Tally {
    fn new() -> Self {
        Self {
            observations: 0,
            solved: 0,
            absent_time: 0,
            absent_station: 0,
            absent_state: 0,
            low: f64::INFINITY,
            high: f64::NEG_INFINITY,
        }
    }
}

fn check_lighttime(
    package: &Path,
    earth_path: Option<&Path>,
    stations_path: Option<&Path>,
) -> Option<Tally> {
    let Some(earth_path) = earth_path else {
        println!(
            "rosetta_od: lighttime absent — no --earth: the station has no ICRS address, and the geocentric substitute is not taken"
        );
        return None;
    };
    let Some(stations_path) = stations_path else {
        println!(
            "rosetta_od: lighttime absent — no --stations: the compiled station table is the station's address"
        );
        return None;
    };
    let stations = load_stations(stations_path)?;
    let ephemerides = load_ephemeris(earth_path)?;
    let oem_path = package.join("ros.oem");
    let text = std::fs::read_to_string(&oem_path).ok()?;
    let segments = oem::parse_oem(&text)?;
    let center = segments.first().map(|s| s.center_name.clone())?;
    let barycentric = center.to_ascii_uppercase().contains("BARYCENTER");
    let center_body = center.to_ascii_lowercase();
    let points = oem::positions_in_center(&segments, &center);
    if points.is_empty() {
        return None;
    }
    let lsk = embedded_lsk()?;
    let mut tally = Tally::new();
    for path in tdm_files(package) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Some(parsed) = tdm::parse_tdm(&text) else {
            continue;
        };
        let file = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<unnamed>")
            .to_string();
        for block in &parsed.blocks {
            let Some(station_name) = block.meta.participant_1.clone() else {
                tally.absent_station += block.observations.len();
                continue;
            };
            let Some(reference) = block.meta.timetag_reference() else {
                tally.absent_time += block.observations.len();
                continue;
            };
            let Some(station) = stations.get(&station_name) else {
                tally.absent_station += block.observations.len();
                continue;
            };
            let declared = block
                .meta
                .timetag_ref
                .as_deref()
                .unwrap_or("<absent, read as RECEIVE>");
            for observation in &block.observations {
                tally.observations += 1;
                let Some(label) = parse_iso_tdb(&observation.time, &lsk) else {
                    tally.absent_time += 1;
                    continue;
                };
                let Some(position) = station_position(station, label, &ephemerides) else {
                    tally.absent_state += 1;
                    continue;
                };
                let station_km = [
                    position[0] / 1000.0,
                    position[1] / 1000.0,
                    position[2] / 1000.0,
                ];
                let target = |t: f64| {
                    target_state(&points, t, &center_body, Some(&ephemerides), barycentric)
                };
                let Some(solution) =
                    tdm::observation_light_time(label, reference, station_km, target)
                else {
                    tally.absent_state += 1;
                    println!(
                        "rosetta_od: lighttime {file} {station_name} {}_{} label {} — state absent",
                        observation.name, observation.link, observation.time
                    );
                    continue;
                };
                tally.solved += 1;
                tally.low = tally.low.min(solution.state_minus_label_s);
                tally.high = tally.high.max(solution.state_minus_label_s);
                println!(
                    "rosetta_od: lighttime {file} {station_name} {}_{} label {} — state minus label {:+.6} s · range {:.3} km · ref {declared}",
                    observation.name,
                    observation.link,
                    observation.time,
                    solution.state_minus_label_s,
                    solution.range_km
                );
            }
        }
    }
    let frame = format!(
        "ICRS barycentric (OEM {center}), station from the compiled reference table, rotation through ephemeris_earth.bin"
    );
    println!(
        "rosetta_od: lighttime {} observation(s) — {} solved, {} label absent, {} station absent, {} state absent; state minus label {:.6} .. {:.6} s; frame {frame}",
        tally.observations,
        tally.solved,
        tally.absent_time,
        tally.absent_station,
        tally.absent_state,
        tally.low,
        tally.high
    );
    Some(tally)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut package = PathBuf::from(".");
    let mut earth = PathBuf::new();
    let mut stations = PathBuf::new();
    let mut gravity = PathBuf::new();
    let mut transponder: Option<f64> = None;
    let mut check = false;
    let mut media = false;
    let mut range = false;
    let mut forces = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--package" => {
                if let Some(value) = args.get(i + 1) {
                    package = PathBuf::from(value);
                }
                i += 2;
            }
            "--earth" => {
                if let Some(value) = args.get(i + 1) {
                    earth = PathBuf::from(value);
                }
                i += 2;
            }
            "--stations" => {
                if let Some(value) = args.get(i + 1) {
                    stations = PathBuf::from(value);
                }
                i += 2;
            }
            "--check-lighttime" => {
                check = true;
                i += 1;
            }
            "--check-media" => {
                media = true;
                i += 1;
            }
            "--check-range" => {
                range = true;
                i += 1;
            }
            "--transponder-delay" => {
                if let Some(value) = args.get(i + 1) {
                    transponder = value.parse().ok();
                }
                i += 2;
            }
            "--gravity" => {
                if let Some(value) = args.get(i + 1) {
                    gravity = PathBuf::from(value);
                }
                i += 2;
            }
            "--check-forces" => {
                forces = true;
                i += 1;
            }
            _ => i += 1,
        }
    }
    let earth_arg = if earth.as_os_str().is_empty() {
        None
    } else {
        Some(earth.as_path())
    };
    let stations_arg = if stations.as_os_str().is_empty() {
        None
    } else {
        Some(stations.as_path())
    };
    if media {
        check_media(&package);
        return;
    }
    if range {
        if check_range(&package, earth_arg, stations_arg, transponder).is_none() {
            println!(
                "rosetta_od: range absent — --earth and --stations name the prior; without them the wrap count has no anchor"
            );
        }
        return;
    }
    if forces {
        let Some(earth_path) = earth_arg else {
            println!("rosetta_od: forces absent — --earth names the body ephemeris");
            return;
        };
        if gravity.as_os_str().is_empty() {
            println!("rosetta_od: forces absent — --gravity names the field");
            return;
        }
        if check_forces(&package, earth_path, &gravity).is_none() {
            println!(
                "rosetta_od: forces absent — {} or {} carry no readable state",
                package.join("ros.oem").display(),
                gravity.display()
            );
        }
        return;
    }
    if !check {
        println!(
            "rosetta_od: no mode named — the checks are `--check-lighttime`, `--check-media`, `--check-range` and `--check-forces`; the fit is not built"
        );
        return;
    }
    if check_lighttime(&package, earth_arg, stations_arg).is_none()
        && earth_arg.is_some()
        && stations_arg.is_some()
    {
        println!(
            "rosetta_od: lighttime absent — {} or {} carry no readable state",
            package.join("ros.oem").display(),
            package.display()
        );
    }
}
