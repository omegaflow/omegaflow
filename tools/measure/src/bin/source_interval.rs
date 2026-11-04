use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use omegaflow::archivar::extract::{ExtractResult, extract};
use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::archivar::parse::parse_sources;

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
    let release = match arg_value(&args, "--register") {
        Some(v) => v,
        None => {
            println!("usage: source_interval --register <sources.φ> --root <sources dir>");
            return;
        }
    };
    let root = match arg_value(&args, "--root") {
        Some(v) => v,
        None => {
            println!("usage: source_interval --register <sources.φ> --root <sources dir>");
            return;
        }
    };
    let max_bytes = arg_value(&args, "--max-bytes")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(32 * 1024 * 1024);

    let text = match fs::read_to_string(&release) {
        Ok(t) => t,
        Err(e) => {
            println!("source_interval: {release}: {e}");
            return;
        }
    };
    let sources = parse_sources(&text);
    let files = index_sources(Path::new(&root));
    let lsk = match embedded_lsk() {
        Some(v) => v,
        None => {
            println!("source_interval: no leap second table");
            return;
        }
    };

    let now = 1.8e9_f64;
    let mut measured = 0usize;
    let mut one_epoch = 0usize;
    let mut no_file = 0usize;
    for src in &sources {
        let Some(name) = src.url.split('/').next_back() else {
            continue;
        };
        let Some(path) = files.get(name) else {
            no_file += 1;
            continue;
        };
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
        let mut epochs: Vec<f64> = channels
            .iter()
            .map(|(c, _)| c.epoch)
            .filter(|e| e.is_finite() && *e > 0.0)
            .collect();
        epochs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        epochs.dedup();
        if epochs.len() < 2 {
            one_epoch += 1;
            continue;
        }
        let mut deltas: Vec<f64> = epochs
            .windows(2)
            .map(|w| w[1] - w[0])
            .filter(|d| d.is_finite() && *d > 0.0)
            .collect();
        if deltas.is_empty() {
            one_epoch += 1;
            continue;
        }
        deltas.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median = deltas[deltas.len() / 2];
        println!(
            "{}\t{median}\t{}\t{}\t{}",
            src.url,
            epochs.len(),
            deltas[0],
            deltas[deltas.len() - 1]
        );
        measured += 1;
    }
    println!(
        "source_interval: {measured} measured, {one_epoch} with less than two epochs, {no_file} without a file"
    );
}
