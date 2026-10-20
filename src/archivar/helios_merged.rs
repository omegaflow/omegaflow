use crate::archivar::harvest_table::{self, Table};

pub const MAGIC: [u8; 4] = *b"HELI";

const NAMES: [&str; 32] = [
    "mjd",
    "year",
    "doy",
    "hour",
    "cr",
    "r",
    "lat",
    "lon",
    "ess",
    "bx",
    "by",
    "bz",
    "br",
    "bt",
    "bn",
    "bmag",
    "speed",
    "elev",
    "azim",
    "density",
    "temp",
    "e6_4_13",
    "e6_13_27",
    "e6_27_37",
    "e6_37_51",
    "e7_3_4_6",
    "e7_6_11",
    "e7_11_21",
    "e7_24_28",
    "e7_32_46",
    "e7_46_57",
    "e7_135_206",
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

    fn row(mjd: f64, bmag: Option<f64>, speed: Option<f64>) -> Vec<Option<f64>> {
        let mut row = vec![None; NAMES.len()];
        row[0] = Some(mjd);
        row[1] = Some(1974.0);
        row[2] = Some(348.0);
        row[3] = Some(0.0);
        row[15] = bmag;
        row[16] = speed;
        row
    }

    fn table() -> Table {
        Table {
            names: NAMES.iter().map(|n| n.to_string()).collect(),
            rows: vec![
                row(41_000.5, Some(4.55), Some(574.9)),
                row(41_001.5, Some(5.1), None),
            ],
        }
    }

    #[test]
    fn roundtrip_reads_mjd_epoch_bmag_and_speed() {
        let bytes = harvest_table::write_bin(MAGIC, &table());
        let parsed = parse_bin(&bytes).expect("table parses");
        assert_eq!(parsed, table());

        let rows = parse_series(&bytes).expect("series parses");
        let lsk = crate::archivar::embedded_lsk().unwrap();
        let expected = crate::maxi::mjd_to_tdb(41_000.5, &lsk).unwrap();
        assert_eq!(rows[0], (expected, 1974.0, 1));
        assert!(
            rows.iter()
                .any(|(_, value, comp)| *comp == 15 && *value == 4.55)
        );
        assert!(
            rows.iter()
                .any(|(_, value, comp)| *comp == 16 && *value == 574.9)
        );
        assert_eq!(component_name(15), Some("bmag"));
        assert_eq!(component_name(20), Some("temp"));
        assert!(component_name(32).is_none());
    }

    #[test]
    fn foreign_column_order_and_truncation_stay_unread() {
        let bytes = harvest_table::write_bin(MAGIC, &table());
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());

        let mut swapped = table();
        swapped.names.swap(15, 16);
        let bytes = harvest_table::write_bin(MAGIC, &swapped);
        assert!(parse_bin(&bytes).is_none());
    }
}
