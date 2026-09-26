pub const MAGIC: [u8; 4] = *b"EMCR";
pub const VERSION: u8 = 1;

pub struct EmcColumn {
    pub name: String,
    pub unit: String,
}

pub struct EmcSeries {
    pub model: String,
    pub variant: String,
    pub columns: Vec<EmcColumn>,
    pub rows: Vec<Vec<f64>>,
}

fn push_str(out: &mut Vec<u8>, s: &str) {
    let b = s.as_bytes();
    out.push(b.len() as u8);
    out.extend_from_slice(b);
}

fn read_u8(bytes: &[u8], pos: &mut usize) -> Option<u8> {
    let v = *bytes.get(*pos)?;
    *pos += 1;
    Some(v)
}

fn read_u32(bytes: &[u8], pos: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(bytes.get(*pos..*pos + 4)?.try_into().ok()?);
    *pos += 4;
    Some(v)
}

fn read_f64(bytes: &[u8], pos: &mut usize) -> Option<f64> {
    let v = f64::from_le_bytes(bytes.get(*pos..*pos + 8)?.try_into().ok()?);
    *pos += 8;
    Some(v)
}

fn read_str(bytes: &[u8], pos: &mut usize) -> Option<String> {
    let len = read_u8(bytes, pos)? as usize;
    let s = std::str::from_utf8(bytes.get(*pos..*pos + len)?).ok()?;
    *pos += len;
    Some(s.to_string())
}

pub fn encode_emc_bin(series: &[EmcSeries]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&(series.len() as u32).to_le_bytes());
    for s in series {
        push_str(&mut out, &s.model);
        push_str(&mut out, &s.variant);
        out.push(s.columns.len() as u8);
        for c in &s.columns {
            push_str(&mut out, &c.name);
            push_str(&mut out, &c.unit);
        }
        out.extend_from_slice(&(s.rows.len() as u32).to_le_bytes());
        for row in &s.rows {
            for v in row {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
    }
    out
}

pub fn parse_emc_bin(bytes: &[u8]) -> Option<Vec<EmcSeries>> {
    if bytes.len() < 9 || bytes[0..4] != MAGIC || bytes[4] != VERSION {
        return None;
    }
    let mut pos = 5usize;
    let n_series = read_u32(bytes, &mut pos)? as usize;
    let mut series: Vec<EmcSeries> = Vec::with_capacity(n_series);
    for _ in 0..n_series {
        let model = read_str(bytes, &mut pos)?;
        let variant = read_str(bytes, &mut pos)?;
        let n_cols = read_u8(bytes, &mut pos)? as usize;
        let mut columns = Vec::with_capacity(n_cols);
        for _ in 0..n_cols {
            let name = read_str(bytes, &mut pos)?;
            let unit = read_str(bytes, &mut pos)?;
            columns.push(EmcColumn { name, unit });
        }
        let n_rows = read_u32(bytes, &mut pos)? as usize;
        let mut rows = Vec::with_capacity(n_rows);
        for _ in 0..n_rows {
            let mut row = Vec::with_capacity(n_cols);
            for _ in 0..n_cols {
                row.push(read_f64(bytes, &mut pos)?);
            }
            rows.push(row);
        }
        series.push(EmcSeries {
            model,
            variant,
            columns,
            rows,
        });
    }
    Some(series)
}

fn unit_line_index(text: &str) -> Option<usize> {
    text.lines().position(|l| l.contains("unit=\""))
}

fn column_of(cell: &str) -> Option<EmcColumn> {
    let name = cell.split('[').next()?.trim();
    if name.is_empty() {
        return None;
    }
    let unit = cell
        .find("unit=\"")
        .and_then(|i| {
            let rest = &cell[i + 6..];
            let end = rest.find('"')?;
            Some(&rest[..end])
        })
        .unwrap_or("");
    Some(EmcColumn {
        name: name.to_string(),
        unit: unit.to_string(),
    })
}

pub fn parse_idv_csv(text: &str, model: &str, variant: &str) -> Option<EmcSeries> {
    let unit_line = unit_line_index(text)?;
    let mut lines = text.lines();
    let header = lines.nth(unit_line)?;
    let columns: Vec<EmcColumn> = header.split(',').filter_map(column_of).collect();
    if columns.is_empty() {
        return None;
    }
    let n_cols = columns.len();
    let mut rows: Vec<Vec<f64>> = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split(',').map(|c| c.trim()).collect();
        if cells.len() != n_cols {
            continue;
        }
        let mut row = Vec::with_capacity(n_cols);
        let mut finite = true;
        for cell in cells {
            match cell.parse::<f64>() {
                Ok(v) if v.is_finite() => row.push(v),
                _ => {
                    finite = false;
                    break;
                }
            }
        }
        if finite {
            rows.push(row);
        }
    }
    if rows.is_empty() {
        return None;
    }
    Some(EmcSeries {
        model: model.to_string(),
        variant: variant.to_string(),
        columns,
        rows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_text() -> &'static str {
        "(index -> (Depth,Radius,Vp,Vs))\n\
         Depth[unit=\"km\"],Radius[unit=\"km\"],Vp[unit=\"km/s\"],Vs[unit=\"km/s\"]\n\
         0.00,6371.00,5.8000,3.3600\n\
         1.00,6370.00,5.8000,3.3600\n\
         2.00,6369.00,5.8000,3.3600\n"
    }

    #[test]
    fn idv_csv_parses_names_units_and_rows() {
        let s = parse_idv_csv(sample_text(), "IASP91", "IASP91").expect("series parses");
        assert_eq!(s.model, "IASP91");
        assert_eq!(s.variant, "IASP91");
        let names: Vec<&str> = s.columns.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["Depth", "Radius", "Vp", "Vs"]);
        assert_eq!(s.columns[1].unit, "km");
        assert_eq!(s.columns[2].unit, "km/s");
        assert_eq!(s.rows.len(), 3);
        assert_eq!(s.rows[0], vec![0.0, 6371.0, 5.8, 3.36]);
    }

    #[test]
    fn idv_csv_integer_values_parse_as_f64() {
        let text = "(index -> (Depth,Vs))\nDepth[unit=\"km\"],Vs[unit=\"km/s\"]\n0,3\n1,3\n";
        let s = parse_idv_csv(text, "MC35", "MC35").expect("series parses");
        assert_eq!(s.rows.len(), 2);
        assert_eq!(s.rows[1], vec![1.0, 3.0]);
    }

    #[test]
    fn idv_csv_non_finite_and_wrong_count_rows_are_skipped() {
        let text = "(index -> (Depth,Vp,Vs))\nDepth[unit=\"km\"],Vp[unit=\"km/s\"],Vs[unit=\"km/s\"]\n0,5.8,3.36\n1,nan,3.36\n2,6.5\n3,8.04,4.47\n";
        let s = parse_idv_csv(text, "T", "T").expect("series parses");
        assert_eq!(s.rows.len(), 2);
        assert_eq!(s.rows[1], vec![3.0, 8.04, 4.47]);
    }

    #[test]
    fn idv_csv_without_unit_line_is_void() {
        assert!(parse_idv_csv("0.00,6371.00,5.8000\n", "T", "T").is_none());
    }

    #[test]
    fn bin_roundtrip_carries_all_series() {
        let a = parse_idv_csv(sample_text(), "IASP91", "IASP91").expect("series parses");
        let b = parse_idv_csv(sample_text(), "IASP91", "IASP91_20101116").expect("series parses");
        let bytes = encode_emc_bin(&[a, b]);
        let back = parse_emc_bin(&bytes).expect("bin parses");
        assert_eq!(back.len(), 2);
        assert_eq!(back[0].model, "IASP91");
        assert_eq!(back[1].variant, "IASP91_20101116");
        assert_eq!(back[0].rows[0], vec![0.0, 6371.0, 5.8, 3.36]);
    }

    #[test]
    fn bin_with_foreign_magic_is_void() {
        let mut bytes = vec![0u8; 9];
        bytes[0..4].copy_from_slice(b"NOPE");
        assert!(parse_emc_bin(&bytes).is_none());
    }
}
