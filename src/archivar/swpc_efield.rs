use crate::archivar::json::{JsonVal, jnum, jpath_val, jstr, parse_json};
use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const COMP_EX: u32 = 0;
pub const COMP_EY: u32 = 1;

const COLUMNS: &[(u32, &str, &str, &str)] = &[
    (COMP_EX, "swpc_efield_ex", "mV/km", "Ex"),
    (COMP_EY, "swpc_efield_ey", "mV/km", "Ey"),
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

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let json = parse_json(text)?;
    let time_tag = jstr(&json, "time_tag")?;
    let JsonVal::Arr(features) = jpath_val(&json, "features")? else {
        return None;
    };
    let lsk = crate::archivar::embedded_lsk()?;
    let tdb = crate::archivar::parse_iso_tdb(&time_tag, &lsk)?;
    let mut out = Vec::new();
    for feature in features {
        let Some(props) = jpath_val(feature, "properties") else {
            continue;
        };
        for (comp, _, _, key) in COLUMNS {
            let Some(v) = jnum(props, key) else {
                continue;
            };
            if !v.is_finite() {
                continue;
            }
            out.push((tdb, v, *comp));
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = concat!(
        r#"{"time_tag": "2026-10-08", "cadence": 60, "product_version": "US-Canada-1D", "#,
        r#""type": "FeatureCollection", "features": ["#,
        r#"{"type": "Feature", "geometry": {"type": "Point", "coordinates": [-139.0, 60.0]}, "#,
        r#""properties": {"Ex": 0.28, "Ey": -2.59, "quality_flag": 5}}, "#,
        r#"{"type": "Feature", "geometry": {"type": "Point", "coordinates": [-138.0, 61.0]}, "#,
        r#""properties": {"Ex": null, "Ey": -3.76, "quality_flag": 4}}]}"#,
    );

    #[test]
    fn each_point_carries_ex_and_ey_at_the_declared_epoch() {
        let rows = parse_series(FIXTURE.as_bytes()).expect("the geojson body parses");
        let lsk = crate::archivar::embedded_lsk().unwrap();
        let tdb = crate::archivar::parse_iso_tdb("2026-10-08", &lsk).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], (tdb, 0.28, COMP_EX));
        assert_eq!(rows[1], (tdb, -2.59, COMP_EY));
        assert_eq!(rows[2], (tdb, -3.76, COMP_EY));
    }

    #[test]
    fn a_null_or_missing_component_stays_absent() {
        let rows = parse_series(FIXTURE.as_bytes()).unwrap();
        assert_eq!(rows.iter().filter(|(_, _, c)| *c == COMP_EX).count(), 1);
        assert_eq!(rows.iter().filter(|(_, _, c)| *c == COMP_EY).count(), 2);
    }

    #[test]
    fn a_body_without_features_or_stamp_is_void() {
        assert!(parse_series(b"").is_none());
        assert!(parse_series(b"not json").is_none());
        assert!(parse_series(br#"{"time_tag": "2026-10-08", "features": []}"#).is_none());
        assert!(parse_series(br#"{"features": [{"properties": {"Ex": 1.0}}]}"#).is_none());
    }

    #[test]
    fn declared_fields_carry_the_read_site_names_and_units() {
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "swpc_efield_ex");
        assert_eq!(fields[0].unit, "mV/km");
        assert_eq!(fields[1].name, "swpc_efield_ey");
        assert_eq!(fields[0].force, force_id_of("electric").unwrap());
        assert_eq!(component_name(COMP_EY), Some("swpc_efield_ey"));
        assert_eq!(component_name(9), None);
    }
}
