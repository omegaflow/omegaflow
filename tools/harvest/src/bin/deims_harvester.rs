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

fn unescape(s: &str) -> String {
    let mut out = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 5 < bytes.len() && bytes[i + 1] == b'u' {
            if let Ok(hex) = std::str::from_utf8(&bytes[i + 2..i + 6]) {
                if let Ok(c) = u32::from_str_radix(hex, 16) {
                    if let Some(ch) = char::from_u32(c) {
                        out.push(ch);
                        i += 6;
                        continue;
                    }
                }
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn extract_all(body: &str, key: &str) -> Vec<String> {
    let mut out = Vec::new();
    let needle = format!("\"{}\":\"", key);
    let mut rest = body;
    while let Some(pos) = rest.find(&needle) {
        let after = &rest[pos + needle.len()..];
        let Some(end) = after.find('"') else {
            break;
        };
        out.push(unescape(&after[..end]).replace("\\/", "/"));
        rest = &after[end..];
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut root = String::from("https://deims.org/api");
    let mut resource = String::from("sites");
    let mut out: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                root = args.get(i + 1).cloned().unwrap_or(root);
                i += 1;
            }
            "--resource" => {
                resource = args.get(i + 1).cloned().unwrap_or(resource);
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let url = format!("{}/{}", root, resource);
    let Some(body) = curl(&url) else {
        eprintln!("deims: {} returned void", url);
        std::process::exit(1);
    };
    let mut buf = String::new();
    if resource == "sites" {
        let titles = extract_all(&body, "title");
        let coords = extract_all(&body, "coordinates");
        for (t, c) in titles.iter().zip(coords.iter()) {
            buf.push_str(&format!("{} | {}\n", c, t));
        }
    } else {
        let titles = extract_all(&body, "title");
        let suffixes = extract_all(&body, "suffix");
        for (t, s) in titles.iter().zip(suffixes.iter()) {
            buf.push_str(&format!("{} | {}\n", s, t));
        }
    }
    let n = buf.lines().count();
    if let Some(path) = out {
        let _ = fs::write(&path, &buf);
        eprintln!("deims {}: {} records → {}", resource, n, path);
    } else {
        eprintln!("deims {}: {} records (--out absent)", resource, n);
    }
}
