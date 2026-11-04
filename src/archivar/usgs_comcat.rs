pub const MAGIC: [u8; 4] = *b"USC1";

pub const COMP_RATE: u32 = 1;
pub const COMP_MAX: u32 = 1;

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

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_RATE => Some("usgs_comcat_m45_rate"),
        _ => None,
    }
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
        if !(COMP_RATE..=COMP_MAX).contains(&comp) {
            return None;
        }
        if !t.is_finite() {
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
            (100_000_000.0, 105.0, COMP_RATE),
            (102_500_000.0, 470.0, COMP_RATE),
        ];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"USC1abc").is_none());
        assert!(parse_bin(b"HSL1").is_none());
    }

    #[test]
    fn rejects_unknown_component() {
        assert!(parse_bin(&write_bin(&[(10.0, 1.0, COMP_MAX + 1)])).is_none());
        assert!(parse_bin(&write_bin(&[(10.0, 1.0, 0)])).is_none());
    }

    #[test]
    fn rejects_non_finite_time() {
        assert!(parse_bin(&write_bin(&[(f64::NAN, 1.0, COMP_RATE)])).is_none());
        assert!(parse_bin(&write_bin(&[(f64::INFINITY, 1.0, COMP_RATE)])).is_none());
    }

    #[test]
    fn component_name_maps_rate() {
        assert_eq!(component_name(COMP_RATE), Some("usgs_comcat_m45_rate"));
        assert_eq!(component_name(COMP_MAX + 1), None);
    }
}
