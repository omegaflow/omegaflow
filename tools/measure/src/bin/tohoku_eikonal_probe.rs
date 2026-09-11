use omegaflow_measure::eikonal::{
    decode_etopo1, dijkstra_times_boundary, node_at, source_node, unwrap_window,
};
use omegaflow_measure::noaa_coops::{fmt_hhmm, haversine_km, measure_station};
use std::fs;

const BEGIN: &str = "20110311";
const END: &str = "20110311";
const QUAKE_MIN: f64 = 346.4;
const THRESHOLD_M: f64 = 0.3;
const QUAKE_LAT: f64 = 38.297;
const QUAKE_LON: f64 = 142.373;
const LON_MIN: f64 = 110.0;
const LON_MAX: f64 = 240.0;
const LAT_MIN: f64 = -20.0;
const LAT_MAX: f64 = 65.0;
const PASSED_STATION_MAX_DEVIATION_MIN: f64 = 18.0;

const STATIONS: [(&str, &str, f64, f64); 6] = [
    ("1820000", "Kwajalein", 8.73, 167.74),
    ("1619910", "Midway Island", 28.22, -177.37),
    ("9461380", "Adak Island", 51.86, -176.64),
    ("1612340", "Honolulu", 21.30, -157.86),
    ("1617760", "Hilo", 19.73, -155.06),
    ("9419750", "Crescent City", 41.75, -124.18),
];

fn grid_arg() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "--grid" {
            return args.next();
        }
    }
    None
}

fn main() {
    let Some(path) = grid_arg() else {
        println!("usage: tohoku_eikonal_probe --grid <etopo1.grd.gz>");
        return;
    };
    println!("=== tohoku 2011 eikonal — Dijkstra over ETOPO1 bathymetry ===");
    println!(
        "source {QUAKE_LAT} N {QUAKE_LON} E (catalog) | window lon [{LON_MIN},{LON_MAX}] lat [{LAT_MIN},{LAT_MAX}] | c = sqrt(g·d), g = 9.80665, no 200 m depth floor (diverges from the slerp probe shelf convention)"
    );
    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            println!("grid {path} unreadable: {e}");
            return;
        }
    };
    let global = match decode_etopo1(&bytes) {
        Ok(g) => g,
        Err(note) => {
            println!("grid {path} decode note: {note:?}");
            return;
        }
    };
    println!(
        "global grid {} x {} lon0 {:.4} lat0 {:.4} dlon {:.6} dlat {:.6}",
        global.nlon, global.nlat, global.lon0, global.lat0, global.dlon, global.dlat
    );
    let Some(grid) = unwrap_window(&global, LON_MIN, LON_MAX, LAT_MIN, LAT_MAX) else {
        println!("window lon [{LON_MIN},{LON_MAX}] lat [{LAT_MIN},{LAT_MAX}] absent from the global grid");
        return;
    };
    drop(global);
    drop(bytes);
    println!(
        "window {} x {} nodes lon0 {:.4} lat0 {:.4}",
        grid.nlon, grid.nlat, grid.lon0, grid.lat0
    );
    let Some(src) = source_node(&grid, QUAKE_LAT, QUAKE_LON) else {
        println!("epicenter node absent from the window");
        return;
    };
    if src.displaced {
        println!(
            "epicenter node on land (depth {:.0} m) — source carried by the nearest water node, offset {:.1} km",
            src.epicenter_depth, src.offset_km
        );
    } else {
        println!("epicenter node water depth {:.0} m", src.epicenter_depth);
    }
    let (times, touched) = dijkstra_times_boundary(&grid, src.node);
    println!();
    println!(
        "{:<14} {:>8} {:>11} {:>11} {:>9}",
        "station", "dist_km", "eikonal_utc", "meas_utc", "diff_min"
    );
    let mut within = 0usize;
    let mut boundary_hits: Vec<&str> = Vec::new();
    for (id, name, lat, lon) in STATIONS.iter() {
        let dist = haversine_km(QUAKE_LAT, QUAKE_LON, *lat, *lon);
        let station_node = node_at(&grid, *lat, *lon);
        let eik_s = match station_node {
            Some(i) => {
                let t = times[i];
                if grid.depths[i] > 0.0 && t.is_finite() {
                    Some(t)
                } else {
                    None
                }
            }
            None => None,
        };
        let boundary = match station_node {
            Some(i) => touched[i],
            None => false,
        };
        if boundary {
            boundary_hits.push(name);
        }
        let eik_min = match eik_s {
            Some(t) => Some(QUAKE_MIN + t as f64 / 60.0),
            None => None,
        };
        let meas = measure_station(id, BEGIN, END, QUAKE_MIN, THRESHOLD_M);
        let meas_min = match meas.as_ref() {
            Some(m) if m.onset_min.is_finite() => Some(m.onset_min),
            _ => None,
        };
        let diff = match (eik_min, meas_min) {
            (Some(e), Some(m)) => Some(e - m),
            _ => None,
        };
        if let Some(d) = diff {
            if d.abs() <= PASSED_STATION_MAX_DEVIATION_MIN {
                within += 1;
            }
        }
        let eik_txt = match eik_min {
            Some(v) => fmt_hhmm(v),
            None => "-".to_string(),
        };
        let meas_txt = match meas_min {
            Some(v) => fmt_hhmm(v),
            None => "-".to_string(),
        };
        let diff_txt = match diff {
            Some(d) => format!("{d:.1}"),
            None => "-".to_string(),
        };
        println!("{name:<14} {dist:>8.0} {eik_txt:>11} {meas_txt:>11} {diff_txt:>9}");
    }
    println!();
    let gate = if within == STATIONS.len() {
        "GATE PASS"
    } else {
        "GATE FAIL"
    };
    println!(
        "gate: {within}/{} within ±{PASSED_STATION_MAX_DEVIATION_MIN:.0} min → {gate}",
        STATIONS.len()
    );
    if boundary_hits.is_empty() {
        println!("boundary_touched: none");
    } else {
        println!("boundary_touched: {}", boundary_hits.join(", "));
    }
}
