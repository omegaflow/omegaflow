use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json, scalar_of};
use omegaflow::lsk::days_from_civil;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "geomag.usgs.gov";
const BASE: &str = "https://geomag.usgs.gov/ws/data/";
const ELEMENTS: [&str; 3] = ["H", "D", "Z"];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("-m")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "usgs_geomag_hdz_compiler: {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn parse_iso_unix(s: &str) -> Option<i64> {
    let text = s.trim();
    let (date, clock) = text.split_once('T')?;
    let mut d = date.split('-');
    let year: i64 = d.next()?.parse().ok()?;
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let clock = clock.strip_suffix('Z').unwrap_or(clock);
    let mut t = clock.split(':');
    let hour: i64 = t.next()?.parse().ok()?;
    let minute: i64 = t.next()?.parse().ok()?;
    let second: i64 = if let Some(sec) = t.next() {
        sec.split('.').next()?.parse().ok()?
    } else {
        let base = hour * 3600 + minute * 60;
        return Some(days * 86400 + base);
    };
    Some(days * 86400 + hour * 3600 + minute * 60 + second)
}

fn member<'a>(root: &'a JsonVal, key: &str) -> Option<&'a JsonVal> {
    match root {
        JsonVal::Obj(map) => map.get(key),
        _ => None,
    }
}

fn element_of(entry: &JsonVal) -> Option<&str> {
    if let Some(JsonVal::Str(id)) = member(entry, "id") {
        return Some(id.as_str());
    }
    match member(entry, "metadata").and_then(|m| member(m, "element")) {
        Some(JsonVal::Str(e)) => Some(e.as_str()),
        _ => None,
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = match arg_value(args, "--station") {
        Some(s) => s,
        None => "FRD".to_string(),
    };
    if station.is_empty() {
        return Err("--station is empty".to_string());
    }
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/{station}_hdz.txt"),
    };
    let url =
        format!("{BASE}?id={station}&type=variation&format=json&elements=H,D,Z&sampling_period=60");

    let body = curl(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let root = parse_json(&body).ok_or_else(|| format!("{url}: response parse void"))?;

    let JsonVal::Arr(times) = member(&root, "times").ok_or("times: no array")? else {
        return Err("times: not an array".to_string());
    };
    let JsonVal::Arr(values) = member(&root, "values").ok_or("values: no array")? else {
        return Err("values: not an array".to_string());
    };

    let mut series: HashMap<&str, Vec<Option<f64>>> = HashMap::new();
    for entry in values {
        let Some(element) = element_of(entry) else {
            continue;
        };
        if !ELEMENTS.contains(&element) {
            continue;
        }
        let Some(JsonVal::Arr(column)) = member(entry, "values") else {
            continue;
        };
        series.insert(element, column.iter().map(scalar_of).collect());
    }
    for element in ELEMENTS {
        if !series.contains_key(element) {
            return Err(format!("{url}: element {element} is absent"));
        }
    }

    let mut rows: Vec<(i64, f64, f64, f64)> = Vec::new();
    for (i, stamp) in times.iter().enumerate() {
        let JsonVal::Str(stamp) = stamp else {
            continue;
        };
        let Some(unix) = parse_iso_unix(stamp) else {
            continue;
        };
        let mut triple = [0.0f64; 3];
        let mut complete = true;
        for (k, element) in ELEMENTS.iter().enumerate() {
            match series.get(element).and_then(|column| column.get(i)) {
                Some(Some(value)) if value.is_finite() => triple[k] = *value,
                _ => {
                    complete = false;
                    break;
                }
            }
        }
        if !complete {
            continue;
        }
        rows.push((unix, triple[0], triple[1], triple[2]));
    }
    if rows.is_empty() {
        return Err(format!(
            "{url} carries no complete H/D/Z row — the asset stays unwritten"
        ));
    }

    let mut text = format!("# USGS geomag {station} variation | unix H D Z | nT | source {url}\n");
    for (unix, h, d, z) in &rows {
        text.push_str(&format!("{unix} {h} {d} {z}\n"));
    }

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, text.as_bytes()).map_err(|e| format!("write {out} returned {e}"))?;

    eprintln!(
        "usgs_geomag_hdz_compiler: {station} {} rows ({} B) written to {out}",
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
        eprintln!("usgs_geomag_hdz_compiler: {msg}");
        std::process::exit(2);
    }
}
