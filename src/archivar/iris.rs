use super::FieldConfig;

pub const MAGIC: [u8; 4] = *b"IRIS";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 20;
pub const UNIT: &str = "count";

pub fn component_name(comp: u32) -> String {
    format!("iris_{comp:04}")
}

pub fn declared_fields(names: &[String], tau: f64) -> Vec<FieldConfig> {
    let Some(force) = crate::force::force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = crate::force::kernel_id_for_force(force) else {
        return Vec::new();
    };
    names
        .iter()
        .enumerate()
        .map(|(comp, _)| {
            let name = component_name(comp as u32);
            FieldConfig {
                key: name.clone(),
                name,
                band_id: None,
                kernel,
                force,
                tau,
                absorption: 0.0,
                advection: 0.0,
                unit: UNIT.to_string(),
                freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
                bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
                fold: None,
                aperture: crate::archivar::Aperture::None,
            }
        })
        .collect()
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count.checked_mul(RECORD_BYTES)? {
        return None;
    }
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        let value = f64::from_le_bytes(bytes.get(off + 8..off + 16)?.try_into().ok()?);
        let comp = u32::from_le_bytes(bytes.get(off + 16..off + 20)?.try_into().ok()?);
        if !t.is_finite() || !value.is_finite() {
            return None;
        }
        out.push((t, value, comp));
        off += RECORD_BYTES;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&(records.len() as u32).to_le_bytes());
        for (t, v, comp) in records {
            out.extend_from_slice(&t.to_le_bytes());
            out.extend_from_slice(&v.to_le_bytes());
            out.extend_from_slice(&comp.to_le_bytes());
        }
        out
    }

    #[test]
    fn bin_roundtrip_carries_time_value_and_component() {
        let records = vec![(1.5e9, 2.5, 0u32), (1.5e9 + 1.0, -3.5, 1u32)];
        let bytes = write_bin(&records);
        let series = parse_bin(&bytes).expect("series parses");
        assert_eq!(series, records);
    }

    #[test]
    fn parse_rejects_foreign_and_truncated_bytes() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(&MAGIC).is_none());
        let bytes = write_bin(&[(1.0, 2.0, 0)]);
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }
}
