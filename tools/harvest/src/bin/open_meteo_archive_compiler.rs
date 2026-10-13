use omegaflow::archivar::hapi_csv::parse_iso_seconds;
use omegaflow::archivar::json::{JsonVal, jpath_val, parse_json, scalar_of};
use omegaflow::cdn::upload_release;
use omegaflow_harvest::table::{Table, arg_or, emit, has_flag, load_source, sanitize_name, stat};

const MAGIC: [u8; 4] = *b"OMET";
const NETLOC: &str = "archive-api.open-meteo.com";
const FORMAT: &str = "open_meteo_archive";
const COMPILER: &str = "tools/harvest/src/bin/open_meteo_archive_compiler.rs";
const DEFAULT_URL: &str = "https://archive-api.open-meteo.com/v1/archive?latitude=52.52&longitude=13.405&hourly=temperature_2m,pressure_msl&start_date=2025-01-01&end_date=2025-01-02&timezone=UTC";
const DEFAULT_OUT: &str = "open_meteo_archive.bin";

fn time_cell(raw: &str) -> Option<f64> {
    let stamp = raw.trim();
    let full = if stamp.len() == 16 {
        format!("{stamp}:00")
    } else {
        stamp.to_string()
    };
    parse_iso_seconds(&full).filter(|v| v.is_finite())
}

fn hourly_table(json: &JsonVal) -> Option<Table> {
    let Some(JsonVal::Obj(hourly)) = jpath_val(json, "hourly") else {
        return None;
    };
    let time = match hourly.get("time") {
        Some(JsonVal::Arr(arr)) => arr,
        _ => return None,
    };
    let mut variables: Vec<&String> = hourly.keys().filter(|k| k.as_str() != "time").collect();
    variables.sort();
    let mut names = vec!["time".to_string()];
    for (i, key) in variables.iter().enumerate() {
        names.push(sanitize_name(key, i + 1));
    }
    if time.is_empty() {
        return None;
    }
    let mut rows = Vec::with_capacity(time.len());
    for i in 0..time.len() {
        let mut row = vec![None; names.len()];
        if let Some(JsonVal::Str(stamp)) = time.get(i) {
            row[0] = time_cell(stamp);
        }
        for (j, key) in variables.iter().enumerate() {
            if let Some(JsonVal::Arr(series)) = hourly.get(key.as_str()) {
                if let Some(value) = series.get(i) {
                    row[j + 1] = scalar_of(value).filter(|v| v.is_finite());
                }
            }
        }
        rows.push(row);
    }
    if rows.is_empty() {
        None
    } else {
        Some(Table { names, rows })
    }
}

fn selftest() {
    let body = r#"{"hourly":{"time":["2025-01-01T00:00","2025-01-01T01:00"],"temperature_2m":[1.5,null],"pressure_msl":[1013.2,1012.8]}}"#;
    let Some(json) = parse_json(body) else {
        eprintln!("selftest: the fixture parses void");
        std::process::exit(1);
    };
    let Some(table) = hourly_table(&json) else {
        eprintln!("selftest: the hourly arrays do not form a table");
        std::process::exit(1);
    };
    if table.names != vec!["time", "pressure_msl", "temperature_2m"] {
        eprintln!("selftest: the hourly keys do not name the columns deterministically");
        std::process::exit(1);
    }
    let epoch = table.rows[0][0];
    if epoch != Some(1_735_689_600.0) {
        eprintln!("selftest: a minute-resolution stamp does not read as epoch seconds");
        std::process::exit(1);
    }
    if table.rows[0][2] != Some(1.5) || table.rows[1][2].is_some() {
        eprintln!("selftest: a null hourly value does not stay absent (0 honored)");
        std::process::exit(1);
    }
    if hourly_table(&JsonVal::Null).is_some() {
        eprintln!("selftest: a body without hourly is not refused");
        std::process::exit(1);
    }
    eprintln!("open_meteo_archive_compiler: selftest passes (hourly arrays + null absence)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if has_flag(&args, "--selftest") {
        selftest();
        return;
    }
    let Some((src, bytes)) = load_source(&args, DEFAULT_URL) else {
        std::process::exit(1);
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{src}: the body is not utf8 — absent (0 honored)");
        std::process::exit(1);
    };
    let Some(json) = parse_json(text) else {
        eprintln!("{src}: the body carries no JSON — absent (0 honored)");
        std::process::exit(1);
    };
    let Some(table) = hourly_table(&json) else {
        eprintln!("{src}: no hourly series survived — absent (0 honored)");
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
    fn hourly_arrays_become_columns_and_null_stays_absent() {
        let json = parse_json(
            r#"{"hourly":{"time":["2025-01-01T00:00","2025-01-01T01:00"],"temperature_2m":[1.5,null],"pressure_msl":[1013.2,1012.8]}}"#,
        )
        .unwrap();
        let t = hourly_table(&json).unwrap();
        assert_eq!(t.names, vec!["time", "pressure_msl", "temperature_2m"]);
        assert_eq!(t.rows.len(), 2);
        assert_eq!(t.rows[0][0], Some(1_735_689_600.0));
        assert_eq!(t.rows[0][1], Some(1013.2));
        assert_eq!(t.rows[0][2], Some(1.5));
        assert_eq!(t.rows[1][2], None);
    }

    #[test]
    fn time_cell_reads_minutes_and_seconds() {
        assert_eq!(time_cell("2025-01-01T00:00"), Some(1_735_689_600.0));
        assert_eq!(time_cell("2025-01-01T00:00:00"), Some(1_735_689_600.0));
        assert_eq!(time_cell("nonsense"), None);
    }

    #[test]
    fn a_body_without_hourly_is_refused() {
        let json = parse_json(r#"{"latitude":52.52}"#).unwrap();
        assert!(hourly_table(&json).is_none());
    }
}
