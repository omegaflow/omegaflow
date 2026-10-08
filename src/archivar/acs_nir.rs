use crate::archivar::{FieldConfig, force_id_of, kernel_id_for_force};

pub const RECORD_BYTES: usize = 2719;
pub const OBT_OFFSET: usize = 22;
pub const OBT_BYTES: usize = 20;
pub const SIZE_OFFSET: usize = 124;
pub const SIZE_BYTES: usize = 10;
pub const ROW_DATA_OFFSET: usize = 135;
pub const ROW_DATA_BYTES: usize = 2;
pub const ROW_DATA_COUNT: usize = 1280;
pub const OBT_TICKS_PER_SECOND: f64 = 16_777_216.0;
pub const TAU_INTEGRATION_S: f64 = 0.059_904;
pub const UNIT: &str = "count";

pub fn component_name(comp: u32) -> String {
    format!("acs_nir_bin_{comp:04}")
}

pub fn declared_fields() -> Vec<FieldConfig> {
    let Some(force) = force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(force) else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(ROW_DATA_COUNT);
    for comp in 0..ROW_DATA_COUNT as u32 {
        let name = component_name(comp);
        out.push(FieldConfig {
            key: name.clone(),
            name,
            band_id: None,
            kernel,
            force,
            tau: TAU_INTEGRATION_S,
            absorption: 0.0,
            advection: 0.0,
            unit: UNIT.to_string(),
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: crate::archivar::Aperture::None,
        });
    }
    out
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.is_empty() || !bytes.len().is_multiple_of(RECORD_BYTES) {
        return None;
    }
    let rows = bytes.len() / RECORD_BYTES;
    let mut out = Vec::with_capacity(rows * ROW_DATA_COUNT);
    for r in 0..rows {
        let rec = &bytes[r * RECORD_BYTES..(r + 1) * RECORD_BYTES];
        let size = crate::archivar::pds4::parse_cell(
            rec.get(SIZE_OFFSET..SIZE_OFFSET + SIZE_BYTES)?,
            "ASCII_NonNegative_Integer",
            None,
        )?;
        if size != ROW_DATA_COUNT as f64 {
            return None;
        }
        let Some(ticks) = crate::archivar::pds4::parse_cell(
            rec.get(OBT_OFFSET..OBT_OFFSET + OBT_BYTES)?,
            "ASCII_Integer",
            None,
        ) else {
            continue;
        };
        let t = ticks / OBT_TICKS_PER_SECOND;
        for k in 0..ROW_DATA_COUNT {
            let from = ROW_DATA_OFFSET + k * ROW_DATA_BYTES;
            let cell = rec.get(from..from + ROW_DATA_BYTES)?;
            if let Some(v) = crate::archivar::pds4::parse_cell(cell, "ASCII_Numeric_Base16", None) {
                out.push((t, v, k as u32));
            }
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LABEL: &str = include_str!("pds4_fixtures/exomars_acs_nir_ec.xml");
    const FIRST1: &[u8] = include_bytes!("pds4_fixtures/exomars_acs_nir_ec_first1.tab");
    const EC_FILE: &str = "acs_raw_sc_nir_20181027T221352-20181027T223353-4140-1-1-EC__4_0.tab";
    const FIRST_OBT_TICKS: f64 = 1_388_712_562_616_064.0;

    #[test]
    fn the_constants_are_the_measured_ec_label() {
        let meta = crate::archivar::pds4::parse_label_for_file(LABEL, Some(EC_FILE))
            .expect("label parses");
        assert_eq!(meta.record_length, Some(RECORD_BYTES));
        assert_eq!(meta.rows, Some(200));
        let obt = meta
            .columns
            .iter()
            .find(|c| c.name == "OBT")
            .expect("OBT column");
        assert_eq!(obt.start_byte, Some(OBT_OFFSET + 1));
        assert_eq!(obt.bytes, Some(OBT_BYTES));
        let size = meta
            .columns
            .iter()
            .find(|c| c.name == "SIZE")
            .expect("SIZE column");
        assert_eq!(size.start_byte, Some(SIZE_OFFSET + 1));
        let first = meta
            .columns
            .iter()
            .find(|c| c.name == "ROW_DATA_0000")
            .expect("first row-data column");
        assert_eq!(first.start_byte, Some(ROW_DATA_OFFSET + 1));
        assert_eq!(first.bytes, Some(ROW_DATA_BYTES));
        let last = meta
            .columns
            .iter()
            .find(|c| c.name == "ROW_DATA_1279")
            .expect("last row-data column");
        assert_eq!(
            last.start_byte,
            Some(ROW_DATA_OFFSET + 1 + (ROW_DATA_COUNT - 1) * ROW_DATA_BYTES)
        );
    }

    #[test]
    fn the_first_fixture_record_emits_1280_raw_bins_on_the_obt_axis() {
        let series = parse_series(FIRST1).expect("the fixture record parses");
        assert_eq!(series.len(), ROW_DATA_COUNT);
        let t = FIRST_OBT_TICKS / OBT_TICKS_PER_SECOND;
        assert!((t - 82_773_718.99).abs() < 0.01);
        assert_eq!(series[0], (t, 215.0, 0));
        assert_eq!(series[1], (t, 45.0, 1));
        assert_eq!(series[2], (t, 216.0, 2));
        assert_eq!(series[1279].0, t);
        assert_eq!(series[1279].2, 1279);
        assert!(
            series
                .iter()
                .all(|(tick, v, _)| { tick.is_finite() && *tick > 0.0 && v.is_finite() })
        );
    }

    #[test]
    fn a_corrupt_base16_cell_stays_absent_and_a_foreign_size_refuses() {
        let mut short = FIRST1.to_vec();
        short[ROW_DATA_OFFSET..ROW_DATA_OFFSET + ROW_DATA_BYTES].copy_from_slice(b"ZZ");
        let series = parse_series(&short).expect("the record still parses");
        assert_eq!(series.len(), ROW_DATA_COUNT - 1);
        assert_eq!(series[0].2, 1);

        let mut drifted = FIRST1.to_vec();
        drifted[SIZE_OFFSET..SIZE_OFFSET + SIZE_BYTES].copy_from_slice(b"0000000001");
        assert!(parse_series(&drifted).is_none());
    }

    #[test]
    fn foreign_truncated_and_empty_bytes_carry_no_series() {
        assert!(parse_series(b"").is_none());
        assert!(parse_series(b"XXXX").is_none());
        assert!(parse_series(&FIRST1[..FIRST1.len() - 1]).is_none());
    }

    #[test]
    fn declared_fields_carry_bin_names_em_count_integration_and_no_band() {
        let fields = declared_fields();
        assert_eq!(fields.len(), ROW_DATA_COUNT);
        assert_eq!(fields[0].name, "acs_nir_bin_0000");
        assert_eq!(fields[0].key, "acs_nir_bin_0000");
        assert_eq!(fields[639].name, "acs_nir_bin_0639");
        assert_eq!(fields[1279].name, "acs_nir_bin_1279");
        assert_eq!(fields[0].force, crate::force::force_id_of("em").unwrap());
        assert_eq!(
            fields[0].kernel,
            crate::archivar::extract::kernel_id_of("inverse-square").unwrap()
        );
        assert_eq!(fields[0].unit, UNIT);
        assert_eq!(fields[0].tau, TAU_INTEGRATION_S);
        assert_eq!(fields[0].freq, crate::archivar::spectral::SPECTRAL_NO_BAND);
        assert_eq!(
            fields[0].bin_width,
            crate::archivar::spectral::SPECTRAL_NO_BAND
        );
        assert_eq!(fields[0].absorption, 0.0);
        assert_eq!(fields[0].advection, 0.0);
    }
}
