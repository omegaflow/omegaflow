pub const MAGIC: [u8; 4] = *b"TSE1";
pub const HEADER_BYTES: usize = 8;
pub const REC_BYTES: usize = 20;
pub const COMP_COUNT: u32 = 8;

const NAMES: [&str; COMP_COUNT as usize] = [
    "see_euv_0_1_7nm_irradiance",
    "see_euv_27_34nm_irradiance",
    "see_euv_304_irradiance",
    "see_euv_335_irradiance",
    "see_euv_368_irradiance",
    "see_lyman_alpha_121_5nm_irradiance",
    "see_133_5nm_irradiance",
    "see_145_165nm_irradiance",
];

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES.checked_add(count.checked_mul(REC_BYTES)?)? {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    let mut off = HEADER_BYTES;
    for _ in 0..count {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        let value = f64::from_le_bytes(bytes.get(off + 8..off + 16)?.try_into().ok()?);
        let comp = u32::from_le_bytes(bytes.get(off + 16..off + 20)?.try_into().ok()?);
        if comp >= COMP_COUNT || !t.is_finite() || !value.is_finite() {
            return None;
        }
        out.push((t, value, comp));
        off += REC_BYTES;
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    NAMES.get(comp as usize).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
        buf.extend_from_slice(&MAGIC);
        buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
        for (t, value, comp) in records {
            buf.extend_from_slice(&t.to_le_bytes());
            buf.extend_from_slice(&value.to_le_bytes());
            buf.extend_from_slice(&comp.to_le_bytes());
        }
        buf
    }

    #[test]
    fn roundtrip_keeps_all_eight_bands() {
        let records: Vec<(f64, f64, u32)> = (0..8)
            .map(|comp| (300.0 + comp as f64, 0.001 * (comp as f64 + 1.0), comp))
            .collect();
        let bytes = write_bin(&records);
        assert_eq!(parse_bin(&bytes), Some(records));
        assert_eq!(component_name(0), Some("see_euv_0_1_7nm_irradiance"));
        assert_eq!(
            component_name(5),
            Some("see_lyman_alpha_121_5nm_irradiance")
        );
        assert_eq!(component_name(8), None);
    }

    #[test]
    fn foreign_truncated_and_out_of_range_bands_stay_unread() {
        let bytes = write_bin(&[(300.0, 0.001, 7)]);
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        assert!(parse_bin(&write_bin(&[(300.0, 0.001, 8)])).is_none());
    }
}
