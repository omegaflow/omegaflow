use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds4::{
    Pds4Column, Pds4Row, Pds4Table, XElem, pack, parse_table, parse_xml,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "sbnarchive.psi.edu";
const OLA_ROUTE: &str = "https://sbnarchive.psi.edu/pds4/orex/orex.ola/data_calibrated/orbit_a/";

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

fn elem_child<'a>(elem: &'a XElem, name: &str) -> Option<&'a XElem> {
    elem.children.iter().find(|c| c.name == name)
}

fn elem_text(elem: &XElem, name: &str) -> Option<String> {
    elem_child(elem, name)
        .map(|c| c.text.trim().to_string())
        .filter(|t| !t.is_empty())
}

#[derive(Clone, Debug)]
struct Field {
    name: String,
    location: usize,
    length: usize,
    data_type: String,
    unit: Option<String>,
}

struct BinaryLabel {
    record_length: usize,
    file_name: Option<String>,
    target_name: Option<String>,
    fields: Vec<Field>,
}

fn parse_binary_label(text: &str) -> Option<BinaryLabel> {
    let root = parse_xml(text)?;
    let fao = elem_child(&root, "File_Area_Observational")?;
    let table = elem_child(fao, "Table_Binary")?;
    let record = elem_child(table, "Record_Binary")?;
    let record_length = elem_text(record, "record_length")?
        .trim()
        .parse::<usize>()
        .ok()?;
    let mut fields = Vec::new();
    for f in record.children.iter().filter(|c| c.name == "Field_Binary") {
        let (Some(name), Some(location), Some(length)) = (
            elem_text(f, "name"),
            elem_text(f, "field_location").and_then(|s| s.trim().parse::<usize>().ok()),
            elem_text(f, "field_length").and_then(|s| s.trim().parse::<usize>().ok()),
        ) else {
            continue;
        };
        if location == 0 || length == 0 {
            continue;
        }
        let Some(data_type) = elem_text(f, "data_type") else {
            continue;
        };
        fields.push(Field {
            name,
            location,
            length,
            data_type,
            unit: elem_text(f, "unit"),
        });
    }
    if fields.is_empty() {
        return None;
    }
    let target_name = elem_child(&root, "Observation_Area")
        .and_then(|o| elem_child(o, "Target_Identification"))
        .and_then(|t| elem_text(t, "name"));
    let file_name = elem_child(fao, "File").and_then(|f| elem_text(f, "file_name"));
    Some(BinaryLabel {
        record_length,
        file_name,
        target_name,
        fields,
    })
}

fn decode_num(field: &[u8], data_type: &str) -> Option<f64> {
    let v = match data_type {
        "IEEE754LSBDouble" => f64::from_le_bytes(field.try_into().ok()?),
        "SignedLSB2" => i16::from_le_bytes(field.try_into().ok()?) as f64,
        _ => return None,
    };
    v.is_finite().then_some(v)
}

fn is_numeric(data_type: &str) -> bool {
    matches!(data_type, "IEEE754LSBDouble" | "SignedLSB2")
}

fn column_of(f: &Field) -> Pds4Column {
    Pds4Column {
        name: f.name.clone(),
        unit: f.unit.clone(),
        data_type: Some(f.data_type.clone()),
        missing_constant: None,
        sampling_name: String::new(),
        sampling_unit: String::new(),
        sampling_min: None,
        sampling_max: None,
        start_byte: Some(f.location),
        bytes: Some(f.length),
    }
}

fn axis_column(f: &Field) -> Pds4Column {
    Pds4Column {
        name: f.name.clone(),
        unit: Some("SECOND".to_string()),
        data_type: Some("TIME".to_string()),
        missing_constant: None,
        sampling_name: String::new(),
        sampling_unit: String::new(),
        sampling_min: None,
        sampling_max: None,
        start_byte: Some(f.location),
        bytes: Some(f.length),
    }
}

fn build_table(label: &BinaryLabel, data: &[u8]) -> Option<(Pds4Table, usize, usize)> {
    let rl = label.record_length;
    let complete = data.len() / rl;
    if complete == 0 {
        return None;
    }
    let trailing = data.len() - complete * rl;
    let axis = label
        .fields
        .iter()
        .position(|f| f.name.eq_ignore_ascii_case("et") && is_numeric(&f.data_type));
    let mut columns: Vec<Pds4Column> = Vec::new();
    let mut order: Vec<usize> = Vec::new();
    if let Some(i) = axis {
        columns.push(axis_column(&label.fields[i]));
        order.push(i);
    }
    for (i, f) in label.fields.iter().enumerate() {
        if Some(i) == axis || !is_numeric(&f.data_type) {
            continue;
        }
        columns.push(column_of(f));
        order.push(i);
    }
    if columns.is_empty() {
        return None;
    }
    let mut rows: Vec<Pds4Row> = Vec::with_capacity(complete);
    let mut skipped = 0usize;
    for r in 0..complete {
        let at = r * rl;
        let rec = &data[at..at + rl];
        let mut values: Vec<Option<f64>> = Vec::with_capacity(order.len());
        for i in &order {
            let f = &label.fields[*i];
            let from = f.location - 1;
            let v = rec
                .get(from..from + f.length)
                .and_then(|cell| decode_num(cell, &f.data_type));
            values.push(v);
        }
        if axis.is_some() {
            if values.first().copied().flatten().is_none() {
                skipped += 1;
                continue;
            }
        } else if values.iter().all(|v| v.is_none()) {
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
        trailing,
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
    label: &BinaryLabel,
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
        cols.push(s);
    }
    let target = match label.target_name.as_deref() {
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
    let Some(label) = parse_binary_label(label_text) else {
        eprintln!("label parse void — no Table_Binary record ({label_spec})");
        return None;
    };
    let Some(dat_bytes) = fetch_or_read(dat_spec) else {
        eprintln!("data fetch void ({dat_spec})");
        return None;
    };
    let file_name = label.file_name.as_deref().unwrap_or("");
    if !file_name.is_empty()
        && !dat_spec
            .to_ascii_lowercase()
            .ends_with(&file_name.to_ascii_lowercase())
    {
        eprintln!(
            "{label_spec}: file_name {file_name} does not name {dat_spec} — label/data unpaired"
        );
        return None;
    }
    let Some((table, skipped, trailing)) = build_table(&label, &dat_bytes) else {
        eprintln!(
            "{} byte(s) carry no binary record of {} byte(s) — the table stays unwritten (0 honored)",
            dat_bytes.len(),
            label.record_length
        );
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
    print_inventory(&asset, &table, &label, skipped, trailing);
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
            collect_pairs(OLA_ROUTE, "dat", ".xml", &mut pairs);
            false
        }
    };
    if pairs.is_empty() {
        eprintln!("no .xml/.dat pair found — nothing written (0 honored)");
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
