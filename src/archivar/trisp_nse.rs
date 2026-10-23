use std::collections::HashSet;

pub const MAGIC: [u8; 4] = *b"TNSE";
pub const COLUMN_NAME_BYTES: usize = 32;
pub const LOG_KEY_BYTES: usize = 32;
pub const LOG_STRIDE: usize = LOG_KEY_BYTES + 8 + 1;

pub const COMP_POL: u32 = 0;
pub const COMP_POLC: u32 = 1;

pub const TAU_UNIT: &str = "ns";
pub const TTA_UNIT: &str = "K";

#[derive(Clone, Debug, PartialEq)]
pub struct TrispNseTable {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<f64>>>,
    pub log: Vec<(String, Option<f64>)>,
}

impl TrispNseTable {
    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c == name)
    }

    pub fn column(&self, name: &str) -> Option<Vec<Option<f64>>> {
        let idx = self.column_index(name)?;
        Some(
            self.rows
                .iter()
                .map(|r| r.get(idx).copied().flatten())
                .collect(),
        )
    }

    pub fn log_value(&self, key: &str) -> Option<f64> {
        self.log
            .iter()
            .find(|(k, _)| k == key)
            .and_then(|(_, v)| *v)
    }

    pub fn series(&self) -> Option<Vec<(f64, f64, u32)>> {
        let tau_idx = self.column_index("tau")?;
        let pol_idx = self.column_index("Pol")?;
        let polc_idx = self.column_index("PolC")?;
        let mut out = Vec::new();
        for row in &self.rows {
            let Some(Some(tau)) = row.get(tau_idx).copied() else {
                continue;
            };
            if let Some(Some(pol)) = row.get(pol_idx).copied() {
                out.push((tau, pol, COMP_POL));
            }
            if let Some(Some(polc)) = row.get(polc_idx).copied() {
                out.push((tau, polc, COMP_POLC));
            }
        }
        if out.is_empty() { None } else { Some(out) }
    }
}

fn parse_cell(cell: &str) -> Option<f64> {
    let s = cell.trim();
    if s.is_empty() {
        return None;
    }
    s.parse::<f64>().ok().filter(|x| x.is_finite())
}

pub fn parse_dat(text: &str) -> Option<TrispNseTable> {
    let mut lines = text.lines();
    let header = lines.next()?;
    let columns: Vec<String> = header.split_whitespace().map(str::to_string).collect();
    if columns.is_empty() {
        return None;
    }
    let mut seen = HashSet::new();
    for name in &columns {
        if !seen.insert(name) {
            return None;
        }
    }
    let mut rows = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split_whitespace().collect();
        if cells.len() != columns.len() {
            continue;
        }
        let values: Vec<Option<f64>> = cells.iter().map(|c| parse_cell(c)).collect();
        if values.iter().all(Option::is_none) {
            continue;
        }
        rows.push(values);
    }
    if rows.is_empty() {
        return None;
    }
    Some(TrispNseTable {
        columns,
        rows,
        log: Vec::new(),
    })
}

pub fn parse_log(text: &str) -> Vec<(String, Option<f64>)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut tokens = line.split_whitespace();
        let Some(key) = tokens.next() else { continue };
        let value = tokens.next().and_then(parse_cell);
        out.push((key.to_string(), value));
    }
    out
}

fn push_str_field(bin: &mut [u8], off: usize, s: &str, len: usize) {
    let b = s.as_bytes();
    let n = b.len().min(len);
    bin[off..off + n].copy_from_slice(&b[..n]);
}

fn read_str_field(bytes: &[u8], from: usize, len: usize) -> Option<String> {
    let field = bytes.get(from..from + len)?;
    let end = field.iter().position(|b| *b == 0).unwrap_or(field.len());
    String::from_utf8(field[..end].to_vec()).ok()
}

pub fn pack(table: &TrispNseTable) -> Vec<u8> {
    let ncols = table.columns.len();
    let nrows = table.rows.len();
    let nlog = table.log.len();
    let words = ncols.div_ceil(64);
    let row_stride = ncols * 8 + words * 8;
    let mut bin =
        vec![0u8; 16 + ncols * COLUMN_NAME_BYTES + nrows * row_stride + nlog * LOG_STRIDE];
    bin[0..4].copy_from_slice(&MAGIC);
    bin[4..8].copy_from_slice(&(ncols as u32).to_le_bytes());
    bin[8..12].copy_from_slice(&(nrows as u32).to_le_bytes());
    bin[12..16].copy_from_slice(&(nlog as u32).to_le_bytes());
    let mut off = 16;
    for name in &table.columns {
        push_str_field(&mut bin, off, name, COLUMN_NAME_BYTES);
        off += COLUMN_NAME_BYTES;
    }
    for row in &table.rows {
        let mut pw = vec![0u64; words];
        for (j, v) in row.iter().enumerate() {
            if let Some(x) = v {
                bin[off + j * 8..off + j * 8 + 8].copy_from_slice(&x.to_le_bytes());
                pw[j / 64] |= 1u64 << (j % 64);
            }
        }
        off += ncols * 8;
        for w in 0..words {
            bin[off + w * 8..off + w * 8 + 8].copy_from_slice(&pw[w].to_le_bytes());
        }
        off += words * 8;
    }
    for (key, value) in &table.log {
        push_str_field(&mut bin, off, key, LOG_KEY_BYTES);
        match value {
            Some(x) => {
                bin[off + LOG_KEY_BYTES..off + LOG_KEY_BYTES + 8].copy_from_slice(&x.to_le_bytes());
                bin[off + LOG_KEY_BYTES + 8] = 1;
            }
            None => {
                bin[off + LOG_KEY_BYTES + 8] = 0;
            }
        }
        off += LOG_STRIDE;
    }
    bin
}

pub fn parse_bin(bytes: &[u8]) -> Option<TrispNseTable> {
    if bytes.len() < 16 || bytes[0..4] != MAGIC {
        return None;
    }
    let ncols = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let nrows = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    let nlog = u32::from_le_bytes(bytes[12..16].try_into().ok()?) as usize;
    if ncols == 0 {
        return None;
    }
    let words = ncols.div_ceil(64);
    let row_stride = ncols * 8 + words * 8;
    let expected = 16 + ncols * COLUMN_NAME_BYTES + nrows * row_stride + nlog * LOG_STRIDE;
    if bytes.len() != expected {
        return None;
    }
    let mut off = 16;
    let mut columns = Vec::with_capacity(ncols);
    for _ in 0..ncols {
        columns.push(read_str_field(bytes, off, COLUMN_NAME_BYTES)?);
        off += COLUMN_NAME_BYTES;
    }
    let mut rows = Vec::with_capacity(nrows);
    for _ in 0..nrows {
        let mut values = Vec::with_capacity(ncols);
        for j in 0..ncols {
            let wb = off + ncols * 8 + (j / 64) * 8;
            let present = u64::from_le_bytes(bytes[wb..wb + 8].try_into().ok()?);
            let v = f64::from_le_bytes(bytes[off + j * 8..off + j * 8 + 8].try_into().ok()?);
            let value = if present & (1u64 << (j % 64)) != 0 && v.is_finite() {
                Some(v)
            } else {
                None
            };
            values.push(value);
        }
        rows.push(values);
        off += row_stride;
    }
    let mut log = Vec::with_capacity(nlog);
    for _ in 0..nlog {
        let key = read_str_field(bytes, off, LOG_KEY_BYTES)?;
        let v = f64::from_le_bytes(
            bytes[off + LOG_KEY_BYTES..off + LOG_KEY_BYTES + 8]
                .try_into()
                .ok()?,
        );
        let present = bytes[off + LOG_KEY_BYTES + 8] != 0;
        log.push((
            key,
            if present && v.is_finite() {
                Some(v)
            } else {
                None
            },
        ));
        off += LOG_STRIDE;
    }
    Some(TrispNseTable { columns, rows, log })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_table() -> TrispNseTable {
        TrispNseTable {
            columns: vec![
                "tau".to_string(),
                "TTA".to_string(),
                "Pol".to_string(),
                "PolC".to_string(),
            ],
            rows: vec![
                vec![Some(1.0), Some(10.0), Some(0.90), Some(0.80)],
                vec![Some(2.0), Some(10.0), Some(0.70), Some(0.60)],
                vec![Some(3.0), None, None, Some(0.50)],
            ],
            log: Vec::new(),
        }
    }

    #[test]
    fn parse_dat_reads_header_and_rows() {
        let text = "tau TTA Pol PolC\n1.0 10.0 0.90 0.80\n2.0 10.0 0.70 0.60\n";
        let table = parse_dat(text).expect("dat parses");
        assert_eq!(table.columns, ["tau", "TTA", "Pol", "PolC"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[1][0], Some(2.0));
    }

    #[test]
    fn parse_log_reads_key_values() {
        let log = parse_log("TTAMEAN 10.0\nGamma 0.5\ndGamma 0.05\nempty\n");
        assert_eq!(log.len(), 4);
        assert_eq!(log[0], ("TTAMEAN".to_string(), Some(10.0)));
        assert_eq!(log[2], ("dGamma".to_string(), Some(0.05)));
        assert_eq!(log[3], ("empty".to_string(), None));
    }

    #[test]
    fn pack_and_roundtrip_hold_the_table() {
        let mut table = sample_table();
        table.log = vec![
            ("TTAMEAN".to_string(), Some(10.0)),
            ("Gamma".to_string(), Some(0.5)),
        ];
        let bin = pack(&table);
        let parsed = parse_bin(&bin).expect("packed table parses");
        assert_eq!(parsed, table);
    }

    #[test]
    fn series_pairs_pol_and_polc() {
        let table = sample_table();
        let series = table.series().expect("series present");
        assert_eq!(series.len(), 5);
        assert_eq!(series[0], (1.0, 0.90, COMP_POL));
        assert_eq!(series[1], (1.0, 0.80, COMP_POLC));
    }

    #[test]
    fn missing_column_and_key_are_rejected() {
        let table = sample_table();
        assert!(table.column("absent").is_none());
        assert!(table.column_index("absent").is_none());
        assert!(table.log_value("absent").is_none());
    }

    #[test]
    fn parse_dat_rejects_foreign_text() {
        assert!(parse_dat("").is_none());
        assert!(parse_dat("no header here\n").is_none());
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"XXXX").is_none());
    }

    #[test]
    fn parse_cell_gates_nan_inf_and_blank() {
        assert_eq!(parse_cell("4.07840"), Some(4.07840));
        assert_eq!(parse_cell("-25.41733"), Some(-25.41733));
        assert_eq!(parse_cell("5.5000e-03"), Some(5.5000e-03));
        assert_eq!(parse_cell(""), None);
        assert_eq!(parse_cell("   "), None);
        assert_eq!(parse_cell("nan"), None);
        assert_eq!(parse_cell("inf"), None);
        assert_eq!(parse_cell("N/A"), None);
        assert_eq!(parse_cell("28521"), Some(28521.0));
    }
}
