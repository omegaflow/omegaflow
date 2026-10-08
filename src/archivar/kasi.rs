use crate::archivar::json::{JsonVal, jnum, parse_json};
use crate::archivar::{FieldConfig, force_id_of, kernel_id_for_force};

pub const COMP_RA: u32 = 0;
pub const COMP_DEC: u32 = 1;

pub const UNIX_JD_OFFSET: f64 = 2440587.5;
pub const SECONDS_PER_DAY: f64 = 86400.0;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_RA => Some("kasi_ra_deg"),
        COMP_DEC => Some("kasi_dec_deg"),
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
    for comp in [COMP_RA, COMP_DEC] {
        let Some(name) = component_name(comp) else {
            continue;
        };
        out.push(FieldConfig {
            key: name.to_string(),
            name: name.to_string(),
            band_id: None,
            kernel,
            force,
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: "deg".to_string(),
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
    let JsonVal::Arr(rows) = parse_json(text)? else {
        return None;
    };
    let lsk = crate::archivar::embedded_lsk()?;
    let mut out = Vec::with_capacity(rows.len() * 2);
    for row in &rows {
        let (Some(ra), Some(dec), Some(jd)) = (jnum(row, "ra"), jnum(row, "dec"), jnum(row, "jd"))
        else {
            continue;
        };
        if !ra.is_finite() || !(0.0..360.0).contains(&ra) {
            continue;
        }
        if !dec.is_finite() || !(-90.0..=90.0).contains(&dec) {
            continue;
        }
        if !jd.is_finite() {
            continue;
        }
        let unix = (jd - UNIX_JD_OFFSET) * SECONDS_PER_DAY;
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        out.push((tdb, ra, COMP_RA));
        out.push((tdb, dec, COMP_DEC));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "[{\"name\":\"MS1412446136\",\"ra\":84.08379234866028,\"dec\":-70.45057767297078,\"jd\":2456935.2589814817,\"dataurl\":\"https://archive.kasi.re.kr/miris/201410/MS1412446136.fits\"},{\"name\":\"void\",\"ra\":400.0,\"dec\":-70.0,\"jd\":2456935.0}]";

    #[test]
    fn both_columns_fold_onto_the_observed_epoch() {
        let rows = parse_series(FIXTURE.as_bytes()).expect("the array parses");
        assert_eq!(rows.len(), 2);
        let lsk = crate::archivar::embedded_lsk().expect("the embedded naif0012 table stands");
        let unix = (2456935.2589814817 - UNIX_JD_OFFSET) * SECONDS_PER_DAY;
        let tdb = lsk
            .unix_to_tdb(unix)
            .expect("the observation epoch folds onto TDB");
        assert_eq!(rows[0], (tdb, 84.08379234866028, COMP_RA));
        assert_eq!(rows[1], (tdb, -70.45057767297078, COMP_DEC));
    }

    #[test]
    fn an_implausible_direction_stays_absent() {
        let rows = parse_series(FIXTURE.as_bytes()).unwrap();
        assert!(!rows.iter().any(|(_, v, c)| *c == COMP_RA && *v == 400.0));
    }

    #[test]
    fn an_empty_array_is_a_held_state_and_a_foreign_body_is_void() {
        assert_eq!(parse_series(b"[]"), Some(Vec::new()));
        assert!(parse_series(b"not json").is_none());
        assert!(parse_series(b"{\"ra\":1.0}").is_none());
    }

    #[test]
    fn declared_columns_carry_the_read_site_names_and_no_default_field() {
        let fields = declared_fields(604800.0);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, "kasi_ra_deg");
        assert_eq!(fields[1].name, "kasi_dec_deg");
        assert!(fields.iter().all(|f| f.unit == "deg"));
    }
}
