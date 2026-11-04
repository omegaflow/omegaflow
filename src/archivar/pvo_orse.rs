pub const WORD_BYTES: usize = 2;
pub const TAPE_INIT_WORDS: usize = 16;
pub const HEADER_WORDS_ORIGINAL: usize = 28;
pub const TRAILER_WORDS_ORIGINAL: usize = 17;
pub const HEADER_WORDS_DSP: usize = 40;
pub const TRAILER_WORDS_PPM: usize = 150;
pub const TRAILER_WORDS_PREDICT: usize = 5;
pub const MIN_RECORD_WORDS: usize = HEADER_WORDS_ORIGINAL;
pub const MAX_RECORD_WORDS: usize = 8192;

const NO_COMPLETE_BYTES: usize = 0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OdrHeader {
    pub word1: u16,
    pub record_number: u16,
    pub record_length_words: u16,
    pub spacecraft: u8,
    pub source: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OdrRecord {
    pub len_words: usize,
    pub header: OdrHeader,
    pub bytes: Vec<u8>,
}

pub const PACK_MAGIC: [u8; 4] = *b"PORO";
pub const PACK_ENTRY_BYTES: usize = 96;

#[derive(Clone, Debug, PartialEq)]
pub struct OdrFile {
    pub name: String,
    pub year: u16,
    pub sha256: [u8; 32],
    pub record_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackedOdr {
    pub files: Vec<OdrFile>,
}

fn be16(bytes: &[u8], i: usize) -> u16 {
    u16::from_be_bytes([bytes[i], bytes[i + 1]])
}

pub fn tape_init_offset(bytes: &[u8]) -> usize {
    if bytes.len() >= TAPE_INIT_WORDS * WORD_BYTES
        && bytes[..4].iter().all(|b| (0x20..=0x7e).contains(b))
    {
        TAPE_INIT_WORDS * WORD_BYTES
    } else {
        0
    }
}

pub fn header(bytes: &[u8], offset: usize) -> Option<OdrHeader> {
    if offset + 4 * WORD_BYTES > bytes.len() {
        return None;
    }
    Some(OdrHeader {
        word1: be16(bytes, offset),
        record_number: be16(bytes, offset + 2),
        record_length_words: be16(bytes, offset + 4),
        spacecraft: bytes[offset + 6],
        source: bytes[offset + 7],
    })
}

pub fn record(bytes: &[u8], offset: usize) -> Option<OdrRecord> {
    let header = header(bytes, offset)?;
    let len_words = header.record_length_words as usize;
    if !(MIN_RECORD_WORDS..=MAX_RECORD_WORDS).contains(&len_words) {
        return None;
    }
    let len_bytes = len_words * WORD_BYTES;
    if offset + len_bytes > bytes.len() {
        return None;
    }
    Some(OdrRecord {
        len_words,
        header,
        bytes: bytes[offset..offset + len_bytes].to_vec(),
    })
}

pub fn record_spans(bytes: &[u8]) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut offset = tape_init_offset(bytes);
    while offset + 3 * WORD_BYTES <= bytes.len() {
        let len_words = be16(bytes, offset + 4) as usize;
        if !(MIN_RECORD_WORDS..=MAX_RECORD_WORDS).contains(&len_words) {
            break;
        }
        let len_bytes = len_words * WORD_BYTES;
        if offset + len_bytes > bytes.len() {
            break;
        }
        spans.push((offset, len_bytes));
        offset += len_bytes;
    }
    spans
}

pub fn record_count(bytes: &[u8]) -> usize {
    record_spans(bytes).len()
}

pub fn complete_bytes(bytes: &[u8]) -> usize {
    match record_spans(bytes).last() {
        Some((offset, len)) => offset + len,
        None => NO_COMPLETE_BYTES,
    }
}

pub fn parse_odr(bytes: &[u8]) -> Option<Vec<OdrRecord>> {
    let spans = record_spans(bytes);
    let mut out = Vec::with_capacity(spans.len());
    for (offset, _) in spans {
        out.push(record(bytes, offset)?);
    }
    Some(out)
}

pub fn pack(raw: &[u8], name: &str, year: u16) -> Vec<u8> {
    pack_many(&[(raw, name, year)])
}

pub fn pack_many(files: &[(&[u8], &str, u16)]) -> Vec<u8> {
    let data_start = 8 + files.len() * PACK_ENTRY_BYTES;
    let data_bytes: usize = files.iter().map(|(raw, _, _)| raw.len()).sum();
    let mut bin = vec![0u8; data_start + data_bytes];
    bin[0..4].copy_from_slice(&PACK_MAGIC);
    bin[4..8].copy_from_slice(&(files.len() as u32).to_le_bytes());
    let mut offset = data_start;
    for (i, (raw, name, year)) in files.iter().enumerate() {
        let base = 8 + i * PACK_ENTRY_BYTES;
        let nameb = name.as_bytes();
        let n = nameb.len().min(32);
        bin[base..base + n].copy_from_slice(&nameb[..n]);
        bin[base + 32..base + 64].copy_from_slice(&crate::archivar::sha256::sha256_raw(raw));
        bin[base + 64..base + 68].copy_from_slice(&(record_count(raw) as u32).to_le_bytes());
        bin[base + 68..base + 70].copy_from_slice(&year.to_le_bytes());
        bin[base + 72..base + 80].copy_from_slice(&(offset as u64).to_le_bytes());
        bin[base + 80..base + 88].copy_from_slice(&(raw.len() as u64).to_le_bytes());
        bin[offset..offset + raw.len()].copy_from_slice(raw);
        offset += raw.len();
    }
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
    let mut files = Vec::with_capacity(count);
    for i in 0..count {
        let entry = &bytes[8 + i * PACK_ENTRY_BYTES..8 + (i + 1) * PACK_ENTRY_BYTES];
        let name_end = entry[0..32].iter().position(|b| *b == 0).unwrap_or(32);
        let name = String::from_utf8(entry[0..name_end].to_vec()).ok()?;
        let mut sha256 = [0u8; 32];
        sha256.copy_from_slice(&entry[32..64]);
        let record_count_stored = u32::from_le_bytes(entry[64..68].try_into().ok()?) as usize;
        let year = u16::from_le_bytes(entry[68..70].try_into().ok()?);
        let data_offset = u64::from_le_bytes(entry[72..80].try_into().ok()?) as usize;
        let data_length = u64::from_le_bytes(entry[80..88].try_into().ok()?) as usize;
        if data_offset + data_length > bytes.len() {
            return None;
        }
        let raw = &bytes[data_offset..data_offset + data_length];
        if crate::archivar::sha256::sha256_raw(raw) != sha256 {
            return None;
        }
        if complete_bytes(raw) != raw.len() {
            return None;
        }
        if record_count(raw) != record_count_stored {
            return None;
        }
        files.push(OdrFile {
            name,
            year,
            sha256,
            record_count: record_count_stored,
        });
    }
    Some(PackedOdr { files })
}

pub const SHARD_BUDGET: usize = 1 << 30;
pub const SHARD_LIMIT: usize = 1 << 31;

pub fn packed_size(file_count: usize, data_bytes: usize) -> usize {
    8 + file_count * PACK_ENTRY_BYTES + data_bytes
}

pub fn shard_ranges(file_bytes: &[usize], budget: usize) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut lo = 0usize;
    while lo < file_bytes.len() {
        let mut hi = lo + 1;
        let mut data = file_bytes[lo];
        while hi < file_bytes.len() && packed_size(hi + 1 - lo, data + file_bytes[hi]) <= budget {
            data += file_bytes[hi];
            hi += 1;
        }
        ranges.push((lo, hi));
        lo = hi;
    }
    ranges
}

pub fn shard_name(prefix: &str, ord: usize) -> String {
    format!("{prefix}_s{ord}.bin")
}

#[cfg(test)]
mod tests {
    use super::*;

    const ODR8231_RECORD_ONE: [u8; 8] = [0xC1, 0x01, 0x00, 0x01, 0x07, 0xFD, 0x0C, 0x2A];
    const ODR030_RECORD_ONE: [u8; 8] = [0x91, 0x04, 0x00, 0x01, 0x05, 0xF8, 0x0C, 0x0E];

    fn sample_record(len_words: usize, record_number: u16, word1: u16) -> Vec<u8> {
        let mut bytes = vec![0u8; len_words * WORD_BYTES];
        bytes[0..2].copy_from_slice(&word1.to_be_bytes());
        bytes[2..4].copy_from_slice(&record_number.to_be_bytes());
        bytes[4..6].copy_from_slice(&(len_words as u16).to_be_bytes());
        bytes[6] = 12;
        bytes[7] = 42;
        bytes
    }

    #[test]
    fn header_decodes_measured_odr8231_record_one() {
        let h = header(&ODR8231_RECORD_ONE, 0).unwrap();
        assert_eq!(h.word1, 0xC101);
        assert_eq!(h.record_number, 1);
        assert_eq!(h.record_length_words, 2045);
        assert_eq!(h.spacecraft, 12);
        assert_eq!(h.source, 42);
        assert!(header(&ODR8231_RECORD_ONE[..7], 0).is_none());
    }

    #[test]
    fn header_decodes_measured_odr030_record_one() {
        let h = header(&ODR030_RECORD_ONE, 0).unwrap();
        assert_eq!(h.record_number, 1);
        assert_eq!(h.record_length_words, 1528);
        assert_eq!(h.spacecraft, 12);
        assert_eq!(h.source, 14);
    }

    #[test]
    fn record_extracts_the_measured_length() {
        let bytes = sample_record(2045, 1, 0xC101);
        let r = record(&bytes, 0).unwrap();
        assert_eq!(r.len_words, 2045);
        assert_eq!(r.bytes.len(), 2045 * WORD_BYTES);
        assert_eq!(r.header.record_length_words, 2045);
        assert!(record(&bytes[..bytes.len() - 1], 0).is_none());
    }

    #[test]
    fn parse_odr_walks_variable_records() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&sample_record(2045, 1, 0xC101));
        bytes.extend_from_slice(&sample_record(2045, 2, 0x0101));
        bytes.extend_from_slice(&sample_record(1528, 3, 0x8101));
        let records = parse_odr(&bytes).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].len_words, 2045);
        assert_eq!(records[1].len_words, 2045);
        assert_eq!(records[2].len_words, 1528);
        assert_eq!(records[2].header.record_number, 3);
        assert_eq!(complete_bytes(&bytes), bytes.len());
        assert_eq!(record_count(&bytes), 3);
        assert!(parse_odr(b"X").unwrap().is_empty());
    }

    #[test]
    fn parse_odr_skips_leading_tape_init_record() {
        let mut bytes = b"DSPR-5205-OP-D-V7.13".to_vec();
        bytes.resize(TAPE_INIT_WORDS * WORD_BYTES, b' ');
        assert_eq!(bytes.len(), TAPE_INIT_WORDS * WORD_BYTES);
        bytes.extend_from_slice(&sample_record(2195, 1, 0xC101));
        bytes.extend_from_slice(&sample_record(2195, 2, 0x0101));
        assert_eq!(tape_init_offset(&bytes), TAPE_INIT_WORDS * WORD_BYTES);
        let records = parse_odr(&bytes).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].len_words, 2195);
        assert_eq!(records[0].header.record_number, 1);
        assert_eq!(complete_bytes(&bytes), bytes.len());
    }

    #[test]
    fn binary_header_is_not_mistaken_for_tape_init() {
        let bytes = sample_record(2045, 1, 0xC101);
        assert_eq!(tape_init_offset(&bytes), 0);
    }

    #[test]
    fn trailing_fragment_is_dropped_not_fabricated() {
        let mut bytes = sample_record(2045, 1, 0xC101);
        bytes.extend_from_slice(&[0u8; 512]);
        let records = parse_odr(&bytes).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(complete_bytes(&bytes), 2045 * WORD_BYTES);
        assert_eq!(record_count(&bytes), 1);
    }

    #[test]
    fn invalid_length_word_ends_the_walk() {
        let mut bytes = sample_record(2045, 1, 0xC101);
        bytes[4..6].copy_from_slice(&1u16.to_be_bytes());
        assert!(record_spans(&bytes).is_empty());
        assert_eq!(record_count(&bytes), 0);
        assert_eq!(complete_bytes(&bytes), 0);
        assert!(parse_odr(&bytes).unwrap().is_empty());
    }

    #[test]
    fn pack_roundtrips_records_and_year() {
        let mut raw = sample_record(2045, 1, 0xC101);
        raw.extend_from_slice(&sample_record(2195, 2, 0x0101));
        let bin = pack(&raw, "ODR8231.ODR", 1984);
        let parsed = parse_packed(&bin).unwrap();
        assert_eq!(parsed.files.len(), 1);
        assert_eq!(parsed.files[0].name, "ODR8231.ODR");
        assert_eq!(parsed.files[0].year, 1984);
        assert_eq!(parsed.files[0].record_count, 2);
        assert_eq!(
            parsed.files[0].sha256,
            crate::archivar::sha256::sha256_raw(&raw)
        );
        assert!(parse_packed(b"X").is_none());
    }

    #[test]
    fn parse_packed_voids_on_wrong_magic_and_corruption() {
        let raw = sample_record(2045, 1, 0xC101);
        let bin = pack(&raw, "ODR8231.ODR", 1984);
        let mut wrong_magic = bin.clone();
        wrong_magic[0] = b'X';
        assert!(parse_packed(&wrong_magic).is_none());
        let mut corrupted = bin;
        let last = corrupted.len() - 1;
        corrupted[last] ^= 0xff;
        assert!(parse_packed(&corrupted).is_none());
    }

    #[test]
    fn pack_many_roundtrips_per_file_entries() {
        let raw = sample_record(2045, 1, 0xC101);
        let other = sample_record(2195, 2, 0x0101);
        let bin = pack_many(&[
            (&raw[..], "ODR8231.ODR", 1984),
            (&other[..], "PXC0001.ODR", 1986),
        ]);
        let parsed = parse_packed(&bin).unwrap();
        assert_eq!(parsed.files.len(), 2);
        assert_eq!(parsed.files[0].name, "ODR8231.ODR");
        assert_eq!(parsed.files[0].year, 1984);
        assert_eq!(parsed.files[1].name, "PXC0001.ODR");
        assert_eq!(parsed.files[1].year, 1986);
        assert_eq!(parsed.files[1].record_count, 1);
    }

    #[test]
    fn shard_ranges_group_files_by_packed_budget() {
        assert!(shard_ranges(&[], SHARD_BUDGET).is_empty());

        let single = vec![1000usize];
        assert_eq!(shard_ranges(&single, SHARD_BUDGET), vec![(0, 1)]);

        let many = vec![1000usize; 10];
        let budget = packed_size(3, 3000);
        assert_eq!(
            shard_ranges(&many, budget),
            vec![(0, 3), (3, 6), (6, 9), (9, 10)]
        );

        for &(lo, hi) in &shard_ranges(&many, budget) {
            let data: usize = many[lo..hi].iter().sum();
            assert!(packed_size(hi - lo, data) <= budget);
        }
    }

    #[test]
    fn shard_name_is_deterministic_and_prefixed() {
        assert_eq!(shard_name("pvo_orse_odr", 0), "pvo_orse_odr_s0.bin");
        assert_eq!(shard_name("pvo_orse_odr", 13), "pvo_orse_odr_s13.bin");
    }
}
