pub const MAGIC: &[u8; 4] = b"AIA1";

pub fn parse_bin(data: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if data.len() < 8 || &data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 20 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 20;
        let t = f64::from_le_bytes(data[base..base + 8].try_into().ok()?);
        let v = f64::from_le_bytes(data[base + 8..base + 16].try_into().ok()?);
        let idx = u32::from_le_bytes(data[base + 16..base + 20].try_into().ok()?);
        out.push((t, v, idx));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aia_lines_bin_roundtrip() {
        let records: Vec<(f64, f64, u32)> = vec![
            (1.7e9, 1.2e6, 0),
            (1.7e9 + 24.0, 3.4e5, 5),
            (1.7e9 + 48.0, 9.8e5, 6),
        ];
        let mut bytes = Vec::with_capacity(8 + records.len() * 20);
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&(records.len() as u32).to_le_bytes());
        for (t, v, idx) in &records {
            bytes.extend_from_slice(&t.to_le_bytes());
            bytes.extend_from_slice(&v.to_le_bytes());
            bytes.extend_from_slice(&idx.to_le_bytes());
        }
        assert_eq!(&bytes[0..4], b"AIA1");
        let parsed = parse_bin(&bytes).expect("parse void");
        assert_eq!(parsed, records);
        assert!(parse_bin(b"XXXX12345678").is_none());
    }
}
