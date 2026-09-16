pub const MAGIC: [u8; 4] = *b"DRSF";
pub const HEADER_BYTES: usize = 16;
pub const REC_BYTES: usize = 24;
pub const CADENCE_HZ: f64 = 1.0;

pub const COMP_GX: u32 = 1;
pub const COMP_GY: u32 = 2;
pub const COMP_GZ: u32 = 3;

fn le_f64(data: &[u8], offset: usize) -> Option<f64> {
    Some(f64::from_le_bytes(
        data.get(offset..offset + 8)?.try_into().ok()?,
    ))
}

pub fn write_bin(records: &[[f64; 3]], epoch: f64) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    out.extend_from_slice(&epoch.to_le_bytes());
    for r in records {
        out.extend_from_slice(&r[0].to_le_bytes());
        out.extend_from_slice(&r[1].to_le_bytes());
        out.extend_from_slice(&r[2].to_le_bytes());
    }
    out
}

pub fn parse_series(data: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if data.len() < HEADER_BYTES || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != HEADER_BYTES + count * REC_BYTES {
        return None;
    }
    let epoch = le_f64(data, 8)?;
    if !epoch.is_finite() || epoch <= 0.0 {
        return None;
    }
    let mut out = Vec::with_capacity(count * 3);
    for i in 0..count {
        let base = HEADER_BYTES + i * REC_BYTES;
        let gx = le_f64(data, base)?;
        let gy = le_f64(data, base + 8)?;
        let gz = le_f64(data, base + 16)?;
        if !gx.is_finite() || !gy.is_finite() || !gz.is_finite() {
            return None;
        }
        let t = epoch + i as f64 / CADENCE_HZ;
        out.push((t, gx, COMP_GX));
        out.push((t, gy, COMP_GY));
        out.push((t, gz, COMP_GZ));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrip_derives_index_time_from_header_epoch() {
        let records = vec![[1.0e-9, -2.0e-9, 3.0e-9], [4.0e-9, -5.0e-9, 6.0e-9]];
        let bytes = write_bin(&records, 1.47e9);
        let series = parse_series(&bytes).unwrap();
        assert_eq!(series.len(), 6);
        assert_eq!(series[0], (1.47e9, 1.0e-9, COMP_GX));
        assert_eq!(series[1], (1.47e9, -2.0e-9, COMP_GY));
        assert_eq!(series[2], (1.47e9, 3.0e-9, COMP_GZ));
        assert_eq!(series[3], (1.47e9 + 1.0, 4.0e-9, COMP_GX));
        assert_eq!(series[5], (1.47e9 + 1.0, 6.0e-9, COMP_GZ));
    }

    #[test]
    fn parse_rejects_truncated_header_and_unknown_magic() {
        assert!(parse_series(b"X").is_none());
        assert!(parse_series(b"DRSF").is_none());
        let bytes = write_bin(&[[1.0, 2.0, 3.0]], 1.47e9);
        assert!(parse_series(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn parse_rejects_absent_epoch() {
        let mut zero = write_bin(&[[1.0, 2.0, 3.0]], 1.47e9);
        zero[8..16].copy_from_slice(&0.0f64.to_le_bytes());
        assert!(parse_series(&zero).is_none());
        let mut nan = write_bin(&[[1.0, 2.0, 3.0]], 1.47e9);
        nan[8..16].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_series(&nan).is_none());
    }

    #[test]
    fn parse_rejects_non_finite_components() {
        let records = [[1.0, f64::NAN, 3.0]];
        let bytes = write_bin(&records, 1.47e9);
        assert!(parse_series(&bytes).is_none());
    }
}
