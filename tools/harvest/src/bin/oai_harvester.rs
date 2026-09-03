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

fn extract_pairs(body: &str) -> Vec<(String, String)> {
    let mut ids = Vec::new();
    let mut titles = Vec::new();
    let mut rest = body;
    while let Some(pos) = rest.find("<identifier>") {
        let after = &rest[pos + 12..];
        let Some(end) = after.find("</identifier>") else {
            break;
        };
        ids.push(after[..end].trim().to_string());
        rest = &after[end..];
    }
    rest = body;
    while let Some(pos) = rest.find("<dc:title>") {
        let after = &rest[pos + 10..];
        let Some(end) = after.find("</dc:title>") else {
            break;
        };
        titles.push(after[..end].trim().to_string());
        rest = &after[end..];
    }
    ids.into_iter().zip(titles.into_iter()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut root: Option<String> = None;
    let mut set: Option<String> = None;
    let mut out: Option<String> = None;
    let mut max_pages: usize = 1000;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                root = args.get(i + 1).cloned();
                i += 1;
            }
            "--set" => {
                set = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--pages" => {
                max_pages = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1000);
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
    let mut records: Vec<(String, String)> = Vec::new();
    let mut token: Option<String> = None;
    let mut pages = 0;
    loop {
        if pages >= max_pages {
            break;
        }
        pages += 1;
        let url = match (&token, &set) {
            (Some(t), _) => format!("{}?verb=ListRecords&resumptionToken={}", root, t),
            (None, Some(s)) => format!("{}?verb=ListRecords&metadataPrefix=oai_dc&set={}", root, s),
            (None, None) => format!("{}?verb=ListRecords&metadataPrefix=oai_dc", root),
        };
        let Some(body) = curl(&url) else {
            eprintln!("page {} returned void", pages);
            break;
        };
        let before = records.len();
        records.extend(extract_pairs(&body));
        let new_token = body.find("<resumptionToken").and_then(|p| {
            let start = p + body[p..].find('>')? + 1;
            let end = body[start..].find("</resumptionToken>")? + start;
            let t = body[start..end].to_string();
            if t.trim().is_empty() { None } else { Some(t) }
        });
        let gained = records.len() - before;
        if gained == 0 && new_token.is_none() {
            break;
        }
        token = new_token;
    }
    let mut buf = String::new();
    for (id, title) in &records {
        buf.push_str(&format!("{} | {}\n", id, title));
    }
    if let Some(path) = out {
        let _ = fs::write(&path, &buf);
        eprintln!("oai: {} records → {}", records.len(), path);
    } else {
        eprintln!("oai: {} records (--out absent)", records.len());
    }
}
