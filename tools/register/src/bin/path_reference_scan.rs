use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                let name = p.file_name().map(|s| s.to_string_lossy().to_string());
                if name == Some("target".into())
                    || name == Some(".git".into())
                    || name == Some("node_modules".into())
                {
                    continue;
                }
                walk(&p, out);
            } else {
                out.push(p);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = args.get(1).cloned().unwrap_or_else(|| ".".to_string());
    let mut files = Vec::new();
    walk(Path::new(&root), &mut files);

    let mut missing = 0usize;
    let mut absolute = 0usize;
    for f in &files {
        let rel = f.to_string_lossy().to_string();
        if rel.contains("/docs/reference/")
            || rel.starts_with("./gate/")
            || rel.starts_with("./mail/")
            || rel.starts_with("./reports/")
            || rel.contains("/node_modules/")
        {
            continue;
        }
        let is_markdown = f.extension().map(|e| e == "md").unwrap_or(false);
        let content = match fs::read_to_string(f) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for (lineno, line) in content.lines().enumerate() {
            let n = lineno + 1;
            if is_markdown && line.starts_with('#') {
                continue;
            }
            for abs in absolute_paths(line) {
                absolute += 1;
                println!("ABS  {}:{}  {}", rel, n, abs);
            }
            if !is_markdown {
                continue;
            }
            for (label, target) in file_refs(line) {
                if target.starts_with("http://")
                    || target.starts_with("https://")
                    || target.starts_with("mailto:")
                {
                    continue;
                }
                if target.starts_with('#') {
                    continue;
                }
                let cleaned = target.split('#').next().unwrap_or("").to_string();
                if cleaned.is_empty() {
                    continue;
                }
                let resolved = resolve(&root, f, &cleaned);
                if !resolved.exists() {
                    missing += 1;
                    println!("MISS {}:{}  {}  ->  {}", rel, n, label, cleaned);
                }
            }
        }
    }
    println!(
        "path_reference_scan: {} files | {} missing refs | {} absolute paths",
        files.len(),
        missing,
        absolute
    );
    if missing > 0 || absolute > 0 {
        std::process::exit(1);
    }
}

fn file_refs(line: &str) -> Vec<(&str, String)> {
    let mut out = Vec::new();
    if let Some(idx) = line.find("see-also:") {
        for tok in line[idx + 9..].split_whitespace() {
            let t = tok.trim_matches(|c| c == ',' || c == ')' || c == '(');
            let looks_like_path = t.contains('/')
                || t.ends_with(".md")
                || t.ends_with(".φ")
                || t.ends_with(".rs")
                || t.ends_with(".txt")
                || t.ends_with(".yml")
                || t.ends_with(".yaml")
                || t.ends_with(".toml");
            if looks_like_path {
                out.push(("see-also", t.to_string()));
            }
        }
    }
    let mut rest = line;
    while let Some(start) = rest.find("](") {
        let after = &rest[start + 2..];
        if let Some(end) = after.find(')') {
            let target = after[..end].to_string();
            out.push(("link", target));
            rest = &after[end + 1..];
        } else {
            break;
        }
    }
    out
}

fn absolute_paths(line: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find("/home/") {
        let mut end = start;
        let bytes = rest.as_bytes();
        while end < bytes.len()
            && !bytes[end].is_ascii_whitespace()
            && bytes[end] != b'"'
            && bytes[end] != b'`'
            && bytes[end] != b')'
        {
            end += 1;
        }
        let seg = &rest[start..end];
        if seg.starts_with("/home/johannes/projects/omegaflow") && seg.ends_with(".rs")
            || seg.contains("/omegaflow-worktrees/")
        {
            out.push(seg);
        }
        rest = &rest[end..];
    }
    out
}

fn resolve(root: &str, file: &Path, target: &str) -> PathBuf {
    let t = target.trim();
    if t.starts_with('/') {
        return PathBuf::from(root).join(t.trim_start_matches('/'));
    }
    if let Some(dir) = file.parent() {
        let candidate = dir.join(t);
        if candidate.exists() {
            return candidate;
        }
    }
    PathBuf::from(root).join(t)
}
