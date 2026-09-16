use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json, scalar_of};

const NETLOC: &str = "erddap.emso.eu";
const OBSEA_URL: &str = "https://erddap.emso.eu/erddap/tabledap/OBSEA_seabed_station_TS_L1c.json?time,latitude,longitude,depth,TEMP,PSAL,PRES,CNDC&time%3E=max(time)-7days";
const DEFAULT_OUT: &str = "emso_obsea_seabed_ts.json";
const TTL: u64 = 3600;
const COLUMNS: [&str; 8] = [
    "time", "latitude", "longitude", "depth", "TEMP", "PSAL", "PRES", "CNDC",
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn cell_number(cell: &JsonVal) -> Option<f64> {
    scalar_of(cell).filter(|v| v.is_finite())
}

fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_number(value: Option<f64>) -> String {
    match value {
        Some(v) => format!("{v}"),
        None => "null".to_string(),
    }
}

struct StationRow {
    time: String,
    cells: [Option<f64>; 7],
}

fn compile_table(body: &str) -> Option<Vec<StationRow>> {
    let parsed = parse_json(body)?;
    let JsonVal::Obj(top) = parsed else {
        return None;
    };
    let JsonVal::Obj(table) = top.get("table")? else {
        return None;
    };
    let names: Vec<String> = match table.get("columnNames") {
        Some(JsonVal::Arr(a)) => a
            .iter()
            .map(|c| match c {
                JsonVal::Str(s) => s.clone(),
                _ => String::new(),
            })
            .collect(),
        _ => return None,
    };
    let index: Vec<usize> = COLUMNS
        .iter()
        .map(|name| names.iter().position(|n| n == name))
        .collect::<Option<Vec<usize>>>()?;
    let rows = match table.get("rows") {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let mut out: Vec<StationRow> = Vec::new();
    for row in rows {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        let time = match cells.get(index[0]) {
            Some(JsonVal::Str(s)) if !s.is_empty() => s.clone(),
            _ => continue,
        };
        let mut values: [Option<f64>; 7] = [None; 7];
        for (slot, col) in values.iter_mut().zip(index.iter().skip(1)) {
            *slot = cells.get(*col).and_then(cell_number);
        }
        let (Some(latitude), Some(longitude)) = (values[0], values[1]) else {
            continue;
        };
        if !(-90.0..=90.0).contains(&latitude) || !(-180.0..=180.0).contains(&longitude) {
            continue;
        }
        out.push(StationRow { time, cells: values });
    }
    out.sort_by(|a, b| a.time.cmp(&b.time));
    Some(out)
}

fn render_json(rows: &[StationRow]) -> String {
    let mut out = String::from("{\"table\":{\"columnNames\":[");
    for (i, name) in COLUMNS.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&json_string(name));
    }
    out.push_str("],\"rows\":[");
    for (i, row) in rows.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push('[');
        out.push_str(&json_string(&row.time));
        for cell in &row.cells {
            out.push(',');
            out.push_str(&json_number(*cell));
        }
        out.push(']');
    }
    out.push_str("]}}");
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let body = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => {
            let url = match arg_value(&args, "--url") {
                Some(v) => v,
                None => OBSEA_URL.to_string(),
            };
            match fetch_raw_bytes(&url, TTL) {
                Some(b) => match String::from_utf8(b) {
                    Ok(s) => s,
                    Err(_) => {
                        eprintln!("{url}: response not utf8");
                        std::process::exit(1);
                    }
                },
                None => {
                    eprintln!("{url}: fetch returned void");
                    std::process::exit(1);
                }
            }
        }
    };
    let Some(rows) = compile_table(&body) else {
        eprintln!(
            "emso: the response carries no table.columnNames/rows — the asset stays unwritten"
        );
        std::process::exit(1);
    };
    if rows.is_empty() {
        eprintln!(
            "emso: no row carries a measured position — the asset stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let json = render_json(&rows);
    if let Err(e) = std::fs::write(&out, &json) {
        eprintln!("write {out} returned void: {e}");
        std::process::exit(1);
    }
    let first = rows.first().map(|r| r.time.as_str()).unwrap_or("");
    let last = rows.last().map(|r| r.time.as_str()).unwrap_or("");
    eprintln!("emso: {} rows, {first} .. {last} → {out}", rows.len());
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("upload_release for {out} returned void");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE: &str = r#"{"table":{"columnNames":["time","latitude","longitude","depth","TEMP","PSAL","PRES","CNDC"],"rows":[["2026-09-16T00:00:00Z",41.182,1.752,20.0,18.5,37.2,2.1,4.5],["2026-09-15T23:30:00Z",41.182,1.752,20.0,"NaN",37.2,null,4.5]]}}"#;

    #[test]
    fn table_compiles_in_time_order_and_honors_absent() {
        let rows = compile_table(TABLE).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].time, "2026-09-15T23:30:00Z");
        assert_eq!(rows[0].cells[3], None);
        assert_eq!(rows[0].cells[5], None);
        assert!(rows[0].cells[0].is_some());
    }

    #[test]
    fn row_without_position_is_skipped() {
        let body = r#"{"table":{"columnNames":["time","latitude","longitude","depth","TEMP","PSAL","PRES","CNDC"],"rows":[["2026-09-16T00:00:00Z",null,1.752,20.0,18.5,37.2,2.1,4.5]]}}"#;
        assert!(compile_table(body).unwrap().is_empty());
    }
}
