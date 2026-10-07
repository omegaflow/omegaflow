#[derive(Clone, Debug, PartialEq)]
pub enum Mat4Data {
    Double(Vec<f64>),
    Single(Vec<f32>),
    Int32(Vec<i32>),
    Int16(Vec<i16>),
    UInt16(Vec<u16>),
    UInt8(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mat4Var {
    pub name: String,
    pub mrows: usize,
    pub ncols: usize,
    pub data: Mat4Data,
}

fn element_width(precision: i32) -> Option<usize> {
    match precision {
        0 => Some(8),
        1 => Some(4),
        2 => Some(4),
        3 => Some(2),
        4 => Some(2),
        5 => Some(1),
        _ => None,
    }
}

fn i32_at(bytes: &[u8], at: usize) -> Option<i32> {
    Some(i32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

fn read_f64(chunk: &[u8], count: usize) -> Option<Vec<f64>> {
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let at = i * 8;
        out.push(f64::from_le_bytes(chunk.get(at..at + 8)?.try_into().ok()?));
    }
    Some(out)
}

fn read_f32(chunk: &[u8], count: usize) -> Option<Vec<f32>> {
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let at = i * 4;
        out.push(f32::from_le_bytes(chunk.get(at..at + 4)?.try_into().ok()?));
    }
    Some(out)
}

fn read_i32(chunk: &[u8], count: usize) -> Option<Vec<i32>> {
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let at = i * 4;
        out.push(i32::from_le_bytes(chunk.get(at..at + 4)?.try_into().ok()?));
    }
    Some(out)
}

fn read_i16(chunk: &[u8], count: usize) -> Option<Vec<i16>> {
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let at = i * 2;
        out.push(i16::from_le_bytes(chunk.get(at..at + 2)?.try_into().ok()?));
    }
    Some(out)
}

fn read_u16(chunk: &[u8], count: usize) -> Option<Vec<u16>> {
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let at = i * 2;
        out.push(u16::from_le_bytes(chunk.get(at..at + 2)?.try_into().ok()?));
    }
    Some(out)
}

pub fn parse_mat4(bytes: &[u8]) -> Option<Vec<Mat4Var>> {
    let mut pos = 0usize;
    let mut out = Vec::new();
    while pos + 20 <= bytes.len() {
        let mopt = i32_at(bytes, pos)?;
        let mrows = i32_at(bytes, pos + 4)?;
        let ncols = i32_at(bytes, pos + 8)?;
        let imagf = i32_at(bytes, pos + 12)?;
        let namelen = i32_at(bytes, pos + 16)?;
        if mopt < 0 || mopt / 100 != 0 || mrows < 0 || ncols < 0 || namelen < 1 {
            return None;
        }
        pos += 20;
        let name_end = pos.checked_add(namelen as usize)?;
        let raw = bytes.get(pos..name_end)?;
        let name = String::from_utf8_lossy(raw.split(|&b| b == 0).next()?).into_owned();
        pos = name_end;
        let count = (mrows as usize).checked_mul(ncols as usize)?;
        let precision = (mopt / 10) % 10;
        let width = element_width(precision)?;
        let body_end = pos.checked_add(count.checked_mul(width)?)?;
        let chunk = bytes.get(pos..body_end)?;
        let data = match precision {
            0 => Mat4Data::Double(read_f64(chunk, count)?),
            1 => Mat4Data::Single(read_f32(chunk, count)?),
            2 => Mat4Data::Int32(read_i32(chunk, count)?),
            3 => Mat4Data::Int16(read_i16(chunk, count)?),
            4 => Mat4Data::UInt16(read_u16(chunk, count)?),
            5 => Mat4Data::UInt8(chunk.to_vec()),
            _ => return None,
        };
        pos = body_end;
        if imagf != 0 {
            pos = pos.checked_add(count.checked_mul(width)?)?;
            if pos > bytes.len() {
                return None;
            }
        }
        out.push(Mat4Var {
            name,
            mrows: mrows as usize,
            ncols: ncols as usize,
            data,
        });
    }
    if pos == bytes.len() { Some(out) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VLF: &[u8] = include_bytes!("mat4_fixtures/vlf_awesome_narrowband_A.mat");

    fn var<'a>(vars: &'a [Mat4Var], name: &str) -> &'a Mat4Var {
        vars.iter().find(|v| v.name == name).expect(name)
    }

    #[test]
    fn parses_the_measured_craam_narrowband_file() {
        let vars = parse_mat4(VLF).expect("the measured CRAAM file parses");
        assert_eq!(
            var(&vars, "VERSION").data,
            Mat4Data::Double(vec![f64::from_bits(0x409F_5870_BE0D_ED29)])
        );
        match &var(&vars, "station_name").data {
            Mat4Data::UInt8(b) => assert_eq!(b.as_slice(), b"EACF"),
            other => panic!("station_name not uint8: {other:?}"),
        }
        match &var(&vars, "call_sign").data {
            Mat4Data::UInt8(b) => assert_eq!(b.as_slice(), b"NAA"),
            other => panic!("call_sign not uint8: {other:?}"),
        }
        match &var(&vars, "is_broadband").data {
            Mat4Data::Int16(v) => assert_eq!(v, &vec![0i16]),
            other => panic!("is_broadband not int16: {other:?}"),
        }
        match &var(&vars, "filter_taps").data {
            Mat4Data::Double(v) => assert_eq!(v.len(), 1000),
            other => panic!("filter_taps not double: {other:?}"),
        }
        let data = var(&vars, "data");
        assert_eq!(data.mrows, 85800);
        assert_eq!(data.ncols, 1);
        match &data.data {
            Mat4Data::Single(v) => {
                assert_eq!(v.len(), 85800);
                assert_eq!(v[0], f32::from_bits(0x4204_6B42));
            }
            other => panic!("data not single: {other:?}"),
        }
    }

    #[test]
    fn a_truncated_file_stays_absent() {
        assert!(parse_mat4(&VLF[..VLF.len() - 4]).is_none());
        assert!(parse_mat4(&VLF[..20]).is_none());
    }
}
