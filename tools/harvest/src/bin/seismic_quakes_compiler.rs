use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, jpath_val, parse_json, scalar_of};

const VEDUR_URL: &str = "https://api.vedur.is/quakes/events";
const WORLDWIDE_URL: &str = "https://services4.arcgis.com/NyRoDRTC1MqEzjL3/arcgis/rest/services/Worldwide_Earthquake_Data/FeatureServer/0/query?where=1%3D1&outFields=*&outSR=4326&f=json&resultRecordCount=500";
const ROMPLUS_URL: &str = "https://services8.arcgis.com/SXiEEy1skwB5SrYh/arcgis/rest/services/ROMPLUS_Earthquake_Catalogue_v2/FeatureServer/0/query?where=1%3D1&outFields=*&outSR=4326&f=json&resultRecordCount=500";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn jnum_at<'a>(node: &'a JsonVal, keys: &[&str]) -> Option<f64> {
    if let JsonVal::Obj(_) = node {
        for k in keys {
            if let Some(v) = jpath_val(node, k) {
                if let Some(n) = scalar_of(v) {
                    return Some(n);
                }
            }
        }
    }
    None
}

fn iso_from_ms(ms: f64) -> Option<String> {
    if !ms.is_finite() || ms.abs() < 1.0e9 {
        return None;
    }
    let sec = (ms / 1000.0) as i64;
    Some(unix_to_iso(sec as f64))
}

fn feature_list(root: &JsonVal) -> Vec<&JsonVal> {
    match jpath_val(root, "features") {
        Some(JsonVal::Arr(a)) => a.iter().collect(),
        _ => Vec::new(),
    }
}

fn collect_vedur(root: &JsonVal, out: &mut Vec<(String, f64, f64, f64, f64, String)>) {
    for f in feature_list(root) {
        let Some(props) = jpath_val(f, "properties") else {
            continue;
        };
        let (Some(t), Some(lat), Some(lon)) = (
            jnum_at(props, &["time"]),
            jnum_at(f, &["geometry.coordinates.1"]),
            jnum_at(f, &["geometry.coordinates.0"]),
        ) else {
            continue;
        };
        let iso = if t.abs() > 1.0e10 {
            iso_from_ms(t).unwrap_or_default()
        } else {
            unix_to_iso(t)
        };
        if iso.is_empty() {
            continue;
        }
        let depth = jnum_at(props, &["depth"]).unwrap_or(f64::NAN);
        let mag = jnum_at(props, &["magnitude"]).unwrap_or(f64::NAN);
        out.push((iso, lat, lon, depth, mag, "vedur".into()));
    }
}

fn collect_arcgis(
    root: &JsonVal,
    out: &mut Vec<(String, f64, f64, f64, f64, String)>,
    depth_keys: &[&str],
    mag_keys: &[&str],
    time_keys: &[&str],
    lat_keys: &[&str],
    lon_keys: &[&str],
    src: &str,
) {
    for f in feature_list(root) {
        let Some(attrs) = jpath_val(f, "attributes") else {
            continue;
        };
        let (Some(t), Some(lat), Some(lon)) = (
            jnum_at(attrs, time_keys),
            jnum_at(attrs, lat_keys).or_else(|| jnum_at(f, &["geometry.y"])),
            jnum_at(attrs, lon_keys).or_else(|| jnum_at(f, &["geometry.x"])),
        ) else {
            continue;
        };
        let iso = iso_from_ms(t).unwrap_or_default();
        if iso.is_empty() {
            continue;
        }
        let depth = jnum_at(attrs, depth_keys).unwrap_or(f64::NAN);
        let mag = jnum_at(attrs, mag_keys).unwrap_or(f64::NAN);
        out.push((iso, lat, lon, depth, mag, src.to_string()));
    }
}

fn collect_romplus(root: &JsonVal, out: &mut Vec<(String, f64, f64, f64, f64, String)>) {
    for f in feature_list(root) {
        let Some(attrs) = jpath_val(f, "attributes") else {
            continue;
        };
        let (Some(y), Some(mo), Some(dd)) = (
            jnum_at(attrs, &["Year_UTC"]),
            jnum_at(attrs, &["Month_UTC"]),
            jnum_at(attrs, &["Day_UTC"]),
        ) else {
            continue;
        };
        let (Some(lat), Some(lon)) = (
            jnum_at(attrs, &["Latitude"]).or_else(|| jnum_at(f, &["geometry.y"])),
            jnum_at(attrs, &["Longitude"]).or_else(|| jnum_at(f, &["geometry.x"])),
        ) else {
            continue;
        };
        let hh = jnum_at(attrs, &["Hour_UTC"]).unwrap_or(0.0);
        let mi = jnum_at(attrs, &["Minute_UTC"]).unwrap_or(0.0);
        let ss = jnum_at(attrs, &["Second_UTC"]).unwrap_or(0.0);
        let iso = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            y as i64, mo as i64, dd as i64, hh as i64, mi as i64, ss as i64
        );
        let depth = jnum_at(attrs, &["Depth"]).unwrap_or(f64::NAN);
        let mag = jnum_at(attrs, &["Mw", "Ml"]).unwrap_or(f64::NAN);
        out.push((iso, lat, lon, depth, mag, "romplus".into()));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "seismic_quakes.csv".to_string());

    let mut rows: Vec<(String, f64, f64, f64, f64, String)> = Vec::new();
    for (url, kind) in [
        (VEDUR_URL, "vedur"),
        (WORLDWIDE_URL, "worldwide"),
        (ROMPLUS_URL, "romplus"),
    ] {
        let body = match fetch_raw_bytes(url, 60) {
            Some(b) => String::from_utf8_lossy(&b).into_owned(),
            None => {
                eprintln!("seismic: {kind} fetch from {url} returned void");
                continue;
            }
        };
        let j = match parse_json(&body) {
            Some(v) => v,
            None => {
                eprintln!("seismic: {kind} JSON parse void");
                continue;
            }
        };
        let before = rows.len();
        match kind {
            "vedur" => collect_vedur(&j, &mut rows),
            "worldwide" => collect_arcgis(
                &j,
                &mut rows,
                &["depth"],
                &["mag"],
                &["time"],
                &["latitude"],
                &["longitude"],
                "worldwide",
            ),
            _ => collect_romplus(&j, &mut rows),
        }
        eprintln!("seismic: {kind} harvested {} rows", rows.len() - before);
    }

    if rows.len() < 200 {
        eprintln!(
            "seismic harvest carries only {} rows — the feeds are incomplete, no file written",
            rows.len()
        );
        std::process::exit(1);
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0).then(a.5.cmp(&b.5)));
    let mut csv = String::from("time,lat,lon,depth_km,mag,source\n");
    for (t, lat, lon, depth, mag, src) in &rows {
        let ds = if depth.is_finite() {
            format!("{depth:.1}")
        } else {
            String::from("absent")
        };
        let ms = if mag.is_finite() {
            format!("{mag:.2}")
        } else {
            String::from("absent")
        };
        csv.push_str(&format!("{t},{lat:.4},{lon:.4},{ds},{ms},{src}\n"));
    }
    if let Err(e) = std::fs::write(&out, &csv) {
        eprintln!("write {} returned void: {}", out, e);
        std::process::exit(1);
    }
    eprintln!("seismic harvested {} rows → {}", rows.len(), out);
    if ci_mode && !upload_release("service.iris.edu", &out) {
        eprintln!("upload_release for {} returned void", out);
        std::process::exit(1);
    }
}

fn unix_to_iso(u: f64) -> String {
    let sec = u as i64;
    let days = sec.div_euclid(86400);
    let s = sec.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}",
        s / 3600,
        (s % 3600) / 60,
        s % 60
    )
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}
