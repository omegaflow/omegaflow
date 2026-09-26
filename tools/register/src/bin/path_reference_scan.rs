use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn tracked_files(root: &Path) -> Vec<PathBuf> {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-files", "-z"])
        .output();
    let out = match out {
        Ok(o) if o.status.success() => o.stdout,
        _ => return Vec::new(),
    };
    out.split(|&b| b == 0)
        .filter(|s| !s.is_empty())
        .map(|s| String::from_utf8_lossy(s).to_string())
        .map(PathBuf::from)
        .collect()
}

fn scan(root: &Path) -> (usize, usize) {
    let files = tracked_files(root);

    let mut missing = 0usize;
    let mut absolute = 0usize;
    for f in &files {
        let rel = f.to_string_lossy().to_string();
        if is_skipped(&rel) {
            continue;
        }
        let full = if Path::new(&rel).is_absolute() {
            f.clone()
        } else {
            root.join(f)
        };
        let is_markdown = f.extension().map(|e| e == "md").unwrap_or(false);
        let content = match fs::read_to_string(&full) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for (lineno, line) in content.lines().enumerate() {
            let n = lineno + 1;
            if is_markdown && line.starts_with('#') {
                continue;
            }
            if !rel.ends_with(".rs") {
                for abs in absolute_paths(line) {
                    absolute += 1;
                    println!("ABS  {}:{}  {}", rel, n, abs);
                }
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
                let resolved = resolve(root, &full, &cleaned);
                if !resolved.exists() {
                    if is_git_ignored(root, &resolved) {
                        continue;
                    }
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
    (missing, absolute)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = match args.get(1).cloned() {
        Some(r) => r,
        None => ".".to_string(),
    };
    let (missing, absolute) = scan(Path::new(&root));
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
    let line = line.split("://").next().unwrap_or(line);
    let markers = ["/home/", "/Users/", "/root/", "/srv/", "/mnt/"];
    for m in markers {
        let mut rest = line;
        while let Some(start) = rest.find(m) {
            let bytes = rest.as_bytes();
            let mut end = start;
            while end < bytes.len()
                && !bytes[end].is_ascii_whitespace()
                && bytes[end] != b'"'
                && bytes[end] != b'`'
                && bytes[end] != b')'
                && bytes[end] != b','
                && bytes[end] != b';'
            {
                end += 1;
            }
            let seg = &rest[start..end];
            out.push(seg);
            rest = &rest[end..];
        }
    }
    out
}

fn is_skipped(rel: &str) -> bool {
    rel.contains("path_reference_scan.rs")
        || rel.contains("/archiv/")
        || rel.contains("docs/reference/")
        || rel == "src/gate/commit_gate_vocab.json"
        || rel.starts_with("mail/")
        || rel.starts_with("state/reports/")
}

fn is_git_ignored(root: &Path, path: &Path) -> bool {
    let rel = match path.strip_prefix(root) {
        Ok(r) => r,
        Err(_) => return false,
    };
    match Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("check-ignore")
        .arg("-q")
        .arg("--")
        .arg(rel)
        .status()
    {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}

fn resolve(root: &Path, file: &Path, target: &str) -> PathBuf {
    let t = target.trim();
    if t.starts_with('/') {
        return root.join(t.trim_start_matches('/'));
    }
    if let Some(dir) = file.parent() {
        let candidate = dir.join(t);
        if candidate.exists() {
            return candidate;
        }
    }
    root.join(t)
}

#[cfg(test)]
fn repo_root() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest.parent().unwrap().parent().unwrap().to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn committed_tree_has_no_broken_references_and_no_absolute_paths() {
        let root = repo_root();
        let (missing, absolute) = scan(&root);
        assert_eq!(missing, 0, "every committed reference resolves in-repo");
        assert_eq!(absolute, 0, "no absolute path may enter the committed tree");
    }

    #[test]
    fn gate_vocab_fixture_is_skipped_but_gate_source_is_scanned() {
        assert!(is_skipped("src/gate/commit_gate_vocab.json"));
        assert!(!is_skipped("src/gate/commit_gate.rs"));
    }

    #[test]
    fn gitignored_target_is_not_a_broken_reference() {
        let root = repo_root();
        let target = root.join("state/zustand/external-state.md");
        assert!(is_git_ignored(&root, &target));
    }

    #[test]
    fn absolute_path_detection_names_leading_home() {
        let found = absolute_paths("wohnt in /home/operator/projects/omegaflow/ dir");
        assert_eq!(found, vec!["/home/operator/projects/omegaflow/"]);
    }

    #[test]
    fn url_contexts_are_not_local_paths() {
        assert!(absolute_paths("served at https://example.com/srv/eng/csw?format=json").is_empty());
        assert_eq!(
            absolute_paths("see /home/operator/a then https://x/srv/b"),
            vec!["/home/operator/a"]
        );
    }
}
