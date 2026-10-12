use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const MAGIC: &[u8; 4] = b"CHF2";
const VERSION: u32 = 1;
const NETLOC: &str = "www.canfar.net";
const SOURCE_URL: &str = "https://www.canfar.net/storage/vault/file/AstroDataCitationDOI/CISTI.CANFAR/25.0066/data/table/chimefrbcat2.csv";
const COMPILER: &str = "tools/harvest/src/bin/chime_frb_compiler.rs";
const FORMAT: &str = "chime_frb_cat2";

const COL_DM: &str = "dm_fitb";
const COL_EXC_NE2001: &str = "dm_exc_ne2001";
const COL_EXC_YMW16: &str = "dm_exc_ymw16";

const OBS_DM: &str = "DM";
const OBS_DM_NE2001: &str = "DM_ne2001";
const OBS_DM_YMW16: &str = "DM_ymw16";

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
        None => SOURCE_URL.to_string(),
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

fn column_index(header: &[String], name: &str) -> Option<usize> {
    header
        .iter()
        .position(|h| h.trim().eq_ignore_ascii_case(name))
}

fn observable_table(header: &[String], rows: &[Vec<String>]) -> Table {
    let names = vec![
        OBS_DM.to_string(),
        OBS_DM_NE2001.to_string(),
        OBS_DM_YMW16.to_string(),
    ];
    let i_dm = column_index(header, COL_DM);
    let i_ne = column_index(header, COL_EXC_NE2001);
    let i_ym = column_index(header, COL_EXC_YMW16);
    let cell = |row: &Vec<String>, idx: Option<usize>| -> Option<f64> {
        idx.and_then(|i| row.get(i)).and_then(|c| cell_num(c))
    };
    let out_rows = rows
        .iter()
        .map(|r| {
            let dm = cell(r, i_dm);
            let galactic = |idx: Option<usize>| match (dm, cell(r, idx)) {
                (Some(total), Some(excess)) => Some(total - excess),
                _ => None,
            };
            vec![dm, galactic(i_ne), galactic(i_ym)]
        })
        .collect();
    Table {
        names,
        rows: out_rows,
    }
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
        None => "chime_frb_cat2.bin".to_string(),
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
    let table = observable_table(&header, &rows);
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

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
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
    fn observable_table_derives_galactic_dm() {
        let head = "tns_name,dm_fitb,dm_exc_ne2001,dm_exc_ymw16";
        let text = format!("{head}\nFRB1,460.8,410.8,420.8\nFRB2,nan,nan,nan\nFRB3,100.0,,90.0\n");
        let (header, rows) = parse_csv(&text).unwrap();
        let t = observable_table(&header, &rows);
        assert_eq!(t.names, vec!["DM", "DM_ne2001", "DM_ymw16"]);
        assert_eq!(t.rows.len(), 3);
        assert!(approx(t.rows[0][0].unwrap(), 460.8));
        assert!(approx(t.rows[0][1].unwrap(), 50.0));
        assert!(approx(t.rows[0][2].unwrap(), 40.0));
        assert_eq!(t.rows[1], vec![None, None, None]);
        assert!(approx(t.rows[2][0].unwrap(), 100.0));
        assert_eq!(t.rows[2][1], None);
        assert!(approx(t.rows[2][2].unwrap(), 10.0));
    }

    #[test]
    fn split_fields_keeps_quoted_commas() {
        let row = split_fields("\"a,b\",1,FRB,x");
        assert_eq!(row, vec!["a,b", "1", "FRB", "x"]);
    }
}
