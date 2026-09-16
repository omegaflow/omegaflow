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

const ZUSTAND_PATH: &str = "docs/zustand/external-state.md";
const POST_PATH: &str = "docs/handover/post.md";

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

enum ZustandStatus {
    Due,
    NotDue,
    Pending,
}

fn is_post_line(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with("An ") && t.contains(':')
}

fn split_table_row(line: &str) -> Option<Vec<String>> {
    let t = line.trim();
    if !t.starts_with('|') || !t.ends_with('|') || t.len() < 2 {
        return None;
    }
    let inner = &t[1..t.len() - 1];
    Some(inner.split('|').map(|c| c.trim().to_string()).collect())
}

fn is_table_separator(cells: &[String]) -> bool {
    !cells.is_empty()
        && cells
            .iter()
            .all(|c| !c.is_empty() && c.chars().all(|ch| ch == '-' || ch == ':'))
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn parse_measured_at(value: &str) -> Option<i64> {
    let mut fields = value.split_whitespace();
    let date = fields.next()?;
    let mut parts = date.split('-');
    let y: i64 = parts.next()?.parse().ok()?;
    let m: i64 = parts.next()?.parse().ok()?;
    let d: i64 = parts.next()?.parse().ok()?;
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    let mut minutes = days_from_civil(y, m, d) * 1440;
    if let Some(clock) = fields.next() {
        let mut hm = clock.split(':');
        let h: i64 = hm.next()?.parse().ok()?;
        let mi: i64 = hm.next()?.parse().ok()?;
        minutes += h * 60 + mi;
    }
    Some(minutes)
}

fn interval_minutes(faellig_lower: &str) -> Option<i64> {
    let f = faellig_lower.replace("2\u{2076}", "64");
    let bytes = f.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if !bytes[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        let value: i64 = match f[start..i].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if f[i..].trim_start().starts_with("min") {
            return Some(value);
        }
    }
    None
}

fn now_minutes() -> Option<i64> {
    let elapsed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?;
    Some((elapsed.as_secs() / 60) as i64)
}

fn current_head_short() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let sha = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if sha.is_empty() {
        None
    } else {
        Some(sha)
    }
}

fn zustand_status(
    measured_at: &str,
    faellig: &str,
    head: Option<&str>,
    now_min: Option<i64>,
) -> ZustandStatus {
    let f = faellig.to_lowercase();
    if f.contains("head") {
        let measured = measured_at.trim();
        return match head {
            Some(h) if !measured.is_empty() => {
                if h.starts_with(measured) || measured.starts_with(h) {
                    ZustandStatus::NotDue
                } else {
                    ZustandStatus::Due
                }
            }
            _ => ZustandStatus::Pending,
        };
    }
    if let Some(interval) = interval_minutes(&f) {
        return match (parse_measured_at(measured_at), now_min) {
            (Some(m), Some(now)) if now - m >= interval => ZustandStatus::Due,
            (Some(_), Some(_)) => ZustandStatus::NotDue,
            _ => ZustandStatus::Pending,
        };
    }
    ZustandStatus::Pending
}

fn scan_zustand(
    path: &Path,
    head: Option<&str>,
    now_min: Option<i64>,
    out: &mut Vec<String>,
) -> usize {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    let mut n = 0;
    for (idx, line) in text.lines().enumerate() {
        let cells = match split_table_row(line) {
            Some(c) => c,
            None => continue,
        };
        if cells.len() < 4 || is_table_separator(&cells) {
            continue;
        }
        if cells[0].to_lowercase().contains("abh\u{e4}ngigkeit") {
            continue;
        }
        let status = match zustand_status(&cells[2], &cells[3], head, now_min) {
            ZustandStatus::Due => "DUE",
            ZustandStatus::NotDue => continue,
            ZustandStatus::Pending => "PENDING",
        };
        let schritt = cells.get(4).map(|s| s.as_str()).unwrap_or("");
        out.push(format!(
            "ZUSTAND\t{}:{}\t{}\t{} | {}",
            path.display(),
            idx + 1,
            status,
            snippet(&cells[0], 60),
            snippet(schritt, 120)
        ));
        n += 1;
    }
    n
}

fn scan_post(path: &Path, out: &mut Vec<String>) -> usize {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    let mut n = 0;
    for (idx, line) in text.lines().enumerate() {
        if !is_post_line(line) {
            continue;
        }
        out.push(format!(
            "POST\t{}:{}\t{}",
            path.display(),
            idx + 1,
            snippet(line, 160)
        ));
        n += 1;
    }
    n
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

    let head = current_head_short();
    let now_min = now_minutes();
    let mut zustand_out: Vec<String> = Vec::new();
    let mut post_out: Vec<String> = Vec::new();
    let zustand = scan_zustand(
        Path::new(ZUSTAND_PATH),
        head.as_deref(),
        now_min,
        &mut zustand_out,
    );
    let post = scan_post(Path::new(POST_PATH), &mut post_out);

    for line in &docs {
        println!("{}", line);
    }
    for line in &unverifiable {
        println!("{}", line);
    }
    for line in &opens {
        println!("{}", line);
    }
    for line in &zustand_out {
        println!("{}", line);
    }
    for line in &post_out {
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
        "register_lookup --live: {} docs, {} open lines, {} released lines, {} duplicates, {} unverifiable, {} zustand due, {} post open [{}]",
        docs.len(),
        opens.len(),
        released.len(),
        dups.len(),
        unverifiable.len(),
        zustand,
        post,
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

fn history_term(args: &[String]) -> Option<&str> {
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--term" {
            return it.next().map(|s| s.as_str());
        }
        if a == "--legacy" {
            it.next();
            continue;
        }
        if a == "--history" || a.starts_with("--") {
            continue;
        }
        return Some(a.as_str());
    }
    None
}

fn removed_lines(repo: Option<&str>, sha: &str, term: &str) -> Vec<String> {
    let mut cmd = Command::new("git");
    if let Some(r) = repo {
        cmd.arg("-C").arg(r);
    }
    let output = match cmd.arg("show").arg(sha).arg("--").arg("docs").output() {
        Ok(o) => o,
        Err(_) => return Vec::new(),
    };
    if !output.status.success() {
        return Vec::new();
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut out = Vec::new();
    for line in stdout.lines() {
        if line.starts_with('-') && !line.starts_with("---") && line.contains(term) {
            out.push(line.to_string());
        }
    }
    out
}

fn scan_rewritten(repo: Option<&str>, term: &str) {
    let mut cmd = Command::new("git");
    if let Some(r) = repo {
        cmd.arg("-C").arg(r);
    }
    cmd.arg("log")
        .arg("--oneline")
        .arg("--all")
        .arg(format!("-S{term}"))
        .arg("--")
        .arg("docs");
    let output = match cmd.output() {
        Ok(o) => o,
        Err(_) => {
            println!("REWRITTEN\t{}\tabsent: git command not available", term);
            return;
        }
    };
    if !output.status.success() {
        println!(
            "REWRITTEN\t{}\tabsent: git log -S returned no listing",
            term
        );
        return;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut shas: Vec<String> = Vec::new();
    let mut n = 0usize;
    for line in stdout.lines() {
        println!("REWRITTEN\t{}\t{}", term, snippet(line, 160));
        n += 1;
        if let Some(sha) = line.split_whitespace().next() {
            shas.push(sha.to_string());
        }
    }
    for sha in &shas {
        let removed = removed_lines(repo, sha, term);
        if removed.is_empty() {
            continue;
        }
        println!("REMOVED\t{}\t{}", sha, term);
        for r in &removed {
            println!("REMOVED\t{}\t{}", term, snippet(r, 160));
        }
        break;
    }
    println!(
        "register_lookup --history {}: {} rewritten-history hit(s)",
        term, n
    );
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
    if let Some(term) = history_term(args) {
        scan_rewritten(repo, term);
    }
    println!(
        "register_lookup --history: {} hits, {} absent, blind spot: lines that vanished inside a rewritten (not deleted) file need --history <term> (git log -S)",
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
        "usage: register_lookup <term>...   (queries the live register: is X already measured/registered?)\n       register_lookup --live            (digest: open points across all live prose documents)\n       register_lookup --history [--legacy <path>] [<term>]   (open points in archived + deleted documents; <term> adds git log -S over rewritten files)"
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

    #[test]
    fn post_line_is_an_address_with_a_step() {
        assert!(is_post_line("An bau: tree red (step: fix)"));
        assert!(is_post_line("  An line: X"));
        assert!(!is_post_line("A note to a line stands here"));
        assert!(!is_post_line("## Post"));
    }

    #[test]
    fn table_row_splits_into_cells() {
        let cells = split_table_row("| a | b | c | d | e |").unwrap();
        assert_eq!(cells, vec!["a", "b", "c", "d", "e"]);
        assert!(split_table_row("plain prose").is_none());
        assert!(is_table_separator(&cells) == false);
        let sep = split_table_row("|---|---|---|").unwrap();
        assert!(is_table_separator(&sep));
    }

    #[test]
    fn interval_minutes_reads_superscript_six() {
        assert_eq!(interval_minutes("new entry or 2\u{2076} min"), Some(64));
        assert_eq!(interval_minutes("all 30 min"), Some(30));
        assert_eq!(interval_minutes("on head change"), None);
    }

    #[test]
    fn measured_at_parses_date_and_clock() {
        let a = parse_measured_at("2026-09-16 07:42").unwrap();
        let b = parse_measured_at("2026-09-17").unwrap();
        assert_eq!(b - a, 1440 - (7 * 60 + 42));
        assert_eq!(parse_measured_at("ce367dd0"), None);
    }

    #[test]
    fn zustand_head_trigger_is_due_when_sha_moved() {
        assert!(matches!(
            zustand_status("ce367dd0", "HEAD change", Some("54bb9b9a"), None),
            ZustandStatus::Due
        ));
        assert!(matches!(
            zustand_status("54bb9b9a", "HEAD change", Some("54bb9b9a"), None),
            ZustandStatus::NotDue
        ));
        assert!(matches!(
            zustand_status("ce367dd0", "HEAD change", None, None),
            ZustandStatus::Pending
        ));
    }

    #[test]
    fn zustand_time_trigger_is_due_after_interval() {
        let measured = parse_measured_at("2026-09-16 07:42").unwrap();
        assert!(matches!(
            zustand_status(
                "2026-09-16 07:42",
                "2\u{2076} min",
                None,
                Some(measured + 64)
            ),
            ZustandStatus::Due
        ));
        assert!(matches!(
            zustand_status(
                "2026-09-16 07:42",
                "2\u{2076} min",
                None,
                Some(measured + 63)
            ),
            ZustandStatus::NotDue
        ));
        assert!(matches!(
            zustand_status("2026-09-16 07:42", "2\u{2076} min", None, None),
            ZustandStatus::Pending
        ));
    }

    #[test]
    fn zustand_unknown_trigger_is_pending_not_zero() {
        assert!(matches!(
            zustand_status("2026-09-16", "new ledger entry", Some("54bb9b9a"), Some(0)),
            ZustandStatus::Pending
        ));
    }
}
