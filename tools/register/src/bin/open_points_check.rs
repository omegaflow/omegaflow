use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const PREFIXES: [&str; 8] = [
    "src/",
    "tools/",
    "docs/",
    "phi/",
    "bin/",
    "state/",
    ".github/",
    "firmware/",
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
    let guardians = mail_home_guardians(&root);
    for g in &guardians {
        println!("OFFEN  {}", g);
    }
    let format_gaps = point_format_gaps(&content);
    let format_gap_count = match &format_gaps {
        Some(gaps) => {
            for gap in gaps {
                println!("format-gap  {}", gap);
            }
            gaps.len()
        }
        None => {
            println!("format-gaps skipped");
            0
        }
    };
    let drifts = owner_drift(&root);
    for d in &drifts {
        println!("{}", d);
    }
    let post_md = post_md_resurrected(&root);
    if post_md {
        println!("post-md-resurrected");
    }
    println!(
        "open_points_check: {}  | {} path refs | {} absent | {} guardians | {} format-gaps | {} owner-drift | {} post-md",
        rel,
        points,
        missing,
        guardians.len(),
        format_gap_count,
        drifts.len(),
        post_md as usize
    );
}

fn mail_home_guardians(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    if !root.join("state").exists() {
        return out;
    }
    if root.join("state/funding/mail").exists() {
        out.push(
            "zweites Mail-Heim: state/funding/mail/ existiert (der Postkorb ist state/mail/)"
                .to_string(),
        );
    }
    if root
        .join("state/mail")
        .symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
    {
        out.push(
            "Mail-Heim ist ein Symlink: state/mail/ muss ein echtes Verzeichnis im Repo sein"
                .to_string(),
        );
    }
    out
}

fn handover_line_owner(path: &Path) -> Option<String> {
    let name = path.file_name()?.to_string_lossy().to_string();
    let stem = name.strip_suffix(".md")?;
    let rest = stem.strip_prefix("handover-")?;
    if rest.len() < 12 {
        return None;
    }
    if rest.as_bytes().get(10) != Some(&b'-') {
        return None;
    }
    let tail = &rest[11..];
    let folge_at = tail.rfind("-folge")?;
    let line = &tail[..folge_at];
    if line.is_empty() {
        return None;
    }
    Some(line.to_string())
}

fn bindung_linie_owner(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("- **") {
        return None;
    }
    let at = t.find("**Bindung:**")?;
    let after = t[at + "**Bindung:**".len()..].trim_start();
    let linie = after.strip_prefix("linie:")?;
    let owner: String = linie
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();
    if owner.is_empty() { None } else { Some(owner) }
}

fn owner_drift(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let dir = root.join("docs/handover");
    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return out,
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        if !path.is_file() {
            continue;
        }
        let Some(owner) = handover_line_owner(&path) else {
            continue;
        };
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for line in content.lines() {
            let Some(y) = bindung_linie_owner(line) else {
                continue;
            };
            if y != owner {
                let rel = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .display()
                    .to_string();
                out.push(format!("owner-drift  {}  Bindung linie:{}", rel, y));
            }
        }
    }
    out
}

fn post_md_resurrected(root: &Path) -> bool {
    root.join("docs/handover/post.md").exists()
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
            '(' | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '<'
                | '>'
                | ','
                | ';'
                | '\''
                | '"'
                | '|'
                | '*'
                | '`'
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
    if cleaned.contains('*') || cleaned.contains("::") {
        return None;
    }
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

const POINT_FIELDS: [&str; 5] = [
    "- **Status:**",
    "- **Trigger:**",
    "- **Lage:**",
    "- **Blockade:**",
    "- **Braucht:**",
];

fn point_format_gaps(content: &str) -> Option<Vec<String>> {
    let lines: Vec<&str> = content.lines().collect();
    let start = lines.iter().position(|l| is_offen_heading(l))?;
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 1)
        .find(|(_, l)| is_h2_heading(l))
        .map(|(i, _)| i)
        .unwrap_or(lines.len());
    let section = &lines[start + 1..end];
    let mut gaps = Vec::new();
    let mut current: Option<(&str, Vec<&str>)> = None;
    for line in section {
        if let Some(name) = line.trim_start().strip_prefix("### ") {
            if let Some((prior, body)) = current.take() {
                check_point(prior, &body, &mut gaps);
            }
            current = Some((name.trim(), Vec::new()));
        } else if let Some((_, body)) = current.as_mut() {
            body.push(line);
        }
    }
    if let Some((name, body)) = current {
        check_point(name, &body, &mut gaps);
    }
    Some(gaps)
}

fn check_point(name: &str, body: &[&str], gaps: &mut Vec<String>) {
    let trimmed: Vec<&str> = body.iter().map(|l| l.trim()).collect();
    for field in POINT_FIELDS {
        if !trimmed.iter().any(|l| l.starts_with(field)) {
            gaps.push(format!("{}  missing {}", name, field));
        }
    }
    if let Some(lage) = trimmed.iter().find(|l| l.starts_with("- **Lage:**")) {
        if !lage.contains("(gemessen") {
            gaps.push(format!("{}  lage unstamped", name));
        }
    }
}

fn is_offen_heading(line: &str) -> bool {
    let Some(rest) = line.trim_start().strip_prefix("## ") else {
        return false;
    };
    let rest = rest.trim_start();
    let low = rest.to_ascii_lowercase();
    low.starts_with("offen")
        && low["offen".len()..]
            .chars()
            .next()
            .map(|c| !c.is_alphanumeric())
            .unwrap_or(true)
}

fn is_h2_heading(line: &str) -> bool {
    line.trim_start().starts_with("## ")
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
        assert!(normalize("/home/operator/x").is_none());
        assert!(normalize("src").is_none());
        assert!(normalize("the").is_none());
    }

    #[test]
    fn ignores_globs_and_register_class_keys() {
        assert!(normalize("phi/*.\u{3c6}").is_none());
        assert!(normalize("phi/blocked_sources.\u{3c6}::gap:unit-auto-detect").is_none());
        assert_eq!(
            normalize("phi/blocked_sources.\u{3c6}").as_deref(),
            Some("phi/blocked_sources.\u{3c6}")
        );
    }

    #[test]
    fn flags_second_mail_home() {
        let base = std::env::temp_dir().join(format!("opc-guard-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("state/funding/mail")).unwrap();
        let g = mail_home_guardians(&base);
        assert!(g.iter().any(|s| s.contains("zweites Mail-Heim")));
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn flags_symlinked_mail_home() {
        let base = std::env::temp_dir().join(format!("opc-guard-sym-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("state")).unwrap();
        std::os::unix::fs::symlink(base.join("elsewhere"), base.join("state/mail")).unwrap();
        let g = mail_home_guardians(&base);
        assert!(g.iter().any(|s| s.contains("Symlink")));
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn accepts_well_formed_point() {
        let content = "## Offen\n\n### Punkt A\n- **Status:** eigen | **Bindung:** eigen\n- **Trigger:** now\n- **Lage:** offen (gemessen 2026-09-24 via sgrep)\n- **Blockade:** keine\n- **Braucht:** step\n";
        let gaps = point_format_gaps(content).unwrap();
        assert!(gaps.is_empty(), "{:?}", gaps);
    }

    #[test]
    fn flags_missing_braucht() {
        let content = "## Offen\n\n### Punkt A\n- **Status:** eigen\n- **Trigger:** now\n- **Lage:** offen (gemessen 2026-09-24 via sgrep)\n- **Blockade:** keine\n";
        let gaps = point_format_gaps(content).unwrap();
        assert_eq!(gaps.len(), 1);
        assert!(gaps[0].contains("Braucht"), "{:?}", gaps);
    }

    #[test]
    fn flags_unstamped_lage() {
        let content = "## Offen\n\n### Punkt A\n- **Status:** eigen\n- **Trigger:** now\n- **Lage:** offen\n- **Blockade:** keine\n- **Braucht:** step\n";
        let gaps = point_format_gaps(content).unwrap();
        assert_eq!(gaps.len(), 1);
        assert!(gaps[0].contains("lage unstamped"), "{:?}", gaps);
    }

    #[test]
    fn skips_without_offen_section() {
        assert!(point_format_gaps("# Titel\n\nText ohne Punkte\n").is_none());
    }

    #[test]
    fn reports_owner_drift_bindung_linie_fremd() {
        let base = std::env::temp_dir().join(format!("opc-drift-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        fs::write(
            base.join("docs/handover/handover-2026-09-24-river-folge5.md"),
            "- **Bindung:** linie:mountain\n",
        )
        .unwrap();
        fs::write(
            base.join("docs/handover/handover-2026-09-24-river-folge6.md"),
            "- **Bindung:** linie:river\n",
        )
        .unwrap();
        let d = owner_drift(&base);
        assert_eq!(d.len(), 1, "{:?}", d);
        assert!(d[0].contains("river-folge5"), "{:?}", d);
        assert!(d[0].contains("linie:mountain"), "{:?}", d);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn reports_post_md_resurrected() {
        let base = std::env::temp_dir().join(format!("opc-post-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        assert!(!post_md_resurrected(&base));
        fs::write(base.join("docs/handover/post.md"), "a point travels here").unwrap();
        assert!(post_md_resurrected(&base));
        let _ = fs::remove_dir_all(&base);
    }
}
