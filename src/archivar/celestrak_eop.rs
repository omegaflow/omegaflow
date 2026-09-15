pub const MAGIC: [u8; 4] = *b"EOP1";

pub const COMP_UT1_UTC: u32 = 1;
pub const COMP_PMX: u32 = 2;
pub const COMP_PMY: u32 = 3;
pub const COMP_MAX: u32 = 3;

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
        if !(COMP_UT1_UTC..=COMP_MAX).contains(&comp) {
            return None;
        }
        if !t.is_finite() || !val.is_finite() {
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
            (10.0, 0.0326338, COMP_UT1_UTC),
            (10.0, -0.012700, COMP_PMX),
            (10.0, 0.213000, COMP_PMY),
        ];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"EOP1abc").is_none());
        assert!(parse_bin(b"OMN1").is_none());
    }

    #[test]
    fn rejects_unknown_component() {
        assert!(parse_bin(&write_bin(&[(10.0, 1.0, COMP_MAX + 1)])).is_none());
        assert!(parse_bin(&write_bin(&[(10.0, 1.0, 0)])).is_none());
    }

    #[test]
    fn rejects_non_finite_time_and_value() {
        assert!(parse_bin(&write_bin(&[(f64::NAN, 1.0, COMP_UT1_UTC)])).is_none());
        assert!(parse_bin(&write_bin(&[(10.0, f64::INFINITY, COMP_PMX)])).is_none());
    }
}
