pub const MAGIC: [u8; 4] = *b"BSN1";

pub fn write_bin(records: &[(i64, f64, f64)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 24);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (d, shift, err) in records {
        buf.extend_from_slice(&d.to_le_bytes());
        buf.extend_from_slice(&shift.to_le_bytes());
        buf.extend_from_slice(&err.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(i64, f64, f64)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * 24 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let d = i64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let shift = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let err = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        out.push((d, shift, err));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![
            (8039, 2.6116599882242364e-7, 7.6730295324456771e-8),
            (8046, -6.2577388392797403e-8, 5.4974610045669586e-8),
        ];
        let bytes = write_bin(&records);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"BSN1\x02\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"GTS1abcdefgh").is_none());
    }
}
