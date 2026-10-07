use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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

const ZUSTAND_PATH: &str = "state/zustand/external-state.md";
const EREIGNISSE_PATH: &str = "state/zustand/ereignisse.\u{3c6}";

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
const DISPOSITION_REGISTER_PATHS: &[&str] = &[
    "phi/declined_sources.\u{3c6}",
    "phi/dead_sources.\u{3c6}",
    "phi/blocked_sources.\u{3c6}",
    "phi/sources.\u{3c6}",
    "phi/pipeline/ledger.\u{3c6}",
    "phi/pipeline/index.\u{3c6}",
    "phi/harvest.\u{3c6}",
    "phi/witnesses.\u{3c6}",
    "phi/footprints.\u{3c6}",
    "phi/nrs_stations.\u{3c6}",
];

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

fn strip_inline_code(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut in_code = false;
    for ch in line.chars() {
        if ch == '`' {
            in_code = !in_code;
        } else if !in_code {
            out.push(ch);
        }
    }
    out
}

fn marker_in_status_context(lower: &str, marker: &str) -> bool {
    let status_token = marker == "blocked";
    lower.match_indices(marker).any(|(idx, _)| {
        let before = lower[..idx].trim_end();
        let after = &lower[idx + marker.len()..];
        let starts_clean = before.is_empty()
            || before.chars().last().map_or(false, |c| {
                if status_token {
                    matches!(c, ':' | '|' | '-' | '(' | '[')
                } else {
                    !c.is_alphanumeric()
                }
            });
        let ends_clean = after.is_empty()
            || after.chars().next().map_or(false, |c| {
                if status_token {
                    matches!(c, ' ' | ':' | '|' | ',' | ')' | ']')
                } else {
                    !c.is_alphanumeric()
                }
            });
        starts_clean && ends_clean
    })
}

fn open_marker_matches(line: &str) -> bool {
    let lower = strip_inline_code(line).to_lowercase();
    OPEN_MARKERS
        .iter()
        .any(|m| marker_in_status_context(&lower, m))
}

fn released_marker_matches(line: &str) -> bool {
    let lower = strip_inline_code(line).to_lowercase();
    RELEASED_MARKERS.iter().any(|m| lower.contains(m))
}

enum ZustandStatus {
    Due,
    NotDue,
    Pending,
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
        Some("parser-def" | "parser-gap") => Some("mountain"),
        Some("asset") => match tokens.next() {
            Some("fehlt") => Some("mountain"),
            _ => None,
        },
        Some("terms") => match tokens.next() {
            Some("unbestimmt" | "ohne-lizenz") => Some("mountain"),
            _ => None,
        },
        Some("ausstehend" | "verifiziert" | "kompiliert" | "pending") => Some("mycelium"),
        Some("blocked") => match tokens.next() {
            Some("account" | "key") => Some("future"),
            Some("ip-blocked") => Some("mycelium"),
            Some("parser-def") => Some("mountain"),
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
        | "review" => Some(StateClass::Open("mycelium")),
        "parser-gap" | "asset fehlt" | "terms unbestimmt" | "terms ohne-lizenz" => {
            Some(StateClass::Open("mountain"))
        }
        "descoped" | "void" | "disponiert" | "erledigt" | "ausgelagert" | "declined"
        | "refused" | "released" => Some(StateClass::Released),
        "asset present" | "index" | "artefakt" | "register" | "infra" | "probe" | "frame"
        | "listen" | "research" | "generiert" => Some(StateClass::Ignored),
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
        if state == "descoped" || state == "released" {
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

        if !state.starts_with("terms ") {
            if let Some((tline, tvalue)) =
                lines.iter().find(|(_, l)| l.trim().starts_with("terms "))
            {
                let tstate = tvalue.trim();
                match disposition_owner(tstate) {
                    Some(owner) => open_out.push(format!(
                        "DISPOSITION\t{}:{}\t[{}] {} | {}",
                        path, tline, owner, tstate, step
                    )),
                    None => open_out.push(format!(
                        "DISPOSITION_UNMAPPED\t{}:{}\t{} | {}",
                        path, tline, tstate, step
                    )),
                }
                n += 1;
            }
        }
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

struct OrphanCandidate {
    register: String,
    lineno: usize,
    owner: String,
    url: String,
    name: String,
    class_key: String,
    no_gap: bool,
}

fn collect_orphan_candidates_in(text: &str, register: &str) -> Vec<OrphanCandidate> {
    let mut out = Vec::new();
    for (start_line, lines) in parse_blocks(text) {
        let state = lines[0].1.trim();
        if state.is_empty() || state.starts_with("note") || state == "descoped" {
            continue;
        }
        let Some(owner) = disposition_owner(state) else {
            continue;
        };
        let url = lines
            .iter()
            .find(|(_, l)| l.trim_start().starts_with("url "))
            .map(|(_, l)| l.trim_start().trim_start_matches("url ").trim().to_string())
            .unwrap_or(String::new());
        let note = lines
            .iter()
            .find(|(_, l)| l.trim_start().starts_with("note "))
            .map(|(_, l)| {
                l.trim_start()
                    .trim_start_matches("note ")
                    .trim()
                    .to_string()
            })
            .unwrap_or(String::new());
        let gap = lines
            .iter()
            .find(|(_, l)| l.trim_start().starts_with("gap "))
            .map(|(_, l)| l.trim_start().trim_start_matches("gap ").trim().to_string())
            .unwrap_or(String::new());
        let name = orphan_key_name(&note, &url);
        if url.is_empty() && name.is_empty() {
            continue;
        }
        let class_key = if gap.is_empty() {
            String::new()
        } else {
            format!("{}::gap:{}", register, gap)
        };
        let no_gap = owner == "mountain"
            && (state.starts_with("parser-def")
                || state.starts_with("parser-gap")
                || state.starts_with("blocked parser-def"))
            && gap.is_empty();
        let terms_candidate = if state.starts_with("terms ") {
            None
        } else {
            lines
                .iter()
                .find(|(_, l)| l.trim_start().starts_with("terms "))
                .and_then(|&(tline, tvalue)| {
                    disposition_owner(tvalue.trim()).map(|towner| OrphanCandidate {
                        register: register.to_string(),
                        lineno: tline,
                        owner: towner.to_string(),
                        url: url.clone(),
                        name: name.clone(),
                        class_key: String::new(),
                        no_gap: false,
                    })
                })
        };
        out.push(OrphanCandidate {
            register: register.to_string(),
            lineno: start_line,
            owner: owner.to_string(),
            url,
            name,
            class_key,
            no_gap,
        });
        if let Some(candidate) = terms_candidate {
            out.push(candidate);
        }
    }
    out
}

fn orphan_key_name(note: &str, url: &str) -> String {
    let leading = leading_region(note).trim();
    if leading.chars().count() >= 4 {
        return leading.to_string();
    }
    if let Some(token) = distinctive_token(&point_key_tokens(note)) {
        return token;
    }
    url.split("//")
        .nth(1)
        .and_then(|rest| rest.split('/').next())
        .unwrap_or("")
        .to_string()
}

fn orphan_held(carrier: &str, url: &str, name: &str) -> bool {
    let url_l = url.to_lowercase();
    let name_l = name.to_lowercase();
    (url_l.len() >= 8 && carrier.contains(&url_l))
        || (name_l.chars().count() >= 4 && carrier.contains(&name_l))
}

fn owner_handover_texts(root: &Path) -> BTreeMap<String, String> {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    let mut dirs: Vec<String> = HANDOVER_DIRS.iter().map(|s| s.to_string()).collect();
    dirs.push(PRIVATE_HANDOVER_DIR.to_string());
    for dir in dirs {
        let entries = match fs::read_dir(root.join(&dir)) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let name = file_name_string(&entry.path());
            if !name.ends_with(".md") || name.starts_with('_') {
                continue;
            }
            let Some((line, _, _)) = parse_handover_name(&name) else {
                continue;
            };
            let Ok(text) = fs::read_to_string(entry.path()) else {
                continue;
            };
            let slot = map.entry(canonical_line(&line).to_string()).or_default();
            slot.push_str(&text.to_lowercase());
            slot.push('\n');
        }
    }
    map
}

fn git_head_text(path: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["show", &format!("HEAD:{}", path)])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

fn orphan_report_with(
    root: &Path,
    head: &dyn Fn(&str) -> Option<String>,
) -> (Vec<String>, BTreeMap<String, (usize, usize)>) {
    let carriers = owner_handover_texts(root);
    let private_carrier_present = root.join(PRIVATE_HANDOVER_DIR).exists();
    let mut candidates: Vec<OrphanCandidate> = Vec::new();
    let mut registers_head: BTreeMap<String, String> = BTreeMap::new();
    for register in DISPOSITION_REGISTER_PATHS {
        let text = match fs::read_to_string(root.join(register)) {
            Ok(t) => t,
            Err(_) => continue,
        };
        if let Some(head_text) = head(register) {
            registers_head.insert(register.to_string(), head_text.to_lowercase());
        }
        candidates.extend(collect_orphan_candidates_in(&text, register));
    }

    let mut class_live: BTreeMap<String, (String, usize)> = BTreeMap::new();
    for c in &candidates {
        if !c.class_key.is_empty() {
            let slot = class_live
                .entry(c.class_key.clone())
                .or_insert((c.owner.clone(), 0));
            slot.1 += 1;
        }
    }
    let mut class_held: BTreeSet<String> = BTreeSet::new();
    let mut drift: Vec<String> = Vec::new();
    for (key, (owner, live)) in &class_live {
        let carrier = carriers.get(owner).cloned().unwrap_or(String::new());
        let key_l = key.to_lowercase();
        if let Some(pos) = carrier.find(&key_l) {
            class_held.insert(key.clone());
            let after = &carrier[pos + key_l.len()..];
            let declared = after
                .trim_start()
                .strip_prefix('\u{d7}')
                .map(|rest| {
                    rest.trim_start()
                        .chars()
                        .take_while(|ch| ch.is_ascii_digit())
                        .collect::<String>()
                })
                .and_then(|digits| digits.parse::<usize>().ok());
            if declared != Some(*live) {
                drift.push(format!(
                    "CARRIER_DRIFT\t{}\tcarrier={}\tlive={}",
                    key,
                    declared
                        .map(|n| n.to_string())
                        .unwrap_or("none".to_string()),
                    live
                ));
            }
        }
    }

    let mut lines: Vec<String> = Vec::new();
    let mut summary: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for c in &candidates {
        if !private_carrier_present && c.owner == "future" {
            let key = if c.url.is_empty() {
                c.name.clone()
            } else {
                c.url.clone()
            };
            lines.push(format!(
                "UNVERIFIABLE_PRIVATE\t{}:{}\t[{}]\t{}",
                c.register, c.lineno, c.owner, key
            ));
            continue;
        }
        let carrier = carriers.get(&c.owner).cloned().unwrap_or(String::new());
        let held = (!c.class_key.is_empty() && class_held.contains(&c.class_key))
            || orphan_held(&carrier, &c.url, &c.name);
        if held {
            continue;
        }
        let committed = registers_head
            .get(&c.register)
            .map(|h| {
                (!c.class_key.is_empty() && h.contains(&c.class_key.to_lowercase()))
                    || orphan_held(h, &c.url, &c.name)
            })
            .unwrap_or(false);
        let class = if committed {
            "ORPHAN_COMMITTED"
        } else {
            "ORPHAN_UNCOMMITTED"
        };
        let key = if c.url.is_empty() {
            c.name.clone()
        } else {
            c.url.clone()
        };
        let suffix = if c.no_gap {
            "\tNO_GAP (gap directive absent)"
        } else {
            ""
        };
        lines.push(format!(
            "{}\t{}:{}\t[{}]\t{}{}",
            class, c.register, c.lineno, c.owner, key, suffix
        ));
        let slot = summary.entry(c.owner.clone()).or_insert((0, 0));
        if committed {
            slot.0 += 1;
        } else {
            slot.1 += 1;
        }
    }
    for (key, (owner, live)) in &class_live {
        if class_held.contains(key) {
            continue;
        }
        lines.push(format!("ORPHAN_CLASS\t{}\t[{}]\t{}", key, owner, live));
    }
    lines.extend(drift);
    for (owner, (committed, uncommitted)) in &summary {
        lines.push(format!(
            "ORPHAN_SUMMARY\t{}\t{} committed {} uncommitted",
            owner, committed, uncommitted
        ));
    }
    (lines, summary)
}

fn orphan_report(root: &Path) -> (Vec<String>, BTreeMap<String, (usize, usize)>) {
    orphan_report_with(root, &git_head_text)
}

fn orphan_total(summary: &BTreeMap<String, (usize, usize)>) -> usize {
    summary.values().map(|(c, u)| c + u).sum()
}

fn owner_arg(args: &[String]) -> Option<String> {
    mode_line_filter(args, "--owner")
        .map(canonical_line)
        .map(str::to_lowercase)
}

fn orphan_line_for_owner(line: &str, owner: &str) -> bool {
    let lowered = line.to_lowercase();
    lowered.contains(&format!("\t[{}]\t", owner))
        || lowered.starts_with(&format!("orphan_summary\t{}\t", owner))
}

fn run_orphans(args: &[String]) {
    let owner = owner_arg(args);
    let fail = args.iter().any(|a| a == "--fail");
    let (lines, summary) = orphan_report(Path::new("."));
    let filtered: BTreeMap<String, (usize, usize)> = match &owner {
        Some(o) => summary
            .iter()
            .filter(|(key, _)| key.as_str() == o.as_str())
            .map(|(key, value)| (key.clone(), *value))
            .collect(),
        None => summary.clone(),
    };
    let mut unverifiable: usize = 0;
    for line in &lines {
        if let Some(o) = &owner {
            if !orphan_line_for_owner(line, o) {
                continue;
            }
        }
        if line.starts_with("UNVERIFIABLE_PRIVATE\t") {
            unverifiable += 1;
        }
        println!("{}", line);
    }
    let owners: Vec<String> = filtered
        .iter()
        .map(|(o, (c, u))| format!("{} {} committed {} uncommitted", o, c, u))
        .collect();
    let total = orphan_total(&filtered);
    let unverifiable_note = if unverifiable > 0 {
        format!(" ({} unverifiable — private carrier absent)", unverifiable)
    } else {
        String::new()
    };
    println!(
        "register_lookup --orphans: {} orphan entries [{}]{}",
        total,
        owners.join(", "),
        unverifiable_note
    );
    if fail && total > 0 {
        std::process::exit(2);
    }
}

fn live_handover_carrier_text(root: &Path) -> String {
    let mut out = String::new();
    for dir in ["docs/handover", PRIVATE_HANDOVER_DIR] {
        let entries = match fs::read_dir(root.join(dir)) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let name = file_name_string(&path);
            if !is_doc_name(&name) || parse_handover_name(&name).is_none() {
                continue;
            }
            if let Ok(text) = fs::read_to_string(&path) {
                out.push_str(&text.to_lowercase());
                out.push('\n');
            }
        }
    }
    out
}

fn doc_carried(carrier: &str, path: &str) -> bool {
    let base = match Path::new(path).file_name() {
        Some(f) => f.to_string_lossy().to_lowercase(),
        None => return false,
    };
    let stem = base.strip_suffix(".md").unwrap_or(&base);
    (base.len() >= 8 && carrier.contains(&base))
        || (stem.chars().count() >= 8 && carrier.contains(stem))
}

fn orphan_docs(root: &Path) -> Vec<String> {
    let carrier = live_handover_carrier_text(root);
    let mut out: Vec<String> = Vec::new();
    for (dir, _) in REGISTER_DIRS {
        if *dir == "docs/handover" {
            continue;
        }
        let entries = match fs::read_dir(root.join(dir)) {
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
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            let (_, _, status) = parse_header(&text);
            if is_closed_status(&status) {
                continue;
            }
            let path_str = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            let mut opens: Vec<String> = Vec::new();
            let mut released: Vec<String> = Vec::new();
            scan_markers(&text, &path_str, "OPEN", &mut opens, &mut released);
            if opens.is_empty() {
                continue;
            }
            if !doc_carried(&carrier, &path_str) {
                out.push(format!("ORPHAN_DOC\t{}\t{}", path_str, opens.len()));
            }
        }
    }
    out.sort();
    out
}

fn run_orphan_docs(root: &Path) {
    let lines = orphan_docs(root);
    for line in &lines {
        println!("{}", line);
    }
    println!(
        "register_lookup --orphan-docs: {} orphan documents carry open markers and no live handover carrier",
        lines.len()
    );
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

fn ereignisse_owner(klasse: &str) -> Option<&'static str> {
    match klasse {
        "account" | "send" => Some("future"),
        _ => None,
    }
}

fn scan_ereignisse(path: &Path, open_out: &mut Vec<String>) -> usize {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    let mut n = 0;
    for (idx, line) in text.lines().enumerate() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = t.split('|').map(str::trim).collect();
        let klasse = match fields.get(3) {
            Some(k) => *k,
            None => continue,
        };
        let Some(owner) = ereignisse_owner(klasse) else {
            continue;
        };
        let gegenstand = fields.get(4).copied().unwrap_or("");
        open_out.push(format!(
            "EREIGNIS\t{}:{}\t[{}]\t{} | {}",
            path.display(),
            idx + 1,
            owner,
            klasse,
            snippet(gegenstand, 120)
        ));
        n += 1;
    }
    n
}

fn collect_disposed_urls(paths: &[&str]) -> BTreeSet<String> {
    let mut urls = BTreeSet::new();
    for path in paths {
        let text = match fs::read_to_string(path) {
            Ok(t) => t,
            Err(_) => continue,
        };
        for line in text.lines() {
            let t = line.trim_start();
            let value = t.strip_prefix("url ").or_else(|| t.strip_prefix("reg "));
            if let Some(url) = value {
                let url = url.trim();
                if !url.is_empty() {
                    urls.insert(url.to_string());
                }
            }
        }
    }
    urls
}

fn scan_catalog_candidates(
    dir: &Path,
    disposed: &BTreeSet<String>,
    out: &mut Vec<String>,
) -> (usize, usize) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return (0, 0),
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    let mut total = 0;
    let mut skipped = 0;
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
            let t = line.trim_start();
            if let Some(url) = t.strip_prefix("candidate ") {
                if disposed.contains(url.trim()) {
                    skipped += 1;
                    continue;
                }
                n += 1;
            }
        }
        if n == 0 {
            continue;
        }
        out.push(format!(
            "CANDIDATES\t{}\t{} \u{2192} mycelium",
            path.to_string_lossy(),
            n
        ));
        total += n;
    }
    (total, skipped)
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

fn scan_future(dir: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return out,
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
        let (class, date, status) = parse_header(&text);
        let open_count = text.lines().filter(|l| open_marker_matches(l)).count();
        out.push(format!(
            "FUNDING\t[future]\t{}\t{}\t{}\t{}\t{}",
            if class.is_empty() {
                "-"
            } else {
                class.as_str()
            },
            if date.is_empty() { "-" } else { date.as_str() },
            if status.is_empty() {
                "no-header"
            } else {
                status.as_str()
            },
            open_count,
            path.to_string_lossy()
        ));
    }
    out
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

    let future_lines = scan_future("state/future");

    let head = current_head_short();
    let now_min = now_minutes();
    let mut zustand_out: Vec<String> = Vec::new();
    let zustand = scan_zustand(
        Path::new(ZUSTAND_PATH),
        head.as_deref(),
        now_min,
        &mut zustand_out,
    );
    let mut dispo_out: Vec<String> = Vec::new();
    let dispo = scan_dispositions(
        Path::new("phi/blocked_sources.\u{3c6}"),
        &mut dispo_out,
        &mut released,
    );

    for line in &docs {
        println!("{}", line);
    }
    for line in &future_lines {
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
    let (orphan_lines, orphan_summary) = orphan_report(Path::new("."));
    for line in &orphan_lines {
        println!("{}", line);
    }
    for line in &zustand_out {
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
        &["pending"],
        &["declined"],
        &mut witnesses_open,
        &mut witnesses_released,
    );
    print_section("WITNESSES", witnesses, &witnesses_open, &witnesses_released);

    let mut footprints_open: Vec<String> = Vec::new();
    let mut footprints_released: Vec<String> = Vec::new();
    let footprints = scan_note_markers(
        Path::new(FOOTPRINTS_PATH),
        &["pending"],
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
        &["pending"],
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

    let mut ereignisse_open: Vec<String> = Vec::new();
    let ereignisse = scan_ereignisse(Path::new(EREIGNISSE_PATH), &mut ereignisse_open);
    print_section("EREIGNISSE", ereignisse, &ereignisse_open, &[]);

    let mut candidates_out: Vec<String> = Vec::new();
    let disposed_urls = collect_disposed_urls(DISPOSITION_REGISTER_PATHS);
    let (candidates, candidates_disposed) =
        scan_catalog_candidates(Path::new(CATALOG_DIR), &disposed_urls, &mut candidates_out);
    for line in &candidates_out {
        println!("{}", line);
    }

    let summary: Vec<String> = class_counts
        .iter()
        .map(|(c, n)| format!("{} {}", c, n))
        .collect();
    println!(
        "register_lookup --open: {} docs, {} future, {} open lines, {} released lines, {} duplicates, {} unverifiable, {} zustand due, {} orphan, {} disposition [{}], pipeline: ledger {} open, index {} open, sources {} open, witnesses {} open, footprints {} open, harvest {} open, nrs {} open, probes {} open, ereignisse {} open, {} candidates ({} disposed)",
        docs.len(),
        future_lines.len(),
        opens.len(),
        released.len(),
        dups.len(),
        unverifiable.len(),
        zustand,
        orphan_total(&orphan_summary),
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
        ereignisse,
        candidates,
        candidates_disposed,
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

const LINE_ALIASES: &[(&str, &str)] = &[
    ("bau", "mountain"),
    ("ernte", "mycelium"),
    ("forschung", "sensory"),
];

const PRIVATE_HANDOVER_DIR: &str = "state/future/handover";

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
    "der",
    "die",
    "das",
    "den",
    "dem",
    "des",
    "ein",
    "eine",
    "einen",
    "einem",
    "eines",
    "und",
    "oder",
    "aber",
    "ist",
    "sind",
    "wird",
    "werden",
    "wurde",
    "nicht",
    "kein",
    "keine",
    "fuer",
    "mit",
    "von",
    "auf",
    "aus",
    "als",
    "auch",
    "nur",
    "noch",
    "the",
    "and",
    "for",
    "with",
    "from",
    "that",
    "this",
    "into",
    "over",
    "after",
    "punkt",
    "status",
    "schritt",
    "offen",
    "wartend",
    "blockiert",
    "pending",
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
    from_heading: bool,
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

fn canonical_line(line: &str) -> &str {
    LINE_ALIASES
        .iter()
        .find(|(from, _)| *from == line)
        .map(|(_, to)| *to)
        .unwrap_or(line)
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
    let words = normalize_words(heading);
    if let Some(first) = words.first() {
        if CONTAINER_HEADS.contains(&first.as_str()) {
            return true;
        }
    }
    words
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

const CONTAINER_HEADS: &[&str] = &[
    "offen",
    "burn",
    "abschluss",
    "lock",
    "postfach",
    "zustand",
    "benchmark",
    "stehender",
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

fn inline_field_end(value: &str) -> usize {
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'|' {
            return i;
        }
        if bytes[i] == b'*'
            && (i == 0 || bytes[i - 1].is_ascii_whitespace() || bytes[i - 1] == b'|')
        {
            let mut j = i;
            while j < bytes.len() && bytes[j] == b'*' {
                j += 1;
            }
            let label_start = j;
            while j < bytes.len() && (bytes[j].is_ascii_alphabetic() || bytes[j] == b'/') {
                j += 1;
            }
            if j > label_start && j < bytes.len() && bytes[j] == b':' {
                return i;
            }
        }
        i += 1;
    }
    value.len()
}

fn field_value_after_label(text: &str, label: &str) -> Option<String> {
    let needle = label.to_ascii_lowercase();
    let lower = text.to_ascii_lowercase();
    let mut from = 0;
    while let Some(rel) = lower[from..].find(&needle) {
        let abs = from + rel;
        if abs > 0
            && text[..abs]
                .chars()
                .last()
                .map(|c| c.is_alphanumeric())
                .unwrap_or(false)
        {
            from = abs + needle.len();
            continue;
        }
        let after = &text[abs + needle.len()..];
        let mut colon: Option<usize> = None;
        for (i, c) in after.char_indices() {
            if c == '*' || c.is_whitespace() {
                continue;
            }
            if c == ':' {
                colon = Some(i);
            }
            break;
        }
        if let Some(i) = colon {
            let value = after[i + 1..].trim();
            let value = value[..inline_field_end(value)]
                .trim()
                .trim_matches(|c: char| c == '*' || c.is_whitespace());
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
        from = abs + needle.len();
    }
    None
}

fn strip_measurement_stamps(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut i = 0usize;
    while i < text.len() {
        let ch = match text[i..].chars().next() {
            Some(c) => c,
            None => break,
        };
        if ch == '(' {
            if let Some(rel) = text[i..].find(')') {
                let close = i + rel;
                let inner = text[i + 1..close].to_ascii_lowercase();
                if inner.contains("gemessen") || inner.contains("measured") {
                    i = close + 1;
                    continue;
                }
            }
        }
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn trigger_fallback(text: &str) -> String {
    if let Some(value) = field_value_after_label(text, "trigger") {
        return value;
    }
    let head = match text.find('|') {
        Some(at) => &text[..at],
        None => text,
    };
    strip_measurement_stamps(leading_region(head))
}

fn extract_open_points(text: &str) -> Vec<OpenPoint> {
    let mut points: Vec<OpenPoint> = Vec::new();
    let mut section_open = false;
    let mut current_heading: Option<(usize, String)> = None;
    let mut in_point_block = false;
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
                    from_heading: false,
                });
            }
            section_open = open;
            current_heading = None;
            in_point_block = false;
            continue;
        }
        if let Some(rest) = trimmed
            .strip_prefix("### ")
            .or_else(|| trimmed.strip_prefix("#### "))
        {
            current_heading = Some((lineno, rest.trim().to_string()));
            in_point_block = false;
            continue;
        }
        if let Some(cells) = split_table_row(raw) {
            if current_heading.is_some() || in_point_block {
                continue;
            }
            if cells.len() < 2 || is_table_separator(&cells) {
                continue;
            }
            let first = cells[0].trim();
            if first.is_empty() || first.eq_ignore_ascii_case("punkt") {
                continue;
            }
            let row_open = section_open || cells.iter().any(|c| tag_in_words(&normalize_words(c)));
            if row_open {
                points.push(OpenPoint {
                    lineno,
                    text: first.to_string(),
                    from_heading: false,
                });
            }
            continue;
        }
        if let Some(body) = strip_bullet_marker(trimmed) {
            if let Some((hlineno, heading)) = current_heading.take() {
                let open = section_open
                    || tag_in_words(&normalize_words(&heading))
                    || heading.to_lowercase().starts_with("punkt");
                if open && !is_container_heading(&heading) {
                    points.push(OpenPoint {
                        lineno: hlineno,
                        text: heading,
                        from_heading: true,
                    });
                }
                in_point_block = true;
                continue;
            }
            if in_point_block {
                continue;
            }
            if section_open || tag_in_words(&normalize_words(leading_region(body))) {
                points.push(OpenPoint {
                    lineno,
                    text: body.to_string(),
                    from_heading: false,
                });
            }
        }
    }
    points.retain(|p| p.from_heading || !is_container_text(&p.text));
    points
}

fn point_key_tokens(text: &str) -> Vec<String> {
    let tokens: Vec<String> = normalize_words(text)
        .into_iter()
        .filter(|w| is_meaningful_word(w) && !is_status_word(w))
        .collect();
    let start = tokens
        .iter()
        .position(|w| !is_enumeration_token(w))
        .unwrap_or(tokens.len());
    tokens[start..].to_vec()
}

fn is_enumeration_token(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_ascii_digit())
}

fn match_prefix(tokens: &[String]) -> Option<String> {
    if tokens.is_empty() {
        return None;
    }
    let take = tokens.len().min(6);
    Some(tokens[..take].join(" "))
}

fn explicit_point_id(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i + 2 < bytes.len() {
        let is_id =
            (bytes[i] | 0x20) == b'i' && (bytes[i + 1] | 0x20) == b'd' && bytes[i + 2] == b':';
        if is_id {
            let boundary = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
            if boundary {
                let slug: String = text[i + 3..]
                    .trim_start_matches([' ', '\t', '*', '`'])
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
                    .collect();
                if !slug.is_empty() {
                    return Some(slug.to_lowercase());
                }
            }
        }
        i += 1;
    }
    None
}

fn canonical_point_key(text: &str) -> Option<String> {
    if let Some(slug) = explicit_point_id(text) {
        return Some(slug);
    }
    let mut tokens = point_key_tokens(text);
    if tokens.is_empty() {
        return None;
    }
    tokens.sort();
    tokens.dedup();
    Some(tokens.join(" "))
}

fn canonical_key_in_carrier(carrier: &str, key: &str) -> bool {
    key.split(' ')
        .filter(|t| !t.is_empty())
        .all(|needle| carrier.split_whitespace().any(|word| word == needle))
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

fn private_successor_exists_in(dir: &str, line: &str, folge: u32) -> bool {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return false,
    };
    let wanted = match folge.checked_add(1) {
        Some(w) => w,
        None => return false,
    };
    for entry in entries.flatten() {
        let name = file_name_string(&entry.path());
        let parsed = match parse_handover_name(&name) {
            Some(p) => p,
            None => continue,
        };
        let (pl, _, pf) = parsed;
        if pf == Some(wanted) && canonical_line(&pl) == line {
            return true;
        }
    }
    false
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
            if !name.ends_with(".md") || name.starts_with('_') {
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
            let line = canonical_line(&line).to_string();
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

fn load_commit_message_corpus() -> Option<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--exclude=refs/safety/*")
        .arg("--all")
        .arg("--format=%B%x00")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

static MESSAGE_CORPUS_LOWER: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();

fn commit_message_corpus_lower() -> Option<&'static str> {
    MESSAGE_CORPUS_LOWER
        .get_or_init(|| load_commit_message_corpus().map(|c| c.to_lowercase()))
        .as_deref()
}

fn commit_token_resolved(token: &str) -> Option<bool> {
    commit_message_corpus_lower().map(|corpus| corpus.contains(token))
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b >= 0x80
}

fn offer_word(word: &str, pending: &mut BTreeSet<String>, found: &mut BTreeSet<String>) {
    if pending.remove(word) {
        found.insert(word.to_string());
        return;
    }
    if word.contains('-') {
        for part in word.split('-') {
            if !part.is_empty() && pending.remove(part) {
                found.insert(part.to_string());
            }
        }
    }
}

fn scan_words_case(content: &str, pending: &mut BTreeSet<String>, found: &mut BTreeSet<String>) {
    let bytes = content.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        while i < bytes.len() && !is_word_byte(bytes[i]) {
            i += 1;
        }
        let start = i;
        while i < bytes.len() && is_word_byte(bytes[i]) {
            i += 1;
        }
        if start < i {
            offer_word(&content[start..i], pending, found);
        }
    }
}

fn load_commit_content_index(tokens: &BTreeSet<String>) -> BTreeSet<String> {
    if tokens.is_empty() {
        return BTreeSet::new();
    }
    let mut pending: BTreeSet<String> = tokens.clone();
    let mut found: BTreeSet<String> = BTreeSet::new();
    let mut child = match Command::new("git")
        .args([
            "log",
            "--exclude=refs/safety/*",
            "--all",
            "--format=%x00",
            "-p",
            "--unified=0",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return found,
    };
    let stdout = match child.stdout.take() {
        Some(s) => s,
        None => {
            let _ = child.wait();
            return found;
        }
    };
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        let Ok(line) = line else { break };
        let bytes = line.as_bytes();
        if bytes.is_empty() {
            continue;
        }
        match bytes[0] {
            b'+' | b'-' if !(bytes.len() > 1 && (bytes[1] == b'+' || bytes[1] == b'-')) => {
                scan_words_case(&line[1..], &mut pending, &mut found);
            }
            _ => continue,
        }
        if pending.is_empty() {
            break;
        }
    }
    let _ = child.wait();
    found
}

fn register_carrier_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    for dir in ["phi", "phi/pipeline"] {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if file_name_string(&path).ends_with(".φ") {
                files.push(path);
            }
        }
    }
    let wartend = PathBuf::from("state/zustand/wartend.φ");
    if wartend.is_file() {
        files.push(wartend);
    }
    files.sort();
    files
}

fn load_register_content_index(tokens: &BTreeSet<String>) -> BTreeSet<String> {
    if tokens.is_empty() {
        return BTreeSet::new();
    }
    let mut found: BTreeSet<String> = BTreeSet::new();
    for path in register_carrier_files() {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for word in normalize_words(&text) {
            if tokens.contains(&word) {
                found.insert(word);
            }
        }
    }
    found
}

fn resolution_status(own_range: Option<bool>, all_lines: Option<bool>) -> &'static str {
    if own_range == Some(true) || all_lines == Some(true) {
        "resolved"
    } else {
        "none"
    }
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

fn count_flag(args: &[String]) -> bool {
    args.iter().any(|a| a == "--count")
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

fn dropped_live_carriers() -> (BTreeMap<String, String>, String) {
    let mut by_line: BTreeMap<String, String> = BTreeMap::new();
    for dir in ["docs/handover", PRIVATE_HANDOVER_DIR] {
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
            if !is_doc_name(&name) {
                continue;
            }
            let Some((line, _, _)) = parse_handover_name(&name) else {
                continue;
            };
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            let slot = by_line
                .entry(canonical_line(&line).to_string())
                .or_default();
            slot.push(' ');
            slot.push_str(&normalize_text(&text));
            slot.push(' ');
        }
    }
    let mut docs = String::new();
    for (dir, _) in REGISTER_DIRS {
        if *dir == "docs/handover" {
            continue;
        }
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
            if !is_doc_name(&name) {
                continue;
            }
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            let (_, _, status) = parse_header(&text);
            if is_closed_status(&status) {
                continue;
            }
            let mut opens: Vec<String> = Vec::new();
            let mut released: Vec<String> = Vec::new();
            scan_markers(&text, "", "OPEN", &mut opens, &mut released);
            if opens.is_empty() {
                continue;
            }
            docs.push(' ');
            docs.push_str(&normalize_text(&text));
            docs.push(' ');
        }
    }
    (by_line, docs)
}

struct DroppedPoint {
    line: String,
    n_path: String,
    lineno: usize,
    next_path: String,
    text: String,
    persist: usize,
    token: Option<String>,
}

fn run_dropped(args: &[String]) {
    let filter = dropped_line_filter(args).map(canonical_line);
    let threshold = persist_threshold(args);
    let count_only = count_flag(args);
    let keys_only = args.iter().any(|a| a == "--dropped-keys");
    let handovers = collect_handovers();
    let (live_by_line, live_docs) = dropped_live_carriers();
    let mut pairs = 0usize;
    let mut candidates = 0usize;
    let mut dropped = 0usize;
    let mut points: Vec<DroppedPoint> = Vec::new();
    let mut token_set: BTreeSet<String> = BTreeSet::new();
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
                let canonical = canonical_point_key(&point.text);
                let carries = |carrier: &str| {
                    carrier.contains(&needle)
                        || canonical
                            .as_deref()
                            .map_or(false, |c| canonical_key_in_carrier(carrier, c))
                };
                if carries(next_padded) {
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
                let carried_forward = padded[index + 2..].iter().any(|later| carries(later));
                let carried_foreign = live_by_line
                    .iter()
                    .any(|(other, text)| other != line && carries(text));
                if carried_forward || carried_foreign || carries(&live_docs) {
                    continue;
                }
                dropped += 1;
                let token = distinctive_token(&tokens);
                if let Some(t) = &token {
                    token_set.insert(t.clone());
                }
                points.push(DroppedPoint {
                    line: line.clone(),
                    n_path: n.path.clone(),
                    lineno: point.lineno,
                    next_path: next.path.clone(),
                    text: point.text.clone(),
                    persist,
                    token,
                });
            }
        }
        if !count_only && !keys_only {
            if let Some(last) = list.last() {
                if let Some(folge) = last.folge {
                    if private_successor_exists_in(PRIVATE_HANDOVER_DIR, line, folge) {
                        println!(
                            "BOUNDARY\t{}\t{}\t{}\tboundary unmeasured: successor private \u{2014} state/future/handover/",
                            line, last.path, folge
                        );
                    }
                }
            }
        }
    }
    let content_index = load_commit_content_index(&token_set);
    let register_index = load_register_content_index(&token_set);
    let mut resolved = 0usize;
    let mut pending = 0usize;
    for p in &points {
        let git_status = match &p.token {
            Some(token) => {
                let by_message = commit_token_resolved(token);
                let touched = by_message == Some(true)
                    || content_index.contains(token)
                    || register_index.contains(token);
                let status = resolution_status(None, Some(touched));
                if status == "resolved" {
                    resolved += 1;
                }
                status
            }
            None => {
                pending += 1;
                "pending"
            }
        };
        if keys_only {
            if p.token.is_some() && git_status != "resolved" {
                if let Some(key) = canonical_point_key(&p.text) {
                    println!("{}", key);
                }
            }
            continue;
        }
        if !count_only {
            println!(
                "DROPPED\t{}\t{}:{}\t{}\t{}\tpersist {}\tgit: {}",
                p.line,
                p.n_path,
                p.lineno,
                p.next_path,
                snippet(&p.text, 160),
                p.persist,
                git_status
            );
        }
    }
    if keys_only {
        return;
    }
    if count_only {
        println!("{}", dropped - resolved - pending);
        return;
    }
    let scope = match filter {
        Some(f) => format!(" {}", f),
        None => String::new(),
    };
    println!(
        "register_lookup --dropped{}: {} pairs, {} candidates, {} dropped, {} commit-resolved, {} pending/unmeasured, persist >= {}",
        scope, pairs, candidates, dropped, resolved, pending, threshold
    );
}

fn canonical_open_point_keys(text: &str) -> BTreeSet<String> {
    let mut keys: BTreeSet<String> = BTreeSet::new();
    for point in extract_open_points(text) {
        if let Some(key) = canonical_point_key(&point.text) {
            keys.insert(key);
        }
    }
    keys
}

fn roster_diff(baseline: &BTreeSet<String>, keys: &BTreeSet<String>) -> (Vec<String>, Vec<String>) {
    let lost = baseline.difference(keys).cloned().collect();
    let new = keys.difference(baseline).cloned().collect();
    (lost, new)
}

fn run_dropped_roster(args: &[String]) {
    let count_only = count_flag(args);
    let public_only = args.iter().any(|a| a == "--public-only");
    let mut keys: BTreeSet<String> = BTreeSet::new();
    let dirs: &[&str] = if public_only {
        &["docs/handover"]
    } else {
        &["docs/handover", PRIVATE_HANDOVER_DIR]
    };
    for dir in dirs {
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
            if !is_doc_name(&name) || parse_handover_name(&name).is_none() {
                continue;
            }
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            keys.extend(canonical_open_point_keys(&text));
        }
    }

    let baseline_path = mode_line_filter(args, "--baseline");
    if let Some(path) = baseline_path {
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(_) => {
                eprintln!("register_lookup --dropped-roster: baseline {path} absent");
                return;
            }
        };
        let baseline: BTreeSet<String> = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(str::to_string)
            .collect();
        let (lost, new) = roster_diff(&baseline, &keys);
        if count_only {
            println!("{}", lost.len());
            return;
        }
        for key in &lost {
            println!("LOST {key}");
        }
        for key in &new {
            println!("NEW {key}");
        }
        return;
    }

    if count_only {
        println!("{}", keys.len());
        return;
    }
    for key in &keys {
        println!("{}", key);
    }
}

fn mode_line_filter<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == flag {
            return match it.next() {
                Some(next) if !next.starts_with("--") => Some(next.as_str()),
                _ => None,
            };
        }
    }
    None
}

fn stale_threshold(args: &[String]) -> usize {
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--persist" {
            return match it.next() {
                Some(value) => value.parse::<usize>().ok().filter(|n| *n >= 1).unwrap_or(3),
                None => 3,
            };
        }
    }
    3
}

fn following_block(text: &str, point_idx0: usize) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    let mut i = point_idx0 + 1;
    while i < lines.len() {
        let t = lines[i].trim();
        if t.is_empty()
            || t.starts_with("## ")
            || t.starts_with("### ")
            || t.starts_with("#### ")
            || t.starts_with("<!--")
        {
            break;
        }
        out.push(lines[i].to_string());
        i += 1;
    }
    out
}

fn block_field(block: &[String], label: &str) -> Option<String> {
    for line in block {
        if let Some(value) = field_value_after_label(line, label) {
            return Some(value);
        }
    }
    None
}

fn point_lage(text: &str, lineno: usize) -> Option<String> {
    let block = following_block(text, lineno.saturating_sub(1));
    block_field(&block, "lage")
}

fn stale_points(
    handovers: &BTreeMap<String, Vec<Handover>>,
    filter: Option<&str>,
    n: usize,
) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
    if n == 0 {
        return out;
    }
    for (line, list) in handovers {
        if let Some(wanted) = filter {
            if wanted != line.as_str() {
                continue;
            }
        }
        if list.len() < n {
            continue;
        }
        for start in 0..=(list.len() - n) {
            let window = &list[start..start + n];
            let mut per_key: BTreeMap<String, Vec<Option<String>>> = BTreeMap::new();
            for h in window {
                let mut local: BTreeMap<String, Option<String>> = BTreeMap::new();
                for point in extract_open_points(&h.text) {
                    let key = normalize_text(&point.text);
                    if key.is_empty() {
                        continue;
                    }
                    let lage = point_lage(&h.text, point.lineno).map(|l| normalize_text(&l));
                    let slot = local.entry(key).or_insert(None);
                    if slot.is_none() {
                        *slot = lage;
                    }
                }
                for (key, lage) in local {
                    per_key.entry(key).or_default().push(lage);
                }
            }
            for (key, vals) in per_key {
                if vals.len() != n {
                    continue;
                }
                let first = match &vals[0] {
                    Some(v) if !v.is_empty() => v.clone(),
                    _ => continue,
                };
                if vals.iter().all(|v| v.as_ref() == Some(&first))
                    && seen.insert((line.clone(), key.clone()))
                {
                    out.push(format!("STALE\t{}\t{}\t{}", line, n, key));
                }
            }
        }
    }
    out
}

fn today_days() -> Option<i64> {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();
    Some((secs / 86400) as i64)
}

fn current_head_full() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let sha = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if sha.is_empty() { None } else { Some(sha) }
}

fn standalone_iso_date(bytes: &[u8], i: usize) -> bool {
    let before = if i == 0 {
        true
    } else {
        !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'-')
    };
    let end = i + 10;
    let after = if end >= bytes.len() {
        true
    } else {
        let c = bytes[end];
        if c.is_ascii_alphanumeric() || c == b'-' {
            (c == b'T' || c == b't') && end + 1 < bytes.len() && bytes[end + 1].is_ascii_digit()
        } else {
            true
        }
    };
    before && after
}

fn find_iso_date(text: &str) -> Option<i64> {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i + 10 <= bytes.len() {
        if bytes[i].is_ascii_digit() {
            let seg = &bytes[i..i + 10];
            if seg[4] == b'-'
                && seg[7] == b'-'
                && seg[..4].iter().all(u8::is_ascii_digit)
                && seg[5..7].iter().all(u8::is_ascii_digit)
                && seg[8..].iter().all(u8::is_ascii_digit)
                && standalone_iso_date(bytes, i)
            {
                let date = std::str::from_utf8(seg).ok()?;
                let y: i64 = date[..4].parse().ok()?;
                let m: i64 = date[5..7].parse().ok()?;
                let d: i64 = date[8..].parse().ok()?;
                if (1..=12).contains(&m) && (1..=31).contains(&d) {
                    return Some(days_from_civil(y, m, d));
                }
            }
        }
        i += 1;
    }
    None
}

fn head_reference(text: &str) -> Option<String> {
    for token in text.split(|c: char| !c.is_ascii_hexdigit()) {
        if token.len() == 40 && token.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Some(token.to_lowercase());
        }
    }
    let lower = text.to_ascii_lowercase();
    if normalize_words(&lower).iter().any(|w| w == "head") {
        for token in text.split(|c: char| !c.is_ascii_alphanumeric()) {
            if token.len() >= 7 && token.len() < 40 && token.bytes().all(|b| b.is_ascii_hexdigit())
            {
                return Some(token.to_lowercase());
            }
        }
    }
    None
}

fn source_token_present(text: &str) -> bool {
    let mut words: Vec<String> = Vec::new();
    for word in normalize_words(text) {
        for part in word.split('-') {
            if !part.is_empty() {
                words.push(part.to_string());
            }
        }
    }
    let has = |t: &str| words.iter().any(|w| w == t);
    let has_id = words
        .iter()
        .any(|w| w.len() >= 5 && w.bytes().all(|b| b.is_ascii_digit()));

    if has("mail_ledger") || has("mail") {
        return has_id
            || has("antwort")
            || words
                .windows(2)
                .any(|w| w[0] == "eingang" && w[1] == "eingetroffen");
    }
    if has("run") || has("lauf") {
        return has_id;
    }
    false
}

fn fired_points(
    handovers: &BTreeMap<String, Vec<Handover>>,
    filter: Option<&str>,
    today: Option<i64>,
    head: Option<&str>,
) -> (Vec<String>, usize) {
    let mut out: Vec<String> = Vec::new();
    let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
    let mut fired = 0usize;
    for (line, list) in handovers {
        if let Some(wanted) = filter {
            if wanted != line.as_str() {
                continue;
            }
        }
        let h = match list.last() {
            Some(h) => h,
            None => continue,
        };
        for point in extract_open_points(&h.text) {
            let block = if point.from_heading {
                following_block(&h.text, point.lineno.saturating_sub(1))
            } else {
                Vec::new()
            };
            let status_text = match block_field(&block, "status") {
                Some(s) => format!("{} {}", point.text, s),
                None => point.text.to_string(),
            };
            let status_words = normalize_words(&status_text);
            let has = |t: &str| status_words.iter().any(|w| w == t);
            if !(has("operator-gebunden") || has("wartend") || has("wartestell") || has("termin")) {
                continue;
            }
            let key = normalize_text(&point.text);
            if key.is_empty() || !seen.insert((line.clone(), key.clone())) {
                continue;
            }
            let mut combined = String::new();
            if let Some(t) = block_field(&block, "trigger") {
                combined.push_str(&t);
            }
            if let Some(b) = block_field(&block, "bindung") {
                if !combined.is_empty() {
                    combined.push(' ');
                }
                combined.push_str(&b);
            }
            if combined.trim().is_empty() {
                combined = trigger_fallback(&point.text);
            }
            let reason = snippet(&combined, 120);
            if combined.contains("Wort:") || combined.contains("wort:") {
                out.push(format!("FIRED_MANUAL\t{}\t{}\t{}", line, key, reason));
                fired += 1;
                continue;
            }
            let lower = combined.to_lowercase();
            if let Some(days) = find_iso_date(&combined) {
                match today {
                    Some(now) if days <= now && days >= now - 366 => {
                        out.push(format!("FIRED\t{}\t{}\t{}", line, key, reason));
                        fired += 1;
                    }
                    Some(_) => {}
                    None => {
                        out.push(format!("FIRED_UNGEMESSEN\t{}\t{}\t{}", line, key, reason));
                    }
                }
                continue;
            }
            let reference = head_reference(&combined);
            let head_word = normalize_words(&lower).iter().any(|w| w == "head");
            if head_word || reference.is_some() {
                match reference {
                    Some(reference) => match head {
                        Some(current)
                            if !(current.starts_with(&reference)
                                || reference.starts_with(current)) =>
                        {
                            out.push(format!("FIRED\t{}\t{}\t{}", line, key, reason));
                            fired += 1;
                        }
                        Some(_) => {}
                        None => {
                            out.push(format!("FIRED_UNGEMESSEN\t{}\t{}\t{}", line, key, reason));
                        }
                    },
                    None => {
                        out.push(format!("FIRED_UNGEMESSEN\t{}\t{}\t{}", line, key, reason));
                    }
                }
                continue;
            }
            if source_token_present(&lower) {
                out.push(format!("FIRED_UNGEMESSEN\t{}\t{}\t{}", line, key, reason));
            }
        }
    }
    (out, fired)
}

fn live_handover_paths(root: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    for dir in ["docs/handover", PRIVATE_HANDOVER_DIR] {
        let entries = match fs::read_dir(root.join(dir)) {
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
            if !is_doc_name(&name) || parse_handover_name(&name).is_none() {
                continue;
            }
            out.push(path);
        }
    }
    out
}

fn collect_live_handovers_in(root: &Path) -> BTreeMap<String, Vec<Handover>> {
    let mut by_line: BTreeMap<String, Vec<Handover>> = BTreeMap::new();
    let mut seen: Vec<String> = Vec::new();
    for path in live_handover_paths(root) {
        let name = file_name_string(&path);
        if seen.iter().any(|s| s == &name) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let (_, _, status) = parse_header(&text);
        if is_closed_status(&status) {
            continue;
        }
        let Some((line, date, folge)) = parse_handover_name(&name) else {
            continue;
        };
        seen.push(name);
        by_line
            .entry(canonical_line(&line).to_string())
            .or_default()
            .push(Handover {
                date,
                folge,
                path: path.to_string_lossy().to_string(),
                text,
            });
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

fn backtick_paths(text: &str) -> Vec<String> {
    let parts: Vec<&str> = text.split('`').collect();
    let mut out = Vec::new();
    let mut i = 1;
    while i < parts.len() {
        let token = parts[i].trim();
        if !token.is_empty() {
            out.push(token.to_string());
        }
        i += 2;
    }
    out
}

fn doc_open_task_markers(text: &str) -> usize {
    let body = match text.find("-->") {
        Some(i) => &text[i + 3..],
        None => text,
    };
    let mut count = 0usize;
    for line in body.lines() {
        let t = line.trim();
        let low = t.to_lowercase();
        if t.starts_with("##") && low.contains("offen") && !low.contains("gekl") {
            count += 1;
            continue;
        }
        if low.contains("n\u{e4}chster schritt")
            || low.contains("n\u{e4}chste schritte")
            || low.contains("naechster schritt")
            || low.contains("naechste schritte")
            || low.contains("todo")
            || low.contains("- [ ]")
            || low.contains("- **braucht:**")
        {
            count += 1;
        }
    }
    count
}

fn descoped_widerlegt(root: &Path) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for path in live_handover_paths(root) {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let lines: Vec<&str> = text.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            let lower = line.to_ascii_lowercase();
            if !(lower.contains("status") && lower.contains("descoped")) {
                continue;
            }
            let block = following_block(&text, idx);
            let source = match block_field(&block, "quelle") {
                Some(q) => q,
                None => continue,
            };
            for rel in backtick_paths(&source) {
                let doc_text = match fs::read_to_string(root.join(&rel)) {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                let markers = doc_open_task_markers(&doc_text);
                if markers > 0 {
                    out.push(format!("descoped-widerlegt\t{}\t{}", rel, markers));
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

fn run_stale(args: &[String]) {
    let filter = mode_line_filter(args, "--stale").map(canonical_line);
    let n = stale_threshold(args);
    let handovers = collect_live_handovers_in(Path::new("."));
    let lines = stale_points(&handovers, filter, n);
    for line in &lines {
        println!("{}", line);
    }
    println!("register_lookup --stale: {} stale points", lines.len());
}

fn run_fired(args: &[String]) {
    let filter = mode_line_filter(args, "--fired").map(canonical_line);
    let today = today_days();
    let head = current_head_full();
    let handovers = collect_live_handovers_in(Path::new("."));
    let (lines, fired) = fired_points(&handovers, filter, today, head.as_deref());
    for line in &lines {
        println!("{}", line);
    }
    println!("register_lookup --fired: {} fired points", fired);
}

fn run_descoped_check(_args: &[String]) {
    let lines = descoped_widerlegt(Path::new("."));
    for line in &lines {
        println!("{}", line);
    }
    println!(
        "register_lookup --descoped-check: {} widerlegt",
        lines.len()
    );
}

fn print_usage() -> ! {
    eprintln!(
        "usage: register_lookup <term>...   (queries the live register: is X already measured/registered?)\n       register_lookup --open            (digest: open points across all live prose documents + the disposition register, owner-tagged)\n       register_lookup --dropped [<line>] [--persist <n>] [--count]   (open points of handover N absent from handover N+1 with no resolving commit in between; --persist <n> reports only points present in at least n consecutive handovers, default 1; --count prints the dropped integer net of commit-resolved points; --dropped-keys prints the canonical point-keys of the unresolved drops, one per line)\n       register_lookup --dropped-roster [--count] [--public-only] [--baseline <datei>]   (the canonical point-key roster of the live handovers — the set form of --dropped; --count prints the distinct-key integer; --public-only skips the private handover dir so the set matches CI; --baseline compares against a stored roster and prints LOST/NEW, with --count printing the LOST integer)\n       register_lookup --orphans [--owner <line>] [--fail]   (owner-tagged open register entries no live handover of that owner names: ORPHAN_COMMITTED (in HEAD) or ORPHAN_UNCOMMITTED (working tree only); --owner restricts to one line; --fail exits 2 when the orphan count is > 0)\n       register_lookup --orphan-docs      (live prose documents under docs/{{surveys,specs,auftrag,blatt,concepts,paper}} carrying open markers that no live handover names: ORPHAN_DOC <path> <markers>)\n       register_lookup --addressed <line> [--fail]   (the `## An <line>` blocks addressed to the own line across the live handovers, sender-named; never a full foreign-handover read; --fail exits 2 when an addressed block stands unbeglichen)\n       register_lookup --stale [<line>] [--persist <n>]   (a point key present across n consecutive live handovers with an identical Lage line: STALE <line> <n> <key>; default n = 3)\n       register_lookup --fired [<line>]   (open points whose trigger is measured as arrived: an ISO date within the last year and <= today, a HEAD/sha reference != HEAD, a Wort: trigger (FIRED_MANUAL), or a ci/mail/run/lauf source token (FIRED_UNGEMESSEN))\n       register_lookup --descoped-check   (descoped handover points whose Quelle document still carries an explicit open-work marker — a `## ...offen...` heading not marked `gekl...`, `naechster Schritt`, `TODO`, `- [ ]`, or `- **Braucht:**`: descoped-widerlegt <path> <markers>)\n       register_lookup --history [--legacy <path>] [<term>]   (open points in archived + deleted documents; <term> adds git log -S over rewritten files)"
    );
    eprintln!(
        "       register_lookup --compilers   (one line per phi/sources.φ block carrying a compiler: <binary> | <format> | <source-url> | at <anchor>, then a domain/count summary over the format prefix)\n       register_lookup --compilers --no-directive   (tree compilers tools/*/src/bin/*_compiler.rs without a compiler directive: classified by measured channel workflow|register:<file>|variant|pending|unregistered)"
    );
    std::process::exit(2);
}

const ADDRESSED_LINES: &[&str] = &["mountain", "river", "mycelium", "sensory", "future"];

fn addressed_target(heading: &str) -> Option<&'static str> {
    let rest = heading.trim_start().strip_prefix("## ")?;
    let rest = rest.trim_start().strip_prefix("An ")?;
    let token: String = rest
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    ADDRESSED_LINES
        .iter()
        .find(|l| token.eq_ignore_ascii_case(l))
        .copied()
}

fn addressed_blocks(text: &str, target: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut iter = text.lines().peekable();
    while let Some(line) = iter.next() {
        if addressed_target(line) != Some(target) {
            continue;
        }
        let mut block = String::new();
        block.push_str(line);
        block.push('\n');
        while let Some(next) = iter.peek() {
            if next.trim_start().starts_with("## ") {
                break;
            }
            block.push_str(next);
            block.push('\n');
            iter.next();
        }
        out.push(block.trim_end().to_string());
    }
    out
}

fn addressed_report(root: &Path, target: &str) -> Vec<String> {
    let target = canonical_line(target);
    let handovers = collect_live_handovers_in(root);
    let mut out = Vec::new();
    let mut count = 0usize;
    for (sender, list) in &handovers {
        for h in list {
            for block in addressed_blocks(&h.text, target) {
                count += 1;
                let folge = match h.folge {
                    Some(n) => format!("folge{}", n),
                    None => "session".to_string(),
                };
                out.push(format!(
                    "ADDRESSED\t{}\t<- {}-{} ({})",
                    target, sender, folge, h.path
                ));
                for line in block.lines() {
                    out.push(line.to_string());
                }
                out.push(String::new());
            }
        }
    }
    if count == 0 {
        out.push(format!(
            "register_lookup --addressed {}: 0 addressed blocks",
            target
        ));
    } else {
        out.push(format!(
            "register_lookup --addressed {}: {} addressed block(s)",
            target, count
        ));
    }
    out
}

fn run_addressed(args: &[String]) {
    let target = match mode_line_filter(args, "--addressed") {
        Some(t) => t.to_string(),
        None => {
            eprintln!(
                "register_lookup --addressed <line>: missing line (mountain|river|mycelium|sensory|future)"
            );
            std::process::exit(2);
        }
    };
    let fail = args.iter().any(|a| a == "--fail");
    let lines = addressed_report(Path::new("."), &target);
    let count = lines.iter().filter(|l| l.starts_with("ADDRESSED")).count();
    for line in &lines {
        println!("{}", line);
    }
    if fail && count > 0 {
        std::process::exit(2);
    }
}

const COMPILER_SUFFIX: &str = "_compiler.rs";
const WORKFLOWS_DIR: &str = ".github/workflows";
const BINDINGS_DIR: &str = "phi/bindings";
const LIVE_HANDOVER_DIR: &str = "docs/handover";

struct CompilerEntry {
    path: String,
    binary: String,
    format: Option<String>,
    url: Option<String>,
    anchor: Option<String>,
    tags: Vec<String>,
}

struct TreeCompiler {
    path: String,
    stem: String,
}

fn sources_value<'a>(lines: &[(usize, &'a str)], name: &str) -> Option<&'a str> {
    for (_, line) in lines {
        let mut fields = line.split_whitespace();
        if fields.next() == Some(name) {
            return fields.next();
        }
    }
    None
}

fn sources_values<'a>(lines: &[(usize, &'a str)], name: &str) -> Vec<&'a str> {
    let mut out = Vec::new();
    for (_, line) in lines {
        let mut fields = line.split_whitespace();
        if fields.next() == Some(name) {
            out.extend(fields);
        }
    }
    out
}

fn compiler_stem(line: &str) -> String {
    match Path::new(line).file_stem() {
        Some(stem) => stem.to_string_lossy().to_string(),
        None => line.to_string(),
    }
}

fn sources_compiler_entries(text: &str) -> Vec<CompilerEntry> {
    let mut out = Vec::new();
    for (_, lines) in parse_blocks(text) {
        let compiler = match sources_value(&lines, "compiler") {
            Some(value) => value,
            None => continue,
        };
        out.push(CompilerEntry {
            path: compiler.to_string(),
            binary: compiler_stem(compiler),
            format: sources_value(&lines, "format").map(|s| s.to_string()),
            url: sources_value(&lines, "url").map(|s| s.to_string()),
            anchor: sources_value(&lines, "at").map(|s| s.to_string()),
            tags: sources_values(&lines, "tags")
                .iter()
                .map(|s| s.to_string())
                .collect(),
        });
    }
    out
}

fn format_domain(format: &str) -> &str {
    match format.split_once('_') {
        Some((prefix, _)) if !prefix.is_empty() => prefix,
        _ => format,
    }
}

fn tree_compilers(root: &Path) -> Vec<TreeCompiler> {
    let mut out = Vec::new();
    let crates = match fs::read_dir(root.join("tools")) {
        Ok(entries) => entries,
        Err(_) => return out,
    };
    for crate_entry in crates.flatten() {
        let bin_dir = crate_entry.path().join("src").join("bin");
        let files = match fs::read_dir(&bin_dir) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for file in files.flatten() {
            let name = file.file_name().to_string_lossy().to_string();
            if !name.ends_with(COMPILER_SUFFIX) {
                continue;
            }
            let path = file.path();
            let rel = path.strip_prefix(root).unwrap_or(&path);
            out.push(TreeCompiler {
                path: rel.to_string_lossy().replace('\\', "/"),
                stem: name.trim_end_matches(".rs").to_string(),
            });
        }
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

fn compiler_difference<'a>(
    tree: &'a [TreeCompiler],
    wired: &BTreeSet<String>,
) -> Vec<&'a TreeCompiler> {
    tree.iter().filter(|tc| !wired.contains(&tc.path)).collect()
}

fn common_prefix_len(a: &str, b: &str) -> usize {
    a.bytes()
        .zip(b.bytes())
        .take_while(|(x, y)| x.eq_ignore_ascii_case(y))
        .count()
}

fn wired_variants(stem: &str, wired_stems: &[String]) -> bool {
    wired_stems
        .iter()
        .any(|w| w != stem && common_prefix_len(stem, w) >= 4)
}

fn workflows_text(root: &Path) -> String {
    let mut out = String::new();
    let entries = match fs::read_dir(root.join(WORKFLOWS_DIR)) {
        Ok(entries) => entries,
        Err(_) => return out,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if !(name.ends_with(".yml") || name.ends_with(".yaml")) {
            continue;
        }
        if let Ok(text) = fs::read_to_string(&path) {
            out.push_str(&text);
            out.push('\n');
        }
    }
    out
}

fn register_anchor(root: &Path, stem: &str) -> Option<String> {
    let mut files = vec![WITNESSES_PATH.to_string(), FOOTPRINTS_PATH.to_string()];
    if let Ok(entries) = fs::read_dir(root.join(BINDINGS_DIR)) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".φ") {
                files.push(format!("{}/{}", BINDINGS_DIR, name));
            }
        }
    }
    for file in files {
        if let Ok(text) = fs::read_to_string(root.join(&file)) {
            if text.contains(stem) {
                return Some(file);
            }
        }
    }
    None
}

fn handover_mentions(root: &Path, stem: &str) -> bool {
    let entries = match fs::read_dir(root.join(LIVE_HANDOVER_DIR)) {
        Ok(entries) => entries,
        Err(_) => return false,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".md") {
            continue;
        }
        if let Ok(text) = fs::read_to_string(&path) {
            if text.contains(stem) {
                return true;
            }
        }
    }
    false
}

fn classify_compiler(
    has_workflow: bool,
    register_file: Option<&str>,
    has_variant: bool,
    in_handover: bool,
) -> String {
    if has_workflow {
        "workflow".to_string()
    } else if let Some(file) = register_file {
        format!("register:{}", file)
    } else if has_variant {
        "variant".to_string()
    } else if in_handover {
        "pending".to_string()
    } else {
        "unregistered".to_string()
    }
}

fn run_compilers_no_directive(root: &Path, entries: &[CompilerEntry]) {
    let wired: BTreeSet<String> = entries.iter().map(|e| e.path.clone()).collect();
    let wired_stems: Vec<String> = entries.iter().map(|e| e.binary.clone()).collect();
    let tree = tree_compilers(root);
    let workflows = workflows_text(root);
    let difference = compiler_difference(&tree, &wired);
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for tc in &difference {
        let register = register_anchor(root, &tc.stem);
        let class = classify_compiler(
            workflows.contains(&tc.stem),
            register.as_deref(),
            wired_variants(&tc.stem, &wired_stems),
            handover_mentions(root, &tc.stem),
        );
        *counts.entry(class.clone()).or_insert(0) += 1;
        println!("{}\t{}", class, tc.path);
    }
    println!(
        "register_lookup --compilers --no-directive: {} tree compilers, {} wired, {} without directive",
        tree.len(),
        wired.len(),
        difference.len()
    );
    for (class, n) in &counts {
        println!("  {}\t{}", class, n);
    }
}

fn run_compilers(args: &[String]) {
    let root = Path::new(".");
    let sources = match fs::read_to_string(SOURCES_PATH) {
        Ok(text) => text,
        Err(_) => {
            println!("register_lookup --compilers: {} absent", SOURCES_PATH);
            return;
        }
    };
    let entries = sources_compiler_entries(&sources);
    if args.iter().any(|a| a == "--no-directive") {
        run_compilers_no_directive(root, &entries);
        return;
    }
    let records = parse_blocks(&sources).len();
    let mut domains: BTreeMap<String, usize> = BTreeMap::new();
    for entry in &entries {
        let format = match entry.format.clone() {
            Some(f) => f,
            None => "absent".to_string(),
        };
        *domains
            .entry(format_domain(&format).to_string())
            .or_insert(0) += 1;
        let url = match entry.url.as_deref() {
            Some(u) => u,
            None => "absent",
        };
        let anchor = match entry.anchor.as_deref() {
            Some(a) => a,
            None => "absent",
        };
        println!("{} | {} | {} | at {}", entry.binary, format, url, anchor);
    }
    let distinct: BTreeSet<&str> = entries.iter().map(|e| e.binary.as_str()).collect();
    let tagged = entries.iter().filter(|e| !e.tags.is_empty()).count();
    println!(
        "register_lookup --compilers: {} records, {} with compiler, {} distinct binaries, {} tagged, {} domains",
        records,
        entries.len(),
        distinct.len(),
        tagged,
        domains.len()
    );
    for (domain, n) in &domains {
        println!("  {}\t{}", domain, n);
    }
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
    if args.iter().any(|a| a == "--dropped-roster") {
        run_dropped_roster(&args);
        return;
    }
    if args.iter().any(|a| a == "--orphans") {
        run_orphans(&args);
        return;
    }
    if args.iter().any(|a| a == "--orphan-docs") {
        run_orphan_docs(Path::new("."));
        return;
    }
    if args.iter().any(|a| a == "--addressed") {
        run_addressed(&args);
        return;
    }
    if args.iter().any(|a| a == "--stale") {
        run_stale(&args);
        return;
    }
    if args.iter().any(|a| a == "--fired") {
        run_fired(&args);
        return;
    }
    if args.iter().any(|a| a == "--descoped-check") {
        run_descoped_check(&args);
        return;
    }
    if args.iter().any(|a| a == "--history") {
        run_history(&args);
        return;
    }
    if args.iter().any(|a| a == "--compilers") {
        run_compilers(&args);
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
    fn compilers_block_parsing_reads_fields() {
        let text = "\
url https://example.org/a.bin
format ephemeris_binary
origin https://example.org/
compiler tools/harvest/src/bin/ephemeris_compiler.rs
at receiver-a
tags alpha beta

url https://example.org/b.bin
compiler tools/measure/src/bin/weberin_verdicts_compiler.rs
";
        let entries = sources_compiler_entries(text);
        assert_eq!(entries.len(), 2);
        assert_eq!(
            entries[0].path,
            "tools/harvest/src/bin/ephemeris_compiler.rs"
        );
        assert_eq!(entries[0].binary, "ephemeris_compiler");
        assert_eq!(entries[0].format.as_deref(), Some("ephemeris_binary"));
        assert_eq!(entries[0].url.as_deref(), Some("https://example.org/a.bin"));
        assert_eq!(entries[0].anchor.as_deref(), Some("receiver-a"));
        assert_eq!(
            entries[0].tags,
            vec!["alpha".to_string(), "beta".to_string()]
        );
        assert_eq!(entries[1].binary, "weberin_verdicts_compiler");
        assert_eq!(entries[1].format, None);
        assert_eq!(entries[1].anchor, None);
    }

    #[test]
    fn compilers_block_parsing_skips_blocks_without_compiler() {
        let text =
            "url https://example.org/a.bin\nat receiver-a\n\nurl https://example.org/b.bin\n";
        assert!(sources_compiler_entries(text).is_empty());
    }

    #[test]
    fn compilers_set_difference_keeps_unwired() {
        let tree = vec![
            TreeCompiler {
                path: "tools/harvest/src/bin/a_compiler.rs".to_string(),
                stem: "a_compiler".to_string(),
            },
            TreeCompiler {
                path: "tools/harvest/src/bin/b_compiler.rs".to_string(),
                stem: "b_compiler".to_string(),
            },
        ];
        let wired: BTreeSet<String> = ["tools/harvest/src/bin/a_compiler.rs".to_string()]
            .into_iter()
            .collect();
        let difference = compiler_difference(&tree, &wired);
        assert_eq!(difference.len(), 1);
        assert_eq!(difference[0].stem, "b_compiler");
    }

    #[test]
    fn compilers_classifier_prefers_stronger_channels() {
        assert_eq!(
            classify_compiler(true, Some("phi/witnesses.\u{3c6}"), true, true),
            "workflow"
        );
        assert_eq!(
            classify_compiler(false, Some("phi/witnesses.\u{3c6}"), true, true),
            "register:phi/witnesses.\u{3c6}"
        );
        assert_eq!(classify_compiler(false, None, true, true), "variant");
        assert_eq!(classify_compiler(false, None, false, true), "pending");
        assert_eq!(classify_compiler(false, None, false, false), "unregistered");
    }

    #[test]
    fn compilers_variant_uses_common_prefix() {
        let wired = vec!["goes_xrs_compiler".to_string()];
        assert!(wired_variants("goes_r_xrs_compiler", &wired));
        assert!(!wired_variants("auger_compiler", &wired));
        assert!(!wired_variants("goes_xrs_compiler", &wired));
    }

    #[test]
    fn compilers_format_domain_takes_prefix() {
        assert_eq!(format_domain("ephemeris_binary"), "ephemeris");
        assert_eq!(format_domain("json"), "json");
        assert_eq!(format_domain("_leading"), "_leading");
    }

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
    fn ereignisse_owner_tags_account_and_send_to_future() {
        assert_eq!(ereignisse_owner("account"), Some("future"));
        assert_eq!(ereignisse_owner("send"), Some("future"));
        assert_eq!(ereignisse_owner("wort"), None);
        assert_eq!(ereignisse_owner("tool:bash"), None);
    }

    #[test]
    fn scan_ereignisse_surfaces_account_and_send_as_future() {
        let dir = env::temp_dir();
        let path = dir.join(format!("register_lookup_ereignisse_{}", std::process::id()));
        fs::write(
            &path,
            "# header\n2026-09-28T17:05:00Z | s1 | future | account | moon.bao.ac.cn\n2026-09-28T17:06:00Z | s1 | future | send | superdarn-af68c4f1\n2026-09-28T17:07:00Z | s1 | plan | wort | ein Wort\n2026-09-28T17:08:00Z | s1 | tool:bash | sread\n",
        )
        .unwrap();
        let mut out = Vec::new();
        let n = scan_ereignisse(&path, &mut out);
        assert_eq!(n, 2);
        assert!(out[0].starts_with("EREIGNIS\t"));
        assert!(out[0].contains("[future]"));
        assert!(out[0].contains("account"));
        assert!(out[1].contains("send"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn open_marker_matches_open_line_and_rejects_closed() {
        assert!(open_marker_matches("offen: X"));
        assert!(!open_marker_matches("closed and finished"));
    }

    #[test]
    fn inline_code_terms_are_not_open_markers() {
        assert!(!open_marker_matches(
            "this is not `pending`, it is a register duty."
        ));
        assert!(!open_marker_matches(
            "An expired entry is `pending` with a due, never a copy."
        ));
        assert!(!open_marker_matches("no `pending`, no deferral."));
        assert!(open_marker_matches(
            "legacy 40-byte bins stay dark, pending recompilation"
        ));
    }

    #[test]
    fn blocked_counts_only_as_a_status_token() {
        assert!(!open_marker_matches(
            "the work is *done or genuinely blocked*, not as a substitute"
        ));
        assert!(!open_marker_matches(
            "not machine-readable/blocked (8 cases)"
        ));
        assert!(open_marker_matches("blocked account: needs a key"));
        assert!(open_marker_matches("**Status:** blocked"));
    }

    #[test]
    fn wartet_marks_a_word_not_a_substring_and_blocked_stays_a_status_token() {
        assert!(!open_marker_matches(
            "ein measure-probe z\u{e4}hlt erwartete vs. leere Zellen"
        ));
        assert!(!open_marker_matches(
            "OA = (Variabilit\u{e4}t_beobachtet \u{2212} Variabilit\u{e4}t_erwartet)"
        ));
        assert!(open_marker_matches("wartet"));
        assert!(!open_marker_matches(
            "the work is *done or genuinely blocked*, not as a substitute"
        ));
        assert!(open_marker_matches("blocked account: needs a key"));
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
            Some("mountain")
        );
        assert_eq!(
            disposition_owner("blocked parser-def odf"),
            Some("mountain")
        );
        assert_eq!(disposition_owner("blocked account"), Some("future"));
        assert_eq!(disposition_owner("blocked key"), Some("future"));
        assert_eq!(disposition_owner("blocked ip-blocked"), Some("mycelium"));
        assert_eq!(disposition_owner("pending"), Some("mycelium"));
        assert_eq!(disposition_owner("terms unbestimmt"), Some("mountain"));
        assert_eq!(disposition_owner("terms ohne-lizenz"), Some("mountain"));
        assert_eq!(disposition_owner("descoped"), None);
    }

    #[test]
    fn status_owner_maps_on_first_token() {
        assert_eq!(disposition_owner("parser-def cdf"), Some("mountain"));
        assert_eq!(disposition_owner("blocked account"), Some("future"));
        assert_eq!(
            disposition_owner("blocked parser-def odf"),
            Some("mountain")
        );
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
        assert!(open_out[0].starts_with("DISPOSITION\tb.\u{3c6}:3\t[mycelium] pending"));
        assert!(open_out[1].contains("[future] blocked account"));
        assert!(released_out[0].starts_with("RELEASED\tb.\u{3c6}:7\t"));
        assert!(released_out[0].contains("descoped"));
    }

    #[test]
    fn scan_dispositions_tags_released_as_released() {
        let text = "released\nurl https://x\nnote → phi/sources.φ:15349\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_dispositions_text(text, "b.\u{3c6}", &mut open_out, &mut released_out);
        assert_eq!(n, 1);
        assert!(open_out.is_empty());
        assert!(released_out[0].starts_with("RELEASED\tb.\u{3c6}:1\t"));
        assert!(released_out[0].contains("released"));
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
    fn scan_dispositions_reads_the_terms_field_orthogonal_to_the_head() {
        let text =
            "pending\nurl https://example.org/x\nterms unbestimmt\nnote keine Lizenz-Direktive\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_dispositions_text(text, "b.\u{3c6}", &mut open_out, &mut released_out);
        assert_eq!(n, 2, "{:?}", open_out);
        assert_eq!(open_out.len(), 2);
        assert!(open_out[0].starts_with("DISPOSITION\tb.\u{3c6}:1\t[mycelium] pending"));
        assert!(open_out[1].starts_with("DISPOSITION\tb.\u{3c6}:3\t[mountain] terms unbestimmt"));
    }

    #[test]
    fn state_class_maps_every_register_state() {
        let table: &[(&str, Option<StateClass>)] = &[
            ("ausstehend", Some(StateClass::Open("mycelium"))),
            ("verifiziert", Some(StateClass::Open("mycelium"))),
            ("kompiliert", Some(StateClass::Open("mycelium"))),
            ("parser-gap", Some(StateClass::Open("mountain"))),
            ("void", Some(StateClass::Released)),
            ("disponiert", Some(StateClass::Released)),
            ("pending", Some(StateClass::Open("mycelium"))),
            ("erledigt", Some(StateClass::Released)),
            ("ausgelagert", Some(StateClass::Released)),
            ("descoped", Some(StateClass::Released)),
            ("released", Some(StateClass::Released)),
            ("fehlt", Some(StateClass::Open("mycelium"))),
            ("offen", Some(StateClass::Open("mycelium"))),
            ("absent", Some(StateClass::Open("mycelium"))),
            ("declined", Some(StateClass::Released)),
            ("refused", Some(StateClass::Released)),
            ("asset fehlt", Some(StateClass::Open("mountain"))),
            ("terms unbestimmt", Some(StateClass::Open("mountain"))),
            ("terms ohne-lizenz", Some(StateClass::Open("mountain"))),
            ("asset present", Some(StateClass::Ignored)),
            ("review", Some(StateClass::Open("mycelium"))),
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
        assert!(open_out[0].starts_with("DISPOSITION\tl.\u{3c6}:1\t[mycelium] ausstehend"));
        assert!(open_out[1].starts_with("DISPOSITION\tl.\u{3c6}:5\t[mountain] parser-gap"));
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
        assert!(open_out[0].starts_with("DISPOSITION\th.\u{3c6}:1\t[mountain] asset fehlt"));
    }

    #[test]
    fn scan_index_text_tags_owner_and_splits_released_and_unmapped() {
        let text = "# header\nausstehend 10 pipeline/queue/x.φ\nerledigt 3 archive/y\nausgelagert 2 archive-root/z\nindex 1 pipeline/catalog/\nvermerkt 9 weird\ngeneriert sources_index.φ\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_index_text(text, "i.\u{3c6}", &mut open_out, &mut released_out);
        assert_eq!(n, 1);
        assert_eq!(open_out.len(), 2);
        assert_eq!(released_out.len(), 2);
        assert!(open_out[0].starts_with("DISPOSITION\ti.\u{3c6}:2\t[mycelium] ausstehend"));
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
        assert!(open_out[0].starts_with("DISPOSITION\ts.\u{3c6}:5\t[mycelium] pending"));
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
    fn witness_absent_is_terminal_not_an_open_duty() {
        let text = "note Band absent, 0 honored\nnote absent; pending (harvest open)\nnote declined by measurement\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_note_markers_text(
            text,
            "w.\u{3c6}",
            &["pending"],
            &["declined"],
            &mut open_out,
            &mut released_out,
        );
        assert_eq!(n, 1);
        assert_eq!(open_out.len(), 1);
        assert_eq!(released_out.len(), 1);
        assert!(open_out[0].starts_with("DISPOSITION\tw.\u{3c6}:2\t[mycelium] pending"));
        assert!(released_out[0].starts_with("RELEASED\tw.\u{3c6}:3\tdeclined"));
    }

    #[test]
    fn footprint_absent_is_terminal_not_an_open_duty() {
        let text = "note Band absent, 0 honored\nnote absent; pending (harvest open)\nnote refused by measurement\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_note_markers_text(
            text,
            "f.\u{3c6}",
            &["pending"],
            &["refused"],
            &mut open_out,
            &mut released_out,
        );
        assert_eq!(n, 1);
        assert_eq!(open_out.len(), 1);
        assert_eq!(released_out.len(), 1);
        assert!(open_out[0].starts_with("DISPOSITION\tf.\u{3c6}:2\t[mycelium] pending"));
        assert!(released_out[0].starts_with("RELEASED\tf.\u{3c6}:3\trefused"));
    }

    #[test]
    fn nrs_absent_is_terminal_not_an_open_duty() {
        let text = "note water depth absent, 0 honored\nnote depth pending (harvest open)\n";
        let mut open_out = Vec::new();
        let mut released_out = Vec::new();
        let n = scan_note_markers_text(
            text,
            "n.\u{3c6}",
            &["pending"],
            &[],
            &mut open_out,
            &mut released_out,
        );
        assert_eq!(n, 1);
        assert_eq!(open_out.len(), 1);
        assert_eq!(released_out.len(), 0);
        assert!(open_out[0].starts_with("DISPOSITION\tn.\u{3c6}:2\t[mycelium] pending"));
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
        assert!(open_out[0].starts_with("DISPOSITION\tp.\u{3c6}:1\t[mycelium] review"));
        assert!(open_out[1].starts_with("DISPOSITION\tp.\u{3c6}:2\t[mycelium] pending"));
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
        let (n, skipped) = scan_catalog_candidates(&dir, &BTreeSet::new(), &mut out);
        assert_eq!(n, 2);
        assert_eq!(skipped, 0);
        assert_eq!(out.len(), 1);
        assert!(out[0].starts_with("CANDIDATES\t"));
        assert!(out[0].contains("cat_a.\u{3c6}\t2 \u{2192} mycelium"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn scan_catalog_candidates_dedupes_against_register_urls() {
        let dir = env::temp_dir().join(format!(
            "register_lookup_catalog_disposed_{}",
            std::process::id()
        ));
        let _ = fs::create_dir_all(&dir);
        let reg = dir.join("declined_tmp.\u{3c6}");
        fs::write(
            &reg,
            "decline no-physical-force\nurl https://dead\nnote gone\n\npending\nreg https://portal\n",
        )
        .unwrap();
        let cat = dir.join("cat_d.\u{3c6}");
        fs::write(
            &cat,
            "# header\ncandidate https://a\ncandidate https://dead\ncandidate https://portal\ncandidate https://b\n",
        )
        .unwrap();
        let disposed = collect_disposed_urls(&[reg.to_str().unwrap()]);
        assert!(disposed.contains("https://dead"));
        assert!(disposed.contains("https://portal"));
        let mut out = Vec::new();
        let (n, skipped) = scan_catalog_candidates(&dir, &disposed, &mut out);
        assert_eq!(n, 2);
        assert_eq!(skipped, 2);
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("cat_d.\u{3c6}\t2 \u{2192} mycelium"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_handover_name_reads_folge_and_single_session() {
        assert_eq!(
            parse_handover_name("handover-2026-09-20-mountain-folge113.md"),
            Some(("mountain".to_string(), "2026-09-20".to_string(), Some(113)))
        );
        assert_eq!(
            parse_handover_name("handover-2026-09-20-operator-entscheidungen.md"),
            Some((
                "operator-entscheidungen".to_string(),
                "2026-09-20".to_string(),
                None
            ))
        );
        assert_eq!(
            parse_handover_name("handover-2026-09-15-mountain-folge33-p8-gate.md"),
            Some(("mountain".to_string(), "2026-09-15".to_string(), Some(33)))
        );
        assert_eq!(parse_handover_name("not-a-handover.md"), None);
    }

    #[test]
    fn extract_open_points_reads_container_rows_and_thread_headings() {
        let text = "# H\n\n## Offen\n\n| Punkt | Status | Bindung | Schritt |\n|---|---|---|---|\n| alpha beta gamma delta epsilon zeta | `wartend` | `termin` | run |\n\n## Stehender Pass (gemessen)\n\n- **HEAD** `abc` == `origin/main`.\n\n## Zwei rote Gates \u{2014} unvollst\u{e4}ndig \u{b7} `pending`\n\n- ein weiterer offener Punkt\n";
        let points = extract_open_points(text);
        let texts: Vec<&str> = points.iter().map(|p| p.text.as_str()).collect();
        assert!(texts.iter().any(|t| t.starts_with("alpha beta gamma")));
        assert!(texts.iter().any(|t| t.starts_with("Zwei rote Gates")));
        assert!(texts.iter().any(|t| t.starts_with("ein weiterer")));
        assert!(!texts.iter().any(|t| t.starts_with("HEAD")));
    }

    #[test]
    fn extract_open_points_keys_shared_status_points_by_their_headings() {
        let text = "## Pending points\n\n### alpha-driver\n- **Status:** pending | **Owner:** self\n- **Need:** alpha\n\n### beta-live-parity\n- **Status:** pending | **Owner:** self\n- **Need:** beta\n";
        let points = extract_open_points(text);
        let keys: Vec<String> = points
            .iter()
            .filter_map(|p| match_prefix(&point_key_tokens(&p.text)))
            .collect();
        let alpha = keys
            .iter()
            .find(|k| k.starts_with("alpha-driver"))
            .expect("alpha-driver heading must be keyed by its title");
        let beta = keys
            .iter()
            .find(|k| k.starts_with("beta-live-parity"))
            .expect("beta-live-parity heading must be keyed by its title");
        assert_ne!(alpha, beta);
        assert!(
            !keys.iter().any(|k| k.contains("owner")),
            "the shared Status line must not be a point key: {:?}",
            keys
        );
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
        assert_eq!(tokens[0], "queue-korpora");
        assert!(!tokens.iter().any(|t| t == "ausstehend"));
        assert_eq!(
            match_prefix(&tokens),
            Some("queue-korpora 30-astro earth-stac-sentinel".to_string())
        );
    }

    #[test]
    fn point_key_ignores_a_leading_enumeration_number() {
        let seven = match_prefix(&point_key_tokens("7. Riss 4 Ksg (WGSL)"));
        let eight = match_prefix(&point_key_tokens("8. Riss 4 Ksg (WGSL)"));
        assert_eq!(seven, Some("riss 4 ksg wgsl".to_string()));
        assert_eq!(seven, eight);
        let other = match_prefix(&point_key_tokens("7. Riss 4 Ksg (Rust)"));
        assert_ne!(seven, other);
    }

    #[test]
    fn count_flag_reads_the_count_switch() {
        assert!(count_flag(&[
            "--dropped".to_string(),
            "--count".to_string()
        ]));
        assert!(!count_flag(&["--dropped".to_string()]));
    }

    #[test]
    fn orphan_candidates_carry_owner_url_and_name() {
        let text = "note preamble\n\nparser-def json\nurl https://example.org/data.json\nnote BGS-FDSN event: field magnitude unit absent\n\ndescoped\nurl https://x\nnote nie gebaut\n\npending\nurl https://y\nnote Argo BGC cores: field unit absent\n";
        let cs = collect_orphan_candidates_in(text, "b.\u{3c6}");
        assert_eq!(cs.len(), 2);
        assert_eq!(cs[0].owner, "mountain");
        assert_eq!(cs[0].lineno, 3);
        assert_eq!(cs[0].url, "https://example.org/data.json");
        assert_eq!(cs[0].name, "BGS-FDSN event");
        assert_eq!(cs[1].owner, "mycelium");
        assert_eq!(cs[1].name, "Argo BGC cores");
    }

    #[test]
    fn orphan_key_name_falls_back_to_host_without_note() {
        assert_eq!(
            orphan_key_name("", "https://coastwatch.noaa.gov/x.csv"),
            "coastwatch.noaa.gov"
        );
    }

    #[test]
    fn orphan_held_matches_url_or_leading_name() {
        let carried = "der punkt giro ionosonde fof2_mhz wartet";
        assert!(orphan_held(carried, "", "GIRO Ionosonde"));
        assert!(!orphan_held(carried, "", "SANSA Ionosonde"));
        assert!(orphan_held(
            "siehe https://gea.esac.esa.int/tap",
            "https://gea.esac.esa.int/tap",
            ""
        ));
        assert!(!orphan_held("nichts", "https://a.example.org/x", ""));
    }

    #[test]
    fn orphan_report_flags_unheld_entries() {
        let base = env::temp_dir().join(format!("rl-orphan-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        fs::create_dir_all(base.join("phi")).unwrap();
        fs::write(
            base.join("docs/handover/handover-2026-09-25-mountain-folge150.md"),
            "### Carried\n- **Braucht:** fix Argo BGC cores\n",
        )
        .unwrap();
        fs::write(
            base.join("phi/blocked_sources.\u{3c6}"),
            "parser-def json\nurl https://example.org/held\nnote Argo BGC cores: unit absent\n\nparser-def json\nurl https://example.org/lost\nnote MIROVA: unit absent\n",
        )
        .unwrap();
        let (lines, summary) = orphan_report_with(&base, &|_| None);
        assert_eq!(orphan_total(&summary), 1, "{:?}", lines);
        assert_eq!(summary.get("mountain"), Some(&(0usize, 1usize)));
        assert!(lines.iter().any(|l| l.starts_with(
            "ORPHAN_UNCOMMITTED\tphi/blocked_sources.\u{3c6}:5\t[mountain]\thttps://example.org/lost"
        )));
        assert!(
            lines
                .iter()
                .all(|l| !l.contains("https://example.org/held"))
        );
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn orphan_report_marks_absent_private_carrier_as_unverifiable() {
        let base = env::temp_dir().join(format!("rl-orphan-private-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        fs::create_dir_all(base.join("phi")).unwrap();
        fs::write(
            base.join("phi/blocked_sources.\u{3c6}"),
            "blocked account\nurl https://moon.bao.ac.cn/\nnote Chang'e GRAS: Konto-gated\n",
        )
        .unwrap();
        assert!(!base.join(PRIVATE_HANDOVER_DIR).exists());
        let (lines, summary) = orphan_report_with(&base, &|_| None);
        assert!(
            lines.iter().any(|l| l.starts_with(
                "UNVERIFIABLE_PRIVATE\tphi/blocked_sources.\u{3c6}:1\t[future]\thttps://moon.bao.ac.cn/"
            )),
            "{:?}",
            lines
        );
        assert_eq!(orphan_total(&summary), 0, "{:?}", lines);
        assert_eq!(summary.get("future"), None);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn orphan_report_marks_a_head_entry_as_committed() {
        let base = env::temp_dir().join(format!("rl-orphan-head-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        fs::create_dir_all(base.join("phi")).unwrap();
        fs::write(
            base.join("phi/blocked_sources.\u{3c6}"),
            "parser-def json\nurl https://example.org/lost\nnote MIROVA: unit absent\n",
        )
        .unwrap();
        let (lines, summary) = orphan_report_with(&base, &|_| {
            Some(
                "parser-def json\nurl https://example.org/lost\nnote MIROVA: unit absent\n"
                    .to_string(),
            )
        });
        assert_eq!(summary.get("mountain"), Some(&(1usize, 0usize)));
        assert!(
            lines
                .iter()
                .any(|l| l
                    .starts_with("ORPHAN_COMMITTED\tphi/blocked_sources.\u{3c6}:1\t[mountain]"))
        );
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn doc_carried_matches_basename_and_stem() {
        let carrier = "see foo-bar-baz and alpha-beta notes";
        assert!(doc_carried(carrier, "docs/specs/foo-bar-baz.md"));
        assert!(doc_carried(carrier, "docs/specs/alpha-beta.md"));
        assert!(!doc_carried(carrier, "docs/specs/nope.md"));
    }

    #[test]
    fn orphan_docs_flags_uncarried_open_document() {
        let base = env::temp_dir().join(format!("rl-orphandocs-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        fs::create_dir_all(base.join("docs/specs")).unwrap();
        fs::write(
            base.join("docs/handover/handover-2026-09-25-mountain-folge150.md"),
            "# h\n- carried: docs/specs/carried.md\n",
        )
        .unwrap();
        let header = "<!--\n  title: t\n  class: ref\n  date: 2026-01-01\n  sha256: x\n-->\n";
        fs::write(
            base.join("docs/specs/carried.md"),
            format!("{header}# c\nOffener Punkt: noch zu bauen\n"),
        )
        .unwrap();
        fs::write(
            base.join("docs/specs/lost.md"),
            format!("{header}# l\nOffener Punkt: noch zu bauen\n"),
        )
        .unwrap();
        let lines = orphan_docs(&base);
        assert!(
            lines
                .iter()
                .any(|l| l.starts_with("ORPHAN_DOC\tdocs/specs/lost.md\t")),
            "{:?}",
            lines
        );
        assert!(lines.iter().all(|l| !l.contains("carried.md")));
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn open_markers_match_the_gate_contract() {
        assert_eq!(
            OPEN_MARKERS.to_vec(),
            omegaflow::commit_gate::DOC_OPEN_MARKERS.to_vec()
        );
    }

    #[test]
    fn orphan_class_carrier_holds_entries_and_reports_drift() {
        let base = env::temp_dir().join(format!("rl-orphan-class-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        fs::create_dir_all(base.join("phi")).unwrap();
        fs::write(
            base.join("docs/handover/handover-2026-09-25-mountain-folge150.md"),
            "### Class carrier\n- **Braucht:** fix phi/blocked_sources.\u{3c6}::gap:unit-auto-detect \u{d7}2\n",
        )
        .unwrap();
        fs::write(
            base.join("phi/blocked_sources.\u{3c6}"),
            "parser-def json\ngap unit-auto-detect\nurl https://example.org/a\nnote Alpha: unit absent\n\nparser-def json\ngap unit-auto-detect\nurl https://example.org/b\nnote Beta: unit absent\n\nparser-def json\ngap force-undetermined\nurl https://example.org/c\nnote Gamma: force undetermined\n",
        )
        .unwrap();
        let (lines, summary) = orphan_report_with(&base, &|_| None);
        assert_eq!(orphan_total(&summary), 1, "{:?}", lines);
        assert!(lines.iter().all(|l| !l.contains("https://example.org/a")));
        assert!(lines.iter().all(|l| !l.contains("https://example.org/b")));
        assert!(lines.iter().any(|l| l.contains("https://example.org/c")));
        assert!(lines.iter().any(|l| l.starts_with(
            "ORPHAN_CLASS\tphi/blocked_sources.\u{3c6}::gap:force-undetermined\t[mountain]\t1"
        )));
        assert!(
            lines.iter().all(|l| !l.starts_with("CARRIER_DRIFT")),
            "{:?}",
            lines
        );
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn orphan_class_reports_carrier_count_drift() {
        let base = env::temp_dir().join(format!("rl-orphan-drift-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        fs::create_dir_all(base.join("phi")).unwrap();
        fs::write(
            base.join("docs/handover/handover-2026-09-25-mountain-folge150.md"),
            "phi/blocked_sources.\u{3c6}::gap:unit-auto-detect \u{d7}5\n",
        )
        .unwrap();
        fs::write(
            base.join("phi/blocked_sources.\u{3c6}"),
            "parser-def json\ngap unit-auto-detect\nurl https://example.org/a\nnote Alpha: unit absent\n",
        )
        .unwrap();
        let (lines, _) = orphan_report_with(&base, &|_| None);
        assert!(
            lines.iter().any(|l| l.starts_with(
                "CARRIER_DRIFT\tphi/blocked_sources.\u{3c6}::gap:unit-auto-detect\tcarrier=5\tlive=1"
            )),
            "{:?}",
            lines
        );
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn orphan_no_gap_marks_mountain_parser_entries() {
        let cs = collect_orphan_candidates_in(
            "parser-def json\nurl https://x\nnote A: unit absent\n",
            "b.\u{3c6}",
        );
        assert_eq!(cs.len(), 1);
        assert!(cs[0].no_gap);
        assert!(cs[0].class_key.is_empty());
    }

    #[test]
    fn distinctive_token_prefers_the_long_word_and_rejects_stopwords() {
        let tokens = vec!["am".to_string(), "head".to_string(), "8218f46a".to_string()];
        assert_eq!(distinctive_token(&tokens), Some("8218f46a".to_string()));
        let words = vec!["der".to_string(), "die".to_string()];
        assert_eq!(distinctive_token(&words), None);
    }

    #[test]
    fn resolution_status_accepts_own_or_foreign_resolution() {
        assert_eq!(resolution_status(Some(true), None), "resolved");
        assert_eq!(resolution_status(None, Some(true)), "resolved");
        assert_eq!(resolution_status(Some(false), Some(true)), "resolved");
        assert_eq!(resolution_status(Some(false), Some(false)), "none");
        assert_eq!(resolution_status(None, None), "none");
    }

    #[test]
    fn canonical_line_maps_renamed_lines_to_their_successor() {
        assert_eq!(canonical_line("bau"), "mountain");
        assert_eq!(canonical_line("ernte"), "mycelium");
        assert_eq!(canonical_line("forschung"), "sensory");
        assert_eq!(canonical_line("mountain"), "mountain");
        assert_eq!(canonical_line("sensory"), "sensory");
        assert_eq!(canonical_line("future"), "future");
    }

    #[test]
    fn private_successor_exists_names_same_line_next_folge_only() {
        let dir = env::temp_dir().join(format!("register_lookup_private_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("handover-2026-09-23-mountain-folge126.md");
        fs::write(&file, "<!--\nclass: handover\n-->\n").unwrap();
        let dir_str = dir.to_str().unwrap().to_string();
        let absent = dir.join("absent");
        let absent_str = absent.to_str().unwrap().to_string();

        assert!(private_successor_exists_in(&dir_str, "mountain", 125));
        assert!(!private_successor_exists_in(&dir_str, "mountain", 124));
        assert!(!private_successor_exists_in(&dir_str, "sensory", 125));
        assert!(!private_successor_exists_in(&absent_str, "mountain", 125));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn stale_points_flag_identical_lage_over_three_folgen() {
        let base = env::temp_dir().join(format!("rl-stale-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        let body = "# h\n\n## Offen\n\n### alpha-driver\n- **Status:** wartend\n- **Lage:** waiting on X\n";
        for (date, folge) in [("2026-09-01", 1u32), ("2026-09-02", 2), ("2026-09-03", 3)] {
            fs::write(
                base.join(format!(
                    "docs/handover/handover-{}-mountain-folge{}.md",
                    date, folge
                )),
                body,
            )
            .unwrap();
        }
        let handovers = collect_live_handovers_in(&base);
        let out = stale_points(&handovers, None, 3);
        assert!(out.iter().any(|l| l.contains("alpha-driver")), "{:?}", out);
        assert_eq!(out.len(), 1, "{:?}", out);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn fired_points_flag_past_date_and_manual_word() {
        let base = env::temp_dir().join(format!("rl-fired-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        let body = "# h\n\n## Offen\n\n### vergangen\n- **Status:** wartend\n- **Trigger:** 2024-01-01\n\n### zukunft\n- **Status:** termin\n- **Trigger:** 2999-01-01\n\n### wort-punkt\n- **Status:** operator-gebunden\n- **Trigger:** Wort: /consent\n";
        fs::write(
            base.join("docs/handover/handover-2026-09-25-mountain-folge9.md"),
            body,
        )
        .unwrap();
        let handovers = collect_live_handovers_in(&base);
        let head = "a".repeat(40);
        let (out, fired) = fired_points(&handovers, None, Some(20000), Some(head.as_str()));
        assert!(
            out.iter()
                .any(|l| l.starts_with("FIRED\t") && l.contains("vergangen")),
            "{:?}",
            out
        );
        assert!(
            out.iter()
                .any(|l| l.starts_with("FIRED_MANUAL") && l.contains("wort-punkt")),
            "{:?}",
            out
        );
        assert!(out.iter().all(|l| !l.contains("zukunft")), "{:?}", out);
        assert_eq!(fired, 2, "{:?}", out);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn fired_points_ignore_waiting_mail_trigger_but_flag_measured_ids() {
        let base = env::temp_dir().join(format!("rl-fired-mail-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        let body = "# h\n\n## Offen\n\n### warte-mail\n- **Status:** wartend\n- **Trigger:** wartend Mail-Eingang\n\n### mail-antwort\n- **Status:** wartend\n- **Trigger:** `mail_ledger:1790533094` Antwort\n\n### ci-lauf\n- **Status:** wartend\n- **Trigger:** ci-check-Lauf 36385567226\n";
        fs::write(
            base.join("docs/handover/handover-2026-09-28-mountain-folge9.md"),
            body,
        )
        .unwrap();
        let handovers = collect_live_handovers_in(&base);
        let head = "a".repeat(40);
        let (out, _fired) = fired_points(&handovers, None, Some(20000), Some(head.as_str()));
        assert!(!out.iter().any(|l| l.contains("warte-mail")), "{:?}", out);
        assert!(out.iter().any(|l| l.contains("mail-antwort")), "{:?}", out);
        assert!(out.iter().any(|l| l.contains("ci-lauf")), "{:?}", out);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn fired_points_read_inline_trigger_not_measurement_stamp() {
        let base = env::temp_dir().join(format!("rl-fired-stamp-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        let body = "# h\n\n## Offen\n\n- **ox64-m2c** (wartend) \u{2014} Trigger Zustellung LZ473049629CN; measured 2024-09-28; ETA 2024-10-20.\n\n- **enso-cut** (wartend) | (measured 2024-09-28) state/zustand/wartend.\u{3c6}:23.\n\n- **enso-zuschnitt** (wartend) (gemessen 2024-09-28) \u{2014} no render carries a status line.\n\n- **inline-trigger** (termin) \u{2014} *Trigger:* 2024-09-01.\n";
        fs::write(
            base.join("docs/handover/handover-2026-09-29-mountain-folge9.md"),
            body,
        )
        .unwrap();
        let handovers = collect_live_handovers_in(&base);
        let head = "a".repeat(40);
        let (out, fired) = fired_points(&handovers, None, Some(20000), Some(head.as_str()));
        assert!(
            !out.iter().any(|l| l.contains("ox64-m2c")),
            "a Lage measurement stamp must not fire: {:?}",
            out
        );
        assert!(
            !out.iter().any(|l| l.contains("enso-cut")),
            "a pipe-field measurement stamp must not fire: {:?}",
            out
        );
        assert!(
            !out.iter().any(|l| l.contains("enso-zuschnitt")),
            "a leading-region measurement stamp before the delimiter must not fire: {:?}",
            out
        );
        assert!(
            out.iter()
                .any(|l| l.starts_with("FIRED\t") && l.contains("inline-trigger")),
            "an inline Trigger date must fire: {:?}",
            out
        );
        assert_eq!(fired, 1, "{:?}", out);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn fired_points_lage_block_stamp_not_trigger() {
        let base = env::temp_dir().join(format!("rl-fired-block-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        let body = "# h\n\n## Offen\n\n### kein-trigger-feld\n- **Status:** wartend\n- **Lage:** (gemessen 2024-09-28) Trigger weder best\u{e4}tigt noch ausgeschlossen.\n\n### trigger-ohne-datum\n- **Status:** wartend\n- **Trigger:** Zustellung LZ473049629CN.\n- **Lage:** (gemessen 2024-09-28) kein Render.\n\n### vergangen\n- **Status:** termin\n- **Trigger:** 2024-09-01.\n";
        fs::write(
            base.join("docs/handover/handover-2026-09-29-mountain-folge9.md"),
            body,
        )
        .unwrap();
        let handovers = collect_live_handovers_in(&base);
        let head = "a".repeat(40);
        let (out, fired) = fired_points(&handovers, None, Some(20000), Some(head.as_str()));
        assert!(
            !out.iter().any(|l| l.contains("kein-trigger-feld")),
            "a Lage line carrying the word Trigger but no Trigger field must not fire: {:?}",
            out
        );
        assert!(
            !out.iter().any(|l| l.contains("trigger-ohne-datum")),
            "a Trigger field without a date must not read the Lage stamp: {:?}",
            out
        );
        assert!(
            out.iter()
                .any(|l| l.starts_with("FIRED\t") && l.contains("vergangen")),
            "a Trigger field with a past date must fire: {:?}",
            out
        );
        assert_eq!(fired, 1, "{:?}", out);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn fired_points_inline_field_stops_before_lage_and_filenames() {
        let base = env::temp_dir().join(format!("rl-fired-inline-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        let body = "# h\n\n## Offen\n\n- **paused-doctrine** \u{2014} carrier is the pause itself. *Braucht:* operator word.\n- **dated-b** \u{2014} *Status:* termin | *Trigger:* 2024-01-01.\n- **inline-lage** \u{2014} *Status:* wartend | *Bindung:* eigen | *Trigger:* hardware. *Lage:* unmeasured, absent (gemessen 2024-09-28); Wort: kein Mess-Akt jetzt.\n- **filename-trigger** \u{2014} *Status:* wartend | *Trigger:* published in `docs/paper/x-2024-09-03.md`.\n";
        fs::write(
            base.join("docs/handover/handover-2026-09-29-mountain-folge9.md"),
            body,
        )
        .unwrap();
        let handovers = collect_live_handovers_in(&base);
        let head = "a".repeat(40);
        let (out, fired) = fired_points(&handovers, None, Some(20000), Some(head.as_str()));
        assert!(
            !out.iter().any(|l| l.contains("paused-doctrine")),
            "a sibling bullet must not inherit the next point's status/trigger: {:?}",
            out
        );
        assert!(
            !out.iter().any(|l| l.contains("inline-lage")),
            "an inline Trigger must stop before the Lage stamp: {:?}",
            out
        );
        assert!(
            !out.iter().any(|l| l.contains("filename-trigger")),
            "an ISO date inside a filename must not fire: {:?}",
            out
        );
        assert!(
            out.iter()
                .any(|l| l.starts_with("FIRED\t") && l.contains("dated-b")),
            "a standalone inline Trigger date must fire: {:?}",
            out
        );
        assert_eq!(fired, 1, "{:?}", out);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn descoped_check_flags_a_contradicting_open_document() {
        let base = env::temp_dir().join(format!("rl-descoped-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover")).unwrap();
        fs::create_dir_all(base.join("docs/specs")).unwrap();
        let header = "<!--\n  title: t\n  class: handover\n  date: 2026-01-01\n  sha256: x\n-->\n";
        fs::write(
            base.join("docs/handover/handover-2026-09-25-mountain-folge9.md"),
            format!(
                "{header}# h\n\n### alt\n- **Status:** descoped\n- **Quelle:** `docs/specs/x.md`\n"
            ),
        )
        .unwrap();
        let doc_header = "<!--\n  title: t\n  class: ref\n  date: 2026-01-01\n  sha256: x\n-->\n";
        fs::write(
            base.join("docs/specs/x.md"),
            format!("{doc_header}# x\n\n- TODO: build it\n"),
        )
        .unwrap();
        let out = descoped_widerlegt(&base);
        assert!(
            out.iter()
                .any(|l| l.starts_with("descoped-widerlegt\tdocs/specs/x.md\t")),
            "{:?}",
            out
        );
        assert_eq!(out.len(), 1, "{:?}", out);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn addressed_report_extracts_only_the_addressed_block() {
        let base = env::temp_dir().join(format!("rl-addressed-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("docs/handover/archiv")).unwrap();
        fs::write(
            base.join("docs/handover/handover-2026-09-28-sensory-folge9.md"),
            "<!--\n  class: handover\n  status: live\n-->\n# H\n\n## An River\n- the HUD line\n- Origin: sensory-folge9\n\n## An Mountain\n- the ports\n- Origin: sensory-folge9\n",
        )
        .unwrap();
        fs::write(
            base.join("docs/handover/archiv/handover-2026-09-27-sensory-folge8.md"),
            "<!--\n  class: handover\n  status: live\n-->\n# H\n\n## An River\n- the old line\n- Origin: sensory-folge8\n",
        )
        .unwrap();
        let river = addressed_report(&base, "river");
        assert!(
            river
                .iter()
                .any(|l| l.starts_with("ADDRESSED\triver\t<- sensory-folge9")),
            "{:?}",
            river
        );
        assert!(river.iter().any(|l| l.contains("the HUD line")));
        assert!(!river.iter().any(|l| l.contains("the ports")));
        assert!(!river.iter().any(|l| l.contains("the old line")));
        let mountain = addressed_report(&base, "mountain");
        assert!(
            mountain.iter().any(|l| l.contains("the ports")),
            "{:?}",
            mountain
        );
        assert!(!mountain.iter().any(|l| l.contains("the HUD line")));
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn canonical_point_key_absorbs_reformulation() {
        let first = canonical_point_key("1. GIC Breitenband Deskriptoren bauen (wartend)");
        let second = canonical_point_key("bauen Deskriptoren Breitenband GIC blockiert");
        assert_eq!(first, second);
        assert_eq!(first.as_deref(), Some("bauen breitenband deskriptoren gic"));
    }

    #[test]
    fn explicit_id_marker_beats_token_set() {
        let marked = canonical_point_key("**ID:** gic-breitenband this is the point");
        assert_eq!(marked.as_deref(), Some("gic-breitenband"));
        let unmarked = canonical_point_key("gic-breitenband this is the point");
        assert_ne!(marked, unmarked);
        assert_eq!(
            canonical_point_key("ID: alpha-1 some later text").as_deref(),
            Some("alpha-1")
        );
    }

    #[test]
    fn canonical_point_key_keeps_distinct_points_apart() {
        let a = canonical_point_key("GIC Breitenband Deskriptoren bauen");
        let b = canonical_point_key("GIC Breitenband Coverage Begleiter bauen");
        assert_ne!(a, b);
    }

    #[test]
    fn canonical_key_matches_a_carrier_token_set() {
        let key = canonical_point_key("GIC Breitenband Deskriptoren bauen").unwrap();
        assert!(canonical_key_in_carrier(
            " some other words gic and bauen and deskriptoren and breitenband here ",
            &key
        ));
        assert!(!canonical_key_in_carrier(
            " gic bauen deskriptoren only ",
            &key
        ));
    }

    #[test]
    fn dropped_roster_collects_distinct_canonical_point_keys() {
        let text = "## Offen\n\n- GIC Breitenband Deskriptoren bauen\n- bauen Deskriptoren Breitenband GIC (wartend)\n- **ID:** coverage-begleiter Coverage Begleiter bauen\n";
        let keys = canonical_open_point_keys(text);
        assert_eq!(keys.len(), 2);
        assert!(keys.contains("bauen breitenband deskriptoren gic"));
        assert!(keys.contains("coverage-begleiter"));
    }

    #[test]
    fn dropped_roster_skips_burn_and_container_headings() {
        let text = "## Offen (extra)\n\n### alpha point\n- **Status:** wartend\n\n## Burn: open 0.0 close 0.1 pending\n\n## Two red gates incomplete pending\n\n- ein offener Punkt\n";
        let keys = canonical_open_point_keys(text);
        assert!(
            !keys.iter().any(|k| k.contains("burn")),
            "the Burn line must not be a roster point: {:?}",
            keys
        );
        assert!(
            !keys.iter().any(|k| k.contains("extra")),
            "the Offen container heading must not be a roster point: {:?}",
            keys
        );
        assert!(
            keys.iter()
                .any(|k| k.contains("red") && k.contains("gates")),
            "a real status heading stays a point: {:?}",
            keys
        );
    }

    #[test]
    fn roster_diff_names_lost_and_new_keys() {
        let baseline: BTreeSet<String> =
            ["a b".to_string(), "c d".to_string()].into_iter().collect();
        let keys: BTreeSet<String> = ["c d".to_string(), "e f".to_string()].into_iter().collect();
        let (lost, new) = roster_diff(&baseline, &keys);
        assert_eq!(lost, vec!["a b".to_string()]);
        assert_eq!(new, vec!["e f".to_string()]);
    }
}
