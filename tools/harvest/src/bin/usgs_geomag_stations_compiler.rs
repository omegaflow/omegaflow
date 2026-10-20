use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json};
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "geomag.usgs.gov";
const URL: &str = "https://geomag.usgs.gov/ws/observatories/?format=json";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "usgs_geomag_stations_compiler: {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn member<'a>(root: &'a JsonVal, key: &str) -> Option<&'a JsonVal> {
    match root {
        JsonVal::Obj(map) => map.get(key),
        _ => None,
    }
}

fn text<'a>(root: &'a JsonVal, key: &str) -> Option<&'a str> {
    match member(root, key) {
        Some(JsonVal::Str(s)) => Some(s.as_str()),
        _ => None,
    }
}

fn deg(v: f64) -> String {
    let rounded = (v * 1e9).round() / 1e9;
    format!("{rounded}")
}

struct Station {
    id: String,
    name: String,
    lat: f64,
    lon: f64,
}

fn stations(root: &JsonVal) -> Option<Vec<Station>> {
    let JsonVal::Arr(features) = member(root, "features")? else {
        return None;
    };
    let mut out = Vec::new();
    for feature in features {
        let Some(id) = text(feature, "id") else {
            continue;
        };
        let Some(geometry) = member(feature, "geometry") else {
            continue;
        };
        let Some(JsonVal::Arr(coords)) = member(geometry, "coordinates") else {
            continue;
        };
        let (Some(JsonVal::Num(lon)), Some(JsonVal::Num(lat))) = (coords.first(), coords.get(1))
        else {
            continue;
        };
        if !lat.is_finite() || !lon.is_finite() || !(-90.0..=90.0).contains(lat) {
            continue;
        }
        let Some(properties) = member(feature, "properties") else {
            continue;
        };
        let Some(name) = text(properties, "name") else {
            continue;
        };
        let name: String = name.split_whitespace().collect::<Vec<&str>>().join("_");
        if name.is_empty() || id.is_empty() {
            continue;
        }
        out.push(Station {
            id: id.to_string(),
            name,
            lat: *lat,
            lon: ((*lon + 180.0).rem_euclid(360.0)) - 180.0,
        });
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Some(out)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/observatories.txt"),
    };

    let body = curl(URL).ok_or_else(|| format!("{URL}: fetch void"))?;
    let root = parse_json(&body).ok_or_else(|| format!("{URL}: response parse void"))?;
    let Some(rows) = stations(&root) else {
        return Err(format!("{URL}: response carries no features array"));
    };
    if rows.is_empty() {
        println!(
            "usgs_geomag_stations_compiler: {URL} carries no station with a measured position — the asset stays unwritten (0 honored)"
        );
        return Ok(());
    }

    let mut text = format!(
        "# usgs geomag observatories | id name lat lon | lon east-positive in [-180,180) | source {URL}\n"
    );
    for s in &rows {
        text.push_str(&format!(
            "{} {} {} {}\n",
            s.id,
            s.name,
            deg(s.lat),
            deg(s.lon)
        ));
    }

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, text.as_bytes()).map_err(|e| format!("write {out} returned {e}"))?;

    eprintln!(
        "usgs_geomag_stations_compiler: {} stations written to {out} ({} B)",
        rows.len(),
        text.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("usgs_geomag_stations_compiler: {msg}");
        std::process::exit(2);
    }
}
