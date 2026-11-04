use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use omegaflow::archivar::extract::{ExtractResult, extract};
use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::archivar::parse::parse_sources;
use omegaflow::archivar::types::Extract;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn index_sources(root: &Path) -> BTreeMap<String, PathBuf> {
    let mut map = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                map.entry(name.to_string()).or_insert(path);
            }
        }
    }
    map
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let register = match arg_value(&args, "--register") {
        Some(v) => v,
        None => {
            println!("usage: source_missing --register <sources.φ> --root <sources dir>");
            return;
        }
    };
    let root = match arg_value(&args, "--root") {
        Some(v) => v,
        None => {
            println!("usage: source_missing --register <sources.φ> --root <sources dir>");
            return;
        }
    };
    let max_bytes = arg_value(&args, "--max-bytes")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(32 * 1024 * 1024);

    let text = match fs::read_to_string(&register) {
        Ok(t) => t,
        Err(e) => {
            println!("source_missing: {register}: {e}");
            return;
        }
    };
    let sources = parse_sources(&text);
    let files = index_sources(Path::new(&root));
    let lsk = match embedded_lsk() {
        Some(v) => v,
        None => {
            println!("source_missing: no leap second table");
            return;
        }
    };

    let now = 1.8e9_f64;
    let mut seen = 0usize;
    let mut read = 0usize;
    for src in &sources {
        let Some(name) = src.url.split('/').next_back() else {
            continue;
        };
        let Some(path) = files.get(name) else {
            continue;
        };
        seen += 1;
        let size = fs::metadata(path).map(|m| m.len()).unwrap_or(u64::MAX);
        if size > max_bytes {
            continue;
        }
        let Some(body) = path.to_str() else {
            continue;
        };
        let channels = match extract(src, body, now, &lsk) {
            ExtractResult::Measurements(channels) | ExtractResult::WithEphemeris(channels, _) => {
                channels
            }
        };
        if channels.is_empty() {
            continue;
        }
        read += 1;
        let declared: Vec<String> = src
            .extracts
            .iter()
            .filter_map(|e| match e {
                Extract::Field(fc) | Extract::First(fc, _) | Extract::Last(fc, _) => {
                    Some(fc.name.clone())
                }
                _ => None,
            })
            .collect();
        if declared.is_empty() {
            continue;
        }
        let missing: Vec<String> = declared
            .iter()
            .filter(|d| !channels.iter().any(|c| c.0.name == **d))
            .cloned()
            .collect();
        if missing.is_empty() {
            continue;
        }
        println!(
            "{}\t{}\t{}\t{}",
            src.url,
            src.format,
            declared.len(),
            missing.join(",")
        );
        let _ = std::io::stdout().flush();
    }
    eprintln!("source_missing: {seen} with a file, {read} read");
}
