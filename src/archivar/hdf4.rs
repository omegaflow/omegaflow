use std::collections::HashMap;

pub const DFTAG_LINKED: u16 = 20;
pub const DFTAG_SD: u16 = 702;
pub const DFTAG_VH: u16 = 1962;
pub const DFTAG_VS: u16 = 1963;
pub const DFTAG_SDD: u16 = 701;
pub const DFTAG_NT: u16 = 106;
pub const DFTAG_NDG: u16 = 720;
pub const DFTAG_VG: u16 = 1965;
pub const DFTAG_CHUNK: u16 = 61;
pub const DFTAG_COMPRESSED: u16 = 40;

const SPECIAL: u16 = 0x4000;
const MAGIC: [u8; 4] = [0x0e, 0x03, 0x13, 0x01];

const SP_LINKED: u16 = 1;
const SP_COMP: u16 = 3;
const SP_CHUNKED: u16 = 5;

const CODER_NONE: u16 = 0;
const CODER_RLE: u16 = 1;
const CODER_NBIT: u16 = 2;
const CODER_SKPHUFF: u16 = 3;
const CODER_DEFLATE: u16 = 4;
const CODER_SZIP: u16 = 5;

const SKPHUFF_MAX_CHAR: usize = 255;
const SUCCMAX: usize = 256;
const TWICEMAX: usize = 513;
const ROOT: usize = 0;

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

type VGroup = (Vec<(u16, u16)>, String, String);

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
                if length < 0 {
                    p += 12;
                    continue;
                }
                if offset < 0 || offset as u64 + length as u64 > i32::MAX as u64 {
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

    fn special_element(&self, tag: u16, ref_: u16) -> Option<Vec<u8>> {
        let dd = self.dd(tag | SPECIAL, ref_)?;
        let end = dd.offset.checked_add(dd.length)?;
        let raw = self.bytes.get(dd.offset..end)?;
        match be_u16(raw, 0)? {
            SP_LINKED => self.linked(dd),
            SP_CHUNKED => self.chunked_data(tag, ref_),
            SP_COMP => self.compressed_element(raw),
            _ => Some(raw.to_vec()),
        }
    }

    fn compressed_element(&self, raw: &[u8]) -> Option<Vec<u8>> {
        if be_u16(raw, 0)? != SP_COMP {
            return None;
        }
        let length = be_i32(raw, 4)?;
        let comp_ref = be_u16(raw, 8)?;
        if length < 0 {
            return None;
        }
        let header = raw.get(10..)?;
        let stream = self.element_bytes(DFTAG_COMPRESSED, comp_ref)?;
        decode_coded(header, &stream, length as usize)
    }

    fn element_bytes(&self, tag: u16, ref_: u16) -> Option<Vec<u8>> {
        if let Some(slice) = self.normal(tag, ref_) {
            return Some(slice.to_vec());
        }
        self.special_element(tag, ref_)
    }

    fn vgroup(&self, ref_: u16) -> Option<VGroup> {
        let buf = self.normal(DFTAG_VG, ref_)?;
        let n = be_u16(buf, 0)? as usize;
        let mut p = 2usize;
        let mut tags = Vec::with_capacity(n);
        for _ in 0..n {
            tags.push(be_u16(buf, p)?);
            p += 2;
        }
        let mut refs = Vec::with_capacity(n);
        for _ in 0..n {
            refs.push(be_u16(buf, p)?);
            p += 2;
        }
        let nl = be_u16(buf, p)? as usize;
        let name = String::from_utf8_lossy(buf.get(p + 2..p + 2 + nl)?)
            .trim_end_matches('\0')
            .to_string();
        p += 2 + nl;
        let cl = be_u16(buf, p)? as usize;
        let class = String::from_utf8_lossy(buf.get(p + 2..p + 2 + cl)?)
            .trim_end_matches('\0')
            .to_string();
        let members = tags.into_iter().zip(refs).collect();
        Some((members, name, class))
    }

    fn ndg_members(&self, ref_: u16) -> Option<Vec<(u16, u16)>> {
        let buf = self.normal(DFTAG_NDG, ref_)?;
        let mut out = Vec::with_capacity(buf.len() / 4);
        let mut p = 0usize;
        while p + 4 <= buf.len() {
            out.push((be_u16(buf, p)?, be_u16(buf, p + 2)?));
            p += 4;
        }
        Some(out)
    }

    fn sdd_dims(&self, ref_: u16) -> Option<Vec<i32>> {
        let buf = self.normal(DFTAG_SDD, ref_)?;
        let rank = be_u16(buf, 0)? as usize;
        if rank > 32 {
            return None;
        }
        let mut dims = Vec::with_capacity(rank);
        let mut p = 2usize;
        for _ in 0..rank {
            dims.push(be_i32(buf, p)?);
            p += 4;
        }
        Some(dims)
    }

    fn nt_type(&self, ref_: u16) -> Option<i16> {
        let buf = self.normal(DFTAG_NT, ref_)?;
        Some(*buf.get(1)? as i16)
    }

    fn vs_attr_value(&self, ref_: u16, vh: &VHeader, idx: usize) -> Option<f64> {
        let f = vh.fields.first()?;
        let size = type_size(f.typ)?;
        let data = self.element_bytes(DFTAG_VS, ref_)?;
        read_num(&data, f.off as usize + idx * size, f.typ)
    }

    fn chunked_data(&self, tag: u16, ref_: u16) -> Option<Vec<u8>> {
        let dd = self.dd(tag | SPECIAL, ref_)?;
        let end = dd.offset.checked_add(dd.length)?;
        let raw = self.bytes.get(dd.offset..end)?;
        if be_u16(raw, 0)? != SP_CHUNKED {
            return None;
        }
        let head_len = be_i32(raw, 2)? as usize;
        let head_end = 6usize.checked_add(head_len)?;
        let head = raw.get(6..head_end)?;
        let total = be_i32(head, 5)?;
        let chunk_size = be_i32(head, 9)?;
        let nt_size = be_i32(head, 13)?;
        let tbl_tag = be_u16(head, 17)?;
        let tbl_ref = be_u16(head, 19)?;
        let ndims = be_i32(head, 25)? as usize;
        if ndims == 0
            || ndims > 32
            || total <= 0
            || chunk_size <= 0
            || nt_size <= 0
            || tbl_tag != DFTAG_VH
        {
            return None;
        }
        let mut dims = Vec::with_capacity(ndims);
        let mut q = 29usize;
        for _ in 0..ndims {
            let dl = be_i32(head, q + 4)?;
            let cl = be_i32(head, q + 8)?;
            if dl <= 0 || cl <= 0 {
                return None;
            }
            dims.push(dl);
            q += 12;
        }
        let tail = raw.get(head_end..)?;
        if be_u16(tail, 0)? != SP_COMP {
            return None;
        }
        let comp_len = be_i32(tail, 2)? as usize;
        let comp = tail.get(6..6 + comp_len)?;
        let vh = self.vheader(tbl_ref)?;
        let nrec = vh.nrecords as usize;
        let ivsize = vh.ivsize as usize;
        if ivsize < 4 {
            return None;
        }
        let vs = self.element_bytes(DFTAG_VS, tbl_ref)?;
        if vs.len() < nrec * ivsize {
            return None;
        }
        let want = (chunk_size as usize).checked_mul(nt_size as usize)?;
        let total_bytes = (total as usize).checked_mul(nt_size as usize)?;
        let mut out = vec![0u8; total_bytes];
        let mut strides = vec![1u64; ndims];
        for d in (0..ndims - 1).rev() {
            strides[d] = strides[d + 1] * dims[d + 1] as u64;
        }
        for i in 0..nrec {
            let rec = vs.get(i * ivsize..(i + 1) * ivsize)?;
            let chk_tag = be_u16(rec, ivsize - 4)?;
            let chk_ref = be_u16(rec, ivsize - 2)?;
            let chunk = self.chunk_bytes(chk_tag, chk_ref, comp, want)?;
            let mut idx = 0u64;
            let mut ok = true;
            for (d, &stride) in strides.iter().enumerate() {
                let origin = be_i32(rec, d * 4)?;
                if origin < 0 {
                    ok = false;
                    break;
                }
                idx += origin as u64 * stride;
            }
            if !ok {
                continue;
            }
            let start = (idx as usize).checked_mul(nt_size as usize)?;
            let dst = out.get_mut(start..)?;
            for (d, s) in dst.iter_mut().zip(chunk.iter()) {
                *d = *s;
            }
        }
        Some(out)
    }

    fn chunk_bytes(
        &self,
        chk_tag: u16,
        chk_ref: u16,
        header: &[u8],
        want: usize,
    ) -> Option<Vec<u8>> {
        if let Some(slice) = self.normal(chk_tag, chk_ref) {
            return decode_coded(header, slice, want);
        }
        let dd = self.dd(chk_tag | SPECIAL, chk_ref)?;
        let end = dd.offset.checked_add(dd.length)?;
        let raw = self.bytes.get(dd.offset..end)?;
        if be_u16(raw, 0)? != SP_COMP {
            return None;
        }
        let comp_ref = be_u16(raw, 8)?;
        let stream = self.element_bytes(DFTAG_COMPRESSED, comp_ref)?;
        decode_coded(header, &stream, want)
    }

    pub fn sds(&self) -> Vec<Sds> {
        let mut names: HashMap<u16, String> = HashMap::new();
        let mut attrs: HashMap<u16, Vec<u16>> = HashMap::new();
        for dd in &self.dds {
            if dd.tag != DFTAG_VG {
                continue;
            }
            let Some((members, name, class)) = self.vgroup(dd.ref_) else {
                continue;
            };
            if !class.starts_with("Var0.") {
                continue;
            }
            let mut ndg = None;
            let mut vhs = Vec::new();
            for &(t, r) in &members {
                match t {
                    DFTAG_NDG => ndg = Some(r),
                    DFTAG_VH => vhs.push(r),
                    _ => {}
                }
            }
            if let Some(nr) = ndg {
                names.insert(nr, name.clone());
                attrs.insert(nr, vhs);
            }
        }
        let mut out = Vec::new();
        for dd in &self.dds {
            if dd.tag != DFTAG_NDG {
                continue;
            }
            let Some(members) = self.ndg_members(dd.ref_) else {
                continue;
            };
            let mut data_ref = None;
            let mut nt_ref = None;
            let mut sdd_ref = None;
            for &(t, r) in &members {
                match t {
                    DFTAG_SD => data_ref = Some(r),
                    DFTAG_NT => nt_ref = Some(r),
                    DFTAG_SDD => sdd_ref = Some(r),
                    _ => {}
                }
            }
            let (Some(data_ref), Some(nt_ref), Some(sdd_ref)) = (data_ref, nt_ref, sdd_ref) else {
                continue;
            };
            let Some(typ) = self.nt_type(nt_ref) else {
                continue;
            };
            let Some(dims) = self.sdd_dims(sdd_ref) else {
                continue;
            };
            let Some(data) = self.element_bytes(DFTAG_SD, data_ref) else {
                continue;
            };
            let mut scale = None;
            let mut offset = None;
            let mut fill = None;
            let mut range = None;
            if let Some(vhs) = attrs.get(&dd.ref_) {
                for &vr in vhs {
                    let Some(vh) = self.vheader(vr) else {
                        continue;
                    };
                    match vh.vsname.as_str() {
                        "scale_factor" => scale = self.vs_attr_value(vr, &vh, 0),
                        "add_offset" => offset = self.vs_attr_value(vr, &vh, 0),
                        "_FillValue" => fill = self.vs_attr_value(vr, &vh, 0),
                        "valid_range" => {
                            let lo = self.vs_attr_value(vr, &vh, 0);
                            let hi = self.vs_attr_value(vr, &vh, 1);
                            if let (Some(lo), Some(hi)) = (lo, hi) {
                                range = Some((lo, hi));
                            }
                        }
                        _ => {}
                    }
                }
            }
            out.push(Sds {
                name: names.get(&dd.ref_).cloned(),
                typ,
                dims,
                data,
                scale,
                offset,
                fill,
                range,
            });
        }
        out
    }
}

pub struct Sds {
    pub name: Option<String>,
    pub typ: i16,
    pub dims: Vec<i32>,
    pub data: Vec<u8>,
    pub scale: Option<f64>,
    pub offset: Option<f64>,
    pub fill: Option<f64>,
    pub range: Option<(f64, f64)>,
}

fn decode_coded(header: &[u8], stream: &[u8], want: usize) -> Option<Vec<u8>> {
    match be_u16(header, 2)? {
        CODER_NONE => Some(stream.to_vec()),
        CODER_RLE => rle_decode(stream, want),
        CODER_DEFLATE => zlib_inflate(stream, want),
        CODER_NBIT => {
            let typ = i16::try_from(be_i32(header, 4)?).ok()?;
            let sign_ext = be_u16(header, 8)? != 0;
            let fill_one = be_u16(header, 10)? != 0;
            let start_bit = be_i32(header, 12)?;
            let bit_len = be_i32(header, 16)?;
            nbit_decode(stream, want, typ, sign_ext, fill_one, start_bit, bit_len)
        }
        CODER_SKPHUFF => {
            let skip_size = be_u32(header, 4)? as usize;
            skphuff_decode(stream, want, skip_size)
        }
        CODER_SZIP => None,
        _ => None,
    }
}

struct BitReader<'a> {
    data: &'a [u8],
    byte: usize,
    bit: u8,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        BitReader {
            data,
            byte: 0,
            bit: 0,
        }
    }

    fn read(&mut self, n: usize) -> Option<u32> {
        if n > 32 {
            return None;
        }
        let mut v = 0u32;
        for _ in 0..n {
            let byte = *self.data.get(self.byte)?;
            let b = ((byte >> (7 - self.bit)) & 1) as u32;
            v = (v << 1) | b;
            self.bit += 1;
            if self.bit == 8 {
                self.bit = 0;
                self.byte += 1;
            }
        }
        Some(v)
    }
}

fn mask8(n: usize) -> u8 {
    if n >= 8 {
        0xFF
    } else {
        ((1u16 << n) - 1) as u8
    }
}

fn mask32(n: usize) -> u32 {
    if n >= 32 { u32::MAX } else { (1u32 << n) - 1 }
}

fn nbit_decode(
    stream: &[u8],
    want: usize,
    typ: i16,
    sign_ext: bool,
    fill_one: bool,
    mask_off: i32,
    mask_len: i32,
) -> Option<Vec<u8>> {
    let nt_size = type_size(typ)?;
    if !(1..=8).contains(&nt_size) || !want.is_multiple_of(nt_size) || mask_len < 1 {
        return None;
    }
    let bits = (nt_size * 8) as i32;
    if !(0..bits).contains(&mask_off) {
        return None;
    }
    let mask_top = mask_off;
    let mask_bot = mask_off - (mask_len - 1);
    if mask_bot < 0 {
        return None;
    }
    let mut info = vec![(0i32, 0usize, 0u8); nt_size];
    let mut top_bit = bits - 1;
    let mut bot_bit = bits - 8;
    for slot in info.iter_mut() {
        if mask_top >= top_bit {
            if mask_bot <= bot_bit {
                *slot = (7, 8, 0xFF);
            } else {
                let len = (top_bit - mask_bot + 1) as usize;
                *slot = (7, len, mask8(len) << (8 - len));
                break;
            }
        } else if mask_top >= bot_bit {
            if mask_bot < bot_bit {
                let len = (mask_top - bot_bit + 1) as usize;
                *slot = (mask_top - bot_bit, len, mask8(len));
            } else {
                let len = (mask_top - mask_bot + 1) as usize;
                *slot = (mask_top - bot_bit, len, mask8(len) << (mask_bot - bot_bit));
                break;
            }
        }
        top_bit -= 8;
        bot_bit -= 8;
    }
    let mut mask_buf = vec![if fill_one { 0xFFu8 } else { 0u8 }; nt_size];
    if fill_one {
        for (i, s) in info.iter().enumerate() {
            mask_buf[i] &= !s.2;
        }
    }
    let sign_ext_mask = !(mask32((mask_off % 8) as usize)) as u8;
    let sign_byte = nt_size as i32 - ((mask_off / 8) + 1);
    let sign_mask = mask32(((mask_off % 8) + 1) as usize) ^ mask32((mask_off % 8) as usize);
    let mut reader = BitReader::new(stream);
    let mut out = vec![0u8; want];
    for item in 0..want / nt_size {
        let base = item * nt_size;
        out[base..base + nt_size].copy_from_slice(&mask_buf);
        if sign_ext {
            let mut sign_bit = 0u32;
            for (j, &(offset, len, mask)) in info.iter().enumerate() {
                if len > 0 {
                    let input = reader.read(len)?;
                    let shift = (offset - len as i32) + 1;
                    if shift >= 0 {
                        out[base + j] |= mask & ((input << shift as u32) as u8);
                    }
                    if j as i32 == sign_byte && (sign_mask & input) != 0 {
                        sign_bit = 1;
                    }
                }
            }
            if sign_bit != (fill_one as u32) && sign_byte >= 0 {
                let sb = sign_byte as usize;
                for b in out[base..base + sb].iter_mut() {
                    *b = if sign_bit == 1 { 0xFF } else { 0x00 };
                }
                if sb < nt_size {
                    if sign_bit == 1 {
                        out[base + sb] |= sign_ext_mask;
                    } else {
                        out[base + sb] &= !sign_ext_mask;
                    }
                }
            }
        } else {
            for (j, &(offset, len, mask)) in info.iter().enumerate() {
                if len > 0 {
                    let input = reader.read(len)?;
                    let shift = (offset - len as i32) + 1;
                    if shift >= 0 {
                        out[base + j] |= mask & ((input << shift as u32) as u8);
                    }
                }
            }
        }
    }
    Some(out)
}

struct SplayTree {
    left: Vec<usize>,
    right: Vec<usize>,
    up: Vec<usize>,
}

impl SplayTree {
    fn new() -> Self {
        SplayTree {
            left: (0..SUCCMAX).map(|j| j << 1).collect(),
            right: (0..SUCCMAX).map(|j| (j << 1) + 1).collect(),
            up: (0..TWICEMAX).map(|i| i >> 1).collect(),
        }
    }

    fn splay(&mut self, plain: u8) {
        let mut a = plain as usize + SUCCMAX;
        loop {
            let c = self.up[a];
            if c != ROOT {
                let d = self.up[c];
                let mut b = self.left[d];
                if c == b {
                    b = self.right[d];
                    self.right[d] = a;
                } else {
                    self.left[d] = a;
                }
                if a == self.left[c] {
                    self.left[c] = b;
                } else {
                    self.right[c] = b;
                }
                self.up[a] = d;
                self.up[b] = c;
                a = d;
            } else {
                a = c;
            }
            if a == ROOT {
                break;
            }
        }
    }
}

fn skphuff_decode(stream: &[u8], want: usize, skip_size: usize) -> Option<Vec<u8>> {
    if !(1..=SUCCMAX).contains(&skip_size) {
        return None;
    }
    let mut trees: Vec<SplayTree> = (0..skip_size).map(|_| SplayTree::new()).collect();
    let mut reader = BitReader::new(stream);
    let mut out = Vec::with_capacity(want);
    let mut skip_pos = 0usize;
    for _ in 0..want {
        let tree = &mut trees[skip_pos];
        let mut a = ROOT;
        loop {
            let bit = reader.read(1)?;
            a = if bit == 0 {
                tree.left[a]
            } else {
                tree.right[a]
            };
            if a > SKPHUFF_MAX_CHAR {
                break;
            }
        }
        let plain = (a - SUCCMAX) as u8;
        tree.splay(plain);
        out.push(plain);
        skip_pos = (skip_pos + 1) % skip_size;
    }
    Some(out)
}

fn rle_decode(stream: &[u8], want: usize) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(want);
    let mut p = 0usize;
    while out.len() < want {
        let c = *stream.get(p)?;
        p += 1;
        if c & 0x80 != 0 {
            let n = (c & 0x7f) as usize + 3;
            let v = *stream.get(p)?;
            p += 1;
            for _ in 0..n {
                if out.len() >= want {
                    break;
                }
                out.push(v);
            }
        } else {
            let n = (c & 0x7f) as usize + 1;
            for _ in 0..n {
                if out.len() >= want {
                    break;
                }
                out.push(*stream.get(p)?);
                p += 1;
            }
        }
    }
    Some(out)
}

fn zlib_inflate(stream: &[u8], want: usize) -> Option<Vec<u8>> {
    let body = stream.get(2..)?;
    let out = crate::archivar::inflate::inflate(body)?;
    if out.len() != want {
        return None;
    }
    let tail = stream.len().checked_sub(4)?;
    if adler32(&out) != be_u32(stream, tail)? {
        return None;
    }
    Some(out)
}

fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &x in data {
        a = (a + x as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
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

    fn wrap(tag: u16, ref_: u16, off: i32, len: i32) -> [u8; 12] {
        let mut e = [0u8; 12];
        e[0..2].copy_from_slice(&tag.to_be_bytes());
        e[2..4].copy_from_slice(&ref_.to_be_bytes());
        e[4..8].copy_from_slice(&off.to_be_bytes());
        e[8..12].copy_from_slice(&len.to_be_bytes());
        e
    }

    fn zlib_stored(payload: &[u8]) -> Vec<u8> {
        let mut s = Vec::new();
        s.extend_from_slice(&[0x78, 0x01, 0x01]);
        s.extend_from_slice(&(payload.len() as u16).to_le_bytes());
        s.extend_from_slice(&(!(payload.len() as u16)).to_le_bytes());
        s.extend_from_slice(payload);
        s.extend_from_slice(&adler32(payload).to_be_bytes());
        s
    }

    #[test]
    fn deleted_dd_slots_are_skipped() {
        let data_off = 4 + 6 + 2 * 12;
        let mut file = Vec::new();
        file.extend_from_slice(&MAGIC);
        file.extend_from_slice(&2u16.to_be_bytes());
        file.extend_from_slice(&0i32.to_be_bytes());
        file.extend_from_slice(&wrap(DFTAG_NT, 1, data_off, 4));
        file.extend_from_slice(&wrap(DFTAG_VS, 2, -1, -1));
        file.extend_from_slice(&[1u8, 23, 16, 1]);
        let hdf = Hdf4::parse(&file).expect("the chain parses past the deleted slot");
        assert_eq!(hdf.dds().len(), 1);
        assert!(hdf.dd(DFTAG_VS, 2).is_none());
        assert_eq!(hdf.nt_type(1).expect("the NT decodes"), 23);
    }

    #[test]
    fn zlib_inflate_decodes_a_stored_block() {
        let stream = [
            0x78, 0x01, 0x01, 0x05, 0x00, 0xFA, 0xFF, b'h', b'e', b'l', b'l', b'o', 0x06, 0x2C,
            0x02, 0x15,
        ];
        assert_eq!(zlib_inflate(&stream, 5).as_deref(), Some(&b"hello"[..]));
        assert!(zlib_inflate(&stream, 6).is_none());
        assert!(zlib_inflate(&stream[..stream.len() - 1], 5).is_none());
    }

    #[test]
    fn rle_decode_expands_runs_and_mixes() {
        assert_eq!(
            rle_decode(&[0x80, 0x41, 0x01, 0x42, 0x43], 5).as_deref(),
            Some(&b"AAABC"[..])
        );
        assert!(rle_decode(&[0x80, 0x41], 5).is_none());
    }

    #[test]
    fn chunked_sds_assembles() {
        let mut bodies: Vec<Vec<u8>> = Vec::new();
        let mut entries: Vec<(u16, u16, usize)> = Vec::new();
        let mut place = |tag: u16, ref_: u16, body: Vec<u8>| {
            entries.push((tag, ref_, bodies.len()));
            bodies.push(body);
        };
        let mut ndg = Vec::new();
        for (t, r) in [(DFTAG_SD, 1u16), (DFTAG_NT, 2), (DFTAG_SDD, 3), (0x02d1, 3)] {
            ndg.extend_from_slice(&t.to_be_bytes());
            ndg.extend_from_slice(&r.to_be_bytes());
        }
        place(DFTAG_NDG, 10, ndg);
        place(DFTAG_NT, 2, vec![1, 23, 16, 1]);
        let mut sdd = Vec::new();
        sdd.extend_from_slice(&2u16.to_be_bytes());
        for d in [4i32, 3] {
            sdd.extend_from_slice(&d.to_be_bytes());
        }
        for _ in 0..3 {
            sdd.extend_from_slice(&DFTAG_NT.to_be_bytes());
            sdd.extend_from_slice(&2u16.to_be_bytes());
        }
        place(DFTAG_SDD, 3, sdd);
        let mut rec = Vec::new();
        rec.extend_from_slice(&SP_CHUNKED.to_be_bytes());
        rec.extend_from_slice(&59i32.to_be_bytes());
        rec.push(0);
        rec.extend_from_slice(&(SP_COMP as i32).to_be_bytes());
        rec.extend_from_slice(&12i32.to_be_bytes());
        rec.extend_from_slice(&6i32.to_be_bytes());
        rec.extend_from_slice(&2i32.to_be_bytes());
        rec.extend_from_slice(&DFTAG_VH.to_be_bytes());
        rec.extend_from_slice(&4u16.to_be_bytes());
        rec.extend_from_slice(&1u16.to_be_bytes());
        rec.extend_from_slice(&0u16.to_be_bytes());
        rec.extend_from_slice(&2i32.to_be_bytes());
        for (f, d, c) in [(1i32, 4i32, 2i32), (0i32, 3i32, 3i32)] {
            rec.extend_from_slice(&f.to_be_bytes());
            rec.extend_from_slice(&d.to_be_bytes());
            rec.extend_from_slice(&c.to_be_bytes());
        }
        rec.extend_from_slice(&2i32.to_be_bytes());
        rec.extend_from_slice(&0u16.to_be_bytes());
        rec.extend_from_slice(&SP_COMP.to_be_bytes());
        rec.extend_from_slice(&6i32.to_be_bytes());
        rec.extend_from_slice(&0u16.to_be_bytes());
        rec.extend_from_slice(&CODER_DEFLATE.to_be_bytes());
        rec.extend_from_slice(&4u16.to_be_bytes());
        place(DFTAG_SD | SPECIAL, 1, rec);
        let mut vh = Vec::new();
        vh.extend_from_slice(&0i16.to_be_bytes());
        vh.extend_from_slice(&2i32.to_be_bytes());
        vh.extend_from_slice(&12u16.to_be_bytes());
        vh.extend_from_slice(&3i16.to_be_bytes());
        for t in [DFNT_INT32, DFNT_UINT16, DFNT_UINT16] {
            vh.extend_from_slice(&t.to_be_bytes());
        }
        for s in [8u16, 2, 2] {
            vh.extend_from_slice(&s.to_be_bytes());
        }
        for o in [0u16, 8, 10] {
            vh.extend_from_slice(&o.to_be_bytes());
        }
        for o in [2u16, 1, 1] {
            vh.extend_from_slice(&o.to_be_bytes());
        }
        for n in ["origin", "chk_tag", "chk_ref"] {
            vh.extend_from_slice(&(n.len() as i16).to_be_bytes());
            vh.extend_from_slice(n.as_bytes());
        }
        vh.extend_from_slice(&3i16.to_be_bytes());
        vh.extend_from_slice(b"tbl");
        vh.extend_from_slice(&3i16.to_be_bytes());
        vh.extend_from_slice(b"cls");
        place(DFTAG_VH, 4, vh);
        let mut vs = Vec::new();
        for (o0, o1, r) in [(0i32, 0i32, 2u16), (2i32, 0i32, 6u16)] {
            vs.extend_from_slice(&o0.to_be_bytes());
            vs.extend_from_slice(&o1.to_be_bytes());
            vs.extend_from_slice(&DFTAG_CHUNK.to_be_bytes());
            vs.extend_from_slice(&r.to_be_bytes());
        }
        place(DFTAG_VS, 4, vs);
        let chunk1: Vec<u8> = (1u16..=6).flat_map(|v| v.to_be_bytes()).collect();
        let chunk2: Vec<u8> = (7u16..=12).flat_map(|v| v.to_be_bytes()).collect();
        let chdr = |comp_ref: u16| {
            let mut h = Vec::new();
            h.extend_from_slice(&SP_COMP.to_be_bytes());
            h.extend_from_slice(&0u16.to_be_bytes());
            h.extend_from_slice(&12i32.to_be_bytes());
            h.extend_from_slice(&comp_ref.to_be_bytes());
            h.extend_from_slice(&0u16.to_be_bytes());
            h.extend_from_slice(&CODER_DEFLATE.to_be_bytes());
            h.extend_from_slice(&4u16.to_be_bytes());
            h
        };
        place(DFTAG_CHUNK | SPECIAL, 2, chdr(3));
        place(DFTAG_COMPRESSED, 3, zlib_stored(&chunk1));
        place(DFTAG_CHUNK | SPECIAL, 6, chdr(7));
        place(DFTAG_COMPRESSED, 7, zlib_stored(&chunk2));
        let mut vg = Vec::new();
        vg.extend_from_slice(&2u16.to_be_bytes());
        for t in [DFTAG_NDG, DFTAG_VH] {
            vg.extend_from_slice(&t.to_be_bytes());
        }
        for r in [10u16, 5] {
            vg.extend_from_slice(&r.to_be_bytes());
        }
        vg.extend_from_slice(&4u16.to_be_bytes());
        vg.extend_from_slice(b"TEST");
        vg.extend_from_slice(&6u16.to_be_bytes());
        vg.extend_from_slice(b"Var0.0");
        place(DFTAG_VG, 20, vg);
        let mut avh = Vec::new();
        avh.extend_from_slice(&0i16.to_be_bytes());
        avh.extend_from_slice(&1i32.to_be_bytes());
        avh.extend_from_slice(&4u16.to_be_bytes());
        avh.extend_from_slice(&1i16.to_be_bytes());
        avh.extend_from_slice(&DFNT_FLOAT32.to_be_bytes());
        avh.extend_from_slice(&4u16.to_be_bytes());
        avh.extend_from_slice(&0u16.to_be_bytes());
        avh.extend_from_slice(&1u16.to_be_bytes());
        avh.extend_from_slice(&6i16.to_be_bytes());
        avh.extend_from_slice(b"VALUES");
        avh.extend_from_slice(&12i16.to_be_bytes());
        avh.extend_from_slice(b"scale_factor");
        avh.extend_from_slice(&7i16.to_be_bytes());
        avh.extend_from_slice(b"Attr0.0");
        place(DFTAG_VH, 5, avh);
        place(DFTAG_VS, 5, 0.02f32.to_bits().to_be_bytes().to_vec());
        let mut file = Vec::new();
        file.extend_from_slice(&MAGIC);
        file.extend_from_slice(&(entries.len() as u16).to_be_bytes());
        file.extend_from_slice(&0i32.to_be_bytes());
        let mut off = 4 + 6 + entries.len() * 12;
        for (tag, ref_, idx) in &entries {
            file.extend_from_slice(&wrap(*tag, *ref_, off as i32, bodies[*idx].len() as i32));
            off += bodies[*idx].len();
        }
        for body in &bodies {
            file.extend_from_slice(body);
        }
        let hdf = Hdf4::parse(&file).expect("the synthetic chunked file parses");
        let list = hdf.sds();
        assert_eq!(list.len(), 1);
        let sds = &list[0];
        assert_eq!(sds.name.as_deref(), Some("TEST"));
        assert_eq!(sds.typ, DFNT_UINT16);
        assert_eq!(sds.dims, vec![4, 3]);
        assert_eq!(sds.data.len(), 24);
        let mut expect = Vec::new();
        for v in 1u16..=12 {
            expect.extend_from_slice(&v.to_be_bytes());
        }
        assert_eq!(sds.data, expect);
        assert!((sds.scale.expect("scale present") - 0.02).abs() < 1e-6);
        assert!(sds.fill.is_none());
    }

    #[test]
    fn nbit_decodes_a_full_width_field() {
        let out = nbit_decode(&[0x12, 0x34], 2, DFNT_UINT16, false, false, 15, 16)
            .expect("the full 16-bit field expands");
        assert_eq!(out, vec![0x12, 0x34]);
    }

    #[test]
    fn nbit_masks_and_fills_the_extracted_bits() {
        assert_eq!(
            nbit_decode(&[0x80], 1, DFNT_UINT8, false, false, 7, 1).as_deref(),
            Some(&[0x80][..])
        );
        assert_eq!(
            nbit_decode(&[0x00], 1, DFNT_UINT8, false, false, 7, 1).as_deref(),
            Some(&[0x00][..])
        );
        assert_eq!(
            nbit_decode(&[0x80], 1, DFNT_UINT8, false, true, 7, 1).as_deref(),
            Some(&[0xFF][..])
        );
        assert_eq!(
            nbit_decode(&[0x00], 1, DFNT_UINT8, false, true, 7, 1).as_deref(),
            Some(&[0x7F][..])
        );
    }

    #[test]
    fn coded_header_routes_nbit() {
        let mut header = Vec::new();
        header.extend_from_slice(&0u16.to_be_bytes());
        header.extend_from_slice(&CODER_NBIT.to_be_bytes());
        header.extend_from_slice(&(DFNT_UINT16 as i32).to_be_bytes());
        header.extend_from_slice(&0u16.to_be_bytes());
        header.extend_from_slice(&0u16.to_be_bytes());
        header.extend_from_slice(&15i32.to_be_bytes());
        header.extend_from_slice(&16i32.to_be_bytes());
        assert_eq!(
            decode_coded(&header, &[0xAB, 0xCD], 2).as_deref(),
            Some(&[0xAB, 0xCD][..])
        );
    }

    #[test]
    fn sp_comp_descriptor_assembles_the_uncompressed_element() {
        let mut body = Vec::new();
        body.extend_from_slice(&SP_COMP.to_be_bytes());
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&2i32.to_be_bytes());
        body.extend_from_slice(&2u16.to_be_bytes());
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&CODER_NBIT.to_be_bytes());
        body.extend_from_slice(&(DFNT_UINT16 as i32).to_be_bytes());
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&15i32.to_be_bytes());
        body.extend_from_slice(&16i32.to_be_bytes());
        let special_off = 4 + 6 + 2 * 12;
        let compressed_off = special_off + body.len();
        let mut file = Vec::new();
        file.extend_from_slice(&MAGIC);
        file.extend_from_slice(&2u16.to_be_bytes());
        file.extend_from_slice(&0i32.to_be_bytes());
        file.extend_from_slice(&wrap(
            DFTAG_SD | SPECIAL,
            1,
            special_off as i32,
            body.len() as i32,
        ));
        file.extend_from_slice(&wrap(DFTAG_COMPRESSED, 2, compressed_off as i32, 2));
        file.extend_from_slice(&body);
        file.extend_from_slice(&[0xAB, 0xCD]);
        let hdf = Hdf4::parse(&file).expect("the synthetic SP_COMP file parses");
        assert_eq!(
            hdf.element_bytes(DFTAG_SD, 1).as_deref(),
            Some(&[0xAB, 0xCD][..])
        );
    }

    fn skphuff_encode(data: &[u8], skip_size: usize) -> Vec<u8> {
        let mut trees: Vec<SplayTree> = (0..skip_size).map(|_| SplayTree::new()).collect();
        let mut bits: Vec<u8> = Vec::new();
        let mut pos = 0usize;
        for &byte in data {
            let tree = &trees[pos];
            let mut path = Vec::new();
            let mut a = byte as usize + SUCCMAX;
            while a != ROOT {
                let parent = tree.up[a];
                path.push(u8::from(tree.right[parent] == a));
                a = parent;
            }
            bits.extend(path.iter().rev());
            trees[pos].splay(byte);
            pos = (pos + 1) % skip_size;
        }
        let mut out = Vec::new();
        let mut acc = 0u8;
        let mut n = 0u8;
        for b in bits {
            acc = (acc << 1) | b;
            n += 1;
            if n == 8 {
                out.push(acc);
                acc = 0;
                n = 0;
            }
        }
        if n > 0 {
            out.push(acc << (8 - n));
        }
        out
    }

    #[test]
    fn skphuff_decodes_the_initial_balanced_code() {
        assert_eq!(
            skphuff_decode(&[0xA0, 0x80], 1, 1).as_deref(),
            Some(&[0x41][..])
        );
    }

    #[test]
    fn skphuff_roundtrips_the_splaying_tree() {
        for skip_size in [1usize, 2, 3] {
            let data = [0x41u8, 0x42, 0x43, 0x41, 0x41, 0x00, 0xFF, 0x42];
            let packed = skphuff_encode(&data, skip_size);
            assert_eq!(
                skphuff_decode(&packed, data.len(), skip_size).as_deref(),
                Some(&data[..]),
                "skip_size {skip_size}"
            );
        }
    }
}
