pub const MARINER5_UNIT: &str = "cps";

pub const MARINER5_SERIES_OFFSET_S: f64 = 2.05;

pub const UNIX_1950: f64 = -631_152_000.0;

pub const MARINER5_RECORD_BYTES: usize = 1206;
pub const MARINER5_BLOCKS: usize = 25;
pub const MARINER5_BLOCK_BYTES: usize = 48;
pub const MARINER5_BLOCK_HEADER_BYTES: usize = 6;

pub const CMR_RECORD_CHARS: usize = 534;
pub const CMR_WORDS: usize = 100;
pub const PT_RECORD_CHARS: usize = 2171;
pub const PT_WORDS: usize = 407;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Framing {
    BigEndian16,
    LittleEndian16,
    LittleEndian16Sequence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Record<'a> {
    pub sequence: Option<u8>,
    pub payload: &'a [u8],
}

pub fn records(bytes: &[u8], framing: Framing) -> Vec<Record<'_>> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 2 <= bytes.len() {
        let len = match framing {
            Framing::BigEndian16 => u16::from_be_bytes([bytes[i], bytes[i + 1]]) as usize,
            Framing::LittleEndian16 | Framing::LittleEndian16Sequence => {
                u16::from_le_bytes([bytes[i], bytes[i + 1]]) as usize
            }
        };
        i += 2;
        if len == 0 {
            out.push(Record {
                sequence: None,
                payload: &[],
            });
            continue;
        }
        if framing == Framing::LittleEndian16Sequence {
            let Some(&sequence) = bytes.get(i) else {
                break;
            };
            i += 1;
            let end = i.saturating_add(len).min(bytes.len());
            out.push(Record {
                sequence: Some(sequence),
                payload: &bytes[i..end],
            });
            i = end;
        } else {
            let end = i.saturating_add(len).min(bytes.len());
            out.push(Record {
                sequence: None,
                payload: &bytes[i..end],
            });
            i = end;
        }
    }
    out
}

pub fn word36(chars: &[u8]) -> Option<u64> {
    if chars.len() < 6 {
        return None;
    }
    Some(
        chars[..6]
            .iter()
            .fold(0u64, |w, &c| (w << 6) | u64::from(c & 0x3f)),
    )
}

pub fn unpack_six_bit(chars: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(chars.len() * 3 / 4 + 1);
    for chunk in chars.chunks(4) {
        let c0 = u32::from(chunk[0] & 0x3f);
        let Some(&raw1) = chunk.get(1) else {
            continue;
        };
        let c1 = u32::from(raw1 & 0x3f);
        out.push(((c0 << 2) | (c1 >> 4)) as u8);
        let Some(&raw2) = chunk.get(2) else {
            continue;
        };
        let c2 = u32::from(raw2 & 0x3f);
        out.push((((c1 & 0x0f) << 4) | (c2 >> 2)) as u8);
        let Some(&raw3) = chunk.get(3) else {
            continue;
        };
        let c3 = u32::from(raw3 & 0x3f);
        out.push((((c2 & 0x03) << 6) | c3) as u8);
    }
    out
}

pub fn pack_six_bit(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len() * 4 / 3 + 1);
    for chunk in bytes.chunks(3) {
        let b0 = u32::from(chunk[0]);
        let b1 = u32::from(*chunk.get(1).unwrap_or(&0));
        let b2 = u32::from(*chunk.get(2).unwrap_or(&0));
        out.push((b0 >> 2) as u8);
        out.push((((b0 & 0x03) << 4) | (b1 >> 4)) as u8);
        out.push((((b1 & 0x0f) << 2) | (b2 >> 6)) as u8);
        out.push((b2 & 0x3f) as u8);
    }
    out
}

pub fn base2_f36(w: u64) -> f64 {
    let sign = if (w >> 35) & 1 == 1 { -1.0 } else { 1.0 };
    let exponent = ((w >> 27) & 0xff) as i32;
    let fraction = (w & ((1u64 << 27) - 1)) as f64 / (1u64 << 27) as f64;
    if exponent == 0 {
        0.0
    } else {
        sign * fraction * 2f64.powi(exponent - 128)
    }
}

pub fn station_from_code(code: u64) -> Option<u8> {
    let byte = ((code >> 8) & 0xff) as u8;
    match byte {
        0x46 => Some(1),
        0x88..=0x8a => Some(2),
        0xcc..=0xce => Some(3),
        0x12 => Some(4),
        0x54 | 0x56 => Some(5),
        0x32 => Some(12),
        0x74 | 0x75 | 0x76 => Some(13),
        0xb8 | 0xba => Some(14),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mariner5Point {
    pub epoch_unix: f64,
    pub doppler_cps: f64,
    pub interval_s: f64,
    pub frequency_increment_hz: f64,
    pub pass_id: u64,
    pub code_word: u64,
    pub weight: u64,
    pub flags: u64,
    pub station: Option<u8>,
    pub record: usize,
    pub block: usize,
}

pub fn mariner5_points(bytes: &[u8]) -> Vec<Mariner5Point> {
    let mut out = Vec::new();
    for (record_index, record) in records(bytes, Framing::BigEndian16)
        .iter()
        .enumerate()
        .skip(1)
    {
        if record.payload.len() < MARINER5_RECORD_BYTES {
            continue;
        }
        for block in 0..MARINER5_BLOCKS {
            let start = block * MARINER5_BLOCK_BYTES + MARINER5_BLOCK_HEADER_BYTES;
            let Some(payload) = record.payload.get(start..start + MARINER5_BLOCK_BYTES) else {
                break;
            };
            let mut words = [0u64; 8];
            let mut complete = true;
            for (slot, word) in words.iter_mut().enumerate() {
                let at = slot * 6;
                let Some(value) = word36(&payload[at..at + 6]) else {
                    complete = false;
                    break;
                };
                *word = value;
            }
            if !complete {
                continue;
            }
            let time_s = base2_f36(words[0]);
            let doppler = base2_f36(words[5]);
            if !time_s.is_finite() || !doppler.is_finite() || time_s <= 0.0 || doppler == 0.0 {
                continue;
            }
            out.push(Mariner5Point {
                epoch_unix: UNIX_1950 + time_s + MARINER5_SERIES_OFFSET_S,
                doppler_cps: doppler,
                interval_s: base2_f36(words[1]),
                frequency_increment_hz: base2_f36(words[2]),
                pass_id: words[3],
                code_word: words[4],
                weight: words[6],
                flags: words[7],
                station: station_from_code(words[4]),
                record: record_index,
                block,
            });
        }
    }
    out.sort_by(|a, b| a.epoch_unix.total_cmp(&b.epoch_unix));
    out
}

#[derive(Clone, Debug, PartialEq)]
pub struct CmrRecord {
    pub index: usize,
    pub time_count: Option<u64>,
    pub words: Vec<u32>,
}

pub fn cmr_records(bytes: &[u8]) -> Vec<CmrRecord> {
    let mut out = Vec::new();
    for (index, record) in records(bytes, Framing::LittleEndian16).iter().enumerate() {
        if record.payload.len() < 70 {
            continue;
        }
        let unpacked = unpack_six_bit(record.payload);
        let mut words = vec![0u32; CMR_WORDS];
        for (slot, word) in words.iter_mut().enumerate() {
            let at = slot * 4;
            let Some(raw) = unpacked.get(at..at + 4) else {
                break;
            };
            *word = u32::from_be_bytes(raw.try_into().expect("four bytes"));
        }
        out.push(CmrRecord {
            index,
            time_count: record.payload.get(12..18).and_then(word36),
            words,
        });
    }
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PtLabelEpoch {
    pub year: i32,
    pub day_of_year: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

pub fn pt_label_epoch(bytes: &[u8]) -> Option<PtLabelEpoch> {
    let first = records(bytes, Framing::LittleEndian16Sequence)
        .into_iter()
        .next()?;
    let sequence = first.sequence?;
    if first.payload.len() != 59 {
        return None;
    }
    let mut packed = Vec::with_capacity(60);
    packed.push(sequence);
    packed.extend_from_slice(first.payload);
    let unpacked = unpack_six_bit(&packed);
    let year = 1900 + i32::from(*unpacked.get(36)?);
    let day = u32::from(u16::from_be_bytes([*unpacked.get(37)?, *unpacked.get(38)?]));
    let hour = u32::from(*unpacked.get(39)?);
    let minute = u32::from(*unpacked.get(40)?);
    let second = u32::from(*unpacked.get(41)?);
    if !(1900..=2100).contains(&year)
        || !(1..=366).contains(&day)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return None;
    }
    Some(PtLabelEpoch {
        year,
        day_of_year: day,
        hour,
        minute,
        second,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct PtRecord {
    pub index: usize,
    pub sequence: u8,
    pub words: Vec<u32>,
    pub range_km: Option<f64>,
    pub doppler_hz: Option<f64>,
}

pub fn pt_records(bytes: &[u8]) -> Vec<PtRecord> {
    let mut out = Vec::new();
    for (index, record) in records(bytes, Framing::LittleEndian16Sequence)
        .iter()
        .enumerate()
    {
        let Some(sequence) = record.sequence else {
            continue;
        };
        if record.payload.len() != PT_RECORD_CHARS {
            continue;
        }
        let mut packed = Vec::with_capacity(record.payload.len() + 1);
        packed.push(sequence);
        packed.extend_from_slice(record.payload);
        let unpacked = unpack_six_bit(&packed);
        let mut words = vec![0u32; PT_WORDS];
        for (slot, word) in words.iter_mut().enumerate() {
            let at = slot * 4;
            let Some(raw) = unpacked.get(at..at + 4) else {
                break;
            };
            *word = u32::from_be_bytes(raw.try_into().expect("four bytes"));
        }
        out.push(PtRecord {
            index,
            sequence,
            range_km: candidate_f32(&words, 3).filter(|v| (1.0e5..1.0e9).contains(v)),
            doppler_hz: candidate_f32(&words, 9).filter(|v| (1.0e2..1.0e8).contains(v)),
            words,
        });
    }
    out
}

fn candidate_f32(words: &[u32], slot: usize) -> Option<f64> {
    let value = f32::from_bits(*words.get(slot)?) as f64;
    (value.is_finite() && value != 0.0).then_some(value)
}

pub fn ebcdic_text(text: &[u8]) -> String {
    text.iter()
        .map(|&byte| match byte {
            0x40 => ' ',
            0x4b => '.',
            0x4c => '<',
            0x4d => '(',
            0x4e => '+',
            0x4f => '|',
            0x5a => '!',
            0x5b => '$',
            0x5c => '*',
            0x5d => ')',
            0x5e => ';',
            0x60 => '-',
            0x61 => '/',
            0x6b => ',',
            0x6c => '%',
            0x6d => '_',
            0x6e => '>',
            0x6f => '?',
            0x7a => ':',
            0x7b => '#',
            0x7c => '@',
            0x7d => '\'',
            0x7e => '=',
            0x7f => '"',
            0x81..=0x89 => (b'a' + byte - 0x81) as char,
            0x91..=0x99 => (b'j' + byte - 0x91) as char,
            0xa2..=0xa9 => (b's' + byte - 0xa2) as char,
            0xc1..=0xc9 => (b'A' + byte - 0xc1) as char,
            0xd1..=0xd9 => (b'J' + byte - 0xd1) as char,
            0xe2..=0xe9 => (b'S' + byte - 0xe2) as char,
            0xf0..=0xf9 => (b'0' + byte - 0xf0) as char,
            _ => ' ',
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_bit_word_follows_the_mariner_block_order() {
        assert_eq!(word36(&[1, 2, 3, 4, 5, 6]).unwrap(), 0x420c_4146);
        assert_eq!(word36(&[0, 0, 0, 0, 0, 0x3f]).unwrap(), 0x3f);
        assert!(word36(&[1, 2, 3]).is_none());
    }

    #[test]
    fn six_bit_pack_and_unsnake_roundtrip() {
        let bytes = [0x00, 0x01, 0xaf, 0x25, 0xff, 0x00];
        let packed = pack_six_bit(&bytes);
        assert_eq!(unpack_six_bit(&packed), bytes);
    }

    #[test]
    fn unsnake_short_groups_keep_the_bytes_the_characters_can_fill() {
        let chars = vec![0u8; CMR_RECORD_CHARS];
        assert_eq!(unpack_six_bit(&chars).len(), 400);
        let chars = vec![0u8; PT_RECORD_CHARS + 1];
        assert_eq!(unpack_six_bit(&chars).len(), 1629);
    }

    #[test]
    fn base2_f36_decodes_the_known_time_word() {
        assert_eq!(base2_f36(0x4f41_a495e), 550_652_656.0);
    }

    #[test]
    fn station_bytes_follow_the_measured_counts() {
        assert_eq!(station_from_code(0x8088_80a), Some(2));
        assert_eq!(station_from_code(0x80b7_60a), Some(13));
        assert_eq!(station_from_code(0x8091_20a), Some(4));
        assert_eq!(station_from_code(0x808c_e0a), Some(3));
        assert_eq!(station_from_code(0x80b3_20a), Some(12));
        assert_eq!(station_from_code(0x8084_60a), Some(1));
        assert_eq!(station_from_code(0x8095_60a), Some(5));
        assert_eq!(station_from_code(0x80bb_a0a), Some(14));
        assert_eq!(station_from_code(0xdead_beef), None);
    }

    #[test]
    fn big_endian_records_split_without_a_sequence() {
        let bytes = [0, 3, 10, 11, 12, 0, 2, 13, 14];
        let split = records(&bytes, Framing::BigEndian16);
        assert_eq!(split.len(), 2);
        assert_eq!(split[0].payload, &[10, 11, 12]);
        assert_eq!(split[1].payload, &[13, 14]);
    }

    #[test]
    fn sequence_records_keep_the_sequence_out_of_the_payload() {
        let bytes = [3, 0, 7, 1, 2, 3, 0, 0];
        let split = records(&bytes, Framing::LittleEndian16Sequence);
        assert_eq!(split.len(), 2);
        assert_eq!(split[0].sequence, Some(7));
        assert_eq!(split[0].payload, &[1, 2, 3]);
        assert_eq!(split[1].sequence, None);
        assert!(split[1].payload.is_empty());
    }

    #[test]
    fn little_endian_records_match_the_cmr_lengths() {
        let bytes = [3, 0, 1, 2, 3, 0, 0];
        let split = records(&bytes, Framing::LittleEndian16);
        assert_eq!(split[0].payload, &[1, 2, 3]);
        assert!(split[1].payload.is_empty());
    }

    #[test]
    fn pt_label_epoch_reads_the_year_day_and_clock() {
        let mut header = vec![0u8; 45];
        header[36] = 77;
        header[37] = 0;
        header[38] = 130;
        header[39] = 17;
        header[40] = 48;
        header[41] = 28;
        let packed = pack_six_bit(&header);
        let mut record = vec![59, 0];
        record.extend_from_slice(&packed);
        let epoch = pt_label_epoch(&record).expect("label epoch");
        assert_eq!(epoch.year, 1977);
        assert_eq!(epoch.day_of_year, 130);
        assert_eq!((epoch.hour, epoch.minute, epoch.second), (17, 48, 28));
    }

    #[test]
    fn empty_and_header_only_files_do_not_invent_points() {
        assert!(mariner5_points(&[]).is_empty());
        assert!(cmr_records(&[]).is_empty());
        assert!(pt_records(&[]).is_empty());
    }
}
