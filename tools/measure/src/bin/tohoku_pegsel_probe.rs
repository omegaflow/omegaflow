use omegaflow_measure::noaa_coops::{fmt_hhmm, haversine_km, measure_station};

const BEGIN: &str = "20110311";
const END: &str = "20110311";
const QUAKE_MIN: f64 = 346.4;
const THRESHOLD_M: f64 = 0.3;
const QUAKE_LAT: f64 = 38.297;
const QUAKE_LON: f64 = 142.373;

const STATIONS: [(&str, &str, f64, f64); 6] = [
    ("1820000", "Kwajalein", 8.73, 167.74),
    ("1619910", "Midway Island", 28.22, -177.37),
    ("9461380", "Adak Island", 51.86, -176.64),
    ("1612340", "Honolulu", 21.30, -157.86),
    ("1617760", "Hilo", 19.73, -155.06),
    ("9419750", "Crescent City", 41.75, -124.18),
];

fn main() {
    println!("=== tohoku 2011 pegsel — NOAA CO-OPS water level vs tide prediction ===");
    println!(
        "quake 2011-03-11 05:46 UTC | onset = first |anomaly| > {:.1} m after the quake",
        THRESHOLD_M
    );
    println!();
    println!(
        "{:<14} {:>8} {:>10} {:>10} {:>12} {:>12}",
        "station", "dist_km", "pre_floor", "onset_utc", "peak_m", "peak_utc"
    );
    for (id, name, lat, lon) in STATIONS.iter() {
        let Some(m) = measure_station(id, BEGIN, END, QUAKE_MIN, THRESHOLD_M) else {
            println!("{:<14} no data", name);
            continue;
        };
        let dist = haversine_km(QUAKE_LAT, QUAKE_LON, *lat, *lon);
        println!(
            "{:<14} {:>8.0} {:>10.2} {:>10} {:>12.2} {:>12}",
            name,
            dist,
            m.pre_floor,
            fmt_hhmm(m.onset_min),
            m.peak_m,
            fmt_hhmm(m.peak_min)
        );
    }
}
