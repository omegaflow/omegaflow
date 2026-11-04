use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;

const VERSION: u32 = 1;

#[derive(Clone, PartialEq, Debug)]
pub struct Table {
    pub names: Vec<String>,
    pub rows: Vec<Vec<Option<f64>>>,
}

pub fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

pub fn arg_or(args: &[String], name: &str, default: &str) -> String {
    match arg_value(args, name) {
        Some(v) => v,
        None => default.to_string(),
    }
}

pub fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

pub fn cell_num(s: &str) -> Option<f64> {
    let t = s.trim().trim_matches('"').trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn is_comment(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with('#') || t.starts_with('!') || t.starts_with('|') || t.starts_with("//")
}

pub fn sanitize_name(s: &str, idx: usize) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    let trimmed = out.trim_matches('_');
    if trimmed.is_empty() {
        format!("col{idx}")
    } else {
        trimmed.to_string()
    }
}

fn split_tokens(line: &str) -> Vec<String> {
    let has_comma = line.contains(',');
    let sep = |c: char| {
        if has_comma {
            c == ','
        } else {
            c.is_whitespace()
        }
    };
    line.split(sep)
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn parse_delimited_table(text: &str) -> Option<Table> {
    let mut kept: Vec<Vec<String>> = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() || is_comment(line) {
            continue;
        }
        let toks = split_tokens(line);
        if !toks.is_empty() {
            kept.push(toks);
        }
    }
    if kept.is_empty() {
        return None;
    }
    let header_is_numeric = kept[0].iter().all(|t| cell_num(t).is_some());
    let (names, start) = if header_is_numeric {
        (
            (0..kept[0].len())
                .map(|i| format!("col{i}"))
                .collect::<Vec<_>>(),
            0usize,
        )
    } else {
        (
            kept[0]
                .iter()
                .enumerate()
                .map(|(i, s)| sanitize_name(s, i))
                .collect::<Vec<_>>(),
            1usize,
        )
    };
    if names.is_empty() {
        return None;
    }
    let ncols = names.len();
    let mut rows = Vec::new();
    for toks in kept.iter().skip(start) {
        let mut row = vec![None; ncols];
        for (j, slot) in row.iter_mut().enumerate() {
            if let Some(tok) = toks.get(j) {
                *slot = cell_num(tok);
            }
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return None;
    }
    Some(Table { names, rows })
}

pub fn write_bin(magic: [u8; 4], table: &Table) -> Vec<u8> {
    let n_cols = table.names.len();
    let words = n_cols.div_ceil(64);
    let mut out = Vec::new();
    out.extend_from_slice(&magic);
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

pub fn parse_bin(magic: [u8; 4], bytes: &[u8]) -> Option<Table> {
    if bytes.len() < 16 || bytes[0..4] != magic {
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

pub fn stat(src: &str, table: &Table) {
    eprintln!("{src}: {} data row(s)", table.rows.len());
    for (j, name) in table.names.iter().enumerate() {
        let present = table.rows.iter().filter(|r| r[j].is_some()).count();
        eprintln!("  {name}: {present} value(s)");
    }
}

pub fn load_source(args: &[String], default_url: &str) -> Option<(String, Vec<u8>)> {
    if let Some(path) = arg_value(args, "--file") {
        match std::fs::read(&path) {
            Ok(bytes) => Some((path, bytes)),
            Err(_) => {
                eprintln!("{path}: the file stays unread");
                None
            }
        }
    } else {
        let url = arg_or(args, "--url", default_url);
        match fetch_raw_bytes(&url) {
            Some(bytes) => Some((url, bytes)),
            None => {
                eprintln!("{url}: the source stays unfetched");
                None
            }
        }
    }
}

pub fn emit(
    src: &str,
    out: &str,
    table: &Table,
    magic: [u8; 4],
    netloc: &str,
    format: &str,
    compiler: &str,
) -> bool {
    if table.rows.is_empty() || table.names.is_empty() {
        eprintln!("{out}: 0 rows — the bin stays unwritten (0 honored)");
        return false;
    }
    let bytes_out = write_bin(magic, table);
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        return false;
    }
    match parse_bin(magic, &bytes_out) {
        Some(parsed) if parsed == *table => {}
        Some(_) => {
            eprintln!("{out}: roundtrip parses but differs from the emitted set");
            return false;
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            return false;
        }
    }
    eprintln!(
        "{out}: {} row(s), {} column(s), {} B — roundtrip parses",
        table.rows.len(),
        table.names.len(),
        bytes_out.len()
    );
    let out_name = match std::path::Path::new(out).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => out.to_string(),
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{netloc}/{out_name}");
    println!("origin {src}");
    println!("compiler {compiler}");
    println!("sha256 {}", sha256_hex(&bytes_out));
    println!("format {format}");
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-12
    }

    #[test]
    fn roundtrip_preserves_absence() {
        let t = Table {
            names: vec!["a".to_string(), "b".to_string()],
            rows: vec![vec![Some(1.5), None], vec![None, Some(-2.0)]],
        };
        assert_eq!(parse_bin(*b"TEST", &write_bin(*b"TEST", &t)), Some(t));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_truncation() {
        assert!(parse_bin(*b"TEST", b"XXXX").is_none());
        let good = write_bin(
            *b"TEST",
            &Table {
                names: vec!["a".to_string()],
                rows: vec![vec![Some(1.0)]],
            },
        );
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(*b"TEST", &short).is_none());
    }

    #[test]
    fn delimited_header_and_rows() {
        let text = "# comment\nra dec flux\n1.0 2.0 3.5\n4.0 5.0 nan\n";
        let t = parse_delimited_table(text).unwrap();
        assert_eq!(t.names, vec!["ra", "dec", "flux"]);
        assert_eq!(t.rows.len(), 2);
        assert!(approx(t.rows[0][2].unwrap(), 3.5));
        assert_eq!(t.rows[1][2], None);
    }

    #[test]
    fn delimited_numeric_first_row_gets_generated_names() {
        let t = parse_delimited_table("1,2\n3,4\n").unwrap();
        assert_eq!(t.names, vec!["col0", "col1"]);
        assert_eq!(t.rows.len(), 2);
    }
}
