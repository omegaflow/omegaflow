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
