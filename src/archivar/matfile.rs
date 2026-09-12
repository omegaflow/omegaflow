use crate::inflate::inflate;

const MI_INT8: u32 = 1;
const MI_INT32: u32 = 5;
const MI_UINT32: u32 = 6;
const MI_SINGLE: u32 = 7;
const MI_DOUBLE: u32 = 9;
const MI_MATRIX: u32 = 14;
const MI_COMPRESSED: u32 = 15;

const MX_CELL_CLASS: u32 = 1;
const MX_STRUCT_CLASS: u32 = 2;
const MX_CHAR: u32 = 4;

#[derive(Clone, Debug)]
pub enum MatData {
    Double(Vec<f64>),
    Single(Vec<f32>),
    Int32(Vec<i32>),
    Char(Vec<u8>),
    Struct(Vec<MatField>),
    Cell(Vec<MatArray>),
    Empty,
}

#[derive(Clone, Debug)]
pub struct MatField {
    pub name: String,
    pub values: Vec<MatArray>,
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

    let data = if cls == MX_STRUCT_CLASS {
        parse_struct_fields(&bytes[pos..], dims.iter().product())?
    } else if cls == MX_CELL_CLASS {
        parse_cell(&bytes[pos..], dims.iter().product())?
    } else {
        let (t, s, align) = read_tag(bytes, &mut pos)?;
        let body = &bytes[pos..pos + s];
        pos = align_to(pos + s, align);
        let mut body_pos = 0usize;
        read_data(body, &mut body_pos, t, s, cls)?
    };

    if complex {
        read_tag(bytes, &mut pos)?;
    }

    Some(MatArray { name, dims, data })
}

fn read_data(body: &[u8], pos: &mut usize, t: u32, s: usize, cls: u32) -> Option<MatData> {
    match (cls, t) {
        (_, MI_DOUBLE) => {
            let n = s / 8;
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                let at = *pos + i * 8;
                v.push(f64::from_le_bytes(body[at..at + 8].try_into().ok()?));
            }
            *pos = align8(*pos + s);
            Some(MatData::Double(v))
        }
        (_, MI_SINGLE) => {
            let n = s / 4;
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                let at = *pos + i * 4;
                v.push(f32::from_le_bytes(body[at..at + 4].try_into().ok()?));
            }
            *pos = align8(*pos + s);
            Some(MatData::Single(v))
        }
        (_, MI_INT32) => {
            let n = s / 4;
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                let at = *pos + i * 4;
                v.push(i32::from_le_bytes(body[at..at + 4].try_into().ok()?));
            }
            *pos = align8(*pos + s);
            Some(MatData::Int32(v))
        }
        (MX_CHAR, MI_INT8) => {
            let v = body[*pos..*pos + s].to_vec();
            *pos = align8(*pos + s);
            Some(MatData::Char(v))
        }
        _ => Some(MatData::Empty),
    }
}

fn read_nested_matrix(body: &[u8], pos: &mut usize) -> Option<MatArray> {
    let (t, s, _) = read_tag(body, pos)?;
    let inner = &body[*pos..*pos + s];
    *pos = align8(*pos + s);
    match t {
        MI_MATRIX => parse_matrix(inner),
        MI_COMPRESSED => {
            let uncomp = zlib_inflate(inner)?;
            let mut upos = 0usize;
            let (mt, ms, _) = read_tag(&uncomp, &mut upos)?;
            if mt != MI_MATRIX {
                return None;
            }
            parse_matrix(&uncomp[upos..upos + ms])
        }
        _ => None,
    }
}

fn parse_struct_fields(body: &[u8], n_elements: usize) -> Option<MatData> {
    let mut pos = 0usize;
    let (_, s, _) = read_tag(body, &mut pos)?;
    pos = align8(pos + s);
    let (t, s, _) = read_tag(body, &mut pos)?;
    if t != MI_INT8 {
        return None;
    }
    let names: Vec<String> = body[pos..pos + s]
        .split(|&b| b == 0)
        .filter(|n| !n.is_empty())
        .map(|n| String::from_utf8_lossy(n).into_owned())
        .collect();
    pos = align8(pos + s);

    let mut fields = Vec::with_capacity(names.len());
    for name in &names {
        let mut values = Vec::with_capacity(n_elements);
        for _ in 0..n_elements {
            values.push(read_nested_matrix(body, &mut pos)?);
        }
        fields.push(MatField {
            name: name.clone(),
            values,
        });
    }
    Some(MatData::Struct(fields))
}

fn parse_cell(body: &[u8], n_elements: usize) -> Option<MatData> {
    let mut pos = 0usize;
    let mut cells = Vec::with_capacity(n_elements);
    for _ in 0..n_elements {
        cells.push(read_nested_matrix(body, &mut pos)?);
    }
    Some(MatData::Cell(cells))
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
            _ => panic!("not double"),
        }
    }

    #[test]
    fn rejects_bad_header() {
        assert!(parse_mat(&[0u8; 100]).is_none());
        let mut bytes = build_header();
        bytes[125] = 0x02;
        assert!(parse_mat(&bytes).is_none());
    }

    fn flags_class(class: u32) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&MI_UINT32.to_le_bytes());
        b.extend_from_slice(&8u32.to_le_bytes());
        b.extend_from_slice(&class.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b
    }

    fn dims_tag(dims: &[i32]) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&MI_INT32.to_le_bytes());
        b.extend_from_slice(&((dims.len() * 4) as u32).to_le_bytes());
        for &d in dims {
            b.extend_from_slice(&d.to_le_bytes());
        }
        b
    }

    fn name_tag(name: &str) -> Vec<u8> {
        let mut b = Vec::new();
        let len = align8(name.len());
        b.extend_from_slice(&MI_INT8.to_le_bytes());
        b.extend_from_slice(&(len as u32).to_le_bytes());
        b.extend_from_slice(name.as_bytes());
        for _ in name.len()..len {
            b.push(0);
        }
        b
    }

    fn single_matrix(name: &str, data: &[f32], dims: &[i32]) -> Vec<u8> {
        let mut body = flags_class(7);
        body.extend_from_slice(&dims_tag(dims));
        body.extend_from_slice(&name_tag(name));
        body.extend_from_slice(&MI_SINGLE.to_le_bytes());
        body.extend_from_slice(&((data.len() * 4) as u32).to_le_bytes());
        for &v in data {
            body.extend_from_slice(&v.to_le_bytes());
        }
        pad_body(&mut body);
        let mut out = Vec::new();
        out.extend_from_slice(&MI_MATRIX.to_le_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
        out
    }

    fn char_matrix(name: &str, text: &[u8], dims: &[i32]) -> Vec<u8> {
        let mut body = flags_class(MX_CHAR);
        body.extend_from_slice(&dims_tag(dims));
        body.extend_from_slice(&name_tag(name));
        body.extend_from_slice(&MI_INT8.to_le_bytes());
        body.extend_from_slice(&(text.len() as u32).to_le_bytes());
        body.extend_from_slice(text);
        pad_body(&mut body);
        let mut out = Vec::new();
        out.extend_from_slice(&MI_MATRIX.to_le_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
        out
    }

    fn pad_body(body: &mut Vec<u8>) {
        while body.len() % 8 != 0 {
            body.push(0);
        }
    }

    fn struct_matrix(name: &str, dims: &[i32], fields: &[(&str, Vec<Vec<u8>>)]) -> Vec<u8> {
        let mut body = flags_class(MX_STRUCT_CLASS);
        body.extend_from_slice(&dims_tag(dims));
        body.extend_from_slice(&name_tag(name));
        let mut table = Vec::new();
        for (f, _) in fields {
            table.extend_from_slice(f.as_bytes());
            table.push(0);
        }
        let field_len = table.len();
        body.extend_from_slice(&5u16.to_le_bytes());
        body.extend_from_slice(&4u16.to_le_bytes());
        body.extend_from_slice(&(field_len as u32).to_le_bytes());
        let table_len = align8(field_len);
        body.extend_from_slice(&MI_INT8.to_le_bytes());
        body.extend_from_slice(&(table_len as u32).to_le_bytes());
        body.extend_from_slice(&table);
        for _ in field_len..table_len {
            body.push(0);
        }
        for (_, values) in fields {
            for v in values {
                body.extend_from_slice(v);
            }
        }
        let mut out = Vec::new();
        out.extend_from_slice(&MI_MATRIX.to_le_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
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

    fn wrap_compressed(inner: &[u8]) -> Vec<u8> {
        let z = zlib_stored(inner);
        let mut out = Vec::new();
        out.extend_from_slice(&MI_COMPRESSED.to_le_bytes());
        out.extend_from_slice(&(z.len() as u32).to_le_bytes());
        out.extend_from_slice(&z);
        out
    }

    fn eeg_struct() -> Vec<u8> {
        let chanlocs = struct_matrix(
            "chanlocs",
            &[1, 2],
            &[(
                "labels",
                vec![
                    char_matrix("labels", b"Fp1", &[1, 3]),
                    char_matrix("labels", b"Fp2", &[1, 3]),
                ],
            )],
        );
        struct_matrix(
            "EEG",
            &[1, 1],
            &[
                ("nbchan", vec![matrix("nbchan", &[2.0], &[1, 1])]),
                ("pnts", vec![matrix("pnts", &[3.0], &[1, 1])]),
                ("srate", vec![matrix("srate", &[100.0], &[1, 1])]),
                (
                    "data",
                    vec![single_matrix(
                        "data",
                        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
                        &[2, 3],
                    )],
                ),
                ("chanlocs", vec![chanlocs]),
            ],
        )
    }

    fn field_double(fields: &[MatField], name: &str) -> Option<f64> {
        fields
            .iter()
            .find(|f| f.name == name)
            .and_then(|f| f.values.first())
            .and_then(|v| match &v.data {
                MatData::Double(d) => d.first().copied(),
                _ => None,
            })
    }

    #[test]
    fn struct_fields_and_nested_chanlocs_parse() {
        let mut bytes = build_header();
        bytes.extend_from_slice(&eeg_struct());
        let arrays = parse_mat(&bytes).unwrap();
        assert_eq!(arrays.len(), 1);
        assert_eq!(arrays[0].name, "EEG");
        let MatData::Struct(fields) = &arrays[0].data else {
            panic!("not struct");
        };
        assert_eq!(field_double(fields, "nbchan"), Some(2.0));
        assert_eq!(field_double(fields, "pnts"), Some(3.0));
        assert_eq!(field_double(fields, "srate"), Some(100.0));
        let data = fields
            .iter()
            .find(|f| f.name == "data")
            .and_then(|f| f.values.first());
        match &data.unwrap().data {
            MatData::Single(v) => assert_eq!(v, &vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
            _ => panic!("not single data"),
        }
        let chanlocs = fields
            .iter()
            .find(|f| f.name == "chanlocs")
            .and_then(|f| f.values.first());
        let MatData::Struct(labels) = &chanlocs.unwrap().data else {
            panic!("not struct chanlocs");
        };
        let labels = labels.iter().find(|f| f.name == "labels").unwrap();
        assert_eq!(labels.values.len(), 2);
        match &labels.values[0].data {
            MatData::Char(c) => assert_eq!(c, b"Fp1"),
            _ => panic!("not char label"),
        }
    }

    #[test]
    fn a_compressed_struct_parses_top_level_and_nested() {
        let mut bytes = build_header();
        bytes.extend_from_slice(&wrap_compressed(&eeg_struct()));
        let arrays = parse_mat(&bytes).unwrap();
        assert_eq!(arrays[0].name, "EEG");

        let data = single_matrix("data", &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
        let eeg = struct_matrix(
            "EEG",
            &[1, 1],
            &[
                ("nbchan", vec![matrix("nbchan", &[2.0], &[1, 1])]),
                ("data", vec![wrap_compressed(&data)]),
            ],
        );
        let mut bytes = build_header();
        bytes.extend_from_slice(&eeg);
        let arrays = parse_mat(&bytes).unwrap();
        let MatData::Struct(fields) = &arrays[0].data else {
            panic!("not struct");
        };
        let data = fields
            .iter()
            .find(|f| f.name == "data")
            .and_then(|f| f.values.first());
        match &data.unwrap().data {
            MatData::Single(v) => assert_eq!(v, &vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
            _ => panic!("not single compressed"),
        }
    }

    #[test]
    fn a_cell_of_doubles_parses() {
        let mut body = flags_class(MX_CELL_CLASS);
        body.extend_from_slice(&dims_tag(&[1, 2]));
        body.extend_from_slice(&name_tag("c"));
        body.extend_from_slice(&matrix("a", &[1.0], &[1, 1]));
        body.extend_from_slice(&matrix("b", &[2.0], &[1, 1]));
        let mut out = Vec::new();
        out.extend_from_slice(&MI_MATRIX.to_le_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);

        let mut bytes = build_header();
        bytes.extend_from_slice(&out);
        let arrays = parse_mat(&bytes).unwrap();
        let MatData::Cell(cells) = &arrays[0].data else {
            panic!("not cell");
        };
        assert_eq!(cells.len(), 2);
        match &cells[0].data {
            MatData::Double(v) => assert_eq!(v, &vec![1.0]),
            _ => panic!("not double cell element"),
        }
        match &cells[1].data {
            MatData::Double(v) => assert_eq!(v, &vec![2.0]),
            _ => panic!("not double cell element"),
        }
    }
}
