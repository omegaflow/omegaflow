use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sL")
        .arg("--max-time")
        .arg("40")
        .arg(&url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

fn extract_between<'a>(s: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = s.find(open)? + open.len();
    let rest = &s[start..];
    let end = rest.find(close)?;
    Some(&rest[..end])
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        if in_tag {
            if c == '>' {
                in_tag = false;
            }
            continue;
        }
        if c == '<' {
            in_tag = true;
            continue;
        }
        out.push(c);
    }
    out
}

fn extract_arxiv_ids(body: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let lower = body.to_lowercase();
    let mut pos = 0;
    while let Some(rel) = lower[pos..].find("arxiv") {
        let idx = pos + rel + 5;
        let rest = &body[idx..];
        let mut j = 0;
        let b = rest.as_bytes();

        while j < b.len() && (b[j] == b':' || b[j] == b' ') {
            j += 1;
        }
        let mut id = String::new();
        while j < b.len() {
            let c = b[j] as char;
            if c.is_ascii_digit() || c == '.' {
                id.push(c);
                j += 1;
            } else if (c == 'v' || c == 'V')
                && id.contains('.')
                && j + 1 < b.len()
                && (b[j + 1] as char).is_ascii_digit()
            {
                id.push(c);
                j += 1;
            } else {
                break;
            }
        }

        let clean = id.trim_end_matches('.').to_string();
        if let Some(dot) = clean.find('.') {
            if dot > 0
                && dot + 1 < clean.len()
                && clean[..dot].chars().all(|c| c.is_ascii_digit())
                && clean[dot + 1..]
                    .chars()
                    .all(|c| c.is_ascii_digit() || c == 'v')
            {
                if !ids.contains(&clean) {
                    ids.push(clean);
                }
            }
        }
        pos = idx + j;
    }
    ids
}

fn extract_dois(body: &str) -> Vec<String> {
    let mut dois = Vec::new();
    let bytes = body.as_bytes();
    let mut i = 0;
    while i + 2 < bytes.len() {
        if bytes[i] == b'1' && bytes[i + 1] == b'0' && bytes[i + 2] == b'.' {
            let mut j = i + 3;
            let mut digits = 0;
            while j < bytes.len() && (bytes[j] as char).is_ascii_digit() {
                digits += 1;
                j += 1;
            }
            if digits >= 4 && j < bytes.len() && bytes[j] == b'/' {
                let start = i;
                let mut k = j + 1;
                let mut suffix = String::new();
                while k < bytes.len() {
                    let c = bytes[k] as char;
                    if c.is_ascii_alphanumeric()
                        || c == '.'
                        || c == '_'
                        || c == '-'
                        || c == '('
                        || c == ')'
                        || c == '/'
                    {
                        suffix.push(c);
                        k += 1;
                    } else {
                        break;
                    }
                }
                let doi = body[start..i + 3 + digits + 1].to_string() + &suffix;
                let clean = doi.trim_end_matches(['.', ')']).to_string();
                if !dois.contains(&clean) {
                    dois.push(clean);
                }
                i = k;
                continue;
            }
        }
        i += 1;
    }
    dois
}

fn verify_arxiv(id: &str) -> (String, String, String) {
    let url = format!("http://export.arxiv.org/api/query?id_list={}", id);
    match fetch(&url) {
        None => ("pending".to_string(), String::new(), String::new()),
        Some(xml) => {
            if let Some(entry) = extract_between(&xml, "<entry>", "</entry>") {
                let title = extract_between(entry, "<title>", "</title>")
                    .map(|t| strip_tags(t).trim().to_string())
                    .unwrap_or_default();
                let mut names = Vec::new();
                let mut rest = entry;
                while let Some(n) = extract_between(rest, "<name>", "</name>") {
                    names.push(strip_tags(n).trim().to_string());
                    let off = rest.find("<name>").unwrap() + 6;
                    rest = &rest[off..];
                    if names.len() >= 4 {
                        break;
                    }
                }
                ("resolved".to_string(), title, names.join("; "))
            } else {
                ("absent".to_string(), String::new(), String::new())
            }
        }
    }
}

fn verify_doi(doi: &str) -> (String, String) {
    let url = format!("https://doi.org/{}", doi);
    let out = Command::new("curl")
        .arg("-s")
        .arg("-o")
        .arg("/dev/null")
        .arg("-w")
        .arg("%{http_code}")
        .arg("--max-time")
        .arg("40")
        .arg(&url)
        .output();
    let code = match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => String::from("pending"),
    };
    let c = code.trim().parse::<u16>().unwrap_or(0);
    match c {
        200 | 201 | 202 | 301 | 302 | 303 | 307 | 308 => {
            ("resolved".to_string(), format!("doi.org http={}", c))
        }
        404 | 410 => (
            "absent".to_string(),
            format!("doi.org http={} — not in the global handle register", c),
        ),
        0 => ("pending".to_string(), "no network / timeout".to_string()),
        _ => ("pending".to_string(), format!("doi.org http={}", c)),
    }
}

fn paper_body(md_path: &Path) -> String {
    let src = fs::read_to_string(md_path).unwrap_or_default();

    if let Some(start) = src.find("<!--") {
        if let Some(end) = src[start + 4..].find("-->") {
            let mut rest = start + 4 + end + 3;
            if src[rest..].starts_with("\r\n") {
                rest += 2;
            } else if src[rest..].starts_with('\n') {
                rest += 1;
            }
            return src[rest..].to_string();
        }
    }
    src
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        if let Ok(rd) = fs::read_dir("docs/paper") {
            for e in rd.flatten() {
                let p = e.path();
                if p.extension().map(|x| x == "md").unwrap_or(false) {
                    args.push(p.to_string_lossy().to_string());
                }
            }
        }
        args.sort();
    }

    let mut any_pending = false;
    let mut any_absent = false;
    for arg in &args {
        let p = Path::new(arg);
        if !p.exists() {
            eprintln!("no such paper: {}", arg);
            continue;
        }
        let slug = p.file_stem().unwrap().to_string_lossy().to_string();
        let body = paper_body(p);
        let arxiv_ids = extract_arxiv_ids(&body);
        let dois = extract_dois(&body);

        if arxiv_ids.is_empty() && dois.is_empty() {
            println!("{}\t— no arXiv id, no DOI in body", slug);
            continue;
        }
        println!("== {} ==", slug);
        for id in &arxiv_ids {
            let (status, title, authors) = verify_arxiv(id);
            if status == "pending" {
                any_pending = true;
            }
            if status == "absent" {
                any_absent = true;
            }
            println!("\tarXiv:{}\t{}\t{}\t{}", id, status, title, authors);
            thread::sleep(Duration::from_secs(3));
        }
        for doi in &dois {
            let (status, note) = verify_doi(doi);
            if status == "pending" {
                any_pending = true;
            }
            if status == "absent" {
                any_absent = true;
            }
            println!("\tDOI:{}\t{}\t{}", doi, status, note);
        }
    }

    if any_pending {
        eprintln!("one or more identifiers are pending (no network/timeout) — named, not assumed");
    }
    if any_absent {
        eprintln!("one or more identifiers are absent in the external register — named");
    }
    if any_absent || any_pending {
        std::process::exit(1);
    }
}
