use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds4_binary::{
    Pds4BinaryRow, Pds4BinaryTable, decode_rows, pack, parse_label, parse_table,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "archives.esac.esa.int";
const DEFAULT_ROUTE: &str = "https://archives.esac.esa.int/psa/ftp/ExoMars2016/em16_tgo_acs/data_raw/Science_Phase/Orbit_Range_6200_6299/Orbit_6200/";
const ASSET_PREFIX: &str = "pds4_binary_";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn hrefs(text: &str) -> Vec<String> {
    let low = text.to_ascii_lowercase();
    let mut out: Vec<String> = Vec::new();
    let mut from = 0usize;
    while let Some(p) = low[from..].find("href=\"") {
        let start = from + p + 6;
        match low[start..].find('"') {
            Some(end) => {
                out.push(text[start..start + end].to_string());
                from = start + end;
            }
            None => break,
        }
    }
    out
}

fn fetch_or_read(spec: &str) -> Option<Vec<u8>> {
    if spec.starts_with("http://") || spec.starts_with("https://") {
        fetch_raw_bytes(spec)
    } else {
        std::fs::read(spec).ok()
    }
}

fn asset_name(label_spec: &str) -> String {
    let base = label_spec.rsplit('/').next().unwrap_or(label_spec);
    let last = base.split_once('?').map(|(h, _)| h).unwrap_or(base);
    let stem = last.split('.').next().unwrap_or(last);
    format!("{ASSET_PREFIX}{}.bin", stem.to_ascii_lowercase())
}

fn print_register_lines(asset: &str) {
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{asset}");
    println!("format pds4_binary");
    println!("ttl 604800");
    println!();
}

fn print_inventory(asset: &str, table: &Pds4BinaryTable, meta_file: &str, skipped: usize) {
    let mut cols = Vec::new();
    for c in &table.columns {
        let mut s = match &c.data_type {
            Some(dt) => format!("{} ({})", c.name, dt),
            None => format!("{} (data_type absent)", c.name),
        };
        if let Some(unit) = &c.unit {
            if !unit.is_empty() {
                s.push_str(&format!(", {unit}"));
            }
        }
        if let Some(m) = c.missing_constant {
            s.push_str(&format!(", missing {m}"));
        }
        cols.push(s);
    }
    eprintln!(
        "{asset}: {} row(s), {} column(s): {}; file {meta_file}; skipped {skipped} row(s)",
        table.rows.len(),
        table.columns.len(),
        cols.join(" | "),
    );
}

fn compile_pair(
    label_spec: &str,
    dat_spec: Option<&str>,
    out_dir: Option<&str>,
    ci_mode: bool,
) -> Option<String> {
    let Some(label_bytes) = fetch_or_read(label_spec) else {
        eprintln!("label fetch/read void ({label_spec})");
        return None;
    };
    let Ok(label_text) = std::str::from_utf8(&label_bytes) else {
        eprintln!("label not utf8 ({label_spec})");
        return None;
    };
    let Some(meta) = parse_label(label_text) else {
        eprintln!(
            "{label_spec}: no Table_Binary File_Area — the label stays untouched (0 honored)"
        );
        return None;
    };
    let dat = match dat_spec {
        Some(d) => d.to_string(),
        None => {
            let Some(name) = meta.file_name.as_deref() else {
                eprintln!("{label_spec}: label carries no file_name — the data file stays unnamed");
                return None;
            };
            match label_spec.rsplit_once('/') {
                Some((dir, _)) => format!("{dir}/{name}"),
                None => name.to_string(),
            }
        }
    };
    let Some(dat_bytes) = fetch_or_read(&dat) else {
        eprintln!("data fetch void ({dat})");
        return None;
    };
    let Some((raw_rows, decode_skipped, trailing)) = decode_rows(&dat_bytes, &meta) else {
        eprintln!(
            "{dat}: {} byte(s) carry no Table_Binary rows — the table stays unwritten (0 honored)",
            dat_bytes.len()
        );
        return None;
    };
    let columns = meta.columns.clone();
    let rows: Vec<Pds4BinaryRow> = raw_rows
        .into_iter()
        .filter(|values| values.iter().any(|v| v.is_some()))
        .map(|values| Pds4BinaryRow { values })
        .collect();
    if rows.is_empty() {
        eprintln!("{dat}: no numeric row survived — the table stays unwritten (0 honored)");
        return None;
    }
    let table = Pds4BinaryTable { columns, rows };
    let asset = asset_name(label_spec);
    let out_path = match out_dir {
        Some(dir) => format!("{}/{asset}", dir.trim_end_matches('/')),
        None => format!("data/{NETLOC}/pds4_binary/{asset}"),
    };
    let bin = pack(&table);
    let Some(parsed) = parse_table(&bin) else {
        eprintln!("{asset}: packed read void — the table stays unverified (0 honored)");
        return None;
    };
    if parsed != table {
        eprintln!("{asset}: roundtrip void — the table stays unverified (0 honored)");
        return None;
    }
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("write {out_path} returned void");
        return None;
    }
    eprintln!(
        "{out_path}: {} row(s) packed, {} byte(s), sha256 {}, roundtrip holds; {trailing} trailing byte(s)",
        table.rows.len(),
        bin.len(),
        sha256_hex(&bin),
    );
    print_inventory(&asset, &table, &dat, decode_skipped);
    let target = match meta.target_name.as_deref() {
        Some(t) => t,
        None => "unknown",
    };
    let instrument = match meta.instrument_name.as_deref() {
        Some(i) => i,
        None => "unknown",
    };
    eprintln!("{asset}: target {target}, instrument {instrument}");
    print_register_lines(&asset);
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("{asset}: CDN upload returned void");
        return None;
    }
    Some(asset)
}

fn collect_labels(dir_url: &str, out: &mut Vec<String>) {
    let Some(bytes) = fetch_raw_bytes(dir_url) else {
        eprintln!("{dir_url}: listing fetch void");
        return;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{dir_url}: listing not utf8");
        return;
    };
    let base = dir_url.trim_end_matches('/');
    let mut names: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.to_ascii_lowercase().ends_with(".xml"))
        .map(|h| {
            h.trim_end_matches('/')
                .rsplit('/')
                .next()
                .unwrap_or("")
                .to_string()
        })
        .filter(|n| !n.starts_with("bundle_") && !n.starts_with("collection_"))
        .collect();
    names.sort();
    names.dedup();
    for name in names {
        out.push(format!("{base}/{name}"));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_arg = arg_value(&args, "--out");
    let dat_arg = arg_value(&args, "--dat");
    let mut labels: Vec<String> = Vec::new();
    match arg_value(&args, "--label") {
        Some(label) => labels.push(label),
        None => {
            let dir = match arg_value(&args, "--dir") {
                Some(d) => d,
                None => DEFAULT_ROUTE.to_string(),
            };
            collect_labels(&dir, &mut labels);
        }
    }
    if labels.is_empty() {
        eprintln!("no .xml label found — nothing written (0 honored)");
        std::process::exit(1);
    }
    let mut written = 0usize;
    for label in &labels {
        if compile_pair(label, dat_arg.as_deref(), out_arg.as_deref(), ci_mode).is_some() {
            written += 1;
        }
    }
    if written == 0 {
        eprintln!("no Table_Binary packed — nothing written (0 honored)");
        std::process::exit(1);
    }
    eprintln!("{written} table(s) packed");
}
