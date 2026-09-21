use std::fs;
use std::path::{Path, PathBuf};

use omegaflow::sha256::sha256_hex;

struct Audit {
    checked: usize,
    ok: usize,
    problems: Vec<String>,
    fixes: Vec<(PathBuf, String)>,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let fix = args.iter().any(|a| a == "--fix");
    let root = match args
        .iter()
        .position(|a| a == "--root")
        .and_then(|i| args.get(i + 1))
    {
        Some(r) => r.clone(),
        None => "docs".to_string(),
    };

    let audit = run_audit(&root);
    if !fix {
        println!(
            "doc_audit | checked {} | ok {} | problems {}",
            audit.checked,
            audit.ok,
            audit.problems.len()
        );
        for p in &audit.problems {
            println!("  {p}");
        }
        if !audit.problems.is_empty() {
            std::process::exit(2);
        }
        return;
    }

    let mut fixed = 0usize;
    for (path, actual) in &audit.fixes {
        let Ok(text) = fs::read_to_string(path) else {
            continue;
        };
        if let Some(new_text) = fix_sha256(&text, actual) {
            if fs::write(path, &new_text).is_ok() {
                fixed += 1;
            }
        }
    }

    let after = run_audit(&root);
    println!(
        "doc_audit --fix | checked {} | ok {} | fixed={fixed} | remaining problems={}",
        after.checked,
        after.ok,
        after.problems.len()
    );
    for p in &after.problems {
        println!("  {p}");
    }
    if !after.problems.is_empty() {
        std::process::exit(2);
    }
}

fn run_audit(root: &str) -> Audit {
    let mut files: Vec<PathBuf> = Vec::new();
    walk(Path::new(root), &mut files);
    files.sort();

    let mut checked = 0usize;
    let mut ok = 0usize;
    let mut problems: Vec<String> = Vec::new();
    let mut fixes: Vec<(PathBuf, String)> = Vec::new();
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
            fixes.push((path.clone(), actual));
        }
        if let Some(slug) = name
            .strip_prefix("handover-")
            .and_then(|s| s.split_once('-').map(|(_, rest)| rest.to_string()))
        {
            let bare = ["future", "mountain", "mycelium", "sensory", "river"]
                .iter()
                .any(|l| {
                    slug.starts_with(l) && slug[l.len()..].chars().all(|c| c.is_ascii_digit())
                });
            if bare {
                problems.push(format!(
                    "{name}: line slug '{slug}' drifts from <line>-folge<N>"
                ));
            }
        }
    }
    Audit {
        checked,
        ok,
        problems,
        fixes,
    }
}

fn fix_sha256(text: &str, actual: &str) -> Option<String> {
    let rest = text.strip_prefix("<!--")?;
    let end = rest.find("\n-->\n")?;
    let header = &rest[..end];
    let body = &rest[end + 5..];

    let mut new_header = String::with_capacity(header.len());
    let mut found = false;
    let mut first = true;
    for line in header.split('\n') {
        if !first {
            new_header.push('\n');
        }
        first = false;
        if !found {
            let trimmed = line.trim_start();
            if let Some(val) = trimmed.strip_prefix("sha256:") {
                let indent = line.len() - trimmed.len();
                let lead = val.len() - val.trim_start().len();
                let core = val.trim();
                new_header.push_str(&line[..indent]);
                new_header.push_str("sha256:");
                new_header.push_str(&val[..lead]);
                new_header.push_str(actual);
                new_header.push_str(&val[lead + core.len()..]);
                found = true;
                continue;
            }
        }
        new_header.push_str(line);
    }
    if !found {
        return None;
    }
    Some(format!("<!--{new_header}\n-->\n{body}"))
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

    #[test]
    fn fix_replaces_only_hash_and_keeps_body() {
        let t = "<!--\n  title: x\n  sha256: deadbeef\n-->\n# Title\nbody\n";
        let new = fix_sha256(t, "cafe").unwrap();
        assert_eq!(
            new,
            "<!--\n  title: x\n  sha256: cafe\n-->\n# Title\nbody\n"
        );
    }

    #[test]
    fn fix_keeps_surrounding_whitespace() {
        let t = "<!--\n  sha256:   deadbeef  \n-->\nbody";
        let new = fix_sha256(t, "cafe").unwrap();
        assert_eq!(new, "<!--\n  sha256:   cafe  \n-->\nbody");
    }

    #[test]
    fn fix_without_sha_line_is_none() {
        let t = "<!--\n  title: x\n-->\nbody";
        assert!(fix_sha256(t, "cafe").is_none());
    }
}
