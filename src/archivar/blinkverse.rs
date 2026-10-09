pub const MAGIC: [u8; 4] = *b"BVFR";
pub const VERSION: u32 = 1;
pub const HEADER_BYTES: usize = 16;

#[derive(Clone, PartialEq, Debug)]
pub struct Table {
    pub names: Vec<String>,
    pub rows: Vec<Vec<Option<f64>>>,
}

pub fn component_name(comp: u32) -> String {
    format!("blinkverse_col_{comp:04}")
}

pub const RA_NAMES: &[&str] = &["ra", "right ascension"];
pub const DEC_NAMES: &[&str] = &["dec", "decl", "declination"];

pub fn column_index(names: &[String], wanted: &[&str]) -> Option<usize> {
    names.iter().position(|name| {
        let n = name.trim();
        wanted.iter().any(|w| n.eq_ignore_ascii_case(w))
    })
}

pub fn write_bin(table: &Table) -> Vec<u8> {
    let n_cols = table.names.len();
    let words = (n_cols + 63) / 64;
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
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

pub fn parse_bin(bytes: &[u8]) -> Option<Table> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
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
    let mut pos = HEADER_BYTES;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> Table {
        Table {
            names: vec!["DM".to_string(), "RA".to_string(), "Dec".to_string()],
            rows: vec![
                vec![Some(460.8), Some(334.375), None],
                vec![None, Some(15.0), Some(-72.192722222222)],
            ],
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
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let good = write_bin(&table());
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn parse_bin_rejects_bad_version_and_leftover() {
        let mut bad_version = write_bin(&table());
        bad_version[4..8].copy_from_slice(&2u32.to_le_bytes());
        assert!(parse_bin(&bad_version).is_none());

        let mut leftover = write_bin(&table());
        leftover.push(0u8);
        assert!(parse_bin(&leftover).is_none());
    }

    #[test]
    fn column_index_reads_ra_dec_by_name_and_refuses_absence() {
        let names = vec![
            "DM".to_string(),
            "RA".to_string(),
            "declination".to_string(),
        ];
        assert_eq!(column_index(&names, RA_NAMES), Some(1));
        assert_eq!(column_index(&names, DEC_NAMES), Some(2));
        assert_eq!(column_index(&names, &["flux"]), None);
    }

    #[test]
    fn parse_bin_refuses_non_finite_values() {
        let nan = Table {
            names: vec!["DM".to_string()],
            rows: vec![vec![Some(f64::NAN)]],
        };
        assert!(parse_bin(&write_bin(&nan)).is_none());
        let inf = Table {
            names: vec!["DM".to_string()],
            rows: vec![vec![Some(f64::INFINITY)]],
        };
        assert!(parse_bin(&write_bin(&inf)).is_none());
    }
}
