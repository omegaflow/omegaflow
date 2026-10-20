use crate::archivar::harvest_table::{self, Table};

pub const MAGIC: [u8; 4] = *b"IMP8";

const NAMES: [&str; 24] = [
    "mjd",
    "year",
    "doy",
    "hour",
    "npoints",
    "x_gse_re",
    "y_gse_re",
    "z_gse_re",
    "y_gsm_re",
    "z_gsm_re",
    "b",
    "b_vec",
    "b_lat_deg",
    "b_lon_deg",
    "bx",
    "by",
    "bz",
    "by_gsm",
    "bz_gsm",
    "sigma_b",
    "sigma_b_vec",
    "sigma_bx",
    "sigma_by",
    "sigma_bz",
];

pub fn parse_bin(bytes: &[u8]) -> Option<Table> {
    let table = harvest_table::parse_bin(MAGIC, bytes)?;
    let names: Vec<&str> = table.names.iter().map(String::as_str).collect();
    if names != NAMES {
        return None;
    }
    Some(table)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let table = parse_bin(bytes)?;
    let lsk = crate::archivar::embedded_lsk()?;
    harvest_table::series_from_table(&table, |mjd| crate::maxi::mjd_to_tdb(mjd, &lsk))
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    NAMES.get(comp as usize).copied()
}

pub fn names() -> Vec<String> {
    NAMES.iter().map(|name| name.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> Table {
        Table {
            names: NAMES.iter().map(|n| n.to_string()).collect(),
            rows: vec![
                vec![
                    Some(42_000.5),
                    Some(1974.0),
                    Some(1.0),
                    Some(0.0),
                    Some(41.0),
                    Some(19.4),
                    Some(1.9),
                    Some(16.0),
                    Some(-2.1),
                    Some(16.0),
                    Some(4.4),
                    Some(2.9),
                    Some(47.2),
                    Some(289.8),
                    Some(0.7),
                    Some(-1.9),
                    Some(2.1),
                    Some(-2.3),
                    Some(1.6),
                    Some(0.6),
                    Some(3.0),
                    Some(2.0),
                    Some(1.2),
                    Some(1.8),
                ],
                vec![
                    Some(42_001.5),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(5.5),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            ],
        }
    }

    #[test]
    fn roundtrip_reads_mjd_epoch_and_magnitude() {
        let bytes = harvest_table::write_bin(MAGIC, &table());
        let parsed = parse_bin(&bytes).expect("table parses");
        assert_eq!(parsed, table());

        let rows = parse_series(&bytes).expect("series parses");
        let lsk = crate::archivar::embedded_lsk().unwrap();
        let expected = crate::maxi::mjd_to_tdb(42_000.5, &lsk).unwrap();
        assert_eq!(rows[0], (expected, 1974.0, 1));
        let b = rows
            .iter()
            .find(|(_, _, comp)| *comp == 10)
            .expect("magnitude survives");
        assert!((b.1 - 4.4).abs() < 1e-12);
        assert_eq!(component_name(10), Some("b"));
        assert_eq!(component_name(19), Some("sigma_b"));
        assert!(component_name(24).is_none());
    }

    #[test]
    fn foreign_column_order_and_truncation_stay_unread() {
        let bytes = harvest_table::write_bin(MAGIC, &table());
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());

        let mut swapped = table();
        swapped.names.swap(0, 1);
        let bytes = harvest_table::write_bin(MAGIC, &swapped);
        assert!(parse_bin(&bytes).is_none());
    }
}
