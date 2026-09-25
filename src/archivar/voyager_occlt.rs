pub const PACK_MAGIC: [u8; 4] = *b"VOCC";
pub const PACK_ENTRY_BYTES: usize = 84;

pub const MED_HEADER_COMPLEX_WORDS: usize = 88;
pub const MED_HEADER_BYTES: usize = MED_HEADER_COMPLEX_WORDS * 4;
pub const MED_RECORD_INT_WORDS: usize = 2528;
pub const MED_RECORD_BYTES: usize = MED_RECORD_INT_WORDS * 2;
pub const MED_DATA_VALUES: usize = 512;
pub const MED_DATA_BYTES: usize = MED_RECORD_BYTES - MED_HEADER_BYTES;
pub const MED_T0_OFFSET: usize = 0;
pub const MED_T0_BYTES: usize = 8;
pub const MED_TIMETAGDAYS_OFFSET: usize = 8;
pub const MED_TIMETAGDAYS_BYTES: usize = 2;

pub const NB_HEADER_COMPLEX_WORDS: usize = 15;
pub const NB_TIME_WORDS_LO: usize = 29;
pub const NB_TIME_WORDS_HI: usize = 33;

#[derive(Clone, Debug, PartialEq)]
pub struct MediumbandRecord {
    pub header: [u8; MED_HEADER_BYTES],
    pub data: [u8; MED_DATA_BYTES],
}

pub fn mediumband_record(bytes: &[u8]) -> Option<MediumbandRecord> {
    if bytes.len() != MED_RECORD_BYTES {
        return None;
    }
    let mut header = [0u8; MED_HEADER_BYTES];
    header.copy_from_slice(&bytes[..MED_HEADER_BYTES]);
    let mut data = [0u8; MED_DATA_BYTES];
    data.copy_from_slice(&bytes[MED_HEADER_BYTES..]);
    Some(MediumbandRecord { header, data })
}

pub fn t0_bytes(r: &MediumbandRecord) -> &[u8] {
    &r.header[MED_T0_OFFSET..MED_T0_OFFSET + MED_T0_BYTES]
}

pub fn timetagdays_bytes(r: &MediumbandRecord) -> &[u8] {
    &r.header[MED_TIMETAGDAYS_OFFSET..MED_TIMETAGDAYS_OFFSET + MED_TIMETAGDAYS_BYTES]
}

pub fn split_records(byte_len: usize) -> (usize, usize) {
    (byte_len / MED_RECORD_BYTES, byte_len % MED_RECORD_BYTES)
}

pub fn parse_mediumband(bytes: &[u8]) -> Option<Vec<MediumbandRecord>> {
    let (n, _) = split_records(bytes.len());
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(mediumband_record(
            &bytes[i * MED_RECORD_BYTES..(i + 1) * MED_RECORD_BYTES],
        )?);
    }
    Some(out)
}

#[derive(Clone, Debug, PartialEq)]
pub struct VoccFile {
    pub name: String,
    pub sha256: [u8; 32],
    pub records: Vec<MediumbandRecord>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackedVocc {
    pub files: Vec<VoccFile>,
}

pub fn pack(raw: &[u8], name: &str) -> Vec<u8> {
    pack_many(&[(raw, name)])
}

pub fn pack_many(files: &[(&[u8], &str)]) -> Vec<u8> {
    let data_start = 8 + files.len() * PACK_ENTRY_BYTES;
    let data_bytes: usize = files.iter().map(|(raw, _)| raw.len()).sum();
    let mut bin = vec![0u8; data_start + data_bytes];
    bin[0..4].copy_from_slice(&PACK_MAGIC);
    bin[4..8].copy_from_slice(&(files.len() as u32).to_le_bytes());
    let mut offset = data_start;
    for (i, (raw, name)) in files.iter().enumerate() {
        let base = 8 + i * PACK_ENTRY_BYTES;
        let nameb = name.as_bytes();
        let n = nameb.len().min(32);
        bin[base..base + n].copy_from_slice(&nameb[..n]);
        bin[base + 32..base + 64].copy_from_slice(&crate::archivar::sha256::sha256_raw(raw));
        bin[base + 64..base + 68]
            .copy_from_slice(&((raw.len() / MED_RECORD_BYTES) as u32).to_le_bytes());
        bin[base + 68..base + 76].copy_from_slice(&(offset as u64).to_le_bytes());
        bin[base + 76..base + 84].copy_from_slice(&(raw.len() as u64).to_le_bytes());
        bin[offset..offset + raw.len()].copy_from_slice(raw);
        offset += raw.len();
    }
    bin
}

pub fn parse_packed(bytes: &[u8]) -> Option<PackedVocc> {
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
        let data_offset = u64::from_le_bytes(entry[68..76].try_into().ok()?) as usize;
        let data_length = u64::from_le_bytes(entry[76..84].try_into().ok()?) as usize;
        if data_offset + data_length > bytes.len() {
            return None;
        }
        let raw = &bytes[data_offset..data_offset + data_length];
        if crate::archivar::sha256::sha256_raw(raw) != sha256 {
            return None;
        }
        if raw.len() != record_count * MED_RECORD_BYTES {
            return None;
        }
        let records = parse_mediumband(raw)?;
        if records.len() != record_count {
            return None;
        }
        files.push(VoccFile {
            name,
            sha256,
            records,
        });
    }
    Some(PackedVocc { files })
}

pub fn parse_series(data: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    parse_packed(data)?;
    Some(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_record() -> Vec<u8> {
        let mut bytes = vec![0u8; MED_RECORD_BYTES];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (i % 251) as u8;
        }
        bytes
    }

    #[test]
    fn mediumband_record_frames_the_measured_record_length() {
        let bytes = sample_record();
        let r = mediumband_record(&bytes).unwrap();
        assert_eq!(r.header.len(), MED_HEADER_BYTES);
        assert_eq!(r.data.len(), MED_DATA_BYTES);
        assert!(mediumband_record(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn t0_and_timetagdays_slots_lie_inside_the_measured_header() {
        assert!(MED_T0_OFFSET + MED_T0_BYTES <= MED_HEADER_BYTES);
        assert!(MED_TIMETAGDAYS_OFFSET + MED_TIMETAGDAYS_BYTES <= MED_HEADER_BYTES);
        let r = mediumband_record(&sample_record()).unwrap();
        assert_eq!(
            t0_bytes(&r),
            &r.header[MED_T0_OFFSET..MED_T0_OFFSET + MED_T0_BYTES]
        );
        assert_eq!(
            timetagdays_bytes(&r),
            &r.header[MED_TIMETAGDAYS_OFFSET..MED_TIMETAGDAYS_OFFSET + MED_TIMETAGDAYS_BYTES]
        );
    }

    #[test]
    fn mediumband_data_value_stride_stays_unmeasured() {
        assert_ne!(MED_DATA_BYTES % MED_DATA_VALUES, 0);
    }

    #[test]
    fn narrowband_record_header_cannot_carry_the_measured_file_time_words() {
        assert!(NB_HEADER_COMPLEX_WORDS * 2 < NB_TIME_WORDS_HI);
    }

    #[test]
    fn parse_mediumband_counts_complete_records() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&sample_record());
        bytes.extend_from_slice(&sample_record());
        bytes.extend_from_slice(&[0u8; 512]);
        assert_eq!(split_records(bytes.len()), (2, 512));
        let recs = parse_mediumband(&bytes).unwrap();
        assert_eq!(recs.len(), 2);
    }

    #[test]
    fn pack_roundtrips_raw_records_with_sha256() {
        let raw = sample_record();
        let records = parse_mediumband(&raw).unwrap();
        let bin = pack(&raw, "S0A.DAT");
        let parsed = parse_packed(&bin).unwrap();
        assert_eq!(parsed.files.len(), 1);
        assert_eq!(parsed.files[0].name, "S0A.DAT");
        assert_eq!(parsed.files[0].records, records);
        assert_eq!(
            parsed.files[0].sha256,
            crate::archivar::sha256::sha256_raw(&raw)
        );
        assert!(parse_packed(b"X").is_none());
    }

    #[test]
    fn parse_packed_voids_on_wrong_magic_and_corruption() {
        let raw = sample_record();
        let bin = pack(&raw, "S0A.DAT");
        let mut wrong_magic = bin.clone();
        wrong_magic[0] = b'X';
        assert!(parse_packed(&wrong_magic).is_none());
        let mut corrupted = bin;
        let last = corrupted.len() - 1;
        corrupted[last] ^= 0xff;
        assert!(parse_packed(&corrupted).is_none());
    }

    #[test]
    fn parse_series_emits_zero_rows_while_the_sample_decode_is_pending() {
        let raw = sample_record();
        let bin = pack(&raw, "S0A.DAT");
        let series = parse_series(&bin).unwrap();
        assert!(series.is_empty());
        assert!(parse_series(b"X").is_none());
    }
}
