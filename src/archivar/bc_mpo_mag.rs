pub const MAGIC: [u8; 4] = *b"BCM1";
pub const FIELDS_PER_RECORD: usize = 7;
pub const RECORD_BYTES: usize = FIELDS_PER_RECORD * 8;

pub fn write_bin(records: &[[f64; FIELDS_PER_RECORD]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_bin(data: &[u8]) -> Option<Vec<[f64; FIELDS_PER_RECORD]>> {
    if data.len() < 8 || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * RECORD_BYTES;
        let mut r = [0.0f64; FIELDS_PER_RECORD];
        for k in 0..FIELDS_PER_RECORD {
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

    #[test]
    fn roundtrip_holds_for_measured_records() {
        let records = [
            [1.5396e9, 1.263e8, 7.584e7, 1.445e5, 69.229, -589.718, 1579.287],
            [1.5396e9 + 1.0, 1.263e8, 7.584e7, 1.445e5, 69.310, -589.687, 1579.239],
        ];
        let bin = write_bin(&records);
        let parsed = parse_bin(&bin).unwrap();
        assert_eq!(parsed, records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"BCM1abc").is_none());
        assert!(parse_bin(&[b'B', b'C', b'M', b'1', 0, 0, 0, 0, 0, 0, 0, 0]).is_none());
    }
}
