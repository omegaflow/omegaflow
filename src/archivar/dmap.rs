pub const DMAP_CODE: u32 = 0x0001_0001;

pub enum ScalarValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Str,
}

impl ScalarValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            ScalarValue::I8(v) => Some(*v as f64),
            ScalarValue::I16(v) => Some(*v as f64),
            ScalarValue::I32(v) => Some(*v as f64),
            ScalarValue::I64(v) => Some(*v as f64),
            ScalarValue::F32(v) => Some(*v as f64),
            ScalarValue::F64(v) => Some(*v),
            ScalarValue::Str => None,
        }
    }
}

pub struct DmapScalar {
    pub name: String,
    pub value: ScalarValue,
}

pub struct DmapArray {
    pub name: String,
    pub nums: Vec<f64>,
}

pub struct DmapRecord {
    pub scalars: Vec<DmapScalar>,
    pub arrays: Vec<DmapArray>,
}

pub fn rec_scalar<'a>(rec: &'a DmapRecord, name: &str) -> Option<&'a ScalarValue> {
    rec.scalars
        .iter()
        .find(|s| s.name == name)
        .map(|s| &s.value)
}

pub fn rec_f64(rec: &DmapRecord, name: &str) -> Option<f64> {
    rec_scalar(rec, name).and_then(ScalarValue::as_f64)
}

pub fn rec_array<'a>(rec: &'a DmapRecord, name: &str) -> Option<&'a DmapArray> {
    rec.arrays.iter().find(|a| a.name == name)
}

pub fn map_grid_value(rec: &DmapRecord) -> Option<(f64, f64)> {
    let vel = rec_array(rec, "vector.vel.median")?.nums.first().copied()?;
    let azm = rec_array(rec, "vector.kvect")?.nums.first().copied()?;
    if vel.is_finite() && azm.is_finite() {
        Some((vel, azm))
    } else {
        None
    }
}

fn le_u16_at(b: &[u8], p: usize) -> Option<u16> {
    let s = b.get(p..p + 2)?;
    let arr: [u8; 2] = s.try_into().ok()?;
    Some(u16::from_le_bytes(arr))
}

fn le_u32_at(b: &[u8], p: usize) -> Option<u32> {
    let s = b.get(p..p + 4)?;
    let arr: [u8; 4] = s.try_into().ok()?;
    Some(u32::from_le_bytes(arr))
}

fn le_i32_at(b: &[u8], p: usize) -> Option<i32> {
    let s = b.get(p..p + 4)?;
    let arr: [u8; 4] = s.try_into().ok()?;
    Some(i32::from_le_bytes(arr))
}

fn le_i64_at(b: &[u8], p: usize) -> Option<i64> {
    let s = b.get(p..p + 8)?;
    let arr: [u8; 8] = s.try_into().ok()?;
    Some(i64::from_le_bytes(arr))
}

fn le_f32_at(b: &[u8], p: usize) -> Option<f32> {
    let s = b.get(p..p + 4)?;
    let arr: [u8; 4] = s.try_into().ok()?;
    Some(f32::from_le_bytes(arr))
}

fn le_f64_at(b: &[u8], p: usize) -> Option<f64> {
    let s = b.get(p..p + 8)?;
    let arr: [u8; 8] = s.try_into().ok()?;
    Some(f64::from_le_bytes(arr))
}

fn cstring_at(b: &[u8], p: usize) -> Option<String> {
    let mut end = p;
    while end < b.len() && b[end] != 0 {
        end += 1;
    }
    if end >= b.len() {
        return None;
    }
    Some(String::from_utf8_lossy(&b[p..end]).into_owned())
}

fn scalar_at(b: &[u8], p: usize, t: u8) -> Option<(ScalarValue, usize)> {
    match t {
        1 => Some((ScalarValue::I8(*b.get(p)? as i8), 1)),
        2 => Some((ScalarValue::I16(le_u16_at(b, p)? as i16), 2)),
        3 => Some((ScalarValue::I32(le_i32_at(b, p)?), 4)),
        4 => Some((ScalarValue::F32(le_f32_at(b, p)?), 4)),
        8 => Some((ScalarValue::F64(le_f64_at(b, p)?), 8)),
        9 => {
            let s = cstring_at(b, p)?;
            let adv = s.len() + 1;
            Some((ScalarValue::Str, adv))
        }
        10 => Some((ScalarValue::I64(le_i64_at(b, p)?), 8)),
        16 => Some((ScalarValue::I8(*b.get(p)? as i8), 1)),
        17 => Some((ScalarValue::I32(le_u16_at(b, p)? as i32), 2)),
        18 => Some((ScalarValue::I64(le_u32_at(b, p)? as i64), 4)),
        19 => {
            let s = b.get(p..p + 8)?;
            let arr: [u8; 8] = s.try_into().ok()?;
            let v = u64::from_le_bytes(arr);
            Some((ScalarValue::I64(i64::try_from(v).ok()?), 8))
        }
        255 => {
            let len = le_u32_at(b, p)? as usize;
            if p + 4 + len > b.len() {
                return None;
            }
            Some((ScalarValue::Str, 4 + len))
        }
        _ => None,
    }
}

fn array_at(b: &[u8], p: usize, t: u8, n: usize, name: String) -> Option<(DmapArray, usize)> {
    let mut nums = Vec::new();
    let mut q = p;
    match t {
        9 => {
            for _ in 0..n {
                let s = cstring_at(b, q)?;
                q += s.len() + 1;
            }
        }
        1 | 16 => {
            for _ in 0..n {
                let v = *b.get(q)?;
                q += 1;
                nums.push(v as f64);
            }
        }
        2 | 17 => {
            for _ in 0..n {
                let v = le_u16_at(b, q)?;
                q += 2;
                nums.push(v as f64);
            }
        }
        3 => {
            for _ in 0..n {
                let v = le_i32_at(b, q)?;
                q += 4;
                nums.push(v as f64);
            }
        }
        18 => {
            for _ in 0..n {
                let v = le_u32_at(b, q)?;
                q += 4;
                nums.push(v as f64);
            }
        }
        4 => {
            for _ in 0..n {
                let v = le_f32_at(b, q)?;
                q += 4;
                nums.push(v as f64);
            }
        }
        8 => {
            for _ in 0..n {
                let v = le_f64_at(b, q)?;
                q += 8;
                nums.push(v);
            }
        }
        10 => {
            for _ in 0..n {
                let v = le_i64_at(b, q)?;
                q += 8;
                nums.push(v as f64);
            }
        }
        19 => {
            for _ in 0..n {
                let s = b.get(q..q + 8)?;
                let arr: [u8; 8] = s.try_into().ok()?;
                q += 8;
                nums.push(u64::from_le_bytes(arr) as f64);
            }
        }
        _ => return None,
    }
    Some((DmapArray { name, nums }, q - p))
}

fn dmap_record_at(bytes: &[u8], p: usize) -> Option<(DmapRecord, usize)> {
    let code = le_u32_at(bytes, p)?;
    let sze = le_u32_at(bytes, p + 4)? as usize;
    if code != DMAP_CODE || sze < 16 || p + sze > bytes.len() {
        return None;
    }
    let rec = &bytes[p..p + sze];
    let sn = le_u32_at(rec, 8)? as usize;
    let an = le_u32_at(rec, 12)? as usize;
    if sn > 4096 || an > 4096 {
        return None;
    }
    let mut q = 16usize;
    let mut scalars = Vec::new();
    for _ in 0..sn {
        let name = cstring_at(rec, q)?;
        q += name.len() + 1;
        let t = *rec.get(q)?;
        q += 1;
        let (value, adv) = scalar_at(rec, q, t)?;
        q += adv;
        scalars.push(DmapScalar { name, value });
    }
    let mut arrays = Vec::new();
    for _ in 0..an {
        let name = cstring_at(rec, q)?;
        q += name.len() + 1;
        let t = *rec.get(q)?;
        q += 1;
        let dim = le_u32_at(rec, q)?;
        q += 4;
        if dim < 1 || dim > 8 {
            return None;
        }
        let mut n: usize = 1;
        for _ in 0..dim as usize {
            let r = le_u32_at(rec, q)?;
            q += 4;
            n = match n.checked_mul(r as usize) {
                Some(v) => v,
                None => return None,
            };
        }
        if n > 200_000_000 {
            return None;
        }
        let (arr, adv) = array_at(rec, q, t, n, name)?;
        q += adv;
        arrays.push(arr);
    }
    if q != rec.len() {
        return None;
    }
    Some((DmapRecord { scalars, arrays }, p + sze))
}

pub fn parse_records(bytes: &[u8]) -> Option<Vec<DmapRecord>> {
    let mut p = 0usize;
    let mut recs = Vec::new();
    while p < bytes.len() {
        if p + 8 > bytes.len() {
            return None;
        }
        let (rec, next) = dmap_record_at(bytes, p)?;
        recs.push(rec);
        p = next;
    }
    if recs.is_empty() {
        return None;
    }
    Some(recs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enc_cstr(out: &mut Vec<u8>, s: &str) {
        out.extend_from_slice(s.as_bytes());
        out.push(0);
    }

    fn enc_scalar_i32(out: &mut Vec<u8>, name: &str, v: i32) {
        enc_cstr(out, name);
        out.push(3);
        out.extend_from_slice(&v.to_le_bytes());
    }

    fn enc_array_f32(out: &mut Vec<u8>, name: &str, data: &[f32]) {
        enc_cstr(out, name);
        out.push(4);
        out.extend_from_slice(&1u32.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        for v in data {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }

    fn sample_record() -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&1u32.to_le_bytes());
        payload.extend_from_slice(&1u32.to_le_bytes());
        enc_scalar_i32(&mut payload, "nrang", 3);
        enc_array_f32(&mut payload, "v", &[1.0, -2.0, 3.5]);
        let sze = 8 + payload.len();
        let mut out = Vec::new();
        out.extend_from_slice(&DMAP_CODE.to_le_bytes());
        out.extend_from_slice(&(sze as u32).to_le_bytes());
        out.extend_from_slice(&payload);
        out
    }

    #[test]
    fn parse_records_reads_a_synthetic_record() {
        let recs = parse_records(&sample_record()).expect("one record");
        assert_eq!(recs.len(), 1);
        assert_eq!(rec_f64(&recs[0], "nrang"), Some(3.0));
        let v = rec_array(&recs[0], "v").expect("v");
        assert_eq!(v.nums, vec![1.0, -2.0, 3.5]);
    }

    #[test]
    fn parse_records_reads_two_records() {
        let mut bytes = sample_record();
        bytes.extend_from_slice(&sample_record());
        let recs = parse_records(&bytes).expect("two records");
        assert_eq!(recs.len(), 2);
    }

    #[test]
    fn parse_records_void_on_a_truncated_stream() {
        let bytes = sample_record();
        assert!(parse_records(&bytes[..bytes.len() - 1]).is_none());
    }

    fn sample_grid_record() -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&0u32.to_le_bytes());
        payload.extend_from_slice(&2u32.to_le_bytes());
        enc_array_f32(&mut payload, "vector.vel.median", &[123.5]);
        enc_array_f32(&mut payload, "vector.kvect", &[45.0]);
        let sze = 8 + payload.len();
        let mut out = Vec::new();
        out.extend_from_slice(&DMAP_CODE.to_le_bytes());
        out.extend_from_slice(&(sze as u32).to_le_bytes());
        out.extend_from_slice(&payload);
        out
    }

    #[test]
    fn map_grid_value_reads_velocity_and_kvect() {
        let recs = parse_records(&sample_grid_record()).expect("one record");
        assert_eq!(map_grid_value(&recs[0]), Some((123.5, 45.0)));
    }

    #[test]
    fn map_grid_value_void_without_kvect() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&0u32.to_le_bytes());
        payload.extend_from_slice(&1u32.to_le_bytes());
        enc_array_f32(&mut payload, "vector.vel.median", &[123.5]);
        let sze = 8 + payload.len();
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&DMAP_CODE.to_le_bytes());
        bytes.extend_from_slice(&(sze as u32).to_le_bytes());
        bytes.extend_from_slice(&payload);
        let recs = parse_records(&bytes).expect("one record");
        assert_eq!(map_grid_value(&recs[0]), None);
    }
}
