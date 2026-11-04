pub const COMP_DBDT: u32 = 0;
pub const MAGIC: &[u8; 4] = b"IMDT";
pub const XYZ_MAGIC: &[u8; 4] = b"IMXZ";

pub fn write_xyz_bin(records: &[(f64, f64, f64, f64)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 32);
    out.extend_from_slice(XYZ_MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, x, y, z) in records {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&x.to_le_bytes());
        out.extend_from_slice(&y.to_le_bytes());
        out.extend_from_slice(&z.to_le_bytes());
    }
    out
}

pub fn parse_xyz_bin(data: &[u8]) -> Option<Vec<(f64, f64, f64, f64)>> {
    if data.len() < 8 || &data[0..4] != XYZ_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 32 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 32;
        let t = f64::from_le_bytes(data[base..base + 8].try_into().ok()?);
        let x = f64::from_le_bytes(data[base + 8..base + 16].try_into().ok()?);
        let y = f64::from_le_bytes(data[base + 16..base + 24].try_into().ok()?);
        let z = f64::from_le_bytes(data[base + 24..base + 32].try_into().ok()?);
        if !t.is_finite() || !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return None;
        }
        out.push((t, x, y, z));
    }
    Some(out)
}

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

    #[test]
    fn intermagnet_xyz_bin_roundtrip() {
        let records: Vec<(f64, f64, f64, f64)> =
            vec![(1.7e9, 1.0, 2.0, 3.0), (1.7e9 + 60.0, 4.0, 5.0, 6.0)];
        let bytes = write_xyz_bin(&records);
        assert_eq!(&bytes[0..4], b"IMXZ");
        let parsed = parse_xyz_bin(&bytes).expect("parse void");
        assert_eq!(parsed, records);
        assert!(parse_xyz_bin(b"IMDT12345678").is_none());
    }
}
