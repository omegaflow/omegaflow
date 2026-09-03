pub const MAGIC: [u8; 4] = *b"GNG1";

pub fn write_bin(modes: &[(u32, i32, f64, f64)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + modes.len() * 24);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(modes.len() as u32).to_le_bytes());
    for (l, m, t, rms) in modes {
        buf.extend_from_slice(&l.to_le_bytes());
        buf.extend_from_slice(&m.to_le_bytes());
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&rms.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(u32, i32, f64, f64)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / 24 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let l = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        let m = i32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let rms = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        out.push((l, m, t, rms));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let modes = vec![(0u32, 0i32, 100.0, 0.12), (5u32, -3i32, 200.0, 0.07)];
        let bytes = write_bin(&modes);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, modes);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"GNG1x").is_none());
        assert!(parse_bin(b"GNG1abcdefghijklmnopqrstuvwxyz").is_none());
    }
}
