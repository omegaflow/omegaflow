use super::*;

static ANOMALY_TEST_GATE: std::sync::Mutex<()> = std::sync::Mutex::new(());

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
        hapi_fill: HashMap::new(),
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
        _ => panic!("expected measurements"),
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
            assert!(channels.is_empty(), "no matching row → fehlt, never 0.0")
        }
        _ => panic!("expected measurements"),
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
        _ => panic!("expected measurements"),
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
        _ => panic!("expected measurements"),
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
        _ => panic!("expected filtered last extract"),
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
        let a = a.unwrap_or_else(|| panic!("conversion returned None"));
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
        _ => panic!("expected measurements"),
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
        _ => panic!("expected measurements"),
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
        _ => panic!("expected measurements"),
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
fn test_parse_json_skips_jina_header() {
    let s = "Title: \n\n\nURL Source: http://api.wheretheiss.at/v1/satellites/25544\n\n\nMarkdown Content:\n{\"name\":\"iss\",\"id\":25544,\"latitude\":-39.79}";
    let v = parse_json(s).unwrap();
    let obj = match v {
        super::JsonVal::Obj(m) => m,
        other => panic!("root is {:?}", other),
    };
    assert!(matches!(obj.get("name"), Some(super::JsonVal::Str(s)) if s == "iss"));
    assert!(matches!(obj.get("id"), Some(super::JsonVal::Num(n)) if (n - 25544.0).abs() < 1e-9));
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
        hapi_fill: HashMap::new(),
    };
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    let url = render_source_url(
        &src,
        0.0,
        0.0,
        0.0,
        8.0e8,
        1000.0,
        &HashMap::new(),
        &HashMap::new(),
        &fixture_lsk,
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
        0.0,
        0.0,
        0.0,
        past_tdb,
        1000.0,
        &HashMap::new(),
        &HashMap::new(),
        &fixture_lsk,
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
        0.0,
        0.0,
        0.0,
        now_tdb,
        1000.0,
        &HashMap::new(),
        &HashMap::new(),
        &fixture_lsk,
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
        0.0,
        0.0,
        0.0,
        pre_2000_tdb,
        0.0,
        &HashMap::new(),
        &HashMap::new(),
        &lsk,
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
        0.0,
        0.0,
        0.0,
        8.0e8,
        0.0,
        &HashMap::new(),
        &HashMap::new(),
        &fixture_lsk,
    )
    .unwrap();
    let harvest_2026 = render_source_url(
        &src,
        0.0,
        0.0,
        0.0,
        1.8e9,
        0.0,
        &HashMap::new(),
        &HashMap::new(),
        &fixture_lsk,
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
        _ => panic!("expected measurements"),
    }
}

#[test]
fn test_epoch_stamp_gate_is_observer_time() {
    let ttl = 3600u64;
    let path = "/tmp/opencode/omegaflow_epoch_stamp_gate_test.json";
    let stamp_path = format!("{}.epoch", path);
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
        hapi_fill: HashMap::new(),
    };
    let fixture_lsk = super::LeapSeconds {
        delta_t_a: 32.184,
        deltas: vec![(37.0, 1483228800.0)],
    };
    let body = render_source_body(
        &src,
        0.0,
        0.0,
        0.0,
        8.0e8,
        100000.0,
        &HashMap::new(),
        &fixture_lsk,
    );
    assert!(body.is_some());
    let b = body.unwrap();
    assert!(b.contains("bbox"));
    assert!(b.contains("{lon_min}"));
    assert!(!b.contains("{today}"));
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
            dist_scale: 1.0,
            plx_key: String::new(),
            z_key: "redshift".into(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: 1.0,
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
        hapi_fill: HashMap::new(),
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
            dist_scale: 1.0,
            plx_key: String::new(),
            z_key: "redshift".into(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: 1.0,
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
        hapi_fill: HashMap::new(),
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
            assert!((dist_scale - 3.085677581e19).abs() < 1e6);
        }
        _ => panic!("expected CelestialMap extract"),
    }
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
            dist_scale: 3.085677581e19,
            plx_key: String::new(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: 1.0,
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
        hapi_fill: HashMap::new(),
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
            dist_scale: 1.0,
            plx_key: "plx".into(),
            z_key: String::new(),
            pmra_key: "pmra".into(),
            pmdec_key: "pmdec".into(),
            rv_key: "rv".into(),
            rv_scale: 1.0,
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
        hapi_fill: HashMap::new(),
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
    assert!(anomalies
        .iter()
        .any(|a| a.category == "Empty Data" && a.url == "https://example.org/e"));
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
            dist_scale: 1.0,
            plx_key: String::new(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: 1.0,
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
        hapi_fill: HashMap::new(),
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
            dist_scale: 3.085677581e16,
            plx_key: String::new(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: 1.0,
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
        hapi_fill: HashMap::new(),
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
            dist_scale: 3.085677581e22,
            plx_key: String::new(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: 1.0,
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
        hapi_fill: HashMap::new(),
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
    let buf = build_buffer(samples, 1.0, Arc::new(eph.clone()), None, Vec::new());
    let query = |floor: [f64; 9], forward: [f64; 3]| {
        let mut out: Vec<SampleRecord> = Vec::new();
        query_hash(
            &buf.cache,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
            0.0,
            &floor,
            1.0,
            forward,
            &mut out,
            &eph,
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
    assert!(Motion::Kepler {
        rec: Arc::new(unbound)
    }
    .at(0.0, 0.0, &eph)
    .is_none());
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
    assert!(gm.extent == 0.0 && gm.tau.is_infinite());
    assert!(radius.extent == 0.0 && radius.tau.is_infinite());
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
    let buf = build_buffer(samples, 1.0, Arc::new(eph.clone()), None, Vec::new());
    let mut records: Vec<SampleRecord> = Vec::new();
    query_hash(
        &buf.cache,
        anchor_p0,
        0.0,
        1.0,
        0.0,
        &[0.0; 9],
        1.0,
        [1.0, 0.0, 0.0],
        &mut records,
        &eph,
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
        hapi_fill: HashMap::new(),
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
        hapi_fill: HashMap::new(),
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
    let content = std::fs::read_to_string("phi/pipeline/queue/master.φ").unwrap();
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
    let (pv, fields, lk, ok) = prof.expect("ProfileMap extract missing");
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
    let (pv, ps, fields, lk, ok, ek) = prof.expect("ProfileMap extract missing");
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
    let slots = vec![
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
    let channels = super::build_netcdf_channels(&srcs[0], &b, &lsk);
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
fn test_port_convert_celestial_and_post() {
    let celestial = "source oac\nttl 86400\nforce em\nurl https://api.example.org/{target}/\nverify false\ntarget SN2014J\nmap .\nlat_key ra\nlon_key dec\nfield name name\n";
    let conv = super::port_block(celestial);
    assert!(conv.contains("ttl 86400\n"));
    assert!(conv.contains("at sun\n"));
    assert!(conv.contains("url https://api.example.org/{target}/\n"));
    let srcs = super::parse_sources(&conv);
    for s in &srcs {
        for e in &s.extracts {
            match e {
                super::Extract::CelestialMap { fields, .. }
                | super::Extract::Map { fields, .. } => assert!(fields.is_empty()),
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
            if t.starts_with("url ") {
                seen.insert(t[4..].trim().to_string());
            }
        }
        ok_text = existing;
    }
    let mut void_text = String::new();
    if let Ok(existing) = std::fs::read_to_string("phi/pipeline/stage/staging_void_ledger.txt") {
        for l in existing.lines() {
            if let Some(u) = l.strip_prefix("void ") {
                if let Some(end) = u.find(' ') {
                    seen.insert(u[..end].to_string());
                }
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
                    let post_body = match &s.post_body {
                        Some(pb) => Some(substitute_test_templates(pb)),
                        None => None,
                    };
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
        hapi_fill: HashMap::new(),
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
    }
}

#[test]
fn test_rotation_matrix_roundtrip() {
    use std::collections::HashMap;
    let tdb = 3.0 * 86400.0;
    let jd = super::J2000_EPOCH + 3.0;
    let tc = (jd - super::J2000_EPOCH) / 36525.0;
    let a = (317.68143f64 - 0.1061 * tc).to_radians();
    let d = (52.88650f64 - 0.0609 * tc).to_radians();
    let w = (176.630f64 + 350.89198226 * (jd - super::J2000_EPOCH) - (317.68143f64 - 0.1061 * tc))
        .to_radians();
    let (sa, ca) = a.sin_cos();
    let (sd, cd) = d.sin_cos();
    let (sw, cw) = w.sin_cos();
    let m: [f64; 9] = [
        cd * ca * cw + sa * sw,
        cd * ca * sw - sa * cw,
        -sd * ca,
        cd * sa * cw - ca * sw,
        cd * sa * sw + ca * cw,
        -sd * sa,
        sd * cw,
        sd * sw,
        cd,
    ];
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
    let a = (317.68143f64 - 0.1061 * tc).to_radians();
    let d = (52.88650f64 - 0.0609 * tc).to_radians();
    let w = (176.630f64 + 350.89198226 * (jd - super::J2000_EPOCH) - (317.68143f64 - 0.1061 * tc))
        .to_radians();
    let (sa, ca) = a.sin_cos();
    let (sd, cd) = d.sin_cos();
    let (sw, cw) = w.sin_cos();
    let m: [f64; 9] = [
        cd * ca * cw + sa * sw,
        cd * ca * sw - sa * cw,
        -sd * ca,
        cd * sa * cw - ca * sw,
        cd * sa * sw + ca * cw,
        -sd * sa,
        sd * cw,
        sd * sw,
        cd,
    ];
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
        hapi_fill: HashMap::new(),
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
            sample
                .motion
                .anchor_body()
                .unwrap_or_else(|| "absent".into())
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
        hapi_fill: HashMap::new(),
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
        _ => panic!("expected filtered last extract"),
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
    };
    let cache = super::build_spatial_hash(vec![sample], 1.0);
    let eph: HashMap<String, super::BodyEphemeris> = HashMap::new();
    let buf = super::Buffer {
        cache,
        eph: Arc::new(eph),
        curves: None,
        spectral: Vec::new(),
    };
    let mut records: Vec<super::SampleRecord> = Vec::new();
    super::sense_membrane(
        &buf,
        [0.0, 0.0, 0.0],
        t + 1.0,
        3.0e12,
        1.0,
        &[0.0; 9],
        2.0e9,
        [0.0, 0.0, 1.0],
        &mut records,
        &HashMap::new(),
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
        3.9860043543609598e14,
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
    assert_eq!(props.gm, Some(3.9860043543609598e14));
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
        3.9860043543609598e14,
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
    assert_eq!(props.gm, Some(3.9860043543609598e14));
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
        fold: None,
    };
    let reach = super::dispatch_reach(&[fc], 60.0).expect("em carries a propagation law");
    assert_eq!(reach, C_LIGHT * 60.0 * 64.0);
    let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e12, 0.0, 0.0, 0.0, 0.0, 0.0)];
    assert!(
        super::presence_gate(&presences, (0.0, 0.0, 0.0), reach, 0.0, None, None),
        "the em source at the presence anchor must be fetched"
    );
}

#[test]
fn test_fetch_dispatch_gate_window_range_does_not_fetch() {
    let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
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
        fold: None,
    };
    let reach = super::dispatch_reach(&[fc], 60.0).expect("thermal carries a propagation law");
    assert_eq!(reach, (2.0 * DIFFUSIVITY_THERMAL * 60.0 * 64.0).sqrt());
    let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0)];
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
    let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e9, 0.0, 0.0, 0.0, 0.0, 0.0)];
    assert!(
        !super::presence_gate(&presences, (50.0, 0.0, 0.0), 10.0, 5.0, None, None),
        "a resting presence fetches only within reach + extent"
    );
}

#[test]
fn test_fetch_gate_thrust_anticipates_within_median_window() {
    let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
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
    let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
        vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e9, 100.0, 0.0, 0.0, 0.0, 0.0)];
    assert!(
        !super::presence_gate(&presences, (1000.0, 0.0, 0.0), 0.0, 0.0, None, Some(20.0)),
        "a frameless anchor carries no velocity — only the rest gate applies"
    );
}

#[test]
fn test_fetch_gate_thrust_without_median_rests() {
    let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
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
    let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
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
    let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
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
    let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
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
fn test_anchor_bodies_have_ephemeris_sources() {
    let content = std::fs::read_to_string("phi/sources.φ").unwrap();
    let sources = super::parse_sources(&content);
    let uses = super::anchor_uses(&sources);
    let eph_bodies: std::collections::HashSet<String> = sources
        .iter()
        .filter(|s| s.format == "ephemeris_binary" || s.format == "orbit_bin")
        .filter_map(|s| s.body.clone())
        .collect();
    let missing: Vec<&String> = uses.keys().filter(|b| !eph_bodies.contains(*b)).collect();
    assert!(
        missing.is_empty(),
        "anchor bodies without an ephemeris source would starve the load gate: {:?}",
        missing
    );
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
    let hash = super::build_spatial_hash(vec![sample.clone()], 1.0);
    let mut recs = Vec::new();
    super::query_hash(
        &hash,
        pos,
        now,
        8.0e6,
        0.0,
        &[1.0e-300_f64; 9],
        1.0,
        [0.0, 0.0, 0.0],
        &mut recs,
        &eph_map,
    );
    assert!(
        !recs.is_empty(),
        "the surface sample within the window pad must reach the membrane, got {} records",
        recs.len()
    );
    let mut recs_ssb = Vec::new();
    super::query_hash(
        &hash,
        [0.0, 0.0, 0.0],
        now,
        2.0e12,
        0.0,
        &[1.0e-300_f64; 9],
        1.0,
        [0.0, 0.0, 0.0],
        &mut recs_ssb,
        &eph_map,
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
    for k in 0..3 {
        let expected = (records[0].1[k] + records[1].1[k]) * 0.5;
        assert!((mid[k] - expected).abs() < 1.0e-3);
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
        hapi_fill: HashMap::new(),
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
        hapi_fill: HashMap::new(),
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
        hapi_fill: HashMap::new(),
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
    let cmap_block =
            "url https://example.org/c\nttl 3600\nformat json\nat sun\ncmap data\nra ra\ndec dec\ntau_key tkey\n";
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
        hapi_fill: HashMap::new(),
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
        hapi_fill: HashMap::new(),
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
fn test_field_in_nested_port_and_flatten_generic() {
    let legacy = "source geosphere\nttl 86400\nforce seismic-body\nurl https://example.org/g\nmap data\nlat_key lat\nlon_key lon\nfield_in geometry.coordinates.2 quake_depth\nfield_in properties.mag quake_mag\n";
    let conv = super::port_block(legacy);
    let srcs = super::parse_sources(&conv);
    assert_eq!(srcs.len(), 0);

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
        hapi_fill: HashMap::new(),
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
            dist_scale: 1.0,
            plx_key: "plx".into(),
            z_key: String::new(),
            pmra_key: String::new(),
            pmdec_key: String::new(),
            rv_key: String::new(),
            rv_scale: 1.0,
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
        hapi_fill: HashMap::new(),
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
        hapi_fill: HashMap::new(),
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
        hapi_fill: HashMap::new(),
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
    assert_eq!(crate::force::kernel_id_for_force(8), Some(1));
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
fn test_ci_probe_render_resolves_templates_and_secrets() {
    let mut env = HashMap::new();
    env.insert("FIRMS_MAP_KEY".to_string(), "ABC123".to_string());
    let url = ci_probe_render(
        "https://example.com/?lat={lat}&lon={lon}&key={FIRMS_MAP_KEY}",
        (52.5, 13.4),
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
            (0.0, 0.0),
            &env,
        )
        .unwrap();
    assert!(!url.contains('{'), "unresolved marker in {}", url);
    assert!(url.contains("start=20"), "missing week_ago in {}", url);
    assert!(url.contains("end=20"), "missing today in {}", url);
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
