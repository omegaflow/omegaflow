use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

fn effective_lines(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for raw in text.lines() {
        let cols: Vec<&str> = raw.split('\t').collect();
        let first = match cols.get(1) {
            Some(correction) if !correction.trim().is_empty() => correction.trim(),
            _ => cols.first().copied().unwrap_or(""),
        };
        if first.starts_with('#') || first.trim().is_empty() || !first.contains(' ') {
            continue;
        }
        out.push(first.to_string());
    }
    out
}

fn blocks_of(lines: &[String]) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for line in lines {
        if line.starts_with("url ") && !current.is_empty() {
            out.push(std::mem::take(&mut current));
        }
        current.push(line.clone());
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

fn field_tokens(field: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    for ch in field.chars() {
        match ch {
            '"' => {
                quoted = !quoted;
                current.push(ch);
            }
            c if c.is_whitespace() && !quoted => {
                if !current.is_empty() {
                    out.push(std::mem::take(&mut current));
                }
            }
            c => current.push(c),
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

fn value_of(block: &[String], key: &str) -> Option<String> {
    let prefix = format!("{key} ");
    block
        .iter()
        .find(|line| line.starts_with(&prefix))
        .map(|line| line[prefix.len()..].trim().to_string())
}

fn stage_report(root: &str, show: usize) {
    let sources_path = format!("{root}/phi/sources.tsv");
    let model_path = format!("{root}/phi/field_model.tsv");
    let instruments_path = format!("{root}/phi/instruments.φ");
    let sources_text = match fs::read_to_string(&sources_path) {
        Ok(t) => t,
        Err(e) => {
            println!("stage-check: {sources_path} unreadable: {e}");
            return;
        }
    };
    let model_text = match fs::read_to_string(&model_path) {
        Ok(t) => t,
        Err(_) => {
            let fallback = match std::env::var_os("HOME") {
                Some(home) => PathBuf::from(home).join("projects/omegaflow/phi/field_model.tsv"),
                None => PathBuf::from("phi/field_model.tsv"),
            };
            match fs::read_to_string(&fallback) {
                Ok(t) => t,
                Err(e) => {
                    println!("stage-check: {} unreadable: {e}", fallback.display());
                    return;
                }
            }
        }
    };
    let instruments_text = match fs::read_to_string(&instruments_path) {
        Ok(t) => t,
        Err(e) => {
            println!("stage-check: {instruments_path} unreadable: {e}");
            return;
        }
    };
    let model_keys: BTreeSet<&str> = model_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| line.split('\t').next())
        .collect();
    let instrument_names: BTreeSet<&str> = instruments_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("instrument "))
        .map(|rest| rest.trim())
        .collect();

    let lines = effective_lines(&sources_text);
    let blocks = blocks_of(&lines);
    let stages = [
        "instrument",
        "field key",
        "unit",
        "interval",
        "anchor",
        "terms",
        "reader",
        "sha256",
        "url",
    ];
    let mut first_failure: BTreeMap<&str, usize> = BTreeMap::new();
    let mut examples: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    let mut passed = 0usize;
    for block in &blocks {
        let mut failing: Option<&str> = None;
        let instrument = match value_of(block, "instrument") {
            Some(v) => v,
            None => String::new(),
        };
        if !instrument_names.contains(instrument.as_str()) {
            failing = Some("instrument");
        }
        let field = match value_of(block, "field") {
            Some(v) => v,
            None => String::new(),
        };
        let tokens = field_tokens(&field);
        let key = match tokens.first() {
            Some(t) => t.trim_matches('"').to_string(),
            None => String::new(),
        };
        if failing.is_none() && !model_keys.contains(key.as_str()) {
            failing = Some("field key");
        }
        let (unit, interval) = if tokens.len() >= 6 {
            (tokens.get(4), tokens.get(5))
        } else {
            (tokens.get(2), tokens.get(3))
        };
        if failing.is_none() && unit.map(|u| u.trim().is_empty()).unwrap_or(true) {
            failing = Some("unit");
        }
        if failing.is_none() && interval.and_then(|v| v.parse::<f64>().ok()).is_none() {
            failing = Some("interval");
        }
        let has_anchor = block.iter().any(|line| {
            [
                "on ",
                "at ",
                "query ",
                "global ",
                "coverage ",
                "orbit ",
                "track ",
                "sky ",
            ]
            .iter()
            .any(|prefix| line.starts_with(prefix))
        });
        if failing.is_none() && !has_anchor {
            failing = Some("anchor");
        }
        let terms_ok = block.iter().any(|line| line.starts_with("terms "))
            && block.iter().any(|line| {
                line.starts_with("terms ")
                    && (line.contains("http://") || line.contains("https://"))
            });
        if failing.is_none() && !terms_ok {
            failing = Some("terms");
        }
        let reader_ok = block
            .iter()
            .any(|line| line.starts_with("format ") || line.starts_with("compiler "));
        if failing.is_none() && !reader_ok {
            failing = Some("reader");
        }
        let hash_ok = block.iter().any(|line| line.starts_with("sha256 "));
        if failing.is_none() && !hash_ok {
            failing = Some("sha256");
        }
        let url = match value_of(block, "url") {
            Some(v) => v,
            None => String::new(),
        };
        if failing.is_none() && !(url.starts_with("http://") || url.starts_with("https://")) {
            failing = Some("url");
        }
        match failing {
            Some(stage) => {
                *first_failure.entry(stage).or_insert(0) += 1;
                examples.entry(stage).or_default().push(url.clone());
            }
            None => passed += 1,
        }
    }
    println!(
        "stage-check: {} blocks, verified in order instrument to url",
        blocks.len()
    );
    for stage in stages {
        let count = match first_failure.get(stage) {
            Some(n) => *n,
            None => 0usize,
        };
        println!("stage-check: first gap at {stage}: {count}");
    }
    println!("stage-check: pass every stage: {passed}");
    for stage in stages {
        if let Some(urls) = examples.get(stage) {
            for url in urls.iter().take(show) {
                println!("stage-check: {stage} missing at {url}");
            }
        }
    }
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

struct SpecRow {
    number: usize,
    url: String,
    line: usize,
}

fn parse_sources(text: &str) -> Vec<String> {
    let mut urls = Vec::new();
    for raw in text.split("\n\n") {
        for line in raw.lines() {
            let cols: Vec<&str> = line.split('\t').collect();
            let first = match cols.get(1) {
                Some(correction) if !correction.trim().is_empty() => correction.trim(),
                _ => cols.first().copied().unwrap_or(""),
            };
            if first.starts_with('#') {
                continue;
            }
            if let Some(v) = first.strip_prefix("url ") {
                urls.push(v.trim().to_string());
            }
        }
    }
    urls
}

fn parse_specs(text: &str) -> Vec<SpecRow> {
    let mut out = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if index == 0 || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 2 {
            continue;
        }
        let Some(number) = cols[0].trim().parse::<usize>().ok() else {
            continue;
        };
        out.push(SpecRow {
            number,
            url: cols[1].trim().to_string(),
            line: index + 1,
        });
    }
    out
}

fn collect_also(args: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for (index, arg) in args.iter().enumerate() {
        if arg == "--also"
            && let Some(value) = args.get(index + 1)
        {
            out.push(value.clone());
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let sources_path = match arg_value(&args, "--sources") {
        Some(v) => v,
        None => "phi/sources.tsv".to_string(),
    };
    let specs_path = match arg_value(&args, "--specs") {
        Some(v) => v,
        None => "phi/field_spec.tsv".to_string(),
    };
    if args.iter().any(|a| a == "--stages") {
        let root = match arg_value(&args, "--root") {
            Some(v) => v,
            None => ".".to_string(),
        };
        let show = arg_value(&args, "--show")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(5usize);
        stage_report(&root, show);
        return;
    }
    let show = arg_value(&args, "--show")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(20usize);

    let sources = PathBuf::from(&sources_path);
    let specs = PathBuf::from(&specs_path);
    let sources_text = match fs::read_to_string(&sources) {
        Ok(t) => t,
        Err(e) => {
            println!("spec-check: {sources_path} unreadable: {e}");
            return;
        }
    };
    let specs_text = match fs::read_to_string(&specs) {
        Ok(t) => t,
        Err(e) => {
            println!("spec-check: {specs_path} unreadable: {e}");
            return;
        }
    };

    let urls = parse_sources(&sources_text);
    let rows = parse_specs(&specs_text);

    let mut elsewhere: BTreeMap<String, String> = BTreeMap::new();
    for path in collect_also(&args) {
        let text = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                println!("spec-check: {path} unreadable: {e}");
                continue;
            }
        };
        for url in parse_sources(&text) {
            elsewhere.insert(url, path.clone());
        }
    }

    let mut by_url: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, url) in urls.iter().enumerate() {
        match by_url.get_mut(url) {
            Some(positions) => positions.push(index),
            None => {
                by_url.insert(url.clone(), vec![index]);
            }
        }
    }
    let repeated: Vec<usize> = by_url
        .values()
        .filter(|positions| positions.len() > 1)
        .map(|positions| positions.len())
        .collect();
    let repeated_extra: usize = repeated.iter().map(|n| n - 1).sum();

    let mut matched = 0usize;
    let mut unmatched: Vec<&SpecRow> = Vec::new();
    let mut number_aligned = 0usize;
    let mut number_shifted = 0usize;
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for row in &rows {
        match seen.get_mut(&row.url) {
            Some(n) => *n += 1,
            None => {
                seen.insert(row.url.clone(), 1);
            }
        }
        match by_url.get(&row.url) {
            Some(positions) => {
                matched += 1;
                if positions[0] + 1 == row.number {
                    number_aligned += 1;
                } else {
                    number_shifted += 1;
                }
            }
            None => unmatched.push(row),
        }
    }
    let spec_duplicates: usize = seen.values().filter(|n| **n > 1).map(|n| n - 1).sum();

    let spec_urls: BTreeSet<&str> = rows.iter().map(|r| r.url.as_str()).collect();
    let blocks_without: Vec<&String> = urls
        .iter()
        .filter(|u| !spec_urls.contains(u.as_str()))
        .collect();

    println!(
        "spec-check: {} block urls in {}, {} rows in {}",
        urls.len(),
        sources_path,
        rows.len(),
        specs_path
    );
    let moved: Vec<&&SpecRow> = unmatched
        .iter()
        .filter(|row| elsewhere.contains_key(&row.url))
        .collect();
    let homeless: Vec<&&SpecRow> = unmatched
        .iter()
        .filter(|row| !elsewhere.contains_key(&row.url))
        .collect();
    println!(
        "spec-check: matched {matched}, rows in another register {}, rows nowhere {}, blocks without row {}",
        moved.len(),
        homeless.len(),
        blocks_without.len()
    );
    let mut by_owner: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &moved {
        if let Some(path) = elsewhere.get(&row.url) {
            match by_owner.get_mut(path.as_str()) {
                Some(n) => *n += 1,
                None => {
                    by_owner.insert(path.as_str(), 1);
                }
            }
        }
    }
    for (path, count) in &by_owner {
        println!("spec-check: moved to {path}: {count}");
    }
    println!(
        "spec-check: numbering aligned {number_aligned}, shifted {number_shifted}; duplicate spec rows {spec_duplicates}, repeated block urls {repeated_extra}"
    );

    for row in homeless.iter().take(show) {
        println!(
            "spec-check: row {} (line {}) has no block: {}",
            row.number, row.line, row.url
        );
    }
    for url in blocks_without.iter().take(show) {
        println!("spec-check: block has no row: {url}");
    }
}
