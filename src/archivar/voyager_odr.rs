pub const RECORD_WORDS: usize = 2528;
pub const HEADER_WORDS: usize = 28;
pub const DATA_WORDS: usize = RECORD_WORDS - HEADER_WORDS;
pub const RECORD_BYTES: usize = RECORD_WORDS * 2;
pub const HEADER_BYTES: usize = HEADER_WORDS * 2;
pub const SAMPLE_BYTES: usize = DATA_WORDS * 2;
pub const DATA_SAMPLES: usize = SAMPLE_BYTES;
pub const COMP_SAMPLE: u32 = 1;

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
    pub decimation_code: u8,
    pub reduction_channel: u8,
    pub sample_count: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OdrRecord {
    pub header: OdrHeader,
    pub samples: [u8; SAMPLE_BYTES],
}

pub const PACK_MAGIC: [u8; 4] = *b"VODR";
pub const PACK_ENTRY_BYTES: usize = 96;

#[derive(Clone, Debug, PartialEq)]
pub struct OdrFile {
    pub name: String,
    pub year: u16,
    pub sha256: [u8; 32],
    pub records: Vec<OdrRecord>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackedOdr {
    pub files: Vec<OdrFile>,
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
        decimation_code: ((be16(bytes, 22) >> 12) & 0x7) as u8,
        reduction_channel: ((be16(bytes, 22) >> 8) & 0x3) as u8,
        sample_count: be32(bytes, 52) as i32,
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
        bin[base + 64..base + 68]
            .copy_from_slice(&((raw.len() / RECORD_BYTES) as u32).to_le_bytes());
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
        let record_count = u32::from_le_bytes(entry[64..68].try_into().ok()?) as usize;
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
        if raw.len() != record_count * RECORD_BYTES {
            return None;
        }
        let recs = parse_odr(raw)?;
        if recs.len() != record_count {
            return None;
        }
        files.push(OdrFile {
            name,
            year,
            sha256,
            records: recs,
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

pub fn channel_sampling_rate_hz(code: u8) -> Option<f64> {
    match code {
        16 => Some(50_000.0),    // 10000
        8 => Some(62_500.0),     // 01000
        0 => Some(75_000.0),     // 00000
        17 => Some(100_000.0),   // 10001
        9 => Some(125_000.0),    // 01001
        1 => Some(150_000.0),    // 00001
        18 => Some(200_000.0),   // 10010
        10 => Some(250_000.0),   // 01010
        2 => Some(300_000.0),    // 00010
        19 => Some(400_000.0),   // 10011
        11 => Some(500_000.0),   // 01011
        3 => Some(600_000.0),    // 00011
        20 => Some(800_000.0),   // 10100
        12 => Some(1_000_000.0), // 01100
        4 => Some(1_200_000.0),  // 00100
        _ => None,
    }
}

pub fn decimation_ratio(code: u8) -> Option<u32> {
    if code <= 7 {
        Some(8 - u32::from(code))
    } else {
        None
    }
}

pub fn time_tag_unix(year: u16, tag: &OdrTimeTag) -> Option<f64> {
    if !(1..=366).contains(&tag.day_of_year)
        || tag.hour > 23
        || tag.minute > 59
        || tag.second > 59
        || tag.microsecond > 999_999
    {
        return None;
    }
    let days = crate::lsk::days_from_civil(year as i64, 1, 1)?;
    let base = (days + tag.day_of_year as i64 - 1) as f64 * 86400.0
        + tag.hour as f64 * 3600.0
        + tag.minute as f64 * 60.0
        + tag.second as f64
        + tag.microsecond as f64 / 1_000_000.0;
    Some(base.round())
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let packed = parse_packed(bytes)?;
    let lsk = crate::archivar::membrane::embedded_lsk()?;
    let total_records: usize = packed.files.iter().map(|f| f.records.len()).sum();
    let mut out = Vec::with_capacity(total_records * DATA_SAMPLES);
    let mut anchor: Option<(f64, f64, u16)> = None;
    for file in &packed.files {
        for rec in &file.records {
            let h = &rec.header;
            let (t0, dt) = if h.time_tag_valid {
                let Some(rate) = channel_sampling_rate_hz(h.channel_sampling_rate_code) else {
                    anchor = None;
                    continue;
                };
                let Some(dec) = decimation_ratio(h.decimation_code) else {
                    anchor = None;
                    continue;
                };
                let dt = dec as f64 / rate;
                let Some(t) =
                    time_tag_unix(file.year, &h.time_tag).and_then(|u| lsk.unix_to_tdb(u))
                else {
                    anchor = None;
                    continue;
                };
                anchor = Some((
                    t + dt * DATA_SAMPLES as f64,
                    dt,
                    h.record_number.wrapping_add(1),
                ));
                (t, dt)
            } else {
                let Some((t_next, dt, next_rec)) = anchor else {
                    continue;
                };
                if h.record_number != next_rec {
                    anchor = None;
                    continue;
                }
                anchor = Some((
                    t_next + dt * DATA_SAMPLES as f64,
                    dt,
                    next_rec.wrapping_add(1),
                ));
                (t_next, dt)
            };
            for (i, s) in rec.samples.iter().enumerate() {
                out.push((t0 + i as f64 * dt, f64::from(*s), COMP_SAMPLE));
            }
        }
    }
    Some(out)
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
        assert_eq!(h.decimation_code, 5);
        assert_eq!(h.reduction_channel, 0);
        assert_eq!(h.sample_count, -299954);
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
        assert_eq!(parsed.files.len(), 1);
        assert_eq!(parsed.files[0].name, "C0XR13AA.ODR");
        assert_eq!(parsed.files[0].year, 1981);
        assert_eq!(parsed.files[0].records, records);
        assert_eq!(
            parsed.files[0].sha256,
            crate::archivar::sha256::sha256_raw(&raw)
        );
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

    fn measured_c0xr13aa_record_one() -> [u8; HEADER_BYTES] {
        [
            0x90, 0x0D, 0x00, 0x01, 0x09, 0xE0, 0x20, 0x2B, 0x00, 0x1E, 0x23, 0x80, 0x40, 0x45,
            0x9B, 0x71, 0x54, 0x25, 0x00, 0x60, 0x00, 0xA2, 0x72, 0xFE, 0xDB, 0x08, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x8A, 0x00, 0xD0, 0x05, 0x00, 0xA2, 0x72, 0x1F, 0xFF, 0xFB, 0x6C, 0x4C,
        ]
    }

    #[test]
    fn header_decodes_measured_c0xr13aa_record_one() {
        let h = header(&measured_c0xr13aa_record_one()).unwrap();
        assert!(h.time_tag_valid);
        assert!(!h.first_record);
        assert!(h.sample_count_valid);
        assert_eq!(h.tape_number, 13);
        assert_eq!(h.record_number, 1);
        assert_eq!(h.record_length_words, 2528);
        assert_eq!(h.spacecraft, 32);
        assert_eq!(h.source_station, 43);
        assert_eq!(h.dra_tape_number, 30);
        assert_eq!(
            h.time_tag,
            OdrTimeTag {
                day_of_year: 238,
                hour: 4,
                minute: 4,
                second: 59,
                microsecond: 749_908,
            }
        );
        assert_eq!(h.reduction_rate_code, 0);
        assert_eq!(h.channel_sampling_rate_code, 2);
        assert_eq!(h.decimation_code, 7);
        assert_eq!(h.reduction_channel, 2);
        assert_eq!(h.sample_count, -299956);
    }

    #[test]
    fn channel_sampling_rate_codes_map_to_hz() {
        assert_eq!(channel_sampling_rate_hz(16), Some(50_000.0));
        assert_eq!(channel_sampling_rate_hz(8), Some(62_500.0));
        assert_eq!(channel_sampling_rate_hz(0), Some(75_000.0));
        assert_eq!(channel_sampling_rate_hz(1), Some(150_000.0));
        assert_eq!(channel_sampling_rate_hz(2), Some(300_000.0));
        assert_eq!(channel_sampling_rate_hz(12), Some(1_000_000.0));
        assert_eq!(channel_sampling_rate_hz(4), Some(1_200_000.0));
        assert_eq!(channel_sampling_rate_hz(5), None);
        assert_eq!(channel_sampling_rate_hz(31), None);
    }

    #[test]
    fn decimation_ratio_maps_code_to_factor() {
        assert_eq!(decimation_ratio(7), Some(1));
        assert_eq!(decimation_ratio(6), Some(2));
        assert_eq!(decimation_ratio(0), Some(8));
        assert_eq!(decimation_ratio(8), None);
    }

    #[test]
    fn time_tag_unix_rounds_to_closest_integral_second() {
        let tag = OdrTimeTag {
            day_of_year: 238,
            hour: 4,
            minute: 4,
            second: 59,
            microsecond: 749_908,
        };
        let days = crate::lsk::days_from_civil(1981, 1, 1).unwrap();
        let expected = (days + 237) as f64 * 86400.0 + 4.0 * 3600.0 + 5.0 * 60.0;
        assert_eq!(time_tag_unix(1981, &tag), Some(expected));

        let early = OdrTimeTag {
            microsecond: 400_000,
            ..tag
        };
        let expected_early = expected - 1.0;
        assert_eq!(time_tag_unix(1981, &early), Some(expected_early));

        let invalid = OdrTimeTag {
            day_of_year: 0,
            ..tag
        };
        assert_eq!(time_tag_unix(1981, &invalid), None);
        let bad_minute = OdrTimeTag { minute: 60, ..tag };
        assert_eq!(time_tag_unix(1981, &bad_minute), None);
    }

    #[test]
    fn parse_series_emits_300khz_counts() {
        let mut raw = Vec::new();
        raw.extend_from_slice(&measured_c0xr13aa_record_one());
        for i in 0..DATA_SAMPLES {
            raw.push(i as u8);
        }
        let bin = pack(&raw, "C0XR13AA.ODR", 1981);
        let series = parse_series(&bin).unwrap();
        assert_eq!(series.len(), DATA_SAMPLES);
        let lsk = crate::archivar::membrane::embedded_lsk().unwrap();
        let t0 = lsk
            .unix_to_tdb(
                time_tag_unix(
                    1981,
                    &header(&measured_c0xr13aa_record_one()).unwrap().time_tag,
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(series[0].0, t0);
        assert_eq!(series[0].1, 0.0);
        assert_eq!(series[0].2, COMP_SAMPLE);
        assert_eq!(series[1].1, 1.0);
        assert!((series[1].0 - series[0].0 - 1.0 / 300_000.0).abs() < 1e-9);
        assert_eq!(series[DATA_SAMPLES - 1].1, (DATA_SAMPLES - 1) as f64);
    }

    #[test]
    fn parse_series_applies_decimation_gt_one_to_dt() {
        let raw = sample_record(1, 0x9006);
        let bin = pack(&raw, "C0XR13AA.ODR", 1981);
        let series = parse_series(&bin).unwrap();
        assert_eq!(series.len(), DATA_SAMPLES);
        let dt = 3.0 / 300_000.0;
        assert!((series[1].0 - series[0].0 - dt).abs() < 1e-9);
    }

    #[test]
    fn parse_series_skips_unanchored_records() {
        let raw = sample_record(1, 0x0006);
        let bin = pack(&raw, "C0XR13AA.ODR", 1981);
        let series = parse_series(&bin).unwrap();
        assert!(series.is_empty());
        assert!(parse_series(b"X").is_none());
    }

    #[test]
    fn parse_series_continues_from_last_valid_anchor() {
        let mut raw = Vec::new();
        raw.extend_from_slice(&measured_c0xr13aa_record_one());
        for i in 0..DATA_SAMPLES {
            raw.push(i as u8);
        }
        let mut second = measured_c0xr13aa_record_one();
        second[0..2].copy_from_slice(&0x0006u16.to_be_bytes());
        second[2..4].copy_from_slice(&2u16.to_be_bytes());
        raw.extend_from_slice(&second);
        for i in 0..DATA_SAMPLES {
            raw.push((i * 2) as u8);
        }
        let bin = pack(&raw, "C0XR13AA.ODR", 1981);
        let series = parse_series(&bin).unwrap();
        assert_eq!(series.len(), 2 * DATA_SAMPLES);
        let dt = 1.0 / 300_000.0;
        assert!((series[DATA_SAMPLES].0 - series[DATA_SAMPLES - 1].0 - dt).abs() < 1e-9);
        assert_eq!(series[DATA_SAMPLES].1, 0.0);
        assert_eq!(series[DATA_SAMPLES].2, COMP_SAMPLE);
    }

    #[test]
    fn parse_series_inherits_config_when_invalid_tag_words_zeroed() {
        let mut raw = Vec::new();
        raw.extend_from_slice(&measured_c0xr13aa_record_one());
        for i in 0..DATA_SAMPLES {
            raw.push(i as u8);
        }
        let mut second = measured_c0xr13aa_record_one();
        second[0..2].copy_from_slice(&0x0006u16.to_be_bytes());
        second[2..4].copy_from_slice(&2u16.to_be_bytes());
        second[18..24].copy_from_slice(&[0u8; 6]);
        raw.extend_from_slice(&second);
        for i in 0..DATA_SAMPLES {
            raw.push((i * 2) as u8);
        }
        let bin = pack(&raw, "C0XR13AA.ODR", 1981);
        let series = parse_series(&bin).unwrap();
        assert_eq!(series.len(), 2 * DATA_SAMPLES);
        let dt = 1.0 / 300_000.0;
        assert!((series[DATA_SAMPLES].0 - series[DATA_SAMPLES - 1].0 - dt).abs() < 1e-9);
        assert_eq!(series[DATA_SAMPLES].1, 0.0);
    }

    #[test]
    fn parse_series_skips_record_number_gaps() {
        let mut raw = Vec::new();
        raw.extend_from_slice(&measured_c0xr13aa_record_one());
        for i in 0..DATA_SAMPLES {
            raw.push(i as u8);
        }
        let mut gap = measured_c0xr13aa_record_one();
        gap[0..2].copy_from_slice(&0x0006u16.to_be_bytes());
        gap[2..4].copy_from_slice(&3u16.to_be_bytes());
        raw.extend_from_slice(&gap);
        for i in 0..DATA_SAMPLES {
            raw.push((i * 3) as u8);
        }
        let bin = pack(&raw, "C0XR13AA.ODR", 1981);
        let series = parse_series(&bin).unwrap();
        assert_eq!(series.len(), DATA_SAMPLES);
    }

    #[test]
    fn parse_series_carries_per_file_year() {
        let mut raw_a = Vec::new();
        raw_a.extend_from_slice(&measured_c0xr13aa_record_one());
        for i in 0..DATA_SAMPLES {
            raw_a.push(i as u8);
        }
        let mut raw_b = raw_a.clone();
        raw_b[2..4].copy_from_slice(&1u16.to_be_bytes());
        let bin = pack_many(&[
            (&raw_a[..], "C0XR13AA.ODR", 1981),
            (&raw_b[..], "C0XR14AA.ODR", 1982),
        ]);
        let series = parse_series(&bin).unwrap();
        assert_eq!(series.len(), 2 * DATA_SAMPLES);
        let lsk = crate::archivar::membrane::embedded_lsk().unwrap();
        let t81 = lsk
            .unix_to_tdb(
                time_tag_unix(
                    1981,
                    &header(&measured_c0xr13aa_record_one()).unwrap().time_tag,
                )
                .unwrap(),
            )
            .unwrap();
        let t82 = lsk
            .unix_to_tdb(
                time_tag_unix(
                    1982,
                    &header(&measured_c0xr13aa_record_one()).unwrap().time_tag,
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(series[0].0, t81);
        assert_eq!(series[DATA_SAMPLES].0, t82);
    }

    #[test]
    fn pack_many_roundtrips_per_file_entries() {
        let mut raw = Vec::new();
        raw.extend_from_slice(&sample_record(1, 0x9006));
        raw.extend_from_slice(&sample_record(2, 0x0006));
        let mut other = raw.clone();
        other[0..2].copy_from_slice(&0x0006u16.to_be_bytes());
        let bin = pack_many(&[(&raw[..], "a.ODR", 1981), (&other[..], "b.ODR", 1982)]);
        let parsed = parse_packed(&bin).unwrap();
        assert_eq!(parsed.files.len(), 2);
        assert_eq!(parsed.files[0].name, "a.ODR");
        assert_eq!(parsed.files[0].year, 1981);
        assert_eq!(parsed.files[1].name, "b.ODR");
        assert_eq!(parsed.files[1].year, 1982);
        assert_eq!(parsed.files[1].records.len(), 2);
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
        assert_eq!(shard_name("voyager_odr", 0), "voyager_odr_s0.bin");
        assert_eq!(shard_name("voyager_odr", 13), "voyager_odr_s13.bin");
    }
}
