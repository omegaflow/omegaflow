use super::tiff::decode_jpeg_raster;

pub struct KeogramColumns {
    pub width: usize,
    pub height: usize,
    pub mean: Vec<f64>,
}

pub fn brightness_columns(jpeg: &[u8]) -> Option<KeogramColumns> {
    let (width, height, pixels) = decode_jpeg_raster(jpeg)?;
    let area = width.checked_mul(height)?;
    if area == 0 || pixels.len() % area != 0 {
        return None;
    }
    let components = pixels.len() / area;
    if components == 0 {
        return None;
    }
    let mut mean = vec![0.0; width];
    for y in 0..height {
        for x in 0..width {
            let base = (y * width + x) * components;
            let mut acc = 0.0;
            for c in 0..components {
                acc += pixels[base + c] as f64;
            }
            mean[x] += acc / components as f64;
        }
    }
    for m in mean.iter_mut() {
        *m /= height as f64;
    }
    Some(KeogramColumns {
        width,
        height,
        mean,
    })
}

pub const MAGIC: [u8; 4] = *b"KGRM";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 8 + 4 + 1 + 8;

pub fn write_bin(rows: &[(f64, u32, f64)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + rows.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(rows.len() as u32).to_le_bytes());
    for (t_unix, comp, mean) in rows {
        buf.extend_from_slice(&t_unix.to_le_bytes());
        buf.extend_from_slice(&comp.to_le_bytes());
        buf.push(1u8);
        buf.extend_from_slice(&mean.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, u32, f64)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + n * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let t_unix = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let comp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        let present = *bytes.get(off)?;
        off += 1;
        let mean = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        if !t_unix.is_finite() {
            return None;
        }
        if present & 1 == 0 {
            continue;
        }
        if !mean.is_finite() {
            return None;
        }
        out.push((t_unix, comp, mean));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASELINE_JPEG: &[u8] = &[
        0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0x00, 0x01, 0x02, 0x00, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x00, 0xff, 0xfe, 0x00, 0x10, 0x4c, 0x61, 0x76, 0x63, 0x36, 0x30,
        0x2e, 0x33, 0x31, 0x2e, 0x31, 0x30, 0x32, 0x00, 0xff, 0xdb, 0x00, 0x43, 0x00, 0x08, 0x0a,
        0x0a, 0x0b, 0x0a, 0x0b, 0x0d, 0x0d, 0x0d, 0x0d, 0x0d, 0x0d, 0x10, 0x0f, 0x10, 0x10, 0x10,
        0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x12, 0x12, 0x12, 0x15, 0x15, 0x15, 0x12,
        0x12, 0x12, 0x10, 0x10, 0x12, 0x12, 0x14, 0x14, 0x15, 0x15, 0x17, 0x17, 0x17, 0x15, 0x15,
        0x15, 0x15, 0x17, 0x17, 0x19, 0x19, 0x19, 0x1e, 0x1e, 0x1c, 0x1c, 0x23, 0x23, 0x24, 0x2b,
        0x2b, 0x33, 0xff, 0xc4, 0x00, 0x4a, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x11, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x00, 0x10, 0x00, 0x10, 0x03, 0x01, 0x12,
        0x00, 0x02, 0x12, 0x00, 0x03, 0x12, 0x00, 0xff, 0xda, 0x00, 0x0c, 0x03, 0x01, 0x00, 0x02,
        0x11, 0x03, 0x11, 0x00, 0x3f, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd9,
    ];

    #[test]
    fn brightness_columns_of_baseline_jpeg() {
        let cols = brightness_columns(BASELINE_JPEG).expect("baseline jpeg decodes");
        assert_eq!(cols.width, 16);
        assert_eq!(cols.height, 16);
        assert_eq!(cols.mean.len(), 16);
        assert!(cols.mean.iter().all(|m| m.is_finite()));
        assert!(cols.mean.iter().all(|&m| (m - 128.0).abs() < 1.5));
    }

    #[test]
    fn brightness_columns_rejects_invalid_stream() {
        assert!(brightness_columns(&[0x00, 0x01, 0x02, 0x03]).is_none());
    }

    #[test]
    fn bin_roundtrip_keeps_rows() {
        let rows = vec![
            (86400.0, 0u32, 12.5),
            (86400.0, 1u32, 128.0),
            (172800.0, 0u32, 3.25),
        ];
        let bytes = write_bin(&rows);
        assert_eq!(bytes.len(), HEADER_BYTES + 3 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(rows));
    }

    #[test]
    fn an_empty_bin_roundtrips_to_no_rows() {
        let bytes = write_bin(&[]);
        assert_eq!(parse_bin(&bytes), Some(Vec::new()));
    }

    #[test]
    fn an_absent_value_bit_drops_the_record() {
        let mut bytes = write_bin(&[(86400.0, 0u32, 12.5)]);
        bytes[HEADER_BYTES + 12] = 0;
        assert_eq!(parse_bin(&bytes), Some(Vec::new()));
    }

    #[test]
    fn a_truncated_or_foreign_bin_is_void() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&[(86400.0, 0u32, 12.5)]);
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }
}
