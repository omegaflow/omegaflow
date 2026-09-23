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

fn v_freq_shelf_rows() -> &'static Vec<ShelfRow> {
    static ROWS: std::sync::OnceLock<Vec<ShelfRow>> = std::sync::OnceLock::new();
    ROWS.get_or_init(|| parse_v_freq_shelf(include_str!("kernels/v_freq_shelf.dat")))
}

pub fn v_at(force: u8, freq: f64, bin_width: f64) -> Option<f64> {
    v_at_in(v_freq_shelf_rows(), force, freq, bin_width)
}

pub const SHELF_GPU_ROW_FLOATS: usize = 4;

pub fn shelf_rows_for_gpu() -> Vec<f32> {
    let mut out = Vec::with_capacity(v_freq_shelf_rows().len() * SHELF_GPU_ROW_FLOATS);
    for row in v_freq_shelf_rows() {
        out.push(row.force as f32);
        out.push(row.freq as f32);
        out.push(row.bin_width as f32);
        out.push(row.v as f32);
    }
    out
}

pub fn v_at_f32(force: u8, freq: f32, bin_width: f32) -> Option<f32> {
    if !(freq > 0.0) {
        return None;
    }
    for r in shelf_rows_for_gpu().chunks_exact(SHELF_GPU_ROW_FLOATS) {
        if r[0] != force as f32 {
            continue;
        }
        let half = (bin_width * 0.5).abs();
        let band_lo = freq - half;
        let band_hi = freq + half;
        let rhalf = (r[2] * 0.5).abs();
        let rlo = r[1] - rhalf;
        let rhi = r[1] + rhalf;
        if band_hi >= rlo && band_lo <= rhi {
            return Some(r[3]);
        }
    }
    None
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

    #[test]
    fn a_shelf_covered_band_answers_the_measured_v() {
        let v = v_at(4, 0.05, 0.0166667).expect("the 0.05 Hz Rayleigh band is on the shelf");
        assert!(
            (v - 2899.8).abs() < 1e-6,
            "the shelf answers the measured 2899.8 m/s at 0.05 Hz: {v}"
        );
        let v_em = v_at(0, 9.861594e15, 0.0).expect("the 304 Å em band is on the shelf");
        assert!(
            (v_em - 2.997925e8).abs() < 1.0,
            "the em shelf row answers c within its 6 digits: {v_em}"
        );
        let spanning = v_at(4, 0.05, 0.1).expect("a wide caller band spanning the row answers");
        assert!((spanning - 2899.8).abs() < 1e-6);
    }

    #[test]
    fn an_uncovered_band_carries_no_shelf_answer() {
        assert!(
            v_at(4, 0.07, 0.0).is_none(),
            "0.07 Hz lies above the Rayleigh rows"
        );
        assert!(
            v_at(4, 0.003, 0.0).is_none(),
            "0.003 Hz lies below the Rayleigh rows"
        );
        assert!(
            v_at(0, 5.0e14, 0.0).is_none(),
            "the em rows are points on their bands"
        );
        assert!(
            v_at(2, 440.0, 0.0).is_none(),
            "acoustic carries no shelf class"
        );
    }

    #[test]
    fn the_gpu_row_serialization_carries_each_row_as_force_freq_bin_width_v() {
        let rows = parse_v_freq_shelf(include_str!("kernels/v_freq_shelf.dat"));
        let gpu = shelf_rows_for_gpu();
        assert_eq!(gpu.len(), rows.len() * SHELF_GPU_ROW_FLOATS);
        let first_rayleigh = rows
            .iter()
            .find(|r| r.force == 4)
            .expect("Rayleigh rows exist");
        let mut found = None;
        for r in gpu.chunks_exact(SHELF_GPU_ROW_FLOATS) {
            if r[0] == first_rayleigh.force as f32
                && (r[1] as f64 - first_rayleigh.freq).abs() < 1e-6
            {
                found = Some(r.to_vec());
            }
        }
        let r = found.expect("the first Rayleigh row is in the gpu rows");
        assert_eq!(r[0], 4.0);
        assert!((r[1] - 0.05).abs() < 1e-6);
        assert!((r[2] - 0.0166667).abs() < 1e-7);
        assert!((r[3] - 2899.8).abs() < 1e-3);
    }

    #[test]
    fn the_f32_lookup_stays_within_tolerance_of_the_f64_reference() {
        let rows = parse_v_freq_shelf(include_str!("kernels/v_freq_shelf.dat"));
        assert!(rows.len() >= 12, "the shelf carries em and Rayleigh rows");
        for row in &rows {
            let gpu = v_at_f32(row.force, row.freq as f32, row.bin_width as f32)
                .expect("the f32 mirror answers its own row");
            let cpu = v_at(row.force, row.freq, row.bin_width).expect("the f64 reference answers");
            let rel = ((gpu as f64 - cpu) / cpu).abs();
            assert!(
                rel < 1e-4,
                "f32/f64 parity: force {} at {} Hz — gpu {gpu} cpu {cpu} rel {rel}",
                row.force,
                row.freq
            );
        }
        assert!(
            v_at_f32(4, 0.07, 0.0).is_none() && v_at(4, 0.07, 0.0).is_none(),
            "an uncovered band stays uncovered in both precisions"
        );
    }
}
