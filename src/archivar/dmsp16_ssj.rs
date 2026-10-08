use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"DS16";
pub const HEADER_BYTES: usize = 8;
pub const FIELDS: usize = 6;
pub const RECORD_BYTES: usize = FIELDS * 8;

pub const COMP_ELE: u32 = 0;
pub const COMP_ION: u32 = 1;

const COLUMNS: &[(u32, &str, &str)] = &[
    (
        COMP_ELE,
        "dmsp16_ssj_ele_total_energy_flux",
        "eV/cm2/ster/s",
    ),
    (
        COMP_ION,
        "dmsp16_ssj_ion_total_energy_flux",
        "eV/cm2/ster/s",
    ),
];

#[derive(Clone, Debug, PartialEq)]
pub struct Dmsp16SsjRecord {
    pub lat: f64,
    pub lon: f64,
    pub r_km: f64,
    pub t_unix: f64,
    pub ele_flux: f64,
    pub ion_flux: f64,
}

impl Dmsp16SsjRecord {
    fn values(&self) -> [(u32, f64); 2] {
        [(COMP_ELE, self.ele_flux), (COMP_ION, self.ion_flux)]
    }
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<Dmsp16SsjRecord>> {
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
        let lat = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let lon = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let r_km = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let t_unix = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let ele_flux = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let ion_flux = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        if !lat.is_finite()
            || !lon.is_finite()
            || !r_km.is_finite()
            || !t_unix.is_finite()
            || !ele_flux.is_finite()
            || !ion_flux.is_finite()
        {
            return None;
        }
        out.push(Dmsp16SsjRecord {
            lat,
            lon,
            r_km,
            t_unix,
            ele_flux,
            ion_flux,
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
        let records = [
            [65.4, 263.2, 7223.9, 1_285_286_400.0, 1.0e10, 2.0e9],
            [-70.1, 10.0, 7100.0, 1_285_286_460.0, 0.0, 0.0],
        ];
        let parsed = parse_bin(&pack(&records)).expect("bin parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].lat, 65.4);
        assert_eq!(parsed[0].lon, 263.2);
        assert_eq!(parsed[0].r_km, 7223.9);
        assert_eq!(parsed[0].t_unix, 1_285_286_400.0);
        assert_eq!(parsed[0].ele_flux, 1.0e10);
        assert_eq!(parsed[0].ion_flux, 2.0e9);
        assert_eq!(parsed[1].ele_flux, 0.0);
    }

    #[test]
    fn foreign_magic_and_short_body_are_void() {
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"DS16").is_none());
        let good = pack(&[[0.0; FIELDS]]);
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn an_absent_value_is_never_a_fabricated_zero() {
        let mut bytes = pack(&[[65.4, 263.2, 7223.9, 1_285_286_400.0, 1.0e10, 2.0e9]]);
        let ele = HEADER_BYTES + 32;
        bytes[ele..ele + 8].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_bin(&bytes).is_none());
    }

    #[test]
    fn component_and_field_carry_the_read_site() {
        assert_eq!(
            component_name(COMP_ELE),
            Some("dmsp16_ssj_ele_total_energy_flux")
        );
        assert_eq!(
            component_name(COMP_ION),
            Some("dmsp16_ssj_ion_total_energy_flux")
        );
        assert_eq!(component_name(2), None);
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].unit, "eV/cm2/ster/s");
        assert_eq!(fields[1].name, "dmsp16_ssj_ion_total_energy_flux");
        assert_eq!(fields[0].force, force_id_of("em").unwrap());
    }

    #[test]
    fn parse_series_yields_two_components_per_record() {
        let bytes = pack(&[[65.4, 263.2, 7223.9, 1_285_286_400.0, 1.0e10, 2.0e9]]);
        let series = parse_series(&bytes).expect("series parses");
        assert_eq!(series.len(), 2);
        assert_eq!(series[0].2, COMP_ELE);
        assert_eq!(series[1].2, COMP_ION);
        assert_eq!(series[0].1, 1.0e10);
        assert_eq!(series[1].1, 2.0e9);
    }
}
