use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct TdmObservation {
    pub name: String,
    pub link: u8,
    pub time: String,
    pub value: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tdm {
    pub version: String,
    pub originator: String,
    pub creation_date: String,
    pub time_system: String,
    pub start_time: String,
    pub stop_time: String,
    pub participant_1: Option<String>,
    pub participant_2: Option<String>,
    pub mode: Option<String>,
    pub transmit_band: Option<String>,
    pub data_quality: Option<String>,
    pub observations: Vec<TdmObservation>,
}

fn key_value(line: &str) -> Option<(&str, &str)> {
    let (k, v) = line.split_once('=')?;
    Some((k.trim(), v.trim()))
}

fn observation_key(key: &str) -> Option<(&str, u8)> {
    if let Some((name, link)) = key.rsplit_once('_')
        && let Ok(link) = link.parse::<u8>()
        && (link == 1 || link == 2)
    {
        return Some((name, link));
    }
    let known = !matches!(
        key,
        "TIME_SYSTEM"
            | "START_TIME"
            | "STOP_TIME"
            | "PARTICIPANT_1"
            | "PARTICIPANT_2"
            | "MODE"
            | "PATH"
            | "TRANSMIT_BAND"
            | "RECEIVE_BAND"
            | "TIMETAG_REF"
            | "DATA_QUALITY"
            | "RANGE_MODULUS"
            | "CORRECTION_RANGE"
            | "CORRECTION_ANGLE_1"
            | "CORRECTION_ANGLE_2"
            | "FREQ_OFFSET"
            | "TRANSMIT_DELAY_1"
            | "TRANSMIT_DELAY_2"
            | "RECEIVE_DELAY_1"
            | "RECEIVE_DELAY_2"
            | "TURNAROUND_NUMERATOR"
            | "TURNAROUND_DENOMINATOR"
            | "INTEGRATION_INTERVAL"
            | "INTEGRATION_REF"
            | "RANGE_UNCERTAINTY"
            | "APERTURE"
    );
    if known && key.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
        return Some((key, 1));
    }
    None
}

pub fn parse_tdm(text: &str) -> Option<Tdm> {
    if !text.contains("DATA_START") || !text.contains("DATA_STOP") {
        return None;
    }
    let mut tdm = Tdm::default();
    let mut in_meta = false;
    let mut in_data = false;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("COMMENT") {
            continue;
        }
        if line == "META_START" {
            in_meta = true;
            continue;
        }
        if line == "META_STOP" {
            in_meta = false;
            continue;
        }
        if line == "DATA_START" {
            in_data = true;
            continue;
        }
        if line == "DATA_STOP" {
            in_data = false;
            continue;
        }
        let Some((key, value)) = key_value(line) else {
            continue;
        };
        if in_meta {
            match key {
                "TIME_SYSTEM" => tdm.time_system = value.to_string(),
                "START_TIME" => tdm.start_time = value.to_string(),
                "STOP_TIME" => tdm.stop_time = value.to_string(),
                "PARTICIPANT_1" => tdm.participant_1 = Some(value.to_string()),
                "PARTICIPANT_2" => tdm.participant_2 = Some(value.to_string()),
                "MODE" => tdm.mode = Some(value.to_string()),
                "TRANSMIT_BAND" => tdm.transmit_band = Some(value.to_string()),
                "DATA_QUALITY" => tdm.data_quality = Some(value.to_string()),
                _ => {}
            }
            continue;
        }
        if in_data {
            let Some((name, link)) = observation_key(key) else {
                continue;
            };
            let mut parts = value.split_whitespace();
            let (Some(time), Some(number)) = (parts.next(), parts.next()) else {
                continue;
            };
            let Ok(value) = number.parse::<f64>() else {
                continue;
            };
            tdm.observations.push(TdmObservation {
                name: name.to_string(),
                link,
                time: time.to_string(),
                value,
            });
            continue;
        }
        match key {
            "CCSDS_TDM_VERS" => tdm.version = value.to_string(),
            "ORIGINATOR" => tdm.originator = value.to_string(),
            "CREATION_DATE" => tdm.creation_date = value.to_string(),
            _ => {}
        }
    }
    if tdm.version.is_empty() || tdm.time_system.is_empty() {
        return None;
    }
    if tdm.start_time.is_empty() || tdm.stop_time.is_empty() {
        return None;
    }
    Some(tdm)
}

pub fn counts(tdm: &Tdm) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    for o in &tdm.observations {
        *out.entry(o.name.clone()).or_insert(0) += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2025-05-13T08:53:44
ORIGINATOR = ESA

META_START
TIME_SYSTEM = UTC
START_TIME = 2005-02-18T06:03:24.000000
STOP_TIME = 2005-02-18T11:05:32.000000
PARTICIPANT_1 = DSS-25
PARTICIPANT_2 = ROSETTA
META_STOP

DATA_START
TRANSMIT_FREQ_1 = 2005-02-18T06:03:24.000000 7167991440.0
RECEIVE_FREQ_1 = 2005-02-18T06:03:24.000000 7.167e9
TRANSMIT_FREQ_RATE_1 = 2005-02-18T07:04:45.000000 -1000.0
DATA_STOP
";

    #[test]
    fn a_tdm_carries_header_meta_and_observations() {
        let tdm = parse_tdm(SAMPLE).expect("a tdm");
        assert_eq!(tdm.version, "2.0");
        assert_eq!(tdm.originator, "ESA");
        assert_eq!(tdm.time_system, "UTC");
        assert_eq!(tdm.participant_1.as_deref(), Some("DSS-25"));
        assert_eq!(tdm.participant_2.as_deref(), Some("ROSETTA"));
        assert_eq!(tdm.observations.len(), 3);
        assert_eq!(tdm.observations[0].name, "TRANSMIT_FREQ");
        assert_eq!(tdm.observations[0].link, 1);
        assert_eq!(tdm.observations[0].value, 7167991440.0);
        assert_eq!(tdm.observations[2].name, "TRANSMIT_FREQ_RATE");
        assert_eq!(tdm.observations[2].value, -1000.0);
    }

    #[test]
    fn a_tdm_without_data_block_is_absent() {
        let text = SAMPLE.replace("DATA_START", "").replace("DATA_STOP", "");
        assert!(parse_tdm(&text).is_none());
    }

    #[test]
    fn a_broken_observation_is_skipped_not_zeroed() {
        let text = SAMPLE.replace(
            "RECEIVE_FREQ_1 = 2005-02-18T06:03:24.000000 7.167e9",
            "RECEIVE_FREQ_1 = 2005-02-18T06:03:24.000000",
        );
        let tdm = parse_tdm(&text).expect("a tdm");
        assert_eq!(tdm.observations.len(), 2);
        assert!(!tdm.observations.iter().any(|o| o.name == "RECEIVE_FREQ"));
    }

    #[test]
    fn an_observation_without_participant_suffix_reads_as_link_one() {
        let text = SAMPLE.replace(
            "RECEIVE_FREQ_1 = 2005-02-18T06:03:24.000000 7.167e9",
            "RANGE = 2005-02-18T06:03:24.000000 0.0073329",
        );
        let tdm = parse_tdm(&text).expect("a tdm");
        let range = tdm
            .observations
            .iter()
            .find(|o| o.name == "RANGE")
            .expect("a range observation");
        assert_eq!(range.link, 1);
        assert_eq!(range.value, 0.0073329);
    }

    #[test]
    fn meta_keys_inside_the_data_block_are_not_observations() {
        let text = SAMPLE.replace(
            "DATA_START",
            "DATA_START\nRANGE_MODULUS = 2005-02-18T06:03:24.000000 749.0",
        );
        let tdm = parse_tdm(&text).expect("a tdm");
        assert!(!tdm.observations.iter().any(|o| o.name == "RANGE_MODULUS"));
    }

    #[test]
    fn counts_group_by_observation_name() {
        let tdm = parse_tdm(SAMPLE).expect("a tdm");
        let c = counts(&tdm);
        assert_eq!(c.get("TRANSMIT_FREQ"), Some(&1));
        assert_eq!(c.get("RECEIVE_FREQ"), Some(&1));
        assert_eq!(c.get("TRANSMIT_FREQ_RATE"), Some(&1));
    }
}
