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

fn extract(block: &str, tag: &str) -> String {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    match block.find(&open) {
        Some(p) => {
            let a = &block[p + open.len()..];
            match a.find(&close) {
                Some(e) => a[..e].trim().to_string(),
                None => String::new(),
            }
        }
        None => String::new(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut root: Option<String> = None;
    let mut record = String::from("repository");
    let mut id_tag = String::from("id");
    let mut title_tag = String::from("name");
    let mut out: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                root = args.get(i + 1).cloned();
                i += 1;
            }
            "--record" => {
                record = args.get(i + 1).cloned().unwrap_or(record);
                i += 1;
            }
            "--id" => {
                id_tag = args.get(i + 1).cloned().unwrap_or(id_tag);
                i += 1;
            }
            "--title" => {
                title_tag = args.get(i + 1).cloned().unwrap_or(title_tag);
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
    let Some(root) = root else {
        eprintln!("--root absent");
        std::process::exit(1);
    };
    let Some(body) = curl(&root) else {
        eprintln!("xml: {} returned void", root);
        std::process::exit(1);
    };
    let open = format!("<{}>", record);
    let close = format!("</{}>", record);
    let mut buf = String::new();
    let mut total = 0usize;
    let mut rest = body.as_str();
    while let Some(rp) = rest.find(&open) {
        let after = &rest[rp + open.len()..];
        let Some(re) = after.find(&close) else {
            break;
        };
        let block = &after[..re];
        let id = extract(block, &id_tag);
        let title = extract(block, &title_tag);
        if !id.is_empty() {
            buf.push_str(&format!("{} | {}\n", id, title));
            total += 1;
        }
        rest = &after[re..];
    }
    if let Some(path) = out {
        let _ = fs::write(&path, &buf);
        eprintln!("xml: {} records → {}", total, path);
    } else {
        eprintln!("xml: {} records (--out absent)", total);
    }
}
