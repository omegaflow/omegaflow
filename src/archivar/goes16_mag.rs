use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"G16M";
pub const VERSION: u8 = 1;
pub const HEADER_BYTES: usize = 12;
pub const RECORD_BYTES: usize = 20;

pub const COMP_BX: u32 = 0;
pub const COMP_BY: u32 = 1;
pub const COMP_BZ: u32 = 2;

const COLUMNS: &[(u32, &str, &str)] = &[
    (COMP_BX, "goes16_mag_bx_nt", "nT"),
    (COMP_BY, "goes16_mag_by_nt", "nT"),
    (COMP_BZ, "goes16_mag_bz_nt", "nT"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct Goes16MagRecord {
    pub t_tdb: f64,
    pub bx_nt: f64,
    pub by_nt: f64,
    pub bz_nt: f64,
}

impl Goes16MagRecord {
    fn values(&self) -> [(u32, f64); 3] {
        [
            (COMP_BX, self.bx_nt),
            (COMP_BY, self.by_nt),
            (COMP_BZ, self.bz_nt),
        ]
    }
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<Goes16MagRecord>> {
    if bytes.len() < HEADER_BYTES || &bytes[0..4] != MAGIC || bytes[4] != VERSION {
        return None;
    }
    let count = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * RECORD_BYTES;
        let t_tdb = f64::from_le_bytes(bytes[base..base + 8].try_into().ok()?);
        let bx_nt = f32::from_le_bytes(bytes[base + 8..base + 12].try_into().ok()?);
        let by_nt = f32::from_le_bytes(bytes[base + 12..base + 16].try_into().ok()?);
        let bz_nt = f32::from_le_bytes(bytes[base + 16..base + 20].try_into().ok()?);
        if !t_tdb.is_finite()
            || !(bx_nt as f64).is_finite()
            || !(by_nt as f64).is_finite()
            || !(bz_nt as f64).is_finite()
        {
            return None;
        }
        out.push(Goes16MagRecord {
            t_tdb,
            bx_nt: bx_nt as f64,
            by_nt: by_nt as f64,
            bz_nt: bz_nt as f64,
        });
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(force) else {
        return Vec::new();
    };
    COLUMNS
        .iter()
        .map(|(_, name, unit)| FieldConfig {
            key: name.to_string(),
            name: name.to_string(),
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

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let records = parse_bin(bytes)?;
    let mut out = Vec::with_capacity(records.len() * 3);
    for r in records {
        for (comp, v) in r.values() {
            out.push((r.t_tdb, v, comp));
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pack(records: &[[f64; 4]]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&MAGIC);
        out.push(VERSION);
        out.extend_from_slice(&[0u8; 3]);
        out.extend_from_slice(&(records.len() as u32).to_le_bytes());
        for r in records {
            out.extend_from_slice(&r[0].to_le_bytes());
            out.extend_from_slice(&(r[1] as f32).to_le_bytes());
            out.extend_from_slice(&(r[2] as f32).to_le_bytes());
            out.extend_from_slice(&(r[3] as f32).to_le_bytes());
        }
        out
    }

    #[test]
    fn roundtrip_holds_for_the_compiler_layout() {
        let records = [[100.0, -1.5, 2.25, -3.0], [200.0, 4.0, 5.0, 6.0]];
        let parsed = parse_bin(&pack(&records)).expect("bin parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].t_tdb, 100.0);
        assert!((parsed[0].bx_nt - (-1.5)).abs() < 1e-6);
        assert!((parsed[0].by_nt - 2.25).abs() < 1e-6);
        assert!((parsed[0].bz_nt - (-3.0)).abs() < 1e-6);
    }

    #[test]
    fn foreign_magic_version_and_short_body_are_void() {
        assert!(parse_bin(b"XXXX").is_none());
        let good = pack(&[[0.0; 4]]);
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
        let mut wrong_version = good.clone();
        wrong_version[4] = 2;
        assert!(parse_bin(&wrong_version).is_none());
    }

    #[test]
    fn a_non_finite_component_is_never_a_measurement() {
        let mut bytes = pack(&[[100.0, 1.0, 2.0, 3.0]]);
        let by = HEADER_BYTES + 8 + 4;
        bytes[by..by + 4].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(parse_bin(&bytes).is_none());
    }

    #[test]
    fn component_and_field_carry_the_read_site() {
        assert_eq!(component_name(COMP_BX), Some("goes16_mag_bx_nt"));
        assert_eq!(component_name(COMP_BY), Some("goes16_mag_by_nt"));
        assert_eq!(component_name(COMP_BZ), Some("goes16_mag_bz_nt"));
        assert_eq!(component_name(3), None);
        let fields = declared_fields(60.0);
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].unit, "nT");
        assert_eq!(fields[2].name, "goes16_mag_bz_nt");
        assert_eq!(fields[0].force, force_id_of("em").unwrap());
    }

    #[test]
    fn parse_series_yields_three_components_per_vector() {
        let bytes = pack(&[[100.0, -1.5, 2.25, -3.0], [200.0, 4.0, 5.0, 6.0]]);
        let series = parse_series(&bytes).expect("series parses");
        assert_eq!(series.len(), 6);
        assert_eq!(series[0], (100.0, -1.5, COMP_BX));
        assert_eq!(series[1].0, 100.0);
        assert_eq!(series[1].2, COMP_BY);
        assert_eq!(series[2].2, COMP_BZ);
        assert_eq!(series[3].0, 200.0);
        assert_eq!(series[3].2, COMP_BX);
    }
}
