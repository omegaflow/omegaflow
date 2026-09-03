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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut root: Option<String> = None;
    let mut out: Option<String> = None;
    let mut pages: usize = 200;
    let mut per_page: usize = 100;
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
            "--pages" => {
                pages = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(200);
                i += 1;
            }
            "--per-page" => {
                per_page = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(100);
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
        let start = p * per_page;
        let url = format!(
            "{}/api/search?q=*&type=dataset&start={}&per_page={}",
            root.trim_end_matches('/'),
            start,
            per_page
        );
        let Some(body) = curl(&url) else {
            break;
        };
        let mut rest = body.as_str();
        let mut gained = 0usize;
        while let Some(pos) = rest.find("\"name\":\"") {
            let after = &rest[pos + 8..];
            let Some(name_end) = after.find('"') else {
                break;
            };
            let name = &after[..name_end];
            let tail = &after[name_end..];
            if !tail.starts_with("\",\"type\":\"dataset\"") {
                rest = tail;
                continue;
            }
            let Some(gp) = tail.find("\"global_id\":\"doi:") else {
                rest = tail;
                continue;
            };
            let g_after = &tail[gp + 17..];
            let Some(g_end) = g_after.find('"') else {
                rest = tail;
                continue;
            };
            let doi = &g_after[..g_end];
            buf.push_str(&format!("{} | {}\n", doi, name));
            gained += 1;
            rest = &g_after[g_end..];
        }
        total += gained;
        if gained == 0 {
            break;
        }
    }
    if let Some(path) = out {
        let _ = fs::write(&path, &buf);
        eprintln!("dataverse: {} records → {}", total, path);
    } else {
        eprintln!("dataverse: {} records (--out absent)", total);
    }
}
