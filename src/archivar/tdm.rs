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
    pub range_units: Option<String>,
    pub transmit_delay_s: Option<f64>,
    pub receive_delay_s: Option<f64>,
    pub transponder_delay_s: Option<f64>,
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
            | "RANGE_UNITS"
            | "CORRECTION_RANGE"
            | "CORRECTION_ANGLE_1"
            | "CORRECTION_ANGLE_2"
            | "FREQ_OFFSET"
            | "TRANSMIT_DELAY_1"
            | "TRANSMIT_DELAY_2"
            | "RECEIVE_DELAY_1"
            | "RECEIVE_DELAY_2"
            | "TRANSPONDER_DELAY"
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
                "RANGE_UNITS" => meta.range_units = Some(value.to_string()),
                "TRANSMIT_DELAY_1" | "TRANSMIT_DELAY_2" => {
                    if meta.transmit_delay_s.is_none() {
                        meta.transmit_delay_s = value.parse().ok();
                    }
                }
                "RECEIVE_DELAY_1" | "RECEIVE_DELAY_2" => {
                    if meta.receive_delay_s.is_none() {
                        meta.receive_delay_s = value.parse().ok();
                    }
                }
                "TRANSPONDER_DELAY" => meta.transponder_delay_s = value.parse().ok(),
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

pub fn observation_light_time_station<S, T>(
    label_tdb: f64,
    reference: TimetagRef,
    station_at: T,
    target_state: S,
) -> Option<LightTime>
where
    S: Fn(f64) -> Option<([f64; 3], [f64; 3])>,
    T: Fn(f64) -> Option<[f64; 3]>,
{
    if !label_tdb.is_finite() {
        return None;
    }
    let c_km_s = crate::archivar::types::C_LIGHT / 1000.0;

    let receive = |t_rx: f64| -> Option<(f64, f64, f64, f64)> {
        let station_rx = station_at(t_rx)?;
        let mut t_b = t_rx;
        for _ in 0..20 {
            let (position, _) = target_state(t_b)?;
            let distance = norm3(sub3(position, station_rx));
            let next = t_rx - distance / c_km_s;
            if !next.is_finite() {
                return None;
            }
            if (next - t_b).abs() < 1.0e-12 {
                t_b = next;
                break;
            }
            t_b = next;
        }
        let mut t_tx = t_b;
        for _ in 0..20 {
            let station_tx = station_at(t_tx)?;
            let (position, _) = target_state(t_b)?;
            let distance = norm3(sub3(position, station_tx));
            let next = t_b - distance / c_km_s;
            if !next.is_finite() {
                return None;
            }
            if (next - t_tx).abs() < 1.0e-12 {
                t_tx = next;
                break;
            }
            t_tx = next;
        }
        let (position, _) = target_state(t_b)?;
        let range_km = norm3(sub3(position, station_rx));
        Some((t_b, t_tx, t_rx, range_km))
    };

    let transmit = |t_tx: f64| -> Option<(f64, f64, f64, f64)> {
        let station_tx = station_at(t_tx)?;
        let mut t_b = t_tx;
        for _ in 0..20 {
            let (position, _) = target_state(t_b)?;
            let distance = norm3(sub3(position, station_tx));
            let next = t_tx + distance / c_km_s;
            if !next.is_finite() {
                return None;
            }
            if (next - t_b).abs() < 1.0e-12 {
                t_b = next;
                break;
            }
            t_b = next;
        }
        let mut t_rx = t_b;
        for _ in 0..20 {
            let station_rx = station_at(t_rx)?;
            let (position, _) = target_state(t_b)?;
            let distance = norm3(sub3(position, station_rx));
            let next = t_b + distance / c_km_s;
            if !next.is_finite() {
                return None;
            }
            if (next - t_rx).abs() < 1.0e-12 {
                t_rx = next;
                break;
            }
            t_rx = next;
        }
        let station_rx = station_at(t_rx)?;
        let (position, _) = target_state(t_b)?;
        let range_km = norm3(sub3(position, station_rx));
        Some((t_b, t_tx, t_rx, range_km))
    };

    let (t_b, t_tx, t_rx, range_km) = match reference {
        TimetagRef::Receive => receive(label_tdb)?,
        TimetagRef::Transmit => transmit(label_tdb)?,
    };
    if !range_km.is_finite() || range_km <= 0.0 {
        return None;
    }
    Some(LightTime {
        label_tdb,
        receive_tdb: t_rx,
        bounce_tdb: t_b,
        transmit_tdb: t_tx,
        state_minus_label_s: t_b - label_tdb,
        range_km,
    })
}

pub const MEDIA_GAP_S: f64 = 600.0;
pub const RANGE_UNIT_S: f64 = 0.94e-9;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeUnit {
    Ru,
    Seconds,
}

impl TdmMeta {
    pub fn range_unit(&self) -> Option<RangeUnit> {
        match self.range_units.as_deref() {
            Some(value) if value.eq_ignore_ascii_case("S") => Some(RangeUnit::Seconds),
            Some(value) if value.eq_ignore_ascii_case("RU") => Some(RangeUnit::Ru),
            Some(_) => None,
            None => self.range_modulus.map(|modulus| {
                if modulus > 1.0 {
                    RangeUnit::Ru
                } else {
                    RangeUnit::Seconds
                }
            }),
        }
    }

    pub fn transponder_delay_s(&self) -> Option<f64> {
        self.transponder_delay_s
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangeReading {
    pub raw: f64,
    pub raw_seconds: f64,
    pub modulus_seconds: Option<f64>,
    pub wraps: i64,
    pub unfolded_seconds: f64,
    pub transponder_s: f64,
    pub one_way_km: f64,
}

pub fn range_unit_seconds(band: &str, carrier_hz: f64) -> Option<f64> {
    if !carrier_hz.is_finite() || carrier_hz <= 0.0 {
        return None;
    }
    match band.trim().to_ascii_uppercase().as_str() {
        "S" => Some(2.0 / carrier_hz),
        "X" => Some((749.0 / 221.0) * 2.0 / carrier_hz),
        "KA" => Some((3599.0 / 221.0) * 2.0 / carrier_hz),
        _ => None,
    }
}

pub fn unfold_range(
    raw: f64,
    modulus: Option<f64>,
    unit_seconds: Option<f64>,
    reference_two_way_s: f64,
    transponder_s: f64,
) -> Option<RangeReading> {
    if !raw.is_finite() || !reference_two_way_s.is_finite() || !transponder_s.is_finite() {
        return None;
    }
    let unit_seconds = unit_seconds.filter(|value| value.is_finite() && *value > 0.0)?;
    let raw_seconds = raw * unit_seconds;
    let modulus_seconds = modulus
        .filter(|value| value.is_finite())
        .map(|value| value * unit_seconds);
    let (wraps, unfolded_seconds) = match modulus_seconds {
        Some(period) if period > 0.0 => {
            let count = ((reference_two_way_s - raw_seconds) / period).round();
            if !count.is_finite() {
                return None;
            }
            (count as i64, raw_seconds + count * period)
        }
        _ => (0, raw_seconds),
    };
    let one_way_km =
        (unfolded_seconds - transponder_s) * (crate::archivar::types::C_LIGHT / 1000.0) / 2.0;
    if !one_way_km.is_finite() {
        return None;
    }
    Some(RangeReading {
        raw,
        raw_seconds,
        modulus_seconds,
        wraps,
        unfolded_seconds,
        transponder_s,
        one_way_km,
    })
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaTerms {
    pub dry_m: Option<f64>,
    pub wet_m: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct TropoSeries {
    dry: Vec<(f64, f64)>,
    wet: Vec<(f64, f64)>,
}

impl TropoSeries {
    pub fn from_tdm(tdm: &Tdm) -> Option<Self> {
        let lsk = crate::archivar::embedded_lsk()?;
        let mut dry = Vec::new();
        let mut wet = Vec::new();
        for observation in observations(tdm) {
            let Some(t) = crate::archivar::parse_iso_tdb(&observation.time, &lsk) else {
                continue;
            };
            match observation.name.as_str() {
                "TROPO_DRY" => dry.push((t, observation.value)),
                "TROPO_WET" => wet.push((t, observation.value)),
                _ => {}
            }
        }
        if dry.is_empty() && wet.is_empty() {
            return None;
        }
        dry.sort_by(|a, b| a.0.total_cmp(&b.0));
        wet.sort_by(|a, b| a.0.total_cmp(&b.0));
        Some(Self { dry, wet })
    }

    fn interpolate(samples: &[(f64, f64)], t: f64) -> Option<f64> {
        if samples.is_empty() || !t.is_finite() {
            return None;
        }
        let last = samples.len() - 1;
        if t < samples[0].0 || t > samples[last].0 {
            return None;
        }
        let index = samples.partition_point(|sample| sample.0 < t);
        if index == 0 {
            return Some(samples[0].1);
        }
        if index > last {
            return Some(samples[last].1);
        }
        let (t0, v0) = samples[index - 1];
        let (t1, v1) = samples[index];
        if t1 == t0 {
            return Some(v1);
        }
        if t1 - t0 > MEDIA_GAP_S {
            return None;
        }
        Some(v0 + (v1 - v0) * (t - t0) / (t1 - t0))
    }

    pub fn at(&self, t: f64) -> MediaTerms {
        MediaTerms {
            dry_m: Self::interpolate(&self.dry, t),
            wet_m: Self::interpolate(&self.wet, t),
        }
    }

    pub fn span(&self) -> Option<(f64, f64)> {
        let first = self.dry.first().or_else(|| self.wet.first())?.0;
        let last = self.dry.last().or_else(|| self.wet.last())?.0;
        Some((first, last))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2025-05-13T08:53:44
ORIGINATOR = TEST

META_START
TIME_SYSTEM = UTC
START_TIME = 2020-01-01T06:03:24.000000
STOP_TIME = 2020-01-01T11:05:32.000000
PARTICIPANT_1 = TEST01
PARTICIPANT_2 = DEMO
FREQ_OFFSET = -1234.5
META_STOP

DATA_START
TRANSMIT_FREQ_1 = 2020-01-01T06:03:24.000000 7000000000.0
RECEIVE_FREQ_1 = 2020-01-01T06:03:24.000000 7.0e9
TRANSMIT_FREQ_RATE_1 = 2020-01-01T07:04:45.000000 -1000.0
DATA_STOP

META_START
TIME_SYSTEM = UTC
START_TIME = 2020-01-01T12:00:00.000000
STOP_TIME = 2020-01-01T13:00:00.000000
CORRECTION_RANGE = 0.0
RANGE_MODULUS = 749.0
META_STOP

DATA_START
RANGE = 2020-01-01T12:00:01.000000 0.0075
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
            "RECEIVE_FREQ_1 = 2020-01-01T06:03:24.000000 7.0e9",
            "RECEIVE_FREQ_1 = 2020-01-01T06:03:24.000000",
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
        assert_eq!(range.value, 0.0075);
    }

    #[test]
    fn meta_keys_inside_the_data_block_are_not_observations() {
        let text = SAMPLE.replace(
            "DATA_START",
            "DATA_START\nRANGE_MODULUS_1 = 2020-01-01T06:03:24.000000 749.0",
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
            "PARTICIPANT_2 = DEMO",
            "PARTICIPANT_2 = DEMO\nTIMETAG_REF = RECEIVE",
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
            "PARTICIPANT_2 = DEMO",
            "PARTICIPANT_2 = DEMO\nTIMETAG_REF = BOTH",
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

    #[test]
    fn a_station_at_its_own_epoch_closes_the_two_way_time() {
        let station_at = |t: f64| Some([0.0, t, 0.0]);
        let target = static_target([300_000.0, 0.0, 0.0]);
        let solution =
            observation_light_time_station(0.0, TimetagRef::Receive, station_at, &target)
                .expect("a solution");
        let (position, _) = target(solution.bounce_tdb).expect("a target");
        let dx = position[0];
        let dy = position[1] - solution.transmit_tdb;
        let distance = (dx * dx + dy * dy).sqrt();
        assert!((solution.bounce_tdb - solution.transmit_tdb - distance / C_KM_S).abs() < 1.0e-9);
        assert!(solution.receive_tdb.abs() < 1.0e-12);
    }

    const TROPO: &str = "CCSDS_TDM_VERS = 2.0
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
START_TIME = 2005-02-17T11:11:00.000000
STOP_TIME = 2005-02-17T11:13:00.000000
PARTICIPANT_1 = NN11
META_STOP
DATA_START
TROPO_DRY = 2005-02-17T11:11:00.000000 2.0
TROPO_WET = 2005-02-17T11:11:00.000000 0.1
TROPO_DRY = 2005-02-17T11:12:00.000000 2.2
TROPO_WET = 2005-02-17T11:12:00.000000 0.2
TROPO_DRY = 2005-02-17T11:13:00.000000 2.4
TROPO_WET = 2005-02-17T11:13:00.000000 0.3
DATA_STOP
";

    const TROPO_GAP: &str = "CCSDS_TDM_VERS = 2.0
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
START_TIME = 2005-02-17T11:11:00.000000
STOP_TIME = 2005-02-17T11:30:00.000000
PARTICIPANT_1 = NN11
META_STOP
DATA_START
TROPO_DRY = 2005-02-17T11:11:00.000000 2.0
TROPO_WET = 2005-02-17T11:11:00.000000 0.1
TROPO_DRY = 2005-02-17T11:30:00.000000 2.4
TROPO_WET = 2005-02-17T11:30:00.000000 0.3
DATA_STOP
";

    #[test]
    fn a_tropo_series_interpolates_between_minutes() {
        let tdm = parse_tdm(TROPO).expect("a tdm");
        let series = TropoSeries::from_tdm(&tdm).expect("a series");
        let lsk = crate::archivar::embedded_lsk().expect("leap seconds");
        let t = crate::archivar::parse_iso_tdb("2005-02-17T11:12:30.000000", &lsk).expect("a time");
        let terms = series.at(t);
        assert!((terms.dry_m.expect("dry") - 2.3).abs() < 1.0e-9);
        assert!((terms.wet_m.expect("wet") - 0.25).abs() < 1.0e-9);
    }

    #[test]
    fn a_tropo_gap_larger_than_the_bound_is_absent() {
        let tdm = parse_tdm(TROPO_GAP).expect("a tdm");
        let series = TropoSeries::from_tdm(&tdm).expect("a series");
        let lsk = crate::archivar::embedded_lsk().expect("leap seconds");
        let t = crate::archivar::parse_iso_tdb("2005-02-17T11:20:00.000000", &lsk).expect("a time");
        let terms = series.at(t);
        assert!(terms.dry_m.is_none());
        assert!(terms.wet_m.is_none());
    }

    #[test]
    fn a_ru_range_unfolds_against_the_reference() {
        let reading = unfold_range(
            38_000_000.0,
            Some(67_108_864.0),
            Some(RANGE_UNIT_S),
            34.0,
            2.0e-6,
        )
        .expect("a range");
        assert!(reading.wraps > 500 && reading.wraps < 560);
        assert!((reading.one_way_km - 5.09e6).abs() < 1.0e5);
        assert!((reading.unfolded_seconds - 34.0).abs() < 0.05);
    }

    #[test]
    fn a_seconds_range_unfolds_against_the_reference() {
        let reading = unfold_range(0.0075, Some(0.015), Some(1.0), 34.0, 2.0e-6).expect("a range");
        assert!(reading.wraps > 2000 && reading.wraps < 2300);
        assert!((reading.unfolded_seconds - 34.0).abs() < 0.01);
    }

    #[test]
    fn an_absent_modulus_leaves_the_raw_range() {
        let reading = unfold_range(1.5, None, Some(RANGE_UNIT_S), 34.0, 0.0).expect("a range");
        assert_eq!(reading.wraps, 0);
        assert!((reading.unfolded_seconds - 1.5 * RANGE_UNIT_S).abs() < 1.0e-18);
    }

    #[test]
    fn the_range_unit_follows_the_carrier() {
        assert!((range_unit_seconds("S", 2.0e9).expect("s") - 1.0e-9).abs() < 1.0e-18);
        assert!((range_unit_seconds("X", 7.16e9).expect("x") - 9.4669e-10).abs() < 1.0e-14);
        assert!(range_unit_seconds("Q", 1.0).is_none());
    }

    #[test]
    fn the_declared_transponder_delay_is_read() {
        let declared = SAMPLE.replace(
            "PARTICIPANT_2 = DEMO",
            "PARTICIPANT_2 = DEMO\nTRANSPONDER_DELAY = 2.0e-6",
        );
        let tdm = parse_tdm(&declared).expect("a tdm");
        assert_eq!(tdm.blocks[0].meta.transponder_delay_s(), Some(2.0e-6));
        let absent = parse_tdm(SAMPLE).expect("a tdm");
        assert_eq!(absent.blocks[0].meta.transponder_delay_s(), None);
    }

    #[test]
    fn the_declared_unit_wins_and_an_absent_one_reads_from_the_modulus() {
        let declared = SAMPLE.replace(
            "RANGE_MODULUS = 749.0",
            "RANGE_MODULUS = 67108864\nRANGE_UNITS = RU",
        );
        let tdm = parse_tdm(&declared).expect("a tdm");
        assert_eq!(tdm.blocks[1].meta.range_unit(), Some(RangeUnit::Ru));
        let inferred = SAMPLE.replace("RANGE_MODULUS = 749.0", "RANGE_MODULUS = [redacted]");
        let tdm = parse_tdm(&inferred).expect("a tdm");
        assert_eq!(tdm.blocks[1].meta.range_unit(), Some(RangeUnit::Seconds));
    }
}
