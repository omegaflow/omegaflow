pub const VERSION: u32 = 1;
pub const HEADER_BYTES: usize = 16;

#[derive(Clone, PartialEq, Debug)]
pub struct Table {
    pub names: Vec<String>,
    pub rows: Vec<Vec<Option<f64>>>,
}

pub fn parse_bin(magic: [u8; 4], bytes: &[u8]) -> Option<Table> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != magic {
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

pub fn series_from_table<F>(table: &Table, t_of: F) -> Option<Vec<(f64, f64, u32)>>
where
    F: Fn(f64) -> Option<f64>,
{
    let mut out = Vec::new();
    for row in &table.rows {
        let Some(t) = row.first().copied().flatten().and_then(|epoch| t_of(epoch)) else {
            continue;
        };
        for (j, value) in row.iter().enumerate().skip(1) {
            if let Some(value) = value {
                out.push((t, *value, j as u32));
            }
        }
    }
    if out.is_empty() { None } else { Some(out) }
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
        for v in row.iter().flatten() {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> Table {
        Table {
            names: vec!["mjd".to_string(), "b".to_string(), "sigma_b".to_string()],
            rows: vec![
                vec![Some(42_000.5), Some(4.4), None],
                vec![Some(42_001.5), None, Some(0.6)],
            ],
        }
    }

    #[test]
    fn roundtrip_keeps_absence_and_column_order() {
        let bytes = write_bin(*b"TEST", &table());
        assert_eq!(parse_bin(*b"TEST", &bytes), Some(table()));
        assert!(parse_bin(*b"NOPE", &bytes).is_none());
    }

    #[test]
    fn truncated_and_non_finite_rows_stay_unread() {
        let bytes = write_bin(*b"TEST", &table());
        assert!(parse_bin(*b"TEST", &bytes[..bytes.len() - 1]).is_none());

        let mut nan = table();
        nan.rows[0][1] = Some(f64::NAN);
        let bytes = write_bin(*b"TEST", &nan);
        assert!(parse_bin(*b"TEST", &bytes).is_none());
    }

    #[test]
    fn series_uses_the_first_column_as_epoch() {
        let table = table();
        let rows = series_from_table(&table, |mjd| Some(mjd + 1.0)).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], (42_001.5, 4.4, 1));
        assert_eq!(rows[1], (42_002.5, 0.6, 2));
    }
}
