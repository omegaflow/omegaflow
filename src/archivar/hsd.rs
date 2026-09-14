use crate::archivar::bzip2;

pub struct HsdFile {
    pub blocks: Vec<(u8, usize)>,
    pub columns: u16,
    pub lines: u16,
    pub bits_per_pixel: Option<u16>,
    pub pixel_values: Vec<u16>,
    pub calibration: Vec<CalibrationBand>,
}

pub struct CalibrationBand {
    pub band: u16,
    pub central_wavelength_um: Option<f64>,
    pub valid_bits: Option<u16>,
    pub error_pixels: Option<u16>,
    pub outside_scan_pixels: Option<u16>,
    pub gain: Option<f64>,
    pub offset: Option<f64>,
}

fn le_u16(b: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(off..off + 2)?.try_into().ok()?))
}

fn le_u32(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

fn le_f64(b: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(b.get(off..off + 8)?.try_into().ok()?))
}

fn parse_calibration(content: &[u8]) -> Option<CalibrationBand> {
    let band = le_u16(content, 0)?;
    Some(CalibrationBand {
        band,
        central_wavelength_um: le_f64(content, 2),
        valid_bits: le_u16(content, 10),
        error_pixels: le_u16(content, 12),
        outside_scan_pixels: le_u16(content, 14),
        gain: le_f64(content, 16),
        offset: le_f64(content, 24),
    })
}

pub fn parse_hsd(data: &[u8]) -> Option<HsdFile> {
    let decompressed;
    let bytes: &[u8] = if data.starts_with(b"BZh") {
        decompressed = bzip2::decompress(data)?;
        &decompressed
    } else {
        data
    };

    let mut blocks = Vec::new();
    let mut columns = None;
    let mut lines = None;
    let mut bits_per_pixel = None;
    let mut total_header_length = None;
    let mut calibration = Vec::new();

    let mut offset = 0usize;
    loop {
        if offset + 3 > bytes.len() {
            break;
        }
        let block_type = bytes[offset];
        let block_len = le_u16(bytes, offset + 1)? as usize;
        if block_len < 3 || offset + block_len > bytes.len() {
            break;
        }
        let content = &bytes[offset + 3..offset + block_len];
        match block_type {
            1 => {
                if content.len() >= 71 {
                    total_header_length = Some(le_u32(content, 67)? as usize);
                }
            }
            2 => {
                if content.len() >= 6 {
                    bits_per_pixel = Some(le_u16(content, 0)?);
                    columns = Some(le_u16(content, 2)?);
                    lines = Some(le_u16(content, 4)?);
                }
            }
            5 => {
                if let Some(cal) = parse_calibration(content) {
                    calibration.push(cal);
                }
            }
            _ => {}
        }
        blocks.push((block_type, block_len));
        offset += block_len;
        if let Some(h) = total_header_length {
            if offset >= h {
                break;
            }
        } else if block_type >= 11 {
            break;
        }
    }

    let columns = columns?;
    let lines = lines?;
    let data_offset = match total_header_length {
        Some(h) => h,
        None => offset,
    };
    let count = columns as usize * lines as usize;
    if data_offset + count * 2 > bytes.len() {
        return None;
    }
    let mut pixel_values = Vec::with_capacity(count);
    for i in 0..count {
        pixel_values.push(le_u16(bytes, data_offset + i * 2)?);
    }

    Some(HsdFile {
        blocks,
        columns,
        lines,
        bits_per_pixel,
        pixel_values,
        calibration,
    })
}

pub const MAGIC_AHI: [u8; 4] = *b"AHI1";
const AHI_HEADER_BYTES: usize = 24;

pub struct AhiSegment {
    pub columns: u16,
    pub lines: u16,
    pub bits_per_pixel: u16,
    pub band: u8,
    pub segment: u8,
    pub satellite: u8,
    pub resolution_m: u16,
    pub obs_sec: f64,
    pub obs_present: u8,
    pub counts: Vec<u16>,
}

pub fn write_segment(seg: &AhiSegment) -> Vec<u8> {
    let mut buf = Vec::with_capacity(AHI_HEADER_BYTES + seg.counts.len() * 2);
    buf.extend_from_slice(&MAGIC_AHI);
    buf.extend_from_slice(&seg.columns.to_le_bytes());
    buf.extend_from_slice(&seg.lines.to_le_bytes());
    buf.extend_from_slice(&seg.bits_per_pixel.to_le_bytes());
    buf.push(seg.band);
    buf.push(seg.segment);
    buf.push(seg.satellite);
    buf.extend_from_slice(&seg.resolution_m.to_le_bytes());
    buf.push(seg.obs_present);
    buf.extend_from_slice(&seg.obs_sec.to_le_bytes());
    for &c in &seg.counts {
        buf.extend_from_slice(&c.to_le_bytes());
    }
    buf
}

pub fn parse_segment(bytes: &[u8]) -> Option<AhiSegment> {
    if bytes.len() < AHI_HEADER_BYTES || bytes[0..4] != MAGIC_AHI {
        return None;
    }
    let columns = u16::from_le_bytes(bytes.get(4..6)?.try_into().ok()?);
    let lines = u16::from_le_bytes(bytes.get(6..8)?.try_into().ok()?);
    let bits_per_pixel = u16::from_le_bytes(bytes.get(8..10)?.try_into().ok()?);
    let band = *bytes.get(10)?;
    let segment = *bytes.get(11)?;
    let satellite = *bytes.get(12)?;
    let resolution_m = u16::from_le_bytes(bytes.get(13..15)?.try_into().ok()?);
    let obs_present = *bytes.get(15)?;
    let obs_sec = f64::from_le_bytes(bytes.get(16..24)?.try_into().ok()?);
    if columns == 0 || lines == 0 {
        return None;
    }
    if obs_present > 1 {
        return None;
    }
    if obs_present == 1 && !obs_sec.is_finite() {
        return None;
    }
    let count = columns as usize * lines as usize;
    if bytes.len() != AHI_HEADER_BYTES + count * 2 {
        return None;
    }
    let mut counts = Vec::with_capacity(count);
    for i in 0..count {
        let off = AHI_HEADER_BYTES + i * 2;
        counts.push(u16::from_le_bytes(
            bytes.get(off..off + 2)?.try_into().ok()?,
        ));
    }
    Some(AhiSegment {
        columns,
        lines,
        bits_per_pixel,
        band,
        segment,
        satellite,
        resolution_m,
        obs_sec,
        obs_present,
        counts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_fixture(columns: u16, lines: u16, pixels: &[u16]) -> Vec<u8> {
        let mut out = Vec::new();

        out.push(1u8);
        out.extend_from_slice(&282u16.to_le_bytes());
        let mut b1 = vec![0u8; 279];
        b1[0..2].copy_from_slice(&11u16.to_le_bytes());
        let header_len = (282 + 50 + 259) as u32;
        b1[67..71].copy_from_slice(&header_len.to_le_bytes());
        out.extend_from_slice(&b1);

        out.push(2u8);
        out.extend_from_slice(&50u16.to_le_bytes());
        let mut b2 = vec![0u8; 47];
        b2[0..2].copy_from_slice(&16u16.to_le_bytes());
        b2[2..4].copy_from_slice(&columns.to_le_bytes());
        b2[4..6].copy_from_slice(&lines.to_le_bytes());
        out.extend_from_slice(&b2);

        out.push(11u8);
        out.extend_from_slice(&259u16.to_le_bytes());
        out.extend_from_slice(&vec![0u8; 256]);

        for &p in pixels {
            out.extend_from_slice(&p.to_le_bytes());
        }
        out
    }

    fn calibration_content(band: u16, gain: f64, offset: f64) -> Vec<u8> {
        let mut c = vec![0u8; 32];
        c[0..2].copy_from_slice(&band.to_le_bytes());
        c[2..10].copy_from_slice(&0.47063f64.to_le_bytes());
        c[10..12].copy_from_slice(&11u16.to_le_bytes());
        c[12..14].copy_from_slice(&65535u16.to_le_bytes());
        c[14..16].copy_from_slice(&65534u16.to_le_bytes());
        c[16..24].copy_from_slice(&gain.to_le_bytes());
        c[24..32].copy_from_slice(&offset.to_le_bytes());
        c
    }

    fn build_fixture_with_calibration(
        columns: u16,
        lines: u16,
        pixels: &[u16],
        cal: &[u8],
    ) -> Vec<u8> {
        let mut out = Vec::new();

        out.push(1u8);
        out.extend_from_slice(&282u16.to_le_bytes());
        let mut b1 = vec![0u8; 279];
        b1[0..2].copy_from_slice(&11u16.to_le_bytes());
        let cal_block_len = 3 + cal.len();
        let header_len = (282 + 50 + cal_block_len + 259) as u32;
        b1[67..71].copy_from_slice(&header_len.to_le_bytes());
        out.extend_from_slice(&b1);

        out.push(2u8);
        out.extend_from_slice(&50u16.to_le_bytes());
        let mut b2 = vec![0u8; 47];
        b2[0..2].copy_from_slice(&16u16.to_le_bytes());
        b2[2..4].copy_from_slice(&columns.to_le_bytes());
        b2[4..6].copy_from_slice(&lines.to_le_bytes());
        out.extend_from_slice(&b2);

        out.push(5u8);
        out.extend_from_slice(&(cal_block_len as u16).to_le_bytes());
        out.extend_from_slice(cal);

        out.push(11u8);
        out.extend_from_slice(&259u16.to_le_bytes());
        out.extend_from_slice(&vec![0u8; 256]);

        for &p in pixels {
            out.extend_from_slice(&p.to_le_bytes());
        }
        out
    }

    #[test]
    fn decodes_calibration_block_5() {
        let pixels = [1u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let gain = 0.3773583529411764;
        let offset = -7.547167058823528;
        let cal = calibration_content(1, gain, offset);
        let data = build_fixture_with_calibration(4, 3, &pixels, &cal);
        let hsd = parse_hsd(&data).unwrap();
        assert_eq!(hsd.calibration.len(), 1);
        let c = &hsd.calibration[0];
        assert_eq!(c.band, 1);
        assert_eq!(c.valid_bits, Some(11));
        assert_eq!(c.error_pixels, Some(65535));
        assert_eq!(c.outside_scan_pixels, Some(65534));
        assert!((c.central_wavelength_um.unwrap() - 0.47063).abs() < 1e-9);
        assert!((c.gain.unwrap() - gain).abs() < 1e-12);
        assert!((c.offset.unwrap() - offset).abs() < 1e-12);
    }

    #[test]
    fn calibration_absent_without_block_5() {
        let pixels = [1u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let data = build_fixture(4, 3, &pixels);
        let hsd = parse_hsd(&data).unwrap();
        assert!(hsd.calibration.is_empty());
    }

    #[test]
    fn parses_structure() {
        let pixels = [1u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let data = build_fixture(4, 3, &pixels);
        let hsd = parse_hsd(&data).unwrap();
        assert_eq!(hsd.columns, 4);
        assert_eq!(hsd.lines, 3);
        assert_eq!(hsd.bits_per_pixel, Some(16));
        assert_eq!(hsd.pixel_values, pixels);
        assert_eq!(hsd.blocks, vec![(1, 282), (2, 50), (11, 259)]);
    }

    fn from_hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn decompresses_bzip2_input() {
        let data = from_hex(
            "425a6839314159265359d0eea83b000011fc04fffc4010010010010000800100082000\
             50a069a19193104aa7ea9e5030d4c9b51aaeee04ddbae23b6f295b5e233e85161694a394\
             061691baca947e2ee48a70a121a1dd5076",
        );
        let hsd = parse_hsd(&data).unwrap();
        assert_eq!(hsd.columns, 4);
        assert_eq!(hsd.lines, 3);
        assert_eq!(
            hsd.pixel_values,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
        );
        assert_eq!(hsd.blocks, vec![(1, 282), (2, 50), (11, 259)]);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_hsd(b"").is_none());
        assert!(parse_hsd(b"PK\x03\x04").is_none());
        assert!(parse_hsd(b"BZh9").is_none());
    }

    #[test]
    fn rejects_truncated_pixels() {
        let data = build_fixture(4, 3, &[1, 2, 3, 4, 5, 6]);
        assert!(parse_hsd(&data).is_none());
    }

    fn segment_fixture() -> AhiSegment {
        AhiSegment {
            columns: 4,
            lines: 3,
            bits_per_pixel: 16,
            band: 1,
            segment: 1,
            satellite: 8,
            resolution_m: 1000,
            obs_sec: 1436234400.0,
            obs_present: 1,
            counts: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        }
    }

    #[test]
    fn segment_roundtrip() {
        let seg = segment_fixture();
        let bytes = write_segment(&seg);
        let parsed = parse_segment(&bytes).unwrap();
        assert_eq!(parsed.columns, 4);
        assert_eq!(parsed.lines, 3);
        assert_eq!(parsed.bits_per_pixel, 16);
        assert_eq!(parsed.band, 1);
        assert_eq!(parsed.segment, 1);
        assert_eq!(parsed.satellite, 8);
        assert_eq!(parsed.resolution_m, 1000);
        assert_eq!(parsed.obs_present, 1);
        assert_eq!(parsed.obs_sec, 1436234400.0);
        assert_eq!(parsed.counts, seg.counts);
    }

    #[test]
    fn segment_absent_observation_roundtrip() {
        let mut seg = segment_fixture();
        seg.obs_present = 0;
        seg.obs_sec = 0.0;
        let bytes = write_segment(&seg);
        let parsed = parse_segment(&bytes).unwrap();
        assert_eq!(parsed.obs_present, 0);
        assert_eq!(parsed.counts, seg.counts);
    }

    #[test]
    fn segment_rejects_foreign_and_malformed() {
        assert!(parse_segment(b"").is_none());
        assert!(parse_segment(b"AHI1").is_none());
        assert!(parse_segment(b"AHI2").is_none());
        assert!(parse_segment(b"GXS1abcd").is_none());

        let bytes = write_segment(&segment_fixture());
        let short = bytes[..bytes.len() - 1].to_vec();
        assert!(parse_segment(&short).is_none());

        let mut bad_obs = bytes.clone();
        bad_obs[15] = 2;
        assert!(parse_segment(&bad_obs).is_none());

        let mut nan_obs = bytes.clone();
        nan_obs[15] = 1;
        nan_obs[16..24].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_segment(&nan_obs).is_none());

        let mut zero_cols = bytes.clone();
        zero_cols[4..6].copy_from_slice(&0u16.to_le_bytes());
        assert!(parse_segment(&zero_cols).is_none());
    }
}
