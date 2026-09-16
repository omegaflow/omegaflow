use crate::inflate::inflate;

const MI_INT8: u32 = 1;
const MI_UINT8: u32 = 2;
const MI_INT16: u32 = 3;
const MI_UINT16: u32 = 4;
const MI_INT32: u32 = 5;
const MI_UINT32: u32 = 6;
const MI_SINGLE: u32 = 7;
const MI_UTF8: u32 = 8;
const MI_DOUBLE: u32 = 9;
const MI_MATRIX: u32 = 14;
const MI_COMPRESSED: u32 = 15;

const MX_CHAR_CLASS: u32 = 4;
const MX_DOUBLE_CLASS: u32 = 6;
const MX_SINGLE_CLASS: u32 = 7;
const MX_INT8_CLASS: u32 = 8;
const MX_UINT8_CLASS: u32 = 9;
const MX_INT16_CLASS: u32 = 10;
const MX_UINT16_CLASS: u32 = 11;
const MX_INT32_CLASS: u32 = 12;
const MX_UINT32_CLASS: u32 = 13;

pub struct MatVar {
    pub name: String,
    pub dims: Vec<usize>,
    pub values: Vec<f64>,
}

struct Tag {
    ty: u32,
    size: usize,
}

fn align8(p: usize) -> usize {
    (p + 7) & !7
}

fn read_tag(data: &[u8], pos: &mut usize) -> Option<Tag> {
    let word = u32::from_le_bytes(data.get(*pos..*pos + 4)?.try_into().ok()?);
    let ty = word & 0xFFFF;
    let small = matches!(ty, MI_INT8 | MI_UINT8 | MI_INT16 | MI_UINT16);
    if small && (word >> 16) != 0 {
        *pos += 4;
        Some(Tag {
            ty,
            size: (word >> 16) as usize,
        })
    } else {
        let size = u32::from_le_bytes(data.get(*pos + 4..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        Some(Tag {
            ty,
            size: size as usize,
        })
    }
}

pub fn parse_mat5(data: &[u8]) -> Option<Vec<MatVar>> {
    if data.len() < 128 {
        return None;
    }
    if data[124] != 0x00 || data[125] != 0x01 || data[126] != b'I' || data[127] != b'M' {
        return None;
    }
    let mut out = Vec::new();
    parse_stream(data, 128, &mut out)?;
    Some(out)
}

fn parse_stream(data: &[u8], mut pos: usize, out: &mut Vec<MatVar>) -> Option<()> {
    while pos + 8 <= data.len() {
        let tag = read_tag(data, &mut pos)?;
        let end = pos.checked_add(tag.size)?;
        if end > data.len() {
            return None;
        }
        match tag.ty {
            MI_MATRIX => {
                if let Some(var) = parse_matrix(&data[pos..end]) {
                    out.push(var);
                }
            }
            MI_COMPRESSED => {
                let uncompressed = zlib_inflate(&data[pos..end])?;
                parse_stream(&uncompressed, 0, out)?;
            }
            _ => {}
        }
        pos = align8(end);
    }
    Some(())
}

fn zlib_inflate(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 2 {
        return None;
    }
    inflate(&data[2..])
}

fn parse_matrix(data: &[u8]) -> Option<MatVar> {
    let mut pos = 0usize;

    let flags = read_tag(data, &mut pos)?;
    if flags.ty != MI_UINT32 || flags.size < 8 {
        return None;
    }
    let array_flags = u32::from_le_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
    let class = array_flags & 0xFF;
    pos = align8(pos + flags.size);

    let dims = read_tag(data, &mut pos)?;
    if dims.ty != MI_INT32 {
        return None;
    }
    let ndims = dims.size / 4;
    let mut dims_out = Vec::with_capacity(ndims);
    for i in 0..ndims {
        let at = pos + i * 4;
        let d = i32::from_le_bytes(data.get(at..at + 4)?.try_into().ok()?);
        if d < 0 {
            return None;
        }
        dims_out.push(d as usize);
    }
    pos = align8(pos + dims.size);

    let name = read_tag(data, &mut pos)?;
    if name.ty != MI_INT8 && name.ty != MI_UTF8 {
        return None;
    }
    let name_bytes = data.get(pos..pos + name.size)?;
    let name_out = match name_bytes.split(|&b| b == 0).next() {
        Some(s) => String::from_utf8_lossy(s).into_owned(),
        None => String::new(),
    };
    pos = align8(pos + name.size);

    let values = match class {
        MX_DOUBLE_CLASS | MX_SINGLE_CLASS | MX_INT8_CLASS | MX_UINT8_CLASS | MX_INT16_CLASS
        | MX_UINT16_CLASS | MX_INT32_CLASS | MX_UINT32_CLASS => {
            let data_tag = read_tag(data, &mut pos)?;
            let body = data.get(pos..pos + data_tag.size)?;
            decode_numeric(body, data_tag.ty)?
        }
        MX_CHAR_CLASS => Vec::new(),
        _ => return None,
    };

    Some(MatVar {
        name: name_out,
        dims: dims_out,
        values,
    })
}

fn decode_numeric(body: &[u8], ty: u32) -> Option<Vec<f64>> {
    let width = match ty {
        MI_DOUBLE => 8,
        MI_SINGLE | MI_INT32 | MI_UINT32 => 4,
        MI_INT16 | MI_UINT16 => 2,
        MI_INT8 | MI_UINT8 => 1,
        _ => return None,
    };
    let n = body.len() / width;
    let mut values = Vec::with_capacity(n);
    for i in 0..n {
        let at = i * width;
        let v = match ty {
            MI_DOUBLE => f64::from_le_bytes(body.get(at..at + 8)?.try_into().ok()?),
            MI_SINGLE => f32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?) as f64,
            MI_INT32 => i32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?) as f64,
            MI_UINT32 => u32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?) as f64,
            MI_INT16 => i16::from_le_bytes(body.get(at..at + 2)?.try_into().ok()?) as f64,
            MI_UINT16 => u16::from_le_bytes(body.get(at..at + 2)?.try_into().ok()?) as f64,
            MI_INT8 => (*body.get(at)?) as i8 as f64,
            MI_UINT8 => (*body.get(at)?) as f64,
            _ => return None,
        };
        values.push(v);
    }
    Some(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header() -> Vec<u8> {
        let mut h = vec![0u8; 128];
        let text = b"MATLAB 5.0 MAT-file";
        h[..text.len()].copy_from_slice(text);
        h[124] = 0x00;
        h[125] = 0x01;
        h[126] = b'I';
        h[127] = b'M';
        h
    }

    fn push_tag(out: &mut Vec<u8>, ty: u32, size: usize) {
        out.extend_from_slice(&ty.to_le_bytes());
        out.extend_from_slice(&(size as u32).to_le_bytes());
    }

    fn push_pad(out: &mut Vec<u8>) {
        while !out.len().is_multiple_of(8) {
            out.push(0);
        }
    }

    fn double_matrix(name: &str, dims: &[i32], data: &[f64]) -> Vec<u8> {
        let mut body = Vec::new();
        push_tag(&mut body, MI_UINT32, 8);
        body.extend_from_slice(&MX_DOUBLE_CLASS.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        push_tag(&mut body, MI_INT32, dims.len() * 4);
        for &d in dims {
            body.extend_from_slice(&d.to_le_bytes());
        }
        let name_len = align8(name.len());
        push_tag(&mut body, MI_INT8, name_len);
        body.extend_from_slice(name.as_bytes());
        body.resize(name_len, 0);
        push_tag(&mut body, MI_DOUBLE, data.len() * 8);
        for &v in data {
            body.extend_from_slice(&v.to_le_bytes());
        }
        push_pad(&mut body);
        let mut out = Vec::new();
        push_tag(&mut out, MI_MATRIX, body.len());
        out.extend_from_slice(&body);
        out
    }

    fn zlib_stored(raw: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&[0x78, 0x01]);
        out.push(0x01);
        let len = raw.len() as u16;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(raw);
        out.extend_from_slice(&[0u8; 4]);
        out
    }

    #[test]
    fn parses_double_matrix() {
        let mut bytes = header();
        bytes.extend_from_slice(&double_matrix("a", &[2, 2], &[1.0, 2.0, 3.0, 4.0]));
        let vars = parse_mat5(&bytes).unwrap();
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].name, "a");
        assert_eq!(vars[0].dims, vec![2, 2]);
        assert_eq!(vars[0].values, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn parses_compressed_matrix() {
        let inner = double_matrix("psd", &[2, 2], &[5.0, 6.0, 7.0, 8.0]);
        let z = zlib_stored(&inner);
        let mut bytes = header();
        push_tag(&mut bytes, MI_COMPRESSED, z.len());
        bytes.extend_from_slice(&z);
        let vars = parse_mat5(&bytes).unwrap();
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].name, "psd");
        assert_eq!(vars[0].values, vec![5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn parses_small_format_name_tag() {
        let mut body = Vec::new();
        push_tag(&mut body, MI_UINT32, 8);
        body.extend_from_slice(&MX_DOUBLE_CLASS.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        push_tag(&mut body, MI_INT32, 8);
        body.extend_from_slice(&1i32.to_le_bytes());
        body.extend_from_slice(&1i32.to_le_bytes());
        body.extend_from_slice(&1u16.to_le_bytes());
        body.extend_from_slice(&1u16.to_le_bytes());
        body.push(b'x');
        push_pad(&mut body);
        push_tag(&mut body, MI_DOUBLE, 8);
        body.extend_from_slice(&42.0f64.to_le_bytes());
        push_pad(&mut body);

        let mut bytes = header();
        push_tag(&mut bytes, MI_MATRIX, body.len());
        bytes.extend_from_slice(&body);

        let vars = parse_mat5(&bytes).unwrap();
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].name, "x");
        assert_eq!(vars[0].dims, vec![1, 1]);
        assert_eq!(vars[0].values, vec![42.0]);
    }

    #[test]
    fn rejects_bad_header() {
        assert!(parse_mat5(&[0u8; 100]).is_none());
        let mut bytes = header();
        bytes[125] = 0x02;
        assert!(parse_mat5(&bytes).is_none());
        let mut bytes = header();
        bytes[126] = b'M';
        assert!(parse_mat5(&bytes).is_none());
    }
}
