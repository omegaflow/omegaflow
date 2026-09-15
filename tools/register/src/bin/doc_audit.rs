use std::fs;
use std::path::{Path, PathBuf};

use omegaflow::sha256::sha256_hex;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = match args
        .iter()
        .position(|a| a == "--root")
        .and_then(|i| args.get(i + 1))
    {
        Some(r) => r.clone(),
        None => "docs".to_string(),
    };
    let mut files: Vec<PathBuf> = Vec::new();
    walk(Path::new(&root), &mut files);
    files.sort();

    let mut checked = 0usize;
    let mut ok = 0usize;
    let mut problems: Vec<String> = Vec::new();
    for path in &files {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name == "_template.md" || name.ends_with(".φ") {
            continue;
        }
        if path.to_string_lossy().contains("/reference/") {
            continue;
        }
        checked += 1;
        let Ok(text) = fs::read_to_string(path) else {
            problems.push(format!("{name}: unreadable"));
            continue;
        };
        let Some((header, body)) = split_header(&text) else {
            problems.push(format!("{name}: no <!-- header -->"));
            continue;
        };
        let Some(claimed) = header_field(header, "sha256") else {
            problems.push(format!("{name}: header carries no sha256"));
            continue;
        };
        let actual = sha256_hex(body.as_bytes());
        if claimed == actual {
            ok += 1;
        } else {
            problems.push(format!("{name}: sha256 {claimed} != body {actual}"));
        }
        if let Some(slug) = name
            .strip_prefix("handover-")
            .and_then(|s| s.split_once('-').map(|(_, rest)| rest.to_string()))
        {
            let bare = ["entscheid", "bau", "ernte", "forschung"].iter().any(|l| {
                slug.starts_with(l) && slug[l.len()..].chars().all(|c| c.is_ascii_digit())
            });
            if bare {
                problems.push(format!(
                    "{name}: line slug '{slug}' drifts from <line>-folge<N>"
                ));
            }
        }
    }
    println!(
        "doc_audit | checked {checked} | ok {ok} | problems {}",
        problems.len()
    );
    for p in &problems {
        println!("  {p}");
    }
    if !problems.is_empty() {
        std::process::exit(2);
    }
}

fn split_header(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix("<!--")?;
    let end = rest.find("\n-->\n")?;
    let header = &rest[..end];
    let body = &rest[end + 5..];
    Some((header, body))
}

fn header_field(header: &str, key: &str) -> Option<String> {
    let pat = format!("{key}:");
    let line = header.lines().find(|l| l.trim_start().starts_with(&pat))?;
    Some(line.split_once(&pat)?.1.trim().to_string())
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, out);
        } else if p.extension().and_then(|x| x.to_str()) == Some("md") {
            out.push(p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_and_body_split() {
        let t = "<!--\n  sha256: abc\n-->\n# Title\nbody\n";
        let (h, b) = split_header(t).unwrap();
        assert_eq!(header_field(h, "sha256").as_deref(), Some("abc"));
        assert_eq!(b, "# Title\nbody\n");
    }

    #[test]
    fn absent_header_is_none() {
        assert!(split_header("# Title\n").is_none());
    }
}
