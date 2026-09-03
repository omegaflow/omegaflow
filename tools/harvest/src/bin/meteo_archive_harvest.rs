use omegaflow::cdn::upload_release;
use std::process::Command;

const CDN_RELEASE: &str = "archive-api.open-meteo.com";

const WINDOW_START: &str = "2026-08-18";
const WINDOW_END: &str = "2026-08-27";

struct Station {
    id: &'static str,
    lat: f64,
    lon: f64,
}

const STATIONS: [Station; 3] = [
    Station {
        id: "gyirong",
        lat: 28.8559,
        lon: 85.2950,
    },
    Station {
        id: "rasuwa",
        lat: 28.2500,
        lon: 85.1000,
    },
    Station {
        id: "kollab",
        lat: 28.2710,
        lon: 85.5150,
    },
];

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

fn open_meteo_url(lat: f64, lon: f64, variables: &str, ws: &str, we: &str) -> String {
    format!(
        "https://archive-api.open-meteo.com/v1/archive?latitude={lat}&longitude={lon}&start_date={ws}&end_date={we}&hourly={variables}&timezone=UTC"
    )
}

struct Series {
    station: &'static str,
    variable: &'static str,
    window_start: String,
    window_end: String,
    t: Vec<f64>,
    v: Vec<f64>,
    pending: Option<String>,
}

impl Series {
    fn to_json(&self) -> String {
        let mut s = String::from("{\"source\":\"open-meteo archive-api\",\"station\":");
        s.push_str(&json_str(self.station));
        s.push_str(",\"variable\":");
        s.push_str(&json_str(self.variable));
        s.push_str(",\"window\":[");
        s.push_str(&json_str(&self.window_start));
        s.push(',');
        s.push_str(&json_str(&self.window_end));
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

fn harvest_open_meteo(station: &Station, ws: &str, we: &str) -> Vec<Series> {
    let url = open_meteo_url(
        station.lat,
        station.lon,
        "temperature_2m,precipitation",
        ws,
        we,
    );
    let body = match curl(&url) {
        Some(b) => b,
        None => {
            return vec![
                Series {
                    station: station.id,
                    variable: "temperature_2m",
                    window_start: ws.to_string(),
                    window_end: we.to_string(),
                    t: vec![],
                    v: vec![],
                    pending: Some("open-meteo fetch failed".into()),
                },
                Series {
                    station: station.id,
                    variable: "precipitation",
                    window_start: ws.to_string(),
                    window_end: we.to_string(),
                    t: vec![],
                    v: vec![],
                    pending: Some("open-meteo fetch failed".into()),
                },
            ];
        }
    };
    let times = json_string_array(&body, "time");
    let temp = json_value_by_key(&body, "temperature_2m").unwrap_or_default();
    let prec = json_value_by_key(&body, "precipitation").unwrap_or_default();
    if times.is_empty() || temp.is_empty() {
        return vec![
            Series {
                station: station.id,
                variable: "temperature_2m",
                window_start: ws.to_string(),
                window_end: we.to_string(),
                t: vec![],
                v: vec![],
                pending: Some("open-meteo delivered no series".into()),
            },
            Series {
                station: station.id,
                variable: "precipitation",
                window_start: ws.to_string(),
                window_end: we.to_string(),
                t: vec![],
                v: vec![],
                pending: Some("open-meteo delivered no series".into()),
            },
        ];
    }
    let build = |var: &'static str, vals: &[String]| -> Series {
        let mut t = Vec::new();
        let mut v = Vec::new();
        for i in 0..times.len().min(vals.len()) {
            t.push(unix_of_datetime_utc(&times[i]));
            v.push(vals[i].parse().unwrap_or(f64::NAN));
        }
        Series {
            station: station.id,
            variable: var,
            window_start: ws.to_string(),
            window_end: we.to_string(),
            t,
            v,
            pending: None,
        }
    };
    vec![
        build("temperature_2m", &temp),
        build("precipitation", &prec),
    ]
}

fn main() {
    let mut ci_mode = false;
    let mut out_dir = String::from("phi/pipeline/collapse_harvest");
    let mut ws = String::from(WINDOW_START);
    let mut we = String::from(WINDOW_END);
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--ci-mode" => ci_mode = true,
            "--out" => out_dir = args.next().unwrap_or(out_dir),
            "--window-start" => ws = args.next().unwrap_or(ws),
            "--window-end" => we = args.next().unwrap_or(we),
            _ => {}
        }
    }

    let _ = std::fs::create_dir_all(&out_dir);

    for station in STATIONS.iter() {
        for s in harvest_open_meteo(station, &ws, &we) {
            let fname = format!(
                "{}/{}_{}_{}.json",
                out_dir, station.id, "open-meteo", s.variable
            );
            write_series(&fname, &s);
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
            eprintln!("upload {pstr} -> cdn tag {CDN_RELEASE}");
            if !upload_release(CDN_RELEASE, &pstr) {
                eprintln!("upload failed: {pstr}");
                std::process::exit(1);
            }
        }
    }
}

fn write_series(path: &str, s: &Series) {
    if std::fs::write(path, s.to_json()).is_err() {
        eprintln!("write {path} returned void");
        std::process::exit(1);
    }
}
