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

const LEDGER_PATH: &str = "phi/pipeline/ledger.\u{3c6}";
const INDEX_PATH: &str = "phi/pipeline/index.\u{3c6}";
const SOURCES_PATH: &str = "phi/sources.\u{3c6}";
const WITNESSES_PATH: &str = "phi/witnesses.\u{3c6}";
const FOOTPRINTS_PATH: &str = "phi/footprints.\u{3c6}";
const HARVEST_PATH: &str = "phi/harvest.\u{3c6}";
const NRS_PATH: &str = "phi/nrs_stations.\u{3c6}";
const PROBE_PATHS: &[&str] = &[
    "phi/pipeline/probe_wave.\u{3c6}",
    "phi/pipeline/probe_hapi_proposed.\u{3c6}",
    "phi/pipeline/probe_batch_skymap.\u{3c6}",
];
const CATALOG_DIR: &str = "phi/pipeline/catalog";

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
    if sha.is_empty() { None } else { Some(sha) }
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

fn disposition_owner(state: &str) -> Option<&'static str> {
    let mut tokens = state.trim().split_whitespace();
    match tokens.next() {
        Some("parser-def" | "parser-gap") => Some("bau"),
        Some("asset") => match tokens.next() {
            Some("fehlt") => Some("bau"),
            _ => None,
        },
        Some("ausstehend" | "verifiziert" | "kompiliert" | "pending") => Some("ernte"),
        Some("blocked") => match tokens.next() {
            Some("account" | "key") => Some("entscheid"),
            Some("ip-blocked") => Some("ernte"),
            Some("parser-def") => Some("bau"),
            _ => None,
        },
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
enum StateClass {
    Open(&'static str),
    Released,
    Ignored,
}

fn state_class(state: &str) -> Option<StateClass> {
    match state.trim() {
        "ausstehend" | "verifiziert" | "kompiliert" | "pending" | "fehlt" | "offen" | "absent"
        | "review" => Some(StateClass::Open("ernte")),
        "parser-gap" | "asset fehlt" => Some(StateClass::Open("bau")),
        "descoped" | "void" | "disponiert" | "erledigt" | "ausgelagert" | "declined"
        | "refused" => Some(StateClass::Released),
        "asset present" | "index" | "artefakt" | "register" | "infra" | "probe" | "frame"
        | "listen" | "research" => Some(StateClass::Ignored),
        _ => None,
    }
}

fn emit_classified(
    path: &str,
    lineno: usize,
    state: &str,
    step: &str,
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
    open_count: &mut usize,
) {
    match state_class(state) {
        Some(StateClass::Open(owner)) => {
            open_out.push(format!(
                "DISPOSITION\t{}:{}\t[{}] {} | {}",
                path, lineno, owner, state, step
            ));
            *open_count += 1;
        }
        Some(StateClass::Released) => {
            released_out.push(format!(
                "RELEASED\t{}:{}\t{} | {}",
                path, lineno, state, step
            ));
        }
        Some(StateClass::Ignored) => {}
        None => {
            open_out.push(format!(
                "DISPOSITION_UNMAPPED\t{}:{}\t{} | {}",
                path, lineno, state, step
            ));
        }
    }
}

fn scan_dispositions_text(
    text: &str,
    path: &str,
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
) -> usize {
    let mut blocks: Vec<(usize, Vec<(usize, &str)>)> = Vec::new();
    let mut current: Vec<(usize, &str)> = Vec::new();
    let mut block_start = 1usize;
    for (idx, line) in text.lines().enumerate() {
        let lineno = idx + 1;
        if line.trim().is_empty() {
            if !current.is_empty() {
                blocks.push((block_start, std::mem::take(&mut current)));
            }
        } else {
            if current.is_empty() {
                block_start = lineno;
            }
            current.push((lineno, line));
        }
    }
    if !current.is_empty() {
        blocks.push((block_start, current));
    }

    let mut n = 0;
    for (start_line, lines) in blocks {
        let state = lines[0].1.trim();
        if state.is_empty() || state.starts_with("note") {
            continue;
        }
        let note = lines
            .iter()
            .find(|(_, l)| l.trim_start().starts_with("note "))
            .map(|(_, l)| l.trim())
            .unwrap_or("");
        let url = lines
            .iter()
            .find(|(_, l)| l.trim_start().starts_with("url "))
            .map(|(_, l)| l.trim())
            .unwrap_or("");
        let step = if note.is_empty() {
            snippet(url, 160)
        } else {
            snippet(note, 160)
        };
        if state == "descoped" {
            released_out.push(format!(
                "RELEASED\t{}:{}\t{} | {}",
                path, start_line, state, step
            ));
            n += 1;
            continue;
        }
        match disposition_owner(state) {
            Some(owner) => {
                let tag = if state == "pending" && note.to_lowercase().contains("antwort offen") {
                    "wartend"
                } else {
                    owner
                };
                open_out.push(format!(
                    "DISPOSITION\t{}:{}\t[{}] {} | {}",
                    path, start_line, tag, state, step
                ));
            }
            None => open_out.push(format!(
                "DISPOSITION_UNMAPPED\t{}:{}\t{} | {}",
                path, start_line, state, step
            )),
        }
        n += 1;
    }
    n
}

fn scan_dispositions(
    path: &Path,
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
) -> usize {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    scan_dispositions_text(&text, &path.to_string_lossy(), open_out, released_out)
}

fn parse_blocks(text: &str) -> Vec<(usize, Vec<(usize, &str)>)> {
    let mut blocks: Vec<(usize, Vec<(usize, &str)>)> = Vec::new();
    let mut current: Vec<(usize, &str)> = Vec::new();
    let mut block_start = 1usize;
    for (idx, line) in text.lines().enumerate() {
        let lineno = idx + 1;
        if line.trim().is_empty() {
            if !current.is_empty() {
                blocks.push((block_start, std::mem::take(&mut current)));
            }
        } else {
            if current.is_empty() {
                block_start = lineno;
            }
            current.push((lineno, line));
        }
    }
    if !current.is_empty() {
        blocks.push((block_start, current));
    }
    blocks
}

fn scan_state_blocks_text(
    text: &str,
    path: &str,
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
) -> usize {
    let mut open = 0;
    for (start_line, lines) in parse_blocks(text) {
        let state = lines[0].1.trim();
        if state.is_empty() || state.starts_with('#') || state.starts_with("note ") {
            continue;
        }
        let note = lines
            .iter()
            .find(|(_, l)| l.trim_start().starts_with("note "))
            .map(|(_, l)| l.trim())
            .unwrap_or("");
        let step = snippet(note, 120);
        emit_classified(
            path,
            start_line,
            state,
            &step,
            open_out,
            released_out,
            &mut open,
        );
    }
    open
}

fn scan_index_text(
    text: &str,
    path: &str,
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
) -> usize {
    let mut open = 0;
    for (idx, line) in text.lines().enumerate() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let state = match t.split_whitespace().next() {
            Some(s) => s,
            None => continue,
        };
        let step = snippet(t, 120);
        emit_classified(
            path,
            idx + 1,
            state,
            &step,
            open_out,
            released_out,
            &mut open,
        );
    }
    open
}

fn scan_note_markers_text(
    text: &str,
    path: &str,
    open_markers: &[&str],
    released_markers: &[&str],
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
) -> usize {
    let mut open = 0;
    for (idx, line) in text.lines().enumerate() {
        let t = line.trim();
        if !t.starts_with("note ") {
            continue;
        }
        let mut state: Option<&str> = None;
        for m in released_markers {
            if t.contains(*m) {
                state = Some(m);
                break;
            }
        }
        if state.is_none() {
            for m in open_markers {
                if t.contains(*m) {
                    state = Some(m);
                    break;
                }
            }
        }
        if let Some(s) = state {
            let step = snippet(t, 120);
            emit_classified(path, idx + 1, s, &step, open_out, released_out, &mut open);
        }
    }
    open
}

fn scan_probe_text(
    text: &str,
    path: &str,
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
) -> usize {
    let mut open = 0;
    for (idx, line) in text.lines().enumerate() {
        let t = line.trim();
        if !t.starts_with('#') {
            continue;
        }
        let body = t[1..].trim_start();
        let state = if body.starts_with("pending ") {
            Some("pending")
        } else if body.ends_with("review") {
            Some("review")
        } else {
            None
        };
        if let Some(s) = state {
            let step = snippet(t, 120);
            emit_classified(path, idx + 1, s, &step, open_out, released_out, &mut open);
        }
    }
    open
}

fn scan_state_blocks(
    path: &Path,
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
) -> usize {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    scan_state_blocks_text(&text, &path.to_string_lossy(), open_out, released_out)
}

fn scan_index(path: &Path, open_out: &mut Vec<String>, released_out: &mut Vec<String>) -> usize {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    scan_index_text(&text, &path.to_string_lossy(), open_out, released_out)
}

fn scan_note_markers(
    path: &Path,
    open_markers: &[&str],
    released_markers: &[&str],
    open_out: &mut Vec<String>,
    released_out: &mut Vec<String>,
) -> usize {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    scan_note_markers_text(
        &text,
        &path.to_string_lossy(),
        open_markers,
        released_markers,
        open_out,
        released_out,
    )
}

fn scan_probe(path: &Path, open_out: &mut Vec<String>, released_out: &mut Vec<String>) -> usize {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    scan_probe_text(&text, &path.to_string_lossy(), open_out, released_out)
}

fn scan_catalog_candidates(dir: &Path, out: &mut Vec<String>) -> usize {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    let mut total = 0;
    for path in paths {
        if !path.is_file() {
            continue;
        }
        let name = file_name_string(&path);
        if !name.ends_with("\u{3c6}") {
            continue;
        }
        let text = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let mut n = 0usize;
        for line in text.lines() {
            if line.trim_start().starts_with("candidate ") {
                n += 1;
            }
        }
        if n == 0 {
            continue;
        }
        out.push(format!(
            "CANDIDATES\t{}\t{} \u{2192} ernte",
            path.to_string_lossy(),
            n
        ));
        total += n;
    }
    total
}

fn print_section(name: &str, open_count: usize, open_lines: &[String], released_lines: &[String]) {
    println!("{} {} offen", name, open_count);
    for line in open_lines {
        println!("{}", line);
    }
    for line in released_lines {
        println!("{}", line);
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

fn run_open() {
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
    let mut dispo_out: Vec<String> = Vec::new();
    let dispo = scan_dispositions(
        Path::new("phi/blocked_sources.\u{3c6}"),
        &mut dispo_out,
        &mut released,
    );

    for line in &docs {
        println!("{}", line);
    }
    for line in &unverifiable {
        println!("{}", line);
    }
    for line in &opens {
        println!("{}", line);
    }
    for line in &dispo_out {
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

    let mut ledger_open: Vec<String> = Vec::new();
    let mut ledger_released: Vec<String> = Vec::new();
    let ledger = scan_state_blocks(
        Path::new(LEDGER_PATH),
        &mut ledger_open,
        &mut ledger_released,
    );
    print_section("LEDGER", ledger, &ledger_open, &ledger_released);

    let mut index_open: Vec<String> = Vec::new();
    let mut index_released: Vec<String> = Vec::new();
    let index = scan_index(Path::new(INDEX_PATH), &mut index_open, &mut index_released);
    print_section("INDEX", index, &index_open, &index_released);

    let mut sources_open: Vec<String> = Vec::new();
    let mut sources_released: Vec<String> = Vec::new();
    let sources = scan_note_markers(
        Path::new(SOURCES_PATH),
        &["pending", "fehlt", "offen"],
        &["descoped"],
        &mut sources_open,
        &mut sources_released,
    );
    print_section("SOURCES", sources, &sources_open, &sources_released);

    let mut witnesses_open: Vec<String> = Vec::new();
    let mut witnesses_released: Vec<String> = Vec::new();
    let witnesses = scan_note_markers(
        Path::new(WITNESSES_PATH),
        &["pending", "absent"],
        &["declined"],
        &mut witnesses_open,
        &mut witnesses_released,
    );
    print_section("WITNESSES", witnesses, &witnesses_open, &witnesses_released);

    let mut footprints_open: Vec<String> = Vec::new();
    let mut footprints_released: Vec<String> = Vec::new();
    let footprints = scan_note_markers(
        Path::new(FOOTPRINTS_PATH),
        &["pending", "absent"],
        &["refused"],
        &mut footprints_open,
        &mut footprints_released,
    );
    print_section(
        "FOOTPRINTS",
        footprints,
        &footprints_open,
        &footprints_released,
    );

    let mut harvest_open: Vec<String> = Vec::new();
    let mut harvest_released: Vec<String> = Vec::new();
    let harvest = scan_state_blocks(
        Path::new(HARVEST_PATH),
        &mut harvest_open,
        &mut harvest_released,
    );
    print_section("HARVEST", harvest, &harvest_open, &harvest_released);

    let mut nrs_open: Vec<String> = Vec::new();
    let mut nrs_released: Vec<String> = Vec::new();
    let nrs = scan_note_markers(
        Path::new(NRS_PATH),
        &["pending", "absent"],
        &[],
        &mut nrs_open,
        &mut nrs_released,
    );
    print_section("NRS", nrs, &nrs_open, &nrs_released);

    let mut probe_open: Vec<String> = Vec::new();
    let mut probe_released: Vec<String> = Vec::new();
    let mut probe = 0;
    for p in PROBE_PATHS {
        probe += scan_probe(Path::new(p), &mut probe_open, &mut probe_released);
    }
    print_section("PROBES", probe, &probe_open, &probe_released);

    let mut candidates_out: Vec<String> = Vec::new();
    let candidates = scan_catalog_candidates(Path::new(CATALOG_DIR), &mut candidates_out);
    for line in &candidates_out {
        println!("{}", line);
    }

    let summary: Vec<String> = class_counts
        .iter()
        .map(|(c, n)| format!("{} {}", c, n))
        .collect();
    println!(
        "register_lookup --open: {} docs, {} open lines, {} released lines, {} duplicates, {} unverifiable, {} zustand due, {} post open, {} disposition [{}], pipeline: ledger {} open, index {} open, sources {} open, witnesses {} open, footprints {} open, harvest {} open, nrs {} open, probes {} open, {} candidates",
        docs.len(),
        opens.len(),
        released.len(),
        dups.len(),
        unverifiable.len(),
        zustand,
        post,
        dispo,
        summary.join(", "),
        ledger,
        index,
        sources,
        witnesses,
        footprints,
        harvest,
        nrs,
        probe,
        candidates,
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

const HANDOVER_DIRS: &[&str] = &["docs/handover", "docs/handover/archiv"];

const DROPPED_STATUS_TAGS: &[&str] = &[
    "wartend",
    "wartestell",
    "operator-gebunden",
    "blockiert",
    "termin",
    "pending",
    "offen",
    "ausstehend",
];

const DROPPED_STOPWORDS: &[&str] = &[
    "der", "die", "das", "den", "dem", "des", "ein", "eine", "einen", "einem", "eines", "und",
    "oder", "aber", "ist", "sind", "wird", "werden", "wurde", "nicht", "kein", "keine", "fuer",
    "mit", "von", "auf", "aus", "als", "auch", "nur", "noch", "the", "and", "for", "with",
    "from", "that", "this", "into", "over", "after", "punkt", "status", "schritt", "offen",
    "wartend", "blockiert", "pending",
];

struct Handover {
    date: String,
    folge: Option<u32>,
    path: String,
    text: String,
}

struct OpenPoint {
    lineno: usize,
    text: String,
}

fn is_date_field(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[8..].iter().all(u8::is_ascii_digit)
}

fn parse_handover_name(name: &str) -> Option<(String, String, Option<u32>)> {
    let stem = name.strip_suffix(".md")?;
    let rest = stem.strip_prefix("handover-")?;
    if rest.len() < 12 {
        return None;
    }
    let date = &rest[..10];
    if !is_date_field(date) || rest.as_bytes()[10] != b'-' {
        return None;
    }
    let tail = &rest[11..];
    if tail.is_empty() {
        return None;
    }
    let (line, folge) = match tail.rfind("-folge") {
        Some(at) => {
            let line = &tail[..at];
            let after = &tail[at + 6..];
            let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
            (line.to_string(), digits.parse::<u32>().ok())
        }
        None => (tail.to_string(), None),
    };
    if line.is_empty() {
        return None;
    }
    Some((line, date.to_string(), folge))
}

fn normalize_words(text: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_alphanumeric() || ch == '-' {
            for lower in ch.to_lowercase() {
                current.push(lower);
            }
        } else if !current.is_empty() {
            words.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

fn normalize_text(text: &str) -> String {
    normalize_words(text).join(" ")
}

fn is_meaningful_word(word: &str) -> bool {
    word.chars().any(|c| c.is_alphanumeric())
}

fn is_status_word(word: &str) -> bool {
    DROPPED_STATUS_TAGS
        .iter()
        .any(|tag| word == *tag || word.starts_with(tag))
}

fn tag_in_words(words: &[String]) -> bool {
    words.iter().any(|w| is_status_word(w))
}

fn is_container_heading(heading: &str) -> bool {
    if heading.to_lowercase().contains("kein auswahlpunkt") {
        return true;
    }
    normalize_words(heading)
        .iter()
        .filter(|w| is_meaningful_word(w) && !is_status_word(w))
        .count()
        == 0
}

const CONTAINER_TAILS: &[&str] = &[
    "punkte",
    "offen",
    "wartend",
    "termin",
    "benchmark",
    "stehender pass",
    "geteilter baum",
    "abschluss",
    "postfach",
    "ci",
];

fn is_container_text(text: &str) -> bool {
    if is_container_heading(text) {
        return true;
    }
    let lower = normalize_text(text).to_lowercase();
    CONTAINER_TAILS.iter().any(|tail| {
        lower == *tail
            || lower.ends_with(&format!(" {}", tail))
            || lower.ends_with(&format!("-{}", tail))
    })
}

fn strip_bullet_marker(trimmed: &str) -> Option<&str> {
    if let Some(rest) = trimmed.strip_prefix("- ") {
        return Some(rest.trim());
    }
    if let Some(rest) = trimmed.strip_prefix("* ") {
        return Some(rest.trim());
    }
    let digits: String = trimmed.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    trimmed[digits.len()..].strip_prefix(". ").map(str::trim)
}

fn leading_region(body: &str) -> &str {
    match body.find(|c| c == '\u{2014}' || c == '\u{2013}' || c == ':') {
        Some(at) => &body[..at],
        None => body,
    }
}

fn extract_open_points(text: &str) -> Vec<OpenPoint> {
    let mut points: Vec<OpenPoint> = Vec::new();
    let mut section_open = false;
    for (idx, raw) in text.lines().enumerate() {
        let lineno = idx + 1;
        let trimmed = raw.trim_start();
        if let Some(rest) = trimmed.strip_prefix("## ") {
            let heading = rest.trim();
            let open = tag_in_words(&normalize_words(heading))
                || heading.to_lowercase().starts_with("punkt");
            if open && !is_container_heading(heading) {
                points.push(OpenPoint {
                    lineno,
                    text: heading.to_string(),
                });
            }
            section_open = open;
            continue;
        }
        if let Some(cells) = split_table_row(raw) {
            if cells.len() < 2 || is_table_separator(&cells) {
                continue;
            }
            let first = cells[0].trim();
            if first.is_empty() || first.eq_ignore_ascii_case("punkt") {
                continue;
            }
            let row_open =
                section_open || cells.iter().any(|c| tag_in_words(&normalize_words(c)));
            if row_open {
                points.push(OpenPoint {
                    lineno,
                    text: first.to_string(),
                });
            }
            continue;
        }
        if let Some(body) = strip_bullet_marker(trimmed) {
            if section_open || tag_in_words(&normalize_words(leading_region(body))) {
                points.push(OpenPoint {
                    lineno,
                    text: body.to_string(),
                });
            }
        }
    }
    points.retain(|p| !is_container_text(&p.text));
    points
}

fn point_key_tokens(text: &str) -> Vec<String> {
    normalize_words(text)
        .into_iter()
        .filter(|w| is_meaningful_word(w) && !is_status_word(w))
        .collect()
}

fn match_prefix(tokens: &[String]) -> Option<String> {
    if tokens.is_empty() {
        return None;
    }
    let take = tokens.len().min(6);
    Some(tokens[..take].join(" "))
}

fn distinctive_token(tokens: &[String]) -> Option<String> {
    let mut best: Option<&String> = None;
    for word in tokens {
        if !is_meaningful_word(word)
            || word.chars().all(|c| c.is_ascii_digit())
            || DROPPED_STOPWORDS.contains(&word.as_str())
        {
            continue;
        }
        if word.chars().count() >= 5 {
            return Some(word.clone());
        }
        if best.map_or(true, |b| word.chars().count() > b.chars().count()) {
            best = Some(word);
        }
    }
    best.cloned()
}

fn collect_handovers() -> BTreeMap<String, Vec<Handover>> {
    let mut by_line: BTreeMap<String, Vec<Handover>> = BTreeMap::new();
    let mut seen: Vec<String> = Vec::new();
    for dir in HANDOVER_DIRS {
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
            if !name.ends_with(".md") || name.starts_with('_') || name == "post.md" {
                continue;
            }
            if seen.iter().any(|s| s == &name) {
                continue;
            }
            let parsed = match parse_handover_name(&name) {
                Some(p) => p,
                None => continue,
            };
            let text = match fs::read_to_string(&path) {
                Ok(t) => t,
                Err(_) => continue,
            };
            seen.push(name);
            let (line, date, folge) = parsed;
            by_line.entry(line).or_default().push(Handover {
                date,
                folge,
                path: path.to_string_lossy().to_string(),
                text,
            });
        }
    }
    for list in by_line.values_mut() {
        list.sort_by(|a, b| {
            a.date
                .cmp(&b.date)
                .then(a.folge.cmp(&b.folge))
                .then(a.path.cmp(&b.path))
        });
    }
    by_line
}

fn commit_for_path(path: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["log", "--diff-filter=A", "--format=%H", "-1", "--", path])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let first = stdout.lines().next()?.trim().to_string();
    if first.is_empty() {
        None
    } else {
        Some(first)
    }
}

fn commit_for_path_cached(
    path: &str,
    cache: &mut BTreeMap<String, Option<String>>,
) -> Option<String> {
    if let Some(value) = cache.get(path) {
        return value.clone();
    }
    let value = commit_for_path(path);
    cache.insert(path.to_string(), value.clone());
    value
}

fn commit_touches(lower: Option<&str>, upper: Option<&str>, token: &str) -> Option<bool> {
    let mut cmd = Command::new("git");
    cmd.arg("log")
        .arg("--oneline")
        .arg(format!("--grep={}", token));
    match (lower, upper) {
        (Some(a), Some(b)) => {
            cmd.arg(format!("{}..{}", a, b));
        }
        (Some(a), None) => {
            cmd.arg(format!("{}..HEAD", a));
        }
        (None, Some(b)) => {
            cmd.arg(b);
        }
        (None, None) => {}
    }
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(!output.stdout.is_empty())
}

fn dropped_line_filter(args: &[String]) -> Option<&str> {
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--dropped" {
            return match it.next() {
                Some(next) if !next.starts_with("--") => Some(next.as_str()),
                _ => None,
            };
        }
    }
    None
}

fn persist_threshold(args: &[String]) -> usize {
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--persist" {
            return match it.next() {
                Some(value) => match value.parse::<usize>() {
                    Ok(n) if n >= 1 => n,
                    _ => 1,
                },
                None => 1,
            };
        }
    }
    1
}

fn run_dropped(args: &[String]) {
    let filter = dropped_line_filter(args);
    let threshold = persist_threshold(args);
    let handovers = collect_handovers();
    let mut commit_cache: BTreeMap<String, Option<String>> = BTreeMap::new();
    let mut pairs = 0usize;
    let mut candidates = 0usize;
    let mut dropped = 0usize;
    let mut resolved = 0usize;
    for (line, list) in &handovers {
        if let Some(wanted) = filter {
            if wanted != line.as_str() {
                continue;
            }
        }
        let padded: Vec<String> = list
            .iter()
            .map(|h| format!(" {} ", normalize_text(&h.text)))
            .collect();
        for index in 0..list.len().saturating_sub(1) {
            let n = &list[index];
            let next = &list[index + 1];
            pairs += 1;
            let next_padded = &padded[index + 1];
            let mut seen_keys: Vec<String> = Vec::new();
            for point in extract_open_points(&n.text) {
                let tokens = point_key_tokens(&point.text);
                let key = match match_prefix(&tokens) {
                    Some(k) => k,
                    None => continue,
                };
                if seen_keys.iter().any(|k| k == &key) {
                    continue;
                }
                seen_keys.push(key.clone());
                candidates += 1;
                let needle = format!(" {} ", key);
                if next_padded.contains(&needle) {
                    continue;
                }
                let mut persist = 1usize;
                let mut back = index;
                while back > 0 {
                    if padded[back - 1].contains(&needle) {
                        persist += 1;
                        back -= 1;
                    } else {
                        break;
                    }
                }
                if persist < threshold {
                    continue;
                }
                let git_status = match distinctive_token(&tokens) {
                    Some(token) => {
                        let lower = commit_for_path_cached(&n.path, &mut commit_cache);
                        let upper = commit_for_path_cached(&next.path, &mut commit_cache);
                        match commit_touches(lower.as_deref(), upper.as_deref(), &token) {
                            Some(true) => {
                                resolved += 1;
                                "resolved"
                            }
                            _ => "none",
                        }
                    }
                    None => "none",
                };
                dropped += 1;
                println!(
                    "DROPPED\t{}\t{}:{}\t{}\t{}\tpersist {}\tgit: {}",
                    line,
                    n.path,
                    point.lineno,
                    next.path,
                    snippet(&point.text, 160),
                    persist,
                    git_status
                );
            }
        }
    }
    let scope = match filter {
        Some(f) => format!(" {}", f),
        None => String::new(),
    };
    println!(
        "register_lookup --dropped{}: {} pairs, {} candidates, {} dropped, {} commit-resolved, persist >= {}",
        scope, pairs, candidates, dropped, resolved, threshold
    );
}

fn print_usage() -> ! {
    eprintln!(
        "usage: register_lookup <term>...   (queries the live register: is X already measured/registered?)\n       register_lookup --open            (digest: open points across all live prose documents + the disposition register, owner-tagged)\n       register_lookup --dropped [<line>] [--persist <n>]   (open points of handover N absent from handover N+1 with no resolving commit in between; --persist <n> reports only points present in at least n consecutive handovers, default 1)\n       register_lookup --history [--legacy <path>] [<term>]   (open points in archived + deleted documents; <term> adds git log -S over rewritten files)"
    );
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--open") {
        run_open();
        return;
    }
    if args.iter().any(|a| a == "--dropped") {
        run_dropped(&args);
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

    #[test]
    fn disposition_owner_maps_every_known_state() {
        assert_eq!(
            disposition_owner("blocked parser-def drs-fits"),
            Some("bau")
        );
        assert_eq!(disposition_owner("blocked parser-def odf"), Some("bau"));
        assert_eq!(disposition_owner("blocked account"), Some("entscheid"));
        assert_eq!(disposition_owner("blocked key"), Some("entscheid"));
        assert_eq!(disposition_owner("blocked ip-blocked"), Some("ernte"));
        assert_eq!(disposition_owner("pending"), Some("ernte"));
        assert_eq!(disposition_owner("descoped"), None);
    }

    #[test]
    fn status_owner_maps_on_first_token() {
        assert_eq!(disposition_owner("parser-def cdf"), Some("bau"));
        assert_eq!(disposition_owner("blocked account"), Some("entscheid"));
        assert_eq!(disposition_owner("blocked parser-def odf"), Some("bau"));
        assert_eq!(disposition_owner("blocked mystery"), None);
    }

    #[test]
    fn scan_dispositions_tags_owner_and_splits_descoped() {
        let text = "note preamble\n\npending\nurl https://a\nnote offen\n\ndescoped\nurl https://b\nnote nie gebaut\n\nblocked account\nurl https://c\nnote Konto\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_dispositions_text(text, "b.\u{3c6}", &mut open_out, &mut released_out);
        assert_eq!(n, 3);
        assert_eq!(open_out.len(), 2);
        assert_eq!(released_out.len(), 1);
        assert!(open_out[0].starts_with("DISPOSITION\tb.\u{3c6}:3\t[ernte] pending"));
        assert!(open_out[1].contains("[entscheid] blocked account"));
        assert!(released_out[0].starts_with("RELEASED\tb.\u{3c6}:7\t"));
        assert!(released_out[0].contains("descoped"));
    }

    #[test]
    fn scan_dispositions_flags_an_unmapped_state() {
        let text = "blocked mystery\nurl https://x\nnote y\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        scan_dispositions_text(text, "b.\u{3c6}", &mut open_out, &mut released_out);
        assert_eq!(open_out.len(), 1);
        assert!(open_out[0].starts_with("DISPOSITION_UNMAPPED\tb.\u{3c6}:1\t"));
    }

    #[test]
    fn scan_dispositions_marks_a_pending_request_as_waiting() {
        let text = "pending\nurl https://x\nnote Anfrage 2026-09-16, Antwort offen.\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        scan_dispositions_text(text, "b.\u{3c6}", &mut open_out, &mut released_out);
        assert_eq!(open_out.len(), 1);
        assert!(open_out[0].starts_with("DISPOSITION\tb.\u{3c6}:1\t[wartend] pending"));
    }

    #[test]
    fn state_class_maps_every_register_state() {
        let table: &[(&str, Option<StateClass>)] = &[
            ("ausstehend", Some(StateClass::Open("ernte"))),
            ("verifiziert", Some(StateClass::Open("ernte"))),
            ("kompiliert", Some(StateClass::Open("ernte"))),
            ("parser-gap", Some(StateClass::Open("bau"))),
            ("void", Some(StateClass::Released)),
            ("disponiert", Some(StateClass::Released)),
            ("pending", Some(StateClass::Open("ernte"))),
            ("erledigt", Some(StateClass::Released)),
            ("ausgelagert", Some(StateClass::Released)),
            ("descoped", Some(StateClass::Released)),
            ("fehlt", Some(StateClass::Open("ernte"))),
            ("offen", Some(StateClass::Open("ernte"))),
            ("absent", Some(StateClass::Open("ernte"))),
            ("declined", Some(StateClass::Released)),
            ("refused", Some(StateClass::Released)),
            ("asset fehlt", Some(StateClass::Open("bau"))),
            ("asset present", Some(StateClass::Ignored)),
            ("review", Some(StateClass::Open("ernte"))),
            ("index", Some(StateClass::Ignored)),
            ("artefakt", Some(StateClass::Ignored)),
            ("register", Some(StateClass::Ignored)),
            ("infra", Some(StateClass::Ignored)),
            ("probe", Some(StateClass::Ignored)),
            ("frame", Some(StateClass::Ignored)),
            ("listen", Some(StateClass::Ignored)),
            ("research", Some(StateClass::Ignored)),
        ];
        for (state, expected) in table {
            assert_eq!(
                state_class(state),
                *expected,
                "state {:?} unmapped or mis-mapped",
                state
            );
        }
    }

    #[test]
    fn state_class_flags_an_unknown_state() {
        assert_eq!(state_class("mystery"), None);
        assert_eq!(state_class("open no-consumer"), None);
    }

    #[test]
    fn scan_state_blocks_text_tags_owner_and_splits_released_and_unmapped() {
        let text = "ausstehend\nkandidat https://a\nnote messung offen\n\nparser-gap\nkandidat https://b\nnote parser fehlt\n\nvoid\nkandidat https://c\nnote tot\n\nmystery\nkandidat https://d\nnote unbekannt\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_state_blocks_text(text, "l.\u{3c6}", &mut open_out, &mut released_out);
        assert_eq!(n, 2);
        assert_eq!(open_out.len(), 3);
        assert_eq!(released_out.len(), 1);
        assert!(open_out[0].starts_with("DISPOSITION\tl.\u{3c6}:1\t[ernte] ausstehend"));
        assert!(open_out[1].starts_with("DISPOSITION\tl.\u{3c6}:5\t[bau] parser-gap"));
        assert!(open_out[2].starts_with("DISPOSITION_UNMAPPED\tl.\u{3c6}:13\tmystery"));
        assert!(released_out[0].starts_with("RELEASED\tl.\u{3c6}:9\tvoid"));
    }

    #[test]
    fn scan_state_blocks_harvest_tags_asset_fehlt_and_ignores_present() {
        let text = "asset fehlt\nformat lro_trk\nnote LRO RSS raw tracking\n\nasset present\nformat gaia_sso\nnote gaia asset\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_state_blocks_text(text, "h.\u{3c6}", &mut open_out, &mut released_out);
        assert_eq!(n, 1);
        assert_eq!(open_out.len(), 1);
        assert_eq!(released_out.len(), 0);
        assert!(open_out[0].starts_with("DISPOSITION\th.\u{3c6}:1\t[bau] asset fehlt"));
    }

    #[test]
    fn scan_index_text_tags_owner_and_splits_released_and_unmapped() {
        let text = "# header\nausstehend 10 pipeline/queue/x.φ\nerledigt 3 archive/y\nausgelagert 2 archive-root/z\nindex 1 pipeline/catalog/\nvermerkt 9 weird\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_index_text(text, "i.\u{3c6}", &mut open_out, &mut released_out);
        assert_eq!(n, 1);
        assert_eq!(open_out.len(), 2);
        assert_eq!(released_out.len(), 2);
        assert!(open_out[0].starts_with("DISPOSITION\ti.\u{3c6}:2\t[ernte] ausstehend"));
        assert!(open_out[1].starts_with("DISPOSITION_UNMAPPED\ti.\u{3c6}:6\tvermerkt"));
        assert!(released_out[0].starts_with("RELEASED\ti.\u{3c6}:3\terledigt"));
        assert!(released_out[1].starts_with("RELEASED\ti.\u{3c6}:4\tausgelagert"));
    }

    #[test]
    fn scan_note_markers_text_tags_owner_and_splits_released() {
        let text = "url https://a\nnote field descoped (gemessen): nie gebaut\n\nurl https://b\nnote sha256 pending (CI re-dispatch)\n\nurl https://c\nnote kein marker\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_note_markers_text(
            text,
            "s.\u{3c6}",
            &["pending", "fehlt", "offen"],
            &["descoped"],
            &mut open_out,
            &mut released_out,
        );
        assert_eq!(n, 1);
        assert_eq!(open_out.len(), 1);
        assert_eq!(released_out.len(), 1);
        assert!(open_out[0].starts_with("DISPOSITION\ts.\u{3c6}:5\t[ernte] pending"));
        assert!(released_out[0].starts_with("RELEASED\ts.\u{3c6}:2\tdescoped"));
    }

    #[test]
    fn scan_note_markers_prefers_released_over_open() {
        let text = "note pending; descoped\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_note_markers_text(
            text,
            "s.\u{3c6}",
            &["pending"],
            &["descoped"],
            &mut open_out,
            &mut released_out,
        );
        assert_eq!(n, 0);
        assert_eq!(open_out.len(), 0);
        assert_eq!(released_out.len(), 1);
        assert!(released_out[0].contains("descoped"));
    }

    #[test]
    fn scan_probe_text_tags_review_and_pending() {
        let text = "# uncertain field x \u{2014} force/unit undetermined, review\n# pending crosswind unit \u{2014} register carries m/s\nfield x x 1 advective hPa 60 0.0 0.0\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_probe_text(text, "p.\u{3c6}", &mut open_out, &mut released_out);
        assert_eq!(n, 2);
        assert_eq!(open_out.len(), 2);
        assert_eq!(released_out.len(), 0);
        assert!(open_out[0].starts_with("DISPOSITION\tp.\u{3c6}:1\t[ernte] review"));
        assert!(open_out[1].starts_with("DISPOSITION\tp.\u{3c6}:2\t[ernte] pending"));
    }

    #[test]
    fn scan_catalog_candidates_counts_leading_tokens_only() {
        let dir = env::temp_dir().join(format!("register_lookup_catalog_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let f1 = dir.join("cat_a.\u{3c6}");
        fs::write(
            &f1,
            "# header\ncandidate https://a\ncandidate https://b\ndecline https://c\n",
        )
        .unwrap();
        let f2 = dir.join("cat_b.\u{3c6}");
        fs::write(
            &f2,
            "doi:10.1 | PhD candidates study\ncandidatex https://no\n",
        )
        .unwrap();
        let mut out = Vec::new();
        let n = scan_catalog_candidates(&dir, &mut out);
        assert_eq!(n, 2);
        assert_eq!(out.len(), 1);
        assert!(out[0].starts_with("CANDIDATES\t"));
        assert!(out[0].contains("cat_a.\u{3c6}\t2 \u{2192} ernte"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_handover_name_reads_folge_and_single_session() {
        assert_eq!(
            parse_handover_name("handover-2026-09-20-bau-folge113.md"),
            Some(("bau".to_string(), "2026-09-20".to_string(), Some(113)))
        );
        assert_eq!(
            parse_handover_name("handover-2026-09-20-pii-llm-budget.md"),
            Some(("pii-llm-budget".to_string(), "2026-09-20".to_string(), None))
        );
        assert_eq!(
            parse_handover_name("handover-2026-09-15-bau-folge33-p8-gate.md"),
            Some(("bau".to_string(), "2026-09-15".to_string(), Some(33)))
        );
        assert_eq!(parse_handover_name("post.md"), None);
        assert_eq!(parse_handover_name("not-a-handover.md"), None);
    }

    #[test]
    fn extract_open_points_reads_container_rows_and_thread_headings() {
        let text = "# H\n\n## Offen\n\n| Punkt | Status | Bindung | Schritt |\n|---|---|---|---|\n| alpha beta gamma delta epsilon zeta | `wartend` | `termin` | run |\n\n## Stehender Pass (gemessen)\n\n- **HEAD** `abc` == `origin/main`.\n\n## Zwei rote Gates \u{2014} unvollst\u{e4}ndig \u{b7} `pending`\n\n- ein weiterer offener Punkt\n";
        let points = extract_open_points(text);
        let texts: Vec<&str> = points.iter().map(|p| p.text.as_str()).collect();
        assert!(texts.iter().any(|t| t.starts_with("alpha beta gamma")));
        assert!(texts
            .iter()
            .any(|t| t.starts_with("Zwei rote Gates")));
        assert!(texts.iter().any(|t| t.starts_with("ein weiterer")));
        assert!(!texts.iter().any(|t| t.starts_with("HEAD")));
    }

    #[test]
    fn status_word_matches_tag_but_not_embedded_substring() {
        assert!(tag_in_words(&normalize_words("das ist `wartend`")));
        assert!(tag_in_words(&normalize_words("bleibt offen")));
        assert!(!tag_in_words(&normalize_words("deterministisch")));
    }

    #[test]
    fn match_prefix_uses_six_words_and_drops_the_status_tag() {
        let tokens = point_key_tokens("3 ausstehend Queue-Korpora (30-astro, earth-stac-sentinel)");
        assert_eq!(tokens[0], "3");
        assert!(!tokens.iter().any(|t| t == "ausstehend"));
        assert_eq!(
            match_prefix(&tokens),
            Some("3 queue-korpora 30-astro earth-stac-sentinel".to_string())
        );
    }

    #[test]
    fn distinctive_token_prefers_the_long_word_and_rejects_stopwords() {
        let tokens = vec!["am".to_string(), "head".to_string(), "8218f46a".to_string()];
        assert_eq!(distinctive_token(&tokens), Some("8218f46a".to_string()));
        let words = vec!["der".to_string(), "die".to_string()];
        assert_eq!(distinctive_token(&words), None);
    }
}
