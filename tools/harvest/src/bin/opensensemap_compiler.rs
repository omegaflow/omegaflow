use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::geo::{COMP_OSM_TEMP, GeoRec, MAGIC_OSM, parse_bin, write_bin};
use omegaflow::archivar::json::{JsonVal, jpath_val, jstr, parse_json, scalar_of};
use omegaflow::archivar::parse_iso_tdb;
use omegaflow::cdn::upload_release;
use omegaflow::zeuge::{FeldIdentitaet, magic_identity};

const NETLOC: &str = "api.opensensemap.org";
const BOXES_URL: &str =
    "https://api.opensensemap.org/boxes?bbox=7.5,47.5,8.5,48.5&phenomenon=Temperatur";
const PHENOMENON: &str = "Temperatur";
const UNIT_CELSIUS: &str = "°C";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn witness_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FeldIdentitaet::Oszillator) => Ok(()),
        Some(other) => Err(format!(
            "{} reads {:?}, not an oscillator — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn box_ids(body: &str) -> Vec<String> {
    let Some(json) = parse_json(body) else {
        return Vec::new();
    };
    let JsonVal::Arr(arr) = json else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for box_val in arr {
        if let Some(id) = jstr(&box_val, "_id") {
            out.push(id);
        }
    }
    out
}

fn extract_temperature(detail: &JsonVal, lsk: &LeapSeconds) -> Option<(f64, f64, f64, f64)> {
    let coords = match jpath_val(detail, "currentLocation.coordinates") {
        Some(JsonVal::Arr(a)) if a.len() >= 2 => a,
        _ => return None,
    };
    let lon = scalar_of(&coords[0])?;
    let lat = scalar_of(&coords[1])?;
    if !lat.is_finite() || !lon.is_finite() {
        return None;
    }
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return None;
    }
    let sensors = match jpath_val(detail, "sensors") {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    for sensor in sensors {
        if jstr(sensor, "title").as_deref() != Some(PHENOMENON) {
            continue;
        }
        if jstr(sensor, "unit").as_deref() != Some(UNIT_CELSIUS) {
            continue;
        }
        let value = match jpath_val(sensor, "lastMeasurement.value") {
            Some(v) => scalar_of(v),
            None => continue,
        };
        let Some(value) = value else { continue };
        if !value.is_finite() {
            continue;
        }
        let t = match jstr(sensor, "lastMeasurement.createdAt") {
            Some(ts) => parse_iso_tdb(&ts, lsk),
            None => None,
        };
        let Some(t) = t else { continue };
        return Some((t, lat, lon, value));
    }
    None
}

fn to_records(details: &[(String, JsonVal)], lsk: &LeapSeconds) -> (Vec<GeoRec>, usize) {
    let mut records = Vec::new();
    let mut skipped = 0usize;
    for (id, detail) in details {
        match extract_temperature(detail, lsk) {
            Some((t, lat, lon, value)) => {
                records.push(GeoRec {
                    t,
                    lat,
                    lon,
                    alt: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    val: value,
                    comp: COMP_OSM_TEMP,
                    station: 0,
                });
            }
            None => {
                skipped += 1;
                eprintln!("{id}: no Temperatur °C measurement — the box stays unrecorded");
            }
        }
    }
    records.sort_by(|a, b| {
        a.t.total_cmp(&b.t)
            .then(a.lat.total_cmp(&b.lat))
            .then(a.lon.total_cmp(&b.lon))
    });
    (records, skipped)
}

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or_else(|| {
        "the embedded naif0012.tls stays unread — the date→TDB step is unavailable".to_string()
    })?;
    witness_identity(MAGIC_OSM)?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/opensensemap_temperatur.bin"),
    };
    let boxes_url = match arg_value(args, "--boxes-url") {
        Some(v) => v,
        None => BOXES_URL.to_string(),
    };

    let list_body =
        fetch_raw(&boxes_url, None, &[]).ok_or_else(|| format!("{boxes_url}: fetch void"))?;
    let mut ids = box_ids(&list_body);
    ids.sort();
    ids.dedup();
    if ids.is_empty() {
        return Err(format!(
            "{boxes_url}: no box id — the asset stays unwritten (0 honored)"
        ));
    }

    let mut details: Vec<(String, JsonVal)> = Vec::new();
    let mut fetch_void = 0usize;
    for id in &ids {
        let url = format!("https://api.opensensemap.org/boxes/{id}");
        match fetch_raw(&url, None, &[]) {
            Some(body) => match parse_json(&body) {
                Some(json) => details.push((id.clone(), json)),
                None => {
                    fetch_void += 1;
                    eprintln!("{url}: json void");
                }
            },
            None => {
                fetch_void += 1;
                eprintln!("{url}: fetch void");
            }
        }
    }

    let (records, skipped) = to_records(&details, &lsk);
    if records.is_empty() {
        return Err(format!(
            "no temperature measurement left the harvest — {} boxes, {} fetch-void, {} skipped; the asset stays unwritten (0 honored)",
            ids.len(),
            fetch_void,
            skipped
        ));
    }
    let bytes = write_bin(MAGIC_OSM, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;
    match parse_bin(MAGIC_OSM, &bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} temperature records ({} boxes, {} fetch-void, {} skipped), roundtrip parses",
                parsed.len(),
                ids.len(),
                fetch_void,
                skipped
            );
            Ok(())
        }
        Some(parsed) => Err(format!(
            "{out}: {} parsed vs {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        )),
        None => Err(format!(
            "{out}: roundtrip parse void — the asset stays unverified"
        )),
    }?;
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("opensensemap_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lsk() -> LeapSeconds {
        LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1_483_228_800.0)],
        }
    }

    #[test]
    fn osm_magic_is_an_oscillator() {
        assert_eq!(magic_identity(MAGIC_OSM), Some(FeldIdentitaet::Oszillator));
        assert!(witness_identity(MAGIC_OSM).is_ok());
    }

    #[test]
    fn box_ids_reads_the_list() {
        let body = r#"[{"_id":"abc","name":"a"},{"_id":"def","name":"b"}]"#;
        assert_eq!(box_ids(body), vec!["abc".to_string(), "def".to_string()]);
        assert!(box_ids(r#"{"_id":"x"}"#).is_empty());
        assert!(box_ids("").is_empty());
    }

    #[test]
    fn extract_matches_the_variable_sensor_index() {
        let lsk = test_lsk();
        let detail = parse_json(
            r#"{"currentLocation":{"coordinates":[7.854586,47.965607]},"sensors":[
            {"title":"PM10","unit":"µg/m³","lastMeasurement":{"value":"3.17","createdAt":"2026-09-23T17:25:09.195Z"}},
            {"title":"PM2.5","unit":"µg/m³","lastMeasurement":{"value":"2.93","createdAt":"2026-09-23T17:25:09.195Z"}},
            {"title":"Temperatur","unit":"°C","lastMeasurement":{"value":"16.20","createdAt":"2026-09-23T17:25:09.195Z"}}
            ]}"#,
        )
        .unwrap();
        let (t, lat, lon, value) = extract_temperature(&detail, &lsk).unwrap();
        assert!(t.is_finite());
        assert!((lat - 47.965607).abs() < 1e-9);
        assert!((lon - 7.854586).abs() < 1e-9);
        assert!((value - 16.20).abs() < 1e-9);
    }

    #[test]
    fn extract_skips_a_non_celsius_unit_and_a_missing_value() {
        let lsk = test_lsk();
        let fahrenheit = parse_json(
            r#"{"currentLocation":{"coordinates":[7.0,47.0]},"sensors":[
            {"title":"Temperatur","unit":"°F","lastMeasurement":{"value":"61.0","createdAt":"2026-09-23T17:25:09.195Z"}}
            ]}"#,
        )
        .unwrap();
        assert!(extract_temperature(&fahrenheit, &lsk).is_none());

        let no_value = parse_json(
            r#"{"currentLocation":{"coordinates":[7.0,47.0]},"sensors":[
            {"title":"Temperatur","unit":"°C","lastMeasurement":{"createdAt":"2026-09-23T17:25:09.195Z"}}
            ]}"#,
        )
        .unwrap();
        assert!(extract_temperature(&no_value, &lsk).is_none());
    }

    #[test]
    fn extract_admits_negative_and_zero_temperature() {
        let lsk = test_lsk();
        let frozen = parse_json(
            r#"{"currentLocation":{"coordinates":[7.0,47.0]},"sensors":[
            {"title":"Temperatur","unit":"°C","lastMeasurement":{"value":"-12.5","createdAt":"2026-09-23T17:25:09.195Z"}}
            ]}"#,
        )
        .unwrap();
        let (_, _, _, value) = extract_temperature(&frozen, &lsk).unwrap();
        assert!((value - (-12.5)).abs() < 1e-9);

        let zero = parse_json(
            r#"{"currentLocation":{"coordinates":[7.0,47.0]},"sensors":[
            {"title":"Temperatur","unit":"°C","lastMeasurement":{"value":"0","createdAt":"2026-09-23T17:25:09.195Z"}}
            ]}"#,
        )
        .unwrap();
        let (_, _, _, value) = extract_temperature(&zero, &lsk).unwrap();
        assert_eq!(value, 0.0);
    }

    #[test]
    fn record_roundtrip_preserves_the_geo_shape() {
        let lsk = test_lsk();
        let detail = parse_json(
            r#"{"currentLocation":{"coordinates":[7.854586,47.965607]},"sensors":[
            {"title":"Temperatur","unit":"°C","lastMeasurement":{"value":"16.20","createdAt":"2026-09-23T17:25:09.195Z"}}
            ]}"#,
        )
        .unwrap();
        let (records, skipped) = to_records(&[("abc".to_string(), detail)], &lsk);
        assert_eq!(skipped, 0);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].comp, COMP_OSM_TEMP);
        assert_eq!(records[0].freq, 0.0);
        assert_eq!(records[0].bin_width, 0.0);
        let bytes = write_bin(MAGIC_OSM, &records);
        let parsed = parse_bin(MAGIC_OSM, &bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert!((parsed[0].val - 16.20).abs() < 1e-9);
    }
}
