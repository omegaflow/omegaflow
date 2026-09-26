pub const PACK_MAGIC: [u8; 4] = *b"VOCC";
pub const PACK_ENTRY_BYTES: usize = 84;
pub const PACK_YEAR_OFFSET: usize = 80;

pub const LEN_PREFIX_BYTES: usize = 2;

pub const MED_PAYLOAD_BYTES: usize = 4800;
pub const MED_RECORD_BYTES: usize = LEN_PREFIX_BYTES + MED_PAYLOAD_BYTES;
pub const MED_HEADER_BYTES: usize = 704;
pub const MED_DATA_VALUES: usize = 512;
pub const MED_DATA_BYTES: usize = MED_DATA_VALUES * 8;
pub const MED_T0_OFFSET: usize = 0;
pub const MED_T0_BYTES: usize = 8;
pub const MED_TIMETAGDAYS_OFFSET: usize = 8;
pub const MED_TIMETAGDAYS_BYTES: usize = 2;
pub const MED_VALUE_COUNT_OFFSET: usize = 10;
pub const MED_MAGIC: u32 = 0x6fff_ffff;

pub const NB_HEADER_PAYLOAD_BYTES: usize = 120;
pub const NB_HEADER_RECORD_BYTES: usize = LEN_PREFIX_BYTES + NB_HEADER_PAYLOAD_BYTES;
pub const NB_DATA_RECORD_PAYLOAD_BYTES: usize = 4096;
pub const NB_DATA_RECORD_BYTES: usize = LEN_PREFIX_BYTES + NB_DATA_RECORD_PAYLOAD_BYTES;
pub const NB_DATA_VALUES: usize = 1024;

pub const COMP_AMP_MIN: u32 = 1;
pub const COMP_AMP_MAX: u32 = 2;
pub const COMP_AMP_MEAN: u32 = 3;

fn be16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes([bytes[0], bytes[1]])
}

fn be32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

#[derive(Clone, Debug, PartialEq)]
pub struct MediumbandRecord {
    pub header: [u8; MED_HEADER_BYTES],
    pub data: [u8; MED_DATA_BYTES],
}

pub fn mediumband_record(record: &[u8]) -> Option<MediumbandRecord> {
    if record.len() != MED_RECORD_BYTES {
        return None;
    }
    if be16(&record[0..LEN_PREFIX_BYTES]) as usize != MED_PAYLOAD_BYTES {
        return None;
    }
    let payload = &record[LEN_PREFIX_BYTES..];
    if be32(&payload[4..8]) != MED_MAGIC {
        return None;
    }
    if be16(&payload[MED_VALUE_COUNT_OFFSET..MED_VALUE_COUNT_OFFSET + 2]) as usize
        != MED_DATA_VALUES
    {
        return None;
    }
    let mut header = [0u8; MED_HEADER_BYTES];
    header.copy_from_slice(&payload[..MED_HEADER_BYTES]);
    let mut data = [0u8; MED_DATA_BYTES];
    data.copy_from_slice(&payload[MED_HEADER_BYTES..]);
    Some(MediumbandRecord { header, data })
}

pub fn parse_mediumband(bytes: &[u8]) -> Option<Vec<MediumbandRecord>> {
    let mut out = Vec::new();
    let mut offset = 0usize;
    while offset < bytes.len() {
        if offset + MED_RECORD_BYTES > bytes.len() {
            return None;
        }
        out.push(mediumband_record(
            &bytes[offset..offset + MED_RECORD_BYTES],
        )?);
        offset += MED_RECORD_BYTES;
    }
    Some(out)
}

pub fn t0_ms(r: &MediumbandRecord) -> u32 {
    be32(&r.header[MED_T0_OFFSET..MED_T0_OFFSET + 4])
}

pub fn timetagdays(r: &MediumbandRecord) -> u16 {
    be16(&r.header[MED_TIMETAGDAYS_OFFSET..MED_TIMETAGDAYS_OFFSET + 2])
}

pub fn epoch_anchor(year: u32) -> Option<f64> {
    if year == 0 {
        return None;
    }
    let days = crate::lsk::days_from_civil(year as i64, 1, 1)?;
    let t = days as f64 * 86_400.0;
    if t.is_finite() && t > 0.0 {
        Some(t)
    } else {
        None
    }
}

pub fn sample_epoch(anchor: f64, r: &MediumbandRecord) -> Option<f64> {
    let doy = timetagdays(r);
    if doy == 0 || doy > 366 {
        return None;
    }
    let t = anchor + (f64::from(doy) - 1.0) * 86_400.0;
    if t.is_finite() && t > 0.0 {
        Some(t)
    } else {
        None
    }
}

pub fn mediumband_amps(r: &MediumbandRecord) -> Option<(f64, f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0f64;
    for k in 0..MED_DATA_VALUES {
        let base = k * 8;
        let i = f32::from_be_bytes([
            r.data[base],
            r.data[base + 1],
            r.data[base + 2],
            r.data[base + 3],
        ]) as f64;
        let q = f32::from_be_bytes([
            r.data[base + 4],
            r.data[base + 5],
            r.data[base + 6],
            r.data[base + 7],
        ]) as f64;
        let amp = i.hypot(q);
        if !amp.is_finite() {
            return None;
        }
        if amp < min {
            min = amp;
        }
        if amp > max {
            max = amp;
        }
        sum += amp;
    }
    Some((min, max, sum / MED_DATA_VALUES as f64))
}

#[derive(Clone, Debug, PartialEq)]
pub struct NarrowbandFile {
    pub header: [u8; NB_HEADER_PAYLOAD_BYTES],
    pub records: Vec<[u8; NB_DATA_RECORD_PAYLOAD_BYTES]>,
}

pub fn parse_narrowband(bytes: &[u8]) -> Option<NarrowbandFile> {
    if bytes.len() < NB_HEADER_RECORD_BYTES {
        return None;
    }
    if be16(&bytes[0..LEN_PREFIX_BYTES]) as usize != NB_HEADER_PAYLOAD_BYTES {
        return None;
    }
    let mut header = [0u8; NB_HEADER_PAYLOAD_BYTES];
    header.copy_from_slice(&bytes[LEN_PREFIX_BYTES..NB_HEADER_RECORD_BYTES]);
    let mut records = Vec::new();
    let mut offset = NB_HEADER_RECORD_BYTES;
    while offset < bytes.len() {
        if offset + NB_DATA_RECORD_BYTES > bytes.len() {
            return None;
        }
        if be16(&bytes[offset..offset + LEN_PREFIX_BYTES]) as usize != NB_DATA_RECORD_PAYLOAD_BYTES
        {
            return None;
        }
        let mut data = [0u8; NB_DATA_RECORD_PAYLOAD_BYTES];
        data.copy_from_slice(&bytes[offset + LEN_PREFIX_BYTES..offset + NB_DATA_RECORD_BYTES]);
        records.push(data);
        offset += NB_DATA_RECORD_BYTES;
    }
    Some(NarrowbandFile { header, records })
}

#[derive(Clone, Debug, PartialEq)]
pub struct VoccFile {
    pub name: String,
    pub sha256: [u8; 32],
    pub year: u32,
    pub raw: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackedVocc {
    pub files: Vec<VoccFile>,
}

pub fn pack(raw: &[u8], name: &str) -> Vec<u8> {
    pack_many(&[(raw, name, 0)])
}

pub fn pack_many(files: &[(&[u8], &str, u32)]) -> Vec<u8> {
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
        bin[base + 64..base + 72].copy_from_slice(&(offset as u64).to_le_bytes());
        bin[base + 72..base + 80].copy_from_slice(&(raw.len() as u64).to_le_bytes());
        bin[base + PACK_YEAR_OFFSET..base + PACK_YEAR_OFFSET + 4]
            .copy_from_slice(&year.to_le_bytes());
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
        let data_offset = u64::from_le_bytes(entry[64..72].try_into().ok()?) as usize;
        let data_length = u64::from_le_bytes(entry[72..80].try_into().ok()?) as usize;
        let year = u32::from_le_bytes(
            entry[PACK_YEAR_OFFSET..PACK_YEAR_OFFSET + 4]
                .try_into()
                .ok()?,
        );
        if data_offset + data_length > bytes.len() {
            return None;
        }
        let raw = &bytes[data_offset..data_offset + data_length];
        if crate::archivar::sha256::sha256_raw(raw) != sha256 {
            return None;
        }
        files.push(VoccFile {
            name,
            sha256,
            year,
            raw: raw.to_vec(),
        });
    }
    Some(PackedVocc { files })
}

pub fn parse_series(data: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let packed = parse_packed(data)?;
    let mut out = Vec::new();
    for file in &packed.files {
        if let Some(records) = parse_mediumband(&file.raw) {
            let Some(anchor) = epoch_anchor(file.year) else {
                continue;
            };
            for r in &records {
                let Some(t) = sample_epoch(anchor, r) else {
                    return None;
                };
                let (mn, mx, mean) = mediumband_amps(r)?;
                out.push((t, mn, COMP_AMP_MIN));
                out.push((t, mx, COMP_AMP_MAX));
                out.push((t, mean, COMP_AMP_MEAN));
            }
            continue;
        }
        if parse_narrowband(&file.raw).is_none() {
            continue;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_mediumband_record() -> Vec<u8> {
        let mut rec = vec![0u8; MED_RECORD_BYTES];
        rec[0..2].copy_from_slice(&(MED_PAYLOAD_BYTES as u16).to_be_bytes());
        rec[LEN_PREFIX_BYTES..LEN_PREFIX_BYTES + 4]
            .copy_from_slice(&1_231_585_997u32.to_be_bytes());
        rec[LEN_PREFIX_BYTES + 4..LEN_PREFIX_BYTES + 8].copy_from_slice(&MED_MAGIC.to_be_bytes());
        rec[LEN_PREFIX_BYTES + 8..LEN_PREFIX_BYTES + 10].copy_from_slice(&317u16.to_be_bytes());
        rec[LEN_PREFIX_BYTES + MED_VALUE_COUNT_OFFSET
            ..LEN_PREFIX_BYTES + MED_VALUE_COUNT_OFFSET + 2]
            .copy_from_slice(&(MED_DATA_VALUES as u16).to_be_bytes());
        let data_base = LEN_PREFIX_BYTES + MED_HEADER_BYTES;
        for k in 0..MED_DATA_VALUES {
            let base = data_base + k * 8;
            let (i, q) = if k == 0 {
                (1.0f32, 0.0f32)
            } else {
                (0.0f32, 1.0f32)
            };
            rec[base..base + 4].copy_from_slice(&i.to_be_bytes());
            rec[base + 4..base + 8].copy_from_slice(&q.to_be_bytes());
        }
        rec
    }

    fn sample_narrowband_file(records: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; NB_HEADER_RECORD_BYTES + records * NB_DATA_RECORD_BYTES];
        bytes[0..2].copy_from_slice(&(NB_HEADER_PAYLOAD_BYTES as u16).to_be_bytes());
        bytes[LEN_PREFIX_BYTES + 64..LEN_PREFIX_BYTES + 66].copy_from_slice(&318u16.to_be_bytes());
        for k in 0..records {
            let base = NB_HEADER_RECORD_BYTES + k * NB_DATA_RECORD_BYTES;
            bytes[base..base + 2]
                .copy_from_slice(&(NB_DATA_RECORD_PAYLOAD_BYTES as u16).to_be_bytes());
            let v = 1.0f32;
            bytes[base + LEN_PREFIX_BYTES..base + LEN_PREFIX_BYTES + 4]
                .copy_from_slice(&v.to_be_bytes());
        }
        bytes
    }

    #[test]
    fn measured_record_totals_hold() {
        assert_eq!(MED_RECORD_BYTES * 3662, 17_584_924);
        assert_eq!(
            NB_HEADER_RECORD_BYTES + 3908 * NB_DATA_RECORD_BYTES,
            16_015_106
        );
        assert_eq!(MED_HEADER_BYTES, 704);
        assert_eq!(MED_DATA_BYTES, MED_DATA_VALUES * 8);
        assert_eq!(NB_HEADER_PAYLOAD_BYTES, 120);
        assert_eq!(NB_DATA_RECORD_PAYLOAD_BYTES / NB_DATA_VALUES, 4);
    }

    #[test]
    fn mediumband_record_frames_the_measured_geometry() {
        let rec = sample_mediumband_record();
        let r = mediumband_record(&rec).unwrap();
        assert_eq!(r.header.len(), MED_HEADER_BYTES);
        assert_eq!(r.data.len(), MED_DATA_BYTES);
        assert_eq!(t0_ms(&r), 1_231_585_997);
        assert_eq!(timetagdays(&r), 317);
        let (mn, mx, mean) = mediumband_amps(&r).unwrap();
        assert_eq!(mn, 1.0);
        assert_eq!(mx, 1.0);
        assert_eq!(mean, 1.0);
    }

    #[test]
    fn mediumband_record_voids_on_little_endian_length_and_bad_magic() {
        let mut rec = sample_mediumband_record();
        rec[0] = 0x00;
        rec[1] = 0x12;
        assert!(mediumband_record(&rec).is_none());
        let mut rec = sample_mediumband_record();
        rec[LEN_PREFIX_BYTES + 4] ^= 0xff;
        assert!(mediumband_record(&rec).is_none());
        assert!(mediumband_record(&rec[..rec.len() - 1]).is_none());
    }

    #[test]
    fn parse_mediumband_counts_complete_records() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&sample_mediumband_record());
        bytes.extend_from_slice(&sample_mediumband_record());
        bytes.extend_from_slice(&[0u8; 512]);
        assert!(parse_mediumband(&bytes).is_none());
        let recs = parse_mediumband(&bytes[..bytes.len() - 512]).unwrap();
        assert_eq!(recs.len(), 2);
    }

    #[test]
    fn narrowband_frames_header_and_data_records() {
        let bytes = sample_narrowband_file(2);
        let nb = parse_narrowband(&bytes).unwrap();
        assert_eq!(nb.header.len(), NB_HEADER_PAYLOAD_BYTES);
        assert_eq!(nb.records.len(), 2);
        assert_eq!(u16::from_be_bytes([nb.header[64], nb.header[65]]), 318);
        assert!(parse_narrowband(&bytes[..bytes.len() - 1]).is_none());
        assert!(parse_narrowband(b"X").is_none());
    }

    #[test]
    fn pack_roundtrips_raw_members_with_sha256() {
        let raw = sample_mediumband_record();
        let bin = pack(&raw, "DD059817_F1.DAT");
        let parsed = parse_packed(&bin).unwrap();
        assert_eq!(parsed.files.len(), 1);
        assert_eq!(parsed.files[0].name, "DD059817_F1.DAT");
        assert_eq!(parsed.files[0].raw, raw);
        assert_eq!(
            parsed.files[0].sha256,
            crate::archivar::sha256::sha256_raw(&raw)
        );
        assert!(parse_packed(b"X").is_none());
    }

    #[test]
    fn parse_packed_voids_on_wrong_magic_and_corruption() {
        let raw = sample_mediumband_record();
        let bin = pack(&raw, "DD059817_F1.DAT");
        let mut wrong_magic = bin.clone();
        wrong_magic[0] = b'X';
        assert!(parse_packed(&wrong_magic).is_none());
        let mut corrupted = bin;
        let last = corrupted.len() - 1;
        corrupted[last] ^= 0xff;
        assert!(parse_packed(&corrupted).is_none());
    }

    #[test]
    fn parse_series_keeps_mediumband_rows_pending_until_epoch_anchor() {
        let raw = sample_mediumband_record();
        let bin = pack(&raw, "DD059817_F1.DAT");
        let series = parse_series(&bin).unwrap();
        assert!(series.is_empty());
        assert!(parse_series(b"X").is_none());
    }

    #[test]
    fn parse_series_builds_epoch_from_packed_year() {
        let raw = sample_mediumband_record();
        let bin = pack_many(&[(&raw[..], "DD059817_F1.DAT", 1980)]);
        let parsed = parse_packed(&bin).unwrap();
        assert_eq!(parsed.files[0].year, 1980);
        let series = parse_series(&bin).unwrap();
        assert_eq!(series.len(), 3);
        assert_eq!(series[0].0, 315_532_800.0 + 316.0 * 86_400.0);
        assert_eq!(series[0].2, COMP_AMP_MIN);
        assert_eq!(series[1].2, COMP_AMP_MAX);
        assert_eq!(series[2].2, COMP_AMP_MEAN);
    }

    #[test]
    fn epoch_anchor_builds_year_start_and_sample_epoch_needs_it() {
        assert!(epoch_anchor(0).is_none());
        assert_eq!(epoch_anchor(1980), Some(315_532_800.0));
        assert!(epoch_anchor(1899).is_none());
        let r = mediumband_record(&sample_mediumband_record()).unwrap();
        assert_eq!(
            sample_epoch(315_532_800.0, &r),
            Some(315_532_800.0 + 316.0 * 86_400.0)
        );
        assert_eq!(sample_epoch(-1.0e12, &r), None);
    }

    #[test]
    fn sample_epoch_voids_on_absent_day_of_year() {
        let mut rec = sample_mediumband_record();
        rec[LEN_PREFIX_BYTES + 8..LEN_PREFIX_BYTES + 10].copy_from_slice(&0u16.to_be_bytes());
        let r = mediumband_record(&rec).unwrap();
        assert!(sample_epoch(315_532_800.0, &r).is_none());
    }

    #[test]
    fn parse_series_keeps_narrowband_rows_pending() {
        let bytes = sample_narrowband_file(2);
        let bin = pack(&bytes, "DD059825_F1.DAT");
        let series = parse_series(&bin).unwrap();
        assert!(series.is_empty());
    }

    #[test]
    fn parse_series_mixes_bands_and_skips_unframed_files() {
        let med = sample_mediumband_record();
        let nb = sample_narrowband_file(1);
        let zero = vec![0u8; MED_RECORD_BYTES];
        let bin = pack_many(&[
            (&med[..], "DD059817_F1.DAT", 0),
            (&nb[..], "DD059825_F1.DAT", 0),
            (&zero[..], "S0A.DAT", 0),
        ]);
        let series = parse_series(&bin).unwrap();
        assert!(series.is_empty());
    }
}
