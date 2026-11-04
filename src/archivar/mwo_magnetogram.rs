pub const MAGIC: [u8; 4] = *b"MWO1";

pub const COMP_MEAN: u32 = 0;
pub const COMP_MEAN_ABS: u32 = 1;

pub const SECONDS_PER_DAY: f64 = 86400.0;

pub type Record = (i64, f64, f64);

pub const RECORD_BYTES: usize = 8 + 2 * 8;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_MEAN => Some("mwo_mean_blos_t"),
        COMP_MEAN_ABS => Some("mwo_mean_abs_blos_t"),
        _ => None,
    }
}

pub fn write_bin(records: &[Record]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (d, mean, mean_abs) in records {
        buf.extend_from_slice(&d.to_le_bytes());
        buf.extend_from_slice(&mean.to_le_bytes());
        buf.extend_from_slice(&mean_abs.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<Record>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let d = i64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let mean = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let mean_abs = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        out.push((d, mean, mean_abs));
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let lsk = crate::archivar::embedded_lsk()?;
    let records = parse_bin(bytes)?;
    let mut out = Vec::with_capacity(records.len() * 2);
    for (day, mean, mean_abs) in records {
        let Some(t) = lsk.unix_to_tdb(day as f64 * SECONDS_PER_DAY) else {
            continue;
        };
        out.push((t, mean, COMP_MEAN));
        out.push((t, mean_abs, COMP_MEAN_ABS));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![(1013, -2.1e-5, 1.18e-3), (1044, 3.4e-5, 1.21e-3)];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"MWO1\x01\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"KPP1abcdefghijklmnop").is_none());
    }

    #[test]
    fn component_names_match_the_register_fields() {
        assert_eq!(component_name(COMP_MEAN), Some("mwo_mean_blos_t"));
        assert_eq!(component_name(COMP_MEAN_ABS), Some("mwo_mean_abs_blos_t"));
        assert_eq!(component_name(2), None);
    }
}
