use crate::archivar::bzip2;

pub struct HsdFile {
    pub blocks: Vec<(u8, usize)>,
    pub columns: u16,
    pub lines: u16,
    pub bits_per_pixel: Option<u16>,
    pub pixel_values: Vec<u16>,
}

fn le_u16(b: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(off..off + 2)?.try_into().ok()?))
}

fn le_u32(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(off..off + 4)?.try_into().ok()?))
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
}
