use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds3_table::{
    Pds3Table, TableColumn, TableRow, decode_rows, pack, parse_label, parse_table,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "pds-geosciences.wustl.edu";
const LABEL_URL: &str = "https://pds-geosciences.wustl.edu/premgn/mg_1001/vikgrav/vmar001l.lbl";
const DAT_URL: &str = "https://pds-geosciences.wustl.edu/premgn/mg_1001/vikgrav/vmar001l.dat";
const ASSET: &str = "viking_grav_vmar001l.bin";
const DEFAULT_OUT: &str = "data/pds-geosciences.wustl.edu/viking_grav/viking_grav_vmar001l.bin";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch_or_read(spec: &str) -> Option<Vec<u8>> {
    if spec.starts_with("http://") || spec.starts_with("https://") {
        fetch_raw_bytes(spec)
    } else {
        std::fs::read(spec).ok()
    }
}

fn is_numeric_type(data_type: &str) -> bool {
    matches!(
        data_type.to_ascii_uppercase().as_str(),
        "INTEGER"
            | "ASCII_INTEGER"
            | "MSB_INTEGER"
            | "LSB_INTEGER"
            | "PC_INTEGER"
            | "REAL"
            | "ASCII_REAL"
            | "FLOAT"
    )
}

fn assemble(
    meta: &omegaflow::archivar::pds3_table::TableMeta,
    rows: Vec<Vec<Option<f64>>>,
) -> Pds3Table {
    let mut columns: Vec<TableColumn> = Vec::new();
    let mut kept: Vec<usize> = Vec::new();
    for (j, c) in meta.columns.iter().enumerate() {
        if c.data_type.as_deref().is_some_and(is_numeric_type) {
            columns.push(c.clone());
            kept.push(j);
        }
    }
    let mut table_rows: Vec<TableRow> = Vec::with_capacity(rows.len());
    for raw in rows {
        let values = kept
            .iter()
            .map(|j| raw.get(*j).copied().flatten())
            .collect();
        table_rows.push(TableRow { values });
    }
    Pds3Table {
        columns,
        rows: table_rows,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(p) => p,
        None => DEFAULT_OUT.to_string(),
    };
    let label_spec = match arg_value(&args, "--label") {
        Some(p) => p,
        None => LABEL_URL.to_string(),
    };
    let dat_spec = match arg_value(&args, "--dat") {
        Some(p) => p,
        None => DAT_URL.to_string(),
    };
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "usage: viking_grav_compiler [--out <asset>] [--label <file|url>] [--dat <file|url>] [--ci-mode]"
        );
        return;
    }

    let Some(label_bytes) = fetch_or_read(&label_spec) else {
        eprintln!("viking_grav: label fetch void ({label_spec}) — the asset stays unwritten");
        std::process::exit(1);
    };
    let Ok(label_text) = std::str::from_utf8(&label_bytes) else {
        eprintln!("viking_grav: label is not utf8 ({label_spec}) — the asset stays unwritten");
        std::process::exit(1);
    };
    let Some(meta) = parse_label(label_text) else {
        eprintln!(
            "viking_grav: the PDS3 label did not parse ({label_spec}) — the asset stays unwritten"
        );
        std::process::exit(1);
    };
    let Some(dat_bytes) = fetch_or_read(&dat_spec) else {
        eprintln!("viking_grav: data fetch void ({dat_spec}) — the asset stays unwritten");
        std::process::exit(1);
    };
    let Some((raw_rows, skipped, trailing)) = decode_rows(&dat_bytes, &meta) else {
        eprintln!(
            "viking_grav: the table records did not decode — the asset stays unwritten (0 honored)"
        );
        std::process::exit(1);
    };
    let table = assemble(&meta, raw_rows);
    if table.columns.is_empty() || table.rows.is_empty() {
        eprintln!(
            "viking_grav: no numeric column/row survived — the asset stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let bin = pack(&table);
    let Some(parsed) = parse_table(&bin) else {
        eprintln!("viking_grav: packed read void — the table stays unverified (0 honored)");
        std::process::exit(1);
    };
    if parsed != table {
        eprintln!("viking_grav: roundtrip void — the table stays unverified (0 honored)");
        std::process::exit(1);
    }
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("viking_grav: write {out_path} returned void — the asset stays unwritten");
        std::process::exit(1);
    }
    let columns = table
        .columns
        .iter()
        .map(|c| c.name.as_str())
        .collect::<Vec<_>>()
        .join(" | ");
    println!(
        "viking_grav: {out_path}: {} row(s), {} column(s) ({columns}), {} byte(s), sha256 {}, roundtrip reads back; skipped {skipped} row(s), {trailing} trailing byte(s)",
        table.rows.len(),
        table.columns.len(),
        bin.len(),
        sha256_hex(&bin),
    );
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{ASSET}");
    println!("format viking_grav");
    println!("ttl 604800");
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!(
            "viking_grav: {out_path} did not reach the CDN — the local asset stands, the manifest is pending"
        );
    }
}
