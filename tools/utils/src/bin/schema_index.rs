use omegaflow::archivar::sniff::sniff;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

const DEFAULT_MIN_FILES: usize = 2;
const DEFAULT_MAX_MB: u64 = 32;
const SKIP_DIRS: &[&str] = &[".git", "node_modules"];
const GENERIC_COLUMNS: &[&str] = &["time", "date", "value", "id", "lat", "lon"];
const MATCH_FLOOR: f64 = 0.5;
const GENERIC_WEIGHT: f64 = 0.2;
const RARE_WEIGHT: f64 = 1.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Format {
    Tsv,
    Json,
}

struct Options {
    roots: Vec<String>,
    min_files: usize,
    format: Format,
    signatures: bool,
    max_mb: u64,
    match_file: Option<String>,
}

enum Parsed {
    Run(Options),
    Help,
}

struct FileColumns {
    display: String,
    format: String,
    order: Vec<String>,
    columns: BTreeSet<String>,
}

struct ColumnStat {
    files: usize,
    formats: BTreeSet<String>,
    first_file: String,
}

struct SignatureStat {
    files: usize,
    columns: usize,
    first_file: String,
    variants: BTreeMap<String, usize>,
}

struct MatchRow {
    file: String,
    score: f64,
    shared: Vec<String>,
    missing: Vec<String>,
    extra: Vec<String>,
    order_equal: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = match parse_args(&args) {
        Ok(Parsed::Help) => {
            usage();
            std::process::exit(0);
        }
        Ok(Parsed::Run(o)) => o,
        Err(e) => {
            eprintln!("schema_index: {e}");
            usage();
            std::process::exit(2);
        }
    };
    match run(&opts) {
        Ok(text) => print!("{text}"),
        Err(e) => {
            eprintln!("schema_index: {e}");
            std::process::exit(2);
        }
    }
}

fn parse_args(args: &[String]) -> Result<Parsed, String> {
    let mut roots: Vec<String> = Vec::new();
    let mut min_files = DEFAULT_MIN_FILES;
    let mut format = Format::Tsv;
    let mut signatures = false;
    let mut max_mb = DEFAULT_MAX_MB;
    let mut match_file: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        let a = args[i].as_str();
        match a {
            "--root" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    return Err("--root needs a <dir> value".to_string());
                };
                if v.is_empty() {
                    return Err("--root needs a non-empty <dir> value".to_string());
                }
                roots.push(v.clone());
            }
            "--min-files" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    return Err("--min-files needs a number".to_string());
                };
                let n = v
                    .parse::<usize>()
                    .map_err(|_| format!("--min-files is not a number: {v}"))?;
                if n == 0 {
                    return Err("--min-files needs a value >= 1".to_string());
                }
                min_files = n;
            }
            "--max-mb" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    return Err("--max-mb needs a number".to_string());
                };
                let n = v
                    .parse::<u64>()
                    .map_err(|_| format!("--max-mb is not a number: {v}"))?;
                if n == 0 {
                    return Err("--max-mb needs a value >= 1".to_string());
                }
                max_mb = n;
            }
            "--format" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    return Err("--format needs tsv or json".to_string());
                };
                format = match v.as_str() {
                    "tsv" => Format::Tsv,
                    "json" => Format::Json,
                    _ => return Err(format!("--format is neither tsv nor json: {v}")),
                };
            }
            "--signatures" => signatures = true,
            "--match" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    return Err("--match needs a <datei> value".to_string());
                };
                if v.is_empty() {
                    return Err("--match needs a non-empty <datei> value".to_string());
                }
                if match_file.is_some() {
                    return Err("--match may only be given once".to_string());
                }
                match_file = Some(v.clone());
            }
            "-h" | "--help" => return Ok(Parsed::Help),
            _ if a.starts_with('-') && a.len() > 1 => {
                return Err(format!("unknown flag: {a}"));
            }
            _ => return Err(format!("unexpected argument: {a}")),
        }
        i += 1;
    }
    if signatures && match_file.is_some() {
        return Err("--signatures and --match are mutually exclusive".to_string());
    }
    if roots.is_empty() {
        roots.push(".".to_string());
    }
    Ok(Parsed::Run(Options {
        roots,
        min_files,
        format,
        signatures,
        max_mb,
        match_file,
    }))
}

fn usage() {
    eprintln!("schema_index — recurring columns across a corpus");
    eprintln!(
        "usage: schema_index [--root <dir>]... [--min-files <n>] [--format tsv|json] [--signatures] [--match <file>] [--max-mb <n>]"
    );
    eprintln!("flags:");
    eprintln!(
        "  --root <dir>     repeatable; directory or file; without --root, \".\" (working directory, not $HOME)"
    );
    eprintln!(
        "  --min-files <n>  columns and signatures present in at least n files (default {DEFAULT_MIN_FILES}); ignored by --match"
    );
    eprintln!("  --format <fmt>   tsv (default) or json");
    eprintln!(
        "  --signatures     signature frequency instead of the column index: sorted unique column set per file"
    );
    eprintln!(
        "  --match <file>   sniff a file and compare it with the corpus; candidates at score {MATCH_FLOOR}; excludes --signatures"
    );
    eprintln!(
        "  --max-mb <n>     do not read files larger than n MiB (default {DEFAULT_MAX_MB})"
    );
    eprintln!("  -h|--help        this help");
    eprintln!("output:");
    eprintln!("  tsv: column, files, formats, first_file — sort files desc, column asc");
    eprintln!("  json: array of {{\"column\",\"files\",\"formats\",\"first_file\"}}");
    eprintln!(
        "  signatures tsv: signature, files, first_file, order_variants, variants — sort files desc, signature asc; variants is a JSON list of observed column orders"
    );
    eprintln!(
        "  signatures json: array of {{\"signature\",\"columns\",\"files\",\"first_file\",\"order_variants\",\"variants\"}}"
    );
    eprintln!(
        "  match tsv: file, score, shared, missing, extra, order_equal — sort score desc, file asc"
    );
    eprintln!(
        "  match json: array of {{\"file\",\"score\",\"shared\",\"missing\",\"extra\",\"order_equal\"}}; shared/missing/extra are lists, empty lists stay empty, order_equal is the exact column order"
    );
    eprintln!(
        "  score is weighted Jaccard similarity: generic columns (time, date, value, id, lat, lon) weigh {GENERIC_WEIGHT}, all others {RARE_WEIGHT}. A score is similarity, not proof and not exactness."
    );
    eprintln!(
        "  columns are trimmed and lowercased; _eff/_err are kept. first_file is the smallest path formed from --root (relative root -> relative paths)."
    );
    eprintln!(
        "  only text files with a recognized header are read; .git and node_modules are skipped."
    );
    eprintln!(
        "  files starting with {{ or [ (JSON) carry no columns."
    );
}

fn run(opts: &Options) -> Result<String, String> {
    let files = collect_files(&opts.roots)?;
    let cap = opts.max_mb.saturating_mul(1_048_576);
    let mut seen: Vec<FileColumns> = Vec::new();
    for (display, path) in files {
        let Some(bytes) = read_bounded(&path, cap) else {
            continue;
        };
        let Some((format, order)) = columns_of(&bytes) else {
            continue;
        };
        let columns: BTreeSet<String> = order.iter().cloned().collect();
        seen.push(FileColumns {
            display,
            format,
            order,
            columns,
        });
    }
    if let Some(match_file) = &opts.match_file {
        return render_match(match_file, &seen, opts.max_mb, opts.format);
    }
    if opts.signatures {
        Ok(render_signatures(&seen, opts.min_files, opts.format))
    } else {
        Ok(render_columns(&seen, opts.min_files, opts.format))
    }
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
        for p in found {
            let key = match std::fs::canonicalize(&p) {
                Ok(key) => key,
                Err(_) => p.clone(),
            };
            if !seen.insert(key) {
                continue;
            }
            files.push((p.to_string_lossy().into_owned(), p));
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
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        let path = entry.path();
        if ft.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if SKIP_DIRS.contains(&name.as_ref()) {
                continue;
            }
            walk(&path, out);
        } else if ft.is_file() {
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

fn columns_of(bytes: &[u8]) -> Option<(String, Vec<String>)> {
    if matches!(
        bytes.iter().find(|b| !b.is_ascii_whitespace()),
        Some(b'{') | Some(b'[')
    ) {
        return None;
    }
    let sniffed = sniff(bytes);
    let header = sniffed.header?;
    let mut order: Vec<String> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for name in header {
        let normalized = name.trim().to_lowercase();
        if !normalized.is_empty() && seen.insert(normalized.clone()) {
            order.push(normalized);
        }
    }
    if order.is_empty() {
        return None;
    }
    Some((sniffed.format, order))
}

fn render_columns(files: &[FileColumns], min_files: usize, format: Format) -> String {
    let mut stats: BTreeMap<String, ColumnStat> = BTreeMap::new();
    for file in files {
        for column in &file.columns {
            let stat = stats.entry(column.clone()).or_insert_with(|| ColumnStat {
                files: 0,
                formats: BTreeSet::new(),
                first_file: file.display.clone(),
            });
            stat.files += 1;
            stat.formats.insert(file.format.clone());
        }
    }
    let mut rows: Vec<(&String, &ColumnStat)> = stats
        .iter()
        .filter(|(_, stat)| stat.files >= min_files)
        .collect();
    rows.sort_by(|a, b| b.1.files.cmp(&a.1.files).then(a.0.cmp(b.0)));
    match format {
        Format::Tsv => {
            let mut out = String::from("column\tfiles\tformats\tfirst_file\n");
            for (column, stat) in rows {
                let formats = stat
                    .formats
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(",");
                out.push_str(&format!(
                    "{}\t{}\t{}\t{}\n",
                    column, stat.files, formats, stat.first_file
                ));
            }
            out
        }
        Format::Json => {
            let mut out = String::from("[\n");
            for (i, (column, stat)) in rows.iter().enumerate() {
                let formats = stat
                    .formats
                    .iter()
                    .map(|f| json_string(f))
                    .collect::<Vec<String>>()
                    .join(",");
                out.push_str(&format!(
                    "  {{\"column\":{},\"files\":{},\"formats\":[{}],\"first_file\":{}}}",
                    json_string(column),
                    stat.files,
                    formats,
                    json_string(&stat.first_file)
                ));
                if i + 1 < rows.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str("]\n");
            out
        }
    }
}

fn render_signatures(files: &[FileColumns], min_files: usize, format: Format) -> String {
    let mut stats: BTreeMap<String, SignatureStat> = BTreeMap::new();
    for file in files {
        let signature = file
            .columns
            .iter()
            .cloned()
            .collect::<Vec<String>>()
            .join(",");
        let stat = stats.entry(signature).or_insert_with(|| SignatureStat {
            files: 0,
            columns: file.columns.len(),
            first_file: file.display.clone(),
            variants: BTreeMap::new(),
        });
        stat.files += 1;
        *stat.variants.entry(file.order.join(",")).or_insert(0) += 1;
    }
    let mut rows: Vec<(&String, &SignatureStat)> = stats
        .iter()
        .filter(|(_, stat)| stat.files >= min_files)
        .collect();
    rows.sort_by(|a, b| b.1.files.cmp(&a.1.files).then(a.0.cmp(b.0)));
    match format {
        Format::Tsv => {
            let mut out = String::from("signature\tfiles\tfirst_file\torder_variants\tvariants\n");
            for (signature, stat) in rows {
                out.push_str(&format!(
                    "{}\t{}\t{}\t{}\t{}\n",
                    signature,
                    stat.files,
                    stat.first_file,
                    stat.variants.len(),
                    json_string_list(&variant_strings(stat))
                ));
            }
            out
        }
        Format::Json => {
            let mut out = String::from("[\n");
            for (i, (signature, stat)) in rows.iter().enumerate() {
                out.push_str(&format!(
                    "  {{\"signature\":{},\"columns\":{},\"files\":{},\"first_file\":{},\"order_variants\":{},\"variants\":{}}}",
                    json_string(signature),
                    stat.columns,
                    stat.files,
                    json_string(&stat.first_file),
                    stat.variants.len(),
                    json_string_list(&variant_strings(stat))
                ));
                if i + 1 < rows.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str("]\n");
            out
        }
    }
}

fn variant_strings(stat: &SignatureStat) -> Vec<String> {
    let mut variants: Vec<(&String, &usize)> = stat.variants.iter().collect();
    variants.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    variants
        .into_iter()
        .map(|(order, _)| order.clone())
        .collect()
}

fn render_match(
    match_file: &str,
    files: &[FileColumns],
    max_mb: u64,
    format: Format,
) -> Result<String, String> {
    let path = PathBuf::from(match_file);
    if !path.is_file() {
        return Err(format!("--match file not found: {match_file}"));
    }
    let cap = max_mb.saturating_mul(1_048_576);
    let Some(bytes) = read_bounded(&path, cap) else {
        return Err(format!(
            "pending — --match file unreadable or larger than --max-mb {max_mb}: {match_file}"
        ));
    };
    let Some((_, match_order)) = columns_of(&bytes) else {
        return Err(format!(
            "pending — no columns in --match file (header absent): {match_file}"
        ));
    };
    let match_columns: BTreeSet<String> = match_order.iter().cloned().collect();
    let match_key = match std::fs::canonicalize(&path) {
        Ok(key) => key,
        Err(_) => path.clone(),
    };
    let mut rows: Vec<MatchRow> = Vec::new();
    for file in files {
        let candidate_key = match std::fs::canonicalize(&file.display) {
            Ok(key) => key,
            Err(_) => PathBuf::from(&file.display),
        };
        if candidate_key == match_key {
            continue;
        }
        let shared: Vec<String> = match_columns
            .intersection(&file.columns)
            .cloned()
            .collect();
        let missing: Vec<String> = match_columns
            .difference(&file.columns)
            .cloned()
            .collect();
        let extra: Vec<String> = file
            .columns
            .difference(&match_columns)
            .cloned()
            .collect();
        let score = weighted_jaccard(&match_columns, &file.columns);
        if score < MATCH_FLOOR {
            continue;
        }
        rows.push(MatchRow {
            file: file.display.clone(),
            score,
            shared,
            missing,
            extra,
            order_equal: match_order == file.order,
        });
    }
    rows.sort_by(|a, b| b.score.total_cmp(&a.score).then(a.file.cmp(&b.file)));
    Ok(match format {
        Format::Tsv => {
            let mut out = String::from("file\tscore\tshared\tmissing\textra\torder_equal\n");
            for row in rows {
                out.push_str(&format!(
                    "{}\t{:.3}\t{}\t{}\t{}\t{}\n",
                    row.file,
                    row.score,
                    join_or_dash(&row.shared),
                    join_or_dash(&row.missing),
                    join_or_dash(&row.extra),
                    row.order_equal
                ));
            }
            out
        }
        Format::Json => {
            let mut out = String::from("[\n");
            for (i, row) in rows.iter().enumerate() {
                out.push_str(&format!(
                    "  {{\"file\":{},\"score\":{:.3},\"shared\":{},\"missing\":{},\"extra\":{},\"order_equal\":{}}}",
                    json_string(&row.file),
                    row.score,
                    json_string_list(&row.shared),
                    json_string_list(&row.missing),
                    json_string_list(&row.extra),
                    row.order_equal
                ));
                if i + 1 < rows.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str("]\n");
            out
        }
    })
}

fn weight(column: &str) -> f64 {
    if GENERIC_COLUMNS.contains(&column) {
        GENERIC_WEIGHT
    } else {
        RARE_WEIGHT
    }
}

fn weighted_jaccard(a: &BTreeSet<String>, b: &BTreeSet<String>) -> f64 {
    let mut intersection = 0.0;
    let mut union = 0.0;
    for column in a.union(b) {
        let w = weight(column);
        union += w;
        if a.contains(column) && b.contains(column) {
            intersection += w;
        }
    }
    if union == 0.0 { 0.0 } else { intersection / union }
}

fn join_or_dash(columns: &[String]) -> String {
    if columns.is_empty() {
        "-".to_string()
    } else {
        columns.join(",")
    }
}

fn json_string_list(values: &[String]) -> String {
    let items: Vec<String> = values.iter().map(|v| json_string(v)).collect();
    format!("[{}]", items.join(","))
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

    fn argv(parts: &[&str]) -> Vec<String> {
        std::iter::once("schema_index")
            .chain(parts.iter().copied())
            .map(String::from)
            .collect()
    }

    fn run_args(parts: &[&str]) -> Options {
        match parse_args(&argv(parts)) {
            Ok(Parsed::Run(o)) => o,
            Ok(Parsed::Help) => panic!("Help"),
            Err(e) => panic!("the args were rejected: {e}"),
        }
    }

    fn temp_root(tag: &str) -> PathBuf {
        let base = env::temp_dir().join(format!("schema_index_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).expect("temp root");
        base
    }

    fn write(path: &Path, text: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("parent dir");
        }
        std::fs::write(path, text).expect("fixture write");
    }

    fn tsv_rows(text: &str) -> Vec<Vec<String>> {
        text.lines()
            .map(|l| l.split('\t').map(String::from).collect())
            .collect()
    }

    #[test]
    fn unknown_flag_is_rejected() {
        assert!(parse_args(&argv(&["--top", "3"])).is_err());
        assert!(parse_args(&argv(&["--root"])).is_err());
        assert!(parse_args(&argv(&["sources"])).is_err());
    }

    #[test]
    fn defaults_are_documented_and_explicit() {
        let o = run_args(&[]);
        assert_eq!(o.roots, vec![".".to_string()]);
        assert_eq!(o.min_files, 2);
        assert_eq!(o.max_mb, 32);
        assert!(matches!(o.format, Format::Tsv));
        assert!(!o.signatures);
    }

    #[test]
    fn repeated_roots_and_format_parse() {
        let o = run_args(&[
            "--root", "a", "--root", "b", "--min-files", "3", "--format", "json", "--signatures",
            "--max-mb", "8",
        ]);
        assert_eq!(o.roots, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(o.min_files, 3);
        assert_eq!(o.max_mb, 8);
        assert_eq!(o.format, Format::Json);
        assert!(o.signatures);
    }

    #[test]
    fn zero_thresholds_are_rejected() {
        assert!(parse_args(&argv(&["--min-files", "0"])).is_err());
        assert!(parse_args(&argv(&["--max-mb", "0"])).is_err());
        assert!(parse_args(&argv(&["--format", "xml"])).is_err());
    }

    #[test]
    fn csv_columns_aggregate_and_normalize() {
        let base = temp_root("csv");
        write(
            &base.join("a.csv"),
            "Time;Value_eff\n2020-01-01;1.5\n2020-01-02;2.5\n",
        );
        write(
            &base.join("sub/b.csv"),
            " time ,VALUE_err\n0,1.0\n1,2.0\n",
        );
        write(
            &base.join("sub/c.csv"),
            "TIME,value_err,extra\n0,1.0,7\n1,2.0,8\n",
        );
        let o = run_args(&["--root", &base.to_string_lossy(), "--min-files", "1"]);
        let text = run(&o).expect("run");
        let rows = tsv_rows(&text);
        assert_eq!(rows[0], vec!["column", "files", "formats", "first_file"]);
        let find = |name: &str| rows.iter().find(|r| r[0] == name).cloned();
        let time = find("time").expect("time column");
        assert_eq!(time[1], "3");
        assert_eq!(time[2], "csv");
        let value_eff = find("value_eff").expect("value_eff kept");
        assert_eq!(value_eff[1], "1");
        let value_err = find("value_err").expect("value_err kept");
        assert_eq!(value_err[1], "2");
        let extra = find("extra").expect("extra");
        assert_eq!(extra[1], "1");
        let ordered: Vec<&str> = rows[1..].iter().map(|r| r[0].as_str()).collect();
        assert_eq!(ordered, vec!["time", "value_err", "extra", "value_eff"]);
        std::fs::remove_dir_all(&base).expect("cleanup");
    }

    #[test]
    fn fixed_width_header_is_indexed() {
        let base = temp_root("fixed");
        write(
            &base.join("one.dat"),
            "FREQ1 FREQ2 FLAG\n1.0 2.0 3\n2.0 3.0 4\n",
        );
        write(
            &base.join("two.dat"),
            "FREQ1 FREQ2 FLAG\n3.0 4.0 5\n4.0 5.0 6\n",
        );
        let o = run_args(&["--root", &base.to_string_lossy()]);
        let text = run(&o).expect("run");
        let rows = tsv_rows(&text);
        let freq1 = rows.iter().find(|r| r[0] == "freq1").expect("freq1");
        assert_eq!(freq1[1], "2");
        assert_eq!(freq1[2], "fixed");
        assert_eq!(rows.len(), 4);
        std::fs::remove_dir_all(&base).expect("cleanup");
    }

    #[test]
    fn min_files_filters_and_headerless_is_ignored() {
        let base = temp_root("minfiles");
        write(&base.join("a.csv"), "time,value\n0,1\n1,2\n");
        write(&base.join("b.csv"), "time,other\n0,1\n1,2\n");
        write(&base.join("c.csv"), "plain prose line\nanother prose line\n");
        let o = run_args(&["--root", &base.to_string_lossy(), "--min-files", "2"]);
        let rows = tsv_rows(&run(&o).expect("run"));
        assert_eq!(
            rows[1..].iter().map(|r| r[0].as_str()).collect::<Vec<_>>(),
            vec!["time"]
        );
        let o = run_args(&["--root", &base.to_string_lossy(), "--min-files", "3"]);
        let rows = tsv_rows(&run(&o).expect("run"));
        assert_eq!(rows.len(), 1);
        std::fs::remove_dir_all(&base).expect("cleanup");
    }

    #[test]
    fn json_documents_are_ignored() {
        let base = temp_root("json_doc");
        write(
            &base.join("a.json"),
            "{\n  \"format\": \"json\",\n  \"params\": {\"dataset\": \"oe417\"},\n  \"n\": 5\n}\n",
        );
        write(&base.join("b.csv"), "time,value\n0,1\n1,2\n");
        let o = run_args(&["--root", &base.to_string_lossy(), "--min-files", "1"]);
        let rows = tsv_rows(&run(&o).expect("run"));
        assert_eq!(
            rows[1..].iter().map(|r| r[0].as_str()).collect::<Vec<_>>(),
            vec!["time", "value"]
        );
        std::fs::remove_dir_all(&base).expect("cleanup");
    }

    #[test]
    fn json_is_parseable_and_ordered() {
        let base = temp_root("json");
        write(&base.join("a.csv"), "time,value\n0,1\n1,2\n");
        write(&base.join("b.csv"), "value,time\n0,1\n1,2\n");
        let o = run_args(&["--root", &base.to_string_lossy(), "--format", "json"]);
        let text = run(&o).expect("run");
        let expected = format!(
            "[\n  {{\"column\":\"time\",\"files\":2,\"formats\":[\"csv\"],\"first_file\":{}}},\n  {{\"column\":\"value\",\"files\":2,\"formats\":[\"csv\"],\"first_file\":{}}}\n]\n",
            json_string(&format!("{}/a.csv", base.to_string_lossy())),
            json_string(&format!("{}/a.csv", base.to_string_lossy()))
        );
        assert_eq!(text, expected);
        assert!(text.starts_with("[\n"));
        assert!(text.trim_end().ends_with("]"));
        std::fs::remove_dir_all(&base).expect("cleanup");
    }

    #[test]
    fn signatures_group_and_sort_by_frequency() {
        let base = temp_root("sigs");
        write(&base.join("a.csv"), "Time,Value\n0,1\n1,2\n");
        write(&base.join("b.csv"), "value,time\n0,1\n1,2\n");
        write(&base.join("c.csv"), "time,value,extra\n0,1,2\n1,2,3\n");
        let o = run_args(&[
            "--root",
            &base.to_string_lossy(),
            "--signatures",
            "--min-files",
            "1",
        ]);
        let rows = tsv_rows(&run(&o).expect("run"));
        assert_eq!(
            rows[0],
            vec![
                "signature",
                "files",
                "first_file",
                "order_variants",
                "variants"
            ]
        );
        assert_eq!(rows[1][0], "time,value");
        assert_eq!(rows[1][1], "2");
        assert_eq!(rows[1][3], "2");
        assert_eq!(rows[1][4], "[\"time,value\",\"value,time\"]");
        assert_eq!(rows[2][0], "extra,time,value");
        assert_eq!(rows[2][1], "1");
        assert_eq!(rows[2][3], "1");
        assert_eq!(rows[2][4], "[\"time,value,extra\"]");
        assert_eq!(rows.len(), 3);
        let o = run_args(&[
            "--root",
            &base.to_string_lossy(),
            "--signatures",
            "--min-files",
            "2",
        ]);
        let rows = tsv_rows(&run(&o).expect("run"));
        assert_eq!(rows.len(), 2);
        std::fs::remove_dir_all(&base).expect("cleanup");
    }

    #[test]
    fn match_flag_parses_and_is_exclusive_with_signatures() {
        let o = run_args(&["--match", "probe.csv", "--root", "sources"]);
        assert_eq!(o.match_file, Some("probe.csv".to_string()));
        assert!(parse_args(&argv(&["--match"])).is_err());
        assert!(parse_args(&argv(&["--match", "a", "--match", "b"])).is_err());
        assert!(parse_args(&argv(&["--match", "a", "--signatures"])).is_err());
    }

    #[test]
    fn match_finds_reordered_twin_with_missing_generic_columns() {
        let base = temp_root("match");
        write(
            &base.join("twin.csv"),
            "temp,flux,time\n1.0,2.0,3.0\n2.0,3.0,4.0\n",
        );
        write(
            &base.join("other.csv"),
            "alpha,beta,gamma\n1,2,3\n4,5,6\n",
        );
        let probe = base.join("probe.csv");
        write(
            &probe,
            "time,date,value,temp,flux\n0,2020-01-01,1,2,3\n1,2020-01-02,2,3,4\n",
        );
        let o = run_args(&[
            "--root",
            &base.to_string_lossy(),
            "--match",
            &probe.to_string_lossy(),
        ]);
        let text = run(&o).expect("run");
        let rows = tsv_rows(&text);
        assert_eq!(
            rows[0],
            vec!["file", "score", "shared", "missing", "extra", "order_equal"]
        );
        assert_eq!(rows.len(), 2);
        assert!(rows[1][0].ends_with("twin.csv"));
        let score: f64 = rows[1][1].parse().expect("score number");
        assert!(score >= 0.5);
        assert!(score < 1.0);
        assert_eq!(rows[1][2], "flux,temp,time");
        assert_eq!(rows[1][3], "date,value");
        assert_eq!(rows[1][4], "-");
        assert_eq!(rows[1][5], "false");
        assert!(!text.to_lowercase().contains("exact"));
        std::fs::remove_dir_all(&base).expect("cleanup");
    }

    #[test]
    fn match_without_columns_is_pending_not_zero() {
        let base = temp_root("match_empty");
        write(&base.join("twin.csv"), "time,value\n0,1\n1,2\n");
        let probe = base.join("probe.txt");
        write(&probe, "plain prose line\nanother prose line\n");
        let o = run_args(&[
            "--root",
            &base.to_string_lossy(),
            "--match",
            &probe.to_string_lossy(),
        ]);
        let err = run(&o).expect_err("must not produce a zero table");
        assert!(err.contains("pending"));
        assert!(err.contains("no columns"));
        assert!(!err.contains(": 0"));
        std::fs::remove_dir_all(&base).expect("cleanup");
    }

    #[test]
    fn match_json_keeps_arrays_and_numeric_score() {
        let base = temp_root("match_json");
        write(&base.join("twin.csv"), "temp,flux,time\n1.0,2.0,3.0\n2.0,3.0,4.0\n");
        let probe = base.join("probe.csv");
        write(
            &probe,
            "time,date,value,temp,flux\n0,2020-01-01,1,2,3\n1,2020-01-02,2,3,4\n",
        );
        let o = run_args(&[
            "--root",
            &base.to_string_lossy(),
            "--match",
            &probe.to_string_lossy(),
            "--format",
            "json",
        ]);
        let text = run(&o).expect("run");
        assert!(text.starts_with("[\n"));
        assert!(text.contains("\"score\":0.846"));
        assert!(text.contains("\"shared\":[\"flux\",\"temp\",\"time\"]"));
        assert!(text.contains("\"missing\":[\"date\",\"value\"]"));
        assert!(text.contains("\"extra\":[]"));
        assert!(text.contains("\"order_equal\":false"));
        std::fs::remove_dir_all(&base).expect("cleanup");
    }

    #[test]
    fn repeated_roots_do_not_double_count() {
        let base = temp_root("dedupe");
        write(&base.join("a.csv"), "time,value\n0,1\n1,2\n");
        let root = base.to_string_lossy().into_owned();
        let o = run_args(&["--root", &root, "--root", &root, "--min-files", "1"]);
        let rows = tsv_rows(&run(&o).expect("run"));
        let time = rows.iter().find(|r| r[0] == "time").expect("time");
        assert_eq!(time[1], "1");
        std::fs::remove_dir_all(&base).expect("cleanup");
    }

    #[test]
    fn max_mb_skips_large_files() {
        let base = temp_root("maxmb");
        write(&base.join("small.csv"), "time,value\n0,1\n1,2\n");
        let mut big = String::from("time,value\n0,1\n1,2\n");
        while big.len() < 1_100_000 {
            big.push(' ');
        }
        write(&base.join("big.csv"), &big);
        let o = run_args(&[
            "--root",
            &base.to_string_lossy(),
            "--min-files",
            "1",
            "--max-mb",
            "1",
        ]);
        let text = run(&o).expect("run");
        assert!(text.contains("small.csv"));
        assert!(!text.contains("big.csv"));
        let o = run_args(&[
            "--root",
            &base.to_string_lossy(),
            "--min-files",
            "1",
            "--max-mb",
            "2",
        ]);
        let text = run(&o).expect("run");
        assert!(text.contains("big.csv"));
        std::fs::remove_dir_all(&base).expect("cleanup");
    }

    #[test]
    fn missing_root_is_an_error() {
        let o = run_args(&["--root", "/nonexistent/schema_index_root"]);
        assert!(run(&o).is_err());
    }
}
