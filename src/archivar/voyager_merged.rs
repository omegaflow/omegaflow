use super::units::ymd_to_days;

pub const MAGIC: &[u8; 4] = b"VYM2";
pub const COMP_B_NT: u32 = 1;
pub const COMP_SPEED_KM_S: u32 = 2;
pub const COMP_DENSITY_N_CC: u32 = 3;
pub const COMP_TEMP_K: u32 = 4;

fn is_fill(tok: &str) -> bool {
    tok.as_bytes().windows(3).any(|w| w == b"999")
}

pub fn parse_text(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let cols: [(usize, u32); 4] = [
        (6, COMP_B_NT),
        (11, COMP_SPEED_KM_S),
        (14, COMP_DENSITY_N_CC),
        (15, COMP_TEMP_K),
    ];
    let mut out = Vec::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        let Some(first) = f.first() else {
            continue;
        };
        if first.starts_with('#') || f.len() < 16 {
            continue;
        }
        let Some(year) = f[0].parse::<i64>().ok() else {
            continue;
        };
        let Some(day) = f[1].parse::<i64>().ok() else {
            continue;
        };
        let Some(hour) = f[2].parse::<i64>().ok() else {
            continue;
        };
        let Some(base) = ymd_to_days(year, 1, 1) else {
            continue;
        };
        let unix = base as f64 * 86400.0 + (day - 1) as f64 * 86400.0 + hour as f64 * 3600.0;
        if !unix.is_finite() {
            continue;
        }
        for (idx, comp) in cols {
            let tok = f[idx];
            if is_fill(tok) {
                continue;
            }
            let Some(value) = tok.parse::<f64>().ok() else {
                continue;
            };
            if !value.is_finite() {
                continue;
            }
            out.push((unix, value, comp));
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn write_series(rows: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + rows.len() * 24);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(rows.len() as u32).to_le_bytes());
    for (t, v, c) in rows {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&v.to_le_bytes());
        out.extend_from_slice(&(*c as f64).to_le_bytes());
    }
    out
}

pub fn parse_series(data: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if data.len() < 8 || &data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 24 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let b = 8 + i * 24;
        let t = f64::from_le_bytes(data.get(b..b + 8)?.try_into().ok()?);
        let v = f64::from_le_bytes(data.get(b + 8..b + 16)?.try_into().ok()?);
        let c = f64::from_le_bytes(data.get(b + 16..b + 24)?.try_into().ok()?);
        if !t.is_finite() || !v.is_finite() {
            return None;
        }
        out.push((t, v, c as u32));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROW: &[u8] = b"1990   1  0  31.07    2.4  205.9 999.999 999.999 999.999 999.999 999.999  386.5    0.1    0.6  0.00719   11026. 9.999e+05\n";

    #[test]
    fn merged_text_keeps_present_components_only() {
        let rows = parse_text(ROW).expect("row parses");
        assert_eq!(rows.len(), 3, "{rows:?}");
        let comps: Vec<u32> = rows.iter().map(|r| r.2).collect();
        assert!(!comps.contains(&COMP_B_NT));
        assert!(comps.contains(&COMP_SPEED_KM_S));
        assert!(comps.contains(&COMP_DENSITY_N_CC));
        assert!(comps.contains(&COMP_TEMP_K));
        assert!((rows[0].1 / 1.0 - 386.5).abs() < 0.01);
    }

    #[test]
    fn series_roundtrip() {
        let rows = vec![(1.0, 2.0, COMP_B_NT), (3.0, 4.0, COMP_SPEED_KM_S)];
        let bin = write_series(&rows);
        assert_eq!(parse_series(&bin).expect("roundtrip"), rows);
        assert!(parse_series(b"XXXX").is_none());
    }
}
