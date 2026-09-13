#[derive(Debug, Clone)]
pub struct GeoTransform {
    pub x0: f64,
    pub y0: f64,
    pub dx: f64,
    pub dy: f64,
}

#[derive(Debug, Clone)]
pub struct TiffImage {
    pub width: u32,
    pub height: u32,
    pub bits_per_sample: Vec<u16>,
    pub samples_per_pixel: u16,
    pub compression: u16,
    pub photometric: Option<u16>,
    pub pixels: Vec<u8>,
    pub geo: Option<GeoTransform>,
    pub geo_keys: Option<Vec<u16>>,
}

#[derive(Clone, Copy)]
struct Entry {
    tag: u16,
    field_type: u16,
    count: u32,
    value_pos: usize,
}

fn u16_at(b: &[u8], off: usize, little: bool) -> Option<u16> {
    let v: [u8; 2] = b.get(off..off + 2)?.try_into().ok()?;
    Some(if little {
        u16::from_le_bytes(v)
    } else {
        u16::from_be_bytes(v)
    })
}

fn u32_at(b: &[u8], off: usize, little: bool) -> Option<u32> {
    let v: [u8; 4] = b.get(off..off + 4)?.try_into().ok()?;
    Some(if little {
        u32::from_le_bytes(v)
    } else {
        u32::from_be_bytes(v)
    })
}

fn f64_at(b: &[u8], off: usize, little: bool) -> Option<f64> {
    let v: [u8; 8] = b.get(off..off + 8)?.try_into().ok()?;
    Some(if little {
        f64::from_le_bytes(v)
    } else {
        f64::from_be_bytes(v)
    })
}

fn type_size(t: u16) -> Option<usize> {
    Some(match t {
        1 | 2 | 6 | 7 => 1,
        3 | 8 => 2,
        4 | 9 | 11 => 4,
        5 | 10 | 12 => 8,
        _ => return None,
    })
}

fn entry_base(data: &[u8], e: &Entry, little: bool, size: usize) -> Option<usize> {
    let byte_len = (e.count as usize).checked_mul(size)?;
    if byte_len <= 4 {
        Some(e.value_pos)
    } else {
        u32_at(data, e.value_pos, little).map(|v| v as usize)
    }
}

fn entry_scalar(data: &[u8], e: &Entry, little: bool) -> Option<u32> {
    let base = entry_base(data, e, little, type_size(e.field_type)?)?;
    match e.field_type {
        3 => u16_at(data, base, little).map(u32::from),
        4 => u32_at(data, base, little),
        _ => None,
    }
}

fn entry_u16_vec(data: &[u8], e: &Entry, little: bool) -> Option<Vec<u16>> {
    let base = entry_base(data, e, little, 2)?;
    let mut out = Vec::with_capacity(e.count as usize);
    for i in 0..e.count as usize {
        out.push(u16_at(data, base + i * 2, little)?);
    }
    Some(out)
}

fn entry_u32_vec(data: &[u8], e: &Entry, little: bool) -> Option<Vec<u32>> {
    let base = entry_base(data, e, little, 4)?;
    let mut out = Vec::with_capacity(e.count as usize);
    for i in 0..e.count as usize {
        out.push(u32_at(data, base + i * 4, little)?);
    }
    Some(out)
}

fn entry_offsets(data: &[u8], e: &Entry, little: bool) -> Option<Vec<u32>> {
    match e.field_type {
        3 => entry_u16_vec(data, e, little).map(|v| v.into_iter().map(u32::from).collect()),
        4 => entry_u32_vec(data, e, little),
        _ => None,
    }
}

fn entry_double_vec(data: &[u8], e: &Entry, little: bool) -> Option<Vec<f64>> {
    let base = entry_base(data, e, little, 8)?;
    let mut out = Vec::with_capacity(e.count as usize);
    for i in 0..e.count as usize {
        out.push(f64_at(data, base + i * 8, little)?);
    }
    Some(out)
}

fn geotransform(
    pixel_scale: Option<Vec<f64>>,
    tiepoint: Option<Vec<f64>>,
    transformation: Option<Vec<f64>>,
) -> Option<GeoTransform> {
    if let Some(m) = transformation {
        if m.len() >= 8 {
            return Some(GeoTransform {
                x0: m[3],
                y0: m[7],
                dx: m[0],
                dy: m[5],
            });
        }
    }
    if let (Some(scale), Some(tie)) = (pixel_scale, tiepoint) {
        if scale.len() >= 2 && tie.len() >= 6 {
            let sx = scale[0];
            let sy = scale[1];
            let i0 = tie[0];
            let j0 = tie[1];
            return Some(GeoTransform {
                x0: tie[3] - i0 * sx,
                y0: tie[4] + j0 * sy,
                dx: sx,
                dy: -sy,
            });
        }
    }
    None
}

fn decode_chunky(
    data: &[u8],
    width: u32,
    height: u32,
    bits_per_sample: &[u16],
    rows_per_strip: u32,
    strip_offsets: &[u32],
    strip_byte_counts: &[u32],
) -> Option<Vec<u8>> {
    let mut bytes_per_pixel = 0usize;
    for &b in bits_per_sample {
        if b % 8 != 0 {
            return None;
        }
        bytes_per_pixel += (b / 8) as usize;
    }
    if bytes_per_pixel == 0 {
        return None;
    }
    let row_bytes = (width as usize).checked_mul(bytes_per_pixel)?;
    let total = row_bytes.checked_mul(height as usize)?;
    let mut out = Vec::with_capacity(total);
    let mut row = 0usize;
    for (i, &off) in strip_offsets.iter().enumerate() {
        if row >= height as usize {
            break;
        }
        let count = *strip_byte_counts.get(i)? as usize;
        let start = off as usize;
        let end = start.checked_add(count)?;
        let strip = data.get(start..end)?;
        let remaining = height as usize - row;
        let rows = (rows_per_strip as usize).min(remaining);
        let need = rows.checked_mul(row_bytes)?;
        if strip.len() < need {
            return None;
        }
        out.extend_from_slice(&strip[..need]);
        row += rows;
    }
    if out.len() != total {
        return None;
    }
    Some(out)
}

pub fn parse_tiff(data: &[u8]) -> Option<TiffImage> {
    let header = data.get(0..8)?;
    let little = match &header[0..2] {
        b"II" => true,
        b"MM" => false,
        _ => return None,
    };
    if u16_at(header, 2, little)? != 42 {
        return None;
    }
    let ifd_offset = u32_at(header, 4, little)? as usize;
    let entry_count = u16_at(data, ifd_offset, little)? as usize;
    let ifd_base = ifd_offset + 2;
    if ifd_base + entry_count * 12 + 4 > data.len() {
        return None;
    }

    let mut entries = Vec::with_capacity(entry_count);
    for i in 0..entry_count {
        let pos = ifd_base + i * 12;
        entries.push(Entry {
            tag: u16_at(data, pos, little)?,
            field_type: u16_at(data, pos + 2, little)?,
            count: u32_at(data, pos + 4, little)?,
            value_pos: pos + 8,
        });
    }

    let mut width = None;
    let mut height = None;
    let mut bits_per_sample = None;
    let mut compression = None;
    let mut photometric = None;
    let mut strip_offsets = None;
    let mut samples_per_pixel = None;
    let mut rows_per_strip = None;
    let mut strip_byte_counts = None;
    let mut planar_configuration = None;
    let mut model_pixel_scale = None;
    let mut model_tiepoint = None;
    let mut model_transformation = None;
    let mut geo_keys = None;

    for e in &entries {
        match e.tag {
            256 => width = entry_scalar(data, e, little),
            257 => height = entry_scalar(data, e, little),
            258 => bits_per_sample = entry_u16_vec(data, e, little),
            259 => compression = entry_scalar(data, e, little).map(|v| v as u16),
            262 => photometric = entry_scalar(data, e, little).map(|v| v as u16),
            273 => strip_offsets = entry_offsets(data, e, little),
            277 => samples_per_pixel = entry_scalar(data, e, little).map(|v| v as u16),
            278 => rows_per_strip = entry_scalar(data, e, little),
            279 => strip_byte_counts = entry_offsets(data, e, little),
            284 => planar_configuration = entry_scalar(data, e, little).map(|v| v as u16),
            33550 => model_pixel_scale = entry_double_vec(data, e, little),
            33922 => model_tiepoint = entry_double_vec(data, e, little),
            34264 => model_transformation = entry_double_vec(data, e, little),
            34735 => geo_keys = entry_u16_vec(data, e, little),
            _ => {}
        }
    }

    let width = width?;
    let height = height?;
    let bits_per_sample = match bits_per_sample {
        Some(v) => v,
        None => Vec::new(),
    };
    let samples_per_pixel = samples_per_pixel.unwrap_or(1);
    let compression = compression.unwrap_or(1);
    let rows_per_strip = rows_per_strip.unwrap_or(u32::MAX);
    let planar_configuration = planar_configuration.unwrap_or(1);

    let geo = geotransform(model_pixel_scale, model_tiepoint, model_transformation);

    let pixels = if compression == 1 && planar_configuration == 1 {
        let offsets: &[u32] = match &strip_offsets {
            Some(v) => v,
            None => &[],
        };
        let counts: &[u32] = match &strip_byte_counts {
            Some(v) => v,
            None => &[],
        };
        match decode_chunky(
            data,
            width,
            height,
            &bits_per_sample,
            rows_per_strip,
            offsets,
            counts,
        ) {
            Some(v) => v,
            None => Vec::new(),
        }
    } else {
        Vec::new()
    };

    Some(TiffImage {
        width,
        height,
        bits_per_sample,
        samples_per_pixel,
        compression,
        photometric,
        pixels,
        geo,
        geo_keys,
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    fn ifd_entry(out: &mut Vec<u8>, tag: u16, field_type: u16, count: u32, value: u32) {
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&field_type.to_le_bytes());
        out.extend_from_slice(&count.to_le_bytes());
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn build_tiff(
        width: u32,
        height: u32,
        compression: u16,
        pixels: &[u8],
        scale: Option<[f64; 3]>,
        tiepoint: Option<[f64; 6]>,
    ) -> Vec<u8> {
        let geo = scale.is_some() && tiepoint.is_some();
        let entry_count = if geo { 11u16 } else { 9u16 };
        let pixel_offset = (8 + 2 + entry_count as usize * 12 + 4) as u32;
        let scale_offset = pixel_offset + pixels.len() as u32;
        let tiepoint_offset = scale_offset + 24;

        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        out.extend_from_slice(&42u16.to_le_bytes());
        out.extend_from_slice(&8u32.to_le_bytes());
        out.extend_from_slice(&entry_count.to_le_bytes());

        ifd_entry(&mut out, 256, 4, 1, width);
        ifd_entry(&mut out, 257, 4, 1, height);
        ifd_entry(&mut out, 258, 3, 1, 8);
        ifd_entry(&mut out, 259, 3, 1, compression as u32);
        ifd_entry(&mut out, 262, 3, 1, 1);
        ifd_entry(&mut out, 273, 4, 1, pixel_offset);
        ifd_entry(&mut out, 277, 3, 1, 1);
        ifd_entry(&mut out, 278, 4, 1, height);
        ifd_entry(&mut out, 279, 4, 1, pixels.len() as u32);
        if geo {
            ifd_entry(&mut out, 33550, 12, 3, scale_offset);
            ifd_entry(&mut out, 33922, 12, 6, tiepoint_offset);
        }
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(pixels);
        if geo {
            for v in scale.unwrap() {
                out.extend_from_slice(&v.to_le_bytes());
            }
            for v in tiepoint.unwrap() {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        out
    }

    #[test]
    fn parses_uncompressed_strip() {
        let data = build_tiff(2, 2, 1, &[0x10, 0x20, 0x30, 0x40], None, None);
        let img = parse_tiff(&data).unwrap();
        assert_eq!(img.width, 2);
        assert_eq!(img.height, 2);
        assert_eq!(img.bits_per_sample, vec![8]);
        assert_eq!(img.samples_per_pixel, 1);
        assert_eq!(img.compression, 1);
        assert_eq!(img.photometric, Some(1));
        assert_eq!(img.pixels, vec![0x10, 0x20, 0x30, 0x40]);
        assert!(img.geo.is_none());
    }

    #[test]
    fn reads_pixel_scale_and_tiepoint() {
        let data = build_tiff(
            2,
            2,
            1,
            &[0x10, 0x20, 0x30, 0x40],
            Some([0.5, 0.5, 0.0]),
            Some([0.0, 0.0, 0.0, 10.0, 20.0, 0.0]),
        );
        let img = parse_tiff(&data).unwrap();
        let geo = img.geo.unwrap();
        assert_eq!(geo.dx, 0.5);
        assert_eq!(geo.dy, -0.5);
        assert_eq!(geo.x0, 10.0);
        assert_eq!(geo.y0, 20.0);
    }

    #[test]
    fn jpeg_compression_yields_no_pixels() {
        let data = build_tiff(2, 2, 7, &[], None, None);
        let img = parse_tiff(&data).unwrap();
        assert_eq!(img.compression, 7);
        assert!(img.pixels.is_empty());
    }

    #[test]
    fn reads_big_endian() {
        let mut out = Vec::new();
        out.extend_from_slice(b"MM");
        out.extend_from_slice(&42u16.to_be_bytes());
        out.extend_from_slice(&8u32.to_be_bytes());
        out.extend_from_slice(&9u16.to_be_bytes());
        let be_entry = |e: &mut Vec<u8>, tag: u16, t: u16, count: u32, value: u32| {
            e.extend_from_slice(&tag.to_be_bytes());
            e.extend_from_slice(&t.to_be_bytes());
            e.extend_from_slice(&count.to_be_bytes());
            if t == 3 {
                e.extend_from_slice(&(value as u16).to_be_bytes());
                e.extend_from_slice(&[0, 0]);
            } else {
                e.extend_from_slice(&value.to_be_bytes());
            }
        };
        be_entry(&mut out, 256, 4, 1, 1);
        be_entry(&mut out, 257, 4, 1, 1);
        be_entry(&mut out, 258, 3, 1, 8);
        be_entry(&mut out, 259, 3, 1, 1);
        be_entry(&mut out, 262, 3, 1, 1);
        be_entry(&mut out, 273, 4, 1, 122);
        be_entry(&mut out, 277, 3, 1, 1);
        be_entry(&mut out, 278, 4, 1, 1);
        be_entry(&mut out, 279, 4, 1, 1);
        out.extend_from_slice(&0u32.to_be_bytes());
        out.push(0xAB);

        let img = parse_tiff(&out).unwrap();
        assert_eq!(img.width, 1);
        assert_eq!(img.height, 1);
        assert_eq!(img.pixels, vec![0xAB]);
    }

    #[test]
    fn reads_geo_key_directory() {
        let entry_count = 10u16;
        let pixel_offset = (8 + 2 + entry_count as usize * 12 + 4) as u32;
        let keys_offset = pixel_offset + 4;
        let keys: [u16; 16] = [1, 1, 0, 3, 1024, 0, 1, 2, 2048, 0, 1, 4326, 1025, 0, 1, 2];

        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        out.extend_from_slice(&42u16.to_le_bytes());
        out.extend_from_slice(&8u32.to_le_bytes());
        out.extend_from_slice(&entry_count.to_le_bytes());
        ifd_entry(&mut out, 256, 4, 1, 2);
        ifd_entry(&mut out, 257, 4, 1, 2);
        ifd_entry(&mut out, 258, 3, 1, 8);
        ifd_entry(&mut out, 259, 3, 1, 1);
        ifd_entry(&mut out, 262, 3, 1, 1);
        ifd_entry(&mut out, 273, 4, 1, pixel_offset);
        ifd_entry(&mut out, 277, 3, 1, 1);
        ifd_entry(&mut out, 278, 4, 1, 2);
        ifd_entry(&mut out, 279, 4, 1, 4);
        ifd_entry(&mut out, 34735, 3, 16, keys_offset);
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&[0x10, 0x20, 0x30, 0x40]);
        for &k in &keys {
            out.extend_from_slice(&k.to_le_bytes());
        }

        let img = parse_tiff(&out).unwrap();
        assert_eq!(img.geo_keys, Some(keys.to_vec()));
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_tiff(b"").is_none());
        assert!(parse_tiff(b"PK\x03\x04").is_none());
        assert!(parse_tiff(b"II\x00\x00").is_none());
        assert!(parse_tiff(b"MM\x00\x2a\x00\x00\x00\x08").is_none());
    }
}
