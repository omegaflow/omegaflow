pub const MAGIC: [u8; 4] = *b"HW3B";
pub const VERSION: u16 = 1;
pub const HEADER_BYTES: usize = 12;

pub const IMPLICIT_NAMES: [&str; 2] = ["ra", "dec"];

#[derive(Clone, PartialEq, Debug)]
pub struct Table {
    pub names: Vec<String>,
    pub rows: Vec<Vec<f64>>,
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
    let n_cols = table.names.len().saturating_sub(2);
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&(n_cols as u16).to_le_bytes());
    out.extend_from_slice(&(table.rows.len() as u32).to_le_bytes());
    for name in table.names.iter().skip(2) {
        let b = name.as_bytes();
        out.extend_from_slice(&(b.len() as u16).to_le_bytes());
        out.extend_from_slice(b);
    }
    for row in &table.rows {
        for v in row {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_bin(bytes: &[u8]) -> Option<Table> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let version = u16::from_le_bytes(bytes[4..6].try_into().ok()?);
    if version != VERSION {
        return None;
    }
    let n_cols = u16::from_le_bytes(bytes[6..8].try_into().ok()?) as usize;
    let n_rows = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    let width = 2 + n_cols;
    let mut pos = HEADER_BYTES;
    let mut names = Vec::with_capacity(width);
    names.push(IMPLICIT_NAMES[0].to_string());
    names.push(IMPLICIT_NAMES[1].to_string());
    for _ in 0..n_cols {
        let len = u16::from_le_bytes(bytes.get(pos..pos + 2)?.try_into().ok()?) as usize;
        pos += 2;
        let raw = bytes.get(pos..pos + len)?;
        names.push(String::from_utf8(raw.to_vec()).ok()?);
        pos += len;
    }
    if bytes.len().checked_sub(pos)? < n_rows.checked_mul(width.checked_mul(8)?)? {
        return None;
    }
    let mut rows = Vec::with_capacity(n_rows);
    for _ in 0..n_rows {
        let mut row = Vec::with_capacity(width);
        for _ in 0..width {
            let v = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
            if !v.is_finite() {
                return None;
            }
            row.push(v);
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
            names: vec!["ra".to_string(), "dec".to_string(), "flux_9_um".to_string()],
            rows: vec![vec![10.5, -20.25, 0.1771], vec![10.75, -20.5, 1.5]],
        }
    }

    #[test]
    fn roundtrip_preserves_rows_and_columns() {
        let t = table();
        let bytes = write_bin(&t);
        assert_eq!(parse_bin(&bytes), Some(t));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_truncation() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"HW3B").is_none());
        let good = write_bin(&table());
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn parse_bin_rejects_bad_version_and_leftover() {
        let mut bad_version = write_bin(&table());
        bad_version[4..6].copy_from_slice(&2u16.to_le_bytes());
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
        assert_eq!(column_index(&names, &["flux_9_um"]), None);
    }

    #[test]
    fn parse_bin_refuses_non_finite_values() {
        let nan = Table {
            names: vec!["ra".to_string(), "dec".to_string(), "flux".to_string()],
            rows: vec![vec![10.0, -2.0, f64::NAN]],
        };
        assert!(parse_bin(&write_bin(&nan)).is_none());
        let inf = Table {
            names: vec!["ra".to_string(), "dec".to_string(), "flux".to_string()],
            rows: vec![vec![10.0, -2.0, f64::INFINITY]],
        };
        assert!(parse_bin(&write_bin(&inf)).is_none());
    }
}
