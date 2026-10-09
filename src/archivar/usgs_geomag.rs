use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const COMP_E_E: u32 = 0;
pub const COMP_E_N: u32 = 1;

pub const MAGIC: [u8; 4] = *b"UGE1";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 20;

const COLUMNS: &[(u32, &str, &str, &str)] = &[
    (COMP_E_E, "usgs_geomag_e_e_mv_km", "mV/km", "E-E"),
    (COMP_E_N, "usgs_geomag_e_n_mv_km", "mV/km", "E-N"),
];

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("electric") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(force) else {
        return Vec::new();
    };
    COLUMNS
        .iter()
        .map(|(_, name, unit, _)| FieldConfig {
            key: name.to_string(),
            name: name.to_string(),
            band_id: None,
            kernel,
            force,
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: (*unit).to_string(),
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: Aperture::None,
        })
        .collect()
}

pub fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, val, element) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&element.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + n * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let val = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let element = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if element > COMP_E_N {
            return None;
        }
        if !t.is_finite() || !val.is_finite() {
            return None;
        }
        out.push((t, val, element));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_keeps_present_components() {
        let records = vec![
            (100.0, 234.814, COMP_E_E),
            (100.0, -12.5, COMP_E_N),
            (160.0, 0.0, COMP_E_E),
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), HEADER_BYTES + 3 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(records));
    }

    #[test]
    fn a_zero_is_a_real_measurement() {
        let bytes = write_bin(&[(100.0, 0.0, COMP_E_N)]);
        assert_eq!(parse_bin(&bytes), Some(vec![(100.0, 0.0, COMP_E_N)]));
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"USC1abc").is_none());
        assert!(parse_bin(b"UGE1abcd").is_none());
    }

    #[test]
    fn rejects_unknown_element_and_non_finite() {
        assert!(parse_bin(&write_bin(&[(10.0, 1.0, COMP_E_N + 1)])).is_none());
        assert!(parse_bin(&write_bin(&[(f64::NAN, 1.0, COMP_E_E)])).is_none());
        assert!(parse_bin(&write_bin(&[(10.0, f64::INFINITY, COMP_E_E)])).is_none());
    }

    #[test]
    fn declared_fields_carry_the_read_site_names_and_units() {
        let fields = declared_fields(60.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "usgs_geomag_e_e_mv_km");
        assert_eq!(fields[0].unit, "mV/km");
        assert_eq!(fields[1].name, "usgs_geomag_e_n_mv_km");
        assert_eq!(fields[0].force, force_id_of("electric").unwrap());
        assert_eq!(component_name(COMP_E_N), Some("usgs_geomag_e_n_mv_km"));
        assert_eq!(component_name(9), None);
    }
}
