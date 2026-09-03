#[cfg(test)]
mod matrix_machine_tests {
    use crate::mathematikerin::machines::*;

    #[test]
    fn shift_pair_pairs_driver_bins_with_target_offset() {
        let driver = vec![(21600.0, 3.0f32), (108000.0, 5.0), (194400.0, 7.0)];
        let target = vec![(108000.0, 1.0f32), (194400.0, 2.0), (280800.0, 4.0)];
        let (ys, xs) = shift_pair(&driver, &target, 86400.0);
        assert_eq!(ys, vec![3.0, 5.0, 7.0]);
        assert_eq!(xs, vec![1.0, 2.0, 4.0]);
    }

    #[test]
    fn shift_pair_negative_shift_extends_backward() {
        let driver = vec![(21600.0, 3.0f32), (108000.0, 5.0)];
        let target = vec![(21600.0, 1.0f32), (108000.0, 2.0)];
        let (ys, xs) = shift_pair(&driver, &target, -86400.0);
        assert_eq!(ys, vec![5.0]);
        assert_eq!(xs, vec![1.0]);
    }

    #[test]
    fn shift_pair_skips_bins_without_driver_match() {
        let driver = vec![(108000.0, 5.0f32)];
        let target = vec![(21600.0, 1.0f32), (108000.0, 2.0), (194400.0, 4.0)];
        let (ys, xs) = shift_pair(&driver, &target, 0.0);
        assert_eq!(ys, vec![5.0]);
        assert_eq!(xs, vec![2.0]);
    }

    #[test]
    fn matrix_cell_desc_covers_scales_dirs_shifts() {
        assert_eq!(matrix_cell_desc(0), (0, 0, -30));
        assert_eq!(matrix_cell_desc(60), (0, 0, 30));
        assert_eq!(matrix_cell_desc(61), (0, 1, -30));
        assert_eq!(matrix_cell_desc(122), (1, 0, -30));
        assert_eq!(matrix_cell_desc(MATRIX_CELLS_PER_ROUND - 1), (2, 1, 30));
    }

    #[test]
    fn matrix_pair_enumerates_combinations_in_order() {
        let present = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let pairs = all_pairs(&present);
        assert_eq!(pairs[0], ("a".to_string(), "b".to_string()));
        assert_eq!(pairs[3], ("b".to_string(), "c".to_string()));
        assert_eq!(pairs[5], ("c".to_string(), "d".to_string()));
        assert_eq!(pairs.len(), 6);
    }

    #[test]
    fn insert_bin_replaces_equal_epochs_and_trims_oldest() {
        let mut ring: Vec<(f64, f32)> = Vec::new();
        for i in 0..MATRIX_RING_MAX + 2 {
            insert_bin(&mut ring, (i as f64) * 21600.0, i as f32);
        }
        assert_eq!(ring.len(), MATRIX_RING_MAX);
        assert_eq!(ring[0].0, 2.0 * 21600.0);
        insert_bin(&mut ring, 5.0 * 21600.0, 99.0);
        assert_eq!(ring[3], (5.0 * 21600.0, 99.0));
    }
}

#[cfg(test)]
mod rows_series_tests {
    use crate::archivar::{embedded_lsk, extract, load_sources_from, ExtractResult};

    const BUOY_BLOCK: &str = "
url https://www.ndbc.noaa.gov/data/realtime2/41001.txt
ttl 86400
on earth 34.7 -72.4 0
rows
epoch YY MM DD hh mm
bin 21600
prefix ndbc_41001
gate WSPD 0 99
gate WTMP -100 900
gate WVHT 0 99
field WSPD wspd_m_s inverse-square advective m/s 21600 0.0 0.0
field WTMP wtmp_c inverse-square thermal C 21600 0.0 0.0
field WVHT wvht_m inverse-square acoustic m 21600 0.0 0.0
fold sin_deg WDIR WDIR inverse-square advective 1 21600
";

    const NDBC_BODY: &str = "#YY  MM DD hh mm WDIR WSPD GST  WVHT  DPD APD MWD PRES ATMP WTMP DEWP VIS PTDY TIDE\n#yr  mo dy hr mn degT m/s  m/s    m   sec sec degT hPa degC degC degC nmi hPa   ft\n2026 08 21 18 50 100  3.0  4.0  1.5   8.0 6.2 100 1013.2 26.1 26.7 23.1 10.0 -0.7  0.5\n2026 08 21 15 00 100   MM  4.0   99    MM  MM  MM 1013.3 26.0 26.7 22.9  MM -0.7   MM\n2026 08 21 12 10 100  5.0  6.0   99    MM  MM 999 1013.0 26.2 26.5 22.8  MM -0.8   MM\n";

    #[test]
    fn rows_series_bins_epochs_skips_sentinels_and_prefixes_names() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let sources = load_sources_from(BUOY_BLOCK);
        assert_eq!(sources.len(), 1);
        let ExtractResult::Measurements(channels) = extract(&sources[0], NDBC_BODY, 0.0, &lsk)
        else {
            panic!("rows block must measure");
        };
        let wspd: Vec<(f64, f64)> = channels
            .iter()
            .filter(|(c, _)| c.name == "ndbc_41001_wspd_m_s")
            .map(|(c, _)| (c.epoch, c.value))
            .collect();
        let wtmp: Vec<(f64, f64)> = channels
            .iter()
            .filter(|(c, _)| c.name == "ndbc_41001_wtmp_c")
            .map(|(c, _)| (c.epoch, c.value))
            .collect();
        let wdir_sin: Vec<f64> = channels
            .iter()
            .filter(|(c, _)| c.name.contains("wdir") || c.name.contains("WDIR"))
            .filter(|(c, _)| c.name.contains("sin"))
            .map(|(c, _)| c.value)
            .collect();
        assert_eq!(wspd.len(), 2);
        let mut wspd_vals: Vec<f64> = wspd.iter().map(|(_, v)| *v).collect();
        wspd_vals.sort_by(|a, b| a.total_cmp(b));
        assert_eq!(wspd_vals, vec![3.0, 5.0]);
        assert_eq!(wtmp.len(), 2);
        let wtmp_sum: f64 = wtmp.iter().map(|(_, v)| v).sum();
        assert!((wtmp_sum - (26.7 + 26.6)).abs() < 1e-9);
        assert_eq!(wdir_sin.len(), 2);
        assert!((wdir_sin[0] - 100f64.to_radians().sin()).abs() < 1e-9);
        let bin0 = wspd[0].0 / 21600.0;
        assert!((bin0 - bin0.floor()).abs() < 1e-9);
    }

    #[test]
    fn rows_series_gates_reject_numeric_sentinels() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let sources = load_sources_from(BUOY_BLOCK);
        let ExtractResult::Measurements(channels) = extract(&sources[0], NDBC_BODY, 0.0, &lsk)
        else {
            panic!("rows block must measure");
        };
        let wvht: Vec<f64> = channels
            .iter()
            .filter(|(c, _)| c.name.contains("wvht"))
            .map(|(c, _)| c.value)
            .collect();
        assert_eq!(wvht, vec![1.5]);
    }
}

#[cfg(test)]
mod matrix_record_tests {
    use crate::archivar::{Channel, FieldConfig, Position};
    use crate::mathematikerin::machines::*;

    fn mk_channel(name: &str, epoch: f64, val: f64) -> (Channel, FieldConfig) {
        (
            Channel {
                z: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                epoch,
                position: Position::Surface {
                    body_name: "earth".to_string(),
                    lat: 34.7,
                    lon: -72.4,
                    alt: 0.0,
                },
                name: name.to_string(),
                value: val,
            },
            FieldConfig {
                key: "WSPD".to_string(),
                name: name.to_string(),
                kernel: 0,
                force: 7,
                tau: 21600.0,
                absorption: 0.0,
                advection: 0.0,
                unit: "m/s".to_string(),
                fold: None,
            },
        )
    }

    fn frame_earth() -> crate::archivar::Frame {
        crate::archivar::Frame::Surface {
            body_name: "earth".to_string(),
            lat: 34.7,
            lon: -72.4,
            alt: 0.0,
        }
    }

    #[test]
    fn record_bins_channels_by_name_replaces_equal_epochs_and_keeps_meta() {
        let (_tx, rx) = mpsc::channel();
        let mut m = MatrixMachine::new(rx);
        m.state_path = "/tmp/omegaflow_matrix_test_state.bin".to_string();
        m.record(
            &frame_earth(),
            vec![
                mk_channel("ndbc_a_wspd", 21600.0, 3.0),
                mk_channel("ndbc_a_wspd", 43200.0, 5.0),
                mk_channel("ndbc_a_wspd", 21600.0, 3.5),
                mk_channel("ndbc_b_wtmp", 21600.0, 20.0),
            ],
        );
        assert_eq!(m.rings.len(), 2);
        assert_eq!(m.rings["ndbc_a_wspd"], vec![(21600.0, 3.5), (43200.0, 5.0)]);
        assert_eq!(m.rings["ndbc_b_wtmp"], vec![(21600.0, 20.0)]);
        assert!(m.metas.contains_key("ndbc_a_wspd"));
        assert_eq!(m.metas["ndbc_a_wspd"].force, 7);
    }

    #[test]
    fn tick_drains_the_machine_channel() {
        let (tx, rx) = mpsc::channel();
        let mut m = MatrixMachine::new(rx);
        m.state_path = "/tmp/omegaflow_matrix_test_state.bin".to_string();
        tx.send((frame_earth(), vec![mk_channel("ndbc_a_wspd", 21600.0, 3.0)]))
            .expect("send");
        m.tick(None, [0.0, 0.0, 0.0], 0.0, 1);
        assert_eq!(m.rings.len(), 1);
    }

    #[test]
    fn state_survives_save_and_load() {
        let path = format!(
            "/tmp/omegaflow_matrix_test_state_{}.bin",
            std::process::id()
        );
        let (_tx, rx) = mpsc::channel();
        let mut m = MatrixMachine::new(rx);
        m.state_path = path.clone();
        m.record(
            &frame_earth(),
            vec![
                mk_channel("ndbc_a_wspd", 21600.0, 3.0),
                mk_channel("ndbc_a_wspd", 43200.0, 5.0),
                mk_channel("ndbc_b_wtmp", 21600.0, 20.0),
            ],
        );
        m.present = vec!["ndbc_a_wspd".to_string()];
        let mut r = PairResult::fresh();
        r.cells = 42;
        r.accum.m = 11;
        r.accum.cells1.push(MatrixCellVerdict {
            dir: 1,
            shift: 4,
            n: 100,
            te: 0.9,
            thr: 0.4,
        });
        m.results.insert(pair_key("ndbc_a_wspd", "ndbc_b_wtmp"), r);
        m.line.arrows = 2;
        m.save_state_to(&path);
        let loaded = MatrixMachine::load_state_from(&path).expect("state loads");
        assert_eq!(loaded.rings.len(), 2);
        assert_eq!(
            loaded.rings["ndbc_a_wspd"],
            vec![(21600.0, 3.0), (43200.0, 5.0)]
        );
        assert_eq!(loaded.metas.len(), 2);
        assert_eq!(loaded.present, vec!["ndbc_a_wspd"]);
        assert_eq!(loaded.results.len(), 1);
        let lr = &loaded.results[&pair_key("ndbc_a_wspd", "ndbc_b_wtmp")];
        assert_eq!(lr.cells, 42);
        assert_eq!(lr.accum.m, 11);
        assert_eq!(lr.accum.cells1.len(), 1);
        assert_eq!(loaded.line.arrows, 2);
        let _ = std::fs::remove_file(path);
    }
}

#[cfg(test)]
mod matrix_rebuild_tests {
    use crate::archivar::{
        body_fixed_to_icrs, BodyEphemeris, BodyProperties, Buffer, ChebyshevGranule,
    };
    use crate::mathematikerin::machines::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn body(x_m: f64, gm: Option<f64>) -> BodyEphemeris {
        let props = BodyProperties {
            α0_deg: 0.0,
            dα0_dt_deg_per_century: 0.0,
            δ0_deg: 90.0,
            dδ0_dt_deg_per_century: 0.0,
            w0_deg: 0.0,
            dw_dt_deg_per_day: 0.0,
            radius_m: 6378137.0,
            flattening: Some(0.0),
            gaussian_inverse_square: 0.0,
            gaussian_inverse: 0.0,
            erfc: 0.0,
            exponential_decay: 0.0,
            patch_levy: 0.0,
            gm,
            j2: None,
            j4: None,
            radii_b: None,
            radii_c: None,
            nut_ra: None,
            nut_dec: None,
            nutation: None,
            omega_g: None,
        };
        let mut cx = [0.0_f64; crate::archivar::CHEBYSHEV_N];
        cx[0] = x_m;
        let granule = ChebyshevGranule {
            t0_jd: crate::archivar::J2000_EPOCH,
            dt_jd: 64.0,
            cx,
            cy: [0.0; crate::archivar::CHEBYSHEV_N],
            cz: [0.0; crate::archivar::CHEBYSHEV_N],
        };
        BodyEphemeris {
            granules: vec![granule],
            rotation_matrices: vec![],
            props: Some(props),
            orbit: None,
            granule_hint: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    fn mk_channel(
        name: &str,
        epoch: f64,
        val: f64,
    ) -> (crate::archivar::Channel, crate::archivar::FieldConfig) {
        (
            crate::archivar::Channel {
                z: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                epoch,
                position: crate::archivar::Position::Surface {
                    body_name: "earth".to_string(),
                    lat: 0.0,
                    lon: 0.0,
                    alt: 0.0,
                },
                name: name.to_string(),
                value: val,
            },
            crate::archivar::FieldConfig {
                key: "TMP".to_string(),
                name: name.to_string(),
                kernel: 0,
                force: 1,
                tau: 21600.0,
                absorption: 0.0,
                advection: 0.0,
                unit: "K".to_string(),
                fold: None,
            },
        )
    }

    #[test]
    fn rebuild_emits_astro_channels_for_all_bodies_including_gm_less() {
        let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
        eph.insert("earth".to_string(), body(0.0, Some(3.986004418e14)));
        eph.insert("moon".to_string(), body(3.844e8, None));
        let buf = Buffer {
            cache: crate::archivar::build_spatial_hash(vec![], 1.0),
            eph: Arc::new(eph),
            curves: None,
            spectral: Vec::new(),
        };
        let (_tx, rx) = mpsc::channel();
        let mut m = MatrixMachine::new(rx);
        m.state_path = "/tmp/omegaflow_matrix_test_state.bin".to_string();
        let frame = crate::archivar::Frame::Surface {
            body_name: "earth".to_string(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        };
        let t0 = 21600.0 * 5.0;
        let mut channels = Vec::new();
        for i in 0..40 {
            channels.push(mk_channel(
                "earth_tmp",
                t0 + (i as f64) * 21600.0,
                20.0 + i as f64,
            ));
        }
        m.record(&frame, channels);
        let presence = body_fixed_to_icrs("earth", 0.0, 0.0, 0.0, t0, &buf.eph).unwrap();
        m.last_field = Some(Arc::new(buf));
        m.rebuild(presence, t0);
        assert!(m.rings.contains_key("eph_earth"));
        assert!(m.rings.contains_key("eph_earth_lon_sin"));
        assert!(m.rings.contains_key("eph_earth_lon_cos"));
        assert!(m.rings.contains_key("eph_moon_lon_sin"));
        assert!(m.rings.contains_key("eph_moon_lon_cos"));
        assert!(!m.rings.contains_key("eph_moon"));
        assert!(m.rings["eph_moon_lon_sin"].len() >= MATRIX_N_GATE);
        for &(_, v) in &m.rings["eph_moon_lon_sin"] {
            assert!(v >= -1.0 && v <= 1.0);
        }
        let earth_sin = &m.rings["eph_earth_lon_sin"];
        let moon_sin = &m.rings["eph_moon_lon_sin"];
        assert_ne!(
            earth_sin[0].1, moon_sin[0].1,
            "earth and moon longitudes must differ"
        );
    }
}
