use omegaflow::archivar::sniff::sniff;
use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

const DEFAULT_DIGITS: &str = "1,2,3,4,5,6";
const DEFAULT_MAX_K: usize = 8;
const DEFAULT_TARGETS: usize = 10;
const DEFAULT_TOLERANCE: &str = "0";
const DEFAULT_MAX_MB: u64 = 32;
const MAX_DIGITS: u32 = 15;
const SKIP_DIRS: &[&str] = &[".git", "node_modules"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Format {
    Tsv,
    Json,
}

#[derive(Clone, Copy, Debug)]
enum Tolerance {
    Exact,
    Absolute(f64),
    Percent(f64),
}

impl Tolerance {
    fn label(self) -> String {
        match self {
            Tolerance::Exact => "0".to_string(),
            Tolerance::Absolute(v) => format!("{v}"),
            Tolerance::Percent(p) => format!("{}%", p * 100.0),
        }
    }
}

struct Options {
    roots: Vec<String>,
    digits: Vec<u32>,
    max_k: usize,
    targets: usize,
    tolerance: Tolerance,
    format: Format,
    max_mb: u64,
}

enum Parsed {
    Run(Options),
    Help,
}

struct Dataset {
    values: Vec<f64>,
}

struct Prepared {
    keys: Vec<String>,
    values: Vec<f64>,
}

struct Row {
    digits: u32,
    k: usize,
    targets: usize,
    unique: usize,
    false_matches: usize,
    k_min_95: Option<usize>,
    k_min_99: Option<usize>,
    hit_rate: Option<f64>,
    gap_min: Option<f64>,
    gap_median: Option<f64>,
}

struct Report {
    datasets: usize,
    target_count: usize,
    max_k: usize,
    tolerance: String,
    rows: Vec<Row>,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse_args(&args) {
        Ok(Parsed::Help) => {
            print_help();
        }
        Ok(Parsed::Run(opts)) => match analyze(&opts) {
            Ok(report) => match opts.format {
                Format::Tsv => print!("{}", render_tsv(&report)),
                Format::Json => print!("{}", render_json(&report)),
            },
            Err(err) => {
                eprintln!("bait_efficiency: {err}");
                std::process::exit(2);
            }
        },
        Err(err) => {
            eprintln!("bait_efficiency: {err}");
            eprintln!("bait_efficiency --help");
            std::process::exit(2);
        }
    }
}

fn parse_args(args: &[String]) -> Result<Parsed, String> {
    let mut roots: Vec<String> = Vec::new();
    let mut digits: Option<Vec<u32>> = None;
    let mut max_k: Option<usize> = None;
    let mut targets: Option<usize> = None;
    let mut tolerance: Option<Tolerance> = None;
    let mut format: Option<Format> = None;
    let mut i = 0usize;
    while i < args.len() {
        let arg = args[i].as_str();
        match arg {
            "-h" | "--help" => return Ok(Parsed::Help),
            "--root" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| "--root requires a directory".to_string())?;
                roots.push(value.clone());
            }
            "--digits" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| "--digits requires a comma list".to_string())?;
                digits = Some(parse_digits(value)?);
            }
            "--max-k" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| "--max-k requires a number".to_string())?;
                max_k = Some(parse_positive(value, "--max-k")?);
            }
            "--targets" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| "--targets requires a number".to_string())?;
                targets = Some(parse_positive(value, "--targets")?);
            }
            "--tolerance" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| "--tolerance requires <abs|N%>".to_string())?;
                tolerance = Some(parse_tolerance(value)?);
            }
            "--format" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| "--format requires tsv or json".to_string())?;
                format = Some(match value.as_str() {
                    "tsv" => Format::Tsv,
                    "json" => Format::Json,
                    other => return Err(format!("unknown format: {other}")),
                });
            }
            other => return Err(format!("unknown argument: {other}")),
        }
        i += 1;
    }
    if roots.is_empty() {
        return Err("--root is required".to_string());
    }
    Ok(Parsed::Run(Options {
        roots,
        digits: match digits {
            Some(v) => v,
            None => parse_digits(DEFAULT_DIGITS)?,
        },
        max_k: match max_k {
            Some(value) => value,
            None => DEFAULT_MAX_K,
        },
        targets: match targets {
            Some(value) => value,
            None => DEFAULT_TARGETS,
        },
        tolerance: match tolerance {
            Some(value) => value,
            None => Tolerance::Exact,
        },
        format: match format {
            Some(value) => value,
            None => Format::Tsv,
        },
        max_mb: DEFAULT_MAX_MB,
    }))
}

fn parse_positive(value: &str, flag: &str) -> Result<usize, String> {
    let parsed: usize = value
        .parse()
        .map_err(|_| format!("{flag} expects a positive integer, got {value}"))?;
    if parsed == 0 {
        return Err(format!("{flag} must be greater than zero"));
    }
    Ok(parsed)
}

fn parse_digits(value: &str) -> Result<Vec<u32>, String> {
    let mut set: BTreeSet<u32> = BTreeSet::new();
    for part in value.split(',') {
        let token = part.trim();
        if token.is_empty() {
            return Err(format!("empty digit count in {value}"));
        }
        let parsed: u32 = token
            .parse()
            .map_err(|_| format!("digit count is not an integer: {token}"))?;
        if parsed == 0 {
            return Err("digit count must be at least 1".to_string());
        }
        if parsed > MAX_DIGITS {
            return Err(format!(
                "digit count {parsed} exceeds the supported maximum {MAX_DIGITS}"
            ));
        }
        set.insert(parsed);
    }
    if set.is_empty() {
        return Err("--digits requires at least one value".to_string());
    }
    Ok(set.into_iter().collect())
}

fn parse_tolerance(value: &str) -> Result<Tolerance, String> {
    let token = value.trim();
    if token.is_empty() {
        return Err("--tolerance is empty".to_string());
    }
    if let Some(percent) = token.strip_suffix('%') {
        let parsed: f64 = percent
            .trim()
            .parse()
            .map_err(|_| format!("tolerance is not a number: {value}"))?;
        if !parsed.is_finite() || parsed < 0.0 {
            return Err(format!("tolerance must be finite and non-negative: {value}"));
        }
        if parsed == 0.0 {
            return Ok(Tolerance::Exact);
        }
        return Ok(Tolerance::Percent(parsed / 100.0));
    }
    let parsed: f64 = token
        .parse()
        .map_err(|_| format!("tolerance is not a number: {value}"))?;
    if !parsed.is_finite() || parsed < 0.0 {
        return Err(format!("tolerance must be finite and non-negative: {value}"));
    }
    if parsed == 0.0 {
        return Ok(Tolerance::Exact);
    }
    Ok(Tolerance::Absolute(parsed))
}

fn print_help() {
    eprintln!("bait_efficiency — measure value count, significant digits and value spacing against a local corpus");
    eprintln!();
    eprintln!("usage:");
    eprintln!("  bait_efficiency --root <dir>... [--digits 1,2,3] [--max-k <n>]");
    eprintln!("                  [--targets <n>] [--tolerance <abs|N%>] [--format tsv|json]");
    eprintln!();
    eprintln!("arguments:");
    eprintln!("  --root <dir>       corpus root; repeatable, files are read recursively");
    eprintln!("  --digits <d>       comma list of significant digits, 1..={MAX_DIGITS} (default {DEFAULT_DIGITS})");
    eprintln!("  --max-k <n>        largest k, checked for k=1..n (default {DEFAULT_MAX_K})");
    eprintln!("  --targets <n>      at most n target datasets, evenly spread over the sorted corpus (default {DEFAULT_TARGETS})");
    eprintln!("  --tolerance <t>    absolute tolerance or percent, e.g. 0.5 or 5% (default {DEFAULT_TOLERANCE}); percent is relative to max(|candidate|,|target|)");
    eprintln!("  --format <f>       tsv or json (default tsv)");
    eprintln!("  -h|--help          this help");
    eprintln!();
    eprintln!("measurement:");
    eprintln!("  a dataset is a text file with a recognized format (csv/tsv/fixed) and at least two numeric values.");
    eprintln!("  values are rounded to d significant digits; duplicates collapse.");
    eprintln!("  example: 7.83 at d=2 is 7.8; 55041.35 at d=5 is 55041.");
    eprintln!("  for a target, the first k distinct rounded values are the baits; the target itself is not a candidate.");
    eprintln!("  a candidate hits when all k baits are in its rounded values (exact or within tolerance).");
    eprintln!("  unique counts targets without a false hit; hit_rate = unique/targets; false_matches counts all candidate hits.");
    eprintln!("  k_min_95/k_min_99 is the smallest k with hit_rate >= 0.95 or >= 0.99; otherwise pending.");
    eprintln!("  gap_min/gap_median are the smallest and median relative gaps between adjacent bait values of a target,");
    eprintln!("  relative to the larger absolute value of the pair; at k=1 there is no gap and output is pending.");
    eprintln!("  files above {DEFAULT_MAX_MB} MiB are not read; .git and node_modules are skipped.");
    eprintln!("  no interpolation; missing data stays pending, never a silent zero.");
    eprintln!();
    eprintln!("tsv output:");
    eprintln!("  digits  k  targets  unique  false_matches  k_min_95  k_min_99  hit_rate  gap_min  gap_median");
    eprintln!("json output:");
    eprintln!("  {{\"tool\",\"datasets\",\"target_count\",\"max_k\",\"tolerance\",\"rows\":[{{...}}]}}");
}

fn collect_files(roots: &[String]) -> Result<Vec<(String, PathBuf)>, String> {
    let mut seen: BTreeSet<PathBuf> = BTreeSet::new();
    let mut files: Vec<(String, PathBuf)> = Vec::new();
    for root in roots {
        let path = PathBuf::from(root);
        let mut found: Vec<PathBuf> = Vec::new();
        if path.is_file() {
            found.push(path);
        } else if path.is_dir() {
            walk(&path, &mut found);
        } else {
            return Err(format!("root not found: {root}"));
        }
        for candidate in found {
            let key = match std::fs::canonicalize(&candidate) {
                Ok(key) => key,
                Err(_) => candidate.clone(),
            };
            if !seen.insert(key) {
                continue;
            }
            files.push((candidate.to_string_lossy().into_owned(), candidate));
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(files)
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        let path = entry.path();
        if file_type.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if SKIP_DIRS.contains(&name.as_ref()) {
                continue;
            }
            walk(&path, out);
        } else if file_type.is_file() {
            out.push(path);
        }
    }
}

fn read_bounded(path: &Path, max_bytes: u64) -> Option<Vec<u8>> {
    let file = File::open(path).ok()?;
    let mut buf = Vec::new();
    file.take(max_bytes.saturating_add(1))
        .read_to_end(&mut buf)
        .ok()?;
    if buf.len() as u64 > max_bytes {
        return None;
    }
    Some(buf)
}

fn load_datasets(files: Vec<(String, PathBuf)>, max_mb: u64) -> Vec<Dataset> {
    let cap = max_mb.saturating_mul(1_048_576);
    let mut datasets: Vec<Dataset> = Vec::new();
    for (_display, path) in files {
        let Some(bytes) = read_bounded(&path, cap) else {
            continue;
        };
        let sniffed = sniff(&bytes);
        if sniffed.magic.is_some() {
            continue;
        }
        if !matches!(sniffed.format.as_str(), "csv" | "tsv" | "fixed") {
            continue;
        }
        let values = extract_values(&bytes, &sniffed);
        if values.len() < 2 {
            continue;
        }
        datasets.push(Dataset { values });
    }
    datasets
}

fn extract_values(bytes: &[u8], sniffed: &omegaflow::archivar::sniff::Sniffed) -> Vec<f64> {
    let text = String::from_utf8_lossy(bytes);
    let mut lines = text.lines().filter(|line| {
        let trimmed = line.trim();
        !trimmed.is_empty() && !trimmed.starts_with('#')
    });
    if sniffed.header.is_some() {
        lines.next();
    }
    let mut values: Vec<f64> = Vec::new();
    for line in lines {
        let fields: Vec<String> = match sniffed.delimiter {
            Some(delimiter) => split_quoted(line, delimiter),
            None => line.split_whitespace().map(str::to_string).collect(),
        };
        for field in fields {
            if let Some(value) = parse_number(&field) {
                values.push(value);
            }
        }
    }
    values
}

fn split_quoted(line: &str, delimiter: char) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    current.push('"');
                } else {
                    quoted = false;
                }
            } else {
                current.push(c);
            }
        } else if c == '"' {
            quoted = true;
        } else if c == delimiter {
            fields.push(std::mem::take(&mut current));
        } else {
            current.push(c);
        }
    }
    fields.push(current);
    fields
}

fn parse_number(field: &str) -> Option<f64> {
    let token = field.trim().trim_matches('"').trim();
    if token.is_empty() {
        return None;
    }
    let value: f64 = token.parse().ok()?;
    if value.is_finite() {
        Some(value)
    } else {
        None
    }
}

fn select_targets(count: usize, wanted: usize) -> Vec<usize> {
    if count == 0 || wanted == 0 {
        return Vec::new();
    }
    if count <= wanted {
        return (0..count).collect();
    }
    (0..wanted).map(|index| index * count / wanted).collect()
}

fn round_significant(value: f64, digits: u32) -> Option<(String, f64)> {
    if value == 0.0 {
        return Some(("0".to_string(), 0.0));
    }
    let text = format!("{:.*e}", digits as usize - 1, value);
    let rounded: f64 = text.parse().ok()?;
    if !rounded.is_finite() {
        return None;
    }
    Some((text, rounded))
}

fn prepare(dataset: &Dataset, digits: u32) -> Prepared {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<f64> = Vec::new();
    for &value in &dataset.values {
        let Some((key, rounded)) = round_significant(value, digits) else {
            continue;
        };
        if seen.insert(key.clone()) {
            keys.push(key);
            values.push(rounded);
        }
    }
    Prepared { keys, values }
}

fn within(
    candidate_key: &str,
    candidate_value: f64,
    target_key: &str,
    target_value: f64,
    tolerance: Tolerance,
) -> bool {
    match tolerance {
        Tolerance::Exact => candidate_key == target_key,
        Tolerance::Absolute(absolute) => (candidate_value - target_value).abs() <= absolute,
        Tolerance::Percent(percent) => {
            (candidate_value - target_value).abs()
                <= percent * candidate_value.abs().max(target_value.abs())
        }
    }
}

fn contains_all(
    candidate: &Prepared,
    probe_keys: &[String],
    probe_values: &[f64],
    tolerance: Tolerance,
) -> bool {
    probe_keys
        .iter()
        .zip(probe_values.iter())
        .all(|(target_key, &target_value)| {
            candidate
                .keys
                .iter()
                .zip(candidate.values.iter())
                .any(|(candidate_key, &candidate_value)| {
                    within(
                        candidate_key,
                        candidate_value,
                        target_key,
                        target_value,
                        tolerance,
                    )
                })
        })
}

fn median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.total_cmp(b));
    let mid = values.len() / 2;
    if values.len() % 2 == 0 {
        Some((values[mid - 1] + values[mid]) / 2.0)
    } else {
        Some(values[mid])
    }
}

fn gaps_of(values: &[f64]) -> Vec<f64> {
    if values.len() < 2 {
        return Vec::new();
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let mut gaps = Vec::new();
    for pair in sorted.windows(2) {
        let a = pair[0];
        let b = pair[1];
        let denominator = a.abs().max(b.abs());
        if denominator > 0.0 {
            gaps.push((b - a).abs() / denominator);
        }
    }
    gaps
}

fn analyze(opts: &Options) -> Result<Report, String> {
    let files = collect_files(&opts.roots)?;
    let datasets = load_datasets(files, opts.max_mb);
    let target_indices = select_targets(datasets.len(), opts.targets);
    let mut rows: Vec<Row> = Vec::new();
    for &digits in &opts.digits {
        let prepared: Vec<Prepared> = datasets
            .iter()
            .map(|dataset| prepare(dataset, digits))
            .collect();
        let start = rows.len();
        let mut min_95: Option<usize> = None;
        let mut min_99: Option<usize> = None;
        for k in 1..=opts.max_k {
            let mut evaluated = 0usize;
            let mut unique = 0usize;
            let mut false_matches = 0usize;
            let mut gaps: Vec<f64> = Vec::new();
            for &target_index in &target_indices {
                let probe_keys = &prepared[target_index].keys;
                let probe_values = &prepared[target_index].values;
                if probe_keys.len() < k {
                    continue;
                }
                evaluated += 1;
                let mut hits = 0usize;
                for (candidate_index, candidate) in prepared.iter().enumerate() {
                    if candidate_index == target_index {
                        continue;
                    }
                    if contains_all(
                        candidate,
                        &probe_keys[..k],
                        &probe_values[..k],
                        opts.tolerance,
                    ) {
                        hits += 1;
                    }
                }
                if hits == 0 {
                    unique += 1;
                } else {
                    false_matches += hits;
                }
                gaps.extend(gaps_of(&prepared[target_index].values[..k]));
            }
            let hit_rate = match evaluated {
                0 => None,
                _ => Some(unique as f64 / evaluated as f64),
            };
            if let Some(rate) = hit_rate {
                if min_95.is_none() && rate >= 0.95 {
                    min_95 = Some(k);
                }
                if min_99.is_none() && rate >= 0.99 {
                    min_99 = Some(k);
                }
            }
            let gap_min = gaps.iter().copied().reduce(f64::min);
            let gap_median = median(&mut gaps);
            rows.push(Row {
                digits,
                k,
                targets: evaluated,
                unique,
                false_matches,
                k_min_95: None,
                k_min_99: None,
                hit_rate,
                gap_min,
                gap_median,
            });
        }
        for row in &mut rows[start..] {
            row.k_min_95 = min_95;
            row.k_min_99 = min_99;
        }
    }
    Ok(Report {
        datasets: datasets.len(),
        target_count: target_indices.len(),
        max_k: opts.max_k,
        tolerance: opts.tolerance.label(),
        rows,
    })
}

fn render_tsv(report: &Report) -> String {
    let mut out = String::from(
        "digits\tk\ttargets\tunique\tfalse_matches\tk_min_95\tk_min_99\thit_rate\tgap_min\tgap_median\n",
    );
    for row in &report.rows {
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            row.digits,
            row.k,
            row.targets,
            row.unique,
            row.false_matches,
            option_k(row.k_min_95),
            option_k(row.k_min_99),
            option_rate(row.hit_rate),
            option_gap(row.gap_min),
            option_gap(row.gap_median)
        ));
    }
    out
}

fn option_rate(value: Option<f64>) -> String {
    match value {
        Some(rate) => format!("{rate:.4}"),
        None => "pending".to_string(),
    }
}

fn option_k(value: Option<usize>) -> String {
    match value {
        Some(k) => k.to_string(),
        None => "pending".to_string(),
    }
}

fn option_gap(value: Option<f64>) -> String {
    match value {
        Some(gap) => format!("{gap:.6}"),
        None => "pending".to_string(),
    }
}

fn render_json(report: &Report) -> String {
    let mut out = String::from("{\n");
    out.push_str("  \"tool\":\"bait_efficiency\",\n");
    out.push_str(&format!("  \"datasets\":{},\n", report.datasets));
    out.push_str(&format!("  \"target_count\":{},\n", report.target_count));
    out.push_str(&format!("  \"max_k\":{},\n", report.max_k));
    out.push_str(&format!(
        "  \"tolerance\":{},\n",
        json_string(&report.tolerance)
    ));
    out.push_str("  \"rows\":[\n");
    for (index, row) in report.rows.iter().enumerate() {
        let rate = match row.hit_rate {
            Some(value) => format!("{value:.4}"),
            None => json_string("pending"),
        };
        let k95 = match row.k_min_95 {
            Some(value) => value.to_string(),
            None => json_string("pending"),
        };
        let k99 = match row.k_min_99 {
            Some(value) => value.to_string(),
            None => json_string("pending"),
        };
        let gap_min = match row.gap_min {
            Some(value) => format!("{value:.6}"),
            None => json_string("pending"),
        };
        let gap_median = match row.gap_median {
            Some(value) => format!("{value:.6}"),
            None => json_string("pending"),
        };
        out.push_str(&format!(
            "    {{\"digits\":{},\"k\":{},\"targets\":{},\"unique\":{},\"false_matches\":{},\"k_min_95\":{},\"k_min_99\":{},\"hit_rate\":{},\"gap_min\":{},\"gap_median\":{}}}",
            row.digits,
            row.k,
            row.targets,
            row.unique,
            row.false_matches,
            k95,
            k99,
            rate,
            gap_min,
            gap_median
        ));
        if index + 1 < report.rows.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ]\n}\n");
    out
}

fn json_string(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::json::{JsonVal, parse_json};
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static SEQ: AtomicUsize = AtomicUsize::new(0);

    fn fixture_dir() -> PathBuf {
        let seq = SEQ.fetch_add(1, Ordering::SeqCst);
        let dir = env::temp_dir().join(format!(
            "bait_efficiency_{}_{}",
            std::process::id(),
            seq
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_fixture() -> PathBuf {
        let dir = fixture_dir();
        let datasets = [
            ("a.csv", "10.10,120.20,2300.30,34000.40\n10.20,122.20,2330.30,34040.40\n"),
            ("b.csv", "11.40,210.30,3200.40,43000.50\n12.40,212.30,3230.40,43040.50\n"),
            ("c.csv", "50.10,520.20,6300.30,74000.40\n51.10,522.20,6330.30,74040.40\n"),
            ("d.csv", "60.10,620.20,7300.30,84000.40\n61.10,622.20,7330.30,84040.40\n"),
            ("e.csv", "70.10,720.20,8300.30,94000.40\n71.10,722.20,8330.30,94040.40\n"),
        ];
        for (name, rows) in datasets {
            let content = format!("v1,v2,v3,v4\n{rows}");
            fs::write(dir.join(name), content).unwrap();
        }
        dir
    }

    fn write_spacing_corpus(wide: bool) -> PathBuf {
        let dir = fixture_dir();
        let datasets: [(&str, [f64; 3]); 5] = if wide {
            [
                ("a.csv", [100.0, 10000.0, 20000.0]),
                ("b.csv", [500.0, 11000.0, 21000.0]),
                ("c.csv", [900.0, 12000.0, 22000.0]),
                ("d.csv", [1300.0, 13000.0, 23000.0]),
                ("e.csv", [1700.0, 14000.0, 24000.0]),
            ]
        } else {
            [
                ("a.csv", [100.0, 111.0, 112.0]),
                ("b.csv", [100.0, 121.0, 122.0]),
                ("c.csv", [100.0, 131.0, 132.0]),
                ("d.csv", [100.0, 141.0, 142.0]),
                ("e.csv", [100.0, 151.0, 152.0]),
            ]
        };
        for (name, values) in datasets {
            let content = format!(
                "v1,v2,v3\n{},{},{}\n",
                values[0], values[1], values[2]
            );
            fs::write(dir.join(name), content).unwrap();
        }
        dir
    }

    fn opts(root: &Path) -> Options {
        Options {
            roots: vec![root.to_string_lossy().into_owned()],
            digits: vec![1, 2, 3],
            max_k: 4,
            targets: 10,
            tolerance: Tolerance::Exact,
            format: Format::Tsv,
            max_mb: DEFAULT_MAX_MB,
        }
    }

    fn row_at(report: &Report, digits: u32, k: usize) -> &Row {
        report
            .rows
            .iter()
            .find(|row| row.digits == digits && row.k == k)
            .unwrap()
    }

    #[test]
    fn higher_digits_lower_k_min() {
        let dir = write_fixture();
        let report = analyze(&opts(&dir)).unwrap();
        assert_eq!(report.datasets, 5);
        assert_eq!(report.target_count, 5);
        assert_eq!(row_at(&report, 1, 1).unique, 3);
        assert_eq!(row_at(&report, 1, 1).false_matches, 2);
        assert_eq!(row_at(&report, 1, 2).unique, 5);
        assert_eq!(row_at(&report, 1, 1).k_min_95, Some(2));
        assert_eq!(row_at(&report, 2, 1).k_min_95, Some(1));
        assert_eq!(row_at(&report, 3, 1).k_min_95, Some(1));
        assert!(
            row_at(&report, 1, 1).k_min_95.unwrap() > row_at(&report, 2, 1).k_min_95.unwrap()
        );
    }

    #[test]
    fn larger_gap_lowers_k_min() {
        let close_dir = write_spacing_corpus(false);
        let wide_dir = write_spacing_corpus(true);
        let mut close_opts = opts(&close_dir);
        close_opts.digits = vec![3];
        let mut wide_opts = opts(&wide_dir);
        wide_opts.digits = vec![3];
        let close = analyze(&close_opts).unwrap();
        let wide = analyze(&wide_opts).unwrap();
        assert_eq!(row_at(&close, 3, 1).k_min_95, Some(2));
        assert_eq!(row_at(&wide, 3, 1).k_min_95, Some(1));
        let close_gap = row_at(&close, 3, 2).gap_min.unwrap();
        let wide_gap = row_at(&wide, 3, 2).gap_min.unwrap();
        let close_median = row_at(&close, 3, 2).gap_median.unwrap();
        let wide_median = row_at(&wide, 3, 2).gap_median.unwrap();
        assert!(wide_gap > close_gap);
        assert!(wide_median > close_median);
        assert!(
            row_at(&wide, 3, 1).k_min_95.unwrap() < row_at(&close, 3, 1).k_min_95.unwrap()
        );
    }

    #[test]
    fn tolerance_raises_k_min() {
        let dir = write_fixture();
        let mut options = opts(&dir);
        options.digits = vec![3];
        options.tolerance = Tolerance::Absolute(2.0);
        let report = analyze(&options).unwrap();
        assert_eq!(row_at(&report, 3, 1).unique, 3);
        assert_eq!(row_at(&report, 3, 1).k_min_95, Some(2));
        options.tolerance = Tolerance::Percent(0.12);
        let report = analyze(&options).unwrap();
        assert_eq!(row_at(&report, 3, 1).k_min_95, Some(2));
    }

    #[test]
    fn json_is_parseable() {
        let dir = write_fixture();
        let mut options = opts(&dir);
        options.digits = vec![1, 3];
        options.max_k = 3;
        options.format = Format::Json;
        let report = analyze(&options).unwrap();
        let text = render_json(&report);
        let parsed = parse_json(&text).unwrap();
        let JsonVal::Obj(root) = parsed else {
            panic!("json root is not an object");
        };
        assert!(matches!(root.get("tool"), Some(JsonVal::Str(t)) if t == "bait_efficiency"));
        let Some(JsonVal::Arr(rows)) = root.get("rows") else {
            panic!("json rows missing");
        };
        assert_eq!(rows.len(), 6);
        for row in rows {
            let JsonVal::Obj(fields) = row else {
                panic!("row is not an object");
            };
            for key in [
                "digits",
                "k",
                "targets",
                "unique",
                "false_matches",
                "k_min_95",
                "k_min_99",
                "hit_rate",
                "gap_min",
                "gap_median",
            ] {
                assert!(fields.contains_key(key), "missing field {key}");
            }
        }
    }

    #[test]
    fn tsv_is_stable() {
        let dir = write_fixture();
        let options = opts(&dir);
        let report = analyze(&options).unwrap();
        let first = render_tsv(&report);
        let second = render_tsv(&report);
        assert_eq!(first, second);
        let lines: Vec<&str> = first.lines().collect();
        assert_eq!(
            lines[0],
            "digits\tk\ttargets\tunique\tfalse_matches\tk_min_95\tk_min_99\thit_rate\tgap_min\tgap_median"
        );
        assert_eq!(lines.len(), 1 + report.rows.len());
        for line in &lines[1..] {
            assert_eq!(line.split('\t').count(), 10);
        }
    }

    #[test]
    fn empty_corpus_reports_pending() {
        let dir = fixture_dir();
        let mut options = opts(&dir);
        options.digits = vec![1, 4];
        options.max_k = 2;
        let report = analyze(&options).unwrap();
        assert_eq!(report.datasets, 0);
        assert_eq!(report.target_count, 0);
        assert_eq!(report.rows.len(), 4);
        assert!(report.rows.iter().all(|row| row.hit_rate.is_none()));
        let text = render_tsv(&report);
        assert!(text.contains("pending"));
        assert!(!text.contains("\t0\t0\t0\t0.0000"));
    }

    #[test]
    fn cli_parses_and_rejects() {
        let args: Vec<String> = [
            "--root",
            "/tmp/corpus",
            "--digits",
            "3,1,3",
            "--max-k",
            "5",
            "--targets",
            "7",
            "--tolerance",
            "5%",
            "--format",
            "json",
        ]
        .iter()
        .map(|value| value.to_string())
        .collect();
        let Ok(Parsed::Run(options)) = parse_args(&args) else {
            panic!("cli did not parse");
        };
        assert_eq!(options.digits, vec![1, 3]);
        assert_eq!(options.max_k, 5);
        assert_eq!(options.targets, 7);
        assert!(matches!(options.tolerance, Tolerance::Percent(value) if (value - 0.05).abs() < 1e-12));
        assert!(matches!(options.format, Format::Json));
        assert!(parse_args(&["--root".to_string()]).is_err());
        assert!(parse_args(&["--root".to_string(), "/x".to_string(), "--max-k".to_string(), "0".to_string()]).is_err());
        assert!(parse_args(&["--root".to_string(), "/x".to_string(), "--tolerance".to_string(), "-1".to_string()]).is_err());
        assert!(parse_args(&["--root".to_string(), "/x".to_string(), "--digits".to_string(), "16".to_string()]).is_err());
        assert!(parse_args(&["--root".to_string(), "/x".to_string(), "--resolutions".to_string(), "1".to_string()]).is_err());
    }
}
