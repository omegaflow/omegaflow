use omegaflow::archivar::geo::{parse_bin, COMP_BGR_AZIM, MAGIC_BGR};
use omegaflow::archivar::{angular_distance_deg, embedded_lsk};
use omegaflow::spectral::civil_from_days;

const TONGA_LAT: f64 = -20.536;
const TONGA_LON: f64 = -175.382;
const LAMB_SPEED_KM_S: f64 = 0.306;
const EARTH_RADIUS_KM: f64 = 6371.0;
const WATER_START_UNIX: f64 = 1642220085.0;
const DEFAULT_BIN: &str = "data/download.bgr.de/bgr_infrasound_IS52_2022.bin";

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

    println!("=== Tonga 2022 Lamb-wave cross-check — BGR detection vs Lamb travel time + water start time ===");
    println!(
        "source: Hunga Tonga-Hunga Ha'apai (lat {TONGA_LAT}, lon {TONGA_LON}), water start {} UTC",
        utc_str(WATER_START_UNIX)
    );
    println!("Lamb wave speed: {LAMB_SPEED_KM_S} km/s (atmospheric Lamb phase speed)");
    println!("raw pressure waveform: blocked (CTBTO-vDEC HTTP 403) — this probe reads only the BGR detection bin");
    println!();

    let Ok(bytes) = std::fs::read(&bin_path) else {
        println!("cross-check pending: BGR detection bin absent ({bin_path})");
        println!("  harvest first: bgr_infrasound_compiler --year 2022 --out-bin <bin> --lsk <naif0012.tls>");
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
