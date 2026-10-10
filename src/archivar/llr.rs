use super::lsk::days_from_civil;

pub const MAGIC: [u8; 4] = *b"LLR ";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 84;

pub const REFLECTOR_APOLLO11: u32 = 0;
pub const REFLECTOR_APOLLO14: u32 = 1;
pub const REFLECTOR_APOLLO15: u32 = 2;
pub const REFLECTOR_LUNA17: u32 = 3;
pub const REFLECTOR_LUNA21: u32 = 4;
pub const REFLECTOR_UNKNOWN: u32 = u32::MAX;

pub const FLAG_TIME_ORDER: u32 = 1 << 0;
pub const FLAG_TIME_RANGE: u32 = 1 << 1;
pub const FLAG_TOF_RANGE: u32 = 1 << 2;
pub const FLAG_UNKNOWN_REFLECTOR: u32 = 1 << 3;
pub const FLAG_UNKNOWN_STATION: u32 = 1 << 4;
pub const FLAG_NO_SESSION: u32 = 1 << 5;
pub const FLAG_NO_TIME: u32 = 1 << 6;

const TOF_LO_S: f64 = 0.5;
const TOF_HI_S: f64 = 6.0;
const EPOCH_LO_UTC: f64 = -31_536_000.0;
const EPOCH_HI_UTC: f64 = 4_133_980_800.0;

const P_EPOCH: u32 = 1 << 0;
const P_SEC: u32 = 1 << 1;
const P_TOF: u32 = 1 << 2;
const P_BIN_RMS: u32 = 1 << 3;
const P_RETURN_RATE: u32 = 1 << 4;
const P_SNR: u32 = 1 << 5;
const P_WINDOW: u32 = 1 << 6;
const P_NUM_RANGES: u32 = 1 << 7;
const P_EPOCH_EVENT: u32 = 1 << 8;
const P_DETECTOR: u32 = 1 << 9;

const NA_VALUEF: f64 = -1.0e30;
const NA_VALUE: i32 = -10_000_000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalPoint {
    pub epoch_utc: Option<f64>,
    pub sec_of_day: Option<f64>,
    pub time_of_flight: Option<f64>,
    pub bin_rms_ps: Option<f64>,
    pub return_rate: Option<f64>,
    pub signal_to_noise: Option<f64>,
    pub np_window_length_ns: Option<f64>,
    pub num_ranges: Option<u32>,
    pub epoch_event: Option<i32>,
    pub detector_channel: Option<i32>,
    pub reflector: u32,
    pub station: u32,
    pub flags: u32,
}

pub fn reflector_code(name: &str) -> Option<u32> {
    match name.to_ascii_lowercase().as_str() {
        "apollo11" => Some(REFLECTOR_APOLLO11),
        "apollo14" => Some(REFLECTOR_APOLLO14),
        "apollo15" => Some(REFLECTOR_APOLLO15),
        "luna17" => Some(REFLECTOR_LUNA17),
        "luna21" => Some(REFLECTOR_LUNA21),
        _ => None,
    }
}

pub fn reflector_name(code: u32) -> Option<&'static str> {
    match code {
        REFLECTOR_APOLLO11 => Some("apollo11"),
        REFLECTOR_APOLLO14 => Some("apollo14"),
        REFLECTOR_APOLLO15 => Some("apollo15"),
        REFLECTOR_LUNA17 => Some("luna17"),
        REFLECTOR_LUNA21 => Some("luna21"),
        _ => None,
    }
}

pub fn component_name(reflector: u32) -> Option<&'static str> {
    match reflector {
        REFLECTOR_APOLLO11 => Some("llr_apollo11_round_trip_s"),
        REFLECTOR_APOLLO14 => Some("llr_apollo14_round_trip_s"),
        REFLECTOR_APOLLO15 => Some("llr_apollo15_round_trip_s"),
        REFLECTOR_LUNA17 => Some("llr_luna17_round_trip_s"),
        REFLECTOR_LUNA21 => Some("llr_luna21_round_trip_s"),
        _ => None,
    }
}

fn station_known(cdp_pad_id: u32) -> bool {
    matches!(cdp_pad_id, 7045 | 7110)
}

fn station_known_mini(ilrs_id: u32) -> bool {
    matches!(ilrs_id, 71110 | 71111 | 71112 | 1910 | 56610 | 7941 | 70610)
}

fn meas_f64(tok: &str) -> Option<f64> {
    if tok.eq_ignore_ascii_case("na") || tok.eq_ignore_ascii_case("nan") {
        return None;
    }
    let v: f64 = tok.parse().ok()?;
    if !v.is_finite() || v <= NA_VALUEF {
        return None;
    }
    Some(v)
}

fn meas_i32(tok: &str) -> Option<i32> {
    if tok.eq_ignore_ascii_case("na") {
        return None;
    }
    let v: i32 = tok.parse().ok()?;
    if v == NA_VALUE {
        return None;
    }
    Some(v)
}

fn meas_u32(tok: &str) -> Option<u32> {
    if tok.eq_ignore_ascii_case("na") {
        return None;
    }
    tok.parse().ok()
}

fn mini_col<'a>(line: &'a [u8], start1: usize, len: usize) -> Option<&'a [u8]> {
    let start = start1.checked_sub(1)?;
    line.get(start..start.checked_add(len)?)
}

fn mini_trim(field: &[u8]) -> &[u8] {
    let start = match field.iter().position(|b| !b.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &field[..0],
    };
    let end = match field.iter().rposition(|b| !b.is_ascii_whitespace()) {
        Some(i) => i + 1,
        None => start,
    };
    &field[start..end]
}

fn mini_all_nines(field: &[u8]) -> bool {
    !field.is_empty() && field.iter().all(|b| *b == b'9')
}

fn mini_f64(field: &[u8]) -> Option<f64> {
    let tok = mini_trim(field);
    let s = std::str::from_utf8(tok).ok()?;
    if s.eq_ignore_ascii_case("na") || s.eq_ignore_ascii_case("nan") {
        return None;
    }
    let v: f64 = s.parse().ok()?;
    if !v.is_finite() || v <= NA_VALUEF {
        return None;
    }
    Some(v)
}

fn mini_u32(field: &[u8]) -> Option<u32> {
    let tok = mini_trim(field);
    std::str::from_utf8(tok).ok()?.parse().ok()
}

fn mini_date(field: &[u8]) -> Option<(i64, i64, i64)> {
    let tok = mini_trim(field);
    if tok.len() != 8 || !tok.iter().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let s = std::str::from_utf8(tok).ok()?;
    Some((
        s[0..4].parse().ok()?,
        s[4..6].parse().ok()?,
        s[6..8].parse().ok()?,
    ))
}

fn mini_sec_of_day(field: &[u8]) -> Option<f64> {
    let tok = mini_trim(field);
    if tok.len() != 13 || !tok.iter().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let s = std::str::from_utf8(tok).ok()?;
    let hh: f64 = s[0..2].parse().ok()?;
    let mm: f64 = s[2..4].parse().ok()?;
    let ss: f64 = s[4..6].parse().ok()?;
    let frac: f64 = s[6..13].parse().ok()?;
    Some(hh * 3600.0 + mm * 60.0 + ss + frac * 1e-7)
}

fn mini_reflector(field: &[u8]) -> u32 {
    match mini_trim(field).first() {
        Some(b'0') => REFLECTOR_APOLLO11,
        Some(b'1') => REFLECTOR_LUNA17,
        Some(b'2') => REFLECTOR_APOLLO14,
        Some(b'3') => REFLECTOR_APOLLO15,
        Some(b'4') => REFLECTOR_LUNA21,
        _ => REFLECTOR_UNKNOWN,
    }
}

pub fn parse_crd(bytes: &[u8]) -> Option<Vec<NormalPoint>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut out: Vec<NormalPoint> = Vec::new();
    let mut saw_header = false;
    let mut session: Option<(i64, i64, i64)> = None;
    let mut station = 0u32;
    let mut reflector = REFLECTOR_UNKNOWN;
    let mut last_epoch: Option<f64> = None;

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        match toks[0].to_ascii_lowercase().as_str() {
            "h1" => {
                let literal = toks.get(1).copied().unwrap_or("");
                if !literal.eq_ignore_ascii_case("crd") {
                    return None;
                }
                match toks.get(2).and_then(|t| t.parse::<i32>().ok()) {
                    Some(1) | Some(2) => saw_header = true,
                    _ => return None,
                }
            }
            "h2" => {
                if let Some(id) = toks.get(2).and_then(|t| t.parse::<u32>().ok()) {
                    station = id;
                }
            }
            "h3" => {
                let name = toks.get(1).copied().unwrap_or("");
                reflector = reflector_code(name).unwrap_or(REFLECTOR_UNKNOWN);
            }
            "h4" => {
                let y = toks.get(2).and_then(|t| t.parse::<i64>().ok());
                let m = toks.get(3).and_then(|t| t.parse::<i64>().ok());
                let d = toks.get(4).and_then(|t| t.parse::<i64>().ok());
                session = match (y, m, d) {
                    (Some(y), Some(m), Some(d)) => Some((y, m, d)),
                    _ => None,
                };
            }
            "11" => {
                let sec_of_day = toks.get(1).and_then(|t| meas_f64(t));
                let time_of_flight = toks.get(2).and_then(|t| meas_f64(t));
                let bin_rms_ps = toks.get(7).and_then(|t| meas_f64(t));
                let return_rate = toks.get(11).and_then(|t| meas_f64(t));
                let signal_to_noise = toks.get(13).and_then(|t| meas_f64(t));
                let np_window_length_ns = toks.get(5).and_then(|t| meas_f64(t));
                let num_ranges = toks.get(6).and_then(|t| meas_u32(t));
                let epoch_event = toks.get(4).and_then(|t| meas_i32(t));
                let detector_channel = toks.get(12).and_then(|t| meas_i32(t));

                let mut flags = 0u32;
                let epoch_utc = match (session, sec_of_day) {
                    (Some((y, m, d)), Some(s)) => {
                        days_from_civil(y, m, d).map(|days| days as f64 * 86400.0 + s)
                    }
                    _ => None,
                };
                if session.is_none() {
                    flags |= FLAG_NO_SESSION;
                }
                if sec_of_day.is_none() {
                    flags |= FLAG_NO_TIME;
                }
                if reflector == REFLECTOR_UNKNOWN {
                    flags |= FLAG_UNKNOWN_REFLECTOR;
                }
                if !station_known(station) {
                    flags |= FLAG_UNKNOWN_STATION;
                }
                if let Some(t) = time_of_flight {
                    if !(TOF_LO_S..=TOF_HI_S).contains(&t) {
                        flags |= FLAG_TOF_RANGE;
                    }
                }
                if let Some(e) = epoch_utc {
                    if !(EPOCH_LO_UTC..EPOCH_HI_UTC).contains(&e) {
                        flags |= FLAG_TIME_RANGE;
                    }
                    if let Some(prev) = last_epoch {
                        if e < prev {
                            flags |= FLAG_TIME_ORDER;
                        }
                    }
                    last_epoch = Some(match last_epoch {
                        Some(p) if p > e => p,
                        _ => e,
                    });
                }

                out.push(NormalPoint {
                    epoch_utc,
                    sec_of_day,
                    time_of_flight,
                    bin_rms_ps,
                    return_rate,
                    signal_to_noise,
                    np_window_length_ns,
                    num_ranges,
                    epoch_event,
                    detector_channel,
                    reflector,
                    station,
                    flags,
                });
            }
            _ => {}
        }
    }
    if !saw_header || out.is_empty() {
        return None;
    }
    Some(out)
}

pub fn parse_mini(bytes: &[u8]) -> Option<Vec<NormalPoint>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut out: Vec<NormalPoint> = Vec::new();
    let mut last_epoch: Option<f64> = None;

    for raw in text.lines() {
        let line = raw.as_bytes();
        if line.len() < 43 || line[0] != b'5' {
            continue;
        }

        let date = mini_col(line, 3, 8).and_then(mini_date);
        let sec_of_day = mini_col(line, 11, 13).and_then(mini_sec_of_day);
        let epoch_utc = match (date, sec_of_day) {
            (Some((y, m, d)), Some(s)) => {
                days_from_civil(y, m, d).map(|days| days as f64 * 86400.0 + s)
            }
            _ => None,
        };

        let reflector = match mini_col(line, 38, 1) {
            Some(field) => mini_reflector(field),
            None => REFLECTOR_UNKNOWN,
        };
        let station = match mini_col(line, 39, 5).and_then(mini_u32) {
            Some(s) => s,
            None => 0,
        };
        let time_of_flight = mini_col(line, 24, 14).and_then(mini_f64).map(|v| v * 1e-13);

        let num_returns = mini_col(line, 44, 3).and_then(mini_u32);
        let return_rate = num_returns.map(|v| v as f64);
        let num_ranges = num_returns;

        let bin_rms_ps = match mini_col(line, 47, 6) {
            Some(field) => {
                let tok = mini_trim(field);
                if tok.is_empty() || mini_all_nines(tok) {
                    None
                } else {
                    mini_f64(tok)
                }
            }
            None => None,
        };

        let signal_to_noise = match mini_col(line, 53, 3) {
            Some(field) => {
                let tok = mini_trim(field);
                if tok.is_empty() || mini_all_nines(tok) {
                    None
                } else {
                    match mini_f64(tok) {
                        Some(v) if v != 0.0 => Some(v / 10.0),
                        _ => None,
                    }
                }
            }
            None => None,
        };

        let mut flags = 0u32;
        if date.is_none() {
            flags |= FLAG_NO_SESSION;
        }
        if sec_of_day.is_none() {
            flags |= FLAG_NO_TIME;
        }
        if reflector == REFLECTOR_UNKNOWN {
            flags |= FLAG_UNKNOWN_REFLECTOR;
        }
        if !station_known_mini(station) {
            flags |= FLAG_UNKNOWN_STATION;
        }
        if let Some(t) = time_of_flight {
            if !(TOF_LO_S..=TOF_HI_S).contains(&t) {
                flags |= FLAG_TOF_RANGE;
            }
        }
        if let Some(e) = epoch_utc {
            if !(EPOCH_LO_UTC..=EPOCH_HI_UTC).contains(&e) {
                flags |= FLAG_TIME_RANGE;
            }
            if let Some(prev) = last_epoch {
                if e < prev {
                    flags |= FLAG_TIME_ORDER;
                }
            }
            last_epoch = Some(match last_epoch {
                Some(p) if p > e => p,
                _ => e,
            });
        }

        out.push(NormalPoint {
            epoch_utc,
            sec_of_day,
            time_of_flight,
            bin_rms_ps,
            return_rate,
            signal_to_noise,
            np_window_length_ns: None,
            num_ranges,
            epoch_event: None,
            detector_channel: None,
            reflector,
            station,
            flags,
        });
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

fn put_f64(out: &mut Vec<u8>, v: Option<f64>, bit: u32, present: &mut u32) -> bool {
    match v {
        Some(x) => {
            if !x.is_finite() {
                return false;
            }
            *present |= bit;
            out.extend_from_slice(&x.to_le_bytes());
        }
        None => out.extend_from_slice(&0.0f64.to_le_bytes()),
    }
    true
}

pub fn write_bin(records: &[NormalPoint]) -> Option<Vec<u8>> {
    let count = u32::try_from(records.len()).ok()?;
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    for r in records {
        let mut present = 0u32;
        if !put_f64(&mut out, r.epoch_utc, P_EPOCH, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.sec_of_day, P_SEC, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.time_of_flight, P_TOF, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.bin_rms_ps, P_BIN_RMS, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.return_rate, P_RETURN_RATE, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.signal_to_noise, P_SNR, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.np_window_length_ns, P_WINDOW, &mut present) {
            return None;
        }
        let num_ranges = match r.num_ranges {
            Some(v) => {
                present |= P_NUM_RANGES;
                v
            }
            None => 0,
        };
        let epoch_event = match r.epoch_event {
            Some(v) => {
                present |= P_EPOCH_EVENT;
                v
            }
            None => 0,
        };
        let detector_channel = match r.detector_channel {
            Some(v) => {
                present |= P_DETECTOR;
                v
            }
            None => 0,
        };
        out.extend_from_slice(&num_ranges.to_le_bytes());
        out.extend_from_slice(&epoch_event.to_le_bytes());
        out.extend_from_slice(&detector_channel.to_le_bytes());
        out.extend_from_slice(&r.reflector.to_le_bytes());
        out.extend_from_slice(&r.station.to_le_bytes());
        out.extend_from_slice(&r.flags.to_le_bytes());
        out.extend_from_slice(&present.to_le_bytes());
    }
    Some(out)
}

struct Reader<'a> {
    bytes: &'a [u8],
    off: usize,
}

impl Reader<'_> {
    fn f64(&mut self) -> Option<f64> {
        let v = f64::from_le_bytes(self.bytes.get(self.off..self.off + 8)?.try_into().ok()?);
        self.off += 8;
        Some(v)
    }

    fn u32(&mut self) -> Option<u32> {
        let v = u32::from_le_bytes(self.bytes.get(self.off..self.off + 4)?.try_into().ok()?);
        self.off += 4;
        Some(v)
    }

    fn i32(&mut self) -> Option<i32> {
        let v = i32::from_le_bytes(self.bytes.get(self.off..self.off + 4)?.try_into().ok()?);
        self.off += 4;
        Some(v)
    }
}

fn present_f64(raw: f64, present: u32, bit: u32) -> Option<Option<f64>> {
    if present & bit != 0 {
        if raw.is_finite() {
            Some(Some(raw))
        } else {
            None
        }
    } else if raw == 0.0 {
        Some(None)
    } else {
        None
    }
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<NormalPoint>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count.checked_mul(RECORD_BYTES)? {
        return None;
    }
    let mut rd = Reader {
        bytes,
        off: HEADER_BYTES,
    };
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let epoch_raw = rd.f64()?;
        let sec_raw = rd.f64()?;
        let tof_raw = rd.f64()?;
        let bin_rms_raw = rd.f64()?;
        let return_rate_raw = rd.f64()?;
        let snr_raw = rd.f64()?;
        let window_raw = rd.f64()?;
        let num_ranges_raw = rd.u32()?;
        let epoch_event_raw = rd.i32()?;
        let detector_channel_raw = rd.i32()?;
        let reflector = rd.u32()?;
        let station = rd.u32()?;
        let flags = rd.u32()?;
        let present = rd.u32()?;

        let num_ranges = if present & P_NUM_RANGES != 0 {
            Some(num_ranges_raw)
        } else if num_ranges_raw == 0 {
            None
        } else {
            return None;
        };
        let epoch_event = if present & P_EPOCH_EVENT != 0 {
            Some(epoch_event_raw)
        } else if epoch_event_raw == 0 {
            None
        } else {
            return None;
        };
        let detector_channel = if present & P_DETECTOR != 0 {
            Some(detector_channel_raw)
        } else if detector_channel_raw == 0 {
            None
        } else {
            return None;
        };

        out.push(NormalPoint {
            epoch_utc: present_f64(epoch_raw, present, P_EPOCH)?,
            sec_of_day: present_f64(sec_raw, present, P_SEC)?,
            time_of_flight: present_f64(tof_raw, present, P_TOF)?,
            bin_rms_ps: present_f64(bin_rms_raw, present, P_BIN_RMS)?,
            return_rate: present_f64(return_rate_raw, present, P_RETURN_RATE)?,
            signal_to_noise: present_f64(snr_raw, present, P_SNR)?,
            np_window_length_ns: present_f64(window_raw, present, P_WINDOW)?,
            num_ranges,
            epoch_event,
            detector_channel,
            reflector,
            station,
            flags,
        });
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let recs = parse_bin(bytes)?;
    let mut out = Vec::with_capacity(recs.len());
    for r in &recs {
        if let (Some(t), Some(tof)) = (r.epoch_utc, r.time_of_flight) {
            out.push((t, tof, r.reflector));
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "h1 CRD  2 2023  4  5 21\n\
h2       APOL 7045 95  1  4       ILRS\n\
h3 apollo15        103  103       na 0 1  3\n\
h4  1 2006  4  7  6 24 27 2006  4  7  6 28 50  4 0 0 0 1 0 2 0\n\
11 23190.0000000 2.6494326499837 std1 2  518.0     44      79.2      na      na       na    na 0   3.5\n\
h8\n";

    fn record() -> NormalPoint {
        NormalPoint {
            epoch_utc: Some(1_143_000_000.0),
            sec_of_day: Some(23190.0),
            time_of_flight: Some(2.6494326499837),
            bin_rms_ps: Some(79.2),
            return_rate: None,
            signal_to_noise: Some(3.5),
            np_window_length_ns: Some(518.0),
            num_ranges: Some(44),
            epoch_event: Some(2),
            detector_channel: Some(0),
            reflector: REFLECTOR_APOLLO15,
            station: 7045,
            flags: 0,
        }
    }

    #[test]
    fn parses_the_measured_normal_point() {
        let recs = parse_crd(SAMPLE.as_bytes()).expect("the sample parses");
        assert_eq!(recs.len(), 1);
        let r = recs[0];
        assert_eq!(r.time_of_flight, Some(2.6494326499837));
        assert_eq!(r.reflector, REFLECTOR_APOLLO15);
        assert_eq!(r.station, 7045);
        assert_eq!(r.num_ranges, Some(44));
        assert_eq!(r.signal_to_noise, Some(3.5));
        assert_eq!(r.bin_rms_ps, Some(79.2));
        assert_eq!(r.return_rate, None);
        assert!(r.epoch_utc.is_some_and(f64::is_finite));
        assert_eq!(r.flags, 0);
    }

    const MINI_SAMPLE: &str =
        "5 19690820025611999999024956468426000071110  8999000  0  79000 120 0 6943 0000";

    #[test]
    fn parses_the_measured_mini_normal_point() {
        let recs = parse_mini(MINI_SAMPLE.as_bytes()).expect("the MINI sample parses");
        assert_eq!(recs.len(), 1);
        let r = recs[0];
        assert!((r.epoch_utc.expect("epoch present") - -11_567_028.000_001).abs() < 1e-3);
        assert!((r.sec_of_day.expect("sec present") - 10571.999999).abs() < 1e-6);
        assert!((r.time_of_flight.expect("tof present") - 2.4956468426).abs() < 1e-12);
        assert_eq!(r.reflector, REFLECTOR_APOLLO11);
        assert_eq!(r.station, 71110);
        assert_eq!(r.num_ranges, Some(8));
        assert_eq!(r.return_rate, Some(8.0));
        assert_eq!(r.bin_rms_ps, None);
        assert_eq!(r.signal_to_noise, None);
        assert_eq!(r.flags & FLAG_TOF_RANGE, 0);
        assert_eq!(r.flags & FLAG_UNKNOWN_REFLECTOR, 0);
        assert_eq!(r.flags & FLAG_UNKNOWN_STATION, 0);
    }

    #[test]
    fn mini_roundtrip_preserves_present_and_absent_fields() {
        let records = parse_mini(MINI_SAMPLE.as_bytes()).expect("the MINI sample parses");
        let bin = write_bin(&records).expect("finite records encode");
        assert_eq!(bin.len(), HEADER_BYTES + RECORD_BYTES);
        assert_eq!(parse_bin(&bin), Some(records));
    }

    #[test]
    fn parse_mini_refuses_foreign_bytes() {
        assert_eq!(parse_mini(b"not a mini file at all"), None);
        assert_eq!(parse_mini(b""), None);
        assert_eq!(parse_mini(SAMPLE.as_bytes()), None);
        assert_eq!(parse_mini(b"LLR \x01\x00\x00\x00"), None);
    }

    #[test]
    fn unknown_reflector_is_marked_not_dropped() {
        let sample = SAMPLE.replace("apollo15", "unknown99");
        let recs = parse_crd(sample.as_bytes()).expect("the sample still parses");
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].reflector, REFLECTOR_UNKNOWN);
        assert_ne!(recs[0].flags & FLAG_UNKNOWN_REFLECTOR, 0);
    }

    #[test]
    fn roundtrip_preserves_present_and_absent_fields() {
        let records = vec![
            record(),
            NormalPoint {
                bin_rms_ps: None,
                signal_to_noise: None,
                num_ranges: None,
                ..record()
            },
        ];
        let bin = write_bin(&records).expect("finite records encode");
        assert_eq!(bin.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bin), Some(records));
    }

    #[test]
    fn parse_bin_refuses_foreign_and_truncated_bytes() {
        assert_eq!(parse_bin(b"LLR "), None);
        assert_eq!(parse_bin(b"XXXX\x01\x00\x00\x00"), None);
        let bin = write_bin(&[record()]).expect("finite record encodes");
        assert_eq!(parse_bin(&bin[..bin.len() - 1]), None);
    }

    #[test]
    fn parse_crd_refuses_foreign_bytes() {
        assert_eq!(parse_crd(b"not a crd file at all"), None);
        assert_eq!(parse_crd(b""), None);
        let without_header = SAMPLE.replace("h1 CRD  2 2023  4  5 21\n", "");
        assert_eq!(parse_crd(without_header.as_bytes()), None);
        let wrong_literal = SAMPLE.replace("h1 CRD", "h1 XYZ");
        assert_eq!(parse_crd(wrong_literal.as_bytes()), None);
    }

    #[test]
    fn parse_series_carries_utc_round_trip_time() {
        let bin = write_bin(&[record()]).expect("finite record encodes");
        let series = parse_series(&bin).expect("the bin reads");
        assert_eq!(series.len(), 1);
        assert_eq!(series[0].1, 2.6494326499837);
        assert_eq!(series[0].2, REFLECTOR_APOLLO15);
    }
}
