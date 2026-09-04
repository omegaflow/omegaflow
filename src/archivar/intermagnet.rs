pub const COMP_DBDT: u32 = 0;
pub const MAGIC: &[u8; 4] = b"IMDT";

pub fn parse_bin(data: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if data.len() < 8 || &data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 16 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 16;
        let t = f64::from_le_bytes(data[base..base + 8].try_into().ok()?);
        let v = f64::from_le_bytes(data[base + 8..base + 16].try_into().ok()?);
        out.push((t, v, COMP_DBDT));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intermagnet_bin_roundtrip() {
        let records: Vec<(f64, f64, u32)> = vec![
            (1.7e9, 0.8366600265337494, COMP_DBDT),
            (1.7e9 + 3600.0, 1.24, COMP_DBDT),
        ];
        let mut bytes = Vec::with_capacity(8 + records.len() * 16);
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&(records.len() as u32).to_le_bytes());
        for (t, v, _) in &records {
            bytes.extend_from_slice(&t.to_le_bytes());
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        assert_eq!(&bytes[0..4], b"IMDT");
        let parsed = parse_bin(&bytes).expect("parse void");
        assert_eq!(parsed.len(), records.len());
        assert_eq!(parsed[0].0, records[0].0);
        assert_eq!(parsed[0].1, records[0].1);
        assert_eq!(parsed[0].2, COMP_DBDT);
        assert!(parse_bin(b"XXXX12345678").is_none());
    }
}
