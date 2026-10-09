use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const MAGIC: &[u8; 4] = b"BVFR";
const VERSION: u32 = 1;
const NETLOC: &str = "blinkverse.zero2x.org";
const BASE: &str = "https://blinkverse.zero2x.org/api/app/adcp-blinkverse/type";
const DEFAULT_TYPE: &str = "FRB_SOURCE";
const COMPILER: &str = "tools/harvest/src/bin/blinkverse_compiler.rs";

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

fn source_url(args: &[String]) -> String {
    match arg_value(args, "--url") {
        Some(u) => u,
        None => {
            let t = match arg_value(args, "--type") {
                Some(t) => t,
                None => DEFAULT_TYPE.to_string(),
            };
            format!("{BASE}/{t}/download")
        }
    }
}

fn split_fields(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_quotes = true;
        } else if c == ',' {
            fields.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    fields.push(cur);
    fields
}

fn cell_num(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn parse_csv(text: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let mut lines = text.lines();
    let header_line = lines.next()?;
    let header_line = header_line
        .trim_start_matches('\u{feff}')
        .trim_end_matches('\r');
    if header_line.is_empty() {
        return None;
    }
    let header = split_fields(header_line);
    let mut rows = Vec::new();
    for line in lines {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }
        rows.push(split_fields(line));
    }
    Some((header, rows))
}

fn numeric_columns(header: &[String], rows: &[Vec<String>]) -> Vec<usize> {
    header
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            rows.iter()
                .any(|r| r.get(*i).and_then(|c| cell_num(c)).is_some())
        })
        .map(|(i, _)| i)
        .collect()
}

fn keep_table(header: &[String], rows: &[Vec<String>]) -> Table {
    let cols = numeric_columns(header, rows);
    let names = cols.iter().map(|i| header[*i].clone()).collect();
    let out_rows = rows
        .iter()
        .map(|r| {
            cols.iter()
                .map(|i| r.get(*i).and_then(|c| cell_num(c)))
                .collect()
        })
        .collect();
    Table {
        names,
        rows: out_rows,
    }
}

fn write_bin(table: &Table) -> Vec<u8> {
    let n_cols = table.names.len();
    let words = (n_cols + 63) / 64;
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
    let words = (n_cols + 63) / 64;
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

fn stat(src: &str, header: &[String], rows: &[Vec<String>]) {
    let cols = numeric_columns(header, rows);
    eprintln!(
        "{src}: {} data row(s), {} column(s), {} numeric column(s)",
        rows.len(),
        header.len(),
        cols.len()
    );
    for i in &cols {
        eprintln!("  numeric column[{i}] = {}", header[*i]);
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
    println!("format blinkverse_frb");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let stat_mode = args.iter().any(|a| a == "--stat");
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => "blinkverse_frb.bin".to_string(),
    };
    let (src, text) = match arg_value(&args, "--file") {
        Some(path) => match std::fs::read(&path) {
            Ok(bytes) => (path, String::from_utf8_lossy(&bytes).into_owned()),
            Err(_) => {
                eprintln!("{path}: the CSV stays unread");
                std::process::exit(1);
            }
        },
        None => {
            let url = source_url(&args);
            match fetch_raw_bytes(&url) {
                Some(bytes) => {
                    let text = String::from_utf8_lossy(&bytes).into_owned();
                    (url, text)
                }
                None => {
                    eprintln!("{url}: the CSV stays unfetched");
                    std::process::exit(1);
                }
            }
        }
    };
    let Some((header, rows)) = parse_csv(&text) else {
        eprintln!("{src}: the header row is absent — the CSV stays unparsed");
        std::process::exit(1);
    };
    if stat_mode {
        stat(&src, &header, &rows);
        return;
    }
    let table = keep_table(&header, &rows);
    emit(&src, &out, &table);
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> Table {
        Table {
            names: vec!["a".to_string(), "b".to_string()],
            rows: vec![vec![Some(1.5), None], vec![None, Some(-2.0)]],
        }
    }

    #[test]
    fn roundtrip_preserves_absence() {
        let t = table();
        let bytes = write_bin(&t);
        assert_eq!(parse_bin(&bytes), Some(t));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_truncation() {
        assert!(parse_bin(b"XXXX").is_none());
        let good = write_bin(&table());
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }

    #[test]
    fn numeric_columns_keep_only_value_bearing() {
        let header = vec![
            "Source".to_string(),
            "DM".to_string(),
            "RA".to_string(),
            "Err".to_string(),
        ];
        let rows = vec![
            vec![
                "FRB1".to_string(),
                "460.8".to_string(),
                "22:17:30.0".to_string(),
                "+0.18/-0.18".to_string(),
            ],
            vec![
                "FRB2".to_string(),
                "".to_string(),
                "01:00:00.0".to_string(),
                "+0.2/-0.2".to_string(),
            ],
        ];
        assert_eq!(numeric_columns(&header, &rows), vec![1]);
    }

    #[test]
    fn split_fields_keeps_quoted_commas() {
        let row = split_fields("\"a,b\",1,[FRB20121102A],x");
        assert_eq!(row, vec!["a,b", "1", "[FRB20121102A]", "x"]);
    }
}
