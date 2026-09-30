use crate::archivar::json::{JsonVal, parse_json};
use crate::lsk::{LeapSeconds, days_from_civil};
use std::collections::HashMap;

pub const MAGIC: [u8; 4] = *b"HCV1";
pub const HEADER_BYTES: usize = 12;
pub const RECORD_BYTES: usize = 20;

#[derive(Clone, Debug, PartialEq)]
pub struct HapiParam {
    pub name: String,
    pub units: String,
    pub fill: Option<f64>,
    pub positive: bool,
}

pub fn write_bin(nchan: u32, records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&nchan.to_le_bytes());
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, val, comp) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&comp.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let nchan = u32::from_le_bytes(bytes[4..8].try_into().ok()?);
    let n = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    if nchan == 0 || n > (bytes.len() - HEADER_BYTES) / RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let val = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let comp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if comp == 0 || comp > nchan {
            return None;
        }
        out.push((t, val, comp));
    }
    Some(out)
}

pub fn parse_info(text: &str) -> Option<Vec<HapiParam>> {
    let JsonVal::Obj(root) = parse_json(text)? else {
        return None;
    };
    let JsonVal::Arr(list) = root.get("parameters")? else {
        return None;
    };
    let mut out = Vec::new();
    for p in list {
        let JsonVal::Obj(entry) = p else {
            continue;
        };
        let Some(JsonVal::Str(name)) = entry.get("name") else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        let units = match entry.get("units") {
            Some(JsonVal::Str(u)) => u.clone(),
            _ => String::new(),
        };
        let fill = match entry.get("fill") {
            Some(JsonVal::Num(n)) if n.is_finite() => Some(*n),
            Some(JsonVal::Str(s)) => s.parse::<f64>().ok().filter(|v| v.is_finite()),
            _ => None,
        };
        out.push(HapiParam {
            name: name.clone(),
            units,
            fill,
            positive: false,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

fn time_like(cell: &str) -> bool {
    cell.to_ascii_lowercase().starts_with("time")
}

pub fn csv_header_fields(csv: &str) -> Option<Vec<String>> {
    for line in csv.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some(body) = trimmed.strip_prefix('#') else {
            break;
        };
        let body = body.trim();
        let body = body.strip_prefix("parameters:").unwrap_or(body).trim();
        let cells: Vec<&str> = body.split(',').map(str::trim).collect();
        if cells.len() >= 2 && time_like(cells[0]) {
            return Some(cells.into_iter().map(str::to_string).collect());
        }
    }
    None
}

pub fn parse_iso_seconds(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 19 {
        return None;
    }
    let year: i64 = s.get(0..4)?.parse().ok()?;
    let month: i64 = s.get(5..7)?.parse().ok()?;
    let day: i64 = s.get(8..10)?.parse().ok()?;
    let hour: i64 = s.get(11..13)?.parse().ok()?;
    let minute: i64 = s.get(14..16)?.parse().ok()?;
    let second: i64 = s.get(17..19)?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let mut t = days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + second as f64;
    if b.get(19) == Some(&b'.') {
        let mut scale = 10.0;
        for &c in &b[20..] {
            if !c.is_ascii_digit() {
                break;
            }
            t += (c - b'0') as f64 / scale;
            scale *= 10.0;
        }
    }
    Some(t)
}

pub fn series_from_csv(csv: &str, params: &[HapiParam], lsk: &LeapSeconds) -> Vec<(f64, f64, u32)> {
    let nchan = params.len().saturating_sub(1) as u32;
    let mut out = Vec::new();
    for line in csv.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = trimmed.split(',').map(str::trim).collect();
        if cells.len() < 2 {
            continue;
        }
        let Some(t_unix) = parse_iso_seconds(cells[0]) else {
            continue;
        };
        let Some(t) = lsk.unix_to_tdb(t_unix) else {
            continue;
        };
        for (i, cell) in cells.iter().enumerate().skip(1) {
            let comp = i as u32;
            if comp > nchan {
                break;
            }
            let Ok(v) = cell.parse::<f64>() else {
                continue;
            };
            if !v.is_finite() {
                continue;
            }
            let Some(param) = params.get(i) else {
                continue;
            };
            if let Some(fill) = param.fill
                && v == fill
            {
                continue;
            }
            if param.positive && v <= 0.0 {
                continue;
            }
            out.push((t, v, comp));
        }
    }
    out
}

pub fn csv_envelope(body: &str, names: &[String], max_rows: Option<usize>) -> Option<JsonVal> {
    let mut rows: Vec<JsonVal> = Vec::new();
    let mut width = 0usize;
    for line in body.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = t.split(',').map(str::trim).collect();
        if width == 0 {
            if cells.len() < 2 {
                return None;
            }
            width = cells.len();
        } else if cells.len() != width {
            break;
        }
        let row: Vec<JsonVal> = cells
            .iter()
            .map(|c| match c.parse::<f64>() {
                Ok(v) if v.is_finite() => JsonVal::Num(v),
                _ => JsonVal::Str(c.to_string()),
            })
            .collect();
        rows.push(JsonVal::Arr(row));
        if max_rows.is_some_and(|m| rows.len() >= m) {
            break;
        }
    }
    if rows.is_empty() {
        return None;
    }
    let mut root = HashMap::new();
    if !names.is_empty() {
        let params: Vec<JsonVal> = names
            .iter()
            .map(|n| {
                let mut entry = HashMap::new();
                entry.insert("name".to_string(), JsonVal::Str(n.clone()));
                JsonVal::Obj(entry)
            })
            .collect();
        root.insert("parameters".to_string(), JsonVal::Arr(params));
    }
    root.insert("data".to_string(), JsonVal::Arr(rows));
    Some(JsonVal::Obj(root))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lsk() -> LeapSeconds {
        LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(32.0, 0.0)],
        }
    }

    #[test]
    fn roundtrip() {
        let records = vec![
            (-946684800.0, 452.15, 1),
            (-946684800.0, 344.2, 2),
            (-946684740.0, 452.3, 1),
        ];
        let bytes = write_bin(4, &records);
        assert_eq!(parse_bin(&bytes), Some(records));
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"HCV1abc").is_none());
    }

    #[test]
    fn rejects_component_past_channel_count() {
        assert!(parse_bin(&write_bin(2, &[(1.0, 1.0, 3)])).is_none());
        assert!(parse_bin(&write_bin(2, &[(1.0, 1.0, 0)])).is_none());
    }

    #[test]
    fn parses_measured_info_table() {
        let text = "{\"HAPI\":\"1.1\",\"parameters\":[{\"name\":\"time\",\"type\":\"isotime\",\"units\":\"UTC\"},{\"name\":\"B_mag\",\"type\":\"double\",\"units\":\"nT\",\"fill\":null}]}";
        let params = parse_info(text).expect("measured DAS2 info parses");
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "time");
        assert_eq!(params[1].name, "B_mag");
        assert_eq!(params[1].units, "nT");
        assert_eq!(params[1].fill, None);
    }

    #[test]
    fn reads_csv_header_names_when_present() {
        let csv = "# HAPI 1.1\n# parameters: time, B_mag\n2016-12-11T13:00:30.000, 4.5215e+02\n";
        let names = csv_header_fields(csv).expect("header fields parse");
        assert_eq!(names, vec!["time", "B_mag"]);
    }

    #[test]
    fn measured_stream_without_header_reads_by_info_order() {
        let csv = "2016-12-11T13:00:30.000, 4.5215e+02\n2016-12-11T13:01:30.000, 4.5482e+02\n";
        assert!(csv_header_fields(csv).is_none());
        let params = vec![
            HapiParam {
                name: "time".into(),
                units: "UTC".into(),
                fill: None,
                positive: false,
            },
            HapiParam {
                name: "B_mag".into(),
                units: "nT".into(),
                fill: None,
                positive: false,
            },
        ];
        let rows = series_from_csv(csv, &params, &lsk());
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].2, 1);
        assert!((rows[0].1 - 452.15).abs() < 1e-9);
        assert!(rows[1].0 > rows[0].0);
    }

    #[test]
    fn fill_value_stays_absent() {
        let csv = "2016-12-11T13:00:30.000, 1.0, 99999.0\n";
        let params = vec![
            HapiParam {
                name: "time".into(),
                units: "UTC".into(),
                fill: None,
                positive: false,
            },
            HapiParam {
                name: "a".into(),
                units: "nT".into(),
                fill: None,
                positive: false,
            },
            HapiParam {
                name: "b".into(),
                units: "nT".into(),
                fill: Some(99999.0),
                positive: false,
            },
        ];
        let rows = series_from_csv(csv, &params, &lsk());
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].2, 1);
    }

    #[test]
    fn positive_channel_drops_non_positive_values_keeps_signed_channels() {
        let csv = "2016-12-11T13:00:30.000, -5.0, -31.0\n2016-12-11T13:01:30.000, 162.5, -30.5\n";
        let params = vec![
            HapiParam {
                name: "time".into(),
                units: "UTC".into(),
                fill: None,
                positive: false,
            },
            HapiParam {
                name: "magnitude".into(),
                units: "nT".into(),
                fill: None,
                positive: true,
            },
            HapiParam {
                name: "X".into(),
                units: "nT".into(),
                fill: None,
                positive: false,
            },
        ];
        let rows = series_from_csv(csv, &params, &lsk());
        let vals: Vec<(f64, u32)> = rows.iter().map(|(_, v, c)| (*v, *c)).collect();
        assert!(vals.contains(&(162.5, 1)));
        assert!(vals.contains(&(-31.0, 2)));
        assert!(vals.contains(&(-30.5, 2)));
        assert_eq!(vals.iter().filter(|(_, c)| *c == 1).count(), 1);
    }

    #[test]
    fn envelope_builds_columnar_shape_with_names_and_absent_cells() {
        let body = "2008-09-10T00:00:00.500, 1.62546e+02,\n2008-09-10T00:00:01.500, 1.62537e+02,-3.10e+01\n";
        let names = vec!["time".into(), "magnitude".into(), "X".into()];
        let JsonVal::Obj(root) = csv_envelope(body, &names, None).expect("envelope parses") else {
            panic!("envelope is not an object");
        };
        let JsonVal::Arr(data) = root.get("data").expect("data present") else {
            panic!("data is not an array");
        };
        assert_eq!(data.len(), 2);
        let JsonVal::Arr(row) = &data[0] else {
            panic!("row is not an array");
        };
        assert!(matches!(row[0], JsonVal::Str(_)));
        assert!(matches!(row[1], JsonVal::Num(v) if v > 0.0));
        assert!(matches!(row[2], JsonVal::Str(ref s) if s.is_empty()));
        let JsonVal::Arr(params) = root.get("parameters").expect("parameters present") else {
            panic!("parameters is not an array");
        };
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn envelope_stops_at_ragged_row_and_reads_headers_when_present() {
        let body = "# parameters: time, B_mag\n2016-12-11T13:00:30.000, 452.15\n2016-12-11T13:01:30.000, 454.82, 1.0\n";
        let JsonVal::Obj(root) = csv_envelope(body, &[], None).expect("envelope parses") else {
            panic!("envelope is not an object");
        };
        let JsonVal::Arr(data) = root.get("data").expect("data present") else {
            panic!("data is not an array");
        };
        assert_eq!(data.len(), 1);
        assert!(!root.contains_key("parameters"));
    }

    #[test]
    fn fractional_seconds_extract() {
        let whole = parse_iso_seconds("2016-12-11T13:00:30.000").expect("whole second parses");
        let frac = parse_iso_seconds("2016-12-11T13:00:30.500").expect("fraction parses");
        assert!((frac - whole - 0.5).abs() < 1e-9);
    }
}
