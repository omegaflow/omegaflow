pub const RECORD_BYTES: usize = 4260;
pub const SFDU_LABEL_BYTES: usize = 20;
pub const SAMPLE_RESOLUTION_OFFSET: usize = 68;
pub const SAMPLE_RATE_OFFSET: usize = 70;
pub const YEAR_OFFSET: usize = 76;
pub const DOY_OFFSET: usize = 78;
pub const SECOND_OFFSET: usize = 80;
pub const DATA_CHDO_OFFSET: usize = 256;
pub const DATA_CHDO_TYPE: u16 = 10;
pub const DATA_CHDO_LENGTH_OFFSET: usize = 258;
pub const SAMPLE_WORDS_OFFSET: usize = 260;
pub const DATA_WORDS: usize = 1000;
pub const SFDU_CONTROL_AUTHORITY: [u8; 4] = *b"NJPL";
pub const SFDU_DATA_DESCRIPTION_ID: [u8; 4] = *b"C997";

pub const MAGIC: [u8; 4] = *b"CRSR";
pub const COMP_I: u32 = 1;
pub const COMP_Q: u32 = 2;

pub const SHARD_BUDGET: usize = 1 << 31;

#[derive(Clone, Debug, PartialEq)]
pub struct RsrRecord {
    pub year: u16,
    pub doy: u16,
    pub second: f64,
    pub rate_ksps: u16,
    pub resolution_bits: u8,
    pub samples: Vec<(i16, i16)>,
}

fn be16(bytes: &[u8], i: usize) -> u16 {
    u16::from_be_bytes([bytes[i], bytes[i + 1]])
}

fn be32(bytes: &[u8], i: usize) -> u32 {
    u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]])
}

pub fn label_data_bytes(rec: &[u8]) -> Option<usize> {
    if rec.len() < SAMPLE_WORDS_OFFSET {
        return None;
    }
    let data_bytes = be16(rec, DATA_CHDO_LENGTH_OFFSET) as usize;
    if data_bytes < 4 || !data_bytes.is_multiple_of(4) {
        return None;
    }
    Some(data_bytes)
}

pub fn record_bytes(rec: &[u8]) -> Option<usize> {
    SAMPLE_WORDS_OFFSET.checked_add(label_data_bytes(rec)?)
}

pub fn read_record(rec: &[u8]) -> Option<RsrRecord> {
    let data_bytes = label_data_bytes(rec)?;
    if rec.len() < SAMPLE_WORDS_OFFSET.checked_add(data_bytes)? {
        return None;
    }
    if rec[0..4] != SFDU_CONTROL_AUTHORITY {
        return None;
    }
    if rec[8..12] != SFDU_DATA_DESCRIPTION_ID {
        return None;
    }
    if be16(rec, DATA_CHDO_OFFSET) != DATA_CHDO_TYPE {
        return None;
    }
    let rate_ksps = be16(rec, SAMPLE_RATE_OFFSET);
    if rate_ksps == 0 {
        return None;
    }
    let resolution_bits = rec[SAMPLE_RESOLUTION_OFFSET];
    let year = be16(rec, YEAR_OFFSET);
    let doy = be16(rec, DOY_OFFSET);
    let second = f64::from_be_bytes(rec[SECOND_OFFSET..SECOND_OFFSET + 8].try_into().ok()?);
    if !(1..=366).contains(&doy) || !second.is_finite() {
        return None;
    }
    let words = data_bytes / 4;
    let mut samples = Vec::with_capacity(words);
    for w in 0..words {
        let word = be32(rec, SAMPLE_WORDS_OFFSET + w * 4);
        let q = (word >> 16) as u16 as i16;
        let i = (word & 0xFFFF) as u16 as i16;
        samples.push((i, q));
    }
    Some(RsrRecord {
        year,
        doy,
        second,
        rate_ksps,
        resolution_bits,
        samples,
    })
}

pub fn parse_records(bytes: &[u8]) -> Option<Vec<RsrRecord>> {
    let stride = record_bytes(bytes)?;
    if !bytes.len().is_multiple_of(stride) {
        return None;
    }
    let n = bytes.len() / stride;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(read_record(&bytes[i * stride..(i + 1) * stride])?);
    }
    Some(out)
}

pub fn record_unix(rec: &RsrRecord) -> Option<f64> {
    let days = crate::lsk::days_from_civil(rec.year as i64, 1, 1)?;
    Some((days + rec.doy as i64 - 1) as f64 * 86400.0 + rec.second)
}

pub fn sample_interval_s(rec: &RsrRecord) -> Option<f64> {
    if rec.rate_ksps == 0 {
        return None;
    }
    Some(1.0 / (f64::from(rec.rate_ksps) * 1000.0))
}

pub fn series(records: &[RsrRecord], lsk: &crate::lsk::LeapSeconds) -> Vec<(f64, f64, u32)> {
    let mut out: Vec<(f64, f64, u32)> = Vec::new();
    for rec in records {
        let Some(unix) = record_unix(rec) else {
            continue;
        };
        let Some(t0) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let Some(dt) = sample_interval_s(rec) else {
            continue;
        };
        for (i, (i_s, q_s)) in rec.samples.iter().enumerate() {
            let t = t0 + i as f64 * dt;
            out.push((t, f64::from(*i_s), COMP_I));
            out.push((t, f64::from(*q_s), COMP_Q));
        }
    }
    out
}

pub fn write_series(rows: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + rows.len() * 20);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(rows.len() as u32).to_le_bytes());
    for (t, v, c) in rows {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&v.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
    }
    out
}

pub fn parse_series(data: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if data.len() < 8 || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 20 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 20;
        let t = f64::from_le_bytes(data.get(base..base + 8)?.try_into().ok()?);
        let v = f64::from_le_bytes(data.get(base + 8..base + 16)?.try_into().ok()?);
        let c = u32::from_le_bytes(data.get(base + 16..base + 20)?.try_into().ok()?);
        if !t.is_finite() || !v.is_finite() {
            return None;
        }
        out.push((t, v, c));
    }
    Some(out)
}

pub fn shard_name(prefix: &str, ord: usize) -> String {
    format!("{prefix}_s{ord}.bin")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measured_header() -> [u8; 88] {
        [
            0x4e, 0x4a, 0x50, 0x4c, 0x32, 0x49, 0x30, 0x30, 0x43, 0x39, 0x39, 0x37, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x10, 0x90, 0x00, 0x01, 0x00, 0xe8, 0x00, 0x02, 0x00, 0x04,
            0x15, 0x04, 0xff, 0x00, 0x00, 0x68, 0x00, 0xdc, 0x30, 0x30, 0x0a, 0xad, 0x00, 0x00,
            0x0a, 0x0e, 0x05, 0x01, 0x00, 0x52, 0x0b, 0x16, 0x58, 0x58, 0x01, 0x00, 0x32, 0x6e,
            0x00, 0x18, 0x18, 0x5e, 0x07, 0xd5, 0x00, 0xc4, 0x00, 0x00, 0xde, 0x93, 0x10, 0x00,
            0x00, 0x01, 0x01, 0x47, 0x1f, 0xa4, 0x07, 0xd5, 0x00, 0xc4, 0x40, 0xeb, 0xd5, 0x20,
            0x00, 0x00, 0x00, 0x00,
        ]
    }

    fn sample_record() -> Vec<u8> {
        let mut r = vec![0u8; RECORD_BYTES];
        r[..88].copy_from_slice(&measured_header());
        r[DATA_CHDO_OFFSET..DATA_CHDO_OFFSET + 2].copy_from_slice(&DATA_CHDO_TYPE.to_be_bytes());
        r[DATA_CHDO_OFFSET + 2..DATA_CHDO_OFFSET + 4].copy_from_slice(&4000u16.to_be_bytes());
        for w in 0..DATA_WORDS {
            let i = (w as i16).wrapping_sub(500);
            let q = 500i16.wrapping_sub(w as i16);
            let word = ((q as u16 as u32) << 16) | (i as u16 as u32);
            let o = SAMPLE_WORDS_OFFSET + w * 4;
            r[o..o + 4].copy_from_slice(&word.to_be_bytes());
        }
        r
    }

    fn gll_record() -> Vec<u8> {
        let data_bytes = 8000usize;
        let mut r = vec![0u8; SAMPLE_WORDS_OFFSET + data_bytes];
        r[..88].copy_from_slice(&measured_header());
        r[DATA_CHDO_OFFSET..DATA_CHDO_OFFSET + 2].copy_from_slice(&DATA_CHDO_TYPE.to_be_bytes());
        r[DATA_CHDO_LENGTH_OFFSET..DATA_CHDO_LENGTH_OFFSET + 2]
            .copy_from_slice(&(data_bytes as u16).to_be_bytes());
        r
    }

    #[test]
    fn decodes_measured_record_one() {
        let rec = read_record(&sample_record()).unwrap();
        assert_eq!(rec.year, 2005);
        assert_eq!(rec.doy, 196);
        assert_eq!(rec.rate_ksps, 1);
        assert_eq!(rec.resolution_bits, 16);
        assert_eq!(rec.second, 57001.0);
        assert_eq!(rec.samples.len(), DATA_WORDS);
        assert_eq!(rec.samples[0], (-500, 500));
        assert_eq!(rec.samples[1], (-499, 499));
    }

    #[test]
    fn sample_interval_is_reciprocal_kilosample_rate() {
        let rec = read_record(&sample_record()).unwrap();
        assert_eq!(sample_interval_s(&rec), Some(0.001));
    }

    #[test]
    fn parse_records_counts_complete_records() {
        let mut bytes = sample_record();
        bytes.extend_from_slice(&sample_record());
        let recs = parse_records(&bytes).unwrap();
        assert_eq!(recs.len(), 2);
        assert!(parse_records(&bytes[..RECORD_BYTES - 1]).is_none());
    }

    #[test]
    fn zero_rate_record_stays_unread() {
        let mut bytes = sample_record();
        bytes[SAMPLE_RATE_OFFSET..SAMPLE_RATE_OFFSET + 2].copy_from_slice(&0u16.to_be_bytes());
        assert!(read_record(&bytes).is_none());
    }

    #[test]
    fn series_emits_i_and_q_at_sample_rate() {
        let recs = parse_records(&sample_record()).unwrap();
        let lsk = crate::archivar::membrane::embedded_lsk().unwrap();
        let rows = series(&recs, &lsk);
        assert_eq!(rows.len(), 2 * DATA_WORDS);
        assert_eq!(rows[0].2, COMP_I);
        assert_eq!(rows[0].1, -500.0);
        assert_eq!(rows[1].2, COMP_Q);
        assert_eq!(rows[1].1, 500.0);
        let tol = 8.0 * f64::EPSILON * rows[2].0.abs();
        assert!((rows[2].0 - rows[0].0 - 0.001).abs() < tol);
    }

    #[test]
    fn series_roundtrip() {
        let rows = vec![(1.5e9, -3.0, COMP_I), (1.5e9, 7.0, COMP_Q)];
        let bytes = write_series(&rows);
        assert_eq!(parse_series(&bytes).unwrap(), rows);
        assert!(parse_series(b"X").is_none());
    }

    #[test]
    fn record_bytes_come_from_the_label() {
        assert_eq!(record_bytes(&sample_record()), Some(RECORD_BYTES));
        assert_eq!(record_bytes(&gll_record()), Some(8260));
    }

    #[test]
    fn gll_sized_record_decodes_two_thousand_samples() {
        let rec = read_record(&gll_record()).unwrap();
        assert_eq!(rec.samples.len(), 2000);
    }

    #[test]
    fn parse_records_counts_gll_sized_records() {
        let mut bytes = gll_record();
        bytes.extend_from_slice(&gll_record());
        let recs = parse_records(&bytes).unwrap();
        assert_eq!(recs.len(), 2);
    }

    #[test]
    fn absent_data_length_reads_none() {
        let mut r = sample_record();
        r[DATA_CHDO_LENGTH_OFFSET..DATA_CHDO_LENGTH_OFFSET + 2]
            .copy_from_slice(&0u16.to_be_bytes());
        assert!(label_data_bytes(&r).is_none());
        assert!(record_bytes(&r).is_none());
        assert!(read_record(&r).is_none());

        r[DATA_CHDO_LENGTH_OFFSET..DATA_CHDO_LENGTH_OFFSET + 2]
            .copy_from_slice(&4002u16.to_be_bytes());
        assert!(label_data_bytes(&r).is_none());
        assert!(read_record(&r).is_none());
    }
}
