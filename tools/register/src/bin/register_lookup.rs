use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const REGISTER: &[(&str, &str)] = &[
    ("phi/sources.\u{3c6}", "live"),
    ("phi/dead_sources.\u{3c6}", "dead"),
    ("phi/declined_sources.\u{3c6}", "declined"),
    ("phi/blocked_sources.\u{3c6}", "blocked"),
    ("phi/witnesses.\u{3c6}", "witness"),
    ("phi/pipeline/ledger.\u{3c6}", "ledger"),
];

const REGISTER_DIRS: &[(&str, &str)] = &[
    ("docs/handover", "handover"),
    ("docs/surveys", "survey"),
    ("docs/plans", "plan"),
    ("docs/auftrag", "auftrag"),
    ("docs/blatt", "blatt"),
    ("docs/concepts", "concept"),
    ("docs/paper", "paper"),
];

const ARCHIV_DIRS: &[&str] = &[
    "docs/handover/archiv",
    "docs/surveys/archiv",
    "docs/auftrag/archiv",
    "docs/blatt/archiv",
];

const OPEN_MARKERS: &[&str] = &[
    "offen",
    "pending",
    "wartet",
    "ausstehend",
    "n\u{e4}chster schritt",
    "n\u{e4}chsten schritt",
    "naechster schritt",
    "naechsten schritt",
    "wiedervorlage",
    "blocked",
    "request-only",
];

const RELEASED_MARKERS: &[&str] = &["descoped"];

fn snippet(line: &str, max: usize) -> String {
    let trimmed = line.trim();
    if trimmed.chars().count() <= max {
        return trimmed.to_string();
    }
    let mut out: String = trimmed.chars().take(max).collect();
    out.push('\u{2026}');
    out
}

fn file_name_string(path: &Path) -> String {
    match path.file_name() {
        Some(f) => f.to_string_lossy().to_string(),
        None => String::new(),
    }
}

fn is_doc_name(name: &str) -> bool {
    name.ends_with(".md") && !name.starts_with('_')
}

fn open_marker_matches(line: &str) -> bool {
    let lower = line.to_lowercase();
    OPEN_MARKERS.iter().any(|m| lower.contains(m))
}

fn released_marker_matches(line: &str) -> bool {
    let lower = line.to_lowercase();
    RELEASED_MARKERS.iter().any(|m| lower.contains(m))
}

fn has_header(text: &str) -> bool {
    match text.find("<!--") {
        Some(open) => text[open..].find("-->").is_some(),
        None => false,
    }
}

fn header_line_count(text: &str) -> usize {
    let open = match text.find("<!--") {
        Some(i) => i,
        None => return 0,
    };
    let close = match text[open..].find("-->") {
        Some(i) => open + i,
        None => return 0,
    };
    text[..close + 3].lines().count()
}

fn parse_header(text: &str) -> (String, String, String) {
    let mut class = String::new();
    let mut date = String::new();
    let mut status = String::new();
    let open = match text.find("<!--") {
        Some(i) => i,
        None => return (class, date, status),
    };
    let close = match text[open..].find("-->") {
        Some(i) => open + i,
        None => return (class, date, status),
    };
    let block = &text[open + 4..close];
    for line in block.lines() {
        let line = line.trim();
        let (key, value) = match line.split_once(':') {
            Some(pair) => pair,
            None => continue,
        };
        match key.trim() {
            "class" => class = value.trim().to_string(),
            "date" => date = value.trim().to_string(),
            "status" => status = value.trim().to_string(),
            _ => {}
        }
    }
    (class, date, status)
}

fn is_closed_status(status: &str) -> bool {
    status == "consumed" || status == "archived" || status == "done"
}

fn scan_markers(
    text: &str,
    path: &str,
    open_prefix: &str,
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
) {
    let header_lines = header_line_count(text);
    for (idx, line) in text.lines().enumerate() {
        if idx < header_lines {
            continue;
        }
        if released_marker_matches(line) {
            released_out.push(format!(
                "RELEASED\t{}:{}\t{}",
                path,
                idx + 1,
                snippet(line, 160)
            ));
        } else if open_marker_matches(line) {
            open_out.push(format!(
                "{}\t{}:{}\t{}",
                open_prefix,
                path,
                idx + 1,
                snippet(line, 160)
            ));
        }
    }
}

fn collect_archiv_basenames() -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for dir in ARCHIV_DIRS {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let name = file_name_string(&path);
            if name.ends_with(".md") {
                map.insert(name, path.to_string_lossy().to_string());
            }
        }
    }
    map
}

fn find_duplicate<'a>(name: &str, archiv: &'a BTreeMap<String, String>) -> Option<&'a str> {
    archiv.get(name).map(|s| s.as_str())
}

fn bump_class(counts: &mut Vec<(String, usize)>, class: &str) {
    for (c, n) in counts.iter_mut() {
        if c == class {
            *n += 1;
            return;
        }
    }
    counts.push((class.to_string(), 1));
}

fn run_live() {
    let archiv = collect_archiv_basenames();
    let mut docs: Vec<String> = Vec::new();
    let mut opens: Vec<String> = Vec::new();
    let mut released: Vec<String> = Vec::new();
    let mut unverifiable: Vec<String> = Vec::new();
    let mut dups: Vec<String> = Vec::new();
    let mut class_counts: Vec<(String, usize)> = Vec::new();

    for (dir, dir_class) in REGISTER_DIRS {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
        paths.sort();
        for path in paths {
            if !path.is_file() {
                continue;
            }
            let name = file_name_string(&path);
            if !is_doc_name(&name) {
                continue;
            }
            let text = match fs::read_to_string(&path) {
                Ok(t) => t,
                Err(_) => continue,
            };
            let path_str = path.to_string_lossy().to_string();

            if let Some(archiv_path) = find_duplicate(&name, &archiv) {
                dups.push(format!("DUP\t{}\t{}", path_str, archiv_path));
            }

            let (class, date, status) = parse_header(&text);
            if is_closed_status(&status) {
                continue;
            }

            if !has_header(&text) {
                unverifiable.push(format!("UNVERIFIABLE\t{}", path_str));
                scan_markers(&text, &path_str, "OPEN", &mut opens, &mut released);
                continue;
            }

            let class = if class.is_empty() {
                (*dir_class).to_string()
            } else {
                class
            };
            docs.push(format!(
                "DOC\t{}\t{}\t{}\t{}",
                class, date, status, path_str
            ));
            bump_class(&mut class_counts, &class);
            scan_markers(&text, &path_str, "OPEN", &mut opens, &mut released);
        }
    }

    for line in &docs {
        println!("{}", line);
    }
    for line in &unverifiable {
        println!("{}", line);
    }
    for line in &opens {
        println!("{}", line);
    }
    for line in &released {
        println!("{}", line);
    }
    for line in &dups {
        println!("{}", line);
    }
    let summary: Vec<String> = class_counts
        .iter()
        .map(|(c, n)| format!("{} {}", c, n))
        .collect();
    println!(
        "register_lookup --live: {} docs, {} open lines, {} released lines, {} duplicates, {} unverifiable [{}]",
        docs.len(),
        opens.len(),
        released.len(),
        dups.len(),
        unverifiable.len(),
        summary.join(", ")
    );
}

fn scan_archiv_dir(
    dir: &Path,
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
    absent_out: &mut Vec<String>,
) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        if !path.is_file() {
            continue;
        }
        let name = file_name_string(&path);
        if !is_doc_name(&name) {
            continue;
        }
        let path_str = path.to_string_lossy().to_string();
        let text = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(_) => {
                absent_out.push(format!("OPEN\t{}\tabsent: file not readable", path_str));
                continue;
            }
        };
        scan_markers(&text, &path_str, "OPEN", open_out, released_out);
    }
}

fn is_hex_sha(line: &str) -> bool {
    (line.len() == 40 || line.len() == 64) && line.bytes().all(|b| b.is_ascii_hexdigit())
}

fn recover_deleted(
    repo: Option<&str>,
    sha: &str,
    path: &str,
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
    absent_out: &mut Vec<String>,
) {
    let spec = format!("{}^:{}", sha, path);
    let mut cmd = Command::new("git");
    if let Some(r) = repo {
        cmd.arg("-C").arg(r);
    }
    let output = match cmd.arg("show").arg(&spec).output() {
        Ok(o) => o,
        Err(_) => {
            absent_out.push(format!("HISTORY\t{}\tabsent: git show not available", path));
            return;
        }
    };
    if !output.status.success() {
        absent_out.push(format!(
            "HISTORY\t{}\tabsent: content not present at {}",
            path, spec
        ));
        return;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    scan_markers(&text, path, "HISTORY", open_out, released_out);
}

fn scan_deleted(
    repo: Option<&str>,
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
    absent_out: &mut Vec<String>,
) {
    let mut cmd = Command::new("git");
    if let Some(r) = repo {
        cmd.arg("-C").arg(r);
    }
    cmd.args([
        "log",
        "--diff-filter=D",
        "--name-only",
        "--pretty=format:%H",
    ]);
    let output = match cmd.output() {
        Ok(o) => o,
        Err(_) => {
            absent_out.push("HISTORY\t-\tabsent: git command not available".to_string());
            return;
        }
    };
    if !output.status.success() {
        absent_out.push("HISTORY\t-\tabsent: git log returned no listing".to_string());
        return;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut sha: Option<&str> = None;
    for line in stdout.lines() {
        if is_hex_sha(line) {
            sha = Some(line);
            continue;
        }
        if !line.ends_with(".md") {
            continue;
        }
        let path = line.to_string();
        match sha {
            Some(s) => recover_deleted(repo, s, &path, open_out, released_out, absent_out),
            None => absent_out.push(format!(
                "HISTORY\t{}\tabsent: deleting commit not listed",
                path
            )),
        }
    }
}

fn legacy_repo(args: &[String]) -> Option<&str> {
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--legacy" {
            return it.next().map(|s| s.as_str());
        }
    }
    None
}

fn run_history(args: &[String]) {
    let repo = legacy_repo(args);
    let mut open_out: Vec<String> = Vec::new();
    let mut released_out: Vec<String> = Vec::new();
    let mut absent_out: Vec<String> = Vec::new();
    for sub in ARCHIV_DIRS {
        let dir_path = match repo {
            Some(r) => Path::new(r).join(sub),
            None => PathBuf::from(sub),
        };
        scan_archiv_dir(&dir_path, &mut open_out, &mut released_out, &mut absent_out);
    }
    scan_deleted(repo, &mut open_out, &mut released_out, &mut absent_out);
    for line in &open_out {
        println!("{}", line);
    }
    for line in &released_out {
        println!("{}", line);
    }
    for line in &absent_out {
        println!("{}", line);
    }
    println!(
        "register_lookup --history: {} hits, {} absent, blind spot: lines that vanished inside a rewritten (not deleted) file are invisible to this scan",
        open_out.len() + released_out.len(),
        absent_out.len()
    );
}

fn scan_file(path: &Path, class: &str, terms: &[String], out: &mut Vec<String>) -> usize {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    let mut n = 0;
    for (idx, line) in text.lines().enumerate() {
        let lower = line.to_lowercase();
        if terms.iter().any(|t| lower.contains(t)) {
            out.push(format!(
                "{}\t{}:{}\t{}",
                class,
                path.display(),
                idx + 1,
                snippet(line, 160)
            ));
            n += 1;
        }
    }
    n
}

fn scan_dir(dir: &Path, class: &str, terms: &[String], out: &mut Vec<String>) -> usize {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    let mut n = 0;
    for path in paths {
        if !path.is_file() {
            continue;
        }
        let name = match path.file_name() {
            Some(f) => f.to_string_lossy().to_string(),
            None => continue,
        };
        if name.ends_with(".md") {
            n += scan_file(&path, class, terms, out);
        }
    }
    n
}

fn print_usage() -> ! {
    eprintln!(
        "usage: register_lookup <term>...   (queries the live register: is X already measured/registered?)\n       register_lookup --live            (digest: open points across all live prose documents)\n       register_lookup --history [--legacy <path>]   (open points in archived + deleted documents)"
    );
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--live") {
        run_live();
        return;
    }
    if args.iter().any(|a| a == "--history") {
        run_history(&args);
        return;
    }
    let terms: Vec<String> = args
        .iter()
        .filter(|a| !a.starts_with("--"))
        .map(|a| a.to_lowercase())
        .collect();
    if terms.is_empty() {
        print_usage();
    }

    let mut out = Vec::new();
    let mut by_class: Vec<(&str, usize)> = Vec::new();
    for (path, class) in REGISTER {
        let n = scan_file(Path::new(path), class, &terms, &mut out);
        if n > 0 {
            by_class.push((class, n));
        }
    }
    for (dir, class) in REGISTER_DIRS {
        let n = scan_dir(Path::new(dir), class, &terms, &mut out);
        if n > 0 {
            by_class.push((class, n));
        }
    }

    for line in &out {
        println!("{}", line);
    }
    let summary: Vec<String> = by_class
        .iter()
        .map(|(class, n)| format!("{} {}", class, n))
        .collect();
    println!(
        "register_lookup: {} hits for {:?} [{}]",
        out.len(),
        terms,
        summary.join(", ")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snippet_leaves_short_lines_alone() {
        assert_eq!(snippet("  a short line  ", 200), "a short line");
    }

    #[test]
    fn snippet_cuts_on_char_boundary() {
        let out = snippet(&"\u{4e2d}".repeat(300), 10);
        assert_eq!(out.chars().count(), 11);
        assert!(out.ends_with('\u{2026}'));
    }

    #[test]
    fn scan_file_matches_a_term_case_insensitively() {
        let dir = env::temp_dir();
        let path = dir.join(format!("register_lookup_{}.md", std::process::id()));
        fs::write(&path, "the Hilbert transform\nplain line\n").unwrap();
        let mut out = Vec::new();
        let n = scan_file(&path, "test", &["hilbert".to_string()], &mut out);
        assert_eq!(n, 1);
        assert!(out[0].starts_with("test\t"));
        assert!(out[0].contains("Hilbert"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn open_marker_matches_open_line_and_rejects_closed() {
        assert!(open_marker_matches("offen: X"));
        assert!(!open_marker_matches("closed and finished"));
    }

    #[test]
    fn todo_is_not_an_open_marker() {
        assert!(!open_marker_matches("todo: check this"));
    }

    #[test]
    fn descoped_is_released_not_open() {
        assert!(released_marker_matches("descoped: never built, not needed"));
        assert!(!open_marker_matches("descoped: never built, not needed"));
        assert!(!released_marker_matches("offen: X"));
    }

    #[test]
    fn parse_header_extracts_fields() {
        let text =
            "<!--\n  title: t\n  class: handover\n  date: 2026-09-15\n  status: live\n-->\nbody";
        let (class, date, status) = parse_header(text);
        assert_eq!(class, "handover");
        assert_eq!(date, "2026-09-15");
        assert_eq!(status, "live");
    }

    #[test]
    fn has_header_true_with_block_and_false_without() {
        assert!(has_header("<!--\nclass: x\n-->\nbody"));
        assert!(!has_header("just a body\nno header here\n"));
        assert!(!has_header("<!--\nunclosed header\n"));
    }

    #[test]
    fn scan_markers_splits_released_from_open() {
        let text = "<!--\nstatus: live\n-->\noffen: a\nbody\ndescoped: b\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        scan_markers(text, "p.md", "OPEN", &mut open_out, &mut released_out);
        assert_eq!(open_out.len(), 1);
        assert!(open_out[0].starts_with("OPEN\tp.md:4\t"));
        assert!(open_out[0].contains("offen: a"));
        assert_eq!(released_out.len(), 1);
        assert!(released_out[0].starts_with("RELEASED\tp.md:6\t"));
        assert!(released_out[0].contains("descoped: b"));
    }

    #[test]
    fn headerless_scan_still_reads_body() {
        let text = "no header block here\noffen: x\n";
        assert!(!has_header(text));
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        scan_markers(text, "p.md", "OPEN", &mut open_out, &mut released_out);
        assert_eq!(open_out.len(), 1);
        assert!(open_out[0].starts_with("OPEN\tp.md:2\t"));
        assert!(open_out[0].contains("offen: x"));
    }

    #[test]
    fn duplicate_flags_basename_in_both() {
        let mut archiv = BTreeMap::new();
        archiv.insert(
            "handover-2026-09-01-x.md".to_string(),
            "docs/handover/archiv/handover-2026-09-01-x.md".to_string(),
        );
        assert_eq!(
            find_duplicate("handover-2026-09-01-x.md", &archiv),
            Some("docs/handover/archiv/handover-2026-09-01-x.md")
        );
        assert_eq!(find_duplicate("other.md", &archiv), None);
    }

    #[test]
    fn template_is_not_a_doc() {
        assert!(is_doc_name("handover-2026-09-15-x.md"));
        assert!(!is_doc_name("_template.md"));
        assert!(!is_doc_name("README.txt"));
    }
}
