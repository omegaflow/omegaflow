pub const RECORD_HEADER_BYTES: usize = 2;
pub const RECORD_DATA_BYTES: usize = 8064;
pub const RECORD_BYTES: usize = RECORD_HEADER_BYTES + RECORD_DATA_BYTES;
pub const SUB_RECORD_BYTES: usize = 288;
pub const SUB_RECORDS_PER_RECORD: usize = RECORD_DATA_BYTES / SUB_RECORD_BYTES;
pub const WORDS_PER_SUB_RECORD: usize = 64;
pub const WORD_BITS: usize = 36;

pub const DOPPLER_W7: u64 = 0x1770;
pub const RANGE_BIT: u64 = 0x000800000;

pub const VSAT_STRIDE: usize = 7;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackingKind {
    Doppler,
    Range,
    Sync,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoyagerSaturnRecord {
    pub kind: TrackingKind,
    pub id_word: u64,
    pub time_word: u64,
    pub flag_word: u64,
    pub counter_word: u64,
    pub value_word: u64,
    pub secondary_word: u64,
    pub status_a: u64,
    pub status_b: u64,
}

pub fn unpack_word(bytes: &[u8], bit_offset: usize) -> Option<u64> {
    if bit_offset + WORD_BITS > bytes.len() * 8 {
        return None;
    }
    let mut value = 0u64;
    for b in 0..WORD_BITS {
        let bit = bit_offset + b;
        value = (value << 1) | u64::from((bytes[bit >> 3] >> (7 - (bit & 7))) & 1);
    }
    Some(value)
}

pub fn sub_words(bytes: &[u8]) -> Option<[u64; WORDS_PER_SUB_RECORD]> {
    if bytes.len() < SUB_RECORD_BYTES {
        return None;
    }
    let mut words = [0u64; WORDS_PER_SUB_RECORD];
    for (i, slot) in words.iter_mut().enumerate() {
        *slot = unpack_word(bytes, i * WORD_BITS)?;
    }
    Some(words)
}

pub fn kind(words: &[u64; WORDS_PER_SUB_RECORD]) -> Option<TrackingKind> {
    let w6 = words[6];
    let w7 = words[7];
    if w7 == DOPPLER_W7 {
        Some(TrackingKind::Doppler)
    } else if w7 == 0 && w6 & RANGE_BIT != 0 {
        Some(TrackingKind::Range)
    } else if w7 == 0 && w6 != 0 {
        Some(TrackingKind::Sync)
    } else {
        None
    }
}

pub fn record(words: &[u64; WORDS_PER_SUB_RECORD]) -> Option<VoyagerSaturnRecord> {
    let kind = kind(words)?;
    Some(VoyagerSaturnRecord {
        kind,
        id_word: words[2],
        time_word: words[3],
        flag_word: words[4],
        counter_word: words[8],
        value_word: words[9],
        secondary_word: words[11],
        status_a: words[15],
        status_b: words[16],
    })
}

pub fn parse(bytes: &[u8]) -> Option<Vec<VoyagerSaturnRecord>> {
    let mut out = Vec::new();
    let mut offset = 0usize;
    while offset < bytes.len() {
        if offset + RECORD_HEADER_BYTES > bytes.len() {
            return None;
        }
        let data_len = usize::from(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
        let end = offset + RECORD_HEADER_BYTES + data_len;
        if end > bytes.len() || data_len % SUB_RECORD_BYTES != 0 {
            return None;
        }
        for s in 0..(data_len / SUB_RECORD_BYTES) {
            let base = offset + RECORD_HEADER_BYTES + s * SUB_RECORD_BYTES;
            let words = sub_words(&bytes[base..base + SUB_RECORD_BYTES])?;
            if let Some(r) = record(&words) {
                out.push(r);
            }
        }
        offset = end;
    }
    Some(out)
}

pub fn kind_code(kind: TrackingKind) -> f64 {
    match kind {
        TrackingKind::Doppler => 0.0,
        TrackingKind::Range => 1.0,
        TrackingKind::Sync => 2.0,
    }
}

pub fn to_bin_row(r: &VoyagerSaturnRecord) -> [f64; VSAT_STRIDE] {
    [
        kind_code(r.kind),
        r.time_word as f64,
        r.counter_word as f64,
        r.value_word as f64,
        r.secondary_word as f64,
        r.status_a as f64,
        r.status_b as f64,
    ]
}

pub fn write_vsat_bin(records: &[[f64; VSAT_STRIDE]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * VSAT_STRIDE * 8);
    out.extend_from_slice(b"VSAT");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_vsat_bin(data: &[u8]) -> Option<Vec<[f64; VSAT_STRIDE]>> {
    if data.len() < 8 || &data[0..4] != b"VSAT" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * VSAT_STRIDE * 8 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * VSAT_STRIDE * 8;
        let mut r = [0.0f64; VSAT_STRIDE];
        for k in 0..VSAT_STRIDE {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUB_A: [u8; 288] = [
        0x00, 0x00, 0x00, 0x01, 0x20, 0x00, 0x00, 0x00, 0x5a, 0x05, 0x00, 0x12, 0x80, 0x00, 0x08,
        0x00, 0x00, 0x00, 0x01, 0xf0, 0x22, 0xb0, 0x10, 0x02, 0x00, 0x00, 0x00, 0x00, 0x28, 0x26,
        0x40, 0x00, 0x00, 0x00, 0x17, 0x70, 0x00, 0x00, 0x5a, 0xbe, 0xe0, 0x00, 0x53, 0x0c, 0x36,
        0x00, 0x00, 0x00, 0x00, 0x0f, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x0d, 0x21, 0x37, 0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x0f, 0xff, 0xff, 0xfd, 0xe9, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xd4, 0xe0, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x7f, 0x9f, 0x1f, 0xff,
        0xff, 0xff, 0xf3, 0xf7, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x21, 0x37, 0xe0,
        0x01, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00,
    ];

    const HEAD: [u8; 20] = [
        0x1f, 0x80, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x0a, 0x65, 0x71, 0x88, 0x40,
        0xe4, 0xcc, 0x14, 0x91, 0x99,
    ];

    #[test]
    fn unpack_word_reads_measured_head() {
        assert_eq!(unpack_word(&SUB_A, 0).unwrap(), 0x12);
        assert_eq!(unpack_word(&SUB_A, 36).unwrap(), 0x5a);
        assert_eq!(unpack_word(&SUB_A, 2 * 36).unwrap(), 0x050012800);
        assert_eq!(unpack_word(&SUB_A, 3 * 36).unwrap(), 0x008000000);
        assert!(unpack_word(&SUB_A, SUB_RECORD_BYTES * 8).is_none());
    }

    #[test]
    fn kind_classifies_measured_signatures() {
        let mut doppler = [0u64; WORDS_PER_SUB_RECORD];
        doppler[6] = 0x002826400;
        doppler[7] = 0x1770;
        assert_eq!(kind(&doppler), Some(TrackingKind::Doppler));

        let mut range = [0u64; WORDS_PER_SUB_RECORD];
        range[6] = 0x08087c400;
        range[7] = 0;
        assert_eq!(kind(&range), Some(TrackingKind::Range));

        let mut sync = [0u64; WORDS_PER_SUB_RECORD];
        sync[6] = 0x08007c400;
        sync[7] = 0;
        assert_eq!(kind(&sync), Some(TrackingKind::Sync));

        let padding = [0u64; WORDS_PER_SUB_RECORD];
        assert_eq!(kind(&padding), None);
    }

    #[test]
    fn record_extracts_measured_fields() {
        let words = sub_words(&SUB_A).unwrap();
        let r = record(&words).unwrap();
        assert_eq!(r.kind, TrackingKind::Doppler);
        assert_eq!(r.id_word, 0x050012800);
        assert_eq!(r.time_word, 0x008000000);
        assert_eq!(r.flag_word, 0x01f022b01);
        assert_eq!(r.counter_word, 0x5abee);
        assert_eq!(r.value_word, 0x530c36);
        assert_eq!(r.secondary_word, 0xfffffffff);
        assert_eq!(r.status_a, 0);
        assert_eq!(r.status_b, 0);
    }

    #[test]
    fn parse_frames_measured_record() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&0x0120u16.to_be_bytes());
        buf.extend_from_slice(&SUB_A);
        let recs = parse(&buf).unwrap();
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].kind, TrackingKind::Doppler);
        assert_eq!(recs[0].time_word, 0x008000000);
    }

    #[test]
    fn parse_rejects_truncated_record() {
        assert!(parse(&HEAD).is_none());
        assert!(parse(b"X").is_none());
    }

    #[test]
    fn vsat_bin_roundtrip() {
        let recs = vec![[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]];
        let bytes = write_vsat_bin(&recs);
        let parsed = parse_vsat_bin(&bytes).unwrap();
        assert_eq!(parsed, recs);
        assert!(parse_vsat_bin(b"X").is_none());
    }
}
