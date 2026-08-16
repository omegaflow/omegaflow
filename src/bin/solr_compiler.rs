use std::env;
use std::fs;
use std::process::Command;

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

fn extract_field<'a>(body: &'a str, key: &str, from: &mut usize) -> Option<&'a str> {
    let needle = format!("\"{}\":\"", key);
    let pos = body[*from..].find(&needle)? + *from + needle.len();
    let end = body[pos..].find('"')? + pos;
    *from = end;
    Some(&body[pos..end])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut root: Option<String> = None;
    let mut out: Option<String> = None;
    let mut query: String = String::from("*:*");
    let mut pages: usize = 100;
    let mut rows: usize = 1000;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                root = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--q" => {
                query = args.get(i + 1).cloned().unwrap_or_default();
                i += 1;
            }
            "--pages" => {
                pages = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(100);
                i += 1;
            }
            "--rows" => {
                rows = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1000);
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let Some(root) = root else {
        eprintln!("--root absent");
        std::process::exit(1);
    };
    let mut buf = String::new();
    let mut total = 0usize;
    for p in 0..pages {
        let start = p * rows;
        let url = format!("{}?q={}&start={}&rows={}&wt=json", root, query, start, rows);
        let Some(body) = curl(&url) else {
            break;
        };
        let mut cursor = 0usize;
        let mut gained = 0usize;
        loop {
            let Some(acr) = extract_field(&body, "entry_acronym_s", &mut cursor) else {
                break;
            };
            let Some(name) = extract_field(&body, "entry_name_s", &mut cursor) else {
                break;
            };
            let Some(typ) = extract_field(&body, "entry_type_s", &mut cursor) else {
                break;
            };
            buf.push_str(&format!("{} | {} | {}\n", acr, name, typ));
            gained += 1;
        }
        total += gained;
        if gained == 0 {
            break;
        }
    }
    if let Some(path) = out {
        let _ = fs::write(&path, &buf);
        eprintln!("solr: {} records → {}", total, path);
    } else {
        eprintln!("solr: {} records (--out absent)", total);
    }
}
