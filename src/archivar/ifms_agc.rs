use super::units::ymd_to_days;

pub const MAGIC: [u8; 4] = *b"IFMS";
pub const COMP_CARRIER_LEVEL: u32 = 1;
pub const COMP_POLAR_ANGLE: u32 = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct IfmsAgcFile {
    pub fields: Vec<(String, String)>,
    pub config: Vec<(String, String)>,
    pub samples: Vec<IfmsAgcSample>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IfmsAgcSample {
    pub unix_time: f64,
    pub carrier_level_dbm: f64,
    pub polar_angle_cycles: f64,
}

fn header_fields(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if !t.starts_with('<') || t.starts_with("</") {
            continue;
        }
        let Some(gt) = t.find('>') else {
            continue;
        };
        let name = &t[1..gt];
        if name.is_empty() {
            continue;
        }
        let open = format!("<{name}>");
        let close = format!("</{name}>");
        let Some(rest) = t.strip_prefix(&open) else {
            continue;
        };
        let Some(value) = rest.strip_suffix(&close) else {
            continue;
        };
        out.push((name.to_string(), value.trim().to_string()));
    }
    out
}

fn config_table(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut in_table = false;
    for line in text.lines() {
        let t = line.trim();
        if t == "<active_table>" {
            in_table = true;
            continue;
        }
        if t == "</active_table>" {
            break;
        }
        if !in_table {
            continue;
        }
        let Some((key, rest)) = t.split_once('=') else {
            continue;
        };
        let key = key.trim().to_string();
        if key.is_empty() {
            continue;
        }
        let value = match rest.split(';').next() {
            Some(v) => v.trim().trim_matches('"').to_string(),
            None => continue,
        };
        out.push((key, value));
    }
    out
}

fn column_indices(comment: &str) -> Option<(usize, usize, usize)> {
    let names: Vec<&str> = comment
        .trim_start_matches("//")
        .split_whitespace()
        .collect();
    let time = names.iter().position(|n| *n == "SampleTime")?;
    let carrier = names.iter().position(|n| *n == "CarrierLevel")?;
    let polar = names.iter().position(|n| *n == "PolarAngle")?;
    Some((time, carrier, polar))
}

fn parse_sample_time(s: &str) -> Option<f64> {
    let mut parts = s.split('.');
    let date = parts.next()?;
    let time = parts.next()?;
    let frac = parts.next();
    if date.len() != 8 || time.len() != 6 || !date.is_ascii() || !time.is_ascii() {
        return None;
    }
    let year = date[0..4].parse::<i64>().ok()?;
    let month = date[4..6].parse::<u32>().ok()?;
    let day = date[6..8].parse::<u32>().ok()?;
    let hour = time[0..2].parse::<f64>().ok()?;
    let minute = time[2..4].parse::<f64>().ok()?;
    let second = time[4..6].parse::<f64>().ok()?;
    let days = ymd_to_days(year, month, day)? as f64;
    let mut unix = days * 86400.0 + hour * 3600.0 + minute * 60.0 + second;
    if let Some(f) = frac {
        if !f.is_empty() {
            let digits = f.parse::<f64>().ok()?;
            unix += digits / 10f64.powi(f.len() as i32);
        }
    }
    if unix.is_finite() { Some(unix) } else { None }
}

pub fn parse_ifms_agc(bytes: &[u8]) -> Option<IfmsAgcFile> {
    let text = std::str::from_utf8(bytes).ok()?;
    let header_open = "<header>";
    let header_close = "</header>";
    let body_open = "<body_Gain>";
    let body_close = "</body_Gain>";

    let hstart = text.find(header_open)?;
    let hend = text.find(header_close)?;
    let header_text = &text[hstart + header_open.len()..hend];

    let bstart = text.find(body_open)?;
    let bend = text.find(body_close)?;
    let body_text = &text[bstart + body_open.len()..bend];

    let fields = header_fields(header_text);
    let config = config_table(header_text);

    let mut indices: Option<(usize, usize, usize)> = None;
    let mut samples: Vec<IfmsAgcSample> = Vec::new();
    for line in body_text.lines() {
        let t = line.trim();
        if t.starts_with("//") {
            indices = column_indices(t);
            continue;
        }
        if t.is_empty() {
            continue;
        }
        let Some((ti, ci, pi)) = indices else {
            continue;
        };
        let toks: Vec<&str> = t.split_whitespace().collect();
        let Some(unix_time) = toks.get(ti).copied().and_then(parse_sample_time) else {
            continue;
        };
        let Some(level) = toks.get(ci).copied().and_then(|s| s.parse::<f64>().ok()) else {
            continue;
        };
        let Some(polar) = toks.get(pi).copied().and_then(|s| s.parse::<f64>().ok()) else {
            continue;
        };
        if !level.is_finite() || !polar.is_finite() {
            continue;
        }
        samples.push(IfmsAgcSample {
            unix_time,
            carrier_level_dbm: level,
            polar_angle_cycles: polar,
        });
    }
    if samples.is_empty() {
        return None;
    }
    Some(IfmsAgcFile {
        fields,
        config,
        samples,
    })
}

pub fn write_series(samples: &[(f64, f64, f64)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + samples.len() * 24);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(samples.len() as u32).to_le_bytes());
    for (t, level, polar) in samples {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&level.to_le_bytes());
        out.extend_from_slice(&polar.to_le_bytes());
    }
    out
}

pub fn parse_series(data: &[u8]) -> Option<Vec<(f64, f64, f64)>> {
    if data.len() < 8 || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 24 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 24;
        let t = f64::from_le_bytes(data.get(base..base + 8)?.try_into().ok()?);
        let level = f64::from_le_bytes(data.get(base + 8..base + 16)?.try_into().ok()?);
        let polar = f64::from_le_bytes(data.get(base + 16..base + 24)?.try_into().ok()?);
        if !t.is_finite() || !level.is_finite() || !polar.is_finite() {
            return None;
        }
        out.push((t, level, polar));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str = "\
<header>
<station_id>NN11</station_id>
<spacecraft_id>ROSE</spacecraft_id>
<total_samples>3</total_samples>
<sample_period>1.</sample_period>
<first_sample_time>20141217.033415.000</first_sample_time>
<active_table>
   UlmMode = \"Normal\" ; // 
   UlmCarNomLvl = 4 ; // dBm
</active_table>
</header>
<body_Gain>\n// Number SampleTime CarrierLevel PolarAngle IncohAgcGain
1 20141217.033415.000 -86.4 0.0680 31.0
2 20141217.033416.000 -87.2 0.0762 31.3
3 20141217.033417.000 -88.4 0.1063 31.3
</body_Gain>
";

    #[test]
    fn parses_header_and_agc_samples() {
        let file = parse_ifms_agc(RAW.as_bytes()).unwrap();
        assert_eq!(file.fields.len(), 5);
        assert_eq!(
            file.fields[0],
            ("station_id".to_string(), "NN11".to_string())
        );
        assert_eq!(file.config.len(), 2);
        assert_eq!(
            file.config[0],
            ("UlmMode".to_string(), "Normal".to_string())
        );
        assert_eq!(file.samples.len(), 3);
        assert!((file.samples[0].unix_time - 1_418_787_255.0).abs() < 1e-6);
        assert!((file.samples[1].unix_time - file.samples[0].unix_time - 1.0).abs() < 1e-9);
        assert!((file.samples[0].carrier_level_dbm + 86.4).abs() < 1e-9);
        assert!((file.samples[0].polar_angle_cycles - 0.0680).abs() < 1e-9);
    }

    #[test]
    fn parses_committed_rsi_fixture() {
        let bytes = std::fs::read("src/archivar/kernels/r32icl1l1a_ag1_072831317_02.RAW").unwrap();
        let file = parse_ifms_agc(&bytes).unwrap();
        assert_eq!(
            file.fields[0],
            ("station_id".to_string(), "NN11".to_string())
        );
        assert_eq!(file.samples.len(), 1994);
        assert!((file.samples[0].carrier_level_dbm + 43.6).abs() < 1e-9);
        assert!((file.samples[0].polar_angle_cycles - 0.2485).abs() < 1e-9);
        assert!((file.samples[1].unix_time - file.samples[0].unix_time - 1.0).abs() < 1e-9);
    }

    #[test]
    fn rejects_non_ifms_text() {
        assert!(parse_ifms_agc(b"").is_none());
        assert!(parse_ifms_agc(b"hello\nworld\n").is_none());
        assert!(parse_series(b"X").is_none());
    }

    #[test]
    fn series_roundtrip() {
        let samples = vec![(1.5e9, -86.4, 0.068), (1.5e9 + 1.0, -87.2, 0.076)];
        let bytes = write_series(&samples);
        let parsed = parse_series(&bytes).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], (1.5e9, -86.4, 0.068));
        assert_eq!(parsed[1], (1.5e9 + 1.0, -87.2, 0.076));
    }
}
