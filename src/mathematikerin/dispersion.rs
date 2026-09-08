use crate::spectral::band_overlap;

pub struct ShelfRow {
    pub force: u8,
    pub freq: f64,
    pub bin_width: f64,
    pub v: f64,
    pub v_unc: f64,
    pub epoch: f64,
}

pub fn parse_v_freq_shelf(raw: &str) -> Vec<ShelfRow> {
    raw.lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let mut it = l.split_whitespace();
            let force = it.next()?.parse::<u8>().ok()?;
            let freq = it.next()?.parse::<f64>().ok()?;
            let bin_width = it.next()?.parse::<f64>().ok()?;
            let v = it.next()?.parse::<f64>().ok()?;
            let v_unc = it.next()?.parse::<f64>().ok()?;
            let epoch = it.next()?.parse::<f64>().ok()?;
            if !freq.is_finite() || !v.is_finite() || !v_unc.is_finite() || !epoch.is_finite() {
                return None;
            }
            Some(ShelfRow {
                force,
                freq,
                bin_width,
                v,
                v_unc,
                epoch,
            })
        })
        .collect()
}

pub fn v_at_in(rows: &[ShelfRow], force: u8, freq: f64, bin_width: f64) -> Option<f64> {
    for row in rows {
        let half = (row.bin_width * 0.5).abs();
        if row.force == force && band_overlap(freq, bin_width, row.freq - half, row.freq + half) {
            return Some(row.v);
        }
    }
    None
}

pub fn v_at(force: u8, freq: f64, bin_width: f64) -> Option<f64> {
    v_at_in(
        &parse_v_freq_shelf(include_str!("kernels/v_freq_shelf.dat")),
        force,
        freq,
        bin_width,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_shelf_carries_no_row() {
        let rows = parse_v_freq_shelf("");
        assert!(rows.is_empty());
        assert!(v_at_in(&rows, 0, 1.0e15, 0.0).is_none());
    }

    #[test]
    fn a_flat_row_answers_the_band() {
        let raw =
            "# v_freq_shelf v1 sha256:abc\n# columns\n0 9.86e15 0.0 299792458.0 1.44e7 2.4e9\n";
        let rows = parse_v_freq_shelf(raw);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].force, 0);
        assert_eq!(rows[0].v, 299792458.0);
        let hit = v_at_in(&rows, 0, 9.86e15, 0.0).unwrap();
        assert_eq!(hit, 299792458.0);
    }

    #[test]
    fn a_row_outside_the_band_does_not_answer() {
        let raw = "0 9.86e15 1.0e13 299792458.0 1.44e7 2.4e9\n";
        let rows = parse_v_freq_shelf(raw);
        assert!(v_at_in(&rows, 0, 1.0e12, 0.0).is_none());
    }

    #[test]
    fn a_different_force_does_not_answer() {
        let raw = "0 9.86e15 0.0 299792458.0 1.44e7 2.4e9\n";
        let rows = parse_v_freq_shelf(raw);
        assert!(v_at_in(&rows, 1, 9.86e15, 0.0).is_none());
    }

    #[test]
    fn foreign_bytes_drop_the_row() {
        let raw = "# h\nnot a row\n0 9.86e15 0.0 299792458.0 1.44e7 2.4e9\n";
        let rows = parse_v_freq_shelf(raw);
        assert_eq!(rows.len(), 1);
    }
}
