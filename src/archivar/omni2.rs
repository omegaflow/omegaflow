pub const MAGIC: [u8; 4] = *b"OMN1";

pub const COMP_V1800: u32 = 1;
pub const COMP_N1800: u32 = 2;
pub const COMP_T1800: u32 = 3;
pub const COMP_BX: u32 = 4;
pub const COMP_BY: u32 = 5;
pub const COMP_BZ: u32 = 6;
pub const COMP_PRESSURE: u32 = 7;

pub fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, val, comp) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&comp.to_le_bytes());
    }
    buf
}

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
        let val = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let comp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if !(COMP_V1800..=COMP_PRESSURE).contains(&comp) {
            return None;
        }
        out.push((t, val, comp));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![
            (-220_000_000.0, 421.0, COMP_V1800),
            (10.0, 7.3, COMP_N1800),
            (10.0, 88_000.0, COMP_T1800),
            (20.0, -3.2, COMP_BX),
            (20.0, 0.4, COMP_BY),
            (20.0, -12.5, COMP_BZ),
            (30.0, 1.41, COMP_PRESSURE),
        ];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"OMN1abc").is_none());
    }

    #[test]
    fn rejects_unknown_component() {
        let bytes = write_bin(&[(10.0, 1.0, 8)]);
        assert!(parse_bin(&bytes).is_none());
        let bytes = write_bin(&[(10.0, 1.0, 0)]);
        assert!(parse_bin(&bytes).is_none());
    }
}
