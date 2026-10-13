use omegaflow::archivar::fetch_raw_bytes_headers;
use omegaflow::archivar::hapi_csv::parse_iso_seconds;
use omegaflow::archivar::json::{JsonVal, jpath_val, jstr, parse_json, scalar_of};
use omegaflow::cdn::upload_release;
use omegaflow_harvest::table::{Table, arg_or, arg_value, emit, has_flag, stat};

const MAGIC: [u8; 4] = *b"OAQ3";
const NETLOC: &str = "api.openaq.org";
const FORMAT: &str = "openaq";
const COMPILER: &str = "tools/harvest/src/bin/openaq_compiler.rs";
const KEY_NAME: &str = "OPENAQ_API_KEY";
const PARAMETER: &str = "pm25";
const DEFAULT_URL: &str = "https://api.openaq.org/v3/sensors/{station}/measurements?limit=1&datetime_from={yesterday}T00:00:00Z";
const DEFAULT_OUT: &str = "openaq.bin";

fn secret(name: &str) -> Option<String> {
    if let Ok(value) = std::env::var(name) {
        if !value.is_empty() {
            return Some(value);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    for line in body.lines() {
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() == name && !value.trim().is_empty() {
                return Some(value.trim().to_string());
            }
        }
    }
    None
}

fn resolve_url(args: &[String]) -> String {
    let mut url = arg_or(args, "--url", DEFAULT_URL);
    if let Some(station) = arg_value(args, "--station") {
        url = url.replace("{station}", &station);
    }
    if let Some(yesterday) = arg_value(args, "--yesterday") {
        url = url.replace("{yesterday}", &yesterday);
    }
    url
}

fn results_table(json: &JsonVal) -> Option<Table> {
    let Some(JsonVal::Arr(results)) = jpath_val(json, "results") else {
        return None;
    };
    let names = vec![
        "openaq_pm25_ugm3".to_string(),
        "latitude".to_string(),
        "longitude".to_string(),
        "time".to_string(),
    ];
    let mut rows = Vec::new();
    for result in results {
        if let Some(parameter) = jstr(result, "parameter.name") {
            if parameter != PARAMETER {
                continue;
            }
        }
        let value = jpath_val(result, "value")
            .and_then(scalar_of)
            .filter(|v| v.is_finite());
        let latitude = jpath_val(result, "coordinates.latitude").and_then(scalar_of);
        let longitude = jpath_val(result, "coordinates.longitude").and_then(scalar_of);
        let time = jstr(result, "period.datetimeFrom.utc")
            .or_else(|| jstr(result, "datetime"))
            .and_then(|stamp| parse_iso_seconds(&stamp))
            .filter(|v| v.is_finite());
        rows.push(vec![value, latitude, longitude, time]);
    }
    if rows.is_empty() {
        None
    } else {
        Some(Table { names, rows })
    }
}

fn selftest() {
    let body = r#"{"results":[
        {"value":12.5,"parameter":{"name":"pm25"},"period":{"datetimeFrom":{"utc":"2026-01-01T00:00:00Z"}},"coordinates":{"latitude":52.5,"longitude":13.4}},
        {"value":30.0,"parameter":{"name":"pm10"},"coordinates":{"latitude":52.5,"longitude":13.4}},
        {"value":null,"parameter":{"name":"pm25"},"coordinates":{"latitude":52.5,"longitude":13.4}}
    ]}"#;
    let Some(json) = parse_json(body) else {
        eprintln!("selftest: the fixture parses void");
        std::process::exit(1);
    };
    let Some(table) = results_table(&json) else {
        eprintln!("selftest: the results do not form a table");
        std::process::exit(1);
    };
    if table.rows.len() != 2 || table.rows[0][0] != Some(12.5) || table.rows[1][0].is_some() {
        eprintln!("selftest: a pm25 value or a null absence is not read");
        std::process::exit(1);
    }
    if table.rows[0][1] != Some(52.5) || table.rows[0][3] != Some(1_767_225_600.0) {
        eprintln!("selftest: coordinates or the datetime stamp are not read");
        std::process::exit(1);
    }
    if results_table(&JsonVal::Null).is_some() {
        eprintln!("selftest: a body without results is not refused");
        std::process::exit(1);
    }
    eprintln!("openaq_compiler: selftest passes (results map + pm25 filter + null absence)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if has_flag(&args, "--selftest") {
        selftest();
        return;
    }
    let (src, bytes) = if let Some(path) = arg_value(&args, "--file") {
        match std::fs::read(&path) {
            Ok(bytes) => (path, bytes),
            Err(_) => {
                eprintln!("{path}: the file stays unread");
                std::process::exit(1);
            }
        }
    } else {
        let url = resolve_url(&args);
        let headers = match secret(KEY_NAME) {
            Some(key) => vec![("X-API-Key".to_string(), key)],
            None => Vec::new(),
        };
        match fetch_raw_bytes_headers(&url, &headers) {
            Some(bytes) => (url, bytes),
            None => {
                eprintln!("{url}: the source stays unfetched");
                std::process::exit(1);
            }
        }
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{src}: the body is not utf8 — absent (0 honored)");
        std::process::exit(1);
    };
    let Some(json) = parse_json(text) else {
        eprintln!("{src}: the body carries no JSON — absent (0 honored)");
        std::process::exit(1);
    };
    let Some(table) = results_table(&json) else {
        eprintln!("{src}: no pm25 result survived — absent (0 honored)");
        std::process::exit(1);
    };
    if has_flag(&args, "--stat") {
        stat(&src, &table);
        return;
    }
    let out = arg_or(&args, "--out", DEFAULT_OUT);
    let netloc = arg_or(&args, "--netloc", NETLOC);
    if !emit(&src, &out, &table, MAGIC, &netloc, FORMAT, COMPILER) {
        std::process::exit(1);
    }
    if has_flag(&args, "--ci-mode") && !upload_release(&netloc, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn results_map_to_rows_and_non_pm25_is_skipped() {
        let json = parse_json(
            r#"{"results":[
                {"value":12.5,"parameter":{"name":"pm25"},"period":{"datetimeFrom":{"utc":"2026-01-01T00:00:00Z"}},"coordinates":{"latitude":52.5,"longitude":13.4}},
                {"value":30.0,"parameter":{"name":"pm10"},"coordinates":{"latitude":52.5,"longitude":13.4}},
                {"value":null,"parameter":{"name":"pm25"},"coordinates":{"latitude":52.5,"longitude":13.4}}
            ]}"#,
        )
        .unwrap();
        let t = results_table(&json).unwrap();
        assert_eq!(t.names.len(), 4);
        assert_eq!(t.rows.len(), 2);
        assert_eq!(t.rows[0][0], Some(12.5));
        assert_eq!(t.rows[0][1], Some(52.5));
        assert_eq!(t.rows[0][3], Some(1_767_225_600.0));
        assert_eq!(t.rows[1][0], None);
    }

    #[test]
    fn a_result_without_parameter_is_kept() {
        let json = parse_json(
            r#"{"results":[{"value":7.0,"coordinates":{"latitude":1.0,"longitude":2.0}}]}"#,
        )
        .unwrap();
        let t = results_table(&json).unwrap();
        assert_eq!(t.rows.len(), 1);
        assert_eq!(t.rows[0][0], Some(7.0));
    }

    #[test]
    fn resolve_url_substitutes_the_template() {
        let args = vec![
            "--station".to_string(),
            "42".to_string(),
            "--yesterday".to_string(),
            "2026-01-01".to_string(),
        ];
        let url = resolve_url(&args);
        assert!(url.contains("/sensors/42/measurements"));
        assert!(url.contains("datetime_from=2026-01-01T00:00:00Z"));
    }
}
