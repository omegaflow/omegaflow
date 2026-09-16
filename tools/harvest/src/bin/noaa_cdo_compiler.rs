use omegaflow::archivar::{
    days_to_ymd, embedded_lsk, fetch_raw_bytes_headers, jnum, jstr, load_env, parse_json,
    render_headers, JsonVal, LeapSeconds,
};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::time::{SystemTime, UNIX_EPOCH};

const CDN_TAG: &str = "www.ncdc.noaa.gov";
const STATIONS_URL: &str = "https://www.ncdc.noaa.gov/cdo-web/api/v2/stations";
const DATA_URL: &str = "https://www.ncdc.noaa.gov/cdo-web/api/v2/data";
const DATATYPE: &str = "TMAX";

const MAGIC: [u8; 4] = *b"NCDO";
const HEADER_BYTES: usize = 8;
const REC_BYTES: usize = 32;

#[derive(Clone, Copy, Debug)]
struct Obs {
    lat_deg: f64,
    lon_deg: f64,
    date_tdb: f64,
    value_c: f64,
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn date_unix(s: &str) -> Option<f64> {
    if s.len() < 10 {
        return None;
    }
    let y: i64 = s.get(0..4)?.parse().ok()?;
    let m: i64 = s.get(5..7)?.parse().ok()?;
    let d: i64 = s.get(8..10)?.parse().ok()?;
    Some(days_from_civil(y, m, d)? as f64 * 86400.0)
}

fn results_array<'a>(json: &'a JsonVal) -> Option<&'a [JsonVal]> {
    match json {
        JsonVal::Obj(map) => match map.get("results") {
            Some(JsonVal::Arr(a)) => Some(a.as_slice()),
            _ => None,
        },
        _ => None,
    }
}

fn parse_stations(text: &str) -> Option<Vec<(String, f64, f64)>> {
    let json = parse_json(text)?;
    let results = results_array(&json)?;
    let mut out = Vec::with_capacity(results.len());
    for st in results {
        let (Some(id), Some(lat), Some(lon)) =
            (jstr(st, "id"), jnum(st, "latitude"), jnum(st, "longitude"))
        else {
            continue;
        };
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            continue;
        }
        out.push((id, lat, lon));
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

fn parse_observations(text: &str, lat: f64, lon: f64, lsk: &LeapSeconds) -> Vec<Obs> {
    let Some(json) = parse_json(text) else {
        return Vec::new();
    };
    let Some(results) = results_array(&json) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for r in results {
        let Some(date) = jstr(r, "date") else {
            continue;
        };
        let Some(unix) = date_unix(&date) else {
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let Some(v) = jnum(r, "value") else {
            continue;
        };
        if !v.is_finite() || !(-100.0..=100.0).contains(&v) {
            continue;
        }
        out.push(Obs {
            lat_deg: lat,
            lon_deg: lon,
            date_tdb: tdb,
            value_c: v,
        });
    }
    out
}

fn write_bin(records: &[Obs]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        out.extend_from_slice(&r.lat_deg.to_le_bytes());
        out.extend_from_slice(&r.lon_deg.to_le_bytes());
        out.extend_from_slice(&r.date_tdb.to_le_bytes());
        out.extend_from_slice(&r.value_c.to_le_bytes());
    }
    out
}

fn read_bin(data: &[u8]) -> Option<Vec<Obs>> {
    if data.len() < HEADER_BYTES || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != HEADER_BYTES + count * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * REC_BYTES;
        let f64_at = |o: usize| -> Option<f64> {
            Some(f64::from_le_bytes(data.get(o..o + 8)?.try_into().ok()?))
        };
        let lat_deg = f64_at(base)?;
        let lon_deg = f64_at(base + 8)?;
        let date_tdb = f64_at(base + 16)?;
        let value_c = f64_at(base + 24)?;
        if !lat_deg.is_finite()
            || !lon_deg.is_finite()
            || !date_tdb.is_finite()
            || !value_c.is_finite()
        {
            return None;
        }
        out.push(Obs {
            lat_deg,
            lon_deg,
            date_tdb,
            value_c,
        });
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: noaa_cdo_compiler --out <bin> [--window-start <YYYY-MM-DD>] [--window-end <YYYY-MM-DD>] [--stations <n>] [--extent <lat_min,lon_min,lat_max,lon_max>] [--limit <n>] [--ci-mode]";
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("{usage}");
            std::process::exit(1);
        }
    };
    let stations_limit: usize = arg_value(&args, "--stations")
        .and_then(|v| v.parse().ok())
        .unwrap_or(50);
    if stations_limit == 0 {
        eprintln!(
            "--stations {} carries no positive fan-out bound",
            stations_limit
        );
        std::process::exit(1);
    }
    let limit: usize = arg_value(&args, "--limit")
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    if limit == 0 {
        eprintln!("--limit {} carries no positive per-station bound", limit);
        std::process::exit(1);
    }
    let now_secs = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => {
            eprintln!("system clock precedes UNIX_EPOCH — the window is unmeasurable");
            std::process::exit(1);
        }
    };
    let today_days = now_secs / 86400;
    let (wy, wm, wd) = days_to_ymd(today_days.saturating_sub(7));
    let (yy, ym, yd) = days_to_ymd(today_days.saturating_sub(1));
    let window_start = match arg_value(&args, "--window-start") {
        Some(v) => v,
        None => format!("{wy:04}-{wm:02}-{wd:02}"),
    };
    let window_end = match arg_value(&args, "--window-end") {
        Some(v) => v,
        None => format!("{yy:04}-{ym:02}-{yd:02}"),
    };

    let env = load_env();
    let headers = render_headers(
        &[("token".to_string(), "{NOAA_CDO_TOKEN}".to_string())],
        &env,
    );
    let Some(lsk) = embedded_lsk() else {
        eprintln!(
            "noaa_cdo_compiler: the embedded naif0012.tls leap table is absent — no date folds to the TDB clock; the harvest stays unwritten (0 honored, pending)"
        );
        std::process::exit(1);
    };

    let mut stations_url = format!(
        "{STATIONS_URL}?datasetid=GHCND&datatypeid={DATATYPE}&sortfield=maxdate&sortorder=desc&limit={stations_limit}"
    );
    if let Some(ext) = arg_value(&args, "--extent") {
        stations_url.push_str(&format!("&extent={ext}"));
    }
    let st_bytes = match fetch_raw_bytes_headers(&stations_url, &headers, 3600) {
        Some(b) => b,
        None => {
            eprintln!("noaa_cdo_compiler: stations fetch void ({stations_url})");
            std::process::exit(1);
        }
    };
    let stations = match parse_stations(&String::from_utf8_lossy(&st_bytes)) {
        Some(s) => s,
        None => {
            eprintln!("noaa_cdo_compiler: stations parse void — no station records");
            std::process::exit(1);
        }
    };
    eprintln!("noaa_cdo_compiler: {} stations", stations.len());

    let mut records: Vec<Obs> = Vec::new();
    let mut void_stations = 0usize;
    for (id, lat, lon) in &stations {
        let data_url = format!(
            "{DATA_URL}?datasetid=GHCND&startdate={window_start}&enddate={window_end}&datatypeid={DATATYPE}&stationid={id}&limit={limit}&units=metric"
        );
        let Some(db) = fetch_raw_bytes_headers(&data_url, &headers, 3600) else {
            void_stations += 1;
            continue;
        };
        let obs = parse_observations(&String::from_utf8_lossy(&db), *lat, *lon, &lsk);
        records.extend(obs);
    }
    if void_stations > 0 {
        eprintln!(
            "noaa_cdo_compiler: {} of {} stations returned void",
            void_stations,
            stations.len()
        );
    }
    if records.is_empty() {
        eprintln!("noaa_cdo_compiler: no observations — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| {
        a.date_tdb
            .total_cmp(&b.date_tdb)
            .then(a.lat_deg.total_cmp(&b.lat_deg))
            .then(a.lon_deg.total_cmp(&b.lon_deg))
    });
    let bin = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("noaa_cdo_compiler: write {out_path} void");
        std::process::exit(1);
    }
    match read_bin(&bin) {
        Some(parsed) => eprintln!(
            "noaa_cdo_compiler: {} observations, {} B -> {out_path} (roundtrip parses)",
            parsed.len(),
            bin.len()
        ),
        None => {
            eprintln!(
                "noaa_cdo_compiler: {out_path}: roundtrip parse void — the asset stays unverified"
            );
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(CDN_TAG, &out_path) {
        eprintln!("noaa_cdo_compiler: upload {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Obs> {
        vec![
            Obs {
                lat_deg: 40.7794,
                lon_deg: -73.9692,
                date_tdb: 810_641_669.184,
                value_c: 27.8,
            },
            Obs {
                lat_deg: 40.7794,
                lon_deg: -73.9692,
                date_tdb: 810_728_069.184,
                value_c: -2.1,
            },
        ]
    }

    #[test]
    fn bin_roundtrip() {
        let recs = sample();
        let bytes = write_bin(&recs);
        let parsed = read_bin(&bytes).expect("parse");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].lat_deg, recs[0].lat_deg);
        assert_eq!(parsed[1].value_c, -2.1);
    }

    #[test]
    fn bin_rejects_bad_magic() {
        assert!(read_bin(b"XXXX").is_none());
        let mut bad = write_bin(&sample());
        bad[0] = b'X';
        assert!(read_bin(&bad).is_none());
    }

    #[test]
    fn bin_rejects_truncation() {
        let bytes = write_bin(&sample());
        assert!(read_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn date_parses_to_unix_days() {
        let unix = date_unix("2026-09-09T00:00:00").expect("date");
        assert_eq!(unix / 86400.0, days_from_civil(2026, 9, 9).unwrap() as f64);
    }

    #[test]
    fn stations_parse_records_position() {
        let text = r#"{"metadata":{"resultset":{"offset":1,"count":2,"limit":2}},"results":[{"latitude":17.11667,"longitude":-61.78333,"id":"GHCND:ACW00011604"},{"latitude":17.13333,"longitude":-61.78333,"id":"GHCND:ACW00011605"}]}"#;
        let stations = parse_stations(text).expect("stations");
        assert_eq!(stations.len(), 2);
        assert_eq!(stations[0].0, "GHCND:ACW00011604");
        assert_eq!(stations[1].1, 17.13333);
    }

    #[test]
    fn observations_parse_value_and_date() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let text = r#"{"results":[{"date":"2026-09-09T00:00:00","datatype":"TMAX","station":"GHCND:USW00094728","value":27.8},{"date":"2026-09-10T00:00:00","datatype":"TMAX","station":"GHCND:USW00094728","value":-3.4}]}"#;
        let obs = parse_observations(text, 40.0, -73.0, &lsk);
        assert_eq!(obs.len(), 2);
        assert_eq!(obs[0].value_c, 27.8);
        assert_eq!(obs[1].value_c, -3.4);
        assert_eq!(obs[0].lat_deg, 40.0);
    }

    #[test]
    fn observations_fold_date_to_tdb() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let text = r#"{"results":[{"date":"2026-09-09T00:00:00","datatype":"TMAX","station":"GHCND:USW00094728","value":27.8}]}"#;
        let obs = parse_observations(text, 40.0, -73.0, &lsk);
        assert_eq!(obs.len(), 1);
        let unix = days_from_civil(2026, 9, 9).unwrap() as f64 * 86400.0;
        let expected = lsk.unix_to_tdb(unix).expect("2026 tdb");
        assert_eq!(obs[0].date_tdb, expected);
    }

    #[test]
    fn observations_skip_pre_1972_dates() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let text = r#"{"results":[{"date":"1969-07-20T00:00:00","datatype":"TMAX","station":"GHCND:USW00094728","value":27.8}]}"#;
        let obs = parse_observations(text, 40.0, -73.0, &lsk);
        assert!(
            obs.is_empty(),
            "a pre-1972 date carries no leap-table entry — absent, never 0.0"
        );
    }
}
