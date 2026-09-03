use crate::inflate::inflate;

const MI_INT8: u32 = 1;
const MI_INT32: u32 = 5;
const MI_UINT32: u32 = 6;
const MI_SINGLE: u32 = 7;
const MI_DOUBLE: u32 = 9;
const MI_MATRIX: u32 = 14;
const MI_COMPRESSED: u32 = 15;

const MX_CHAR: u32 = 4;

#[derive(Clone, Debug)]
pub enum MatData {
    Double(Vec<f64>),
    Single(Vec<f32>),
    Int32(Vec<i32>),
    Char(Vec<u8>),
    Empty,
}

#[derive(Clone, Debug)]
pub struct MatArray {
    pub name: String,
    pub dims: Vec<usize>,
    pub data: MatData,
}

fn align8(p: usize) -> usize {
    (p + 7) & !7
}

fn zlib_inflate(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 2 {
        return None;
    }
    inflate(&data[2..])
}

pub fn parse_mat(bytes: &[u8]) -> Option<Vec<MatArray>> {
    if bytes.len() < 128 {
        return None;
    }
    let version = u16::from_le_bytes([bytes[124], bytes[125]]);
    if version != 0x0100 {
        return None;
    }
    if bytes.get(126..128) != Some(b"IM") {
        return None;
    }
    let mut out = Vec::new();
    parse_stream(bytes, 128, &mut out)?;
    Some(out)
}

fn parse_stream(bytes: &[u8], mut pos: usize, out: &mut Vec<MatArray>) -> Option<()> {
    while pos + 8 <= bytes.len() {
        let tag_type = u32::from_le_bytes(bytes[pos..pos + 4].try_into().ok()?);
        let tag_size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().ok()?) as usize;
        pos += 8;
        let end = pos.checked_add(tag_size)?;
        if end > bytes.len() {
            return None;
        }
        match tag_type {
            MI_COMPRESSED => {
                let uncomp = zlib_inflate(&bytes[pos..end])?;
                parse_stream(&uncomp, 0, out)?;
                pos = end;
            }
            MI_MATRIX => {
                out.push(parse_matrix(&bytes[pos..end])?);
                pos = align8(end);
            }
            _ => {
                pos = align8(end);
            }
        }
    }
    Some(())
}

fn align_to(p: usize, a: usize) -> usize {
    (p + a - 1) & !(a - 1)
}

fn read_tag(bytes: &[u8], pos: &mut usize) -> Option<(u32, usize, usize)> {
    if *pos + 4 > bytes.len() {
        return None;
    }
    let v = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().ok()?);
    let ty = v & 0xFFFF;
    let small_type = (1..=7).contains(&ty) || ty == 9;
    if small_type && (v >> 16) != 0 {
        let nbytes = (v >> 16) as usize;
        *pos += 4;
        Some((ty, nbytes, 4))
    } else {
        if *pos + 8 > bytes.len() {
            return None;
        }
        let nbytes = u32::from_le_bytes(bytes[*pos + 4..*pos + 8].try_into().ok()?) as usize;
        *pos += 8;
        Some((v, nbytes, 8))
    }
}

fn parse_matrix(bytes: &[u8]) -> Option<MatArray> {
    let mut pos = 0usize;
    let (t, s, align) = read_tag(bytes, &mut pos)?;
    if t != MI_UINT32 || s < 8 {
        return None;
    }
    let flags_class = u32::from_le_bytes(bytes[pos..pos + 4].try_into().ok()?);
    pos = align_to(pos + 8, align);
    let cls = flags_class & 0xFF;
    let complex = flags_class & 0x0800 != 0;

    let (t, s, align) = read_tag(bytes, &mut pos)?;
    if t != MI_INT32 {
        return None;
    }
    let ndims = s / 4;
    let mut dims = Vec::with_capacity(ndims);
    for i in 0..ndims {
        let d = i32::from_le_bytes(bytes[pos + i * 4..pos + i * 4 + 4].try_into().ok()?);
        dims.push(d as usize);
    }
    pos = align_to(pos + s, align);

    let (t, s, align) = read_tag(bytes, &mut pos)?;
    if t != MI_INT8 {
        return None;
    }
    let raw_name = &bytes[pos..pos + s];
    let name = raw_name.split(|&b| b == 0).next().unwrap_or(b"");
    let name = String::from_utf8_lossy(name).into_owned();
    pos = align_to(pos + s, align);

    let (t, s, align) = read_tag(bytes, &mut pos)?;
    let data = read_data(bytes, pos, t, s, cls)?;
    pos = align_to(pos + s, align);

    if complex {
        read_tag(bytes, &mut pos)?;
    }

    Some(MatArray { name, dims, data })
}

fn read_data(bytes: &[u8], pos: usize, t: u32, s: usize, cls: u32) -> Option<MatData> {
    match (cls, t) {
        (_, MI_DOUBLE) => {
            let n = s / 8;
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                v.push(f64::from_le_bytes(
                    bytes[pos + i * 8..pos + i * 8 + 8].try_into().ok()?,
                ));
            }
            Some(MatData::Double(v))
        }
        (_, MI_SINGLE) => {
            let n = s / 4;
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                v.push(f32::from_le_bytes(
                    bytes[pos + i * 4..pos + i * 4 + 4].try_into().ok()?,
                ));
            }
            Some(MatData::Single(v))
        }
        (_, MI_INT32) => {
            let n = s / 4;
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                v.push(i32::from_le_bytes(
                    bytes[pos + i * 4..pos + i * 4 + 4].try_into().ok()?,
                ));
            }
            Some(MatData::Int32(v))
        }
        (MX_CHAR, MI_INT8) => Some(MatData::Char(bytes[pos..pos + s].to_vec())),
        _ => Some(MatData::Empty),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_header() -> Vec<u8> {
        let mut h = vec![0u8; 128];
        let text = b"MATLAB 5.0 MAT-file";
        h[..text.len()].copy_from_slice(text);
        h[124] = 0x00;
        h[125] = 0x01;
        h[126] = b'I';
        h[127] = b'M';
        h
    }

    fn matrix(name: &str, data: &[f64], dims: &[i32]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&MI_UINT32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&6u32.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&MI_INT32.to_le_bytes());
        body.extend_from_slice(&((dims.len() * 4) as u32).to_le_bytes());
        for &d in dims {
            body.extend_from_slice(&d.to_le_bytes());
        }
        let name_len = align8(name.len());
        body.extend_from_slice(&MI_INT8.to_le_bytes());
        body.extend_from_slice(&(name_len as u32).to_le_bytes());
        body.extend_from_slice(name.as_bytes());
        for _ in name.len()..name_len {
            body.push(0);
        }
        body.extend_from_slice(&MI_DOUBLE.to_le_bytes());
        body.extend_from_slice(&((data.len() * 8) as u32).to_le_bytes());
        for &v in data {
            body.extend_from_slice(&v.to_le_bytes());
        }
        let mut out = Vec::new();
        out.extend_from_slice(&MI_MATRIX.to_le_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
        out
    }

    #[test]
    fn parse_uncompressed_double_matrix() {
        let mut bytes = build_header();
        bytes.extend_from_slice(&matrix("a", &[1.0, 2.0, 3.0, 4.0], &[2, 2]));
        let arrays = parse_mat(&bytes).unwrap();
        assert_eq!(arrays.len(), 1);
        assert_eq!(arrays[0].name, "a");
        assert_eq!(arrays[0].dims, vec![2, 2]);
        match &arrays[0].data {
            MatData::Double(v) => assert_eq!(v, &vec![1.0, 2.0, 3.0, 4.0]),
            _ => panic!("expected double"),
        }
    }

    #[test]
    fn rejects_bad_header() {
        assert!(parse_mat(&[0u8; 100]).is_none());
        let mut bytes = build_header();
        bytes[125] = 0x02;
        assert!(parse_mat(&bytes).is_none());
    }
}
