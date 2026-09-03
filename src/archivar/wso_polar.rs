pub const MAGIC: [u8; 4] = *b"WSP1";

pub fn write_bin(records: &[(i64, f64, f64, f64)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 32);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (d, north, south, avg) in records {
        buf.extend_from_slice(&d.to_le_bytes());
        buf.extend_from_slice(&north.to_le_bytes());
        buf.extend_from_slice(&south.to_le_bytes());
        buf.extend_from_slice(&avg.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(i64, f64, f64, f64)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * 32 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let d = i64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let north = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let south = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let avg = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        out.push((d, north, south, avg));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![
            (9300, 8.9e-5, -1.26e-4, 1.08e-4),
            (9400, 7.1e-5, -1.35e-4, 1.03e-4),
        ];
        let bytes = write_bin(&records);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"WSP1\x02\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"GTS1abcdefgh").is_none());
    }
}
