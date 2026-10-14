use omegaflow::archivar::quakeml::parse_quakeml;
use omegaflow_harvest::table::{Table, arg_or, emit, has_flag, load_source, stat};

const MAGIC: [u8; 4] = *b"FDSN";
const NETLOC: &str = "geofon.gfz-potsdam.de";
const FORMAT: &str = "fdsn_event";
const COMPILER: &str = "tools/harvest/src/bin/fdsn_event_compiler.rs";
const DEFAULT_URL: &str =
    "https://geofon.gfz-potsdam.de/fdsnws/event/1/query?format=xml&limit=20000";
const DEFAULT_OUT: &str = "fdsn_event.bin";

const COLUMNS: &[&str] = &["time", "latitude", "longitude", "depth_m", "magnitude"];

fn gather(body: &str) -> Option<Table> {
    let events = parse_quakeml(body);
    if events.is_empty() {
        eprintln!("fdsn_event: no event parsed — absent (0 honored)");
        return None;
    }
    let names: Vec<String> = COLUMNS.iter().map(|n| (*n).to_string()).collect();
    let mut rows = Vec::with_capacity(events.len());
    for e in &events {
        let time = if e.time.is_finite() {
            Some(e.time)
        } else {
            None
        };
        let lat = if e.lat.is_finite() { Some(e.lat) } else { None };
        let lon = if e.lon.is_finite() { Some(e.lon) } else { None };
        let depth_m = if e.depth_km.is_finite() {
            Some(e.depth_km * 1000.0)
        } else {
            None
        };
        let magnitude = e.magnitude.filter(|v| v.is_finite());
        rows.push(vec![time, lat, lon, depth_m, magnitude]);
    }
    Some(Table { names, rows })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((src, bytes)) = load_source(&args, DEFAULT_URL) else {
        std::process::exit(1);
    };
    let Ok(body) = std::str::from_utf8(&bytes) else {
        eprintln!(
            "{src}: {} B carry no utf8 QuakeML — absent (0 honored)",
            bytes.len()
        );
        std::process::exit(1);
    };
    let Some(out_table) = gather(body) else {
        std::process::exit(1);
    };
    if has_flag(&args, "--stat") {
        stat(&src, &out_table);
        return;
    }
    let out = arg_or(&args, "--out", DEFAULT_OUT);
    let netloc = arg_or(&args, "--netloc", NETLOC);
    if !emit(&src, &out, &out_table, MAGIC, &netloc, FORMAT, COMPILER) {
        std::process::exit(1);
    }
    if has_flag(&args, "--ci-mode") && !omegaflow::cdn::upload_release(&netloc, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const QUAKEML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<q:quakeml xmlns:q="http://quakeml.org/xmlns/quakeml/1.2" xmlns="http://quakeml.org/xmlns/bed/1.2">
  <eventParameters publicID="smi:geofon/bulletin">
    <event publicID="smi:geofon/evid=1">
      <origin publicID="smi:geofon/origid=1">
        <time><value>2024-01-01T07:10:10.41Z</value></time>
        <latitude><value>37.4747</value></latitude>
        <longitude><value>137.3070</value></longitude>
        <depth><value>9704.0</value></depth>
      </origin>
      <magnitude publicID="smi:geofon/magid=1">
        <mag><value>6.0</value></mag>
        <type>mb</type>
      </magnitude>
    </event>
    <event publicID="smi:geofon/evid=2">
      <origin publicID="smi:geofon/origid=2">
        <time><value>2024-01-01T07:06:05.77Z</value></time>
        <latitude><value>37.4929</value></latitude>
        <longitude><value>137.2624</value></longitude>
        <depth><value>10742.6</value></depth>
      </origin>
    </event>
  </eventParameters>
</q:quakeml>"#;

    #[test]
    fn gather_binds_the_five_numeric_columns() {
        let table = gather(QUAKEML).expect("gather");
        assert_eq!(table.names.len(), COLUMNS.len());
        assert_eq!(table.names[0], "time");
        assert_eq!(table.names[3], "depth_m");
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0][1], Some(37.4747));
        let depth = table.rows[0][3].expect("depth present");
        assert!((depth - 9704.0).abs() < 1e-6);
        assert_eq!(table.rows[0][4], Some(6.0));
        assert_eq!(table.rows[1][4], None);
    }

    #[test]
    fn an_empty_document_stays_absent() {
        assert!(gather("<q:quakeml></q:quakeml>").is_none());
    }
}
