use crate::archivar::FieldConfig;

pub const MAGIC: [u8; 4] = *b"M4OD";
pub const VERSION: u8 = 1;
pub const HEADER_LEN: usize = 13;
pub const REC_BYTES: usize = 18;

pub const UNIT: &str = "cps";

pub fn component_name(comp: u32) -> String {
    format!("mariner4_odg_dtype_{comp:02}")
}

pub fn declared_fields(names: &[String], tau: f64) -> Vec<FieldConfig> {
    let Some(channel_ref) = crate::force::force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = crate::force::kernel_id_for_force(channel_ref) else {
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
                channel_ref: Some(channel_ref),
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

pub fn write_header(out: &mut Vec<u8>, n_rows: u64) {
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&n_rows.to_le_bytes());
}

pub fn parse_header(b: &[u8]) -> Option<u64> {
    if b.len() < HEADER_LEN || b[0..4] != MAGIC || b[4] != VERSION {
        return None;
    }
    Some(u64::from_le_bytes(b[5..13].try_into().ok()?))
}

pub fn write_bin(rows: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + rows.len() * REC_BYTES);
    write_header(&mut out, rows.len() as u64);
    for &(t, v, comp) in rows {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&v.to_le_bytes());
        out.extend_from_slice(&(comp as u16).to_le_bytes());
    }
    out
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let n = parse_header(bytes)? as usize;
    let body = bytes.get(HEADER_LEN..)?;
    if body.len() < n.checked_mul(REC_BYTES)? {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let at = i * REC_BYTES;
        let t = f64::from_le_bytes(body.get(at..at + 8)?.try_into().ok()?);
        let v = f64::from_le_bytes(body.get(at + 8..at + 16)?.try_into().ok()?);
        let comp = u16::from_le_bytes(body.get(at + 16..at + 18)?.try_into().ok()?);
        if !t.is_finite() || !v.is_finite() {
            return None;
        }
        out.push((t, v, comp as u32));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrip_keeps_time_value_and_dtype() {
        let rows = vec![(1.0e9, 2114.0, 8u32), (1.0e9 + 60.0, -42.5, 8)];
        let bytes = write_bin(&rows);
        assert_eq!(parse_bin(&bytes), Some(rows));
    }

    #[test]
    fn parse_rejects_foreign_truncated_and_non_finite() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(&MAGIC).is_none());
        let bytes = write_bin(&[(1.0, 2.0, 8)]);
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        let nan = write_bin(&[(1.0, f64::NAN, 8)]);
        assert!(parse_bin(&nan).is_none());
    }

    #[test]
    fn declared_fields_carry_dtype_names_em_and_no_band() {
        let names: Vec<String> = (1..=3).map(component_name).collect();
        let fields = declared_fields(&names, 60.0);
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].name, "mariner4_odg_dtype_01");
        assert_eq!(fields[0].unit, UNIT);
        assert_eq!(fields[0].freq, crate::archivar::spectral::SPECTRAL_NO_BAND);
    }
}
