use super::lsk::days_from_civil;

pub const MAGIC: [u8; 4] = *b"RAD ";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 52;

pub const TIME_SCALE_UT2: u32 = 0;
pub const TIME_SCALE_UTC: u32 = 1;

const FREQ_HZ_EARLY: f64 = 769.42875e6;
const FREQ_HZ_LATE: f64 = 5010.024e6;

const P_EPOCH: u32 = 1 << 0;
const P_ROUNDTRIP: u32 = 1 << 1;
const P_SIGMA: u32 = 1 << 2;
const P_FREQ: u32 = 1 << 3;
const P_TIME_SCALE: u32 = 1 << 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeScale {
    Ut2,
    Utc,
}

impl TimeScale {
    fn code(self) -> u32 {
        match self {
            TimeScale::Ut2 => TIME_SCALE_UT2,
            TimeScale::Utc => TIME_SCALE_UTC,
        }
    }

    fn from_code(code: u32) -> Option<Self> {
        match code {
            TIME_SCALE_UT2 => Some(TimeScale::Ut2),
            TIME_SCALE_UTC => Some(TimeScale::Utc),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RadarRange {
    pub planet: u8,
    pub xmit_station: u16,
    pub rcvr_station: u16,
    pub epoch_utc: Option<f64>,
    pub roundtrip_us: Option<f64>,
    pub sigma_us: Option<f64>,
    pub time_scale: Option<TimeScale>,
    pub freq_hz: Option<f64>,
}

fn positive(tok: &str) -> Option<f64> {
    let v: f64 = tok.parse().ok()?;
    if v.is_finite() && v > 0.0 {
        Some(v)
    } else {
        None
    }
}

fn freq_hz_for_year(year: i64) -> Option<f64> {
    if year <= 1986 {
        Some(FREQ_HZ_EARLY)
    } else if year >= 1988 {
        Some(FREQ_HZ_LATE)
    } else {
        None
    }
}

fn parse_datetime(field: &str) -> Option<(i64, i64, i64, i64, i64)> {
    let (date, frac) = field.split_once('.')?;
    if date.len() != 8 || !date.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if frac.is_empty() || frac.len() > 4 || !frac.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let year: i64 = date.get(0..4)?.parse().ok()?;
    let month: i64 = date.get(4..6)?.parse().ok()?;
    let day: i64 = date.get(6..8)?.parse().ok()?;
    let mut hhmm = String::with_capacity(4);
    for _ in frac.len()..4 {
        hhmm.push('0');
    }
    hhmm.push_str(frac);
    let hour: i64 = hhmm.get(0..2)?.parse().ok()?;
    let minute: i64 = hhmm.get(2..4)?.parse().ok()?;
    if hour > 23 || minute > 59 {
        return None;
    }
    Some((year, month, day, hour, minute))
}

fn parse_row(line: &str) -> Option<RadarRange> {
    if line.len() < 58 {
        return None;
    }
    let planet: u8 = match line.get(3..4)? {
        "1" => 1,
        "2" => 2,
        "4" => 4,
        _ => return None,
    };
    let xmit_station: u16 = line.get(6..8)?.trim().parse().ok()?;
    let rcvr_station: u16 = line.get(10..12)?.trim().parse().ok()?;
    let (year, month, day, hour, minute) = parse_datetime(line.get(14..27)?.trim())?;
    let time_scale = match line.get(29..32)?.trim() {
        "UT2" => Some(TimeScale::Ut2),
        "UTC" => Some(TimeScale::Utc),
        _ => None,
    };
    let epoch_utc = days_from_civil(year, month, day)
        .map(|days| days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0);
    let roundtrip_us = positive(line.get(35..49)?.trim());
    let sigma_us = positive(line.get(50..58)?.trim());
    let freq_hz = freq_hz_for_year(year);
    Some(RadarRange {
        planet,
        xmit_station,
        rcvr_station,
        epoch_utc,
        roundtrip_us,
        sigma_us,
        time_scale,
        freq_hz,
    })
}

pub fn parse_rad(bytes: &[u8]) -> Option<Vec<RadarRange>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut out: Vec<RadarRange> = Vec::new();
    for raw in text.lines() {
        if let Some(record) = parse_row(raw) {
            out.push(record);
        }
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

pub fn write_bin(records: &[RadarRange]) -> Option<Vec<u8>> {
    let count = u32::try_from(records.len()).ok()?;
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    for r in records {
        let mut present = 0u32;
        if !put_f64(&mut out, r.epoch_utc, P_EPOCH, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.roundtrip_us, P_ROUNDTRIP, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.sigma_us, P_SIGMA, &mut present) {
            return None;
        }
        if !put_f64(&mut out, r.freq_hz, P_FREQ, &mut present) {
            return None;
        }
        let time_scale = match r.time_scale {
            Some(s) => {
                present |= P_TIME_SCALE;
                s.code()
            }
            None => 0,
        };
        out.extend_from_slice(&u32::from(r.planet).to_le_bytes());
        out.extend_from_slice(&u32::from(r.xmit_station).to_le_bytes());
        out.extend_from_slice(&u32::from(r.rcvr_station).to_le_bytes());
        out.extend_from_slice(&time_scale.to_le_bytes());
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

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<RadarRange>> {
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
        let roundtrip_raw = rd.f64()?;
        let sigma_raw = rd.f64()?;
        let freq_raw = rd.f64()?;
        let planet_raw = rd.u32()?;
        let xmit_raw = rd.u32()?;
        let rcvr_raw = rd.u32()?;
        let scale_raw = rd.u32()?;
        let present = rd.u32()?;

        let planet = u8::try_from(planet_raw).ok()?;
        let xmit_station = u16::try_from(xmit_raw).ok()?;
        let rcvr_station = u16::try_from(rcvr_raw).ok()?;
        let time_scale = if present & P_TIME_SCALE != 0 {
            Some(TimeScale::from_code(scale_raw)?)
        } else if scale_raw == 0 {
            None
        } else {
            return None;
        };

        out.push(RadarRange {
            planet,
            xmit_station,
            rcvr_station,
            epoch_utc: present_f64(epoch_raw, present, P_EPOCH)?,
            roundtrip_us: present_f64(roundtrip_raw, present, P_ROUNDTRIP)?,
            sigma_us: present_f64(sigma_raw, present, P_SIGMA)?,
            time_scale,
            freq_hz: present_f64(freq_raw, present, P_FREQ)?,
        });
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        "Venus radar observations from Eupatoria, Crimea\n",
        "\n",
        "   2   8   8  19621021.0956  UT2    337074800.000  120.000\n",
        "   2   8   8  19621021.1010  UT2    337021750.000  120.000\n",
        "   2   8   8  19621021.1026  UT2    336961390.000  120.000\n",
    );

    #[test]
    fn parses_the_measured_venus_rows() {
        let recs = parse_rad(SAMPLE.as_bytes()).expect("the sample parses");
        assert_eq!(recs.len(), 3);
        let r = recs[0];
        assert_eq!(r.planet, 2);
        assert_eq!(r.xmit_station, 8);
        assert_eq!(r.rcvr_station, 8);
        assert_eq!(r.epoch_utc, Some(-227_109_840.0));
        assert_eq!(r.roundtrip_us, Some(337_074_800.0));
        assert_eq!(r.sigma_us, Some(120.0));
        assert_eq!(r.time_scale, Some(TimeScale::Ut2));
        assert_eq!(r.freq_hz, Some(FREQ_HZ_EARLY));
    }

    #[test]
    fn skips_a_title_line_and_keeps_the_data_rows() {
        let recs = parse_rad(SAMPLE.as_bytes()).expect("the sample parses");
        assert_eq!(recs.len(), 3);
        assert_eq!(recs[2].roundtrip_us, Some(336_961_390.0));
    }

    #[test]
    fn late_era_carries_the_5010_mhz_frequency() {
        let sample = concat!(
            "Mars radar observations\n",
            "\n",
            "   4  10  10  19950411.1530  UTC    981256715.600    5.000\n",
        );
        let recs = parse_rad(sample.as_bytes()).expect("the sample parses");
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].time_scale, Some(TimeScale::Utc));
        assert_eq!(recs[0].freq_hz, Some(FREQ_HZ_LATE));
    }

    #[test]
    fn roundtrip_preserves_present_and_absent_fields() {
        let records = parse_rad(SAMPLE.as_bytes()).expect("the sample parses");
        let bin = write_bin(&records).expect("finite records encode");
        assert_eq!(bin.len(), HEADER_BYTES + records.len() * RECORD_BYTES);
        assert_eq!(parse_bin(&bin), Some(records.clone()));

        let mut blank = records[0];
        blank.sigma_us = None;
        blank.freq_hz = None;
        blank.time_scale = None;
        let bin = write_bin(&[blank]).expect("a record with absent fields encodes");
        assert_eq!(parse_bin(&bin), Some(vec![blank]));
    }

    #[test]
    fn parse_rad_refuses_foreign_bytes() {
        assert_eq!(parse_rad(b""), None);
        assert_eq!(parse_rad(b"Venus radar observations\n\n"), None);
        assert_eq!(
            parse_rad(b"not a radar file at all, no fixed columns"),
            None
        );
        assert_eq!(parse_rad(&[0xff, 0xfe, 0x00, 0x01]), None);
    }

    #[test]
    fn parse_bin_refuses_foreign_and_truncated_bytes() {
        assert_eq!(parse_bin(b"RAD "), None);
        assert_eq!(parse_bin(b"XXXX\x01\x00\x00\x00"), None);
        let records = parse_rad(SAMPLE.as_bytes()).expect("the sample parses");
        let bin = write_bin(&records).expect("finite records encode");
        assert_eq!(parse_bin(&bin[..bin.len() - 1]), None);
    }
}
