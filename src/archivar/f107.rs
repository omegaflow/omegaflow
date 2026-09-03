pub const MAGIC: [u8; 4] = *b"F107";

pub fn write_bin(records: &[(i64, f64)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 16);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (d, v) in records {
        buf.extend_from_slice(&d.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
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
    for i in 0..n {
        let o = 8 + i * 16;
        let d = i64::from_le_bytes(bytes[o..o + 8].try_into().ok()?);
        let v = f64::from_le_bytes(bytes[o + 8..o + 16].try_into().ok()?);
        out.push((d, v));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![(-1040, 90.5e-22), (-400, 120.3e-22), (20420, 82.1e-22)];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"F107abc").is_none());
        assert!(parse_bin(b"F107\x02\x00\x00\x00\x00").is_none());
    }
}
