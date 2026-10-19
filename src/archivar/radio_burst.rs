use crate::archivar::json::{JsonVal, jstr, parse_json};
use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};
use crate::lsk::days_from_civil;

pub const MAGIC: [u8; 4] = *b"SWRB";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 8 + 1 + 5 * 8;

pub const COMP_FREQ_LOW: u32 = 0;
pub const COMP_FREQ_HIGH: u32 = 1;
pub const COMP_DURATION: u32 = 2;
pub const COMP_TYPE: u32 = 3;
pub const COMP_SUBTYPE: u32 = 4;

const COLUMNS: &[(u32, &str, &str)] = &[
    (COMP_FREQ_LOW, "swpc_radio_burst_freq_low_mhz", "MHz"),
    (COMP_FREQ_HIGH, "swpc_radio_burst_freq_high_mhz", "MHz"),
    (COMP_DURATION, "swpc_radio_burst_duration_s", "s"),
    (COMP_TYPE, "swpc_radio_burst_type_code", "1"),
    (COMP_SUBTYPE, "swpc_radio_burst_subtype_code", "1"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct BurstRecord {
    pub t_unix: f64,
    pub freq_low_mhz: Option<f64>,
    pub freq_high_mhz: Option<f64>,
    pub duration_s: Option<f64>,
    pub type_code: Option<f64>,
    pub subtype_code: Option<f64>,
}

impl BurstRecord {
    fn values(&self) -> [Option<f64>; 5] {
        [
            self.freq_low_mhz,
            self.freq_high_mhz,
            self.duration_s,
            self.type_code,
            self.subtype_code,
        ]
    }
}

fn parse_iso_unix(s: &str) -> Option<f64> {
    let (date, time) = s.split_once('T')?;
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let mo: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let days = days_from_civil(y, mo, d)?;
    let time = time.trim_end_matches('Z');
    let mut tp = time.split(':');
    let h: i64 = tp.next()?.parse().ok()?;
    let mi: i64 = tp.next()?.parse().ok()?;
    let sec: f64 = tp.next()?.parse().ok()?;
    if !(0.0..60.0).contains(&sec) {
        return None;
    }
    Some((days * 86_400 + h * 3600 + mi * 60) as f64 + sec)
}

fn roman_type(s: &str) -> Option<f64> {
    let head = s.split('/').next()?.trim().to_ascii_uppercase();
    let v = match head.as_str() {
        "I" => 1.0,
        "II" => 2.0,
        "III" => 3.0,
        "IV" => 4.0,
        "V" => 5.0,
        _ => head.parse::<f64>().ok()?,
    };
    Some(v)
}

fn subtype_of(s: &str) -> Option<f64> {
    let (_, tail) = s.split_once('/')?;
    tail.trim().parse::<f64>().ok().filter(|v| v.is_finite())
}

fn parse_frequency(s: &str) -> (Option<f64>, Option<f64>) {
    let s = s.trim();
    if let Some((lo, hi)) = s.split_once('-') {
        let lo = lo.trim().parse::<f64>().ok().filter(|v| v.is_finite());
        let hi = hi.trim().parse::<f64>().ok().filter(|v| v.is_finite());
        (lo, hi)
    } else {
        (s.parse::<f64>().ok().filter(|v| v.is_finite()), None)
    }
}

pub fn parse_text(text: &str) -> Vec<BurstRecord> {
    let Some(json) = parse_json(text) else {
        return Vec::new();
    };
    let JsonVal::Arr(events) = json else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for event in &events {
        if jstr(event, "type").as_deref() != Some("RSP") {
            continue;
        }
        let Some(begin) = jstr(event, "begin_datetime") else {
            continue;
        };
        let Some(t_unix) = parse_iso_unix(&begin) else {
            continue;
        };
        let (freq_low_mhz, freq_high_mhz) = jstr(event, "frequency")
            .map(|s| parse_frequency(&s))
            .unwrap_or((None, None));
        let duration_s = match (
            Some(t_unix),
            jstr(event, "end_datetime").and_then(|s| parse_iso_unix(&s)),
        ) {
            (Some(start), Some(end)) if end >= start => Some(end - start),
            _ => None,
        };
        let burst_type = jstr(event, "particulars1");
        let type_code = burst_type.as_deref().and_then(roman_type);
        let subtype_code = burst_type.as_deref().and_then(subtype_of);
        out.push(BurstRecord {
            t_unix,
            freq_low_mhz,
            freq_high_mhz,
            duration_s,
            type_code,
            subtype_code,
        });
    }
    out
}

pub fn write_bin(records: &[BurstRecord]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.t_unix.to_le_bytes());
        let mut present = 0u8;
        let mut vals = [0.0f64; 5];
        for (i, v) in r.values().iter().enumerate() {
            if let Some(x) = v {
                present |= 1 << i;
                vals[i] = *x;
            }
        }
        buf.push(present);
        for x in vals {
            buf.extend_from_slice(&x.to_le_bytes());
        }
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<BurstRecord>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + n * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let t_unix = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let present = *bytes.get(off)?;
        off += 1;
        if !t_unix.is_finite() || present & !0b1_1111 != 0 {
            return None;
        }
        let mut vals = [None; 5];
        for (i, slot) in vals.iter_mut().enumerate() {
            let raw = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
            if present & (1 << i) != 0 {
                if !raw.is_finite() {
                    return None;
                }
                *slot = Some(raw);
            }
        }
        out.push(BurstRecord {
            t_unix,
            freq_low_mhz: vals[0],
            freq_high_mhz: vals[1],
            duration_s: vals[2],
            type_code: vals[3],
            subtype_code: vals[4],
        });
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(channel_ref) = force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(channel_ref) else {
        return Vec::new();
    };
    COLUMNS
        .iter()
        .map(|(_, name, unit)| FieldConfig {
            key: name.to_string(),
            name: name.to_string(),
            band_id: None,
            kernel,
            channel_ref: Some(channel_ref),
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: (*unit).to_string(),
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: Aperture::None,
        })
        .collect()
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let records = parse_bin(bytes)?;
    let lsk = crate::archivar::embedded_lsk()?;
    let mut out = Vec::new();
    for r in records {
        let Some(t) = lsk.unix_to_tdb(r.t_unix) else {
            continue;
        };
        for (comp, v) in [
            (COMP_FREQ_LOW, r.freq_low_mhz),
            (COMP_FREQ_HIGH, r.freq_high_mhz),
            (COMP_DURATION, r.duration_s),
            (COMP_TYPE, r.type_code),
            (COMP_SUBTYPE, r.subtype_code),
        ] {
            if let Some(v) = v {
                out.push((t, v, comp));
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"[
      {"begin_datetime":"2026-10-10T20:40:24","end_datetime":"2026-10-10T20:40:42","type":"RSP","frequency":"025-074","particulars1":"III/1"},
      {"begin_datetime":"2026-10-10T20:12:48","end_datetime":"2026-10-10T20:13:06","type":"RSP","frequency":"025-180","particulars1":"III/2"},
      {"begin_datetime":"2026-10-10T19:00:00","end_datetime":null,"type":"RBR","frequency":"1415","particulars1":"130"}
    ]"#;

    #[test]
    fn the_sweep_list_reads_time_frequency_type_and_duration() {
        let records = parse_text(SAMPLE);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].freq_low_mhz, Some(25.0));
        assert_eq!(records[0].freq_high_mhz, Some(74.0));
        assert_eq!(records[0].duration_s, Some(18.0));
        assert_eq!(records[0].type_code, Some(3.0));
        assert_eq!(records[0].subtype_code, Some(1.0));
        assert_eq!(records[1].freq_high_mhz, Some(180.0));
        assert_eq!(records[1].subtype_code, Some(2.0));
        assert!((records[0].t_unix - 1_791_664_824.0).abs() < 1e-9);
    }

    #[test]
    fn the_bin_roundtrip_keeps_present_and_absent_columns() {
        let records = parse_text(SAMPLE);
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(records));
    }

    #[test]
    fn a_non_sweep_event_is_not_pressed_into_the_sweep_format() {
        let only_rbr =
            r#"[{"begin_datetime":"2026-10-10T19:00:00","type":"RBR","frequency":"1415"}]"#;
        assert!(parse_text(only_rbr).is_empty());
    }
}
