pub const RECORD_BYTES: usize = 2666;
pub const HEADER_BYTES: usize = 166;
pub const AD_GROUP_BYTES: usize = 4;
pub const AD_REPETITIONS: usize = 625;
pub const DATA_BYTES: usize = AD_REPETITIONS * AD_GROUP_BYTES;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OdrHeader {
    pub record_number: u16,
    pub record_words: u16,
    pub spacecraft: u8,
    pub spc: u8,
    pub year: u8,
    pub doy: u16,
    pub time_tag_ms: u32,
    pub sample_rate: u16,
}

#[derive(Clone, Debug)]
pub struct OdrRecord {
    pub header: OdrHeader,
    pub ad: [[u8; AD_GROUP_BYTES]; AD_REPETITIONS],
}

fn be16(bytes: &[u8], i: usize) -> u16 {
    u16::from_be_bytes([bytes[i], bytes[i + 1]])
}

fn be32(bytes: &[u8], i: usize) -> u32 {
    u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]])
}

pub fn header(bytes: &[u8]) -> Option<OdrHeader> {
    if bytes.len() < HEADER_BYTES {
        return None;
    }
    let date = be16(bytes, 10);
    Some(OdrHeader {
        record_number: be16(bytes, 2),
        record_words: be16(bytes, 4),
        spacecraft: bytes[8],
        spc: bytes[9],
        year: (date >> 9) as u8,
        doy: date & 0x1FF,
        time_tag_ms: be32(bytes, 12),
        sample_rate: be16(bytes, 158),
    })
}

pub fn record(bytes: &[u8]) -> Option<OdrRecord> {
    if bytes.len() < RECORD_BYTES {
        return None;
    }
    let header = header(bytes)?;
    let mut ad = [[0u8; AD_GROUP_BYTES]; AD_REPETITIONS];
    for i in 0..AD_REPETITIONS {
        let base = HEADER_BYTES + i * AD_GROUP_BYTES;
        for k in 0..AD_GROUP_BYTES {
            ad[i][k] = bytes[base + k];
        }
    }
    Some(OdrRecord { header, ad })
}

pub fn split_records(byte_len: usize) -> (usize, usize) {
    (byte_len / RECORD_BYTES, byte_len % RECORD_BYTES)
}

pub fn parse_odr(bytes: &[u8]) -> Option<Vec<OdrRecord>> {
    let (n, _) = split_records(bytes.len());
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(record(&bytes[i * RECORD_BYTES..(i + 1) * RECORD_BYTES])?);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_record(record_number: u16) -> Vec<u8> {
        let mut bytes = vec![0u8; RECORD_BYTES];
        bytes[0] = 0xD2;
        bytes[2..4].copy_from_slice(&record_number.to_be_bytes());
        bytes[4..6].copy_from_slice(&1333u16.to_be_bytes());
        bytes[8] = 77;
        bytes[9] = 60;
        bytes[10..12].copy_from_slice(&0xC23Au16.to_be_bytes());
        bytes[12..16].copy_from_slice(&0x01EE6280u32.to_be_bytes());
        bytes[158..160].copy_from_slice(&1250u16.to_be_bytes());
        for i in 0..AD_REPETITIONS {
            let base = HEADER_BYTES + i * AD_GROUP_BYTES;
            bytes[base] = i as u8;
            bytes[base + 1] = (i * 2) as u8;
            bytes[base + 2] = (i * 3) as u8;
            bytes[base + 3] = (i * 4) as u8;
        }
        bytes
    }

    #[test]
    fn header_decodes_big_endian_fields() {
        let bytes = sample_record(1);
        let h = header(&bytes).unwrap();
        assert_eq!(h.record_number, 1);
        assert_eq!(h.record_words, 1333);
        assert_eq!(h.spacecraft, 77);
        assert_eq!(h.spc, 60);
        assert_eq!(h.year, 97);
        assert_eq!(h.doy, 58);
        assert_eq!(h.time_tag_ms, 0x01EE6280);
        assert_eq!(h.sample_rate, 1250);
        assert!(header(&[0u8; 100]).is_none());
    }

    #[test]
    fn record_decodes_interleaved_ad_quads() {
        let bytes = sample_record(1);
        let r = record(&bytes).unwrap();
        assert_eq!(r.header.record_number, 1);
        assert_eq!(r.ad[0], [0, 0, 0, 0]);
        assert_eq!(r.ad[1], [1, 2, 3, 4]);
        assert_eq!(r.ad[2], [2, 4, 6, 8]);
        assert_eq!(r.ad[AD_REPETITIONS - 1], [112, 224, 80, 192]);
        assert!(record(&[0u8; RECORD_BYTES - 1]).is_none());
    }

    #[test]
    fn parse_odr_counts_complete_records() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&sample_record(1));
        bytes.extend_from_slice(&sample_record(2));
        bytes.extend_from_slice(&sample_record(3));
        bytes.extend_from_slice(&[0u8; 880]);
        assert_eq!(split_records(bytes.len()), (3, 880));
        let recs = parse_odr(&bytes).unwrap();
        assert_eq!(recs.len(), 3);
        assert_eq!(recs[0].header.record_number, 1);
        assert_eq!(recs[2].header.record_number, 3);
        assert!(parse_odr(b"X").unwrap().is_empty());
    }

    #[test]
    fn trailing_partial_record_header_stays_readable() {
        let mut bytes = sample_record(13865);
        bytes.truncate(880);
        assert_eq!(split_records(bytes.len()), (0, 880));
        let h = header(&bytes).unwrap();
        assert_eq!(h.record_number, 13865);
        assert_eq!(h.record_words, 1333);
        assert_eq!(h.spacecraft, 77);
        assert_eq!(h.spc, 60);
    }
}
