use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json};
use omegaflow::lsk::days_from_civil;
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "flood-api.open-meteo.com";
const BASE: &str = "https://flood-api.open-meteo.com/v1/flood";

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
            "openmeteo_flood_compiler: {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn parse_iso_date(s: &str) -> Option<i64> {
    let mut d = s.trim().split('-');
    let year: i64 = d.next()?.parse().ok()?;
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    Some(days_from_civil(year, month, day)? * 86400)
}

fn member<'a>(root: &'a JsonVal, key: &str) -> Option<&'a JsonVal> {
    match root {
        JsonVal::Obj(map) => map.get(key),
        _ => None,
    }
}

fn series(root: &JsonVal, variable: &str) -> Option<Vec<(i64, f64)>> {
    let daily = member(root, "daily")?;
    let JsonVal::Arr(times) = member(daily, "time")? else {
        return None;
    };
    let JsonVal::Arr(values) = member(daily, variable)? else {
        return None;
    };
    let mut out = Vec::new();
    for (t, v) in times.iter().zip(values.iter()) {
        let JsonVal::Str(stamp) = t else {
            continue;
        };
        let JsonVal::Num(value) = v else {
            continue;
        };
        let Some(unix) = parse_iso_date(stamp) else {
            continue;
        };
        if value.is_finite() {
            out.push((unix, *value));
        }
    }
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
    let variable = match arg_value(args, "--variable") {
        Some(v) => v,
        None => "river_discharge".to_string(),
    };
    let forecast_days = match arg_value(args, "--forecast-days") {
        Some(v) => v,
        None => "92".to_string(),
    };
    let url = format!(
        "{BASE}?latitude={lat}&longitude={lon}&daily={variable}&forecast_days={forecast_days}&timezone=UTC"
    );
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/openmeteo_{variable}_{lat}_{lon}.txt"),
    };

    let body = curl(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let root = parse_json(&body).ok_or_else(|| format!("{url}: response parse void"))?;
    let Some(rows) = series(&root, &variable) else {
        println!(
            "openmeteo_flood_compiler: {url} carries no daily {variable} series — the asset stays unwritten (0 honored)"
        );
        return Ok(());
    };
    if rows.is_empty() {
        println!(
            "openmeteo_flood_compiler: {url} carries no measured {variable} row — the asset stays unwritten (0 honored)"
        );
        return Ok(());
    }

    let mut text = format!(
        "# open-meteo GloFAS {variable} | lat {lat} lon {lon} | value m3/s | source {url}\n"
    );
    for (t, v) in &rows {
        text.push_str(&format!("{t} {v}\n"));
    }

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, text.as_bytes()).map_err(|e| format!("write {out} returned {e}"))?;

    eprintln!(
        "openmeteo_flood_compiler: {} {variable} rows written to {out}",
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
        eprintln!("openmeteo_flood_compiler: {msg}");
        std::process::exit(2);
    }
}
