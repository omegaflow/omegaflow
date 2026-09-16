pub const RECORD_WORDS: usize = 2528;
pub const HEADER_WORDS: usize = 28;
pub const DATA_WORDS: usize = RECORD_WORDS - HEADER_WORDS;
pub const RECORD_BYTES: usize = RECORD_WORDS * 2;
pub const HEADER_BYTES: usize = HEADER_WORDS * 2;
pub const SAMPLE_BYTES: usize = DATA_WORDS * 2;

pub const WORD1_TIME_TAG_VALID: u16 = 0x8000;
pub const WORD1_FIRST_RECORD: u16 = 0x4000;
pub const WORD1_COPY_SOURCE_ERROR: u16 = 0x2000;
pub const WORD1_SAMPLE_COUNT_VALID: u16 = 0x1000;
pub const WORD1_TAPE_NUMBER: u16 = 0x00FF;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OdrTimeTag {
    pub day_of_year: u16,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub microsecond: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OdrHeader {
    pub time_tag_valid: bool,
    pub first_record: bool,
    pub copy_source_error: bool,
    pub sample_count_valid: bool,
    pub tape_number: u8,
    pub record_number: u16,
    pub record_length_words: u16,
    pub spacecraft: u8,
    pub source_station: u8,
    pub dra_tape_number: u16,
    pub time_tag: OdrTimeTag,
    pub reduction_rate_code: u8,
    pub channel_sampling_rate_code: u8,
    pub sample_count: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OdrRecord {
    pub header: OdrHeader,
    pub samples: [u8; SAMPLE_BYTES],
}

pub const PACK_MAGIC: [u8; 4] = *b"VODR";
pub const PACK_ENTRY_BYTES: usize = 96;

#[derive(Clone, Debug, PartialEq)]
pub struct PackedOdr {
    pub name: String,
    pub year: u16,
    pub sha256: [u8; 32],
    pub records: Vec<OdrRecord>,
}

fn be16(bytes: &[u8], i: usize) -> u16 {
    u16::from_be_bytes([bytes[i], bytes[i + 1]])
}

fn be32(bytes: &[u8], i: usize) -> u32 {
    u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]])
}

fn time_tag(bytes: &[u8]) -> OdrTimeTag {
    OdrTimeTag {
        day_of_year: u16::from(bytes[0] >> 4) * 100
            + u16::from(bytes[0] & 0x0F) * 10
            + u16::from(bytes[1] >> 4),
        hour: (bytes[1] & 0x0F) * 10 + (bytes[2] >> 4),
        minute: (bytes[2] & 0x0F) * 10 + (bytes[3] >> 4),
        second: (bytes[3] & 0x0F) * 10 + (bytes[4] >> 4),
        microsecond: u32::from(bytes[4] & 0x0F) << 16
            | u32::from(bytes[5]) << 8
            | u32::from(bytes[6]),
    }
}

pub fn header(bytes: &[u8]) -> Option<OdrHeader> {
    if bytes.len() < HEADER_BYTES {
        return None;
    }
    let word1 = be16(bytes, 0);
    Some(OdrHeader {
        time_tag_valid: word1 & WORD1_TIME_TAG_VALID != 0,
        first_record: word1 & WORD1_FIRST_RECORD != 0,
        copy_source_error: word1 & WORD1_COPY_SOURCE_ERROR != 0,
        sample_count_valid: word1 & WORD1_SAMPLE_COUNT_VALID != 0,
        tape_number: (word1 & WORD1_TAPE_NUMBER) as u8,
        record_number: be16(bytes, 2),
        record_length_words: be16(bytes, 4),
        spacecraft: bytes[6],
        source_station: bytes[7],
        dra_tape_number: be16(bytes, 8),
        time_tag: time_tag(&bytes[10..18]),
        reduction_rate_code: (be16(bytes, 18) & 0x001F) as u8,
        channel_sampling_rate_code: (be16(bytes, 20) & 0x001F) as u8,
        sample_count: be32(bytes, 52),
    })
}

pub fn record(bytes: &[u8]) -> Option<OdrRecord> {
    if bytes.len() < RECORD_BYTES {
        return None;
    }
    let header = header(bytes)?;
    let mut samples = [0u8; SAMPLE_BYTES];
    samples.copy_from_slice(&bytes[HEADER_BYTES..RECORD_BYTES]);
    Some(OdrRecord { header, samples })
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

pub fn pack(raw: &[u8], name: &str, year: u16) -> Vec<u8> {
    let record_count = raw.len() / RECORD_BYTES;
    let data_start = 8 + PACK_ENTRY_BYTES;
    let mut bin = vec![0u8; data_start + raw.len()];
    bin[0..4].copy_from_slice(&PACK_MAGIC);
    bin[4..8].copy_from_slice(&1u32.to_le_bytes());
    let nameb = name.as_bytes();
    let n = nameb.len().min(32);
    bin[8..8 + n].copy_from_slice(&nameb[..n]);
    bin[8 + 32..8 + 64].copy_from_slice(&crate::archivar::sha256::sha256_raw(raw));
    bin[8 + 64..8 + 68].copy_from_slice(&(record_count as u32).to_le_bytes());
    bin[8 + 68..8 + 70].copy_from_slice(&year.to_le_bytes());
    bin[8 + 72..8 + 80].copy_from_slice(&(data_start as u64).to_le_bytes());
    bin[8 + 80..8 + 88].copy_from_slice(&(raw.len() as u64).to_le_bytes());
    bin[data_start..].copy_from_slice(raw);
    bin
}

pub fn parse_packed(bytes: &[u8]) -> Option<PackedOdr> {
    if bytes.len() < 8 + PACK_ENTRY_BYTES || bytes[0..4] != PACK_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if count == 0 || bytes.len() < 8 + count * PACK_ENTRY_BYTES {
        return None;
    }
    let mut records = Vec::new();
    let mut name = String::new();
    let mut year = 0u16;
    let mut sha256 = [0u8; 32];
    for i in 0..count {
        let entry = &bytes[8 + i * PACK_ENTRY_BYTES..8 + (i + 1) * PACK_ENTRY_BYTES];
        let name_end = entry[0..32]
            .iter()
            .position(|b| *b == 0)
            .unwrap_or(32);
        name = String::from_utf8(entry[0..name_end].to_vec()).ok()?;
        sha256.copy_from_slice(&entry[32..64]);
        let record_count = u32::from_le_bytes(entry[64..68].try_into().ok()?) as usize;
        year = u16::from_le_bytes(entry[68..70].try_into().ok()?);
        let data_offset = u64::from_le_bytes(entry[72..80].try_into().ok()?) as usize;
        let data_length = u64::from_le_bytes(entry[80..88].try_into().ok()?) as usize;
        if data_offset + data_length > bytes.len() {
            return None;
        }
        let raw = &bytes[data_offset..data_offset + data_length];
        if crate::archivar::sha256::sha256_raw(raw) != sha256 {
            return None;
        }
        if raw.len() != record_count * RECORD_BYTES {
            return None;
        }
        let recs = parse_odr(raw)?;
        if recs.len() != record_count {
            return None;
        }
        records.extend(recs);
    }
    Some(PackedOdr {
        name,
        year,
        sha256,
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measured_header() -> [u8; HEADER_BYTES] {
        [
            0x90, 0x06, 0x00, 0x01, 0x09, 0xE0, 0x20, 0x2B, 0x00, 0x1D, 0x23, 0x72, 0x03, 0x55,
            0x9F, 0x41, 0x5E, 0x25, 0x00, 0x60, 0x00, 0xA2, 0x50, 0xFE, 0xDB, 0x08, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x8B, 0x81, 0x4C, 0x59, 0x00, 0xA2, 0x50, 0x1D, 0xFF, 0xFB, 0x6C, 0x4E,
        ]
    }

    fn sample_record(record_number: u16, word1: u16) -> Vec<u8> {
        let mut bytes = vec![0u8; RECORD_BYTES];
        bytes[..HEADER_BYTES].copy_from_slice(&measured_header());
        bytes[0..2].copy_from_slice(&word1.to_be_bytes());
        bytes[2..4].copy_from_slice(&record_number.to_be_bytes());
        for i in 0..SAMPLE_BYTES {
            bytes[HEADER_BYTES + i] = i as u8;
        }
        bytes
    }

    #[test]
    fn header_decodes_measured_record_one() {
        let bytes = sample_record(1, 0x9006);
        let h = header(&bytes).unwrap();
        assert!(h.time_tag_valid);
        assert!(!h.first_record);
        assert!(!h.copy_source_error);
        assert!(h.sample_count_valid);
        assert_eq!(h.tape_number, 6);
        assert_eq!(h.record_number, 1);
        assert_eq!(h.record_length_words, 2528);
        assert_eq!(h.spacecraft, 32);
        assert_eq!(h.source_station, 43);
        assert_eq!(h.dra_tape_number, 29);
        assert_eq!(
            h.time_tag,
            OdrTimeTag {
                day_of_year: 237,
                hour: 20,
                minute: 35,
                second: 59,
                microsecond: 999_774,
            }
        );
        assert_eq!(h.reduction_rate_code, 0);
        assert_eq!(h.channel_sampling_rate_code, 2);
        assert_eq!(h.sample_count, 0xFFFB_6C4E);
        assert!(header(&[0u8; HEADER_BYTES - 1]).is_none());
    }

    #[test]
    fn record_extracts_flat_samples() {
        let bytes = sample_record(1, 0x9006);
        let r = record(&bytes).unwrap();
        assert_eq!(r.header.record_number, 1);
        assert_eq!(r.samples[0], 0);
        assert_eq!(r.samples[1], 1);
        assert_eq!(r.samples[SAMPLE_BYTES - 1], (SAMPLE_BYTES - 1) as u8);
        assert!(record(&[0u8; RECORD_BYTES - 1]).is_none());
    }

    #[test]
    fn parse_odr_counts_complete_records() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&sample_record(1, 0x9006));
        bytes.extend_from_slice(&sample_record(2, 0x0006));
        bytes.extend_from_slice(&sample_record(3, 0x0006));
        bytes.extend_from_slice(&[0u8; 512]);
        assert_eq!(split_records(bytes.len()), (3, 512));
        let recs = parse_odr(&bytes).unwrap();
        assert_eq!(recs.len(), 3);
        assert_eq!(recs[0].header.record_number, 1);
        assert!(recs[0].header.time_tag_valid);
        assert_eq!(recs[1].header.record_number, 2);
        assert!(!recs[1].header.time_tag_valid);
        assert_eq!(recs[2].header.record_number, 3);
        assert!(parse_odr(b"X").unwrap().is_empty());
    }

    #[test]
    fn trailing_partial_record_header_stays_readable() {
        let mut bytes = sample_record(13865, 0x9006);
        bytes.truncate(512);
        assert_eq!(split_records(bytes.len()), (0, 512));
        let h = header(&bytes).unwrap();
        assert_eq!(h.record_number, 13865);
        assert_eq!(h.record_length_words, 2528);
        assert_eq!(h.spacecraft, 32);
        assert_eq!(h.source_station, 43);
    }

    #[test]
    fn pack_roundtrips_records_and_year() {
        let mut raw = Vec::new();
        raw.extend_from_slice(&sample_record(1, 0x9006));
        raw.extend_from_slice(&sample_record(2, 0x0006));
        let records = parse_odr(&raw).unwrap();
        let bin = pack(&raw, "C0XR13AA.ODR", 1981);
        let parsed = parse_packed(&bin).unwrap();
        assert_eq!(parsed.name, "C0XR13AA.ODR");
        assert_eq!(parsed.year, 1981);
        assert_eq!(parsed.records, records);
        assert_eq!(parsed.sha256, crate::archivar::sha256::sha256_raw(&raw));
        assert!(parse_packed(b"X").is_none());
    }

    #[test]
    fn parse_packed_voids_on_wrong_magic_and_corruption() {
        let raw = sample_record(1, 0x9006);
        let bin = pack(&raw, "C0XR13AA.ODR", 1981);
        let mut wrong_magic = bin.clone();
        wrong_magic[0] = b'X';
        assert!(parse_packed(&wrong_magic).is_none());
        let mut corrupted = bin;
        let last = corrupted.len() - 1;
        corrupted[last] ^= 0xff;
        assert!(parse_packed(&corrupted).is_none());
    }
}
