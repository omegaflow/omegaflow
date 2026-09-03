use omegaflow::json::{JsonVal, jpath_val, parse_json};
use std::env;
use std::fs;
use std::process::Command;

fn scalar(j: &JsonVal) -> String {
    match j {
        JsonVal::Str(s) => s.clone(),
        JsonVal::Num(n) => format!("{}", n),
        JsonVal::Bool(b) => format!("{}", b),
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

fn curl_post(url: &str, body: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("120")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(body)
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
    let mut post: Option<String> = None;
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
            "--post" => {
                post = args.get(i + 1).cloned();
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
        let body = match &post {
            Some(tmpl) => Some(
                tmpl.replace("{page}", &(p + 1).to_string())
                    .replace("{start}", &(p * size).to_string())
                    .replace("{size}", &size.to_string()),
            ),
            None => None,
        };
        let url = root
            .replace("{page}", &(p + 1).to_string())
            .replace("{size}", &size.to_string());
        let fetched = match &body {
            Some(b) => curl_post(&url, b),
            None => curl(&url),
        };
        let Some(resp) = fetched else {
            break;
        };
        let Some(parsed) = parse_json(&resp) else {
            break;
        };
        let Some(items_json) = jpath_val(&parsed, &items) else {
            break;
        };
        let JsonVal::Arr(arr) = items_json else {
            break;
        };
        let mut gained = 0usize;
        for item in arr {
            let id = jpath_val(item, &id_field).map(scalar).unwrap_or_default();
            let title = jpath_val(item, &title_field)
                .map(scalar)
                .unwrap_or_default();
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
