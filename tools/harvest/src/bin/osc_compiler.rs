use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sexagesimal::{sexagesimal_dec_to_deg, sexagesimal_ra_to_deg};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{JsonVal, jnum, jstr, parse_json};
use omegaflow::cdn::upload_release;

const MAGIC: &[u8; 4] = b"OSC1";
const VERSION: u32 = 1;
const NETLOC: &str = "sne.space";
const API_BASE: &str = "https://sne.space/api/transients?format=json&limit=250&page=";
const COMPILER: &str = "tools/harvest/src/bin/osc_compiler.rs";
const FORMAT: &str = "osc_transients";

const COL_RA: &str = "ra";
const COL_DEC: &str = "dec";
const COL_Z: &str = "redshift";
const COL_MAG: &str = "max_app_mag";

#[derive(Clone, PartialEq, Debug)]
struct Table {
    names: Vec<String>,
    rows: Vec<Vec<Option<f64>>>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn ra_deg(s: &str) -> Option<f64> {
    let t = s.trim();
    match t.parse::<f64>() {
        Ok(v) if v.is_finite() => Some(v),
        _ => sexagesimal_ra_to_deg(t),
    }
}

fn dec_deg(s: &str) -> Option<f64> {
    let t = s.trim();
    match t.parse::<f64>() {
        Ok(v) if v.is_finite() => Some(v),
        _ => sexagesimal_dec_to_deg(t),
    }
}

fn item_row(item: &JsonVal) -> Vec<Option<f64>> {
    let ra = match jstr(item, COL_RA) {
        Some(s) => ra_deg(&s),
        None => None,
    };
    let dec = match jstr(item, COL_DEC) {
        Some(s) => dec_deg(&s),
        None => None,
    };
    let z = match jnum(item, "z") {
        Some(v) if v.is_finite() => Some(v),
        _ => None,
    };
    let mag = match jnum(item, "mag") {
        Some(v) if v.is_finite() => Some(v),
        _ => None,
    };
    vec![ra, dec, z, mag]
}

fn page_items(root: &JsonVal) -> Vec<&JsonVal> {
    match root {
        JsonVal::Obj(map) => match map.get("items") {
            Some(JsonVal::Arr(arr)) => arr.iter().collect(),
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}

fn has_next(root: &JsonVal) -> bool {
    match jnum(root, "has_next") {
        Some(v) => v != 0.0,
        None => false,
    }
}

fn observable_table(items: &[&JsonVal]) -> Table {
    let names = vec![
        COL_RA.to_string(),
        COL_DEC.to_string(),
        COL_Z.to_string(),
        COL_MAG.to_string(),
    ];
    let rows = items.iter().map(|item| item_row(item)).collect();
    Table { names, rows }
}

fn fetch_pages(base: &str, max_pages: usize) -> Option<Vec<JsonVal>> {
    let mut roots = Vec::new();
    let mut page = 1usize;
    while page <= max_pages {
        let url = format!("{base}{page}");
        let bytes = fetch_raw_bytes(&url)?;
        let text = String::from_utf8_lossy(&bytes).into_owned();
        let root = parse_json(&text)?;
        let next = has_next(&root);
        roots.push(root);
        if !next {
            break;
        }
        page += 1;
    }
    Some(roots)
}

fn write_bin(table: &Table) -> Vec<u8> {
    let n_cols = table.names.len();
    let words = n_cols.div_ceil(64);
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&(n_cols as u32).to_le_bytes());
    out.extend_from_slice(&(table.rows.len() as u32).to_le_bytes());
    for name in &table.names {
        let b = name.as_bytes();
        out.extend_from_slice(&(b.len() as u16).to_le_bytes());
        out.extend_from_slice(b);
    }
    for row in &table.rows {
        let mut mask = vec![0u64; words];
        for (j, cell) in row.iter().enumerate() {
            if cell.is_some() {
                mask[j / 64] |= 1u64 << (j % 64);
            }
        }
        for w in &mask {
            out.extend_from_slice(&w.to_le_bytes());
        }
        for cell in row {
            if let Some(v) = cell {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Table> {
    if bytes.len() < 16 || &bytes[0..4] != MAGIC {
        return None;
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().ok()?);
    if version != VERSION {
        return None;
    }
    let n_cols = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    let n_rows = u32::from_le_bytes(bytes[12..16].try_into().ok()?) as usize;
    if n_cols == 0 || n_cols > bytes.len() / 2 {
        return None;
    }
    let mut pos = 16usize;
    let mut names = Vec::with_capacity(n_cols);
    for _ in 0..n_cols {
        let len = u16::from_le_bytes(bytes.get(pos..pos + 2)?.try_into().ok()?) as usize;
        pos += 2;
        let raw = bytes.get(pos..pos + len)?;
        names.push(String::from_utf8(raw.to_vec()).ok()?);
        pos += len;
    }
    let words = n_cols.div_ceil(64);
    let min_row = words.checked_mul(8)?;
    if bytes.len().checked_sub(pos)? < n_rows.checked_mul(min_row)? {
        return None;
    }
    let mut rows = Vec::with_capacity(n_rows);
    for _ in 0..n_rows {
        let mut mask = vec![0u64; words];
        for w in mask.iter_mut() {
            *w = u64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
        }
        let mut row = vec![None; n_cols];
        for (j, slot) in row.iter_mut().enumerate() {
            if mask[j / 64] & (1u64 << (j % 64)) != 0 {
                let v = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
                pos += 8;
                if !v.is_finite() {
                    return None;
                }
                *slot = Some(v);
            }
        }
        rows.push(row);
    }
    if pos != bytes.len() {
        return None;
    }
    Some(Table { names, rows })
}

fn stat(src: &str, table: &Table) {
    eprintln!("{src}: {} data row(s)", table.rows.len());
    for (j, name) in table.names.iter().enumerate() {
        let present = table.rows.iter().filter(|r| r[j].is_some()).count();
        eprintln!("  {name}: {present} value(s)");
    }
}

fn emit(src: &str, out: &str, table: &Table) {
    if table.rows.is_empty() {
        eprintln!("{out}: 0 rows — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes_out = write_bin(table);
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes_out) {
        Some(parsed) if parsed == *table => {
            eprintln!(
                "{out}: {} row(s), {} column(s), {} B — roundtrip parses",
                table.rows.len(),
                table.names.len(),
                bytes_out.len()
            );
        }
        Some(parsed) => {
            eprintln!(
                "{out}: roundtrip parses {} row(s) but differs from the emitted set",
                parsed.rows.len()
            );
            std::process::exit(1);
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    let out_name = match std::path::Path::new(out).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => out.to_string(),
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{out_name}");
    println!("origin {src}");
    println!("compiler {COMPILER}");
    println!("sha256 {}", sha256_hex(&bytes_out));
    println!("format {FORMAT}");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let stat_mode = args.iter().any(|a| a == "--stat");
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => "osc_transients.bin".to_string(),
    };
    let max_pages = match arg_value(&args, "--max-pages") {
        Some(v) => match v.parse::<usize>() {
            Ok(n) => n,
            Err(_) => 100_000,
        },
        None => 100_000,
    };
    let (src, roots): (String, Vec<JsonVal>) = match arg_value(&args, "--file") {
        Some(path) => match std::fs::read(&path) {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes).into_owned();
                match parse_json(&text) {
                    Some(root) => (path, vec![root]),
                    None => {
                        eprintln!("{path}: the JSON stays unparsed");
                        std::process::exit(1);
                    }
                }
            }
            Err(_) => {
                eprintln!("{path}: the JSON stays unread");
                std::process::exit(1);
            }
        },
        None => {
            let base = match arg_value(&args, "--url") {
                Some(u) => u,
                None => API_BASE.to_string(),
            };
            match fetch_pages(&base, max_pages) {
                Some(pages) => (base, pages),
                None => {
                    eprintln!("{base}: the JSON stays unfetched");
                    std::process::exit(1);
                }
            }
        }
    };
    let items: Vec<&JsonVal> = roots.iter().flat_map(page_items).collect();
    let table = observable_table(&items);
    if stat_mode {
        stat(&src, &table);
        return;
    }
    emit(&src, &out, &table);
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page_fixture() -> &'static str {
        r#"{"total":2,"page":1,"limit":250,"pages":1,"has_next":false,"items":[
            {"name":"SN1","ra":"03:16:41.828","dec":"+80:48:09.12","z":0.0077,"mag":19.08},
            {"name":"SN2","ra":"02:25:05.075","dec":"+22:13:11.52","z":null,"mag":null}
        ]}"#
    }

    #[test]
    fn roundtrip_preserves_absence() {
        let t = Table {
            names: vec!["a".to_string(), "b".to_string()],
            rows: vec![vec![Some(1.5), None], vec![None, Some(-2.0)]],
        };
        let bytes = write_bin(&t);
        assert_eq!(parse_bin(&bytes), Some(t));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_truncation() {
        assert!(parse_bin(b"XXXX").is_none());
        let good = write_bin(&Table {
            names: vec!["a".to_string()],
            rows: vec![vec![Some(1.0)]],
        });
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }

    #[test]
    fn page_items_reads_sexagesimal_position_and_magnitude() {
        let root = parse_json(page_fixture()).unwrap();
        assert!(!has_next(&root));
        let items = page_items(&root);
        assert_eq!(items.len(), 2);
        let t = observable_table(&items);
        assert_eq!(t.names, vec!["ra", "dec", "redshift", "max_app_mag"]);
        assert!((t.rows[0][0].unwrap() - 49.1742834).abs() < 1e-4);
        assert!((t.rows[0][1].unwrap() - 80.8025333).abs() < 1e-4);
        assert_eq!(t.rows[0][2], Some(0.0077));
        assert_eq!(t.rows[0][3], Some(19.08));
    }

    #[test]
    fn absent_magnitude_and_redshift_stay_none() {
        let root = parse_json(page_fixture()).unwrap();
        let items = page_items(&root);
        let t = observable_table(&items);
        assert_eq!(t.rows[1][2], None);
        assert_eq!(t.rows[1][3], None);
    }
}
