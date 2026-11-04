pub const MAGIC: [u8; 4] = *b"EIT1";

pub const COMP_195_MEAN: u32 = 0;

pub const SECONDS_PER_DAY: f64 = 86400.0;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_195_MEAN => Some("soho_eit_195_mean"),
        _ => None,
    }
}

pub fn write_bin(records: &[(i64, f64)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 16);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (d, mean) in records {
        buf.extend_from_slice(&d.to_le_bytes());
        buf.extend_from_slice(&mean.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(i64, f64)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * 16 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let d = i64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let mean = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        out.push((d, mean));
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let lsk = crate::archivar::embedded_lsk()?;
    let records = parse_bin(bytes)?;
    let mut out = Vec::with_capacity(records.len());
    for (day, mean) in records {
        let Some(t) = lsk.unix_to_tdb(day as f64 * SECONDS_PER_DAY) else {
            continue;
        };
        out.push((t, mean, COMP_195_MEAN));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![(9700, 812.25), (9728, 799.5)];
        let bytes = write_bin(&records);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"EIT1\x02\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"AIA1abcdefghijklmnop").is_none());
    }

    #[test]
    fn component_name_is_the_eit_195_mean() {
        assert_eq!(component_name(COMP_195_MEAN), Some("soho_eit_195_mean"));
        assert_eq!(component_name(1), None);
    }
}
