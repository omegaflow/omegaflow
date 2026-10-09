use crate::archivar::FieldConfig;

pub const MAGIC: [u8; 4] = *b"TASI";
pub const HEADER_BYTES: usize = 12;
pub const PIXEL_BYTES: usize = 2;

pub const UNIT: &str = "relative";

pub fn component_name(comp: u32) -> String {
    format!("themis_asi_px_{comp:04}")
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

    fn write_bin(frames: &[(f64, Vec<u16>)], pixels: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_BYTES + frames.len() * (8 + pixels * PIXEL_BYTES));
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&(frames.len() as u32).to_le_bytes());
        out.extend_from_slice(&(pixels as u32).to_le_bytes());
        for (epoch, values) in frames {
            out.extend_from_slice(&epoch.to_le_bytes());
            for v in values {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        out
    }

    #[test]
    fn bin_roundtrip_flattens_each_frame_to_pixel_rows() {
        let frames = vec![
            (1.5e9, vec![10u16, 20, 30]),
            (1.5e9 + 1.0, vec![40u16, 50, 60]),
        ];
        let bytes = write_bin(&frames, 3);
        let series = parse_bin(&bytes).expect("series parses");
        assert_eq!(
            series,
            vec![
                (1.5e9, 10.0, 0),
                (1.5e9, 20.0, 1),
                (1.5e9, 30.0, 2),
                (1.5e9 + 1.0, 40.0, 0),
                (1.5e9 + 1.0, 50.0, 1),
                (1.5e9 + 1.0, 60.0, 2),
            ]
        );
    }

    #[test]
    fn parse_rejects_foreign_truncated_and_non_finite() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(&MAGIC).is_none());
        let bytes = write_bin(&[(1.0, vec![1u16, 2])], 2);
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        let nan = write_bin(&[(f64::NAN, vec![1u16, 2])], 2);
        assert!(parse_bin(&nan).is_none());
    }

    #[test]
    fn declared_fields_carry_pixel_names_em_relative_and_no_band() {
        let names: Vec<String> = (0..2).map(component_name).collect();
        let fields = declared_fields(&names, 60.0);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, "themis_asi_px_0000");
        assert_eq!(fields[1].name, "themis_asi_px_0001");
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
