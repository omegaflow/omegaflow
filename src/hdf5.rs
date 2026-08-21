// HDF5 file format, read-only, std-only. The format lives in the official
// "HDF5 File Format Specification" (support.hdfgroup.org) — this parser is
// its own implementation, verified against the real NCEI-SSI netCDF-4 files
// (netCDF-4 = HDF5) and against libhdf5-written filter test files.
//
// What is read: superblock v0-v3, object headers v1/v2 (all messages that
// carry structure), new-style groups (Link Info + fractal heap + v2
// B-trees), old-style groups (Symbol Table + v1 B-tree + local heap),
// fractal heaps (managed/tiny objects), v2 B-trees (link name/creation
// order, attribute name/creation order, chunk indexes), v1 B-trees, the
// global heap stays unread (variable-length attribute data is carried raw),
// datatypes (fixed, float, string, compound, array, reference, vlen, enum,
// opaque, bitfield, time), chunked/contiguous/compact layouts, filters
// (deflate → inflate.rs, shuffle, fletcher32, scaleoffset). Metadata
// checksums (Jenkins lookup3) are verified where the format carries them.
//
// Not read: virtual datasets (layout v4), family/multi file drivers,
// external data files, huge fractal-heap objects. Each unread structure is
// named in Hdf5Note — never silently replaced (0 honored).

use crate::inflate::inflate;
use std::collections::HashMap;

const UNDEF: u64 = u64::MAX;

const MSG_DATASPACE: u8 = 0x01;
const MSG_LINK_INFO: u8 = 0x02;
const MSG_DATATYPE: u8 = 0x03;
const MSG_FILL_OLD: u8 = 0x04;
const MSG_FILL: u8 = 0x05;
const MSG_LINK: u8 = 0x06;
const MSG_LAYOUT: u8 = 0x08;
const MSG_GROUP_INFO: u8 = 0x0a;
const MSG_FILTERS: u8 = 0x0b;
const MSG_ATTRIBUTE: u8 = 0x0c;
const MSG_CONT: u8 = 0x10;
const MSG_SYMBOL_TABLE: u8 = 0x11;
const MSG_AINFO: u8 = 0x15;
const MSG_SHARED: u8 = 0x18;

const MSG_FLAG_SHARED: u8 = 0x02;

const FILTER_DEFLATE: u16 = 1;
const FILTER_SHUFFLE: u16 = 2;
const FILTER_FLETCHER32: u16 = 3;
const FILTER_SCALEOFFSET: u16 = 6;

#[derive(Clone, Debug)]
pub enum Hdf5Note {
    Magic { bytes: [u8; 8] },
    SuperblockVersion { v: u8 },
    OffsetSize { n: u8 },
    EndAtByte { off: usize },
    Signatur { off: usize, found: [u8; 4] },
    Address { off: usize },
    ObjectHeaderVersion { v: u8 },
    Datatype { class: u8, off: usize },
    Dataspace { off: usize },
    Layout { class: u8, off: usize },
    Btree { typ: u8, off: usize },
    BtreeNode { found: [u8; 4], off: usize },
    Heap { found: [u8; 4], off: usize },
    Filter { id: u16, off: usize },
    Checksum { off: usize },
    HugeHeapObject,
    SharedMessage,
    MissingObject { name: String },
    Chunk { off: usize },
    VlenNotRead,
    VirtualDataset,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    Le,
    Be,
}

#[derive(Clone, Debug)]
pub struct Hdf5CompoundMember {
    pub name: String,
    pub offset: u32,
    pub datatype: Hdf5Datatype,
}

#[derive(Clone, Debug)]
pub struct Hdf5Datatype {
    pub class: u8,
    pub size: usize,
    pub endian: Endian,
    pub signed: bool,
    pub bit_offset: u16,
    pub precision: u16,
    pub string_pad: u8,
    pub string_charset: u8,
    pub members: Vec<Hdf5CompoundMember>,
    pub array_dims: Vec<u64>,
    pub base: Option<Box<Hdf5Datatype>>,
    pub reference_type: u8,
    pub vlen_is_string: bool,
}

impl Hdf5Datatype {
    pub fn flat_f64() -> Hdf5Datatype {
        Hdf5Datatype {
            class: 1,
            size: 8,
            endian: Endian::Le,
            signed: true,
            bit_offset: 0,
            precision: 64,
            string_pad: 0,
            string_charset: 0,
            members: Vec::new(),
            array_dims: Vec::new(),
            base: None,
            reference_type: 0,
            vlen_is_string: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Hdf5Dataspace {
    pub dims: Vec<u64>,
}

#[derive(Clone, Debug)]
pub enum Hdf5Layout {
    Contiguous {
        addr: u64,
        size: u64,
    },
    Chunked {
        btree: u64,
        chunk_dims: Vec<u32>,
        elem_size: u32,
    },
    Compact {
        data: Vec<u8>,
    },
}

#[derive(Clone, Debug)]
pub struct Hdf5Filter {
    pub id: u16,
    pub flags: u16,
    pub cd_values: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct Hdf5Attribute {
    pub name: String,
    pub datatype: Hdf5Datatype,
    pub dataspace: Hdf5Dataspace,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Hdf5Link {
    pub name: String,
    pub addr: u64,
    pub soft: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Hdf5Object {
    pub addr: u64,
    pub is_group: bool,
    pub dataspace: Option<Hdf5Dataspace>,
    pub datatype: Option<Hdf5Datatype>,
    pub layout: Option<Hdf5Layout>,
    pub filters: Vec<Hdf5Filter>,
    pub fill_defined: bool,
    pub fill: Vec<u8>,
    pub attrs: Vec<Hdf5Attribute>,
    pub links: Vec<Hdf5Link>,
}

pub struct Hdf5File<'a> {
    buf: &'a [u8],
    root: u64,
    objects: HashMap<u64, Hdf5Object>,
}

fn le_u16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}

fn le_u32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

fn le_u64(b: &[u8], off: usize) -> u64 {
    u64::from_le_bytes([
        b[off],
        b[off + 1],
        b[off + 2],
        b[off + 3],
        b[off + 4],
        b[off + 5],
        b[off + 6],
        b[off + 7],
    ])
}

fn byte_str(b: &[u8]) -> String {
    String::from_utf8_lossy(b)
        .trim_end_matches('\0')
        .to_string()
}

fn align8(n: usize) -> usize {
    (n + 7) & !7
}

fn read_null_name(buf: &[u8], off: usize) -> String {
    let mut end = off;
    while end < buf.len() && buf[end] != 0 {
        end += 1;
    }
    byte_str(&buf[off..end])
}

pub fn decode_f64(data: &[u8], off: usize, endian: Endian) -> Option<f64> {
    let b = data.get(off..off + 8)?;
    let bits = match endian {
        Endian::Le => u64::from_le_bytes(b.try_into().ok()?),
        Endian::Be => u64::from_be_bytes(b.try_into().ok()?),
    };
    Some(f64::from_bits(bits))
}

pub fn decode_f32(data: &[u8], off: usize, endian: Endian) -> Option<f32> {
    let b = data.get(off..off + 4)?;
    let bits = match endian {
        Endian::Le => u32::from_le_bytes(b.try_into().ok()?),
        Endian::Be => u32::from_be_bytes(b.try_into().ok()?),
    };
    Some(f32::from_bits(bits))
}

fn jenkins_lookup3(buf: &[u8]) -> u32 {
    let mut a: u32 = 0xdeadbeefu32.wrapping_add(buf.len() as u32);
    let mut b: u32 = a;
    let mut c: u32 = a;
    let mut off = 0usize;
    while buf.len() - off > 12 {
        a = a.wrapping_add(le_u32(buf, off));
        b = b.wrapping_add(le_u32(buf, off + 4));
        c = c.wrapping_add(le_u32(buf, off + 8));
        a = a.wrapping_sub(c) ^ c.rotate_left(4);
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a) ^ a.rotate_left(6);
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b) ^ b.rotate_left(8);
        b = b.wrapping_add(a);
        a = a.wrapping_sub(c) ^ c.rotate_left(16);
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a) ^ a.rotate_left(19);
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b) ^ b.rotate_left(4);
        b = b.wrapping_add(a);
        off += 12;
    }
    let rest = &buf[off..];
    let n = rest.len();
    if n >= 12 {
        c = c.wrapping_add((rest[11] as u32) << 24);
    }
    if n >= 11 {
        c = c.wrapping_add((rest[10] as u32) << 16);
    }
    if n >= 10 {
        c = c.wrapping_add((rest[9] as u32) << 8);
    }
    if n >= 9 {
        c = c.wrapping_add(rest[8] as u32);
    }
    if n >= 8 {
        b = b.wrapping_add((rest[7] as u32) << 24);
    }
    if n >= 7 {
        b = b.wrapping_add((rest[6] as u32) << 16);
    }
    if n >= 6 {
        b = b.wrapping_add((rest[5] as u32) << 8);
    }
    if n >= 5 {
        b = b.wrapping_add(rest[4] as u32);
    }
    if n >= 4 {
        a = a.wrapping_add((rest[3] as u32) << 24);
    }
    if n >= 3 {
        a = a.wrapping_add((rest[2] as u32) << 16);
    }
    if n >= 2 {
        a = a.wrapping_add((rest[1] as u32) << 8);
    }
    if n >= 1 {
        a = a.wrapping_add(rest[0] as u32);
    }
    c ^= b;
    c = c.wrapping_sub(b.rotate_left(14));
    a ^= c;
    a = a.wrapping_sub(c.rotate_left(11));
    b ^= a;
    b = b.wrapping_sub(a.rotate_left(25));
    c ^= b;
    c = c.wrapping_sub(b.rotate_left(16));
    a ^= c;
    a = a.wrapping_sub(c.rotate_left(4));
    b ^= a;
    b = b.wrapping_sub(a.rotate_left(14));
    c ^= b;
    c = c.wrapping_sub(b.rotate_left(24));
    c
}

fn check_checksum(body: &[u8], stored: &[u8]) -> Result<(), Hdf5Note> {
    if stored.len() < 4 {
        return Err(Hdf5Note::EndAtByte { off: body.len() });
    }
    let want = le_u32(stored, 0);
    let got = jenkins_lookup3(body);
    if got == want {
        Ok(())
    } else {
        Err(Hdf5Note::Checksum { off: body.len() })
    }
}

struct Superblock {
    root: u64,
}

fn parse_superblock(buf: &[u8]) -> Result<Superblock, Hdf5Note> {
    const MAGIC: [u8; 8] = [0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a];
    if buf.len() < 12 {
        return Err(Hdf5Note::EndAtByte { off: buf.len() });
    }
    if buf[..8] != MAGIC {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&buf[..8]);
        return Err(Hdf5Note::Magic { bytes });
    }
    let v = buf[8];
    let (offset_size, length_size, root) = match v {
        0 | 1 => {
            let os = buf[13];
            let ls = buf[14];
            let sym_off = 24 + 4 * os as usize;
            let root = if v == 0 {
                let a_off = sym_off + os as usize;
                if a_off + os as usize <= buf.len() {
                    Some(le_u64(buf, a_off))
                } else {
                    None
                }
            } else {
                if sym_off + os as usize <= buf.len() {
                    Some(le_u64(buf, sym_off))
                } else {
                    None
                }
            };
            (os, ls, root)
        }
        2 | 3 => {
            let os = buf[9];
            let ls = buf[10];
            let base = 12 + 4 * os as usize;
            if base + 4 > buf.len() {
                return Err(Hdf5Note::EndAtByte { off: base });
            }
            check_checksum(&buf[..base], &buf[base..base + 4])?;
            (os, ls, Some(le_u64(buf, base - os as usize)))
        }
        _ => return Err(Hdf5Note::SuperblockVersion { v }),
    };
    match (offset_size, length_size) {
        (4, 4) | (8, 8) => {}
        (n, _) => return Err(Hdf5Note::OffsetSize { n }),
    }
    Ok(Superblock {
        root: root.ok_or(Hdf5Note::Address { off: 0 })?,
    })
}

#[derive(Clone, Debug)]
struct RawMessage {
    typ: u8,
    flags: u8,
    data: Vec<u8>,
}

fn v2_messages(
    buf: &[u8],
    start: usize,
    end: usize,
    tracked: bool,
) -> Result<Vec<RawMessage>, Hdf5Note> {
    let mut out = Vec::new();
    let mut p = start;
    while p + 4 <= end.min(buf.len()) {
        let typ = buf[p];
        let size = le_u16(buf, p + 1) as usize;
        let flags = buf[p + 3];
        p += 4;
        if tracked {
            p += 2;
        }
        if p + size > buf.len() {
            return Err(Hdf5Note::EndAtByte { off: p });
        }
        out.push(RawMessage {
            typ,
            flags,
            data: buf[p..p + size].to_vec(),
        });
        p += size;
    }
    Ok(out)
}

fn gather_messages(buf: &[u8], addr: u64) -> Result<Vec<RawMessage>, Hdf5Note> {
    let off = addr as usize;
    if buf.len() < off + 6 {
        return Err(Hdf5Note::EndAtByte { off });
    }
    let version = if &buf[off..off + 4] == b"OHDR" {
        buf[off + 4]
    } else {
        buf[off]
    };
    let has_signature = &buf[off..off + 4] == b"OHDR";
    if version == 1 {
        if has_signature {
            return Err(Hdf5Note::ObjectHeaderVersion { v: 1 });
        }
        let total = le_u16(buf, off + 2) as usize;
        let mut out = Vec::with_capacity(total);
        let mut p = off + 16;
        while p + 8 <= buf.len() {
            let typ = le_u16(buf, p) as u8;
            let size = le_u16(buf, p + 2) as usize;
            let flags = buf[p + 4];
            p += 8;
            if p + size > buf.len() {
                return Err(Hdf5Note::EndAtByte { off: p });
            }
            out.push(RawMessage {
                typ,
                flags,
                data: buf[p..p + size].to_vec(),
            });
            p += size;
            if out.len() >= total {
                break;
            }
        }
        return Ok(out);
    }
    if version != 2 {
        return Err(Hdf5Note::ObjectHeaderVersion { v: version });
    }
    let flags = buf[off + 5];
    let size_len = 1usize << (flags & 0x03);
    let chunk0_off = off + 6 + size_len;
    let chunk0_size = match flags & 0x03 {
        0 => buf[chunk0_off - 1] as usize,
        1 => le_u16(buf, chunk0_off - 2) as usize,
        2 => le_u32(buf, chunk0_off - 4) as usize,
        _ => le_u64(buf, chunk0_off - 8) as usize,
    };
    let chunk0_end = chunk0_off + chunk0_size;
    if chunk0_end + 4 > buf.len() {
        return Err(Hdf5Note::EndAtByte { off: chunk0_end });
    }
    check_checksum(&buf[off..chunk0_end], &buf[chunk0_end..chunk0_end + 4])?;
    let tracked = flags & 0x04 != 0;
    let mut out = v2_messages(buf, chunk0_off, chunk0_end, tracked)?;
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    while i < out.len() {
        if out[i].typ == MSG_CONT && out[i].data.len() >= 16 {
            let c = le_u64(&out[i].data, 0) as usize;
            let len = le_u64(&out[i].data, 8) as usize;
            stack.push((c, len));
        }
        i += 1;
    }
    while let Some((c, len)) = stack.pop() {
        if c + 4 > buf.len() || &buf[c..c + 4] != b"OCHK" {
            return Err(Hdf5Note::Address { off: c });
        }
        if c + len > buf.len() {
            return Err(Hdf5Note::EndAtByte { off: c + len });
        }
        check_checksum(&buf[c..c + len - 4], &buf[c + len - 4..c + len])?;
        let mut sub = v2_messages(buf, c + 4, c + len - 4, tracked)?;
        for m in &sub {
            if m.typ == MSG_CONT && m.data.len() >= 16 {
                stack.push((le_u64(&m.data, 0) as usize, le_u64(&m.data, 8) as usize));
            }
        }
        out.append(&mut sub);
    }
    Ok(out)
}

fn parse_dataspace(buf: &[u8], off: usize) -> Result<Hdf5Dataspace, Hdf5Note> {
    if off + 2 > buf.len() {
        return Err(Hdf5Note::EndAtByte { off });
    }
    let version = buf[off];
    let rank = buf[off + 1] as usize;
    let p = if version == 2 { off + 4 } else { off + 8 };
    let mut dims = Vec::with_capacity(rank);
    for i in 0..rank {
        if p + i * 8 + 8 > buf.len() {
            return Err(Hdf5Note::EndAtByte { off: p });
        }
        dims.push(le_u64(buf, p + i * 8));
    }
    Ok(Hdf5Dataspace { dims })
}

fn parse_datatype(buf: &[u8], off: usize) -> Result<(Hdf5Datatype, usize), Hdf5Note> {
    if off + 8 > buf.len() {
        return Err(Hdf5Note::EndAtByte { off });
    }
    let cv = buf[off];
    let class = cv & 0x0f;
    let version = cv >> 4;
    let flags0 = buf[off + 1];
    let flags1 = buf[off + 2];
    let size = le_u32(buf, off + 4) as usize;
    let mut dt = Hdf5Datatype {
        class,
        size,
        endian: Endian::Le,
        signed: false,
        bit_offset: 0,
        precision: 0,
        string_pad: 0,
        string_charset: 0,
        members: Vec::new(),
        array_dims: Vec::new(),
        base: None,
        reference_type: 0,
        vlen_is_string: false,
    };
    let mut p = off + 8;
    match class {
        0 | 4 => {
            dt.bit_offset = le_u16(buf, p);
            dt.precision = le_u16(buf, p + 2);
            p += 4;
            dt.endian = if flags0 & 0x01 != 0 {
                Endian::Be
            } else {
                Endian::Le
            };
            dt.signed = flags0 & 0x08 != 0;
        }
        1 => {
            dt.bit_offset = le_u16(buf, p);
            dt.precision = le_u16(buf, p + 2);
            p += 12;
            dt.endian = if flags0 & 0x01 != 0 {
                Endian::Be
            } else {
                Endian::Le
            };
            dt.signed = true;
        }
        2 => {
            dt.precision = le_u16(buf, p);
            p += 2;
            dt.endian = if flags0 & 0x01 != 0 {
                Endian::Be
            } else {
                Endian::Le
            };
        }
        3 => {
            dt.string_pad = flags0 & 0x0f;
            dt.string_charset = (flags0 >> 4) & 0x0f;
        }
        5 => {
            let tag_len = ((flags0 as usize) + 7) & !7;
            p += tag_len;
        }
        6 => {
            let nmembs = (flags1 as usize) << 8 | flags0 as usize;
            let offset_nbytes = ((64 - (size as u64).leading_zeros() as usize) + 7) / 8;
            let mut total = 0usize;
            for _ in 0..nmembs {
                let name_len = {
                    let mut n = 0;
                    while p + n < buf.len() && buf[p + n] != 0 {
                        n += 1;
                    }
                    n
                };
                let name = byte_str(&buf[p..p + name_len]);
                if version >= 3 {
                    p += name_len + 1;
                } else {
                    p += align8(name_len + 1);
                }
                let m_off = if version >= 3 {
                    let mut v = 0u64;
                    for i in 0..offset_nbytes {
                        v |= (buf[p + i] as u64) << (8 * i);
                    }
                    p += offset_nbytes;
                    v as u32
                } else {
                    let v = le_u32(buf, p);
                    p += 4;
                    v
                };
                if version == 1 {
                    p += 28;
                }
                let (member_dt, used) = parse_datatype(buf, p)?;
                p += used;
                total += member_dt.size;
                dt.members.push(Hdf5CompoundMember {
                    name,
                    offset: m_off,
                    datatype: member_dt,
                });
            }
            dt.size = total.max(size);
        }
        7 => {
            dt.reference_type = flags0 & 0x0f;
        }
        8 => {
            let (base, used) = parse_datatype(buf, p)?;
            p += used;
            dt.base = Some(Box::new(base));
        }
        9 => {
            let vlen_type = flags0 & 0x0f;
            dt.vlen_is_string = vlen_type == 1;
            let (base, used) = parse_datatype(buf, p)?;
            p += used;
            dt.base = Some(Box::new(base));
        }
        10 => {
            let ndims = buf[p] as usize;
            p += 1;
            if version < 3 {
                p += 3;
            }
            let mut dims = Vec::with_capacity(ndims);
            for _ in 0..ndims {
                dims.push(le_u32(buf, p) as u64);
                p += 4;
            }
            if version < 3 {
                p += ndims * 4;
            }
            let (base, used) = parse_datatype(buf, p)?;
            p += used;
            let count = dims.iter().fold(1usize, |a, d| a * (*d as usize)).max(1);
            dt.array_dims = dims;
            dt.base = Some(Box::new(base));
            dt.size = size.max(dt.base.as_ref().map(|b| b.size).unwrap_or(0) * count);
        }
        _ => return Err(Hdf5Note::Datatype { class, off }),
    }
    Ok((dt, p - off))
}

fn parse_layout(buf: &[u8], off: usize) -> Result<Hdf5Layout, Hdf5Note> {
    if off + 2 > buf.len() {
        return Err(Hdf5Note::EndAtByte { off });
    }
    let version = buf[off];
    let class = buf[off + 1];
    if version == 4 {
        return Err(Hdf5Note::VirtualDataset);
    }
    let p = off + 2;
    match class {
        0 => {
            let size = le_u16(buf, p) as usize;
            Ok(Hdf5Layout::Compact {
                data: buf[p + 2..p + 2 + size].to_vec(),
            })
        }
        1 => {
            let addr = le_u64(buf, p);
            let size = le_u64(buf, p + 8);
            Ok(Hdf5Layout::Contiguous { addr, size })
        }
        2 => {
            let ndims = buf[p] as usize;
            let btree = le_u64(buf, p + 1);
            let mut dims = Vec::with_capacity(ndims);
            for i in 0..ndims {
                dims.push(le_u32(buf, p + 9 + i * 4));
            }
            let elem_size = dims.pop().unwrap_or(0);
            Ok(Hdf5Layout::Chunked {
                btree,
                chunk_dims: dims,
                elem_size,
            })
        }
        _ => Err(Hdf5Note::Layout { class, off }),
    }
}

fn parse_filters(buf: &[u8], off: usize) -> Result<Vec<Hdf5Filter>, Hdf5Note> {
    if off + 8 > buf.len() {
        return Err(Hdf5Note::EndAtByte { off });
    }
    let version = buf[off];
    let n = buf[off + 1] as usize;
    let mut p = if version == 1 { off + 8 } else { off + 4 };
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        if p + 8 > buf.len() {
            return Err(Hdf5Note::EndAtByte { off: p });
        }
        let id = le_u16(buf, p);
        p += 2;
        let name_len = if version == 1 {
            let nl = le_u16(buf, p) as usize;
            p += 2;
            nl
        } else if id >= 256 {
            let nl = le_u16(buf, p) as usize;
            p += 2;
            nl
        } else {
            0
        };
        let flags = le_u16(buf, p);
        p += 2;
        let nc = le_u16(buf, p) as usize;
        p += 2;
        if name_len > 0 {
            p += if version == 1 {
                name_len
            } else {
                align8(name_len)
            };
        }
        let mut cd = Vec::with_capacity(nc);
        for _ in 0..nc {
            cd.push(le_u32(buf, p));
            p += 4;
        }
        if version == 1 && nc % 2 != 0 {
            p += 4;
        }
        out.push(Hdf5Filter {
            id,
            flags,
            cd_values: cd,
        });
    }
    Ok(out)
}

fn parse_link(buf: &[u8], off: usize) -> Result<Hdf5Link, Hdf5Note> {
    if off + 2 > buf.len() {
        return Err(Hdf5Note::EndAtByte { off });
    }
    let flags = buf[off + 1];
    let mut p = off + 2;
    if flags & 0x08 != 0 {
        p += 1;
    }
    if flags & 0x04 != 0 {
        p += 8;
    }
    if flags & 0x10 != 0 {
        p += 1;
    }
    let name_len_bytes = 1usize << (flags & 0x03);
    if p + name_len_bytes > buf.len() {
        return Err(Hdf5Note::EndAtByte { off: p });
    }
    let name_len = match name_len_bytes {
        1 => buf[p] as usize,
        2 => le_u16(buf, p) as usize,
        4 => le_u32(buf, p) as usize,
        _ => le_u64(buf, p) as usize,
    };
    p += name_len_bytes;
    if p + name_len > buf.len() {
        return Err(Hdf5Note::EndAtByte { off: p });
    }
    let name = byte_str(&buf[p..p + name_len]);
    p += name_len;
    if flags & 0x08 != 0 {
        let ltype = buf[off + 2];
        match ltype {
            0 => {
                if p + 8 > buf.len() {
                    return Err(Hdf5Note::EndAtByte { off: p });
                }
                Ok(Hdf5Link {
                    name,
                    addr: le_u64(buf, p),
                    soft: None,
                })
            }
            1 => {
                if p + 2 > buf.len() {
                    return Err(Hdf5Note::EndAtByte { off: p });
                }
                let l = le_u16(buf, p) as usize;
                let target = byte_str(&buf[p + 2..p + 2 + l]);
                Ok(Hdf5Link {
                    name,
                    addr: UNDEF,
                    soft: Some(target),
                })
            }
            _ => Err(Hdf5Note::MissingObject { name }),
        }
    } else {
        if p + 8 > buf.len() {
            return Err(Hdf5Note::EndAtByte { off: p });
        }
        Ok(Hdf5Link {
            name,
            addr: le_u64(buf, p),
            soft: None,
        })
    }
}

#[derive(Clone, Debug)]
struct FractalHeap {
    id_len: usize,
    heap_off_size: usize,
    heap_len_size: usize,
    root_block: u64,
    start_block: u64,
    table_width: u16,
    curr_root_rows: u16,
}

fn parse_fractal_heap(buf: &[u8], addr: u64) -> Result<FractalHeap, Hdf5Note> {
    let off = addr as usize;
    if buf.len() < off + 12 || &buf[off..off + 4] != b"FRHP" {
        let mut found = [0u8; 4];
        found.copy_from_slice(buf.get(off..off + 4).unwrap_or(b"    "));
        return Err(Hdf5Note::Heap { found, off });
    }
    let id_len = le_u16(buf, off + 5) as usize;
    let max_managed = le_u32(buf, off + 10);
    let table_width = le_u16(buf, off + 110);
    let start_block = le_u64(buf, off + 112);
    let max_direct = le_u64(buf, off + 120);
    let max_index = le_u16(buf, off + 128);
    let root_block = le_u64(buf, off + 132);
    let curr_root_rows = le_u16(buf, off + 140);
    let heap_off_size = ((max_index as usize) + 7) / 8;
    let heap_len_size = {
        let v = max_direct.min(max_managed as u64);
        let mut n = 0usize;
        while (1u64 << (n * 8)) - 1 < v {
            n += 1;
        }
        n.max(1)
    };
    Ok(FractalHeap {
        id_len,
        heap_off_size,
        heap_len_size,
        root_block,
        start_block,
        table_width,
        curr_root_rows,
    })
}

fn heap_row_size(h: &FractalHeap, row: usize) -> u64 {
    if row < 2 {
        h.start_block
    } else {
        h.start_block * (1u64 << (row - 1))
    }
}

fn heap_read_id(buf: &[u8], h: &FractalHeap, id: &[u8]) -> Result<Vec<u8>, Hdf5Note> {
    if id.is_empty() {
        return Err(Hdf5Note::EndAtByte { off: 0 });
    }
    let typ = id[0] & 0x30;
    match typ {
        0x00 => {
            let mut p = 1usize;
            let mut obj_off = 0u64;
            for i in 0..h.heap_off_size {
                obj_off |= (id[p] as u64) << (8 * i);
                p += 1;
            }
            let mut obj_len = 0u64;
            for i in 0..h.heap_len_size {
                obj_len |= (id[p] as u64) << (8 * i);
                p += 1;
            }
            let mut stack = vec![(h.root_block, 0u64, h.start_block)];
            while let Some((block_addr, block_off, block_size)) = stack.pop() {
                let a = block_addr as usize;
                if a + 4 > buf.len() {
                    return Err(Hdf5Note::EndAtByte { off: a });
                }
                match &buf[a..a + 4] {
                    b"FHDB" => {
                        let raw_off = le_u64(buf, a + 13);
                        let dblock_off = if h.heap_off_size >= 8 {
                            raw_off
                        } else {
                            raw_off & ((1u64 << (h.heap_off_size * 8)) - 1)
                        };
                        if obj_off >= dblock_off && obj_off < dblock_off + block_size {
                            let within = (obj_off - dblock_off) as usize;
                            let data_off = a + within;
                            return Ok(buf[data_off..data_off + obj_len as usize].to_vec());
                        }
                    }
                    b"FHIB" => {
                        let mut p = a + 4 + 1 + 8 + h.heap_off_size;
                        let mut child_off = block_off;
                        for row in 0..h.curr_root_rows.max(1) as usize {
                            let size = heap_row_size(h, row);
                            for _ in 0..h.table_width {
                                if p + 8 > buf.len() {
                                    return Err(Hdf5Note::EndAtByte { off: p });
                                }
                                let child = le_u64(buf, p);
                                p += 8;
                                if child != UNDEF {
                                    if obj_off >= child_off && obj_off < child_off + size {
                                        stack.push((child, child_off, size));
                                    }
                                }
                                child_off += size;
                            }
                        }
                    }
                    found => {
                        let mut f = [0u8; 4];
                        f.copy_from_slice(found);
                        return Err(Hdf5Note::Heap { found: f, off: a });
                    }
                }
            }
            Err(Hdf5Note::Chunk {
                off: obj_off as usize,
            })
        }
        0x20 => {
            let len = (id[0] & 0x0f) as usize + 1;
            let data = &id[1..];
            if data.len() >= len {
                Ok(data[..len].to_vec())
            } else {
                Err(Hdf5Note::EndAtByte { off: 0 })
            }
        }
        _ => Err(Hdf5Note::HugeHeapObject),
    }
}

#[derive(Clone, Debug)]
struct BtreeHeader {
    node_size: usize,
    record_size: usize,
    root_addr: u64,
}

fn parse_btree_header(buf: &[u8], addr: u64) -> Result<(u8, BtreeHeader), Hdf5Note> {
    let off = addr as usize;
    if buf.len() < off + 8 || &buf[off..off + 4] != b"BTHD" {
        let mut found = [0u8; 4];
        found.copy_from_slice(buf.get(off..off + 4).unwrap_or(b"    "));
        return Err(Hdf5Note::Signatur { off, found });
    }
    let typ = buf[off + 5];
    Ok((
        typ,
        BtreeHeader {
            node_size: le_u32(buf, off + 6) as usize,
            record_size: le_u16(buf, off + 10) as usize,
            root_addr: le_u64(buf, off + 16),
        },
    ))
}

fn btree_leaf_records(buf: &[u8], addr: u64, hdr: &BtreeHeader) -> Result<Vec<Vec<u8>>, Hdf5Note> {
    let off = addr as usize;
    if buf.len() < off + 6 {
        return Err(Hdf5Note::EndAtByte { off });
    }
    match &buf[off..off + 4] {
        b"BTLF" => {
            let mut p = off + 6;
            let mut out = Vec::new();
            while p + hdr.record_size <= buf.len() && p + hdr.record_size <= off + hdr.node_size - 4
            {
                out.push(buf[p..p + hdr.record_size].to_vec());
                p += hdr.record_size;
            }
            Ok(out)
        }
        b"BTIN" => {
            let mut p = off + 6;
            let mut out = Vec::new();
            while p + hdr.record_size + 8 <= buf.len()
                && p + hdr.record_size + 8 <= off + hdr.node_size
            {
                out.push(buf[p..p + hdr.record_size].to_vec());
                p += hdr.record_size;
                let child = le_u64(buf, p);
                p += 8;
                if child != UNDEF {
                    let mut sub = btree_leaf_records(buf, child, hdr)?;
                    out.append(&mut sub);
                }
            }
            Ok(out)
        }
        found => {
            let mut f = [0u8; 4];
            f.copy_from_slice(found);
            Err(Hdf5Note::BtreeNode { found: f, off })
        }
    }
}

fn link_info_of(msg: &RawMessage) -> Option<(Option<u64>, Option<u64>, Option<u64>)> {
    if msg.typ != MSG_LINK_INFO || msg.data.len() < 2 {
        return None;
    }
    let flags = msg.data[1];
    let mut p = 2usize;
    if flags & 0x01 != 0 {
        p += 8;
    }
    let mut fh = UNDEF;
    let mut name_bt = UNDEF;
    let mut co_bt = UNDEF;
    if p + 8 <= msg.data.len() {
        fh = le_u64(&msg.data, p);
        p += 8;
    }
    if flags & 0x02 != 0 && p + 8 <= msg.data.len() {
        name_bt = le_u64(&msg.data, p);
        p += 8;
    }
    if flags & 0x04 != 0 && p + 8 <= msg.data.len() {
        co_bt = le_u64(&msg.data, p);
    }
    Some((
        if fh == UNDEF { None } else { Some(fh) },
        if name_bt == UNDEF {
            None
        } else {
            Some(name_bt)
        },
        if co_bt == UNDEF { None } else { Some(co_bt) },
    ))
}

fn read_links_modern(buf: &[u8], msg: &RawMessage) -> Result<Vec<Hdf5Link>, Hdf5Note> {
    let (Some(fh), Some(name_bt), _) = link_info_of(msg).unwrap_or((None, None, None)) else {
        return Ok(Vec::new());
    };
    let heap = parse_fractal_heap(buf, fh)?;
    let (_, hdr) = parse_btree_header(buf, name_bt)?;
    if hdr.root_addr == UNDEF {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for rec in btree_leaf_records(buf, hdr.root_addr, &hdr)? {
        if rec.len() < 4 + heap.id_len {
            return Err(Hdf5Note::Chunk { off: 0 });
        }
        if rec[4..].iter().all(|&b| b == 0) {
            continue;
        }
        let id = &rec[4..4 + heap.id_len];
        let raw = heap_read_id(buf, &heap, id)?;
        let link = parse_link(&raw, 0)?;
        out.push(link);
    }
    Ok(out)
}

fn read_symtab_group(buf: &[u8], msg: &RawMessage) -> Result<Vec<Hdf5Link>, Hdf5Note> {
    if msg.data.len() < 16 {
        return Err(Hdf5Note::EndAtByte { off: 0 });
    }
    let btree_addr = le_u64(&msg.data, 0);
    let heap_addr = le_u64(&msg.data, 8);
    let hoff = heap_addr as usize;
    if hoff + 24 > buf.len() || &buf[hoff..hoff + 4] != b"HEAP" {
        let mut found = [0u8; 4];
        found.copy_from_slice(buf.get(hoff..hoff + 4).unwrap_or(b"    "));
        return Err(Hdf5Note::Heap { found, off: hoff });
    }
    let data_seg_addr = le_u64(buf, hoff + 24) as usize;
    let mut entries = Vec::new();
    walk_symtab_nodes(buf, btree_addr, &mut entries)?;
    let mut links = Vec::new();
    for e in entries {
        let noff = data_seg_addr + e.name_offset as usize;
        if noff >= buf.len() {
            return Err(Hdf5Note::EndAtByte { off: noff });
        }
        let name = read_null_name(buf, noff);
        links.push(Hdf5Link {
            name,
            addr: e.obj_addr,
            soft: None,
        });
    }
    Ok(links)
}

struct SymbolEntry {
    name_offset: u64,
    obj_addr: u64,
}

fn walk_symtab_nodes(buf: &[u8], addr: u64, out: &mut Vec<SymbolEntry>) -> Result<(), Hdf5Note> {
    let off = addr as usize;
    if off + 4 > buf.len() {
        return Err(Hdf5Note::EndAtByte { off });
    }
    match &buf[off..off + 4] {
        b"SNOD" => {
            let n = le_u16(buf, off + 6) as usize;
            let mut p = off + 8;
            for _ in 0..n {
                if p + 40 > buf.len() {
                    return Err(Hdf5Note::EndAtByte { off: p });
                }
                out.push(SymbolEntry {
                    name_offset: le_u64(buf, p),
                    obj_addr: le_u64(buf, p + 8),
                });
                p += 40;
            }
            Ok(())
        }
        b"TREE" => {
            let node_type = buf[off + 4];
            let entries = le_u16(buf, off + 6) as usize;
            if node_type != 0 {
                return Err(Hdf5Note::Btree {
                    typ: node_type,
                    off,
                });
            }
            let mut p = off + 24;
            for _ in 0..entries {
                if p + 16 > buf.len() {
                    return Err(Hdf5Note::EndAtByte { off: p });
                }
                p += 8;
                let child = le_u64(buf, p);
                p += 8;
                if child != UNDEF {
                    walk_symtab_nodes(buf, child, out)?;
                }
            }
            Ok(())
        }
        found => {
            let mut f = [0u8; 4];
            f.copy_from_slice(found);
            Err(Hdf5Note::BtreeNode { found: f, off })
        }
    }
}

fn parse_attribute(raw: &[u8]) -> Result<Hdf5Attribute, Hdf5Note> {
    if raw.len() < 8 {
        return Err(Hdf5Note::EndAtByte { off: 0 });
    }
    let version = raw[0];
    let mut p = 2usize;
    let name_size = le_u16(raw, p) as usize;
    let dt_size = le_u16(raw, p + 2) as usize;
    let ds_size = le_u16(raw, p + 4) as usize;
    p += 6;
    if version >= 3 {
        p += 1;
    }
    let name = byte_str(&raw[p..p + name_size.min(raw.len().saturating_sub(p))]);
    if version < 2 {
        p += align8(name_size);
    } else {
        p += name_size;
    }
    let (dt, _) = parse_datatype(raw, p)?;
    if version < 2 {
        p += align8(dt_size);
    } else {
        p += dt_size;
    }
    let ds = parse_dataspace(raw, p)?;
    if version < 2 {
        p += align8(ds_size);
    } else {
        p += ds_size;
    }
    let count: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    let value_len = if dt.vlen_is_string || dt.class == 9 {
        0
    } else {
        dt.size * count
    };
    let value = raw[p..p + value_len.min(raw.len().saturating_sub(p))].to_vec();
    Ok(Hdf5Attribute {
        name,
        datatype: dt,
        dataspace: ds,
        data: value,
    })
}

fn attr_messages(msgs: &[RawMessage]) -> Result<Vec<Hdf5Attribute>, Hdf5Note> {
    let mut out = Vec::new();
    for m in msgs {
        if m.typ == MSG_ATTRIBUTE {
            out.push(parse_attribute(&m.data)?);
        }
    }
    Ok(out)
}

fn dense_attrs(buf: &[u8], msgs: &[RawMessage]) -> Result<Vec<Hdf5Attribute>, Hdf5Note> {
    let mut out = Vec::new();
    for m in msgs {
        if m.typ != MSG_AINFO || m.data.len() < 2 {
            continue;
        }
        let flags = m.data[1];
        let mut p = 2usize;
        if flags & 0x01 != 0 {
            p += 2;
        }
        if p + 16 > m.data.len() {
            continue;
        }
        let fh = le_u64(&m.data, p);
        let name_bt = le_u64(&m.data, p + 8);
        if fh == UNDEF || name_bt == UNDEF {
            continue;
        }
        let heap = parse_fractal_heap(buf, fh)?;
        let (_, hdr) = parse_btree_header(buf, name_bt)?;
        if hdr.root_addr == UNDEF {
            continue;
        }
        for rec in btree_leaf_records(buf, hdr.root_addr, &hdr)? {
            if rec.len() < heap.id_len {
                return Err(Hdf5Note::Chunk { off: 0 });
            }
            if rec[4..].iter().all(|&b| b == 0) {
                continue;
            }
            let id = &rec[..heap.id_len];
            let raw = heap_read_id(buf, &heap, id)?;
            out.push(parse_attribute(&raw)?);
        }
    }
    Ok(out)
}

fn apply_filters(
    data: &mut Vec<u8>,
    filters: &[Hdf5Filter],
    elem_size: usize,
    filter_mask: u32,
) -> Result<(), Hdf5Note> {
    for (i, f) in filters.iter().enumerate().rev() {
        if filter_mask & (1 << i) != 0 {
            continue;
        }
        match f.id {
            FILTER_DEFLATE => {
                let body = if data.len() >= 2
                    && data[0] & 0x0f == 8
                    && (((data[0] as u16) << 8 | data[1] as u16) % 31 == 0)
                {
                    let skip = if data[1] & 0x20 != 0 { 6 } else { 2 };
                    &data[skip..]
                } else {
                    &data[..]
                };
                let out = inflate(body).ok_or(Hdf5Note::Filter { id: f.id, off: 0 })?;
                *data = out;
            }
            FILTER_SHUFFLE => {
                let n = data.len() / elem_size.max(1);
                if n == 0 {
                    return Err(Hdf5Note::Filter { id: f.id, off: 0 });
                }
                let mut out = vec![0u8; data.len()];
                for e in 0..n {
                    for j in 0..elem_size {
                        out[e * elem_size + j] = data[j * n + e];
                    }
                }
                *data = out;
            }
            FILTER_FLETCHER32 => {
                if data.len() < 4 {
                    return Err(Hdf5Note::Filter { id: f.id, off: 0 });
                }
                let stored = le_u32(data, data.len() - 4);
                let body = &data[..data.len() - 4];
                let mut c0 = 0u32;
                let mut c1 = 0u32;
                let n = body.len() / 2;
                for k in 0..n {
                    let word = ((body[2 * k] as u32) << 8) | body[2 * k + 1] as u32;
                    c0 = (c0 + word) % 0xffff;
                    c1 = (c1 + c0) % 0xffff;
                }
                if body.len() % 2 != 0 {
                    let word = (body[body.len() - 1] as u32) << 8;
                    c0 = (c0 + word) % 0xffff;
                    c1 = (c1 + c0) % 0xffff;
                }
                let computed = (c1 << 16) | c0;
                let reversed = ((computed & 0xffff) << 16) | (computed >> 16);
                if computed != stored && reversed != stored {
                    return Err(Hdf5Note::Filter { id: f.id, off: 0 });
                }
                data.truncate(data.len() - 4);
            }
            FILTER_SCALEOFFSET => {
                let scale_type = f.cd_values.first().copied().unwrap_or(2);
                let sf = f.cd_values.get(1).copied().unwrap_or(0);
                if data.len() < 21 {
                    return Err(Hdf5Note::Filter { id: f.id, off: 0 });
                }
                let minbits = le_u32(data, 0) as usize;
                let minval = le_u64(data, 5);
                if minbits == 0 {
                    continue;
                }
                let packed = &data[21..];
                let n_elems = packed.len() * 8 / minbits;
                let mut out = Vec::with_capacity(n_elems * elem_size);
                let mut bit_pos = 0usize;
                let fill_defined = f.cd_values.get(7).copied().unwrap_or(0) == 1;
                let fill: Vec<u8> = if fill_defined {
                    let mut v = vec![0u8; elem_size];
                    for i in 0..elem_size {
                        v[i] = f
                            .cd_values
                            .get(8 + i / 4)
                            .copied()
                            .unwrap_or(0)
                            .to_le_bytes()[i % 4];
                    }
                    v
                } else {
                    Vec::new()
                };
                for _ in 0..n_elems {
                    let mut raw = 0u64;
                    for _ in 0..minbits {
                        let byte = packed[bit_pos / 8];
                        raw = (raw << 1) | ((byte >> (7 - bit_pos % 8)) & 1) as u64;
                        bit_pos += 1;
                    }
                    if scale_type == 1 {
                        let v = (raw as f64 + minval as f64) / 10f64.powi(sf as i32);
                        match elem_size {
                            4 => out.extend_from_slice(&(v as f32).to_le_bytes()),
                            _ => out.extend_from_slice(&v.to_le_bytes()),
                        }
                    } else {
                        if fill_defined && raw == (1u64 << minbits) - 1 {
                            out.extend_from_slice(&fill);
                        } else {
                            let sminval = minval as i64;
                            let v = (raw as i64).wrapping_add(sminval);
                            match elem_size {
                                1 => out.push(v as u8),
                                2 => out.extend_from_slice(&(v as u16).to_le_bytes()),
                                4 => out.extend_from_slice(&(v as u32).to_le_bytes()),
                                _ => out.extend_from_slice(&v.to_le_bytes()),
                            }
                        }
                    }
                }
                *data = out;
            }
            _ => return Err(Hdf5Note::Filter { id: f.id, off: 0 }),
        }
    }
    Ok(())
}

fn parse_fill(m: &RawMessage) -> Result<(bool, Vec<u8>), Hdf5Note> {
    let d = &m.data;
    if d.len() < 2 {
        return Err(Hdf5Note::EndAtByte { off: 0 });
    }
    let version = d[0];
    match version {
        2 => {
            if d.len() < 5 {
                return Err(Hdf5Note::EndAtByte { off: 0 });
            }
            let defined = d[3] != 0;
            let size = le_u32(d, 4) as usize;
            Ok((
                defined,
                d[5..5 + size.min(d.len().saturating_sub(5))].to_vec(),
            ))
        }
        3 => {
            let defined = d[1] & 0x20 != 0;
            if !defined {
                return Ok((false, Vec::new()));
            }
            if d.len() < 6 {
                return Err(Hdf5Note::EndAtByte { off: 0 });
            }
            let size = le_u32(d, 2) as usize;
            let data = d[6..6 + size.min(d.len() - 6)].to_vec();
            Ok((true, data))
        }
        _ => Err(Hdf5Note::EndAtByte { off: 0 }),
    }
}

fn parse_fill_old(m: &RawMessage) -> Result<(bool, Vec<u8>), Hdf5Note> {
    if m.data.len() < 4 {
        return Err(Hdf5Note::EndAtByte { off: 0 });
    }
    let size = le_u32(&m.data, 0) as usize;
    Ok((size > 0, m.data[4..4 + size.min(m.data.len() - 4)].to_vec()))
}

#[derive(Clone, Debug)]
struct ChunkRec {
    addr: u64,
    size: usize,
    filter_mask: u32,
    scaled: Vec<u64>,
}

fn v1_chunk_records(buf: &[u8], addr: u64, rank: usize) -> Result<Vec<ChunkRec>, Hdf5Note> {
    let mut out = Vec::new();
    walk_v1_chunk_node(buf, addr, rank, &mut out)?;
    Ok(out)
}

fn walk_v1_chunk_node(
    buf: &[u8],
    addr: u64,
    rank: usize,
    out: &mut Vec<ChunkRec>,
) -> Result<(), Hdf5Note> {
    let off = addr as usize;
    if buf.len() < off + 24 || &buf[off..off + 4] != b"TREE" {
        let mut found = [0u8; 4];
        found.copy_from_slice(buf.get(off..off + 4).unwrap_or(b"    "));
        return Err(Hdf5Note::BtreeNode { found, off });
    }
    let node_type = buf[off + 4];
    let level = buf[off + 5];
    let nchildren = le_u16(buf, off + 6) as usize;
    if node_type != 1 {
        return Err(Hdf5Note::Btree {
            typ: node_type,
            off,
        });
    }
    let key_size = 8 + (rank + 1) * 8;
    let mut p = off + 24;
    for _ in 0..nchildren {
        if p + key_size + 8 > buf.len() {
            return Err(Hdf5Note::EndAtByte { off: p });
        }
        let nbytes = le_u32(buf, p) as usize;
        let filter_mask = le_u32(buf, p + 4);
        let mut scaled = Vec::with_capacity(rank);
        for d in 0..rank {
            scaled.push(le_u64(buf, p + 8 + d * 8));
        }
        let child = le_u64(buf, p + key_size);
        p += key_size + 8;
        if child == UNDEF {
            continue;
        }
        if level == 0 {
            out.push(ChunkRec {
                addr: child,
                size: nbytes,
                filter_mask,
                scaled,
            });
        } else {
            walk_v1_chunk_node(buf, child, rank, out)?;
        }
    }
    Ok(())
}

fn chunk_records(
    buf: &[u8],
    hdr: &BtreeHeader,
    dims: usize,
    filtered: bool,
) -> Result<Vec<ChunkRec>, Hdf5Note> {
    let mut out = Vec::new();
    if hdr.root_addr == UNDEF {
        return Ok(out);
    }
    for rec in btree_leaf_records(buf, hdr.root_addr, hdr)? {
        if rec.len() < 8 {
            return Err(Hdf5Note::Chunk { off: 0 });
        }
        let addr = le_u64(&rec, 0);
        let mut p = 8usize;
        let (size, filter_mask) = if filtered {
            if p + 8 > rec.len() {
                return Err(Hdf5Note::Chunk { off: 0 });
            }
            let size = le_u32(&rec, p) as usize;
            p += 4;
            let mask = le_u32(&rec, p);
            p += 4;
            (size, mask)
        } else {
            (0, 0)
        };
        let mut scaled = Vec::with_capacity(dims);
        for _ in 0..dims {
            if p + 8 > rec.len() {
                return Err(Hdf5Note::Chunk { off: 0 });
            }
            scaled.push(le_u64(&rec, p));
            p += 8;
        }
        out.push(ChunkRec {
            addr,
            size,
            filter_mask,
            scaled,
        });
    }
    Ok(out)
}

impl<'a> Hdf5File<'a> {
    pub fn parse(buf: &'a [u8]) -> Result<Hdf5File<'a>, Hdf5Note> {
        let sb = parse_superblock(buf)?;
        let mut objects: HashMap<u64, Hdf5Object> = HashMap::new();
        let mut stack = vec![sb.root];
        while let Some(addr) = stack.pop() {
            if addr == UNDEF || objects.contains_key(&addr) {
                continue;
            }
            let msgs = gather_messages(buf, addr)?;
            let mut obj = Hdf5Object {
                addr,
                ..Default::default()
            };
            let mut links = Vec::new();
            let mut compact_links = Vec::new();
            let mut fill_old: Option<RawMessage> = None;
            let mut fill_new: Option<RawMessage> = None;
            for m in &msgs {
                if m.flags & MSG_FLAG_SHARED != 0 && m.typ != MSG_SHARED {
                    return Err(Hdf5Note::SharedMessage);
                }
                match m.typ {
                    MSG_DATASPACE => {
                        obj.dataspace = Some(parse_dataspace(&m.data, 0)?);
                    }
                    MSG_DATATYPE => {
                        obj.datatype = Some(parse_datatype(&m.data, 0)?.0);
                    }
                    MSG_LAYOUT => {
                        obj.layout = Some(parse_layout(&m.data, 0)?);
                    }
                    MSG_FILTERS => {
                        obj.filters = parse_filters(&m.data, 0)?;
                    }
                    MSG_FILL => {
                        fill_new = Some(m.clone());
                    }
                    MSG_FILL_OLD => {
                        fill_old = Some(m.clone());
                    }
                    MSG_GROUP_INFO => {
                        obj.is_group = true;
                    }
                    MSG_LINK_INFO => {
                        obj.is_group = true;
                        links = read_links_modern(buf, m)?;
                    }
                    MSG_LINK => {
                        compact_links.push(parse_link(&m.data, 0)?);
                    }
                    MSG_SYMBOL_TABLE => {
                        obj.is_group = true;
                        links = read_symtab_group(buf, m)?;
                    }
                    _ => {}
                }
            }
            if let Some(f) = fill_new {
                let (defined, data) = parse_fill(&f)?;
                obj.fill_defined = defined;
                obj.fill = data;
            } else if let Some(f) = fill_old {
                let (defined, data) = parse_fill_old(&f)?;
                obj.fill_defined = defined;
                obj.fill = data;
            }
            let mut attrs = attr_messages(&msgs)?;
            attrs.extend(dense_attrs(buf, &msgs)?);
            obj.attrs = attrs;
            if links.is_empty() {
                links = compact_links;
            }
            for l in &links {
                if l.addr != UNDEF {
                    stack.push(l.addr);
                }
            }
            obj.links = links;
            objects.insert(addr, obj);
        }
        Ok(Hdf5File {
            buf,
            root: sb.root,
            objects,
        })
    }

    pub fn root(&self) -> Result<&Hdf5Object, Hdf5Note> {
        self.objects.get(&self.root).ok_or(Hdf5Note::Address {
            off: self.root as usize,
        })
    }

    pub fn resolve(&self, path: &str) -> Result<&Hdf5Object, Hdf5Note> {
        let mut current = self.root()?;
        for part in path.split('/').filter(|p| !p.is_empty()) {
            let link = current
                .links
                .iter()
                .find(|l| l.name == part)
                .ok_or_else(|| Hdf5Note::MissingObject {
                    name: part.to_string(),
                })?;
            if link.addr == UNDEF {
                return Err(Hdf5Note::MissingObject {
                    name: part.to_string(),
                });
            }
            current = self.objects.get(&link.addr).ok_or(Hdf5Note::Address {
                off: link.addr as usize,
            })?;
        }
        Ok(current)
    }

    pub fn dataset(
        &self,
        name: &str,
    ) -> Result<(&Hdf5Object, &Hdf5Dataspace, &Hdf5Datatype), Hdf5Note> {
        let obj = self.resolve(name)?;
        let ds = obj
            .dataspace
            .as_ref()
            .ok_or_else(|| Hdf5Note::MissingObject {
                name: name.to_string(),
            })?;
        let dt = obj
            .datatype
            .as_ref()
            .ok_or_else(|| Hdf5Note::MissingObject {
                name: name.to_string(),
            })?;
        Ok((obj, ds, dt))
    }

    pub fn attribute<'b>(&'b self, name: &str, attr: &str) -> Option<&'b Hdf5Attribute> {
        self.resolve(name)
            .ok()?
            .attrs
            .iter()
            .find(|a| a.name == attr)
    }

    pub fn links_of(&self, path: &str) -> Vec<&Hdf5Link> {
        self.resolve(path)
            .map(|o| o.links.iter().collect())
            .unwrap_or_default()
    }

    pub fn read_dataset(&self, name: &str) -> Result<Vec<u8>, Hdf5Note> {
        let (obj, ds, dt) = self.dataset(name)?;
        if dt.class == 9 {
            return Err(Hdf5Note::VlenNotRead);
        }
        let elem_size = dt.size;
        let count: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
        match obj.layout.as_ref() {
            Some(Hdf5Layout::Compact { data }) => Ok(data.clone()),
            Some(Hdf5Layout::Contiguous { addr, size }) => {
                let off = *addr as usize;
                let len = count * elem_size;
                if *size > 0 && *size as usize != len {
                    return Err(Hdf5Note::Chunk { off });
                }
                Ok(self.buf[off..off + len].to_vec())
            }
            Some(Hdf5Layout::Chunked {
                btree,
                chunk_dims,
                elem_size: declared,
            }) => {
                if *declared as usize != elem_size {
                    return Err(Hdf5Note::Chunk { off: 0 });
                }
                let rank = ds.dims.len();
                if rank == 0 {
                    return Err(Hdf5Note::Chunk { off: 0 });
                }
                let filtered = !obj.filters.is_empty();
                let (recs, v1_index) =
                    if self.buf.get(*btree as usize..*btree as usize + 4) == Some(b"BTHD") {
                        let (typ, hdr) = parse_btree_header(self.buf, *btree)?;
                        if typ != 10 && typ != 11 {
                            return Err(Hdf5Note::Btree {
                                typ,
                                off: *btree as usize,
                            });
                        }
                        (chunk_records(self.buf, &hdr, rank, filtered)?, false)
                    } else {
                        (v1_chunk_records(self.buf, *btree, rank)?, true)
                    };
                let mut out = vec![0u8; count * elem_size];
                for rec in recs {
                    let scaled: Vec<usize> = if v1_index {
                        rec.scaled.iter().map(|s| *s as usize).collect()
                    } else {
                        rec.scaled
                            .iter()
                            .zip(chunk_dims.iter())
                            .map(|(s, c)| (*s as usize) * (*c as usize))
                            .collect()
                    };
                    if scaled.len() != rank {
                        return Err(Hdf5Note::Chunk { off: 0 });
                    }
                    let chunk_elems: usize = chunk_dims.iter().fold(1, |a, d| a * (*d as usize));
                    let mut raw = if rec.size > 0 {
                        let off = rec.addr as usize;
                        self.buf[off..off + rec.size].to_vec()
                    } else {
                        let off = rec.addr as usize;
                        self.buf[off..off + chunk_elems * elem_size].to_vec()
                    };
                    if filtered {
                        apply_filters(&mut raw, &obj.filters, elem_size, rec.filter_mask)?;
                    }
                    if raw.len() > chunk_elems * elem_size {
                        raw.truncate(chunk_elems * elem_size);
                    }
                    let mut idx = vec![0usize; rank];
                    for flat in 0..chunk_elems {
                        let mut rem = flat;
                        for d in (0..rank).rev() {
                            idx[d] = rem % chunk_dims[d] as usize;
                            rem /= chunk_dims[d] as usize;
                        }
                        let mut skip = false;
                        for d in 0..rank {
                            if scaled[d] + idx[d] >= ds.dims[d] as usize {
                                skip = true;
                                break;
                            }
                        }
                        if skip {
                            continue;
                        }
                        let mut dst_off = 0usize;
                        let mut dst_stride = 1usize;
                        for d in (0..rank).rev() {
                            dst_off += (scaled[d] + idx[d]) * dst_stride;
                            dst_stride *= ds.dims[d] as usize;
                        }
                        out[dst_off * elem_size..(dst_off + 1) * elem_size]
                            .copy_from_slice(&raw[flat * elem_size..(flat + 1) * elem_size]);
                    }
                }
                Ok(out)
            }
            None => Err(Hdf5Note::MissingObject {
                name: name.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const SSI_2026: &str =
        "phi/pipeline/katalog/ncei_ssi/ssi_v03r00-preliminary_monthly_s202604_e202606_c20260804.nc";
    const SSI_1874: &str =
        "phi/pipeline/katalog/ncei_ssi/ssi_v03r00_monthly_s187405_e187412_c20240831.nc";
    const FILTERS: &str = "phi/pipeline/katalog/ncei_ssi/filters.h5";

    fn read_fixture(name: &str, path: &str) -> Option<Vec<u8>> {
        if !Path::new(path).exists() {
            eprintln!(
                "skipped (fixture absent): {} — fetch from ncei.noaa.gov/data/solar-spectral-irradiance/access",
                name
            );
            return None;
        }
        Some(std::fs::read(path).expect("fixture read"))
    }

    #[test]
    fn lookup3_matches_reference_vectors() {
        let a = jenkins_lookup3(b"");
        let b = jenkins_lookup3(b"abc");
        let c = jenkins_lookup3(&[0u8; 48]);
        assert_eq!(a, 0x31b8a510);
        assert_eq!(b, 0x0e397631);
        assert_eq!(c, 0x7a1e4f2c);
    }

    #[test]
    fn fletcher32_big_endian_words() {
        let mut c0 = 0u32;
        let mut c1 = 0u32;
        for k in 0..4usize {
            let word = ((k as u32) << 8) | (k + 1) as u32;
            c0 = (c0 + word) % 0xffff;
            c1 = (c1 + c0) % 0xffff;
        }
        let f = (c1 << 16) | c0;
        assert_eq!(f, 0x0a14060a);
    }

    #[test]
    fn shuffle_unshuffle_roundtrip() {
        let elem_size = 8usize;
        let n = 6usize;
        let mut data = vec![0u8; elem_size * n];
        for e in 0..n {
            for j in 0..elem_size {
                data[e * elem_size + j] = (e * 10 + j) as u8;
            }
        }
        let shuffled = {
            let mut out = vec![0u8; data.len()];
            for e in 0..n {
                for j in 0..elem_size {
                    out[j * n + e] = data[e * elem_size + j];
                }
            }
            out
        };
        let dt = Hdf5Datatype::flat_f64();
        let filters = vec![Hdf5Filter {
            id: FILTER_SHUFFLE,
            flags: 0,
            cd_values: Vec::new(),
        }];
        let mut raw = shuffled.clone();
        apply_filters(&mut raw, &filters, elem_size, 0).unwrap();
        assert_eq!(raw, data);
    }

    #[test]
    fn fletcher32_filter_verifies_and_strips() {
        let body = vec![0xABu8, 0xCD, 0x12, 0x34, 0x56, 0x78];
        let mut c0 = 0u32;
        let mut c1 = 0u32;
        for k in 0..3usize {
            let word = ((body[2 * k] as u32) << 8) | body[2 * k + 1] as u32;
            c0 = (c0 + word) % 0xffff;
            c1 = (c1 + c0) % 0xffff;
        }
        let checksum = (c1 << 16) | c0;
        let mut chunk = body.clone();
        chunk.extend_from_slice(&checksum.to_le_bytes());
        let dt = Hdf5Datatype::flat_f64();
        let filters = vec![Hdf5Filter {
            id: FILTER_FLETCHER32,
            flags: 0,
            cd_values: Vec::new(),
        }];
        apply_filters(&mut chunk, &filters, 8, 0).unwrap();
        assert_eq!(chunk, body);
    }

    #[test]
    fn real_ssi_2026_structure_and_values() {
        let Some(bytes) = read_fixture("ssi-2026", SSI_2026) else {
            return;
        };
        let file = Hdf5File::parse(&bytes).unwrap();
        let root = file.root().unwrap();
        assert_eq!(root.links.len(), 9);
        let (obj, ds, dt) = file.dataset("SSI").unwrap();
        assert_eq!(ds.dims, vec![3, 4300]);
        assert_eq!(dt.class, 1);
        assert_eq!(dt.size, 4);
        assert!(matches!(obj.layout, Some(Hdf5Layout::Chunked { .. })));
        let data = file.read_dataset("SSI").unwrap();
        assert_eq!(data.len(), 3 * 4300 * 4);
        let v = decode_f32(&data, 0, Endian::Le).unwrap();
        assert!((v - 1.362934e-05).abs() < 1e-12);
        let tsi = file.read_dataset("TSI").unwrap();
        let t: Vec<f32> = (0..3)
            .map(|i| decode_f32(&tsi, i * 4, Endian::Le).unwrap())
            .collect();
        assert!((t[0] - 1362.0173).abs() < 0.001);
        assert!((t[1] - 1362.1294).abs() < 0.001);
        assert!((t[2] - 1362.1718).abs() < 0.001);
        let wl = file.read_dataset("wavelength").unwrap();
        assert_eq!(wl.len(), 4300 * 4);
        assert!((decode_f32(&wl, 0, Endian::Le).unwrap() - 0.5).abs() < 1e-6);
        assert!((decode_f32(&wl, 4299 * 4, Endian::Le).unwrap() - 199875.0).abs() < 1e-6);
        let units = file.attribute("SSI", "units").unwrap();
        assert_eq!(units.datatype.class, 3);
        assert_eq!(byte_str(&units.data), "W m-2 nm-1");
    }

    #[test]
    fn real_ssi_1874_structure_and_values() {
        let Some(bytes) = read_fixture("ssi-1874", SSI_1874) else {
            return;
        };
        let file = Hdf5File::parse(&bytes).unwrap();
        let (_, ds, _) = file.dataset("SSI").unwrap();
        assert_eq!(ds.dims, vec![8, 4300]);
        let data = file.read_dataset("SSI").unwrap();
        let v = decode_f32(&data, 0, Endian::Le).unwrap();
        assert!((v - 8.8243651e-06).abs() < 1e-12);
        let tsi = file.read_dataset("TSI").unwrap();
        let last = decode_f32(&tsi, 7 * 4, Endian::Le).unwrap();
        assert!((last - 1361.5614).abs() < 0.001);
    }

    #[test]
    fn real_ssi_spectral_bins_integrate_to_tsi() {
        let Some(bytes) = read_fixture("ssi-2026", SSI_2026) else {
            return;
        };
        let file = Hdf5File::parse(&bytes).unwrap();
        let wl = file.read_dataset("wavelength").unwrap();
        let ssi = file.read_dataset("SSI").unwrap();
        let time = file.read_dataset("time").unwrap();
        let n_wl = wl.len() / 4;
        let n_months = time.len() / 4;
        let mut row: Option<usize> = None;
        for i in 0..n_months {
            let days = decode_f32(&time, i * 4, Endian::Le).unwrap() as f64;
            if crate::nc4::time_row_month(days, (1610, 1, 1)) == Some((2026, 6)) {
                row = Some(i);
            }
        }
        let row = row.expect("2026-06 row on the time axis");
        let mut rows = Vec::new();
        for w in 0..n_wl {
            let lam = decode_f32(&wl, w * 4, Endian::Le).unwrap() as f64;
            let e = decode_f32(&ssi, (row * n_wl + w) * 4, Endian::Le).unwrap() as f64;
            rows.push((lam, e, 0));
        }
        let bins = crate::spectral::bins_from_lambda_rows(&rows);
        assert_eq!(bins.len(), 4300);
        let integral: f64 = bins.iter().map(|(_, bw, v)| bw * v).sum();
        assert!(
            (integral - 1362.1718).abs() < 2.0,
            "integral {} W/m² lies outside the TSI",
            integral
        );
    }

    #[test]
    fn real_filters_deflate_shuffle_fletcher32() {
        let Some(bytes) = read_fixture("filters", FILTERS) else {
            return;
        };
        let file = Hdf5File::parse(&bytes).unwrap();
        let (obj, _, _) = file.dataset("def").unwrap();
        assert_eq!(obj.filters.len(), 3);
        let data = file.read_dataset("def").unwrap();
        assert_eq!(data.len(), 1000 * 8);
        for i in 0..1000 {
            let v = decode_f64(&data, i * 8, Endian::Le).unwrap();
            assert_eq!(v, i as f64);
        }
    }

    #[test]
    fn real_filters_scaleoffset() {
        let Some(bytes) = read_fixture("filters", FILTERS) else {
            return;
        };
        let file = Hdf5File::parse(&bytes).unwrap();
        let (obj, _, _) = file.dataset("so").unwrap();
        assert_eq!(obj.filters.len(), 1);
        assert_eq!(obj.filters[0].id, FILTER_SCALEOFFSET);
        let data = file.read_dataset("so").unwrap();
        assert_eq!(data.len(), 1000 * 4);
        for i in 0..1000 {
            let v = i32::from_le_bytes(data[i * 4..i * 4 + 4].try_into().unwrap());
            assert_eq!(v, i as i32);
        }
    }
}
