use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use omegaflow::archivar::extract::{ExtractResult, extract};
use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::archivar::parse::parse_sources;
use omegaflow::archivar::spectral::civil_from_days;

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

fn iso_utc(unix: f64) -> Option<String> {
    if !unix.is_finite() {
        return None;
    }
    let secs = unix.floor() as i64;
    let frac = unix - unix.floor() as f64;
    let millis = (frac * 1000.0).round().clamp(0.0, 999.0) as u32;
    let (y, m, d) = civil_from_days(secs.div_euclid(86_400))?;
    let sod = secs.rem_euclid(86_400) as u32;
    Some(format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}.{millis:03}Z",
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let register = match arg_value(&args, "--register") {
        Some(v) => v,
        None => {
            println!("usage: source_coverage --register <sources.φ> --root <sources dir>");
            return;
        }
    };
    let root = match arg_value(&args, "--root") {
        Some(v) => v,
        None => {
            println!("usage: source_coverage --register <sources.φ> --root <sources dir>");
            return;
        }
    };
    let max_bytes = arg_value(&args, "--max-bytes")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(32 * 1024 * 1024);

    let text = match fs::read_to_string(&register) {
        Ok(t) => t,
        Err(e) => {
            println!("source_coverage: {register}: {e}");
            return;
        }
    };
    let sources = parse_sources(&text);
    let files = index_sources(Path::new(&root));
    let lsk = match embedded_lsk() {
        Some(v) => v,
        None => {
            println!("source_coverage: no leap second table");
            return;
        }
    };

    let now = 1.8e9_f64;
    let mut measured = 0usize;
    let mut no_epoch = 0usize;
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
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        let mut n = 0usize;
        for (c, _) in &channels {
            if !c.epoch.is_finite() {
                continue;
            }
            lo = lo.min(c.epoch);
            hi = hi.max(c.epoch);
            n += 1;
        }
        if n == 0 {
            no_epoch += 1;
            continue;
        }
        let (Some(start), Some(end)) = (lsk.tdb_to_unix(lo), lsk.tdb_to_unix(hi)) else {
            no_epoch += 1;
            continue;
        };
        let (Some(start), Some(end)) = (iso_utc(start), iso_utc(end)) else {
            no_epoch += 1;
            continue;
        };
        println!("{}\t{start}\t{end}\t{n}", src.url);
        measured += 1;
    }
    eprintln!(
        "source_coverage: {measured} measured, {no_epoch} without a usable epoch, {no_file} without a file"
    );
}
