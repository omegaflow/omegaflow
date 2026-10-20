use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::lsk::days_from_civil;

pub const COMP_OCEAN_TEMPERATURE: u32 = 1;
pub const COMP_OCEAN_SALINITY: u32 = 2;
pub const COMP_DISSOLVED_OXYGEN: u32 = 3;
pub const COMP_CHLOROPHYLL_A: u32 = 4;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_OCEAN_TEMPERATURE => Some("argo_bgc_ocean_temperature_c"),
        COMP_OCEAN_SALINITY => Some("argo_bgc_ocean_salinity_psu"),
        COMP_DISSOLVED_OXYGEN => Some("argo_bgc_dissolved_oxygen_umol_kg"),
        COMP_CHLOROPHYLL_A => Some("argo_bgc_chlorophyll_a_mg_m3"),
        _ => None,
    }
}

pub const DEFAULT_URL: &str = "https://github.com/omegaflow/sources/releases/download/data-argo.ifremer.fr/argo_bgc_synthetic_profile_index.txt";
pub const ORIGIN: &str = "https://data-argo.ifremer.fr/argo_synthetic-profile_index.txt";

#[derive(Clone, Debug, PartialEq)]
pub struct IndexRow {
    pub file: String,
    pub t: f64,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub ocean: String,
    pub profiler_type: String,
    pub institution: String,
    pub parameters: String,
    pub data_mode: String,
    pub date_update: String,
}

fn split_fields(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    for c in line.chars() {
        if c == '"' {
            quoted = !quoted;
        } else if c == ',' && !quoted {
            out.push(field.trim().to_string());
            field.clear();
        } else {
            field.push(c);
        }
    }
    out.push(field.trim().to_string());
    out
}

fn parse_f64(cell: &str) -> Option<f64> {
    cell.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn stamp_unix(stamp: &str) -> Option<f64> {
    if stamp.len() < 14 || !stamp[..14].bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let year = stamp.get(0..4)?.parse::<i64>().ok()?;
    let month = stamp.get(4..6)?.parse::<i64>().ok()?;
    let day = stamp.get(6..8)?.parse::<i64>().ok()?;
    let hour = stamp.get(8..10)?.parse::<i64>().ok()?;
    let minute = stamp.get(10..12)?.parse::<i64>().ok()?;
    let second = stamp.get(12..14)?.parse::<i64>().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0 + (hour * 3600 + minute * 60 + second) as f64)
}

pub fn parse_index(bytes: &[u8]) -> Option<Vec<IndexRow>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut header_seen = false;
    let mut rows = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if !header_seen {
            header_seen = true;
            if line.starts_with("file,date") {
                continue;
            }
        }
        let cols = split_fields(line);
        if cols.len() < 10 {
            continue;
        }
        let Some(t) = stamp_unix(&cols[1]) else {
            continue;
        };
        rows.push(IndexRow {
            file: cols[0].clone(),
            t,
            lat: parse_f64(&cols[2]),
            lon: parse_f64(&cols[3]),
            ocean: cols[4].clone(),
            profiler_type: cols[5].clone(),
            institution: cols[6].clone(),
            parameters: cols[7].clone(),
            data_mode: cols[8].clone(),
            date_update: cols[9].clone(),
        });
    }
    (!rows.is_empty()).then_some(rows)
}

fn has_parameter(row: &IndexRow, code: &str) -> bool {
    row.parameters.split_whitespace().any(|p| p == code)
}

pub fn declared(rows: &[IndexRow]) -> [(u32, &'static str, usize); 4] {
    [
        (
            COMP_OCEAN_TEMPERATURE,
            "TEMP",
            rows.iter().filter(|r| has_parameter(r, "TEMP")).count(),
        ),
        (
            COMP_OCEAN_SALINITY,
            "PSAL",
            rows.iter().filter(|r| has_parameter(r, "PSAL")).count(),
        ),
        (
            COMP_DISSOLVED_OXYGEN,
            "DOXY",
            rows.iter().filter(|r| has_parameter(r, "DOXY")).count(),
        ),
        (
            COMP_CHLOROPHYLL_A,
            "CHLA",
            rows.iter().filter(|r| has_parameter(r, "CHLA")).count(),
        ),
    ]
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load(args: &[String]) -> Option<(String, Vec<u8>)> {
    if let Some(path) = arg_value(args, "--file") {
        std::fs::read(&path).ok().map(|b| (path, b))
    } else {
        let url = match arg_value(&args, "--url") {
            Some(v) => v,
            None => DEFAULT_URL.to_string(),
        };
        fetch_raw_bytes(&url).map(|b| (url, b))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((src, bytes)) = load(&args) else {
        eprintln!("argo_index: {ORIGIN} stays unfetched");
        std::process::exit(1);
    };
    let Some(rows) = parse_index(&bytes) else {
        eprintln!("{src}: {} B carry no GDAC index rows", bytes.len());
        std::process::exit(1);
    };
    eprintln!("{src}: {} profile index row(s)", rows.len());
    for row in rows.iter().take(3) {
        eprintln!(
            "  {} t={:.0} lat={} lon={} params=[{}]",
            row.file,
            row.t,
            match row.lat {
                Some(v) => format!("{v:.3}"),
                None => String::from("absent"),
            },
            match row.lon {
                Some(v) => format!("{v:.3}"),
                None => String::from("absent"),
            },
            row.parameters
        );
    }
    for (comp, code, count) in declared(&rows) {
        eprintln!(
            "  {}: absent in index ({} row(s) declare {code})",
            component_name(comp).unwrap_or("unknown"),
            count
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAD: &[u8] = b"# Title : Synthetic-Profile directory file of the Argo Global Data Assembly Center\n\
# Format version : 2.2\n\
file,date,latitude,longitude,ocean,profiler_type,institution,parameters,parameter_data_mode,date_update\n\
aoml/1900722/profiles/SD1900722_001.nc,20061022021624,-40.316,73.389,I,846,AO,PRES TEMP PSAL DOXY,DDDD,20220628080801\n\
aoml/1900722/profiles/SD1900722_002.nc,20061101064423,-40.390,73.528,I,846,AO,PRES TEMP PSAL DOXY,DDDD,20220628080813\n";

    #[test]
    fn reads_real_gdac_head_rows() {
        let rows = parse_index(HEAD).expect("index head parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].file, "aoml/1900722/profiles/SD1900722_001.nc");
        assert!((rows[0].lat.unwrap() - -40.316).abs() < 1e-9);
        assert!((rows[0].lon.unwrap() - 73.389).abs() < 1e-9);
        assert_eq!(rows[0].t, stamp_unix("20061022021624").unwrap());
    }

    #[test]
    fn parameters_are_declared_but_never_measured() {
        let rows = parse_index(HEAD).unwrap();
        let d = declared(&rows);
        assert_eq!(d[0].2, 2); // TEMP
        assert_eq!(d[1].2, 2); // PSAL
        assert_eq!(d[2].2, 2); // DOXY
        assert_eq!(d[3].2, 0); // CHLA absent
        assert!(rows.iter().all(|r| !r.parameters.is_empty()));
    }

    #[test]
    fn absent_for_non_index_bytes() {
        assert!(parse_index(b"\x00\x01\x02").is_none());
    }
}
