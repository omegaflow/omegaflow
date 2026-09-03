pub const MAGIC: [u8; 4] = *b"BSN2";

pub const BAND_LOW: u8 = 0;
pub const BAND_MID: u8 = 1;
pub const BAND_HIGH: u8 = 2;

pub fn write_bin(records: &[(u8, i64, f64, f64)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 25);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (band, d, shift, err) in records {
        buf.push(*band);
        buf.extend_from_slice(&d.to_le_bytes());
        buf.extend_from_slice(&shift.to_le_bytes());
        buf.extend_from_slice(&err.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(u8, i64, f64, f64)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * 25 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let band = *bytes.get(off)?;
        off += 1;
        let d = i64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let shift = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let err = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        out.push((band, d, shift, err));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![
            (BAND_HIGH, 3153, -2.563e-7, 1.401e-7),
            (BAND_MID, 3153, -9.62e-8, 8.40e-8),
            (BAND_LOW, 4087, -1.370e-7, 2.97e-8),
        ];
        let bytes = write_bin(&records);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"BSN2\x02\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"BSN1abcdefgh").is_none());
    }
}
