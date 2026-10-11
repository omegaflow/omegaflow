use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds3_table::{
    Pds3Table, TableColumn, TableMeta, TableRow, decode_rows, odl_kv, pack, parse_label,
    parse_table,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "pds-geosciences.wustl.edu";
const CDR_ROUTE: &str = "https://pds-geosciences.wustl.edu/messenger/mess-e_v_h-mla-3_4-cdr_rdr-data-v1/messmla_2001/cdr/2004/aug/";
const LABEL_ROUTE: &str = "https://pds-geosciences.wustl.edu/messenger/mess-e_v_h-mla-3_4-cdr_rdr-data-v1/messmla_2001/label/";

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

fn merge_structure(label_text: &str, structure_text: &str) -> String {
    match label_text.find("END_OBJECT") {
        Some(cut) => format!("{}\n{structure_text}", &label_text[..cut]),
        None => format!("{label_text}\n{structure_text}"),
    }
}

fn assemble(meta: &TableMeta, raw_rows: Vec<Vec<Option<f64>>>) -> Option<(Pds3Table, usize)> {
    let mut columns: Vec<TableColumn> = Vec::new();
    let mut kept: Vec<usize> = Vec::new();
    for (j, c) in meta.columns.iter().enumerate() {
        if c.data_type.as_deref().is_some_and(is_numeric_type) {
            columns.push(c.clone());
            kept.push(j);
        }
    }
    if columns.is_empty() {
        return None;
    }
    let mut rows: Vec<TableRow> = Vec::new();
    let mut skipped = 0usize;
    for raw in raw_rows {
        let mut values: Vec<Option<f64>> = Vec::with_capacity(kept.len());
        for j in &kept {
            values.push(match raw.get(*j) {
                Some(Some(v)) => Some(*v),
                _ => None,
            });
        }
        if values.iter().all(|v| v.is_none()) {
            skipped += 1;
            continue;
        }
        rows.push(TableRow { values });
    }
    if rows.is_empty() {
        return None;
    }
    Some((Pds3Table { columns, rows }, skipped))
}

fn asset_name(dat_spec: &str) -> String {
    let base = dat_spec.rsplit('/').next().unwrap_or(dat_spec);
    let last = base.split_once('?').map(|(h, _)| h).unwrap_or(base);
    let stem = last.split('.').next().unwrap_or(last);
    format!("pds3_fixed_width_{}.bin", stem.to_ascii_lowercase())
}

fn print_register_lines(asset: &str) {
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{asset}");
    println!("format pds3_fixed_width");
    println!("ttl 604800");
    println!();
}

fn print_inventory(
    asset: &str,
    table: &Pds3Table,
    meta: &TableMeta,
    skipped: usize,
    trailing: usize,
) {
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
        match c.missing_constant {
            Some(m) => s.push_str(&format!(", missing {m}")),
            None => {}
        }
        cols.push(s);
    }
    let target = match meta.target_name.as_deref() {
        Some(t) => format!(", target {t}"),
        None => String::new(),
    };
    eprintln!(
        "{asset}: {} row(s), {} column(s): {}{target}; skipped {skipped} row(s), {trailing} trailing byte(s)",
        table.rows.len(),
        table.columns.len(),
        cols.join(" | "),
    );
}

fn compile_entry(
    dat_spec: &str,
    label_spec: &str,
    structure_override: Option<&str>,
    pair_mode: bool,
    out_dir: Option<&str>,
    netloc: &str,
    ci_mode: bool,
) -> Option<String> {
    let Some(label_bytes) = fetch_or_read(label_spec) else {
        eprintln!("label fetch void ({label_spec})");
        return None;
    };
    let Ok(label_text) = std::str::from_utf8(&label_bytes) else {
        eprintln!("label not utf8 ({label_spec})");
        return None;
    };
    let structure_name = odl_kv(label_text)
        .into_iter()
        .find(|(k, _)| k == "^STRUCTURE")
        .map(|(_, v)| v.trim_matches('"').trim().to_ascii_lowercase());
    let structure_spec = match structure_override {
        Some(s) => s.to_string(),
        None => match &structure_name {
            Some(name) => format!("{LABEL_ROUTE}{name}"),
            None => {
                eprintln!("{label_spec}: ^STRUCTURE absent — the columns stay unnamed (0 honored)");
                return None;
            }
        },
    };
    let Some(structure_bytes) = fetch_or_read(&structure_spec) else {
        eprintln!("structure fetch void ({structure_spec})");
        return None;
    };
    let Ok(structure_text) = std::str::from_utf8(&structure_bytes) else {
        eprintln!("structure not utf8 ({structure_spec})");
        return None;
    };
    let merged = merge_structure(label_text, structure_text);
    let Some(meta) = parse_label(&merged) else {
        eprintln!("label parse void ({label_spec})");
        return None;
    };
    if meta.interchange.as_deref() != Some("ASCII") {
        eprintln!(
            "interchange {:?} — the ASCII fixed-width arm leaves the table untouched",
            meta.interchange
        );
        return None;
    }
    let Some(dat_bytes) = fetch_or_read(dat_spec) else {
        eprintln!("data fetch void ({dat_spec})");
        return None;
    };
    let Some((raw_rows, decode_skipped, trailing)) = decode_rows(&dat_bytes, &meta) else {
        eprintln!(
            "{} byte(s) carry no fixed-width rows — the table stays unwritten (0 honored)",
            dat_bytes.len()
        );
        return None;
    };
    let Some((table, assemble_skipped)) = assemble(&meta, raw_rows) else {
        eprintln!("no numeric row survived — the table stays unwritten (0 honored)");
        return None;
    };
    let asset = asset_name(dat_spec);
    let out_path = match (pair_mode, out_dir) {
        (true, Some(f)) => f.to_string(),
        (true, None) => format!("data/{netloc}/pds3_fixed_width/{asset}"),
        (false, Some(d)) => format!("{}/{asset}", d.trim_end_matches('/')),
        (false, None) => format!("data/{netloc}/pds3_fixed_width/{asset}"),
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
        "{out_path}: {} row(s) packed, {} byte(s), sha256 {}, roundtrip holds",
        table.rows.len(),
        bin.len(),
        sha256_hex(&bin)
    );
    print_inventory(
        &asset,
        &table,
        &meta,
        decode_skipped + assemble_skipped,
        trailing,
    );
    print_register_lines(&asset);
    if ci_mode && !upload_release(netloc, &out_path) {
        eprintln!("{asset}: CDN upload returned void");
        return None;
    }
    Some(asset)
}

fn collect_pairs(dir_url: &str, data_ext: &str, label_ext: &str, out: &mut Vec<(String, String)>) {
    let Some(bytes) = fetch_raw_bytes(dir_url) else {
        eprintln!("{dir_url}: listing fetch void");
        return;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{dir_url}: listing not utf8");
        return;
    };
    let low = data_ext.to_ascii_lowercase();
    let mut names: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.to_ascii_lowercase().ends_with(label_ext))
        .map(|h| {
            h.trim_end_matches('/')
                .rsplit('/')
                .next()
                .unwrap_or("")
                .to_string()
        })
        .collect();
    names.sort();
    names.dedup();
    let base = dir_url.trim_end_matches('/');
    for name in &names {
        let lower = name.to_ascii_lowercase();
        let Some(stem) = lower.strip_suffix(label_ext) else {
            continue;
        };
        let dat_name = format!("{stem}.{low}");
        out.push((format!("{base}/{name}"), format!("{base}/{dat_name}")));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_arg = arg_value(&args, "--out");
    let structure_arg = arg_value(&args, "--structure");
    let netloc = match arg_value(&args, "--netloc") {
        Some(n) if !n.is_empty() => n,
        _ => NETLOC.to_string(),
    };
    let mut pairs: Vec<(String, String)> = Vec::new();
    let pair_mode = match (arg_value(&args, "--dat"), arg_value(&args, "--label")) {
        (Some(dat), Some(label)) => {
            pairs.push((label, dat));
            true
        }
        (Some(_), None) | (None, Some(_)) => {
            eprintln!("--dat <file|url> and --label <file|url> come as a pair");
            std::process::exit(2);
        }
        (None, None) => {
            collect_pairs(CDR_ROUTE, "tab", ".lbl", &mut pairs);
            false
        }
    };
    if pairs.is_empty() {
        eprintln!("no .lbl/.tab pair found — nothing written (0 honored)");
        std::process::exit(1);
    }
    let mut written = 0usize;
    for (label_spec, dat_spec) in &pairs {
        if compile_entry(
            dat_spec,
            label_spec,
            structure_arg.as_deref(),
            pair_mode,
            out_arg.as_deref(),
            &netloc,
            ci_mode,
        )
        .is_some()
        {
            written += 1;
        }
    }
    if written == 0 {
        eprintln!("no table packed — nothing written (0 honored)");
        std::process::exit(1);
    }
    eprintln!("{written} table(s) packed");
}
