use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds4::{
    Pds4Column, Pds4Meta, Pds4Table, assemble, axis_of, decode_rows, pack, parse_label_for_file,
    parse_table,
};
use omegaflow::archivar::quantity_kind_for_unit;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::force::quantity_kind_name;
use omegaflow::json::{JsonVal, parse_json};
use std::collections::{BTreeSet, HashMap};
use std::process::Command;

const NETLOC: &str = "pds-ppi.igpp.ucla.edu";
const TAP_DEFAULT: &str = "https://vo-pds-ppi.igpp.ucla.edu/tap/sync";
const EPN_SUFFIX: &str = ".epn_core";

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

fn file_basename(spec: &str) -> &str {
    let base = spec.rsplit('/').next().unwrap_or(spec);
    base.split_once('?').map(|(head, _)| head).unwrap_or(base)
}

fn tap_query(root: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("300")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=json")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(root)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "pds_ppi tap http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn as_arr(j: &JsonVal) -> Option<&Vec<JsonVal>> {
    match j {
        JsonVal::Arr(a) => Some(a),
        _ => None,
    }
}

fn as_obj(j: &JsonVal) -> Option<&HashMap<String, JsonVal>> {
    match j {
        JsonVal::Obj(o) => Some(o),
        _ => None,
    }
}

fn cell_str(c: &JsonVal) -> String {
    match c {
        JsonVal::Str(s) => s.clone(),
        JsonVal::Num(v) => format!("{}", v),
        _ => String::new(),
    }
}

fn col_pos(fields: &Option<Vec<String>>, name: &str, fallback: usize) -> Option<usize> {
    match fields {
        Some(fs) => fs.iter().position(|f| f.eq_ignore_ascii_case(name)),
        None => Some(fallback),
    }
}

fn rows_from_json(body: &str) -> Option<(Option<Vec<String>>, Vec<Vec<String>>)> {
    let parsed = parse_json(body)?;
    let JsonVal::Obj(m) = &parsed else {
        eprintln!("pds_ppi tap body is not an object: {} bytes", body.len());
        return None;
    };
    let fields: Option<Vec<String>> = m.get("columns").and_then(as_arr).map(|cols| {
        cols.iter()
            .filter_map(|c| {
                as_obj(c).and_then(|o| match o.get("name") {
                    Some(JsonVal::Str(s)) => Some(s.clone()),
                    _ => None,
                })
            })
            .collect()
    });
    let rows: Vec<Vec<String>> = match m.get("data").and_then(as_arr) {
        Some(data) => data
            .iter()
            .filter_map(as_arr)
            .map(|r| r.iter().map(cell_str).collect())
            .collect(),
        None => Vec::new(),
    };
    Some((fields, rows))
}

fn tap_rows(root: &str, adql: &str) -> Option<(Option<Vec<String>>, Vec<Vec<String>>)> {
    let body = tap_query(root, adql)?;
    rows_from_json(&body)
}

fn pair_from_url(url: &str) -> Option<(String, String)> {
    let path = url.split_once('?').map(|(h, _)| h).unwrap_or(url);
    if !path.to_ascii_lowercase().ends_with(".tab") {
        return None;
    }
    let stem = &path[..path.len() - 4];
    Some((format!("{stem}.xml"), url.to_string()))
}

fn collect_tap_pairs(root: &str, table: &str, out: &mut Vec<(String, String)>) {
    let adql =
        format!("SELECT granule_uid,access_url,time_min,time_max FROM {table} ORDER BY access_url");
    let Some((fields, rows)) = tap_rows(root, &adql) else {
        eprintln!("{table}: the epn_core inventory stays unread");
        return;
    };
    let Some(u) = col_pos(&fields, "access_url", 1) else {
        eprintln!("{table}: no access_url column in the epn_core response");
        return;
    };
    let mut seen = BTreeSet::new();
    for cells in &rows {
        let Some(url) = cells.get(u) else {
            continue;
        };
        if url.is_empty() {
            continue;
        }
        let Some(pair) = pair_from_url(url) else {
            continue;
        };
        if seen.insert(pair.1.clone()) {
            out.push(pair);
        }
    }
}

fn list_tables(root: &str) -> Option<Vec<String>> {
    let adql = format!(
        "SELECT table_name FROM tap_schema.tables WHERE table_name LIKE '%{EPN_SUFFIX}' ORDER BY table_name"
    );
    let (_, rows) = tap_rows(root, &adql)?;
    Some(rows.iter().filter_map(|c| c.first().cloned()).collect())
}

fn asset_name(dat_spec: &str) -> String {
    let after_scheme = match dat_spec.split_once("://") {
        Some((_, rest)) => {
            let mut it = rest.split('/');
            it.next();
            it.filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
                .join("_")
        }
        None => dat_spec
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("_"),
    };
    let low = after_scheme.to_ascii_lowercase();
    let stem = low.strip_suffix(".tab").unwrap_or(&low);
    format!("pds_ppi_{stem}.bin")
}

fn print_register_lines(asset: &str) {
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{asset}");
    println!("format pds4_fixed_width");
    println!("ttl 604800");
}

const QUANTITY_KERNEL: &str = "point";
const QUANTITY_TAU: f64 = 604800.0;

fn register_token(name: &str) -> String {
    let mut token = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            token.push(ch.to_ascii_lowercase());
        } else if !token.is_empty() && !token.ends_with('_') {
            token.push('_');
        }
    }
    token.trim_matches('_').to_string()
}

fn quantity_line(column: &Pds4Column, tau: f64) -> Option<String> {
    let unit = column.unit.as_deref()?.trim();
    if unit.is_empty() {
        return None;
    }
    let (kind, canonical) = quantity_kind_for_unit(unit)?;
    let token = register_token(&column.name);
    if token.is_empty() {
        return None;
    }
    Some(format!(
        "quantity {token} {token} {QUANTITY_KERNEL} {} {canonical} {tau} 0.0 0.0",
        quantity_kind_name(kind),
    ))
}

fn print_quantity_lines(table: &Pds4Table) {
    for column in &table.columns {
        match quantity_line(column, QUANTITY_TAU) {
            Some(line) => println!("{line}"),
            None => eprintln!(
                "{}: no registerable unit (\"{}\") — quantity stays pending",
                column.name,
                column.unit.as_deref().unwrap_or("")
            ),
        }
    }
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

fn family_name(table: &str) -> String {
    let stem = table.strip_suffix(EPN_SUFFIX).unwrap_or(table);
    format!("pds_ppi_{stem}.manifest")
}

fn write_manifest(
    family: &str,
    shards: &[(String, String)],
    out_dir: Option<&str>,
    ci_mode: bool,
) -> bool {
    let dir = match out_dir {
        Some(d) => d.trim_end_matches('/').to_string(),
        None => format!("data/{NETLOC}/pds4_fixed_width"),
    };
    let path = format!("{dir}/{family}");
    let mut text = String::new();
    for (name, sha) in shards {
        text.push_str(&format!("{name} {sha}\n"));
    }
    if std::fs::write(&path, &text).is_err() {
        eprintln!("{family}: write {path} returned void");
        return false;
    }
    eprintln!("{family}: {} shard(s) listed", shards.len());
    if ci_mode && !upload_release(NETLOC, &path) {
        eprintln!("{family}: CDN upload returned void");
        return false;
    }
    true
}

fn compile_entry(
    dat_spec: &str,
    label_spec: &str,
    out_dir: Option<&str>,
    ci_mode: bool,
) -> Option<(String, String)> {
    let Some(label_bytes) = fetch_or_read(label_spec) else {
        eprintln!("label fetch void ({label_spec})");
        return None;
    };
    let Ok(label_text) = std::str::from_utf8(&label_bytes) else {
        eprintln!("label not utf8 ({label_spec})");
        return None;
    };
    let Some(meta) = parse_label_for_file(label_text, Some(file_basename(dat_spec))) else {
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
    let out_path = match out_dir {
        Some(d) => format!("{}/{asset}", d.trim_end_matches('/')),
        None => format!("data/{NETLOC}/pds4_fixed_width/{asset}"),
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
    let sha = sha256_hex(&bin);
    eprintln!(
        "{out_path}: {} row(s) packed, {} byte(s), sha256 {sha}, roundtrip holds",
        table.rows.len(),
        bin.len()
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
    print_quantity_lines(&table);
    println!();
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("{asset}: CDN upload returned void");
        return None;
    }
    Some((asset, sha))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: pds_ppi_compiler (--table <t> | --all | --list) [--pairs] [--tap <root>] [--out <dir>] [--ci-mode]";
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_arg = arg_value(&args, "--out");
    let root = match arg_value(&args, "--tap") {
        Some(value) => value,
        None => TAP_DEFAULT.to_string(),
    };

    if args.iter().any(|a| a == "--list") {
        match list_tables(&root) {
            Some(tables) => {
                for t in &tables {
                    println!("{t}");
                }
            }
            None => {
                eprintln!("the EPN-TAP table inventory stays unread");
                std::process::exit(1);
            }
        }
        return;
    }

    let mut tables: Vec<String> = Vec::new();
    if args.iter().any(|a| a == "--all") {
        match list_tables(&root) {
            Some(ts) => tables = ts,
            None => {
                eprintln!("the EPN-TAP table inventory stays unread");
                std::process::exit(1);
            }
        }
    } else if let Some(t) = arg_value(&args, "--table") {
        tables.push(t);
    } else {
        eprintln!("{usage}");
        std::process::exit(2);
    }

    let mut per_table: Vec<(String, Vec<(String, String)>)> = Vec::new();
    let mut found = 0usize;
    for t in &tables {
        let mut table_pairs: Vec<(String, String)> = Vec::new();
        collect_tap_pairs(&root, t, &mut table_pairs);
        eprintln!("{t}: {} pair(s)", table_pairs.len());
        found += table_pairs.len();
        per_table.push((t.clone(), table_pairs));
    }
    if found == 0 {
        eprintln!("no .TAB/.xml pair found — nothing written (0 honored)");
        std::process::exit(1);
    }
    if args.iter().any(|a| a == "--pairs") {
        for (_, pairs) in &per_table {
            for (label, dat) in pairs {
                println!("{label}\t{dat}");
            }
        }
        return;
    }
    let mut written = 0usize;
    for (table, pairs) in &per_table {
        let mut shards: Vec<(String, String)> = Vec::new();
        for (label_spec, dat_spec) in pairs {
            if let Some(entry) = compile_entry(dat_spec, label_spec, out_arg.as_deref(), ci_mode) {
                shards.push(entry);
                written += 1;
            }
        }
        if !shards.is_empty() {
            write_manifest(&family_name(table), &shards, out_arg.as_deref(), ci_mode);
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
    fn pair_from_url_derives_the_sibling_label() {
        let url = "https://pds-ppi.igpp.ucla.edu/data/galileo-hic-jup-raw/data/ORB_29_HIGH_RES_UNCALIB.TAB";
        let (label, dat) = pair_from_url(url).unwrap();
        assert_eq!(
            label,
            "https://pds-ppi.igpp.ucla.edu/data/galileo-hic-jup-raw/data/ORB_29_HIGH_RES_UNCALIB.xml"
        );
        assert_eq!(dat, url);
    }

    #[test]
    fn pair_from_url_skips_a_non_tab_url() {
        assert!(pair_from_url("https://example.org/index.html").is_none());
        assert!(pair_from_url("https://example.org/a.tab").is_some());
    }

    #[test]
    fn asset_name_carries_the_bundle_and_stem() {
        let url = "https://pds-ppi.igpp.ucla.edu/data/galileo-hic-jup-raw/data/ORB_29_HIGH_RES_UNCALIB.TAB";
        assert_eq!(
            asset_name(url),
            "pds_ppi_data_galileo-hic-jup-raw_data_orb_29_high_res_uncalib.bin"
        );
    }

    #[test]
    fn family_name_strips_the_epn_suffix() {
        assert_eq!(
            family_name("galileo_hic_jup_raw.epn_core"),
            "pds_ppi_galileo_hic_jup_raw.manifest"
        );
        assert_eq!(family_name("odd"), "pds_ppi_odd.manifest");
    }

    #[test]
    fn rows_from_json_reads_the_columns_and_data_shape() {
        let body = r#"{"columns":[{"name":"granule_uid"},{"name":"access_url"},{"name":"time_min"}],"data":[["u1","https://x/a.TAB",2451907.16],["u2","https://x/b.TAB",2451907.2]]}"#;
        let (fields, rows) = rows_from_json(body).unwrap();
        let fields = fields.unwrap();
        assert_eq!(col_pos(&Some(fields.clone()), "access_url", 0), Some(1));
        assert_eq!(col_pos(&Some(fields.clone()), "ACCESS_URL", 0), Some(1));
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][1], "https://x/a.TAB");
        assert_eq!(cell_str(&JsonVal::Num(2451907.16)), "2451907.16");
    }

    #[test]
    fn rows_from_json_yields_no_rows_for_a_body_without_data() {
        let (_, rows) = rows_from_json("{}").unwrap();
        assert!(rows.is_empty());
    }

    fn column(name: &str, unit: Option<&str>) -> Pds4Column {
        Pds4Column {
            name: name.to_string(),
            unit: unit.map(|u| u.to_string()),
            data_type: None,
            missing_constant: None,
            sampling_name: String::new(),
            sampling_unit: String::new(),
            sampling_min: None,
            sampling_max: None,
            start_byte: None,
            bytes: None,
        }
    }

    #[test]
    fn quantity_line_is_projected_from_the_measured_label_unit() {
        assert_eq!(
            quantity_line(&column("BR", Some("nT")), 604800.0).unwrap(),
            "quantity br br point index nt 604800 0.0 0.0"
        );
        assert_eq!(
            quantity_line(&column("ZXX_RE", Some("Ohm")), 604800.0).unwrap(),
            "quantity zxx_re zxx_re point impedance ohm 604800 0.0 0.0"
        );
    }

    #[test]
    fn quantity_line_stays_absent_without_a_registerable_unit() {
        assert!(quantity_line(&column("TIME", None), 604800.0).is_none());
        assert!(quantity_line(&column("RANGE", Some("Saturn Radii")), 604800.0).is_none());
        assert!(quantity_line(&column("LATITUDE", Some("Degree")), 604800.0).is_none());
        assert!(quantity_line(&column("LOCAL HOUR", Some("hr")), 604800.0).is_none());
    }

    #[test]
    fn register_token_folds_whitespace_to_an_underscore() {
        assert_eq!(register_token("EAST LONGITUDE"), "east_longitude");
        assert_eq!(register_token("BR"), "br");
    }
}
