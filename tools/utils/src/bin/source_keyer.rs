use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::process::ExitCode;

const USAGE: &str = "source_keyer <queue.φ> <keytable.φ> <register.φ> [--apply]";
const COORD_TOL: f64 = 0.05;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Verdict {
    Name(String),
    Pending,
    Riss,
}

struct Register {
    stations: Vec<(String, f64, f64)>,
}

impl Register {
    fn load(path: &str) -> io::Result<Register> {
        let content = fs::read_to_string(path)?;
        let mut stations = Vec::new();
        for line in content.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('#') {
                continue;
            }
            let mut parts = t.split_whitespace();
            let code = match parts.next() {
                Some(c) => c.to_string(),
                None => continue,
            };
            let lat = match parts.next().and_then(|p| p.parse::<f64>().ok()) {
                Some(v) => v,
                None => continue,
            };
            let lon = match parts.next().and_then(|p| p.parse::<f64>().ok()) {
                Some(v) => v,
                None => continue,
            };
            stations.push((code, lat, lon));
        }
        Ok(Register { stations })
    }

    fn resolve_coords(&self, lat: f64, lon: f64) -> Option<&str> {
        let mut found: Option<&str> = None;
        for (code, slat, slon) in &self.stations {
            if (slat - lat).abs() <= COORD_TOL && (slon - lon).abs() <= COORD_TOL {
                match found {
                    Some(_) => return None,
                    None => found = Some(code.as_str()),
                }
            }
        }
        found
    }
}

struct Report {
    blocks: usize,
    matched: usize,
    applied: usize,
    pending: usize,
    riss_resolved: usize,
    riss_carried: usize,
    riss_lines: Vec<String>,
}

impl Report {
    fn new() -> Report {
        Report {
            blocks: 0,
            matched: 0,
            applied: 0,
            pending: 0,
            riss_resolved: 0,
            riss_carried: 0,
            riss_lines: Vec::new(),
        }
    }
}

fn key_table_load(path: &str) -> io::Result<HashMap<String, Verdict>> {
    let content = fs::read_to_string(path)?;
    let mut table = HashMap::new();
    for line in content.lines() {
        let t = line.trim();
        let rest = match t.strip_prefix("url ") {
            Some(r) => r,
            None => continue,
        };
        let (url, verdict) = match rest.split_once(" | ") {
            Some(pair) => pair,
            None => continue,
        };
        let url_key = fold_url(url.trim());
        let v = if let Some(name) = verdict.strip_prefix("source ") {
            Verdict::Name(name.trim().to_string())
        } else if verdict.starts_with("# pending") {
            Verdict::Pending
        } else if verdict.starts_with("# riss") {
            Verdict::Riss
        } else {
            continue;
        };
        table.insert(url_key, v);
    }
    Ok(table)
}

fn split_path_name(name: &str) -> (String, String) {
    let rest = match name.strip_prefix("magnetosphere_") {
        Some(r) => r,
        None => name,
    };
    let (prefix, code_part) = match rest.strip_prefix("intermagnet_") {
        Some(r) => ("intermagnet_", r),
        None => ("", rest),
    };
    let code = match code_part.split('_').next() {
        Some(c) => c.to_string(),
        None => code_part.to_string(),
    };
    (prefix.to_string(), code)
}

fn fold_url(url: &str) -> String {
    let bare = match url.strip_prefix("https://") {
        Some(s) => s,
        None => match url.strip_prefix("http://") {
            Some(s) => s,
            None => url,
        },
    };
    let mut out = String::with_capacity(bare.len());
    let mut rest = bare;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        match rest[open + 1..].find('}') {
            Some(close) => {
                out.push('*');
                rest = &rest[open + 1 + close + 1..];
            }
            None => {
                out.push('{');
                rest = &rest[open + 1..];
            }
        }
    }
    out.push_str(rest);
    out
}

fn block_url(lines: &[String]) -> Option<String> {
    for line in lines {
        let t = line.trim();
        if let Some(u) = t.strip_prefix("url ") {
            return Some(u.trim().to_string());
        }
    }
    None
}

fn block_coords(lines: &[String]) -> Option<(f64, f64)> {
    for line in lines {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("on earth ") {
            let mut parts = rest.split_whitespace();
            let lat = parts.next().and_then(|p| p.parse::<f64>().ok())?;
            let lon = parts.next().and_then(|p| p.parse::<f64>().ok())?;
            return Some((lat, lon));
        }
    }
    None
}

fn primary_path_code(lines: &[String]) -> Option<(String, String)> {
    for line in lines {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("path 1.0 ") {
            let name = match rest.split_whitespace().next() {
                Some(n) => n.trim(),
                None => "",
            };
            if name.is_empty() {
                continue;
            }
            let (prefix, code) = split_path_name(name);
            return Some((prefix, code));
        }
    }
    None
}

fn source_line_index(lines: &[String]) -> Option<usize> {
    lines
        .iter()
        .position(|l| l.trim_start().starts_with("source "))
}

fn insert_index(lines: &[String]) -> usize {
    for (i, l) in lines.iter().enumerate() {
        let t = l.trim_start();
        if t.starts_with("ttl ") || t.starts_with("force ") {
            return i;
        }
    }
    lines.len()
}

fn apply_source(lines: &mut Vec<String>, name: &str) -> usize {
    let directive = format!("  source {}", name);
    match source_line_index(lines) {
        Some(idx) => {
            if lines[idx] == directive {
                return 0;
            }
            lines[idx] = directive;
            1
        }
        None => {
            let idx = insert_index(lines);
            lines.insert(idx, directive);
            1
        }
    }
}

fn round_key(v: f64) -> i64 {
    (v * 1000.0).round() as i64
}

fn coord_codes_map(lines: &[String]) -> HashMap<(i64, i64), Vec<String>> {
    let mut map: HashMap<(i64, i64), Vec<String>> = HashMap::new();
    let mut block: Vec<&str> = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            register_block(&mut block, &mut map);
        } else {
            block.push(line);
        }
    }
    if !block.is_empty() {
        register_block(&mut block, &mut map);
    }
    map
}

fn register_block(block: &mut Vec<&str>, map: &mut HashMap<(i64, i64), Vec<String>>) {
    let owned: Vec<String> = block.iter().map(|s| s.to_string()).collect();
    if let (Some((lat, lon)), Some((_, code))) = (block_coords(&owned), primary_path_code(&owned)) {
        let key = (round_key(lat), round_key(lon));
        let entry = map.entry(key).or_insert_with(Vec::new);
        if !entry.contains(&code) {
            entry.push(code);
        }
    }
    block.clear();
}

fn resolve_riss(
    block: &mut Vec<String>,
    url: &str,
    register: &Register,
    coord_codes: &HashMap<(i64, i64), Vec<String>>,
    report: &mut Report,
) {
    let coords = match block_coords(block) {
        Some(c) => c,
        None => {
            report.riss_carried += 1;
            report
                .riss_lines
                .push(format!("riss {url}: no on-earth coords"));
            return;
        }
    };
    let code = match register.resolve_coords(coords.0, coords.1) {
        Some(c) => c,
        None => {
            report.riss_carried += 1;
            report
                .riss_lines
                .push(format!("riss {url}: coords unresolved"));
            return;
        }
    };
    let key = (round_key(coords.0), round_key(coords.1));
    if let Some(codes) = coord_codes.get(&key) {
        let divergent = codes.iter().any(|c| !c.eq_ignore_ascii_case(code));
        if divergent {
            report.riss_carried += 1;
            report
                .riss_lines
                .push(format!("riss {url}: register {code} vs {:?}", codes));
            return;
        }
    }
    let (prefix, _path_code) = match primary_path_code(block) {
        Some(p) => p,
        None => {
            report.riss_carried += 1;
            report
                .riss_lines
                .push(format!("riss {url}: no primary path"));
            return;
        }
    };
    let name = format!("magnetosphere_{}{}_hapi", prefix, code.to_lowercase());
    report.riss_resolved += 1;
    report.applied += apply_source(block, &name);
    report
        .riss_lines
        .push(format!("riss {url} -> source {name}"));
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let apply = args.iter().any(|a| a == "--apply");
    let positional: Vec<&String> = args
        .iter()
        .skip(1)
        .filter(|a| a.as_str() != "--apply")
        .collect();
    if positional.len() != 3 {
        eprintln!("{}", USAGE);
        return ExitCode::from(2);
    }
    let queue_path = positional[0].as_str();
    let keytable_path = positional[1].as_str();
    let register_path = positional[2].as_str();

    let table = match key_table_load(keytable_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("key table unreadable: {} ({})", keytable_path, e);
            return ExitCode::from(1);
        }
    };
    let register = match Register::load(register_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("register unreadable: {} ({})", register_path, e);
            return ExitCode::from(1);
        }
    };
    let content = match fs::read_to_string(queue_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("queue unreadable: {} ({})", queue_path, e);
            return ExitCode::from(1);
        }
    };

    let lines: Vec<String> = content.split('\n').map(String::from).collect();
    let coord_codes = coord_codes_map(&lines);

    let mut report = Report::new();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());

    let mut block: Vec<String> = Vec::new();
    for line in lines.iter() {
        if line.trim().is_empty() {
            if !block.is_empty() {
                process_block(
                    &mut block,
                    &table,
                    &register,
                    &coord_codes,
                    &mut out,
                    &mut report,
                );
            }
            out.push(line.clone());
        } else {
            block.push(line.clone());
        }
    }
    if !block.is_empty() {
        process_block(
            &mut block,
            &table,
            &register,
            &coord_codes,
            &mut out,
            &mut report,
        );
    }

    let joined = out.join("\n");
    if apply {
        if let Err(e) = fs::write(queue_path, &joined) {
            eprintln!("queue unwritable: {} ({})", queue_path, e);
            return ExitCode::from(1);
        }
    }

    report_write(&report, apply);
    ExitCode::SUCCESS
}

fn process_block(
    block: &mut Vec<String>,
    table: &HashMap<String, Verdict>,
    register: &Register,
    coord_codes: &HashMap<(i64, i64), Vec<String>>,
    out: &mut Vec<String>,
    report: &mut Report,
) {
    let url = block_url(block);
    if let Some(u) = url {
        report.blocks += 1;
        let key = fold_url(&u);
        match table.get(&key) {
            Some(Verdict::Name(name)) => {
                report.matched += 1;
                report.applied += apply_source(block, name);
            }
            Some(Verdict::Pending) => {
                report.matched += 1;
                report.pending += 1;
            }
            Some(Verdict::Riss) => {
                report.matched += 1;
                resolve_riss(block, &u, register, coord_codes, report);
            }
            None => {}
        }
    }
    for l in block.iter() {
        out.push(l.clone());
    }
    block.clear();
}

fn report_write(report: &Report, applied: bool) {
    let mode = if applied { "applied" } else { "dry-run" };
    println!("source_keyer {mode}: blocks={}", report.blocks);
    println!(
        "matched={} applied={} pending={}",
        report.matched, report.applied, report.pending
    );
    println!(
        "riss_resolved={} riss_carried={}",
        report.riss_resolved, report.riss_carried
    );
    for line in &report.riss_lines {
        println!("{line}");
    }
}
