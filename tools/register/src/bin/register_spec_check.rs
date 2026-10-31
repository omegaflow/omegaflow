use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

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
            let first = line.split('\t').next().unwrap_or("");
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
