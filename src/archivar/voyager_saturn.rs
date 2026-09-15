pub const RECORD_HEADER_BYTES: usize = 2;
pub const RECORD_DATA_BYTES: usize = 8064;
pub const RECORD_BYTES: usize = RECORD_HEADER_BYTES + RECORD_DATA_BYTES;
pub const SUB_RECORD_BYTES: usize = 288;
pub const SUB_RECORDS_PER_RECORD: usize = RECORD_DATA_BYTES / SUB_RECORD_BYTES;
pub const WORDS_PER_SUB_RECORD: usize = 64;
pub const WORD_BITS: usize = 36;

pub const RECORD_TYPE_LOW_RATE: u64 = 90;
pub const RECORD_TYPE_HIGH_RATE: u64 = 91;

pub const NETWORK_DSN: u64 = 2;

pub const VSAT_STRIDE: usize = 19;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackingKind {
    Doppler,
    Range,
    Angle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoyagerSaturnTime {
    pub year: u64,
    pub day_of_year: u64,
    pub hour: u64,
    pub minute: u64,
    pub second: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoyagerSaturnRecord {
    pub kind: TrackingKind,
    pub record_type: u64,
    pub time: VoyagerSaturnTime,
    pub spacecraft_id: u64,
    pub network: u64,
    pub station: u64,
    pub downlink: u64,
    pub ground_mode: u64,
    pub range_type: u64,
    pub angle_type: u64,
    pub sample_time_cs: u64,
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

pub fn record_type(words: &[u64; WORDS_PER_SUB_RECORD]) -> u64 {
    words[1]
}

pub fn is_tracking(words: &[u64; WORDS_PER_SUB_RECORD]) -> bool {
    matches!(
        record_type(words),
        RECORD_TYPE_LOW_RATE | RECORD_TYPE_HIGH_RATE
    )
}

pub fn time_of(words: &[u64; WORDS_PER_SUB_RECORD]) -> VoyagerSaturnTime {
    VoyagerSaturnTime {
        year: (words[2] >> 24) & 0xfff,
        day_of_year: (words[2] >> 8) & 0xffff,
        hour: words[2] & 0xff,
        minute: (words[3] >> 24) & 0xfff,
        second: (words[3] >> 16) & 0xff,
    }
}

pub fn spacecraft_id(words: &[u64; WORDS_PER_SUB_RECORD]) -> u64 {
    ((words[3] & 0xffff) << 12) | ((words[4] >> 24) & 0xfff)
}

pub fn network(words: &[u64; WORDS_PER_SUB_RECORD]) -> u64 {
    (words[4] >> 16) & 0xff
}

pub fn station(words: &[u64; WORDS_PER_SUB_RECORD]) -> u64 {
    (words[4] >> 8) & 0xff
}

pub fn downlink(words: &[u64; WORDS_PER_SUB_RECORD]) -> u64 {
    words[4] & 0xff
}

pub fn ground_mode(words: &[u64; WORDS_PER_SUB_RECORD]) -> u64 {
    (words[5] >> 24) & 0xfff
}

pub fn range_type(words: &[u64; WORDS_PER_SUB_RECORD]) -> u64 {
    (words[5] >> 16) & 0xff
}

pub fn angle_type(words: &[u64; WORDS_PER_SUB_RECORD]) -> u64 {
    (words[5] >> 8) & 0xff
}

pub fn kind(words: &[u64; WORDS_PER_SUB_RECORD]) -> Option<TrackingKind> {
    if !is_tracking(words) {
        return None;
    }
    let mode = ground_mode(words);
    if (1..=4).contains(&mode) {
        Some(TrackingKind::Doppler)
    } else if range_type(words) != 0 {
        Some(TrackingKind::Range)
    } else if angle_type(words) != 0 {
        Some(TrackingKind::Angle)
    } else {
        None
    }
}

pub fn record(words: &[u64; WORDS_PER_SUB_RECORD]) -> Option<VoyagerSaturnRecord> {
    Some(VoyagerSaturnRecord {
        kind: kind(words)?,
        record_type: record_type(words),
        time: time_of(words),
        spacecraft_id: spacecraft_id(words),
        network: network(words),
        station: station(words),
        downlink: downlink(words),
        ground_mode: ground_mode(words),
        range_type: range_type(words),
        angle_type: angle_type(words),
        sample_time_cs: words[7],
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
        TrackingKind::Angle => 2.0,
    }
}

pub fn to_bin_row(r: &VoyagerSaturnRecord) -> [f64; VSAT_STRIDE] {
    [
        kind_code(r.kind),
        r.time.year as f64,
        r.time.day_of_year as f64,
        r.time.hour as f64,
        r.time.minute as f64,
        r.time.second as f64,
        r.spacecraft_id as f64,
        r.network as f64,
        r.station as f64,
        r.downlink as f64,
        r.ground_mode as f64,
        r.range_type as f64,
        r.angle_type as f64,
        r.sample_time_cs as f64,
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
    fn decode_reads_measured_time_and_station() {
        let words = sub_words(&SUB_A).unwrap();
        assert_eq!(record_type(&words), RECORD_TYPE_LOW_RATE);
        let t = time_of(&words);
        assert_eq!(t.year, 80);
        assert_eq!(t.day_of_year, 296);
        assert_eq!(t.hour, 0);
        assert_eq!(t.minute, 8);
        assert_eq!(t.second, 0);
        assert_eq!(spacecraft_id(&words), 31);
        assert_eq!(network(&words), NETWORK_DSN);
        assert_eq!(station(&words), 43);
        assert_eq!(downlink(&words), 1);
        assert_eq!(ground_mode(&words), 2);
        assert_eq!(range_type(&words), 0);
        assert_eq!(words[7], 6000);
    }

    #[test]
    fn kind_classifies_measured_record() {
        let words = sub_words(&SUB_A).unwrap();
        assert_eq!(kind(&words), Some(TrackingKind::Doppler));

        let mut range = [0u64; WORDS_PER_SUB_RECORD];
        range[1] = RECORD_TYPE_LOW_RATE;
        range[5] = 6 << 24 | 2 << 16;
        assert_eq!(kind(&range), Some(TrackingKind::Range));

        let mut angle = [0u64; WORDS_PER_SUB_RECORD];
        angle[1] = RECORD_TYPE_HIGH_RATE;
        angle[5] = 1 << 8;
        assert_eq!(kind(&angle), Some(TrackingKind::Angle));

        let mut untracked = [0u64; WORDS_PER_SUB_RECORD];
        untracked[1] = 30;
        assert_eq!(kind(&untracked), None);

        let padding = [0u64; WORDS_PER_SUB_RECORD];
        assert_eq!(kind(&padding), None);
    }

    #[test]
    fn record_extracts_measured_fields() {
        let words = sub_words(&SUB_A).unwrap();
        let r = record(&words).unwrap();
        assert_eq!(r.kind, TrackingKind::Doppler);
        assert_eq!(r.record_type, 90);
        assert_eq!(r.time.year, 80);
        assert_eq!(r.time.day_of_year, 296);
        assert_eq!(r.time.minute, 8);
        assert_eq!(r.spacecraft_id, 31);
        assert_eq!(r.network, 2);
        assert_eq!(r.station, 43);
        assert_eq!(r.downlink, 1);
        assert_eq!(r.ground_mode, 2);
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
        assert_eq!(recs[0].time.day_of_year, 296);
        assert_eq!(recs[0].station, 43);
    }

    #[test]
    fn parse_rejects_truncated_record() {
        assert!(parse(&HEAD).is_none());
        assert!(parse(b"X").is_none());
    }

    #[test]
    fn vsat_bin_roundtrip() {
        let recs = vec![[0.0; VSAT_STRIDE]];
        let bytes = write_vsat_bin(&recs);
        let parsed = parse_vsat_bin(&bytes).unwrap();
        assert_eq!(parsed, recs);
        assert!(parse_vsat_bin(b"X").is_none());
    }
}
