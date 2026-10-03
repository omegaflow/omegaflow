use super::units::ymd_to_days;

pub const MAGIC: &[u8; 4] = b"VKTX";
pub const COMP_RANGE_KM: u32 = 1;
pub const COMP_RANGE_RATE_KM_S: u32 = 2;

const KM_PER_US_RT: f64 = 0.149_896_229;

fn month_index(s: &str) -> Option<u32> {
    let m = match s.to_ascii_uppercase().as_str() {
        "JAN" => 1,
        "FEB" => 2,
        "MAR" => 3,
        "APR" => 4,
        "MAY" => 5,
        "JUN" => 6,
        "JUL" => 7,
        "AUG" => 8,
        "SEP" => 9,
        "OCT" => 10,
        "NOV" => 11,
        "DEC" => 12,
        _ => return None,
    };
    Some(m)
}

fn hms_seconds(s: &str) -> Option<f64> {
    let mut it = s.split(':');
    let h = it.next()?.parse::<f64>().ok()?;
    let m = it.next()?.parse::<f64>().ok()?;
    let sec = it.next()?.parse::<f64>().ok()?;
    if it.next().is_some() {
        return None;
    }
    let out = h * 3600.0 + m * 60.0 + sec;
    if out.is_finite() { Some(out) } else { None }
}

pub fn parse_text(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = t.split_whitespace().collect();
        if f.len() < 9 {
            continue;
        }
        let Some(sc) = f[0].parse::<i64>().ok() else {
            continue;
        };
        if sc != 1 && sc != 2 {
            continue;
        }
        let Some(year) = f[3].parse::<i64>().ok() else {
            continue;
        };
        let Some(month) = month_index(f[4]) else {
            continue;
        };
        let Some(day) = f[5].parse::<u32>().ok() else {
            continue;
        };
        let Some(secs) = hms_seconds(f[6]) else {
            continue;
        };
        let Some(days) = ymd_to_days(year, month, day) else {
            continue;
        };
        let unix = days as f64 * 86400.0 + secs;
        let Some(value) = f[7].parse::<f64>().ok() else {
            continue;
        };
        if !unix.is_finite() || !value.is_finite() {
            continue;
        }
        let comp = if f.len() >= 10 {
            COMP_RANGE_RATE_KM_S
        } else {
            COMP_RANGE_KM
        };
        out.push((unix, KM_PER_US_RT * value, comp));
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

    const RANGE: &[u8] =
        b"# Viking lander range data\n#\n 1 43 43 1976 JUL 21 07:58:52   2285395397.612   .047\n";
    const DIFFERENCED: &[u8] =
        b"# Viking Lander Differenced Range\n 1 43 43 1976 JUL 21 07:15:32    18.4601646559   1.33   60.0\n";

    #[test]
    fn range_text_becomes_one_way_km() {
        let rows = parse_text(RANGE).expect("range parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].2, COMP_RANGE_KM);
        assert!((rows[0].1 / 1.0e6 - 342.6).abs() < 1.0, "{}", rows[0].1);
    }

    #[test]
    fn differenced_text_becomes_range_rate() {
        let rows = parse_text(DIFFERENCED).expect("differenced parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].2, COMP_RANGE_RATE_KM_S);
        assert!((rows[0].1 / 1.0 - 2.767).abs() < 0.01, "{}", rows[0].1);
    }

    #[test]
    fn series_roundtrip() {
        let rows = vec![(1.0, 2.0, COMP_RANGE_KM), (3.0, 4.0, COMP_RANGE_RATE_KM_S)];
        let bin = write_series(&rows);
        assert_eq!(parse_series(&bin).expect("roundtrip"), rows);
        assert!(parse_series(b"XXXX").is_none());
    }
}
