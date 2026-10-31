use std::collections::BTreeMap;

use crate::mathematikerin::receiver::two_way_light_time;

#[derive(Clone, Debug, PartialEq)]
pub struct TdmObservation {
    pub name: String,
    pub link: u8,
    pub time: String,
    pub value: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TdmMeta {
    pub time_system: String,
    pub start_time: String,
    pub stop_time: String,
    pub participant_1: Option<String>,
    pub participant_2: Option<String>,
    pub mode: Option<String>,
    pub path: Option<String>,
    pub timetag_ref: Option<String>,
    pub transmit_band: Option<String>,
    pub receive_band: Option<String>,
    pub data_quality: Option<String>,
    pub freq_offset: Option<f64>,
    pub correction_range: Option<f64>,
    pub range_modulus: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TdmBlock {
    pub meta: TdmMeta,
    pub observations: Vec<TdmObservation>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tdm {
    pub version: String,
    pub originator: String,
    pub creation_date: String,
    pub blocks: Vec<TdmBlock>,
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
    let mut current = TdmBlock::default();
    let mut in_meta = false;
    let mut in_data = false;
    let mut saw_data = false;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("COMMENT") {
            continue;
        }
        if line == "META_START" {
            if !current.observations.is_empty() {
                tdm.blocks.push(std::mem::take(&mut current));
            }
            current = TdmBlock::default();
            in_meta = true;
            in_data = false;
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
            if !current.observations.is_empty() {
                tdm.blocks.push(std::mem::take(&mut current));
            }
            continue;
        }
        let Some((key, value)) = key_value(line) else {
            continue;
        };
        if in_meta {
            let meta = &mut current.meta;
            match key {
                "TIME_SYSTEM" => meta.time_system = value.to_string(),
                "START_TIME" => meta.start_time = value.to_string(),
                "STOP_TIME" => meta.stop_time = value.to_string(),
                "PARTICIPANT_1" => meta.participant_1 = Some(value.to_string()),
                "PARTICIPANT_2" => meta.participant_2 = Some(value.to_string()),
                "MODE" => meta.mode = Some(value.to_string()),
                "PATH" => meta.path = Some(value.to_string()),
                "TIMETAG_REF" => meta.timetag_ref = Some(value.to_string()),
                "TRANSMIT_BAND" => meta.transmit_band = Some(value.to_string()),
                "RECEIVE_BAND" => meta.receive_band = Some(value.to_string()),
                "DATA_QUALITY" => meta.data_quality = Some(value.to_string()),
                "FREQ_OFFSET" => meta.freq_offset = value.parse().ok(),
                "CORRECTION_RANGE" => meta.correction_range = value.parse().ok(),
                "RANGE_MODULUS" => meta.range_modulus = value.parse().ok(),
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
            current.observations.push(TdmObservation {
                name: name.to_string(),
                link,
                time: time.to_string(),
                value,
            });
            saw_data = true;
            continue;
        }
        match key {
            "CCSDS_TDM_VERS" => tdm.version = value.to_string(),
            "ORIGINATOR" => tdm.originator = value.to_string(),
            "CREATION_DATE" => tdm.creation_date = value.to_string(),
            _ => {}
        }
    }
    if !current.observations.is_empty() {
        tdm.blocks.push(current);
    }
    if tdm.version.is_empty() || !saw_data {
        return None;
    }
    let missing_meta = tdm
        .blocks
        .iter()
        .any(|b| b.meta.time_system.is_empty() || b.meta.start_time.is_empty());
    if missing_meta {
        return None;
    }
    Some(tdm)
}

pub fn observations(tdm: &Tdm) -> impl Iterator<Item = &TdmObservation> {
    tdm.blocks.iter().flat_map(|b| b.observations.iter())
}

pub fn counts(tdm: &Tdm) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    for o in observations(tdm) {
        *out.entry(o.name.clone()).or_insert(0) += 1;
    }
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimetagRef {
    Receive,
    Transmit,
}

impl TimetagRef {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "RECEIVE" => Some(Self::Receive),
            "TRANSMIT" => Some(Self::Transmit),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Receive => "RECEIVE",
            Self::Transmit => "TRANSMIT",
        }
    }
}

impl TdmMeta {
    pub fn timetag_reference(&self) -> Option<TimetagRef> {
        match self.timetag_ref.as_deref() {
            None => Some(TimetagRef::Receive),
            Some(value) => TimetagRef::parse(value),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightTime {
    pub label_tdb: f64,
    pub receive_tdb: f64,
    pub bounce_tdb: f64,
    pub transmit_tdb: f64,
    pub state_minus_label_s: f64,
    pub range_km: f64,
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn norm3(a: [f64; 3]) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

pub fn observation_light_time<S>(
    label_tdb: f64,
    reference: TimetagRef,
    station_icrs_km: [f64; 3],
    target_state: S,
) -> Option<LightTime>
where
    S: Fn(f64) -> Option<([f64; 3], [f64; 3])>,
{
    if !label_tdb.is_finite() {
        return None;
    }
    let solve = |receive_tdb: f64| {
        let solution =
            two_way_light_time(receive_tdb, station_icrs_km, station_icrs_km, &target_state)?;
        let range_km = norm3(sub3(solution.bounce_pos_icrs_km, station_icrs_km));
        if !range_km.is_finite() || range_km <= 0.0 {
            return None;
        }
        Some((solution, range_km))
    };

    let (receive_tdb, solution, range_km) = match reference {
        TimetagRef::Receive => {
            let (solution, range_km) = solve(label_tdb)?;
            (label_tdb, solution, range_km)
        }
        TimetagRef::Transmit => {
            let (first, _) = solve(label_tdb)?;
            let mut receive_tdb = label_tdb + (label_tdb - first.t_tx_tdb);
            let mut settled: Option<(f64, crate::mathematikerin::receiver::TwoWayLightTime, f64)> =
                None;
            for _ in 0..20 {
                let (solution, range_km) = solve(receive_tdb)?;
                let residual = label_tdb - solution.t_tx_tdb;
                let next = receive_tdb + residual;
                if !next.is_finite() {
                    return None;
                }
                if (next - receive_tdb).abs() < 1.0e-9 {
                    settled = Some((receive_tdb, solution, range_km));
                    break;
                }
                receive_tdb = next;
            }
            let (receive_tdb, solution, range_km) = settled?;
            (receive_tdb, solution, range_km)
        }
    };

    Some(LightTime {
        label_tdb,
        receive_tdb,
        bounce_tdb: solution.t_bounce_tdb,
        transmit_tdb: solution.t_tx_tdb,
        state_minus_label_s: solution.t_bounce_tdb - label_tdb,
        range_km,
    })
}

pub const LIGHT_TIME_CONVENTION: &str = "label is the TDM time tag in its declared TIMETAG_REF; the spacecraft state is evaluated at the bounce epoch; the station is rotated from ITRF to ICRS at the receive epoch; state_minus_label_s = t_bounce - t_label; absent state or station is None, never zero";

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
FREQ_OFFSET = -1234.5
META_STOP

DATA_START
TRANSMIT_FREQ_1 = 2005-02-18T06:03:24.000000 7167991440.0
RECEIVE_FREQ_1 = 2005-02-18T06:03:24.000000 7.167e9
TRANSMIT_FREQ_RATE_1 = 2005-02-18T07:04:45.000000 -1000.0
DATA_STOP

META_START
TIME_SYSTEM = UTC
START_TIME = 2005-02-18T12:00:00.000000
STOP_TIME = 2005-02-18T13:00:00.000000
CORRECTION_RANGE = 0.0
RANGE_MODULUS = 749.0
META_STOP

DATA_START
RANGE = 2005-02-18T12:00:01.000000 0.00733291893913
DATA_STOP
";

    #[test]
    fn a_tdm_carries_blocks_with_their_own_meta() {
        let tdm = parse_tdm(SAMPLE).expect("a tdm");
        assert_eq!(tdm.version, "2.0");
        assert_eq!(tdm.originator, "ESA");
        assert_eq!(tdm.blocks.len(), 2);
        assert_eq!(tdm.blocks[0].meta.participant_1.as_deref(), Some("DSS-25"));
        assert_eq!(tdm.blocks[0].meta.freq_offset, Some(-1234.5));
        assert_eq!(tdm.blocks[0].observations.len(), 3);
        assert_eq!(tdm.blocks[1].meta.range_modulus, Some(749.0));
        assert_eq!(tdm.blocks[1].meta.correction_range, Some(0.0));
        assert_eq!(tdm.blocks[1].observations.len(), 1);
        assert_eq!(tdm.blocks[1].observations[0].name, "RANGE");
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
        assert!(!observations(&tdm).any(|o| o.name == "RECEIVE_FREQ"));
    }

    #[test]
    fn an_observation_without_participant_suffix_reads_as_link_one() {
        let tdm = parse_tdm(SAMPLE).expect("a tdm");
        let range = observations(&tdm)
            .find(|o| o.name == "RANGE")
            .expect("a range observation");
        assert_eq!(range.link, 1);
        assert_eq!(range.value, 0.00733291893913);
    }

    #[test]
    fn meta_keys_inside_the_data_block_are_not_observations() {
        let text = SAMPLE.replace(
            "DATA_START",
            "DATA_START\nRANGE_MODULUS_1 = 2005-02-18T06:03:24.000000 749.0",
        );
        let tdm = parse_tdm(&text).expect("a tdm");
        assert!(!observations(&tdm).any(|o| o.name == "RANGE_MODULUS"));
    }

    #[test]
    fn counts_group_by_observation_name() {
        let tdm = parse_tdm(SAMPLE).expect("a tdm");
        let c = counts(&tdm);
        assert_eq!(c.get("TRANSMIT_FREQ"), Some(&1));
        assert_eq!(c.get("RECEIVE_FREQ"), Some(&1));
        assert_eq!(c.get("TRANSMIT_FREQ_RATE"), Some(&1));
        assert_eq!(c.get("RANGE"), Some(&1));
    }

    const C_KM_S: f64 = 299_792.458;

    fn static_target(position: [f64; 3]) -> impl Fn(f64) -> Option<([f64; 3], [f64; 3])> {
        move |_t| Some((position, [0.0; 3]))
    }

    #[test]
    fn a_tdm_carries_its_timetag_reference() {
        let text = SAMPLE.replace(
            "PARTICIPANT_2 = ROSETTA",
            "PARTICIPANT_2 = ROSETTA\nTIMETAG_REF = RECEIVE",
        );
        let tdm = parse_tdm(&text).expect("a tdm");
        assert_eq!(tdm.blocks[0].meta.timetag_ref.as_deref(), Some("RECEIVE"));
        assert_eq!(
            tdm.blocks[0].meta.timetag_reference(),
            Some(TimetagRef::Receive)
        );
    }

    #[test]
    fn an_absent_timetag_reference_reads_as_receive() {
        let tdm = parse_tdm(SAMPLE).expect("a tdm");
        assert_eq!(tdm.blocks[0].meta.timetag_ref, None);
        assert_eq!(
            tdm.blocks[0].meta.timetag_reference(),
            Some(TimetagRef::Receive)
        );
    }

    #[test]
    fn an_unknown_timetag_reference_is_absent() {
        let text = SAMPLE.replace(
            "PARTICIPANT_2 = ROSETTA",
            "PARTICIPANT_2 = ROSETTA\nTIMETAG_REF = BOTH",
        );
        let tdm = parse_tdm(&text).expect("a tdm");
        assert_eq!(tdm.blocks[0].meta.timetag_ref.as_deref(), Some("BOTH"));
        assert_eq!(tdm.blocks[0].meta.timetag_reference(), None);
    }

    #[test]
    fn a_static_target_sits_one_light_time_behind_the_label() {
        let range_km = 300_000.0;
        let station = [0.0, 0.0, 0.0];
        let target = static_target([range_km, 0.0, 0.0]);
        let label = 1_000_000.0;
        let solution = observation_light_time(label, TimetagRef::Receive, station, target)
            .expect("a solution");
        let light_time = range_km / C_KM_S;
        assert!((solution.state_minus_label_s + light_time).abs() < 1.0e-9);
        assert!((solution.range_km - range_km).abs() < 1.0e-6);
        assert!((solution.receive_tdb - label).abs() < 1.0e-12);
        assert!((solution.bounce_tdb + light_time - label).abs() < 1.0e-9);
    }

    #[test]
    fn a_receding_target_is_closer_when_the_signal_left() {
        let station = [0.0, 0.0, 0.0];
        let speed_km_s = 10.0;
        let moving = move |t: f64| {
            Some((
                [300_000.0 + speed_km_s * t, 0.0, 0.0],
                [speed_km_s, 0.0, 0.0],
            ))
        };
        let solution =
            observation_light_time(0.0, TimetagRef::Receive, station, moving).expect("a solution");
        let static_light_time = 300_000.0 / C_KM_S;
        assert!(solution.state_minus_label_s < 0.0);
        assert!(solution.state_minus_label_s > -static_light_time);
        assert!(solution.range_km < 300_000.0);
    }

    #[test]
    fn a_transmit_label_recovers_the_receive_epoch() {
        let station = [0.0, 0.0, 0.0];
        let target = static_target([300_000.0, 0.0, 0.0]);
        let transmit_label = -2.0;
        let solution =
            observation_light_time(transmit_label, TimetagRef::Transmit, station, target)
                .expect("a solution");
        assert!((solution.transmit_tdb - transmit_label).abs() < 1.0e-9);
        let expected_receive = transmit_label + 2.0 * 300_000.0 / C_KM_S;
        assert!((solution.receive_tdb - expected_receive).abs() < 1.0e-6);
        assert!((solution.state_minus_label_s - 300_000.0 / C_KM_S).abs() < 1.0e-6);
    }

    #[test]
    fn an_absent_target_state_is_absent() {
        let absent = |_t: f64| -> Option<([f64; 3], [f64; 3])> { None };
        assert!(observation_light_time(0.0, TimetagRef::Receive, [0.0; 3], absent).is_none());
    }
}
