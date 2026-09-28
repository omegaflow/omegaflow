use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds3_table::unix_of_iso;
use omegaflow::archivar::pds4::{
    Pds4Column, Pds4Meta, Pds4Row, Pds4Table, decode_rows, pack, parse_label, parse_table,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "sbnarchive.psi.edu";
const CDR_ROUTE: &str = "https://sbnarchive.psi.edu/pds4/hayabusa/hay.lidar/data_calibrated/";
const AXIS_UNIT: &str = "SECOND";
const MET_TICK_SCALE: f64 = 1.0 / 32.0;

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
        "ASCII_INTEGER" | "ASCII_NONNEGATIVE_INTEGER" | "ASCII_REAL"
    )
}

enum Axis {
    IsoTime(usize),
    Offset(usize, f64, f64),
}

fn axis_of(meta: &Pds4Meta) -> Option<(Axis, String)> {
    for (i, c) in meta.columns.iter().enumerate() {
        if c.data_type
            .to_ascii_uppercase()
            .starts_with("ASCII_DATE_TIME")
        {
            return Some((
                Axis::IsoTime(i),
                format!("{} ({} date-time)", c.name, c.data_type),
            ));
        }
    }
    for (i, c) in meta.columns.iter().enumerate() {
        if c.name.eq_ignore_ascii_case("MET") {
            let base = match meta.start_time.as_deref() {
                Some(st) => match unix_of_iso(st) {
                    Some(b) => b,
                    None => return None,
                },
                None => return None,
            };
            return Some((
                Axis::Offset(i, base, MET_TICK_SCALE),
                format!(
                    "{} (MET {} s ticks + START_TIME {base:.3})",
                    c.name, MET_TICK_SCALE
                ),
            ));
        }
    }
    None
}

fn axis_column(meta: &Pds4Meta, idx: usize) -> Pds4Column {
    let c = &meta.columns[idx];
    Pds4Column {
        name: c.name.clone(),
        unit: AXIS_UNIT.to_string(),
        data_type: "TIME".to_string(),
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
    meta: &Pds4Meta,
    raw_rows: Vec<Vec<Option<f64>>>,
    axis: Option<(Axis, String)>,
) -> Option<(Pds4Table, usize, String)> {
    let mut columns: Vec<Pds4Column> = Vec::new();
    let mut kept: Vec<usize> = Vec::new();
    let axis_note = match &axis {
        Some((_, note)) => note.clone(),
        None => String::from("none"),
    };
    match &axis {
        Some((Axis::IsoTime(i), _)) | Some((Axis::Offset(i, _, _), _)) => {
            columns.push(axis_column(meta, *i));
            for (j, c) in meta.columns.iter().enumerate() {
                if j != *i && is_numeric_type(&c.data_type) {
                    columns.push(c.clone());
                    kept.push(j);
                }
            }
        }
        None => {
            for (j, c) in meta.columns.iter().enumerate() {
                if is_numeric_type(&c.data_type) {
                    columns.push(c.clone());
                    kept.push(j);
                }
            }
        }
    }
    let mut rows: Vec<Pds4Row> = Vec::new();
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
            Some((Axis::Offset(i, base, scale), _)) => {
                let v = match raw.get(*i) {
                    Some(Some(v)) => Some(v * scale + base),
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
        rows.push(Pds4Row { values });
    }
    if rows.is_empty() {
        return None;
    }
    Some((
        Pds4Table {
            columns,
            rows,
            delimited: false,
            delimiter: None,
        },
        skipped,
        axis_note,
    ))
}

fn asset_name(dat_spec: &str) -> String {
    let base = dat_spec.rsplit('/').next().unwrap_or(dat_spec);
    let last = base.split_once('?').map(|(h, _)| h).unwrap_or(base);
    let stem = last.split('.').next().unwrap_or(last);
    format!("pds4_fixed_width_{}.bin", stem.to_ascii_lowercase())
}

fn print_register_lines(asset: &str) {
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{asset}");
    println!("format pds4_fixed_width");
    println!("ttl 604800");
    println!();
}

fn print_inventory(
    asset: &str,
    table: &Pds4Table,
    meta: &Pds4Meta,
    axis_note: &str,
    skipped: usize,
    trailing: usize,
) {
    let mut cols = Vec::new();
    for c in &table.columns {
        let mut s = format!("{} ({})", c.name, c.data_type);
        if !c.unit.is_empty() {
            s.push_str(&format!(", {}", c.unit));
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
    if meta.table_class != "Table_Character" {
        eprintln!(
            "{} — the fixed-width arm leaves the table untouched",
            meta.table_class
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
        (true, None) => format!("data/{NETLOC}/pds4_fixed_width/{asset}"),
        (false, Some(d)) => format!("{}/{asset}", d.trim_end_matches('/')),
        (false, None) => format!("data/{NETLOC}/pds4_fixed_width/{asset}"),
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
        if stem.starts_with("collection_") || stem.starts_with("bundle_") {
            continue;
        }
        let dat_name = format!("{stem}.{low}");
        out.push((format!("{base}/{name}"), format!("{base}/{dat_name}")));
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
            collect_pairs(CDR_ROUTE, "tab", ".xml", &mut pairs);
            false
        }
    };
    if pairs.is_empty() {
        eprintln!("no .xml/.tab pair found — nothing written (0 honored)");
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
