use std::env;
use std::fs;
use std::process::Command;

enum Json {
    Obj(Vec<(String, Json)>),
    Arr(Vec<Json>),
    Str(String),
    Num(f64),
    Null,
    Bool(bool),
}

fn skip_ws(b: &[u8], i: &mut usize) {
    while *i < b.len() && (b[*i] == b' ' || b[*i] == b'\n' || b[*i] == b'\t' || b[*i] == b'\r') {
        *i += 1;
    }
}

fn parse_str(b: &[u8], i: &mut usize) -> Option<String> {
    *i += 1;
    let mut out = String::new();
    loop {
        match b.get(*i)? {
            b'"' => {
                *i += 1;
                return Some(out);
            }
            b'\\' => {
                *i += 1;
                match b.get(*i)? {
                    b'n' => out.push('\n'),
                    b't' => out.push('\t'),
                    b'r' => out.push('\r'),
                    b'\\' => out.push('\\'),
                    b'"' => out.push('"'),
                    b'/' => out.push('/'),
                    b'u' => {
                        let hex = std::str::from_utf8(&b[*i + 1..*i + 5]).ok()?;
                        let c = u32::from_str_radix(hex, 16).ok()?;
                        out.push(char::from_u32(c)?);
                        *i += 4;
                    }
                    _ => return None,
                }
                *i += 1;
            }
            c => {
                out.push(*c as char);
                *i += 1;
            }
        }
    }
}

fn parse_num(b: &[u8], i: &mut usize) -> Option<f64> {
    let start = *i;
    while *i < b.len()
        && (b[*i].is_ascii_digit()
            || b[*i] == b'-'
            || b[*i] == b'.'
            || b[*i] == b'e'
            || b[*i] == b'E'
            || b[*i] == b'+')
    {
        *i += 1;
    }
    std::str::from_utf8(&b[start..*i]).ok()?.parse().ok()
}

fn parse_value(b: &[u8], i: &mut usize) -> Option<Json> {
    skip_ws(b, i);
    match b.get(*i)? {
        b'{' => parse_obj(b, i),
        b'[' => parse_arr(b, i),
        b'"' => parse_str(b, i).map(Json::Str),
        b't' => {
            *i += 4;
            Some(Json::Bool(true))
        }
        b'f' => {
            *i += 5;
            Some(Json::Bool(false))
        }
        b'n' => {
            *i += 4;
            Some(Json::Null)
        }
        _ => parse_num(b, i).map(Json::Num),
    }
}

fn parse_obj(b: &[u8], i: &mut usize) -> Option<Json> {
    *i += 1;
    let mut map = Vec::new();
    skip_ws(b, i);
    if b.get(*i) == Some(&b'}') {
        *i += 1;
        return Some(Json::Obj(map));
    }
    loop {
        skip_ws(b, i);
        let key = parse_str(b, i)?;
        skip_ws(b, i);
        if b.get(*i) != Some(&b':') {
            return None;
        }
        *i += 1;
        let val = parse_value(b, i)?;
        map.push((key, val));
        skip_ws(b, i);
        match b.get(*i) {
            Some(b',') => *i += 1,
            Some(b'}') => {
                *i += 1;
                return Some(Json::Obj(map));
            }
            _ => return None,
        }
    }
}

fn parse_arr(b: &[u8], i: &mut usize) -> Option<Json> {
    *i += 1;
    let mut arr = Vec::new();
    skip_ws(b, i);
    if b.get(*i) == Some(&b']') {
        *i += 1;
        return Some(Json::Arr(arr));
    }
    loop {
        arr.push(parse_value(b, i)?);
        skip_ws(b, i);
        match b.get(*i) {
            Some(b',') => *i += 1,
            Some(b']') => {
                *i += 1;
                return Some(Json::Arr(arr));
            }
            _ => return None,
        }
    }
}

fn parse_json(s: &str) -> Option<Json> {
    let mut i = 0;
    parse_value(s.trim().as_bytes(), &mut i)
}

fn jpath<'a>(j: &'a Json, path: &str) -> Option<&'a Json> {
    let mut cur = j;
    for seg in path.split('.') {
        if seg.is_empty() {
            continue;
        }
        cur = match cur {
            Json::Obj(m) => m.iter().find(|(k, _)| k == seg).map(|(_, v)| v)?,
            Json::Arr(a) => {
                let idx: usize = seg.parse().ok()?;
                a.get(idx)?
            }
            _ => return None,
        };
    }
    Some(cur)
}

fn scalar(j: &Json) -> String {
    match j {
        Json::Str(s) => s.clone(),
        Json::Num(n) => format!("{}", n),
        Json::Bool(b) => format!("{}", b),
        _ => String::new(),
    }
}

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut root: Option<String> = None;
    let mut items: Option<String> = None;
    let mut id_field: Option<String> = None;
    let mut title_field: Option<String> = None;
    let mut out: Option<String> = None;
    let mut pages: usize = 100;
    let mut size: usize = 100;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                root = args.get(i + 1).cloned();
                i += 1;
            }
            "--items" => {
                items = args.get(i + 1).cloned();
                i += 1;
            }
            "--id" => {
                id_field = args.get(i + 1).cloned();
                i += 1;
            }
            "--title" => {
                title_field = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--pages" => {
                pages = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(100);
                i += 1;
            }
            "--size" => {
                size = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(100);
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let (Some(root), Some(items), Some(id_field), Some(title_field)) =
        (root, items, id_field, title_field)
    else {
        eprintln!("--root/--items/--id/--title absent");
        std::process::exit(1);
    };
    let mut buf = String::new();
    let mut total = 0usize;
    for p in 0..pages {
        let url = root
            .replace("{page}", &(p + 1).to_string())
            .replace("{size}", &size.to_string());
        let Some(body) = curl(&url) else {
            break;
        };
        let Some(parsed) = parse_json(&body) else {
            break;
        };
        let Some(items_json) = jpath(&parsed, &items) else {
            break;
        };
        let Json::Arr(arr) = items_json else {
            break;
        };
        let mut gained = 0usize;
        for item in arr {
            let id = jpath(item, &id_field).map(scalar).unwrap_or_default();
            let title = jpath(item, &title_field).map(scalar).unwrap_or_default();
            if id.is_empty() {
                continue;
            }
            buf.push_str(&format!("{} | {}\n", id, title));
            gained += 1;
        }
        total += gained;
        if gained == 0 {
            break;
        }
    }
    if let Some(path) = out {
        let _ = fs::write(&path, &buf);
        eprintln!("rest: {} records → {}", total, path);
    } else {
        eprintln!("rest: {} records (--out absent)", total);
    }
}
