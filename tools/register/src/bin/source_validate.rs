use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::process::exit;

use omegaflow::archivar::kernel_id_of;
use omegaflow::mathematikerin::force::{force_id_of, quantity_kind_of};

const DEFAULT_PATH: &str = "phi/sources.φ";

struct Ctx {
    fail: bool,
    warnings: bool,
}

fn parse_f64(s: &str) -> Option<f64> {
    match s.parse::<f64>() {
        Ok(v) => {
            if v.is_finite() {
                Some(v)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn parse_u64(s: &str) -> Option<u64> {
    match s.parse::<u64>() {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn numeric_run_start(parts: &[&str]) -> Option<usize> {
    let mut end = parts.len();
    while end > 0 && parse_f64(parts[end - 1]).is_none() {
        end -= 1;
    }
    if end == 0 {
        return None;
    }
    let mut idx = end - 1;
    while parse_f64(parts[idx]).is_some() {
        if idx == 0 {
            return None;
        }
        idx -= 1;
    }
    Some(idx)
}

fn tail_unit_cadence<'a>(parts: &[&'a str]) -> Option<(&'a str, Option<&'a str>)> {
    let idx = numeric_run_start(parts)?;
    let unit = parts[idx];
    let next = parts.get(idx + 1).copied();
    let numeric_unit = next == Some("1")
        && (force_id_of(unit).is_some()
            || quantity_kind_of(unit).is_some()
            || kernel_id_of(unit).is_some());
    if numeric_unit {
        Some((parts[idx + 1], parts.get(idx + 2).copied()))
    } else {
        Some((unit, next))
    }
}

fn field_name(parts: &[&str]) -> String {
    let mut idx = 1usize;
    let quoted_column = parts.get(idx).is_some_and(|t| t.starts_with('"'));
    if quoted_column {
        while idx < parts.len() {
            let token = parts[idx];
            idx += 1;
            if token.len() > 1 && token.ends_with('"') {
                break;
            }
        }
    }
    if idx >= parts.len() {
        return String::new();
    }
    let next_quoted = parts.get(idx + 1).is_some_and(|t| t.starts_with('"'));
    if !quoted_column && !next_quoted && idx + 1 < parts.len() {
        parts[idx + 1].to_string()
    } else {
        parts[idx].to_string()
    }
}

fn has_observables(block: &[&str]) -> bool {
    block.iter().any(|line| {
        matches!(
            line.split_whitespace().next(),
            Some("field") | Some("quantity")
        )
    })
}

fn blocks(content: &str) -> Vec<(usize, Vec<&str>)> {
    let mut out: Vec<(usize, Vec<&str>)> = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    let mut start = 0usize;
    for (idx, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            if !cur.is_empty() {
                out.push((start, std::mem::take(&mut cur)));
            }
        } else {
            if cur.is_empty() {
                start = idx + 1;
            }
            cur.push(line);
        }
    }
    if !cur.is_empty() {
        out.push((start, cur));
    }
    out
}

fn field_of<'a>(block: &[&'a str], name: &str) -> Option<Vec<&'a str>> {
    for line in block {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.first() == Some(&name) {
            return Some(parts);
        }
    }
    None
}

fn label(block: &[&str], line: usize) -> (String, String) {
    let fmt = match field_of(block, "format") {
        Some(p) => match p.get(1) {
            Some(v) => (*v).to_string(),
            None => String::new(),
        },
        None => String::new(),
    };
    let url = match field_of(block, "url") {
        Some(p) => match p.get(1) {
            Some(v) => (*v).to_string(),
            None => String::new(),
        },
        None => String::new(),
    };
    let name = if fmt.is_empty() {
        let head: String = url.chars().rev().take_while(|c| *c != '/').collect();
        head.chars().rev().collect()
    } else {
        fmt
    };
    (name, line.to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut path = DEFAULT_PATH.to_string();
    let mut ctx = Ctx {
        fail: false,
        warnings: false,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                match args.get(i) {
                    Some(p) => path = p.clone(),
                    None => exit(2),
                }
            }
            "--fail" => ctx.fail = true,
            "--warnings" => ctx.warnings = true,
            _ => {}
        }
        i += 1;
    }
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("source_validate: {path} is not readable");
            exit(2);
        }
    };

    let mut findings: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut hard = 0usize;
    let mut soft = 0usize;

    let push = |rule: &str,
                key: String,
                hard_rule: bool,
                findings: &mut BTreeMap<String, BTreeSet<String>>,
                hard: &mut usize,
                soft: &mut usize| {
        let inserted = findings.entry(rule.to_string()).or_default().insert(key);
        if !inserted {
            return;
        }
        if hard_rule {
            *hard += 1;
        } else {
            *soft += 1;
        }
    };

    for (line, block) in blocks(&content) {
        let (name, tag) = label(&block, line);

        if field_of(&block, "url").is_some() && field_of(&block, "terms").is_none() {
            push(
                "missing_terms",
                format!("{tag}\t{name}"),
                true,
                &mut findings,
                &mut hard,
                &mut soft,
            );
        }

        if let Some(p) = field_of(&block, "compiler") {
            if let Some(rel) = p.get(1) {
                if rel.contains('/') && !std::path::Path::new(rel).exists() {
                    push(
                        "compiler_missing",
                        format!("{tag}\t{name}\t{rel}"),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                }
            }
        }

        if let Some(p) = field_of(&block, "sha256") {
            let ok = match p.get(1) {
                Some(v) => v.len() == 64 && v.chars().all(|c| c.is_ascii_hexdigit()),
                None => false,
            };
            if !ok {
                push(
                    "sha_bad",
                    format!("{tag}\t{name}"),
                    true,
                    &mut findings,
                    &mut hard,
                    &mut soft,
                );
            }
        }

        let no_cadence = field_of(&block, "no-cadence").is_some();
        let mut names: BTreeSet<String> = BTreeSet::new();

        for line_text in &block {
            let parts: Vec<&str> = line_text.split_whitespace().collect();
            let head = match parts.first() {
                Some(h) => *h,
                None => continue,
            };
            if head != "field" && head != "quantity" {
                continue;
            }
            let fname = field_name(&parts);
            if !names.insert(fname.clone()) {
                push(
                    "dup_field",
                    format!("{tag}\t{name}\t{fname}"),
                    true,
                    &mut findings,
                    &mut hard,
                    &mut soft,
                );
            }
            let (unit, cadence) = match tail_unit_cadence(&parts) {
                Some(v) => v,
                None => {
                    push(
                        "field_unparsed",
                        format!("{tag}\t{name}\t{}", parts.len()),
                        false,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                    continue;
                }
            };
            let unit_num = parse_f64(unit).is_some();
            if unit.is_empty() || (unit_num && unit != "1") {
                push(
                    "unit_bad",
                    format!("{tag}\t{name}\t{fname}\t{unit}"),
                    true,
                    &mut findings,
                    &mut hard,
                    &mut soft,
                );
            }
            if no_cadence {
                continue;
            }
            match cadence.and_then(parse_f64) {
                Some(c) => {
                    if c <= 0.0 {
                        push(
                            "cadence_bad",
                            format!("{tag}\t{name}\t{fname}\t0"),
                            true,
                            &mut findings,
                            &mut hard,
                            &mut soft,
                        );
                    }
                }
                None => {
                    let raw = match cadence {
                        Some(v) => v,
                        None => "",
                    };
                    push(
                        "cadence_bad",
                        format!("{tag}\t{name}\t{fname}\t{raw}"),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                }
            }
        }

        let mut anchor_seen = false;
        for line_text in &block {
            let parts: Vec<&str> = line_text.split_whitespace().collect();
            let head = match parts.first() {
                Some(h) => *h,
                None => continue,
            };
            if head == "at" {
                anchor_seen = true;
                let body = match parts.get(1) {
                    Some(b) => *b,
                    None => {
                        push(
                            "anchor_body",
                            format!("{tag}\t{name}\t-"),
                            true,
                            &mut findings,
                            &mut hard,
                            &mut soft,
                        );
                        continue;
                    }
                };
                let ok = !body.is_empty()
                    && body
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
                if !ok {
                    push(
                        "anchor_body",
                        format!("{tag}\t{name}\t{body}"),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                }
                if let Some(extra) = parts.get(2) {
                    if parse_f64(extra).is_none() {
                        push(
                            "anchor_body",
                            format!("{tag}\t{name}\t{extra}"),
                            true,
                            &mut findings,
                            &mut hard,
                            &mut soft,
                        );
                    }
                }
            }
            if head == "on" {
                anchor_seen = true;
                if parts.len() != 5 {
                    push(
                        "anchor_arity",
                        format!("{tag}\t{name}\t{}", parts.len()),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                    continue;
                }
                let lat = parse_f64(parts[2]);
                let lon = parse_f64(parts[3]);
                let elev = parse_f64(parts[4]);
                match (lat, lon, elev) {
                    (Some(a), Some(o), Some(e)) => {
                        if a < -90.0 || a > 90.0 {
                            push(
                                "anchor_range",
                                format!("{tag}\t{name}\tlat={a}"),
                                true,
                                &mut findings,
                                &mut hard,
                                &mut soft,
                            );
                        }
                        if o < -180.0 || o > 360.0 {
                            push(
                                "anchor_range",
                                format!("{tag}\t{name}\tlon={o}"),
                                true,
                                &mut findings,
                                &mut hard,
                                &mut soft,
                            );
                        }
                        if e < -500.0 || e > 1.0e9 {
                            push(
                                "anchor_range",
                                format!("{tag}\t{name}\telev={e}"),
                                true,
                                &mut findings,
                                &mut hard,
                                &mut soft,
                            );
                        }
                        if e == 0.0 && a == 0.0 && o == 0.0 {
                            push(
                                "elev_zero",
                                format!("{tag}\t{name}\tlat={a}\tlon={o}"),
                                false,
                                &mut findings,
                                &mut hard,
                                &mut soft,
                            );
                        }
                    }
                    _ => {
                        push(
                            "anchor_range",
                            format!("{tag}\t{name}\tnicht-numerisch"),
                            true,
                            &mut findings,
                            &mut hard,
                            &mut soft,
                        );
                    }
                }
            }
            if head == "gate" {
                if parts.len() != 4 {
                    push(
                        "gate_arity",
                        format!("{tag}\t{name}\t{}", parts.len()),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                    continue;
                }
                match (parse_f64(parts[2]), parse_f64(parts[3])) {
                    (Some(lo), Some(hi)) => {
                        if lo > hi {
                            push(
                                "gate_inverted",
                                format!("{tag}\t{name}\t{}={lo}>{hi}", parts[1]),
                                true,
                                &mut findings,
                                &mut hard,
                                &mut soft,
                            );
                        }
                    }
                    _ => {
                        push(
                            "gate_inverted",
                            format!("{tag}\t{name}\t{}", parts[1]),
                            true,
                            &mut findings,
                            &mut hard,
                            &mut soft,
                        );
                    }
                }
            }
        }

        if let Some(orig) = field_of(&block, "origin") {
            if let Some(u) = orig.get(1) {
                if let Some(pos) = u.find("parameters=") {
                    let rest = &u[pos + 11..];
                    let end = match rest.find('&') {
                        Some(e) => e,
                        None => rest.len(),
                    };
                    let list = &rest[..end];
                    let mut hapi_tokens: Vec<&str> = Vec::new();
                    if let Some(h) = field_of(&block, "hapi") {
                        for t in h.iter().skip(1) {
                            hapi_tokens.push(t);
                        }
                    }
                    for param in list.split(',') {
                        let ptrim = param.trim();
                        if ptrim.is_empty() {
                            continue;
                        }
                        let direct = format!("{ptrim}=");
                        let dotted = format!("{ptrim}.");
                        let mut seen = false;
                        for t in &hapi_tokens {
                            if t.starts_with(&direct) || t.starts_with(&dotted) {
                                seen = true;
                            }
                        }
                        if !seen {
                            push(
                                "hapi_param_unmapped",
                                format!("{tag}\t{name}\t{ptrim}"),
                                true,
                                &mut findings,
                                &mut hard,
                                &mut soft,
                            );
                        }
                    }
                }
            }
        }

        if !anchor_seen && field_of(&block, "compiler").is_some() && has_observables(&block) {
            push(
                "anchor_missing",
                format!("{tag}\t{name}"),
                false,
                &mut findings,
                &mut hard,
                &mut soft,
            );
        }
    }

    for (rule, keys) in &findings {
        if !ctx.warnings {
            let soft_rule = rule == "anchor_missing";
            if soft_rule {
                continue;
            }
        }
        for key in keys {
            println!("{rule}\t{key}");
        }
    }

    eprintln!(
        "source_validate: {} findings ({} hard, {} soft)",
        findings.values().map(|v| v.len()).sum::<usize>(),
        hard,
        soft
    );
    if ctx.fail && hard > 0 {
        exit(2);
    }
}
