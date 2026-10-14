pub const MAGIC: [u8; 4] = *b"VNRD";
pub const HEADER_BYTES: usize = 8;
pub const REC_BYTES: usize = 20;

pub const COMP_INC: u32 = 0;
pub const COMP_OCP: u32 = 1;
pub const COMP_SCP: u32 = 2;

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count.checked_mul(REC_BYTES)? {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    let mut off = HEADER_BYTES;
    for _ in 0..count {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        let v = f64::from_le_bytes(bytes.get(off + 8..off + 16)?.try_into().ok()?);
        let comp = u32::from_le_bytes(bytes.get(off + 16..off + 20)?.try_into().ok()?);
        if comp > COMP_SCP || !t.is_finite() || !v.is_finite() {
            return None;
        }
        out.push((t, v, comp));
        off += REC_BYTES;
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_INC => Some("venus_radar_inc_deg"),
        COMP_OCP => Some("venus_radar_ocp"),
        COMP_SCP => Some("venus_radar_scp"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
        buf.extend_from_slice(&MAGIC);
        buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
        for (t, val, comp) in records {
            buf.extend_from_slice(&t.to_le_bytes());
            buf.extend_from_slice(&val.to_le_bytes());
            buf.extend_from_slice(&comp.to_le_bytes());
        }
        buf
    }

    #[test]
    fn roundtrip() {
        let records = vec![
            (1.0, 45.5, COMP_INC),
            (1.0, 0.032, COMP_OCP),
            (1.0, 0.011, COMP_SCP),
        ];
        let bytes = write_bin(&records);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"VNRDabc").is_none());
        assert!(parse_bin(b"EOP1").is_none());
    }

    #[test]
    fn rejects_a_line_indexed_asset() {
        assert!(parse_bin(&write_bin(&[(0.0, 3.0, 11999)])).is_none());
    }

    #[test]
    fn names_the_three_quantities() {
        assert_eq!(component_name(COMP_INC), Some("venus_radar_inc_deg"));
        assert_eq!(component_name(COMP_OCP), Some("venus_radar_ocp"));
        assert_eq!(component_name(COMP_SCP), Some("venus_radar_scp"));
        assert_eq!(component_name(COMP_SCP + 1), None);
    }
}
