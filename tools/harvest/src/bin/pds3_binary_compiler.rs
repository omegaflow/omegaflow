use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds3_binary::{
    BinColumn, BinMeta, BinRow, Pds3BinaryTable, decode_rows, is_array_column, pack, parse_label,
    parse_table,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "data.darts.isas.jaxa.jp";
const LRS_SAMPLE_LABEL: &str = "https://data.darts.isas.jaxa.jp/pub/pds3/sln-l-lrs-2-sndr-waveform-high-v1.0/20071120/data/LRS_SW_WF_00N_007080E.lbl";
const LRS_SAMPLE_DAT: &str = "https://data.darts.isas.jaxa.jp/pub/pds3/sln-l-lrs-2-sndr-waveform-high-v1.0/20071120/data/LRS_SW_WF_00N_007080E.tbl";

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

fn int_type(data_type: &str) -> bool {
    matches!(
        data_type.to_ascii_uppercase().as_str(),
        "MSB_INTEGER"
            | "SUN_INTEGER"
            | "SIGNED_INTEGER"
            | "LSB_INTEGER"
            | "INTEL_INTEGER"
            | "MSB_UNSIGNED_INTEGER"
            | "SUN_UNSIGNED_INTEGER"
            | "LSB_UNSIGNED_INTEGER"
            | "INTEL_UNSIGNED_INTEGER"
    )
}

fn real_type(data_type: &str) -> bool {
    matches!(
        data_type.to_ascii_uppercase().as_str(),
        "IEEE_REAL" | "MSB_REAL" | "LSB_REAL" | "PC_REAL"
    )
}

fn decodable(c: &BinColumn) -> bool {
    let Some(dt) = c.data_type.as_deref() else {
        return false;
    };
    let up = dt.to_ascii_uppercase();
    if up == "TIME" || up.starts_with("ASCII") {
        return c.bytes > 0;
    }
    if c.bytes == 0 {
        return false;
    }
    if int_type(&up) {
        return matches!(c.bytes, 1 | 2 | 4 | 8) || c.bytes > 8;
    }
    real_type(&up) && matches!(c.bytes, 4 | 8)
}

fn assemble(
    meta: &BinMeta,
    raw_rows: Vec<Vec<Option<f64>>>,
    dat_bytes: &[u8],
) -> Option<(Pds3BinaryTable, usize, usize)> {
    let mut columns: Vec<BinColumn> = Vec::new();
    let mut kept: Vec<usize> = Vec::new();
    for (j, c) in meta.columns.iter().enumerate() {
        if decodable(c) || is_array_column(c) {
            columns.push(c.clone());
            kept.push(j);
        }
    }
    if columns.is_empty() {
        return None;
    }
    let stride = meta.record_bytes?;
    if stride == 0 {
        return None;
    }
    let dropped = meta.columns.len() - columns.len();
    let mut rows: Vec<BinRow> = Vec::new();
    let mut skipped = 0usize;
    for (r, raw) in raw_rows.into_iter().enumerate() {
        let values: Vec<Option<f64>> = kept
            .iter()
            .map(|j| raw.get(*j).copied().flatten())
            .collect();
        if values.iter().all(|v| v.is_none()) {
            skipped += 1;
            continue;
        }
        let rec = dat_bytes.get(r * stride..r * stride + stride)?;
        let mut arrays: Vec<Vec<u8>> = Vec::with_capacity(columns.len());
        for c in &columns {
            if !is_array_column(c) {
                arrays.push(Vec::new());
                continue;
            }
            let from = c.start_byte.checked_sub(1)?;
            arrays.push(rec.get(from..from + c.bytes)?.to_vec());
        }
        rows.push(BinRow { values, arrays });
    }
    if rows.is_empty() {
        return None;
    }
    Some((Pds3BinaryTable { columns, rows }, dropped, skipped))
}

fn asset_name(dat_spec: &str) -> String {
    let base = dat_spec.rsplit('/').next().unwrap_or(dat_spec);
    let last = base.split_once('?').map(|(h, _)| h).unwrap_or(base);
    let stem = last.split('.').next().unwrap_or(last);
    format!("pds3_binary_{}.bin", stem.to_ascii_lowercase())
}

fn print_register_lines(asset: &str) {
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{asset}");
    println!("format pds3_binary");
    println!("ttl 604800");
    println!();
}

fn print_inventory(
    asset: &str,
    table: &Pds3BinaryTable,
    meta: &BinMeta,
    dropped: usize,
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
        "{asset}: {} row(s), {} column(s): {}{target}; dropped {dropped} undecodable column(s), skipped {skipped} row(s), {trailing} trailing byte(s)",
        table.rows.len(),
        table.columns.len(),
        cols.join(" | "),
    );
}

fn compile_entry(
    dat_spec: &str,
    label_spec: &str,
    pair_mode: bool,
    out_dir: Option<&str>,
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
    let Some(meta) = parse_label(label_text) else {
        eprintln!("label parse void ({label_spec})");
        return None;
    };
    if meta.columns.is_empty() {
        eprintln!("no COLUMN object in the label ({label_spec})");
        return None;
    }
    if meta.record_type.as_deref() != Some("FIXED_LENGTH") {
        eprintln!(
            "record_type {:?} — the binary arm leaves the table untouched",
            meta.record_type
        );
        return None;
    }
    let Some(dat_bytes) = fetch_or_read(dat_spec) else {
        eprintln!("data fetch void ({dat_spec})");
        return None;
    };
    let Some((raw_rows, decode_skipped, trailing)) = decode_rows(&dat_bytes, &meta) else {
        eprintln!(
            "{} byte(s) carry no fixed-length records — the table stays unwritten (0 honored)",
            dat_bytes.len()
        );
        return None;
    };
    let Some((table, dropped, assemble_skipped)) = assemble(&meta, raw_rows, &dat_bytes) else {
        eprintln!("no decodable numeric row survived — the table stays unwritten (0 honored)");
        return None;
    };
    let asset = asset_name(dat_spec);
    let out_path = match (pair_mode, out_dir) {
        (true, Some(f)) => f.to_string(),
        (true, None) => format!("data/{NETLOC}/pds3_binary/{asset}"),
        (false, Some(d)) => format!("{}/{asset}", d.trim_end_matches('/')),
        (false, None) => format!("data/{NETLOC}/pds3_binary/{asset}"),
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
        dropped,
        decode_skipped + assemble_skipped,
        trailing,
    );
    print_register_lines(&asset);
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("{asset}: CDN upload returned void");
        return None;
    }
    Some(asset)
}

fn pairs_from_hrefs(hrefs: &[String], data_ext: &str, label_ext: &str) -> Vec<(String, String)> {
    let data_suffix = format!(".{}", data_ext.to_ascii_lowercase());
    let label_suffix = label_ext.to_ascii_lowercase();
    let mut data_by_stem: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut label_names: Vec<String> = Vec::new();
    for h in hrefs {
        let name = h
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .unwrap_or("")
            .to_string();
        if name.is_empty() {
            continue;
        }
        let lower = name.to_ascii_lowercase();
        if let Some(stem) = lower.strip_suffix(&data_suffix) {
            data_by_stem.insert(stem.to_string(), name.clone());
        }
        if lower.ends_with(&label_suffix) {
            label_names.push(name);
        }
    }
    label_names.sort();
    label_names.dedup();
    let mut out = Vec::new();
    for name in &label_names {
        let lower = name.to_ascii_lowercase();
        let Some(stem) = lower.strip_suffix(&label_suffix) else {
            continue;
        };
        let Some(dat_name) = data_by_stem.get(stem) else {
            eprintln!("{name}: no sibling {data_suffix} in the listing — pair skipped");
            continue;
        };
        out.push((name.clone(), dat_name.clone()));
    }
    out
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
    let base = dir_url.trim_end_matches('/');
    for (label, dat) in pairs_from_hrefs(&hrefs(text), data_ext, label_ext) {
        out.push((format!("{base}/{label}"), format!("{base}/{dat}")));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_arg = arg_value(&args, "--out");
    let mut pairs: Vec<(String, String)> = Vec::new();
    let dir_arg = arg_value(&args, "--dir");
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
            match dir_arg.as_deref() {
                Some(dir) => collect_pairs(dir, "tbl", ".lbl", &mut pairs),
                None => pairs.push((LRS_SAMPLE_LABEL.to_string(), LRS_SAMPLE_DAT.to_string())),
            }
            false
        }
    };
    if pairs.is_empty() {
        eprintln!("no .lbl/.tbl pair found — nothing written (0 honored)");
        std::process::exit(1);
    }
    let mut written = 0usize;
    for (label_spec, dat_spec) in &pairs {
        if compile_entry(dat_spec, label_spec, pair_mode, out_arg.as_deref(), ci_mode).is_some() {
            written += 1;
        }
    }
    if written == 0 {
        eprintln!("no table packed — nothing written (0 honored)");
        std::process::exit(1);
    }
    eprintln!("{written} table(s) packed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_preserve_the_listing_case() {
        let listing = vec![
            "?C=N;O=D".to_string(),
            "LRS_SW_WF_00N_007080E.lbl".to_string(),
            "LRS_SW_WF_00N_007080E.tbl".to_string(),
            "LRS_SW_WF_00S_007080E.lbl".to_string(),
            "LRS_SW_WF_00S_007080E.tbl".to_string(),
        ];
        let pairs = pairs_from_hrefs(&listing, "tbl", ".lbl");
        assert_eq!(
            pairs,
            vec![
                (
                    "LRS_SW_WF_00N_007080E.lbl".to_string(),
                    "LRS_SW_WF_00N_007080E.tbl".to_string()
                ),
                (
                    "LRS_SW_WF_00S_007080E.lbl".to_string(),
                    "LRS_SW_WF_00S_007080E.tbl".to_string()
                ),
            ]
        );
    }

    #[test]
    fn a_label_without_sibling_data_is_not_paired() {
        let listing = vec!["ORPHAN.lbl".to_string(), "OTHER.tbl".to_string()];
        assert!(pairs_from_hrefs(&listing, "tbl", ".lbl").is_empty());
    }
}
