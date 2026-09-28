use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds3_table::{
    Pds3Table, TableColumn, TableMeta, TableRow, decode_rows, pack, parse_label, parse_table,
    unix_of_iso,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "pds-smallbodies.astro.umd.edu";
const KRFM_ROUTE: &str =
    "https://pds-smallbodies.astro.umd.edu/holdings/phb2-m-krfm-3-photometry-v1.0/data/";
const VEGA_ROUTE: &str = "https://pds-smallbodies.astro.umd.edu/holdings/vega2-c_sw-mischa-3-rdr-original-v1.0/data/ascii/";
const AXIS_UNIT: &str = "SECOND";

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

enum Axis {
    IsoTime(usize),
    Offset(usize, f64),
}

fn axis_of(meta: &TableMeta) -> Option<(Axis, String)> {
    for (i, c) in meta.columns.iter().enumerate() {
        if c.data_type
            .as_deref()
            .is_some_and(|d| d.eq_ignore_ascii_case("TIME"))
        {
            return Some((Axis::IsoTime(i), format!("{} (DATA_TYPE TIME)", c.name)));
        }
    }
    for (i, c) in meta.columns.iter().enumerate() {
        if c.name.eq_ignore_ascii_case("TIME_OFFSET") {
            let base = match meta.start_time.as_deref() {
                Some(st) => match unix_of_iso(st) {
                    Some(b) => b,
                    None => return None,
                },
                None => return None,
            };
            return Some((
                Axis::Offset(i, base),
                format!("{} (TIME_OFFSET + START_TIME {base:.3})", c.name),
            ));
        }
    }
    None
}

fn axis_column(meta: &TableMeta, idx: usize) -> TableColumn {
    let c = &meta.columns[idx];
    TableColumn {
        name: c.name.clone(),
        unit: Some(AXIS_UNIT.to_string()),
        data_type: Some("TIME".to_string()),
        missing_constant: c.missing_constant,
        sampling_name: c.sampling_name.clone(),
        sampling_unit: c.sampling_unit.clone(),
        sampling_min: c.sampling_min,
        sampling_max: c.sampling_max,
        start_byte: c.start_byte,
        bytes: c.bytes,
    }
}

fn assemble(
    meta: &TableMeta,
    raw_rows: Vec<Vec<Option<f64>>>,
    axis: Option<(Axis, String)>,
) -> Option<(Pds3Table, usize, String)> {
    let mut columns: Vec<TableColumn> = Vec::new();
    let mut kept: Vec<usize> = Vec::new();
    let axis_note = match &axis {
        Some((_, note)) => note.clone(),
        None => String::from("none"),
    };
    match &axis {
        Some((Axis::IsoTime(i), _)) | Some((Axis::Offset(i, _), _)) => {
            columns.push(axis_column(meta, *i));
            for (j, c) in meta.columns.iter().enumerate() {
                if j != *i && c.data_type.as_deref().is_some_and(is_numeric_type) {
                    columns.push(c.clone());
                    kept.push(j);
                }
            }
        }
        None => {
            for (j, c) in meta.columns.iter().enumerate() {
                if c.data_type.as_deref().is_some_and(is_numeric_type) {
                    columns.push(c.clone());
                    kept.push(j);
                }
            }
        }
    }
    let mut rows: Vec<TableRow> = Vec::new();
    let mut skipped = 0usize;
    for raw in raw_rows {
        let mut values: Vec<Option<f64>> = Vec::new();
        match &axis {
            Some((Axis::IsoTime(i), _)) => {
                let v = match raw.get(*i) {
                    Some(Some(v)) => Some(*v),
                    _ => None,
                };
                match v {
                    Some(v) => values.push(Some(v)),
                    None => {
                        skipped += 1;
                        continue;
                    }
                }
            }
            Some((Axis::Offset(i, base), _)) => {
                let v = match raw.get(*i) {
                    Some(Some(v)) => Some(v + base),
                    _ => None,
                };
                match v {
                    Some(v) => values.push(Some(v)),
                    None => {
                        skipped += 1;
                        continue;
                    }
                }
            }
            None => {}
        }
        for j in &kept {
            values.push(match raw.get(*j) {
                Some(Some(v)) => Some(*v),
                _ => None,
            });
        }
        if axis.is_none() && values.iter().all(|v| v.is_none()) {
            skipped += 1;
            continue;
        }
        rows.push(TableRow { values });
    }
    if rows.is_empty() {
        return None;
    }
    Some((Pds3Table { columns, rows }, skipped, axis_note))
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
    axis_note: &str,
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
        match (c.sampling_min, c.sampling_max) {
            (Some(lo), Some(hi)) => s.push_str(&format!(", sampling {lo}..{hi}")),
            (Some(lo), None) => s.push_str(&format!(", sampling_min {lo}")),
            (None, Some(hi)) => s.push_str(&format!(", sampling_max {hi}")),
            (None, None) => {}
        }
        if !c.sampling_unit.is_empty() && c.unit.as_deref() != Some(c.sampling_unit.as_str()) {
            s.push_str(&format!(" {}", c.sampling_unit));
        }
        cols.push(s);
    }
    let target = match meta.target_name.as_deref() {
        Some(t) => format!(", target {t}"),
        None => String::new(),
    };
    eprintln!(
        "{asset}: {} row(s), axis {axis_note}, {} column(s): {}{target}; skipped {skipped} row(s), {trailing} trailing byte(s)",
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
    let axis = axis_of(&meta);
    let Some((table, assemble_skipped, axis_note)) = assemble(&meta, raw_rows, axis) else {
        eprintln!("no numeric row survived — the table stays unwritten (0 honored)");
        return None;
    };
    let asset = asset_name(dat_spec);
    let out_path = match (pair_mode, out_dir) {
        (true, Some(f)) => f.to_string(),
        (true, None) => format!("data/{NETLOC}/pds3_fixed_width/{asset}"),
        (false, Some(d)) => format!("{}/{asset}", d.trim_end_matches('/')),
        (false, None) => format!("data/{NETLOC}/pds3_fixed_width/{asset}"),
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
        &axis_note,
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

fn collect_dirs(dir_url: &str) -> Vec<String> {
    let Some(bytes) = fetch_raw_bytes(dir_url) else {
        eprintln!("{dir_url}: listing fetch void");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{dir_url}: listing not utf8");
        return Vec::new();
    };
    let mut dirs: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| {
            h.ends_with('/')
                && !h.contains('?')
                && !h.contains("://")
                && !h.starts_with('/')
                && !h.starts_with('.')
        })
        .map(|h| {
            let name = h.trim_end_matches('/').rsplit('/').next().unwrap_or("");
            format!("{}/{name}/", dir_url.trim_end_matches('/'))
        })
        .filter(|h| !h.ends_with("//") && !h.ends_with("/../"))
        .collect();
    dirs.sort();
    dirs.dedup();
    dirs
}

fn walk_vega(out: &mut Vec<(String, String)>) {
    for res in collect_dirs(VEGA_ROUTE) {
        for year in collect_dirs(&res) {
            collect_pairs(&year, "tab", ".lbl", out);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_arg = arg_value(&args, "--out");
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
            collect_pairs(KRFM_ROUTE, "dat", ".lbl", &mut pairs);
            walk_vega(&mut pairs);
            false
        }
    };
    if pairs.is_empty() {
        eprintln!("no .lbl/.dat pair found — nothing written (0 honored)");
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
