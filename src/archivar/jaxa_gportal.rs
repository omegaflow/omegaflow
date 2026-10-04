use crate::archivar::json::{JsonVal, jnum, jpath_val, jstr, parse_json};
use crate::archivar::{FieldConfig, force_id_of, kernel_id_for_force};

pub const COMP_LON: u32 = 0;
pub const COMP_LAT: u32 = 1;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_LON => Some("jaxa_gportal_lon_deg"),
        COMP_LAT => Some("jaxa_gportal_lat_deg"),
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
    let mut out = Vec::with_capacity(2);
    for comp in [COMP_LON, COMP_LAT] {
        let Some(name) = component_name(comp) else {
            continue;
        };
        out.push(FieldConfig {
            key: name.to_string(),
            name: name.to_string(),
            kernel,
            force,
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: "deg".to_string(),
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
            fold: None,
        });
    }
    out
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let json = parse_json(text)?;
    let JsonVal::Arr(records) = jpath_val(&json, "records")? else {
        return None;
    };
    let lsk = crate::archivar::embedded_lsk()?;
    let mut out = Vec::with_capacity(records.len() * 2);
    for row in records {
        let (Some(lon), Some(lat)) = (jnum(row, "lon"), jnum(row, "lat")) else {
            continue;
        };
        let Some(begin) = jstr(row, "begin") else {
            continue;
        };
        if !lon.is_finite() || !(-180.0..=360.0).contains(&lon) {
            continue;
        }
        if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
            continue;
        }
        let Some(tdb) = crate::archivar::parse_iso_tdb(&begin, &lsk) else {
            continue;
        };
        out.push((tdb, lon, COMP_LON));
        out.push((tdb, lat, COMP_LAT));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"{"dataset":"12001000","from":"2015/01/01","to":"2015/01/01","numberOfRecordsMatched":2,"numberOfRecordsReturned":2,"records":[{"id":"PROD_A","lon":11.0,"lat":21.0,"points":4,"bbox":[10.0,20.0,12.0,22.0],"size":100,"begin":"2015-01-01T01:29:32.201Z"},{"id":"PROD_B","lon":400.0,"lat":200.0,"points":2,"bbox":[1.0,2.0,3.0,4.0],"size":200,"begin":"2015-01-01T03:00:00"}]}"#;

    #[test]
    fn both_columns_fold_onto_the_observed_epoch() {
        let rows = parse_series(FIXTURE.as_bytes()).expect("the manifest parses");
        assert_eq!(rows.len(), 2);
        let lsk = crate::archivar::embedded_lsk().expect("the embedded naif0012 table stands");
        let tdb = crate::archivar::parse_iso_tdb("2015-01-01T01:29:32.201Z", &lsk)
            .expect("the observation epoch folds onto TDB");
        assert_eq!(rows[0], (tdb, 11.0, COMP_LON));
        assert_eq!(rows[1], (tdb, 21.0, COMP_LAT));
    }

    #[test]
    fn an_implausible_footprint_stays_absent() {
        let rows = parse_series(FIXTURE.as_bytes()).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(!rows.iter().any(|(_, v, _)| *v == 400.0 || *v == 200.0));
    }

    #[test]
    fn an_empty_manifest_is_a_held_state_and_a_foreign_body_is_void() {
        assert_eq!(parse_series(br#"{"records":[]}"#), Some(Vec::new()));
        assert!(parse_series(b"not json").is_none());
        assert!(parse_series(br#"{"result":"error"}"#).is_none());
    }

    #[test]
    fn a_record_without_begin_stays_absent() {
        let body = br#"{"records":[{"id":"PROD_C","lon":11.0,"lat":21.0,"size":100}]}"#;
        assert_eq!(parse_series(body), Some(Vec::new()));
    }

    #[test]
    fn declared_columns_carry_the_read_site_names_and_no_default_field() {
        let fields = declared_fields(604800.0);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, "jaxa_gportal_lon_deg");
        assert_eq!(fields[1].name, "jaxa_gportal_lat_deg");
        assert!(fields.iter().all(|f| f.unit == "deg"));
    }
}
