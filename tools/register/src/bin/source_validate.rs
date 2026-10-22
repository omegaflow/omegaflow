use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::process::exit;

use omegaflow::archivar::{kernel_id_of, parse_source_blocks, validate_source_blocks};
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

fn directive_rank(head: &str) -> Option<u8> {
    match head {
        "url" => Some(1),
        "terms" | "origin" | "compiler" | "sha256" | "witness" | "record" | "harvest" | "live" => {
            Some(2)
        }
        "at" | "on" | "global" | "coverage" | "query" | "track" | "sky" => Some(3),
        "ttl" | "no-cadence" => Some(4),
        "format" | "header" | "post_body" | "body" | "force" | "range" | "band" | "footprint"
        | "norm" | "max_degree" | "gm" | "radius" | "tide" => Some(5),
        "station" | "stations" | "stations_path" | "stations_lon" | "stations_lat"
        | "stations_id" | "stations_filter" | "stations_flatten" | "fanout" | "fanout_center"
        | "fanout_delay" => Some(6),
        "map" | "cmap" | "rows" | "keplermap" | "geojson" | "quakeml" | "lat" | "lon" | "alt"
        | "epoch" | "ra" | "dec" | "dist" | "plx" | "pmra" | "pmdec" | "radvel" | "z"
        | "z_key" | "rv_scale" | "dist_scale" | "catalog_epoch" | "hapi" | "hapi_fill" | "field"
        | "quantity" | "first" | "last" | "lastrow" | "path" | "deep" | "objlast" | "regex"
        | "tau_key" | "ma" | "om" | "w" | "lat_key" | "lon_key" | "lat_sign" | "lon_sign"
        | "mag_type_key" | "epoch_scale" | "gate" | "prefix" | "bin" | "profile" | "fold"
        | "cgm_lat" | "cgm_source" | "flux_from_mag" | "abs_mag_from" => Some(7),
        "channel" | "coupling" | "constraint" => Some(8),
        "medium" => Some(9),
        _ => None,
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
    if matches!(unit, "measured" | "derived" | "pending") && idx >= 2 {
        return Some((parts[idx - 2], Some(parts[idx - 1])));
    }
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

struct BlockReport {
    findings: BTreeMap<String, BTreeSet<String>>,
    hard: usize,
}

fn new_block_text(block: &[&str]) -> Option<String> {
    let is_new = block.iter().any(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        parts.first() == Some(&"field") && line.contains("alt_offset=")
    });
    if !is_new {
        return None;
    }
    let mut out = String::new();
    for line in block {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let Some(head) = parts.first() else {
            continue;
        };
        if matches!(
            *head,
            "url" | "terms" | "origin" | "ttl" | "on" | "query" | "track" | "field"
        ) {
            out.push_str(line.trim());
            out.push('\n');
        }
    }
    Some(out)
}

fn source_model_directives(block: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for line in block {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let Some(head) = parts.first() else {
            continue;
        };
        if matches!(
            *head,
            "kernel"
                | "tau"
                | "tau_src"
                | "absorption"
                | "advection"
                | "medium"
                | "fold"
                | "gate"
                | "flux_from_mag"
                | "cgm_lat"
                | "cgm_source"
                | "geomag_lat"
                | "tide"
        ) {
            out.push((*head).to_string());
        }
    }
    out
}

fn validate_blocks(content: &str) -> BlockReport {
    let mut report = BlockReport {
        findings: BTreeMap::new(),
        hard: 0,
    };
    let push = |rule: &str, key: String, report: &mut BlockReport| {
        if report.findings.entry(rule.to_string()).or_default().insert(key) {
            report.hard += 1;
        }
    };
    match parse_source_blocks(content) {
        Err(reason) => push("source_block_parse", reason, &mut report),
        Ok(blocks) if blocks.is_empty() => {
            push("source_block_parse", "no source block found".to_string(), &mut report)
        }
        Ok(blocks) => {
            if let Err(errors) = validate_source_blocks(&blocks) {
                for error in errors {
                    push("source_block", error, &mut report);
                }
            }
        }
    }
    report
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut path = DEFAULT_PATH.to_string();
    let mut blocks_mode = false;
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
            "--blocks" => blocks_mode = true,
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

    if blocks_mode {
        let report = validate_blocks(&content);
        for (rule, keys) in &report.findings {
            for key in keys {
                println!("{rule}\t{key}");
            }
        }
        eprintln!(
            "source_validate: {} findings ({} hard, 0 soft)",
            report.findings.values().map(|v| v.len()).sum::<usize>(),
            report.hard
        );
        if ctx.fail && report.hard > 0 {
            exit(2);
        }
        return;
    }

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

        if let Some(source_text) = new_block_text(&block) {
            let report = validate_blocks(&source_text);
            for (rule, keys) in report.findings {
                for key in keys {
                    push(&rule, key, true, &mut findings, &mut hard, &mut soft);
                }
            }
            for head in source_model_directives(&block) {
                push(
                    "source_model_field",
                    format!("{tag}\t{name}\t{head}"),
                    true,
                    &mut findings,
                    &mut hard,
                    &mut soft,
                );
            }
            if field_of(&block, "format").and_then(|p| p.get(1).copied()) == Some("reference") {
                push(
                    "reference_not_source",
                    format!("{tag}\t{name}"),
                    true,
                    &mut findings,
                    &mut hard,
                    &mut soft,
                );
            }
            continue;
        }

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
        let mut last_rank = 0u8;

        for line_text in &block {
            let parts: Vec<&str> = line_text.split_whitespace().collect();
            let head = match parts.first() {
                Some(h) => *h,
                None => continue,
            };
            if head == "format" && parts.get(1) == Some(&"reference") {
                push(
                    "reference_not_source",
                    format!("{tag}\t{name}"),
                    true,
                    &mut findings,
                    &mut hard,
                    &mut soft,
                );
                continue;
            }
            match directive_rank(head) {
                Some(rank) => {
                    if rank < last_rank {
                        push(
                            "directive_order",
                            format!("{tag}\t{name}\t{head}"),
                            true,
                            &mut findings,
                            &mut hard,
                            &mut soft,
                        );
                    }
                    last_rank = rank;
                }
                None => {
                    push(
                        "directive_order_unknown",
                        format!("{tag}\t{name}\t{head}"),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                    continue;
                }
            }
            if matches!(
                head,
                "first" | "last" | "lastrow" | "path" | "deep" | "regex" | "objlast"
            ) {
                if !matches!(
                    parts.get(7).copied(),
                    Some("measured" | "derived" | "pending")
                ) {
                    push(
                        "field_tau_src_missing",
                        format!("{tag}\t{name}\t{}", parts.len()),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                }
                continue;
            }
            if head != "field" && head != "quantity" {
                continue;
            }
            let standard_field =
                head == "quantity" || (head == "field" && kernel_id_of(parts[3]).is_some());
            if standard_field
                && !matches!(
                    parts.get(7).copied(),
                    Some("measured" | "derived" | "pending")
                )
            {
                push(
                    "field_tau_src_missing",
                    format!("{tag}\t{name}\t{}", parts.len()),
                    true,
                    &mut findings,
                    &mut hard,
                    &mut soft,
                );
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
                if !parts.get(2).is_some_and(|p| p.starts_with("frame=")) {
                    push(
                        "anchor_frame",
                        format!("{tag}\t{name}\t-"),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                }
            }
            if head == "on" {
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
                let attr = |key: &str| {
                    parts[2..]
                        .iter()
                        .filter_map(|p| p.split_once('='))
                        .find(|(k, _)| *k == key)
                        .map(|(_, v)| v)
                };
                if attr("frame").is_none() {
                    push(
                        "anchor_frame",
                        format!("{tag}\t{name}\t-"),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                }
                let lat = attr("lat").and_then(parse_f64);
                let lon = attr("lon").and_then(parse_f64);
                let elev = attr("alt").and_then(|v| {
                    if v == "absent" || v == "pending" {
                        None
                    } else {
                        parse_f64(v)
                    }
                });
                if let Some(a) = lat {
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
                }
                if let Some(o) = lon {
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
                }
                if let Some(e) = elev {
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
                    if e == 0.0 && lat == Some(0.0) && lon == Some(0.0) {
                        push(
                            "elev_zero",
                            format!("{tag}\t{name}\tlat=0\tlon=0"),
                            false,
                            &mut findings,
                            &mut hard,
                            &mut soft,
                        );
                    }
                }
            }
            if matches!(
                head,
                "query" | "track" | "coverage" | "global" | "orbit" | "sky" | "absent"
            ) {
                anchor_seen = true;
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

#[cfg(test)]
mod tests {
    use super::*;

    const BLOCK: &str = concat!(
        "url https://example.org/ndbc/51003.txt\n",
        "terms PD-USGov https://example.org/terms\n",
        "ttl 60\n",
        "on earth frame=body lat=19.5 lon=-155.6 alt_ref=0 datum=sea_surface",
        " time=2026-10-22T00:00:00Z time_scale=UTC\n",
        "field WSPD wind_speed m/s source_column=4 alt_offset=4.0\n",
        "field ATMP air_temp C source_column=5 alt_offset=3.0 alt_min_offset=0.0 alt_max_offset=5.0\n",
    );

    #[test]
    fn a_source_block_with_fields_validates_without_findings() {
        let report = validate_blocks(BLOCK);
        assert_eq!(report.hard, 0, "{:?}", report.findings);
        assert!(report.findings.is_empty());
    }

    #[test]
    fn an_ambiguous_url_field_key_is_a_hard_source_block_finding() {
        let duplicated = BLOCK.replace("field ATMP air_temp", "field WSPD air_temp");
        let report = validate_blocks(&duplicated);
        assert!(
            report.findings.contains_key("source_block"),
            "{:?}",
            report.findings
        );
        assert!(report.hard >= 1);
    }

    #[test]
    fn a_field_without_alt_offset_is_a_hard_parse_finding() {
        let report = validate_blocks(&BLOCK.replace(" alt_offset=4.0", ""));
        assert!(
            report.findings.contains_key("source_block_parse"),
            "{:?}",
            report.findings
        );
        assert!(report.hard >= 1);
    }

    #[test]
    fn a_layer_with_min_not_below_max_is_a_hard_parse_finding() {
        let report = validate_blocks(&BLOCK.replace(
            "alt_min_offset=0.0 alt_max_offset=5.0",
            "alt_min_offset=5.0 alt_max_offset=5.0",
        ));
        assert!(
            report.findings.contains_key("source_block_parse"),
            "{:?}",
            report.findings
        );
        assert!(report.hard >= 1);
    }

    #[test]
    fn new_block_metadata_is_ignored_but_model_directives_are_hard() {
        let with_metadata = format!(
            "{}format csv\nsha256 {}\n",
            BLOCK,
            "a".repeat(64)
        );
        let block: Vec<&str> = with_metadata.lines().collect();
        let text = new_block_text(&block).expect("new block");
        let report = validate_blocks(&text);
        assert_eq!(report.hard, 0, "{:?}", report.findings);
        assert!(!report.findings.contains_key("source_model_field"));

        let mut content = String::from(BLOCK);
        content.push_str("kernel 1\n");
        let block: Vec<&str> = content.lines().collect();
        let text = new_block_text(&block).expect("new block");
        assert!(!text.contains("kernel"), "metadata is not passed to the parser");
        assert_eq!(source_model_directives(&block), vec!["kernel".to_string()]);
    }

    #[test]
    fn an_empty_register_is_a_hard_finding() {
        let report = validate_blocks("");
        assert!(report.findings.contains_key("source_block_parse"));
        assert_eq!(report.hard, 1);
    }
}
