use super::*;

type PresenceSample = (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64);
type EventTuple<'a> = (f64, f64, f64, f64, Option<f64>, Option<&'a str>);

static ANOMALY_TEST_GATE: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn beat_source_precedence_is_total() {
    use crate::archivar::main_flow::{BeatSource, select_beat_source};
    assert_eq!(
        select_beat_source(true, Some("AA:BB"), Some("/f")),
        BeatSource::Serial
    );
    assert_eq!(
        select_beat_source(false, Some("AA:BB"), Some("/f")),
        BeatSource::Ble
    );
    assert_eq!(select_beat_source(false, None, Some("/f")), BeatSource::Fit);
    assert_eq!(select_beat_source(false, None, None), BeatSource::None);
    assert_eq!(
        select_beat_source(false, Some("  "), Some("/f")),
        BeatSource::Fit
    );
    assert_eq!(select_beat_source(false, None, Some("")), BeatSource::None);
}

fn field_fixture(name: &str, tau: f64) -> FieldConfig {
    FieldConfig {
        key: name.into(),
        name: name.into(),
        kernel: 0,
        force: 0,
        tau,
        absorption: 0.0,
        advection: 0.0,
        unit: String::new(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
    }
}

fn source_fixture(format: &str, extracts: Vec<Extract>) -> SourceConfig {
    SourceConfig {
        ttl: 3600,
        url: "https://example.com/x".into(),
        frame: Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: format.into(),
        extracts,
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    }
}

fn fixture_lsk() -> LeapSeconds {
    LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    }
}

fn euvs_json() -> &'static str {
    r#"[
        {"time_tag": "2026-08-20T08:50:00Z", "line": "304", "value": 1.1e-4},
        {"time_tag": "2026-08-20T08:50:00Z", "line": "284", "value": 2.2e-4},
        {"time_tag": "2026-08-20T08:51:00Z", "line": "304", "value": 3.3e-4},
        {"time_tag": "2026-08-20T08:51:00Z", "line": "284", "value": 4.4e-4},
        {"time_tag": "2026-08-20T08:51:00Z", "line": "mgii_index", "value": 0.278}
    ]"#
}

fn last_where_fixture(name: &str, key: &str, fk: &str, fv: &str) -> Extract {
    let mut fc = field_fixture(name, 60.0);
    fc.key = key.into();
    Extract::Last(fc, Some((fk.into(), fv.into())))
}

#[test]
fn test_last_where_picks_matching_row() {
    let src = source_fixture(
        "json",
        vec![
            last_where_fixture("euv304", "value", "line", "304"),
            last_where_fixture("euv284", "value", "line", "284"),
        ],
    );
    match extract(&src, euvs_json(), 8.0e8, &fixture_lsk()) {
        ExtractResult::Measurements(channels) => {
            let vals: HashMap<&str, f64> = channels
                .iter()
                .map(|(c, fc)| (fc.name.as_str(), c.value))
                .collect();
            assert_eq!(vals.get("euv304"), Some(&3.3e-4));
            assert_eq!(vals.get("euv284"), Some(&4.4e-4));
        }
        _ => panic!("extract is not Measurements"),
    }
}

#[test]
fn test_last_where_no_match_absent() {
    let src = source_fixture(
        "json",
        vec![last_where_fixture("euv999", "value", "line", "999")],
    );
    match extract(&src, euvs_json(), 8.0e8, &fixture_lsk()) {
        ExtractResult::Measurements(channels) => {
            assert!(channels.is_empty(), "no matching row → absent, never 0.0")
        }
        _ => panic!("extract is not Measurements"),
    }
}

#[test]
fn test_parse_window_directive() {
    let content = "url https://example.com/x\nttl 600\nat sun\nwindow 1746727000 1746728000\nfield value euv304 inverse-square em W/m2 60.0 0.0 0.0\n";
    let sources = parse_sources(content);
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].window, Some((1746727000.0, 1746728000.0)));
}

#[test]
fn test_parse_window_from_after_until_absent() {
    let content = "url https://example.com/x\nttl 600\nat sun\nwindow 1746728000 1746727000\nfield value euv304 inverse-square em W/m2 60.0 0.0 0.0\n";
    let sources = parse_sources(content);
    assert_eq!(sources.len(), 1);
    assert_eq!(
        sources[0].window, None,
        "from > until stays absent, never an empty window"
    );
}

#[test]
fn test_parse_live_directive_marks_live_only() {
    let content = "url https://example.com/rolling\nttl 60\nlive\nat sun\nfield value euv inverse-square em W/m2 60.0 0.0 0.0\nurl https://example.com/mirrored\nttl 60\nat sun\nfield value euv inverse-square em W/m2 60.0 0.0 0.0\n";
    let sources = parse_sources(content);
    assert_eq!(sources.len(), 2);
    assert!(
        sources[0].live_only,
        "the live directive marks the source live-only (no CDN, no mirror)"
    );
    assert!(
        !sources[1].live_only,
        "without the live directive the source keeps its CDN path"
    );
}

#[test]
fn test_extract_window_filters_by_epoch() {
    let lsk = fixture_lsk();
    let now_tdb = 8.0e8;
    let now_unix = lsk.tdb_to_unix(now_tdb).unwrap();
    let mut src = source_fixture(
        "json",
        vec![last_where_fixture("euv304", "value", "line", "304")],
    );
    src.window = Some((now_unix - 100.0, now_unix + 100.0));
    match extract(&src, euvs_json(), now_tdb, &lsk) {
        ExtractResult::Measurements(channels) => {
            assert!(!channels.is_empty(), "epoch inside the window survives")
        }
        _ => panic!("extract is not Measurements"),
    }
    src.window = Some((now_unix + 1000.0, now_unix + 2000.0));
    match extract(&src, euvs_json(), now_tdb, &lsk) {
        ExtractResult::Measurements(channels) => {
            assert!(channels.is_empty(), "epoch outside the window is dropped")
        }
        _ => panic!("extract is not Measurements"),
    }
}

#[test]
fn test_first_where_picks_first_matching_row() {
    let mut fc = field_fixture("euv304_first", 60.0);
    fc.key = "value".into();
    let src = source_fixture(
        "json",
        vec![Extract::First(fc, Some(("line".into(), "304".into())))],
    );
    match extract(&src, euvs_json(), 8.0e8, &fixture_lsk()) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            assert_eq!(channels[0].0.value, 1.1e-4);
        }
        _ => panic!("extract is not Measurements"),
    }
}

#[test]
fn test_last_where_numeric_filter_value() {
    let json = r#"[
        {"time_tag": "2026-08-20T08:50:00Z", "satellite": 18, "flux": 7.0},
        {"time_tag": "2026-08-20T08:51:00Z", "satellite": 19, "flux": 9.0}
    ]"#;
    let src = source_fixture(
        "json",
        vec![last_where_fixture("g18", "flux", "satellite", "18")],
    );
    match extract(&src, json, 8.0e8, &fixture_lsk()) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            assert_eq!(channels[0].0.value, 7.0);
        }
        _ => panic!("extract is not Measurements"),
    }
}

#[test]
fn test_extract_series_where_filters_rows() {
    let src = source_fixture(
        "json",
        vec![last_where_fixture("euv304", "value", "line", "304")],
    );
    let series = extract_series(&src, euvs_json(), &fixture_lsk());
    assert_eq!(series.len(), 2);
    assert_eq!(series[0].1, 1.1e-4);
    assert_eq!(series[1].1, 3.3e-4);
}

#[test]
fn test_parse_where_clause() {
    let content = "url https://example.com/euvs.json\nttl 600\nat sun\nlast value euv304 inverse-square em W/m2 60.0 0.0 0.0 where line 304\n";
    let sources = parse_sources(content);
    assert_eq!(sources.len(), 1);
    match &sources[0].extracts[0] {
        Extract::Last(fc, Some((fk, fv))) => {
            assert_eq!(fc.name, "euv304");
            assert_eq!(fk, "line");
            assert_eq!(fv, "304");
        }
        _ => panic!("parsed extract is not a filtered last extract"),
    }
}

#[test]
fn test_parse_where_malformed_refused() {
    let content = "url https://example.com/euvs.json\nttl 600\nat sun\nlast value euv304 inverse-square em W/m2 60.0 0.0 0.0 where line\n";
    let sources = parse_sources(content);
    assert_eq!(sources.len(), 1);
    assert!(
        sources[0].extracts.is_empty(),
        "malformed where must refuse the line loudly"
    );
}

#[test]
fn test_parse_where_refused_on_field() {
    let content = "url https://example.com/euvs.json\nttl 600\nat sun\nfield value euv304 inverse-square em W/m2 60.0 0.0 0.0 where line 304\n";
    let sources = parse_sources(content);
    assert_eq!(sources.len(), 1);
    assert!(sources[0].extracts.is_empty());
}

#[test]
fn test_convert_to_si() {
    let close = |a: Option<f64>, b: f64| {
        let a = match a {
            Some(v) => v,
            None => panic!("conversion returned None"),
        };
        assert!((a - b).abs() < 1e-9 * b.abs().max(1e-12), "{a} vs {b}");
    };
    close(convert_to_si(5.0, "km"), 5000.0);
    close(convert_to_si(1000.0, "hPa"), 100000.0);
    close(convert_to_si(30.0, "nT"), 30e-9);
    close(convert_to_si(20.0, "C"), 293.15);
    close(convert_to_si(2.0, "mgal"), 2e-5);
    close(convert_to_si(3.0, "ppm"), 3e-6);
    close(convert_to_si(72.0, "km/h"), 20.0);
    close(convert_to_si(7.0, "m"), 7.0);
    close(convert_to_si(-1.0, "km"), -1000.0);
    close(convert_to_si(90.0, "deg"), std::f64::consts::PI / 2.0);
    close(convert_to_si(1.0, "M_sun"), 1.98847e30);
    close(convert_to_si(1.0, "MW"), 1e6);
    close(convert_to_si(2.0, "d"), 172800.0);
    close(convert_to_si(1.0, "uatm"), 0.101325);
    close(convert_to_si(1.0, "cfs"), 0.028316846592);
    close(convert_to_si(1.0, "%"), 0.01);
    close(convert_to_si(1.0, "pc/cm3"), 3.085677581e22);
    close(convert_to_si(1.0, "knot"), 0.514444);
    close(convert_to_si(1.0, "Jy_km/s"), 1e-23);
    close(convert_to_si(1.0, "Crab"), 2.4e-14);
    close(convert_to_si(4.4, "logg"), 251.188643150958);
    close(convert_to_si(7.2, "Mw"), 10.0f64.powf(1.5 * 7.2 + 9.1));
    close(convert_to_si(334.0, "cpm"), 1e-6 / 3600.0);
    close(convert_to_si(1.0, "decibar"), 1e4);
    close(convert_to_si(1.0, "mV/m"), 1e-3);
    close(convert_to_si(1.0, "nPa"), 1e-9);
    close(convert_to_si(1.0, "sfu"), 1e-22);
    close(convert_to_si(2.0, "W/m^2/nm"), 2.0e9);
    close(convert_to_si(3.0, "microMoleQuanta/m^2/sec"), 3.0e-6);
    close(convert_to_si(40.0, "dbhz"), 1.0e4);
    close(convert_to_si(30.0, "dBHz"), 1.0e3);
    close(convert_to_si(1.0, "Bq/L"), 1.0e3);
    close(convert_to_si(2.0, "BQ/M³"), 2.0);
    assert!(convert_to_si(9.0, "weird").is_none());
    assert!(convert_to_si(7.2, "M").is_none());
    assert!(convert_to_si(5.0, "mag").is_none());
    assert!(convert_to_si(1.0, "dex").is_none());
    assert!(convert_to_si(0.0, "").is_some());
}

#[test]
fn test_anomaly_reporter() {
    let _gate = ANOMALY_TEST_GATE.lock();
    ANOMALY_COLLECT.with(|c| c.set(true));
    report_anomaly(
        "API Unreachable",
        "https://example.org/x",
        "fetch returned void",
    );
    report_anomaly("Malformed Data", "https://example.org/y", "JSON parse void");
    let _ = parse_sources(
        "url https://example.org/em\nttl 60\nformat json\nfield mag mag inverse-square em K 60 0 0\n",
    );
    let _ = parse_sources(
        "url https://example.org/syn\nttl 60\nformat json\non earth\nfield p p inverse-square electroweak K 60 0 0\n",
    );
    let anomalies = take_anomalies();
    assert_eq!(anomalies.len(), 5);
    assert_eq!(anomalies[0].category, "API Unreachable");
    assert_eq!(anomalies[0].url, "https://example.org/x");
    assert_eq!(anomalies[1].category, "Malformed Data");
    assert_eq!(anomalies[2].category, "Physics Mismatch");
    assert_eq!(anomalies[3].category, "Invalid Syntax");
    assert_eq!(anomalies[4].category, "Invalid Syntax");
    let body = anomaly_issue_body(&anomalies);
    assert!(body.starts_with("| Category | URL | Details |\n|---|---|---|\n"));
    assert!(body.contains("| API Unreachable | https://example.org/x | fetch returned void |"));
    assert!(body.contains("| Malformed Data | https://example.org/y | JSON parse void |"));
    assert!(body.contains(
        "| Physics Mismatch | https://example.org/em | field mag: unit \"K\" not in force registry |"
    ));
    assert!(body.contains(
        "| Invalid Syntax | https://example.org/syn | on needs <body> <lat> <lon> [alt]: on earth |"
    ));
    assert!(take_anomalies().is_empty());
    ANOMALY_COLLECT.with(|c| c.set(false));
}

#[test]
fn test_allowed_units_for_force() {
    assert!(allowed_units_for_force(0).contains(&"nt"));
    assert!(allowed_units_for_force(5).contains(&"k"));
    assert!(allowed_units_for_force(7).contains(&"m/s"));
    assert!(allowed_units_for_force(9).is_empty());
    assert!(allowed_units_for_force(0).contains(&"mag"));
    assert!(allowed_units_for_force(0).contains(&"jy_km/s"));
    assert!(allowed_units_for_force(0).contains(&"k.m/s"));
    assert!(allowed_units_for_force(1).contains(&"logg"));
    assert!(allowed_units_for_force(1).contains(&"m_sun"));
    assert!(allowed_units_for_force(3).contains(&"mw"));
    assert!(allowed_units_for_force(5).contains(&"mw"));
    assert!(allowed_units_for_force(6).contains(&"du"));
    assert!(allowed_units_for_force(6).contains(&"ug/m3"));
    assert!(allowed_units_for_force(7).contains(&"cfs"));
    assert!(allowed_units_for_force(8).contains(&"ua/m2"));
    assert!(!allowed_units_for_force(0).contains(&"k"));
    assert!(!allowed_units_for_force(2).contains(&"c"));
    assert!(!allowed_units_for_force(6).contains(&"kt"));
    assert!(allowed_units_for_force(0).contains(&"count"));
    assert!(allowed_units_for_force(0).contains(&"rad"));
    assert!(allowed_units_for_force(0).contains(&"dbhz"));
    assert!(allowed_units_for_force(0).contains(&"m-2.s-1.tev-1"));
    assert!(allowed_units_for_force(0).contains(&"tev"));
    assert!(allowed_units_for_force(0).contains(&"bq/l"));
    assert!(allowed_units_for_force(0).contains(&"bq/m3"));
    assert!(allowed_units_for_force(0).contains(&"s"));
    assert!(allowed_units_for_force(0).contains(&"d"));
    assert!(allowed_units_for_force(0).contains(&"ms"));
    assert!(allowed_units_for_force(0).contains(&"j"));
    assert!(allowed_units_for_force(0).contains(&"km/s"));
    assert!(allowed_units_for_force(0).contains(&"arcsec"));
    assert!(allowed_units_for_force(0).contains(&"%"));
    assert!(allowed_units_for_force(1).contains(&"m/s"));
    assert!(allowed_units_for_force(2).contains(&"npa"));
    assert!(allowed_units_for_force(5).contains(&"km/s"));
    assert!(allowed_units_for_force(6).contains(&"cm"));
    assert!(allowed_units_for_force(6).contains(&"mm"));
    assert!(allowed_units_for_force(7).contains(&"cm/s"));
    assert!(allowed_units_for_force(7).contains(&"deg"));
    assert_eq!(convert_to_si(7.0, "count"), Some(7.0));
}

#[test]
fn test_unit_from_name_suffix() {
    assert_eq!(unit_from_name_suffix("omni_imf_bx_gse_nt"), Some("nT"));
    assert_eq!(
        unit_from_name_suffix("omni_solarwind_flow_speed_kms"),
        Some("km/s")
    );
    assert_eq!(
        unit_from_name_suffix("omni_solarwind_density_percc"),
        Some("cm-3")
    );
    assert_eq!(
        unit_from_name_suffix("omni_solarwind_pressure_npa"),
        Some("nPa")
    );
    assert_eq!(unit_from_name_suffix("omni_solarwind_temp_k"), Some("K"));
    assert_eq!(
        unit_from_name_suffix("gracefo_kbr_inter_satellite_distance_m"),
        Some("m")
    );
    assert_eq!(
        unit_from_name_suffix("gracefo_kbr_absolute_electron_density_m3"),
        Some("1/m3")
    );
    assert_eq!(unit_from_name_suffix("swarm_ion_density_cm3"), Some("cm-3"));
    assert_eq!(
        unit_from_name_suffix("swarm_spacecraft_potential_v"),
        Some("V")
    );
    assert_eq!(
        unit_from_name_suffix("gracefo_kbr_range_rate_m_s"),
        Some("m/s")
    );
    assert_eq!(
        unit_from_name_suffix("gracefo_kbr_range_accl_m_s2"),
        Some("m/s2")
    );
    assert_eq!(unit_from_name_suffix("no_suffix_here"), None);
}

#[test]
fn test_cycle_unit_identity_and_physics_gate() {
    assert_eq!(convert_to_si(1.0, "cycle"), Some(1.0));
    let _gate = ANOMALY_TEST_GATE.lock();
    ANOMALY_COLLECT.with(|c| c.set(true));
    let _ = take_anomalies();
    let sources = parse_sources(
        "url https://example.org/cycle\nttl 60\nformat json\nat earth\nfield n n inverse-square em cycle 60 0 0\n",
    );
    assert_eq!(sources.len(), 1);
    let anomalies = take_anomalies();
    assert!(
        !anomalies.iter().any(|a| a.category == "Physics Mismatch"),
        "a field carrying unit cycle must pass the physics-mismatch gate"
    );
    ANOMALY_COLLECT.with(|c| c.set(false));
}

#[test]
fn test_normalize_unit() {
    assert_eq!(normalize_unit("nT"), "nt");
    assert_eq!(normalize_unit(" M_sun "), "m_sun");
    assert_eq!(normalize_unit("µg/m3"), "ug/m3");
    assert_eq!(normalize_unit("m/s²"), "m/s2");
    assert_eq!(normalize_unit("K"), "k");
    assert_eq!(normalize_unit("Pa"), "pa");
}

#[test]
fn test_hapi_fill_skipped_and_component_index() {
    let json = r#"{
        "parameters": [
            {"name": "Time"},
            {"name": "VEC", "fill": "-1.0e31"},
            {"name": "SCAL", "fill": "-1.0e31"}
        ],
        "data": [
            ["2026-08-18T00:00:00Z", [-1.0e31, -1.0e31, -1.0e31], 9.0],
            ["2026-08-18T01:00:00Z", [1.1, 2.2, 3.3], 7.5]
        ]
    }"#;
    let src = source_fixture(
        "json",
        vec![
            Extract::Field(field_fixture("x", 3600.0)),
            Extract::Field(field_fixture("y", 3600.0)),
            Extract::Field(field_fixture("z", 3600.0)),
            Extract::Field(field_fixture("s", 3600.0)),
            Extract::Hapi(vec![
                ("VEC.0".into(), "x".into()),
                ("VEC.1".into(), "y".into()),
                ("VEC.2".into(), "z".into()),
                ("SCAL".into(), "s".into()),
            ]),
        ],
    );
    match extract(&src, json, 8.0e8, &fixture_lsk()) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 4);
            let vals: Vec<(&str, f64)> = channels
                .iter()
                .map(|(c, fc)| (fc.name.as_str(), c.value))
                .collect();
            assert!(vals.contains(&("x", 1.1)));
            assert!(vals.contains(&("y", 2.2)));
            assert!(vals.contains(&("z", 3.3)));
            assert!(vals.contains(&("s", 7.5)));
        }
        _ => panic!("extract is not Measurements"),
    }
    let fill_only = r#"{
        "parameters": [{"name": "Time"}, {"name": "SCAL", "fill": "-1.0e31"}],
        "data": [["2026-08-18T01:00:00Z", -1.0e31]]
    }"#;
    let src_fill = source_fixture(
        "json",
        vec![
            Extract::Field(field_fixture("s", 3600.0)),
            Extract::Hapi(vec![("SCAL".into(), "s".into())]),
        ],
    );
    match extract(&src_fill, fill_only, 8.0e8, &fixture_lsk()) {
        ExtractResult::Measurements(channels) => {
            assert!(channels.is_empty(), "fill must not be ingested");
        }
        _ => panic!("extract is not Measurements"),
    }
}

#[test]
fn test_hapi_without_parameters_array_vector_and_declared_fill() {
    let json = r#"{
        "status": {"code": 1200},
        "data": [
            ["2026-08-20T00:00Z", [99999.0, 99999.0, 99999.0], 53558.8],
            ["2026-08-20T00:01Z", [10864.4, 2071.9, 52404.9], 53546.8]
        ]
    }"#;
    let mut src = source_fixture(
        "json",
        vec![
            Extract::Field(field_fixture("x", 3600.0)),
            Extract::Field(field_fixture("y", 3600.0)),
            Extract::Field(field_fixture("z", 3600.0)),
            Extract::Field(field_fixture("f", 3600.0)),
            Extract::Hapi(vec![
                ("Field_Vector.0".into(), "x".into()),
                ("Field_Vector.1".into(), "y".into()),
                ("Field_Vector.2".into(), "z".into()),
                ("Field_Magnitude".into(), "f".into()),
            ]),
        ],
    );
    src.hapi_fill.insert("Field_Vector".into(), 99999.0);
    src.hapi_fill.insert("Field_Magnitude".into(), 99999.0);
    match extract(&src, json, 8.0e8, &fixture_lsk()) {
        ExtractResult::Measurements(channels) => {
            let vals: Vec<(&str, f64)> = channels
                .iter()
                .map(|(c, fc)| (fc.name.as_str(), c.value))
                .collect();
            assert!(vals.contains(&("x", 10864.4)));
            assert!(vals.contains(&("y", 2071.9)));
            assert!(vals.contains(&("z", 52404.9)));
            assert!(vals.contains(&("f", 53546.8)));
        }
        _ => panic!("extract is not Measurements"),
    }
}
use std::collections::HashMap;

fn full_fixture_lsk() -> super::LeapSeconds {
    super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![
            (10.0, 63072000.0),
            (11.0, 78796800.0),
            (12.0, 94694400.0),
            (13.0, 126230400.0),
            (14.0, 157766400.0),
            (15.0, 189302400.0),
            (16.0, 220924800.0),
            (17.0, 252460800.0),
            (18.0, 283996800.0),
            (19.0, 315532800.0),
            (20.0, 362793600.0),
            (21.0, 394329600.0),
            (22.0, 425865600.0),
            (23.0, 489024000.0),
            (24.0, 567993600.0),
            (25.0, 631152000.0),
            (26.0, 662688000.0),
            (27.0, 709948800.0),
            (28.0, 741484800.0),
            (29.0, 773020800.0),
            (30.0, 820454400.0),
            (31.0, 867715200.0),
            (32.0, 915148800.0),
            (33.0, 1136073600.0),
            (34.0, 1230768000.0),
            (35.0, 1341100800.0),
            (36.0, 1435708800.0),
            (37.0, 1483228800.0),
        ],
    }
}

#[test]
fn test_render_source_url_substitutions() {
    let src = SourceConfig {
        ttl: 100,
        url: "https://example.com?sstr={target}&from={catalog}".into(),
        frame: super::Frame::Surface {
            body_name: "body_test".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "json".into(),
        extracts: vec![],
        headers: vec![],
        post_body: None,
        target: Some("Ceres".into()),
        catalog: Some("fp_psc".into()),
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    let url = render_source_url(
        &src,
        RenderCtx {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            tdb: 8.0e8,
            r: 1000.0,
            eph: &HashMap::new(),
            lsk: &fixture_lsk,
        },
        &HashMap::new(),
    );
    let url = url.unwrap();
    assert!(url.contains("Ceres"));
    assert!(url.contains("fp_psc"));
    assert!(!url.contains("{target}"));
    assert!(!url.contains("{catalog}"));
}

#[test]
fn test_render_source_url_carries_observer_epoch() {
    let mut src = source_fixture("json", vec![]);
    src.url = "https://example.com/field?start={week_ago}&end={today}".into();
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    let past_tdb = 8.0e8;
    let past = render_source_url(
        &src,
        RenderCtx {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            tdb: past_tdb,
            r: 1000.0,
            eph: &HashMap::new(),
            lsk: &fixture_lsk,
        },
        &HashMap::new(),
    )
    .unwrap();
    let past_unix = fixture_lsk.tdb_to_unix(past_tdb).unwrap() as u64;
    let (ty, tm, td) = super::days_to_ymd(past_unix / 86400);
    let (wy, wm, wd) = super::days_to_ymd(past_unix / 86400 - 7);
    assert!(
        past.contains(&format!("end={}-{:02}-{:02}", ty, tm, td)),
        "past url {}",
        past
    );
    assert!(
        past.contains(&format!("start={}-{:02}-{:02}", wy, wm, wd)),
        "past url {}",
        past
    );
    let now_tdb = fixture_lsk.system_now_tdb().unwrap();
    let present = render_source_url(
        &src,
        RenderCtx {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            tdb: now_tdb,
            r: 1000.0,
            eph: &HashMap::new(),
            lsk: &fixture_lsk,
        },
        &HashMap::new(),
    )
    .unwrap();
    let now_unix = fixture_lsk.tdb_to_unix(now_tdb).unwrap() as u64;
    let (ny, nm, nd) = super::days_to_ymd(now_unix / 86400);
    assert!(
        present.contains(&format!("end={}-{:02}-{:02}", ny, nm, nd)),
        "present url {}",
        present
    );
    assert_ne!(
        past, present,
        "the rendered URL must follow the observer epoch, not the machine now"
    );
}

#[test]
fn test_render_source_url_pre_2000_epoch() {
    let mut src = source_fixture("json", vec![]);
    src.url = "https://example.com/field?start={week_ago}&end={today}".into();
    let lsk = super::embedded_lsk().expect("the embedded naif0012 table is program identity");
    let pre_2000_tdb = -4.0e7;
    let url = render_source_url(
        &src,
        RenderCtx {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            tdb: pre_2000_tdb,
            r: 0.0,
            eph: &HashMap::new(),
            lsk: &lsk,
        },
        &HashMap::new(),
    )
    .unwrap();
    let unix = lsk.tdb_to_unix(pre_2000_tdb).unwrap() as u64;
    let (ty, tm, td) = super::days_to_ymd(unix / 86400);
    assert!(
        url.contains(&format!("end={}-{:02}-{:02}", ty, tm, td)),
        "a pre-2000 observer epoch (negative TDB-J2000) must render its own dates: {}",
        url
    );
    assert!(
        !url.contains("2026"),
        "a pre-2000 observer epoch must not render machine-now dates: {}",
        url
    );
}

#[test]
fn test_temporal_urls_carry_distinct_cache_identity() {
    let mut src = source_fixture("json", vec![]);
    src.url = "https://example.com/field?start={week_ago}&end={today}".into();
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    let harvest_2005 = render_source_url(
        &src,
        RenderCtx {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            tdb: 8.0e8,
            r: 0.0,
            eph: &HashMap::new(),
            lsk: &fixture_lsk,
        },
        &HashMap::new(),
    )
    .unwrap();
    let harvest_2026 = render_source_url(
        &src,
        RenderCtx {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            tdb: 1.8e9,
            r: 0.0,
            eph: &HashMap::new(),
            lsk: &fixture_lsk,
        },
        &HashMap::new(),
    )
    .unwrap();
    assert_ne!(
        source_name_from_url(&harvest_2005),
        source_name_from_url(&harvest_2026),
        "the 2005 harvest and the 2026 harvest are distinct cache identities — \
         a scroll must never overwrite the other epoch's harvest"
    );
}

#[test]
fn test_extract_default_epoch_is_observer_epoch() {
    let mut fc = field_fixture("temp_c", 60.0);
    fc.key = "temp_c".into();
    let src = source_fixture(
        "json",
        vec![Extract::Map {
            arr_path: "rows".into(),
            lat_key: "lat".into(),
            lon_key: "lon".into(),
            alt_key: String::new(),
            epoch_key: String::new(),
            val_key: String::new(),
            alt_scale: 1.0,
            vel_key: String::new(),
            vel_scale: 1.0,
            trk_key: String::new(),
            vr_key: String::new(),
            fields: vec![fc],
            lat_sign: None,
            lon_sign: None,
            epoch_scale: 1.0,
            tau_key: String::new(),
            mag_type_key: String::new(),
        }],
    );
    let body = r#"{"rows":[{"lat":1.0,"lon":2.0,"temp_c":21.0}]}"#;
    let scrolled = 8.0e8;
    match extract(&src, body, scrolled, &fixture_lsk()) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            assert!(
                (channels[0].0.epoch - scrolled).abs() < 1e-6,
                "an epoch-less row carries the observer's epoch, not the machine now"
            );
        }
        _ => panic!("extract is not Measurements"),
    }
}

#[test]
fn test_epoch_stamp_gate_is_observer_time() {
    let ttl = 3600u64;
    let path = "/tmp/opencode/omegaflow_epoch_stamp_gate_test.json";
    let stamp_path = format!("{}.epoch", path);
    let _ = std::fs::create_dir_all("/tmp/opencode");
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(&stamp_path);
    assert!(
        !cache_fresh_at(path, ttl, 8.0e8),
        "an unstamped cache is never fresh"
    );
    write_epoch_stamp(path, 8.0e8);
    assert!(
        cache_fresh_at(path, ttl, 8.0e8 + 100.0),
        "within the ttl the harvest serves"
    );
    assert!(
        !cache_fresh_at(path, ttl, 8.0e8 + ttl as f64 + 100.0),
        "beyond the ttl the harvest is stale"
    );
    assert!(
        !cache_fresh_at(path, ttl, 8.0e8 - ttl as f64 - 100.0),
        "a scroll before the stamp epoch is stale — no 2026 harvest at 2005"
    );
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(&stamp_path);
}

#[test]
fn test_post_body_rendering() {
    let src = SourceConfig {
        ttl: 100,
        url: "https://earth-search.aws.element84.com/v0/search".into(),
        frame: super::Frame::Surface {
            body_name: "body_test".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "json".into(),
        extracts: vec![],
        headers: vec![("Content-Type".into(), "application/stac+json".into())],
        post_body: Some(
            "{\"bbox\":[{lon_min},{lat_min},{lon_max},{lat_max}],\"datetime\":\"{today}/{today}\"}"
                .into(),
        ),
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    let body = render_source_body(
        &src,
        RenderCtx {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            tdb: 8.0e8,
            r: 100000.0,
            eph: &HashMap::new(),
            lsk: &fixture_lsk,
        },
    );
    assert!(body.is_some());
    let b = body.unwrap();
    assert!(b.contains("bbox"));
    assert!(b.contains("{lon_min}"));
    assert!(!b.contains("{today}"));
}

#[test]
fn test_csv_zip_post_body_resolves_secret() {
    let src = SourceConfig {
        ttl: 100,
        url: "https://www.wis-tns.org/system/files/tns_public_objects/tns_public_objects.csv.zip"
            .into(),
        frame: super::Frame::Surface {
            body_name: "body_test".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "csv_zip".into(),
        extracts: vec![],
        headers: vec![("user-agent".into(), "{TNS_UA}".into())],
        post_body: Some("api_key={TNS_API_KEY}".into()),
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let mut env = HashMap::new();
    env.insert("TNS_API_KEY".to_string(), "secret123".to_string());
    env.insert(
        "TNS_UA".to_string(),
        "tns_marker{\"tns_id\":1,\"type\":\"bot\",\"name\":\"probe\"}".to_string(),
    );
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    let body = render_source_body(
        &src,
        RenderCtx {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            tdb: 8.0e8,
            r: 0.0,
            eph: &HashMap::new(),
            lsk: &fixture_lsk,
        },
    )
    .map(|b| super::resolve_secret(&b, &env));
    assert_eq!(body.as_deref(), Some("api_key=secret123"));
    let headers = render_headers(&src.headers, &env);
    assert_eq!(
        headers[0].1,
        "tns_marker{\"tns_id\":1,\"type\":\"bot\",\"name\":\"probe\"}"
    );
}

#[test]
fn test_csv_to_json_tns_shape() {
    let csv = "2026-08-13 00:00:00 - 23:59:59\n\"objid\",\"ra\",\"declination\",\"redshift\",\"discoverymag\"\n\"1\",\"89.8\",\"53.6\",\"0.027\",\"19.8\"\n\"2\",\"35.0\",\"-24.4\",\"\",\"19.4\"\n";
    let j = csv_to_json(csv).unwrap();
    let arr = match j {
        JsonVal::Arr(a) => a,
        _ => panic!("expected array"),
    };
    assert_eq!(arr.len(), 2);
    match &arr[0] {
        JsonVal::Obj(m) => {
            assert_eq!(scalar_of(m.get("ra").unwrap()), Some(89.8));
            assert_eq!(scalar_of(m.get("redshift").unwrap()), Some(0.027));
        }
        _ => panic!("expected object"),
    }
    match &arr[1] {
        JsonVal::Obj(m) => {
            assert_eq!(scalar_of(m.get("redshift").unwrap()), None);
        }
        _ => panic!("expected object"),
    }
}

#[test]
fn test_csv_to_json_nul_separated() {
    let csv = "catID\0cluID\0dec_\0ra\n1\x0010.5\0-20.0\x00123.4\n2\x0011.5\0-21.0\x00124.5\n";
    let j = csv_to_json(csv).unwrap();
    let arr = match j {
        JsonVal::Arr(a) => a,
        _ => panic!("expected array"),
    };
    assert_eq!(arr.len(), 2);
    match &arr[0] {
        JsonVal::Obj(m) => {
            assert_eq!(scalar_of(m.get("ra").unwrap()), Some(123.4));
            assert_eq!(scalar_of(m.get("dec_").unwrap()), Some(-20.0));
        }
        _ => panic!("expected object"),
    }
}

#[test]
fn test_celestial_map_redshift_distance() {
    let src = SourceConfig {
        ttl: 100,
        url: "https://example.com/x".into(),
        frame: Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: "json".into(),
        extracts: vec![Extract::CelestialMap {
            arr_path: ".".into(),
            ra_key: "ra".into(),
            dec_key: "declination".into(),
            dist_key: String::new(),
            dist_scale: Some(1.0),
            plx_key: String::new(),
            z_key: "redshift".into(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: Some(1.0),
            epoch_key: String::new(),
            fields: vec![FieldConfig {
                key: "discoverymag".into(),
                name: "tns_transient_flux".into(),
                kernel: 0,
                force: 0,
                tau: 3600.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            tau_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: Some("tns_transient_flux".into()),
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let fixture_lsk = LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    let body = r#"[{"ra":89.8,"declination":53.6,"redshift":0.027,"discoverymag":19.8},{"ra":35.0,"declination":-24.4,"redshift":0.0,"discoverymag":19.4}]"#;
    match extract(&src, body, 8.0e8, &fixture_lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            assert_eq!(channels[0].1.name, "tns_transient_flux");
            assert!((channels[0].0.z - 0.027).abs() < 1e-9);
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
}

#[test]
fn test_extract_csv_zip_end_to_end() {
    let csv = "2026-08-13 00:00:00 - 23:59:59\n\"objid\",\"ra\",\"declination\",\"redshift\",\"discoverymag\"\n\"1\",\"89.8\",\"53.6\",\"0.027\",\"19.8\"\n\"2\",\"35.0\",\"-24.4\",\"\",\"19.4\"\n";
    let mut zip = Vec::new();
    zip.extend_from_slice(b"PK\x03\x04");
    zip.extend_from_slice(&20u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u32.to_le_bytes());
    zip.extend_from_slice(&(csv.len() as u32).to_le_bytes());
    zip.extend_from_slice(&(csv.len() as u32).to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(csv.as_bytes());
    let path = std::env::temp_dir().join("omegaflow_test_tns.zip");
    std::fs::write(&path, &zip).unwrap();
    let src = SourceConfig {
        ttl: 100,
        url: "https://example.com/x".into(),
        frame: Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: "csv_zip".into(),
        extracts: vec![Extract::CelestialMap {
            arr_path: ".".into(),
            ra_key: "ra".into(),
            dec_key: "declination".into(),
            dist_key: String::new(),
            dist_scale: Some(1.0),
            plx_key: String::new(),
            z_key: "redshift".into(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: Some(1.0),
            epoch_key: String::new(),
            fields: vec![FieldConfig {
                key: "discoverymag".into(),
                name: "tns_transient_flux".into(),
                kernel: 0,
                force: 0,
                tau: 3600.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            tau_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: Some("tns_transient_flux".into()),
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let fixture_lsk = LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match extract(&src, path.to_str().unwrap(), 8.0e8, &fixture_lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            assert_eq!(channels[0].1.name, "tns_transient_flux");
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_parse_sources_dist_scale() {
    let phi = "url https://example.com/comets.json\n\
ttl 604800\n\
at sun\n\
cmap .\n\
ra ra\n\
dec dec\n\
dist dist_au\n\
dist_scale 3.085677581e19\n\
field H comet_h_mag gaussian-inverse-square em mag 604800 0.0 0.0\n";
    let sources = parse_sources(phi);
    assert_eq!(sources.len(), 1);
    match &sources[0].extracts[0] {
        Extract::CelestialMap { dist_scale, .. } => {
            assert_eq!(*dist_scale, Some(3.085677581e19));
        }
        _ => panic!("expected CelestialMap extract"),
    }
}

#[test]
fn test_parse_sources_dist_without_scale_is_absent() {
    let phi = "url https://example.com/comets.json\n\
ttl 604800\n\
at sun\n\
cmap .\n\
ra ra\n\
dec dec\n\
dist dist_au\n";
    let sources = parse_sources(phi);
    assert_eq!(sources.len(), 1);
    match &sources[0].extracts[0] {
        Extract::CelestialMap { dist_scale, .. } => assert_eq!(*dist_scale, None),
        _ => panic!("expected CelestialMap extract"),
    }
}

#[test]
fn test_parse_sources_rv_scale_directive() {
    let phi = "url https://example.com/x.json\n\
ttl 604800\n\
at sun\n\
cmap .\n\
ra ra\n\
dec dec\n\
radvel rv\n\
rv_scale 1000.0\n";
    let sources = parse_sources(phi);
    assert_eq!(sources.len(), 1);
    match &sources[0].extracts[0] {
        Extract::CelestialMap { rv_scale, .. } => assert_eq!(*rv_scale, Some(1000.0)),
        _ => panic!("expected CelestialMap extract"),
    }
}

#[test]
fn test_parse_euclid_tap_block() {
    let phi = "url https://eas.esac.esa.int/tap-server/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY=SELECT+TOP+5000+right_ascension,declination,flux_detection_total+FROM+catalogue.mer_catalogue+WHERE+flux_detection_total+IS+NOT+NULL\n\
format tap\n\
ttl 604800\n\
at sun\n\
cmap .\n\
ra right_ascension\n\
dec declination\n\
field flux_detection_total euclid_detection_flux inverse-square em uJy 604800 0.0 0.0\n";
    let sources = parse_sources(phi);
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].format, "tap");
    match &sources[0].extracts[0] {
        Extract::CelestialMap {
            ra_key,
            dec_key,
            fields,
            ..
        } => {
            assert_eq!(ra_key, "right_ascension");
            assert_eq!(dec_key, "declination");
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].key, "flux_detection_total");
            assert_eq!(fields[0].name, "euclid_detection_flux");
            assert_eq!(fields[0].unit, "uJy");
            assert_eq!(fields[0].force, 0);
            assert_eq!(fields[0].kernel, 0);
        }
        _ => panic!("expected CelestialMap extract"),
    }
}

#[test]
fn test_parse_reference_seat_holds() {
    let phi = "url https://arxiv.org/e-print/2012.08534\n\
format reference\n\
sha256 bc86e4e424dbda6c3fb17a0f5141090c9ed59be2ad0665fddddce07d9a2085e7\n\
ttl 86400\n";
    let sources = parse_sources(phi);
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].format, "reference");
    assert!(matches!(sources[0].frame, Frame::Manifest));
    assert!(sources[0].extracts.is_empty());
    assert_eq!(
        sources[0].sha256.as_deref(),
        Some("bc86e4e424dbda6c3fb17a0f5141090c9ed59be2ad0665fddddce07d9a2085e7")
    );
}

#[test]
fn test_parse_fieldless_block_without_reference_stays_refused() {
    let phi = "url https://example.com/bare.dat\n\
ttl 86400\n";
    assert!(parse_sources(phi).is_empty());
}

#[test]
fn test_parse_map_lat_lon_without_frame_seats_manifest_source() {
    let phi = "url https://example.org/surface.json\n\
ttl 900\n\
map data\n\
lat lat\n\
lon lon\n";
    let sources = parse_sources(phi);
    assert_eq!(sources.len(), 1);
    assert!(matches!(sources[0].frame, Frame::Manifest));
    assert_eq!(sources[0].url, "https://example.org/surface.json");
    let Some(Extract::Map {
        arr_path,
        lat_key,
        lon_key,
        ..
    }) = sources[0].extracts.first()
    else {
        panic!("the seated source carries a Map extract");
    };
    assert_eq!(arr_path, "data");
    assert_eq!(lat_key, "lat");
    assert_eq!(lon_key, "lon");
}

#[test]
fn test_parse_reference_without_sha256_still_seats() {
    let phi = "url https://example.com/dataset.csv\n\
format reference\n\
ttl 86400\n";
    let sources = parse_sources(phi);
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].format, "reference");
    assert_eq!(sources[0].sha256, None);
}

#[test]
fn test_dead_grammar_refused() {
    let phi = "url https://example.com/dead.json\n\
ttl 60\n\
on earth 0.0 0.0 0.0\n\
cmap .\n\
force em\n\
field temp temp_c\n";
    let sources = parse_sources(phi);
    assert_eq!(sources.len(), 1);
    match &sources[0].extracts[0] {
        Extract::CelestialMap { fields, .. } => assert!(fields.is_empty()),
        _ => panic!("expected CelestialMap extract"),
    }
}

#[test]
fn test_extract_cmap_dist_scale_kpc() {
    let json = r#"[{"ra":0.0,"dec":0.0,"dist_kpc":1.0,"H":5.5}]"#;
    let src = SourceConfig {
        ttl: 604800,
        url: "https://example.com/x".into(),
        frame: Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: "json".into(),
        extracts: vec![Extract::CelestialMap {
            arr_path: ".".into(),
            ra_key: "ra".into(),
            dec_key: "dec".into(),
            dist_key: "dist_kpc".into(),
            dist_scale: Some(3.085677581e19),
            plx_key: String::new(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: Some(1.0),
            epoch_key: String::new(),
            fields: vec![FieldConfig {
                key: "H".into(),
                name: "comet_h_mag".into(),
                kernel: 0,
                force: 0,
                tau: 604800.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            tau_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let fixture_lsk = LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match extract(&src, json, 8.0e8, &fixture_lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            assert_eq!(channels[0].1.name, "comet_h_mag");
            assert_eq!(channels[0].0.value, 5.5);
            if let Position::StateVector { p, .. } = channels[0].0.position {
                let expect = 3.085677581e19;
                assert!((p[0] - expect).abs() / expect < 1e-12);
                assert!(p[1].abs() / expect < 1e-12);
                assert!(p[2].abs() / expect < 1e-12);
            } else {
                panic!("expected StateVector position");
            }
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
}

#[test]
fn test_extract_cmap_dist_without_scale_is_absent() {
    let json = r#"[{"ra":0.0,"dec":0.0,"dist_kpc":1.0,"H":5.5}]"#;
    let src = SourceConfig {
        ttl: 604800,
        url: "https://example.com/x".into(),
        frame: Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: "json".into(),
        extracts: vec![Extract::CelestialMap {
            arr_path: ".".into(),
            ra_key: "ra".into(),
            dec_key: "dec".into(),
            dist_key: "dist_kpc".into(),
            dist_scale: None,
            plx_key: String::new(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: Some(1.0),
            epoch_key: String::new(),
            fields: vec![FieldConfig {
                key: "H".into(),
                name: "comet_h_mag".into(),
                kernel: 0,
                force: 0,
                tau: 604800.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            tau_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let fixture_lsk = LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match extract(&src, json, 8.0e8, &fixture_lsk) {
        ExtractResult::Measurements(channels) => {
            assert!(
                channels.is_empty(),
                "dist without a measured scale is absent"
            );
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
}

#[test]
fn test_extract_cmap_rv_without_scale_is_absent() {
    let json = r#"[{"ra":0.0,"dec":0.0,"plx":100.0,"rv":50.0,"H":5.5}]"#;
    let src = SourceConfig {
        ttl: 604800,
        url: "https://example.com/x".into(),
        frame: Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: "json".into(),
        extracts: vec![Extract::CelestialMap {
            arr_path: ".".into(),
            ra_key: "ra".into(),
            dec_key: "dec".into(),
            dist_key: String::new(),
            dist_scale: None,
            plx_key: "plx".into(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: "rv".into(),
            rv_scale: None,
            epoch_key: String::new(),
            fields: vec![FieldConfig {
                key: "H".into(),
                name: "comet_h_mag".into(),
                kernel: 0,
                force: 0,
                tau: 604800.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            tau_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let fixture_lsk = LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match extract(&src, json, 8.0e8, &fixture_lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            if let Position::StateVector { v, .. } = channels[0].0.position {
                assert!(
                    v[0].abs() < 1e-9,
                    "radial velocity without a scale is absent"
                );
                assert!(v[1].abs() < 1e-9);
                assert!(v[2].abs() < 1e-9);
            } else {
                panic!("expected StateVector position");
            }
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
}

#[test]
fn test_extract_cmap_pm_radvel_plx() {
    let json =
        r#"[{"ra":0.0,"dec":0.0,"plx":100.0,"pmra":1000.0,"pmdec":2000.0,"rv":50.0,"H":5.5}]"#;
    let src = SourceConfig {
        ttl: 604800,
        url: "https://example.com/x".into(),
        frame: Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: "json".into(),
        extracts: vec![Extract::CelestialMap {
            arr_path: ".".into(),
            ra_key: "ra".into(),
            dec_key: "dec".into(),
            dist_key: String::new(),
            dist_scale: Some(1.0),
            plx_key: "plx".into(),
            z_key: String::new(),
            pmra_key: "pmra".into(),
            pmdec_key: "pmdec".into(),
            rv_key: "rv".into(),
            rv_scale: Some(1.0),
            epoch_key: String::new(),
            fields: vec![FieldConfig {
                key: "H".into(),
                name: "comet_h_mag".into(),
                kernel: 0,
                force: 0,
                tau: 604800.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            tau_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let fixture_lsk = LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match extract(&src, json, 8.0e8, &fixture_lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            if let Position::StateVector { p, v, .. } = channels[0].0.position {
                let d = super::PARSEC_M * 1000.0 / 100.0;
                assert!((p[0] - d).abs() / d < 1e-12);
                assert!(p[1].abs() < 1.0);
                assert!(p[2].abs() < 1.0);
                let mu_a = 1000.0 * super::MAS_YR_TO_RAD_S;
                let mu_d = 2000.0 * super::MAS_YR_TO_RAD_S;
                let expect_v = [50.0, d * mu_a, d * mu_d];
                assert!((v[0] - expect_v[0]).abs() < 1e-6);
                assert!((v[1] - expect_v[1]).abs() < 1e-6);
                assert!((v[2] - expect_v[2]).abs() < 1e-6);
            } else {
                panic!("expected StateVector position");
            }
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
}

#[test]
fn test_empty_data_anomaly() {
    let _gate = ANOMALY_TEST_GATE.lock();
    ANOMALY_COLLECT.with(|c| c.set(true));
    let sources = parse_sources(
        "url https://example.org/e\nttl 3600\nformat json\nat earth\nmap features\nlat lat\nlon lon\nfield magnitude mag gaussian-inverse-square em mag 3600 0 0\n",
    );
    assert_eq!(sources.len(), 1);
    let src = &sources[0];
    let lsk = full_fixture_lsk();
    let _ = take_anomalies();
    check_empty_data(src, r#"{"features":[]}"#, 0.0, &lsk);
    let anomalies = take_anomalies();
    assert!(
        anomalies
            .iter()
            .any(|a| a.category == "Empty Data" && a.url == "https://example.org/e")
    );
    check_empty_data(
        src,
        r#"{"features":[{"lat":10.0,"lon":20.0,"magnitude":5.0}]}"#,
        0.0,
        &lsk,
    );
    let anomalies = take_anomalies();
    assert!(!anomalies.iter().any(|a| a.category == "Empty Data"));
    ANOMALY_COLLECT.with(|c| c.set(false));
}

#[test]
fn test_extract_cmap_no_distance_skipped() {
    let json = r#"[{"ra":0.0,"dec":0.0,"H":5.5}]"#;
    let src = SourceConfig {
        ttl: 604800,
        url: "https://example.com/x".into(),
        frame: Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: "json".into(),
        extracts: vec![Extract::CelestialMap {
            arr_path: ".".into(),
            ra_key: "ra".into(),
            dec_key: "dec".into(),
            dist_key: String::new(),
            dist_scale: Some(1.0),
            plx_key: String::new(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: Some(1.0),
            epoch_key: String::new(),
            fields: vec![FieldConfig {
                key: "H".into(),
                name: "comet_h_mag".into(),
                kernel: 0,
                force: 0,
                tau: 604800.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            tau_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let fixture_lsk = LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match extract(&src, json, 8.0e8, &fixture_lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 0);
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
}

#[test]
fn test_extract_cmap_null_dist_skipped() {
    let json = r#"[{"ra":0.0,"dec":0.0,"dist_pc":null,"H":5.5}]"#;
    let src = SourceConfig {
        ttl: 604800,
        url: "https://example.com/x".into(),
        frame: Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: "json".into(),
        extracts: vec![Extract::CelestialMap {
            arr_path: ".".into(),
            ra_key: "ra".into(),
            dec_key: "dec".into(),
            dist_key: "dist_pc".into(),
            dist_scale: Some(3.085677581e16),
            plx_key: String::new(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: Some(1.0),
            epoch_key: String::new(),
            fields: vec![FieldConfig {
                key: "H".into(),
                name: "comet_h_mag".into(),
                kernel: 0,
                force: 0,
                tau: 604800.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            tau_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let fixture_lsk = LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match extract(&src, json, 8.0e8, &fixture_lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 0);
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
}

#[test]
fn test_extract_cmap_csv_dist_scale_mpc() {
    let csv = "AGCNr,Name,RAdeg_HI,Decdeg_HI,RAdeg_OC,DECdeg_OC,Vhelio,W50,errW50,HIflux,errflux,SNR,RMS,Dist,logMsun,HIcode,OCcode,NoteFlag\n\
331061,456-013,0.01042,15.87222,0.00875,15.88167,6007,260,45,1.13,0.09,6.5,2.40,85.2,9.29,1,I,\"\"\n\
331405,\"\",0.01375,26.01639,0.01458,26.01389,10409,315,8,2.62,0.09,16.1,2.05,143.8,10.11,1,I,\"\"\n";
    let src = SourceConfig {
        ttl: 604800,
        url: "https://example.com/x".into(),
        frame: Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: "csv".into(),
        extracts: vec![Extract::CelestialMap {
            arr_path: ".".into(),
            ra_key: "RAdeg_HI".into(),
            dec_key: "Decdeg_HI".into(),
            dist_key: "Dist".into(),
            dist_scale: Some(3.085677581e22),
            plx_key: String::new(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: Some(1.0),
            epoch_key: String::new(),
            fields: vec![FieldConfig {
                key: "HIflux".into(),
                name: "alfalfa_hi_flux".into(),
                kernel: 0,
                force: 0,
                tau: 604800.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            tau_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let fixture_lsk = LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match extract(&src, csv, 8.0e8, &fixture_lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 2);
            assert_eq!(channels[0].1.name, "alfalfa_hi_flux");
            assert_eq!(channels[0].0.value, 1.13);
            assert_eq!(channels[1].0.value, 2.62);
            if let Position::StateVector { p, .. } = channels[0].0.position {
                let expect = 85.2 * 3.085677581e22;
                let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
                assert!((r - expect).abs() / expect < 1e-12);
            } else {
                panic!("expected StateVector position");
            }
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
}

#[test]
fn test_star_samples_build_tau() {
    let mut bin = Vec::new();
    bin.extend_from_slice(&0f64.to_le_bytes());
    bin.extend_from_slice(&0f64.to_le_bytes());
    bin.extend_from_slice(&0f32.to_le_bytes());
    bin.extend_from_slice(&0f32.to_le_bytes());
    bin.extend_from_slice(&100f32.to_le_bytes());
    bin.extend_from_slice(&0f32.to_le_bytes());
    bin.extend_from_slice(&1f32.to_le_bytes());
    bin.extend_from_slice(&1.2f32.to_le_bytes());
    bin.extend_from_slice(&30000f32.to_le_bytes());
    let samples = build_star_samples(&bin);
    assert_eq!(samples.len(), 1);
    assert!(samples[0].tau > 0.0);
    assert_eq!(samples[0].val, 1.0);
    assert_eq!(samples[0].force_type, 0.0);
    assert_eq!(samples[0].kernel_id, 0.0);
    assert_eq!(samples[0].ttl, samples[0].tau);
    assert_eq!(samples[0].epoch, 0.0);
    assert!(samples[0].extent.is_infinite());
    assert!((samples[0].color_index - 1.2).abs() < 1e-4);
    let Motion::Spherical { rec } = &samples[0].motion else {
        panic!("spherical motion");
    };
    assert!((rec.plx_mas - 100.0).abs() < 1e-6);
    assert!((rec.rv_m_s - 30000.0).abs() < 1e-4);
    assert!((rec.color_index - 1.2).abs() < 1e-4);
    let (p, _) = star_position_at(rec, 0.0);
    let d = 10.0 * PARSEC_M;
    assert!((p[0] - d).abs() / d < 1e-9);
    assert!((samples[0].anchor_p0[0] - d).abs() / d < 1e-9);
    let short = [0u8; 36];
    assert!(parse_star_record(&short).is_none());
    let legacy = [0u8; 40];
    assert!(parse_star_record(&legacy).is_none());
    assert_eq!(build_star_samples(&bin[..40]).len(), 0);
}

#[test]
fn test_star_record_rejects_non_finite_color() {
    let mut bin = Vec::new();
    bin.extend_from_slice(&0f64.to_le_bytes());
    bin.extend_from_slice(&0f64.to_le_bytes());
    bin.extend_from_slice(&0f32.to_le_bytes());
    bin.extend_from_slice(&0f32.to_le_bytes());
    bin.extend_from_slice(&100f32.to_le_bytes());
    bin.extend_from_slice(&1f32.to_le_bytes());
    bin.extend_from_slice(&1f32.to_le_bytes());
    bin.extend_from_slice(&f32::NAN.to_le_bytes());
    bin.extend_from_slice(&0f32.to_le_bytes());
    assert!(parse_star_record(&bin).is_none());
    bin[36..40].copy_from_slice(&f32::INFINITY.to_le_bytes());
    assert!(parse_star_record(&bin).is_none());
    bin[36..40].copy_from_slice(&1.2f32.to_le_bytes());
    let rec = parse_star_record(&bin).unwrap();
    assert!((rec.color_index - 1.2).abs() < 1e-6);
}

#[test]
fn test_star_samples_diode() {
    let mut bin = Vec::new();
    bin.extend_from_slice(&0f64.to_le_bytes());
    bin.extend_from_slice(&0f64.to_le_bytes());
    bin.extend_from_slice(&0f32.to_le_bytes());
    bin.extend_from_slice(&0f32.to_le_bytes());
    bin.extend_from_slice(&100f32.to_le_bytes());
    bin.extend_from_slice(&0f32.to_le_bytes());
    bin.extend_from_slice(&1f32.to_le_bytes());
    bin.extend_from_slice(&1.2f32.to_le_bytes());
    bin.extend_from_slice(&12000f32.to_le_bytes());
    let samples = build_star_samples(&bin);
    assert_eq!(samples.len(), 1);
    let d = 10.0 * PARSEC_M;
    assert!((samples[0].anchor_p0[0] - d).abs() / d < 1e-9);
    let eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let buf = build_buffer(
        samples.into_iter().map(Arc::new).collect(),
        1.0,
        Arc::new(eph.clone()),
        None,
        Vec::new(),
        Vec::new(),
        None,
    );
    let query = |floor: [f64; 9], forward: [f64; 3]| {
        let mut out: Vec<SampleRecord> = Vec::new();
        query_hash(
            &buf.cache,
            MembraneCtx {
                center: [0.0, 0.0, 0.0],
                t2: 0.0,
                pad: 1.0,
                delta_t_cache: 0.0,
                floor: &floor,
                softening: 1.0,
                forward,
                eph: &eph,
            },
            &mut out,
        );
        out
    };
    let dark = query([0.0; 9], [1.0, 0.0, 0.0]);
    assert_eq!(dark.len(), 0);
    let loud = query([1e9; 9], [1.0, 0.0, 0.0]);
    assert_eq!(loud.len(), 0);
    let off_axis = query([0.5; 9], [0.0, 0.0, 1.0]);
    assert_eq!(off_axis.len(), 0);
    let on_axis = query([0.5; 9], [1.0, 0.0, 0.0]);
    assert_eq!(on_axis.len(), 1);
    assert!((on_axis[0].0 - d).abs() / d < 1e-9);
    assert_eq!(on_axis[0].3, 1.0);
    assert_eq!(on_axis[0].7, 0.0);
    assert_eq!(on_axis[0].8, 0.0);
    assert_eq!(on_axis[0].9, 0.0);
    assert!((on_axis[0].21 - 1.2).abs() < 1e-4);
}

fn kepler_rec_fixture() -> AsteroidRec {
    AsteroidRec {
        number: 1,
        epoch_jd: J2000_EPOCH,
        a_au: 1.0,
        e: 0.0,
        incl_deg: 0.0,
        node_deg: 0.0,
        peri_deg: 0.0,
        ma_deg: 0.0,
        h: 0.0,
        g: 0.0,
        albedo: 0.0,
        rot_period_h: 0.0,
        radius_km: 0.0,
        gm_km3_s2: 0.0,
        sptype: [0u8; 5],
    }
}

#[test]
fn test_motion_kepler_at_anchor_body_and_law_bounds() {
    let au_m = crate::kepler::AU_M;
    let gm_sun = crate::kepler::GM_SUN_M3_S2;
    let rec = kepler_rec_fixture();
    let motion = Motion::Kepler {
        rec: Arc::new(rec.clone()),
    };
    assert!(motion.anchor_body().is_none());
    let eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let p0 = motion.at(0.0, 0.0, &eph).expect("kepler position at epoch");
    let r0 = (p0[0] * p0[0] + p0[1] * p0[1] + p0[2] * p0[2]).sqrt();
    assert!((r0 - au_m).abs() / au_m < 1e-12);
    let p_dt = motion
        .at(1e-1, 0.0, &eph)
        .expect("kepler position at epoch+dt");
    let v_fd = [
        (p_dt[0] - p0[0]) / 1e-1,
        (p_dt[1] - p0[1]) / 1e-1,
        (p_dt[2] - p0[2]) / 1e-1,
    ];
    let speed = (v_fd[0] * v_fd[0] + v_fd[1] * v_fd[1] + v_fd[2] * v_fd[2]).sqrt();
    let v_circ = (gm_sun / au_m).sqrt();
    assert!((speed - v_circ).abs() / v_circ < 1e-3);
    let (vmax, amax, p_anchor) = law_bounds(&motion, 0.0, 0.0, &eph).expect("kepler law bounds");
    assert!((p_anchor[0] - au_m).abs() / au_m < 1e-12);
    assert!((vmax / Φ - v_circ).abs() / v_circ < 1e-4);
    assert!(amax > 0.0 && amax.is_finite());
    let mut unbound = rec;
    unbound.e = 1.5;
    assert!(
        Motion::Kepler {
            rec: Arc::new(unbound)
        }
        .at(0.0, 0.0, &eph)
        .is_none()
    );
}

#[test]
fn test_build_asteroid_samples_gm_radius_and_query() {
    let mut bin: Vec<u8> = Vec::new();
    let mut with_radius = kepler_rec_fixture();
    with_radius.number = 1;
    with_radius.gm_km3_s2 = 0.5;
    with_radius.radius_km = 3.0;
    crate::dastcom::encode_record(&with_radius, &mut bin);
    let mut far = kepler_rec_fixture();
    far.number = 2;
    far.a_au = 2.0;
    far.gm_km3_s2 = 0.25;
    crate::dastcom::encode_record(&far, &mut bin);
    let mut unbound = kepler_rec_fixture();
    unbound.number = 3;
    unbound.e = 1.5;
    unbound.gm_km3_s2 = 0.5;
    crate::dastcom::encode_record(&unbound, &mut bin);

    let samples = build_asteroid_samples(&bin, 86400);
    assert_eq!(samples.len(), 3);
    let gm = &samples[0];
    let radius = &samples[1];
    let far_gm = &samples[2];
    assert_eq!(gm.name, "dastcom.mass");
    assert_eq!(radius.name, "dastcom.radius");
    assert_eq!(gm.val, 5.0e8);
    assert_eq!(radius.val, 3000.0);
    assert_eq!(far_gm.val, 2.5e8);
    assert_eq!(gm.kernel_id, 0.0);
    assert_eq!(radius.kernel_id, 1.0);
    assert_eq!(gm.force_type, 1.0);
    assert_eq!(radius.force_type, 1.0);
    assert!(gm.extent == 3000.0 && gm.tau.is_infinite());
    assert!(radius.extent == 3000.0 && radius.tau.is_infinite());
    let Motion::Kepler { rec: rec_gm } = &gm.motion else {
        panic!("kepler motion");
    };
    let Motion::Kepler { rec: rec_radius } = &radius.motion else {
        panic!("kepler motion");
    };
    assert!(Arc::ptr_eq(rec_gm, rec_radius));
    let au_m = crate::kepler::AU_M;
    assert!((gm.anchor_p0[0] - au_m).abs() / au_m < 1e-9);
    let anchor_p0 = gm.anchor_p0;

    let eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let buf = build_buffer(
        samples.into_iter().map(Arc::new).collect(),
        1.0,
        Arc::new(eph.clone()),
        None,
        Vec::new(),
        Vec::new(),
        None,
    );
    let mut records: Vec<SampleRecord> = Vec::new();
    query_hash(
        &buf.cache,
        MembraneCtx {
            center: anchor_p0,
            t2: 0.0,
            pad: 1.0,
            delta_t_cache: 0.0,
            floor: &[0.0; 9],
            softening: 1.0,
            forward: [1.0, 0.0, 0.0],
            eph: &eph,
        },
        &mut records,
    );
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].3, 5.0e8);
    assert_eq!(records[1].3, 3000.0);
    assert_eq!(records[0].9, 1.0);
}

#[test]
fn test_motion_spherical_at_anchor_body_and_law_bounds() {
    let rec = StarRec {
        ra_deg: 0.0,
        dec_deg: 0.0,
        pm_ra_masyr: 1000.0,
        pm_de_masyr: 0.0,
        plx_mas: 100.0,
        flux: 1.0,
        mag: 0.0,
        tau: 0.0,
        color_index: 0.0,
        rv_m_s: 0.0,
    };
    let motion = Motion::Spherical {
        rec: Arc::new(rec.clone()),
    };
    assert!(motion.anchor_body().is_none());
    let eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let t_yr = 86400.0 * 365.25;
    let p = motion.at(t_yr, 0.0, &eph).expect("spherical position");
    let (p_ref, v_ref) = star_position_at(&rec, t_yr);
    let d = 10.0 * PARSEC_M;
    for k in 0..3 {
        assert!((p[k] - p_ref[k]).abs() < 1e-9 * d);
    }
    let (vmax, amax, p0) = law_bounds(&motion, 0.0, 0.0, &eph).expect("spherical law bounds");
    assert!((p0[0] - d).abs() / d < 1e-9);
    let speed = (v_ref[0] * v_ref[0] + v_ref[1] * v_ref[1] + v_ref[2] * v_ref[2]).sqrt();
    assert!((vmax / Φ - speed).abs() / speed < 1e-2);
    assert!(amax.is_finite());
}

#[test]
fn test_source_name_flat_and_collision_overrides() {
    let q1 = source_name_from_url("https://h/api?station=1");
    let q2 = source_name_from_url("https://h/api?station=2");
    assert_ne!(q1, q2);
    let again = source_name_from_url("https://h/api?station=1");
    assert_eq!(q1, again);
    let buoy1 = source_name_from_url("https://www.ndbc.noaa.gov/data/realtime/41009.txt");
    let buoy2 = source_name_from_url("https://www.ndbc.noaa.gov/data/realtime/41010.txt");
    assert_ne!(buoy1, buoy2);
    let slash = source_name_from_url("https://h/api/a/b");
    let dash = source_name_from_url("https://h/api/a-b");
    assert_eq!(slash, dash);
    let map = cdn_manifest_for(
        ["https://h/api/a/b", "https://h/api/a-b"]
            .into_iter()
            .map(|s| s.to_string()),
    );
    assert_eq!(
        map.get("https://h/api/a-b").unwrap(),
        &format!("{}-2", slash)
    );
    assert!(!map.contains_key("https://h/api/a/b"));
}

#[test]
fn test_render_headers_secret_substitution() {
    let mut env = HashMap::new();
    env.insert("PURPLEAIR_KEY".to_string(), "secret123".to_string());
    let headers = vec![
        ("X-API-Key".to_string(), "{PURPLEAIR_KEY}".to_string()),
        ("User-Agent".to_string(), "plain".to_string()),
    ];
    let rendered = render_headers(&headers, &env);
    assert_eq!(rendered[0].1, "secret123");
    assert_eq!(rendered[1].1, "plain");
}

#[test]
fn test_parse_station_entries() {
    let j = parse_json(
        r#"{"results":[{"id":"GHCND:AA1","latitude":17.1,"longitude":-61.8,"elevation":10.0},{"id":"GHCND:BB2","latitude":40.9,"longitude":-74.0},{"id":7,"latitude":52.5,"longitude":13.4}]}"#,
    )
    .unwrap();
    let src = SourceConfig {
        ttl: 300,
        url: "https://example.com/x".into(),
        frame: Frame::Surface {
            body_name: "earth".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "json".into(),
        extracts: vec![],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: "results".into(),
        stations_lat: "latitude".into(),
        stations_lon: "longitude".into(),
        stations_id: "id".into(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let stations = parse_station_entries(&j, &src);
    assert_eq!(stations.len(), 3);
    assert_eq!(stations[0].id, "GHCND:AA1");
    assert_eq!(stations[0].lat, 17.1);
    assert_eq!(stations[0].lon, -61.8);
    assert_eq!(stations[2].id, "7");
    assert_eq!(stations[2].lat, 52.5);
}

#[test]
fn test_parse_station_entries_flatten_filter() {
    let j = parse_json(
        r#"{"results":[{"coordinates":{"latitude":40.8,"longitude":-73.9},"sensors":[{"id":671,"parameter":{"name":"o3"}},{"id":673,"parameter":{"name":"pm25"}}]},{"coordinates":{"latitude":40.9,"longitude":-74.0},"sensors":[{"id":1097,"parameter":{"name":"pm25"}}]}]}"#,
    )
    .unwrap();
    let src = SourceConfig {
        ttl: 300,
        url: "https://example.com/x".into(),
        frame: Frame::Surface {
            body_name: "earth".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "json".into(),
        extracts: vec![],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: "results".into(),
        stations_lat: "coordinates.latitude".into(),
        stations_lon: "coordinates.longitude".into(),
        stations_id: "id".into(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: "sensors".into(),
        stations_filter: Some(("parameter.name".into(), "pm25".into())),
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let stations = parse_station_entries(&j, &src);
    assert_eq!(stations.len(), 2);
    assert_eq!(stations[0].id, "673");
    assert_eq!(stations[0].lat, 40.8);
    assert_eq!(stations[0].lon, -73.9);
    assert_eq!(stations[1].id, "1097");
    assert_eq!(stations[1].lat, 40.9);
    assert_eq!(stations[1].lon, -74.0);
}

#[test]
fn temp_port_convert_check() {
    let Ok(content) = std::fs::read_to_string("phi/pipeline/queue/master.φ") else {
        return;
    };
    let mut blocks = 0usize;
    let mut parsed = 0usize;
    let mut with_extracts = 0usize;
    let mut block = String::new();
    for line in content.lines() {
        let t = line.trim_start();
        if (t.starts_with("url ") || t.starts_with("source ")) && !block.is_empty() {
            blocks += 1;
            let conv = super::port_block(&block);
            let srcs = super::parse_sources(&conv);
            if !srcs.is_empty() {
                parsed += 1;
                with_extracts += srcs.iter().filter(|s| !s.extracts.is_empty()).count();
            }
            block = String::new();
        }
        block.push_str(line);
        block.push('\n');
    }
    eprintln!(
        "port convert: {} blocks, {} parsed, {} with extracts",
        blocks, parsed, with_extracts
    );
}

#[test]
fn test_profile_map_parse() {
    let block = "url https://argovis-api.colorado.edu/argo?data=temperature,salinity,pressure\nttl 86400\non earth 0 0 0\nprofile .\nlat geolocation.coordinates.1\nlon geolocation.coordinates.0\nepoch timestamp\npressure pressure\nfield temperature argo_temperature_c erfc thermal C 86400 0.0 0.0\nfield salinity argo_salinity_psu erfc diffusion psu 86400 0.0 0.0\n";
    let srcs = super::parse_sources(block);
    assert_eq!(srcs.len(), 1);
    let prof = srcs[0].extracts.iter().find_map(|e| match e {
        super::Extract::ProfileMap {
            pressure_var,
            fields,
            lat_key,
            lon_key,
            ..
        } => Some((
            pressure_var.clone(),
            fields,
            lat_key.clone(),
            lon_key.clone(),
        )),
        _ => None,
    });
    let (pv, fields, lk, ok) = prof.expect("ProfileMap extract absent");
    assert_eq!(pv, "pressure");
    assert_eq!(lk, "geolocation.coordinates.1");
    assert_eq!(ok, "geolocation.coordinates.0");
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].key, "temperature");
    assert_eq!(fields[1].key, "salinity");
}

#[test]
fn test_netcdf_grammar_alt_decibar() {
    let block = "url https://data-argo.ifremer.fr/dac/aoml/1901843/profiles/R1901843_357.nc\nttl 604800\non earth 0 0 0\nformat netcdf\nprofile .\nlat LATITUDE\nlon LONGITUDE\nepoch JULD\nalt PRES decibar\nfield TEMP argo_dac_temp_c erfc thermal C 604800 0.0 0.0\nfield PSAL argo_dac_salinity_psu erfc diffusion psu 604800 0.0 0.0\n";
    let srcs = super::parse_sources(block);
    assert_eq!(srcs.len(), 1);
    assert_eq!(srcs[0].format, "netcdf");
    let prof = srcs[0].extracts.iter().find_map(|e| match e {
        super::Extract::ProfileMap {
            pressure_var,
            pressure_scale,
            fields,
            lat_key,
            lon_key,
            epoch_key,
            ..
        } => Some((
            pressure_var.clone(),
            *pressure_scale,
            fields,
            lat_key.clone(),
            lon_key.clone(),
            epoch_key.clone(),
        )),
        _ => None,
    });
    let (pv, ps, fields, lk, ok, ek) = prof.expect("ProfileMap extract absent");
    assert_eq!(pv, "PRES");
    assert_eq!(ps, 1.0);
    assert_eq!(lk, "LATITUDE");
    assert_eq!(ok, "LONGITUDE");
    assert_eq!(ek, "JULD");
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].key, "TEMP");
    assert_eq!(fields[1].key, "PSAL");
}

#[test]
fn test_build_netcdf_channels() {
    use std::collections::HashMap;
    let block = "url https://data-argo.ifremer.fr/dac/aoml/1901843/profiles/R1901843_357.nc\nttl 604800\non earth 0 0 0\nformat netcdf\nprofile .\nlat LATITUDE\nlon LONGITUDE\nepoch JULD\nalt PRES decibar\nfield TEMP argo_dac_temp_c erfc thermal C 604800 0.0 0.0\nfield PSAL argo_dac_salinity_psu erfc diffusion psu 604800 0.0 0.0\n";
    let srcs = super::parse_sources(block);
    let u32b = |x: u32| x.to_be_bytes().to_vec();
    let f64b = |x: f64| x.to_bits().to_be_bytes().to_vec();
    let f32b = |x: f32| x.to_bits().to_be_bytes().to_vec();
    let name = |s: &str| {
        let mut b = u32b(s.len() as u32);
        b.extend_from_slice(s.as_bytes());
        while b.len() % 4 != 0 {
            b.push(0);
        }
        b
    };
    let mut b = Vec::new();
    b.extend([0x43, 0x44, 0x46, 0x01]);
    b.extend(u32b(0));
    b.extend(u32b(0x0A));
    b.extend(u32b(2));
    b.extend(name("N_PROF"));
    b.extend(u32b(1));
    b.extend(name("N_LEVELS"));
    b.extend(u32b(3));
    b.extend(u32b(0));
    b.extend(u32b(0));
    b.extend(u32b(0x0B));
    b.extend(u32b(6));
    let var = |b: &mut Vec<u8>,
               nm: &str,
               rank: u32,
               dims: &[u32],
               fill: Option<f32>,
               t: u32,
               vsize: u32|
     -> usize {
        b.extend(name(nm));
        b.extend(u32b(rank));
        for &d in dims {
            b.extend(u32b(d));
        }
        match fill {
            Some(fv) => {
                b.extend(u32b(0x0C));
                b.extend(u32b(1));
                b.extend(name("_FillValue"));
                b.extend(u32b(5));
                b.extend(u32b(1));
                b.extend(f32b(fv));
            }
            None => {
                b.extend(u32b(0));
                b.extend(u32b(0));
            }
        }
        b.extend(u32b(t));
        b.extend(u32b(vsize));
        let slot = b.len();
        b.extend(u32b(0));
        slot
    };
    let slots = [
        var(&mut b, "LATITUDE", 1, &[0], None, 6, 8),
        var(&mut b, "LONGITUDE", 1, &[0], None, 6, 8),
        var(&mut b, "JULD", 1, &[0], None, 6, 8),
        var(&mut b, "PRES", 2, &[0, 1], Some(99999.0), 5, 4),
        var(&mut b, "TEMP", 2, &[0, 1], Some(99999.0), 5, 4),
        var(&mut b, "PSAL", 2, &[0, 1], Some(99999.0), 5, 4),
    ];
    let data_start = b.len() as u64;
    let begins = [
        data_start,
        data_start + 8,
        data_start + 16,
        data_start + 24,
        data_start + 36,
        data_start + 48,
    ];
    for (slot, beg) in slots.iter().zip(begins.iter()) {
        b[*slot..*slot + 4].copy_from_slice(&(*beg as u32).to_be_bytes());
    }
    b.extend(f64b(-15.77751));
    b.extend(f64b(57.37286));
    b.extend(f64b(27965.773125022904));
    for p in [1.08f32, 2.0, 99999.0] {
        b.extend(f32b(p));
    }
    for t in [25.862f32, f32::NAN, 10.0] {
        b.extend(f32b(t));
    }
    for s in [35.0314f32, 35.0, 34.9] {
        b.extend(f32b(s));
    }
    let lsk = full_fixture_lsk();
    let expected_epoch = lsk
        .unix_to_tdb((27965.773125022904f64 - 7305.0) * 86400.0)
        .unwrap();
    let mut earth_cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
    earth_cx[0] = 1.5e9;
    let earth_props = super::BodyProperties {
        α0_deg: 0.0,
        dα0_dt_deg_per_century: 0.0,
        δ0_deg: 90.0,
        dδ0_dt_deg_per_century: 0.0,
        w0_deg: 190.147,
        dw_dt_deg_per_day: 360.9856235,
        radius_m: 6378136.6,
        flattening: Some(0.0033528131084554157),
        gaussian_inverse_square: 0.0,
        gaussian_inverse: 0.0,
        erfc: 0.0,
        patch_levy: 0.0,
        exponential_decay: 0.0,
        gm: None,
        j2: None,
        j4: None,
        radii_b: None,
        radii_c: None,
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let earth_eph = super::BodyEphemeris {
        granules: vec![super::ChebyshevGranule {
            t0_jd: super::J2000_EPOCH + expected_epoch / 86400.0,
            dt_jd: 1.0,
            cx: earth_cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        }],
        rotation_matrices: vec![],
        props: Some(earth_props),
        orbit: None,
        granule_hint: std::sync::atomic::AtomicUsize::new(0).into(),
    };
    let mut eph_map = HashMap::new();
    eph_map.insert("earth".to_string(), earth_eph);
    let presence_p = super::body_fixed_to_icrs(
        "earth",
        -15.77751,
        57.37286,
        -1.08,
        expected_epoch,
        &eph_map,
    )
    .expect("the earth fixture resolves the profile position");
    let presences: Vec<PresenceSample> = vec![(
        expected_epoch,
        presence_p[0],
        presence_p[1],
        presence_p[2],
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
    )];
    let channels = super::build_netcdf_channels(
        &srcs[0],
        &b,
        &lsk,
        expected_epoch,
        &presences,
        Some(6378136.6),
        &eph_map,
    );
    assert_eq!(channels.len(), 3);
    assert_eq!(channels[0].0.name, "argo_dac_temp_c");
    assert_eq!(channels[1].0.name, "argo_dac_salinity_psu");
    assert_eq!(channels[2].0.name, "argo_dac_salinity_psu");
    assert!((channels[0].0.value - 25.862).abs() < 1e-3);
    assert!((channels[1].0.value - 35.0314).abs() < 1e-3);
    assert!((channels[2].0.value - 35.0).abs() < 1e-6);
    assert!((channels[0].0.epoch - expected_epoch).abs() < 1e-6);
    assert!((channels[1].0.epoch - expected_epoch).abs() < 1e-6);
    assert!((channels[2].0.epoch - expected_epoch).abs() < 1e-6);
    let alts: Vec<f64> = channels
        .iter()
        .map(|(c, _)| match &c.position {
            super::Position::Surface { alt, lat, lon, .. } => {
                assert!((*lat + 15.77751).abs() < 1e-4);
                assert!((*lon - 57.37286).abs() < 1e-4);
                *alt
            }
            _ => panic!("position is not Surface"),
        })
        .collect();
    assert!((alts[0] + 1.08).abs() < 1e-2);
    assert!((alts[1] + 1.08).abs() < 1e-2);
    assert!((alts[2] + 2.0).abs() < 1e-2);
}

#[test]
fn test_load_gate_clips_records_outside_enclosure() {
    let fc = FieldConfig {
        key: "em".into(),
        name: "em".into(),
        kernel: 0,
        force: 0,
        tau: 60.0,
        absorption: 0.0,
        advection: 0.0,
        unit: String::new(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
    };
    let now = 8.0e8;
    let near: Vec<PresenceSample> = vec![(now, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0)];
    assert!(
        super::record_in_enclosure(
            &near,
            Some([1.0, 0.0, 0.0]),
            now,
            now,
            EnclosureField {
                config: &fc,
                body_props: None,
                body_radius: None,
            },
            AnchorEnvelope {
                vmax: 0.0,
                amax: 0.0,
                pad: 0.0,
                ttl: 60.0,
            },
        ),
        "a record inside the dilated enclosure materializes"
    );
    assert!(
        !super::record_in_enclosure(
            &near,
            Some([1.0e6, 0.0, 0.0]),
            now,
            now,
            EnclosureField {
                config: &fc,
                body_props: None,
                body_radius: None,
            },
            AnchorEnvelope {
                vmax: 0.0,
                amax: 0.0,
                pad: 0.0,
                ttl: 60.0,
            },
        ),
        "a record beyond reach_signal + extent + rho stays raw"
    );
    let no_law = FieldConfig {
        force: 9,
        ..fc.clone()
    };
    assert!(
        super::record_in_enclosure(
            &near,
            Some([1.0e12, 0.0, 0.0]),
            now,
            now,
            EnclosureField {
                config: &no_law,
                body_props: None,
                body_radius: None,
            },
            AnchorEnvelope {
                vmax: 0.0,
                amax: 0.0,
                pad: 0.0,
                ttl: 60.0,
            },
        ),
        "a field without a propagation law keeps the record"
    );
    assert!(
        super::record_in_enclosure(
            &near,
            None,
            now,
            now,
            EnclosureField {
                config: &fc,
                body_props: None,
                body_radius: None,
            },
            AnchorEnvelope {
                vmax: 0.0,
                amax: 0.0,
                pad: 0.0,
                ttl: 60.0,
            },
        ),
        "an unresolvable record position keeps the record"
    );
    let jump: Vec<PresenceSample> = vec![(
        now,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        crate::mathematikerin::JUMP_GRID,
    )];
    let snap = super::Φ * crate::mathematikerin::JUMP_GRID;
    assert!(
        super::record_in_enclosure(
            &jump,
            Some([snap, 0.0, 0.0]),
            now,
            now,
            EnclosureField {
                config: &fc,
                body_props: None,
                body_radius: None,
            },
            AnchorEnvelope {
                vmax: 0.0,
                amax: 0.0,
                pad: 0.0,
                ttl: 60.0,
            },
        ),
        "the jump snap widens the enclosure to Φ·grid_step"
    );
    assert!(
        !super::record_in_enclosure(
            &jump,
            Some([snap + 1.0, 0.0, 0.0]),
            now,
            now,
            EnclosureField {
                config: &fc,
                body_props: None,
                body_radius: None,
            },
            AnchorEnvelope {
                vmax: 0.0,
                amax: 0.0,
                pad: 0.0,
                ttl: 60.0,
            },
        ),
        "a record beyond the jump snap stays raw"
    );
    let thermal = FieldConfig {
        kernel: 3,
        force: 5,
        ..fc.clone()
    };
    assert!(
        !super::record_in_enclosure(
            &near,
            Some([12.0, 0.0, 0.0]),
            now + 10.0,
            now,
            EnclosureField {
                config: &thermal,
                body_props: None,
                body_radius: None,
            },
            AnchorEnvelope {
                vmax: 0.0,
                amax: 0.0,
                pad: 0.0,
                ttl: 60.0,
            },
        ),
        "without anchor motion the thermal enclosure stays tight"
    );
    assert!(
        super::record_in_enclosure(
            &near,
            Some([12.0, 0.0, 0.0]),
            now + 10.0,
            now,
            EnclosureField {
                config: &thermal,
                body_props: None,
                body_radius: None,
            },
            AnchorEnvelope {
                vmax: 1.0,
                amax: 0.0,
                pad: 0.0,
                ttl: 60.0,
            },
        ),
        "rho widens the enclosure by anchor_vmax·age"
    );
    assert!(
        !super::record_in_enclosure(
            &near,
            Some([0.0, 0.0, 0.0]),
            now + 60.0 * 64.0 + 1.0,
            now,
            EnclosureField {
                config: &fc,
                body_props: None,
                body_radius: None,
            },
            AnchorEnvelope {
                vmax: 0.0,
                amax: 0.0,
                pad: 0.0,
                ttl: 60.0,
            },
        ),
        "a record older than effective_ttl·2⁶ drops"
    );
}

#[test]
fn test_wind_waves_loader_respects_load_gate() {
    use std::collections::HashMap;
    let now = 8.0e8;
    let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
    cx[0] = 1.5e9;
    let props = super::BodyProperties {
        α0_deg: 0.0,
        dα0_dt_deg_per_century: 0.0,
        δ0_deg: 90.0,
        dδ0_dt_deg_per_century: 0.0,
        w0_deg: 190.147,
        dw_dt_deg_per_day: 360.9856235,
        radius_m: 6378136.6,
        flattening: Some(0.0033528131084554157),
        gaussian_inverse_square: 0.0,
        gaussian_inverse: 0.0,
        erfc: 0.0,
        patch_levy: 0.0,
        exponential_decay: 0.0,
        gm: None,
        j2: None,
        j4: None,
        radii_b: None,
        radii_c: None,
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let eph = super::BodyEphemeris {
        granules: vec![super::ChebyshevGranule {
            t0_jd: super::J2000_EPOCH + now / 86400.0,
            dt_jd: 1.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        }],
        rotation_matrices: vec![],
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::atomic::AtomicUsize::new(0).into(),
    };
    let mut map = HashMap::new();
    map.insert("earth".to_string(), eph);
    let frame = super::Frame::Barycenter {
        body_name: "earth".into(),
        scale: 1.0,
    };
    let records = crate::wind::parse_bin(&crate::wind::write_bin(&[(
        now,
        1075.0e3,
        3.0e3,
        1.2e-4,
        crate::wind::RECEIVER_RAD1,
    )]))
    .expect("the wind_waves fixture carries one record");
    let fc = FieldConfig {
        key: "wind_waves_rad1".into(),
        name: "wind_waves_rad1".into(),
        kernel: 0,
        force: 0,
        tau: 60.0,
        absorption: 0.0,
        advection: 0.0,
        unit: String::new(),
        freq: 1075.0e3,
        bin_width: 3.0e3,
        fold: None,
    };
    let body_radius = 6378136.6;
    let loader_keep = |presences: &[PresenceSample],
                       eph_map: &HashMap<String, super::BodyEphemeris>| {
        let mut kept = 0usize;
        let mut dropped = 0usize;
        for &(t, _, _, _, _) in &records {
            let mut keep = true;
            if let Some(motion) = super::frame_motion(&frame, None, None, t, eph_map)
                && let Some((anchor_vmax, anchor_amax, _)) =
                    super::law_bounds(&motion, t, 0.0, eph_map)
            {
                keep = super::record_in_enclosure(
                    presences,
                    motion.at(t, t, eph_map),
                    t,
                    now,
                    EnclosureField {
                        config: &fc,
                        body_props: eph_map.get("earth").and_then(|e| e.props.as_ref()),
                        body_radius: Some(body_radius),
                    },
                    AnchorEnvelope {
                        vmax: anchor_vmax,
                        amax: anchor_amax,
                        pad: 0.0,
                        ttl: 604800.0,
                    },
                );
            }
            if keep {
                kept += 1;
            } else {
                dropped += 1;
            }
        }
        (kept, dropped)
    };
    let near: Vec<PresenceSample> = vec![(now, 1.5e9, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0)];
    assert_eq!(
        loader_keep(&near, &map),
        (1, 0),
        "the presence at the frame body materializes the wind_waves record"
    );
    let far: Vec<PresenceSample> =
        vec![(now, 1.5e9 + 1.0e12, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0)];
    assert_eq!(
        loader_keep(&far, &map),
        (0, 1),
        "the presence far outside the frame enclosure leaves the record raw"
    );
    let empty_map = HashMap::new();
    assert_eq!(
        loader_keep(&near, &empty_map),
        (1, 0),
        "an unresolvable frame keeps the record — never a drop"
    );
}

#[test]
fn test_port_convert_celestial_and_post() {
    let celestial = "source oac\nttl 86400\nforce em\nurl https://api.example.org/{target}/\nverify false\ntarget SN2014J\nmap .\nlat_key ra\nlon_key dec\nfield name name\n";
    let conv = super::port_block(celestial);
    assert!(conv.contains("ttl 86400\n"));
    assert!(conv.contains("at sun\n"));
    assert!(conv.contains("url https://api.example.org/{target}/\n"));
    assert!(
        conv.contains("# declined field name — not an oscillator (no physical force)"),
        "a nominal name is declined, never fabricated, got: {conv}"
    );
    let srcs = super::parse_sources(&conv);
    for s in &srcs {
        for e in &s.extracts {
            match e {
                super::Extract::CelestialMap { fields, .. }
                | super::Extract::Map { fields, .. } => {
                    assert_eq!(fields.len(), 0);
                }
                _ => {}
            }
        }
    }
    let post = "source stac\nttl 86400\nforce em\nurl https://example.org/search\nmethod post\nbody {\"collections\":[\"x\"],\"bbox\":[{lon_min},{lat_min},{lon_max},{lat_max}]}\nmap features\nlat_key properties.centroid.lat\nlon_key properties.centroid.lon\nfield id scene\n";
    let conv = super::port_block(post);
    assert!(conv.contains(
        "post_body {\"collections\":[\"x\"],\"bbox\":[{lon_min},{lat_min},{lon_max},{lat_max}]}\n"
    ));
    let srcs = super::parse_sources(&conv);
    assert!(
        srcs.is_empty()
            || srcs.iter().all(|s| s.extracts.iter().all(|e| match e {
                super::Extract::Map { fields, .. } => fields.is_empty(),
                _ => true,
            }))
    );
    let array_post = "source arr\nttl 86400\nforce em\nurl https://example.org/search\nmethod POST\nbody [1,2,3]\nmap features\nlat_key properties.centroid.lat\nlon_key properties.centroid.lon\nfield id scene\n";
    let conv = super::port_block(array_post);
    assert!(conv.contains("post_body [1,2,3]\n"));
    let srcs = super::parse_sources(&conv);
    assert!(
        srcs.is_empty()
            || srcs.iter().all(|s| s.extracts.iter().all(|e| match e {
                super::Extract::Map { fields, .. } => fields.is_empty(),
                _ => true,
            }))
    );
    let form_post = "source form\nttl 86400\nforce em\nurl https://example.org/search\nmethod POST\nbody collection=landsat&limit=10\nmap features\nlat_key lat\nlon_key lon\nfield id scene\n";
    let conv = super::port_block(form_post);
    assert!(conv.contains("post_body collection=landsat&limit=10\n"));
    let srcs = super::parse_sources(&conv);
    assert!(
        srcs.is_empty()
            || srcs.iter().all(|s| s.extracts.iter().all(|e| match e {
                super::Extract::Map { fields, .. } => fields.is_empty(),
                _ => true,
            }))
    );
    let no_method =
        "source kt\nttl 86400\nforce em\nurl https://example.org/x\nformat kernel_text\nbody 399\n";
    let conv = super::port_block(no_method);
    assert!(conv.contains("body 399\n"));
    let srcs = super::parse_sources(&conv);
    assert!(
        srcs.is_empty()
            || srcs.iter().all(|s| s.extracts.iter().all(|e| match e {
                super::Extract::Map { fields, .. } => fields.is_empty(),
                _ => true,
            }))
    );
}
#[test]
fn test_port_convert_ra_dec_key() {
    let celestial = "source vizier\nttl 86400\nforce em\nurl https://tap.example.org/sync\nformat text\nrows\nra_key RAJ2000\ndec_key DEJ2000\nplx_key Plx\nz_key z\ndist_key dist\nfield Name star_name\n";
    let conv = super::port_block(celestial);
    assert!(conv.contains("at sun\n"));
    assert!(conv.contains("ra RAJ2000\n"));
    assert!(conv.contains("dec DEJ2000\n"));
    assert!(conv.contains("plx Plx\n"));
    assert!(conv.contains("z z\n"));
    assert!(!conv.contains("dist "));
    let srcs = super::parse_sources(&conv);
    assert!(!srcs.is_empty());
    let mut found = false;
    for s in &srcs {
        for e in &s.extracts {
            if let super::Extract::CelestialMap {
                ra_key,
                dec_key,
                plx_key,
                z_key,
                ..
            } = e
            {
                assert_eq!(ra_key, "RAJ2000");
                assert_eq!(dec_key, "DEJ2000");
                assert_eq!(plx_key, "Plx");
                assert_eq!(z_key, "z");
                found = true;
            }
        }
    }
    assert!(found, "celestial map extract present");
}
#[test]
fn test_port_convert_dist_pm_rv_keys() {
    let with_scale = "source vizier\nttl 86400\nforce em\nurl https://tap.example.org/sync\nformat text\nrows\nra_key RAJ2000\ndec_key DEJ2000\npmra_key pmRA\npmdec_key pmDE\nradvel_key RadialVelocity\nrv_scale 1000.0\ndist_key sy_dist\ndist_scale 3.085677581e16\nfield Name star_name\n";
    let conv = super::port_block(with_scale);
    assert!(conv.contains("pmra pmRA\n"));
    assert!(conv.contains("pmdec pmDE\n"));
    assert!(conv.contains("radvel RadialVelocity\n"));
    assert!(conv.contains("rv_scale 1000.0\n"));
    assert!(conv.contains("dist sy_dist\n"));
    assert!(conv.contains("dist_scale 3.085677581e16\n"));
    let srcs = super::parse_sources(&conv);
    assert!(!srcs.is_empty());
    let mut found = false;
    for s in &srcs {
        for e in &s.extracts {
            if let super::Extract::CelestialMap {
                dist_key,
                dist_scale,
                pmra_key,
                pmdec_key,
                rv_key,
                rv_scale,
                ..
            } = e
            {
                assert_eq!(dist_key, "sy_dist");
                assert_eq!(*dist_scale, Some(3.085677581e16));
                assert_eq!(pmra_key, "pmRA");
                assert_eq!(pmdec_key, "pmDE");
                assert_eq!(rv_key, "RadialVelocity");
                assert_eq!(*rv_scale, Some(1000.0));
                found = true;
            }
        }
    }
    assert!(found, "celestial map extract present");

    let parallax = "source vizier\nttl 86400\nforce em\nurl https://tap.example.org/sync\nformat text\nrows\nra_key RA_ICRS\ndec_key DE_ICRS\ndist_key 1000/Plx\n";
    let conv = super::port_block(parallax);
    assert!(conv.contains("plx Plx\n"));
    assert!(!conv.contains("dist "));
    assert!(!conv.contains("dist_scale "));

    let no_scale = "source vizier\nttl 86400\nforce em\nurl https://tap.example.org/sync\nformat text\nrows\nra_key RAJ2000\ndec_key DEJ2000\ndist_key Dist\n";
    let conv = super::port_block(no_scale);
    assert!(!conv.contains("dist "));
    assert!(!conv.contains("dist_scale "));
}
#[test]
fn test_walk_celestial_cmap() {
    let j = super::parse_json("{\"results\":[{\"ra\":1.5,\"dec\":-2.5,\"mag\":12.3}]}").unwrap();
    let mut fields = String::new();
    let mut coords = String::new();
    let mut map_path: Option<String> = None;
    let mut budget = 48usize;
    super::walk_json_probe(&j, "", &mut fields, &mut coords, &mut map_path, &mut budget);
    assert!(coords.contains("ra "));
    assert!(coords.contains("dec "));
    assert_eq!(map_path.as_deref(), Some("results"));
    assert!(fields.contains("field"));
    let (frame, _) = super::derive_frame(&j, &coords);
    assert!(frame.starts_with("at sun"));
}

#[test]
fn test_tap_to_json_rows() {
    let j = super::parse_json(
            "{\"metadata\":[{\"name\":\"RAJ2000\"},{\"name\":\"DEJ2000\"},{\"name\":\"Ksmag\"}],\"data\":[[1.5,-2.5,12.3],[3.0,4.0,10.1]]}",
        )
        .unwrap();
    let flat = super::tap_to_json(&j).unwrap();
    let mut fields = String::new();
    let mut coords = String::new();
    let mut map_path: Option<String> = None;
    let mut budget = 48usize;
    super::walk_json_probe(
        &flat,
        "",
        &mut fields,
        &mut coords,
        &mut map_path,
        &mut budget,
    );
    assert!(coords.contains("ra RAJ2000"));
    assert!(coords.contains("dec DEJ2000"));
    assert_eq!(map_path.as_deref(), Some("."));
    let (frame, _) = super::derive_frame(&flat, &coords);
    assert!(frame.starts_with("at sun"));
}

#[test]
fn test_parse_stations_xml() {
    let xml = "<?xml version=\"1.0\" ?><GINServices>\n <ObservatoryList>\n  <Observatory>\n   <Code>AAE</Code>\n   <Name>Addis Ababa</Name>\n   <Latitude>9.035</Latitude>   <Longitude>38.770</Longitude>   <Elevation>2441</Elevation>\n  </Observatory>\n  <Observatory>\n   <Code>YKC</Code>\n   <Latitude>62.48</Latitude>   <Longitude>-114.48</Longitude>   <Elevation>181</Elevation>\n  </Observatory>\n </ObservatoryList>\n</GINServices>";
    let st = parse_stations_xml(xml);
    assert_eq!(st.len(), 2);
    assert_eq!(st[0].id, "aae");
    assert_eq!(st[0].lat, 9.035);
    assert_eq!(st[0].lon, 38.770);
    assert_eq!(st[1].id, "ykc");
    assert_eq!(st[1].lat, 62.48);
}

#[test]
#[ignore = "local source-port backlog verifier — reads and writes phi/pipeline/stage/ working data"]
fn test_backlog_batches_verify() {
    fn substitute_test_templates(url: &str) -> String {
        let mut u = url.to_string();
        for (k, v) in [
            ("{today}", "2026-08-07"),
            ("{yesterday}", "2026-08-06"),
            ("{tomorrow}", "2026-08-08"),
            ("{now}", "2026-08-07T12:00:00Z"),
            ("{year}", "2026"),
            ("{month}", "08"),
            ("{day}", "07"),
            ("{lat}", "29.5"),
            ("{lon}", "-95.0"),
            ("{ra}", "0.0"),
            ("{dec}", "0.0"),
            ("{target}", "Ceres"),
            ("{week_ago}", "2026-07-31"),
            ("{hour_ago}", "2026-08-07T11:00:00Z"),
            ("{body}", "ISS"),
            ("{lon_min}", "-95.0"),
            ("{lon_max}", "-94.0"),
            ("{lat_min}", "29.0"),
            ("{lat_max}", "30.0"),
            ("{grid}", "29.5,-95.0|29.6,-95.0"),
            ("{nearest_station}", "8518750"),
        ] {
            u = u.replace(k, v);
        }
        u
    }
    let live: std::collections::HashSet<String> = super::load_sources()
        .iter()
        .map(|s| s.url.clone())
        .collect();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut ok_text = String::from("# staging: backlog blocks verified with samples\n");
    if let Ok(existing) = std::fs::read_to_string("phi/pipeline/stage/staging_verified.φ") {
        for l in existing.lines() {
            let t = l.trim_start();
            if let Some(u) = t.strip_prefix("url ") {
                seen.insert(u.trim().to_string());
            }
        }
        ok_text = existing;
    }
    let mut void_text = String::new();
    if let Ok(existing) = std::fs::read_to_string("phi/pipeline/stage/staging_void_ledger.txt") {
        for l in existing.lines() {
            if let Some(u) = l.strip_prefix("void ")
                && let Some(end) = u.find(' ')
            {
                seen.insert(u[..end].to_string());
            }
        }
        void_text = existing;
    }
    let fixture_lsk = full_fixture_lsk();
    let now = fixture_lsk.system_now_tdb().unwrap();
    let env = super::load_env();
    let mut limit = 300usize;
    let mut ok = 0usize;
    let mut empty = 0usize;
    for e in std::fs::read_dir("phi/pipeline/stage").unwrap().flatten() {
        let fname = e.file_name().to_string_lossy().to_string();
        if !fname.ends_with("_converted.φ") {
            continue;
        }
        let content = std::fs::read_to_string(e.path()).unwrap();
        let mut block = String::new();
        for line in content.lines().chain(std::iter::once("url __eof__")) {
            let t = line.trim_start();
            if t.starts_with("url ") && !block.is_empty() {
                if limit == 0 {
                    break;
                }
                let srcs = super::parse_sources(&block);
                for s in &srcs {
                    if s.fanout_cap > 0 || s.format == "csv_zip" || s.format == "kernel_text" {
                        break;
                    }
                    let mut url = substitute_test_templates(&s.url);
                    url = super::resolve_secret(&url, &env);
                    url = url.replace(' ', "%20");
                    if live.contains(&s.url) || !seen.insert(s.url.clone()) {
                        break;
                    }
                    limit -= 1;
                    let headers = super::render_headers(&s.headers, &env);
                    let post_body = s.post_body.as_ref().map(|pb| substitute_test_templates(pb));
                    let post = post_body.as_deref();
                    let body = match super::fetch_raw_probe(&url, post, &headers) {
                        Some(b) => b,
                        None => {
                            empty += 1;
                            void_text
                                .push_str(&format!("void {} {}\n", s.url, "fetch returned empty"));
                            break;
                        }
                    };
                    let n_samples = match super::extract(s, &body, now, &fixture_lsk) {
                        super::ExtractResult::Measurements(v) => v.len(),
                        super::ExtractResult::WithEphemeris(v, _) => v.len(),
                    };
                    if n_samples == 0 {
                        empty += 1;
                        void_text.push_str(&format!(
                            "void {} {}\n",
                            s.url,
                            super::diagnose_no_samples(s, &body)
                        ));
                    } else {
                        ok += 1;
                        ok_text.push_str(&format!("# from {}\n", fname));
                        ok_text.push_str(&block);
                        ok_text.push('\n');
                    }
                }
                block = String::new();
            }
            block.push_str(line);
            block.push('\n');
        }
    }
    eprintln!("=== BACKLOG VERIFY: {} ok, {} empty ===", ok, empty);
    let ok_path = "phi/pipeline/stage/staging_verified.φ";
    let void_path = "phi/pipeline/stage/staging_void_ledger.txt";
    std::fs::write(ok_path, &ok_text).unwrap();
    std::fs::write(void_path, &void_text).unwrap();
    eprintln!("staged: {} and {}", ok_path, void_path);
}

#[test]
fn test_erddap_argo_map_extract() {
    let src = SourceConfig {
        ttl: 43200,
        url: "https://erddap.ifremer.fr/erddap/tabledap/ArgoFloats.json".into(),
        frame: super::Frame::Surface {
            body_name: "body_test".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "json".into(),
        extracts: vec![super::Extract::Map {
            arr_path: "table.rows".into(),
            lat_key: "2".into(),
            lon_key: "1".into(),
            alt_key: "3".into(),
            epoch_key: "0".into(),
            val_key: String::new(),
            alt_scale: -1.0,
            vel_key: String::new(),
            vel_scale: 1.0,
            trk_key: String::new(),
            vr_key: String::new(),
            fields: vec![FieldConfig {
                key: "4".into(),
                name: "argo_temp_c".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            lat_sign: None,
            lon_sign: None,
            epoch_scale: 1.0,
            tau_key: String::new(),
            mag_type_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let body = r#"{"table":{"columnNames":["time","longitude","latitude","pres","temp"],"columnTypes":["String","double","double","float","float"],"rows":[["2026-07-30T21:40:30Z",-14.408395,34.49025,3.1,23.478],["2026-07-30T22:00:00Z",-12.5,35.0,1000.0,4.681]]}}"#;
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    let test_epoch = super::parse_iso_tdb("2026-07-30T21:40:30Z", &fixture_lsk).unwrap();
    let now = test_epoch + 86400.0;
    eprintln!("now={} test_epoch={}", now, test_epoch);
    let result = super::extract(&src, body, now, &fixture_lsk);
    let channels = match result {
        super::ExtractResult::Measurements(v) => v,
        _ => {
            panic!("ExtractResult is WithEphemeris, 0 Channels");
        }
    };
    assert_eq!(channels.len(), 2);
    let (p0, _f0) = &channels[0];
    assert!(p0.epoch < now);
    let test_epoch = super::parse_iso_tdb("2026-07-30T21:40:30Z", &fixture_lsk).unwrap();
    assert!((p0.epoch - test_epoch).abs() < 1e-6);
    match p0.position {
        super::Position::Surface {
            lat,
            lon,
            alt,
            body_name: _,
        } => {
            assert!((lat - 34.49025).abs() < 1e-6);
            assert!((lon - -14.408395).abs() < 1e-6);
            assert!((alt - -3.1).abs() < 1e-6);
        }
        _ => panic!("position is {:?}", p0.position),
    }
    assert_eq!(p0.name, "argo_temp_c");
    assert!((p0.value - 23.478).abs() < 1e-6);
    let (p1, _) = &channels[1];
    match p1.position {
        super::Position::Surface { alt, .. } => {
            assert!((alt - -1000.0).abs() < 1e-6);
        }
        _ => panic!("position is {:?}", p1.position),
    }
}

#[test]
fn test_ymd_days_roundtrip() {
    for (y, m, d) in [
        (1970, 1, 1),
        (2000, 2, 29),
        (2026, 7, 31),
        (2026, 12, 31),
        (2026, 1, 1),
    ] {
        let days = super::ymd_to_days(y, m, d).unwrap();
        let (y2, m2, d2) = super::days_to_ymd(days);
        assert_eq!(
            (y2 as i64, m2, d2),
            (y, m, d),
            "roundtrip {} {}-{}-{}",
            days,
            y,
            m,
            d
        );
    }
    assert_eq!(super::ymd_to_days(1970, 1, 1).unwrap(), 0);
    assert!(super::ymd_to_days(1900, 1, 1).is_none());
    assert_eq!(super::ymd_to_days(1970, 1, 1).unwrap(), 0);
}

#[test]
fn test_kernel_id_of() {
    assert_eq!(super::kernel_id_of("inverse-square"), Some(0));
    assert_eq!(super::kernel_id_of("gaussian-inverse-square"), Some(1));
    assert_eq!(super::kernel_id_of("gaussian-inverse"), Some(2));
    assert_eq!(super::kernel_id_of("erfc"), Some(3));
    assert_eq!(super::kernel_id_of("exponential-decay"), Some(4));
    assert_eq!(super::kernel_id_of("patch-levy"), Some(5));
    assert_eq!(super::kernel_id_of("inverse-linear"), Some(6));
    assert_eq!(super::kernel_id_of("nonexistent"), None);
}

#[test]
fn test_universal_auto_detect_celestial() {
    let body = r#"{"data":[{"ra":83.63,"dec":22.01,"plx":3.14,"val":14.2,"t":2457389.5}]}"#;
    let j = super::parse_json(body).unwrap();
    let extracts = super::universal_auto_detect(&j);
    assert_eq!(extracts.len(), 1);
    match &extracts[0] {
        super::Extract::CelestialMap {
            ra_key,
            dec_key,
            plx_key,
            pmra_key,
            pmdec_key,
            arr_path,
            fields,
            ..
        } => {
            assert_eq!(ra_key, "ra");
            assert_eq!(dec_key, "dec");
            assert_eq!(plx_key, "plx");
            assert_eq!(pmra_key, "");
            assert_eq!(pmdec_key, "");
            assert_eq!(arr_path, "data");
            assert!(!fields.is_empty());
        }
        _ => panic!("auto_detect returned Map/Rows/Field, CelestialMap absent"),
    }
}

#[test]
fn test_universal_auto_detect_terrestrial() {
    let body = r#"{"data":[{"lat":52.5,"lon":13.4,"val":28.5,"alt":50.0}]}"#;
    let j = super::parse_json(body).unwrap();
    let extracts = super::universal_auto_detect(&j);
    assert_eq!(extracts.len(), 1);
    match &extracts[0] {
        super::Extract::Map {
            lat_key,
            lon_key,
            alt_key,
            arr_path,
            ..
        } => {
            assert_eq!(lat_key, "lat");
            assert_eq!(lon_key, "lon");
            assert_eq!(alt_key, "alt");
            assert_eq!(arr_path, "data");
        }
        _ => panic!("auto_detect returned CelestialMap/Rows/Field, Map absent"),
    }
}

#[test]
fn test_wgccre_roundtrip() {
    use std::collections::HashMap;
    let tdb = 3.0 * 86400.0;
    let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
    cx[0] = 1.5e9;
    let props = super::BodyProperties {
        α0_deg: 317.68143,
        dα0_dt_deg_per_century: -0.1061,
        δ0_deg: 52.88650,
        dδ0_dt_deg_per_century: -0.0609,
        w0_deg: 176.630,
        dw_dt_deg_per_day: 350.89198226,
        radius_m: 3389500.0,
        flattening: Some(0.00589),
        gaussian_inverse_square: 0.0,
        gaussian_inverse: 0.0,
        erfc: 0.0,
        patch_levy: 0.0,
        exponential_decay: 0.0,
        gm: None,
        j2: None,
        j4: None,
        radii_b: None,
        radii_c: None,
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let granule = super::ChebyshevGranule {
        t0_jd: super::J2000_EPOCH,
        dt_jd: 32.0,
        cx,
        cy: [0.0; super::CHEBYSHEV_N],
        cz: [0.0; super::CHEBYSHEV_N],
    };
    let eph = super::BodyEphemeris {
        granules: vec![granule],
        rotation_matrices: vec![],
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    let mut map = HashMap::new();
    map.insert("mars".to_string(), eph);
    let cases = [
        (35.0, -15.0, 0.0),
        (0.0, 90.0, 0.0),
        (89.9, 45.0, 0.0),
        (-60.0, 170.0, 5000.0),
    ];
    for (lat, lon, alt) in cases {
        let p = super::body_fixed_to_icrs("mars", lat, lon, alt, tdb, &map).unwrap();
        let (lat2, lon2) =
            super::icrs_to_body_surface(p[0], p[1], p[2], tdb, "mars", &map).unwrap();
        assert!((lat2 - lat).abs() < 1e-6, "lat {} vs {}", lat2, lat);
        assert!((lon2 - lon).abs() < 1e-6, "lon {} vs {}", lon2, lon);
        let (lat3, lon3, depth) =
            super::icrs_to_body_geodetic(p[0], p[1], p[2], tdb, "mars", &map).unwrap();
        assert!((lat3 - lat).abs() < 1e-6, "geo lat {} vs {}", lat3, lat);
        assert!((lon3 - lon).abs() < 1e-6, "geo lon {} vs {}", lon3, lon);
        assert!(
            (depth - (-alt / 1000.0)).abs() < 1e-2,
            "depth {} vs {}",
            depth,
            -alt / 1000.0
        );
    }
}

#[test]
fn test_earth_zenith_geometry() {
    use std::collections::HashMap;
    let jd = 2448782.70138981;
    let tdb = (jd - super::J2000_EPOCH) * 86400.0;
    let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
    cx[0] = 1.5e9;
    let props = super::BodyProperties {
        α0_deg: 0.0,
        dα0_dt_deg_per_century: 0.0,
        δ0_deg: 90.0,
        dδ0_dt_deg_per_century: 0.0,
        w0_deg: 190.147,
        dw_dt_deg_per_day: 360.9856235,
        radius_m: 6378136.6,
        flattening: Some(0.0033528131084554157),
        gaussian_inverse_square: 0.0,
        gaussian_inverse: 0.0,
        erfc: 0.0,
        patch_levy: 0.0,
        exponential_decay: 0.0,
        gm: None,
        j2: None,
        j4: None,
        radii_b: None,
        radii_c: None,
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let granule = super::ChebyshevGranule {
        t0_jd: super::J2000_EPOCH - 0.0,
        dt_jd: 5000.0,
        cx,
        cy: [0.0; super::CHEBYSHEV_N],
        cz: [0.0; super::CHEBYSHEV_N],
    };
    let eph = super::BodyEphemeris {
        granules: vec![granule],
        rotation_matrices: vec![],
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::atomic::AtomicUsize::new(0).into(),
    };
    let mut map = HashMap::new();
    map.insert("earth".to_string(), eph);
    let p = super::body_fixed_to_icrs("earth", -22.534444444, -45.5825, 1810.7, tdb, &map).unwrap();
    let b = super::body_barycenter_position("earth", tdb, &map).unwrap();
    let w = [p[0] - b[0], p[1] - b[1], p[2] - b[2]];
    let n = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
    let lat = (w[2] / n).asin().to_degrees();
    let ra = w[1].atan2(w[0]).to_degrees().rem_euclid(360.0);
    let d = jd - 2451545.0;
    let t = d / 36525.0;
    let gmst = (280.46061837 + 360.98564736629 * d + 0.000387933 * t * t - t * t * t / 38710000.0)
        .rem_euclid(360.0);
    let ra_expect = (gmst + (-45.5825)).rem_euclid(360.0);
    assert!(
        (lat - (-22.3989)).abs() < 0.01,
        "station geocentric latitude {lat} vs -22.3989"
    );
    assert!(
        (ra - ra_expect).abs() < 0.6,
        "station RA {ra} vs GMST+lon {ra_expect}"
    );
}

#[test]
fn test_rotation_matrix_roundtrip() {
    use std::collections::HashMap;
    let tdb = 3.0 * 86400.0;
    let jd = super::J2000_EPOCH + 3.0;
    let tc = (jd - super::J2000_EPOCH) / 36525.0;
    let m: [f64; 9] = super::ephemeris::rotation_matrix_from_angles(
        317.68143 - 0.1061 * tc,
        52.88650 - 0.0609 * tc,
        176.630 + 350.89198226 * (jd - super::J2000_EPOCH),
    );
    let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
    cx[0] = 1.5e9;
    let props = super::BodyProperties {
        α0_deg: 317.68143,
        dα0_dt_deg_per_century: -0.1061,
        δ0_deg: 52.88650,
        dδ0_dt_deg_per_century: -0.0609,
        w0_deg: 176.630,
        dw_dt_deg_per_day: 350.89198226,
        radius_m: 3389500.0,
        flattening: Some(0.00589),
        gaussian_inverse_square: 0.0,
        gaussian_inverse: 0.0,
        erfc: 0.0,
        patch_levy: 0.0,
        exponential_decay: 0.0,
        gm: None,
        j2: None,
        j4: None,
        radii_b: None,
        radii_c: None,
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let granule = super::ChebyshevGranule {
        t0_jd: super::J2000_EPOCH,
        dt_jd: 32.0,
        cx,
        cy: [0.0; super::CHEBYSHEV_N],
        cz: [0.0; super::CHEBYSHEV_N],
    };
    let eph = super::BodyEphemeris {
        granules: vec![granule],
        rotation_matrices: vec![(jd, m)],
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    let mut map = HashMap::new();
    map.insert("mars".to_string(), eph);
    let cases = [
        (35.0, -15.0, 0.0),
        (0.0, 90.0, 0.0),
        (89.9, 45.0, 0.0),
        (-60.0, 170.0, 5000.0),
    ];
    for (lat, lon, alt) in cases {
        let p = super::body_fixed_to_icrs("mars", lat, lon, alt, tdb, &map).unwrap();
        let (lat2, lon2) =
            super::icrs_to_body_surface(p[0], p[1], p[2], tdb, "mars", &map).unwrap();
        assert!((lat2 - lat).abs() < 1e-6, "lat {} vs {}", lat2, lat);
        assert!((lon2 - lon).abs() < 1e-6, "lon {} vs {}", lon2, lon);
    }
}

#[test]
fn test_matrix_vs_wgccre_agreement() {
    use std::collections::HashMap;
    let tdb = 3.0 * 86400.0;
    let jd = super::J2000_EPOCH + 3.0;
    let tc = (jd - super::J2000_EPOCH) / 36525.0;
    let m: [f64; 9] = super::ephemeris::rotation_matrix_from_angles(
        317.68143 - 0.1061 * tc,
        52.88650 - 0.0609 * tc,
        176.630 + 350.89198226 * (jd - super::J2000_EPOCH),
    );
    let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
    cx[0] = 1.5e9;
    let props = super::BodyProperties {
        α0_deg: 317.68143,
        dα0_dt_deg_per_century: -0.1061,
        δ0_deg: 52.88650,
        dδ0_dt_deg_per_century: -0.0609,
        w0_deg: 176.630,
        dw_dt_deg_per_day: 350.89198226,
        radius_m: 3389500.0,
        flattening: Some(0.00589),
        gaussian_inverse_square: 0.0,
        gaussian_inverse: 0.0,
        erfc: 0.0,
        patch_levy: 0.0,
        exponential_decay: 0.0,
        gm: None,
        j2: None,
        j4: None,
        radii_b: None,
        radii_c: None,
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let granule = super::ChebyshevGranule {
        t0_jd: super::J2000_EPOCH,
        dt_jd: 32.0,
        cx,
        cy: [0.0; super::CHEBYSHEV_N],
        cz: [0.0; super::CHEBYSHEV_N],
    };
    let eph_matrix = super::BodyEphemeris {
        granules: vec![granule.clone()],
        rotation_matrices: vec![(jd, m)],
        props: Some(props.clone()),
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    let eph_test = super::BodyEphemeris {
        granules: vec![granule],
        rotation_matrices: vec![],
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    let mut map_matrix = HashMap::new();
    map_matrix.insert("mars".to_string(), eph_matrix);
    let mut map_props = HashMap::new();
    map_props.insert("mars".to_string(), eph_test);
    let cases = [(35.0, -15.0, 0.0), (0.0, 90.0, 0.0), (-60.0, 170.0, 5000.0)];
    for (lat, lon, alt) in cases {
        let pm = super::body_fixed_to_icrs("mars", lat, lon, alt, tdb, &map_matrix).unwrap();
        let pf = super::body_fixed_to_icrs("mars", lat, lon, alt, tdb, &map_props).unwrap();
        let d2 = (pm[0] - pf[0]).powi(2) + (pm[1] - pf[1]).powi(2) + (pm[2] - pf[2]).powi(2);
        assert!(
            d2 < 1.0,
            "matrix vs wgccre disagree at ({}, {}): d2={}",
            lat,
            lon,
            d2
        );
    }
}

#[test]
fn test_matrix_path_matches_analytic_across_bodies_and_epochs() {
    use std::collections::HashMap;
    let bodies: [(f64, f64, f64, f64); 4] = [
        (317.68143, 52.88650, 176.630, 350.89198226),
        (0.0, 90.0, 280.46, 360.9856),
        (123.45, -17.5, 200.0, 100.0),
        (40.66, 83.54, 200.39, -6.52),
    ];
    let jd0 = super::J2000_EPOCH + 5.0;
    let lat_lon = [(35.0, -15.0, 0.0), (0.0, 90.0, 0.0), (-60.0, 170.0, 5000.0)];
    for (a0, d0, w0, rate) in bodies {
        let props = super::BodyProperties {
            α0_deg: a0,
            dα0_dt_deg_per_century: 0.0,
            δ0_deg: d0,
            dδ0_dt_deg_per_century: 0.0,
            w0_deg: w0,
            dw_dt_deg_per_day: rate,
            radius_m: 3.0e6,
            flattening: Some(0.0),
            gaussian_inverse_square: 0.0,
            gaussian_inverse: 0.0,
            erfc: 0.0,
            patch_levy: 0.0,
            exponential_decay: 0.0,
            gm: None,
            j2: None,
            j4: None,
            radii_b: None,
            radii_c: None,
            nut_ra: None,
            nut_dec: None,
            nutation: None,
            omega_g: None,
        };
        let w_mt = w0 + rate * (jd0 - super::J2000_EPOCH);
        let m = super::ephemeris::rotation_matrix_from_angles(a0, d0, w_mt);
        let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
        cx[0] = 1.5e9;
        let granule = super::ChebyshevGranule {
            t0_jd: super::J2000_EPOCH,
            dt_jd: 32.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        };
        let hint = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let eph_matrix = super::BodyEphemeris {
            granules: vec![granule.clone()],
            rotation_matrices: vec![(jd0, m)],
            props: Some(props.clone()),
            orbit: None,
            granule_hint: hint.clone(),
        };
        let eph_analytic = super::BodyEphemeris {
            granules: vec![granule],
            rotation_matrices: vec![],
            props: Some(props),
            orbit: None,
            granule_hint: hint,
        };
        let mut map_matrix = HashMap::new();
        map_matrix.insert("body".to_string(), eph_matrix);
        let mut map_analytic = HashMap::new();
        map_analytic.insert("body".to_string(), eph_analytic);
        for offset_d in [0.0, 0.25, 0.5, 1.0] {
            let jd = jd0 + offset_d;
            let tdb = (jd - super::J2000_EPOCH) * 86400.0;
            for (lat, lon, alt) in lat_lon {
                let pm =
                    super::body_fixed_to_icrs("body", lat, lon, alt, tdb, &map_matrix).unwrap();
                let pf =
                    super::body_fixed_to_icrs("body", lat, lon, alt, tdb, &map_analytic).unwrap();
                let d2 =
                    (pm[0] - pf[0]).powi(2) + (pm[1] - pf[1]).powi(2) + (pm[2] - pf[2]).powi(2);
                assert!(
                    d2 < 1.0,
                    "matrix vs analytic disagree (a0={a0} d0={d0} w0={w0} rate={rate} off={offset_d} lat={lat} lon={lon}): d2={d2}"
                );
            }
        }
    }
}

#[test]
fn test_rotation_matrix_empty_props() {
    use std::collections::HashMap;
    let tdb = 3.0 * 86400.0;
    let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
    cx[0] = 1.5e9;
    let props = super::BodyProperties {
        α0_deg: 317.68143,
        dα0_dt_deg_per_century: -0.1061,
        δ0_deg: 52.88650,
        dδ0_dt_deg_per_century: -0.0609,
        w0_deg: 176.630,
        dw_dt_deg_per_day: 350.89198226,
        radius_m: 3389500.0,
        flattening: Some(0.00589),
        gaussian_inverse_square: 0.0,
        gaussian_inverse: 0.0,
        erfc: 0.0,
        patch_levy: 0.0,
        exponential_decay: 0.0,
        gm: None,
        j2: None,
        j4: None,
        radii_b: None,
        radii_c: None,
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let granule = super::ChebyshevGranule {
        t0_jd: super::J2000_EPOCH,
        dt_jd: 32.0,
        cx,
        cy: [0.0; super::CHEBYSHEV_N],
        cz: [0.0; super::CHEBYSHEV_N],
    };
    let eph = super::BodyEphemeris {
        granules: vec![granule],
        rotation_matrices: vec![],
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    let mut map = HashMap::new();
    map.insert("mars".to_string(), eph);
    let p = super::body_fixed_to_icrs("mars", 35.0, -15.0, 0.0, tdb, &map).unwrap();
    assert!(p[0] > 1.0e9);
}

#[test]
fn test_restored_extract_variants() {
    let j =
        super::parse_json(r#"{"data":[{"a":1,"nested":{"b":9}},{"a":2},{"a":3}],"x":[10,20,30]}"#)
            .unwrap();
    assert_eq!(super::jfirst(&j, "data.a"), Some(1.0));
    assert_eq!(super::jlast(&j, "data.a"), Some(3.0));
    assert_eq!(super::jdeep_find_num(&j, "b"), Some(9.0));
    assert_eq!(super::jcount(&j, "data"), Some(3.0));
    assert_eq!(super::jpath(&j, "x.-1"), Some(30.0));
    let j2 = super::parse_json(r#"[["t","a"],["x",1],["y",2]]"#).unwrap();
    assert_eq!(super::j2d_last_row(&j2, "a"), Some(2.0));
    let csv = "# time temp\n1 10\n2 20\n";
    assert_eq!(super::text_last_col(csv, "temp"), Some(20.0));
    assert_eq!(
        super::extract_regex_val(r#"{"totalItems":5,"x":1}"#, r#"("totalItems":...,)"#),
        Some(5.0)
    );
    assert_eq!(
        super::extract_regex_val("<Count>5</Count>", "<Count>([0-9]+)</Count>"),
        Some(5.0)
    );
    assert_eq!(
        super::jcount(&super::parse_json(r"[1,2,3]").unwrap(), "."),
        Some(3.0)
    );
}

#[test]
fn test_text_last_col_headerless_numeric_index() {
    let lc = "55051.500000 3.774494 0.186457 2.214435 0.147425\n55053.500000 3.597870 0.091686 2.025944 0.071018\n";
    assert_eq!(super::text_last_col(lc, "1"), Some(3.597870));
    assert_eq!(super::text_last_col(lc, "0"), Some(55053.5));
    assert_eq!(super::text_last_col(lc, "3"), Some(2.025944));
}

#[test]
fn test_rows_headerless_text_barycenter_frame() {
    let body = "55051.500000 3.774494 0.186457\n55053.500000 3.597870 0.091686\n";
    let mut fc = field_fixture("maxi_flux", 0.0);
    fc.key = "1".into();
    let src = source_fixture(
        "text",
        vec![Extract::Rows {
            last_line: true,
            lat_key: String::new(),
            lon_key: String::new(),
            fields: vec![fc],
            tau_key: String::new(),
            epoch_cols: vec![],
            gates: vec![],
            bin_s: 0,
            name_prefix: String::new(),
        }],
    );
    let lsk = fixture_lsk();
    let now = 8.0e8;
    match extract(&src, body, now, &lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            let (c, _fc) = &channels[0];
            assert_eq!(c.name, "maxi_flux");
            assert!((c.value - 3.597870).abs() < 1e-12);
            assert!(matches!(c.position, super::Position::Barycenter { .. }));
        }
        _ => panic!("extract is not Measurements"),
    }
}

#[test]
fn test_rinex2_obs_epoch_line_satellites() {
    let mut s = String::new();
    s.push_str(
        "     2.11           OBSERVATION DATA    M (MIXED)           RINEX VERSION / TYPE\n",
    );
    s.push_str(
        "                                                            END OF HEADER       \n",
    );
    s.push_str(&format!(
        "{:>3}{:>3}{:>3}{:>3}{:>3}{:>11.7}{:>3}{:>3}G01G02R03\n",
        26i64, 1i64, 1i64, 0i64, 0i64, 0.0, 0i64, 3i64
    ));
    let cell = |v: Option<f64>| -> String {
        match v {
            Some(x) => format!("{x:>14.3}  "),
            None => "                ".to_string(),
        }
    };
    for (c1, l1) in [
        (21345678.123, -12345678.123),
        (21345679.123, -12345679.123),
        (21345680.123, -12345680.123),
    ] {
        s.push_str(&format!(
            "{}{}{}{}\n",
            cell(Some(c1)),
            cell(Some(l1)),
            cell(None),
            cell(Some(1234.567))
        ));
    }
    let epochs = super::parse_rinex_obs(&s, 4);
    assert_eq!(epochs.len(), 1);
    let e = &epochs[0];
    assert_eq!(e.sats.len(), 3);
    assert_eq!(e.sats[0].sat, "G01");
    assert_eq!(e.sats[1].sat, "G02");
    assert_eq!(e.sats[2].sat, "R03");
    assert!((e.sats[0].values[0].unwrap() - 21345678.123).abs() < 1e-3);
    assert!(e.sats[0].values[2].is_none(), "blank S1 stays absent");
}

#[test]
fn test_anchor_body_agnostic() {
    use std::collections::HashMap;
    let frame = super::Frame::Surface {
        body_name: "mars".into(),
        lat: 0.0,
        lon: 0.0,
        alt: 0.0,
    };
    let src = super::SourceConfig {
        ttl: 3600,
        url: "https://example.com".into(),
        frame,
        format: "json".into(),
        extracts: vec![Extract::Field(FieldConfig {
            key: "v".into(),
            name: "v".into(),
            kernel: 1,
            force: 0,
            tau: 0.0,
            absorption: 0.0,
            advection: 0.0,
            unit: String::new(),
            freq: 0.0,
            bin_width: 0.0,
            fold: None,
        })],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: Some("mars".into()),
        stations_url: None,
        stations_path: "stations".into(),
        stations_lat: "lat".into(),
        stations_lon: "lon".into(),
        stations_id: "id".into(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let channel = super::Channel {
        z: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        epoch: 0.0,
        position: super::Position::Surface {
            body_name: "mars".into(),
            lat: 14.0,
            lon: 90.0,
            alt: 0.0,
        },
        name: "v".into(),
        value: 1.0,
    };
    let sensor = super::FieldConfig {
        key: "v".into(),
        name: "v".into(),
        kernel: 1,
        force: 0,
        tau: 60.0,
        absorption: 0.0,
        advection: 0.0,
        unit: String::new(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
    };
    let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
    cx[0] = 1.5e9;
    let props = super::BodyProperties {
        α0_deg: 317.68143,
        dα0_dt_deg_per_century: -0.1061,
        δ0_deg: 52.88650,
        dδ0_dt_deg_per_century: -0.0609,
        w0_deg: 176.630,
        dw_dt_deg_per_day: 350.89198226,
        radius_m: 3389500.0,
        flattening: Some(0.00589),
        gaussian_inverse_square: 0.0,
        gaussian_inverse: 0.0,
        erfc: 0.0,
        patch_levy: 0.0,
        exponential_decay: 0.0,
        gm: None,
        j2: None,
        j4: None,
        radii_b: None,
        radii_c: None,
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let granule = super::ChebyshevGranule {
        t0_jd: super::J2000_EPOCH,
        dt_jd: 32.0,
        cx,
        cy: [0.0; super::CHEBYSHEV_N],
        cz: [0.0; super::CHEBYSHEV_N],
    };
    let mars_eph = super::BodyEphemeris {
        granules: vec![granule],
        rotation_matrices: vec![],
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    let mut eph = HashMap::new();
    eph.insert("mars".to_string(), mars_eph);
    let mut origin_state = super::OriginState {
        fetched: 0.0,
        started: 0.0,
        prev_epoch: 0.0,
        prev_abs: [0.0, 0.0, 0.0],
        prev_motion: None,
        resid_ema: 0.0,
        has_prev: false,
        failures: 0,
        in_flight: false,
    };
    let sample = super::anchor(
        &channel,
        &sensor,
        3600.0,
        Some(0),
        Some(&src.frame),
        Some(&mut origin_state),
        &eph,
    );
    assert!(sample.is_some(), "sample is None");
    let sample = sample.unwrap();
    if let super::Motion::Surface { body_name, .. } = &sample.motion {
        assert_eq!(
            body_name,
            "mars",
            "body name: {}",
            sample.motion.anchor_body().unwrap_or("absent")
        );
    } else {
        panic!("motion is Barycenter or Linear, Surface absent");
    }
}

#[test]
fn test_anchor_applies_declared_unit() {
    use std::collections::HashMap;
    let frame = super::Frame::Surface {
        body_name: "mars".into(),
        lat: 0.0,
        lon: 0.0,
        alt: 0.0,
    };
    let src = super::SourceConfig {
        ttl: 3600,
        url: "https://example.com".into(),
        frame,
        format: "json".into(),
        extracts: vec![],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let channel = super::Channel {
        z: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        epoch: 0.0,
        position: super::Position::Surface {
            body_name: "mars".into(),
            lat: 14.0,
            lon: 90.0,
            alt: 0.0,
        },
        name: "imf".into(),
        value: 7.0,
    };
    let sensor = super::FieldConfig {
        key: "bz".into(),
        name: "imf".into(),
        kernel: 1,
        force: 0,
        tau: 60.0,
        absorption: 0.0,
        advection: 0.0,
        unit: "nT".into(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
    };
    let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
    cx[0] = 1.5e9;
    let props = super::BodyProperties {
        α0_deg: 317.68143,
        dα0_dt_deg_per_century: -0.1061,
        δ0_deg: 52.88650,
        dδ0_dt_deg_per_century: -0.0609,
        w0_deg: 176.630,
        dw_dt_deg_per_day: 350.89198226,
        radius_m: 3389500.0,
        flattening: Some(0.00589),
        gaussian_inverse_square: 0.0,
        gaussian_inverse: 0.0,
        erfc: 0.0,
        patch_levy: 0.0,
        exponential_decay: 0.0,
        gm: None,
        j2: None,
        j4: None,
        radii_b: None,
        radii_c: None,
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let granule = super::ChebyshevGranule {
        t0_jd: super::J2000_EPOCH,
        dt_jd: 32.0,
        cx,
        cy: [0.0; super::CHEBYSHEV_N],
        cz: [0.0; super::CHEBYSHEV_N],
    };
    let mars_eph = super::BodyEphemeris {
        granules: vec![granule],
        rotation_matrices: vec![],
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    let mut eph = HashMap::new();
    eph.insert("mars".to_string(), mars_eph);
    let mut origin_state = super::OriginState {
        fetched: 0.0,
        started: 0.0,
        prev_epoch: 0.0,
        prev_abs: [0.0, 0.0, 0.0],
        prev_motion: None,
        resid_ema: 0.0,
        has_prev: false,
        failures: 0,
        in_flight: false,
    };
    let sample = super::anchor(
        &channel,
        &sensor,
        3600.0,
        Some(0),
        Some(&src.frame),
        Some(&mut origin_state),
        &eph,
    );
    assert!(sample.is_some(), "sample is None");
    let sample = sample.unwrap();
    assert!(
        (sample.val - 7e-9).abs() < 1e-20,
        "nT must convert to Tesla at the anchor, was {}",
        sample.val
    );
}

#[test]
fn test_last_form_captures_unit() {
    let content = "url https://example.com/mag.json\nttl 60\nat sun\nlast bz_gsm imf_bz inverse-square em nT 6.0 0.0 0.0 where satellite 18\n";
    let sources = parse_sources(content);
    assert_eq!(sources.len(), 1);
    match &sources[0].extracts[0] {
        Extract::Last(fc, Some((fk, fv))) => {
            assert_eq!(fc.unit, "nT");
            assert_eq!(fk, "satellite");
            assert_eq!(fv, "18");
        }
        _ => panic!("parsed extract is not a filtered last extract"),
    }
}

#[test]
fn test_convert_luminosity_and_particle_units() {
    let au2 = 1.495978707e11 * 1.495978707e11;
    let v = convert_to_si(1.8e-6, "wm2_1au").unwrap();
    assert!((v - 1.8e-6 * au2).abs() < 1e-6 * 1.8e-6 * au2);
    assert!((convert_to_si(2.0, "pfu").unwrap() - 2.0e4).abs() < 1e-9);
    assert!((convert_to_si(1.0, "1").unwrap() - 1.0).abs() < 1e-12);
    assert!((convert_to_si(1.5, "1e-4w/m2").unwrap() - 1.5e-4).abs() < 1e-16);
}

#[test]
fn test_embedded_lsk_parses_and_covers_now() {
    let lsk = embedded_lsk().expect("the embedded kernel must parse");
    let now_unix = 1.78e9;
    assert_eq!(lsk.leap_at(now_unix), Some(37.0));
    assert!(
        lsk.system_now_tdb().is_some(),
        "the time base exists without any fetch"
    );
}

#[test]
fn test_temporal_ring_never_trims_static_under_overflow() {
    let mk = |epoch: f64, static_sample: bool| super::Sample {
        source: if static_sample {
            super::SampleSource::Ephemeris
        } else {
            super::SampleSource::Source(0)
        },
        epoch,
        ttl: 1e10,
        extent: if static_sample { f64::INFINITY } else { 0.0 },
        tau: 1e10,
        kernel_id: 0.0,
        force_type: 0.0,
        absorption: 0.0,
        advection: 0.0,
        anchor_vmax: 0.0,
        anchor_amax: 0.0,
        anchor_p0: [0.0, 0.0, 0.0],
        motion: super::Motion::Linear {
            p: [0.0, 0.0, 0.0],
            v: [0.0, 0.0, 0.0],
        },
        val: 1.0,
        name: "temporal_ring_test".into(),
        z: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        color_index: 0.0,
        phase: None,
    };
    let static_catalog: Vec<super::Sample> = vec![mk(0.0, true), mk(0.0, true), mk(0.0, true)];
    let mut temporal: Vec<super::Sample> = Vec::new();
    for _ in 0..2 {
        temporal.push(mk(1.0e9, false));
    }
    for _ in 0..4 {
        temporal.push(mk(0.0, false));
    }
    let cap = 2;
    let ring = super::temporal_ring(&static_catalog, &mut temporal, cap);
    assert_eq!(ring.static_in, 3, "all static samples are counted");
    assert_eq!(ring.temporal_in, 6);
    assert_eq!(ring.temporal_kept, 2, "only the newest temporal survive");
    assert_eq!(ring.temporal_dropped, 4);
    let fresh_kept = temporal.iter().filter(|s| s.epoch > 0.0).count();
    assert_eq!(fresh_kept, 2, "the newest temporal samples survive");
    assert_eq!(
        static_catalog.len(),
        3,
        "the static domain is never trimmed"
    );
}

#[test]
fn test_temporal_ring_under_cap_keeps_everything() {
    let mk = |epoch: f64, static_sample: bool| super::Sample {
        source: if static_sample {
            super::SampleSource::Ephemeris
        } else {
            super::SampleSource::Source(0)
        },
        epoch,
        ttl: 1e10,
        extent: if static_sample { f64::INFINITY } else { 0.0 },
        tau: 1e10,
        kernel_id: 0.0,
        force_type: 0.0,
        absorption: 0.0,
        advection: 0.0,
        anchor_vmax: 0.0,
        anchor_amax: 0.0,
        anchor_p0: [0.0, 0.0, 0.0],
        motion: super::Motion::Linear {
            p: [0.0, 0.0, 0.0],
            v: [0.0, 0.0, 0.0],
        },
        val: 1.0,
        name: "temporal_ring_test".into(),
        z: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        color_index: 0.0,
        phase: None,
    };
    let static_catalog: Vec<super::Sample> = vec![mk(0.0, true), mk(0.0, true)];
    let mut temporal: Vec<super::Sample> = (0..5).map(|_| mk(1.0e9, false)).collect();
    let cap = 10;
    let ring = super::temporal_ring(&static_catalog, &mut temporal, cap);
    assert_eq!(ring.static_in, 2);
    assert_eq!(ring.temporal_in, 5);
    assert_eq!(ring.temporal_kept, 5);
    assert_eq!(ring.temporal_dropped, 0);
    assert_eq!(temporal.len(), 5, "no sample drops under the cap");
}

#[test]
fn test_temporal_ring_shared_matches_owned_ring() {
    let mk = |epoch: f64| super::Sample {
        source: super::SampleSource::Source(0),
        epoch,
        ttl: 1e10,
        extent: 0.0,
        tau: 1e10,
        kernel_id: 0.0,
        force_type: 0.0,
        absorption: 0.0,
        advection: 0.0,
        anchor_vmax: 0.0,
        anchor_amax: 0.0,
        anchor_p0: [0.0, 0.0, 0.0],
        motion: super::Motion::Linear {
            p: [0.0, 0.0, 0.0],
            v: [0.0, 0.0, 0.0],
        },
        val: 1.0,
        name: "temporal_ring_shared_test".into(),
        z: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        color_index: 0.0,
        phase: None,
    };
    let static_epochs = [0.0f64, 1.0];
    let temporal_epochs: Vec<f64> = (0..7).map(|i| 2.0 + i as f64).collect();
    let cap = 4;

    let static_owned: Vec<super::Sample> = static_epochs.iter().map(|&e| mk(e)).collect();
    let mut temporal_owned: Vec<super::Sample> = temporal_epochs.iter().map(|&e| mk(e)).collect();
    let owned = super::temporal_ring(&static_owned, &mut temporal_owned, cap);

    let static_shared: Vec<Arc<super::Sample>> =
        static_epochs.iter().map(|&e| Arc::new(mk(e))).collect();
    let mut temporal_shared: Vec<Arc<super::Sample>> =
        temporal_epochs.iter().map(|&e| Arc::new(mk(e))).collect();
    let shared = super::temporal_ring_shared(&static_shared, &mut temporal_shared, cap);

    assert_eq!(owned.static_in, shared.static_in);
    assert_eq!(owned.temporal_in, shared.temporal_in);
    assert_eq!(owned.temporal_kept, shared.temporal_kept);
    assert_eq!(owned.temporal_dropped, shared.temporal_dropped);
    let owned_epochs: Vec<f64> = temporal_owned.iter().map(|s| s.epoch).collect();
    let shared_epochs: Vec<f64> = temporal_shared.iter().map(|s| s.epoch).collect();
    assert_eq!(owned_epochs, shared_epochs, "kept epochs match in order");
    assert_eq!(
        owned_epochs,
        vec![8.0, 7.0, 6.0, 5.0],
        "the newest cap survive"
    );

    let mut under: Vec<Arc<super::Sample>> = temporal_epochs[..3]
        .iter()
        .map(|&e| Arc::new(mk(e)))
        .collect();
    let under_ring = super::temporal_ring_shared(&static_shared, &mut under, 10);
    assert_eq!(under_ring.temporal_dropped, 0);
    assert_eq!(under_ring.temporal_kept, 3);
    assert_eq!(under.len(), 3, "no sample drops under the cap");
}

#[test]
fn test_rebuild_retains_shared_sample_identity() {
    let mk = |epoch: f64| super::Sample {
        source: super::SampleSource::Source(0),
        epoch,
        ttl: 1e10,
        extent: 0.0,
        tau: 1e10,
        kernel_id: 0.0,
        force_type: 0.0,
        absorption: 0.0,
        advection: 0.0,
        anchor_vmax: 0.0,
        anchor_amax: 0.0,
        anchor_p0: [0.0, 0.0, 0.0],
        motion: super::Motion::Linear {
            p: [0.0, 0.0, 0.0],
            v: [0.0, 0.0, 0.0],
        },
        val: 1.0,
        name: "rebuild_shared_test".into(),
        z: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        color_index: 0.0,
        phase: None,
    };
    let origin = Arc::new(mk(0.0));
    let first = super::build_spatial_hash(vec![Arc::clone(&origin)], 1.0);
    let retained: Vec<Arc<super::Sample>> = first
        .cells
        .values()
        .chain(std::iter::once(&first.unbounded))
        .flat_map(|v| v.iter().cloned())
        .collect();
    let second = super::build_spatial_hash(retained, 1.0);
    let again = &second.cells.values().next().expect("cell present")[0];
    assert!(
        Arc::ptr_eq(&origin, again),
        "a retained sample crosses the rebuild as one allocation, not a deep copy"
    );
}

#[test]
fn test_sense_membrane_delivers_sun_sample_with_zero_floor() {
    use std::collections::HashMap;
    let t = 8.0e8;
    let sample = super::Sample {
        source: super::SampleSource::Source(0),
        epoch: t,
        ttl: 60.0,
        extent: 0.0,
        tau: 6.0,
        kernel_id: 0.0,
        force_type: 0.0,
        absorption: 0.0,
        advection: 0.0,
        anchor_vmax: 0.0,
        anchor_amax: 0.0,
        anchor_p0: [0.0, 0.0, 0.0],
        motion: super::Motion::Linear {
            p: [0.0, 0.0, 0.0],
            v: [0.0, 0.0, 0.0],
        },
        val: 4.0e16,
        name: "sun_xray".into(),
        z: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        color_index: 0.0,
        phase: None,
    };
    let cache = super::build_spatial_hash(vec![Arc::new(sample)], 1.0);
    let eph: HashMap<String, super::BodyEphemeris> = HashMap::new();
    let buf = super::Buffer {
        cache,
        eph: Arc::new(eph),
        curves: None,
        spectral: Vec::new(),
        volumes: Vec::new(),
        bayestar: None,
    };
    let mut records: Vec<super::SampleRecord> = Vec::new();
    super::sense_membrane(
        &buf,
        super::MembraneCtx {
            center: [0.0, 0.0, 0.0],
            t2: t + 1.0,
            pad: 3.0e12,
            delta_t_cache: 1.0,
            floor: &[0.0; 9],
            softening: 2.0e9,
            forward: [0.0, 0.0, 1.0],
            eph: &HashMap::new(),
        },
        &mut records,
    );
    assert_eq!(
        records.len(),
        1,
        "the sun sample must reach the window with a zero floor"
    );
    assert_eq!(records[0].3, 4.0e16);
}

#[test]
fn test_parse_ephemeris_binary_v2() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&[0xCF, 0x86, 0x01, 0x00]);
    buf.extend_from_slice(&4u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..56 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&12u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    let params: [f64; 12] = [
        270.0,
        0.003,
        66.54,
        0.013,
        38.31,
        14460.0,
        6378136.6,
        6378136.6,
        6356751.9,
        1.08262668e-3,
        -1.6196e-6,
        3.986_004_354_360_96e14,
    ];
    for p in params {
        buf.extend_from_slice(&p.to_le_bytes());
    }
    buf.extend_from_slice(&2u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..5 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    buf.extend_from_slice(&4u32.to_le_bytes());
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&2451545.0_f64.to_le_bytes());
    buf.extend_from_slice(&16.0_f64.to_le_bytes());
    for c in [1.0_f64, 0.5, 2.0, 0.25, 0.5, 0.125] {
        buf.extend_from_slice(&c.to_le_bytes());
    }
    let eph = super::parse_ephemeris_binary(&buf).unwrap();
    let props = eph.props.unwrap();
    assert_eq!(props.gm, Some(3.986_004_354_360_96e14));
    assert_eq!(props.j2, Some(1.08262668e-3));
    assert_eq!(props.j4, Some(-1.6196e-6));
    assert_eq!(props.radii_b, Some(6378136.6));
    assert_eq!(props.radii_c, Some(6356751.9));
    assert!((props.flattening.unwrap() - (6378136.6 - 6356751.9) / 6378136.6).abs() < 1e-15);
    assert_eq!(eph.granules.len(), 1);
    let deltas = super::nutation_deltas_at(&props, 2451545.0).unwrap();
    assert!((deltas.0 - 1.0).abs() < 1e-12);
    assert!((deltas.1 - 2.0).abs() < 1e-12);
    assert!((deltas.2 - 0.5).abs() < 1e-12);
}

#[test]
fn test_parse_ephemeris_binary_v3_mask() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&[0xCF, 0x86, 0x02, 0x00]);
    buf.extend_from_slice(&3u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..56 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&12u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    let mut params: [f64; 12] = [
        270.0,
        0.003,
        66.54,
        0.013,
        38.31,
        14460.0,
        6378136.6,
        6378136.6,
        6356751.9,
        1.08262668e-3,
        -1.6196e-6,
        3.986_004_354_360_96e14,
    ];
    let mask: u16 = 0xFFFF ^ (1 << 9);
    params[9] = 0.0;
    for p in params {
        buf.extend_from_slice(&p.to_le_bytes());
    }
    buf.extend_from_slice(&mask.to_le_bytes());
    buf.extend_from_slice(&[0u8; 6]);
    buf.extend_from_slice(&2u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..5 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    let eph = super::parse_ephemeris_binary(&buf).unwrap();
    let props = eph.props.unwrap();
    assert_eq!(props.gm, Some(3.986_004_354_360_96e14));
    assert_eq!(props.j2, None);
    assert_eq!(props.j4, Some(-1.6196e-6));
    assert_eq!(props.radii_c, Some(6356751.9));
    assert!((props.flattening.unwrap() - (6378136.6 - 6356751.9) / 6378136.6).abs() < 1e-15);
    assert_eq!(eph.granules.len(), 1);
}

#[test]
fn test_parse_ephemeris_binary_stype2_medium_constants() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&[0xCF, 0x86, 0x01, 0x00]);
    buf.extend_from_slice(&3u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..56 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&12u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..12 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    buf.extend_from_slice(&2u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for &p in &crate::media::medium_params_of("earth")
        .expect("earth row")
        .wire()
    {
        buf.extend_from_slice(&p.to_le_bytes());
    }
    let eph = super::parse_ephemeris_binary(&buf).unwrap();
    let props = eph.props.unwrap();
    assert_eq!(props.gaussian_inverse_square, 340.2);
    assert_eq!(props.gaussian_inverse, 5950.0);
    assert_eq!(props.erfc, 3630.0);
    assert_eq!(props.exponential_decay, 2.18e-5);
    assert_eq!(props.patch_levy, 2.00e-5);
}

#[test]
fn test_parse_ephemeris_binary_stype7_omega_g() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&[0xCF, 0x86, 0x02, 0x00]);
    buf.extend_from_slice(&4u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..56 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&12u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..12 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    buf.extend_from_slice(&(0xFFFFu16).to_le_bytes());
    buf.extend_from_slice(&[0u8; 6]);
    buf.extend_from_slice(&2u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..5 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    buf.extend_from_slice(&7u32.to_le_bytes());
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&1.277e-6_f64.to_le_bytes());
    buf.extend_from_slice(&2.0e-8_f64.to_le_bytes());
    buf.extend_from_slice(&0.0_f64.to_le_bytes());
    buf.extend_from_slice(&0.0_f64.to_le_bytes());
    buf.extend_from_slice(&0.0_f64.to_le_bytes());
    let eph = super::parse_ephemeris_binary(&buf).unwrap();
    let props = eph.props.unwrap();
    let (omega_g, sigma) = props.omega_g.unwrap();
    assert!((omega_g - 1.277e-6).abs() < 1e-18);
    assert!((sigma - 2.0e-8).abs() < 1e-18);
}

#[test]
fn test_fetch_dispatch_gate_admits_em_source() {
    let fc = FieldConfig {
        key: "flux".into(),
        name: "flux".into(),
        kernel: 0,
        force: 0,
        tau: 60.0,
        absorption: 0.0,
        advection: 0.0,
        unit: String::new(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
    };
    let reach = super::dispatch_reach(&[fc], 60.0).expect("em carries a propagation law");
    assert_eq!(reach, C_LIGHT * 60.0 * 64.0);
    let presences: Vec<PresenceSample> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e12, 0.0, 0.0, 0.0, 0.0, 0.0)];
    assert!(
        super::presence_gate(&presences, (0.0, 0.0, 0.0), reach, 0.0, None, None),
        "the em source at the presence anchor must be fetched"
    );
}

#[test]
fn test_fetch_dispatch_gate_window_range_does_not_fetch() {
    let presences: Vec<PresenceSample> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0)];
    assert!(
        !super::presence_gate(&presences, (50.0, 0.0, 0.0), 0.0, 0.0, None, None),
        "the window range is not a fetch radius — an anchor 50 m out stays refused"
    );
    assert!(
        !super::presence_gate(&presences, (1.0e6, 0.0, 0.0), 0.0, 0.0, None, None),
        "an anchor far outside the window stays refused without a physical reach"
    );
}

#[test]
fn test_fetch_dispatch_gate_thermal_reach_governs_geometry() {
    let fc = FieldConfig {
        key: "temp".into(),
        name: "temp".into(),
        kernel: 3,
        force: 5,
        tau: 60.0,
        absorption: 0.0,
        advection: 0.0,
        unit: String::new(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
    };
    let reach = super::dispatch_reach(&[fc], 60.0).expect("thermal carries a propagation law");
    assert_eq!(reach, (2.0 * DIFFUSIVITY_THERMAL * 60.0 * 64.0).sqrt());
    let presences: Vec<PresenceSample> = vec![(8.0e8, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0)];
    assert!(
        super::presence_gate(&presences, (10.0, 0.0, 0.0), reach, 0.0, None, None),
        "the thermal front over the sample lifetime reaches 10 m"
    );
    assert!(
        !super::presence_gate(&presences, (1.0e6, 0.0, 0.0), reach, 0.0, None, None),
        "a thermal anchor 1000 km away is out of physical reach"
    );
}

#[test]
fn test_fetch_gate_rest_rejects_out_of_reach_anchor() {
    let presences: Vec<PresenceSample> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e9, 0.0, 0.0, 0.0, 0.0, 0.0)];
    assert!(
        !super::presence_gate(&presences, (50.0, 0.0, 0.0), 10.0, 5.0, None, None),
        "a resting presence fetches only within reach + extent"
    );
}

#[test]
fn test_fetch_gate_thrust_anticipates_within_median_window() {
    let presences: Vec<PresenceSample> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e9, 100.0, 0.0, 0.0, 0.0, 0.0)];
    assert!(
        super::presence_gate(
            &presences,
            (1000.0, 0.0, 0.0),
            0.0,
            0.0,
            Some([0.0, 0.0, 0.0]),
            Some(20.0)
        ),
        "a presence closing at 100 m/s anticipates 10 s out when the median allows it"
    );
    assert!(
        !super::presence_gate(
            &presences,
            (1000.0, 0.0, 0.0),
            0.0,
            0.0,
            Some([0.0, 0.0, 0.0]),
            Some(5.0)
        ),
        "the anticipation window scales with the median — too short a median stays refused"
    );
}

#[test]
fn test_fetch_gate_thrust_without_anchor_velocity_rests() {
    let presences: Vec<PresenceSample> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e9, 100.0, 0.0, 0.0, 0.0, 0.0)];
    assert!(
        !super::presence_gate(&presences, (1000.0, 0.0, 0.0), 0.0, 0.0, None, Some(20.0)),
        "a frameless anchor carries no velocity — only the rest gate applies"
    );
}

#[test]
fn test_fetch_gate_thrust_without_median_rests() {
    let presences: Vec<PresenceSample> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e9, 100.0, 0.0, 0.0, 0.0, 0.0)];
    assert!(
        !super::presence_gate(
            &presences,
            (1000.0, 0.0, 0.0),
            0.0,
            0.0,
            Some([0.0, 0.0, 0.0]),
            None
        ),
        "without a measured median there is no anticipation — only the rest gate"
    );
}

#[test]
fn test_fetch_gate_thrust_receding_rests() {
    let presences: Vec<PresenceSample> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e9, -100.0, 0.0, 0.0, 0.0, 0.0)];
    assert!(
        !super::presence_gate(
            &presences,
            (1000.0, 0.0, 0.0),
            0.0,
            0.0,
            Some([0.0, 0.0, 0.0]),
            Some(20.0)
        ),
        "a presence receding from the anchor never anticipates"
    );
}

#[test]
fn test_fetch_gate_snap_radius_scales_with_grid_step() {
    let presences: Vec<PresenceSample> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e12, 0.0, 0.0, 0.0, 0.0, 1.0e8)];
    let snap = super::Φ * 1.0e8;
    assert!(
        super::presence_gate(&presences, (snap - 1.0, 0.0, 0.0), 0.0, 0.0, None, None),
        "the snap radius reaches one golden grid step beyond a bodyless anchor"
    );
    assert!(
        !super::presence_gate(&presences, (snap + 1.0, 0.0, 0.0), 0.0, 0.0, None, None),
        "beyond the golden grid step the rest gate refuses"
    );
}

#[test]
fn test_fetch_gate_snap_radius_respects_body_radius() {
    let presences: Vec<PresenceSample> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e12, 0.0, 0.0, 0.0, 0.0, 1.0)];
    assert!(
        super::presence_gate(&presences, (6.0e6, 0.0, 0.0), 0.0, 6.0e6, None, None),
        "the body radius dominates a tiny grid step — the surface is in reach"
    );
    assert!(
        !super::presence_gate(&presences, (6.0e6 + 1.0, 0.0, 0.0), 0.0, 6.0e6, None, None),
        "one metre beyond the body radius stays refused"
    );
}

#[test]
fn test_fetch_duration_ring_median_and_wrap() {
    let mut ring = [0.0_f64; super::FETCH_DURATION_RING];
    let mut len = 0usize;
    let mut idx = 0usize;
    assert!(
        super::median_fetch_duration(&ring, len).is_none(),
        "an empty ring carries no median"
    );
    for d in [4.0, 2.0, 8.0, 6.0] {
        super::record_fetch_duration(&mut ring, &mut len, &mut idx, d);
    }
    assert_eq!(len, 4);
    let median = super::median_fetch_duration(&ring, len).expect("four durations carry a median");
    assert_eq!(median, 5.0, "the median of [4, 2, 8, 6] is 5");
    for d in 0..20 {
        super::record_fetch_duration(&mut ring, &mut len, &mut idx, 10.0 + d as f64);
    }
    assert_eq!(len, super::FETCH_DURATION_RING, "the ring caps at 2^4");
    assert!(super::median_fetch_duration(&ring, len).is_some());
}

#[test]
fn test_chebyshev_evaluate_deriv_matches_basis_slopes() {
    let mut c1 = [0.0_f64; super::CHEBYSHEV_N];
    c1[1] = 1.0;
    for tau in [-0.9, -0.3, 0.0, 0.5, 0.9] {
        let d = super::chebyshev_evaluate_deriv(&c1, tau);
        assert!((d - 1.0).abs() < 1e-12, "T1 = tau derives to 1, was {d}");
    }
    let mut c2 = [0.0_f64; super::CHEBYSHEV_N];
    c2[2] = 1.0;
    for tau in [-0.9, -0.3, 0.0, 0.5, 0.9] {
        let d = super::chebyshev_evaluate_deriv(&c2, tau);
        assert!(
            (d - 4.0 * tau).abs() < 1e-12,
            "T2 = 2tau^2-1 derives to 4tau, was {d}"
        );
    }
}

#[test]
fn test_body_barycenter_position_at_granule_boundary() {
    for i in 0..32 {
        let t0_jd = 2459000.0 + i as f64 * 0.7;
        let dt_jd = 0.03 + i as f64 * 0.013;
        let boundary_jd = t0_jd + dt_jd;
        let mut eph = super::BodyEphemeris {
            granules: Vec::new(),
            rotation_matrices: Vec::new(),
            props: None,
            orbit: None,
            granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        };
        let mut left = [0.0_f64; super::CHEBYSHEV_N];
        left[0] = 1.0e9;
        let mut right = [0.0_f64; super::CHEBYSHEV_N];
        right[0] = 2.0e9;
        eph.granules.push(super::ChebyshevGranule {
            t0_jd,
            dt_jd,
            cx: left,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        });
        eph.granules.push(super::ChebyshevGranule {
            t0_jd: boundary_jd + dt_jd,
            dt_jd,
            cx: right,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        });
        let mut map = std::collections::HashMap::new();
        map.insert("boundary".to_string(), eph);
        let tdb = (boundary_jd - super::J2000_EPOCH) * 86400.0;
        let p = match super::body_barycenter_position("boundary", tdb, &map) {
            Some(p) => p,
            None => panic!("boundary {boundary_jd} (t0 {t0_jd}, dt {dt_jd}) uncovered"),
        };
        assert_eq!(p[0], 1.0e9);
        let v = match super::body_barycenter_velocity("boundary", tdb, &map) {
            Some(v) => v,
            None => panic!("boundary {boundary_jd} uncovered by the velocity lookup"),
        };
        assert!(v[0].is_finite());
    }
}

#[test]
fn test_body_barycenter_velocity_linear_granule() {
    let now = 840511523.88;
    let jd_now = super::J2000_EPOCH + now / 86400.0;
    let mut eph = super::BodyEphemeris {
        granules: Vec::new(),
        rotation_matrices: Vec::new(),
        props: None,
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    for i in -1..=1 {
        let t0 = jd_now + i as f64 * 16.0;
        let mut cx = [0.0_f64; super::CHEBYSHEV_N];
        cx[1] = 1.0e6;
        eph.granules.push(super::ChebyshevGranule {
            t0_jd: t0,
            dt_jd: 16.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        });
    }
    let mut eph_map = std::collections::HashMap::new();
    eph_map.insert("earth".to_string(), eph);
    let v = super::body_barycenter_velocity("earth", now, &eph_map).expect("earth velocity");
    let expect = 1.0e6 / (16.0 * 86400.0);
    assert!(
        (v[0] - expect).abs() < 1e-6,
        "a linear granule (cx[1] = 1e6 m per half-width) moves at 1e6/(16*86400) m/s, was {}",
        v[0]
    );
    assert_eq!(v[1], 0.0);
    assert_eq!(v[2], 0.0);
}

#[test]
fn test_fetch_dispatch_gate_forceless_field_refused() {
    let fc = FieldConfig {
        key: "x".into(),
        name: "x".into(),
        kernel: 0,
        force: 9,
        tau: 60.0,
        absorption: 0.0,
        advection: 0.0,
        unit: String::new(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
    };
    assert!(
        super::dispatch_reach(&[fc], 60.0).is_none(),
        "a force without a propagation law is refused, never 0.0"
    );
}

#[test]
fn test_extract_fields_reads_profile_map() {
    let fc = field_fixture("temp", 60.0);
    let ext = Extract::ProfileMap {
        arr_path: ".".into(),
        lat_key: "lat".into(),
        lon_key: "lon".into(),
        epoch_key: "t".into(),
        pressure_var: "pressure".into(),
        pressure_scale: 1.0,
        fields: vec![fc.clone()],
    };
    let fields = super::extract_fields(&ext);
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].key, "temp");
    let reach = super::dispatch_reach(&fields, 60.0).expect("the profile field gates");
    assert_eq!(reach, C_LIGHT * 60.0 * 64.0);
}

#[test]
fn test_extract_fields_reads_geojson_events() {
    let ext = Extract::GeojsonEvents {
        mag_key: "mag".into(),
        min_mag: 0.0,
        outputs: vec!["seismic_magnitude_mw".into(), "seismic_depth_km".into()],
        tau: 6.0,
        absorption: 0.0,
        advection: 0.0,
        mag_type_key: String::new(),
    };
    let fields = super::extract_fields(&ext);
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "seismic_magnitude_mw");
    assert_eq!(fields[0].force, 3);
    assert_eq!(fields[1].name, "seismic_depth_km");
    let reach = super::dispatch_reach(&fields, 60.0).expect("the geojson fields gate");
    assert_eq!(reach, SEISMIC_BODY_SPEED * 60.0 * 64.0);
    let single = Extract::GeojsonEvents {
        mag_key: "mag".into(),
        min_mag: 0.0,
        outputs: vec!["seismic_magnitude_mw".into()],
        tau: 6.0,
        absorption: 0.0,
        advection: 0.0,
        mag_type_key: String::new(),
    };
    assert!(
        super::extract_fields(&single).is_empty(),
        "a geojson extract with one output emits nothing"
    );
}

#[test]
fn test_extract_fields_reads_quakeml_events() {
    let ext = Extract::QuakeMlEvents {
        outputs: vec!["usgs_mt_m0_nm".into(), "usgs_mt_mww".into()],
        tau: 6.0,
        absorption: 0.0,
        advection: 0.0,
    };
    let fields = super::extract_fields(&ext);
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "usgs_mt_m0_nm");
    assert_eq!(fields[0].kernel, 1);
    assert_eq!(fields[0].force, 3);
    assert_eq!(fields[0].unit, "N m");
    assert_eq!(fields[1].name, "usgs_mt_mww");
    assert_eq!(fields[1].kernel, 3);
    assert_eq!(fields[1].force, 4);
    assert_eq!(fields[1].unit, "Mw");
    let reach = super::dispatch_reach(&fields, 60.0).expect("the quakeml fields gate");
    assert_eq!(reach, SEISMIC_BODY_SPEED * 60.0 * 64.0);
    let single = Extract::QuakeMlEvents {
        outputs: vec!["usgs_mt_m0_nm".into()],
        tau: 6.0,
        absorption: 0.0,
        advection: 0.0,
    };
    assert!(
        super::extract_fields(&single).is_empty(),
        "a quakeml extract with one output emits nothing"
    );
}

const QUAKEML_BLOCK: &str = "url https://example.org/mt\nttl 60\non earth 0 0 0\nformat quakeml\nquakeml usgs_mt_m0_nm usgs_mt_mww 6.0 0.0 0.0\n";

const QUAKEML_TENSOR_BODY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<q:quakeml xmlns="http://quakeml.org/xmlns/bed/1.2">
 <eventParameters>
  <event publicID="quakeml:test/event">
   <origin publicID="quakeml:test/origin">
    <time><value>2023-02-06T01:17:34.342Z</value></time>
    <latitude><value>37.2256</value></latitude>
    <longitude><value>37.0143</value></longitude>
    <depth><value>10000</value></depth>
   </origin>
   <magnitude publicID="quakeml:test/mag"><mag><value>7.8</value></mag><type>mww</type></magnitude>
   <focalMechanism publicID="quakeml:test/mww">
    <momentTensor publicID="quakeml:test/mww#mt">
     <scalarMoment><value>5.39E+20</value></scalarMoment>
    </momentTensor>
   </focalMechanism>
  </event>
 </eventParameters>
</q:quakeml>"#;

fn quakeml_fixture_lsk() -> super::LeapSeconds {
    super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    }
}

#[test]
fn test_quakeml_events_emit_m0_and_mww_at_epicenter_epoch() {
    let srcs = parse_sources(QUAKEML_BLOCK);
    assert_eq!(srcs.len(), 1);
    let lsk = quakeml_fixture_lsk();
    let now = 1.7e9;
    match super::extract(&srcs[0], QUAKEML_TENSOR_BODY, now, &lsk) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 2);
            let (m0, fc0) = &v[0];
            assert_eq!(m0.name, "usgs_mt_m0_nm");
            assert_eq!(m0.value, 5.39e20);
            assert_eq!(fc0.unit, "N m");
            assert_eq!(fc0.kernel, 1);
            assert_eq!(fc0.force, 3);
            assert!((m0.z - 10.0).abs() < 1e-9);
            let expected = lsk.unix_to_tdb(1675646254.342).unwrap();
            assert!((m0.epoch - expected).abs() < 1e-6);
            match &m0.position {
                super::Position::Surface {
                    body_name,
                    lat,
                    lon,
                    alt,
                } => {
                    assert_eq!(body_name, "earth");
                    assert!((lat - 37.2256).abs() < 1e-9);
                    assert!((lon - 37.0143).abs() < 1e-9);
                    assert_eq!(*alt, 0.0);
                }
                other => panic!("position variant: {:?} unexpected", other),
            }
            let (mww, fc1) = &v[1];
            assert_eq!(mww.name, "usgs_mt_mww");
            assert!((mww.value - 7.8).abs() < 1e-9);
            assert_eq!(fc1.unit, "Mw");
            assert_eq!(fc1.kernel, 3);
            assert_eq!(fc1.force, 4);
            assert!((mww.epoch - expected).abs() < 1e-6);
        }
        _ => panic!("extract variant unexpected"),
    }
}

#[test]
fn test_quakeml_absent_scalar_moment_emits_no_m0() {
    let srcs = parse_sources(QUAKEML_BLOCK);
    assert_eq!(srcs.len(), 1);
    let body = r#"<?xml version="1.0" encoding="UTF-8"?>
<q:quakeml xmlns="http://quakeml.org/xmlns/bed/1.2">
 <eventParameters>
  <event publicID="quakeml:test/event">
   <origin publicID="quakeml:test/origin">
    <time><value>2023-02-06T01:17:34.342Z</value></time>
    <latitude><value>37.2256</value></latitude>
    <longitude><value>37.0143</value></longitude>
    <depth><value>10000</value></depth>
   </origin>
   <magnitude publicID="quakeml:test/mag"><mag><value>7.8</value></mag><type>mww</type></magnitude>
  </event>
 </eventParameters>
</q:quakeml>"#;
    let lsk = quakeml_fixture_lsk();
    match super::extract(&srcs[0], body, 1.7e9, &lsk) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 1);
            assert_eq!(v[0].0.name, "usgs_mt_mww");
            assert!(
                v.iter().all(|(c, _)| c.name != "usgs_mt_m0_nm"),
                "an absent scalarMoment emits no M0 channel"
            );
        }
        _ => panic!("extract variant unexpected"),
    }
}

const ARPANSA_UV_BODY: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<stations>
  <location id="Adelaide">
    <name>adl</name>
    <index>1.3</index>
    <time>4:05 PM</time>
    <date>18/09/2026</date>
    <fulldate>Friday, 18 September 2026</fulldate>
    <utcdatetime>2026/09/18 06:35</utcdatetime>
    <status>ok</status>
  </location>
  <location id="Alice Springs">
    <name>asp</name>
    <index>7.8</index>
    <utcdatetime>2026/09/18 06:05</utcdatetime>
    <status>ok</status>
  </location>
</stations>"#;

#[test]
fn test_arpansa_uv_xml_emits_station_channels() {
    let src = source_fixture(
        "arpansa",
        vec![Extract::Field(field_fixture("uv_index", 60.0))],
    );
    match super::extract(&src, ARPANSA_UV_BODY, 1.7e9, &fixture_lsk()) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 2);
            assert_eq!(v[0].0.name, "Adelaide");
            assert_eq!(v[0].0.value, 1.3);
            assert_eq!(v[1].0.name, "Alice Springs");
            assert_eq!(v[1].0.value, 7.8);
            assert_eq!(v[0].1.force, 0);
            assert_eq!(v[0].1.kernel, 0);
            assert_eq!(v[0].1.unit, "UVI");
            assert!(matches!(
                &v[0].0.position,
                super::Position::Surface { lat, lon, .. }
                    if (*lat - -34.95).abs() < 1e-9 && (*lon - 138.52).abs() < 1e-9
            ));
        }
        _ => panic!("extract variant unexpected"),
    }
}

#[test]
fn test_extract_igra_zip_emits_level_measurements() {
    let header = format!(
        "#{:<11} {:<4} {:<2} {:<2} {:<2} {:<4} {:>4} {:<8} {:<8} {:>7} {:>8}",
        "USM00070026",
        "2026",
        "01",
        "01",
        "00",
        "2330",
        1usize,
        "ncdc-nws",
        "ncdc-gts",
        "712889",
        "-1567833"
    );
    let data = format!(
        "{:<2} {:>5} {:>6}{}{:>5}{}{:>5}{}{:>5} {:>5} {:>5} {:>5}",
        "21", "0", "103574", "B", "14", " ", "-254", "B", "820", "22", "329", "62"
    );
    let text = format!("{header}\n{data}\n");
    let mut zip = Vec::new();
    zip.extend_from_slice(b"PK\x03\x04");
    zip.extend_from_slice(&20u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u32.to_le_bytes());
    zip.extend_from_slice(&(text.len() as u32).to_le_bytes());
    zip.extend_from_slice(&(text.len() as u32).to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(text.as_bytes());
    let path = std::env::temp_dir().join("omegaflow_test_igra.zip");
    std::fs::write(&path, &zip).unwrap();
    let src = source_fixture("igra_zip", vec![]);
    match super::extract(&src, &path.to_string_lossy(), 1.7e9, &fixture_lsk()) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 6);
            let press = v
                .iter()
                .find(|(c, _)| c.name == "igra_air_pressure_hpa")
                .expect("pressure channel");
            assert_eq!(press.0.value, 1035.74);
            assert_eq!(press.1.force, 7);
            assert_eq!(press.1.kernel, 5);
            assert_eq!(press.1.unit, "hPa");
            let temp = v
                .iter()
                .find(|(c, _)| c.name == "igra_air_temp_c")
                .expect("temperature channel");
            assert_eq!(temp.0.value, -25.4);
            assert_eq!(temp.1.force, 5);
            assert_eq!(temp.1.kernel, 4);
            assert_eq!(temp.1.unit, "C");
            if let super::Position::Surface { lat, lon, alt, .. } = &press.0.position {
                assert!((lat - 71.2889).abs() < 1e-9);
                assert!((lon - -156.7833).abs() < 1e-9);
                assert_eq!(*alt, 14.0);
            } else {
                panic!("position is not Surface");
            }
        }
        _ => panic!("extract variant unexpected"),
    }
}

fn fugin_cube_fixture() -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut header: Vec<u8> = Vec::new();
    header.extend_from_slice(&fits_card("SIMPLE", "T"));
    header.extend_from_slice(&fits_card("BITPIX", "-32"));
    header.extend_from_slice(&fits_card("NAXIS", "3"));
    header.extend_from_slice(&fits_card("NAXIS1", "3"));
    header.extend_from_slice(&fits_card("NAXIS2", "3"));
    header.extend_from_slice(&fits_card("NAXIS3", "4"));
    header.extend_from_slice(&fits_card("CTYPE1", "'GLON-SFL'"));
    header.extend_from_slice(&fits_card("CTYPE2", "'GLAT-SFL'"));
    header.extend_from_slice(&fits_card("CTYPE3", "'VRAD'"));
    header.extend_from_slice(&fits_card("CRVAL1", "20.0"));
    header.extend_from_slice(&fits_card("CRVAL2", "0.0"));
    header.extend_from_slice(&fits_card("CRVAL3", "-99675.0"));
    header.extend_from_slice(&fits_card("CRPIX1", "2.0"));
    header.extend_from_slice(&fits_card("CRPIX2", "2.0"));
    header.extend_from_slice(&fits_card("CDELT1", "0.5"));
    header.extend_from_slice(&fits_card("CDELT2", "0.5"));
    header.extend_from_slice(&fits_card("CDELT3", "650.0"));
    header.extend_from_slice(&fits_card("CUNIT3", "'m/s'"));
    header.extend_from_slice(&fits_card("BTYPE", "'Intensity'"));
    header.extend_from_slice(&fits_card("BUNIT", "'K'"));
    header.extend_from_slice(&fits_card("END", ""));
    while !header.len().is_multiple_of(2880) {
        header.extend_from_slice(&[b' '; 80]);
    }
    buf.extend_from_slice(&header);
    let (nx, ny, nz) = (3usize, 3usize, 4usize);
    let mut data = vec![f32::NAN; nx * ny * nz];
    let mut set = |x: usize, y: usize, z: usize, v: f32| {
        data[z * (ny * nx) + y * nx + x] = v;
    };
    set(1, 1, 0, 1.0);
    set(1, 1, 1, 2.0);
    set(1, 1, 2, f32::NAN);
    set(1, 1, 3, 3.0);
    for z in 0..nz {
        set(2, 2, z, 5.0);
    }
    for v in data {
        buf.extend_from_slice(&v.to_be_bytes());
    }
    while !buf.len().is_multiple_of(2880) {
        buf.push(0);
    }
    buf
}

#[test]
fn test_fugin_cube_moment0_integrates_and_maps_positions() {
    let buf = fugin_cube_fixture();
    let pixels = super::fugin::parse_fugin_cube(&buf).expect("3x3x4 cube parses");
    assert_eq!(
        pixels.len(),
        2,
        "all-NaN pixels stay absent, two pixels carry finite channels"
    );
    let center = pixels
        .iter()
        .find(|p| (p.moment0_k_ms - 3900.0).abs() < 1e-9)
        .expect("center pixel: finite 1.0+2.0+3.0 over 650 m/s");
    assert!(center.ra_deg.is_finite() && center.dec_deg.is_finite());
    let (ra_exp, dec_exp) = crate::mathematikerin::healpix::galactic_to_icrs(
        (90.0f64).to_radians(),
        20.0f64.to_radians(),
    );
    assert!(
        (center.ra_deg - ra_exp).abs() < 1e-9,
        "glon=20 glat=0 maps to ICRS through the theta/phi convention"
    );
    assert!((center.dec_deg - dec_exp).abs() < 1e-9);
    let corner = pixels
        .iter()
        .find(|p| (p.moment0_k_ms - 13000.0).abs() < 1e-9)
        .expect("corner pixel: 4 x 5.0 over 650 m/s");
    let (ra2, dec2) = crate::mathematikerin::healpix::galactic_to_icrs(
        (90.0f64 - 0.5).to_radians(),
        20.5f64.to_radians(),
    );
    assert!((corner.ra_deg - ra2).abs() < 1e-9);
    assert!((corner.dec_deg - dec2).abs() < 1e-9);
}

#[test]
fn test_extract_fugin_cube_emits_state_vector_channels() {
    let buf = fugin_cube_fixture();
    let path = std::env::temp_dir().join("omegaflow_test_fugin.fits");
    std::fs::write(&path, &buf).unwrap();
    let src = source_fixture(
        "fugin_cube",
        vec![Extract::Field(field_fixture("fugin_moment0", 31536000.0))],
    );
    match super::extract(&src, &path.to_string_lossy(), 1.7e9, &fixture_lsk()) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 2);
            let center = v
                .iter()
                .find(|(c, _)| (c.value - 3900.0).abs() < 1e-9)
                .expect("center channel");
            assert_eq!(center.1.force, 0);
            assert_eq!(center.1.kernel, 0);
            assert_eq!(center.1.unit, "K m/s");
            assert_eq!(center.0.name, "fugin_moment0");
            let (ra_exp, dec_exp) = crate::mathematikerin::healpix::galactic_to_icrs(
                (90.0f64).to_radians(),
                20.0f64.to_radians(),
            );
            let ra = ra_exp.to_radians();
            let dec = dec_exp.to_radians();
            let (sa, ca) = ra.sin_cos();
            let (sd, cd) = dec.sin_cos();
            let expected = [cd * ca, cd * sa, sd];
            if let super::Position::StateVector { p, .. } = &center.0.position {
                for i in 0..3 {
                    assert!((p[i] - expected[i]).abs() < 1e-9);
                }
            } else {
                panic!("position is not StateVector");
            }
        }
        _ => panic!("extract variant unexpected"),
    }
}

#[test]
fn test_fetch_dispatch_gate_advective_uses_field_advection() {
    let fc = FieldConfig {
        key: "wind".into(),
        name: "wind".into(),
        kernel: 5,
        force: 7,
        tau: 60.0,
        absorption: 0.0,
        advection: 400000.0,
        unit: String::new(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
    };
    let reach = super::dispatch_reach(&[fc], 60.0).expect("advective carries a propagation law");
    assert_eq!(reach, 400000.0 * 60.0 * 64.0);
}

fn origin_fixture(failures: u32, in_flight: bool) -> super::OriginState {
    super::OriginState {
        fetched: 0.0,
        started: 0.0,
        prev_epoch: 0.0,
        prev_abs: [0.0, 0.0, 0.0],
        prev_motion: None,
        resid_ema: 0.0,
        has_prev: false,
        failures,
        in_flight,
    }
}

#[test]
fn test_origin_stale_holds_while_fetch_in_flight() {
    let mut origins = std::collections::HashMap::new();
    origins.insert(0u32, origin_fixture(0, true));
    assert!(
        !super::origin_stale(&origins, 0, 60, 1.0e9, None),
        "a running fetch blocks the re-dispatch, however stale the origin"
    );
    origins.insert(0u32, origin_fixture(0, false));
    assert!(
        super::origin_stale(&origins, 0, 60, 1.0e9, None),
        "a settled stale origin dispatches again"
    );
}

#[test]
fn test_origin_stale_jump_epoch_forces_redispatch() {
    let mut origins = std::collections::HashMap::new();
    let mut settled = origin_fixture(0, false);
    settled.fetched = 1.0e9;
    origins.insert(0u32, settled);
    assert!(
        !super::origin_stale(&origins, 0, 60, 1.0e9 + 1.0, None),
        "without a jump the origin stays held inside the backoff"
    );
    assert!(
        super::origin_stale(&origins, 0, 60, 1.0e9 + 1.0, Some(1.0e9 + 0.5)),
        "a jump after the last settle reopens the origin"
    );
    assert!(
        !super::origin_stale(&origins, 0, 60, 1.0e9 + 1.0, Some(1.0e9 - 0.5)),
        "a jump before the last settle leaves the origin held"
    );
}

#[test]
fn test_fetch_void_backoff_grows_power_of_two_and_caps() {
    for failures in 0..=super::FETCH_VOID_CAP + 2 {
        let mut origins = std::collections::HashMap::new();
        origins.insert(0u32, origin_fixture(failures, false));
        let factor = 2f64.powi(failures.min(super::FETCH_VOID_CAP) as i32);
        let backoff = 60.0 / super::Φ * factor;
        assert!(
            !super::origin_stale(&origins, 0, 60, backoff - 0.5, None),
            "failures {}: fresh inside the backoff stays held",
            failures
        );
        assert!(
            super::origin_stale(&origins, 0, 60, backoff + 0.5, None),
            "failures {}: the backoff ttl/Φ·2ⁿ expires",
            failures
        );
    }
}

#[test]
fn test_connect_timeout_is_declared_not_derived_from_ttl() {
    assert_eq!(
        super::CONNECT_BOUND_S,
        32,
        "connect bound is a declared power-of-two handshake budget"
    );
    assert!(
        super::ttl_transfer_bound(86400) > super::CONNECT_BOUND_S,
        "the payload transfer budget stays ttl-scaled and exceeds the connect bound"
    );
    assert_eq!(
        super::ttl_transfer_bound(86400),
        ((86400.0) / (super::Φ * super::Φ)).ceil() as u64,
        "transfer scales with ttl; connect does not"
    );
    assert_eq!(
        super::ttl_transfer_bound(1),
        ((1.0) / (super::Φ * super::Φ)).ceil() as u64,
        "a tiny ttl keeps a tiny transfer budget"
    );
}

#[test]
fn test_settle_fetch_resets_voids_on_ok_and_caps_on_void() {
    let mut st = origin_fixture(3, true);
    super::settle_fetch(&mut st, true, 100.0);
    assert_eq!(st.failures, 0, "a delivered fetch resets the void count");
    assert!(!st.in_flight);
    assert_eq!(st.fetched, 100.0);
    super::settle_fetch(&mut st, false, 200.0);
    assert_eq!(st.failures, 1, "a fetch void counts one failure");
    st.failures = super::FETCH_VOID_CAP;
    super::settle_fetch(&mut st, false, 300.0);
    assert_eq!(
        st.failures,
        super::FETCH_VOID_CAP,
        "the void count caps at the power-of-2 ceiling"
    );
}

#[test]
fn test_origin_clock_roundtrip_keeps_stale_verdict() {
    let dir = std::env::temp_dir().join(format!("omegaflow_origin_clock_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("origin_clock.φ").to_string_lossy().into_owned();
    let mut clock = std::collections::HashMap::new();
    clock.insert("https://example.com/a".to_string(), (1.0e9, 0));
    clock.insert("https://example.com/b".to_string(), (1.0e9 - 30.0, 3));
    super::save_origin_clock(&path, &clock);
    let loaded = super::load_origin_clock(&path);
    assert_eq!(loaded.len(), 2, "both clock rows survive the roundtrip");
    for (url, (fetched, failures)) in &loaded {
        let (orig_fetched, orig_failures) = clock.get(url).unwrap();
        assert_eq!(*fetched, *orig_fetched, "fetched survives the roundtrip");
        assert_eq!(*failures, *orig_failures, "failures survives the roundtrip");
        for now in [1.0e9 - 60.0, 1.0e9, 1.0e9 + 1.0e3, 1.0e9 + 1.0e4] {
            assert_eq!(
                super::stale_after(*fetched, *failures, 60, now),
                super::stale_after(*orig_fetched, *orig_failures, 60, now),
                "the reloaded clock yields the identical stale verdict"
            );
        }
    }
    let missing = dir.join("never_written.φ").to_string_lossy().into_owned();
    assert!(
        super::load_origin_clock(&missing).is_empty(),
        "an absent clock file loads absent"
    );
    let foreign = dir.join("foreign_version.φ").to_string_lossy().into_owned();
    let _ = std::fs::write(&foreign, "origin_clock 2\n1.0e9 0 https://example.com/a\n");
    assert!(
        super::load_origin_clock(&foreign).is_empty(),
        "a foreign header version loads absent"
    );
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&foreign);
    let _ = std::fs::remove_dir(&dir);
}

fn rfc1123_from_unix(secs: u64) -> String {
    let (y, m, d) = super::civil_date(secs);
    let tod = secs % 86400;
    let hh = tod / 3600;
    let mm = (tod % 3600) / 60;
    let ss = tod % 60;
    let wd = (secs / 86400 + 4) % 7;
    let weekdays = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];
    format!(
        "{}, {:02} {} {} {:02}:{:02}:{:02} GMT",
        weekdays[wd as usize],
        d,
        super::month_abbr(m),
        y,
        hh,
        mm,
        ss
    )
}

fn local_http_head(
    last_modified: String,
    connections: usize,
) -> (String, std::thread::JoinHandle<()>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let handle = std::thread::spawn(move || {
        for _ in 0..connections {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 4096];
                let _ = std::io::Read::read(&mut stream, &mut buf);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nLast-Modified: {}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                    last_modified
                );
                let _ = std::io::Write::write_all(&mut stream, response.as_bytes());
            }
        }
    });
    (format!("http://127.0.0.1:{}", port), handle)
}

#[test]
fn test_cdn_fresh_uses_ttl_alone_without_floor() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (under_url, under) = local_http_head(rfc1123_from_unix(now - 55), 1);
    assert!(
        super::cdn_fresh(&under_url, 60),
        "an asset 55 s old is fresh under ttl 60 — the ttl alone gates"
    );
    under.join().unwrap();
    let (over_url, over) = local_http_head(rfc1123_from_unix(now - 61), 1);
    assert!(
        !super::cdn_fresh(&over_url, 60),
        "an asset 61 s old is stale under ttl 60 — no 300 s floor"
    );
    over.join().unwrap();
}

fn local_http_head_get(
    stale_lm: String,
    body: &'static str,
) -> (String, std::thread::JoinHandle<()>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let port = listener.local_addr().unwrap().port();
    let handle = std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while std::time::Instant::now() < deadline {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0u8; 4096];
                    let n = match std::io::Read::read(&mut stream, &mut buf) {
                        Ok(n) => n,
                        Err(_) => continue,
                    };
                    let req = String::from_utf8_lossy(&buf[..n]).to_string();
                    let response = if req.starts_with("HEAD") {
                        format!(
                            "HTTP/1.1 200 OK\r\nLast-Modified: {}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                            stale_lm
                        )
                    } else if req.starts_with("GET /live-void.json ") {
                        "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                            .to_string()
                    } else {
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            body.len(),
                            body
                        )
                    };
                    let _ = std::io::Write::write_all(&mut stream, response.as_bytes());
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(20));
                }
                Err(_) => break,
            }
        }
    });
    (format!("http://127.0.0.1:{}", port), handle)
}

#[test]
fn test_fetch_one_serves_stale_cdn_asset_when_live_voids() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (server, handle) = local_http_head_get(rfc1123_from_unix(now - 86400), "{\"real\":true}");
    let state = "/tmp/opencode/omegaflow_fetch_fallback_state";
    let _ = std::fs::remove_dir_all(state);
    unsafe {
        std::env::set_var("OMEGAFLOW_STATE", state);
        std::env::set_var("OMEGAFLOW_CDN_BASE", &server);
    }
    let live = format!("{}/live-void.json", server);
    let body = super::fetch_one(&live, None, &[], 60, Some(1.0e9));
    unsafe {
        std::env::remove_var("OMEGAFLOW_CDN_BASE");
        std::env::remove_var("OMEGAFLOW_STATE");
    }
    handle.join().unwrap();
    assert_eq!(
        body.as_deref(),
        Some("{\"real\":true}"),
        "a real stale CDN asset is served when the live path voids — 0 honored only where the CDN is truly absent"
    );
}

#[test]
fn test_live_only_source_skips_the_cdn_fallback_when_live_voids() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (server, handle) = local_http_head_get(rfc1123_from_unix(now - 86400), "{\"real\":true}");
    let state = "/tmp/opencode/omegaflow_live_only_state";
    let _ = std::fs::remove_dir_all(state);
    unsafe {
        std::env::set_var("OMEGAFLOW_STATE", state);
        std::env::set_var("OMEGAFLOW_CDN_BASE", &server);
    }
    let live = format!("{}/live-void.json", server);
    let result = super::fetch_one_with_age(&live, None, &[], 60, Some(1.0e9), true);
    unsafe {
        std::env::remove_var("OMEGAFLOW_CDN_BASE");
        std::env::remove_var("OMEGAFLOW_STATE");
    }
    handle.join().unwrap();
    assert!(
        result.is_none(),
        "a live-only source voids without probing the CDN — no mirror exists for a rolling window"
    );
}

#[test]
fn test_url_has_fixed_window_distinguishes_static_from_live() {
    assert!(
        super::url_has_fixed_window(
            "https://vires.services/hapi/data?id=CS_OPER_MAG&start=2018-10-10T00:00:00Z&stop=2018-10-10T00:59:59Z&parameters=F&format=json"
        ),
        "a fixed start/stop window is static"
    );
    assert!(
        super::url_has_fixed_window(
            "https://archive-api.open-meteo.com/v1/archive?start_date=2026-08-18&end_date=2026-08-27&hourly=temperature_2m"
        ),
        "a fixed start_date/end_date archive window is static"
    );
    assert!(
        !super::url_has_fixed_window(
            "https://services.swpc.noaa.gov/json/goes/primary/differential-protons-1-day.json"
        ),
        "a rolling 1-day window is live"
    );
    assert!(
        !super::url_has_fixed_window(
            "https://cdaweb.gsfc.nasa.gov/hapi/data?id=OMNI2_H0_MRG1HR&time.min={week_ago}T00:00:00Z&time.max={now}Z"
        ),
        "a relative placeholder window is live"
    );
    assert!(
        !super::url_has_fixed_window(
            "https://earthquake.usgs.gov/fdsnws/event/1/query?starttime={hour_ago}&minmagnitude=2.0"
        ),
        "an hour_ago placeholder is live"
    );
}

#[test]
fn test_fetch_one_with_age_carries_the_cdn_age_on_fallback() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (server, handle) = local_http_head_get(rfc1123_from_unix(now - 86400), "{\"real\":true}");
    let state = "/tmp/opencode/omegaflow_fetch_age_state";
    let _ = std::fs::remove_dir_all(state);
    unsafe {
        std::env::set_var("OMEGAFLOW_STATE", state);
        std::env::set_var("OMEGAFLOW_CDN_BASE", &server);
    }
    let live = format!("{}/live-void.json", server);
    let (body, age) = super::fetch_one_with_age(&live, None, &[], 60, Some(1.0e9), false).unwrap();
    unsafe {
        std::env::remove_var("OMEGAFLOW_CDN_BASE");
        std::env::remove_var("OMEGAFLOW_STATE");
    }
    handle.join().unwrap();
    assert_eq!(body.as_str(), "{\"real\":true}");
    let age = age.expect("the served CDN body carries its measured age");
    assert!(
        (86400..=86400 + 5).contains(&age),
        "the served asset is one day stale (age {})",
        age
    );
}

#[test]
fn test_cache_fresh_cdn_stamp_equality_and_release_branch() {
    let dir = std::env::temp_dir().join(format!("omegaflow_cdn_stamp_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("asset.json").to_string_lossy().into_owned();
    let _ = std::fs::write(&path, "payload");
    assert!(
        super::cache_fresh_cdn(&path, 3600, "https://cdn.example.org/x/asset.json"),
        "a fresh local cache with a non-release URL is fresh — no CDN stamp read"
    );
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let lm = rfc1123_from_unix(now - 500);
    let (server, handle) = local_http_head(lm.clone(), 1);
    let release_url = format!("{}/releases/download/v1/asset.json", server);
    assert!(
        !super::cache_fresh_cdn(&path, 3600, &release_url),
        "a release-URL cache without a cdn stamp is not fresh"
    );
    handle.join().unwrap();
    let (server2, handle2) = local_http_head(lm, 2);
    let release_url2 = format!("{}/releases/download/v1/asset.json", server2);
    super::write_cdn_stamp(&path, &release_url2);
    assert!(
        super::cache_fresh_cdn(&path, 3600, &release_url2),
        "a release-URL cache whose cdn stamp equals the current last-modified is fresh"
    );
    handle2.join().unwrap();
    let (server3, handle3) = local_http_head(rfc1123_from_unix(now - 400), 1);
    let release_url3 = format!("{}/releases/download/v1/asset.json", server3);
    assert!(
        !super::cache_fresh_cdn(&path, 3600, &release_url3),
        "a release-URL cache whose cdn stamp differs from the current last-modified is stale"
    );
    handle3.join().unwrap();
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{path}.cdn"));
    let _ = std::fs::remove_dir(&dir);
}

#[test]
fn test_anchor_bodies_have_ephemeris_sources() {
    let content = std::fs::read_to_string("phi/sources.φ").unwrap();
    let sources = super::parse_sources(&content);
    let uses = super::anchor_uses(&sources);
    let eph_bodies: std::collections::HashSet<String> = sources
        .iter()
        .filter(|s| s.format == "ephemeris_binary" || s.format == "orbit_bin")
        .filter_map(|s| s.body.clone())
        .collect();
    let absent: Vec<&String> = uses.keys().filter(|b| !eph_bodies.contains(*b)).collect();
    assert!(
        absent.is_empty(),
        "anchor bodies without an ephemeris source would starve the load gate: {:?}",
        absent
    );
}

#[test]
fn test_gracefo_l1b_tar_gz_yaml_source_parses() {
    let content = std::fs::read_to_string("phi/sources.φ").unwrap();
    let sources = super::parse_sources(&content);
    let src = sources
        .iter()
        .find(|s| s.format == "tar_gz_yaml")
        .expect("the GRACE-FO L1B tar.gz+YAML source is registered");
    assert!(
        src.url.ends_with(".tgz"),
        "the registered granule is a .tgz"
    );
    assert!(
        src.headers
            .iter()
            .any(|(k, v)| k == "Authorization" && v == "{EARTHDATA_EDL_TOKEN}"),
        "the Earthdata bearer header carries the token marker"
    );
    let keys: Vec<String> = src
        .extracts
        .iter()
        .flat_map(super::extract::extract_fields)
        .map(|fc| fc.key)
        .collect();
    assert!(keys.iter().any(|k| k == "KBR1B.range_rate"));
    assert!(keys.iter().any(|k| k == "KBR1B.range_accl"));
}

#[test]
fn test_query_admits_surface_sample_within_window() {
    let now = 840511523.88;
    let jd_now = super::J2000_EPOCH + now / 86400.0;
    let props = super::BodyProperties {
        α0_deg: 270.0,
        dα0_dt_deg_per_century: 0.003,
        δ0_deg: 66.54,
        dδ0_dt_deg_per_century: 0.013,
        w0_deg: 190.147,
        dw_dt_deg_per_day: 360.9856235,
        radius_m: 6378136.6,
        flattening: Some((6378136.6 - 6356751.9) / 6378136.6),
        gaussian_inverse_square: 340.2,
        gaussian_inverse: 5950.0,
        erfc: 3630.0,
        exponential_decay: 2.18e-5,
        patch_levy: 2.00e-5,
        gm: Some(3.986004418e14),
        j2: Some(1.08262668e-3),
        j4: Some(-1.619e-6),
        radii_b: Some(6378136.6),
        radii_c: Some(6356751.9),
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let mut eph = super::BodyEphemeris {
        granules: Vec::new(),
        rotation_matrices: Vec::new(),
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    for i in -1..=1 {
        let t0 = jd_now + i as f64 * 16.0;
        let mut cx = [0.0_f64; super::CHEBYSHEV_N];
        cx[0] = 1.5e11;
        eph.granules.push(super::ChebyshevGranule {
            t0_jd: t0,
            dt_jd: 16.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        });
    }
    let mut eph_map = std::collections::HashMap::new();
    eph_map.insert("earth".to_string(), eph);
    let pos = super::body_barycenter_position("earth", now, &eph_map).expect("earth pos");
    let channel = Channel {
        z: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        name: "argo_dac_temp_c".into(),
        value: 25.0,
        position: Position::Surface {
            body_name: "earth".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        epoch: now - 2.15e6,
    };
    let sensor = FieldConfig {
        key: "TEMP".into(),
        name: "argo_dac_temp_c".into(),
        kernel: 3,
        force: 5,
        tau: 604800.0,
        absorption: 0.0,
        advection: 0.0,
        unit: "C".into(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
    };
    let frame = Frame::Surface {
        body_name: "earth".into(),
        lat: 0.0,
        lon: 0.0,
        alt: 0.0,
    };
    let sample = super::anchor(
        &channel,
        &sensor,
        604800.0,
        Some(157),
        Some(&frame),
        None,
        &eph_map,
    )
    .expect("argo sample anchors");
    let hash = super::build_spatial_hash(vec![Arc::new(sample)], 1.0);
    let mut recs = Vec::new();
    super::query_hash(
        &hash,
        super::MembraneCtx {
            center: pos,
            t2: now,
            pad: 8.0e6,
            delta_t_cache: 0.0,
            floor: &[1.0e-300_f64; 9],
            softening: 1.0,
            forward: [0.0, 0.0, 0.0],
            eph: &eph_map,
        },
        &mut recs,
    );
    assert!(
        !recs.is_empty(),
        "the surface sample within the window pad must reach the membrane, got {} records",
        recs.len()
    );
    let mut recs_ssb = Vec::new();
    super::query_hash(
        &hash,
        super::MembraneCtx {
            center: [0.0, 0.0, 0.0],
            t2: now,
            pad: 2.0e12,
            delta_t_cache: 0.0,
            floor: &[1.0e-300_f64; 9],
            softening: 1.0,
            forward: [0.0, 0.0, 0.0],
            eph: &eph_map,
        },
        &mut recs_ssb,
    );
    assert!(
        !recs_ssb.is_empty(),
        "the earth surface sample within the boot window must reach the SSB presence"
    );
}

#[test]
fn test_wind_orbit_bin_positions_when_present() {
    let path = "/tmp/opencode/wind_orbit_test.bin";
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => return,
    };
    let records = match crate::wind_orbit::parse_bin(&bytes) {
        Some(r) => r,
        None => return,
    };
    assert!(records.len() >= 144);
    let rec = std::sync::Arc::new(crate::wind_orbit::orbit_rec(&records));
    let eph = super::BodyEphemeris {
        granules: Vec::new(),
        rotation_matrices: Vec::new(),
        props: None,
        orbit: Some(rec),
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    let mut map = std::collections::HashMap::new();
    map.insert("wind".to_string(), eph);
    let first_t = records[0].0;
    let p = super::body_barycenter_position("wind", first_t, &map).unwrap();
    assert!((p[0] - records[0].1[0]).abs() < 1.0e-3);
    assert!((p[1] - records[0].1[1]).abs() < 1.0e-3);
    assert!((p[2] - records[0].1[2]).abs() < 1.0e-3);
    let mid_t = (records[0].0 + records[1].0) * 0.5;
    let mid = super::body_barycenter_position("wind", mid_t, &map).unwrap();
    for (k, m) in mid.iter().enumerate() {
        let expected = (records[0].1[k] + records[1].1[k]) * 0.5;
        assert!((m - expected).abs() < 1.0e-3);
    }
    assert!(
        super::body_barycenter_position("wind", records[records.len() - 1].0 + 1.0e8, &map)
            .is_none()
    );
    assert!(super::body_barycenter_position("wind", records[0].0 - 1.0e8, &map).is_none());
}

#[test]
fn test_parse_ephemeris_binary_rejects_non_v2_props() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&[0xCF, 0x86, 0x01, 0x00]);
    buf.extend_from_slice(&3u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..56 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..8 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    assert!(super::parse_ephemeris_binary(&buf).is_none());
}

#[test]
fn test_parse_ephemeris_binary_rejects_truncated_granules() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&[0xCF, 0x86, 0x01, 0x00]);
    buf.extend_from_slice(&2u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&2u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..56 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    assert!(super::parse_ephemeris_binary(&buf).is_none());
}

#[test]
fn test_parse_ephemeris_binary_rejects_truncated_props() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&[0xCF, 0x86, 0x01, 0x00]);
    buf.extend_from_slice(&2u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..56 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    buf.extend_from_slice(&1u32.to_le_bytes());
    buf.extend_from_slice(&12u32.to_le_bytes());
    buf.extend_from_slice(&17u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for _ in 0..6 {
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    assert!(super::parse_ephemeris_binary(&buf).is_none());
}

#[test]
fn test_live_sources_extract() {
    let srcs = super::load_sources();
    eprintln!("load_sources returned {} sources", srcs.len());
    let fixture_lsk = full_fixture_lsk();
    let now = fixture_lsk.system_now_tdb().unwrap();
    let env = super::load_env();
    let (ok, findings) = super::live_sweep(&env, now, &fixture_lsk, 600);
    eprintln!(
        "\n=== LIVE SOURCE EXTRACTION: {} ok, {} void (of {} tested) ===",
        ok,
        findings.len(),
        ok + findings.len()
    );
    for f in findings.iter() {
        eprintln!("  void {}  {}  {}", f.class.as_str(), f.url, f.detail);
    }
}

#[test]
fn test_dst_and_co2_mlo_channels_extract_live() {
    let dst_block = "url https://services.swpc.noaa.gov/json/geospace/geospace_dst_1_hour.json
ttl 3600
at earth
map .
epoch time_tag
field dst magnetosphere_dst_nt inverse-square em 1 3600.0 0.0 0.0
";
    let co2_block = "url https://gml.noaa.gov/webdata/ccgg/trends/co2/co2_weekly_mlo.csv
ttl 3600
format text
on earth 19.5 -155.6 0
rows .
epoch year month day 0 0
gate average 300.0 600.0
field 4 co2_ppm_weekly gaussian-inverse-square diffusion ppm 3600.0 0.0 0.0
";
    let fixture_lsk = full_fixture_lsk();
    let now = fixture_lsk.system_now_tdb().unwrap();

    let dst_srcs = super::parse_sources(dst_block);
    assert_eq!(dst_srcs.len(), 1);
    let co2_srcs = super::parse_sources(co2_block);
    assert_eq!(co2_srcs.len(), 1);

    let dst_src = &dst_srcs[0];
    let co2_src = &co2_srcs[0];
    assert!(matches!(&dst_src.extracts[0], super::Extract::Map { .. }));
    assert!(matches!(&co2_src.extracts[0], super::Extract::Rows { .. }));

    let dst_body = match super::fetch_raw(&dst_src.url, None, &[], 3600) {
        Some(b) => b,
        None => {
            eprintln!("dst fetch void — network-dependent, the series stays unread");
            return;
        }
    };
    let co2_body = match super::fetch_raw(&co2_src.url, None, &[], 3600) {
        Some(b) => b,
        None => {
            eprintln!("co2 fetch void — network-dependent, the series stays unread");
            return;
        }
    };

    match super::extract(dst_src, &dst_body, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert!(!v.is_empty(), "dst series must not be empty");
            assert_eq!(v[0].0.name, "magnetosphere_dst_nt");
            let has_real = v.iter().any(|(c, _)| c.value.abs() < 1000.0);
            assert!(has_real, "dst must carry at least one real nT value");
            assert_eq!(v[0].1.force, 0, "dst is em (force 0)");
        }
        other => {
            let _ = other;
            panic!("dst expected Measurements");
        }
    }

    match super::extract(co2_src, &co2_body, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert!(
                !v.is_empty(),
                "co2 series must not be empty (extract produced 0 channels)"
            );
            assert_eq!(v[0].0.name, "co2_ppm_weekly");
            assert_eq!(v[0].1.force, 6, "co2 is diffusion (force 6)");
            let mauna = v.iter().any(|(c, _)| (300.0..600.0).contains(&c.value));
            assert!(mauna, "co2 must carry at least one in-gate ppm value");
            let epochs: Vec<f64> = v.iter().map(|(c, _)| c.epoch).collect();
            let sorted = {
                let mut e = epochs.clone();
                e.sort_by(|a, b| a.total_cmp(b));
                e
            };
            assert_eq!(epochs, sorted, "co2 epochs must ascend");
            assert!(epochs.first().is_some_and(|e| *e < now));
        }
        other => {
            let _ = other;
            panic!("co2 expected Measurements");
        }
    }

    let qbo_body = match super::fetch_raw(
        "https://www.cpc.ncep.noaa.gov/data/indices/qbo.u30.index",
        None,
        &[],
        3600,
    ) {
        Some(b) => b,
        None => {
            eprintln!("qbo fetch void — network-dependent, the series stays unread");
            return;
        }
    };
    let qbo_csv = transpose_qbo_table(&qbo_body);
    assert!(!qbo_csv.is_empty(), "qbo transposition must produce rows");
    let qbo_block =
        "url https://github.com/omegaflow/sources/releases/download/cpc.ncep.noaa.gov/qbo_30hpa.csv
ttl 3600
format text
on earth 1.35 103.99 0
rows .
epoch year month day 0 0
gate qbo_30hpa_ms -300.0 300.0
field 3 qbo_30hpa_ms patch-levy advective m/s 2592000.0 0.0 0.0
";
    let qsrcs = super::parse_sources(qbo_block);
    assert_eq!(qsrcs.len(), 1);
    match super::extract(&qsrcs[0], &qbo_csv, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert!(!v.is_empty(), "qbo series must not be empty");
            assert_eq!(v[0].0.name, "qbo_30hpa_ms");
            assert_eq!(v[0].1.force, 7, "qbo is advective (force 7)");
            let in_gate = v.iter().any(|(c, _)| (-300.0..300.0).contains(&c.value));
            assert!(in_gate, "qbo must carry in-gate m/s values");
            let first = v.first().unwrap().0.epoch;
            assert!(
                first < -500_000_000.0,
                "qbo first epoch must be ~1979 (TDB, J2000-relative), got {}",
                first
            );
        }
        other => {
            let _ = other;
            panic!("qbo expected Measurements");
        }
    }

    let oulu_body = match super::fetch_raw(
        "https://www.nmdb.eu/nest/draw_graph.php?wget=1&stations%5B%5D=OULU&output=ascii&tabchoice=ori&dtype=corr_for_efficiency&date_choice=last&last_days=7&tresolution=60&yunits=0",
        None,
        &[],
        3600,
    ) {
        Some(b) => b,
        None => {
            eprintln!("oulu fetch void — network-dependent, the series stays unread");
            return;
        }
    };
    assert!(
        oulu_body.lines().any(|l| l.starts_with("20")),
        "oulu body must carry data lines"
    );
    let oulu_block = "url https://www.nmdb.eu/nest/draw_graph.php
ttl 3600
format text
on earth 65.06 25.47 0
rows .
epoch 0
field 1 oulu_neutron_corr_for_eff inverse-square em cpm 3600.0 0.0 0.0
";
    let osrcs = super::parse_sources(oulu_block);
    assert_eq!(osrcs.len(), 1);
    match super::extract(&osrcs[0], &oulu_body, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert!(!v.is_empty(), "oulu series must not be empty");
            assert_eq!(v[0].0.name, "oulu_neutron_corr_for_eff");
            assert_eq!(v[0].1.force, 0, "oulu is em (force 0)");
            let plausible = v.iter().all(|(c, _)| c.value > 1.0 && c.value < 500.0);
            assert!(plausible, "oulu neutron values must be plausible %");
        }
        other => {
            let _ = other;
            panic!("oulu expected Measurements");
        }
    }

    let d20_body = match super::fetch_raw(
        "https://data.pmel.noaa.gov/pmel/erddap/tabledap/pmelTaoDyIso.csv?time,longitude,latitude,station,ISO_6,QI_5006&latitude>=-2&latitude<=2&longitude>=200&longitude<=280&time>=2026-06-01T00:00:00Z&time<=2026-07-03T00:00:00Z",
        None,
        &[],
        3600,
    ) {
        Some(b) => b,
        None => {
            eprintln!("d20 fetch void — network-dependent, the series stays unread");
            return;
        }
    };
    assert!(
        d20_body.lines().any(|l| l.starts_with("20")),
        "d20 body must carry data lines"
    );
    let d20_block = "url https://github.com/omegaflow/sources/releases/download/data.pmel.noaa.gov/d20_thermocline.csv
ttl 3600
format text
on earth 0.0 -120.0 0
rows .
epoch 0
gate iso_6 0.0 500.0
field 4 d20_thermocline_depth_m erfc gravity m 86400.0 0.0 0.0
";
    let dsrcs = super::parse_sources(d20_block);
    assert_eq!(dsrcs.len(), 1);
    match super::extract(&dsrcs[0], &d20_body, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert!(!v.is_empty(), "d20 series must not be empty");
            assert_eq!(v[0].0.name, "d20_thermocline_depth_m");
            assert_eq!(
                v[0].1.force, 1,
                "d20 depth is a gravity-anchored length (force 1)"
            );
            let plausible = v.iter().all(|(c, _)| c.value > 0.0 && c.value < 500.0);
            assert!(plausible, "d20 depths must be plausible m");
        }
        other => {
            let _ = other;
            panic!("d20 expected Measurements");
        }
    }

    let drifter_block = "url https://erddap.aoml.noaa.gov/gdp/erddap/tabledap/drifter_hourly_qc.json?latitude,longitude,sst,ve,vn&time%3E=max(time)-7days
ttl 3600
on earth 0 0 0
map table.rows
lat 0
lon 1
field 2 hydrosphere_drifter_sst_k exponential-decay thermal K 360.0 0.0 0.0
field 3 hydrosphere_drifter_current_east_m_s patch-levy advective m/s 360.0 0.0 0.0
field 4 hydrosphere_drifter_current_north_m_s patch-levy advective m/s 360.0 0.0 0.0
";
    let drifter_body = match super::fetch_raw(
        "https://erddap.aoml.noaa.gov/gdp/erddap/tabledap/drifter_hourly_qc.json?latitude,longitude,sst,ve,vn&time%3E=max(time)-7days",
        None,
        &[],
        3600,
    ) {
        Some(b) => b,
        None => {
            eprintln!("drifter fetch void — network-dependent, the series stays unread");
            return;
        }
    };
    let dsrcs = super::parse_sources(drifter_block);
    assert_eq!(dsrcs.len(), 1);
    match super::extract(&dsrcs[0], &drifter_body, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            let ve: Vec<f64> = v
                .iter()
                .filter(|(c, _)| c.name == "hydrosphere_drifter_current_east_m_s")
                .map(|(c, _)| c.value)
                .collect();
            assert!(!ve.is_empty(), "drifter current east must carry values");
            let plausible = ve.iter().all(|v| v.is_finite() && v.abs() < 10.0);
            assert!(plausible, "drifter currents must be plausible m/s");
            let vn: Vec<f64> = v
                .iter()
                .filter(|(c, _)| c.name == "hydrosphere_drifter_current_north_m_s")
                .map(|(c, _)| c.value)
                .collect();
            assert_eq!(ve.len(), vn.len(), "east and north currents must pair");
        }
        other => {
            let _ = other;
            panic!("drifter expected Measurements");
        }
    }
}

fn transpose_qbo_table(body: &str) -> String {
    let mut csv = String::from("year,month,day,qbo_30hpa_ms\n");
    for line in body.lines() {
        let t = line.trim();
        if t.is_empty()
            || t.starts_with("30 mb")
            || t.starts_with("YEAR")
            || t.starts_with("ORIGINAL")
            || t.starts_with("PREDICTED")
            || t.starts_with('*')
        {
            continue;
        }
        let cols: Vec<&str> = t.split_whitespace().collect();
        if cols.len() < 13 {
            continue;
        }
        let year: i64 = match cols[0].parse() {
            Ok(y) if y >= 1979 => y,
            _ => continue,
        };
        for (m, c) in cols.iter().enumerate().take(12).skip(1) {
            let val: f64 = match c.trim().parse() {
                Ok(v) => v,
                Err(_) => continue,
            };
            if !val.is_finite() || val.abs() >= 900.0 {
                continue;
            }
            csv.push_str(&format!("{},{},{},{}\n", year, m, 1, val));
        }
    }
    csv
}
#[test]
fn test_diagnose_no_samples() {
    let base = super::SourceConfig {
        ttl: 60,
        url: "https://example.com/q".into(),
        frame: super::Frame::Surface {
            body_name: "earth".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "json".into(),
        extracts: vec![super::Extract::Map {
            arr_path: "features".into(),
            lat_key: "geometry.coordinates.1".into(),
            lon_key: "geometry.coordinates.0".into(),
            alt_key: String::new(),
            epoch_key: String::new(),
            val_key: String::new(),
            alt_scale: 1.0,
            vel_key: String::new(),
            vel_scale: 1.0,
            trk_key: String::new(),
            vr_key: String::new(),
            fields: vec![FieldConfig {
                key: "properties.mag".into(),
                name: "mag".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            lat_sign: None,
            lon_sign: None,
            epoch_scale: 1.0,
            tau_key: String::new(),
            mag_type_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let empty_geojson =
        r#"{"type":"FeatureCollection","metadata":{"api":"2.7","count":0},"features":[]}"#;
    let d_empty = super::diagnose_no_samples(&base, empty_geojson);
    eprintln!("empty geojson -> {}", d_empty);
    assert!(d_empty.contains("empty-response"), "got: {}", d_empty);

    let filled_geojson = r#"{"type":"FeatureCollection","features":[{"geometry":{"coordinates":[-104.0,39.5,10.0]},"properties":{"mag":3.2}}]}"#;
    let d_filled = super::diagnose_no_samples(&base, filled_geojson);
    eprintln!("filled geojson -> {}", d_filled);
    assert!(d_filled.contains("data-present"), "got: {}", d_filled);

    let html = "<html>GraceDB down</html>";
    let d_html = super::diagnose_no_samples(&base, html);
    eprintln!("html -> {}", d_html);
    assert!(d_html.contains("non-JSON"), "got: {}", d_html);
}

#[test]
fn test_refusal_ledger_dedup_and_reload() {
    let path =
        std::env::temp_dir().join(format!("omegaflow_refusal_ledger_{}.φ", std::process::id()));
    let _ = std::fs::remove_file(&path);
    {
        let mut ledger = super::RefusalLedger::new(path.to_str().unwrap());
        ledger.register("https://a.example/q", "extract-void");
        ledger.register("https://a.example/q", "extract-void");
        ledger.register("https://b.example/q", "fetch-void");
    }
    let content = std::fs::read_to_string(&path).unwrap();
    assert_eq!(content.lines().count(), 2, "one entry per class+url");
    assert!(content.contains("extract-void https://a.example/q"));
    assert!(content.contains("fetch-void https://b.example/q"));
    {
        let mut ledger = super::RefusalLedger::new(path.to_str().unwrap());
        ledger.register("https://a.example/q", "extract-void");
    }
    let reloaded = std::fs::read_to_string(&path).unwrap();
    assert_eq!(
        reloaded.lines().count(),
        2,
        "a reloaded ledger never repeats an entry"
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_map_single_object_alt_scale_epoch_default() {
    let src = super::SourceConfig {
        ttl: 10,
        url: "https://api.wheretheiss.at/v1/satellites/25544".into(),
        frame: super::Frame::Barycenter {
            body_name: "earth".into(),
            scale: 1.0,
        },
        format: "json".into(),
        extracts: vec![super::Extract::Map {
            arr_path: ".".into(),
            lat_key: "latitude".into(),
            lon_key: "longitude".into(),
            alt_key: "altitude".into(),
            epoch_key: String::new(),
            val_key: String::new(),
            alt_scale: 1000.0,
            vel_key: String::new(),
            vel_scale: 1.0,
            trk_key: String::new(),
            vr_key: String::new(),
            fields: vec![FieldConfig {
                key: "velocity".into(),
                name: "velocity".into(),
                kernel: 0,
                force: 0,
                tau: 1.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            lat_sign: None,
            lon_sign: None,
            epoch_scale: 1.0,
            tau_key: String::new(),
            mag_type_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let body = r#"{"latitude":-47.75,"longitude":78.87,"altitude":438.28,"velocity":27528.0}"#;
    let now = 8.4e8;
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match super::extract(&src, body, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 1);
            let (ch, fc) = &v[0];
            assert_eq!(ch.epoch, now);
            assert_eq!(ch.value, 27528.0);
            assert_eq!(fc.tau, 1.0);
            match &ch.position {
                super::Position::Surface { lat, lon, alt, .. } => {
                    assert!((lat - -47.75).abs() < 1e-9);
                    assert!((lon - 78.87).abs() < 1e-9);
                    assert!((alt - 438280.0).abs() < 1e-6);
                }
                other => panic!("position variant: {:?} unexpected", other),
            }
        }
        _ => panic!("extract variant unexpected"),
    }
}

#[test]
fn test_map_vel_unit_and_tau_key_override() {
    let src = super::SourceConfig {
        ttl: 10,
        url: "https://example.org/flow".into(),
        frame: super::Frame::Surface {
            body_name: "earth".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "json".into(),
        extracts: vec![super::Extract::Map {
            arr_path: "data".into(),
            lat_key: "lat".into(),
            lon_key: "lon".into(),
            alt_key: "alt".into(),
            epoch_key: String::new(),
            val_key: String::new(),
            alt_scale: 1.0,
            vel_key: "spd".into(),
            vel_scale: 1.0 / 3.6,
            trk_key: "hdg".into(),
            vr_key: "vr".into(),
            fields: vec![FieldConfig {
                key: "v".into(),
                name: "flow_value".into(),
                kernel: 0,
                force: 0,
                tau: 7.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            lat_sign: None,
            lon_sign: None,
            epoch_scale: 1.0,
            tau_key: "row_tau".into(),
            mag_type_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let body = r#"{"data":[
            {"lat":10.0,"lon":20.0,"alt":0.0,"spd":72.0,"hdg":90.0,"vr":3.6,"row_tau":60.0,"v":5.0},
            {"lat":11.0,"lon":21.0,"alt":0.0,"spd":36.0,"hdg":0.0,"vr":1.8,"row_tau":0.0,"v":5.0},
            {"lat":12.0,"lon":22.0,"alt":0.0,"spd":18.0,"hdg":270.0,"vr":1.0,"v":5.0}
        ]}"#;
    let now = 8.4e8;
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match super::extract(&src, body, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 2);
            let (ch0, fc0) = &v[0];
            match &ch0.position {
                super::Position::SurfaceFlow {
                    speed,
                    track,
                    vrate,
                    ..
                } => {
                    assert!((speed - 20.0).abs() < 1e-9);
                    assert!((track - 90.0).abs() < 1e-9);
                    assert!((vrate.unwrap() - 1.0).abs() < 1e-9);
                }
                other => panic!("expected SurfaceFlow, got {:?}", other),
            }
            assert!((fc0.tau - 60.0).abs() < 1e-9);
            let (ch1, fc1) = &v[1];
            match &ch1.position {
                super::Position::SurfaceFlow { speed, vrate, .. } => {
                    assert!((speed - 5.0).abs() < 1e-9);
                    assert!((vrate.unwrap() - 1.0 / 3.6).abs() < 1e-9);
                }
                other => panic!("expected SurfaceFlow, got {:?}", other),
            }
            assert!((fc1.tau - 7.0).abs() < 1e-9);
        }
        _ => panic!("extract variant unexpected"),
    }
}

#[test]
fn test_parse_spectral_block() {
    let block = "url https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/spectra.bin\nformat spectral\non earth 19.82 -155.47 0\nttl 86400\nfield irradiance spectral_irradiance_W_m2_Hz inverse-square em W/m2/Hz 2628000 0.0 0.0\n";
    let srcs = super::parse_sources(block);
    assert_eq!(srcs.len(), 1);
    assert_eq!(srcs[0].format, "spectral");
    match &srcs[0].frame {
        super::Frame::Surface {
            body_name,
            lat,
            lon,
            alt,
        } => {
            assert_eq!(body_name, "earth");
            assert!((*lat - 19.82).abs() < 1e-12);
            assert!((*lon + 155.47).abs() < 1e-12);
            assert_eq!(*alt, 0.0);
        }
        other => {
            let _ = other;
            panic!("expected Surface frame")
        }
    }
    match &srcs[0].extracts[0] {
        super::Extract::Field(fc) => {
            assert_eq!(fc.name, "spectral_irradiance_W_m2_Hz");
            assert_eq!(fc.unit, "W/m2/Hz");
            assert!((fc.tau - 2628000.0).abs() < 1e-9);
            assert_eq!(fc.force as u32, 0);
            assert_eq!(fc.kernel as u32, 0);
        }
        other => {
            let _ = other;
            panic!("expected Field extract")
        }
    }
}

#[test]
fn test_parse_sky1_cdn_asset_block() {
    let blocks = [
        ("vtscat_flux", "m-2.s-1.tev-1"),
        ("hess_dl3", "tev"),
        ("magic_dl3", "tev"),
    ];
    for (name, unit) in blocks {
        let block = format!(
            "url https://github.com/omegaflow/sources/releases/download/tag/{name}.sky1\nformat sky1\nat sun\nttl 31536000\nfield {name} {name} inverse-square em {unit} 31536000 0.0 0.0\n"
        );
        let srcs = super::parse_sources(&block);
        assert_eq!(
            srcs.len(),
            1,
            "{name}: sky1 asset line must register one source"
        );
        assert_eq!(srcs[0].format, "sky1", "{name}: format is sky1");
        match &srcs[0].extracts[0] {
            super::Extract::Field(fc) => {
                assert_eq!(fc.name, name);
                assert_eq!(fc.unit, unit);
                assert_eq!(fc.force as u32, 0);
                assert_eq!(fc.kernel as u32, 0);
                assert!((fc.tau - 31536000.0).abs() < 1e-9);
            }
            other => {
                let _ = other;
                panic!("{name}: expected Field extract")
            }
        }
    }
    let witness = "witness s2-direction\nurl https://github.com/VERITAS-Observatory/VERITAS-VTSCat\nrecord sky1\nforce em\nnote VTSCat\n";
    assert!(
        super::parse_sources(witness).is_empty(),
        "a witness block alone carries no ttl/field and is not a field source"
    );
}

#[test]
fn test_parse_vel_unit_and_tau_key_directives() {
    let block = "url https://example.org/flow\nttl 3600\nformat json\non earth 0 0 0\nmap data\nlat lat\nlon lon\nvel spd km/h\ntau_key row_tau\nfield v flow_value inverse-square thermal W 10 0.0 0.0\n";
    let srcs = super::parse_sources(block);
    assert_eq!(srcs.len(), 1);
    match &srcs[0].extracts[0] {
        super::Extract::Map {
            vel_key,
            vel_scale,
            tau_key,
            fields,
            ..
        } => {
            assert_eq!(vel_key, "spd");
            assert!((*vel_scale - 1.0 / 3.6).abs() < 1e-12);
            assert_eq!(tau_key, "row_tau");
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].unit, "W");
        }
        other => {
            let _ = other;
            panic!("expected Map extract")
        }
    }
    let cmap_block = "url https://example.org/c\nttl 3600\nformat json\nat sun\ncmap data\nra ra\ndec dec\ntau_key tkey\n";
    let csrcs = super::parse_sources(cmap_block);
    assert_eq!(csrcs.len(), 1);
    match &csrcs[0].extracts[0] {
        super::Extract::CelestialMap { tau_key, .. } => assert_eq!(tau_key, "tkey"),
        other => {
            let _ = other;
            panic!("expected CelestialMap extract")
        }
    }
    let rows_block = "url https://example.org/r\nttl 3600\nformat text\non earth 1 2 0\nrows\nlast_line true\ntau_key rtau\nlastrow T val thermal K 10 0 0\n";
    let rsrcs = super::parse_sources(rows_block);
    assert_eq!(rsrcs.len(), 1);
    match &rsrcs[0].extracts[0] {
        super::Extract::Rows { tau_key, .. } => assert_eq!(tau_key, "rtau"),
        other => {
            let _ = other;
            panic!("expected Rows extract")
        }
    }
}

#[test]
fn test_fold_directive_parse_and_extract() {
    let block = "url https://example.org/f\nttl 3600\nformat json\non earth 0 0 0\nmap data\nlat lat\nlon lon\nfold mean nh sh diffusion ppm 100\nfold diff nh sh diffusion ppm 100\n";
    let srcs = super::parse_sources(block);
    assert_eq!(srcs.len(), 1);
    let fields = match &srcs[0].extracts[0] {
        super::Extract::Map { fields, .. } => fields,
        other => {
            let _ = other;
            panic!("expected Map extract")
        }
    };
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "fold_mean_nh_sh");
    assert!(matches!(&fields[0].fold, Some((1, b)) if b == "sh"));
    assert!(matches!(&fields[1].fold, Some((2, b)) if b == "sh"));
    assert!((fields[0].tau - 100.0).abs() < 1e-12);
    assert_eq!(fields[0].unit, "ppm");
    let refused =
        "url https://example.org/f\nttl 3600\nformat json\nat sun\nfold mean a b em mag 100\n";
    let rsrcs = super::parse_sources(refused);
    assert_eq!(rsrcs.len(), 1);
    assert!(rsrcs[0].extracts.is_empty());

    let src = super::SourceConfig {
        ttl: 10,
        url: "https://example.org/f".into(),
        frame: super::Frame::Surface {
            body_name: "earth".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "json".into(),
        extracts: vec![super::Extract::Map {
            arr_path: "data".into(),
            lat_key: "lat".into(),
            lon_key: "lon".into(),
            alt_key: "alt".into(),
            epoch_key: String::new(),
            val_key: String::new(),
            alt_scale: 1.0,
            vel_key: String::new(),
            vel_scale: 1.0,
            trk_key: String::new(),
            vr_key: String::new(),
            fields: vec![
                FieldConfig {
                    key: "nh".into(),
                    name: "fold_mean_nh_sh".into(),
                    kernel: 0,
                    force: 6,
                    tau: 100.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: "ppm".into(),
                    freq: 0.0,
                    bin_width: 0.0,
                    fold: Some((1, "sh".into())),
                },
                FieldConfig {
                    key: "nh".into(),
                    name: "fold_diff_nh_sh".into(),
                    kernel: 0,
                    force: 6,
                    tau: 100.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: "ppm".into(),
                    freq: 0.0,
                    bin_width: 0.0,
                    fold: Some((2, "sh".into())),
                },
            ],
            lat_sign: None,
            lon_sign: None,
            epoch_scale: 1.0,
            tau_key: String::new(),
            mag_type_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let body = r#"{"data":[
            {"lat":1.0,"lon":2.0,"alt":0.0,"nh":420.0,"sh":410.0},
            {"lat":3.0,"lon":4.0,"alt":0.0,"nh":420.0}
        ]}"#;
    let now = 8.4e8;
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match super::extract(&src, body, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 2);
            assert!((v[0].0.value - 415.0).abs() < 1e-9);
            assert!((v[1].0.value - 10.0).abs() < 1e-9);
        }
        _ => panic!("extract variant unexpected"),
    }
}

#[test]
fn test_keplermap_elements_to_icrs() {
    let block = "url https://example.org/k\nttl 3600\nformat json\nat sun\nkeplermap data a e i\nom om\nw w\nma ma\nepoch epoch\nqr q\ntp tp\nfield H abs_mag inverse-square em mag 100 0 0\n";
    let srcs = super::parse_sources(block);
    assert_eq!(srcs.len(), 1);
    match &srcs[0].extracts[0] {
        super::Extract::KeplerMap {
            a_key,
            e_key,
            i_key,
            om_key,
            w_key,
            ma_key,
            epoch_key,
            q_key,
            tp_key,
            fields,
            ..
        } => {
            assert_eq!(a_key, "a");
            assert_eq!(e_key, "e");
            assert_eq!(i_key, "i");
            assert_eq!(om_key, "om");
            assert_eq!(w_key, "w");
            assert_eq!(ma_key, "ma");
            assert_eq!(epoch_key, "epoch");
            assert_eq!(q_key, "q");
            assert_eq!(tp_key, "tp");
            assert_eq!(fields.len(), 1);
        }
        other => {
            let _ = other;
            panic!("expected KeplerMap extract")
        }
    }
    let mk_src = |a_key: &str, ma_key: &str, q_key: &str, tp_key: &str| super::SourceConfig {
        ttl: 10,
        url: "https://example.org/k".into(),
        frame: super::Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: "json".into(),
        extracts: vec![super::Extract::KeplerMap {
            arr_path: "data".into(),
            a_key: a_key.into(),
            e_key: "e".into(),
            i_key: "i".into(),
            om_key: "om".into(),
            w_key: "w".into(),
            ma_key: ma_key.into(),
            epoch_key: "epoch".into(),
            q_key: q_key.into(),
            tp_key: tp_key.into(),
            fields: vec![FieldConfig {
                key: "H".into(),
                name: "abs_mag".into(),
                kernel: 0,
                force: 0,
                tau: 100.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let au = 1.495978707e11;
    let expect_v = (1.32712440018e20_f64 / au).sqrt();
    let now = 8.4e8;
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    let body_ma = r#"{"data":[{"a":1.0,"e":0.0,"i":0.0,"om":0.0,"w":0.0,"ma":0.0,"epoch":2451545.0,"H":12.0}]}"#;
    let src_ma = mk_src("a", "ma", "", "");
    match super::extract(&src_ma, body_ma, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 1);
            match &v[0].0.position {
                super::Position::StateVector { p, v: vel, .. } => {
                    let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
                    let sp = (vel[0] * vel[0] + vel[1] * vel[1] + vel[2] * vel[2]).sqrt();
                    assert!((r - au).abs() < au * 1e-6);
                    assert!((sp - expect_v).abs() < 1.0);
                }
                other => {
                    let _ = other;
                    panic!("expected StateVector position")
                }
            }
            assert!((v[0].0.value - 12.0).abs() < 1e-9);
        }
        _ => panic!("extract variant unexpected"),
    }
    let body_tp = r#"{"data":[{"q":1.0,"e":0.0,"i":0.0,"om":0.0,"w":0.0,"tp":2451545.0,"epoch":2451545.0,"H":12.0}]}"#;
    let src_tp = mk_src("", "", "q", "tp");
    match super::extract(&src_tp, body_tp, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 1);
            match &v[0].0.position {
                super::Position::StateVector { p, .. } => {
                    let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
                    assert!((r - au).abs() < au * 1e-6);
                }
                other => {
                    let _ = other;
                    panic!("expected StateVector position")
                }
            }
        }
        _ => panic!("extract variant unexpected"),
    }
}

#[test]
fn test_port_block_without_force_classifies_physical() {
    let block = "source geosphere\nttl 86400\nurl https://example.org/g\nmap data\nlat_key lat\nlon_key lon\nfield_in geometry.coordinates.2 quake_depth\n";
    let conv = super::port_block(block);
    assert!(
        conv.contains(
            "field geometry.coordinates.2 quake_depth gaussian-inverse-square seismic-body km 10 0.0 0.0\n"
        ),
        "a physical name without a force directive synthesizes the registry line, got: {conv}"
    );
}

#[test]
fn test_port_block_without_force_declines_non_physical() {
    let block = "source geosphere\nttl 86400\nurl https://example.org/g\nmap data\nlat_key lat\nlon_key lon\nfield_in geometry.coordinates.2 station_id\n";
    let conv = super::port_block(block);
    assert!(
        conv.contains("# declined field station_id — not an oscillator (no physical force)"),
        "a non-physical name is declined, never looped as pending, got: {conv}"
    );
}

#[test]
fn test_port_block_without_force_undetermined_stays_pending() {
    let block = "source geosphere\nttl 86400\nurl https://example.org/g\nmap data\nlat_key lat\nlon_key lon\nfield_in geometry.coordinates.2 sommerfeld_ratio\n";
    let conv = super::port_block(block);
    assert!(
        conv.contains("# pending field sommerfeld_ratio — force undetermined, review"),
        "an unclassified name stays pending, got: {conv}"
    );
}

#[test]
fn test_port_block_with_force_and_no_unit_stays_pending() {
    let block = "source geosphere\nttl 86400\nforce seismic-body\nurl https://example.org/g\nmap data\nlat_key lat\nlon_key lon\nfield_in geometry.coordinates.2 quake_depth\n";
    let conv = super::port_block(block);
    assert!(
        conv.contains("# pending field quake_depth — unit or cadence absent, review"),
        "a force directive without a measured unit stays pending, never the literal 1, got: {conv}"
    );
    assert!(
        !conv.contains("seismic-body 1 "),
        "the fabricated unit literal is gone, got: {conv}"
    );
}

#[test]
fn test_port_block_hapi_live_measure_classifies_em() {
    let block = "url https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=CLF/best-avail/PT1M/xyzf\nttl 86400\non earth 48.025 2.26\nforce gravity\npath 1.0 magnetosphere_intermagnet_clf_x_nt\n";
    let measure = super::PortMeasure {
        hapi: true,
        unit: Some("nT".to_string()),
        tau: Some(60.0),
    };
    let conv = super::port_block_measured(block, &measure);
    assert!(
        conv.contains(
            "path 1.0 magnetosphere_intermagnet_clf_x_nt inverse-square em nT 60 0.0 0.0\n"
        ),
        "the hapi path measures unit and cadence and classifies the magnetic field as em, not the block's gravity, got: {conv}"
    );
}

#[test]
fn test_flush_port_block_carries_pending_review_marker() {
    let block = "source geosphere\nttl 86400\nurl https://example.org/g\nmap data\nlat_key lat\nlon_key lon\nfield_in geometry.coordinates.2 sommerfeld_ratio\n";
    let mut converted = String::new();
    let mut total = 0usize;
    let mut parsed = 0usize;
    let mut pending = 0usize;
    let mut declined = 0usize;
    let measure = super::PortMeasure {
        hapi: false,
        unit: None,
        tau: None,
    };
    super::flush_port_block(
        block,
        &mut converted,
        &mut total,
        &mut parsed,
        &mut pending,
        &mut declined,
        &measure,
    );
    assert_eq!(total, 1);
    assert_eq!(parsed, 0);
    assert_eq!(pending, 1, "the review block is counted as pending");
    assert_eq!(declined, 0);
    assert!(
        converted.contains("# pending field sommerfeld_ratio — force undetermined, review"),
        "the pending review marker must reach the port output, got: {converted}"
    );
}

#[test]
fn test_flush_port_block_carries_declined_disposition() {
    let block = "source geosphere\nttl 86400\nurl https://example.org/g\nfield_in geometry.coordinates.2 dataset_title\n";
    let mut converted = String::new();
    let mut total = 0usize;
    let mut parsed = 0usize;
    let mut pending = 0usize;
    let mut declined = 0usize;
    let measure = super::PortMeasure {
        hapi: false,
        unit: None,
        tau: None,
    };
    super::flush_port_block(
        block,
        &mut converted,
        &mut total,
        &mut parsed,
        &mut pending,
        &mut declined,
        &measure,
    );
    assert_eq!(total, 1);
    assert_eq!(pending, 0, "a declined field is not looped as pending");
    assert_eq!(declined, 1);
    assert!(
        converted
            .contains("# declined field dataset_title — not an oscillator (no physical force)"),
        "the declined disposition must reach the port output, got: {converted}"
    );
}

#[test]
fn test_flush_port_block_keeps_synthesized_block_refused_at_parse() {
    let block = "url https://example.org/g\nfield_in geometry.coordinates.2 quake_depth\n";
    let mut converted = String::new();
    let mut total = 0usize;
    let mut parsed = 0usize;
    let mut pending = 0usize;
    let mut declined = 0usize;
    let measure = super::PortMeasure {
        hapi: false,
        unit: None,
        tau: None,
    };
    super::flush_port_block(
        block,
        &mut converted,
        &mut total,
        &mut parsed,
        &mut pending,
        &mut declined,
        &measure,
    );
    assert_eq!(total, 1);
    assert_eq!(parsed, 0);
    assert_eq!(declined, 0);
    assert_eq!(pending, 1);
    assert!(
        converted.contains("field geometry.coordinates.2 quake_depth"),
        "the synthesized line stays visible, got: {converted}"
    );
    assert!(
        converted.contains("# pending block — refused at parse (ttl/frame gate), review"),
        "the block-level refusal reaches the port output, got: {converted}"
    );
}

#[test]
fn test_flush_port_block_drops_a_block_with_no_recognized_content() {
    let block = "source nothing\n";
    let mut converted = String::new();
    let mut total = 0usize;
    let mut parsed = 0usize;
    let mut pending = 0usize;
    let mut declined = 0usize;
    let measure = super::PortMeasure {
        hapi: false,
        unit: None,
        tau: None,
    };
    super::flush_port_block(
        block,
        &mut converted,
        &mut total,
        &mut parsed,
        &mut pending,
        &mut declined,
        &measure,
    );
    assert_eq!(total, 1);
    assert_eq!(parsed, 0);
    assert_eq!(pending, 0);
    assert_eq!(declined, 0);
    assert!(
        converted.is_empty(),
        "a block with nothing to say stays silent, got: {converted}"
    );
}

#[test]
fn test_field_in_nested_port_and_flatten_generic() {
    let legacy = "source geosphere\nttl 86400\nforce seismic-body\nurl https://example.org/g\nmap data\nlat_key lat\nlon_key lon\nfield_in geometry.coordinates.2 quake_depth\nfield_in properties.mag quake_mag\n";
    let conv = super::port_block(legacy);
    let srcs = super::parse_sources(&conv);
    assert_eq!(srcs.len(), 1);

    let src = super::SourceConfig {
        ttl: 10,
        url: "https://example.org/f".into(),
        frame: super::Frame::Surface {
            body_name: "earth".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "json".into(),
        extracts: vec![super::Extract::Flatten {
            arr_path: "rows".into(),
            geom_path: "pts".into(),
            epoch_key: "t".into(),
            fields: vec![FieldConfig {
                key: "v".into(),
                name: "v".into(),
                kernel: 0,
                force: 0,
                tau: 10.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let body = r#"{"rows":[
            {"t":1000000.0,"pts":[[[10.0,20.0,5.0],[11.0,21.0,6.0]],[[12.0,22.0,7.0]]],"v":3.5},
            {"t":2000000.0,"pts":[30.0,40.0,8.0],"v":2.5}
        ]}"#;
    let now = 8.4e8;
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match super::extract(&src, body, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 4);
            for (ch, _) in &v {
                match &ch.position {
                    super::Position::Surface { lat, lon, .. } => {
                        assert!(*lat >= 20.0 && *lat <= 40.0);
                        assert!(*lon >= 10.0 && *lon <= 30.0);
                    }
                    other => {
                        let _ = other;
                        panic!("expected Surface position")
                    }
                }
            }
        }
        _ => panic!("extract variant unexpected"),
    }
}

#[test]
fn test_flux_from_mag_manifests() {
    let src = super::SourceConfig {
        ttl: 10,
        url: "https://example.org/cat".into(),
        frame: super::Frame::Barycenter {
            body_name: "sun".into(),
            scale: 1.0,
        },
        format: "json".into(),
        extracts: vec![super::Extract::CelestialMap {
            arr_path: ".".into(),
            ra_key: "ra".into(),
            dec_key: "dec".into(),
            dist_key: String::new(),
            dist_scale: Some(1.0),
            plx_key: "plx".into(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: Some(1.0),
            epoch_key: String::new(),
            fields: vec![FieldConfig {
                key: "mag".into(),
                name: "cat_vmag".into(),
                kernel: 0,
                force: 0,
                tau: 100.0,
                absorption: 0.0,
                advection: 0.0,
                unit: "mag".into(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            tau_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: Some("mag".into()),
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let body = r#"[{"ra":89.8,"dec":53.6,"mag":12.0,"plx":10.0}]"#;
    let now = 8.0e8;
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match super::extract(&src, body, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 1);
            let expect = 10.0f64.powf(-0.4 * 12.0);
            assert!((v[0].0.value - expect).abs() < 1e-12);
            assert_eq!(v[0].1.unit, "");
        }
        _ => panic!("extract variant unexpected"),
    }
}

#[test]
fn test_map_lat_sign_lon_sign() {
    let src = SourceConfig {
        ttl: 3600,
        url: "https://example.com/fireball".into(),
        frame: super::Frame::Surface {
            body_name: "earth".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "json".into(),
        extracts: vec![Extract::Map {
            arr_path: "data".into(),
            lat_key: "3".into(),
            lon_key: "5".into(),
            alt_key: "7".into(),
            epoch_key: "0".into(),
            val_key: String::new(),
            alt_scale: 1000.0,
            vel_key: String::new(),
            vel_scale: 1.0,
            trk_key: String::new(),
            vr_key: String::new(),
            fields: vec![FieldConfig {
                key: "1".into(),
                name: "fireball_energy_e10j".into(),
                kernel: 0,
                force: 0,
                tau: 3600.0,
                absorption: 0.0,
                advection: 0.0,
                unit: "e10j".into(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            lat_sign: Some("4".into()),
            lon_sign: Some("6".into()),
            epoch_scale: 1.0,
            tau_key: String::new(),
            mag_type_key: String::new(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let body = r#"{"data":[["2026-08-01 17:43:48","2.9","0.1","19.5","S","176.2","E","45.0",null],["2026-07-21 01:14:45","3.2","0.11","9.4","N","57.4","W","31.5",null]]}"#;
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match super::extract(&src, body, 8.0e8, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 2);
            match (&v[0].0.position, &v[1].0.position) {
                (
                    super::Position::Surface {
                        lat: la0, lon: lo0, ..
                    },
                    super::Position::Surface {
                        lat: la1, lon: lo1, ..
                    },
                ) => {
                    assert!((la0 - (-19.5)).abs() < 1e-9, "lat S: {}", la0);
                    assert!((lo0 - 176.2).abs() < 1e-9, "lon E: {}", lo0);
                    assert!((la1 - 9.4).abs() < 1e-9, "lat N: {}", la1);
                    assert!((lo1 - (-57.4)).abs() < 1e-9, "lon W: {}", lo1);
                }
                _ => panic!("position variant unexpected"),
            }
        }
        _ => panic!("extract variant unexpected"),
    }
}

#[test]
fn test_mag_type_gating() {
    assert!(is_moment_magnitude("mww"));
    assert!(is_moment_magnitude("Mw"));
    assert!(is_moment_magnitude("MWP"));
    assert!(is_moment_magnitude("mwpd"));
    assert!(!is_moment_magnitude("ml"));
    assert!(!is_moment_magnitude("md"));
    assert!(!is_moment_magnitude("mb"));
    assert!(!is_moment_magnitude("m"));
    assert!(!is_moment_magnitude("Mj"));

    let src = super::SourceConfig {
        ttl: 60,
        url: "https://example.org/quake".into(),
        frame: super::Frame::Surface {
            body_name: "earth".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        },
        format: "json".into(),
        extracts: vec![super::Extract::Map {
            arr_path: "data".into(),
            lat_key: "lat".into(),
            lon_key: "lon".into(),
            alt_key: "alt".into(),
            epoch_key: String::new(),
            val_key: String::new(),
            alt_scale: 1.0,
            vel_key: String::new(),
            vel_scale: 1.0,
            trk_key: String::new(),
            vr_key: String::new(),
            fields: vec![FieldConfig {
                key: "mag".into(),
                name: "quake_moment".into(),
                kernel: 0,
                force: 3,
                tau: 6.0,
                absorption: 0.0,
                advection: 0.0,
                unit: "Mw".into(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
            }],
            lat_sign: None,
            lon_sign: None,
            epoch_scale: 1.0,
            tau_key: String::new(),
            mag_type_key: "magType".into(),
        }],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        hapi_fill: HashMap::new(),
        window: None,
        live_only: false,
    };
    let body = r#"{"data":[
            {"lat":1.0,"lon":2.0,"alt":0.0,"magType":"mww","mag":5.5},
            {"lat":3.0,"lon":4.0,"alt":0.0,"magType":"ml","mag":3.0},
            {"lat":5.0,"lon":6.0,"alt":0.0,"magType":"Mw","mag":6.0}
        ]}"#;
    let now = 8.4e8;
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    match super::extract(&src, body, now, &fixture_lsk) {
        super::ExtractResult::Measurements(v) => {
            assert_eq!(v.len(), 2);
            assert!((v[0].0.value - 5.5).abs() < 1e-12);
            assert!((v[1].0.value - 6.0).abs() < 1e-12);
        }
        _ => panic!("extract variant unexpected"),
    }
}

#[test]
fn test_force_id_electric() {
    assert_eq!(crate::force::force_id_of("electric"), Some(8));
    assert_eq!(crate::force::force_id_of("biotic"), None);
    assert_eq!(crate::force::kernel_id_for_force(8), Some(0));
}

#[test]
fn test_route_key_strips_query_and_www() {
    assert_eq!(
        route_key("https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&limit=2"),
        Some("earthquake.usgs.gov/fdsnws/event/1/query".to_string())
    );
    assert_eq!(
        route_key("https://www.example.com/"),
        Some("example.com".to_string())
    );
}

#[test]
fn test_route_key_normalizes_template() {
    assert_eq!(
        route_key("https://api.example.com/{lat}/{lon}"),
        Some("api.example.com/*/*".to_string())
    );
}

#[test]
fn test_route_prefix_keys_most_specific_first() {
    let keys = route_prefix_keys("https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson");
    assert_eq!(
        keys,
        vec![
            "earthquake.usgs.gov/fdsnws/event/1/query".to_string(),
            "earthquake.usgs.gov/fdsnws/event/1".to_string(),
            "earthquake.usgs.gov/fdsnws/event".to_string(),
            "earthquake.usgs.gov/fdsnws".to_string(),
            "earthquake.usgs.gov".to_string(),
        ]
    );
}

#[test]
fn test_frame_registry_distinguishes_routes_on_one_host() {
    let mut reg: HashMap<String, String> = HashMap::new();
    reg.insert(
        "api.example.com/weather".to_string(),
        "on earth".to_string(),
    );
    reg.insert(
        "api.example.com/asteroids".to_string(),
        "at sun".to_string(),
    );
    let (weather, _) = draft_frame_guess("https://api.example.com/weather?city=berlin", "", &reg);
    let (asteroids, _) = draft_frame_guess("https://api.example.com/asteroids/433", "", &reg);
    assert_eq!(weather, "on earth\n");
    assert_eq!(asteroids, "at sun\n");
}

#[test]
fn test_frame_registry_prefix_match() {
    let mut reg: HashMap<String, String> = HashMap::new();
    reg.insert(
        "api.example.com/weather".to_string(),
        "on earth".to_string(),
    );
    let (frame, _) = draft_frame_guess("https://api.example.com/weather/current", "", &reg);
    assert_eq!(frame, "on earth\n");
}

#[test]
fn test_ci_classification_plain_secret_template_fanout() {
    assert!(!url_has_template("https://example.com/a/b.json"));
    assert!(!url_has_template(
        "https://firms.modaps.eosdis.nasa.gov/api/area/csv/{FIRMS_MAP_KEY}/MODIS_NRT/world/1"
    ));
    assert!(url_has_template("https://example.com/{lat}/{lon}"));
    assert!(url_has_template(
        "https://earthquake.usgs.gov/fdsnws/event/1/query?starttime={hour_ago}&latitude={lat}"
    ));
    assert!(url_is_fanout(
        "https://example.com/stations/{station}/readings"
    ));
    assert!(url_is_fanout(
        "https://example.com/?station={nearest_station}"
    ));
    assert!(!url_is_fanout("https://example.com/{lat}/{lon}"));
}

#[test]
fn test_fanout_host_void_lock_skips_fetch() {
    let mut env = HashMap::new();
    env.insert("OCEANNETWORKS_TOKEN".to_string(), "secret".to_string());

    let mut src = source_fixture("json", vec![]);
    src.url = "https://data.oceannetworks.ca/api/scalardata?token={OCEANNETWORKS_TOKEN}&locationCode={station}".into();
    src.stations_url = Some("https://data.oceannetworks.ca/api/locations?token={OCEANNETWORKS_TOKEN}&deviceCategoryCode=CTD".into());

    let mut host_void = HashSet::new();
    host_void.insert("data.oceannetworks.ca".to_string());

    let mut reachable = 0usize;
    let mut dead = 0usize;
    let mut pending = 0usize;
    probe_fanout(
        &src,
        &[],
        &env,
        &mut reachable,
        &mut dead,
        &mut pending,
        &mut host_void,
    );

    assert_eq!(pending, 1, "known-void host must be declared pending");
    assert_eq!(dead, 0, "known-void host must not be re-fetched as dead");
    assert_eq!(reachable, 0);
}

#[test]
fn test_fanout_stations_secret_void() {
    let mut env = HashMap::new();
    env.insert("OCEANNETWORKS_TOKEN".to_string(), "".to_string());

    let mut src = source_fixture("json", vec![]);
    src.url = "https://data.oceannetworks.ca/api/scalardata?token={OCEANNETWORKS_TOKEN}&locationCode={station}".into();
    src.stations_url = Some("https://data.oceannetworks.ca/api/locations?token={OCEANNETWORKS_TOKEN}&deviceCategoryCode=CTD".into());

    assert!(
        fanout_stations_secret_void(&src, &env),
        "empty token is void"
    );

    env.insert("OCEANNETWORKS_TOKEN".to_string(), "secret".to_string());
    assert!(
        !fanout_stations_secret_void(&src, &env),
        "declared token is not void"
    );

    env.remove("OCEANNETWORKS_TOKEN");
    assert!(
        fanout_stations_secret_void(&src, &env),
        "absent token is void"
    );
}

#[test]
fn test_ci_probe_render_resolves_templates_and_secrets() {
    let mut env = HashMap::new();
    env.insert("FIRMS_MAP_KEY".to_string(), "ABC123".to_string());
    let url = ci_probe_render(
        "https://example.com/?lat={lat}&lon={lon}&key={FIRMS_MAP_KEY}",
        Some((52.5, 13.4)),
        &env,
    )
    .unwrap();
    assert!(url.contains("lat=52.500000"), "got {}", url);
    assert!(url.contains("lon=13.400000"), "got {}", url);
    assert!(url.contains("key=ABC123"), "got {}", url);
    assert!(!url.contains('{'), "unresolved marker in {}", url);
}

#[test]
fn test_ci_probe_render_bbox_and_temporal() {
    let env = HashMap::new();
    let url = ci_probe_render(
            "https://example.com/?bBox={lon_min},{lat_min},{lon_max},{lat_max}&start={week_ago}&end={today}",
            Some((0.0, 0.0)),
            &env,
        )
        .unwrap();
    assert!(!url.contains('{'), "unresolved marker in {}", url);
    assert!(url.contains("start=20"), "absent week_ago in {}", url);
    assert!(url.contains("end=20"), "absent today in {}", url);
}

#[test]
fn test_ci_probe_render_coord_anchor_absent_is_pending() {
    let env = HashMap::new();
    let url = ci_probe_render("https://example.com/?lat={lat}&lon={lon}", None, &env);
    assert!(
        url.is_none(),
        "absent anchor with a coordinate template is pending"
    );
}

#[test]
fn test_ci_probe_render_temporal_template_needs_no_anchor() {
    let env = HashMap::new();
    let url = ci_probe_render(
        "https://example.com/?start={week_ago}&end={today}",
        None,
        &env,
    )
    .unwrap();
    assert!(!url.contains('{'), "unresolved marker in {}", url);
    assert!(url.contains("start=20"), "absent week_ago in {}", url);
    assert!(url.contains("end=20"), "absent today in {}", url);
}

#[test]
fn test_secret_resolves_void_distinguishes_absent_and_empty() {
    let mut env = HashMap::new();
    env.insert("SET_KEY".to_string(), "v".to_string());
    env.insert("EMPTY_KEY".to_string(), String::new());
    assert!(secret_resolves_void("{ABSENT_KEY}", &env));
    assert!(secret_resolves_void("{EMPTY_KEY}", &env));
    assert!(!secret_resolves_void("{SET_KEY}", &env));
}

#[test]
fn test_alerce_object_and_detection_parse() {
    let list = r#"{"total":null,"items":[{"oid":"ZTF17aaaaaal","meanra":210.5,"meandec":-12.25,"firstmjd":58000.0},{"oid":"ZTF18bbbbbbb","meanra":null,"meandec":null}]}"#;
    let objs = alerce_objects(&parse_json(list).unwrap());
    assert_eq!(objs.len(), 1);
    assert_eq!(objs[0].0, "ZTF17aaaaaal");
    assert!((objs[0].1 - 210.5).abs() < 1e-12);
    assert!((objs[0].2 + 12.25).abs() < 1e-12);
    let det = r#"[{"ra":210.5,"dec":-12.25,"mjd":60123.4,"magpsf":18.1,"magap":18.4},{"ra":"absent","dec":0.0,"mjd":60123.4,"magpsf":19.0,"magap":19.0}]"#;
    let rows = alerce_detection_rows(&parse_json(det).unwrap());
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0], (210.5, -12.25, 60123.4, 18.1, 18.4));
}

#[test]
fn test_finals_channels_last_occupied_line() {
    let line = format!(
        "{:>2}{:>2}{:>2} {:8.2} {:1} {:9.6}{:10.6} {:9.6}{:10.6} {:1} {:10.7}",
        25, 8, 21, 61638.00, "P", 0.269050, 0.000012, 0.372959, 0.000014, "P", -0.0683654
    );
    let text = format!("{}\n{:>2}{:>2}{:>2} {:8.2}\n", line, 25, 8, 22, 61639.00);
    let src = source_fixture(
        "finals",
        vec![
            Extract::Field(field_fixture("ut1_utc", 86400.0)),
            Extract::Field(field_fixture("pmx", 86400.0)),
            Extract::Field(field_fixture("pmy", 86400.0)),
        ],
    );
    let lsk = fixture_lsk();
    let channels = build_finals_channels(&src, &text, &lsk);
    assert_eq!(channels.len(), 3);
    let expect_epoch = lsk.unix_to_tdb((61638.0 - 40587.0) * 86400.0).unwrap();
    for (c, fc) in &channels {
        assert_eq!(c.epoch, expect_epoch);
        match fc.name.as_str() {
            "ut1_utc" => assert_eq!(c.value, -0.0683654),
            "pmx" => assert_eq!(c.value, 0.269050),
            "pmy" => assert_eq!(c.value, 0.372959),
            other => panic!("unexpected field {}", other),
        }
    }
}

#[test]
fn test_ionex_channels_two_lat_five_lon() {
    let mut text = String::new();
    text.push_str("     1            CODEX                       IONEX VERSION / TYPE\n");
    text.push_str("    -1                                                              EXPONENT\n");
    text.push_str("     4 START OF TEC MAP\n");
    text.push_str(
        "  2026     8    18    12     0     0                        EPOCH OF CURRENT MAP\n",
    );
    text.push_str(
        "    87.5-180.0 180.0  90.0 450.0                            LAT/LON1/LON2/DLON/H\n",
    );
    let mut line = String::from("    87.5-180.0 180.0  90.0 450.0");
    for w in 101..=105 {
        line.push_str(&format!("{:>5}", w));
    }
    text.push_str(&line);
    text.push('\n');
    let mut line2 = String::from("    72.5-180.0 180.0  90.0 450.0");
    for w in 201..=205 {
        line2.push_str(&format!("{:>5}", w));
    }
    text.push_str(&line2);
    text.push('\n');
    text.push_str("     8 END OF TEC MAP\n");
    let src = source_fixture("ionex", vec![Extract::Field(field_fixture("tec", 7200.0))]);
    let lsk = fixture_lsk();
    let now = 1786968000.0 + 69.184 + 3600.0;
    let channels = build_ionex_channels(&src, &text, now, &lsk);
    assert_eq!(channels.len(), 10);
    let mut seen_lat = [false; 2];
    for (c, _) in &channels {
        if let Position::Surface { lat, lon, alt, .. } = &c.position {
            assert!(*alt > 400_000.0, "ionex H must set the shell altitude");
            assert!((lon + 180.0) % 90.0 == 0.0, "lon grid mismatch");
            if *lat == 87.5 {
                seen_lat[0] = true;
                assert!(c.value >= 10.1 && c.value <= 10.5);
            } else if *lat == 72.5 {
                seen_lat[1] = true;
                assert!(c.value >= 20.1 && c.value <= 20.5);
            } else {
                panic!("unexpected lat {}", lat);
            }
        }
    }
    assert!(seen_lat[0] && seen_lat[1], "both lat rows present");
}

#[test]
fn test_bl_narrowband_read_path_honors_freq_bin_width() {
    let events = vec![crate::bl_narrowband::BlNarrowbandEvent {
        ra_deg: 344.3679166,
        dec_deg: 20.7689,
        epoch_tdb: 1455000000.0,
        freq_hz: 1116.651519e6,
        bin_width_hz: 1.628e3,
        val: 107.386932,
    }];
    let bytes = crate::bl_narrowband::write_bin(&events).expect("write");
    let parsed = crate::bl_narrowband::parse_bin(&bytes).expect("parse");
    assert_eq!(parsed.len(), 1);
    let fc = field_fixture("snr", 604800.0);
    let channels: Vec<(Channel, FieldConfig)> = parsed
        .iter()
        .map(|ev| {
            (
                Channel {
                    z: 0.0,
                    freq: ev.freq_hz,
                    bin_width: ev.bin_width_hz,
                    epoch: ev.epoch_tdb,
                    position: Position::Source,
                    name: fc.name.clone(),
                    value: ev.val,
                },
                fc.clone(),
            )
        })
        .collect();
    let (ch, _) = &channels[0];
    assert_eq!(ch.freq, events[0].freq_hz);
    assert_eq!(ch.bin_width, events[0].bin_width_hz);
    assert!(
        ch.freq > 0.0 && ch.bin_width > 0.0,
        "compiled line read-back must not hard-0 the band slots"
    );
}

#[test]
fn gbco_depth_threads_hold_stations_on_the_measured_surface() {
    let recs = vec![
        crate::geo::GbcoRec {
            lat: 72.49,
            lon: -156.6,
            elev: -833.0,
        },
        crate::geo::GbcoRec {
            lat: 51.2,
            lon: 10.4,
            elev: 283.0,
        },
    ];
    let bytes = crate::geo::write_gbco(&recs);
    let parsed = crate::geo::parse_gbco(&bytes).expect("the compiler contract parses");
    assert_eq!(parsed.len(), 2);
    let threads = gbco_threads("earth", &parsed);
    assert_eq!(threads.len(), 2);
    let nrs = &threads[0];
    assert_eq!(nrs.lat, 72.49);
    assert_eq!(nrs.lon, -156.6);
    assert_eq!(nrs.alt, -833.0);
    let view = station_view(&threads);
    assert!(
        view.contains("-833"),
        "the station view carries the NRS depth: {view}"
    );
    assert!(
        view.contains("283"),
        "the station view carries the land sibling: {view}"
    );
    match nrs.motion() {
        Motion::Surface {
            body_name,
            lat,
            lon,
            alt,
        } => {
            assert_eq!(body_name, "earth");
            assert_eq!(lat, 72.49);
            assert_eq!(lon, -156.6);
            assert_eq!(alt, -833.0);
        }
        _ => panic!("the depth thread is not a surface motion"),
    }
}

#[test]
fn gbco_station_thread_projects_to_icrs_through_motion_surface() {
    let now = 840511523.88;
    let recs = vec![crate::geo::GbcoRec {
        lat: 72.49,
        lon: -156.6,
        elev: -833.0,
    }];
    let parsed = crate::geo::parse_gbco(&crate::geo::write_gbco(&recs)).unwrap();
    let threads = gbco_threads("earth", &parsed);
    let props = super::BodyProperties {
        α0_deg: 270.0,
        dα0_dt_deg_per_century: 0.003,
        δ0_deg: 66.54,
        dδ0_dt_deg_per_century: 0.013,
        w0_deg: 190.147,
        dw_dt_deg_per_day: 360.9856235,
        radius_m: 6378136.6,
        flattening: Some((6378136.6 - 6356751.9) / 6378136.6),
        gaussian_inverse_square: 340.2,
        gaussian_inverse: 5950.0,
        erfc: 3630.0,
        exponential_decay: 2.18e-5,
        patch_levy: 2.00e-5,
        gm: Some(3.986004418e14),
        j2: Some(1.08262668e-3),
        j4: Some(-1.619e-6),
        radii_b: Some(6378136.6),
        radii_c: Some(6356751.9),
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let jd_now = super::J2000_EPOCH + now / 86400.0;
    let mut eph = super::BodyEphemeris {
        granules: Vec::new(),
        rotation_matrices: Vec::new(),
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    for i in -1..=1 {
        let t0 = jd_now + i as f64 * 16.0;
        let mut cx = [0.0_f64; super::CHEBYSHEV_N];
        cx[0] = 1.5e11;
        eph.granules.push(super::ChebyshevGranule {
            t0_jd: t0,
            dt_jd: 16.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        });
    }
    let mut eph_map = std::collections::HashMap::new();
    eph_map.insert("earth".to_string(), eph);
    let icrs = threads[0]
        .icrs_at(now, &eph_map)
        .expect("the surface thread projects to ICRS at the epoch");
    assert!(icrs[0].is_finite() && icrs[1].is_finite() && icrs[2].is_finite());
}

#[test]
fn gestalt_surface_threads_roundtrip_projects_to_icrs_finite() {
    let now = 840511523.88;
    let recs = vec![
        crate::geo::GbcoRec {
            lat: -67.6,
            lon: 62.87,
            elev: -833.0,
        },
        crate::geo::GbcoRec {
            lat: 51.2,
            lon: 10.4,
            elev: 283.0,
        },
        crate::geo::GbcoRec {
            lat: 0.0,
            lon: -156.6,
            elev: 0.0,
        },
    ];
    let parsed =
        crate::geo::parse_gbco(&crate::geo::write_gbco(&recs)).expect("the GBCO roundtrip parses");
    assert_eq!(parsed.len(), 3);
    let props = super::BodyProperties {
        α0_deg: 270.0,
        dα0_dt_deg_per_century: 0.003,
        δ0_deg: 66.54,
        dδ0_dt_deg_per_century: 0.013,
        w0_deg: 190.147,
        dw_dt_deg_per_day: 360.9856235,
        radius_m: 6378136.6,
        flattening: Some((6378136.6 - 6356751.9) / 6378136.6),
        gaussian_inverse_square: 340.2,
        gaussian_inverse: 5950.0,
        erfc: 3630.0,
        exponential_decay: 2.18e-5,
        patch_levy: 2.00e-5,
        gm: Some(3.986004418e14),
        j2: Some(1.08262668e-3),
        j4: Some(-1.619e-6),
        radii_b: Some(6378136.6),
        radii_c: Some(6356751.9),
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let jd_now = super::J2000_EPOCH + now / 86400.0;
    let mut eph = super::BodyEphemeris {
        granules: Vec::new(),
        rotation_matrices: Vec::new(),
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    for i in -1..=1 {
        let t0 = jd_now + i as f64 * 16.0;
        let mut cx = [0.0_f64; super::CHEBYSHEV_N];
        cx[0] = 1.5e11;
        eph.granules.push(super::ChebyshevGranule {
            t0_jd: t0,
            dt_jd: 16.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        });
    }
    let mut eph_map = std::collections::HashMap::new();
    eph_map.insert("earth".to_string(), eph);
    let motions = gestalt_surface_threads(&parsed, "earth");
    assert_eq!(motions.len(), 3);
    for (m, rec) in motions.iter().zip(recs.iter()) {
        let icrs = m
            .at(now, now, &eph_map)
            .expect("the gestalt surface thread projects to ICRS");
        assert!(
            icrs[0].is_finite() && icrs[1].is_finite() && icrs[2].is_finite(),
            "the projected gestalt station is finite"
        );
        match m {
            Motion::Surface {
                body_name,
                lat,
                lon,
                alt,
                ..
            } => {
                assert_eq!(body_name, "earth");
                assert_eq!(*lat, rec.lat);
                assert_eq!(*lon, rec.lon);
                assert_eq!(*alt, rec.elev);
            }
            _ => panic!("the gestalt thread is not a surface motion"),
        }
    }
}

#[test]
fn gestalt_surface_threads_skip_records_without_a_finite_measured_depth() {
    let recs = vec![
        crate::geo::GbcoRec {
            lat: -67.6,
            lon: 62.87,
            elev: -833.0,
        },
        crate::geo::GbcoRec {
            lat: 44.0,
            lon: 12.0,
            elev: f64::NAN,
        },
    ];
    let motions = gestalt_surface_threads(&recs, "earth");
    assert_eq!(motions.len(), 1);
    match &motions[0] {
        Motion::Surface { alt, .. } => assert_eq!(*alt, -833.0),
        _ => panic!("the surviving gestalt thread is not a surface motion"),
    }
}

#[test]
fn gbco_asset_load_holds_gestalt_surface_threads_that_project() {
    let now = 840511523.88;
    let recs = vec![
        crate::geo::GbcoRec {
            lat: -67.6,
            lon: 62.87,
            elev: -833.0,
        },
        crate::geo::GbcoRec {
            lat: 51.2,
            lon: 10.4,
            elev: 283.0,
        },
        crate::geo::GbcoRec {
            lat: 0.0,
            lon: -156.6,
            elev: 0.0,
        },
    ];
    let path = "/tmp/opencode/gbco_gestalt_asset_test.gbco";
    if std::fs::write(path, crate::geo::write_gbco(&recs)).is_err() {
        return;
    }
    let held = load_gestalt_surface_threads(path, "earth")
        .expect("the .gbco asset load holds the gestalt surface threads");
    assert_eq!(held.len(), 3);
    let props = super::BodyProperties {
        α0_deg: 270.0,
        dα0_dt_deg_per_century: 0.003,
        δ0_deg: 66.54,
        dδ0_dt_deg_per_century: 0.013,
        w0_deg: 190.147,
        dw_dt_deg_per_day: 360.9856235,
        radius_m: 6378136.6,
        flattening: Some((6378136.6 - 6356751.9) / 6378136.6),
        gaussian_inverse_square: 340.2,
        gaussian_inverse: 5950.0,
        erfc: 3630.0,
        exponential_decay: 2.18e-5,
        patch_levy: 2.00e-5,
        gm: Some(3.986004418e14),
        j2: Some(1.08262668e-3),
        j4: Some(-1.619e-6),
        radii_b: Some(6378136.6),
        radii_c: Some(6356751.9),
        nut_ra: None,
        nut_dec: None,
        nutation: None,
        omega_g: None,
    };
    let jd_now = super::J2000_EPOCH + now / 86400.0;
    let mut eph = super::BodyEphemeris {
        granules: Vec::new(),
        rotation_matrices: Vec::new(),
        props: Some(props),
        orbit: None,
        granule_hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };
    for i in -1..=1 {
        let t0 = jd_now + i as f64 * 16.0;
        let mut cx = [0.0_f64; super::CHEBYSHEV_N];
        cx[0] = 1.5e11;
        eph.granules.push(super::ChebyshevGranule {
            t0_jd: t0,
            dt_jd: 16.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        });
    }
    let mut eph_map = std::collections::HashMap::new();
    eph_map.insert("earth".to_string(), eph);
    for (m, rec) in held.iter().zip(recs.iter()) {
        let icrs = m
            .at(now, now, &eph_map)
            .expect("the held gestalt surface thread projects to ICRS");
        assert!(
            icrs[0].is_finite() && icrs[1].is_finite() && icrs[2].is_finite(),
            "the held gestalt surface thread projects to a finite ICRS position"
        );
        match m {
            Motion::Surface {
                body_name,
                lat,
                lon,
                alt,
                ..
            } => {
                assert_eq!(body_name, "earth");
                assert_eq!(*lat, rec.lat);
                assert_eq!(*lon, rec.lon);
                assert_eq!(*alt, rec.elev);
            }
            _ => panic!("the held gestalt thread is not a surface motion"),
        }
    }
}

#[test]
fn iss_lis_geo_series_roundtrip_and_component_name() {
    let recs = vec![crate::geo::GeoRec {
        t: 753_440_003.0,
        lat: -3.117,
        lon: -76.283,
        alt: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        val: 12.5,
        comp: crate::geo::COMP_ISSLIS_FLASH_RAD,
        station: 0,
    }];
    let magic = crate::geo::magic_of("iss_lis").expect("the iss_lis format carries a magic");
    let bytes = crate::geo::write_bin(magic, &recs);
    let parsed =
        super::extract::geo_series_parse_bin("iss_lis", &bytes).expect("iss_lis bin parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].val, 12.5);
    assert_eq!(parsed[0].comp, crate::geo::COMP_ISSLIS_FLASH_RAD);
    assert_eq!(
        crate::geo::comp_max("iss_lis"),
        Some(crate::geo::COMP_ISSLIS_MAX)
    );
    assert_eq!(
        super::extract::geo_series_component_name("iss_lis", crate::geo::COMP_ISSLIS_FLASH_RAD),
        Some("iss_lis_flash_radiance_uj_sr_m2_um")
    );
}

#[test]
fn iss_lis_register_field_matches_component_name() {
    let srcs = super::load_sources();
    let src = srcs
        .iter()
        .find(|s| s.format == "iss_lis")
        .expect("phi/sources.φ registers the iss_lis source");
    let Some(Extract::Field(fc)) = src.extracts.first() else {
        panic!("the iss_lis block carries a field line");
    };
    assert_eq!(fc.name, "iss_lis_flash_radiance_uj_sr_m2_um");
    assert_eq!(fc.force, 0);
}

#[test]
fn lis_otd_geo_series_roundtrip_and_component_name() {
    let recs = vec![crate::geo::GeoRec {
        t: -148_800_000.0,
        lat: 29.98,
        lon: 48.97,
        alt: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        val: 205_838.0,
        comp: crate::geo::COMP_LISOTD_FLASH_RAD,
        station: 0,
    }];
    let magic = crate::geo::magic_of("lis_otd").expect("the lis_otd format carries a magic");
    let bytes = crate::geo::write_bin(magic, &recs);
    let parsed =
        super::extract::geo_series_parse_bin("lis_otd", &bytes).expect("lis_otd bin parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].val, 205_838.0);
    assert_eq!(parsed[0].comp, crate::geo::COMP_LISOTD_FLASH_RAD);
    assert_eq!(
        crate::geo::comp_max("lis_otd"),
        Some(crate::geo::COMP_LISOTD_MAX)
    );
    assert_eq!(
        super::extract::geo_series_component_name("lis_otd", crate::geo::COMP_LISOTD_FLASH_RAD),
        Some("lis_otd_flash_radiance_uj_sr")
    );
}

#[test]
fn trmm_lis_geo_series_roundtrip_and_component_name() {
    let recs = vec![crate::geo::GeoRec {
        t: -63_072_000.0,
        lat: -3.117,
        lon: -76.283,
        alt: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        val: 12.5,
        comp: crate::geo::COMP_TRMMLIS_FLASH_RAD,
        station: 0,
    }];
    let magic = crate::geo::magic_of("trmm_lis").expect("the trmm_lis format carries a magic");
    let bytes = crate::geo::write_bin(magic, &recs);
    let parsed =
        super::extract::geo_series_parse_bin("trmm_lis", &bytes).expect("trmm_lis bin parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].val, 12.5);
    assert_eq!(parsed[0].comp, crate::geo::COMP_TRMMLIS_FLASH_RAD);
    assert_eq!(
        crate::geo::comp_max("trmm_lis"),
        Some(crate::geo::COMP_TRMMLIS_MAX)
    );
    assert_eq!(
        super::extract::geo_series_component_name("trmm_lis", crate::geo::COMP_TRMMLIS_FLASH_RAD),
        Some("trmm_lis_flash_radiance_uj_sr_m2_um")
    );
}

#[test]
fn trmm_lis_register_field_matches_component_name() {
    let srcs = super::load_sources();
    let src = srcs
        .iter()
        .find(|s| s.format == "trmm_lis")
        .expect("phi/sources.φ registers the trmm_lis source");
    let Some(Extract::Field(fc)) = src.extracts.first() else {
        panic!("the trmm_lis block carries a field line");
    };
    assert_eq!(fc.name, "trmm_lis_flash_radiance_uj_sr_m2_um");
    assert_eq!(fc.force, 0);
}

#[test]
fn glm_l1b_geo_series_roundtrip_and_component_name() {
    let recs = vec![crate::geo::GeoRec {
        t: 669_124_869.184,
        lat: -33.7162944650696,
        lon: -66.4323253483825,
        alt: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        val: 1.867_074_491_567_66e-15,
        comp: crate::geo::COMP_GLML1B_FLASH_ENERGY,
        station: 0,
    }];
    let magic = crate::geo::magic_of("glm_l1b").expect("the glm_l1b format carries a magic");
    let bytes = crate::geo::write_bin(magic, &recs);
    let parsed =
        super::extract::geo_series_parse_bin("glm_l1b", &bytes).expect("glm_l1b bin parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].val, 1.867_074_491_567_66e-15);
    assert_eq!(parsed[0].comp, crate::geo::COMP_GLML1B_FLASH_ENERGY);
    assert_eq!(
        crate::geo::comp_max("glm_l1b"),
        Some(crate::geo::COMP_GLML1B_MAX)
    );
    assert_eq!(
        super::extract::geo_series_component_name("glm_l1b", crate::geo::COMP_GLML1B_FLASH_ENERGY),
        Some("glm_l1b_flash_radiant_energy_j")
    );
}

#[test]
fn glm_l2_geo_series_roundtrip_and_component_name() {
    let recs = vec![crate::geo::GeoRec {
        t: 820_497_600.0,
        lat: 15.772_092,
        lon: -107.996_979,
        alt: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        val: 1.428_5e-14,
        comp: crate::geo::COMP_GLML2_FLASH_ENERGY,
        station: 0,
    }];
    let magic = crate::geo::magic_of("glm_l2").expect("the glm_l2 format carries a magic");
    let bytes = crate::geo::write_bin(magic, &recs);
    let parsed = super::extract::geo_series_parse_bin("glm_l2", &bytes).expect("glm_l2 bin parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].val, 1.428_5e-14);
    assert_eq!(parsed[0].comp, crate::geo::COMP_GLML2_FLASH_ENERGY);
    assert_eq!(
        crate::geo::comp_max("glm_l2"),
        Some(crate::geo::COMP_GLML2_MAX)
    );
    assert_eq!(
        super::extract::geo_series_component_name("glm_l2", crate::geo::COMP_GLML2_FLASH_ENERGY),
        Some("glm_l2_flash_radiant_energy_j")
    );
}

#[test]
fn glm_l2_register_field_matches_component_name() {
    let srcs = super::load_sources();
    let src = srcs
        .iter()
        .find(|s| s.format == "glm_l2")
        .expect("phi/sources.φ registers the glm_l2 source");
    let Some(Extract::Field(fc)) = src.extracts.first() else {
        panic!("the glm_l2 block carries a field line");
    };
    assert_eq!(fc.name, "glm_l2_flash_radiant_energy_j");
    assert_eq!(fc.force, 0);
}

#[test]
fn glm_l1b_register_field_matches_component_name() {
    let srcs = super::load_sources();
    let src = srcs
        .iter()
        .find(|s| s.format == "glm_l1b")
        .expect("phi/sources.φ registers the glm_l1b source");
    let Some(Extract::Field(fc)) = src.extracts.first() else {
        panic!("the glm_l1b block carries a field line");
    };
    assert_eq!(fc.name, "glm_l1b_flash_radiant_energy_j");
    assert_eq!(fc.force, 0);
}

#[test]
fn supermag_geo_series_roundtrip_and_component_name() {
    let recs = vec![
        crate::geo::GeoRec {
            t: 753_440_003.0,
            lat: 69.66,
            lon: 18.94,
            alt: 0.0,
            freq: 0.0,
            bin_width: 60.0,
            val: -177.9,
            comp: crate::geo::COMP_SMG_N_NEZ,
            station: crate::geo::pack_iaga("TRO").unwrap(),
        },
        crate::geo::GeoRec {
            t: 753_440_003.0,
            lat: 69.66,
            lon: 18.94,
            alt: 0.0,
            freq: 0.0,
            bin_width: 60.0,
            val: -186.0,
            comp: crate::geo::COMP_SMG_N_GEO,
            station: crate::geo::pack_iaga("TRO").unwrap(),
        },
    ];
    let magic =
        crate::geo::magic_of("supermag_1m").expect("the supermag_1m format carries a magic");
    let bytes = crate::geo::write_bin(magic, &recs);
    let parsed = super::extract::geo_series_parse_bin("supermag_1m", &bytes)
        .expect("supermag_1m bin parses");
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].val, -177.9);
    assert_eq!(parsed[0].comp, crate::geo::COMP_SMG_N_NEZ);
    assert_eq!(parsed[1].comp, crate::geo::COMP_SMG_N_GEO);
    assert_eq!(
        crate::geo::iaga_of(parsed[0].station).as_deref(),
        Some("TRO")
    );
    assert_eq!(
        crate::geo::comp_max("supermag_1m"),
        Some(crate::geo::COMP_SMG_MAX)
    );
    assert_eq!(
        super::extract::geo_series_component_name("supermag_1m", crate::geo::COMP_SMG_N_NEZ),
        Some("supermag_n_nez_nt")
    );
    assert_eq!(
        super::extract::geo_series_component_name("supermag_1m", crate::geo::COMP_SMG_Z_GEO),
        Some("supermag_z_geo_nt")
    );
}

#[test]
fn supermag_register_field_matches_component_name() {
    let srcs = super::load_sources();
    let src = srcs
        .iter()
        .find(|s| s.format == "supermag_1m")
        .expect("phi/sources.φ registers the supermag_1m source");
    let names: Vec<&str> = src
        .extracts
        .iter()
        .filter_map(|e| match e {
            Extract::Field(fc) => Some(fc.name.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(
        names,
        vec![
            "supermag_n_nez_nt",
            "supermag_e_nez_nt",
            "supermag_z_nez_nt",
            "supermag_n_geo_nt",
            "supermag_e_geo_nt",
            "supermag_z_geo_nt",
        ]
    );
    let Some(Extract::Field(fc)) = src.extracts.first() else {
        panic!("the supermag_1m block carries a field line");
    };
    assert_eq!(fc.force, 0);
}

#[test]
fn noaa_nodd_geo_series_roundtrip_and_component_names() {
    let recs = vec![
        crate::geo::GeoRec {
            t: 800_000_000.0,
            lat: 40.7,
            lon: -74.0,
            alt: 10.0,
            freq: 0.0,
            bin_width: 86400.0,
            val: 28.9,
            comp: crate::geo::COMP_GHCN_TMAX,
            station: 0,
        },
        crate::geo::GeoRec {
            t: 800_000_000.0,
            lat: 40.7,
            lon: -74.0,
            alt: 10.0,
            freq: 0.0,
            bin_width: 86400.0,
            val: 5.0,
            comp: crate::geo::COMP_GHCN_PRCP,
            station: 0,
        },
    ];
    let magic =
        crate::geo::magic_of("noaa_ghcn_d").expect("the noaa_ghcn_d format carries a magic");
    let bytes = crate::geo::write_bin(magic, &recs);
    let parsed = super::extract::geo_series_parse_bin("noaa_ghcn_d", &bytes)
        .expect("noaa_ghcn_d bin parses");
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].val, 28.9);
    assert_eq!(parsed[1].comp, crate::geo::COMP_GHCN_PRCP);
    assert_eq!(
        crate::geo::comp_max("noaa_ghcn_d"),
        Some(crate::geo::COMP_GHCN_MAX)
    );
    assert_eq!(
        crate::geo::comp_max("noaa_gsod"),
        Some(crate::geo::COMP_GSOD_MAX)
    );
    assert_eq!(
        crate::geo::comp_max("noaa_isd"),
        Some(crate::geo::COMP_ISD_MAX)
    );
    assert_eq!(
        super::extract::geo_series_component_name("noaa_ghcn_d", crate::geo::COMP_GHCN_TMAX),
        Some("noaa_ghcn_d_tmax_c")
    );
    assert_eq!(
        super::extract::geo_series_component_name("noaa_ghcn_d", crate::geo::COMP_GHCN_SNWD),
        Some("noaa_ghcn_d_snwd_mm")
    );
    assert_eq!(
        super::extract::geo_series_component_name("noaa_gsod", crate::geo::COMP_GSOD_GUST),
        Some("noaa_gsod_gust_ms")
    );
    assert_eq!(
        super::extract::geo_series_component_name("noaa_isd", crate::geo::COMP_ISD_WDIR),
        Some("noaa_isd_wdir_deg")
    );
}

#[test]
fn noaa_nodd_register_field_matches_component_name() {
    let srcs = super::load_sources();
    let names = |format: &str| -> Vec<String> {
        let src = match srcs.iter().find(|s| s.format == format) {
            Some(s) => s,
            None => panic!("phi/sources.φ registers {format}"),
        };
        src.extracts
            .iter()
            .filter_map(|e| match e {
                Extract::Field(fc) => Some(fc.name.clone()),
                _ => None,
            })
            .collect()
    };
    assert_eq!(names("noaa_cdo_ghcnd_tmax"), vec!["noaa_cdo_tmax_c"]);
    assert_eq!(
        names("noaa_isd"),
        vec![
            "noaa_isd_temp_c",
            "noaa_isd_dewp_c",
            "noaa_isd_wdir_deg",
            "noaa_isd_wspd_ms",
            "noaa_isd_slp_hpa",
        ]
    );
    assert_eq!(
        names("noaa_gsod"),
        vec![
            "noaa_gsod_temp_c",
            "noaa_gsod_dewp_c",
            "noaa_gsod_slp_hpa",
            "noaa_gsod_wdsp_ms",
            "noaa_gsod_gust_ms",
            "noaa_gsod_max_c",
            "noaa_gsod_min_c",
            "noaa_gsod_prcp_mm",
        ]
    );
}

#[test]
fn noaa_nodd_parsers_convert_and_skip_missing() {
    let lsk = crate::lsk::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1_483_228_800.0)],
    };

    let ghcn = "ID,DATE,ELEMENT,DATA_VALUE,M_FLAG,Q_FLAG,S_FLAG,OBS_TIME\n\
USW00094728,20250301,TMAX,289,,,X,\n\
USW00094728,20250301,TMIN,-5,,,X,\n\
USW00094728,20250301,PRCP,50,,,X,\n\
USW00094728,20250301,PRCP,-9999,,,X,\n";
    let recs = super::noaa_nodd::parse_ghcn(ghcn, (40.7, -74.0, 10.0), &lsk);
    let pairs: Vec<(u32, f64)> = recs.iter().map(|r| (r.comp, r.val)).collect();
    assert_eq!(
        pairs,
        vec![
            (crate::geo::COMP_GHCN_TMAX, 28.9),
            (crate::geo::COMP_GHCN_TMIN, -0.5),
            (crate::geo::COMP_GHCN_PRCP, 5.0),
        ]
    );

    let gsod = "\"STATION\",\"DATE\",\"LATITUDE\",\"LONGITUDE\",\"ELEVATION\",\"NAME\",\"TEMP\",\"DEWP\",\"SLP\",\"WDSP\",\"GUST\",\"MAX\",\"MIN\",\"PRCP\"\n\
\"01001099999\",\"2025-01-01\",\"70.9333333\",\"-8.6666667\",\"9.0\",\"JAN MAYEN\",\"17.4\",\"10.3\",\"1014.9\",\"2.0\",\"27.2\",\"39.4\",\"9.7\",\"0.05\"\n\
\"01001099999\",\"2025-01-02\",\"70.9333333\",\"-8.6666667\",\"9.0\",\"JAN MAYEN\",\"9999.9\",\"9999.9\",\"9999.9\",\"999.9\",\"999.9\",\"9999.9\",\"9999.9\",\"99.99\"\n";
    let recs = super::noaa_nodd::parse_gsod(gsod, &lsk);
    assert_eq!(recs.len(), 8);
    let temp = recs
        .iter()
        .find(|r| r.comp == crate::geo::COMP_GSOD_TEMP)
        .map(|r| r.val)
        .unwrap();
    assert!((temp - (17.4 - 32.0) * 5.0 / 9.0).abs() < 1e-9);
    let wdsp = recs
        .iter()
        .find(|r| r.comp == crate::geo::COMP_GSOD_WDSP)
        .map(|r| r.val)
        .unwrap();
    assert!((wdsp - 2.0 * 0.514_444).abs() < 1e-9);
    let prcp = recs
        .iter()
        .find(|r| r.comp == crate::geo::COMP_GSOD_PRCP)
        .map(|r| r.val)
        .unwrap();
    assert!((prcp - 0.05 * 25.4).abs() < 1e-9);

    let isd = "\"STATION\",\"DATE\",\"LATITUDE\",\"LONGITUDE\",\"ELEVATION\",\"WND\",\"TMP\",\"DEW\",\"SLP\"\n\
\"01001099999\",\"2025-01-01T00:00:00\",\"70.9333333\",\"-8.6666667\",\"9.0\",\"328,1,N,0070,1\",\"-0042,1\",\"-0084,1\",\"10119,1\"\n\
\"01001099999\",\"2025-01-01T03:00:00\",\"70.9333333\",\"-8.6666667\",\"9.0\",\"99999,9,9,9\",\"+9999,1\",\"+9999,1\",\"99999,1\"\n";
    let recs = super::noaa_nodd::parse_isd(isd, &lsk);
    assert_eq!(recs.len(), 5);
    let tmp = recs
        .iter()
        .find(|r| r.comp == crate::geo::COMP_ISD_TEMP)
        .map(|r| r.val)
        .unwrap();
    assert!((tmp - -4.2).abs() < 1e-9);
    let slp = recs
        .iter()
        .find(|r| r.comp == crate::geo::COMP_ISD_SLP)
        .map(|r| r.val)
        .unwrap();
    assert!((slp - 1011.9).abs() < 1e-9);
    let wdir = recs
        .iter()
        .find(|r| r.comp == crate::geo::COMP_ISD_WDIR)
        .map(|r| r.val)
        .unwrap();
    assert_eq!(wdir, 328.0);
    let wspd = recs
        .iter()
        .find(|r| r.comp == crate::geo::COMP_ISD_WSPD)
        .map(|r| r.val)
        .unwrap();
    assert!((wspd - 7.0).abs() < 1e-9);
}

#[test]
fn uscrn_parsers_convert_and_skip_missing() {
    let lsk = crate::lsk::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1_483_228_800.0)],
    };

    let stations = "WBAN\tCOUNTRY\tSTATE\tLOCATION\tVECTOR\tNAME\tLATITUDE\tLONGITUDE\tELEVATION\tSTATUS\tCOMMISSIONING\tCLOSING\tOPERATION\tPAIRING\tNETWORK\tSTATION_ID\n\
23583\tUS\tAK\tAleknagik\t1 NNE\tCity of Aleknagik, Aleknagik Airport\t59.28\t-158.61\t80\tCommissioned\t2020-10-13 00:00:00.0\t\tOperational\t\tUSCRN\t1801\n";
    let anchors = super::noaa_nodd::parse_uscrn_stations(stations);
    let anchor = anchors
        .get("23583")
        .expect("the Aleknagik WBAN carries an anchor");
    assert_eq!(anchor.0, 59.28);
    assert_eq!(anchor.1, -158.61);
    assert!((anchor.2 - 80.0 * 0.3048).abs() < 1e-9);

    let uscrn = "23583 20260101 0100 20251231 1600  2.514 -158.61   59.28   -16.3   -15.8   -15.5   -16.7     0.0     37 0     78 0      2 0 C   -16.5 0   -15.9 0   -17.5 0    74 0 -99.000 -99.000 -99.000 -99.000 -99.000 -9999.0 -9999.0 -9999.0 -9999.0 -9999.0\n\
23583 20260101 0200 20251231 1700  2.514 -158.61   59.28 -9999.0   -16.9   -16.0   -17.6     0.0      2 0     21 0      0 0 C   -18.4 0   -17.5 0   -19.2 0    76 0 -99.000 -99.000 -99.000 -99.000 -99.000 -9999.0 -9999.0 -9999.0 -9999.0 -9999.0\n";
    assert_eq!(
        super::noaa_nodd::uscrn_wban(uscrn).as_deref(),
        Some("23583")
    );
    let recs = super::noaa_nodd::parse_uscrn(uscrn, anchor.2, &lsk);
    assert_eq!(recs.len(), 1);
    assert_eq!(recs[0].comp, crate::geo::COMP_USCRN_TEMP);
    assert!((recs[0].val - -16.3).abs() < 1e-9);
    assert!((recs[0].bin_width - 3600.0).abs() < 1e-9);
    assert_eq!(recs[0].lat, 59.28);
    assert_eq!(recs[0].lon, -158.61);
    assert!((recs[0].alt - anchor.2).abs() < 1e-9);

    let magic = crate::geo::magic_of("us_crn_hourly").expect("us_crn_hourly carries a magic");
    let bytes = crate::geo::write_bin(magic, &recs);
    let parsed = super::extract::geo_series_parse_bin("us_crn_hourly", &bytes)
        .expect("us_crn_hourly bin parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(
        super::extract::geo_series_component_name("us_crn_hourly", crate::geo::COMP_USCRN_TEMP),
        Some("us_crn_hourly_temp_c")
    );
    assert_eq!(
        crate::geo::comp_max("us_crn_hourly"),
        Some(crate::geo::COMP_USCRN_MAX)
    );
    assert_eq!(
        super::zeuge::magic_identity(magic),
        Some(super::zeuge::FeldIdentitaet::Oszillator)
    );
}

#[test]
fn galileo_odr_register_field_matches_component_name() {
    let srcs = super::load_sources();
    let src = srcs
        .iter()
        .find(|s| s.format == "galileo_odr")
        .expect("phi/sources.φ registers the galileo_odr source");
    let names: Vec<&str> = src
        .extracts
        .iter()
        .filter_map(|e| match e {
            Extract::Field(fc) => Some(fc.name.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(
        names,
        vec![
            "galileo_odr_ad1_count",
            "galileo_odr_ad2_count",
            "galileo_odr_ad3_count",
            "galileo_odr_ad4_count",
        ]
    );
    let Some(Extract::Field(fc)) = src.extracts.first() else {
        panic!("the galileo_odr block carries a field line");
    };
    assert_eq!(fc.force, 0);
    assert_eq!(fc.kernel, 0);
    assert_eq!(fc.unit, "count");
}

#[test]
fn register_hapi_units_of_maps_short_parameter_to_field_unit() {
    let content = "url https://vires.services/hapi/data?id=CH_OPER_WND_ACC_2_&start=2009-06-01T00:00:00Z&stop=2009-06-01T00:59:59Z&parameters=crosswind&format=json\nttl 10\nat earth\nhapi crosswind=champ_thermosphere_crosswind_ms\nfield champ_thermosphere_crosswind_ms champ_thermosphere_crosswind_ms gaussian-inverse-square advective m/s 10 0.0 0.0\n";
    let srcs = super::parse_sources(content);
    let map = super::register_hapi_units_of(&srcs);
    assert_eq!(
        map.get(&("CH_OPER_WND_ACC_2_".to_string(), "crosswind".to_string())),
        Some(&"m/s".to_string())
    );
}

#[test]
fn hapi_draft_names_register_unit_when_server_unit_is_off_registry() {
    let url = "https://vires.services/hapi/data?id=CH_OPER_WND_ACC_2_&start=2010-09-04T20:00:00Z&stop=2010-09-04T20:59:59Z&parameters=crosswind&format=json";
    let body = r#"{"parameters":[{"name":"Timestamp","units":"UTC"},{"name":"crosswind","units":"kg/m3"}],"data":[["2010-09-04T20:00:00.000Z",Infinity],["2010-09-04T20:00:10.000Z",12.5]]}"#;
    let parsed = super::parse_json(body).expect("hapi fixture parses");
    let mut fields = String::new();
    assert!(super::hapi_draft_fields(
        url,
        &parsed,
        &HashMap::new(),
        &mut fields
    ));
    assert!(
        fields.contains("# unit kg/m3 not in force registry — register carries m/s — review"),
        "off-registry note names the register unit: {fields}"
    );
    assert!(
        fields.contains("field crosswind crosswind patch-levy advective kg/m3 60 0.0 0.0"),
        "field line keeps the server unit verbatim: {fields}"
    );
    assert!(
        fields.contains("# crosswind = 12.5 — first row server fill, first finite sample shown"),
        "sample shows the first finite row: {fields}"
    );
}

fn fits_card(kw: &str, value: &str) -> [u8; 80] {
    let mut card = [b' '; 80];
    let k = kw.as_bytes();
    card[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
    card[8] = b'=';
    let v = value.as_bytes();
    card[10..10 + v.len().min(20)].copy_from_slice(&v[..v.len().min(20)]);
    card
}

fn fits_bintable_fixture() -> Vec<u8> {
    let mut buf = Vec::new();
    let mut header: Vec<u8> = Vec::new();
    header.extend_from_slice(&fits_card("SIMPLE", "T"));
    header.extend_from_slice(&fits_card("BITPIX", "8"));
    header.extend_from_slice(&fits_card("NAXIS", "0"));
    header.extend_from_slice(&fits_card("END", ""));
    while !header.len().is_multiple_of(2880) {
        header.extend_from_slice(&[b' '; 80]);
    }
    buf.extend_from_slice(&header);

    let mut ext: Vec<u8> = Vec::new();
    ext.extend_from_slice(&fits_card("XTENSION", "'BINTABLE'"));
    ext.extend_from_slice(&fits_card("BITPIX", "8"));
    ext.extend_from_slice(&fits_card("NAXIS", "2"));
    ext.extend_from_slice(&fits_card("NAXIS1", "17"));
    ext.extend_from_slice(&fits_card("NAXIS2", "1"));
    ext.extend_from_slice(&fits_card("PCOUNT", "0"));
    ext.extend_from_slice(&fits_card("GCOUNT", "1"));
    ext.extend_from_slice(&fits_card("TFIELDS", "4"));
    ext.extend_from_slice(&fits_card("TTYPE1", "'FLUX'"));
    ext.extend_from_slice(&fits_card("TFORM1", "D"));
    ext.extend_from_slice(&fits_card("TTYPE2", "'ID'"));
    ext.extend_from_slice(&fits_card("TFORM2", "J"));
    ext.extend_from_slice(&fits_card("TTYPE3", "'NAME'"));
    ext.extend_from_slice(&fits_card("TFORM3", "4A"));
    ext.extend_from_slice(&fits_card("TTYPE4", "'OK'"));
    ext.extend_from_slice(&fits_card("TFORM4", "L"));
    ext.extend_from_slice(&fits_card("END", ""));
    while !ext.len().is_multiple_of(2880) {
        ext.extend_from_slice(&[b' '; 80]);
    }
    buf.extend_from_slice(&ext);

    let mut row = Vec::new();
    row.extend_from_slice(&42.5f64.to_be_bytes());
    row.extend_from_slice(&7i32.to_be_bytes());
    row.extend_from_slice(b"AB  ");
    row.push(b'T');
    buf.extend_from_slice(&row);
    while buf.len() % 2880 != 0 {
        buf.push(0);
    }
    buf
}

#[test]
fn fits_bintable_roundtrips_typed_row() {
    let buf = fits_bintable_fixture();
    let (t, _next) = fits::FitsTable::parse(&buf, 2880).unwrap();
    assert_eq!(t.n_rows, 1);
    assert_eq!(t.row_bytes, 17);
    let cols: Vec<&str> = t.columns.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(cols, vec!["FLUX", "ID", "NAME", "OK"]);
    let row = t.row(&buf, 0).unwrap();
    assert_eq!(
        row,
        vec![
            fits::FitsValue::Float(42.5),
            fits::FitsValue::Int(7),
            fits::FitsValue::Str("AB".to_string()),
            fits::FitsValue::Bool(true),
        ]
    );
    assert!(t.row(&buf, 1).is_none());
}

#[test]
fn fits_format_extracts_last_row() {
    let buf = fits_bintable_fixture();
    let dir = std::env::temp_dir();
    let path = dir.join("omegaflow_fits_test.fits");
    std::fs::write(&path, &buf).unwrap();
    let fc = FieldConfig {
        key: "FLUX".into(),
        name: "flux".into(),
        kernel: 0,
        force: 0,
        tau: 604800.0,
        absorption: 0.0,
        advection: 0.0,
        unit: String::new(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
    };
    let src = SourceConfig {
        ttl: 604800,
        url: "https://example.com/x.fits".into(),
        frame: Frame::Manifest,
        format: "fits".into(),
        extracts: vec![Extract::Last(fc, None)],
        headers: vec![],
        post_body: None,
        target: None,
        catalog: None,
        max_freq: None,
        min_freq: None,
        body: None,
        stations_url: None,
        stations_path: String::new(),
        stations_lat: String::new(),
        stations_lon: String::new(),
        stations_id: String::new(),
        hapi_fill: HashMap::new(),
        flux_from_mag: None,
        abs_mag_from: None,
        catalog_epoch: None,
        repeat_ra_bins: 0,
        fanout_cap: 0,
        stations_flatten: String::new(),
        stations_filter: None,
        fanout_delay: 0,
        sha256: None,
        window: None,
        live_only: false,
    };
    match extract(&src, path.to_str().unwrap(), 8.0e8, &fixture_lsk()) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            assert_eq!(channels[0].1.name, "flux");
            assert_eq!(channels[0].0.value, 42.5);
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
    let _ = std::fs::remove_file(&path);
}

fn tar_member_fixture(name: &str, content: &[u8]) -> Vec<u8> {
    let mut h = [0u8; 512];
    h[..name.len()].copy_from_slice(name.as_bytes());
    let octal = format!("{:011o}", content.len());
    h[124..135].copy_from_slice(octal.as_bytes());
    h[156] = b'0';
    h[257..262].copy_from_slice(b"ustar");
    h[148..156].fill(b' ');
    let sum: u32 = h.iter().map(|&b| b as u32).sum();
    let checksum = format!("{sum:06o}");
    h[148..154].copy_from_slice(checksum.as_bytes());
    h[154] = 0;
    h[155] = b' ';
    let mut out = h.to_vec();
    out.extend_from_slice(content);
    out.resize(out.len().div_ceil(512) * 512, 0);
    out
}

fn gzip_stored_fixture(content: &[u8]) -> Vec<u8> {
    let mut gz = vec![0x1f, 0x8b, 0x08, 0x00, 0, 0, 0, 0, 0x00, 0x03];
    let len = content.len() as u16;
    gz.push(0x01);
    gz.extend_from_slice(&len.to_le_bytes());
    gz.extend_from_slice(&(!len).to_le_bytes());
    gz.extend_from_slice(content);
    gz
}

fn tar_gz_yaml_fixture() -> Vec<u8> {
    let kbr = "header:\n  dimensions:\n    num_records: 2\n  variables:\n    - gps_time:\n        comment: 1st column\n        units: second\n    - biased_range:\n        comment: 2nd column\n        units: m\n    - range_rate:\n        comment: 3rd column\n        units: m/s\n    - range_accl:\n        comment: 4th column\n        units: m/s2\n# End of YAML header\n580282650 1200.5 -0.25 0.001\n580282658 1200.6 -0.24 0.002\n";
    let empty = "header:\n  dimensions:\n    num_records: 0\n  variables:\n    - gps_time:\n        comment: 1st column\n# End of YAML header\n";
    let mut tar = Vec::new();
    tar.extend_from_slice(&tar_member_fixture(
        "ACT1B_2018-05-22_C_04.txt",
        empty.as_bytes(),
    ));
    tar.extend_from_slice(&tar_member_fixture(
        "KBR1B_2018-05-22_Y_04.txt",
        kbr.as_bytes(),
    ));
    tar.extend_from_slice(&[0u8; 1024]);
    gzip_stored_fixture(&tar)
}

#[test]
fn tar_gz_yaml_format_extracts_member_last_row() {
    let buf = tar_gz_yaml_fixture();
    let path = std::env::temp_dir().join("omegaflow_tar_gz_test.tgz");
    std::fs::write(&path, &buf).unwrap();
    let fc = FieldConfig {
        key: "KBR1B.range_rate".into(),
        name: "gracefo_kbr_range_rate".into(),
        kernel: 0,
        force: 4,
        tau: 86400.0,
        absorption: 0.0,
        advection: 0.0,
        unit: "m/s".into(),
        freq: 0.0,
        bin_width: 0.0,
        fold: None,
    };
    let mut src = source_fixture("tar_gz_yaml", vec![Extract::Last(fc, None)]);
    src.frame = Frame::Manifest;
    match extract(&src, path.to_str().unwrap(), 8.0e8, &fixture_lsk()) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            assert_eq!(channels[0].1.name, "gracefo_kbr_range_rate");
            assert_eq!(channels[0].0.value, -0.24);
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
    let _ = std::fs::remove_file(&path);
}

#[test]
#[ignore = "reads the GRACE-FO tarball named by OMEGAFLOW_GRACE_TGZ"]
fn real_gracefo_l1b_tarball_parses_a_member() {
    let path =
        std::env::var("OMEGAFLOW_GRACE_TGZ").expect("OMEGAFLOW_GRACE_TGZ names a .tgz on disk");
    let mut fc = field_fixture("KBR1B.range_rate", 86400.0);
    fc.name = "gracefo_kbr_range_rate_m_s".into();
    let src = source_fixture("tar_gz_yaml", vec![Extract::Last(fc, None)]);
    match extract(&src, &path, 8.0e8, &fixture_lsk()) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(
                channels.len(),
                1,
                "the real KBR1B series carries a last row"
            );
            assert!(channels[0].0.value.is_finite());
        }
        ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
    }
}

#[test]
fn cosmic_ro_geo_series_roundtrip_and_component_name() {
    let recs = vec![
        crate::geo::GeoRec {
            t: 753_440_003.0,
            lat: -20.0,
            lon: -76.0,
            alt: 10000.0,
            freq: 0.0,
            bin_width: 0.0,
            val: 250.0,
            comp: crate::geo::COMP_COSMIC_REFRACT,
            station: 0,
        },
        crate::geo::GeoRec {
            t: 753_440_003.0,
            lat: -20.0,
            lon: -76.0,
            alt: 10000.0,
            freq: 0.0,
            bin_width: 0.0,
            val: 220.0,
            comp: crate::geo::COMP_COSMIC_TEMP,
            station: 0,
        },
    ];
    let magic = crate::geo::magic_of("cosmic_ro").expect("cosmic_ro carries a magic");
    let bytes = crate::geo::write_bin(magic, &recs);
    let parsed =
        super::extract::geo_series_parse_bin("cosmic_ro", &bytes).expect("cosmic_ro bin parses");
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].val, 250.0);
    assert_eq!(parsed[0].comp, crate::geo::COMP_COSMIC_REFRACT);
    assert_eq!(
        super::extract::geo_series_component_name("cosmic_ro", crate::geo::COMP_COSMIC_REFRACT),
        Some("cosmic_ro_refractivity_n_units")
    );
    assert_eq!(
        super::extract::geo_series_component_name("cosmic_ro", crate::geo::COMP_COSMIC_TEMP),
        Some("cosmic_ro_temperature_k")
    );
    assert_eq!(
        super::extract::geo_series_component_name("cosmic_ro", crate::geo::COMP_COSMIC_PRES),
        Some("cosmic_ro_pressure_hpa")
    );
    assert_eq!(
        super::extract::geo_series_component_name("cosmic_ro", 99),
        None
    );
}

#[test]
fn maxi_series_roundtrip_and_component_name() {
    let curves = vec![crate::maxi::MaxiCurve {
        ra_deg: 1.5814,
        dec_deg: 20.2029,
        band: crate::maxi::BAND_2_20,
        freq_hz: 2.6e18,
        bin_width_hz: 4.3e18,
        samples: vec![
            crate::maxi::MaxiSample {
                t_tdb: 8.0e8,
                flux: 0.04,
                err: 0.02,
            },
            crate::maxi::MaxiSample {
                t_tdb: 8.1e8,
                flux: -0.03,
                err: 0.01,
            },
        ],
    }];
    let bytes = crate::maxi::write_bin(&curves).expect("maxi bin writes");
    let parsed = super::extract::series_parse_bin("maxi", &bytes).expect("maxi series parses");
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].0, 8.0e8);
    assert_eq!(parsed[0].1, 0.04f32 as f64);
    assert_eq!(parsed[0].2, crate::maxi::BAND_2_20);
    assert_eq!(parsed[1].1, -0.03f32 as f64);
    assert_eq!(
        super::extract::series_component_name("maxi", crate::maxi::BAND_2_20),
        Some("maxi_2_20kev_flux_ph_s_cm2")
    );
    assert_eq!(
        super::extract::series_component_name("maxi", crate::maxi::BAND_2_4),
        Some("maxi_2_4kev_flux_ph_s_cm2")
    );
    assert_eq!(
        super::extract::series_component_name("maxi", crate::maxi::BAND_4_10),
        Some("maxi_4_10kev_flux_ph_s_cm2")
    );
    assert_eq!(
        super::extract::series_component_name("maxi", crate::maxi::BAND_10_20),
        Some("maxi_10_20kev_flux_ph_s_cm2")
    );
    assert_eq!(super::extract::series_component_name("maxi", 99), None);
}

#[test]
fn atdf_series_roundtrip_and_component_name() {
    let mut row = [0.0f64; 14];
    row[0] = 729_777_632.0;
    row[1] = 2292.0e6 + 1.5e6;
    let bytes = crate::atdf::write_bin(&[row]);
    let parsed = super::extract::series_parse_bin("atdf", &bytes).expect("atdf series parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].0, row[0]);
    assert_eq!(parsed[0].1, row[1]);
    assert_eq!(parsed[0].2, crate::atdf::COMP_SKYFREQ);
    assert_eq!(
        super::extract::series_component_name("atdf", crate::atdf::COMP_SKYFREQ),
        Some("pioneer_sky_frequency_hz")
    );
    assert_eq!(super::extract::series_component_name("atdf", 99), None);
}

#[test]
fn atdf_series_skips_absent_frequency() {
    let mut row = [0.0f64; 14];
    row[0] = 729_777_632.0;
    row[1] = 0.0;
    let bytes = crate::atdf::write_bin(&[row]);
    assert!(super::extract::series_parse_bin("atdf", &bytes).is_none());
}

#[test]
fn himawari_hsd_series_roundtrip_and_component_name() {
    let seg = crate::hsd::AhiSegment {
        columns: 2,
        lines: 1,
        bits_per_pixel: 11,
        band: 7,
        segment: 1,
        satellite: 8,
        resolution_m: 2000,
        obs_sec: 729_777_632.0,
        obs_present: 1,
        calib_present: 1,
        error_pixels: 0,
        outside_scan_pixels: 0,
        counts: Vec::new(),
        radiance: vec![1.5, 2.5],
    };
    let bytes = crate::hsd::write_segment(&seg);
    let parsed =
        super::extract::series_parse_bin("himawari_hsd", &bytes).expect("himawari series parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].0, seg.obs_sec);
    assert_eq!(parsed[0].1, 2.0);
    assert_eq!(parsed[0].2, crate::hsd::COMP_RADIANCE);
    assert_eq!(
        super::extract::series_component_name("himawari_hsd", crate::hsd::COMP_RADIANCE),
        Some("himawari_ahi_radiance")
    );
    assert_eq!(
        super::extract::series_component_name("himawari_hsd", 99),
        None
    );
}

#[test]
fn gk2a_ami_series_roundtrip_and_component_name() {
    let granule = crate::gk2a_ami::AmiGranule {
        t: 729_777_632.227_242_1,
        band_id: 87,
        calib: crate::gk2a_ami::CALIB_GSICS,
        band_wavelength: 8.7,
        esun: 0.0,
        kappa0: 0.0,
        center_lat: 0.0,
        center_lon: 128.2,
        rad_mean: 5.56,
        rad_std: 0.5,
        rad_min: 0.1,
        rad_max: 12.0,
        valid: 30_000_000,
        total: 30_250_000,
    };
    let bytes =
        crate::gk2a_ami::write_bin(std::slice::from_ref(&granule)).expect("gk2a bin writes");
    let parsed = super::extract::series_parse_bin("gk2a_ami", &bytes).expect("gk2a series parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].0, granule.t);
    assert_eq!(parsed[0].1, 5.56f32 as f64);
    assert_eq!(parsed[0].2, crate::gk2a_ami::COMP_RADIANCE);
    assert_eq!(
        super::extract::series_component_name("gk2a_ami", crate::gk2a_ami::COMP_RADIANCE),
        Some("gk2a_ami_radiance")
    );
    assert_eq!(super::extract::series_component_name("gk2a_ami", 99), None);
}

#[test]
fn goes_abi_series_roundtrip_and_component_name() {
    let granule = crate::goes_abi::AbiGranule {
        t: 729_777_632.227_242_1,
        band_id: 1,
        calib: crate::goes_abi::CALIB_GSICS,
        band_wavelength: 0.47,
        esun: 0.0,
        kappa0: 0.0,
        sub_lon: -75.0,
        persp_h: 35_786_000.0,
        rad_mean: 5.56,
        rad_std: 0.5,
        rad_min: 0.1,
        rad_max: 12.0,
        valid: 30_000_000,
        total: 30_250_000,
    };
    let bytes =
        crate::goes_abi::write_bin(std::slice::from_ref(&granule)).expect("goes_abi bin writes");
    let parsed =
        super::extract::series_parse_bin("goes_abi", &bytes).expect("goes_abi series parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].0, granule.t);
    assert_eq!(parsed[0].1, 5.56f32 as f64);
    assert_eq!(parsed[0].2, crate::goes_abi::COMP_RADIANCE);
    assert_eq!(
        super::extract::series_component_name("goes_abi", crate::goes_abi::COMP_RADIANCE),
        Some("goes_abi_radiance")
    );
    assert_eq!(super::extract::series_component_name("goes_abi", 99), None);
}

#[test]
fn voyager_saturn_series_dispatch_and_component_names() {
    let mut row = [0.0f64; super::voyager_saturn::VSAT_STRIDE];
    row[0] = 0.0;
    row[1] = 80.0;
    row[2] = 296.0;
    row[3] = 0.0;
    row[4] = 8.0;
    row[5] = 0.0;
    row[14] = 0x5abee as f64;
    row[15] = 0x530c36 as f64;
    let bytes = super::voyager_saturn::write_vsat_bin(&[row]);
    let parsed = super::extract::series_parse_bin("voyager_saturn", &bytes)
        .expect("voyager_saturn series parses");
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].1, 0x5abee as f64);
    assert_eq!(parsed[0].2, super::voyager_saturn::COMP_DOPPLER_HP);
    assert_eq!(parsed[1].1, 0x530c36 as f64);
    assert_eq!(parsed[1].2, super::voyager_saturn::COMP_DOPPLER_LP);
    assert_eq!(
        super::extract::series_component_name(
            "voyager_saturn",
            super::voyager_saturn::COMP_DOPPLER_HP
        ),
        Some("voyager_saturn_doppler_count_hp")
    );
    assert_eq!(
        super::extract::series_component_name(
            "voyager_saturn",
            super::voyager_saturn::COMP_ANGLE_A
        ),
        Some("voyager_saturn_angle_a")
    );
    assert_eq!(
        super::extract::series_component_name("voyager_saturn", 99),
        None
    );
}

#[test]
fn voyager_saturn_register_field_names_match_components() {
    let srcs = super::load_sources();
    let src = srcs
        .iter()
        .find(|s| s.format == "voyager_saturn")
        .expect("phi/sources.φ registers the voyager_saturn source");
    let names: Vec<&str> = src
        .extracts
        .iter()
        .filter_map(|e| match e {
            Extract::Field(fc) => Some(fc.name.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(
        names,
        vec![
            "voyager_saturn_doppler_count_hp",
            "voyager_saturn_doppler_count_lp",
            "voyager_saturn_angle_a",
            "voyager_saturn_angle_b",
        ]
    );
}

#[test]
fn mariner_occlt_series_dispatch_and_component_names() {
    let row = [
        1.0e9,
        38.0,
        4.0,
        20.0,
        25.0,
        637_641_753.0,
        4096.0,
        -14.0,
        14.0,
        -13.9,
        0.0,
        0.0,
    ];
    let bytes = super::mariner_occlt::write_mocc_bin(&[row]);
    let parsed = super::extract::series_parse_bin("mariner_occlt", &bytes)
        .expect("mariner_occlt series parses");
    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[0].0, 1.0e9);
    assert_eq!(parsed[0].1, -14.0);
    assert_eq!(parsed[0].2, super::mariner_occlt::COMP_AMP_MIN);
    assert_eq!(parsed[1].1, 14.0);
    assert_eq!(parsed[1].2, super::mariner_occlt::COMP_AMP_MAX);
    assert_eq!(parsed[2].1, -13.9);
    assert_eq!(parsed[2].2, super::mariner_occlt::COMP_AMP_MEAN);
    assert_eq!(
        super::extract::series_component_name("mariner_occlt", super::mariner_occlt::COMP_AMP_MIN),
        Some("mariner10_occlt_amp_min")
    );
    assert_eq!(
        super::extract::series_component_name("mariner_occlt", super::mariner_occlt::COMP_AMP_MAX),
        Some("mariner10_occlt_amp_max")
    );
    assert_eq!(
        super::extract::series_component_name("mariner_occlt", super::mariner_occlt::COMP_AMP_MEAN),
        Some("mariner10_occlt_amp_mean")
    );
    assert_eq!(
        super::extract::series_component_name("mariner_occlt", 99),
        None
    );
}

#[test]
fn drs_fits_series_dispatch_and_component_names() {
    let rows = [[1.0e-9, -2.0e-9, 3.0e-9]];
    let bytes = super::drs_fits::write_bin(&rows, 1.47e9);
    let parsed =
        super::extract::series_parse_bin("drs_fits", &bytes).expect("drs_fits series parses");
    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[0].0, 1.47e9);
    assert_eq!(parsed[0].1, 1.0e-9);
    assert_eq!(parsed[0].2, super::drs_fits::COMP_GX);
    assert_eq!(parsed[1].1, -2.0e-9);
    assert_eq!(parsed[1].2, super::drs_fits::COMP_GY);
    assert_eq!(parsed[2].1, 3.0e-9);
    assert_eq!(parsed[2].2, super::drs_fits::COMP_GZ);
    assert_eq!(
        super::extract::series_component_name("drs_fits", super::drs_fits::COMP_GX),
        Some("lpf_drs_dg_x_ms2")
    );
    assert_eq!(
        super::extract::series_component_name("drs_fits", super::drs_fits::COMP_GY),
        Some("lpf_drs_dg_y_ms2")
    );
    assert_eq!(
        super::extract::series_component_name("drs_fits", super::drs_fits::COMP_GZ),
        Some("lpf_drs_dg_z_ms2")
    );
    assert_eq!(super::extract::series_component_name("drs_fits", 99), None);
}

#[test]
fn demeter_isl_series_dispatch_and_component_names() {
    let mut blk = vec![0u8; super::demeter::BLOCK_BYTES];
    blk[26..34].copy_from_slice(b"TOULOUSE");
    blk[204..214].copy_from_slice(b"ISL SURVEY");
    blk[8] = 0x07;
    blk[9] = 0xd4;
    blk[10] = 0x00;
    blk[11] = 0x08;
    blk[12] = 0x00;
    blk[13] = 0x0b;
    blk[14] = 0x00;
    blk[15] = 0x0f;
    blk[16] = 0x00;
    blk[17] = 0x39;
    blk[18] = 0x00;
    blk[19] = 0x24;
    blk[22] = 0x02;
    blk[23] = 0x49;
    blk[265..289].copy_from_slice(&[
        0x47, 0x2a, 0xbc, 0xb1, 0x47, 0x0c, 0xee, 0x33, 0x45, 0x43, 0x5a, 0xe9, 0x3f, 0x82, 0x5a,
        0x97, 0xbd, 0xf5, 0xc2, 0x8e, 0xbd, 0xd9, 0x10, 0xc5,
    ]);
    let b = super::demeter::parse_block(&blk).unwrap();
    let mut bin = Vec::new();
    super::demeter::write_bin(&[b], &mut bin);
    let parsed =
        super::extract::series_parse_bin("demeter_isl", &bin).expect("demeter_isl series parses");
    let t = 1092239856.0;
    assert_eq!(parsed.len(), 6);
    assert_eq!(parsed[0], (t, 585.0, super::demeter::COMP_ORBIT));
    assert_eq!(parsed[1], (t, b.ne as f64, super::demeter::COMP_NE));
    assert_eq!(parsed[2], (t, b.ni as f64, super::demeter::COMP_NI));
    assert_eq!(parsed[3], (t, b.te as f64, super::demeter::COMP_TE));
    assert_eq!(parsed[4], (t, b.vf as f64, super::demeter::COMP_VF));
    assert_eq!(parsed[5], (t, b.vi0 as f64, super::demeter::COMP_VI0));
    assert_eq!(
        super::extract::series_component_name("demeter_isl", super::demeter::COMP_ORBIT),
        Some("demeter_isl_orbit_count")
    );
    assert_eq!(
        super::extract::series_component_name("demeter_isl", super::demeter::COMP_NE),
        Some("demeter_isl_ne_cm3")
    );
    assert_eq!(
        super::extract::series_component_name("demeter_isl", super::demeter::COMP_NI),
        Some("demeter_isl_ni_cm3")
    );
    assert_eq!(
        super::extract::series_component_name("demeter_isl", super::demeter::COMP_TE),
        Some("demeter_isl_te_k")
    );
    assert_eq!(
        super::extract::series_component_name("demeter_isl", super::demeter::COMP_VF),
        Some("demeter_isl_vf_v")
    );
    assert_eq!(
        super::extract::series_component_name("demeter_isl", super::demeter::COMP_VI0),
        Some("demeter_isl_vi0_ms")
    );
    assert_eq!(
        super::extract::series_component_name("demeter_isl", 99),
        None
    );
}

#[test]
fn drs_fits_register_field_names_match_components() {
    let srcs = super::load_sources();
    let src = srcs
        .iter()
        .find(|s| s.format == "drs_fits")
        .expect("phi/sources.φ registers the drs_fits source");
    let names: Vec<&str> = src
        .extracts
        .iter()
        .filter_map(|e| match e {
            Extract::Field(fc) => Some(fc.name.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(
        names,
        vec!["lpf_drs_dg_x_ms2", "lpf_drs_dg_y_ms2", "lpf_drs_dg_z_ms2"]
    );
}

#[test]
fn odf_series_dispatch_and_component_names() {
    let rows = [[
        753_440_003.0,
        -382_738.66,
        2.3e9,
        43.0,
        43.0,
        11.0,
        2.0,
        77.0,
        60.0,
    ]];
    let bytes = super::odf::write_podf_bin(&rows);
    let parsed = super::extract::series_parse_bin("mars_express_odf", &bytes)
        .expect("mars_express_odf series parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].0, 753_440_003.0);
    assert_eq!(parsed[0].1, -382_738.66);
    assert_eq!(parsed[0].2, super::odf::COMP_OBSERVABLE);
    assert_eq!(
        super::extract::series_component_name("mars_express_odf", super::odf::COMP_OBSERVABLE),
        Some("mars_express_odf_observable_hz")
    );
    assert_eq!(
        super::extract::series_component_name("juno_odf", super::odf::COMP_OBSERVABLE),
        Some("juno_odf_observable_hz")
    );
    assert_eq!(
        super::extract::series_component_name("juno_ocru_odf", super::odf::COMP_OBSERVABLE),
        Some("juno_ocru_odf_observable_hz")
    );
    assert_eq!(
        super::extract::series_component_name("dawn_odf", super::odf::COMP_OBSERVABLE),
        Some("dawn_odf_observable_hz")
    );
    assert_eq!(
        super::extract::series_component_name("pioneer10_odf", super::odf::COMP_OBSERVABLE),
        Some("pioneer10_odf_observable_hz")
    );
    assert_eq!(
        super::extract::series_component_name("mars_express_odf", 99),
        None
    );
    assert!(super::extract::series_parse_bin("mars_express_odf", b"X").is_none());
}

#[test]
fn rosetta_odf_series_dispatch_and_component_names() {
    let samples = [(1.5e9, -86.4, 0.068), (1.5e9 + 1.0, -87.2, 0.076)];
    let bytes = super::ifms_agc::write_series(&samples);
    let parsed =
        super::extract::series_parse_bin("rosetta_odf", &bytes).expect("rosetta_odf series parses");
    assert_eq!(parsed.len(), 4);
    assert_eq!(parsed[0].0, 1.5e9);
    assert_eq!(parsed[0].1, -86.4);
    assert_eq!(parsed[0].2, super::ifms_agc::COMP_CARRIER_LEVEL);
    assert_eq!(parsed[1].1, 0.068);
    assert_eq!(parsed[1].2, super::ifms_agc::COMP_POLAR_ANGLE);
    assert_eq!(parsed[2].1, -87.2);
    assert_eq!(parsed[2].2, super::ifms_agc::COMP_CARRIER_LEVEL);
    assert_eq!(parsed[3].1, 0.076);
    assert_eq!(parsed[3].2, super::ifms_agc::COMP_POLAR_ANGLE);
    assert_eq!(
        super::extract::series_component_name("rosetta_odf", super::ifms_agc::COMP_CARRIER_LEVEL),
        Some("rosetta_odf_carrier_level_dbm")
    );
    assert_eq!(
        super::extract::series_component_name("rosetta_odf", super::ifms_agc::COMP_POLAR_ANGLE),
        Some("rosetta_odf_polar_angle_cycles")
    );
    assert_eq!(
        super::extract::series_component_name("rosetta_odf", 99),
        None
    );
}

#[test]
fn odf_register_field_names_match_components() {
    let srcs = super::load_sources();
    for format in [
        "juno_odf",
        "juno_ocru_odf",
        "magellan_odf",
        "mgs_odf",
        "mro_odf",
        "odyssey_odf",
        "messenger_odf",
        "mars_express_odf",
        "vex_odf",
        "galileo_odf",
        "dawn_odf",
    ] {
        let src = match srcs.iter().find(|s| s.format == format) {
            Some(s) => s,
            None => panic!("phi/sources.φ registers the {format} source"),
        };
        let names: Vec<&str> = src
            .extracts
            .iter()
            .filter_map(|e| match e {
                Extract::Field(fc) => Some(fc.name.as_str()),
                _ => None,
            })
            .collect();
        let expected = format!("{format}_observable_hz");
        assert_eq!(names, vec![expected.as_str()], "{format} field name drift");
    }
}

#[test]
fn odr_series_dispatch_and_component_names() {
    let mut raw = vec![0u8; super::voyager_odr::RECORD_BYTES];
    raw[..super::voyager_odr::HEADER_BYTES].copy_from_slice(&[
        0x90, 0x0D, 0x00, 0x01, 0x09, 0xE0, 0x20, 0x2B, 0x00, 0x1E, 0x23, 0x80, 0x40, 0x45, 0x9B,
        0x71, 0x54, 0x25, 0x00, 0x60, 0x00, 0xA2, 0x72, 0xFE, 0xDB, 0x08, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8A,
        0x00, 0xD0, 0x05, 0x00, 0xA2, 0x72, 0x1F, 0xFF, 0xFB, 0x6C, 0x4C,
    ]);
    for (i, b) in raw[super::voyager_odr::HEADER_BYTES..]
        .iter_mut()
        .enumerate()
    {
        *b = i as u8;
    }
    let bin = super::voyager_odr::pack(&raw, "C0XR13AA.ODR", 1981);
    let parsed =
        super::extract::series_parse_bin("voyager_odr", &bin).expect("voyager_odr series parses");
    assert_eq!(parsed.len(), super::voyager_odr::DATA_SAMPLES);
    assert_eq!(parsed[0].2, super::voyager_odr::COMP_SAMPLE);
    assert_eq!(
        super::extract::series_component_name("voyager_odr", super::voyager_odr::COMP_SAMPLE),
        Some("voyager_odr_sample_count")
    );
    assert_eq!(
        super::extract::series_component_name("galileo_odr", super::galileo_odr::COMP_AD1),
        Some("galileo_odr_ad1_count")
    );
    assert_eq!(
        super::extract::series_component_name("galileo_odr", super::galileo_odr::COMP_AD2),
        Some("galileo_odr_ad2_count")
    );
    assert_eq!(
        super::extract::series_component_name("galileo_odr", super::galileo_odr::COMP_AD3),
        Some("galileo_odr_ad3_count")
    );
    assert_eq!(
        super::extract::series_component_name("galileo_odr", super::galileo_odr::COMP_AD4),
        Some("galileo_odr_ad4_count")
    );
    assert_eq!(
        super::extract::series_component_name("voyager_odr", 99),
        None
    );
    assert_eq!(
        super::extract::series_component_name("galileo_odr", 99),
        None
    );
    assert!(super::extract::series_parse_bin("galileo_odr", b"X").is_none());
}

#[test]
fn odr_register_field_names_match_components() {
    let srcs = super::load_sources();
    let expected: &[(&str, &[&str])] = &[
        ("voyager_odr", &["voyager_odr_sample_count"]),
        (
            "galileo_odr",
            &[
                "galileo_odr_ad1_count",
                "galileo_odr_ad2_count",
                "galileo_odr_ad3_count",
                "galileo_odr_ad4_count",
            ],
        ),
    ];
    for (format, names) in expected {
        let src = match srcs.iter().find(|s| s.format == *format) {
            Some(s) => s,
            None => panic!("phi/sources.φ registers the {format} source"),
        };
        let field_names: Vec<&str> = src
            .extracts
            .iter()
            .filter_map(|e| match e {
                Extract::Field(fc) => Some(fc.name.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(field_names, *names, "{format} field name drift");
    }
}

#[test]
fn gdp_drifter_geo_series_roundtrip_and_component_name() {
    let rec = crate::gdp_drifter::DrifterRecord {
        id: 12345,
        time: 729_777_632.227_242_1,
        lon: -45.5,
        lat: 30.25,
        sst: Some(300.15),
    };
    let bytes =
        crate::gdp_drifter::write_bin(std::slice::from_ref(&rec)).expect("gdp_drifter bin writes");
    let parsed = super::extract::geo_series_parse_bin("gdp_drifter", &bytes)
        .expect("gdp_drifter bin parses");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].lat, 30.25);
    assert_eq!(parsed[0].lon, -45.5);
    assert!((parsed[0].val - 300.15).abs() < 1e-3);
    assert_eq!(parsed[0].comp, crate::gdp_drifter::COMP_SST);
    assert_eq!(
        crate::geo::magic_of("gdp_drifter"),
        Some(crate::geo::MAGIC_GDP)
    );
    assert_eq!(
        crate::geo::comp_max("gdp_drifter"),
        Some(crate::gdp_drifter::COMP_SST)
    );
    assert_eq!(
        super::extract::geo_series_component_name("gdp_drifter", crate::gdp_drifter::COMP_SST),
        Some("gdp_drifter_sst_k")
    );
    assert_eq!(
        super::extract::geo_series_component_name("gdp_drifter", 99),
        None
    );
}

#[test]
fn iscb_bin_roundtrip_and_rejections() {
    fn encode(events: &[EventTuple]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"ISCB");
        out.push(1u8);
        out.extend_from_slice(&(events.len() as u64).to_le_bytes());
        for &(time, lat, lon, depth, mag, mag_type) in events {
            let mut rec = [0u8; 56];
            if mag.is_some() {
                rec[0] |= 0x01;
            }
            if mag_type.is_some() {
                rec[0] |= 0x02;
            }
            rec[8..16].copy_from_slice(&time.to_le_bytes());
            rec[16..24].copy_from_slice(&lat.to_le_bytes());
            rec[24..32].copy_from_slice(&lon.to_le_bytes());
            rec[32..40].copy_from_slice(&depth.to_le_bytes());
            let mag_bits: [u8; 8] = match mag {
                Some(m) => m.to_le_bytes(),
                None => [0u8; 8],
            };
            rec[40..48].copy_from_slice(&mag_bits);
            if let Some(ty) = mag_type {
                let b = ty.as_bytes();
                let n = b.len().min(8);
                rec[48..48 + n].copy_from_slice(&b[..n]);
            }
            out.extend_from_slice(&rec);
        }
        out
    }
    let bytes = encode(&[
        (
            1_704_092_765.77,
            37.4747,
            137.3070,
            9.704,
            Some(6.0),
            Some("mb"),
        ),
        (1_704_092_765.0, 37.4929, 137.2624, 10.7426, None, None),
    ]);
    let parsed = super::extract::parse_iscb_bin(&bytes).expect("iscb parses");
    assert_eq!(parsed.len(), 2);
    assert!((parsed[0].lat - 37.4747).abs() < 1e-9);
    assert_eq!(parsed[0].magnitude, Some(6.0));
    assert_eq!(parsed[0].mag_type.as_deref(), Some("mb"));
    assert_eq!(parsed[1].magnitude, None);
    assert_eq!(parsed[1].mag_type, None);
    assert!(super::extract::parse_iscb_bin(b"X").is_none());
    assert!(super::extract::parse_iscb_bin(b"ISCB").is_none());
    assert!(super::extract::parse_iscb_bin(&bytes[..bytes.len() - 1]).is_none());
}

#[test]
fn nexrad_level2_roundtrip_and_component_name() {
    fn encode(
        recs: &[(f64, f64, f64, f64, f64, u32)],
        site: Option<([u8; 4], f64, f64, f64)>,
    ) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"NXR1");
        out.extend_from_slice(&(recs.len() as u32).to_le_bytes());
        match site {
            Some((stid, lat, lon, alt)) => {
                out.push(1u8);
                out.extend_from_slice(&stid);
                out.extend_from_slice(&lat.to_le_bytes());
                out.extend_from_slice(&lon.to_le_bytes());
                out.extend_from_slice(&alt.to_le_bytes());
            }
            None => out.extend_from_slice(&[0u8; 29]),
        }
        for &(t, az, el, range, value, kind) in recs {
            out.extend_from_slice(&t.to_le_bytes());
            out.extend_from_slice(&az.to_le_bytes());
            out.extend_from_slice(&el.to_le_bytes());
            out.extend_from_slice(&range.to_le_bytes());
            out.extend_from_slice(&value.to_le_bytes());
            out.extend_from_slice(&kind.to_le_bytes());
        }
        out
    }
    let bytes = encode(
        &[
            (
                1_704_067_204.932,
                90.0,
                0.5,
                2.125,
                -32.0,
                crate::geo::COMP_NXR_REF,
            ),
            (
                1_704_067_204.932,
                90.5,
                0.5,
                2.375,
                10.5,
                crate::geo::COMP_NXR_VEL,
            ),
        ],
        Some((*b"KTLX", 35.33306, -97.2775, 1213.0 * 0.3048)),
    );
    let parsed = super::extract::parse_nexrad_level2_bin(&bytes).expect("nexrad level2 bin parses");
    assert_eq!(parsed.samples.len(), 2);
    let site = parsed.site.expect("the site anchor roundtrips");
    assert_eq!(site.stid, *b"KTLX");
    assert!((site.lat_deg - 35.33306).abs() < 1e-9);
    assert!((site.lon_deg - -97.2775).abs() < 1e-9);
    assert!((site.alt_m - 1213.0 * 0.3048).abs() < 1e-9);
    assert_eq!(parsed.samples[0].value, -32.0);
    assert_eq!(parsed.samples[0].kind, crate::geo::COMP_NXR_REF);
    assert_eq!(parsed.samples[1].range_km, 2.375);
    assert_eq!(parsed.samples[1].az_deg, 90.5);
    assert_eq!(
        super::extract::nexrad_component_name(crate::geo::COMP_NXR_REF),
        Some("nexrad_level2_ref_dbz")
    );
    assert_eq!(
        super::extract::nexrad_component_name(crate::geo::COMP_NXR_VEL),
        Some("nexrad_level2_vel_ms")
    );
    assert_eq!(
        super::extract::nexrad_component_name(crate::geo::COMP_NXR_SW),
        Some("nexrad_level2_sw_ms")
    );
    assert_eq!(super::extract::nexrad_component_name(9), None);
    assert_eq!(
        crate::geo::magic_of("nexrad_level2"),
        Some(crate::geo::MAGIC_NXR)
    );
    assert_eq!(
        crate::geo::comp_max("nexrad_level2"),
        Some(crate::geo::COMP_NXR_MAX)
    );
    assert!(super::extract::parse_nexrad_level2_bin(b"X").is_none());
    assert!(super::extract::parse_nexrad_level2_bin(b"NXR1abc").is_none());
    assert!(super::extract::parse_nexrad_level2_bin(&bytes[..bytes.len() - 1]).is_none());

    let no_site = encode(
        &[(
            1_704_067_204.932,
            90.0,
            0.5,
            2.125,
            -32.0,
            crate::geo::COMP_NXR_REF,
        )],
        None,
    );
    let parsed_no_site =
        super::extract::parse_nexrad_level2_bin(&no_site).expect("absent-site bin parses");
    assert!(parsed_no_site.site.is_none());
    assert_eq!(parsed_no_site.samples.len(), 1);
}

const EPA_AQS_HEADER: &str = "\"State Code\",\"County Code\",\"Site Num\",\"Parameter Code\",\"POC\",\"Latitude\",\"Longitude\",\"Datum\",\"Parameter Name\",\"Sample Duration\",\"Pollutant Standard\",\"Date Local\",\"Units of Measure\",\"Event Type\",\"Observation Count\",\"Observation Percent\",\"Arithmetic Mean\"\n";

fn epa_aqs_block() -> &'static str {
    "url https://aqs.epa.gov/aqsweb/airdata/daily_88101_2025.zip
ttl 86400
format csv_zip
on earth 0 0 0
rows
epoch 11
lat 5
lon 6
field 16 pm25_daily_ug_m3 gaussian-inverse-square diffusion ug/m3 86400.0 0.0 0.0
"
}

fn epa_aqs_rows() -> String {
    let mut body = String::from(EPA_AQS_HEADER);
    body.push_str("\"01\",\"003\",\"0010\",\"88101\",3,30.497478,-87.880258,\"NAD83\",\"PM2.5\",\"1 HOUR\",\"\",\"2025-01-01\",\"ug/m3\",\"None\",24,100.0,3.625\n");
    body.push_str("\"01\",\"003\",\"0010\",\"88101\",3,30.497478,-87.880258,\"NAD83\",\"PM2.5\",\"1 HOUR\",\"\",\"2025-01-02\",\"ug/m3\",\"None\",24,100.0,6.791667\n");
    body
}

fn stored_zip(csv: &str) -> Vec<u8> {
    let mut zip = Vec::new();
    zip.extend_from_slice(b"PK\x03\x04");
    zip.extend_from_slice(&20u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u32.to_le_bytes());
    zip.extend_from_slice(&(csv.len() as u32).to_le_bytes());
    zip.extend_from_slice(&(csv.len() as u32).to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(&0u16.to_le_bytes());
    zip.extend_from_slice(csv.as_bytes());
    zip
}

#[test]
fn test_parse_rows_lat_lon_epoch_directives() {
    let srcs = super::parse_sources(epa_aqs_block());
    assert_eq!(srcs.len(), 1);
    match &srcs[0].extracts[0] {
        Extract::Rows {
            lat_key,
            lon_key,
            epoch_cols,
            ..
        } => {
            assert_eq!(lat_key, "5");
            assert_eq!(lon_key, "6");
            assert_eq!(epoch_cols, &vec!["11".to_string()]);
        }
        _ => panic!("expected Rows extract"),
    }
}

#[test]
fn test_parse_iso_tdb_date_only_is_midnight() {
    let lsk = fixture_lsk();
    assert_eq!(
        super::parse_iso_tdb("2025-01-01", &lsk),
        super::parse_iso_tdb("2025-01-01T00:00:00Z", &lsk)
    );
}

#[test]
fn test_rows_per_row_lat_lon_and_date_epoch() {
    let block = "url https://example.com/aqs.csv
ttl 86400
format csv
on earth 0 0 0
rows
epoch 11
lat 5
lon 6
field 16 pm25_daily_ug_m3 gaussian-inverse-square diffusion ug/m3 86400.0 0.0 0.0
";
    let srcs = super::parse_sources(block);
    assert_eq!(srcs.len(), 1);
    let body = epa_aqs_rows();
    let lsk = fixture_lsk();
    match extract(&srcs[0], &body, 8.0e8, &lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 2);
            for (c, fc) in &channels {
                assert_eq!(fc.force, 6);
                assert_eq!(fc.tau, 86400.0);
                match &c.position {
                    super::Position::Surface { lat, lon, alt, .. } => {
                        assert!((lat - 30.497478).abs() < 1e-9);
                        assert!((lon + 87.880258).abs() < 1e-9);
                        assert_eq!(*alt, 0.0);
                    }
                    _ => panic!("expected per-row Surface position"),
                }
            }
            assert!((channels[0].0.value - 3.625).abs() < 1e-12);
            assert!((channels[1].0.value - 6.791667).abs() < 1e-12);
            let e0 = super::parse_iso_tdb("2025-01-01", &lsk).unwrap();
            let e1 = super::parse_iso_tdb("2025-01-02", &lsk).unwrap();
            assert!((channels[0].0.epoch - e0).abs() < 1e-6);
            assert!((channels[1].0.epoch - e1).abs() < 1e-6);
        }
        _ => panic!("expected Measurements"),
    }
}

#[test]
fn test_rows_absent_lat_skips_row() {
    let block = "url https://example.com/aqs.csv
ttl 86400
format csv
on earth 0 0 0
rows
epoch 11
lat 5
lon 6
field 16 pm25_daily_ug_m3 gaussian-inverse-square diffusion ug/m3 86400.0 0.0 0.0
";
    let srcs = super::parse_sources(block);
    let mut body = String::from(EPA_AQS_HEADER);
    body.push_str("\"01\",\"003\",\"0010\",\"88101\",3,,-87.880258,\"NAD83\",\"PM2.5\",\"1 HOUR\",\"\",\"2025-01-01\",\"ug/m3\",\"None\",24,100.0,3.625\n");
    let lsk = fixture_lsk();
    match extract(&srcs[0], &body, 8.0e8, &lsk) {
        ExtractResult::Measurements(channels) => assert!(channels.is_empty()),
        _ => panic!("expected Measurements"),
    }
}

#[test]
fn test_rows_epoch_iso_datetime_column() {
    let block = "url https://example.com/d20.csv
ttl 3600
format text
on earth 0.0 -120.0 0
rows .
epoch 0
field 4 d20_thermocline_temp_c erfc thermal C 86400.0 0.0 0.0
";
    let srcs = super::parse_sources(block);
    assert_eq!(srcs.len(), 1);
    let body = "time,station,lon,lat,iso_6\n2026-05-03T12:00:00Z,0n110w,250,0,99.21654\n";
    let lsk = fixture_lsk();
    match extract(&srcs[0], body, 8.0e8, &lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 1);
            assert!((channels[0].0.value - 99.21654).abs() < 1e-12);
            let expected = super::parse_iso_tdb("2026-05-03T12:00:00Z", &lsk).unwrap();
            assert!((channels[0].0.epoch - expected).abs() < 1e-6);
        }
        _ => panic!("expected Measurements"),
    }
}

#[test]
fn test_extract_csv_zip_rows_per_row_position() {
    let csv = epa_aqs_rows();
    let zip = stored_zip(&csv);
    let path = std::env::temp_dir().join("omegaflow_test_aqs_rows.zip");
    std::fs::write(&path, &zip).unwrap();
    let srcs = super::parse_sources(epa_aqs_block());
    let lsk = fixture_lsk();
    let path_str = path.to_string_lossy().to_string();
    let result = extract(&srcs[0], &path_str, 8.0e8, &lsk);
    let _ = std::fs::remove_file(&path);
    match result {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 2);
            assert!((channels[0].0.value - 3.625).abs() < 1e-12);
            match &channels[0].0.position {
                super::Position::Surface { lat, lon, .. } => {
                    assert!((lat - 30.497478).abs() < 1e-9);
                    assert!((lon + 87.880258).abs() < 1e-9);
                }
                _ => panic!("expected per-row Surface position"),
            }
        }
        _ => panic!("expected Measurements"),
    }
}

#[test]
fn test_epa_aqs_source_registered() {
    let content = std::fs::read_to_string("phi/sources.φ").unwrap();
    let sources = super::parse_sources(&content);
    let src = sources
        .iter()
        .find(|s| {
            s.url
                .contains("aqs.epa.gov/aqsweb/airdata/daily_88101_2025.zip")
        })
        .expect("the EPA AQS daily PM2.5 source is registered");
    assert_eq!(src.format, "csv_zip");
    assert_eq!(src.ttl, 86400);
    match &src.extracts[0] {
        Extract::Rows {
            lat_key,
            lon_key,
            epoch_cols,
            fields,
            ..
        } => {
            assert_eq!(lat_key, "5");
            assert_eq!(lon_key, "6");
            assert_eq!(epoch_cols, &vec!["11".to_string()]);
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].key, "16");
            assert_eq!(fields[0].force, 6);
            assert_eq!(fields[0].tau, 86400.0);
        }
        _ => panic!("expected Rows extract"),
    }
}

#[test]
fn test_parse_rows_verbatim_header_key() {
    let block = "url https://example.com/aqs.csv
ttl 86400
format csv
on earth 0 0 0
rows
epoch 11
lat 5
lon 6
field \"Arithmetic Mean\" pm25_daily_ug_m3 gaussian-inverse-square diffusion µg/m3 86400.0 0.0 0.0
";
    let srcs = super::parse_sources(block);
    assert_eq!(srcs.len(), 1);
    match &srcs[0].extracts[0] {
        Extract::Rows { fields, .. } => {
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].key, "Arithmetic Mean");
            assert_eq!(fields[0].force, 6);
            assert_eq!(fields[0].tau, 86400.0);
        }
        _ => panic!("expected Rows extract"),
    }
}

#[test]
fn test_rows_verbatim_header_key_extracts() {
    let block = "url https://example.com/aqs.csv
ttl 86400
format csv
on earth 0 0 0
rows
epoch 11
lat 5
lon 6
field \"Arithmetic Mean\" pm25_daily_ug_m3 gaussian-inverse-square diffusion µg/m3 86400.0 0.0 0.0
";
    let srcs = super::parse_sources(block);
    let body = epa_aqs_rows();
    let lsk = fixture_lsk();
    match extract(&srcs[0], &body, 8.0e8, &lsk) {
        ExtractResult::Measurements(channels) => {
            assert_eq!(channels.len(), 2);
            assert!((channels[0].0.value - 3.625).abs() < 1e-12);
            assert!((channels[1].0.value - 6.791667).abs() < 1e-12);
            match &channels[0].0.position {
                super::Position::Surface { lat, lon, .. } => {
                    assert!((lat - 30.497478).abs() < 1e-9);
                    assert!((lon + 87.880258).abs() < 1e-9);
                }
                _ => panic!("expected per-row Surface position"),
            }
        }
        _ => panic!("expected Measurements"),
    }
}

#[test]
fn test_sample_phase_writes_the_producer_law_for_phase_counters() {
    let mut sensor = field_fixture("cassini_tnf_ul_phase_cycles", 604800.0);
    sensor.key = "ul_phase_cycles".into();
    sensor.unit = "cycle".into();
    let mut channel = Channel {
        z: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        epoch: 1.0e9,
        position: Position::Source,
        name: "cassini_tnf_ul_phase_cycles".into(),
        value: 42.75,
    };
    let phase = sample_phase(&channel, &sensor).expect("a phase counter carries a phase");
    assert!((phase - 0.75 * 2.0 * std::f64::consts::PI).abs() < 1e-12);
    channel.value = f64::NAN;
    assert!(sample_phase(&channel, &sensor).is_none());
    sensor.key = "polar_angle_cycles".into();
    assert!(
        sample_phase(&channel, &sensor).is_none(),
        "the polar angle is an angle, not a carrier phase"
    );
}

#[test]
fn test_sample_phase_matches_the_producer_on_a_podf_row() {
    let row: [f64; 9] = [
        1.0e9,
        42.75,
        7.183_446_125e9,
        26.0,
        98.0,
        0.0,
        2.0,
        7.0,
        0.0,
    ];
    let mut bytes = Vec::with_capacity(8 + 72);
    bytes.extend_from_slice(b"PODF");
    bytes.extend_from_slice(&1u32.to_le_bytes());
    for v in row {
        bytes.extend_from_slice(&v.to_le_bytes());
    }
    let series = super::odf::tnf_phase_series(&bytes).expect("podf parses");
    let mut sensor = field_fixture("cassini_tnf_ul_phase_cycles", 604800.0);
    sensor.key = "ul_phase_cycles".into();
    let channel = Channel {
        z: 0.0,
        freq: 0.0,
        bin_width: 0.0,
        epoch: 1.0e9,
        position: Position::Source,
        name: "cassini_tnf_ul_phase_cycles".into(),
        value: row[super::odf::PODF_COL_OBSERVABLE],
    };
    let arm_phase = sample_phase(&channel, &sensor);
    assert_eq!(arm_phase, series[0].phase, "arm and producer share the law");
    assert_eq!(series[0].freq, row[super::odf::TNF_ROW_SUPPORT]);
    assert_eq!(series[0].bin_width, 0.0);
}

#[test]
fn test_series_rows_carries_row_freq_value_and_band_for_tnf() {
    let row: [f64; 9] = [
        1.0e9,
        42.75,
        7.183_446_125e9,
        26.0,
        98.0,
        0.0,
        2.0,
        7.0,
        0.0,
    ];
    let mut bytes = Vec::with_capacity(8 + 72);
    bytes.extend_from_slice(b"PODF");
    bytes.extend_from_slice(&1u32.to_le_bytes());
    for v in row {
        bytes.extend_from_slice(&v.to_le_bytes());
    }
    let rows = super::extract::series_rows("maven_tnf", &bytes).expect("tnf rows parse");
    assert_eq!(rows.len(), 1);
    let r = rows[0];
    assert_eq!(r.t, 1.0e9);
    assert_eq!(
        r.value, 42.75,
        "the raw cycles ride, the arm derives the phase"
    );
    assert_eq!(r.comp, super::odf::TNF_COMP_UL_PHASE);
    assert_eq!(
        r.freq,
        row[super::odf::TNF_ROW_SUPPORT],
        "the band rides the row"
    );
    assert_eq!(r.bin_width, 0.0, "a phase counter carries no band");
}

#[test]
fn test_series_rows_keeps_the_no_band_pad_for_a_bandless_series() {
    let mut row = [0.0f64; 14];
    row[0] = 729_777_632.0;
    row[1] = 2292.0e6 + 1.5e6;
    let bytes = crate::atdf::write_bin(&[row]);
    let rows = super::extract::series_rows("atdf", &bytes).expect("atdf series parses");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].t, row[0]);
    assert_eq!(rows[0].value, row[1]);
    assert_eq!(rows[0].comp, crate::atdf::COMP_SKYFREQ);
    assert_eq!(
        rows[0].freq, 0.0,
        "a bandless series carries the (0, ·) pad"
    );
    assert_eq!(rows[0].bin_width, 0.0);
}
