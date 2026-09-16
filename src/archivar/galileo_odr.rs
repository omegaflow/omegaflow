pub const RECORD_BYTES: usize = 2666;
pub const HEADER_BYTES: usize = 166;
pub const AD_GROUP_BYTES: usize = 4;
pub const AD_REPETITIONS: usize = 625;
pub const DATA_BYTES: usize = AD_REPETITIONS * AD_GROUP_BYTES;
pub const COMP_AD1: u32 = 1;
pub const COMP_AD2: u32 = 2;
pub const COMP_AD3: u32 = 3;
pub const COMP_AD4: u32 = 4;
pub const AD_TIMETAG_QUAD: f64 = 2.0;
pub const PACK_MAGIC: [u8; 4] = *b"GODR";
pub const PACK_ENTRY_BYTES: usize = 96;

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
    pub eight_bit: bool,
}

#[derive(Clone, Debug)]
pub struct OdrRecord {
    pub header: OdrHeader,
    pub ad: [[u8; AD_GROUP_BYTES]; AD_REPETITIONS],
}

#[derive(Clone, Debug)]
pub struct PackedOdrFile {
    pub name: String,
    pub sha256: [u8; 32],
    pub record_count: u32,
    pub sample_rate: u16,
    pub bytes: Vec<u8>,
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
        time_tag_ms: be32(bytes, 12) & 0x07FF_FFFF,
        sample_rate: be16(bytes, 158),
        eight_bit: bytes[0] & 0x10 != 0,
    })
}

pub fn record(bytes: &[u8]) -> Option<OdrRecord> {
    if bytes.len() < RECORD_BYTES {
        return None;
    }
    let header = header(bytes)?;
    let mut ad = [[0u8; AD_GROUP_BYTES]; AD_REPETITIONS];
    for (i, group) in ad.iter_mut().enumerate() {
        let base = HEADER_BYTES + i * AD_GROUP_BYTES;
        group.copy_from_slice(&bytes[base..base + AD_GROUP_BYTES]);
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

pub fn year_full(year: u8) -> Option<u16> {
    match year {
        90..=99 => Some(1900 + u16::from(year)),
        _ => None,
    }
}

pub fn time_tag_unix(year: u16, doy: u16, time_tag_ms: u32) -> Option<f64> {
    if !(1..=366).contains(&doy) || time_tag_ms > 86_399_999 {
        return None;
    }
    let days = crate::lsk::days_from_civil(year as i64, 1, 1)?;
    Some((days + doy as i64 - 1) as f64 * 86400.0 + time_tag_ms as f64 / 1000.0)
}

pub fn parse_godr_bin(bytes: &[u8]) -> Option<Vec<PackedOdrFile>> {
    if bytes.len() < 8 + PACK_ENTRY_BYTES || bytes[0..4] != PACK_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if count == 0 || bytes.len() < 8 + count * PACK_ENTRY_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let entry = &bytes[8 + i * PACK_ENTRY_BYTES..8 + (i + 1) * PACK_ENTRY_BYTES];
        let name_end = entry[0..32]
            .iter()
            .position(|b| *b == 0)
            .unwrap_or(32);
        let name = String::from_utf8(entry[0..name_end].to_vec()).ok()?;
        let mut sha256 = [0u8; 32];
        sha256.copy_from_slice(&entry[32..64]);
        let record_count = u32::from_le_bytes(entry[64..68].try_into().ok()?);
        let sample_rate = u16::from_le_bytes(entry[68..70].try_into().ok()?);
        let data_offset = u64::from_le_bytes(entry[72..80].try_into().ok()?) as usize;
        let data_length = u64::from_le_bytes(entry[80..88].try_into().ok()?) as usize;
        if data_offset + data_length > bytes.len() {
            return None;
        }
        out.push(PackedOdrFile {
            name,
            sha256,
            record_count,
            sample_rate,
            bytes: bytes[data_offset..data_offset + data_length].to_vec(),
        });
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let files = parse_godr_bin(bytes)?;
    let lsk = crate::archivar::membrane::embedded_lsk()?;
    let mut out = Vec::new();
    for f in &files {
        let recs = parse_odr(&f.bytes)?;
        for rec in &recs {
            let h = &rec.header;
            if !h.eight_bit || h.sample_rate == 0 {
                continue;
            }
            let Some(year) = year_full(h.year) else {
                continue;
            };
            let Some(unix) = time_tag_unix(year, h.doy, h.time_tag_ms) else {
                continue;
            };
            let Some(tdb) = lsk.unix_to_tdb(unix) else {
                continue;
            };
            let dt = 1.0 / f64::from(h.sample_rate);
            let t0 = tdb - AD_TIMETAG_QUAD * dt;
            for (i, quad) in rec.ad.iter().enumerate() {
                let t = t0 + i as f64 * dt;
                out.push((t, f64::from(quad[0]), COMP_AD1));
                out.push((t, f64::from(quad[1]), COMP_AD2));
                out.push((t, f64::from(quad[2]), COMP_AD3));
                out.push((t, f64::from(quad[3]), COMP_AD4));
            }
        }
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
        assert!(h.eight_bit);
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

    fn measured_63131033_record_one() -> [u8; HEADER_BYTES] {
        [
            0x92, 0x00, 0x1D, 0xC5, 0x05, 0x35, 0x2B, 0x00, 0x4D, 0x28, 0xC1, 0x39, 0x02, 0x43,
            0xFC, 0x90, 0x53, 0x47, 0x37, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x75, 0x43,
            0x10, 0x36, 0x74, 0x13, 0x14, 0x51, 0x02, 0x43, 0xF8, 0xA8, 0x07, 0x43, 0x10, 0x36,
            0x74, 0x09, 0x60, 0x92, 0x02, 0x43, 0xF8, 0xBC, 0x50, 0x09, 0x04, 0x91, 0xD4, 0xE9,
            0x49, 0x12, 0x00, 0x00, 0xD4, 0xE9, 0x49, 0x11, 0x20, 0x00, 0x11, 0x10, 0x02, 0x43,
            0xF8, 0xB2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF,
            0xF1, 0x5A, 0x33, 0x33, 0x33, 0x33, 0x77, 0x28, 0x77, 0x77, 0x00, 0x00, 0x00, 0x00,
            0x02, 0x43, 0xE0, 0x02, 0x00, 0x03, 0x03, 0x03, 0x00, 0x0E, 0x00, 0x0C, 0x20, 0x20,
            0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22, 0x43, 0xE0, 0x02, 0x02, 0xF5, 0x02, 0xF5,
            0x02, 0xF5, 0x02, 0xF5, 0xD2, 0x3F, 0x00, 0x00, 0x00, 0x00, 0xD2, 0x3F, 0x00, 0x00,
            0x00, 0x00, 0xD2, 0x3F, 0x00, 0x00, 0x00, 0x00, 0xD2, 0x3F, 0x00, 0x00, 0x00, 0x00,
            0x02, 0x43, 0xF8, 0xA8, 0x04, 0xE2, 0xA5, 0x5A, 0x00, 0x00, 0x35, 0x55,
        ]
    }

    #[test]
    fn header_decodes_measured_63131033_record_one() {
        let h = header(&measured_63131033_record_one()).unwrap();
        assert_eq!(h.record_number, 7621);
        assert_eq!(h.record_words, 1333);
        assert_eq!(h.spacecraft, 77);
        assert_eq!(h.spc, 40);
        assert_eq!(h.year, 96);
        assert_eq!(h.doy, 313);
        assert_eq!(h.time_tag_ms, 38_010_000);
        assert_eq!(h.sample_rate, 1250);
        assert!(h.eight_bit);
    }

    #[test]
    fn year_full_extends_the_measured_decade() {
        assert_eq!(year_full(96), Some(1996));
        assert_eq!(year_full(97), Some(1997));
        assert_eq!(year_full(90), Some(1990));
        assert_eq!(year_full(99), Some(1999));
        assert_eq!(year_full(3), None);
        assert_eq!(year_full(0), None);
    }

    #[test]
    fn time_tag_unix_builds_ms_epoch() {
        let days = crate::lsk::days_from_civil(1996, 1, 1).unwrap();
        let expected = (days + 312) as f64 * 86400.0 + 38_010.0;
        assert_eq!(time_tag_unix(1996, 313, 38_010_000), Some(expected));
        assert_eq!(time_tag_unix(1996, 0, 38_010_000), None);
        assert_eq!(time_tag_unix(1996, 313, 86_400_000), None);
    }

    fn godr_pack(records: &[Vec<u8>]) -> Vec<u8> {
        let count = records.len();
        let data_start = 8 + count * PACK_ENTRY_BYTES;
        let mut bin = vec![0u8; data_start];
        bin[0..4].copy_from_slice(&PACK_MAGIC);
        bin[4..8].copy_from_slice(&(count as u32).to_le_bytes());
        let mut offset = data_start as u64;
        for (i, r) in records.iter().enumerate() {
            let base = 8 + i * PACK_ENTRY_BYTES;
            let name = format!("rec{i}.ODR");
            bin[base..base + name.len()].copy_from_slice(name.as_bytes());
            bin[base + 32..base + 64]
                .copy_from_slice(&crate::archivar::sha256::sha256_raw(r));
            bin[base + 64..base + 68]
                .copy_from_slice(&((r.len() / RECORD_BYTES) as u32).to_le_bytes());
            bin[base + 68..base + 70].copy_from_slice(&1250u16.to_le_bytes());
            bin[base + 72..base + 80].copy_from_slice(&offset.to_le_bytes());
            bin[base + 80..base + 88].copy_from_slice(&(r.len() as u64).to_le_bytes());
            offset += r.len() as u64;
        }
        for r in records {
            bin.extend_from_slice(r);
        }
        bin
    }

    #[test]
    fn parse_godr_bin_reads_pack_structure() {
        let mut rec_bytes = sample_record(1);
        rec_bytes[0] = 0x92;
        let bin = godr_pack(&[rec_bytes.clone()]);
        let files = parse_godr_bin(&bin).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "rec0.ODR");
        assert_eq!(files[0].record_count, 1);
        assert_eq!(files[0].sample_rate, 1250);
        assert_eq!(files[0].bytes, rec_bytes);
        assert!(parse_godr_bin(b"X").is_none());
        let mut bad = bin;
        bad[80..88].copy_from_slice(&1_000_000u64.to_le_bytes());
        assert!(parse_godr_bin(&bad).is_none());
    }

    #[test]
    fn parse_series_emits_ad_quads_at_1250hz() {
        let mut rec_bytes = sample_record(1);
        rec_bytes[0..HEADER_BYTES].copy_from_slice(&measured_63131033_record_one());
        let bin = godr_pack(&[rec_bytes]);
        let series = parse_series(&bin).unwrap();
        assert_eq!(series.len(), AD_REPETITIONS * AD_GROUP_BYTES);
        let lsk = crate::archivar::membrane::embedded_lsk().unwrap();
        let tdb_tag = lsk.unix_to_tdb(time_tag_unix(1996, 313, 38_010_000).unwrap()).unwrap();
        let dt = 1.0 / 1250.0;
        assert_eq!(series[0].0, tdb_tag - AD_TIMETAG_QUAD * dt);
        assert!((series[8].0 - tdb_tag).abs() < 1e-6);
        assert_eq!(series[0].1, 0.0);
        assert_eq!(series[0].2, COMP_AD1);
        assert_eq!(series[1].1, 0.0);
        assert_eq!(series[1].2, COMP_AD2);
        assert_eq!(series[2].2, COMP_AD3);
        assert_eq!(series[3].2, COMP_AD4);
        assert!((series[4].0 - series[0].0 - dt).abs() < 1e-12);
        assert_eq!(series[4].1, 1.0);
    }

    #[test]
    fn parse_series_skips_twelve_bit_and_void_rate_records() {
        let mut twelve = sample_record(1);
        twelve[0] = 0xC2;
        let bin = godr_pack(&[twelve]);
        assert!(parse_series(&bin).unwrap().is_empty());

        let mut no_rate = sample_record(1);
        no_rate[158..160].copy_from_slice(&0u16.to_be_bytes());
        let bin = godr_pack(&[no_rate]);
        assert!(parse_series(&bin).unwrap().is_empty());

        assert!(parse_series(b"X").is_none());
    }
}
