use std::collections::HashMap;

pub const DFTAG_LINKED: u16 = 20;
pub const DFTAG_SD: u16 = 702;
pub const DFTAG_VH: u16 = 1962;
pub const DFTAG_VS: u16 = 1963;

const SPECIAL: u16 = 0x4000;
const MAGIC: [u8; 4] = [0x0e, 0x03, 0x13, 0x01];

pub const DFNT_CHAR8: i16 = 4;
pub const DFNT_UCHAR8: i16 = 3;
pub const DFNT_FLOAT32: i16 = 5;
pub const DFNT_FLOAT64: i16 = 6;
pub const DFNT_INT8: i16 = 20;
pub const DFNT_UINT8: i16 = 21;
pub const DFNT_INT16: i16 = 22;
pub const DFNT_UINT16: i16 = 23;
pub const DFNT_INT32: i16 = 24;
pub const DFNT_UINT32: i16 = 25;
pub const DFNT_INT64: i16 = 26;
pub const DFNT_UINT64: i16 = 27;

fn be_u16(b: &[u8], o: usize) -> Option<u16> {
    let hi = *b.get(o)? as u16;
    let lo = *b.get(o + 1)? as u16;
    Some((hi << 8) | lo)
}

fn be_i16(b: &[u8], o: usize) -> Option<i16> {
    Some(be_u16(b, o)? as i16)
}

fn be_u32(b: &[u8], o: usize) -> Option<u32> {
    let b0 = *b.get(o)? as u32;
    let b1 = *b.get(o + 1)? as u32;
    let b2 = *b.get(o + 2)? as u32;
    let b3 = *b.get(o + 3)? as u32;
    Some((b0 << 24) | (b1 << 16) | (b2 << 8) | b3)
}

fn be_i32(b: &[u8], o: usize) -> Option<i32> {
    Some(be_u32(b, o)? as i32)
}

fn be_u64(b: &[u8], o: usize) -> Option<u64> {
    let hi = be_u32(b, o)? as u64;
    let lo = be_u32(b, o + 4)? as u64;
    Some((hi << 32) | lo)
}

#[derive(Clone, Copy)]
pub struct Dd {
    pub tag: u16,
    pub ref_: u16,
    pub offset: usize,
    pub length: usize,
}

pub struct VField {
    pub name: String,
    pub typ: i16,
    pub isize: u16,
    pub off: u16,
    pub order: u16,
}

pub struct VHeader {
    pub interlace: i16,
    pub nrecords: i32,
    pub ivsize: u16,
    pub fields: Vec<VField>,
    pub vsname: String,
    pub vsclass: String,
}

pub struct Hdf4<'a> {
    bytes: &'a [u8],
    dds: Vec<Dd>,
    index: HashMap<(u16, u16), usize>,
}

impl<'a> Hdf4<'a> {
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        if bytes.get(0..4)? != MAGIC {
            return None;
        }
        let mut dds = Vec::new();
        let mut off = 4usize;
        loop {
            let ndds = be_u16(bytes, off)?;
            let next = be_i32(bytes, off + 2)?;
            if ndds == 0 {
                return None;
            }
            let mut p = off + 6;
            for _ in 0..ndds {
                let tag = be_u16(bytes, p)?;
                let ref_ = be_u16(bytes, p + 2)?;
                let offset = be_i32(bytes, p + 4)?;
                let length = be_i32(bytes, p + 8)?;
                if offset < 0 || length < 0 {
                    return None;
                }
                dds.push(Dd {
                    tag,
                    ref_,
                    offset: offset as usize,
                    length: length as usize,
                });
                p += 12;
            }
            if next == 0 {
                break;
            }
            off = next as usize;
        }
        let mut index = HashMap::with_capacity(dds.len());
        for (i, dd) in dds.iter().enumerate() {
            index.insert((dd.tag, dd.ref_), i);
        }
        Some(Hdf4 { bytes, dds, index })
    }

    pub fn dd(&self, tag: u16, ref_: u16) -> Option<&Dd> {
        self.index.get(&(tag, ref_)).map(|&i| &self.dds[i])
    }

    pub fn dds(&self) -> &[Dd] {
        &self.dds
    }

    fn normal(&self, tag: u16, ref_: u16) -> Option<&'a [u8]> {
        let dd = self.dd(tag, ref_)?;
        let end = dd.offset.checked_add(dd.length)?;
        self.bytes.get(dd.offset..end)
    }

    pub fn element(&self, tag: u16, ref_: u16) -> Option<Vec<u8>> {
        if let Some(slice) = self.normal(tag, ref_) {
            return Some(slice.to_vec());
        }
        let dd = self.dd(tag | SPECIAL, ref_)?;
        self.linked(dd)
    }

    fn linked(&self, dd: &Dd) -> Option<Vec<u8>> {
        let base = dd.offset;
        let total = be_i32(self.bytes, base + 2)?;
        let nblocks = be_i32(self.bytes, base + 10)?;
        let link_ref = be_u16(self.bytes, base + 14)?;
        if total < 0 || nblocks <= 0 || link_ref == 0 {
            return None;
        }
        let total = total as usize;
        let nblocks = nblocks as usize;
        let mut out: Vec<u8> = Vec::with_capacity(total);
        let mut lr = link_ref;
        while lr != 0 && out.len() < total {
            let tbl = self.normal(DFTAG_LINKED, lr)?;
            let next = be_u16(tbl, 0)?;
            let mut i = 2usize;
            for _ in 0..nblocks {
                if out.len() >= total {
                    break;
                }
                let br = be_u16(tbl, i)?;
                i += 2;
                if br == 0 {
                    continue;
                }
                let blk = self.normal(DFTAG_LINKED, br)?;
                let need = total - out.len();
                out.extend_from_slice(&blk[..blk.len().min(need)]);
            }
            lr = next;
        }
        if out.len() != total {
            return None;
        }
        Some(out)
    }

    pub fn vheader(&self, ref_: u16) -> Option<VHeader> {
        let buf = self.normal(DFTAG_VH, ref_)?;
        let interlace = be_i16(buf, 0)?;
        let nrecords = be_i32(buf, 2)?;
        let ivsize = be_u16(buf, 6)?;
        let nf = be_i16(buf, 8)?;
        if nf < 0 || nrecords < 0 {
            return None;
        }
        let nf = nf as usize;
        let mut q = 10usize;
        let mut types = Vec::with_capacity(nf);
        for _ in 0..nf {
            types.push(be_i16(buf, q)?);
            q += 2;
        }
        let mut isizes = Vec::with_capacity(nf);
        for _ in 0..nf {
            isizes.push(be_u16(buf, q)?);
            q += 2;
        }
        let mut offs = Vec::with_capacity(nf);
        for _ in 0..nf {
            offs.push(be_u16(buf, q)?);
            q += 2;
        }
        let mut orders = Vec::with_capacity(nf);
        for _ in 0..nf {
            orders.push(be_u16(buf, q)?);
            q += 2;
        }
        let mut fields = Vec::with_capacity(nf);
        for i in 0..nf {
            let nl = be_i16(buf, q)?;
            if nl < 0 {
                return None;
            }
            let nl = nl as usize;
            let name = String::from_utf8_lossy(buf.get(q + 2..q + 2 + nl)?)
                .trim_end_matches('\0')
                .to_string();
            q += 2 + nl;
            fields.push(VField {
                name,
                typ: types[i],
                isize: isizes[i],
                off: offs[i],
                order: orders[i],
            });
        }
        let nl = be_i16(buf, q)?;
        if nl < 0 {
            return None;
        }
        let nl = nl as usize;
        let vsname = String::from_utf8_lossy(buf.get(q + 2..q + 2 + nl)?)
            .trim_end_matches('\0')
            .to_string();
        q += 2 + nl;
        let cl = be_i16(buf, q)?;
        if cl < 0 {
            return None;
        }
        let cl = cl as usize;
        let vsclass = String::from_utf8_lossy(buf.get(q + 2..q + 2 + cl)?)
            .trim_end_matches('\0')
            .to_string();
        Some(VHeader {
            interlace,
            nrecords,
            ivsize,
            fields,
            vsname,
            vsclass,
        })
    }

    pub fn vdata(&self, ref_: u16) -> Option<(VHeader, Vec<u8>)> {
        let vh = self.vheader(ref_)?;
        let data = self.element(DFTAG_VS, ref_)?;
        Some((vh, data))
    }
}

pub fn type_size(typ: i16) -> Option<usize> {
    match typ {
        DFNT_CHAR8 | DFNT_UCHAR8 | DFNT_INT8 | DFNT_UINT8 => Some(1),
        DFNT_INT16 | DFNT_UINT16 => Some(2),
        DFNT_INT32 | DFNT_UINT32 | DFNT_FLOAT32 => Some(4),
        DFNT_INT64 | DFNT_UINT64 | DFNT_FLOAT64 => Some(8),
        _ => None,
    }
}

pub fn read_num(b: &[u8], o: usize, typ: i16) -> Option<f64> {
    match typ {
        DFNT_INT8 => Some(*b.get(o)? as i8 as f64),
        DFNT_UINT8 | DFNT_CHAR8 | DFNT_UCHAR8 => Some(*b.get(o)? as f64),
        DFNT_INT16 => Some(be_i16(b, o)? as f64),
        DFNT_UINT16 => Some(be_u16(b, o)? as f64),
        DFNT_INT32 => Some(be_i32(b, o)? as f64),
        DFNT_UINT32 => Some(be_u32(b, o)? as f64),
        DFNT_INT64 => Some(be_u64(b, o)? as i64 as f64),
        DFNT_UINT64 => Some(be_u64(b, o)? as f64),
        DFNT_FLOAT32 => Some(f32::from_bits(be_u32(b, o)?) as f64),
        DFNT_FLOAT64 => Some(f64::from_bits(be_u64(b, o)?)),
        _ => None,
    }
}

pub fn field_values(vh: &VHeader, data: &[u8], rec: usize, name: &str) -> Option<Vec<f64>> {
    let f = vh.fields.iter().find(|f| f.name == name)?;
    let base = rec.checked_mul(vh.ivsize as usize)?;
    let off = base.checked_add(f.off as usize)?;
    let size = type_size(f.typ)?;
    let mut out = Vec::with_capacity(f.order as usize);
    for k in 0..f.order as usize {
        out.push(read_num(data, off + k * size, f.typ)?);
    }
    Some(out)
}

pub fn field_value(vh: &VHeader, data: &[u8], rec: usize, name: &str) -> Option<f64> {
    let v = field_values(vh, data, rec, name)?;
    let first = *v.first()?;
    Some(first)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vh_bytes() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0i16.to_be_bytes());
        b.extend_from_slice(&1i32.to_be_bytes());
        b.extend_from_slice(&16u16.to_be_bytes());
        b.extend_from_slice(&2i16.to_be_bytes());
        for t in [DFNT_INT32, DFNT_FLOAT32] {
            b.extend_from_slice(&t.to_be_bytes());
        }
        for s in [4u16, 4] {
            b.extend_from_slice(&s.to_be_bytes());
        }
        for o in [0u16, 4] {
            b.extend_from_slice(&o.to_be_bytes());
        }
        for o in [1u16, 1] {
            b.extend_from_slice(&o.to_be_bytes());
        }
        for n in ["seq", "rad"] {
            b.extend_from_slice(&(n.len() as i16).to_be_bytes());
            b.extend_from_slice(n.as_bytes());
        }
        b.extend_from_slice(&5i16.to_be_bytes());
        b.extend_from_slice(b"flash");
        b.extend_from_slice(&10i16.to_be_bytes());
        b.extend_from_slice(b"statistics");
        b
    }

    #[test]
    fn vheader_decodes() {
        let body = vh_bytes();
        let data_offset: i32 = 4 + 6 + 12;
        let mut file = Vec::new();
        file.extend_from_slice(&MAGIC);
        file.extend_from_slice(&1u16.to_be_bytes());
        file.extend_from_slice(&0i32.to_be_bytes());
        file.extend_from_slice(&DFTAG_VH.to_be_bytes());
        file.extend_from_slice(&1u16.to_be_bytes());
        file.extend_from_slice(&data_offset.to_be_bytes());
        file.extend_from_slice(&(body.len() as i32).to_be_bytes());
        file.extend_from_slice(&body);
        let hdf = Hdf4::parse(&file).expect("the synthetic file parses");
        let vh = hdf.vheader(1).expect("the VH decodes");
        assert_eq!(vh.nrecords, 1);
        assert_eq!(vh.ivsize, 16);
        assert_eq!(vh.vsname, "flash");
        assert_eq!(vh.vsclass, "statistics");
        assert_eq!(vh.fields.len(), 2);
        assert_eq!(vh.fields[0].name, "seq");
        assert_eq!(vh.fields[1].name, "rad");
    }

    #[test]
    fn a_bad_magic_is_absent() {
        assert!(Hdf4::parse(&[0u8; 8]).is_none());
    }

    #[test]
    fn linked_element_assembles() {
        let mut file = Vec::new();
        file.extend_from_slice(&MAGIC);
        file.extend_from_slice(&3u16.to_be_bytes());
        file.extend_from_slice(&0i32.to_be_bytes());
        let special_off = 4 + 6 + 3 * 12;
        let table_off = special_off + 16;
        let block_off = table_off + 4;
        let mut dd = |tag: u16, ref_: u16, offset: i32, length: i32| {
            file.extend_from_slice(&tag.to_be_bytes());
            file.extend_from_slice(&ref_.to_be_bytes());
            file.extend_from_slice(&offset.to_be_bytes());
            file.extend_from_slice(&length.to_be_bytes());
        };
        dd(DFTAG_VS | SPECIAL, 1, special_off, 16);
        dd(DFTAG_LINKED, 100, table_off, 4);
        dd(DFTAG_LINKED, 101, block_off, 5);
        file.extend_from_slice(&[0u8, 0]);
        file.extend_from_slice(&5i32.to_be_bytes());
        file.extend_from_slice(&5i32.to_be_bytes());
        file.extend_from_slice(&1i32.to_be_bytes());
        file.extend_from_slice(&100u16.to_be_bytes());
        file.extend_from_slice(&0u16.to_be_bytes());
        file.extend_from_slice(&101u16.to_be_bytes());
        file.extend_from_slice(b"hello");
        let hdf = Hdf4::parse(&file).expect("the synthetic linked file parses");
        assert_eq!(hdf.element(DFTAG_VS, 1).as_deref(), Some(&b"hello"[..]));
    }
}
