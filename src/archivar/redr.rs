pub const RECORD_BYTES: usize = 1692;
pub const HEADER_BYTES: usize = 12;
pub const SAMPLE_BYTES: usize = 1600;
pub const TRAILER_BYTES: usize = 80;
pub const SAMPLES_PER_RECORD: usize = 200;
pub const X_SAMPLES_PER_RECORD: usize = 600;
pub const SAMPLE_RATE_HZ: u32 = 10000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RedrHeader {
    pub year: u8,
    pub day: u16,
    pub hour: u8,
    pub minute: u8,
    pub second: f64,
    pub validity: u8,
    pub sample_rate: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RedrRecord {
    pub header: RedrHeader,
    pub s_band: [i8; SAMPLES_PER_RECORD],
    pub x_band: [i8; X_SAMPLES_PER_RECORD],
}

impl RedrHeader {
    pub fn valid(&self) -> bool {
        self.validity == 0
    }

    pub fn recreated(&self) -> bool {
        self.validity == 2
    }
}

pub fn header(record: &[u8]) -> Option<RedrHeader> {
    if record.len() < HEADER_BYTES {
        return None;
    }
    let second = u16::from_be_bytes([record[5], record[6]]);
    let rate = u32::from_be_bytes([record[8], record[9], record[10], record[11]]);
    Some(RedrHeader {
        year: record[0],
        day: u16::from_be_bytes([record[1], record[2]]),
        hour: record[3],
        minute: record[4],
        second: second as f64 / 100.0,
        validity: record[7],
        sample_rate: rate,
    })
}

pub fn record(bytes: &[u8]) -> Option<RedrRecord> {
    if bytes.len() < RECORD_BYTES {
        return None;
    }
    let header = header(bytes)?;
    let mut s_band = [0i8; SAMPLES_PER_RECORD];
    let mut x_band = [0i8; X_SAMPLES_PER_RECORD];
    let data = &bytes[HEADER_BYTES..HEADER_BYTES + SAMPLE_BYTES];
    for group in 0..SAMPLES_PER_RECORD {
        let at = group * 8;
        s_band[group] = data[at] as i8;
        x_band[group * 3] = data[at + 2] as i8;
        x_band[group * 3 + 1] = data[at + 4] as i8;
        x_band[group * 3 + 2] = data[at + 6] as i8;
    }
    Some(RedrRecord {
        header,
        s_band,
        x_band,
    })
}

pub fn parse(bytes: &[u8]) -> Option<Vec<RedrRecord>> {
    if !bytes.len().is_multiple_of(RECORD_BYTES) {
        return None;
    }
    let mut out = Vec::with_capacity(bytes.len() / RECORD_BYTES);
    for chunk in bytes.chunks_exact(RECORD_BYTES) {
        out.push(record(chunk)?);
    }
    Some(out)
}

pub fn spacecraft_id(record: &[u8]) -> Option<u8> {
    let at = 1675 - 1;
    record.get(at).copied()
}

pub fn station_id(record: &[u8]) -> Option<u8> {
    let at = 1676 - 1;
    record.get(at).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic(validity: u8) -> Vec<u8> {
        let mut bytes = vec![0u8; RECORD_BYTES];
        bytes[0] = 79;
        bytes[1] = 6;
        bytes[2] = 4;
        bytes[3] = 18;
        bytes[4] = 46;
        bytes[5..7].copy_from_slice(&2000u16.to_be_bytes());
        bytes[7] = validity;
        bytes[8..12].copy_from_slice(&SAMPLE_RATE_HZ.to_be_bytes());
        for group in 0..SAMPLES_PER_RECORD {
            let at = HEADER_BYTES + group * 8;
            bytes[at] = (group as i16 - 100) as i8 as u8;
            bytes[at + 2] = 1;
            bytes[at + 4] = 2;
            bytes[at + 6] = 3;
        }
        bytes[1674] = 31;
        bytes[1675] = 63;
        bytes
    }

    #[test]
    fn the_record_is_1692_bytes_and_ten_thousand_fit_the_jupiter_file() {
        assert_eq!(HEADER_BYTES + SAMPLE_BYTES + TRAILER_BYTES, RECORD_BYTES);
        assert_eq!(RECORD_BYTES * 10_000, 16_920_000);
    }

    #[test]
    fn a_record_reads_its_time_tag_and_rate() {
        let r = record(&synthetic(0)).expect("a record");
        assert_eq!(r.header.year, 79);
        assert_eq!(r.header.day, 1540);
        assert_eq!(r.header.hour, 18);
        assert_eq!(r.header.minute, 46);
        assert!((r.header.second - 20.0).abs() < 1.0e-9);
        assert_eq!(r.header.sample_rate, SAMPLE_RATE_HZ);
        assert!(r.header.valid());
    }

    #[test]
    fn the_three_x_band_samples_trail_each_s_band_sample() {
        let r = record(&synthetic(0)).expect("a record");
        assert_eq!(r.s_band[0], -100);
        assert_eq!(r.s_band[199], 99);
        assert_eq!(r.x_band[0], 1);
        assert_eq!(r.x_band[599], 3);
    }

    #[test]
    fn a_recreated_record_is_named_not_silently_accepted() {
        let r = record(&synthetic(2)).expect("a record");
        assert!(!r.header.valid());
        assert!(r.header.recreated());
    }

    #[test]
    fn a_partial_record_is_absent() {
        let mut bytes = synthetic(0);
        bytes.pop();
        assert!(parse(&bytes).is_none());
    }

    #[test]
    fn the_trailer_names_the_spacecraft_and_the_station() {
        let bytes = synthetic(0);
        assert_eq!(spacecraft_id(&bytes), Some(31));
        assert_eq!(station_id(&bytes), Some(63));
    }

    const RECORD0: &str = include_str!("kernels/redr_record0.hex");

    fn fixture() -> Vec<u8> {
        let mut out = Vec::new();
        for line in RECORD0.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            for token in line.split_whitespace() {
                if let Ok(byte) = u8::from_str_radix(token, 16) {
                    out.push(byte);
                }
            }
        }
        out
    }

    #[test]
    fn the_first_record_of_the_jupiter_file_reads_as_1979_day_64() {
        let bytes = fixture();
        assert_eq!(bytes.len(), RECORD_BYTES);
        let r = record(&bytes).expect("the frozen record");
        assert_eq!(r.header.year, 79);
        assert_eq!(r.header.day, 64);
        assert_eq!(r.header.hour, 16);
        assert_eq!(r.header.minute, 3);
        assert!(r.header.second.abs() < 1.0e-9);
        assert!(r.header.valid());
        assert_eq!(r.header.sample_rate, SAMPLE_RATE_HZ);
        assert_eq!(r.s_band[0], -36);
        assert_eq!(spacecraft_id(&bytes), Some(31));
        assert_eq!(station_id(&bytes), Some(63));
    }
}
