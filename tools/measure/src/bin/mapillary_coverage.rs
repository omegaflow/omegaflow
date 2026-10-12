use std::env;
use std::path::PathBuf;
use std::process::Command;

const ENDPOINT: &str = "https://graph.mapillary.com/images";
const FIELDS: &str = "id,captured_at,compass_angle,geometry,thumb_1024_url";
const TOKEN_KEY: &str = "MAPILLARY_ACCESS_TOKEN";
const DEFAULT_LIMIT: usize = 100;

struct Image {
    id: String,
    lat: f64,
    lon: f64,
    captured: i64,
    compass: Option<f64>,
    thumb: String,
}

fn repo_root() -> Option<PathBuf> {
    if let Ok(root) = env::var("OMEGAFLOW_REPO") {
        let p = PathBuf::from(root);
        if p.join(".secrets.local").is_file() {
            return Some(p);
        }
    }
    let mut dir = env::current_dir().ok()?;
    loop {
        if dir.join(".secrets.local").is_file() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

fn secrets_value(contents: &str, name: &str) -> Option<String> {
    for line in contents.lines() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == name {
                let v = v.trim();
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        }
    }
    None
}

fn read_token() -> Option<String> {
    let root = repo_root()?;
    let contents = std::fs::read_to_string(root.join(".secrets.local")).ok()?;
    secrets_value(&contents, TOKEN_KEY)
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_pair(s: &str) -> Option<(f64, f64)> {
    let (a, b) = s.split_once(',')?;
    Some((a.trim().parse().ok()?, b.trim().parse().ok()?))
}

fn parse_quad(s: &str) -> Option<(f64, f64, f64, f64)> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 4 {
        return None;
    }
    let minlon: f64 = parts[0].trim().parse().ok()?;
    let minlat: f64 = parts[1].trim().parse().ok()?;
    let maxlon: f64 = parts[2].trim().parse().ok()?;
    let maxlat: f64 = parts[3].trim().parse().ok()?;
    Some((minlon, minlat, maxlon, maxlat))
}

fn bbox_from_span(lat: f64, lon: f64, span_deg: f64) -> (f64, f64, f64, f64) {
    let dlat = span_deg;
    let dlon = span_deg / lat.to_radians().cos();
    (lon - dlon, lat - dlat, lon + dlon, lat + dlat)
}

fn fetch(url: &str) -> Result<String, String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("--max-time")
        .arg("60")
        .arg("-w")
        .arg("\n%{http_code}")
        .arg(url)
        .output();
    let out = match out {
        Ok(o) => o,
        Err(e) => return Err(format!("curl spawn {e}")),
    };
    let raw = String::from_utf8_lossy(&out.stdout).into_owned();
    let (body, code) = match raw.rfind('\n') {
        Some(i) => (raw[..i].to_string(), raw[i + 1..].trim().to_string()),
        None => (String::new(), String::new()),
    };
    if code.starts_with('2') {
        Ok(body)
    } else if code.is_empty() {
        Err(format!("curl transport {:?}", out.status.code()))
    } else {
        Err(format!("HTTP {code}"))
    }
}

fn balanced(after: &str, open: char, close: char) -> Option<&str> {
    if !after.starts_with(open) {
        return None;
    }
    let mut depth = 0i32;
    let mut in_str = false;
    let mut escaped = false;
    for (i, c) in after.char_indices() {
        if in_str {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_str = false;
            }
            continue;
        }
        if c == '"' {
            in_str = true;
        } else if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some(&after[..i + 1]);
            }
        }
    }
    None
}

fn array_after<'a>(hay: &'a str, key: &str) -> Option<&'a str> {
    let marker = format!("\"{}\"", key);
    let pos = hay.find(&marker)?;
    let rest = &hay[pos + marker.len()..];
    let colon = rest.find(':')?;
    balanced(rest[colon + 1..].trim_start(), '[', ']')
}

fn object_after<'a>(hay: &'a str, key: &str) -> Option<&'a str> {
    let marker = format!("\"{}\"", key);
    let mut search = 0;
    while let Some(rel) = hay[search..].find(&marker) {
        let pos = search + rel;
        let rest = &hay[pos + marker.len()..];
        let after = rest.trim_start();
        if let Some(colon) = after.strip_prefix(':') {
            if let Some(block) = balanced(colon.trim_start(), '{', '}') {
                return Some(block);
            }
        }
        search = pos + marker.len();
    }
    None
}

fn string_after(hay: &str, key: &str) -> Option<String> {
    let marker = format!("\"{}\"", key);
    let pos = hay.find(&marker)?;
    let rest = &hay[pos + marker.len()..];
    let colon = rest.find(':')?;
    let after = rest[colon + 1..].trim_start();
    if !after.starts_with('"') {
        return None;
    }
    let mut out = String::new();
    let mut chars = after[1..].chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => {
                if let Some(n) = chars.next() {
                    out.push(n);
                }
            }
            c => out.push(c),
        }
    }
    None
}

fn number_after(hay: &str, key: &str) -> Option<f64> {
    let marker = format!("\"{}\"", key);
    let pos = hay.find(&marker)?;
    let rest = &hay[pos + marker.len()..];
    let colon = rest.find(':')?;
    let after = rest[colon + 1..].trim_start();
    let token: String = after
        .chars()
        .take_while(|c| {
            c.is_ascii_digit() || *c == '-' || *c == '+' || *c == '.' || *c == 'e' || *c == 'E'
        })
        .collect();
    token.parse::<f64>().ok()
}

fn int_after(hay: &str, key: &str) -> Option<i64> {
    let marker = format!("\"{}\"", key);
    let pos = hay.find(&marker)?;
    let rest = &hay[pos + marker.len()..];
    let colon = rest.find(':')?;
    let after = rest[colon + 1..].trim_start();
    let token: String = after
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '-')
        .collect();
    token.parse::<i64>().ok()
}

fn top_objects(array: &str) -> Vec<&str> {
    let inner = &array[1..array.len() - 1];
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut in_str = false;
    let mut escaped = false;
    let mut start: Option<usize> = None;
    for (i, c) in inner.char_indices() {
        if in_str {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_str = false;
            }
            continue;
        }
        if c == '"' {
            in_str = true;
        } else if c == '{' {
            if depth == 0 {
                start = Some(i);
            }
            depth += 1;
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                if let Some(s) = start {
                    out.push(&inner[s..i + 1]);
                }
                start = None;
            }
        }
    }
    out
}

fn coordinates(object: &str) -> Option<(f64, f64)> {
    let geometry = object_after(object, "geometry")?;
    let coords = array_after(geometry, "coordinates")?;
    let inner = coords.strip_prefix('[')?.split(']').next()?;
    let mut it = inner.split(',');
    let lon: f64 = it.next()?.trim().parse().ok()?;
    let lat: f64 = it.next()?.trim().parse().ok()?;
    Some((lat, lon))
}

fn image_from(object: &str) -> Option<Image> {
    let id = match string_after(object, "id") {
        Some(v) => v,
        None => int_after(object, "id")?.to_string(),
    };
    let (lat, lon) = coordinates(object)?;
    let captured = int_after(object, "captured_at")?;
    let compass = number_after(object, "compass_angle");
    let thumb = string_after(object, "thumb_1024_url")?;
    Some(Image {
        id,
        lat,
        lon,
        captured,
        compass,
        thumb,
    })
}

fn images_in(body: &str) -> Vec<Image> {
    let Some(data) = array_after(body, "data") else {
        return Vec::new();
    };
    top_objects(data)
        .into_iter()
        .filter_map(image_from)
        .collect()
}

fn next_page(body: &str) -> Option<String> {
    let paging = object_after(body, "paging")?;
    string_after(paging, "next")
}

fn angular_separation_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let p1 = lat1.to_radians();
    let p2 = lat2.to_radians();
    let dp = (lat2 - lat1).to_radians();
    let dl = (lon2 - lon1).to_radians();
    let a = (dp / 2.0).sin().powi(2) + p1.cos() * p2.cos() * (dl / 2.0).sin().powi(2);
    let a = a.min(1.0);
    (2.0 * a.sqrt().asin()).to_degrees()
}

fn usage() -> String {
    "usage: mapillary_coverage --bbox <minlon,minlat,maxlon,maxlat> [--limit <n>] | --at <lat,lon> --span-deg <d> [--limit <n>]".to_string()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let at = match arg_value(&args, "--at") {
        Some(v) => match parse_pair(&v) {
            Some(p) => Some(p),
            None => {
                eprintln!("--at carries no lat,lon");
                std::process::exit(2);
            }
        },
        None => None,
    };
    let span_deg: Option<f64> = match arg_value(&args, "--span-deg") {
        Some(v) => match v.parse() {
            Ok(n) => Some(n),
            Err(_) => {
                eprintln!("--span-deg carries no number");
                std::process::exit(2);
            }
        },
        None => None,
    };
    let bbox = match arg_value(&args, "--bbox") {
        Some(v) => match parse_quad(&v) {
            Some(q) => Some(q),
            None => {
                eprintln!("--bbox carries no minlon,minlat,maxlon,maxlat");
                std::process::exit(2);
            }
        },
        None => None,
    };
    let limit: usize = match arg_value(&args, "--limit") {
        Some(v) => match v.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("--limit carries no number");
                std::process::exit(2);
            }
        },
        None => DEFAULT_LIMIT,
    };

    let bbox = match (bbox, at, span_deg) {
        (Some(q), _, _) => q,
        (None, Some((lat, lon)), Some(d)) => bbox_from_span(lat, lon, d),
        (None, Some(_), None) => {
            eprintln!("--at needs --span-deg");
            std::process::exit(2);
        }
        (None, None, _) => {
            eprintln!("{}", usage());
            std::process::exit(2);
        }
    };

    let token = match read_token() {
        Some(t) => t,
        None => {
            eprintln!("{TOKEN_KEY} absent");
            std::process::exit(2);
        }
    };

    let bbox_word = format!("{},{},{},{}", bbox.0, bbox.1, bbox.2, bbox.3);
    let mut url =
        format!("{ENDPOINT}?access_token={token}&fields={FIELDS}&bbox={bbox_word}&limit={limit}");

    let mut images: Vec<Image> = Vec::new();
    let mut pages = 0usize;
    loop {
        let body = match fetch(&url) {
            Ok(b) => b,
            Err(status) => {
                eprintln!("{status}");
                std::process::exit(2);
            }
        };
        images.extend(images_in(&body));
        pages += 1;
        if images.len() >= limit {
            break;
        }
        match next_page(&body) {
            Some(n) if !n.is_empty() && pages < 64 => url = n,
            _ => break,
        }
    }
    if images.len() > limit {
        images.truncate(limit);
    }

    println!("images {}", images.len());
    for img in &images {
        let compass = match img.compass {
            Some(c) => format!("{c}"),
            None => "absent".to_string(),
        };
        println!(
            "id {} at {:.6} {:.6} captured {} compass {} thumb {}",
            img.id, img.lat, img.lon, img.captured, compass, img.thumb
        );
    }

    if let Some((lat, lon)) = at {
        let mut best: Option<(&Image, f64)> = None;
        for img in &images {
            let d = angular_separation_deg(lat, lon, img.lat, img.lon);
            if best.map(|(_, bd)| d < bd).unwrap_or(true) {
                best = Some((img, d));
            }
        }
        match best {
            Some((img, d)) => println!(
                "nearest {} {:.6} {:.6} d={:.4} deg",
                img.id, img.lat, img.lon, d
            ),
            None => println!("nearest absent"),
        }
    }
}
