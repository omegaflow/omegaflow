pub const MAGIC: [u8; 4] = *b"GTS1";

pub fn write_bin(modes: &[(u32, i32, i64, f64)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + modes.len() * 24);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(modes.len() as u32).to_le_bytes());
    for (l, n, d, freq) in modes {
        buf.extend_from_slice(&l.to_le_bytes());
        buf.extend_from_slice(&n.to_le_bytes());
        buf.extend_from_slice(&d.to_le_bytes());
        buf.extend_from_slice(&freq.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(u32, i32, i64, f64)>> {
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
        let l = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        let n = i32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        let d = i64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let freq = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        out.push((l, n, d, freq));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let modes = vec![
            (0u32, 14i32, 9000i64, 2.0934465e-3),
            (0u32, 17i32, 9036i64, 2.4963196e-3),
            (1u32, 17i32, 9036i64, 2.5589609e-3),
        ];
        let bytes = write_bin(&modes);
        assert_eq!(parse_bin(&bytes).unwrap(), modes);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"GTS1\x02\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"F107abcdefgh").is_none());
    }
}
