use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{COMP_TOAR_O3, GeoRec, MAGIC_TOAR, parse_bin, write_bin};
use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::cdn::upload_release;
use std::collections::HashMap;

const NETLOC: &str = "toar-data.fz-juelich.de";
const API_BASE: &str = "https://toar-data.fz-juelich.de/api/v2";
const O3_VARIABLE_ID: u32 = 5;
const DEFAULT_START_ID: u64 = 1000;
const DEFAULT_END_ID: u64 = 2000;
const ALT_MISSING_SENTINEL: f64 = -999.0;
const ALT_PLAUSIBLE_FLOOR: f64 = -500.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_u64(args: &[String], name: &str) -> Option<u64> {
    arg_value(args, name).and_then(|v| v.parse::<u64>().ok())
}

fn obj_of(v: &JsonVal) -> Option<&HashMap<String, JsonVal>> {
    match v {
        JsonVal::Obj(m) => Some(m),
        _ => None,
    }
}

fn field_of<'a>(m: &'a HashMap<String, JsonVal>, key: &str) -> Option<&'a JsonVal> {
    m.get(key)
}

fn str_at(m: &HashMap<String, JsonVal>, key: &str) -> Option<String> {
    match field_of(m, key)? {
        JsonVal::Str(s) => Some(s.clone()),
        _ => None,
    }
}

fn num_at(m: &HashMap<String, JsonVal>, key: &str) -> Option<f64> {
    match field_of(m, key)? {
        JsonVal::Num(n) => Some(*n),
        _ => None,
    }
}

fn plausible_elevation(alt: f64) -> Option<f64> {
    if alt.is_finite() && alt > ALT_PLAUSIBLE_FLOOR && alt != ALT_MISSING_SENTINEL {
        Some(alt)
    } else {
        None
    }
}

fn flag_ok(flags: &str) -> bool {
    let f = flags.to_ascii_lowercase();
    !f.contains("erroneous") && !f.contains("flagged")
}

fn value_gate(v: f64) -> Option<f64> {
    if v.is_finite() && v > 0.0 {
        Some(v)
    } else {
        None
    }
}

fn parse_datetime_unix(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 19
        || b[4] != b'-'
        || b[7] != b'-'
        || b[10] != b'T'
        || b[13] != b':'
        || b[16] != b':'
    {
        return None;
    }
    let digits = |r: std::ops::Range<usize>| -> Option<i64> {
        let slice = b.get(r)?;
        if !slice.iter().all(u8::is_ascii_digit) {
            return None;
        }
        std::str::from_utf8(slice).ok()?.parse::<i64>().ok()
    };
    let y = digits(0..4)?;
    let mo = digits(5..7)?;
    let d = digits(8..10)?;
    let hh = digits(11..13)?;
    let mm = digits(14..16)?;
    let ss = digits(17..19)?;
    if !(1..=12).contains(&mo) || !(1..=31).contains(&d) || hh > 23 || mm > 59 || ss > 60 {
        return None;
    }
    let offset = parse_offset(&b[19..])?;
    let days = days_from_civil(y, mo, d)?;
    Some(days as f64 * 86400.0 + hh as f64 * 3600.0 + mm as f64 * 60.0 + ss as f64 - offset)
}

fn parse_offset(rest: &[u8]) -> Option<f64> {
    let mut i = 0usize;
    if i < rest.len() && rest[i] == b'.' {
        i += 1;
        while i < rest.len() && rest[i].is_ascii_digit() {
            i += 1;
        }
    }
    if i >= rest.len() {
        return Some(0.0);
    }
    match rest[i] {
        b'Z' | b'z' => Some(0.0),
        b'+' | b'-' => {
            let two = |r: std::ops::Range<usize>| -> Option<f64> {
                let s = rest.get(r)?;
                if !s.iter().all(u8::is_ascii_digit) {
                    return None;
                }
                std::str::from_utf8(s).ok()?.parse::<f64>().ok()
            };
            let oh = two(i + 1..i + 3)?;
            let om = two(i + 4..i + 6)?;
            if oh > 23.0 || om > 59.0 {
                return None;
            }
            let sign = if rest[i] == b'+' { 1.0 } else { -1.0 };
            Some(sign * (oh * 3600.0 + om * 60.0))
        }
        _ => None,
    }
}

struct Station {
    lat: f64,
    lon: f64,
    alt: Option<f64>,
}

fn station_of(metadata: &HashMap<String, JsonVal>) -> Option<Station> {
    let station = obj_of(field_of(metadata, "station")?)?;
    let coords = obj_of(field_of(station, "coordinates")?)?;
    let lat = num_at(coords, "lat")?;
    let lon = num_at(coords, "lng")?;
    if !lat.is_finite()
        || !lon.is_finite()
        || !(-90.0..=90.0).contains(&lat)
        || !(-180.0..=180.0).contains(&lon)
    {
        return None;
    }
    let mut alt = num_at(coords, "alt").and_then(plausible_elevation);
    if alt.is_none() {
        if let Some(globalmeta) = obj_of(field_of(station, "globalmeta")?) {
            alt = num_at(globalmeta, "mean_topography_srtm_alt_90m_year1994")
                .and_then(plausible_elevation);
        }
    }
    Some(Station { lat, lon, alt })
}

fn is_o3(metadata: &HashMap<String, JsonVal>) -> bool {
    match field_of(metadata, "variable").and_then(obj_of) {
        Some(variable) => {
            let name_matches = matches!(str_at(variable, "name").as_deref(), Some("o3"));
            let id_matches = match num_at(variable, "id").map(|n| n as u32) {
                Some(id) => id == O3_VARIABLE_ID,
                None => false,
            };
            name_matches || id_matches
        }
        None => false,
    }
}

fn data_records(
    data_arr: &[JsonVal],
    station: &Station,
    lsk: &omegaflow::archivar::LeapSeconds,
) -> Vec<GeoRec> {
    let mut out = Vec::new();
    for item in data_arr {
        let Some(row) = obj_of(item) else {
            continue;
        };
        let Some(dt) = str_at(row, "datetime") else {
            continue;
        };
        let Some(flags) = str_at(row, "flags") else {
            continue;
        };
        if !flag_ok(&flags) {
            continue;
        }
        let Some(val) = num_at(row, "value").and_then(value_gate) else {
            continue;
        };
        let Some(unix) = parse_datetime_unix(&dt) else {
            continue;
        };
        let Some(t) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let Some(alt) = station.alt else {
            continue;
        };
        out.push(GeoRec {
            t,
            lat: station.lat,
            lon: station.lon,
            alt,
            freq: 0.0,
            bin_width: 0.0,
            val,
            comp: COMP_TOAR_O3,
            station: 0,
        });
    }
    out
}

struct Harvest {
    o3_timeseries: usize,
    o3_series_void: usize,
    data_rows: usize,
    records: Vec<GeoRec>,
}

fn harvest(
    start_id: u64,
    end_id: u64,
    max_timeseries: Option<u64>,
    lsk: &omegaflow::archivar::LeapSeconds,
) -> Harvest {
    let mut h = Harvest {
        o3_timeseries: 0,
        o3_series_void: 0,
        data_rows: 0,
        records: Vec::new(),
    };
    for id in start_id..=end_id {
        if let Some(cap) = max_timeseries {
            if h.o3_timeseries as u64 >= cap {
                break;
            }
        }
        let meta_url = format!("{API_BASE}/timeseries/id/{id}");
        let Some(meta_bytes) = fetch_raw_bytes(&meta_url) else {
            continue;
        };
        let Some(meta_json) = String::from_utf8(meta_bytes)
            .ok()
            .and_then(|t| parse_json(&t))
        else {
            continue;
        };
        let Some(metadata) = obj_of(&meta_json).cloned() else {
            continue;
        };
        if !is_o3(&metadata) {
            continue;
        }
        let Some(station) = station_of(&metadata) else {
            eprintln!("toar: timeseries {id} carries no plausible station position — skipped");
            continue;
        };
        let data_url = format!("{API_BASE}/data/timeseries/id/{id}");
        let Some(data_bytes) = fetch_raw_bytes(&data_url) else {
            h.o3_series_void += 1;
            eprintln!("toar: timeseries {id} data fetch void — series skipped");
            continue;
        };
        let Some(data_json) = String::from_utf8(data_bytes)
            .ok()
            .and_then(|t| parse_json(&t))
        else {
            h.o3_series_void += 1;
            eprintln!("toar: timeseries {id} carries no json — series skipped");
            continue;
        };
        let Some(data_obj) = obj_of(&data_json) else {
            h.o3_series_void += 1;
            continue;
        };
        let data_arr = match field_of(data_obj, "data") {
            Some(JsonVal::Arr(a)) => a,
            _ => {
                h.o3_series_void += 1;
                continue;
            }
        };
        let n = data_arr.len();
        let recs = data_records(data_arr, &station, lsk);
        h.o3_timeseries += 1;
        h.data_rows += n;
        eprintln!(
            "toar: timeseries {id} → {} records ({} data rows)",
            recs.len(),
            n
        );
        h.records.extend(recs);
    }
    h
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_bin = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => {
            eprintln!(
                "usage: toar_timeseries_compiler --out-bin <path> [--start-id <n>] [--end-id <n>] [--max-timeseries <n>] [--ci-mode]"
            );
            std::process::exit(1);
        }
    };
    let start_id = match arg_u64(&args, "--start-id") {
        Some(v) => v,
        None => DEFAULT_START_ID,
    };
    let end_id = match arg_u64(&args, "--end-id") {
        Some(v) => v,
        None => DEFAULT_END_ID,
    };
    let max_timeseries = arg_u64(&args, "--max-timeseries");
    let lsk = match embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!(
                "toar: embedded naif0012.tls stays unread — the date→TDB step is unavailable"
            );
            std::process::exit(1);
        }
    };

    let mut h = harvest(start_id, end_id, max_timeseries, &lsk);
    if h.records.is_empty() {
        eprintln!(
            "toar: no measured O3 record left the harvest — {} O3 series read, {} series void, {} data rows — the bin stays unwritten (0 honored)",
            h.o3_timeseries, h.o3_series_void, h.data_rows
        );
        std::process::exit(1);
    }
    h.records.sort_by(|a, b| {
        a.t.total_cmp(&b.t)
            .then(a.lat.total_cmp(&b.lat))
            .then(a.lon.total_cmp(&b.lon))
    });
    let bytes = write_bin(MAGIC_TOAR, &h.records);
    if let Some(parent) = std::path::Path::new(&out_bin).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_bin, &bytes).is_err() {
        eprintln!("toar: write {out_bin} returned void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_TOAR, &bytes) {
        Some(parsed) => eprintln!(
            "toar: {} geo records ({} O3 series, {} series void, {} data rows), {} B → {out_bin}, roundtrip parses",
            parsed.len(),
            h.o3_timeseries,
            h.o3_series_void,
            h.data_rows,
            bytes.len()
        ),
        None => {
            eprintln!("toar: {out_bin} roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_bin) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datetime_parse_handles_utc_z_and_offset_and_fraction() {
        let a = parse_datetime_unix("2016-03-10T07:00:00+00:00").expect("offset");
        let b = parse_datetime_unix("2016-03-10T07:00:00Z").expect("z");
        let c = parse_datetime_unix("2016-03-10T07:00:00.500+00:00").expect("fraction");
        assert_eq!(a, b);
        assert!((c - a - 0.5).abs() < 1e-9);
        let d = parse_datetime_unix("2016-03-10T07:00:00+01:00").expect("plus offset");
        assert_eq!(d, a - 3600.0);
    }

    #[test]
    fn datetime_parse_rejects_malformed_tokens() {
        assert!(parse_datetime_unix("").is_none());
        assert!(parse_datetime_unix("2016-13-10T07:00:00+00:00").is_none());
        assert!(parse_datetime_unix("2016-02-30T07:00:00+00:00").is_none());
        assert!(parse_datetime_unix("2016-03-10 07:00:00+00:00").is_none());
        assert!(parse_datetime_unix("2016-03-10T25:00:00+00:00").is_none());
        assert!(parse_datetime_unix("2016-03-10T07:00:00").is_none());
    }

    #[test]
    fn elevation_gate_admits_plausible_and_rejects_the_missing_sentinel() {
        assert_eq!(plausible_elevation(5.0), Some(5.0));
        assert_eq!(plausible_elevation(0.0), Some(0.0));
        assert_eq!(plausible_elevation(-430.0), Some(-430.0));
        assert_eq!(plausible_elevation(ALT_MISSING_SENTINEL), None);
        assert_eq!(plausible_elevation(f64::NAN), None);
        assert_eq!(plausible_elevation(-9999.0), None);
    }

    #[test]
    fn flag_gate_keeps_ok_and_drops_erroneous() {
        assert!(flag_ok("OK validated QC passed"));
        assert!(!flag_ok("erroneous validated flagged (1)"));
        assert!(!flag_ok("ERRONEOUS"));
        assert!(flag_ok(""));
    }

    #[test]
    fn value_gate_admits_only_finite_positive() {
        assert_eq!(value_gate(78.0), Some(78.0));
        assert_eq!(value_gate(-1.0), None);
        assert_eq!(value_gate(0.0), None);
        assert_eq!(value_gate(f64::NAN), None);
        assert_eq!(value_gate(f64::INFINITY), None);
    }

    #[test]
    fn station_parses_coordinates_and_falls_back_to_srtm_elevation() {
        let mk = |json: &str| -> HashMap<String, JsonVal> {
            match parse_json(json).expect("json") {
                JsonVal::Obj(m) => m,
                _ => panic!("not an object"),
            }
        };
        let with_alt = mk(
            r#"{"station":{"coordinates":{"lat":42.6,"lng":-72.5,"alt":82.0},
                "globalmeta":{"mean_topography_srtm_alt_90m_year1994":84.5}}}"#,
        );
        let s = station_of(&with_alt).expect("station");
        assert_eq!(s.alt, Some(82.0));

        let missing_alt = mk(
            r#"{"station":{"coordinates":{"lat":42.6,"lng":-72.5,"alt":-999.0},
                "globalmeta":{"mean_topography_srtm_alt_90m_year1994":84.5}}}"#,
        );
        let s = station_of(&missing_alt).expect("station");
        assert_eq!(s.alt, Some(84.5));

        let bad_lat = mk(
            r#"{"station":{"coordinates":{"lat":95.0,"lng":-72.5,"alt":82.0},
                "globalmeta":{"mean_topography_srtm_alt_90m_year1994":84.5}}}"#,
        );
        assert!(station_of(&bad_lat).is_none());
    }

    #[test]
    fn is_o3_reads_name_and_id() {
        let mk = |json: &str| -> HashMap<String, JsonVal> {
            match parse_json(json).expect("json") {
                JsonVal::Obj(m) => m,
                _ => panic!("not an object"),
            }
        };
        assert!(is_o3(&mk(r#"{"variable":{"name":"o3","id":5}}"#)));
        assert!(is_o3(&mk(r#"{"variable":{"name":"x","id":5}}"#)));
        assert!(is_o3(&mk(r#"{"variable":{"name":"o3","id":0}}"#)));
        assert!(!is_o3(&mk(r#"{"variable":{"name":"no2","id":6}}"#)));
    }
}
