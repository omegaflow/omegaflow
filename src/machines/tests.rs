#[cfg(test)]
mod enso_parser_tests {
    use crate::archivar::embedded_lsk;
    use crate::machines::*;

    fn series_of<'a>(
        parsed: &'a [(EnsoSeries, Vec<(f64, f64)>)],
        s: EnsoSeries,
    ) -> &'a [(f64, f64)] {
        parsed
            .iter()
            .find(|(k, _)| *k == s)
            .map(|(_, v)| v.as_slice())
            .unwrap_or(&[])
    }

    const FIXTURE: &str = "#YY  MM DD hh mm WDIR WSPD GST  WVHT   DPD   APD MWD   PRES  ATMP  WTMP  DEWP  VIS PTDY  TIDE\n#yr  mo dy hr mn degT m/s  m/s     m   sec   sec degT   hPa  degC  degC  degC  nmi  hPa    ft\n2026 08 21 15 10 100  3.0  4.0    MM    MM    MM  MM 1013.5  26.0  26.7  23.1   MM   MM    MM\n2026 08 21 15 00 100   MM  4.0    MM    MM    MM  MM 1013.3  26.0  26.7  22.9   MM -0.7    MM\n2026 08 21 14 50 100  4.0  5.0   1.5     8   6.2 100 1013.2  26.1    MM  23.0   MM   MM    MM\n";

    #[test]
    fn enso_ndbc_parse_splits_wind_and_sst_with_mm_skips() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let parsed = enso_ndbc_parse(FIXTURE, &lsk).expect("fixture parses");
        let wind = series_of(&parsed, EnsoSeries::Wspd);
        let sst = series_of(&parsed, EnsoSeries::Wtmp);
        assert_eq!(wind.len(), 2);
        assert_eq!(sst.len(), 2);
        assert_eq!(wind[0].1, 3.0);
        assert_eq!(wind[1].1, 4.0);
        assert_eq!(sst[0].1, 26.7);
        assert_eq!(sst[1].1, 26.7);
        assert_eq!(wind[0].0 - wind[1].0, 1200.0);
        assert_eq!(sst[0].0 - sst[1].0, 600.0);
    }

    #[test]
    fn enso_ndbc_parse_skips_stdmet_sentinel_values() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY  MM DD hh mm WDIR WSPD GST  WVHT   DPD   APD MWD   PRES  ATMP  WTMP  DEWP  VIS  TIDE\n#yr  mo dy hr mn degT m/s  m/s     m   sec   sec degT   hPa  degC  degC  degC  nmi  hPa    ft\n2026 01 01 00 00 115  2.6  3.9 99.00 99.00 99.00 999 1019.3  25.2  25.8  21.3 99.0 99.00\n2026 01 01 00 10 112 99.0  4.0  1.81 11.43  8.43 301 1019.3  25.2 999.0  21.3 99.0 99.00\n2026 01 01 00 20 118  3.2  4.5 99.00 99.00 99.00 999 1019.3  25.2  25.9  21.4 99.0 99.00\n";
        let parsed = enso_ndbc_parse(body, &lsk).expect("stdmet parses");
        let wind = series_of(&parsed, EnsoSeries::Wspd);
        let sst = series_of(&parsed, EnsoSeries::Wtmp);
        assert_eq!(wind.len(), 2);
        assert_eq!(sst.len(), 2);
        assert_eq!(wind[0].1, 2.6);
        assert_eq!(wind[1].1, 3.2);
        assert_eq!(sst[0].1, 25.8);
        assert_eq!(sst[1].1, 25.9);
    }

    #[test]
    fn enso_ndbc_parse_carries_all_present_columns_with_gates() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY  MM DD hh mm WDIR WSPD GST  WVHT   DPD   APD MWD   PRES  ATMP  WTMP  DEWP  VIS PTDY  TIDE\n2026 08 21 15 00 100  3.0  4.5   1.5   8.0   6.2 100 1013.2  26.1  25.8  23.0 10.0 -0.7  0.55\n";
        let parsed = enso_ndbc_parse(body, &lsk).expect("matrix parses");
        assert_eq!(series_of(&parsed, EnsoSeries::Wspd)[0].1, 3.0);
        assert_eq!(series_of(&parsed, EnsoSeries::Gst)[0].1, 4.5);
        assert_eq!(series_of(&parsed, EnsoSeries::Wvht)[0].1, 1.5);
        assert_eq!(series_of(&parsed, EnsoSeries::Dpd)[0].1, 8.0);
        assert_eq!(series_of(&parsed, EnsoSeries::Apd)[0].1, 6.2);
        assert_eq!(series_of(&parsed, EnsoSeries::Pres)[0].1, 1013.2);
        assert_eq!(series_of(&parsed, EnsoSeries::Ptdy)[0].1, -0.7);
        assert_eq!(series_of(&parsed, EnsoSeries::Atmp)[0].1, 26.1);
        assert_eq!(series_of(&parsed, EnsoSeries::Wtmp)[0].1, 25.8);
        assert_eq!(series_of(&parsed, EnsoSeries::Dewp)[0].1, 23.0);
        assert_eq!(series_of(&parsed, EnsoSeries::Vis)[0].1, 10.0);
        assert_eq!(series_of(&parsed, EnsoSeries::Tide)[0].1, 0.55);
    }

    #[test]
    fn enso_ndbc_parse_missing_column_leaves_series_absent() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY  MM DD hh mm WDIR WSPD GST\n2026 08 21 15 10 100  3.0  4.0\n";
        let parsed = enso_ndbc_parse(body, &lsk).expect("wspd parses");
        assert_eq!(series_of(&parsed, EnsoSeries::Wspd).len(), 1);
        assert!(series_of(&parsed, EnsoSeries::Wtmp).is_empty());
        assert!(series_of(&parsed, EnsoSeries::Pres).is_empty());
    }

    #[test]
    fn enso_ndbc_parse_direction_columns_flow_as_sin_cos() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY  MM DD hh mm WDIR WSPD GST WVHT DPD APD MWD PRES ATMP WTMP DEWP VIS TIDE\n2026 08 21 15 10   0  3.0 4.0  1.5 8.0 6.2  90 1013.2 26.1 25.8 23.0 10.0 0.5\n2026 08 21 15 00  90  3.0 4.0  1.5 8.0 6.2 999 1013.2 26.1 25.8 23.0 10.0 0.5\n";
        let parsed = enso_ndbc_parse(body, &lsk).expect("directions parse");
        let sin = series_of(&parsed, EnsoSeries::WdirSin);
        let cos = series_of(&parsed, EnsoSeries::WdirCos);
        let msin = series_of(&parsed, EnsoSeries::MwdSin);
        assert_eq!(sin.len(), 2);
        assert_eq!(cos.len(), 2);
        assert_eq!(sin[0].1, 0.0);
        assert_eq!(cos[0].1, 1.0);
        assert!((sin[1].1 - 1.0).abs() < 1e-9);
        assert!(cos[1].1.abs() < 1e-9);
        assert_eq!(msin.len(), 1);
        assert!((msin[0].1 - 1.0).abs() < 1e-9);
    }

    #[test]
    fn enso_ndbc_parse_rain_column_when_present() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY MM DD hh mm WDIR WSPD GST WVHT DPD APD MWD PRES ATMP WTMP DEWP VIS TIDE RAIN\n2026 08 21 15 10 100 3.0 4.0 1.5 8.0 6.2 100 1013.2 26.1 25.8 23.0 10.0 0.5 0.12\n2026 08 21 15 00 100 3.0 4.0 1.5 8.0 6.2 100 1013.2 26.1 25.8 23.0 10.0 0.5 99.0\n";
        let parsed = enso_ndbc_parse(body, &lsk).expect("rain parses");
        let rain = series_of(&parsed, EnsoSeries::Rain);
        assert_eq!(rain.len(), 1);
        assert_eq!(rain[0].1, 0.12);
    }

    #[test]
    fn enso_ndbc_parse_all_missing_rows_is_none() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY  MM DD hh mm WDIR WSPD GST  WVHT   DPD   APD MWD   PRES  ATMP  WTMP  DEWP  VIS PTDY  TIDE\n2026 08 21 15 10  MM   MM   MM    MM    MM    MM  MM    MM    MM    MM    MM   MM   MM    MM\n";
        assert!(enso_ndbc_parse(body, &lsk).is_none());
    }
}

// ---- Verbraucher-Seite: die Maschinen (Rückbau Atom 2) ----
// Solar und ENSO halten je ihren Ring, ihren Rotor, ihre TE-Puffer und
// ihre eigene GPU-Pipeline. NativeApp hält `solar:`/`enso:` und ruft
// `tick(ring_gen)` — der ring_gen-Parameter hält die Surrogat-Seeds
// byte-identisch zur alten Inline-Fassung.

#[cfg(test)]
mod machine_tests {
    use crate::machines::*;

    #[test]
    fn enso_shift_pair_pairs_driver_bins_with_target_offset() {
        let driver = vec![(10u64, 3.0f32), (14, 5.0), (18, 7.0)];
        let target = vec![(14u64, 1.0f32), (18, 2.0), (22, 4.0)];
        let (ys, xs) = enso_shift_pair(&driver, &target, 4);
        assert_eq!(ys, vec![3.0, 5.0, 7.0]);
        assert_eq!(xs, vec![1.0, 2.0, 4.0]);
    }

    #[test]
    fn enso_shift_pair_negative_shift_extends_backward() {
        let driver = vec![(10u64, 3.0f32), (14, 5.0)];
        let target = vec![(10u64, 1.0f32), (14, 2.0)];
        let (ys, xs) = enso_shift_pair(&driver, &target, -4);
        assert_eq!(ys, vec![5.0]);
        assert_eq!(xs, vec![1.0]);
    }

    #[test]
    fn enso_shift_pair_skips_bins_without_driver_match() {
        let driver = vec![(14u64, 5.0f32)];
        let target = vec![(10u64, 1.0f32), (14, 2.0), (18, 4.0)];
        let (ys, xs) = enso_shift_pair(&driver, &target, 0);
        assert_eq!(ys, vec![5.0]);
        assert_eq!(xs, vec![2.0]);
    }

    #[test]
    fn enso_cell_desc_covers_scales_dirs_shifts() {
        assert_eq!(enso_cell_desc(0), (0, 0, -30));
        assert_eq!(enso_cell_desc(60), (0, 0, 30));
        assert_eq!(enso_cell_desc(61), (0, 1, -30));
        assert_eq!(enso_cell_desc(122), (1, 0, -30));
        assert_eq!(enso_cell_desc(ENSO_CELLS_PER_ROUND - 1), (2, 1, 30));
    }
}
