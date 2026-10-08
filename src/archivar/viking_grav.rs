use crate::archivar::pds3_table::parse_table;
use crate::archivar::{FieldConfig, force_id_of, kernel_id_for_force};

pub const COMP_LATITUDE: u32 = 0;
pub const COMP_LONGITUDE: u32 = 1;
pub const COMP_ACCELERATION: u32 = 2;
pub const COMP_ALTITUDE: u32 = 3;

const VIKING_GRAV_DECLARED_CATALOG_EPOCH_ISO: &str = "1976-01-01T00:00:00";

const COLUMNS: &[(u32, &str, &str, &str)] = &[
    (COMP_LATITUDE, "LATITUDE", "viking_grav_latitude_deg", "deg"),
    (
        COMP_LONGITUDE,
        "LONGITUDE",
        "viking_grav_longitude_deg",
        "deg",
    ),
    (
        COMP_ACCELERATION,
        "ACCELERATION",
        "viking_grav_acceleration_mm_s2",
        "mm/s2",
    ),
    (COMP_ALTITUDE, "ALTITUDE", "viking_grav_altitude_km", "km"),
];

fn comp_of_column(name: &str) -> Option<u32> {
    COLUMNS
        .iter()
        .find(|c| c.1.eq_ignore_ascii_case(name))
        .map(|c| c.0)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.2)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(force) else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(COLUMNS.len());
    for (comp, _, name, unit) in COLUMNS {
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
    let table = parse_table(bytes)?;
    let lsk = crate::archivar::embedded_lsk()?;
    let tdb = crate::archivar::parse_iso_tdb(VIKING_GRAV_DECLARED_CATALOG_EPOCH_ISO, &lsk)?;
    let mut out = Vec::new();
    for row in &table.rows {
        for (ci, c) in table.columns.iter().enumerate() {
            let Some(comp) = comp_of_column(&c.name) else {
                continue;
            };
            if let Some(v) = row.values.get(ci).copied().flatten() {
                out.push((tdb, v, comp));
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::pds3_table::{Pds3Table, TableColumn, TableRow};

    fn column(name: &str, unit: &str, start_byte: usize, bytes: usize) -> TableColumn {
        TableColumn {
            name: name.to_string(),
            unit: Some(unit.to_string()),
            data_type: Some("REAL".to_string()),
            missing_constant: None,
            sampling_name: String::new(),
            sampling_unit: String::new(),
            sampling_min: None,
            sampling_max: None,
            start_byte,
            bytes,
        }
    }

    fn fixture() -> Pds3Table {
        Pds3Table {
            columns: vec![
                column("LATITUDE", "DEGREES", 1, 8),
                column("LONGITUDE", "DEGREES", 11, 9),
                column("ACCELERATION", "MM/(SEC^2)", 22, 7),
                column("ALTITUDE", "KILOMETERS", 31, 9),
            ],
            rows: vec![
                TableRow {
                    values: vec![Some(12.5), Some(310.25), Some(-140.5), Some(1500.0)],
                },
                TableRow {
                    values: vec![Some(-3.0), Some(-180.0), None, Some(200.0)],
                },
            ],
        }
    }

    #[test]
    fn each_present_cell_folds_onto_the_declared_catalog_epoch() {
        let bin = crate::archivar::pds3_table::pack(&fixture());
        let series = parse_series(&bin).expect("the packed table reads back");
        let lsk = crate::archivar::embedded_lsk().unwrap();
        let tdb =
            crate::archivar::parse_iso_tdb(VIKING_GRAV_DECLARED_CATALOG_EPOCH_ISO, &lsk).unwrap();
        assert_eq!(series.len(), 7);
        assert_eq!(series[0], (tdb, 12.5, COMP_LATITUDE));
        assert_eq!(series[1], (tdb, 310.25, COMP_LONGITUDE));
        assert_eq!(series[2], (tdb, -140.5, COMP_ACCELERATION));
        assert_eq!(series[3], (tdb, 1500.0, COMP_ALTITUDE));
        assert_eq!(series[4], (tdb, -3.0, COMP_LATITUDE));
        assert_eq!(series[5], (tdb, -180.0, COMP_LONGITUDE));
        assert_eq!(series[6], (tdb, 200.0, COMP_ALTITUDE));
    }

    #[test]
    fn an_absent_cell_stays_absent() {
        let bin = crate::archivar::pds3_table::pack(&fixture());
        let series = parse_series(&bin).unwrap();
        assert_eq!(
            series
                .iter()
                .filter(|(_, _, c)| *c == COMP_ACCELERATION)
                .count(),
            1
        );
    }

    #[test]
    fn a_foreign_asset_is_void() {
        assert!(parse_series(b"XXXX").is_none());
    }

    #[test]
    fn declared_columns_carry_the_read_site_names_and_units() {
        let fields = declared_fields(604800.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "viking_grav_latitude_deg");
        assert_eq!(fields[2].unit, "mm/s2");
        assert_eq!(
            component_name(COMP_ACCELERATION),
            Some("viking_grav_acceleration_mm_s2")
        );
        assert_eq!(component_name(9), None);
    }
}
