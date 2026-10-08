use crate::archivar::{FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"GPMK";
pub const REC_BYTES: usize = 26 * 8;
pub const COMP_ECHO_POWER: u32 = 0;
pub const TTL_S: f64 = 86400.0;
pub const TAU_S: f64 = 86400.0;
pub const FORCE_EM: f64 = 0.0;
pub const KERNEL_INVERSE_SQUARE: f64 = 0.0;
pub const SLOT_VAL: usize = 3;
pub const SLOT_EPOCH: usize = 4;
pub const SLOT_PRESENCE: usize = 25;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_ECHO_POWER => Some("jaxa_gpm_ku_echo_power_w"),
        _ => None,
    }
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(force) else {
        return Vec::new();
    };
    vec![FieldConfig {
        key: "jaxa_gpm_ku_echo_power_w".to_string(),
        name: "jaxa_gpm_ku_echo_power_w".to_string(),
        band_id: None,
        kernel,
        force,
        tau,
        absorption: 0.0,
        advection: 0.0,
        unit: "W".to_string(),
        freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
        bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
        fold: None,
        aperture: crate::archivar::Aperture::None,
    }]
}

pub fn write_bin(records: &[[f64; 26]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<[f64; 26]>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let base = 8 + i * REC_BYTES;
        let mut r = [0.0f64; 26];
        for (k, slot) in r.iter_mut().enumerate() {
            let o = base + k * 8;
            *slot = f64::from_le_bytes(bytes[o..o + 8].try_into().ok()?);
        }
        out.push(r);
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let records = parse_bin(bytes)?;
    Some(
        records
            .iter()
            .filter(|r| r[SLOT_PRESENCE] == 1.0)
            .map(|r| (r[SLOT_EPOCH], r[SLOT_VAL], COMP_ECHO_POWER))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(val: Option<f64>, epoch: f64) -> [f64; 26] {
        let mut r = [0.0f64; 26];
        r[0] = 1.0;
        r[SLOT_EPOCH] = epoch;
        r[7] = 100.0;
        r[8] = KERNEL_INVERSE_SQUARE;
        r[9] = FORCE_EM;
        match val {
            Some(v) => {
                r[SLOT_VAL] = v;
                r[SLOT_PRESENCE] = 1.0;
            }
            None => r[SLOT_PRESENCE] = 0.0,
        }
        r
    }

    #[test]
    fn bin_roundtrip_keeps_every_wire_slot() {
        let records = vec![record(Some(1.0e-12), 1400.0), record(None, 1400.0)];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        assert_eq!(parse_bin(&bytes).expect("roundtrip parses"), records);
    }

    #[test]
    fn an_absent_bin_keeps_no_series_row() {
        let bytes = write_bin(&[record(Some(2.0e-13), 1400.0), record(None, 1400.0)]);
        assert_eq!(
            parse_series(&bytes).expect("series parses"),
            vec![(1400.0, 2.0e-13, COMP_ECHO_POWER)]
        );
    }

    #[test]
    fn a_foreign_or_truncated_bin_is_void() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&[record(Some(1.0), 2.0)]);
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        assert!(parse_series(b"GPMK\x01\x00\x00\x00").is_none());
    }

    #[test]
    fn declared_field_carries_the_read_site_name_and_unit() {
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "jaxa_gpm_ku_echo_power_w");
        assert_eq!(fields[0].unit, "W");
        assert_eq!(
            component_name(COMP_ECHO_POWER),
            Some("jaxa_gpm_ku_echo_power_w")
        );
        assert_eq!(component_name(99), None);
    }
}
