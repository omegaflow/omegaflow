pub const MAGIC: [u8; 4] = *b"BSV1";

pub fn write_bin(records: &[(f64, f64)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 16);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64)>> {
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
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let v = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        out.push((t, v));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![
            (2191.0 * 86400.0, -1.23456),
            (2191.0 * 86400.0 + 40.0, 2.78901),
        ];
        let bytes = write_bin(&records);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"BSV1\x02\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"GXS1abcdefgh").is_none());
    }
}
