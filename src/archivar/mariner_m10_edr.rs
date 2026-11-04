use crate::archivar::FieldConfig;

pub const MAGIC: [u8; 4] = *b"M10E";
pub const HEADER_BYTES: usize = 12;
pub const PIXEL_BYTES: usize = 2;

pub const UNIT: &str = "relative";

pub fn component_name(comp: u32) -> String {
    format!("mariner_m10_edr_col_{comp:04}")
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
                tau_src: crate::archivar::types::TauSource::Measured,
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

pub fn write_bin(images: &[(f64, Vec<u16>)]) -> Option<Vec<u8>> {
    let pixels = images.first()?.1.len();
    let mut out = Vec::with_capacity(HEADER_BYTES + images.len() * (8 + pixels * PIXEL_BYTES));
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(images.len() as u32).to_le_bytes());
    out.extend_from_slice(&(pixels as u32).to_le_bytes());
    for (epoch, values) in images {
        if values.len() != pixels {
            return None;
        }
        out.extend_from_slice(&epoch.to_le_bytes());
        for v in values {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let pixels = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    let per = 8 + pixels.checked_mul(PIXEL_BYTES)?;
    if bytes.len() != HEADER_BYTES + count.checked_mul(per)? {
        return None;
    }
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count.saturating_mul(pixels));
    for _ in 0..count {
        let epoch = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        if !epoch.is_finite() {
            return None;
        }
        for j in 0..pixels {
            let at = off + 8 + j * PIXEL_BYTES;
            let value = u16::from_le_bytes(bytes.get(at..at + PIXEL_BYTES)?.try_into().ok()?);
            out.push((epoch, value as f64, j as u32));
        }
        off += per;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrip_flattens_each_image_to_column_rows() {
        let images = vec![
            (1.5e8, vec![10u16, 20, 30]),
            (1.5e8 + 1.0, vec![40u16, 50, 60]),
        ];
        let bytes = write_bin(&images).expect("bin writes");
        let series = parse_bin(&bytes).expect("series parses");
        assert_eq!(
            series,
            vec![
                (1.5e8, 10.0, 0),
                (1.5e8, 20.0, 1),
                (1.5e8, 30.0, 2),
                (1.5e8 + 1.0, 40.0, 0),
                (1.5e8 + 1.0, 50.0, 1),
                (1.5e8 + 1.0, 60.0, 2),
            ]
        );
    }

    #[test]
    fn parse_rejects_foreign_truncated_and_non_finite() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(&MAGIC).is_none());
        let bytes = write_bin(&[(1.0, vec![1u16, 2])]).unwrap();
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        let nan = write_bin(&[(f64::NAN, vec![1u16, 2])]).unwrap();
        assert!(parse_bin(&nan).is_none());
    }

    #[test]
    fn declared_fields_carry_column_names_em_relative_and_no_band() {
        let names: Vec<String> = (0..2).map(component_name).collect();
        let fields = declared_fields(&names, 60.0);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, "mariner_m10_edr_col_0000");
        assert_eq!(fields[1].name, "mariner_m10_edr_col_0001");
        assert_eq!(fields[0].unit, UNIT);
        assert_eq!(fields[0].tau, 60.0);
        assert_eq!(fields[0].freq, crate::archivar::spectral::SPECTRAL_NO_BAND);
        assert_eq!(
            fields[0].bin_width,
            crate::archivar::spectral::SPECTRAL_NO_BAND
        );
        assert_eq!(fields[0].aperture, crate::archivar::Aperture::None);
    }
}
