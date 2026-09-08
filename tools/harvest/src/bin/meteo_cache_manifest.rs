use omegaflow::archivar::naming::source_name_from_url;
use omegaflow::cdn::upload_release;
use omegaflow::json::{jnum, jpath_val, jstr, parse_json, JsonVal};
use std::process::Command;

const CDN_TAG: &str = "archive-api.open-meteo.com";

struct Event {
    cdn: String,
    window_start: String,
    window_end: String,
    stations: Vec<Station>,
    variables: Vec<String>,
}

struct Station {
    lat: f64,
    lon: f64,
}

fn event_from(path: &str) -> Option<Event> {
    let body = std::fs::read_to_string(path).ok()?;
    let j = parse_json(&body)?;
    let cdn = match jstr(&j, "cdn") {
        Some(c) => c,
        None => CDN_TAG.to_string(),
    };
    let JsonVal::Arr(window) = jpath_val(&j, "window")? else {
        return None;
    };
    if window.len() < 2 {
        return None;
    }
    let window_start = match &window[0] {
        JsonVal::Str(s) => s.clone(),
        _ => return None,
    };
    let window_end = match &window[1] {
        JsonVal::Str(s) => s.clone(),
        _ => return None,
    };
    let JsonVal::Arr(stations) = jpath_val(&j, "stations")? else {
        return None;
    };
    let mut sts = Vec::new();
    for st in stations {
        let Some(lat) = jnum(st, "lat") else {
            continue;
        };
        let Some(lon) = jnum(st, "lon") else {
            continue;
        };
        sts.push(Station { lat, lon });
    }
    if sts.is_empty() {
        return None;
    }
    let JsonVal::Arr(variables) = jpath_val(&j, "variables")? else {
        return None;
    };
    let mut vars = Vec::new();
    for v in variables {
        match v {
            JsonVal::Str(s) => vars.push(s.clone()),
            _ => return None,
        }
    }
    if vars.is_empty() {
        return None;
    }
    Some(Event {
        cdn,
        window_start,
        window_end,
        stations: sts,
        variables: vars,
    })
}

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("--silent")
        .arg("--location")
        .arg("--max-time")
        .arg("60")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!("curl {url}: {}", out.status);
        None
    }
}

fn curl(url: &str) -> Option<String> {
    Some(String::from_utf8(curl_bytes(url)?).ok()?)
}

fn open_meteo_url(lat: f64, lon: f64, variable: &str, ws: &str, we: &str) -> String {
    format!(
        "https://archive-api.open-meteo.com/v1/archive?latitude={lat}&longitude={lon}&start_date={ws}&end_date={we}&hourly={variable}&timezone=UTC"
    )
}

fn body_carries_series(body: &str, variable: &str) -> bool {
    let Some(j) = parse_json(body) else {
        return false;
    };
    let Some(series) = jpath_val(&j, &format!("hourly.{variable}")) else {
        return false;
    };
    matches!(series, JsonVal::Arr(a) if !a.is_empty())
}

fn main() {
    let mut ci_mode = false;
    let mut out_dir = String::from("phi/pipeline/meteo_harvest");
    let mut event_path = String::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--ci-mode" => ci_mode = true,
            "--out" => {
                if let Some(v) = args.next() {
                    out_dir = v;
                }
            }
            "--event" => {
                if let Some(v) = args.next() {
                    event_path = v;
                }
            }
            _ => {}
        }
    }
    if event_path.is_empty() {
        eprintln!("--event <event.json> absent");
        std::process::exit(2);
    }
    let Some(e) = event_from(&event_path) else {
        eprintln!("event json {event_path} not readable");
        std::process::exit(2);
    };
    let _ = std::fs::create_dir_all(&out_dir);
    let mut uploads: Vec<String> = Vec::new();
    for station in &e.stations {
        for variable in &e.variables {
            let url = open_meteo_url(
                station.lat,
                station.lon,
                variable,
                &e.window_start,
                &e.window_end,
            );
            let Some(body) = curl(&url) else {
                eprintln!("origin {url} void — cache asset absent");
                continue;
            };
            if !body_carries_series(&body, variable) {
                eprintln!("origin {url}: no hourly.{variable} series — cache asset absent");
                continue;
            }
            let name = source_name_from_url(&url);
            let fpath = format!("{}/{}.json", out_dir, name);
            if std::fs::write(&fpath, body.as_bytes()).is_err() {
                eprintln!("cache asset {fpath}: write void");
                std::process::exit(1);
            }
            eprintln!("cache asset {fpath}: origin verbatim, {} bytes", body.len());
            uploads.push(fpath);
        }
    }
    if ci_mode {
        for path in &uploads {
            eprintln!("upload {path} -> cdn tag {}", e.cdn);
            if !upload_release(&e.cdn, path) {
                eprintln!("upload {path} void");
                std::process::exit(1);
            }
        }
    }
}
