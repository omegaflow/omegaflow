use omegaflow::json::{jpath_val, parse_json, JsonVal};
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

fn curl_sparql(endpoint: &str, query: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("120")
        .arg("-G")
        .arg(endpoint)
        .arg("--data-urlencode")
        .arg(format!("query={}", query))
        .arg("--data-urlencode")
        .arg("format=json")
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
    let mut endpoint: Option<String> = None;
    let mut query: Option<String> = None;
    let mut id_var = String::from("s");
    let mut title_var = String::from("title");
    let mut out: Option<String> = None;
    let mut pages: usize = 100;
    let mut size: usize = 1000;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--endpoint" => {
                endpoint = args.get(i + 1).cloned();
                i += 1;
            }
            "--query" => {
                query = args.get(i + 1).cloned();
                i += 1;
            }
            "--id" => {
                id_var = args.get(i + 1).cloned().unwrap_or(id_var);
                i += 1;
            }
            "--title" => {
                title_var = args.get(i + 1).cloned().unwrap_or(title_var);
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
                size = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1000);
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let (Some(endpoint), Some(query)) = (endpoint, query) else {
        eprintln!("--endpoint/--query absent");
        std::process::exit(1);
    };
    let mut buf = String::new();
    let mut total = 0usize;
    for p in 0..pages {
        let q = query
            .replace("{limit}", &size.to_string())
            .replace("{offset}", &(p * size).to_string());
        let Some(body) = curl_sparql(&endpoint, &q) else {
            break;
        };
        let Some(parsed) = parse_json(&body) else {
            break;
        };
        let Some(bindings) = jpath_val(&parsed, "results")
            .and_then(|r| jpath_val(r, "bindings"))
            .and_then(|b| match b {
                JsonVal::Arr(a) => Some(a),
                _ => None,
            })
        else {
            break;
        };
        let mut gained = 0usize;
        for binding in bindings {
            let get_val = |var: &str| -> String {
                jpath_val(binding, var)
                    .and_then(|v| jpath_val(v, "value"))
                    .map(scalar)
                    .unwrap_or_default()
            };
            let id = get_val(&id_var);
            let title = get_val(&title_var);
            if !id.is_empty() {
                buf.push_str(&format!("{} | {}\n", id, title));
                gained += 1;
            }
        }
        total += gained;
        if gained == 0 {
            break;
        }
    }
    if let Some(path) = out {
        let _ = fs::write(&path, &buf);
        eprintln!("sparql: {} records → {}", total, path);
    } else {
        eprintln!("sparql: {} records (--out absent)", total);
    }
}
