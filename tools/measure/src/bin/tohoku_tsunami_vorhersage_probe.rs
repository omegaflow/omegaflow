use omegaflow::archivar::{jnum, parse_json, JsonVal};
use omegaflow_measure::noaa_coops::{curl_text, fmt_hhmm, haversine_km, measure_station};

const BEGIN: &str = "20110311";
const END: &str = "20110311";
const QUAKE_MIN: f64 = 346.4;
const THRESHOLD_M: f64 = 0.3;
const QUAKE_LAT: f64 = 38.297;
const QUAKE_LON: f64 = 142.373;
const SAMPLES: usize = 25;
const G: f64 = 9.81;

const STATIONS: [(&str, &str, f64, f64); 6] = [
    ("1820000", "Kwajalein", 8.73, 167.74),
    ("1619910", "Midway Island", 28.22, -177.37),
    ("9461380", "Adak Island", 51.86, -176.64),
    ("1612340", "Honolulu", 21.30, -157.86),
    ("1617760", "Hilo", 19.73, -155.06),
    ("9419750", "Crescent City", 41.75, -124.18),
];

fn slerp(lat1: f64, lon1: f64, lat2: f64, lon2: f64, f: f64) -> (f64, f64) {
    let (a1, b1) = (lat1.to_radians(), lon1.to_radians());
    let (a2, b2) = (lat2.to_radians(), lon2.to_radians());
    let v1 = [a1.cos() * b1.cos(), a1.cos() * b1.sin(), a1.sin()];
    let v2 = [a2.cos() * b2.cos(), a2.cos() * b2.sin(), a2.sin()];
    let dot = (v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]).clamp(-1.0, 1.0);
    let omega = dot.acos();
    let sin_omega = omega.sin();
    if sin_omega.abs() < 1e-12 {
        return (lat1, lon1);
    }
    let w1 = ((1.0 - f) * omega).sin() / sin_omega;
    let w2 = (f * omega).sin() / sin_omega;
    let v = [
        w1 * v1[0] + w2 * v2[0],
        w1 * v1[1] + w2 * v2[1],
        w1 * v1[2] + w2 * v2[2],
    ];
    let lat = v[2].atan2((v[0] * v[0] + v[1] * v[1]).sqrt()).to_degrees();
    let lon = v[1].atan2(v[0]).to_degrees();
    (lat, lon)
}

fn fetch_depths(points: &[(f64, f64)]) -> Option<Vec<f64>> {
    let locs: Vec<String> = points
        .iter()
        .map(|(lat, lon)| format!("{lat},{lon}"))
        .collect();
    let url = format!(
        "https://api.opentopodata.org/v1/gebco2020?locations={}",
        locs.join("|")
    );
    let text = curl_text(&url)?;
    let json = parse_json(&text)?;
    let results = match &json {
        JsonVal::Obj(map) => map.get("results"),
        _ => None,
    }?;
    let JsonVal::Arr(items) = results else {
        return None;
    };
    let mut out = Vec::new();
    for item in items {
        out.push(jnum(item, "elevation")?);
    }
    if out.len() == points.len() {
        Some(out)
    } else {
        None
    }
}

fn travel_time_min(depths: &[f64], total_km: f64) -> f64 {
    let n = depths.len();
    let seg_km = total_km / (n as f64 - 1.0);
    let mut t_s = 0.0;
    for i in 0..n - 1 {
        let d1 = depths[i].abs().max(200.0);
        let d2 = depths[i + 1].abs().max(200.0);
        let d_avg = 0.5 * (d1 + d2);
        t_s += seg_km * 1000.0 / (G * d_avg).sqrt();
    }
    t_s / 60.0
}

fn main() {
    println!("=== tohoku 2011 vorhersage — GEBCO depth integration vs measured pegsel ===");
    println!("source 38.297 N 142.373 E (catalog) | speed sqrt(g·d) per segment, depth floor 200 m (shelf convention)");
    println!();
    println!(
        "{:<14} {:>8} {:>11} {:>11} {:>9} {:>8} {:>7} {:>11}",
        "station", "dist_km", "pred_utc", "meas_utc", "diff_min", "mean_m", "min_m", "min_pt"
    );
    for (id, name, lat, lon) in STATIONS.iter() {
        let dist = haversine_km(QUAKE_LAT, QUAKE_LON, *lat, *lon);
        let mut points = Vec::new();
        for k in 0..SAMPLES {
            let f = k as f64 / (SAMPLES as f64 - 1.0);
            points.push(slerp(QUAKE_LAT, QUAKE_LON, *lat, *lon, f));
        }
        let Some(depths) = fetch_depths(&points) else {
            println!("{:<14} GEBCO unreadable", name);
            continue;
        };
        let t_pred = travel_time_min(&depths, dist);
        let pred_utc = QUAKE_MIN + t_pred;
        let mean_d = depths.iter().map(|d| d.abs()).sum::<f64>() / depths.len() as f64;
        let mut min_d = f64::INFINITY;
        let mut min_idx = 0;
        for (i, d) in depths.iter().enumerate() {
            if d.abs() < min_d {
                min_d = d.abs();
                min_idx = i;
            }
        }
        let min_pt = points[min_idx];
        let Some(m) = measure_station(id, BEGIN, END, QUAKE_MIN, THRESHOLD_M) else {
            println!("{:<14} no water_level data", name);
            continue;
        };
        let diff = pred_utc - m.onset_min;
        println!(
            "{:<14} {:>8.0} {:>11} {:>11} {:>9.1} {:>8.0} {:>7.0} {:>11}",
            name,
            dist,
            fmt_hhmm(pred_utc),
            fmt_hhmm(m.onset_min),
            diff,
            mean_d,
            min_d,
            format!("{:.1},{:.1}", min_pt.0, min_pt.1)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slerp_midpoint_stays_on_the_great_circle() {
        let (lat, lon) = slerp(0.0, 0.0, 0.0, 90.0, 0.5);
        assert!((lat - 0.0).abs() < 0.1, "equator mid lat {lat}");
        assert!((lon - 45.0).abs() < 0.1, "mid lon {lon}");
        let (lat2, _) = slerp(45.0, 0.0, 45.0, 180.0, 0.5);
        assert!(
            lat2 > 85.0,
            "the 45N-180 great circle crosses the pole, mid lat {lat2}"
        );
    }

    #[test]
    fn slerp_crosses_the_antimeridian() {
        let (_, lon) = slerp(38.0, 142.0, 20.0, -155.0, 0.5);
        assert!(
            lon > 140.0 || lon < -140.0,
            "mid lon {lon} should sit near ±180"
        );
    }

    #[test]
    fn constant_depth_gives_the_shallow_water_speed() {
        let d = 4000.0;
        let depths = vec![d; 10];
        let dist_km = 4000.0;
        let t = travel_time_min(&depths, dist_km);
        let expect = dist_km * 1000.0 / (G * d).sqrt() / 60.0;
        assert!((t - expect).abs() < 0.5, "t {t} vs expect {expect}");
    }
}
