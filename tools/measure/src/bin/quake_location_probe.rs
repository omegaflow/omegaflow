use omegaflow::ak135::p_travel_depth;
use omegaflow::archivar::{
    body_fixed_to_icrs, cache_root, embedded_lsk, fetch_raw_bytes, parse_ephemeris_binary,
    BodyEphemeris,
};
use omegaflow_measure::miniseed::decode_body;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;

const ORIGIN_UNIX: f64 = 1786744701.505;
const START: &str = "2026-08-14T21:58:21";
const END: &str = "2026-08-14T22:58:21";
const ROUTE: &str = "https://service.earthscope.org/fdsnws/dataselect/1/query";
const EARTH_EPH_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/ephemeris_earth.bin";

const STA_WINDOW_S: f64 = 1.0;
const LTA_WINDOW_S: f64 = 30.0;
const STA_LTA_RATIO: f64 = 4.0;
const R_EARTH_M: f64 = 6371000.0;
const PI: f64 = std::f64::consts::PI;
const DEPTHS: [f64; 19] = [
    0.0, 10.0, 15.0, 20.0, 35.0, 50.0, 100.0, 150.0, 200.0, 250.0, 300.0, 350.0, 410.0, 450.0,
    500.0, 550.0, 600.0, 660.0, 700.0,
];

const STATIONS: [(&str, &str, f64, f64); 19] = [
    ("II", "KAPI", -5.0142, 119.7517),
    ("II", "COCO", -12.1901, 96.8349),
    ("IU", "PMG", -9.4047, 147.1597),
    ("IU", "GUMO", 13.5893, 144.8684),
    ("IU", "TATO", 24.9735, 121.4971),
    ("IU", "CHTO", 18.8140, 98.9445),
    ("II", "TAU", -42.9082, 147.3210),
    ("II", "PALK", 7.2728, 80.7022),
    ("IU", "MAJO", 36.5457, 138.2041),
    ("II", "DGAR", -7.4121, 72.4525),
    ("IU", "ULN", 47.8651, 107.0532),
    ("II", "TLY", 51.6807, 103.6438),
    ("II", "NIL", 33.6506, 73.2686),
    ("IU", "MAKZ", 46.8080, 81.9770),
    ("II", "UOSS", 24.9453, 56.2042),
    ("II", "ABPO", -19.0180, 47.2290),
    ("G", "ATD", 11.5307, 42.8466),
    ("IU", "GNI", 40.1480, 44.7410),
    ("II", "KIV", 43.9553, 42.6863),
];

fn curl_bytes(url: &str) -> (Option<u16>, Vec<u8>) {
    let tmp = env::temp_dir().join(format!("quake_{}.mseed", std::process::id()));
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-g")
        .arg("-m")
        .arg("120")
        .arg("--connect-timeout")
        .arg("20")
        .arg("-o")
        .arg(&tmp)
        .arg("-w")
        .arg("%{http_code}")
        .arg(url)
        .output();
    let code = match out {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout);
            s.trim().parse::<u16>().ok().filter(|c| *c > 0)
        }
        Err(_) => None,
    };
    let body = match fs::read(&tmp) {
        Ok(b) => b,
        Err(_) => Vec::new(),
    };
    let _ = fs::remove_file(&tmp);
    (code, body)
}

fn ephemeris_cache_bytes(name: &str, url: &str) -> Option<Vec<u8>> {
    let path = cache_root().join(format!("quake_location_{name}.bin"));
    if let Ok(b) = std::fs::read(&path) {
        return Some(b);
    }
    let b = fetch_raw_bytes(url, 3600)?;
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&path, &b).is_err() {
        println!(
            "quake location probe: the ephemeris cache was not written ({})",
            path.display()
        );
    }
    Some(b)
}

fn sta_lta_arrival(samples: &[(f64, f64)], rate: f64) -> Option<f64> {
    let n_sta = (STA_WINDOW_S * rate).round().max(1.0) as usize;
    let n_lta = (LTA_WINDOW_S * rate).round().max(1.0) as usize;
    if samples.len() < n_lta + 1 {
        return None;
    }
    let n = samples.len();
    let mut prefix = Vec::with_capacity(n + 1);
    let mut acc = 0.0;
    prefix.push(0.0);
    for (_, v) in samples.iter() {
        acc += v.abs();
        prefix.push(acc);
    }
    for i in (n_lta - 1)..n {
        let sta = (prefix[i + 1] - prefix[i + 1 - n_sta]) / n_sta as f64;
        let lta = (prefix[i + 1] - prefix[i + 1 - n_lta]) / n_lta as f64;
        if lta > 1e-12 && sta / lta >= STA_LTA_RATIO {
            return Some(samples[i].0);
        }
    }
    None
}

fn median_abs(xs: &[f64]) -> f64 {
    let mut sorted: Vec<f64> = xs.iter().map(|v| v.abs()).collect();
    sorted.sort_by(|a, b| a.total_cmp(b));
    sorted[sorted.len() / 2]
}

fn bandpass(samples: &[(f64, f64)], rate: f64) -> Vec<f64> {
    let Some(&(_, first)) = samples.first() else {
        return Vec::new();
    };
    let dt = 1.0 / rate;
    let rc_hp = 1.0 / (2.0 * PI * 0.5);
    let a_hp = rc_hp / (rc_hp + dt);
    let rc_lp = 1.0 / (2.0 * PI * 2.0);
    let a_lp = dt / (rc_lp + dt);
    let mut hp = 0.0;
    let mut lp = 0.0;
    let mut prev_x = first;
    let mut out = Vec::with_capacity(samples.len());
    for &(_, x) in samples {
        hp = a_hp * (hp + x - prev_x);
        prev_x = x;
        lp += a_lp * (hp - lp);
        out.push(lp);
    }
    out
}

fn first_break_arrival(samples: &[(f64, f64)], rate: f64) -> Option<f64> {
    let vals = bandpass(samples, rate);
    let noise_len = ((20.0 * rate).round() as usize).min(vals.len() / 2).max(10);
    let floor = median_abs(&vals[..noise_len]);
    if floor <= 1e-12 {
        return None;
    }
    let threshold = 5.0 * floor;
    let sustain = (1.0 * rate).round().max(1.0) as usize;
    let mut count = 0usize;
    for i in noise_len..vals.len() {
        if vals[i].abs() > threshold {
            count += 1;
            if count >= sustain {
                return Some(samples[i - sustain + 1].0);
            }
        } else {
            count = 0;
        }
    }
    None
}

fn p_onset(samples: &[(f64, f64)], rate: f64) -> Option<f64> {
    first_break_arrival(samples, rate).or_else(|| sta_lta_arrival(samples, rate))
}

fn chord(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn median(xs: &mut [f64]) -> f64 {
    xs.sort_by(|a, b| a.total_cmp(b));
    xs[xs.len() / 2]
}

fn residual_at(
    surface_point: &dyn Fn(f64, f64) -> Option<[f64; 3]>,
    station_xyz: &[[f64; 3]],
    arrival: &[f64],
    depth_km: f64,
    lat: f64,
    lon: f64,
) -> Option<f64> {
    let cand = surface_point(lat, lon)?;
    let n = station_xyz.len();
    let mut travel = vec![0.0f64; n];
    for i in 0..n {
        travel[i] = p_travel_depth(arc_deg(&cand, &station_xyz[i]), depth_km)?;
    }
    let mut offsets: Vec<f64> = (0..n).map(|i| arrival[i] - travel[i]).collect();
    let t0 = median(&mut offsets);
    let mut s = 0.0;
    for i in 0..n {
        let r = arrival[i] - t0 - travel[i];
        s += r * r;
    }
    Some(s)
}

fn best_in_box(
    surface_point: &dyn Fn(f64, f64) -> Option<[f64; 3]>,
    station_xyz: &[[f64; 3]],
    arrival: &[f64],
    depth_km: f64,
    center_lat: f64,
    center_lon: f64,
    half_lat: f64,
    half_lon: f64,
    step: f64,
) -> (f64, f64, f64) {
    let mut best = (f64::INFINITY, center_lat, center_lon);
    let mut lat = center_lat - half_lat;
    while lat <= center_lat + half_lat + 1e-12 {
        let mut lon = center_lon - half_lon;
        while lon <= center_lon + half_lon + 1e-12 {
            if let Some(r) = residual_at(surface_point, station_xyz, arrival, depth_km, lat, lon) {
                if r < best.0 {
                    best = (r, lat, lon);
                }
            }
            lon += step;
        }
        lat += step;
    }
    best
}

fn locate(
    surface_point: &dyn Fn(f64, f64) -> Option<[f64; 3]>,
    station_xyz: &[[f64; 3]],
    arrival: &[f64],
    depth_km: f64,
) -> Option<(f64, f64, f64)> {
    let (mut r, mut lat, mut lon) = best_in_box(
        surface_point,
        station_xyz,
        arrival,
        depth_km,
        0.0,
        0.0,
        90.0,
        180.0,
        2.0,
    );
    let (r1, lat1, lon1) = best_in_box(
        surface_point,
        station_xyz,
        arrival,
        depth_km,
        lat,
        lon,
        2.0,
        2.0,
        0.2,
    );
    if r1 < r {
        r = r1;
        lat = lat1;
        lon = lon1;
    }
    let (r2, lat2, lon2) = best_in_box(
        surface_point,
        station_xyz,
        arrival,
        depth_km,
        lat,
        lon,
        0.2,
        0.2,
        0.02,
    );
    if r2 < r {
        r = r2;
        lat = lat2;
        lon = lon2;
    }
    let (r3, lat3, lon3) = best_in_box(
        surface_point,
        station_xyz,
        arrival,
        depth_km,
        lat,
        lon,
        0.02,
        0.02,
        0.002,
    );
    if r3 < r {
        r = r3;
        lat = lat3;
        lon = lon3;
    }
    Some((lat, lon, r))
}

fn locate_depth(
    surface_point: &dyn Fn(f64, f64) -> Option<[f64; 3]>,
    station_xyz: &[[f64; 3]],
    arrival: &[f64],
) -> Option<(f64, f64, f64, f64)> {
    let mut best = (f64::INFINITY, 0.0f64, 0.0f64, 0.0f64);
    for &depth in DEPTHS.iter() {
        if let Some((lat, lon, r)) = locate(surface_point, station_xyz, arrival, depth) {
            if r < best.0 {
                best = (r, lat, lon, depth);
            }
        }
    }
    Some((best.1, best.2, best.3, best.0))
}

fn arc_deg(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    let x = (chord(a, b) / (2.0 * R_EARTH_M)).clamp(-1.0, 1.0);
    2.0 * x.asin() * (180.0 / PI)
}

fn origin_median(
    surface_point: &dyn Fn(f64, f64) -> Option<[f64; 3]>,
    station_xyz: &[[f64; 3]],
    arrival: &[f64],
    depth_km: f64,
    lat: f64,
    lon: f64,
) -> Option<f64> {
    let cand = surface_point(lat, lon)?;
    let mut offsets: Vec<f64> = station_xyz
        .iter()
        .zip(arrival.iter())
        .map(|(xyz, a)| a - p_travel_depth(arc_deg(&cand, xyz), depth_km).unwrap_or(f64::NAN))
        .collect();
    if offsets.iter().any(|v| !v.is_finite()) {
        return None;
    }
    Some(median(&mut offsets))
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let origin_unix = match arg_value(&args, "--origin-unix").and_then(|s| s.parse::<f64>().ok()) {
        Some(t) => t,
        None => ORIGIN_UNIX,
    };
    let start = match arg_value(&args, "--start") {
        Some(v) => v,
        None => START.to_string(),
    };
    let end = match arg_value(&args, "--end") {
        Some(v) => v,
        None => END.to_string(),
    };

    let lsk = match embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!("quake location probe: no leap-second table — abort");
            return;
        }
    };
    let earth_b = match ephemeris_cache_bytes("earth", EARTH_EPH_CDN) {
        Some(b) => b,
        None => {
            eprintln!("quake location probe: earth ephemeris unreadable — abort");
            return;
        }
    };
    let eph = match parse_ephemeris_binary(&earth_b) {
        Some(e) => e,
        None => {
            eprintln!("quake location probe: earth ephemeris unparsed — abort");
            return;
        }
    };
    let t_ref = match lsk.unix_to_tdb(origin_unix) {
        Some(t) => t,
        None => {
            eprintln!("quake location probe: origin time outside TDB range — abort");
            return;
        }
    };
    let mut map: HashMap<String, BodyEphemeris> = HashMap::new();
    map.insert("earth".to_string(), eph);

    println!(
        "=== quake location — {} BHZ stations, P-wave difference inversion ===",
        STATIONS.len()
    );
    println!("origin_unix = {:.3}  window {start} … {end}", origin_unix);
    println!("travel time: ak135 P, depth-variable (kuratierte Klasse)");
    println!(
        "picker: bandpass 0.5–2 Hz + first break (5×noise floor), STA/LTA fallback {} s/{} s ratio {}",
        STA_WINDOW_S, LTA_WINDOW_S, STA_LTA_RATIO
    );
    println!("t_ref = {:.3} s (TDB since J2000)", t_ref);

    let mut keys: Vec<String> = Vec::new();
    let mut station_xyz: Vec<[f64; 3]> = Vec::new();
    let mut arrival: Vec<f64> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();

    for (net, sta, lat, lon) in STATIONS.iter() {
        let key = format!("{}.{}", net, sta);
        let url = format!(
            "{}?network={}&station={}&channel=BHZ&starttime={}&endtime={}&format=miniseed",
            ROUTE, net, sta, start, end
        );
        let (code, body) = curl_bytes(&url);
        if code != Some(200) || body.is_empty() {
            eprintln!("{} dataselect HTTP {:?} — skipped", key, code);
            skipped.push(key);
            continue;
        }
        let Some((samples, rate)) = decode_body(&body) else {
            eprintln!("{} carries no decodable record — skipped", key);
            skipped.push(key);
            continue;
        };
        let Some(t) = p_onset(&samples, rate) else {
            eprintln!("{} carries no P pick — skipped", key);
            skipped.push(key);
            continue;
        };
        let Some(xyz) = body_fixed_to_icrs("earth", *lat, *lon, 0.0, t_ref, &map) else {
            eprintln!("{} worldline absent — skipped", key);
            skipped.push(key);
            continue;
        };
        keys.push(key);
        station_xyz.push(xyz);
        arrival.push(t);
    }

    println!(
        "{} of {} stations carry a P pick",
        arrival.len(),
        STATIONS.len()
    );
    if arrival.len() < 4 {
        eprintln!("fewer than four picks — abort");
        return;
    }

    let surface_point =
        |lat: f64, lon: f64| body_fixed_to_icrs("earth", lat, lon, 0.0, t_ref, &map);

    let Some((lat_all, lon_all, depth_all, res_all)) =
        locate_depth(&surface_point, &station_xyz, &arrival)
    else {
        eprintln!("no location found — abort");
        return;
    };
    let rms_all = (res_all / arrival.len() as f64).sqrt();

    let mut work_xyz: Vec<[f64; 3]> = station_xyz.clone();
    let mut work_arr: Vec<f64> = arrival.clone();
    let mut work_keys: Vec<String> = keys.clone();
    let mut rejected: Vec<String> = Vec::new();
    let mut robust = (lat_all, lon_all, depth_all, res_all);
    for _ in 0..6 {
        let (lat, lon, depth, res) = robust;
        let rms = (res / work_arr.len() as f64).sqrt();
        let Some(cand) = surface_point(lat, lon) else {
            break;
        };
        let Some(t0) = origin_median(&surface_point, &work_xyz, &work_arr, depth, lat, lon) else {
            break;
        };
        let mut drop: Vec<usize> = Vec::new();
        for i in 0..work_xyz.len() {
            let Some(t) = p_travel_depth(arc_deg(&cand, &work_xyz[i]), depth) else {
                continue;
            };
            if (work_arr[i] - t0 - t).abs() > 3.0 * rms {
                drop.push(i);
            }
        }
        if drop.is_empty() {
            break;
        }
        drop.sort();
        drop.reverse();
        for &i in &drop {
            rejected.push(work_keys.remove(i));
            work_xyz.remove(i);
            work_arr.remove(i);
        }
        let Some(next) = locate_depth(&surface_point, &work_xyz, &work_arr) else {
            break;
        };
        robust = next;
    }
    let (lat2, lon2, depth2, res2) = robust;
    let rms2 = (res2 / work_arr.len() as f64).sqrt();

    let cand2 = match surface_point(lat2, lon2) {
        Some(p) => p,
        None => {
            eprintln!("located epicenter worldline absent — abort");
            return;
        }
    };
    let t0 = origin_median(&surface_point, &station_xyz, &arrival, depth2, lat2, lon2);

    println!();
    println!(
        "located (all {} picks): lat = {:.4}  lon = {:.4}  depth = {:.1} km  rms = {:.3} s",
        arrival.len(),
        lat_all,
        lon_all,
        depth_all,
        rms_all
    );
    println!(
        "located ({} picks, {} rejected): lat = {:.4}  lon = {:.4}  depth = {:.1} km  rms = {:.3} s",
        work_arr.len(),
        rejected.len(),
        lat2,
        lon2,
        depth2,
        rms2
    );
    if !rejected.is_empty() {
        println!("rejected (>3σ): {}", rejected.join(", "));
    }
    println!();
    println!("station     arrival_unix     arc_deg  residual_s");
    for (idx, key) in keys.iter().enumerate() {
        let a = arc_deg(&cand2, &station_xyz[idx]);
        let residual = match t0 {
            Some(t) => arrival[idx] - t - p_travel_depth(a, depth2).unwrap_or(f64::NAN),
            None => f64::NAN,
        };
        let mark = if rejected.contains(key) {
            "  rejected"
        } else {
            ""
        };
        println!(
            "{:>12} {:>15.3} {:>8.2} {:>10.2}{}",
            key, arrival[idx], a, residual, mark
        );
    }
    for key in skipped.iter() {
        println!("{:>12} skipped (no pick)", key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn geodetic_to_ecef(lat_deg: f64, lon_deg: f64, alt_m: f64) -> [f64; 3] {
        let a = 6378137.0;
        let f = 1.0 / 298.257223563;
        let e2 = f * (2.0 - f);
        let lat = lat_deg.to_radians();
        let lon = lon_deg.to_radians();
        let n = a / (1.0 - e2 * lat.sin() * lat.sin()).sqrt();
        let x = (n + alt_m) * lat.cos() * lon.cos();
        let y = (n + alt_m) * lat.cos() * lon.sin();
        let z = (n * (1.0 - e2) + alt_m) * lat.sin();
        [x, y, z]
    }

    fn ecef_surface(lat: f64, lon: f64) -> Option<[f64; 3]> {
        Some(geodetic_to_ecef(lat, lon, 0.0))
    }

    #[test]
    fn the_real_station_geometry_recovers_the_synthetic_epicenter() {
        let t0 = ORIGIN_UNIX;
        let true_lat = -8.3514;
        let true_lon = 121.3478;
        let source = geodetic_to_ecef(true_lat, true_lon, 0.0);
        let mut station_xyz: Vec<[f64; 3]> = Vec::new();
        let mut arrival: Vec<f64> = Vec::new();
        for (_, _, lat, lon) in STATIONS.iter() {
            let s = geodetic_to_ecef(*lat, *lon, 0.0);
            let a = arc_deg(&s, &source);
            station_xyz.push(s);
            arrival.push(t0 + p_travel_depth(a, 0.0).unwrap());
        }
        let (lat, lon, depth, _) = locate_depth(&ecef_surface, &station_xyz, &arrival).unwrap();
        let d_deg = {
            let a = geodetic_to_ecef(lat, lon, 0.0);
            arc_deg(&a, &source)
        };
        assert!(
            d_deg < 0.01,
            "recovered ({:.4}, {:.4}) vs true ({}, {}) — {:.4} deg off",
            lat,
            lon,
            true_lat,
            true_lon,
            d_deg
        );
        assert!(depth < 15.0, "surface source recovered depth {depth} km");
    }

    #[test]
    fn a_deep_source_recovers_its_depth() {
        let t0 = ORIGIN_UNIX;
        let true_lat = -8.3514;
        let true_lon = 121.3478;
        let true_depth = 35.0;
        let source = geodetic_to_ecef(true_lat, true_lon, 0.0);
        let mut station_xyz: Vec<[f64; 3]> = Vec::new();
        let mut arrival: Vec<f64> = Vec::new();
        for (_, _, lat, lon) in STATIONS.iter() {
            let s = geodetic_to_ecef(*lat, *lon, 0.0);
            let a = arc_deg(&s, &source);
            station_xyz.push(s);
            arrival.push(t0 + p_travel_depth(a, true_depth).unwrap());
        }
        let (_, _, depth, _) = locate_depth(&ecef_surface, &station_xyz, &arrival).unwrap();
        assert!(
            (depth - true_depth).abs() < 15.0,
            "recovered depth {depth} km vs true {true_depth} km"
        );
    }

    #[test]
    fn a_step_onset_is_picked_at_its_time() {
        let rate = 40.0;
        let n = (60.0 * rate) as usize;
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = 1000.0 + i as f64 / rate;
            let v = if t >= 1030.0 { 10.0 } else { 0.01 };
            samples.push((t, v));
        }
        let pick = sta_lta_arrival(&samples, rate).unwrap();
        assert!(
            (pick - 1030.0).abs() < 1.0,
            "picked {} but onset at 1030.0",
            pick
        );
    }

    #[test]
    fn a_quiet_trace_carries_no_pick() {
        let rate = 40.0;
        let n = (60.0 * rate) as usize;
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = 1000.0 + i as f64 / rate;
            samples.push((t, 0.01));
        }
        assert!(sta_lta_arrival(&samples, rate).is_none());
        assert!(p_onset(&samples, rate).is_none());
    }

    #[test]
    fn an_emergent_onset_is_picked_at_its_first_break() {
        let rate = 40.0;
        let n = (120.0 * rate) as usize;
        let onset = 1060.0;
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = 1000.0 + i as f64 / rate;
            let v = if t < onset {
                0.005 * (2.0 * PI * 0.2 * t).sin()
            } else {
                let ramp = ((t - onset) / 3.0).min(1.0);
                ramp * 2.0 * (2.0 * PI * 2.0 * t).sin()
            };
            samples.push((t, v));
        }
        let pick = first_break_arrival(&samples, rate).unwrap();
        assert!(
            (pick - onset).abs() < 1.0,
            "first break {} should sit near the {} onset",
            pick,
            onset
        );
    }

    #[test]
    fn bandpass_removes_a_slow_drift() {
        let rate = 40.0;
        let n = 4000usize;
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = i as f64 / rate;
            samples.push((t, 50.0 * t));
        }
        let vals = bandpass(&samples, rate);
        let tail = &vals[n / 2..];
        let min = tail.iter().copied().fold(f64::INFINITY, f64::min);
        let max = tail.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        assert!(
            max - min < 2.0,
            "a 50 t/s ramp must flatten after the high-pass, spread {}",
            max - min
        );
    }
}
