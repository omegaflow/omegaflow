use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"SSUI";
pub const HEADER_BYTES: usize = 8;
pub const FIELDS: usize = 3;
pub const RECORD_BYTES: usize = FIELDS * 8;

pub const COMP_POWER_NORTH: u32 = 0;
pub const COMP_POWER_SOUTH: u32 = 1;

const COLUMNS: &[(u32, &str, &str)] = &[
    (COMP_POWER_NORTH, "ssusi_hemisphere_power_north", "GW"),
    (COMP_POWER_SOUTH, "ssusi_hemisphere_power_south", "GW"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct SsusiAuroraRecord {
    pub t_unix: f64,
    pub power_north_gw: f64,
    pub power_south_gw: f64,
}

impl SsusiAuroraRecord {
    fn values(&self) -> [(u32, f64); 2] {
        [
            (COMP_POWER_NORTH, self.power_north_gw),
            (COMP_POWER_SOUTH, self.power_south_gw),
        ]
    }
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<SsusiAuroraRecord>> {
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
        let power_north_gw = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let power_south_gw = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        if !t_unix.is_finite() || !power_north_gw.is_finite() || !power_south_gw.is_finite() {
            return None;
        }
        out.push(SsusiAuroraRecord {
            t_unix,
            power_north_gw,
            power_south_gw,
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

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let records = parse_bin(bytes)?;
    let lsk = crate::archivar::embedded_lsk()?;
    let mut out = Vec::with_capacity(records.len() * 2);
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
        let records = [[1_767_225_600.0, 12.5, 3.25], [1_767_225_660.0, 14.0, 4.0]];
        let parsed = parse_bin(&pack(&records)).expect("bin parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].t_unix, 1_767_225_600.0);
        assert_eq!(parsed[0].power_north_gw, 12.5);
        assert_eq!(parsed[0].power_south_gw, 3.25);
        assert_eq!(parsed[1].power_north_gw, 14.0);
    }

    #[test]
    fn foreign_magic_and_short_body_are_void() {
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"SSUI").is_none());
        let good = pack(&[[0.0; FIELDS]]);
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn an_absent_value_is_never_a_fabricated_zero() {
        let mut bytes = pack(&[[100.0, 1.0, 2.0]]);
        let z = HEADER_BYTES + 8;
        bytes[z..z + 8].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_bin(&bytes).is_none());
    }

    #[test]
    fn component_and_field_carry_the_read_site() {
        assert_eq!(
            component_name(COMP_POWER_NORTH),
            Some("ssusi_hemisphere_power_north")
        );
        assert_eq!(
            component_name(COMP_POWER_SOUTH),
            Some("ssusi_hemisphere_power_south")
        );
        assert_eq!(component_name(2), None);
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].unit, "GW");
        assert_eq!(fields[1].name, "ssusi_hemisphere_power_south");
        assert_eq!(fields[0].force, force_id_of("em").unwrap());
    }

    #[test]
    fn parse_series_yields_both_hemispheres_per_record() {
        let bytes = pack(&[[1_767_225_600.0, 12.5, 3.25]]);
        let series = parse_series(&bytes).expect("series parses");
        assert_eq!(series.len(), 2);
        assert_eq!(series[0].1, 12.5);
        assert_eq!(series[0].2, COMP_POWER_NORTH);
        assert_eq!(series[1].1, 3.25);
        assert_eq!(series[1].2, COMP_POWER_SOUTH);
    }
}
