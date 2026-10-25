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
            Some("field") | Some("quantity") | Some("last")
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

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

use omegaflow::archivar::kernel_id_of;
use omegaflow::mathematikerin::force::{force_id_of, quantity_kind_of};

const DEFAULT_PATH: &str = "phi/sources.φ";
const DEFAULT_SPECS: &str = "tools/register/data/instruments.φ";

struct Ctx {
    fail: bool,
    warnings: bool,
    probe_max_days: i64,
}

struct Spec {
    host: String,
    applies: Vec<String>,
    instrument: String,
    units: BTreeSet<String>,
    source_units: BTreeSet<String>,
    fields: BTreeSet<String>,
    samplings: BTreeSet<u64>,
    intervals: BTreeSet<u64>,
    api_cite: bool,
    instrument_cite: bool,
    host_cite: bool,
    ttl: Option<u64>,
    probes: Vec<Probe>,
}

struct Probe {
    date: String,
    status: String,
    url: String,
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn parse_date(s: &str) -> Option<i64> {
    let mut it = s.split('-');
    let y: i64 = it.next()?.parse().ok()?;
    let m: i64 = it.next()?.parse().ok()?;
    let d: i64 = it.next()?.parse().ok()?;
    if it.next().is_some() || !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    Some(days_from_civil(y, m, d))
}

fn now_secs() -> Option<i64> {
    Some(SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64)
}

fn parse_probe_ts(s: &str) -> Option<i64> {
    if let Some((date, time)) = s.split_once('T') {
        let day = parse_date(date)?;
        let mut it = time.trim_end_matches('Z').split(':');
        let h: i64 = it.next()?.parse().ok()?;
        let m: i64 = it.next()?.parse().ok()?;
        let sec: i64 = it.next()?.parse().ok()?;
        if h > 23 || m > 59 || sec > 60 {
            return None;
        }
        Some(day * 86400 + h * 3600 + m * 60 + sec)
    } else {
        Some(parse_date(s)? * 86400 + 86399)
    }
}

fn probe_admits(status: &str) -> bool {
    status
        .parse::<u16>()
        .ok()
        .is_some_and(|code| (200..300).contains(&code))
}

fn origin_stem(url: &str) -> &str {
    let cut = url.find('?').into_iter().chain(url.find('{')).min().unwrap_or(url.len());
    &url[..cut]
}

fn host_of(url: &str) -> Option<String> {
    let rest = url.split_once("://")?.1;
    let host = rest.split(['/', '?']).next()?;
    if host.is_empty() {
        return None;
    }
    let host = host.strip_prefix("www.").unwrap_or(host);
    if host == "github.com"
        && let Some(tag) = rest
            .strip_prefix("github.com/omegaflow/sources/releases/download/")
            .and_then(|p| p.split('/').next())
        && !tag.is_empty()
    {
        return Some(tag.to_string());
    }
    Some(host.to_string())
}

fn load_specs(path: &str) -> Vec<Spec> {
    let mut out: Vec<Spec> = Vec::new();
    let Ok(content) = fs::read_to_string(path) else {
        return out;
    };
    for line in content.lines() {
        let p: Vec<&str> = line.split_whitespace().collect();
        let value = p.get(1).map(|s| s.to_string());
        match p.first().copied() {
            Some("host") => {
                if let Some(token) = value {
                    out.push(Spec {
                        host: token.clone(),
                        applies: Vec::new(),
                        instrument: String::new(),
                        units: BTreeSet::new(),
                        source_units: BTreeSet::new(),
                        fields: BTreeSet::new(),
                        samplings: BTreeSet::new(),
                        intervals: BTreeSet::new(),
                        api_cite: false,
                        instrument_cite: false,
                        host_cite: false,
                        ttl: None,
                        probes: Vec::new(),
                    });
                }
            }
            Some(key) => {
                let Some(entry) = out.last_mut() else {
                    continue;
                };
                match key {
                    "instrument" => {
                        if let Some(v) = value {
                            entry.instrument = v;
                        }
                    }
                    "fields" => {
                        for f in &p[1..] {
                            entry.fields.insert((*f).to_string());
                        }
                    }
                    "unit" => {
                        for raw in &p[1..] {
                            for u in raw.split(';').filter(|s| !s.is_empty()) {
                                if force_id_of(u).is_some()
                                    || quantity_kind_of(u).is_some()
                                    || kernel_id_of(u).is_some()
                                {
                                    entry.units.insert(u.to_string());
                                } else {
                                    entry.source_units.insert(u.to_string());
                                }
                            }
                        }
                    }
                    "source_unit" => {
                        for raw in &p[1..] {
                            for u in raw.split(';').filter(|s| !s.is_empty()) {
                                entry.source_units.insert(u.to_string());
                            }
                        }
                    }
                    "sampling" => {
                        for c in &p[1..] {
                            if let Some(v) = parse_f64(c) {
                                entry.samplings.insert(v as u64);
                            }
                        }
                    }
                    "interval" => {
                        for c in &p[1..] {
                            if let Some(v) = parse_f64(c) {
                                entry.intervals.insert(v as u64);
                            }
                        }
                    }
                    "applies" => {
                        if let Some(v) = value {
                            entry.applies.push(v);
                        }
                    }
                    "api_cite" => entry.api_cite = value.is_some(),
                    "instrument_cite" => entry.instrument_cite = value.is_some(),
                    "host_cite" => entry.host_cite = value.is_some(),
                    "ttl" => entry.ttl = value.as_deref().and_then(parse_f64).map(|v| v as u64),
                    "probe" => {
                        let (Some(date), Some(status), Some(url)) =
                            (value.clone(), p.get(2), p.get(3))
                        else {
                            continue;
                        };
                        entry.probes.push(Probe {
                            date,
                            status: (*status).to_string(),
                            url: (*url).to_string(),
                        });
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }
    out
}
fn url_id(url: &str) -> Option<String> {
    let rest = url.split_once("id=")?.1;
    let end = rest.find('&').unwrap_or(rest.len());
    let id = &rest[..end];
    if id.is_empty() {
        None
    } else {
        Some(id.replace("%40", "@").replace("%2F", "/").replace("%2f", "/"))
    }
}

fn applies_len(s: &Spec, url: &str) -> Option<usize> {
    s.applies
        .iter()
        .filter(|a| {
            url.starts_with(a.as_str())
                || url.split(';').any(|u| u.trim().starts_with(a.as_str()))
        })
        .map(|a| a.len())
        .max()
}

fn spec_for_inst<'a>(
    specs: &'a [Spec],
    host: &str,
    url: &str,
    inst: &str,
    keys: &[String],
) -> Result<&'a Spec, Vec<String>> {
    let keyed: Vec<&Spec> = specs
        .iter()
        .filter(|s| {
            !s.fields.is_empty() && keys.iter().any(|k| s.fields.contains(k))
        })
        .collect();
    let cands: Vec<&Spec> = if keyed.is_empty() {
        specs
            .iter()
            .filter(|s| s.host == host && (s.applies.is_empty() || applies_len(s, url).is_some()))
            .collect()
    } else {
        keyed
    };
    if cands.is_empty() {
        return Err(Vec::new());
    }
    let exact: Vec<&Spec> = cands
        .iter()
        .copied()
        .filter(|s| s.instrument.eq_ignore_ascii_case(inst))
        .collect();
    if exact.is_empty() {
        let mut names: Vec<String> = cands.iter().map(|s| s.instrument.clone()).collect();
        names.sort();
        names.dedup();
        return Err(names);
    }
    Ok(*exact
        .iter()
        .max_by_key(|s| applies_len(s, url))
        .expect("exact ist nicht leer"))
}

fn list_open_blocks(content: &str, specs: &[Spec]) {
    println!("line\tkind\thost\tinstrument\torigin\tfield\tunit");
    for (start, block) in blocks(content) {
        if !has_observables(&block) {
            continue;
        }
        let origin_rest = field_of(&block, "origin").map(|p| p[1..].join(" "));
        let (host, origin) = match origin_rest.as_deref() {
            Some(s) => (s.split_whitespace().next().and_then(host_of), s),
            None => (None, "-"),
        };
        let host = match &host {
            Some(h) => h.as_str(),
            None => "-",
        };
        let mut instruments: Vec<String> = Vec::new();
        let mut fname = String::new();
        let mut unit = String::new();
        for line in &block {
            let p: Vec<&str> = line.split_whitespace().collect();
            if p.first() == Some(&"instrument")
                && let Some(t) = p.get(1)
                && !instruments.iter().any(|x| x.eq_ignore_ascii_case(t))
            {
                instruments.push((*t).to_string());
            }
            if fname.is_empty() && matches!(p.first(), Some(&"field") | Some(&"quantity") | Some(&"last")) {
                fname = field_name(&p);
                if let Some((u, _)) = tail_unit_cadence(&p) {
                    unit = u.to_string();
                }
            }
        }
        if instruments.is_empty() {
            println!("{start}\tno_instrument\t{host}\t-\t{origin}\t{fname}\t{unit}");
            continue;
        }
        for inst in &instruments {
            match spec_for_inst(specs, &host, &origin, inst, &[]) {
                Ok(_) => {}
                Err(names) if names.is_empty() => {
                    println!("{start}\tno_spec\t{host}\t{inst}\t{origin}\t{fname}\t{unit}");
                }
                Err(names) => {
                    println!(
                        "{start}\tmismatch\t{host}\t{inst}\t{origin}\t{}\t{fname}",
                        names.join(",")
                    );
                }
            }
        }
    }
}

fn list_without_fields(content: &str) {
    println!("line\tformat\torigin\tinstrument");
    for (start, block) in blocks(content) {
        let has_fields = block.iter().any(|l| {
            let p: Vec<&str> = l.split_whitespace().collect();
            matches!(p.first(), Some(&"field") | Some(&"quantity") | Some(&"last"))
        });
        if has_fields {
            continue;
        }
        let get = |name: &str| {
            block.iter().find_map(|l| {
                let p: Vec<&str> = l.split_whitespace().collect();
                (p.first() == Some(&name)).then(|| p[1..].join(" "))
            })
        };
        let fmt = get("format");
        let origin = get("origin");
        let inst = get("instrument");
        println!(
            "{start}\t{}\t{}\t{}",
            fmt.as_deref().unwrap_or("-"),
            origin.as_deref().unwrap_or("-"),
            inst.as_deref().unwrap_or("-")
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut path = DEFAULT_PATH.to_string();
    let mut specs_path = DEFAULT_SPECS.to_string();
    let mut ctx = Ctx {
        fail: false,
        warnings: false,
        probe_max_days: 0,
    };
    let mut list_open = false;
    let mut list_nofield = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--list-open" => list_open = true,
            "--list-nofield" => list_nofield = true,
            "--file" => {
                i += 1;
                match args.get(i) {
                    Some(p) => path = p.clone(),
                    None => exit(2),
                }
            }
            "--specs" => {
                i += 1;
                match args.get(i) {
                    Some(p) => specs_path = p.clone(),
                    None => exit(2),
                }
            }
            "--fail" => ctx.fail = true,
            "--warnings" => ctx.warnings = true,
            "--probe-max-days" => {
                i += 1;
                match args.get(i).and_then(|v| v.parse::<i64>().ok()) {
                    Some(v) => ctx.probe_max_days = v,
                    None => exit(2),
                }
            }
            _ => {}
        }
        i += 1;
    }
    let specs = load_specs(&specs_path);
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("source_validate: {path} is not readable");
            exit(2);
        }
    };

    if list_open {
        list_open_blocks(&content, &specs);
        return;
    }
    if list_nofield {
        list_without_fields(&content);
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

        let field_keys: Vec<String> = block
            .iter()
            .filter_map(|l| {
                let p: Vec<&str> = l.split_whitespace().collect();
                matches!(p.first(), Some(&"field") | Some(&"quantity") | Some(&"last"))
                    .then(|| field_name(&p))
            })
            .collect();
        let no_cadence = field_of(&block, "no-sampling").is_some();
        let instruments: Vec<String> = {
            let mut v: Vec<String> = Vec::new();
            for line in &block {
                let p: Vec<&str> = line.split_whitespace().collect();
                if p.first() == Some(&"instrument")
                    && let Some(tok) = p.get(1)
                    && !v.iter().any(|x| x.eq_ignore_ascii_case(tok))
                {
                    v.push((*tok).to_string());
                }
            }
            v
        };
        let origin_rest: Option<String> = field_of(&block, "origin").map(|p| p[1..].join(" "));
        let origin_url = origin_rest.as_deref().and_then(|s| s.split_whitespace().next())
            .or_else(|| {
                let url = field_of(&block, "url").and_then(|p| p.get(1).copied())?;
                (!url.starts_with("https://github.com/omegaflow/sources/releases/download/"))
                    .then_some(url)
            });
        let block_host = origin_url.and_then(host_of);
        let mut names: BTreeSet<String> = BTreeSet::new();

        if has_observables(&block) && instruments.is_empty() {
            push(
                "instrument_missing",
                format!("{tag}\t{name}"),
                true,
                &mut findings,
                &mut hard,
                &mut soft,
            );
        }

        if has_observables(&block) {
            let stem = origin_url.map(origin_stem);
            let host_rows: Vec<&Spec> = match block_host.as_ref() {
                Some(h) => specs.iter().filter(|s| s.host == *h).collect(),
                None => Vec::new(),
            };
            match stem {
                None => {}
                Some(_) if host_rows.is_empty() => push(
                    "probe_missing",
                    format!("{tag}\t{name}\t{}", block_host.as_deref().unwrap_or("-")),
                    true,
                    &mut findings,
                    &mut hard,
                    &mut soft,
                ),
                Some(stem) => {
                    let mut matching: Vec<&Probe> = host_rows
                        .iter()
                        .flat_map(|s| s.probes.iter())
                        .filter(|p| {
                            let same_arm = match (url_id(&p.url), origin_url.and_then(url_id)) {
                                (Some(a), Some(b)) => a == b,
                                _ => true,
                            };
                            same_arm && (p.url.starts_with(stem) || stem.starts_with(&p.url))
                        })
                        .collect();
                    if matching.is_empty() {
                        let any = host_rows.iter().any(|s| !s.probes.is_empty());
                        push(
                            if any {
                                "probe_path_mismatch"
                            } else {
                                "probe_missing"
                            },
                            format!("{tag}\t{name}\t{stem}"),
                            true,
                            &mut findings,
                            &mut hard,
                            &mut soft,
                        );
                    } else if !matching.iter().any(|p| probe_admits(&p.status)) {
                        let p = matching[0];
                        push(
                            "probe_unanswered",
                            format!("{tag}\t{name}\t{}\t{}", p.status, p.url),
                            true,
                            &mut findings,
                            &mut hard,
                            &mut soft,
                        );
                    }
                    matching.sort_by_key(|p| std::cmp::Reverse(parse_probe_ts(&p.date)));
                    if let Some(probe) = matching.first() {
                        match (parse_probe_ts(&probe.date), now_secs()) {
                            (Some(ts), Some(now)) => {
                                if ts > now + 86400 {
                                    push(
                                        "probe_date",
                                        format!("{tag}\t{name}\t{}\tfuture", probe.date),
                                        true,
                                        &mut findings,
                                        &mut hard,
                                        &mut soft,
                                    );
                                } else if ctx.probe_max_days > 0 {
                                    let limit = ctx.probe_max_days * 86400;
                                    if now - ts > limit {
                                        push(
                                            "probe_stale",
                                            format!(
                                                "{tag}\t{name}\t{}\t{}s>{limit}s",
                                                probe.date,
                                                now - ts
                                            ),
                                            true,
                                            &mut findings,
                                            &mut hard,
                                            &mut soft,
                                        );
                                    }
                                }
                            }
                            _ => push(
                                "probe_date",
                                format!("{tag}\t{name}\t{}", probe.date),
                                true,
                                &mut findings,
                                &mut hard,
                                &mut soft,
                            ),
                        }
                    }
                }
            }
        }

        let mut matched: Vec<&Spec> = Vec::new();
        if has_observables(&block)
            && let (Some(h), Some(u)) = (block_host.as_ref(), origin_rest.as_deref().or(origin_url))
        {
            for inst in &instruments {
                match spec_for_inst(&specs, h, u, inst, &field_keys) {
                    Ok(entry) => matched.push(entry),
                    Err(names) if names.is_empty() => push(
                        "spec_missing_for_host",
                        format!("{tag}\t{name}\t{inst}"),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    ),
                    Err(names) => push(
                        "instrument_mismatch",
                        format!("{tag}\t{name}\t{inst}\t{}", names.join(",")),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    ),
                }
            }
            for entry in &matched {
                if !entry.api_cite || !entry.instrument_cite {
                    push(
                        "spec_incomplete",
                        format!("{tag}\t{name}\t{}", entry.instrument),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                }
                if !entry.host_cite {
                    push(
                        "host_doc_missing",
                        format!("{tag}\t{name}\t{}", entry.instrument),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                }
            }
            if let Some(block_ttl) = field_of(&block, "ttl")
                .and_then(|p| p.get(1).copied())
                .and_then(parse_f64)
            {
                for entry in &matched {
                    if let Some(doc) = entry.ttl
                        && block_ttl as u64 != doc
                    {
                        push(
                            "ttl_mismatch",
                            format!(
                                "{tag}\t{name}\t{}\t{}!={doc}",
                                entry.instrument, block_ttl as u64
                            ),
                            true,
                            &mut findings,
                            &mut hard,
                            &mut soft,
                        );
                    }
                }
            }
        }

        for line_text in &block {
            let parts: Vec<&str> = line_text.split_whitespace().collect();
            let head = match parts.first() {
                Some(h) => *h,
                None => continue,
            };
            if head != "field" && head != "quantity" && head != "last" {
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
            if !matched.is_empty() {
                if !matched.iter().any(|e| e.units.is_empty() || e.units.contains(unit)) {
                    push(
                        "combo_unit",
                        format!("{tag}\t{name}\t{}\t{unit}", matched.iter().map(|e| e.instrument.as_str()).collect::<Vec<_>>().join(",")),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                }
                if !no_cadence
                    && let Some(c) = cadence.and_then(parse_f64)
                    && matched.iter().any(|e| !e.intervals.is_empty())
                    && !matched.iter().any(|e| e.intervals.contains(&(c as u64)))
                {
                    push(
                        "interval_mismatch",
                        format!("{tag}\t{name}\t{}\t{c}", matched.iter().map(|e| e.instrument.as_str()).collect::<Vec<_>>().join(",")),
                        true,
                        &mut findings,
                        &mut hard,
                        &mut soft,
                    );
                }
            }
            if no_cadence {
                continue;
            }
            match cadence.and_then(parse_f64) {
                Some(c) => {
                    if c <= 0.0 {
                        push(
                            "sampling_bad",
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
                        "sampling_bad",
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
                            format!("{tag}\t{name}\tnon-numeric"),
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
