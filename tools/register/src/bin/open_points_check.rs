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
    let stale = stale_citations(&root, &rel, &content);
    for s in &stale {
        println!("{}", s);
    }
    let done = done_carried(&rel, &content);
    for d in &done {
        println!("{}", d);
    }
    let words = word_carried(&root, &rel, &content);
    for w in &words {
        println!("{}", w);
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
        "open_points_check: {}  | {} path refs | {} absent | {} stale-citations | {} done-carried | {} word-carried | {} guardians | {} format-gaps | {} owner-drift | {} post-md",
        rel,
        points,
        missing,
        stale.len(),
        done.len(),
        words.len(),
        guardians.len(),
        format_gap_count,
        drifts.len(),
        post_md as usize
    );
    if !done.is_empty() || !stale.is_empty() || !words.is_empty() {
        std::process::exit(1);
    }
}

fn mail_home_guardians(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    if !root.join("state").exists() {
        return out;
    }
    if root.join("state/future/mail").exists() {
        out.push(
            "zweites Mail-Heim: state/future/mail/ existiert (der Postkorb ist state/mail/)"
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

const CITATION_REGISTERS: [(&str, &str, bool); 6] = [
    (".secrets.local", ".secrets.local", true),
    (
        "mail_ledger.\u{3c6}",
        "state/mail/mail_ledger.\u{3c6}",
        false,
    ),
    (
        "sent_ledger.\u{3c6}",
        "state/mail/sent_ledger.\u{3c6}",
        false,
    ),
    (
        "external-state.md",
        "state/zustand/external-state.md",
        false,
    ),
    ("wartend.\u{3c6}", "state/zustand/wartend.\u{3c6}", false),
    ("standing-pass.md", "state/zustand/standing-pass.md", false),
];

fn trim_citation(token: &str) -> &str {
    let t = token.trim();
    let t = t.trim_start_matches(|c: char| "`'\"([{<".contains(c));
    t.trim_end_matches(|c: char| "`'\"»>)]},;.|*".contains(c))
}

fn resolve_citation(token: &str) -> Option<(String, usize, bool)> {
    let t = trim_citation(token);
    let colon = t.rfind(':')?;
    let n = t[colon + 1..].parse::<usize>().ok()?;
    if n == 0 {
        return None;
    }
    let path = trim_citation(&t[..colon]);
    for (suffix, canonical, secrets) in CITATION_REGISTERS {
        if path == suffix || path == canonical || path.ends_with(&format!("/{}", suffix)) {
            return Some((canonical.to_string(), n, secrets));
        }
    }
    None
}

fn citation_line_ok(line: &str, secrets: bool) -> bool {
    if !secrets {
        return true;
    }
    match line.split_once('=') {
        Some((key, _)) => {
            !key.is_empty()
                && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                && key.chars().any(|c| c.is_ascii_uppercase())
        }
        None => false,
    }
}

fn stale_citations(root: &Path, rel: &str, content: &str) -> Vec<String> {
    let mut out = Vec::new();
    if !root.join("state").exists() {
        return out;
    }
    for (idx, line) in content.lines().enumerate() {
        for word in line.split_whitespace() {
            let Some((canonical, n, secrets)) = resolve_citation(word) else {
                continue;
            };
            let ok = match fs::read_to_string(root.join(&canonical)) {
                Ok(text) => text
                    .lines()
                    .nth(n - 1)
                    .map(|l| citation_line_ok(l, secrets))
                    .unwrap_or(false),
                Err(_) => false,
            };
            if !ok {
                out.push(format!(
                    "STALE-CITATION {}:{} {}:{}",
                    rel,
                    idx + 1,
                    canonical,
                    n
                ));
            }
        }
    }
    out
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

const DONE_MARKERS: [&str; 4] = ["GESENDET", "GESCHLOSSEN", "Erledigt", "Entschieden"];

fn is_open_area_heading(line: &str) -> bool {
    let Some(rest) = line.trim_start().strip_prefix("## ") else {
        return false;
    };
    let low = rest.trim_start().to_ascii_lowercase();
    low.starts_with("offen") || low.starts_with("operator-queue") || low.starts_with("queue")
}

const WORD_STOP: [&str; 34] = [
    "status",
    "trigger",
    "lage",
    "blockade",
    "braucht",
    "bindung",
    "wort",
    "eigen",
    "operator",
    "gesendet",
    "geschlossen",
    "erledigt",
    "entschieden",
    "offen",
    "lock",
    "termin",
    "dass",
    "eine",
    "dieser",
    "http",
    "korrektur",
    "korrigiert",
    "abgrenzung",
    "empfehlung",
    "frage",
    "antwort",
    "nicht",
    "keine",
    "kein",
    "nichts",
    "oder",
    "warten",
    "wartend",
    "regel",
];

fn first_bold_words(line: &str) -> Vec<String> {
    let Some(start) = line.find("**") else {
        return Vec::new();
    };
    let rest = &line[start + 2..];
    let Some(end) = rest.find("**") else {
        return Vec::new();
    };
    rest[..end]
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() >= 4)
        .filter(|w| w.chars().any(|c| c.is_alphabetic()))
        .map(|w| w.to_string())
        .collect()
}

fn word_carried(root: &Path, rel: &str, content: &str) -> Vec<String> {
    let dir = root.join("state/operator-gespraeche");
    let mut corpus: Vec<(String, usize, String)> = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let name = p
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if let Ok(text) = fs::read_to_string(&p) {
                for (i, l) in text.lines().enumerate() {
                    corpus.push((name.clone(), i + 1, l.to_lowercase()));
                }
            }
        }
    }
    if corpus.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut in_area = false;
    for (idx, line) in content.lines().enumerate() {
        if is_h2_heading(line) {
            in_area = is_open_area_heading(line);
            continue;
        }
        if !in_area
            || line.contains("operator-gespraeche")
            || !line.trim_start().starts_with("- **")
        {
            continue;
        }
        let Some(name) = first_bold_words(line)
            .into_iter()
            .find(|w| w.len() >= 5 && !WORD_STOP.contains(&w.to_lowercase().as_str()))
        else {
            continue;
        };
        let low = name.to_lowercase();
        if let Some((file, n, _)) = corpus.iter().find(|(_, _, l)| l.contains(&low)) {
            out.push(format!(
                "WORD-CARRIED {}:{} {} -> {}:{}",
                rel,
                idx + 1,
                name,
                file,
                n
            ));
        }
    }
    out
}

fn done_carried(rel: &str, content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_area = false;
    for (idx, line) in content.lines().enumerate() {
        if is_h2_heading(line) {
            in_area = is_open_area_heading(line);
            continue;
        }
        if !in_area {
            continue;
        }
        for marker in DONE_MARKERS {
            if line.contains(marker) {
                out.push(format!("DONE-CARRIED {}:{} {}", rel, idx + 1, marker));
                break;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_done_work_carried_in_open_area() {
        let doc = "## Offen (eigen)\n- **CSES** — GESCHLOSSEN (2026-09-29).\n- **Offen** — braucht Wort.\n## Haus\n- Gesendet: x\n";
        let hits = done_carried("h.md", doc);
        assert_eq!(hits.len(), 1, "{:?}", hits);
        assert!(hits[0].contains("GESCHLOSSEN"), "{:?}", hits);
    }

    #[test]
    fn done_carried_scans_the_operator_queue() {
        let doc = "## Operator-Queue\n- **NSE** — **GESENDET (Operator 2026-09-29)**.\n";
        assert_eq!(done_carried("h.md", doc).len(), 1);
    }

    #[test]
    fn extracts_bold_keywords() {
        assert!(first_bold_words("- **NCIS-Research-Grant** — x").contains(&"NCIS".to_string()));
    }

    #[test]
    fn done_carried_ignores_non_open_sections_and_lowercase() {
        let doc =
            "## Postlage\n- Gesendet (belegt): Voyager.\n## An river\n- **X** — geschlossen.\n";
        assert!(done_carried("h.md", doc).is_empty());
    }

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
        fs::create_dir_all(base.join("state/future/mail")).unwrap();
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

    #[test]
    fn resolves_register_citations() {
        assert_eq!(
            resolve_citation("state/.secrets.local:23").map(|c| c.0),
            Some(".secrets.local".to_string())
        );
        assert_eq!(
            resolve_citation("`.secrets.local:83`").map(|c| c.0),
            Some(".secrets.local".to_string())
        );
        assert_eq!(
            resolve_citation("`state/mail/mail_ledger.\u{3c6}:191`").map(|c| c.0),
            Some("state/mail/mail_ledger.\u{3c6}".to_string())
        );
        assert_eq!(
            resolve_citation("wartend.\u{3c6}:4.").map(|c| c.0),
            Some("state/zustand/wartend.\u{3c6}".to_string())
        );
        assert!(resolve_citation("docs/paper/gic-causal-driver.md:531").is_none());
        assert!(resolve_citation("src/gate/commit_gate.rs:1804").is_none());
    }

    #[test]
    fn flags_stale_citation_and_accepts_live_one() {
        let base = std::env::temp_dir().join(format!("opc-cite-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("state/mail")).unwrap();
        fs::write(base.join("state/mail/mail_ledger.\u{3c6}"), "a\nb\nc\n").unwrap();
        assert!(
            stale_citations(&base, "h.md", "siehe `state/mail/mail_ledger.\u{3c6}:2`").is_empty()
        );
        let miss = stale_citations(&base, "h.md", "siehe `state/mail/mail_ledger.\u{3c6}:9`");
        assert_eq!(miss.len(), 1, "{:?}", miss);
        assert!(miss[0].contains("STALE-CITATION"), "{:?}", miss);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn citation_check_is_skipped_without_state() {
        let base = std::env::temp_dir().join(format!("opc-nostate-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        assert!(stale_citations(&base, "h.md", "`state/.secrets.local:1`").is_empty());
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn secrets_citation_needs_a_key_name() {
        let base = std::env::temp_dir().join(format!("opc-secret-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("state")).unwrap();
        fs::write(
            base.join(".secrets.local"),
            "CORE_API_KEY=abcdef\n# a comment line\n",
        )
        .unwrap();
        assert!(stale_citations(&base, "h.md", "`.secrets.local:1`").is_empty());
        let bad = stale_citations(&base, "h.md", "`.secrets.local:2`");
        assert_eq!(bad.len(), 1, "{:?}", bad);
        let _ = fs::remove_dir_all(&base);
    }
}
