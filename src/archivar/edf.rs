pub const HEADER_FIXED_BYTES: usize = 256;
pub const SIGNAL_HEADER_BYTES: usize = 256;

#[derive(Clone, Debug, PartialEq)]
pub struct EdfSignal {
    pub label: String,
    pub transducer: String,
    pub unit: String,
    pub physical_min: f64,
    pub physical_max: f64,
    pub digital_min: f64,
    pub digital_max: f64,
    pub prefiltering: String,
    pub samples_per_record: usize,
    pub sample_bytes: usize,
    pub reserved: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EdfHeader {
    pub version: String,
    pub patient_id: String,
    pub recording_id: String,
    pub start_date: String,
    pub start_time: String,
    pub header_bytes: usize,
    pub reserved: String,
    pub data_records: i64,
    pub record_duration_s: f64,
    pub signals: Vec<EdfSignal>,
}

impl EdfHeader {
    pub fn record_bytes(&self) -> usize {
        self.signals
            .iter()
            .map(|s| s.samples_per_record * s.sample_bytes)
            .sum()
    }
}

fn field(b: &[u8], start: usize, len: usize) -> Option<&[u8]> {
    b.get(start..start.checked_add(len)?)
}

fn text(b: &[u8], start: usize, len: usize) -> Option<String> {
    Some(
        String::from_utf8_lossy(field(b, start, len)?)
            .trim()
            .to_string(),
    )
}

fn integer(b: &[u8], start: usize, len: usize) -> Option<i64> {
    let s = std::str::from_utf8(field(b, start, len)?).ok()?.trim();
    if s.is_empty() {
        return None;
    }
    s.parse().ok()
}

fn number(b: &[u8], start: usize, len: usize) -> Option<f64> {
    let s = std::str::from_utf8(field(b, start, len)?).ok()?.trim();
    if s.is_empty() {
        return None;
    }
    s.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn physical_value(signal: &EdfSignal, digital: f64) -> Option<f64> {
    if signal.digital_max == signal.digital_min {
        return None;
    }
    let value = signal.physical_min
        + (digital - signal.digital_min) * (signal.physical_max - signal.physical_min)
            / (signal.digital_max - signal.digital_min);
    value.is_finite().then_some(value)
}

fn read_digital(b: &[u8], off: usize, bytes: usize) -> Option<f64> {
    match bytes {
        2 => Some(i16::from_le_bytes([*b.get(off)?, *b.get(off + 1)?]) as f64),
        3 => {
            let b0 = *b.get(off)? as i32;
            let b1 = *b.get(off + 1)? as i32;
            let b2 = *b.get(off + 2)? as i32;
            let raw = b0 | (b1 << 8) | (b2 << 16);
            let signed = if raw & 0x80_0000 != 0 {
                raw - 0x100_0000
            } else {
                raw
            };
            Some(signed as f64)
        }
        _ => None,
    }
}

pub fn parse_edf(bytes: &[u8]) -> Option<EdfHeader> {
    let ns = integer(bytes, 252, 4)?;
    if ns < 1 {
        return None;
    }
    let ns = ns as usize;
    let header_bytes = integer(bytes, 184, 8)?;
    if header_bytes < 0 {
        return None;
    }
    let expected = HEADER_FIXED_BYTES.checked_add(ns.checked_mul(SIGNAL_HEADER_BYTES)?)?;
    if header_bytes as usize != expected || bytes.len() < expected {
        return None;
    }
    let data_records = integer(bytes, 236, 8)?;
    let record_duration_s = number(bytes, 244, 8)?;

    let labels = HEADER_FIXED_BYTES;
    let transducers = labels.checked_add(ns.checked_mul(16)?)?;
    let units = transducers.checked_add(ns.checked_mul(80)?)?;
    let physical_min = units.checked_add(ns.checked_mul(8)?)?;
    let physical_max = physical_min.checked_add(ns.checked_mul(8)?)?;
    let digital_min = physical_max.checked_add(ns.checked_mul(8)?)?;
    let digital_max = digital_min.checked_add(ns.checked_mul(8)?)?;
    let prefiltering = digital_max.checked_add(ns.checked_mul(8)?)?;
    let samples = prefiltering.checked_add(ns.checked_mul(80)?)?;
    let reserved = samples.checked_add(ns.checked_mul(8)?)?;

    let mut signals = Vec::with_capacity(ns);
    for i in 0..ns {
        let step = i.checked_mul(8)?;
        let label = text(bytes, labels.checked_add(i.checked_mul(16)?)?, 16)?;
        let transducer = text(bytes, transducers.checked_add(i.checked_mul(80)?)?, 80)?;
        let unit = text(bytes, units.checked_add(step)?, 8)?;
        let p_min = number(bytes, physical_min.checked_add(step)?, 8)?;
        let p_max = number(bytes, physical_max.checked_add(step)?, 8)?;
        if p_max <= p_min {
            return None;
        }
        let d_min = number(bytes, digital_min.checked_add(step)?, 8)?;
        let d_max = number(bytes, digital_max.checked_add(step)?, 8)?;
        let pre = text(bytes, prefiltering.checked_add(i.checked_mul(80)?)?, 80)?;
        let per_signal = integer(bytes, samples.checked_add(step)?, 8)?;
        let (samples_per_record, sample_bytes) = if per_signal < 0 {
            ((-per_signal) as usize, 3)
        } else {
            (per_signal as usize, 2)
        };
        let sig_reserved = text(bytes, reserved.checked_add(i.checked_mul(32)?)?, 32)?;
        signals.push(EdfSignal {
            label,
            transducer,
            unit,
            physical_min: p_min,
            physical_max: p_max,
            digital_min: d_min,
            digital_max: d_max,
            prefiltering: pre,
            samples_per_record,
            sample_bytes,
            reserved: sig_reserved,
        });
    }

    Some(EdfHeader {
        version: text(bytes, 0, 8)?,
        patient_id: text(bytes, 8, 80)?,
        recording_id: text(bytes, 88, 80)?,
        start_date: text(bytes, 168, 8)?,
        start_time: text(bytes, 176, 8)?,
        header_bytes: header_bytes as usize,
        reserved: text(bytes, 192, 44)?,
        data_records,
        record_duration_s,
        signals,
    })
}

pub fn signal_samples(bytes: &[u8], header: &EdfHeader, signal_index: usize) -> Vec<Option<f64>> {
    let Some(signal) = header.signals.get(signal_index) else {
        return Vec::new();
    };
    let record_bytes = header.record_bytes();
    if record_bytes == 0 {
        return Vec::new();
    }
    let data_start = header.header_bytes;
    let available = bytes.len().saturating_sub(data_start);
    let records = available / record_bytes;
    if records == 0 {
        return Vec::new();
    }
    let signal_offset: usize = header.signals[..signal_index]
        .iter()
        .map(|s| s.samples_per_record * s.sample_bytes)
        .sum();
    let mut out = Vec::with_capacity(records * signal.samples_per_record);
    for r in 0..records {
        let base = data_start + r * record_bytes + signal_offset;
        for s in 0..signal.samples_per_record {
            let off = base + s * signal.sample_bytes;
            match read_digital(bytes, off, signal.sample_bytes) {
                Some(digital) => out.push(physical_value(signal, digital)),
                None => out.push(None),
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put(v: &mut Vec<u8>, s: &str, len: usize) {
        let mut f = s.as_bytes().to_vec();
        f.resize(len, b' ');
        v.extend_from_slice(&f);
    }

    fn write_field(b: &mut [u8], at: usize, s: &str, len: usize) {
        let mut f = s.as_bytes().to_vec();
        f.resize(len, b' ');
        b[at..at + len].copy_from_slice(&f);
    }

    fn synth_edf() -> Vec<u8> {
        let mut h = Vec::new();
        put(&mut h, "0", 8);
        put(&mut h, "patient", 80);
        put(&mut h, "recording", 80);
        put(&mut h, "01.01.20", 8);
        put(&mut h, "00.00.00", 8);
        put(&mut h, "768", 8);
        put(&mut h, "EDF+C", 44);
        put(&mut h, "1", 8);
        put(&mut h, "1", 8);
        put(&mut h, "2", 4);
        put(&mut h, "EEG Fpz-Cz", 16);
        put(&mut h, "EOG", 16);
        put(&mut h, "AgAgCl", 80);
        put(&mut h, "AgAgCl", 80);
        put(&mut h, "uV", 8);
        put(&mut h, "mV", 8);
        put(&mut h, "0", 8);
        put(&mut h, "-1", 8);
        put(&mut h, "10", 8);
        put(&mut h, "1", 8);
        put(&mut h, "0", 8);
        put(&mut h, "-100", 8);
        put(&mut h, "100", 8);
        put(&mut h, "100", 8);
        put(&mut h, "", 80);
        put(&mut h, "", 80);
        put(&mut h, "4", 8);
        put(&mut h, "2", 8);
        put(&mut h, "", 32);
        put(&mut h, "", 32);
        for v in [0i16, 25, 50, 100] {
            h.extend_from_slice(&v.to_le_bytes());
        }
        for v in [0i16, 100] {
            h.extend_from_slice(&v.to_le_bytes());
        }
        h
    }

    #[test]
    fn edf_parses_header_and_signals() {
        let bytes = synth_edf();
        let header = parse_edf(&bytes).expect("synthetic EDF header");
        assert_eq!(header.signals.len(), 2);
        assert_eq!(header.signals[0].label, "EEG Fpz-Cz");
        assert_eq!(header.signals[1].label, "EOG");
        assert_eq!(header.signals[0].unit, "uV");
        assert_eq!(header.signals[1].unit, "mV");
        assert_eq!(header.signals[0].samples_per_record, 4);
        assert_eq!(header.signals[0].sample_bytes, 2);
        assert_eq!(header.record_duration_s, 1.0);
        assert_eq!(header.record_bytes(), 12);
    }

    #[test]
    fn edf_converts_digital_sample_to_physical() {
        let bytes = synth_edf();
        let header = parse_edf(&bytes).unwrap();
        assert_eq!(
            signal_samples(&bytes, &header, 0),
            vec![Some(0.0), Some(2.5), Some(5.0), Some(10.0)]
        );
        assert_eq!(
            signal_samples(&bytes, &header, 1),
            vec![Some(0.0), Some(1.0)]
        );
    }

    #[test]
    fn edf_reads_three_byte_samples() {
        let mut h = Vec::new();
        put(&mut h, "0", 8);
        put(&mut h, "patient", 80);
        put(&mut h, "recording", 80);
        put(&mut h, "01.01.20", 8);
        put(&mut h, "00.00.00", 8);
        put(&mut h, "512", 8);
        put(&mut h, "EDF+C", 44);
        put(&mut h, "1", 8);
        put(&mut h, "1", 8);
        put(&mut h, "1", 4);
        put(&mut h, "Cz", 16);
        put(&mut h, "", 80);
        put(&mut h, "uV", 8);
        put(&mut h, "0", 8);
        put(&mut h, "100", 8);
        put(&mut h, "0", 8);
        put(&mut h, "100", 8);
        put(&mut h, "", 80);
        put(&mut h, "-2", 8);
        put(&mut h, "", 32);
        h.extend_from_slice(&[0xff, 0xff, 0xff]);
        h.extend_from_slice(&[0x05, 0x00, 0x00]);
        let header = parse_edf(&h).expect("synthetic 24-bit EDF header");
        assert_eq!(header.signals[0].sample_bytes, 3);
        assert_eq!(header.signals[0].samples_per_record, 2);
        assert_eq!(signal_samples(&h, &header, 0), vec![Some(-1.0), Some(5.0)]);
    }

    #[test]
    fn edf_malformed_header_is_absent() {
        assert!(parse_edf(&[]).is_none());
        let mut bytes = synth_edf();
        write_field(&mut bytes, 480, "0", 8);
        assert!(parse_edf(&bytes).is_none());
    }
}
