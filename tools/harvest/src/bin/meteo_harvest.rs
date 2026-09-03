use omegaflow::cdn::upload_release;
use std::process::Command;

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

fn json_str(x: &str) -> String {
    let mut s = String::from("\"");
    for c in x.chars() {
        match c {
            '"' => s.push_str("\\\""),
            '\\' => s.push_str("\\\\"),
            _ => s.push(c),
        }
    }
    s.push('"');
    s
}

fn format_f64(x: f64) -> String {
    if x.is_nan() {
        return "null".to_string();
    }
    if x == x.floor() && x.abs() < 1e15 {
        format!("{x:.0}")
    } else {
        format!("{x}")
    }
}

fn unix_of_datetime_utc(dt: &str) -> f64 {
    let parts: Vec<&str> = dt.split(['-', 'T', ':', ' ']).collect();
    if parts.len() < 5 {
        return 0.0;
    }
    let y: i64 = parts[0].parse().unwrap_or(0);
    let mo: i64 = parts[1].parse().unwrap_or(0);
    let d: i64 = parts[2].parse().unwrap_or(0);
    let h: i64 = parts[3].parse().unwrap_or(0);
    let mi: i64 = parts[4].parse().unwrap_or(0);
    let mut days = 0;
    let mut yy = 1970;
    while yy < y {
        days += if is_leap(yy) { 366 } else { 365 };
        yy += 1;
    }
    let mdays = [
        31,
        if is_leap(y) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    for i in 0..(mo - 1) as usize {
        days += mdays[i];
    }
    days += d - 1;
    (days as f64) * 86400.0 + (h as f64) * 3600.0 + (mi as f64) * 60.0
}

fn is_leap(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn json_string_array(body: &str, key: &str) -> Vec<String> {
    let mut out = Vec::new();
    let pat = format!("\"{key}\":");

    let mut last_start = 0usize;
    let mut idx = 0;
    while let Some(rel) = body[idx..].find(&pat) {
        last_start = idx + rel;
        idx = last_start + pat.len();
    }
    let rest = &body[last_start + pat.len()..];
    let rest = rest.trim_start();
    if rest.starts_with('[') {
        let mut depth = 0;
        let mut i = 0;
        for (j, ch) in rest.char_indices() {
            if ch == '[' {
                depth += 1;
            } else if ch == ']' {
                depth -= 1;
                if depth == 0 {
                    i = j + 1;
                    break;
                }
            }
        }
        let raw = &rest[..i];
        for item in raw.split(',') {
            let t = item
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim_matches('"');
            if !t.is_empty() {
                out.push(t.to_string());
            }
        }
    }
    out
}

fn json_value_by_key(body: &str, key: &str) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let pat = format!("\"{key}\":");
    let mut last_start = 0usize;
    let mut idx = 0;
    while let Some(rel) = body[idx..].find(&pat) {
        last_start = idx + rel;
        idx = last_start + pat.len();
    }
    let rest = &body[last_start + pat.len()..];
    let rest = rest.trim_start();
    if rest.starts_with('[') {
        let mut depth = 0;
        let mut i = 0;
        for (j, ch) in rest.char_indices() {
            if ch == '[' {
                depth += 1;
            } else if ch == ']' {
                depth -= 1;
                if depth == 0 {
                    i = j + 1;
                    break;
                }
            }
        }
        let raw = &rest[..i];
        for num in split_numbers(raw) {
            out.push(num.to_string());
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

fn split_numbers(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == 'e' || ch == 'E' || ch == '+' {
            cur.push(ch);
        } else if !cur.is_empty() {
            if cur.parse::<f64>().is_ok() {
                out.push(cur.clone());
            }
            cur.clear();
        }
    }
    if !cur.is_empty() && cur.parse::<f64>().is_ok() {
        out.push(cur);
    }
    out
}

struct Event {
    id: String,
    source: String,
    cdn: String,
    window_start: String,
    window_end: String,
    stations: Vec<Station>,
}

const HOURLY_KATALOG: &[&str] = &[
    "temperature_2m",
    "relative_humidity_2m",
    "dew_point_2m",
    "apparent_temperature",
    "precipitation_probability",
    "precipitation",
    "rain",
    "showers",
    "snowfall",
    "snow_depth",
    "freezing_level_height",
    "weather_code",
    "pressure_msl",
    "surface_pressure",
    "cloud_cover",
    "cloud_cover_low",
    "cloud_cover_mid",
    "cloud_cover_high",
    "wind_speed_10m",
    "wind_speed_80m",
    "wind_speed_120m",
    "wind_speed_180m",
    "wind_direction_10m",
    "wind_direction_80m",
    "wind_direction_120m",
    "wind_direction_180m",
    "wind_gusts_10m",
    "shortwave_radiation",
    "direct_radiation",
    "diffuse_radiation",
    "direct_normal_irradiance",
    "global_tilted_irradiance",
    "vapour_pressure_deficit",
    "et0_fao_evapotranspiration",
    "evapotranspiration",
    "surface_temperature",
    "soil_temperature_0cm",
    "soil_temperature_6cm",
    "soil_temperature_18cm",
    "soil_temperature_54cm",
    "soil_moisture_0_1cm",
    "soil_moisture_1_3cm",
    "soil_moisture_3_9cm",
    "soil_moisture_9_27cm",
    "soil_moisture_27_81cm",
    "is_day",
    "wet_bulb_temperature_2m",
    "total_column_integrated_water_vapour",
    "snowfall_water_equivalent",
    "leaf_wetness_probability",
    "sunshine_duration",
];

struct Station {
    id: String,
    lat: f64,
    lon: f64,
}

fn json_field(body: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\":");
    let start = body.find(&pat)? + pat.len();
    let rest = &body[start..];
    let trimmed = rest.trim_start();
    if trimmed.starts_with('"') {
        let end = trimmed[1..].find('"')? + 1;
        return Some(trimmed[1..end].to_string());
    }
    if trimmed.starts_with('[') {
        let close = trimmed.find(']')?;
        let inner = &trimmed[1..close];

        let a = inner
            .split(',')
            .next()
            .unwrap_or("")
            .trim()
            .trim_matches('"');
        return Some(a.to_string());
    }
    None
}

fn station_fields(body: &str) -> Vec<(String, f64, f64)> {
    let mut out = Vec::new();
    let sp = "\"stations\":";
    let start = match body.find(sp) {
        Some(s) => s + sp.len(),
        None => return out,
    };
    let rest = &body[start..];
    let rest = rest.trim_start();
    if !rest.starts_with('[') {
        return out;
    }

    let mut depth = 0;
    let mut i = 0;
    for (j, ch) in rest.char_indices() {
        if ch == '[' {
            depth += 1;
        } else if ch == ']' {
            depth -= 1;
            if depth == 0 {
                i = j + 1;
                break;
            }
        }
    }
    let arr = &rest[..i];
    let pat = "\"id\":";
    let mut idx = 0;
    while let Some(rel) = arr[idx..].find(pat) {
        let pos = idx + rel + pat.len();
        let trimmed = arr[pos..].trim_start();
        if trimmed.starts_with('"') {
            let end = trimmed[1..].find('"').map(|e| e + 1).unwrap_or(0);
            let id = trimmed[1..end].to_string();
            let seg = &arr[pos + end..];
            let lat = json_number(seg, "lat").unwrap_or(f64::NAN);
            let lon = json_number(seg, "lon").unwrap_or(f64::NAN);
            out.push((id, lat, lon));
        }
        idx = pos + 1;
    }
    out
}

fn json_number(body: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{key}\":");
    let start = body.find(&pat)? + pat.len();
    let rest = &body[start..];
    let trimmed = rest.trim_start();
    let mut num = String::new();
    for ch in trimmed.chars() {
        if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == 'e' || ch == 'E' || ch == '+' {
            num.push(ch);
        } else {
            break;
        }
    }
    num.parse().ok()
}

fn parse_event(path: &str) -> Option<Event> {
    let body = std::fs::read_to_string(path).ok()?;
    let window = json_string_array(&body, "window");
    let mut window_start = String::new();
    let mut window_end = String::new();
    if window.len() >= 2 {
        window_start = window[0].clone();
        window_end = window[1].clone();
    }
    let stations = station_fields(&body);
    if window_start.is_empty() || stations.is_empty() {
        return None;
    }
    Some(Event {
        id: json_field(&body, "id").unwrap_or_else(|| "event".into()),
        source: json_field(&body, "source").unwrap_or_else(|| "unknown".into()),
        cdn: json_field(&body, "cdn").unwrap_or_else(|| "archive-api.open-meteo.com".into()),
        window_start,
        window_end,
        stations: stations
            .into_iter()
            .map(|(id, lat, lon)| Station { id, lat, lon })
            .collect(),
    })
}

fn open_meteo_url(e: &Event, lat: f64, lon: f64) -> String {
    let variables = HOURLY_KATALOG.join(",");
    format!(
        "https://archive-api.open-meteo.com/v1/archive?latitude={lat}&longitude={lon}&start_date={}&end_date={}&hourly={variables}&timezone=UTC",
        e.window_start, e.window_end
    )
}

struct Series {
    station: String,
    variable: String,
    t: Vec<f64>,
    v: Vec<f64>,
    pending: Option<String>,
}

impl Series {
    fn to_json(&self, e: &Event) -> String {
        let mut s = String::from("{\"source\":");
        s.push_str(&json_str(&e.source));
        s.push_str(",\"event\":");
        s.push_str(&json_str(&e.id));
        s.push_str(",\"station\":");
        s.push_str(&json_str(&self.station));
        s.push_str(",\"variable\":");
        s.push_str(&json_str(&self.variable));
        s.push_str(",\"window\":[");
        s.push_str(&json_str(&e.window_start));
        s.push(',');
        s.push_str(&json_str(&e.window_end));
        s.push_str("],\"n\":");
        s.push_str(&self.t.len().to_string());
        s.push_str(",\"points\":[");
        for i in 0..self.t.len() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(&format!(
                "{{\"t\":{:.1},\"v\":{}}}",
                self.t[i],
                format_f64(self.v[i])
            ));
        }
        s.push(']');
        if let Some(p) = &self.pending {
            s.push_str(",\"pending\":");
            s.push_str(&json_str(p));
        }
        s.push('}');
        s
    }
}

fn harvest_open_meteo(e: &Event, station: &Station) -> Vec<Series> {
    let url = open_meteo_url(e, station.lat, station.lon);
    let body = match curl(&url) {
        Some(b) => b,
        None => {
            return HOURLY_KATALOG
                .iter()
                .map(|v| Series {
                    station: station.id.clone(),
                    variable: v.to_string(),
                    t: vec![],
                    v: vec![],
                    pending: Some("open-meteo fetch failed".into()),
                })
                .collect();
        }
    };
    let times = json_string_array(&body, "time");
    let mut series = Vec::new();
    for v in HOURLY_KATALOG {
        let vals = json_value_by_key(&body, v).unwrap_or_default();
        let mut t = Vec::new();
        let mut vs = Vec::new();
        if times.is_empty() || vals.is_empty() {
            series.push(Series {
                station: station.id.clone(),
                variable: v.to_string(),
                t,
                v: vs,
                pending: Some("open-meteo delivered no series".into()),
            });
            continue;
        }
        for i in 0..times.len().min(vals.len()) {
            t.push(unix_of_datetime_utc(&times[i]));
            vs.push(vals[i].parse().unwrap_or(f64::NAN));
        }
        series.push(Series {
            station: station.id.clone(),
            variable: v.to_string(),
            t,
            v: vs,
            pending: None,
        });
    }
    series
}

fn write_series(path: &str, s: &Series, e: &Event) {
    if std::fs::write(path, s.to_json(e)).is_err() {
        eprintln!("write {path} returned void");
        std::process::exit(1);
    }
}

fn main() {
    let mut ci_mode = false;
    let mut out_dir = String::new();
    let mut event_path = String::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--ci-mode" => ci_mode = true,
            "--out" => out_dir = args.next().unwrap_or(out_dir),
            "--event" => event_path = args.next().unwrap_or(event_path),
            _ => {}
        }
    }
    if event_path.is_empty() {
        eprintln!("--event <event.json> absent");
        std::process::exit(2);
    }
    let Some(e) = parse_event(&event_path) else {
        eprintln!("event json {event_path} not readable");
        std::process::exit(2);
    };
    if out_dir.is_empty() {
        out_dir = format!("phi/pipeline/meteo_harvest/{}", e.id);
    }
    let _ = std::fs::create_dir_all(&out_dir);

    for station in &e.stations {
        for s in harvest_open_meteo(&e, station) {
            let fname = format!(
                "{}/{}_{}_{}.json",
                out_dir, station.id, "open-meteo", s.variable
            );
            write_series(&fname, &s, &e);
            eprintln!(
                "{fname}: n = {} {}",
                s.t.len(),
                s.pending.as_deref().unwrap_or("measured")
            );
        }
    }

    if ci_mode {
        for f in std::fs::read_dir(&out_dir).expect("read out_dir") {
            let p = f.unwrap().path();
            let pstr = p.to_string_lossy().to_string();
            eprintln!("upload {pstr} -> cdn tag {}", e.cdn);
            if !upload_release(&e.cdn, &pstr) {
                eprintln!("upload failed: {pstr}");
                std::process::exit(1);
            }
        }
    }
}
