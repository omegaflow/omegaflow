use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use omegaflow::archivar::trisp_nse::{self, TrispNseTable};

const DEFAULT_SHARE_KEYS: [&str; 2] = ["Gamma", "dGamma"];

struct Run {
    source: String,
    table: TrispNseTable,
}

struct Args {
    dir: PathBuf,
    out: Option<PathBuf>,
    list: bool,
    share_temperature: bool,
    share_keys: Vec<String>,
}

fn usage() {
    eprintln!(
        "usage: trisp_export --dir <path> --out <path> [--share-temperature] [--share-log KEY]...\n       trisp_export --dir <path> --list\n\nfull_runs.csv and full_log.csv stay local. share.csv carries no source name and only the selected log keys; temperature is omitted unless --share-temperature is set."
    );
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_args() -> Option<Args> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return None;
    }
    let dir = match arg_value(&args, "--dir") {
        Some(d) => PathBuf::from(d),
        None => {
            usage();
            return None;
        }
    };
    let list = args.iter().any(|a| a == "--list");
    let out = arg_value(&args, "--out").map(PathBuf::from);
    if !list && out.is_none() {
        usage();
        return None;
    }
    let mut share_keys = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        if arg == "--share-log"
            && let Some(key) = args.get(i + 1)
        {
            share_keys.push(key.clone());
        }
    }
    if share_keys.is_empty() {
        share_keys = DEFAULT_SHARE_KEYS.iter().map(|s| s.to_string()).collect();
    }
    Some(Args {
        dir,
        out,
        list,
        share_temperature: args.iter().any(|a| a == "--share-temperature"),
        share_keys,
    })
}

fn source_name(path: &Path) -> Option<String> {
    path.file_name().map(|s| s.to_string_lossy().into_owned())
}

fn attach_log(path: &Path, table: &mut TrispNseTable) {
    let log_path = path.with_extension("log");
    if let Ok(log_bytes) = fs::read(&log_path)
        && let Ok(log_text) = String::from_utf8(log_bytes)
    {
        table.log = trisp_nse::parse_log(&log_text);
    }
}

fn read_table(path: &Path) -> Option<TrispNseTable> {
    let bytes = fs::read(path).ok()?;
    match path.extension().and_then(|e| e.to_str()) {
        Some("bin") => {
            let mut table = trisp_nse::parse_bin(&bytes)?;
            if table.log.is_empty() {
                attach_log(path, &mut table);
            }
            Some(table)
        }
        Some("dat") => {
            let text = String::from_utf8(bytes).ok()?;
            let mut table = trisp_nse::parse_dat(&text)?;
            attach_log(path, &mut table);
            Some(table)
        }
        _ => None,
    }
}

fn stem(path: &Path) -> Option<String> {
    path.file_stem().map(|s| s.to_string_lossy().into_owned())
}

fn collect_runs(dir: &Path) -> (Vec<Run>, usize) {
    let Ok(entries) = fs::read_dir(dir) else {
        return (Vec::new(), 0);
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    let mut runs = Vec::new();
    let mut skipped = 0usize;
    let mut done_stems = HashSet::new();
    for path in &paths {
        if path.extension().and_then(|e| e.to_str()) != Some("bin") {
            continue;
        }
        let Some(table) = read_table(path) else {
            skipped += 1;
            continue;
        };
        let Some(source) = source_name(path) else {
            skipped += 1;
            continue;
        };
        if let Some(stem) = stem(path) {
            done_stems.insert(stem);
        }
        runs.push(Run { source, table });
    }
    for path in &paths {
        if path.extension().and_then(|e| e.to_str()) != Some("dat") {
            continue;
        }
        let Some(stem) = stem(path) else {
            skipped += 1;
            continue;
        };
        if done_stems.contains(&stem) {
            continue;
        }
        let Some(table) = read_table(path) else {
            skipped += 1;
            continue;
        };
        let Some(source) = source_name(path) else {
            skipped += 1;
            continue;
        };
        runs.push(Run { source, table });
    }
    (runs, skipped)
}

fn finite_column(table: &TrispNseTable, name: &str) -> Vec<f64> {
    match table.column(name) {
        Some(v) => v.into_iter().flatten().filter(|x| x.is_finite()).collect(),
        None => Vec::new(),
    }
}

fn median(mut values: Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.total_cmp(b));
    let mid = values.len() / 2;
    if values.len().is_multiple_of(2) {
        Some((values[mid - 1] + values[mid]) / 2.0)
    } else {
        Some(values[mid])
    }
}

fn temperature(table: &TrispNseTable) -> (Option<f64>, &'static str) {
    if let Some(t) = table.log_value("TTAMEAN").filter(|x| x.is_finite()) {
        return (Some(t), "log:TTAMEAN");
    }
    match median(finite_column(table, "TTA")) {
        Some(t) => (Some(t), "column:TTA"),
        None => (None, "absent"),
    }
}

fn tau_summary(table: &TrispNseTable) -> (usize, Option<f64>, Option<f64>) {
    let values = finite_column(table, "tau");
    if values.is_empty() {
        return (0, None, None);
    }
    let lo = values.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (values.len(), Some(lo), Some(hi))
}

fn log_value(table: &TrispNseTable, key: &str) -> Option<f64> {
    table
        .log
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(key))
        .and_then(|(_, v)| *v)
        .filter(|x| x.is_finite())
}

fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn num(value: Option<f64>) -> String {
    match value {
        Some(x) => format!("{x:.6e}"),
        None => "absent".to_string(),
    }
}

fn temp_num(value: Option<f64>) -> String {
    match value {
        Some(x) => format!("{x:.6}"),
        None => "absent".to_string(),
    }
}

fn full_run_row(run: &Run) -> String {
    let (t, from) = temperature(&run.table);
    let (n_tau, tau_lo, tau_hi) = tau_summary(&run.table);
    [
        csv_field(&run.source),
        temp_num(t),
        from.to_string(),
        run.table.rows.len().to_string(),
        n_tau.to_string(),
        num(tau_lo),
        num(tau_hi),
    ]
    .join(",")
}

fn share_row(run: &Run, id: usize, keys: &[String], with_temperature: bool) -> String {
    let (t, _) = temperature(&run.table);
    let (n_tau, tau_lo, tau_hi) = tau_summary(&run.table);
    let mut fields = vec![format!("run_{id:03}")];
    if with_temperature {
        fields.push(temp_num(t));
    }
    for key in keys {
        fields.push(num(log_value(&run.table, key)));
    }
    fields.push(n_tau.to_string());
    fields.push(num(tau_lo));
    fields.push(num(tau_hi));
    fields.join(",")
}

fn write_full(runs: &[Run], out: &Path) -> std::io::Result<()> {
    let mut text =
        String::from("source,temperature_k,temperature_from,n_rows,n_tau,tau_min_ns,tau_max_ns\n");
    for run in runs {
        text.push_str(&full_run_row(run));
        text.push('\n');
    }
    fs::write(out.join("full_runs.csv"), text)?;
    let mut log_text = String::from("source,key,value\n");
    for run in runs {
        for (key, value) in &run.table.log {
            log_text.push_str(&csv_field(&run.source));
            log_text.push(',');
            log_text.push_str(&csv_field(key));
            log_text.push(',');
            log_text.push_str(&num(*value));
            log_text.push('\n');
        }
    }
    fs::write(out.join("full_log.csv"), log_text)
}

fn write_share(runs: &[Run], args: &Args, out: &Path) -> std::io::Result<()> {
    let mut ordered: Vec<&Run> = runs.iter().collect();
    ordered.sort_by(|a, b| {
        let (ta, _) = temperature(&a.table);
        let (tb, _) = temperature(&b.table);
        ta.unwrap_or(f64::INFINITY)
            .total_cmp(&tb.unwrap_or(f64::INFINITY))
            .then_with(|| a.source.cmp(&b.source))
    });
    let mut text = String::from("run_id");
    if args.share_temperature {
        text.push_str(",temperature_k");
    }
    for key in &args.share_keys {
        text.push(',');
        text.push_str(&csv_field(key));
    }
    text.push_str(",n_tau,tau_min_ns,tau_max_ns\n");
    for (i, run) in ordered.iter().enumerate() {
        text.push_str(&share_row(
            run,
            i + 1,
            &args.share_keys,
            args.share_temperature,
        ));
        text.push('\n');
    }
    fs::write(out.join("share.csv"), text)
}

fn list_schema(runs: &[Run]) -> bool {
    let Some(run) = runs.first() else {
        return false;
    };
    let columns: Vec<&str> = run.table.columns.iter().map(String::as_str).collect();
    let keys: Vec<&str> = run.table.log.iter().map(|(k, _)| k.as_str()).collect();
    println!("columns: {}", columns.join(","));
    println!("log_keys: {}", keys.join(","));
    true
}

fn main() {
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let Some(args) = parse_args() else {
        std::process::exit(2);
    };
    let (runs, skipped) = collect_runs(&args.dir);
    if runs.is_empty() {
        eprintln!("no readable run found");
        std::process::exit(1);
    }
    if args.list {
        if !list_schema(&runs) {
            std::process::exit(1);
        }
        return;
    }
    let Some(out) = args.out.as_deref() else {
        usage();
        std::process::exit(2);
    };
    if let Err(err) = fs::create_dir_all(out) {
        eprintln!("output directory could not be created: {err}");
        std::process::exit(1);
    }
    if let Err(err) = write_full(&runs, out).and_then(|_| write_share(&runs, &args, out)) {
        eprintln!("write failed: {err}");
        std::process::exit(1);
    }
    eprintln!(
        "runs: {}, skipped: {}; share.csv and full_*.csv written",
        runs.len(),
        skipped
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> TrispNseTable {
        TrispNseTable {
            columns: vec![
                "tau".to_string(),
                "TTA".to_string(),
                "Pol".to_string(),
                "PolC".to_string(),
            ],
            rows: vec![
                vec![Some(1.0), Some(10.0), Some(0.9), Some(0.8)],
                vec![Some(2.0), Some(10.0), Some(0.7), Some(0.6)],
            ],
            log: vec![
                ("TTAMEAN".to_string(), Some(10.0)),
                ("Gamma".to_string(), Some(0.5)),
                ("dGamma".to_string(), Some(0.05)),
            ],
        }
    }

    fn run() -> Run {
        Run {
            source: "private_name.bin".to_string(),
            table: table(),
        }
    }

    #[test]
    fn temperature_prefers_the_log() {
        let (t, from) = temperature(&table());
        assert_eq!(t, Some(10.0));
        assert_eq!(from, "log:TTAMEAN");
    }

    #[test]
    fn temperature_falls_back_to_the_column() {
        let mut table = table();
        table.log.clear();
        let (t, from) = temperature(&table);
        assert_eq!(t, Some(10.0));
        assert_eq!(from, "column:TTA");
    }

    #[test]
    fn median_handles_even_and_odd_counts() {
        assert_eq!(median(vec![3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(vec![4.0, 1.0, 3.0, 2.0]), Some(2.5));
        assert_eq!(median(Vec::new()), None);
    }

    #[test]
    fn tau_summary_reads_the_finite_range() {
        let (n, lo, hi) = tau_summary(&table());
        assert_eq!(n, 2);
        assert_eq!(lo, Some(1.0));
        assert_eq!(hi, Some(2.0));
    }

    #[test]
    fn share_has_no_source_and_no_extra_log_keys() {
        let row = share_row(&run(), 1, &["Gamma".to_string()], false);
        assert_eq!(row, "run_001,5.000000e-1,2,1.000000e0,2.000000e0");
        assert!(!row.contains("private_name"));
        assert!(!row.contains("TTAMEAN"));
    }

    #[test]
    fn share_can_carry_temperature_on_request() {
        let row = share_row(&run(), 2, &["dGamma".to_string()], true);
        assert_eq!(row, "run_002,10.000000,5.000000e-2,2,1.000000e0,2.000000e0");
    }

    #[test]
    fn absent_values_stay_absent() {
        let row = share_row(&run(), 1, &["missing".to_string()], false);
        assert!(row.contains(",absent,"));
        assert!(!row.contains(",0,"));
    }

    #[test]
    fn csv_fields_are_escaped() {
        assert_eq!(csv_field("plain"), "plain");
        assert_eq!(csv_field("a,b"), "\"a,b\"");
        assert_eq!(csv_field("a\"b"), "\"a\"\"b\"");
    }

    fn temp_dir(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("trisp_export_{}_{}", std::process::id(), name))
    }

    #[test]
    fn collect_runs_prefers_bin_and_skips_dat() {
        let dir = temp_dir("prefer_bin");
        fs::create_dir_all(&dir).expect("temp dir");
        fs::write(dir.join("run.bin"), trisp_nse::pack(&table())).expect("bin");
        fs::write(dir.join("run.dat"), "tau\n1.0\n").expect("dat");
        let (runs, skipped) = collect_runs(&dir);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].source, "run.bin");
        assert_eq!(skipped, 0);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn collect_runs_attaches_a_dat_log() {
        let dir = temp_dir("dat_log");
        fs::create_dir_all(&dir).expect("temp dir");
        fs::write(dir.join("run.dat"), "tau TTA\n1.0 10.0\n").expect("dat");
        fs::write(dir.join("run.log"), "TTAMEAN 10.0\nGamma 0.5\n").expect("log");
        let (runs, skipped) = collect_runs(&dir);
        assert_eq!(runs.len(), 1);
        assert_eq!(skipped, 0);
        assert_eq!(runs[0].table.log_value("Gamma"), Some(0.5));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn collect_runs_attaches_a_log_to_a_bin_without_one() {
        let dir = temp_dir("bin_log");
        fs::create_dir_all(&dir).expect("temp dir");
        let mut table = table();
        table.log.clear();
        fs::write(dir.join("run.bin"), trisp_nse::pack(&table)).expect("bin");
        fs::write(dir.join("run.log"), "TTAMEAN 10.0\nGamma 0.5\n").expect("log");
        let (runs, skipped) = collect_runs(&dir);
        assert_eq!(runs.len(), 1);
        assert_eq!(skipped, 0);
        assert_eq!(runs[0].table.log_value("Gamma"), Some(0.5));
        fs::remove_dir_all(&dir).ok();
    }
}
