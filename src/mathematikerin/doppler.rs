pub fn write_bin(records: &[[f64; 6]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 48);
    out.extend_from_slice(b"PDPL");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_bin(data: &[u8]) -> Option<Vec<[f64; 6]>> {
    if data.len() < 8 || &data[0..4] != b"PDPL" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 48 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 48;
        let mut r = [0.0f64; 6];
        for k in 0..6 {
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
    fn test_pnav_roundtrip() {
        let records: Vec<[f64; 9]> = vec![
            [
                -8.28095e8,
                5.012066e5,
                2.1108144e9,
                1.98e3,
                12.0,
                0.0,
                12.0,
                12.0,
                12.0,
            ],
            [0.0, 0.0, 2.1e9, 1.0e3, 13.0, 0.0, 11.0, 43.0, 13.0],
        ];
        let bin = write_pnav_bin(&records);
        assert_eq!(&bin[0..4], b"PNAV");
        let parsed = parse_pnav_bin(&bin).expect("parse void");
        assert_eq!(parsed.len(), records.len());
        for (a, b) in parsed.iter().zip(records.iter()) {
            for k in 0..9 {
                assert_eq!(a[k], b[k]);
            }
        }
        assert!(parse_pnav_bin(&bin[..8]).is_none());
        assert!(parse_pnav_bin(b"XXXX12345678").is_none());
    }
}

pub fn write_pnav_bin(records: &[[f64; 9]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 72);
    out.extend_from_slice(b"PNAV");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_pnav_bin(data: &[u8]) -> Option<Vec<[f64; 9]>> {
    if data.len() < 8 || &data[0..4] != b"PNAV" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 72 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 72;
        let mut r = [0.0f64; 9];
        for k in 0..9 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}
