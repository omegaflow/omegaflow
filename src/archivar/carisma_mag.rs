use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"CRSM";
pub const HEADER_BYTES: usize = 8;
pub const FIELDS: usize = 4;
pub const RECORD_BYTES: usize = FIELDS * 8;

pub const COMP_X: u32 = 0;
pub const COMP_Y: u32 = 1;
pub const COMP_Z: u32 = 2;

const COLUMNS: &[(u32, &str, &str)] = &[
    (COMP_X, "carisma_mag_x_nt", "nT"),
    (COMP_Y, "carisma_mag_y_nt", "nT"),
    (COMP_Z, "carisma_mag_z_nt", "nT"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct CarismaMagRecord {
    pub t_unix: f64,
    pub x_nt: f64,
    pub y_nt: f64,
    pub z_nt: f64,
}

impl CarismaMagRecord {
    fn values(&self) -> [(u32, f64); 3] {
        [
            (COMP_X, self.x_nt),
            (COMP_Y, self.y_nt),
            (COMP_Z, self.z_nt),
        ]
    }
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<CarismaMagRecord>> {
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
        let t_unix = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let x_nt = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let y_nt = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let z_nt = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        if !t_unix.is_finite() || !x_nt.is_finite() || !y_nt.is_finite() || !z_nt.is_finite() {
            return None;
        }
        out.push(CarismaMagRecord {
            t_unix,
            x_nt,
            y_nt,
            z_nt,
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
    let lsk = crate::archivar::embedded_lsk()?;
    let mut out = Vec::with_capacity(records.len() * 3);
    for r in records {
        let Some(t) = lsk.unix_to_tdb(r.t_unix) else {
            continue;
        };
        for (comp, v) in r.values() {
            out.push((t, v, comp));
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pack(records: &[[f64; FIELDS]]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&(records.len() as u32).to_le_bytes());
        for r in records {
            for v in r {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        out
    }

    #[test]
    fn roundtrip_holds_for_the_compiler_layout() {
        let records = [
            [1_231_200_000.0, 11191.027, 4689.269, 56870.568],
            [1_231_200_001.0, -115.212, 0.0, 59011.588],
        ];
        let parsed = parse_bin(&pack(&records)).expect("bin parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].t_unix, 1_231_200_000.0);
        assert_eq!(parsed[0].x_nt, 11191.027);
        assert_eq!(parsed[0].y_nt, 4689.269);
        assert_eq!(parsed[0].z_nt, 56870.568);
        assert_eq!(parsed[1].x_nt, -115.212);
    }

    #[test]
    fn foreign_magic_and_short_body_are_void() {
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"CRSM").is_none());
        let good = pack(&[[0.0; FIELDS]]);
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn an_absent_value_is_never_a_fabricated_zero() {
        let mut bytes = pack(&[[100.0, 1.0, 2.0, 3.0]]);
        let z = HEADER_BYTES + 24;
        bytes[z..z + 8].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_bin(&bytes).is_none());
    }

    #[test]
    fn component_and_field_carry_the_read_site() {
        assert_eq!(component_name(COMP_X), Some("carisma_mag_x_nt"));
        assert_eq!(component_name(COMP_Y), Some("carisma_mag_y_nt"));
        assert_eq!(component_name(COMP_Z), Some("carisma_mag_z_nt"));
        assert_eq!(component_name(3), None);
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].unit, "nT");
        assert_eq!(fields[2].name, "carisma_mag_z_nt");
        assert_eq!(fields[0].force, force_id_of("em").unwrap());
    }

    #[test]
    fn parse_series_yields_three_components_per_vector() {
        let bytes = pack(&[[1_231_200_000.0, 11191.027, 4689.269, 56870.568]]);
        let series = parse_series(&bytes).expect("series parses");
        assert_eq!(series.len(), 3);
        assert_eq!(series[0].2, COMP_X);
        assert_eq!(series[1].2, COMP_Y);
        assert_eq!(series[2].2, COMP_Z);
        assert_eq!(series[2].1, 56870.568);
    }
}
