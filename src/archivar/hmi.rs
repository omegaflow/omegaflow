pub const MAGIC: [u8; 4] = *b"HMI1";

pub const COMP_FRONT: u32 = 1;
pub const COMP_SIDE: u32 = 2;
pub const COMP_COMBINED: u32 = 3;

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / 20 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let v = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let c = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if !(COMP_FRONT..=COMP_COMBINED).contains(&c) || !t.is_finite() || !v.is_finite() {
            return None;
        }
        out.push((t, v, c));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(records: &[(f64, f64, u32)]) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&MAGIC);
        b.extend_from_slice(&(records.len() as u32).to_le_bytes());
        for (t, v, c) in records {
            b.extend_from_slice(&t.to_le_bytes());
            b.extend_from_slice(&v.to_le_bytes());
            b.extend_from_slice(&c.to_le_bytes());
        }
        b
    }

    #[test]
    fn roundtrip_keeps_the_camera_channel() {
        let b = encode(&[(1.0, -0.108827, COMP_COMBINED), (2.0, 0.21, COMP_SIDE)]);
        let r = parse_bin(&b).unwrap();
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].2, COMP_COMBINED);
        assert!((r[0].1 + 0.108827).abs() < 1e-12);
    }

    #[test]
    fn short_and_bad_camera_are_refused() {
        assert!(parse_bin(b"HMI1").is_none());
        let b = encode(&[(1.0, 0.2, 7)]);
        assert!(parse_bin(&b).is_none());
    }
}
