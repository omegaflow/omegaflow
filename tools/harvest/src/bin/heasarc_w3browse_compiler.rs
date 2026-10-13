use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip;
use omegaflow::tdat::parse_tdat;
use std::io::Write;

const NETLOC: &str = "heasarc.gsfc.nasa.gov";
const BASE: &str = "https://heasarc.gsfc.nasa.gov/FTP/heasarc/dbase/tdat_files/";
const MAGIC: &[u8; 4] = b"HW3B";
const VERSION: u16 = 1;

fn write_bin(columns: &[String], rows: &[Vec<f64>]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&(columns.len() as u16).to_le_bytes());
    out.extend_from_slice(&(rows.len() as u32).to_le_bytes());
    for c in columns {
        let bytes = c.as_bytes();
        out.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
        out.extend_from_slice(bytes);
    }
    for r in rows {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn parse_bin(data: &[u8]) -> Option<(Vec<String>, Vec<Vec<f64>>)> {
    if data.len() < 12 || &data[0..4] != MAGIC {
        return None;
    }
    let version = u16::from_le_bytes(data[4..6].try_into().ok()?);
    if version != VERSION {
        return None;
    }
    let ncols = u16::from_le_bytes(data[6..8].try_into().ok()?) as usize;
    let nrows = u32::from_le_bytes(data[8..12].try_into().ok()?) as usize;
    let mut off = 12usize;
    let mut columns = Vec::with_capacity(ncols);
    for _ in 0..ncols {
        let len = u16::from_le_bytes(data.get(off..off + 2)?.try_into().ok()?) as usize;
        off += 2;
        let name = std::str::from_utf8(data.get(off..off + len)?)
            .ok()?
            .to_string();
        off += len;
        columns.push(name);
    }
    let width = 2 + ncols;
    let mut rows = Vec::with_capacity(nrows);
    for _ in 0..nrows {
        let mut row = Vec::with_capacity(width);
        for _ in 0..width {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(data.get(off..off + 8)?);
            off += 8;
            row.push(f64::from_le_bytes(buf));
        }
        rows.push(row);
    }
    if off != data.len() {
        return None;
    }
    Some((columns, rows))
}

fn run(args: &[String]) -> Result<(), String> {
    let mut table: Option<String> = None;
    let mut out_path: Option<String> = None;
    let mut input: Option<String> = None;
    let mut columns: Vec<String> = Vec::new();
    let mut ci_mode = false;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--table" => match args.get(i + 1) {
                Some(v) => {
                    table = Some(v.clone());
                    i += 1;
                }
                None => return Err("--table <w3browse table name> — refused".into()),
            },
            "--column" => match args.get(i + 1) {
                Some(v) => {
                    columns.push(v.clone());
                    i += 1;
                }
                None => return Err("--column <tdat column> — refused".into()),
            },
            "--out" => match args.get(i + 1) {
                Some(v) => {
                    out_path = Some(v.clone());
                    i += 1;
                }
                None => return Err("--out <bin path> — refused".into()),
            },
            "--input" => match args.get(i + 1) {
                Some(v) => {
                    input = Some(v.clone());
                    i += 1;
                }
                None => return Err("--input <tdat.gz path> — refused".into()),
            },
            "--ci-mode" => ci_mode = true,
            other => return Err(format!("unknown argument {other} — refused")),
        }
        i += 1;
    }
    let Some(table) = table else {
        return Err(
            "usage: heasarc_w3browse_compiler --table <w3browse table> --column <tdat column> --out <bin> [--input <tdat.gz>] [--ci-mode] — refused"
                .into(),
        );
    };
    let Some(out_path) = out_path else {
        return Err("--out <bin path>: the asset path is never silent — refused".into());
    };
    if columns.is_empty() {
        return Err(
            "--column <tdat column>: no observable is declared, the bin stays unwritten — refused"
                .into(),
        );
    }

    let url = format!("{BASE}heasarc_{table}.tdat.gz");
    let raw: Vec<u8> = match input {
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?
        }
        None => match fetch_raw_bytes(&url) {
            Some(b) => b,
            None => {
                return Err(format!(
                    "{url}: fetch void — the bin stays unwritten (0 honored)"
                ));
            }
        },
    };
    let plain = match gunzip(&raw) {
        Some(p) => p,
        None => {
            return Err(format!(
                "{url}: gzip stream stays unreadable — the bin stays unwritten (0 honored)"
            ));
        }
    };
    let Some(parsed) = parse_tdat(&plain) else {
        return Err(format!(
            "{url}: no <HEADER>/<DATA> table in the tdat — the bin stays unwritten (0 honored)"
        ));
    };
    if parsed.row_index("ra").is_none() || parsed.row_index("dec").is_none() {
        return Err(format!(
            "{table}: ra/dec absent in the tdat — the catalogue position stays void (0 honored)"
        ));
    }
    for c in &columns {
        if parsed.row_index(c).is_none() {
            return Err(format!("{table}: column {c} absent in the tdat — refused"));
        }
    }

    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut skipped = 0u64;
    for r in 0..parsed.rows.len() {
        let Some(ra) = parsed.f64_cell(r, "ra") else {
            skipped += 1;
            continue;
        };
        let Some(dec) = parsed.f64_cell(r, "dec") else {
            skipped += 1;
            continue;
        };
        if !ra.is_finite() || !dec.is_finite() {
            skipped += 1;
            continue;
        }
        let mut row: Vec<f64> = Vec::with_capacity(2 + columns.len());
        row.push(ra);
        row.push(dec);
        let mut complete = true;
        for c in &columns {
            match parsed.f64_cell(r, c) {
                Some(v) if v.is_finite() => row.push(v),
                _ => {
                    complete = false;
                    break;
                }
            }
        }
        if !complete {
            skipped += 1;
            continue;
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Err(format!(
            "{table}: no row carries every declared column — the bin stays unwritten (0 honored)"
        ));
    }

    let bytes = write_bin(&columns, &rows);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut f = std::fs::File::create(&out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    f.write_all(&bytes)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    f.flush()
        .map_err(|e| format!("flush {out_path} returned void: {e}"))?;

    let Some((read_columns, read_rows)) = parse_bin(&bytes) else {
        return Err(format!("{out_path}: the roundtrip parse stays unread"));
    };
    eprintln!(
        "{out_path}: {} rows, {} columns {:?} from table {table} ({} rows skipped) — roundtrip verified",
        read_rows.len(),
        2 + read_columns.len(),
        read_columns,
        skipped
    );
    let name = match std::path::Path::new(&out_path)
        .file_name()
        .and_then(|n| n.to_str())
    {
        Some(n) => n,
        None => return Err(format!("{out_path}: carries no file name")),
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
    println!("format heasarc_w3browse");
    println!("origin {url}");
    println!("compiler tools/harvest/src/bin/heasarc_w3browse_compiler.rs");
    println!("sha256 {}", sha256_hex(&bytes));
    println!("at earth");
    println!("ttl 604800");
    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("heasarc_w3browse_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrip_preserves_rows_and_columns() {
        let columns = vec!["flux_9_um".to_string()];
        let rows = vec![vec![10.5, -20.25, 0.1771], vec![10.75, -20.5, 1.5]];
        let bytes = write_bin(&columns, &rows);
        let (read_columns, read_rows) = parse_bin(&bytes).expect("the bin parses");
        assert_eq!(read_columns, columns);
        assert_eq!(read_rows, rows);
    }

    #[test]
    fn parse_bin_refuses_foreign_magic_and_short_body() {
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"HW3B").is_none());
        let good = write_bin(&["c".to_string()], &[vec![1.0, 2.0, 3.0]]);
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }
}
