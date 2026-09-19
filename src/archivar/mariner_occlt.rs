pub const RECORD_HEADER_BYTES: usize = 2;
pub const RECORD_PAYLOAD_BYTES: usize = 4106;
pub const RECORD_BYTES: usize = RECORD_HEADER_BYTES + RECORD_PAYLOAD_BYTES;
pub const SUB_HEADER_BYTES: usize = 10;
pub const SAMPLES_PER_RECORD: usize = RECORD_PAYLOAD_BYTES - SUB_HEADER_BYTES;

pub const PAYLOAD_LENGTH: u16 = 4106;
pub const SUB_HEADER_CONST: [u8; 4] = [0x0a, 0x03, 0x19, 0x1c];

pub const CLOCK_HOUR_MIN: u64 = 36;
pub const CLOCK_HOUR_MAX: u64 = 59;
pub const MINUTE_MAX: u64 = 59;
pub const SECOND_MAX: u64 = 59;

pub const MOCC_STRIDE: usize = 12;

pub const COMP_AMP_MIN: u32 = 1;
pub const COMP_AMP_MAX: u32 = 2;
pub const COMP_AMP_MEAN: u32 = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MarinerOccltRecord {
    pub clock_hh: u64,
    pub clock_mm: u64,
    pub second: u64,
    pub frac: u64,
    pub tag: u64,
    pub amp_min: f64,
    pub amp_max: f64,
    pub amp_mean: f64,
}

pub fn sample(b: u8) -> f64 {
    let mag = f64::from(b & 0x0f);
    if b & 0x20 != 0 { -mag } else { mag }
}

pub fn record(payload: &[u8]) -> Option<MarinerOccltRecord> {
    if payload.len() != RECORD_PAYLOAD_BYTES {
        return None;
    }
    if payload[0..4] != SUB_HEADER_CONST {
        return None;
    }
    if payload[8] != 0 || payload[9] != 0 {
        return None;
    }
    let clock_hh = u64::from(payload[4]);
    if !(CLOCK_HOUR_MIN..=CLOCK_HOUR_MAX).contains(&clock_hh) {
        return None;
    }
    let clock_mm = u64::from(payload[5]);
    if clock_mm > MINUTE_MAX {
        return None;
    }
    let second = u64::from(payload[6]);
    if second > SECOND_MAX {
        return None;
    }
    let frac = u64::from(payload[7]);
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0f64;
    for b in &payload[SUB_HEADER_BYTES..] {
        let v = sample(*b);
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
        sum += v;
    }
    Some(MarinerOccltRecord {
        clock_hh,
        clock_mm,
        second,
        frac,
        tag: u64::from(u32::from_be_bytes([
            payload[4], payload[5], payload[6], payload[7],
        ])),
        amp_min: min,
        amp_max: max,
        amp_mean: sum / SAMPLES_PER_RECORD as f64,
    })
}

pub fn parse(bytes: &[u8]) -> Option<Vec<MarinerOccltRecord>> {
    let mut out = Vec::new();
    let mut offset = 0usize;
    while offset < bytes.len() {
        if offset + RECORD_HEADER_BYTES > bytes.len() {
            return None;
        }
        let data_len = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        if data_len != PAYLOAD_LENGTH {
            return None;
        }
        let end = offset + RECORD_BYTES;
        if end > bytes.len() {
            return None;
        }
        out.push(record(&bytes[offset + RECORD_HEADER_BYTES..end])?);
        offset = end;
    }
    Some(out)
}

pub fn to_bin_row(
    r: &MarinerOccltRecord,
    file_index: u64,
    record_index: u64,
) -> [f64; MOCC_STRIDE] {
    [
        row_epoch(r),
        r.clock_hh as f64,
        r.clock_mm as f64,
        r.second as f64,
        r.frac as f64,
        r.tag as f64,
        SAMPLES_PER_RECORD as f64,
        r.amp_min,
        r.amp_max,
        r.amp_mean,
        file_index as f64,
        record_index as f64,
    ]
}

const DAY_BASE_S: f64 = 1496.0 * 86400.0;

fn row_epoch(r: &MarinerOccltRecord) -> f64 {
    DAY_BASE_S
        + (r.clock_hh as f64 - CLOCK_HOUR_MIN as f64) * 3600.0
        + r.clock_mm as f64 * 60.0
        + r.second as f64
}

pub fn write_mocc_bin(records: &[[f64; MOCC_STRIDE]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * MOCC_STRIDE * 8);
    out.extend_from_slice(b"MOCC");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_mocc_bin(data: &[u8]) -> Option<Vec<[f64; MOCC_STRIDE]>> {
    if data.len() < 8 || &data[0..4] != b"MOCC" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * MOCC_STRIDE * 8 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * MOCC_STRIDE * 8;
        let mut r = [0.0f64; MOCC_STRIDE];
        for k in 0..MOCC_STRIDE {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        if !r.iter().all(|v| v.is_finite()) {
            return None;
        }
        out.push(r);
    }
    Some(out)
}

pub fn parse_series(data: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let rows = parse_mocc_bin(data)?;
    let mut out = Vec::with_capacity(rows.len() * 3);
    for r in &rows {
        if r[0] <= 0.0 {
            return None;
        }
        out.push((r[0], r[7], COMP_AMP_MIN));
        out.push((r[0], r[8], COMP_AMP_MAX));
        out.push((r[0], r[9], COMP_AMP_MEAN));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASURED_SAMPLE_PREFIX: [u8; 16] = [
        0x04, 0x35, 0x37, 0x3d, 0x3e, 0x38, 0x1e, 0x00, 0x3e, 0x09, 0x00, 0x01, 0x03, 0x05, 0x05,
        0x0a,
    ];

    fn measured_record_bytes(fill: u8) -> Vec<u8> {
        let mut buf = Vec::with_capacity(RECORD_BYTES);
        buf.extend_from_slice(&PAYLOAD_LENGTH.to_be_bytes());
        buf.extend_from_slice(&SUB_HEADER_CONST);
        buf.extend_from_slice(&[0x26, 0x04, 0x14, 0x19, 0x00, 0x00]);
        buf.extend_from_slice(&MEASURED_SAMPLE_PREFIX);
        buf.resize(RECORD_BYTES, fill);
        buf
    }

    #[test]
    fn sample_decodes_sign_magnitude() {
        assert_eq!(sample(0x04), 4.0);
        assert_eq!(sample(0x35), -5.0);
        assert_eq!(sample(0x3e), -14.0);
        assert_eq!(sample(0x1e), 14.0);
        assert_eq!(sample(0x2e), -14.0);
        assert_eq!(sample(0x00), 0.0);
        assert_eq!(sample(0x3f), -15.0);
        assert_eq!(sample(0x0f), 15.0);
    }

    #[test]
    fn day_base_matches_calendar() {
        assert_eq!(super::super::ymd_to_days(1974, 2, 5).unwrap(), 1496);
    }

    #[test]
    fn record_extracts_measured_fields() {
        let bytes = measured_record_bytes(0x3e);
        let r = record(&bytes[2..]).unwrap();
        assert_eq!(r.clock_hh, 38);
        assert_eq!(r.clock_mm, 4);
        assert_eq!(r.second, 20);
        assert_eq!(r.frac, 25);
        assert_eq!(r.tag, 0x26041419);
        assert_eq!(r.amp_min, -14.0);
        assert_eq!(r.amp_max, 14.0);
        assert_eq!(r.amp_mean, -57130.0 / SAMPLES_PER_RECORD as f64);
    }

    #[test]
    fn record_rejects_bad_constant_and_pad() {
        let mut bytes = measured_record_bytes(0x3e);
        bytes[2] = 0x00;
        assert!(record(&bytes[2..]).is_none());
        let mut bytes = measured_record_bytes(0x3e);
        bytes[10] = 0x01;
        assert!(record(&bytes[2..]).is_none());
    }

    #[test]
    fn record_rejects_invalid_clock_fields() {
        let mut bytes = measured_record_bytes(0x3e);
        bytes[6] = 0x23;
        assert!(record(&bytes[2..]).is_none());
        let mut bytes = measured_record_bytes(0x3e);
        bytes[6] = 0x3c;
        assert!(record(&bytes[2..]).is_none());
        let mut bytes = measured_record_bytes(0x3e);
        bytes[7] = 0x3c;
        assert!(record(&bytes[2..]).is_none());
        let mut bytes = measured_record_bytes(0x3e);
        bytes[8] = 60;
        assert!(record(&bytes[2..]).is_none());
    }

    #[test]
    fn parse_frames_measured_record() {
        let bytes = measured_record_bytes(0x3e);
        let recs = parse(&bytes).unwrap();
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].clock_hh, 38);
        assert_eq!(recs[0].tag, 0x26041419);
    }

    #[test]
    fn parse_rejects_truncated_and_bad_length() {
        assert!(parse(b"X").is_none());
        let bytes = measured_record_bytes(0x3e);
        assert!(parse(&bytes[..bytes.len() - 1]).is_none());
        let mut bad = measured_record_bytes(0x3e);
        bad[0] = 0x10;
        bad[1] = 0x0b;
        assert!(parse(&bad).is_none());
    }

    #[test]
    fn mocc_bin_roundtrip() {
        let recs = vec![[0.0; MOCC_STRIDE]];
        let bytes = write_mocc_bin(&recs);
        let parsed = parse_mocc_bin(&bytes).unwrap();
        assert_eq!(parsed, recs);
        assert!(parse_mocc_bin(b"X").is_none());
    }

    #[test]
    fn series_dispatch_names_measured_components() {
        let bytes = measured_record_bytes(0x3e);
        let r = record(&bytes[2..]).unwrap();
        let bin = write_mocc_bin(&[to_bin_row(&r, 0, 0)]);
        let series = parse_series(&bin).unwrap();
        assert_eq!(series.len(), 3);
        let expected_t = super::super::ymd_to_days(1974, 2, 5).unwrap() as f64 * 86400.0
            + 2.0 * 3600.0
            + 4.0 * 60.0
            + 20.0;
        assert_eq!(series[0], (expected_t, -14.0, COMP_AMP_MIN));
        assert_eq!(series[1], (expected_t, 14.0, COMP_AMP_MAX));
        assert_eq!(series[2], (expected_t, -57130.0 / 4096.0, COMP_AMP_MEAN));
    }

    #[test]
    fn series_rejects_non_positive_epoch() {
        let mut row = [1.0; MOCC_STRIDE];
        row[0] = 0.0;
        let bin = write_mocc_bin(&[row]);
        assert!(parse_series(&bin).is_none());
    }
}
