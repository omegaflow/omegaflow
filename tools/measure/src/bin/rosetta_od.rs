use std::collections::HashMap;
use std::path::{Path, PathBuf};

use omegaflow::archivar::ephemeris::body_of_ephemeris_asset;
use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, body_fixed_vector_to_icrs, embedded_lsk, oem,
    parse_ephemeris_binary, parse_iso_tdb, tdm,
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
    let mut check = false;
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
            _ => i += 1,
        }
    }
    if !check {
        println!(
            "rosetta_od: no mode named — the light-time check is `--check-lighttime`; the fit is not built"
        );
        return;
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
