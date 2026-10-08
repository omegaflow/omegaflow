use crate::archivar::{FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"GRRL";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 51;

pub const COMP_G: u32 = 0;
pub const COMP_PARALLAX: u32 = 1;
pub const COMP_BP_RP: u32 = 2;

const RRL_DECLARED_CATALOG_EPOCH_ISO: &str = "2016-01-01T00:00:00";

const COLUMNS: &[(u32, &str, &str)] = &[
    (COMP_G, "gaia_rrl_g_mag", "mag"),
    (COMP_PARALLAX, "gaia_rrl_parallax_mas", "mas"),
    (COMP_BP_RP, "gaia_rrl_bp_rp", "mag"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct RrlRecord {
    pub source_id: u64,
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub g_mag: Option<f64>,
    pub parallax_mas: Option<f64>,
    pub bp_rp: Option<f64>,
}

fn read_optional(bytes: &[u8], off: &mut usize) -> Option<Option<f64>> {
    let flag = *bytes.get(*off)?;
    *off += 1;
    let raw = f64::from_le_bytes(bytes.get(*off..*off + 8)?.try_into().ok()?);
    *off += 8;
    match flag {
        0 => Some(None),
        1 if raw.is_finite() => Some(Some(raw)),
        _ => None,
    }
}

pub fn parse_records(bytes: &[u8]) -> Option<Vec<RrlRecord>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count * RECORD_BYTES {
        return None;
    }
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let source_id = u64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        let ra_deg = f64::from_le_bytes(bytes.get(off + 8..off + 16)?.try_into().ok()?);
        let dec_deg = f64::from_le_bytes(bytes.get(off + 16..off + 24)?.try_into().ok()?);
        if !ra_deg.is_finite()
            || !(0.0..360.0).contains(&ra_deg)
            || !dec_deg.is_finite()
            || !(-90.0..=90.0).contains(&dec_deg)
        {
            return None;
        }
        off += 24;
        let g_mag = read_optional(bytes, &mut off)?;
        let parallax_mas = read_optional(bytes, &mut off)?;
        let bp_rp = read_optional(bytes, &mut off)?;
        out.push(RrlRecord {
            source_id,
            ra_deg,
            dec_deg,
            g_mag,
            parallax_mas,
            bp_rp,
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
    let mut out = Vec::with_capacity(COLUMNS.len());
    for (comp, name, unit) in COLUMNS {
        if component_name(*comp).is_none() {
            continue;
        }
        out.push(FieldConfig {
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
            aperture: crate::archivar::Aperture::None,
        });
    }
    out
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let records = parse_records(bytes)?;
    let lsk = crate::archivar::embedded_lsk()?;
    let tdb = crate::archivar::parse_iso_tdb(RRL_DECLARED_CATALOG_EPOCH_ISO, &lsk)?;
    let mut out = Vec::new();
    for r in records {
        if let Some(v) = r.g_mag {
            out.push((tdb, v, COMP_G));
        }
        if let Some(v) = r.parallax_mas {
            out.push((tdb, v, COMP_PARALLAX));
        }
        if let Some(v) = r.bp_rp {
            out.push((tdb, v, COMP_BP_RP));
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(records: &[RrlRecord]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&(records.len() as u32).to_le_bytes());
        for r in records {
            out.extend_from_slice(&r.source_id.to_le_bytes());
            out.extend_from_slice(&r.ra_deg.to_le_bytes());
            out.extend_from_slice(&r.dec_deg.to_le_bytes());
            for v in [r.g_mag, r.parallax_mas, r.bp_rp] {
                match v {
                    Some(x) => {
                        out.push(1);
                        out.extend_from_slice(&x.to_le_bytes());
                    }
                    None => {
                        out.push(0);
                        out.extend_from_slice(&0.0f64.to_le_bytes());
                    }
                }
            }
        }
        out
    }

    fn record(source_id: u64, g: Option<f64>, par: Option<f64>, bp: Option<f64>) -> RrlRecord {
        RrlRecord {
            source_id,
            ra_deg: 227.281350641993,
            dec_deg: -49.807940181362135,
            g_mag: g,
            parallax_mas: par,
            bp_rp: bp,
        }
    }

    #[test]
    fn roundtrip_keeps_present_and_absent_columns() {
        let records = vec![
            record(
                5902329227124762112,
                Some(17.71831),
                Some(0.1135535987233241),
                Some(0.9884186),
            ),
            record(6070124184786140032, None, None, Some(1.869997)),
        ];
        let bytes = encode(&records);
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_records(&bytes), Some(records));
    }

    #[test]
    fn absent_is_never_a_fabricated_zero() {
        let bytes = encode(&[record(2, None, None, None)]);
        let parsed = parse_records(&bytes).expect("the encoded asset reads back");
        assert_eq!(parsed[0].g_mag, None);
        assert_eq!(parsed[0].parallax_mas, None);
        assert_eq!(parsed[0].bp_rp, None);
        assert_eq!(parse_series(&bytes), Some(Vec::new()));
    }

    #[test]
    fn series_folds_each_present_field_onto_the_declared_catalog_epoch() {
        let bytes = encode(&[record(2, Some(17.7), None, Some(0.98))]);
        let series = parse_series(&bytes).expect("the encoded asset reads back");
        assert_eq!(series.len(), 2);
        let lsk = crate::archivar::embedded_lsk().unwrap();
        let tdb = crate::archivar::parse_iso_tdb(RRL_DECLARED_CATALOG_EPOCH_ISO, &lsk).unwrap();
        assert_eq!(series[0], (tdb, 17.7, COMP_G));
        assert_eq!(series[1], (tdb, 0.98, COMP_BP_RP));
    }

    #[test]
    fn a_truncated_or_foreign_asset_is_void() {
        assert!(parse_records(b"XXXX").is_none());
        let bytes = encode(&[record(2, Some(1.0), None, None)]);
        assert!(parse_records(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn declared_fields_carry_the_read_site_names_and_units() {
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "gaia_rrl_g_mag");
        assert_eq!(fields[1].unit, "mas");
        assert_eq!(component_name(COMP_G), Some("gaia_rrl_g_mag"));
        assert_eq!(component_name(9), None);
    }
}
