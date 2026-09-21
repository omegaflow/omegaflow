use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const PREFIXES: [&str; 8] = [
    "src/", "tools/", "docs/", "phi/", "bin/", "state/", ".github/", "firmware/",
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut root = PathBuf::from(".");
    let mut file: Option<PathBuf> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                i += 1;
                if let Some(r) = args.get(i) {
                    root = PathBuf::from(r);
                }
            }
            "-h" | "--help" => {
                println!(
                    "usage: open_points_check [<handover.md>] [--root <dir>]  (default: newest live docs/handover/*.md)"
                );
                return;
            }
            other => file = Some(PathBuf::from(other)),
        }
        i += 1;
    }

    let handover = match file {
        Some(f) => f,
        None => match newest_live_handover(&root) {
            Some(f) => f,
            None => {
                println!("open_points_check: no live handover under docs/handover/");
                return;
            }
        },
    };

    let content = match fs::read_to_string(&handover) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("open_points_check: {} reads not: {}", handover.display(), e);
            std::process::exit(2);
        }
    };

    let rel = handover
        .strip_prefix(&root)
        .unwrap_or(&handover)
        .display()
        .to_string();
    let mut points = 0usize;
    let mut missing = 0usize;
    for (idx, line) in content.lines().enumerate() {
        for token in path_tokens(line) {
            points += 1;
            if !root.join(&token).exists() {
                missing += 1;
                println!("ABSENT  {}:{}  {}", rel, idx + 1, token);
            }
        }
    }
    println!(
        "open_points_check: {}  | {} path refs | {} absent",
        rel, points, missing
    );
}

fn newest_live_handover(root: &Path) -> Option<PathBuf> {
    let dir = root.join("docs/handover");
    let entries = fs::read_dir(&dir).ok()?;
    let mut best: Option<(SystemTime, PathBuf)> = None;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map(|e| e != "md").unwrap_or(true) {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('_') {
            continue;
        }
        let mtime = entry
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        if best.as_ref().map(|(t, _)| mtime > *t).unwrap_or(true) {
            best = Some((mtime, path));
        }
    }
    best.map(|(_, p)| p)
}

fn path_tokens(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find('`') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('`') else { break };
        for word in after[..end].split_whitespace() {
            if let Some(t) = normalize(word) {
                out.push(t);
            }
        }
        rest = &after[end + 1..];
    }
    for word in line.split_whitespace() {
        if let Some(t) = normalize(word) {
            out.push(t);
        }
    }
    out.sort();
    out.dedup();
    out
}

fn normalize(word: &str) -> Option<String> {
    let trimmed = word.trim_start_matches(|c: char| {
        matches!(
            c,
            '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | ',' | ';' | '\'' | '"' | '|' | '*' | '`'
        )
    });
    if trimmed.starts_with('/') || trimmed.starts_with("http") {
        return None;
    }
    let without_lines = match trimmed.find(':') {
        Some(pos)
            if trimmed[pos + 1..]
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false) =>
        {
            &trimmed[..pos]
        }
        _ => trimmed,
    };
    let cleaned = without_lines.trim_end_matches(|c: char| {
        matches!(
            c,
            ')' | ']' | '}' | '>' | ',' | ';' | ':' | '\'' | '"' | '|' | '*' | '`' | '.'
        )
    });
    let cleaned = cleaned.strip_prefix("./").unwrap_or(cleaned);
    if !is_repo_path(cleaned) {
        return None;
    }
    Some(cleaned.to_string())
}

fn is_repo_path(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }
    if matches!(token, "AGENTS.md" | "Cargo.toml" | "README.md" | "LICENSE") {
        return true;
    }
    PREFIXES.iter().any(|p| token.starts_with(p)) && token.contains('/')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_backticked_and_bare_paths() {
        let toks = path_tokens("step: `tools/service/src/bin/smail.rs` and docs/handover/post.md");
        assert!(toks.contains(&"tools/service/src/bin/smail.rs".to_string()));
        assert!(toks.contains(&"docs/handover/post.md".to_string()));
    }

    #[test]
    fn strips_line_numbers_and_trailing_punctuation() {
        assert_eq!(
            normalize("src/gate/commit_gate.rs:1804,").as_deref(),
            Some("src/gate/commit_gate.rs")
        );
        assert_eq!(
            normalize("`docs/handover/post.md`").as_deref(),
            Some("docs/handover/post.md")
        );
        assert_eq!(
            normalize("(`.github/workflows/hyperscanning-te.yml:41`).").as_deref(),
            Some(".github/workflows/hyperscanning-te.yml")
        );
        assert_eq!(
            normalize("docs/handover/post.md).").as_deref(),
            Some("docs/handover/post.md")
        );
        assert_eq!(
            normalize("`state/mail/mail_ledger.φ`:").as_deref(),
            Some("state/mail/mail_ledger.φ")
        );
        assert_eq!(
            normalize("`docs/zustand/dropped-baseline.md`.").as_deref(),
            Some("docs/zustand/dropped-baseline.md")
        );
    }

    #[test]
    fn ignores_urls_absolute_paths_and_plain_words() {
        assert!(normalize("https://example.org/src/x").is_none());
        assert!(normalize("/home/johannes/x").is_none());
        assert!(normalize("src").is_none());
        assert!(normalize("the").is_none());
    }
}
