use crate::archivar::FieldConfig;

pub const MAGIC: [u8; 4] = *b"G2CB";
pub const HEADER_BYTES: usize = 8;
pub const REC_BYTES: usize = 24;

pub const GATE_UNIT: &str = "none";

pub fn gate_name(comp: u32) -> String {
    format!("gras_2c_gate_{comp:04}")
}

pub fn gate_field(comp: u32, tau: f64) -> Option<FieldConfig> {
    let force = crate::force::force_id_of("em")?;
    let kernel = crate::force::kernel_id_for_force(force)?;
    let name = gate_name(comp);
    Some(FieldConfig {
        key: name.clone(),
        name,
        band_id: None,
        kernel,
        force,
        tau,
        absorption: 0.0,
        advection: 0.0,
        unit: GATE_UNIT.to_string(),
        freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
        bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
        fold: None,
        aperture: crate::archivar::Aperture::None,
    })
}

fn le_f64(data: &[u8], offset: usize) -> Option<f64> {
    Some(f64::from_le_bytes(
        data.get(offset..offset + 8)?.try_into().ok()?,
    ))
}

pub fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, comp) in records {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&v.to_le_bytes());
        out.extend_from_slice(&comp.to_le_bytes());
        out.extend_from_slice(&[0u8; 4]);
    }
    out
}

pub fn parse_series(data: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if data.len() < HEADER_BYTES || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != HEADER_BYTES + count * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * REC_BYTES;
        let t = le_f64(data, base)?;
        let v = le_f64(data, base + 8)?;
        let comp = u32::from_le_bytes(data[base + 16..base + 20].try_into().ok()?);
        if !t.is_finite() || !v.is_finite() {
            return None;
        }
        out.push((t, v, comp));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrip_carries_time_value_and_channel() {
        let records = vec![(1.5e9, 2.5, 0u32), (1.5e9 + 1.0, -3.5, 1u32)];
        let bytes = write_bin(&records);
        let series = parse_series(&bytes).expect("series parses");
        assert_eq!(series, records);
    }

    #[test]
    fn parse_rejects_foreign_truncated_and_non_finite() {
        assert!(parse_series(b"").is_none());
        assert!(parse_series(b"XXXX").is_none());
        assert!(parse_series(b"G2CB").is_none());
        let bytes = write_bin(&[(1.0, 2.0, 0)]);
        assert!(parse_series(&bytes[..bytes.len() - 1]).is_none());
        let mut nan = write_bin(&[(f64::NAN, 2.0, 0)]);
        assert!(parse_series(&nan).is_none());
        nan = write_bin(&[(1.0, f64::INFINITY, 0)]);
        assert!(parse_series(&nan).is_none());
    }

    #[test]
    fn gate_field_carries_no_spectral_band_until_a_wire_slot_is_earned() {
        let fc = gate_field(0, 600.0).expect("gate field declares");
        assert_eq!(fc.freq, crate::archivar::spectral::SPECTRAL_NO_BAND);
        assert_eq!(fc.bin_width, crate::archivar::spectral::SPECTRAL_NO_BAND);
    }
}
