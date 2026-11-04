use std::collections::BTreeMap;

pub const MAGIC: [u8; 4] = *b"KCD1";
pub const REC_BYTES: usize = 20;

pub const COMP_ARRAY_E: u32 = 1;
pub const COMP_ARRAY_XC: u32 = 2;
pub const COMP_ARRAY_YC: u32 = 3;
pub const COMP_ARRAY_ZE: u32 = 4;
pub const COMP_ARRAY_AZ: u32 = 5;
pub const COMP_ARRAY_NE: u32 = 6;
pub const COMP_ARRAY_NMU: u32 = 7;
pub const COMP_ARRAY_AGE: u32 = 8;
pub const COMP_CALO_NHAD: u32 = 9;
pub const COMP_CALO_EHAD: u32 = 10;
pub const COMP_GRANDE_XC: u32 = 11;
pub const COMP_GRANDE_YC: u32 = 12;
pub const COMP_GRANDE_ZE: u32 = 13;
pub const COMP_GRANDE_AZ: u32 = 14;
pub const COMP_GRANDE_NCH: u32 = 15;
pub const COMP_GRANDE_NMU: u32 = 16;
pub const COMP_GRANDE_AGE: u32 = 17;
pub const COMP_GEN_T: u32 = 18;
pub const COMP_GEN_P: u32 = 19;
pub const COMP_LOPES_EFIELDMAX: u32 = 20;

pub fn array_comp(token: &str) -> Option<u32> {
    match token {
        "E" => Some(COMP_ARRAY_E),
        "Xc" => Some(COMP_ARRAY_XC),
        "Yc" => Some(COMP_ARRAY_YC),
        "Ze" => Some(COMP_ARRAY_ZE),
        "Az" => Some(COMP_ARRAY_AZ),
        "Ne" => Some(COMP_ARRAY_NE),
        "Nmu" => Some(COMP_ARRAY_NMU),
        "Age" => Some(COMP_ARRAY_AGE),
        _ => None,
    }
}

pub fn grande_comp(token: &str) -> Option<u32> {
    match token {
        "Xc" => Some(COMP_GRANDE_XC),
        "Yc" => Some(COMP_GRANDE_YC),
        "Ze" => Some(COMP_GRANDE_ZE),
        "Az" => Some(COMP_GRANDE_AZ),
        "Nch" => Some(COMP_GRANDE_NCH),
        "Nmu" => Some(COMP_GRANDE_NMU),
        "Age" => Some(COMP_GRANDE_AGE),
        _ => None,
    }
}

pub fn calorimeter_comp(token: &str) -> Option<u32> {
    match token {
        "Nhad" => Some(COMP_CALO_NHAD),
        "Ehad" => Some(COMP_CALO_EHAD),
        _ => None,
    }
}

pub fn general_comp(token: &str) -> Option<u32> {
    match token {
        "T" => Some(COMP_GEN_T),
        "P" => Some(COMP_GEN_P),
        _ => None,
    }
}

pub fn lopes_comp(token: &str) -> Option<u32> {
    match token {
        "EfieldMaxAbs" => Some(COMP_LOPES_EFIELDMAX),
        _ => None,
    }
}

pub fn is_log10(comp: u32) -> bool {
    matches!(
        comp,
        COMP_ARRAY_E
            | COMP_ARRAY_NE
            | COMP_ARRAY_NMU
            | COMP_GRANDE_NCH
            | COMP_GRANDE_NMU
            | COMP_CALO_EHAD
    )
}

pub fn plausible(comp: u32, v: f64) -> bool {
    if !v.is_finite() {
        return false;
    }
    match comp {
        COMP_ARRAY_E => (1.0e13..=1.0e19).contains(&v),
        COMP_ARRAY_XC | COMP_ARRAY_YC => (-91.0..=91.0).contains(&v),
        COMP_ARRAY_ZE => (0.0..=60.0).contains(&v),
        COMP_ARRAY_AZ => (0.0..=360.0).contains(&v),
        COMP_ARRAY_NE => (100.0..=5.0e8).contains(&v),
        COMP_ARRAY_NMU => (100.0..=5.0e7).contains(&v),
        COMP_ARRAY_AGE => (0.1..=1.48).contains(&v),
        COMP_CALO_NHAD => (0.0..=511.0).contains(&v),
        COMP_CALO_EHAD => (1.0e10..=1.0e16).contains(&v),
        COMP_GRANDE_XC => (-500.0..=100.0).contains(&v),
        COMP_GRANDE_YC => (-600.0..=100.0).contains(&v),
        COMP_GRANDE_ZE => (0.0..=40.0).contains(&v),
        COMP_GRANDE_AZ => (0.0..=360.0).contains(&v),
        COMP_GRANDE_NCH => (11111.0..=1.0e9).contains(&v),
        COMP_GRANDE_NMU => (1500.0..=1.0e8).contains(&v),
        COMP_GRANDE_AGE => (-0.385..=1.485).contains(&v),
        COMP_GEN_T => (-20.0..=50.0).contains(&v),
        COMP_GEN_P => (960.0..=1030.0).contains(&v),
        COMP_LOPES_EFIELDMAX => (0.0..=50000.0).contains(&v),
        _ => false,
    }
}

pub struct Table {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<f64>>>,
}

fn is_numeric_token(tok: &str) -> bool {
    tok.parse::<f64>().is_ok()
}

pub fn read_table(text: &str) -> Option<Table> {
    let mut columns: Option<Vec<String>> = None;
    let mut rows: Vec<Vec<Option<f64>>> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }
        match &columns {
            None => {
                if tokens.iter().all(|t| !is_numeric_token(t)) {
                    columns = Some(tokens.iter().map(|t| t.to_string()).collect());
                }
            }
            Some(cols) => {
                if tokens.len() == cols.len() {
                    rows.push(
                        tokens
                            .iter()
                            .map(|t| t.parse::<f64>().ok().filter(|v| v.is_finite()))
                            .collect(),
                    );
                }
            }
        }
    }
    Some(Table {
        columns: columns?,
        rows,
    })
}

pub struct RowMapping {
    pub tables: Vec<String>,
    pub rows: Vec<Vec<i64>>,
}

pub fn parse_row_mapping(text: &str) -> Option<RowMapping> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let tables: Vec<String> = lines
        .next()?
        .split_whitespace()
        .map(str::to_string)
        .collect();
    if tables.is_empty() {
        return None;
    }
    let mut rows = Vec::new();
    for line in lines {
        let vals: Vec<i64> = line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .ok()?;
        if vals.len() != tables.len() {
            return None;
        }
        rows.push(vals);
    }
    Some(RowMapping { tables, rows })
}

pub fn own_gt_times(table: &Table) -> Vec<Option<f64>> {
    let Some(i) = col_of(table, "Gt") else {
        return Vec::new();
    };
    table
        .rows
        .iter()
        .map(|row| row.get(i).copied().flatten().filter(|t| *t > 0.0))
        .collect()
}

pub fn mapped_times(
    mapping: &RowMapping,
    general_times: &[Option<f64>],
    table_name: &str,
) -> Vec<Option<f64>> {
    let Some(col) = mapping.tables.iter().position(|t| t == table_name) else {
        return Vec::new();
    };
    let Some(gcol) = mapping.tables.iter().position(|t| t == "general") else {
        return Vec::new();
    };
    let mut out: Vec<Option<f64>> = Vec::new();
    for row in &mapping.rows {
        let r = row[col];
        if r < 0 {
            continue;
        }
        let g = row[gcol];
        let t = if g >= 0 {
            general_times.get(g as usize).copied().flatten()
        } else {
            None
        };
        let idx = r as usize;
        if idx >= out.len() {
            out.resize(idx + 1, None);
        }
        out[idx] = t;
    }
    out
}

pub fn col_of(table: &Table, token: &str) -> Option<usize> {
    table.columns.iter().position(|c| c == token)
}

pub fn join_key(table: &Table, row: &[Option<f64>]) -> Option<(i64, i64)> {
    let r_i = col_of(table, "R")?;
    let ev_i = col_of(table, "Ev")?;
    let r = row.get(r_i).copied().flatten()? as i64;
    let ev = row.get(ev_i).copied().flatten()? as i64;
    Some((r, ev))
}

pub fn row_time(
    table: &Table,
    row: &[Option<f64>],
    gt_index: &BTreeMap<(i64, i64), f64>,
) -> Option<f64> {
    if let Some(i) = col_of(table, "Gt")
        && let Some(Some(t)) = row.get(i)
        && *t > 0.0
    {
        return Some(*t);
    }
    gt_index.get(&join_key(table, row)?).copied()
}

pub fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * REC_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, comp) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
        buf.extend_from_slice(&comp.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let v = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let comp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if !t.is_finite() || !v.is_finite() {
            return None;
        }
        out.push((t, v, comp));
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    parse_bin(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_bin() {
        let recs = vec![
            (894645350.0, 1.0e14, COMP_ARRAY_E),
            (894645351.0, 3.2e5, COMP_ARRAY_NE),
        ];
        let bytes = write_bin(&recs);
        assert_eq!(parse_bin(&bytes), Some(recs.clone()));
        let mut bad = bytes.clone();
        bad[0] = b'X';
        assert!(parse_bin(&bad).is_none());
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn read_table_reads_the_header_and_keeps_absent_cells_absent() {
        let text = "# KCDC array\nE Xc Ze Ne Gt\n\
                    1.4e14 -12.5 33.1 4.2e5 894645350\n\
                    3.0e15 4.0 55.0 - 894645351\n\
                    short-row\n";
        let table = read_table(text).expect("header parses");
        assert_eq!(table.columns, vec!["E", "Xc", "Ze", "Ne", "Gt"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0][0], Some(1.4e14));
        assert_eq!(table.rows[1][3], None);
        assert_eq!(table.rows[1][4], Some(894645351.0));
    }

    #[test]
    fn read_table_stays_void_without_a_header() {
        let text = "1.4e14 -12.5 33.1\n3.0e15 4.0 55.0\n";
        assert!(read_table(text).is_none());
    }

    #[test]
    fn comp_tokens_map_per_component() {
        assert_eq!(array_comp("E"), Some(COMP_ARRAY_E));
        assert_eq!(grande_comp("Ze"), Some(COMP_GRANDE_ZE));
        assert_eq!(calorimeter_comp("Nhad"), Some(COMP_CALO_NHAD));
        assert_eq!(general_comp("P"), Some(COMP_GEN_P));
        assert_eq!(lopes_comp("EfieldMaxAbs"), Some(COMP_LOPES_EFIELDMAX));
        assert_eq!(array_comp("Nch"), None);
    }

    #[test]
    fn log10_and_plausibility_follow_the_measured_ranges() {
        assert!(is_log10(COMP_ARRAY_E));
        assert!(is_log10(COMP_GRANDE_NMU));
        assert!(!is_log10(COMP_ARRAY_AGE));
        assert!(plausible(COMP_ARRAY_E, 1.0e15));
        assert!(!plausible(COMP_ARRAY_E, 1.0e8));
        assert!(plausible(COMP_ARRAY_ZE, 33.0));
        assert!(!plausible(COMP_ARRAY_ZE, 70.0));
        assert!(plausible(COMP_GEN_T, 12.5));
        assert!(!plausible(COMP_GEN_P, 900.0));
        assert!(!plausible(COMP_ARRAY_E, f64::NAN));
    }

    #[test]
    fn row_time_falls_back_to_the_joined_general_row() {
        let general = read_table("R Ev Gt T P\n877 1001 894645350 12.5 998.0\n").unwrap();
        let mut gt = BTreeMap::new();
        if let Some(gt_i) = col_of(&general, "Gt") {
            for row in &general.rows {
                if let (Some(key), Some(Some(t))) = (join_key(&general, row), row.get(gt_i)) {
                    gt.insert(key, *t);
                }
            }
        }
        let array = read_table("R Ev E Xc\n877 1001 1.4e14 -12.5\n").unwrap();
        assert_eq!(row_time(&array, &array.rows[0], &gt), Some(894645350.0));
        let own = read_table("R Ev E Gt\n877 1002 3.0e15 894645399\n").unwrap();
        assert_eq!(row_time(&own, &own.rows[0], &gt), Some(894645399.0));
        let alone = read_table("R Ev E\n877 1003 2.0e15\n").unwrap();
        assert_eq!(row_time(&alone, &alone.rows[0], &gt), None);
    }

    #[test]
    fn join_key_needs_both_columns() {
        let both = read_table("R Ev E\n877 1001 1.4e14\n").unwrap();
        assert_eq!(join_key(&both, &both.rows[0]), Some((877, 1001)));
        let half = read_table("Ev E\n1001 1.4e14\n").unwrap();
        assert_eq!(join_key(&half, &half.rows[0]), None);
    }

    #[test]
    fn row_mapping_joins_component_rows_to_general_time() {
        let mapping = parse_row_mapping(
            "calorimeter\tgrande\tgeneral\tarray\tlopes\n\
             -1\t-1\t0\t0\t-1\n\
             -1\t0\t1\t-1\t-1\n\
             -1\t1\t2\t-1\t-1\n\
             -1\t-1\t3\t1\t-1\n",
        )
        .unwrap();
        assert_eq!(
            mapping.tables,
            vec!["calorimeter", "grande", "general", "array", "lopes"]
        );
        let general = read_table(
            "Datetime\tEv\tGt\tP\tR\tT\n\
             2005-06-01T00:00:00\t608195\t100\t1.01130e+03\t5376\t7.7\n\
             2005-06-01T00:00:01\t608198\t200\t1.01130e+03\t5376\t7.7\n\
             2005-06-01T00:00:04\t608210\t300\t1.01130e+03\t5376\t7.7\n\
             2005-06-01T00:00:05\t608212\t400\t1.01130e+03\t5376\t7.7\n",
        )
        .unwrap();
        let gt = own_gt_times(&general);
        assert_eq!(gt, vec![Some(100.0), Some(200.0), Some(300.0), Some(400.0)]);
        assert_eq!(
            mapped_times(&mapping, &gt, "general"),
            vec![Some(100.0), Some(200.0), Some(300.0), Some(400.0)]
        );
        assert_eq!(
            mapped_times(&mapping, &gt, "array"),
            vec![Some(100.0), Some(400.0)]
        );
        assert_eq!(
            mapped_times(&mapping, &gt, "grande"),
            vec![Some(200.0), Some(300.0)]
        );
        assert_eq!(
            mapped_times(&mapping, &gt, "absent"),
            Vec::<Option<f64>>::new()
        );
    }
}
