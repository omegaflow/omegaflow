use crate::archivar::json::{JsonVal, jnum, jpath_val, parse_json};
use crate::archivar::{FieldConfig, force_id_of, kernel_id_for_force};

pub const EEA_SNLD55: u32 = 0;
pub const EEA_SNLD65: u32 = 1;
pub const EEA_SNLD75: u32 = 2;
pub const EEA_SNLN50: u32 = 3;
pub const EEA_SNLN60: u32 = 4;
pub const EEA_SNLN70: u32 = 5;
pub const EEA_NLD5559: u32 = 6;
pub const EEA_NLD6064: u32 = 7;
pub const EEA_NLD6569: u32 = 8;
pub const EEA_NLD7074: u32 = 9;
pub const EEA_NLN5054: u32 = 10;
pub const EEA_NLN5559: u32 = 11;
pub const EEA_NLN6064: u32 = 12;
pub const EEA_NLN6569: u32 = 13;
pub const EEA_N_INHAB: u32 = 14;

const SNAPSHOT_ISO: &str = "2025-01-01T00:00:00";

const COLUMNS: &[(u32, &str, &str)] = &[
    (EEA_SNLD55, "eea_noise_snld55_count", "SNLD55"),
    (EEA_SNLD65, "eea_noise_snld65_count", "SNLD65"),
    (EEA_SNLD75, "eea_noise_snld75_count", "SNLD75"),
    (EEA_SNLN50, "eea_noise_snln50_count", "SNLN50"),
    (EEA_SNLN60, "eea_noise_snln60_count", "SNLN60"),
    (EEA_SNLN70, "eea_noise_snln70_count", "SNLN70"),
    (EEA_NLD5559, "eea_noise_nld5559_count", "NLD5559"),
    (EEA_NLD6064, "eea_noise_nld6064_count", "NLD6064"),
    (EEA_NLD6569, "eea_noise_nld6569_count", "NLD6569"),
    (EEA_NLD7074, "eea_noise_nld7074_count", "NLD7074"),
    (EEA_NLN5054, "eea_noise_nln5054_count", "NLN5054"),
    (EEA_NLN5559, "eea_noise_nln5559_count", "NLN5559"),
    (EEA_NLN6064, "eea_noise_nln6064_count", "NLN6064"),
    (EEA_NLN6569, "eea_noise_nln6569_count", "NLN6569"),
    (EEA_N_INHAB, "eea_noise_n_inhab_count", "N_INHAB"),
];

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("acoustic") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(force) else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(COLUMNS.len());
    for (comp, name, _) in COLUMNS {
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
            unit: "1".to_string(),
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: crate::archivar::Aperture::None,
        });
    }
    out
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let json = parse_json(text)?;
    let JsonVal::Arr(features) = jpath_val(&json, "features")? else {
        return None;
    };
    let lsk = crate::archivar::embedded_lsk()?;
    let tdb = crate::archivar::parse_iso_tdb(SNAPSHOT_ISO, &lsk)?;
    let mut out = Vec::new();
    for feature in features {
        let Some(attrs) = jpath_val(feature, "attributes") else {
            continue;
        };
        for (comp, _, key) in COLUMNS {
            let Some(v) = jnum(attrs, key) else {
                continue;
            };
            if !v.is_finite() || v < 0.0 {
                continue;
            }
            out.push((tdb, v, *comp));
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"{"geometryType":"esriGeometryPoint","features":[{"attributes":{"SNLN70":1200,"N_INHAB":1977300,"Country":"Austria"},"geometry":{"x":4794019.5,"y":2807644.3}},{"attributes":{"SNLN70":null,"N_INHAB":-5},"geometry":{"x":1.0,"y":2.0}}]}"#;

    #[test]
    fn each_band_folds_onto_the_snapshot_epoch() {
        let rows = parse_series(FIXTURE.as_bytes()).expect("the arcgis body parses");
        assert_eq!(rows.len(), 2);
        let lsk = crate::archivar::embedded_lsk().unwrap();
        let tdb = crate::archivar::parse_iso_tdb(SNAPSHOT_ISO, &lsk).unwrap();
        assert_eq!(rows[0], (tdb, 1200.0, EEA_SNLN70));
        assert_eq!(rows[1], (tdb, 1977300.0, EEA_N_INHAB));
    }

    #[test]
    fn a_null_or_negative_count_stays_absent() {
        let rows = parse_series(FIXTURE.as_bytes()).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|(_, v, _)| v.is_finite() && *v >= 0.0));
        assert_eq!(rows.iter().filter(|(_, _, c)| *c == EEA_SNLN70).count(), 1);
    }

    #[test]
    fn a_foreign_body_is_void() {
        assert!(parse_series(b"not json").is_none());
        assert!(parse_series(br#"{"result":"error"}"#).is_none());
    }

    #[test]
    fn declared_columns_carry_the_read_site_names_and_unit() {
        let fields = declared_fields(604800.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "eea_noise_snld55_count");
        assert!(fields.iter().all(|f| f.unit == "1"));
    }
}
