use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json};
use omegaflow::lsk::days_from_civil;
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "power.larc.nasa.gov";
const BASE: &str = "https://power.larc.nasa.gov/api/temporal/daily/point";

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
            "nasa_power_compiler: {url} returned ({}): {}",
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

fn parse_yyyymmdd(s: &str) -> Option<i64> {
    let text = s.trim();
    if text.len() != 8 {
        return None;
    }
    let year: i64 = text.get(0..4)?.parse().ok()?;
    let month: i64 = text.get(4..6)?.parse().ok()?;
    let day: i64 = text.get(6..8)?.parse().ok()?;
    days_from_civil(year, month, day).map(|days| days * 86400)
}

fn fill_value(root: &JsonVal) -> Option<f64> {
    let header = member(root, "header")?;
    match member(header, "fill_value")? {
        JsonVal::Num(v) => Some(*v),
        _ => None,
    }
}

fn units_of(root: &JsonVal, parameter: &str) -> Option<String> {
    let parameters = member(root, "parameters")?;
    let entry = member(parameters, parameter)?;
    match member(entry, "units")? {
        JsonVal::Str(units) => Some(units.clone()),
        _ => None,
    }
}

fn to_kelvin(value: f64, units: &str) -> Option<f64> {
    match units {
        "K" => Some(value),
        "C" => Some(value + 273.15),
        _ => None,
    }
}

fn rows(root: &JsonVal, parameter: &str, units: &str) -> Option<Vec<(i64, f64)>> {
    let properties = member(root, "properties")?;
    let parameters = member(properties, "parameter")?;
    let series = member(parameters, parameter)?;
    let JsonVal::Obj(map) = series else {
        return None;
    };
    let fill = fill_value(root);
    let mut out = Vec::new();
    for (stamp, v) in map.iter() {
        let Some(unix) = parse_yyyymmdd(stamp) else {
            continue;
        };
        let JsonVal::Num(value) = v else {
            continue;
        };
        if !value.is_finite() || fill == Some(*value) {
            continue;
        }
        let Some(value) = to_kelvin(*value, units) else {
            continue;
        };
        out.push((unix, value));
    }
    out.sort_by_key(|(unix, _)| *unix);
    Some(out)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let lat: f64 = arg_value(args, "--lat")
        .ok_or("--lat <deg> required")?
        .parse()
        .map_err(|_| "--lat is not a number".to_string())?;
    let lon: f64 = arg_value(args, "--lon")
        .ok_or("--lon <deg> required")?
        .parse()
        .map_err(|_| "--lon is not a number".to_string())?;
    if !lat.is_finite() || !lon.is_finite() {
        return Err("--lat/--lon are not finite".to_string());
    }
    let start = arg_value(args, "--start").ok_or("--start <YYYYMMDD> required")?;
    let end = arg_value(args, "--end").ok_or("--end <YYYYMMDD> required")?;
    let parameter = match arg_value(args, "--parameter") {
        Some(v) => v,
        None => "T2M".to_string(),
    };
    let url = format!(
        "{BASE}?parameters={parameter}&community=RE&longitude={lon}&latitude={lat}&start={start}&end={end}&format=JSON"
    );
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/nasa_power_{parameter}_{lat}_{lon}.txt"),
    };

    let body = curl(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let root = parse_json(&body).ok_or_else(|| format!("{url}: response parse void"))?;
    let Some(units) = units_of(&root, &parameter) else {
        println!(
            "nasa_power_compiler: {url} names no units for {parameter} — the asset stays unwritten (0 honored)"
        );
        return Ok(());
    };
    let Some(rows) = rows(&root, &parameter, &units) else {
        println!(
            "nasa_power_compiler: {url} carries no {parameter} series — the asset stays unwritten (0 honored)"
        );
        return Ok(());
    };
    if rows.is_empty() {
        println!(
            "nasa_power_compiler: {url} carries no measured {parameter} row — the asset stays unwritten (0 honored)"
        );
        return Ok(());
    }

    let mut text = format!(
        "# NASA POWER {parameter} | lat {lat} lon {lon} | native units {units}, carried as K | source {url}\n"
    );
    for (unix, value) in &rows {
        text.push_str(&format!("{unix} {value}\n"));
    }

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, text.as_bytes()).map_err(|e| format!("write {out} returned {e}"))?;

    eprintln!(
        "nasa_power_compiler: {} {parameter} rows written to {out}",
        rows.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("nasa_power_compiler: {msg}");
        std::process::exit(2);
    }
}
