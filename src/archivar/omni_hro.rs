pub const MAGIC: [u8; 4] = *b"OMH1";

pub const COMP_IMF_F: u32 = 1;
pub const COMP_IMF_BX_GSE: u32 = 2;
pub const COMP_IMF_BY_GSM: u32 = 3;
pub const COMP_IMF_BZ_GSM: u32 = 4;
pub const COMP_SW_FLOW_SPEED: u32 = 5;
pub const COMP_SW_DENSITY: u32 = 6;
pub const COMP_SW_TEMP: u32 = 7;
pub const COMP_SW_PRESSURE: u32 = 8;
pub const COMP_MAX: u32 = 8;

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
        if !(COMP_IMF_F..=COMP_MAX).contains(&comp) {
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
            (-220_000_000.0, 4.9, COMP_IMF_F),
            (10.0, -3.2, COMP_IMF_BX_GSE),
            (20.0, 2.9, COMP_IMF_BY_GSM),
            (20.0, -12.5, COMP_IMF_BZ_GSM),
            (30.0, 421.0, COMP_SW_FLOW_SPEED),
            (30.0, 6.3, COMP_SW_DENSITY),
            (30.0, 44_793.0, COMP_SW_TEMP),
            (30.0, 1.41, COMP_SW_PRESSURE),
        ];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"OMH1abc").is_none());
    }

    #[test]
    fn rejects_unknown_component() {
        let bytes = write_bin(&[(10.0, 1.0, COMP_MAX + 1)]);
        assert!(parse_bin(&bytes).is_none());
        let bytes = write_bin(&[(10.0, 1.0, 0)]);
        assert!(parse_bin(&bytes).is_none());
    }
}
