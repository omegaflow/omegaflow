pub const MAGIC: [u8; 4] = *b"WAV1";

pub const RECEIVER_RAD1: u32 = 1;
pub const RECEIVER_RAD2: u32 = 2;
pub const RECEIVER_TNR: u32 = 3;

pub const RECORD_BYTES: usize = 36;

pub fn receiver_name(r: u32) -> &'static str {
    match r {
        RECEIVER_RAD1 => "RAD1",
        RECEIVER_RAD2 => "RAD2",
        RECEIVER_TNR => "TNR",
        _ => "unknown",
    }
}

pub fn write_bin(records: &[(f64, f64, f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for &(t, freq, binw, val, recv) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&freq.to_le_bytes());
        buf.extend_from_slice(&binw.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&recv.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, f64, f64, u32)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let freq = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let binw = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let val = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let recv = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if !matches!(recv, RECEIVER_RAD1 | RECEIVER_RAD2 | RECEIVER_TNR) {
            return None;
        }
        out.push((t, freq, binw, val, recv));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![
            (10.0, 1075.0e3, 3.0e3, 1.2e-4, RECEIVER_RAD2),
            (10.0, 1078.0e3, 3.0e3, -0.5e-4, RECEIVER_RAD2),
            (20.0, 40.0e3, 2.0e3, 2.0e-5, RECEIVER_TNR),
        ];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"WAV1abc").is_none());
    }

    #[test]
    fn rejects_unknown_receiver() {
        let bytes = write_bin(&[(10.0, 1.0e3, 1.0e3, 1.0, 7)]);
        assert!(parse_bin(&bytes).is_none());
    }
}
