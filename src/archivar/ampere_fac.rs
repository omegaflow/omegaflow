use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"AMPF";
pub const HEADER_BYTES: usize = 8;
pub const FIELDS: usize = 5;
pub const RECORD_BYTES: usize = FIELDS * 8;

pub const COMP_FAC: u32 = 0;

const COLUMNS: &[(u32, &str, &str)] = &[(COMP_FAC, "ampere_fac_uam2", "uA/m2")];

#[derive(Clone, Debug, PartialEq)]
pub struct AmpereFacRecord {
    pub t_unix: f64,
    pub lat: f64,
    pub lon: f64,
    pub r_km: f64,
    pub fac_uam2: f64,
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<AmpereFacRecord>> {
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
        let fac_uam2 = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        if !lat.is_finite()
            || !lon.is_finite()
            || !r_km.is_finite()
            || !t_unix.is_finite()
            || !fac_uam2.is_finite()
        {
            return None;
        }
        out.push(AmpereFacRecord {
            t_unix,
            lat,
            lon,
            r_km,
            fac_uam2,
        });
    }
    Some(out)
}

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
    let mut out = Vec::with_capacity(records.len());
    for r in records {
        let Some(t) = lsk.unix_to_tdb(r.t_unix) else {
            continue;
        };
        out.push((t, r.fac_uam2, COMP_FAC));
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
            [80.0, 100.0, 6800.0, 1_672_531_200.0, 1.25],
            [75.0, 250.0, 6900.0, 1_672_531_320.0, -0.5],
        ];
        let parsed = parse_bin(&pack(&records)).expect("bin parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].lat, 80.0);
        assert_eq!(parsed[0].lon, 100.0);
        assert_eq!(parsed[0].r_km, 6800.0);
        assert_eq!(parsed[0].t_unix, 1_672_531_200.0);
        assert_eq!(parsed[0].fac_uam2, 1.25);
        assert_eq!(parsed[1].fac_uam2, -0.5);
    }

    #[test]
    fn foreign_magic_and_short_body_are_void() {
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"AMPF").is_none());
        let good = pack(&[[0.0; FIELDS]]);
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn an_absent_value_is_never_a_fabricated_zero() {
        let mut bytes = pack(&[[80.0, 100.0, 6800.0, 100.0, 1.25]]);
        let fac = HEADER_BYTES + 32;
        bytes[fac..fac + 8].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_bin(&bytes).is_none());
    }

    #[test]
    fn component_and_field_carry_the_read_site() {
        assert_eq!(component_name(COMP_FAC), Some("ampere_fac_uam2"));
        assert_eq!(component_name(1), None);
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "ampere_fac_uam2");
        assert_eq!(fields[0].unit, "uA/m2");
        assert_eq!(fields[0].force, force_id_of("electric").unwrap());
    }

    #[test]
    fn parse_series_maps_every_record_to_the_one_component() {
        let bytes = pack(&[
            [80.0, 100.0, 6800.0, 1_672_531_200.0, 1.25],
            [75.0, 250.0, 6900.0, 1_672_531_320.0, -0.5],
        ]);
        let series = parse_series(&bytes).expect("series parses");
        assert_eq!(series.len(), 2);
        assert_eq!(series[0].2, COMP_FAC);
        assert_eq!(series[0].1, 1.25);
        assert_eq!(series[1].1, -0.5);
        assert!(series.iter().all(|(_, _, c)| *c == COMP_FAC));
    }
}
