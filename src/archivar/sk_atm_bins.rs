use crate::archivar::harvest_table::{self, Table};

pub const MAGIC: [u8; 4] = *b"SKAT";

const NAMES: [&str; 6] = ["bin", "logp_lo", "logp_hi", "cosz_lo", "cosz_hi", "count"];

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
    harvest_table::series_from_table(&table, Some)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        1 => Some("logp_lo"),
        2 => Some("logp_hi"),
        3 => Some("cosz_lo"),
        4 => Some("cosz_hi"),
        5 => Some("count"),
        _ => None,
    }
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
                    Some(0.0),
                    Some(2.0),
                    Some(2.4),
                    Some(-1.0),
                    Some(-0.84),
                    Some(185.0),
                ],
                vec![
                    Some(1.0),
                    Some(2.0),
                    Some(2.4),
                    Some(-0.84),
                    Some(-0.64),
                    Some(172.0),
                ],
            ],
        }
    }

    #[test]
    fn roundtrip_reads_bin_edges_and_count() {
        let bytes = harvest_table::write_bin(MAGIC, &table());
        let parsed = parse_bin(&bytes).expect("table parses");
        assert_eq!(parsed, table());

        let rows = parse_series(&bytes).expect("series parses");
        assert!(
            rows.iter()
                .any(|(t, value, comp)| *t == 0.0 && *comp == 5 && *value == 185.0)
        );
        assert!(
            rows.iter()
                .any(|(t, value, comp)| *t == 1.0 && *comp == 3 && *value == -0.84)
        );
        assert_eq!(component_name(5), Some("count"));
        assert!(component_name(0).is_none());
        assert!(component_name(6).is_none());
    }

    #[test]
    fn foreign_column_order_and_truncation_stay_unread() {
        let bytes = harvest_table::write_bin(MAGIC, &table());
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());

        let mut swapped = table();
        swapped.names.swap(1, 2);
        let bytes = harvest_table::write_bin(MAGIC, &swapped);
        assert!(parse_bin(&bytes).is_none());
    }
}
