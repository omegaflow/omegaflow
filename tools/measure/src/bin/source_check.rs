use std::collections::BTreeMap;
use std::fs;
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

struct Outcome {
    with_file: usize,
    read: usize,
    all_names: usize,
    partial: usize,
    none: usize,
    no_file: usize,
    skipped: usize,
    above_cap: BTreeMap<String, usize>,
    unreadable: BTreeMap<String, usize>,
    extracts_by_format: BTreeMap<String, (usize, usize)>,
    examples: Vec<String>,
}

impl Outcome {
    fn new() -> Outcome {
        Outcome {
            with_file: 0,
            read: 0,
            all_names: 0,
            partial: 0,
            none: 0,
            no_file: 0,
            skipped: 0,
            above_cap: BTreeMap::new(),
            unreadable: BTreeMap::new(),
            extracts_by_format: BTreeMap::new(),
            examples: Vec::new(),
        }
    }

    fn merge(&mut self, other: Outcome) {
        self.with_file += other.with_file;
        self.read += other.read;
        self.all_names += other.all_names;
        self.partial += other.partial;
        self.none += other.none;
        self.no_file += other.no_file;
        self.skipped += other.skipped;
        for (k, v) in other.above_cap {
            *self.above_cap.entry(k).or_insert(0) += v;
        }
        for (k, v) in other.unreadable {
            *self.unreadable.entry(k).or_insert(0) += v;
        }
        for (k, (b, e)) in other.extracts_by_format {
            let slot = self.extracts_by_format.entry(k).or_insert((0, 0));
            slot.0 += b;
            slot.1 += e;
        }
    }
}

fn audit(
    src: &omegaflow::archivar::types::SourceConfig,
    files: &BTreeMap<String, PathBuf>,
    lsk: &omegaflow::archivar::lsk::LeapSeconds,
    max_bytes: u64,
    show: usize,
    verbose: bool,
) -> Outcome {
    let mut out = Outcome::new();
    let name = match src.url.split('/').next_back() {
        Some(n) => n.to_string(),
        None => return out,
    };
    let path = match files.get(&name) {
        Some(p) => p,
        None => {
            if !src.url.is_empty() {
                out.no_file += 1;
                if out.examples.len() < show {
                    out.examples
                        .push(format!("no file for {} (format {})", name, src.format));
                }
            }
            return out;
        }
    };
    out.with_file += 1;
    {
        let entry = out
            .extracts_by_format
            .entry(src.format.clone())
            .or_insert((0, 0));
        entry.0 += 1;
        entry.1 += src.extracts.len();
    }
    let size = fs::metadata(path).map(|m| m.len()).unwrap_or(u64::MAX);
    if size > max_bytes {
        out.skipped += 1;
        *out.above_cap.entry(src.format.clone()).or_insert(0) += 1;
        if out.examples.len() < show {
            out.examples
                .push(format!("{name}: {size} bytes above the cap of {max_bytes}"));
        }
        return out;
    }
    let Some(body) = path.to_str() else {
        return out;
    };
    let channels = match extract(src, body, 1.8e9_f64, lsk) {
        ExtractResult::Measurements(channels) | ExtractResult::WithEphemeris(channels, _) => {
            channels
        }
    };
    let verdict = if channels.is_empty() { "none" } else { "read" };
    if verbose {
        println!(
            "source_check_block: {verdict} {} {} {}",
            src.url,
            channels.len(),
            src.extracts.len()
        );
    }
    if channels.is_empty() {
        *out.unreadable.entry(src.format.clone()).or_insert(0) += 1;
        if out.examples.len() < show {
            out.examples.push(format!(
                "{name}: format {} with {} extracts read nothing",
                src.format,
                src.extracts.len()
            ));
        }
        return out;
    }
    out.read += 1;
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
        out.all_names += 1;
        return out;
    }
    let hit = declared
        .iter()
        .filter(|d| channels.iter().any(|c| c.0.name == **d))
        .count();
    if hit == declared.len() {
        out.all_names += 1;
    } else if hit > 0 {
        out.partial += 1;
        if out.examples.len() < show {
            out.examples.push(format!(
                "{name}: {hit} of {} declared names among {} channels",
                declared.len(),
                channels.len()
            ));
        }
    } else {
        out.none += 1;
        if out.examples.len() < show {
            out.examples.push(format!(
                "{name}: none of {} declared names among {} channels",
                declared.len(),
                channels.len()
            ));
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let release = match arg_value(&args, "--register") {
        Some(v) => v,
        None => {
            println!(
                "usage: source_check --register <sources.φ> --root <sources dir> [--show <n>] [--threads <n>]"
            );
            return;
        }
    };
    let root = match arg_value(&args, "--root") {
        Some(v) => v,
        None => "sources".to_string(),
    };
    let max_bytes = arg_value(&args, "--max-bytes")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(32 * 1024 * 1024);
    let show = arg_value(&args, "--show")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(10usize);
    let threads = arg_value(&args, "--threads")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1usize);
    if threads == 0 {
        println!("source_check: --threads needs at least one");
        return;
    }

    let text = match fs::read_to_string(&release) {
        Ok(t) => t,
        Err(e) => {
            println!("source_check: {release}: {e}");
            return;
        }
    };
    let sources = parse_sources(&text);
    let files = index_sources(Path::new(&root));
    let lsk = match embedded_lsk() {
        Some(v) => v,
        None => {
            println!("source_check: no leap second table");
            return;
        }
    };
    println!(
        "source_check: {} sources parsed, {} files under {root}",
        sources.len(),
        files.len()
    );

    let verbose = threads == 1;
    let mut total = Outcome::new();
    if threads == 1 {
        for src in &sources {
            total.merge(audit(src, &files, &lsk, max_bytes, show, verbose));
        }
    } else {
        let next = std::sync::atomic::AtomicUsize::new(0);
        let mut outcomes = std::thread::scope(|scope| {
            let mut handles = Vec::new();
            for _ in 0..threads {
                let files = &files;
                let lsk = &lsk;
                let sources = &sources;
                let next = &next;
                handles.push(scope.spawn(move || {
                    let mut out = Outcome::new();
                    loop {
                        let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let Some(src) = sources.get(i) else {
                            break;
                        };
                        out.merge(audit(src, files, lsk, max_bytes, show, false));
                    }
                    out
                }));
            }
            handles
                .into_iter()
                .map(|h| h.join().expect("a worker panicked"))
                .collect::<Vec<Outcome>>()
        });
        for out in outcomes.drain(..) {
            total.merge(out);
        }
    }

    println!(
        "source_check: {} above the byte cap, not measured",
        total.skipped
    );
    println!(
        "source_check: {} with a local file, {} read by the pipeline, {} with every declared name present, {} partial, {} none, {} without a file",
        total.with_file, total.read, total.all_names, total.partial, total.none, total.no_file
    );
    let mut top: Vec<(&String, &usize)> = total.unreadable.iter().collect();
    top.sort_by(|a, b| b.1.cmp(a.1));
    for (format, count) in top.iter().take(show) {
        println!("source_check: format {format} produced nothing: {count}");
    }
    let mut per_format: Vec<(&String, &(usize, usize))> = total.extracts_by_format.iter().collect();
    per_format.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    for (format, (blocks, extracts)) in per_format.iter().take(show) {
        println!(
            "source_check: format {format}: {blocks} blocks with a file, {extracts} extracts in sum"
        );
    }
    for example in total.examples.iter() {
        println!("source_check: {example}");
    }
}
