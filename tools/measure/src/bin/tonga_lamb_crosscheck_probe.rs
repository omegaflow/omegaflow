use omegaflow::archivar::geo::{COMP_BGR_AZIM, MAGIC_BGR, parse_bin};
use omegaflow::archivar::{angular_distance_deg, embedded_lsk, fetch_raw_bytes};
use omegaflow::inflate::inflate;
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::civil_from_days;

const TONGA_LAT: f64 = -20.536;
const TONGA_LON: f64 = -175.382;
const LAMB_SPEED_KM_S: f64 = 0.306;
const EARTH_RADIUS_KM: f64 = 6371.0;
const WATER_START_UNIX: f64 = 1642220085.0;
const DEFAULT_BIN: &str = "data/download.bgr.de/bgr_infrasound_IS52_2022.bin";
const KYOTO_LAT: f64 = 35.02938;
const KYOTO_LON: f64 = 135.78347;
const JST_OFFSET_S: f64 = 32400.0;
const KYOTO_DAY_MEMBER: &str = "220115.txt";
const ZENODO_ARCHIVE_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/zenodo.org/data.zip";
const ZENODO_ARCHIVE_LIVE: &str = "https://zenodo.org/records/8098323/files/data.zip";

struct PressureSample {
    unix: f64,
    hpa: f64,
}

fn load_pressure_archive() -> Option<Vec<u8>> {
    fetch_raw_bytes(ZENODO_ARCHIVE_CDN, 86400)
        .or_else(|| fetch_raw_bytes(ZENODO_ARCHIVE_LIVE, 86400))
}

fn zip_member(data: &[u8], member: &str) -> Option<Vec<u8>> {
    let mut i = 0usize;
    while i + 30 <= data.len() {
        if &data[i..i + 4] != b"PK\x03\x04" {
            i += 1;
            continue;
        }
        let flags = u16::from_le_bytes([data[i + 6], data[i + 7]]);
        let method = u16::from_le_bytes([data[i + 8], data[i + 9]]);
        let comp_size =
            u32::from_le_bytes([data[i + 18], data[i + 19], data[i + 20], data[i + 21]]) as usize;
        let name_len = u16::from_le_bytes([data[i + 26], data[i + 27]]) as usize;
        let extra_len = u16::from_le_bytes([data[i + 28], data[i + 29]]) as usize;
        let name_start = i + 30;
        if name_start + name_len > data.len() {
            return None;
        }
        let name = String::from_utf8_lossy(&data[name_start..name_start + name_len]);
        let start = name_start + name_len + extra_len;
        if start > data.len() {
            return None;
        }
        if name == member {
            let payload = if comp_size == 0 {
                &data[start..]
            } else {
                if start + comp_size > data.len() {
                    return None;
                }
                &data[start..start + comp_size]
            };
            return match method {
                0 => Some(payload.to_vec()),
                8 => inflate(payload),
                _ => None,
            };
        }
        if flags & 0x08 != 0 || comp_size == 0 || start + comp_size > data.len() {
            return None;
        }
        i = start + comp_size;
    }
    None
}

fn parse_pressure(text: &str) -> Vec<PressureSample> {
    let mut samples = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 8 {
            continue;
        }
        let (Ok(year), Ok(month), Ok(day), Ok(hour), Ok(minute), Ok(second), Ok(hpa)) = (
            cols[0].parse::<i64>(),
            cols[1].parse::<i64>(),
            cols[2].parse::<i64>(),
            cols[3].parse::<i64>(),
            cols[4].parse::<i64>(),
            cols[5].parse::<f64>(),
            cols[7].parse::<f64>(),
        ) else {
            continue;
        };
        let Some(days) = days_from_civil(year, month, day) else {
            continue;
        };
        let unix = days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + second
            - JST_OFFSET_S;
        samples.push(PressureSample { unix, hpa });
    }
    samples
}

fn kyoto_pressure_section() {
    let Some(archive) = load_pressure_archive() else {
        println!(
            "raw pressure waveform: Zenodo 8098323 route absent (CDN mirror + live both void) — cross-check pending"
        );
        println!();
        return;
    };
    let Some(bytes) = zip_member(&archive, KYOTO_DAY_MEMBER) else {
        println!(
            "raw pressure waveform: {KYOTO_DAY_MEMBER} absent from the Zenodo archive — cross-check pending"
        );
        println!();
        return;
    };
    let text = String::from_utf8_lossy(&bytes);
    let samples = parse_pressure(&text);
    if samples.is_empty() {
        println!(
            "raw pressure waveform: {KYOTO_DAY_MEMBER} carries no parseable pressure rows (0 honored)"
        );
        println!();
        return;
    }
    let d = distance_km(KYOTO_LAT, KYOTO_LON, TONGA_LAT, TONGA_LON);
    let predicted = WATER_START_UNIX + d / LAMB_SPEED_KM_S;
    let min = samples.iter().min_by(|a, b| a.hpa.total_cmp(&b.hpa));
    let max = samples.iter().max_by(|a, b| a.hpa.total_cmp(&b.hpa));
    println!("Kyoto-A 1-Hz surface pressure (Zenodo 8098323, Kazama 2023, {KYOTO_DAY_MEMBER}):");
    println!(
        "station: lat {KYOTO_LAT}, lon {KYOTO_LON} ({} samples)",
        samples.len()
    );
    println!("great-circle distance to the source: {d:.1} km");
    println!("predicted Lamb arrival (direct): {}", utc_str(predicted));
    if let Some(min) = min {
        println!(
            "measured pressure minimum: {:.2} hPa at {}",
            min.hpa,
            utc_str(min.unix)
        );
    }
    if let Some(max) = max {
        println!(
            "measured pressure maximum: {:.2} hPa at {}",
            max.hpa,
            utc_str(max.unix)
        );
        println!(
            "measured − predicted (maximum): {:.0} s",
            max.unix - predicted
        );
    }
    println!();
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn bearing_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r1 = lat1.to_radians();
    let r2 = lat2.to_radians();
    let dl = (lon2 - lon1).to_radians();
    let y = dl.sin() * r2.cos();
    let x = r1.cos() * r2.sin() - r1.sin() * r2.cos() * dl.cos();
    (y.atan2(x).to_degrees() + 360.0) % 360.0
}

fn distance_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    angular_distance_deg(lat1, lon1, lat2, lon2).to_radians() * EARTH_RADIUS_KM
}

fn wrap_deg(a: f64) -> f64 {
    (a + 180.0).rem_euclid(360.0) - 180.0
}

fn utc_str(unix: f64) -> String {
    let days = unix.div_euclid(86400.0) as i64;
    let secs = unix - days as f64 * 86400.0;
    let (y, m, d) = match civil_from_days(days) {
        Some(t) => t,
        None => return format!("day {days}"),
    };
    let h = (secs / 3600.0) as u32;
    let mi = ((secs % 3600.0) / 60.0) as u32;
    let s = (secs % 60.0) as u32;
    format!("{y:04}-{m:02}-{d:02} {h:02}:{mi:02}:{s:02}")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let bin_path = match arg_value(&args, "--bin") {
        Some(v) => v,
        None => DEFAULT_BIN.to_string(),
    };

    println!(
        "=== Tonga 2022 Lamb-wave cross-check — BGR detection vs Lamb travel time + water start time ==="
    );
    println!(
        "source: Hunga Tonga-Hunga Ha'apai (lat {TONGA_LAT}, lon {TONGA_LON}), water start {} UTC",
        utc_str(WATER_START_UNIX)
    );
    println!("Lamb wave speed: {LAMB_SPEED_KM_S} km/s (atmospheric Lamb phase speed)");
    println!();

    kyoto_pressure_section();

    let Ok(bytes) = std::fs::read(&bin_path) else {
        println!("cross-check pending: BGR detection bin absent ({bin_path})");
        println!(
            "  harvest first: bgr_infrasound_compiler --year 2022 --out-bin <bin> --lsk <naif0012.tls>"
        );
        return;
    };
    let Some(records) = parse_bin(MAGIC_BGR, &bytes) else {
        println!("cross-check pending: {bin_path} carries no BGR1 contract");
        return;
    };
    let azim: Vec<_> = records.iter().filter(|r| r.comp == COMP_BGR_AZIM).collect();
    if azim.is_empty() {
        println!("cross-check pending: the bin carries no back-azimuth detections (0 honored)");
        return;
    }
    let Some(lsk) = embedded_lsk() else {
        println!("cross-check pending: the embedded leap-second table parses void");
        return;
    };
    let Some(water_start_tdb) = lsk.unix_to_tdb(WATER_START_UNIX) else {
        println!("cross-check pending: the water-start epoch maps to no TDB");
        return;
    };

    let (slat, slon) = (azim[0].lat, azim[0].lon);
    let d = distance_km(slat, slon, TONGA_LAT, TONGA_LON);
    let c = 2.0 * std::f64::consts::PI * EARTH_RADIUS_KM;
    let back_azim = bearing_deg(slat, slon, TONGA_LAT, TONGA_LON);

    println!("station: lat {slat}, lon {slon} (from the BGR bin)");
    println!("great-circle distance to the source: {d:.1} km");
    println!("predicted back-azimuth (station → source): {back_azim:.1} deg");
    println!();
    println!("predicted Lamb arrivals (water start + travel time):");
    println!("{:>14}  {:>14}  {:>20}", "path", "travel h", "arrival UTC");
    let paths: [(&str, f64, f64); 4] = [
        ("direct", d, back_azim),
        ("antipodal", c - d, (back_azim + 180.0) % 360.0),
        ("direct+1lap", c + d, back_azim),
        ("antipodal+1lap", 2.0 * c - d, (back_azim + 180.0) % 360.0),
    ];
    for (name, path_km, path_azim) in &paths {
        let travel_h = path_km / LAMB_SPEED_KM_S / 3600.0;
        let arrival_tdb = water_start_tdb + path_km / LAMB_SPEED_KM_S;
        let Some(arrival_unix) = lsk.tdb_to_unix(arrival_tdb) else {
            continue;
        };
        println!(
            "{:>14}  {:>14.2}  {:>20}  (azim {:.1})",
            name,
            travel_h,
            utc_str(arrival_unix),
            path_azim
        );
    }
    println!();
    println!("nearest BGR back-azimuth detection to each predicted arrival:");
    println!(
        "{:>14}  {:>20}  {:>9}  {:>9}  {:>9}",
        "path", "detected UTC", "dt s", "azim deg", "resid deg"
    );
    for (name, path_km, path_azim) in &paths {
        let pred_tdb = water_start_tdb + path_km / LAMB_SPEED_KM_S;
        let mut best: Option<(f64, f64, f64)> = None;
        for r in &azim {
            let dt = r.t - pred_tdb;
            if best.map_or(true, |(bdt, _, _)| dt.abs() < bdt.abs()) {
                best = Some((dt, r.val.rem_euclid(360.0), r.t));
            }
        }
        match best {
            Some((dt, azi, tdb)) => {
                let Some(unix) = lsk.tdb_to_unix(tdb) else {
                    continue;
                };
                println!(
                    "{:>14}  {:>20}  {:>9.0}  {:>9.1}  {:>9.1}",
                    name,
                    utc_str(unix),
                    dt,
                    azi,
                    wrap_deg(azi - path_azim)
                );
            }
            None => println!("{name:>14}  (no detection)"),
        }
    }
}
