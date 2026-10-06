use crate::archivar::json::{JsonVal, jnum, jstr, parse_json};
use crate::archivar::{FieldConfig, LeapSeconds, force_id_of, kernel_id_for_force};

pub const FINK_RA: u32 = 0;
pub const FINK_DEC: u32 = 1;
pub const FINK_PSF_FLUX: u32 = 2;
pub const FINK_PSF_FLUX_ERR: u32 = 3;
pub const FINK_AP_FLUX: u32 = 4;
pub const FINK_SCIENCE_FLUX: u32 = 5;
pub const FINK_SNR: u32 = 6;
pub const FINK_EXTENDEDNESS: u32 = 7;
pub const FINK_RELIABILITY: u32 = 8;

const COLUMNS: &[(u32, &str, &str, &str)] = &[
    (FINK_RA, "fink_ra_deg", "r:ra", "deg"),
    (FINK_DEC, "fink_dec_deg", "r:dec", "deg"),
    (FINK_PSF_FLUX, "fink_psf_flux_njy", "r:psfFlux", "nJy"),
    (
        FINK_PSF_FLUX_ERR,
        "fink_psf_flux_err_njy",
        "r:psfFluxErr",
        "nJy",
    ),
    (FINK_AP_FLUX, "fink_ap_flux_njy", "r:apFlux", "nJy"),
    (
        FINK_SCIENCE_FLUX,
        "fink_science_flux_njy",
        "r:scienceFlux",
        "nJy",
    ),
    (FINK_SNR, "fink_snr", "r:snr", "1"),
    (
        FINK_EXTENDEDNESS,
        "fink_extendedness",
        "r:extendedness",
        "1",
    ),
    (FINK_RELIABILITY, "fink_reliability", "r:reliability", "1"),
];

const MJD_UNIX_EPOCH: f64 = 40587.0;
const SECS_PER_DAY: f64 = 86400.0;

#[derive(Clone, Debug, PartialEq)]
pub struct FinkSource {
    pub midpoint_mjd_tai: Option<f64>,
    pub band: Option<String>,
    pub ra_deg: Option<f64>,
    pub dec_deg: Option<f64>,
    pub psf_flux: Option<f64>,
    pub psf_flux_err: Option<f64>,
    pub ap_flux: Option<f64>,
    pub science_flux: Option<f64>,
    pub snr: Option<f64>,
    pub extendedness: Option<f64>,
    pub reliability: Option<f64>,
    pub is_dipole: Option<bool>,
}

fn num(row: &JsonVal, key: &str) -> Option<f64> {
    match jnum(row, key) {
        Some(v) if v.is_finite() => Some(v),
        _ => None,
    }
}

fn text(row: &JsonVal, key: &str) -> Option<String> {
    match jstr(row, key) {
        Some(s) if !s.is_empty() => Some(s),
        _ => None,
    }
}

fn flag(row: &JsonVal, key: &str) -> Option<bool> {
    match row {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Bool(b)) => Some(*b),
            _ => None,
        },
        _ => None,
    }
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
    for (comp, name, _, unit) in COLUMNS {
        if component_name(*comp).is_none() {
            continue;
        }
        out.push(FieldConfig {
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
            aperture: crate::archivar::Aperture::None,
        });
    }
    out
}

fn plausible(comp: u32, v: f64) -> bool {
    if !v.is_finite() {
        return false;
    }
    match comp {
        FINK_RA => (0.0..360.0).contains(&v),
        FINK_DEC => (-90.0..=90.0).contains(&v),
        FINK_EXTENDEDNESS | FINK_RELIABILITY => (0.0..=1.0).contains(&v),
        _ => true,
    }
}

fn midpoint_tdb(mjd_tai: f64, lsk: &LeapSeconds) -> Option<f64> {
    let unix_tai = (mjd_tai - MJD_UNIX_EPOCH) * SECS_PER_DAY;
    let leap = lsk.leap_at(unix_tai)?;
    lsk.unix_to_tdb(unix_tai - leap)
}

pub fn parse_sources(bytes: &[u8]) -> Option<Vec<FinkSource>> {
    let body = std::str::from_utf8(bytes).ok()?;
    let JsonVal::Arr(rows) = parse_json(body)? else {
        return None;
    };
    let mut out = Vec::with_capacity(rows.len());
    for row in &rows {
        let JsonVal::Obj(_) = row else {
            return None;
        };
        out.push(FinkSource {
            midpoint_mjd_tai: num(row, "r:midpointMjdTai"),
            band: text(row, "r:band"),
            ra_deg: num(row, "r:ra"),
            dec_deg: num(row, "r:dec"),
            psf_flux: num(row, "r:psfFlux"),
            psf_flux_err: num(row, "r:psfFluxErr"),
            ap_flux: num(row, "r:apFlux"),
            science_flux: num(row, "r:scienceFlux"),
            snr: num(row, "r:snr"),
            extendedness: num(row, "r:extendedness"),
            reliability: num(row, "r:reliability"),
            is_dipole: flag(row, "r:isDipole"),
        });
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let sources = parse_sources(bytes)?;
    let lsk = crate::archivar::embedded_lsk()?;
    let mut out = Vec::new();
    for s in &sources {
        let Some(mjd) = s.midpoint_mjd_tai else {
            continue;
        };
        let Some(tdb) = midpoint_tdb(mjd, &lsk) else {
            continue;
        };
        let values = [
            (FINK_RA, s.ra_deg),
            (FINK_DEC, s.dec_deg),
            (FINK_PSF_FLUX, s.psf_flux),
            (FINK_PSF_FLUX_ERR, s.psf_flux_err),
            (FINK_AP_FLUX, s.ap_flux),
            (FINK_SCIENCE_FLUX, s.science_flux),
            (FINK_SNR, s.snr),
            (FINK_EXTENDEDNESS, s.extendedness),
            (FINK_RELIABILITY, s.reliability),
        ];
        for (comp, value) in values {
            if let Some(v) = value
                && plausible(comp, v)
            {
                out.push((tdb, v, comp));
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &[u8] =
        include_bytes!("../../tools/measure/src/weberin/fink_object_sources.json");

    #[test]
    fn a_detection_folds_onto_its_mjd_epoch_and_components() {
        let rows = parse_series(FIXTURE).expect("the fink array parses");
        assert_eq!(rows.len(), COLUMNS.len());
        let value = |comp: u32| rows.iter().find(|r| r.2 == comp).map(|r| r.1).unwrap();
        assert!((value(FINK_RA) - 54.4145363293).abs() < 1e-9);
        assert!((value(FINK_DEC) + 29.5128706656).abs() < 1e-9);
        assert!((value(FINK_PSF_FLUX) - 763.55914).abs() < 1e-6);
        assert!((value(FINK_PSF_FLUX_ERR) - 150.5558).abs() < 1e-6);
        assert!((value(FINK_AP_FLUX) - 467.64093).abs() < 1e-6);
        assert!((value(FINK_SCIENCE_FLUX) - 343.3149).abs() < 1e-6);
        assert!((value(FINK_SNR) - 5.016104).abs() < 1e-9);
        assert!((value(FINK_EXTENDEDNESS) - 0.21991141).abs() < 1e-9);
        assert!((value(FINK_RELIABILITY) - 0.1184054).abs() < 1e-9);
        let lsk = crate::archivar::embedded_lsk().unwrap();
        let tdb = midpoint_tdb(61058.0826902019, &lsk).unwrap();
        assert!(rows.iter().all(|r| r.0 == tdb));
    }

    #[test]
    fn the_band_and_dipole_read_from_the_detection() {
        let sources = parse_sources(FIXTURE).expect("the fink array parses");
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].band.as_deref(), Some("r"));
        assert_eq!(sources[0].is_dipole, Some(false));
    }

    #[test]
    fn a_detection_without_a_midpoint_keeps_no_epoch() {
        let body = br#"[{"r:ra":54.4,"r:psfFlux":763.5}]"#;
        let rows = parse_series(body).expect("the fink array parses");
        assert!(rows.is_empty());
    }

    #[test]
    fn an_absent_field_is_never_a_fabricated_zero() {
        let body = br#"[{"r:midpointMjdTai":61058.0826902019,"r:ra":54.4,"r:psfFlux":null}]"#;
        let rows = parse_series(body).expect("the fink array parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].2, FINK_RA);
    }

    #[test]
    fn an_empty_array_is_a_realized_empty() {
        assert_eq!(parse_series(b"[]"), Some(Vec::new()));
    }

    #[test]
    fn a_foreign_body_is_void() {
        assert!(parse_series(b"not json").is_none());
        assert!(parse_series(b"{}").is_none());
    }

    #[test]
    fn declared_columns_carry_the_read_site_names_and_units() {
        let fields = declared_fields(604800.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "fink_ra_deg");
        assert_eq!(fields[0].unit, "deg");
        assert_eq!(component_name(FINK_PSF_FLUX), Some("fink_psf_flux_njy"));
        assert_eq!(component_name(99), None);
    }
}
