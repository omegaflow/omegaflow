use crate::inflate::inflate;
use std::collections::{HashMap, HashSet};

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

const MSG_FLAG_SHARED: u8 = 0x02;

const FILTER_DEFLATE: u16 = 1;
const FILTER_SHUFFLE: u16 = 2;
const FILTER_FLETCHER32: u16 = 3;
const FILTER_SCALEOFFSET: u16 = 6;

const MAX_CONT_BLOCKS: usize = 1 << 12;
const MAX_READ_BYTES: u64 = 1 << 24;
const MAX_FETCH_BYTES: u64 = 1 << 28;
const MAX_FETCHES: usize = 1 << 12;
const MAX_TRAVERSAL_BYTES: u64 = 1 << 26;

#[derive(Clone, Debug)]
pub enum Hdf5Note {
    Magic { bytes: [u8; 8] },
    SuperblockVersion { v: u8 },
    OffsetSize { n: u8 },
    EndAtByte { off: usize },
    AbsentAtByte { off: usize },
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
    AbsentObject { name: String },
    Chunk { off: usize },
    ReadLength { off: usize, len: u64 },
    FetchBudget { off: usize, reads: usize },
    TraversalBudget { off: usize, bytes: u64 },
    VlenNotRead,
    VirtualDataset,
}

pub struct ChunkReadDiag {
    stage: &'static str,
    note: Option<Hdf5Note>,
}

impl std::fmt::Debug for ChunkReadDiag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.note {
            Some(n) => write!(f, "{} ({n:?})", self.stage),
            None => write!(f, "{}", self.stage),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct HeaderDiag {
    pub version: u8,
    pub msgs_initial: usize,
    pub msgs_cont: usize,
    pub cont_blocks: usize,
    pub declared: Option<u16>,
    pub symtab_found: bool,
    pub link_info_found: bool,
}

impl std::fmt::Debug for HeaderDiag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "header v{}, {} messages in chunk0",
            self.version, self.msgs_initial
        )?;
        if self.cont_blocks > 0 {
            write!(
                f,
                ", {} messages in {} continuation blocks",
                self.msgs_cont, self.cont_blocks
            )?;
        }
        if let Some(n) = self.declared {
            write!(f, ", declared {n}")?;
        }
        write!(
            f,
            ", symbol table {}, link info {}",
            if self.symtab_found { "found" } else { "absent" },
            if self.link_info_found {
                "found"
            } else {
                "absent"
            }
        )
    }
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
    pub committed_datatype: Option<u64>,
    pub header: HeaderDiag,
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
    offset_size: usize,
    length_size: usize,
}

const SHARE_TYPE_COMMITTED: u8 = 2;

struct SharedRef {
    share_type: u8,
    addr: u64,
}

fn parse_shared_ref(
    data: &[u8],
    offset_size: usize,
    length_size: usize,
) -> Result<SharedRef, Hdf5Note> {
    let version = *data.first().ok_or(Hdf5Note::EndAtByte { off: 0 })?;
    if version > 2 {
        return Err(Hdf5Note::ObjectHeaderVersion { v: version });
    }
    let mut p = 1usize;
    let share_type = if version >= 1 {
        let t = *data.get(p).ok_or(Hdf5Note::EndAtByte { off: p })?;
        p += 1;
        t
    } else {
        SHARE_TYPE_COMMITTED
    };
    if version == 0 {
        p = 8 + length_size;
    }
    let addr = limited_uint(data, p, offset_size).ok_or(Hdf5Note::EndAtByte { off: p })?;
    Ok(SharedRef { share_type, addr })
}

struct Hdf5WindowReader<'a, F> {
    base: &'a [u8],
    fetch: F,
    cache: HashMap<u64, Vec<u8>>,
    fetched_bytes: u64,
    fetches: usize,
    read_bytes: u64,
}

impl<'a, F: FnMut(u64, u64) -> Option<Vec<u8>>> Hdf5WindowReader<'a, F> {
    fn new(base: &'a [u8], fetch: F) -> Hdf5WindowReader<'a, F> {
        Hdf5WindowReader {
            base,
            fetch,
            cache: HashMap::new(),
            fetched_bytes: 0,
            fetches: 0,
            read_bytes: 0,
        }
    }

    fn read(&mut self, off: u64, len: u64) -> Result<Vec<u8>, Hdf5Note> {
        let start = off as usize;
        let end = start
            .checked_add(len as usize)
            .ok_or(Hdf5Note::EndAtByte { off: start })?;
        if self.read_bytes.saturating_add(len) > MAX_TRAVERSAL_BYTES {
            return Err(Hdf5Note::TraversalBudget {
                off: start,
                bytes: self.read_bytes,
            });
        }
        if end <= self.base.len() {
            self.read_bytes = self.read_bytes.saturating_add(len);
            return Ok(self.base[start..end].to_vec());
        }
        for (&wstart, window) in &self.cache {
            let ws = wstart as usize;
            if start >= ws && end <= ws + window.len() {
                self.read_bytes = self.read_bytes.saturating_add(len);
                return Ok(window[start - ws..end - ws].to_vec());
            }
        }
        if len > MAX_READ_BYTES {
            return Err(Hdf5Note::ReadLength { off: start, len });
        }
        if self.fetches >= MAX_FETCHES || self.fetched_bytes.saturating_add(len) > MAX_FETCH_BYTES {
            return Err(Hdf5Note::FetchBudget {
                off: start,
                reads: self.fetches,
            });
        }
        let window = (self.fetch)(off, len).ok_or(Hdf5Note::AbsentAtByte { off: start })?;
        if (window.len() as u64) < len {
            return Err(Hdf5Note::EndAtByte { off: start });
        }
        self.fetches += 1;
        self.fetched_bytes = self.fetched_bytes.saturating_add(window.len() as u64);
        self.read_bytes = self.read_bytes.saturating_add(len);
        self.cache.insert(off, window.clone());
        Ok(window)
    }

    fn read_upto(&mut self, off: u64, len: u64) -> Result<Vec<u8>, Hdf5Note> {
        match self.read(off, len) {
            Ok(bytes) => Ok(bytes),
            Err(Hdf5Note::AbsentAtByte { .. }) => {
                let start = off as usize;
                if start >= self.base.len() {
                    return Err(Hdf5Note::AbsentAtByte { off: start });
                }
                let end = start.saturating_add(len as usize).min(self.base.len());
                Ok(self.base[start..end].to_vec())
            }
            Err(note) => Err(note),
        }
    }
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
        offset_size: offset_size as usize,
        length_size: length_size as usize,
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
    base: usize,
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
            return Err(Hdf5Note::EndAtByte { off: base + p });
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

fn v1_messages(
    buf: &[u8],
    start: usize,
    end: usize,
    base: usize,
) -> Result<Vec<RawMessage>, Hdf5Note> {
    let mut out = Vec::new();
    let mut p = start;
    while p + 8 <= end.min(buf.len()) {
        let typ = le_u16(buf, p) as u8;
        let size = le_u16(buf, p + 2) as usize;
        let flags = buf[p + 4];
        p += 8;
        if p + size > buf.len() {
            return Err(Hdf5Note::EndAtByte { off: base + p });
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

fn cont_target(msg: &RawMessage, offset_size: usize, length_size: usize) -> Option<(u64, u64)> {
    if msg.typ != MSG_CONT {
        return None;
    }
    let addr_len = offset_size + length_size;
    if msg.data.len() < addr_len {
        return None;
    }
    let c = match offset_size {
        4 => le_u32(&msg.data, 0) as u64,
        8 => le_u64(&msg.data, 0),
        _ => return None,
    };
    let len = match length_size {
        4 => le_u32(&msg.data, offset_size) as u64,
        8 => le_u64(&msg.data, offset_size),
        _ => return None,
    };
    Some((c, len))
}

fn gather_messages<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    addr: u64,
    offset_size: usize,
    length_size: usize,
) -> Result<(Vec<RawMessage>, HeaderDiag), Hdf5Note> {
    let off = addr as usize;
    let head = r.read_upto(addr, 64)?;
    if head.len() < 6 {
        return Err(Hdf5Note::EndAtByte {
            off: off + head.len(),
        });
    }
    let has_signature = &head[..4] == b"OHDR";
    let version = if has_signature { head[4] } else { head[0] };
    let mut diag = HeaderDiag {
        version,
        ..HeaderDiag::default()
    };
    let out: Vec<RawMessage> = match version {
        1 => {
            if has_signature {
                return Err(Hdf5Note::ObjectHeaderVersion { v: 1 });
            }
            if head.len() < 12 {
                return Err(Hdf5Note::EndAtByte {
                    off: off + head.len(),
                });
            }
            let total = le_u16(&head, 2) as usize;
            let chunk0_size = le_u32(&head, 8) as usize;
            diag.declared = Some(le_u16(&head, 2));
            let buf = r.read(addr, (16 + chunk0_size) as u64)?;
            let mut out = v1_messages(&buf, 16, 16 + chunk0_size, off)?;
            diag.msgs_initial = out.len();
            let mut stack: Vec<(u64, u64)> = Vec::new();
            for m in &out {
                if let Some(target) = cont_target(m, offset_size, length_size) {
                    stack.push(target);
                }
            }
            let mut visited: HashSet<u64> = HashSet::new();
            while let Some((c, len)) = stack.pop() {
                if out.len() >= total || diag.cont_blocks >= MAX_CONT_BLOCKS {
                    break;
                }
                if !visited.insert(c) {
                    continue;
                }
                let len = len as usize;
                let chunk = r.read(c, len as u64)?;
                let mut sub = v1_messages(&chunk, 0, len, c as usize)?;
                diag.cont_blocks += 1;
                diag.msgs_cont += sub.len();
                for m in &sub {
                    if let Some(target) = cont_target(m, offset_size, length_size) {
                        stack.push(target);
                    }
                }
                out.append(&mut sub);
            }
            out
        }
        2 => {
            let flags = head[5];
            let mut extra = 0usize;
            if flags & 0x20 != 0 {
                extra += 16;
            }
            if flags & 0x10 != 0 {
                extra += 4;
            }
            let size_len = 1usize << (flags & 0x03);
            let header_span = 6 + extra + size_len;
            if head.len() < header_span {
                return Err(Hdf5Note::EndAtByte {
                    off: off + head.len(),
                });
            }
            let chunk0_off = header_span;
            let chunk0_size = match flags & 0x03 {
                0 => head[header_span - 1] as usize,
                1 => le_u16(&head, header_span - 2) as usize,
                2 => le_u32(&head, header_span - 4) as usize,
                _ => le_u64(&head, header_span - 8) as usize,
            };
            let chunk0_end = chunk0_off + chunk0_size;
            let ohdr_span = chunk0_end + 4;
            let buf = r.read(addr, ohdr_span as u64)?;
            if buf.len() < ohdr_span {
                return Err(Hdf5Note::EndAtByte {
                    off: off + buf.len(),
                });
            }
            check_checksum(&buf[..chunk0_end], &buf[chunk0_end..chunk0_end + 4])?;
            let tracked = flags & 0x04 != 0;
            let mut out = v2_messages(&buf, chunk0_off, chunk0_end, tracked, off)?;
            diag.msgs_initial = out.len();
            let mut stack: Vec<(u64, u64)> = Vec::new();
            for m in &out {
                if let Some(target) = cont_target(m, offset_size, length_size) {
                    stack.push(target);
                }
            }
            let mut visited: HashSet<u64> = HashSet::new();
            while let Some((c, len)) = stack.pop() {
                if diag.cont_blocks >= MAX_CONT_BLOCKS {
                    break;
                }
                if !visited.insert(c) {
                    continue;
                }
                let len = len as usize;
                let chunk = r.read(c, len as u64)?;
                if chunk.len() < 4 {
                    return Err(Hdf5Note::EndAtByte {
                        off: c as usize + chunk.len(),
                    });
                }
                if &chunk[..4] != b"OCHK" {
                    return Err(Hdf5Note::Address { off: c as usize });
                }
                if chunk.len() < len {
                    return Err(Hdf5Note::EndAtByte {
                        off: c as usize + chunk.len(),
                    });
                }
                check_checksum(&chunk[..len - 4], &chunk[len - 4..len])?;
                let mut sub = v2_messages(&chunk, 4, len - 4, tracked, c as usize)?;
                diag.cont_blocks += 1;
                diag.msgs_cont += sub.len();
                for m in &sub {
                    if let Some(target) = cont_target(m, offset_size, length_size) {
                        stack.push(target);
                    }
                }
                out.append(&mut sub);
            }
            out
        }
        v => return Err(Hdf5Note::ObjectHeaderVersion { v }),
    };
    for m in &out {
        if m.typ == MSG_SYMBOL_TABLE {
            diag.symtab_found = true;
        }
        if m.typ == MSG_LINK_INFO {
            diag.link_info_found = true;
        }
    }
    Ok((out, diag))
}

fn parse_object_header<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    reader: &mut Hdf5WindowReader<F>,
    addr: u64,
    offset_size: usize,
    length_size: usize,
) -> Result<Hdf5Object, Hdf5Note> {
    let (msgs, diag) = gather_messages(reader, addr, offset_size, length_size)?;
    let mut obj = Hdf5Object {
        addr,
        header: diag,
        ..Default::default()
    };
    let mut links = Vec::new();
    let mut compact_links = Vec::new();
    let mut fill_old: Option<RawMessage> = None;
    let mut fill_new: Option<RawMessage> = None;
    for m in &msgs {
        if m.flags & MSG_FLAG_SHARED != 0 {
            let shared = parse_shared_ref(&m.data, offset_size, length_size)?;
            match (m.typ, shared.share_type) {
                (MSG_DATATYPE, SHARE_TYPE_COMMITTED) => {
                    obj.committed_datatype = Some(shared.addr);
                }
                _ => {
                    return Err(Hdf5Note::SharedMessage);
                }
            }
            continue;
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
                links = read_links_modern(reader, m)?;
            }
            MSG_LINK => {
                compact_links.push(parse_link(&m.data, 0)?);
            }
            MSG_SYMBOL_TABLE => {
                obj.is_group = true;
                links = read_symtab_group(reader, m)?;
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
    attrs.extend(dense_attrs(reader, &msgs)?);
    obj.attrs = attrs;
    if links.is_empty() {
        links = compact_links;
    }
    obj.links = links;
    Ok(obj)
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
            let offset_nbytes = (64 - (size as u64).leading_zeros() as usize).div_ceil(8);
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
            let count = dims.iter().fold(1usize, |a, d| a * (*d as usize));
            dt.array_dims = dims;
            let base_size = base.size;
            dt.base = Some(Box::new(base));
            dt.size = size.max(base_size * count);
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
            let elem_size = match dims.pop() {
                Some(v) => v,
                None => return Err(Hdf5Note::Layout { class, off }),
            };
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
    if off + 2 > buf.len() {
        return Err(Hdf5Note::EndAtByte { off });
    }
    let version = buf[off];
    let n = buf[off + 1] as usize;
    let mut p = if version == 1 { off + 8 } else { off + 2 };
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        if p + 6 > buf.len() {
            return Err(Hdf5Note::EndAtByte { off: p });
        }
        let id = le_u16(buf, p);
        p += 2;
        let nl = if version == 1 || id >= 256 {
            if p + 2 > buf.len() {
                return Err(Hdf5Note::EndAtByte { off: p });
            }
            let nl = le_u16(buf, p) as usize;
            p += 2;
            nl
        } else {
            0
        };
        if p + 4 > buf.len() {
            return Err(Hdf5Note::EndAtByte { off: p });
        }
        let flags = le_u16(buf, p);
        p += 2;
        let nc = le_u16(buf, p) as usize;
        p += 2;
        if p + nl > buf.len() {
            return Err(Hdf5Note::EndAtByte { off: p });
        }
        p += nl;
        if p + nc * 4 > buf.len() {
            return Err(Hdf5Note::EndAtByte { off: p });
        }
        let mut cd = Vec::with_capacity(nc);
        for _ in 0..nc {
            cd.push(le_u32(buf, p));
            p += 4;
        }
        if version == 1 && !nc.is_multiple_of(2) {
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
            _ => Err(Hdf5Note::AbsentObject { name }),
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

fn parse_fractal_heap<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    addr: u64,
) -> Result<FractalHeap, Hdf5Note> {
    let off = addr as usize;
    let buf = r.read(addr, 144)?;
    if buf.len() < 142 {
        return Err(Hdf5Note::EndAtByte {
            off: off + buf.len(),
        });
    }
    if &buf[..4] != b"FRHP" {
        let mut found = [0u8; 4];
        found.copy_from_slice(&buf[..4]);
        return Err(Hdf5Note::Heap { found, off });
    }
    let id_len = le_u16(&buf, 5) as usize;
    let max_managed = le_u32(&buf, 10);
    let table_width = le_u16(&buf, 110);
    let start_block = le_u64(&buf, 112);
    let max_direct = le_u64(&buf, 120);
    let max_index = le_u16(&buf, 128);
    let root_block = le_u64(&buf, 132);
    let curr_root_rows = le_u16(&buf, 140);
    let heap_off_size = (max_index as usize).div_ceil(8);
    let heap_len_size = {
        let v = max_direct.min(max_managed as u64);
        let mut n = 0usize;
        while (1u64 << (n * 8)) - 1 < v {
            n += 1;
        }
        n
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

fn heap_read_id<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    h: &FractalHeap,
    id: &[u8],
) -> Result<Vec<u8>, Hdf5Note> {
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
                let sig = r.read(block_addr, 4)?;
                if sig.len() < 4 {
                    return Err(Hdf5Note::EndAtByte {
                        off: block_addr as usize + sig.len(),
                    });
                }
                match &sig[..] {
                    b"FHDB" => {
                        let buf = r.read(block_addr, block_size)?;
                        if buf.len() < 13 + h.heap_off_size {
                            return Err(Hdf5Note::EndAtByte {
                                off: block_addr as usize + buf.len(),
                            });
                        }
                        let raw_off = le_u64(&buf, 13);
                        let dblock_off = if h.heap_off_size >= 8 {
                            raw_off
                        } else {
                            raw_off & ((1u64 << (h.heap_off_size * 8)) - 1)
                        };
                        if obj_off >= dblock_off && obj_off < dblock_off + block_size {
                            let within = (obj_off - dblock_off) as usize;
                            let end = within
                                .checked_add(obj_len as usize)
                                .filter(|e| *e <= buf.len())
                                .ok_or(Hdf5Note::EndAtByte {
                                    off: block_addr as usize + buf.len(),
                                })?;
                            return Ok(buf[within..end].to_vec());
                        }
                    }
                    b"FHIB" => {
                        let table_len = 4
                            + 1
                            + 8
                            + h.heap_off_size
                            + 8 * h.table_width as usize * h.curr_root_rows as usize;
                        let buf = r.read(block_addr, table_len as u64)?;
                        if buf.len() < table_len {
                            return Err(Hdf5Note::EndAtByte {
                                off: block_addr as usize + buf.len(),
                            });
                        }
                        let mut p = 4 + 1 + 8 + h.heap_off_size;
                        let mut child_off = block_off;
                        for row in 0..h.curr_root_rows as usize {
                            let size = heap_row_size(h, row);
                            for _ in 0..h.table_width {
                                let child = le_u64(&buf, p);
                                p += 8;
                                if child != UNDEF
                                    && obj_off >= child_off
                                    && obj_off < child_off + size
                                {
                                    stack.push((child, child_off, size));
                                }
                                child_off += size;
                            }
                        }
                    }
                    found => {
                        let mut f = [0u8; 4];
                        f.copy_from_slice(found);
                        return Err(Hdf5Note::Heap {
                            found: f,
                            off: block_addr as usize,
                        });
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
    depth: u16,
    root_addr: u64,
    root_nrec: u16,
    total_records: u64,
}

fn parse_btree_header<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    addr: u64,
) -> Result<(u8, BtreeHeader), Hdf5Note> {
    let buf = r.read(addr, 38)?;
    parse_btree_header_bytes(&buf, addr)
}

fn parse_btree_header_bytes(buf: &[u8], addr: u64) -> Result<(u8, BtreeHeader), Hdf5Note> {
    let off = addr as usize;
    let mut found = [0u8; 4];
    found.copy_from_slice(buf.get(..4).unwrap_or(b"    "));
    if buf.len() < 4 || &buf[..4] != b"BTHD" {
        return Err(Hdf5Note::Signatur { off, found });
    }
    if buf.len() < 38 {
        return Err(Hdf5Note::EndAtByte {
            off: off + buf.len(),
        });
    }
    check_checksum(&buf[..34], &buf[34..38])?;
    let typ = buf[5];
    Ok((
        typ,
        BtreeHeader {
            node_size: le_u32(&buf, 6) as usize,
            record_size: le_u16(&buf, 10) as usize,
            depth: le_u16(&buf, 12),
            root_addr: le_u64(&buf, 16),
            root_nrec: le_u16(&buf, 24),
            total_records: le_u64(&buf, 26),
        },
    ))
}

fn limit_enc_size(v: u64) -> usize {
    if v <= 0xff {
        1
    } else if v <= 0xffff {
        2
    } else if v <= 0xffff_ffff {
        4
    } else if v <= 0xffff_ffff_ffff {
        6
    } else {
        8
    }
}

fn limited_uint(data: &[u8], off: usize, size: usize) -> Option<u64> {
    match size {
        1 => data.get(off).map(|&b| b as u64),
        2 => data
            .get(off..off + 2)
            .and_then(|s| Some(u16::from_le_bytes(s.try_into().ok()?) as u64)),
        4 => data
            .get(off..off + 4)
            .and_then(|s| Some(u32::from_le_bytes(s.try_into().ok()?) as u64)),
        6 => {
            let s = data.get(off..off + 6)?;
            Some(
                s[0] as u64
                    | (s[1] as u64) << 8
                    | (s[2] as u64) << 16
                    | (s[3] as u64) << 24
                    | (s[4] as u64) << 32
                    | (s[5] as u64) << 40,
            )
        }
        8 => data
            .get(off..off + 8)
            .and_then(|s| Some(u64::from_le_bytes(s.try_into().ok()?))),
        _ => None,
    }
}

fn btree_records<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    addr: u64,
    hdr: &BtreeHeader,
    depth: u16,
    nrec: usize,
) -> Result<Vec<Vec<u8>>, Hdf5Note> {
    let off = addr as usize;
    if hdr.node_size < 6 {
        return Err(Hdf5Note::EndAtByte { off });
    }
    let buf = r.read(addr, hdr.node_size as u64)?;
    if buf.len() < 6 {
        return Err(Hdf5Note::EndAtByte {
            off: off + buf.len(),
        });
    }
    match &buf[..4] {
        b"BTLF" => {
            let pos = 6 + nrec * hdr.record_size;
            if pos + 4 > hdr.node_size {
                return Err(Hdf5Note::Chunk { off });
            }
            if buf.len() < pos + 4 {
                return Err(Hdf5Note::EndAtByte {
                    off: off + buf.len(),
                });
            }
            check_checksum(&buf[..pos], &buf[pos..pos + 4])?;
            let mut out = Vec::with_capacity(nrec);
            let mut p = 6usize;
            for _ in 0..nrec {
                out.push(buf[p..p + hdr.record_size].to_vec());
                p += hdr.record_size;
            }
            Ok(out)
        }
        b"BTIN" => {
            if depth == 0 {
                return Err(Hdf5Note::BtreeNode {
                    found: *b"BTIN",
                    off,
                });
            }
            let total_guess = limit_enc_size(hdr.total_records);
            let candidates: Vec<(usize, usize)> = if depth > 1 {
                vec![
                    (1, total_guess),
                    (2, total_guess),
                    (4, total_guess),
                    (1, 2),
                    (2, 2),
                    (1, 4),
                    (2, 4),
                    (1, 1),
                    (2, 1),
                ]
            } else {
                vec![(1, 0), (2, 0), (4, 0), (6, 0), (8, 0)]
            };
            for (nsz, tsz) in candidates {
                let triplet = 8 + nsz + tsz;
                let pos = 6 + nrec * hdr.record_size + (nrec + 1) * triplet;
                if pos + 4 > hdr.node_size || buf.len() < pos + 4 {
                    continue;
                }
                if check_checksum(&buf[..pos], &buf[pos..pos + 4]).is_err() {
                    continue;
                }
                return internal_node_try(&buf, r, hdr, depth, nrec, nsz, tsz);
            }
            Err(Hdf5Note::Checksum { off })
        }
        found => {
            let mut f = [0u8; 4];
            f.copy_from_slice(found);
            Err(Hdf5Note::BtreeNode { found: f, off })
        }
    }
}

fn internal_node_try<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    buf: &[u8],
    r: &mut Hdf5WindowReader<F>,
    hdr: &BtreeHeader,
    depth: u16,
    nrec: usize,
    nsz: usize,
    tsz: usize,
) -> Result<Vec<Vec<u8>>, Hdf5Note> {
    let mut p = 6usize;
    let mut recs = Vec::with_capacity(nrec);
    for _ in 0..nrec {
        recs.push(buf[p..p + hdr.record_size].to_vec());
        p += hdr.record_size;
    }
    let mut children: Vec<(u64, usize)> = Vec::with_capacity(nrec + 1);
    for _ in 0..=nrec {
        if p + 8 + nsz + tsz > buf.len() {
            return Err(Hdf5Note::Chunk { off: p });
        }
        let child = le_u64(buf, p);
        p += 8;
        let child_nrec = match limited_uint(buf, p, nsz) {
            Some(v) => v as usize,
            None => return Err(Hdf5Note::Chunk { off: p }),
        };
        p += nsz + tsz;
        children.push((child, child_nrec));
    }
    let mut out = Vec::new();
    for i in 0..nrec {
        let (child, child_nrec) = children[i];
        if child != UNDEF {
            let mut sub = btree_records(r, child, hdr, depth - 1, child_nrec)?;
            out.append(&mut sub);
        }
        out.push(recs[i].clone());
    }
    let (child, child_nrec) = children[nrec];
    if child != UNDEF {
        let mut sub = btree_records(r, child, hdr, depth - 1, child_nrec)?;
        out.append(&mut sub);
    }
    Ok(out)
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
    if p + 8 <= msg.data.len() {
        name_bt = le_u64(&msg.data, p);
        p += 8;
    }
    if flags & 0x02 != 0 && p + 8 <= msg.data.len() {
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

fn read_links_modern<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    msg: &RawMessage,
) -> Result<Vec<Hdf5Link>, Hdf5Note> {
    let (Some(fh), Some(name_bt), _) = link_info_of(msg).unwrap_or((None, None, None)) else {
        return Ok(Vec::new());
    };
    let heap = parse_fractal_heap(r, fh)?;
    let (_, hdr) = parse_btree_header(r, name_bt)?;
    if hdr.root_addr == UNDEF {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for rec in btree_records(r, hdr.root_addr, &hdr, hdr.depth, hdr.root_nrec as usize)? {
        if rec.len() < 4 + heap.id_len {
            return Err(Hdf5Note::Chunk { off: 0 });
        }
        if rec[4..].iter().all(|&b| b == 0) {
            continue;
        }
        let id = &rec[4..4 + heap.id_len];
        let raw = heap_read_id(r, &heap, id)?;
        let link = parse_link(&raw, 0)?;
        out.push(link);
    }
    Ok(out)
}

fn read_symtab_group<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    msg: &RawMessage,
) -> Result<Vec<Hdf5Link>, Hdf5Note> {
    if msg.data.len() < 16 {
        return Err(Hdf5Note::EndAtByte { off: 0 });
    }
    let btree_addr = le_u64(&msg.data, 0);
    let heap_addr = le_u64(&msg.data, 8);
    let heap = r.read(heap_addr, 32)?;
    if heap.len() < 32 || &heap[..4] != b"HEAP" {
        let mut found = [0u8; 4];
        found.copy_from_slice(heap.get(..4).unwrap_or(b"    "));
        return Err(Hdf5Note::Heap {
            found,
            off: heap_addr as usize,
        });
    }
    let data_seg_addr = le_u64(&heap, 24) as usize;
    let mut entries = Vec::new();
    walk_symtab_nodes(r, btree_addr, &mut entries)?;
    let mut links = Vec::new();
    for e in entries {
        let noff = data_seg_addr + e.name_offset as usize;
        let win = r.read(noff as u64, 512)?;
        let name = if win.contains(&0) {
            read_null_name(&win, 0)
        } else {
            let wide = r.read(noff as u64, 4096)?;
            if wide.contains(&0) {
                read_null_name(&wide, 0)
            } else {
                return Err(Hdf5Note::EndAtByte {
                    off: noff + wide.len(),
                });
            }
        };
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

fn walk_symtab_nodes<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    addr: u64,
    out: &mut Vec<SymbolEntry>,
) -> Result<(), Hdf5Note> {
    let off = addr as usize;
    let sig = r.read(addr, 4)?;
    if sig.len() < 4 {
        return Err(Hdf5Note::EndAtByte {
            off: off + sig.len(),
        });
    }
    match &sig[..] {
        b"SNOD" => {
            let head = r.read(addr, 8)?;
            if head.len() < 8 {
                return Err(Hdf5Note::EndAtByte {
                    off: off + head.len(),
                });
            }
            let n = le_u16(&head, 6) as usize;
            let span = 8usize
                .checked_add(40 * n)
                .ok_or(Hdf5Note::EndAtByte { off })?;
            let buf = r.read(addr, span as u64)?;
            if buf.len() < span {
                return Err(Hdf5Note::EndAtByte {
                    off: off + buf.len(),
                });
            }
            let mut p = 8usize;
            for _ in 0..n {
                out.push(SymbolEntry {
                    name_offset: le_u64(&buf, p),
                    obj_addr: le_u64(&buf, p + 8),
                });
                p += 40;
            }
            Ok(())
        }
        b"TREE" => {
            let head = r.read(addr, 24)?;
            if head.len() < 24 {
                return Err(Hdf5Note::EndAtByte {
                    off: off + head.len(),
                });
            }
            let node_type = head[4];
            let entries = le_u16(&head, 6) as usize;
            if node_type != 0 {
                return Err(Hdf5Note::Btree {
                    typ: node_type,
                    off,
                });
            }
            let span = 24usize
                .checked_add(16 * entries)
                .ok_or(Hdf5Note::EndAtByte { off })?;
            let buf = r.read(addr, span as u64)?;
            if buf.len() < span {
                return Err(Hdf5Note::EndAtByte {
                    off: off + buf.len(),
                });
            }
            let mut p = 24usize;
            for _ in 0..entries {
                p += 8;
                let child = le_u64(&buf, p);
                p += 8;
                if child != UNDEF {
                    walk_symtab_nodes(r, child, out)?;
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

fn dense_attrs<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    msgs: &[RawMessage],
) -> Result<Vec<Hdf5Attribute>, Hdf5Note> {
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
        let heap = parse_fractal_heap(r, fh)?;
        let (_, hdr) = parse_btree_header(r, name_bt)?;
        if hdr.root_addr == UNDEF {
            continue;
        }
        for rec in btree_records(r, hdr.root_addr, &hdr, hdr.depth, hdr.root_nrec as usize)? {
            if rec.len() < heap.id_len {
                return Err(Hdf5Note::Chunk { off: 0 });
            }
            if rec[4..].iter().all(|&b| b == 0) {
                continue;
            }
            let id = &rec[..heap.id_len];
            let raw = heap_read_id(r, &heap, id)?;
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
                    && ((data[0] as u16) << 8 | data[1] as u16).is_multiple_of(31)
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
                if elem_size == 0 {
                    return Err(Hdf5Note::Filter { id: f.id, off: 0 });
                }
                let n = data.len() / elem_size;
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
                if !body.len().is_multiple_of(2) {
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
                let sf = f.cd_values.get(1).copied();
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
                let fill_defined = match f.cd_values.get(7) {
                    Some(&v) => v == 1,
                    None => false,
                };
                let fill: Vec<u8> = if fill_defined {
                    let mut v = vec![0u8; elem_size];
                    for (i, slot) in v.iter_mut().enumerate() {
                        let word = match f.cd_values.get(8 + i / 4) {
                            Some(&w) => w,
                            None => 0,
                        };
                        *slot = word.to_le_bytes()[i % 4];
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
                        let sf = match sf {
                            Some(v) => v,
                            None => return Err(Hdf5Note::Filter { id: f.id, off: 0 }),
                        };
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

fn unallocated_contiguous(fill_defined: bool, fill: &[u8], len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    if fill_defined && !fill.is_empty() {
        for (i, b) in out.iter_mut().enumerate() {
            *b = fill[i % fill.len()];
        }
    }
    out
}

#[derive(Clone, Debug)]
struct ChunkRec {
    addr: u64,
    size: usize,
    filter_mask: u32,
    scaled: Vec<u64>,
}

fn v1_chunk_records<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    addr: u64,
    rank: usize,
    head: &[u8],
) -> Result<Vec<ChunkRec>, Hdf5Note> {
    let mut out = Vec::new();
    walk_v1_chunk_node(r, addr, rank, head, &mut out)?;
    Ok(out)
}

fn walk_v1_chunk_node<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    addr: u64,
    rank: usize,
    head: &[u8],
    out: &mut Vec<ChunkRec>,
) -> Result<(), Hdf5Note> {
    let off = addr as usize;
    if head.len() < 24 {
        return Err(Hdf5Note::EndAtByte {
            off: off + head.len(),
        });
    }
    if &head[..4] != b"TREE" {
        let mut found = [0u8; 4];
        found.copy_from_slice(&head[..4]);
        return Err(Hdf5Note::BtreeNode { found, off });
    }
    let node_type = head[4];
    let level = head[5];
    let nchildren = le_u16(&head, 6) as usize;
    if node_type != 1 {
        return Err(Hdf5Note::Btree {
            typ: node_type,
            off,
        });
    }
    let key_size = 8 + (rank + 1) * 8;
    let span = 24usize
        .checked_add(
            nchildren
                .checked_mul(key_size + 8)
                .ok_or(Hdf5Note::EndAtByte { off })?,
        )
        .ok_or(Hdf5Note::EndAtByte { off })?;
    if nchildren == 0 {
        return Ok(());
    }
    let body = r.read(addr + 24, (span - 24) as u64)?;
    let mut p = 0usize;
    for _ in 0..nchildren {
        let nbytes = le_u32(&body, p) as usize;
        let filter_mask = le_u32(&body, p + 4);
        let mut scaled = Vec::with_capacity(rank);
        for d in 0..rank {
            scaled.push(le_u64(&body, p + 8 + d * 8));
        }
        let child = le_u64(&body, p + key_size);
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
            let child_head = r.read(child, 24)?;
            walk_v1_chunk_node(r, child, rank, &child_head, out)?;
        }
    }
    Ok(())
}

fn chunk_records<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    r: &mut Hdf5WindowReader<F>,
    hdr: &BtreeHeader,
    dims: usize,
    filtered: bool,
) -> Result<Vec<ChunkRec>, Hdf5Note> {
    let mut out = Vec::new();
    if hdr.root_addr == UNDEF {
        return Ok(out);
    }
    for rec in btree_records(r, hdr.root_addr, hdr, hdr.depth, hdr.root_nrec as usize)? {
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

fn scaled_to_coords(scaled: &[u64], chunk_dims: &[u32], v1_index: bool) -> Option<Vec<u64>> {
    if scaled.len() != chunk_dims.len() {
        return None;
    }
    let mut out = Vec::with_capacity(scaled.len());
    for (s, c) in scaled.iter().zip(chunk_dims.iter()) {
        if v1_index {
            let c = *c as u64;
            if c == 0 || s % c != 0 {
                return None;
            }
            out.push(s / c);
        } else {
            out.push(*s);
        }
    }
    Some(out)
}

fn chunk_records_with<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    reader: &mut Hdf5WindowReader<F>,
    btree: u64,
    rank: usize,
    filtered: bool,
) -> Result<(Vec<ChunkRec>, bool), Hdf5Note> {
    let head = reader.read(btree, 24)?;
    if &head[..4] == b"BTHD" {
        let mut hbuf = head;
        let tail = reader.read(btree + 24, 14)?;
        hbuf.extend_from_slice(&tail);
        let (typ, hdr) = parse_btree_header_bytes(&hbuf, btree)?;
        if typ != 10 && typ != 11 {
            return Err(Hdf5Note::Btree {
                typ,
                off: btree as usize,
            });
        }
        Ok((chunk_records(reader, &hdr, rank, filtered)?, false))
    } else {
        Ok((v1_chunk_records(reader, btree, rank, &head)?, true))
    }
}

fn chunk_read_plan(
    obj: &Hdf5Object,
    ds: &Hdf5Dataspace,
    dt: &Hdf5Datatype,
    coords: &[u64],
) -> Result<(Vec<u32>, u64, bool), ChunkReadDiag> {
    if dt.class == 9 {
        return Err(ChunkReadDiag {
            stage: "vlen dataset",
            note: Some(Hdf5Note::VlenNotRead),
        });
    }
    let elem_size = dt.size;
    let rank = ds.dims.len();
    if rank == 0 {
        return Err(ChunkReadDiag {
            stage: "rank zero",
            note: None,
        });
    }
    if coords.len() != rank {
        return Err(ChunkReadDiag {
            stage: "coords rank mismatch",
            note: None,
        });
    }
    let (chunk_dims, btree) = match obj.layout.as_ref() {
        Some(Hdf5Layout::Chunked {
            chunk_dims,
            elem_size: declared,
            btree,
        }) => {
            if *declared as usize != elem_size {
                return Err(ChunkReadDiag {
                    stage: "declared elem size mismatch",
                    note: None,
                });
            }
            (chunk_dims.clone(), *btree)
        }
        _ => {
            return Err(ChunkReadDiag {
                stage: "layout not chunked",
                note: None,
            });
        }
    };
    Ok((chunk_dims, btree, !obj.filters.is_empty()))
}

fn read_chunk_resolved<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    base: &[u8],
    obj: &Hdf5Object,
    ds: &Hdf5Dataspace,
    dt: &Hdf5Datatype,
    coords: &[u64],
    mut fetch: F,
) -> Result<Vec<f64>, ChunkReadDiag> {
    let (chunk_dims, btree, filtered) = chunk_read_plan(obj, ds, dt, coords)?;
    let rank = ds.dims.len();
    let mut reader = Hdf5WindowReader::new(base, &mut fetch);
    let (recs, v1_index) =
        chunk_records_with(&mut reader, btree, rank, filtered).map_err(|note| ChunkReadDiag {
            stage: "chunk index read",
            note: Some(note),
        })?;
    drop(reader);
    chunk_values_from(&recs, v1_index, obj, ds, dt, coords, &chunk_dims, fetch)
}

fn chunk_values_from<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    recs: &[ChunkRec],
    v1_index: bool,
    obj: &Hdf5Object,
    ds: &Hdf5Dataspace,
    dt: &Hdf5Datatype,
    coords: &[u64],
    chunk_dims: &[u32],
    mut fetch: F,
) -> Result<Vec<f64>, ChunkReadDiag> {
    let elem_size = dt.size;
    let rank = ds.dims.len();
    let rec = recs
        .iter()
        .find(|r| scaled_to_coords(&r.scaled, chunk_dims, v1_index).as_deref() == Some(coords))
        .ok_or(ChunkReadDiag {
            stage: "chunk not found",
            note: None,
        })?;
    let chunk_elems: usize = chunk_dims.iter().fold(1usize, |a, d| a * (*d as usize));
    let stored_len = if rec.size > 0 {
        rec.size
    } else {
        chunk_elems * elem_size
    };
    let mut raw = fetch(rec.addr, stored_len as u64).ok_or(ChunkReadDiag {
        stage: "chunk data fetch void",
        note: Some(Hdf5Note::AbsentAtByte {
            off: rec.addr as usize,
        }),
    })?;
    if !obj.filters.is_empty() {
        apply_filters(&mut raw, &obj.filters, elem_size, rec.filter_mask).map_err(|note| {
            ChunkReadDiag {
                stage: "chunk filter",
                note: Some(note),
            }
        })?;
    }
    let mut actual_elems = 1usize;
    for d in 0..rank {
        let start = coords[d]
            .checked_mul(chunk_dims[d] as u64)
            .ok_or(ChunkReadDiag {
                stage: "coords overflow",
                note: None,
            })?;
        let avail = ds.dims[d].saturating_sub(start);
        actual_elems *= avail.min(chunk_dims[d] as u64) as usize;
    }
    let raw_elems = raw.len() / elem_size;
    let n = raw_elems.min(actual_elems);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(decode_numeric(&raw, i, dt).map_err(|note| ChunkReadDiag {
            stage: "numeric decode",
            note: Some(note),
        })?);
    }
    let scale = obj
        .attrs
        .iter()
        .find(|a| a.name == "scale_factor")
        .and_then(attr_number);
    if let Some(scale) = scale {
        let offset = obj
            .attrs
            .iter()
            .find(|a| a.name == "add_offset")
            .and_then(attr_number);
        for v in out.iter_mut() {
            *v = match offset {
                Some(o) => *v * scale + o,
                None => *v * scale,
            };
        }
    }
    Ok(out)
}

const WGS84_EQUATORIAL_M: f64 = 6_378_137.0;
const WGS84_POLAR_M: f64 = 6_356_752.314_245;

#[derive(Clone, Debug)]
pub struct GeostationaryProjection {
    pub sub_longitude_deg: f64,
    pub perspective_height_m: f64,
    pub sweep_angle_axis: Option<String>,
}

pub fn geostationary_lat_lon(
    x_rad: f64,
    y_rad: f64,
    sub_longitude_deg: f64,
    perspective_height_m: f64,
) -> (f64, f64) {
    let h = perspective_height_m + WGS84_EQUATORIAL_M;
    let ratio = WGS84_EQUATORIAL_M / WGS84_POLAR_M;
    let cosx = x_rad.cos();
    let cosy = y_rad.cos();
    let sinx = x_rad.sin();
    let siny = y_rad.sin();
    let a = sinx * sinx + cosx * cosx * (cosy * cosy + ratio * ratio * siny * siny);
    let b = -2.0 * h * cosx * cosy;
    let c = h * h - WGS84_EQUATORIAL_M * WGS84_EQUATORIAL_M;
    let r_s = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    let s_x = r_s * cosx * cosy;
    let s_y = -r_s * sinx;
    let s_z = r_s * cosx * siny;
    let lat = (ratio * ratio * s_z / ((h - s_x).powi(2) + s_y * s_y).sqrt()).atan();
    let lon = sub_longitude_deg.to_radians() - (s_y / (h - s_x)).atan();
    (lat.to_degrees(), lon.to_degrees())
}

pub fn geostationary_lat_lon_grid(
    x_scan: &[f64],
    y_scan: &[f64],
    sub_longitude_deg: f64,
    perspective_height_m: f64,
) -> (Vec<f64>, Vec<f64>) {
    let mut lats = Vec::with_capacity(x_scan.len() * y_scan.len());
    let mut lons = Vec::with_capacity(x_scan.len() * y_scan.len());
    for &y in y_scan {
        for &x in x_scan {
            let (lat, lon) = geostationary_lat_lon(x, y, sub_longitude_deg, perspective_height_m);
            lats.push(lat);
            lons.push(lon);
        }
    }
    (lats, lons)
}

fn attr_number(a: &Hdf5Attribute) -> Option<f64> {
    match a.datatype.class {
        1 => match a.datatype.size {
            4 => decode_f32(&a.data, 0, a.datatype.endian).map(|v| v as f64),
            8 => decode_f64(&a.data, 0, a.datatype.endian),
            _ => None,
        },
        _ => None,
    }
}

fn decode_numeric(raw: &[u8], i: usize, dt: &Hdf5Datatype) -> Result<f64, Hdf5Note> {
    let off = i * dt.size;
    match dt.class {
        0 => {
            let v = if dt.signed {
                match dt.size {
                    1 => raw.get(off).map(|&b| b as i8 as f64),
                    2 => raw
                        .get(off..off + 2)
                        .and_then(|s| Some(i16::from_le_bytes(s.try_into().ok()?) as f64)),
                    4 => raw
                        .get(off..off + 4)
                        .and_then(|s| Some(i32::from_le_bytes(s.try_into().ok()?) as f64)),
                    8 => raw
                        .get(off..off + 8)
                        .and_then(|s| Some(i64::from_le_bytes(s.try_into().ok()?) as f64)),
                    _ => None,
                }
            } else {
                match dt.size {
                    1 => raw.get(off).map(|&b| b as f64),
                    2 => raw
                        .get(off..off + 2)
                        .and_then(|s| Some(u16::from_le_bytes(s.try_into().ok()?) as f64)),
                    4 => raw
                        .get(off..off + 4)
                        .and_then(|s| Some(u32::from_le_bytes(s.try_into().ok()?) as f64)),
                    8 => raw
                        .get(off..off + 8)
                        .and_then(|s| Some(u64::from_le_bytes(s.try_into().ok()?) as f64)),
                    _ => None,
                }
            };
            v.ok_or(Hdf5Note::EndAtByte { off })
        }
        1 => match dt.size {
            4 => decode_f32(raw, off, dt.endian)
                .map(|v| v as f64)
                .ok_or(Hdf5Note::EndAtByte { off }),
            8 => decode_f64(raw, off, dt.endian).ok_or(Hdf5Note::EndAtByte { off }),
            _ => Err(Hdf5Note::Datatype {
                class: dt.class,
                off,
            }),
        },
        _ => Err(Hdf5Note::Datatype {
            class: dt.class,
            off,
        }),
    }
}

impl<'a> Hdf5File<'a> {
    pub fn parse(buf: &'a [u8]) -> Result<Hdf5File<'a>, Hdf5Note> {
        Hdf5File::parse_fetch(buf, |_off: u64, _len: u64| -> Option<Vec<u8>> { None })
    }

    pub fn parse_fetch<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
        buf: &'a [u8],
        fetch: F,
    ) -> Result<Hdf5File<'a>, Hdf5Note> {
        let sb = parse_superblock(buf)?;
        let offset_size = sb.offset_size;
        let length_size = sb.length_size;
        let mut reader = Hdf5WindowReader::new(buf, fetch);
        let mut objects: HashMap<u64, Hdf5Object> = HashMap::new();
        let mut stack = vec![sb.root];
        while let Some(addr) = stack.pop() {
            if addr == UNDEF || objects.contains_key(&addr) {
                continue;
            }
            let obj = parse_object_header(&mut reader, addr, offset_size, length_size)?;
            if let Some(c) = obj.committed_datatype {
                stack.push(c);
            }
            for l in &obj.links {
                if l.addr != UNDEF {
                    stack.push(l.addr);
                }
            }
            objects.insert(addr, obj);
        }
        let committed: Vec<(u64, u64)> = objects
            .iter()
            .filter_map(|(&a, o)| o.committed_datatype.map(|c| (a, c)))
            .collect();
        for (obj_addr, committed_addr) in committed {
            let dt = objects
                .get(&committed_addr)
                .and_then(|o| o.datatype.clone());
            if let Some(o) = objects.get_mut(&obj_addr) {
                o.datatype = dt;
            }
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

    pub fn root_header_diag(&self) -> Option<&HeaderDiag> {
        self.objects.get(&self.root).map(|o| &o.header)
    }

    pub fn resolve(&self, path: &str) -> Result<&Hdf5Object, Hdf5Note> {
        let mut current = self.root()?;
        for part in path.split('/').filter(|p| !p.is_empty()) {
            let link = current
                .links
                .iter()
                .find(|l| l.name == part)
                .ok_or_else(|| Hdf5Note::AbsentObject {
                    name: part.to_string(),
                })?;
            if link.addr == UNDEF {
                return Err(Hdf5Note::AbsentObject {
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
            .ok_or_else(|| Hdf5Note::AbsentObject {
                name: name.to_string(),
            })?;
        let dt = obj
            .datatype
            .as_ref()
            .ok_or_else(|| Hdf5Note::AbsentObject {
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
        match self.resolve(path) {
            Ok(o) => o.links.iter().collect(),
            Err(_) => Vec::new(),
        }
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
                let len = count * elem_size;
                if *size > 0 && *size as usize != len {
                    return Err(Hdf5Note::Chunk {
                        off: *addr as usize,
                    });
                }
                if *addr == UNDEF {
                    return Ok(unallocated_contiguous(obj.fill_defined, &obj.fill, len));
                }
                let off = *addr as usize;
                let Some(end) = off.checked_add(len).filter(|e| *e <= self.buf.len()) else {
                    return Err(Hdf5Note::EndAtByte {
                        off: self.buf.len(),
                    });
                };
                Ok(self.buf[off..end].to_vec())
            }
            Some(Hdf5Layout::Chunked {
                btree: _,
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
                    self.chunk_records_of(
                        obj,
                        rank,
                        &mut |_off: u64, _len: u64| -> Option<Vec<u8>> { None },
                    )?;
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
                    let mut raw = {
                        let off = rec.addr as usize;
                        let len = if rec.size > 0 {
                            rec.size
                        } else {
                            chunk_elems * elem_size
                        };
                        let Some(end) = off.checked_add(len).filter(|e| *e <= self.buf.len())
                        else {
                            return Err(Hdf5Note::EndAtByte {
                                off: self.buf.len(),
                            });
                        };
                        self.buf[off..end].to_vec()
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
            None => Err(Hdf5Note::AbsentObject {
                name: name.to_string(),
            }),
        }
    }

    pub fn read_f64_dataset(&self, name: &str) -> Result<Vec<f64>, Hdf5Note> {
        let (obj, ds, dt) = self.dataset(name)?;
        let raw = self.read_dataset(name)?;
        let count: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
        let mut out = Vec::with_capacity(count);
        for i in 0..count {
            out.push(decode_numeric(&raw, i, dt)?);
        }
        let scale = obj
            .attrs
            .iter()
            .find(|a| a.name == "scale_factor")
            .and_then(attr_number);
        if let Some(scale) = scale {
            let offset = obj
                .attrs
                .iter()
                .find(|a| a.name == "add_offset")
                .and_then(attr_number);
            for v in out.iter_mut() {
                *v = match offset {
                    Some(o) => *v * scale + o,
                    None => *v * scale,
                };
            }
        }
        Ok(out)
    }

    pub fn attr_f64(&self, name: &str, attr: &str) -> Option<f64> {
        self.attribute(name, attr).and_then(attr_number)
    }

    pub fn dims(&self, name: &str) -> Option<Vec<u64>> {
        self.dataset(name).ok().map(|(_, ds, _)| ds.dims.clone())
    }

    fn chunk_records_of(
        &self,
        obj: &Hdf5Object,
        rank: usize,
        fetch: &mut impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Result<(Vec<ChunkRec>, bool), Hdf5Note> {
        let btree = match obj.layout.as_ref() {
            Some(Hdf5Layout::Chunked { btree, .. }) => *btree,
            _ => {
                return Err(Hdf5Note::AbsentObject {
                    name: String::new(),
                });
            }
        };
        let filtered = !obj.filters.is_empty();
        let mut reader = Hdf5WindowReader::new(self.buf, fetch);
        chunk_records_with(&mut reader, btree, rank, filtered)
    }

    pub fn chunk_index(&self, dataset: &str) -> Option<Vec<(Vec<u64>, u64)>> {
        let (obj, ds, _dt) = self.dataset(dataset).ok()?;
        let rank = ds.dims.len();
        if rank == 0 {
            return None;
        }
        let chunk_dims = match obj.layout.as_ref()? {
            Hdf5Layout::Chunked { chunk_dims, .. } => chunk_dims,
            _ => return None,
        };
        let (recs, v1_index) = self
            .chunk_records_of(obj, rank, &mut |_off: u64, _len: u64| -> Option<Vec<u8>> {
                None
            })
            .ok()?;
        let mut out = Vec::with_capacity(recs.len());
        for rec in recs {
            let coords = match scaled_to_coords(&rec.scaled, chunk_dims, v1_index) {
                Some(c) => c,
                None => continue,
            };
            out.push((coords, rec.addr));
        }
        Some(out)
    }

    pub fn read_chunk(
        &self,
        dataset: &str,
        coords: &[u64],
        fetch: impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Option<Vec<f64>> {
        self.read_chunk_impl(dataset, coords, fetch).ok()
    }

    pub fn read_chunk_diag(
        &self,
        dataset: &str,
        coords: &[u64],
        fetch: impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Result<Vec<f64>, ChunkReadDiag> {
        self.read_chunk_impl(dataset, coords, fetch)
    }

    fn read_chunk_impl(
        &self,
        dataset: &str,
        coords: &[u64],
        fetch: impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Result<Vec<f64>, ChunkReadDiag> {
        let (obj, ds, dt) = self.dataset(dataset).map_err(|note| ChunkReadDiag {
            stage: "dataset absent",
            note: Some(note),
        })?;
        read_chunk_resolved(self.buf, obj, ds, dt, coords, fetch)
    }

    pub fn geostationary_projection(&self) -> Result<GeostationaryProjection, Hdf5Note> {
        if let Ok(proj) = self.resolve("goes_imager_projection") {
            let sub_lon = proj
                .attrs
                .iter()
                .find(|a| a.name == "longitude_of_projection_origin")
                .and_then(attr_number);
            let height = proj
                .attrs
                .iter()
                .find(|a| a.name == "perspective_point_height")
                .and_then(attr_number);
            if let (Some(sub_lon), Some(height)) = (sub_lon, height) {
                return Ok(GeostationaryProjection {
                    sub_longitude_deg: sub_lon,
                    perspective_height_m: height,
                    sweep_angle_axis: proj
                        .attrs
                        .iter()
                        .find(|a| a.name == "sweep_angle_axis")
                        .map(|a| byte_str(&a.data)),
                });
            }
        }
        for path in ["image_pixel_values", ""] {
            let Ok(obj) = self.resolve(path) else {
                continue;
            };
            let sub_lon = obj
                .attrs
                .iter()
                .find(|a| a.name == "sub_longitude")
                .and_then(attr_number);
            let height = obj
                .attrs
                .iter()
                .find(|a| a.name == "nominal_satellite_height")
                .and_then(attr_number);
            if let (Some(sub_lon), Some(height)) = (sub_lon, height) {
                return Ok(GeostationaryProjection {
                    sub_longitude_deg: sub_lon.to_degrees(),
                    perspective_height_m: height - WGS84_EQUATORIAL_M,
                    sweep_angle_axis: obj
                        .attrs
                        .iter()
                        .find(|a| a.name == "sweep_angle_axis")
                        .map(|a| byte_str(&a.data)),
                });
            }
        }
        Err(Hdf5Note::AbsentObject {
            name: "geostationary_projection".to_string(),
        })
    }
}

pub trait Hdf5Access {
    fn resolve<'s>(&'s mut self, path: &str) -> Result<&'s Hdf5Object, Hdf5Note>;
    fn dataset<'s>(
        &'s mut self,
        name: &str,
    ) -> Result<(&'s Hdf5Object, &'s Hdf5Dataspace, &'s Hdf5Datatype), Hdf5Note>;
    fn attribute<'s>(&'s mut self, name: &str, attr: &str) -> Option<&'s Hdf5Attribute>;
    fn root_header_diag<'s>(&'s mut self) -> Option<&'s HeaderDiag>;
    fn read_chunk(
        &mut self,
        dataset: &str,
        coords: &[u64],
        fetch: impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Option<Vec<f64>>;
    fn read_chunk_diag(
        &mut self,
        dataset: &str,
        coords: &[u64],
        fetch: impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Result<Vec<f64>, ChunkReadDiag>;
}

impl<'a> Hdf5Access for Hdf5File<'a> {
    fn resolve<'s>(&'s mut self, path: &str) -> Result<&'s Hdf5Object, Hdf5Note> {
        Hdf5File::resolve(self, path)
    }

    fn dataset<'s>(
        &'s mut self,
        name: &str,
    ) -> Result<(&'s Hdf5Object, &'s Hdf5Dataspace, &'s Hdf5Datatype), Hdf5Note> {
        Hdf5File::dataset(self, name)
    }

    fn attribute<'s>(&'s mut self, name: &str, attr: &str) -> Option<&'s Hdf5Attribute> {
        Hdf5File::attribute(self, name, attr)
    }

    fn root_header_diag<'s>(&'s mut self) -> Option<&'s HeaderDiag> {
        Hdf5File::root_header_diag(self)
    }

    fn read_chunk(
        &mut self,
        dataset: &str,
        coords: &[u64],
        fetch: impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Option<Vec<f64>> {
        Hdf5File::read_chunk(self, dataset, coords, fetch)
    }

    fn read_chunk_diag(
        &mut self,
        dataset: &str,
        coords: &[u64],
        fetch: impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Result<Vec<f64>, ChunkReadDiag> {
        Hdf5File::read_chunk_diag(self, dataset, coords, fetch)
    }
}

pub struct LazyHdf5<'a, F: FnMut(u64, u64) -> Option<Vec<u8>>> {
    reader: Hdf5WindowReader<'a, F>,
    objects: HashMap<u64, Hdf5Object>,
    chunk_index_cache: HashMap<u64, (Vec<ChunkRec>, bool)>,
    root: u64,
    offset_size: usize,
    length_size: usize,
}

impl<'a, F: FnMut(u64, u64) -> Option<Vec<u8>>> LazyHdf5<'a, F> {
    pub fn open(buf: &'a [u8], fetch: F) -> Result<LazyHdf5<'a, F>, Hdf5Note> {
        let sb = parse_superblock(buf)?;
        Ok(LazyHdf5 {
            reader: Hdf5WindowReader::new(buf, fetch),
            objects: HashMap::new(),
            chunk_index_cache: HashMap::new(),
            root: sb.root,
            offset_size: sb.offset_size,
            length_size: sb.length_size,
        })
    }

    fn ensure_object(&mut self, addr: u64) -> Result<(), Hdf5Note> {
        if self.objects.contains_key(&addr) {
            return Ok(());
        }
        let (offset_size, length_size) = (self.offset_size, self.length_size);
        let obj = parse_object_header(&mut self.reader, addr, offset_size, length_size)?;
        self.objects.insert(addr, obj);
        let committed = self.objects[&addr].committed_datatype;
        if let Some(c) = committed {
            if let Err(note) = self.ensure_object(c) {
                self.objects.remove(&addr);
                return Err(note);
            }
            let dt = self.objects.get(&c).and_then(|o| o.datatype.clone());
            self.objects.get_mut(&addr).expect("inserted").datatype = dt;
        }
        Ok(())
    }

    fn path_addr(&mut self, path: &str) -> Result<u64, Hdf5Note> {
        self.ensure_object(self.root)?;
        let mut current = self.root;
        for part in path.split('/').filter(|p| !p.is_empty()) {
            let addr = match self.objects[&current].links.iter().find(|l| l.name == part) {
                Some(l) if l.addr != UNDEF => l.addr,
                _ => {
                    return Err(Hdf5Note::AbsentObject {
                        name: part.to_string(),
                    });
                }
            };
            self.ensure_object(addr)?;
            current = addr;
        }
        Ok(current)
    }

    pub fn resolve<'s>(&'s mut self, path: &str) -> Result<&'s Hdf5Object, Hdf5Note> {
        let addr = self.path_addr(path)?;
        self.objects
            .get(&addr)
            .ok_or(Hdf5Note::Address { off: addr as usize })
    }

    pub fn dataset<'s>(
        &'s mut self,
        name: &str,
    ) -> Result<(&'s Hdf5Object, &'s Hdf5Dataspace, &'s Hdf5Datatype), Hdf5Note> {
        let addr = self.path_addr(name)?;
        let obj = self
            .objects
            .get(&addr)
            .ok_or(Hdf5Note::Address { off: addr as usize })?;
        let ds = obj
            .dataspace
            .as_ref()
            .ok_or_else(|| Hdf5Note::AbsentObject {
                name: name.to_string(),
            })?;
        let dt = obj
            .datatype
            .as_ref()
            .ok_or_else(|| Hdf5Note::AbsentObject {
                name: name.to_string(),
            })?;
        Ok((obj, ds, dt))
    }

    pub fn attribute<'s>(&'s mut self, name: &str, attr: &str) -> Option<&'s Hdf5Attribute> {
        let addr = self.path_addr(name).ok()?;
        self.objects
            .get(&addr)?
            .attrs
            .iter()
            .find(|a| a.name == attr)
    }

    pub fn root_header_diag<'s>(&'s mut self) -> Option<&'s HeaderDiag> {
        let root = self.root;
        self.ensure_object(root).ok()?;
        self.objects.get(&root).map(|o| &o.header)
    }

    pub fn read_chunk(
        &mut self,
        dataset: &str,
        coords: &[u64],
        fetch: impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Option<Vec<f64>> {
        self.read_chunk_diag(dataset, coords, fetch).ok()
    }

    pub fn read_chunk_diag(
        &mut self,
        dataset: &str,
        coords: &[u64],
        mut fetch: impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Result<Vec<f64>, ChunkReadDiag> {
        let base = self.reader.base;
        let (obj, ds, dt) = self.dataset(dataset).map_err(|note| ChunkReadDiag {
            stage: "dataset absent",
            note: Some(note),
        })?;
        let obj = obj.clone();
        let ds = ds.clone();
        let dt = dt.clone();
        let (chunk_dims, btree, filtered) = chunk_read_plan(&obj, &ds, &dt, coords)?;
        let rank = ds.dims.len();
        let (recs, v1_index) = match self.chunk_index_cache.get(&btree) {
            Some(cached) => cached.clone(),
            None => {
                let mut reader = Hdf5WindowReader::new(base, &mut fetch);
                let recs_v1 =
                    chunk_records_with(&mut reader, btree, rank, filtered).map_err(|note| {
                        ChunkReadDiag {
                            stage: "chunk index read",
                            note: Some(note),
                        }
                    })?;
                self.chunk_index_cache.insert(btree, recs_v1.clone());
                recs_v1
            }
        };
        chunk_values_from(&recs, v1_index, &obj, &ds, &dt, coords, &chunk_dims, fetch)
    }

    pub fn read_dataset(&mut self, name: &str) -> Result<Vec<u8>, Hdf5Note> {
        let obj = self.resolve(name)?.clone();
        let ds = obj
            .dataspace
            .as_ref()
            .ok_or_else(|| Hdf5Note::AbsentObject {
                name: name.to_string(),
            })?;
        let dt = obj
            .datatype
            .as_ref()
            .ok_or_else(|| Hdf5Note::AbsentObject {
                name: name.to_string(),
            })?;
        if dt.class == 9 {
            return Err(Hdf5Note::VlenNotRead);
        }
        let elem_size = dt.size;
        let count: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
        match obj.layout.as_ref() {
            Some(Hdf5Layout::Compact { data }) => Ok(data.clone()),
            Some(Hdf5Layout::Contiguous { addr, size }) => {
                let len = count * elem_size;
                if *size > 0 && *size as usize != len {
                    return Err(Hdf5Note::Chunk {
                        off: *addr as usize,
                    });
                }
                if *addr == UNDEF {
                    return Ok(unallocated_contiguous(obj.fill_defined, &obj.fill, len));
                }
                Ok(self.reader.read(*addr, len as u64)?)
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
                let btree = *btree;
                let chunk_dims = chunk_dims.clone();
                let filtered = !obj.filters.is_empty();
                let (recs, v1_index) = chunk_records_with(&mut self.reader, btree, rank, filtered)?;
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
                    let len = if rec.size > 0 {
                        rec.size
                    } else {
                        chunk_elems * elem_size
                    };
                    let mut raw = self.reader.read(rec.addr, len as u64)?;
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
            None => Err(Hdf5Note::AbsentObject {
                name: name.to_string(),
            }),
        }
    }
}

impl<'a, F: FnMut(u64, u64) -> Option<Vec<u8>>> Hdf5Access for LazyHdf5<'a, F> {
    fn resolve<'s>(&'s mut self, path: &str) -> Result<&'s Hdf5Object, Hdf5Note> {
        LazyHdf5::resolve(self, path)
    }

    fn dataset<'s>(
        &'s mut self,
        name: &str,
    ) -> Result<(&'s Hdf5Object, &'s Hdf5Dataspace, &'s Hdf5Datatype), Hdf5Note> {
        LazyHdf5::dataset(self, name)
    }

    fn attribute<'s>(&'s mut self, name: &str, attr: &str) -> Option<&'s Hdf5Attribute> {
        LazyHdf5::attribute(self, name, attr)
    }

    fn root_header_diag<'s>(&'s mut self) -> Option<&'s HeaderDiag> {
        LazyHdf5::root_header_diag(self)
    }

    fn read_chunk(
        &mut self,
        dataset: &str,
        coords: &[u64],
        fetch: impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Option<Vec<f64>> {
        LazyHdf5::read_chunk(self, dataset, coords, fetch)
    }

    fn read_chunk_diag(
        &mut self,
        dataset: &str,
        coords: &[u64],
        fetch: impl FnMut(u64, u64) -> Option<Vec<u8>>,
    ) -> Result<Vec<f64>, ChunkReadDiag> {
        LazyHdf5::read_chunk_diag(self, dataset, coords, fetch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const SSI_2026: &str =
        "phi/pipeline/catalog/ncei_ssi/ssi_v03r00-preliminary_monthly_s202604_e202606_c20260804.nc";
    const SSI_1874: &str =
        "phi/pipeline/catalog/ncei_ssi/ssi_v03r00_monthly_s187405_e187412_c20240831.nc";
    const FILTERS: &str = "phi/pipeline/catalog/ncei_ssi/filters.h5";
    const GOES_XRS: &str =
        "phi/pipeline/catalog/ncei_goes_xrs/sci_xrsf-l2-avg1m_g14_d20200101_v2-2-1.nc";
    const GOES16_ABI: &str = "phi/pipeline/catalog/noaa_goes16/OR_ABI-L1b-RadC-M6C01_G16_s20240010001173_e20240010003546_c20240010004005.nc";
    const GK2A_AMI: &str =
        "phi/pipeline/catalog/noaa_gk2a/gk2a_ami_le1b_ir087_fd020ge_202302160000.nc";

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

    fn synthetic_chunked_image() -> (Vec<u8>, u64) {
        fn message(typ: u8, data: Vec<u8>) -> Vec<u8> {
            let mut m = vec![typ, data.len() as u8, (data.len() >> 8) as u8, 0];
            m.extend_from_slice(&data);
            m
        }
        fn header(messages: Vec<Vec<u8>>) -> Vec<u8> {
            let mut body = vec![b'O', b'H', b'D', b'R', 2, 0];
            let m: usize = messages.iter().map(|x| x.len()).sum();
            body.push(m as u8);
            for msg in messages {
                body.extend_from_slice(&msg);
            }
            let ck = jenkins_lookup3(&body);
            body.extend_from_slice(&ck.to_le_bytes());
            body
        }
        fn dataspace(dims: &[u64]) -> Vec<u8> {
            let mut d = vec![2u8, dims.len() as u8, 0, 0];
            for dim in dims {
                d.extend_from_slice(&dim.to_le_bytes());
            }
            d
        }
        fn datatype_f64() -> Vec<u8> {
            let mut d = vec![
                0x11, 0x00, 0x00, 0x00, 0x08, 0, 0, 0, 0x00, 0x00, 0x40, 0x00,
            ];
            d.extend_from_slice(&[0u8; 8]);
            d
        }
        fn layout_chunked(btree: u64, chunk: u32, elem: u32) -> Vec<u8> {
            let mut d = vec![3u8, 2, 2];
            d.extend_from_slice(&btree.to_le_bytes());
            d.extend_from_slice(&chunk.to_le_bytes());
            d.extend_from_slice(&elem.to_le_bytes());
            d
        }
        fn link(name: &str, addr: u64) -> Vec<u8> {
            let mut d = vec![0, 0, name.len() as u8];
            d.extend_from_slice(name.as_bytes());
            d.extend_from_slice(&addr.to_le_bytes());
            d
        }
        fn chunk_node(entries: &[(u64, u64)]) -> Vec<u8> {
            let mut node = vec![b'T', b'R', b'E', b'E', 1u8, 0u8, entries.len() as u8, 0u8];
            node.extend_from_slice(&[0u8; 16]);
            for (scaled, addr) in entries {
                node.extend_from_slice(&0u32.to_le_bytes());
                node.extend_from_slice(&0u32.to_le_bytes());
                node.extend_from_slice(&scaled.to_le_bytes());
                node.extend_from_slice(&scaled.to_le_bytes());
                node.extend_from_slice(&addr.to_le_bytes());
            }
            node
        }

        let root_proto = header(vec![message(MSG_LINK, link("d", 0))]);
        let d_proto = header(vec![
            message(MSG_DATASPACE, dataspace(&[4])),
            message(MSG_DATATYPE, datatype_f64()),
            message(MSG_LAYOUT, layout_chunked(0, 2, 8)),
        ]);
        let root_addr = 48u64;
        let d_addr = root_addr + root_proto.len() as u64;
        let btree_addr = d_addr + d_proto.len() as u64;
        let chunk0_addr = btree_addr + 88u64;
        let chunk1_addr = chunk0_addr + 16u64;

        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        buf.push(2);
        buf.push(8);
        buf.push(8);
        buf.push(0);
        buf.resize(36, 0);
        buf.extend_from_slice(&root_addr.to_le_bytes());
        buf.extend_from_slice(&[0, 0, 0, 0]);
        let ck = jenkins_lookup3(&buf[..44]);
        buf[44..48].copy_from_slice(&ck.to_le_bytes());
        assert_eq!(buf.len(), 48);
        buf.extend_from_slice(&header(vec![message(MSG_LINK, link("d", d_addr))]));
        buf.extend_from_slice(&header(vec![
            message(MSG_DATASPACE, dataspace(&[4])),
            message(MSG_DATATYPE, datatype_f64()),
            message(MSG_LAYOUT, layout_chunked(btree_addr, 2, 8)),
        ]));
        buf.extend_from_slice(&chunk_node(&[(0, chunk0_addr), (2, chunk1_addr)]));
        buf.extend_from_slice(&1.0f64.to_le_bytes());
        buf.extend_from_slice(&2.0f64.to_le_bytes());
        buf.extend_from_slice(&3.0f64.to_le_bytes());
        buf.extend_from_slice(&4.0f64.to_le_bytes());

        (buf, btree_addr)
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
        let filters = vec![Hdf5Filter {
            id: FILTER_FLETCHER32,
            flags: 0,
            cd_values: Vec::new(),
        }];
        apply_filters(&mut chunk, &filters, 8, 0).unwrap();
        assert_eq!(chunk, body);
    }

    #[test]
    fn scaled_offsets_to_chunk_coordinates() {
        let dims = [2u32, 3];
        assert_eq!(scaled_to_coords(&[4, 9], &dims, true).unwrap(), vec![2, 3]);
        assert_eq!(scaled_to_coords(&[4, 9], &dims, false).unwrap(), vec![4, 9]);
        assert_eq!(scaled_to_coords(&[5, 9], &dims, true), None);
        assert_eq!(scaled_to_coords(&[4], &dims, true), None);
        assert_eq!(scaled_to_coords(&[0, 0], &[0, 3], true), None);
    }

    #[test]
    fn chunked_layout_chunk_index_and_stream() {
        let (buf, btree_addr) = synthetic_chunked_image();
        let chunk0_addr = btree_addr + 88u64;
        let chunk1_addr = chunk0_addr + 16u64;

        let file = Hdf5File::parse(&buf).unwrap();
        let (_, ds, dt) = file.dataset("d").unwrap();
        assert_eq!(ds.dims, vec![4]);
        assert_eq!(dt.class, 1);
        assert_eq!(dt.size, 8);

        let mut index = file.chunk_index("d").unwrap();
        assert_eq!(index.len(), 2);
        index.sort();
        assert_eq!(index[0], (vec![0], chunk0_addr));
        assert_eq!(index[1], (vec![1], chunk1_addr));

        let fetch = |off: u64, len: u64| {
            buf.get(off as usize..(off + len) as usize)
                .map(|s| s.to_vec())
        };
        assert_eq!(file.read_chunk("d", &[0], fetch).unwrap(), vec![1.0, 2.0]);
        assert_eq!(file.read_chunk("d", &[1], fetch).unwrap(), vec![3.0, 4.0]);
        assert!(file.read_chunk("d", &[2], fetch).is_none());
        assert_eq!(
            file.read_f64_dataset("d").unwrap(),
            vec![1.0, 2.0, 3.0, 4.0]
        );
    }

    #[test]
    fn lazy_chunk_read_reuses_the_index_window() {
        use std::cell::RefCell;

        let (image, btree_addr) = synthetic_chunked_image();
        let base = &image[..48];
        let log: RefCell<Vec<(u64, u64)>> = RefCell::new(Vec::new());

        let mut lazy = LazyHdf5::open(base, |off, len| {
            log.borrow_mut().push((off, len));
            image
                .get(off as usize..(off + len) as usize)
                .map(|s| s.to_vec())
        })
        .unwrap();

        let first = lazy
            .read_chunk_diag("d", &[0], |off, len| {
                log.borrow_mut().push((off, len));
                image
                    .get(off as usize..(off + len) as usize)
                    .map(|s| s.to_vec())
            })
            .unwrap();
        assert_eq!(first, vec![1.0, 2.0]);
        let after_first = log.borrow().len();
        assert!(
            log.borrow()[..after_first]
                .iter()
                .any(|(off, _)| *off == btree_addr),
            "the first chunk read traverses the B-tree index"
        );

        let second = lazy
            .read_chunk_diag("d", &[1], |off, len| {
                log.borrow_mut().push((off, len));
                image
                    .get(off as usize..(off + len) as usize)
                    .map(|s| s.to_vec())
            })
            .unwrap();
        assert_eq!(second, vec![3.0, 4.0]);

        let btree_fetches_second = log.borrow()[after_first..]
            .iter()
            .filter(|(off, _)| *off == btree_addr)
            .count();
        assert_eq!(
            btree_fetches_second, 0,
            "the second chunk read reuses the cached index; measured {btree_fetches_second} B-tree fetches"
        );
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
    fn real_ssi_chunk_stream_matches_whole_read() {
        let Some(bytes) = read_fixture("ssi-2026", SSI_2026) else {
            return;
        };
        let file = Hdf5File::parse(&bytes).unwrap();
        let index = file.chunk_index("SSI").unwrap();
        assert!(!index.is_empty());
        for (_coords, off) in &index {
            assert!((*off as usize) < bytes.len());
        }
        let (obj, ds, _) = file.dataset("SSI").unwrap();
        let rank = ds.dims.len();
        let chunk_dims = match obj.layout.as_ref().unwrap() {
            Hdf5Layout::Chunked { chunk_dims, .. } => chunk_dims,
            _ => panic!("SSI is not chunked"),
        };
        let full = file.read_f64_dataset("SSI").unwrap();
        let mut stride = vec![1usize; rank];
        for d in (0..rank.saturating_sub(1)).rev() {
            stride[d] = stride[d + 1] * ds.dims[d + 1] as usize;
        }
        let fetch = |off: u64, len: u64| {
            bytes
                .get(off as usize..(off + len) as usize)
                .map(|s| s.to_vec())
        };
        for (coords, _off) in index.iter().take(3) {
            let chunk = file.read_chunk("SSI", coords, fetch).unwrap();
            assert!(!chunk.is_empty());
            assert!(chunk.iter().all(|v| v.is_finite()));
            let mut flat = 0usize;
            for d in 0..rank {
                flat += (coords[d] * chunk_dims[d] as u64) as usize * stride[d];
            }
            assert_eq!(chunk[0], full[flat]);
        }
    }

    #[test]
    fn parses_goes_xrs_science_file() {
        if !Path::new(GOES_XRS).exists() {
            eprintln!(
                "skipped (fixture absent): goes-xrs — fetch from ncei.noaa.gov/data/goes-space-environment-monitor/access/science/xrs/goes14/xrsf-l2-avg1m_science/2020/01"
            );
            return;
        }
        let bytes = std::fs::read(GOES_XRS).expect("fixture read");
        let file = Hdf5File::parse(&bytes).unwrap();
        let root = file.root().unwrap();
        assert_eq!(root.links.len(), 9);
        let (_, ds, dt) = file.dataset("xrsa_flux").unwrap();
        assert_eq!(ds.dims, vec![1440]);
        assert_eq!(dt.class, 1);
        assert_eq!(dt.size, 4);
        let data = file.read_dataset("xrsa_flux").unwrap();
        assert_eq!(data.len(), 1440 * 4);
        let time = file.read_dataset("time").unwrap();
        assert_eq!(time.len(), 1440 * 8);
        let t0 = decode_f64(&time, 0, Endian::Le).unwrap();
        assert_eq!(t0, 631108800.0);
        let units = file.attribute("time", "units").unwrap();
        assert_eq!(units.datatype.class, 3);
        assert_eq!(
            byte_str(&units.data),
            "seconds since 2000-01-01 12:00:00 UTC"
        );
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
        assert!((v - 8.824_365e-6).abs() < 1e-12);
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

    #[test]
    fn contiguous_layout_reads_declared_extent() {
        fn message(typ: u8, data: Vec<u8>) -> Vec<u8> {
            let mut m = vec![typ, data.len() as u8, (data.len() >> 8) as u8, 0];
            m.extend_from_slice(&data);
            m
        }
        fn header(messages: Vec<Vec<u8>>) -> Vec<u8> {
            let mut body = vec![b'O', b'H', b'D', b'R', 2, 0];
            let m: usize = messages.iter().map(|x| x.len()).sum();
            body.push(m as u8);
            for msg in messages {
                body.extend_from_slice(&msg);
            }
            let ck = jenkins_lookup3(&body);
            body.extend_from_slice(&ck.to_le_bytes());
            body
        }
        fn dataspace(dims: &[u64]) -> Vec<u8> {
            let mut d = vec![2u8, dims.len() as u8, 0, 0];
            for dim in dims {
                d.extend_from_slice(&dim.to_le_bytes());
            }
            d
        }
        fn datatype_i32() -> Vec<u8> {
            vec![
                0x10, 0x08, 0x00, 0x00, 0x04, 0, 0, 0, 0x00, 0x00, 0x20, 0x00,
            ]
        }
        fn layout(addr: u64, size: u64) -> Vec<u8> {
            let mut d = vec![3u8, 1];
            d.extend_from_slice(&addr.to_le_bytes());
            d.extend_from_slice(&size.to_le_bytes());
            d
        }
        fn link(name: &str, addr: u64) -> Vec<u8> {
            let mut d = vec![0, 0, name.len() as u8];
            d.extend_from_slice(name.as_bytes());
            d.extend_from_slice(&addr.to_le_bytes());
            d
        }

        let root_len = 43usize;
        let d_addr = 48u64 + root_len as u64;
        let d_len = 65usize;
        let payload_addr = d_addr as usize + d_len;
        let u_addr = (payload_addr + 16) as u64;

        let root_header = header(vec![
            message(MSG_LINK, link("d", d_addr)),
            message(MSG_LINK, link("u", u_addr)),
        ]);
        assert_eq!(root_header.len(), root_len);
        let ds_msg = message(MSG_DATASPACE, dataspace(&[4]));
        let dt_msg = message(MSG_DATATYPE, datatype_i32());
        let d_header = header(vec![
            ds_msg.clone(),
            dt_msg.clone(),
            message(MSG_LAYOUT, layout(payload_addr as u64, 16)),
        ]);
        assert_eq!(d_header.len(), d_len);
        let u_header = header(vec![
            ds_msg,
            dt_msg,
            message(MSG_LAYOUT, layout(u64::MAX, 16)),
        ]);
        assert_eq!(u_header.len(), d_len);

        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        buf.push(2);
        buf.push(8);
        buf.push(8);
        buf.push(0);
        buf.resize(36, 0);
        buf.extend_from_slice(&48u64.to_le_bytes());
        buf.extend_from_slice(&[0, 0, 0, 0]);
        let ck = jenkins_lookup3(&buf[..44]);
        buf[44..48].copy_from_slice(&ck.to_le_bytes());
        assert_eq!(buf.len(), 48);
        buf.extend_from_slice(&root_header);
        buf.extend_from_slice(&d_header);
        let payload: Vec<u8> = [10i32, 20, 30, 40]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        buf.extend_from_slice(&payload);
        buf.extend_from_slice(&u_header);

        let file = Hdf5File::parse(&buf).unwrap();
        let (_, ds, dt) = file.dataset("d").unwrap();
        assert_eq!(ds.dims, vec![4]);
        let data = file.read_dataset("d").unwrap();
        assert_eq!(data, payload);
        assert_eq!(data.len() / dt.size, 4);
        let (_, ds_u, _) = file.dataset("u").unwrap();
        assert_eq!(ds_u.dims, vec![4]);
        let unalloc = file.read_dataset("u").unwrap();
        assert_eq!(unalloc.len(), 16);
        assert_eq!(unalloc, vec![0u8; 16]);
    }

    #[test]
    fn geostationary_sub_satellite_maps_to_origin() {
        let sub_lon = -75.0_f64;
        let height = 35_786_023.0_f64;
        let (lat, lon) = geostationary_lat_lon(0.0, 0.0, sub_lon, height);
        assert!(lat.abs() < 1e-9, "lat {}", lat);
        assert!((lon - sub_lon).abs() < 1e-9, "lon {}", lon);
        let (lat_e, lon_e) = geostationary_lat_lon(0.1, 0.0, sub_lon, height);
        assert!(lon_e > sub_lon, "eastward scan must increase longitude");
        assert!(lat_e.abs() < 1e-6, "y=0 stays on the equator");
        let (lat_n, lon_n) = geostationary_lat_lon(0.0, 0.1, sub_lon, height);
        assert!(lat_n > 0.0, "northward scan must increase latitude");
        assert!(
            (lon_n - sub_lon).abs() < 1e-9,
            "x=0 stays on the sub-satellite meridian"
        );
    }

    #[test]
    fn geostationary_grid_matches_pointwise() {
        let xs = vec![-0.1_f64, 0.0, 0.1];
        let ys = vec![-0.05_f64, 0.05];
        let sub_lon = 128.2_f64;
        let height = 35_785_864.0_f64;
        let (lats, lons) = geostationary_lat_lon_grid(&xs, &ys, sub_lon, height);
        assert_eq!(lats.len(), 6);
        assert_eq!(lons.len(), 6);
        let mut idx = 0;
        for &y in &ys {
            for &x in &xs {
                let (lat, lon) = geostationary_lat_lon(x, y, sub_lon, height);
                assert_eq!(lats[idx], lat);
                assert_eq!(lons[idx], lon);
                idx += 1;
            }
        }
    }

    #[test]
    fn committed_datatype_shared_message_resolves() {
        fn message(typ: u8, data: Vec<u8>) -> Vec<u8> {
            let mut m = vec![typ, data.len() as u8, (data.len() >> 8) as u8, 0];
            m.extend_from_slice(&data);
            m
        }
        fn message_flags(typ: u8, flags: u8, data: Vec<u8>) -> Vec<u8> {
            let mut m = vec![typ, data.len() as u8, (data.len() >> 8) as u8, flags];
            m.extend_from_slice(&data);
            m
        }
        fn header(messages: Vec<Vec<u8>>) -> Vec<u8> {
            let mut body = vec![b'O', b'H', b'D', b'R', 2, 0];
            let m: usize = messages.iter().map(|x| x.len()).sum();
            body.push(m as u8);
            for msg in messages {
                body.extend_from_slice(&msg);
            }
            let ck = jenkins_lookup3(&body);
            body.extend_from_slice(&ck.to_le_bytes());
            body
        }
        fn dataspace(dims: &[u64]) -> Vec<u8> {
            let mut d = vec![2u8, dims.len() as u8, 0, 0];
            for dim in dims {
                d.extend_from_slice(&dim.to_le_bytes());
            }
            d
        }
        fn datatype_i32() -> Vec<u8> {
            vec![
                0x10, 0x08, 0x00, 0x00, 0x04, 0, 0, 0, 0x00, 0x00, 0x20, 0x00,
            ]
        }
        fn layout(addr: u64, size: u64) -> Vec<u8> {
            let mut d = vec![3u8, 1];
            d.extend_from_slice(&addr.to_le_bytes());
            d.extend_from_slice(&size.to_le_bytes());
            d
        }
        fn link(name: &str, addr: u64) -> Vec<u8> {
            let mut d = vec![0, 0, name.len() as u8];
            d.extend_from_slice(name.as_bytes());
            d.extend_from_slice(&addr.to_le_bytes());
            d
        }
        fn shared_datatype(addr: u64) -> Vec<u8> {
            let mut data = vec![2u8, SHARE_TYPE_COMMITTED];
            data.extend_from_slice(&addr.to_le_bytes());
            data
        }

        let root_len = 27usize;
        let var_len = 63usize;
        let dt_len = 27usize;
        let var_addr = (48 + root_len) as u64;
        let dt_addr = (48 + root_len + var_len) as u64;
        let payload_addr = (48 + root_len + var_len + dt_len) as u64;

        let root_header = header(vec![message(MSG_LINK, link("v", var_addr))]);
        assert_eq!(root_header.len(), root_len);
        let var_header = header(vec![
            message(MSG_DATASPACE, dataspace(&[4])),
            message_flags(MSG_DATATYPE, MSG_FLAG_SHARED, shared_datatype(dt_addr)),
            message(MSG_LAYOUT, layout(payload_addr, 16)),
        ]);
        assert_eq!(var_header.len(), var_len);
        let dt_header = header(vec![message(MSG_DATATYPE, datatype_i32())]);
        assert_eq!(dt_header.len(), dt_len);

        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        buf.push(2);
        buf.push(8);
        buf.push(8);
        buf.push(0);
        buf.resize(36, 0);
        buf.extend_from_slice(&48u64.to_le_bytes());
        buf.extend_from_slice(&[0, 0, 0, 0]);
        let ck = jenkins_lookup3(&buf[..44]);
        buf[44..48].copy_from_slice(&ck.to_le_bytes());
        assert_eq!(buf.len(), 48);
        buf.extend_from_slice(&root_header);
        buf.extend_from_slice(&var_header);
        buf.extend_from_slice(&dt_header);
        let payload: Vec<u8> = [10i32, 20, 30, 40]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        buf.extend_from_slice(&payload);

        let file = Hdf5File::parse(&buf).unwrap();
        let (_, ds, dt) = file.dataset("v").unwrap();
        assert_eq!(ds.dims, vec![4]);
        assert_eq!(dt.class, 0);
        assert_eq!(dt.size, 4);
        let data = file.read_dataset("v").unwrap();
        assert_eq!(data, payload);
        let vals = file.read_f64_dataset("v").unwrap();
        assert_eq!(vals, vec![10.0, 20.0, 30.0, 40.0]);
    }

    #[test]
    fn real_wod_ragged_temperature_profiles() {
        const WOD_MBT: &str = "phi/pipeline/catalog/noaa_wod/wod_mbt_1903.nc";
        if !Path::new(WOD_MBT).exists() {
            eprintln!(
                "skipped (fixture absent): wod — fetch from noaa-wod-pds.s3.amazonaws.com/1903/wod_mbt_1903.nc"
            );
            return;
        }
        let bytes = std::fs::read(WOD_MBT).expect("fixture read");
        let mut file = Hdf5File::parse(&bytes).unwrap();
        let root = file.root().unwrap();
        assert!(root.links.iter().any(|l| l.name == "Temperature"));
        assert!(root.links.iter().any(|l| l.name == "Temperature_row_size"));
        let group = crate::netcdf::nc4_group(&mut file, "").unwrap();
        assert!(group.variables.iter().any(|v| v.name == "Temperature"));
        let flat = file.read_f64_dataset("Temperature").unwrap();
        let profiles =
            crate::netcdf::nc4_ragged_f64(&file, "Temperature", "Temperature_row_size").unwrap();
        assert_eq!(profiles.len(), 1);
        let total: usize = profiles.iter().map(|p| p.len()).sum();
        assert_eq!(total, flat.len());
    }

    #[test]
    fn real_goes16_abi_and_gk2a_navigation() {
        let goes = Path::new(GOES16_ABI);
        let gk2a = Path::new(GK2A_AMI);
        if !goes.exists() && !gk2a.exists() {
            eprintln!(
                "skipped (fixtures absent): goes16/gk2a — fetch from noaa-goes16 / noaa-gk2a-pds S3 buckets"
            );
            return;
        }
        if goes.exists() {
            let bytes = std::fs::read(GOES16_ABI).expect("fixture read");
            let file = Hdf5File::parse(&bytes).unwrap();
            let root = file.root().unwrap();
            assert!(root.links.len() >= 50, "root links {}", root.links.len());
            assert!(
                root.links
                    .iter()
                    .any(|l| l.name == "goes_imager_projection")
            );
            assert!(root.links.iter().any(|l| l.name == "x"));
            assert!(root.links.iter().any(|l| l.name == "y"));
            let proj = file.geostationary_projection().unwrap();
            assert!((proj.sub_longitude_deg - (-75.2)).abs() < 1.0);
            assert!(proj.perspective_height_m > 35_000_000.0);
            let x = file.read_f64_dataset("x").unwrap();
            let y = file.read_f64_dataset("y").unwrap();
            assert!(!x.is_empty() && !y.is_empty());
            let (lats, lons) = geostationary_lat_lon_grid(
                &x[..x.len().min(64)],
                &y[..y.len().min(64)],
                proj.sub_longitude_deg,
                proj.perspective_height_m,
            );
            assert_eq!(lats.len(), 64 * 64);
            assert_eq!(lons.len(), 64 * 64);
        }
        if gk2a.exists() {
            let bytes = std::fs::read(GK2A_AMI).expect("fixture read");
            let file = Hdf5File::parse(&bytes).unwrap();
            let (_, ds, dt) = file.dataset("image_pixel_values").unwrap();
            assert_eq!(ds.dims, vec![5500, 5500]);
            assert_eq!(dt.class, 0);
            assert_eq!(dt.size, 2);
            let proj = file.geostationary_projection().unwrap();
            assert!((proj.sub_longitude_deg - 128.2).abs() < 1.0);
            assert!(proj.perspective_height_m > 35_000_000.0);
        }
    }

    #[test]
    fn parse_fetch_resolves_object_header_beyond_base() {
        let root_addr = 1u64 << 20;
        let mut full = vec![0u8; (root_addr + 128) as usize];
        full[..8].copy_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        full[8] = 2;
        full[9] = 8;
        full[10] = 8;
        full[36..44].copy_from_slice(&root_addr.to_le_bytes());
        let sb_ck = jenkins_lookup3(&full[..44]);
        full[44..48].copy_from_slice(&sb_ck.to_le_bytes());

        let o = root_addr as usize;
        full[o..o + 4].copy_from_slice(b"OHDR");
        full[o + 4] = 2;
        full[o + 5] = 0;
        full[o + 6] = 4;
        full[o + 7] = MSG_GROUP_INFO;
        full[o + 8..o + 11].copy_from_slice(&[0, 0, 0]);
        let ohdr_ck = jenkins_lookup3(&full[o..o + 11]);
        full[o + 11..o + 15].copy_from_slice(&ohdr_ck.to_le_bytes());

        let base = &full[..512];
        let mut fetched: Vec<(u64, u64)> = Vec::new();
        let file = Hdf5File::parse_fetch(base, |off, len| {
            fetched.push((off, len));
            full.get(off as usize..(off + len) as usize)
                .map(|s| s.to_vec())
        })
        .expect("parse_fetch resolves the beyond-base object header via the fetch closure");
        let root = file.root().expect("root object gathered");
        assert!(
            root.is_group,
            "the fetched object header carries its group message"
        );
        assert!(
            fetched.iter().any(|&(off, _)| off == root_addr),
            "the fetch closure served the object header address"
        );
    }

    #[test]
    fn parse_fetch_reads_a_v1_object_header_beyond_base() {
        let root_addr = 1u64 << 20;
        let mut full = vec![0u8; (root_addr + 64) as usize];
        full[..8].copy_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        full[8] = 0;
        full[13] = 8;
        full[14] = 8;
        full[64..72].copy_from_slice(&root_addr.to_le_bytes());

        let o = root_addr as usize;
        full[o] = 1;
        full[o + 2..o + 4].copy_from_slice(&1u16.to_le_bytes());
        full[o + 8..o + 12].copy_from_slice(&8u32.to_le_bytes());
        full[o + 16] = MSG_GROUP_INFO;

        let base = &full[..512];
        let file = Hdf5File::parse_fetch(base, |off, len| {
            full.get(off as usize..(off + len) as usize)
                .map(|s| s.to_vec())
        })
        .expect("the v1 object header is 16 bytes of prefix plus header_size");
        let root = file.root().expect("root object gathered");
        assert!(
            root.is_group,
            "the v1 header message at offset 16 lies inside 16 + header_size"
        );
    }

    #[test]
    fn lazy_resolution_fetches_only_the_requested_path() {
        fn message(typ: u8, data: Vec<u8>) -> Vec<u8> {
            let mut m = vec![typ, data.len() as u8, (data.len() >> 8) as u8, 0];
            m.extend_from_slice(&data);
            m
        }
        fn header(messages: Vec<Vec<u8>>) -> Vec<u8> {
            let mut body = vec![b'O', b'H', b'D', b'R', 2, 0];
            let m: usize = messages.iter().map(|x| x.len()).sum();
            body.push(m as u8);
            for msg in messages {
                body.extend_from_slice(&msg);
            }
            let ck = jenkins_lookup3(&body);
            body.extend_from_slice(&ck.to_le_bytes());
            body
        }
        fn dataspace(dims: &[u64]) -> Vec<u8> {
            let mut d = vec![2u8, dims.len() as u8, 0, 0];
            for dim in dims {
                d.extend_from_slice(&dim.to_le_bytes());
            }
            d
        }
        fn datatype_i32() -> Vec<u8> {
            vec![
                0x10, 0x08, 0x00, 0x00, 0x04, 0, 0, 0, 0x00, 0x00, 0x20, 0x00,
            ]
        }
        fn layout(addr: u64, size: u64) -> Vec<u8> {
            let mut d = vec![3u8, 1];
            d.extend_from_slice(&addr.to_le_bytes());
            d.extend_from_slice(&size.to_le_bytes());
            d
        }
        fn link(name: &str, addr: u64) -> Vec<u8> {
            let mut d = vec![0, 0, name.len() as u8];
            d.extend_from_slice(name.as_bytes());
            d.extend_from_slice(&addr.to_le_bytes());
            d
        }

        const ROOT: usize = 128;
        const A_ADDR: u64 = 1 << 20;
        const B_ADDR: u64 = (1 << 20) + 256;
        const A_PAYLOAD: u64 = (1 << 20) + 512;
        const B_PAYLOAD: u64 = (1 << 20) + 768;

        let a_header = header(vec![
            message(MSG_DATASPACE, dataspace(&[4])),
            message(MSG_DATATYPE, datatype_i32()),
            message(MSG_LAYOUT, layout(A_PAYLOAD, 16)),
        ]);
        let b_header = header(vec![
            message(MSG_DATASPACE, dataspace(&[2])),
            message(MSG_DATATYPE, datatype_i32()),
            message(MSG_LAYOUT, layout(B_PAYLOAD, 8)),
        ]);
        let root_header = header(vec![
            message(MSG_GROUP_INFO, vec![0, 0]),
            message(MSG_LINK, link("A", A_ADDR)),
            message(MSG_LINK, link("B", B_ADDR)),
        ]);

        let mut full = vec![0u8; (B_PAYLOAD + 32) as usize];
        full[..8].copy_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        full[8] = 2;
        full[9] = 8;
        full[10] = 8;
        full[36..44].copy_from_slice(&(ROOT as u64).to_le_bytes());
        let sb_ck = jenkins_lookup3(&full[..44]);
        full[44..48].copy_from_slice(&sb_ck.to_le_bytes());
        full[ROOT..ROOT + root_header.len()].copy_from_slice(&root_header);
        full[A_ADDR as usize..A_ADDR as usize + a_header.len()].copy_from_slice(&a_header);
        full[B_ADDR as usize..B_ADDR as usize + b_header.len()].copy_from_slice(&b_header);

        let base = &full[..512];
        let served = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let full_ref: &[u8] = &full;
        let mut file = LazyHdf5::open(base, {
            let served = std::rc::Rc::clone(&served);
            move |off, len| {
                served.borrow_mut().push((off, len));
                full_ref
                    .get(off as usize..(off + len) as usize)
                    .map(|s| s.to_vec())
            }
        })
        .expect("open reads the superblock only");
        let (_, ds, dt) = file.dataset("A").expect("dataset A resolves on demand");
        assert_eq!(ds.dims, vec![4]);
        assert_eq!(dt.class, 0);
        assert_eq!(dt.size, 4);
        let served_after_a: Vec<(u64, u64)> = served.borrow().clone();
        assert_eq!(
            served_after_a
                .iter()
                .filter(|&&(off, _)| off == A_ADDR)
                .count(),
            1,
            "the requested dataset header is fetched exactly once"
        );
        assert!(
            !served_after_a.iter().any(|&(off, _)| off == B_ADDR),
            "the sibling dataset B header is never fetched"
        );
        let (_again, _again_ds, _again_dt) = file.dataset("A").expect("dataset A resolves again");
        assert_eq!(
            served.borrow().len(),
            served_after_a.len(),
            "the second resolution of A adds zero fetches"
        );
        drop(file);

        let mut eager_served: Vec<(u64, u64)> = Vec::new();
        let eager = Hdf5File::parse_fetch(base, |off, len| {
            eager_served.push((off, len));
            full.get(off as usize..(off + len) as usize)
                .map(|s| s.to_vec())
        })
        .expect("the eager parse resolves every reachable object header");
        drop(eager);
        assert!(
            eager_served.iter().any(|&(off, _)| off == B_ADDR),
            "the eager face serves B — the lazy omission is design, not accident"
        );
    }

    #[test]
    fn lazy_rollback_retries_a_failed_datatype_chase() {
        fn message(typ: u8, data: Vec<u8>) -> Vec<u8> {
            let mut m = vec![typ, data.len() as u8, (data.len() >> 8) as u8, 0];
            m.extend_from_slice(&data);
            m
        }
        fn message_flags(typ: u8, flags: u8, data: Vec<u8>) -> Vec<u8> {
            let mut m = vec![typ, data.len() as u8, (data.len() >> 8) as u8, flags];
            m.extend_from_slice(&data);
            m
        }
        fn header(messages: Vec<Vec<u8>>) -> Vec<u8> {
            let mut body = vec![b'O', b'H', b'D', b'R', 2, 0];
            let m: usize = messages.iter().map(|x| x.len()).sum();
            body.push(m as u8);
            for msg in messages {
                body.extend_from_slice(&msg);
            }
            let ck = jenkins_lookup3(&body);
            body.extend_from_slice(&ck.to_le_bytes());
            body
        }
        fn dataspace(dims: &[u64]) -> Vec<u8> {
            let mut d = vec![2u8, dims.len() as u8, 0, 0];
            for dim in dims {
                d.extend_from_slice(&dim.to_le_bytes());
            }
            d
        }
        fn layout(addr: u64, size: u64) -> Vec<u8> {
            let mut d = vec![3u8, 1];
            d.extend_from_slice(&addr.to_le_bytes());
            d.extend_from_slice(&size.to_le_bytes());
            d
        }
        fn link(name: &str, addr: u64) -> Vec<u8> {
            let mut d = vec![0, 0, name.len() as u8];
            d.extend_from_slice(name.as_bytes());
            d.extend_from_slice(&addr.to_le_bytes());
            d
        }
        fn shared_datatype(addr: u64) -> Vec<u8> {
            let mut data = vec![2u8, SHARE_TYPE_COMMITTED];
            data.extend_from_slice(&addr.to_le_bytes());
            data
        }

        const ROOT: usize = 128;
        const VAR_ADDR: u64 = 1 << 20;
        const DT_ADDR: u64 = (1 << 20) + 512;
        const PAYLOAD_ADDR: u64 = (1 << 20) + 1024;

        let var_header = header(vec![
            message(MSG_DATASPACE, dataspace(&[4])),
            message_flags(MSG_DATATYPE, MSG_FLAG_SHARED, shared_datatype(DT_ADDR)),
            message(MSG_LAYOUT, layout(PAYLOAD_ADDR, 16)),
        ]);
        let root_header = header(vec![
            message(MSG_GROUP_INFO, vec![0, 0]),
            message(MSG_LINK, link("v", VAR_ADDR)),
        ]);

        let mut full = vec![0u8; (PAYLOAD_ADDR + 64) as usize];
        full[..8].copy_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        full[8] = 2;
        full[9] = 8;
        full[10] = 8;
        full[36..44].copy_from_slice(&(ROOT as u64).to_le_bytes());
        let sb_ck = jenkins_lookup3(&full[..44]);
        full[44..48].copy_from_slice(&sb_ck.to_le_bytes());
        full[ROOT..ROOT + root_header.len()].copy_from_slice(&root_header);
        full[VAR_ADDR as usize..VAR_ADDR as usize + var_header.len()].copy_from_slice(&var_header);

        let base = &full[..512];
        let served = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let full_ref: &[u8] = &full;
        let mut file = LazyHdf5::open(base, {
            let served = std::rc::Rc::clone(&served);
            move |off, len| {
                served.borrow_mut().push((off, len));
                if off == DT_ADDR {
                    return None;
                }
                full_ref
                    .get(off as usize..(off + len) as usize)
                    .map(|s| s.to_vec())
            }
        })
        .expect("open reads the superblock only");

        let first = file.dataset("v");
        assert!(
            matches!(first, Err(Hdf5Note::AbsentAtByte { off }) if off == DT_ADDR as usize),
            "the committed-datatype chase reports the true note, found {first:?}"
        );
        let served_first: Vec<(u64, u64)> = served.borrow().clone();
        assert_eq!(
            served_first
                .iter()
                .filter(|&&(off, _)| off == VAR_ADDR)
                .count(),
            1
        );
        let second = file.dataset("v");
        assert!(
            matches!(second, Err(Hdf5Note::AbsentAtByte { .. })),
            "the second attempt still fails with the true note, found {second:?}"
        );
        assert_eq!(
            served
                .borrow()
                .iter()
                .filter(|&&(off, _)| off == VAR_ADDR)
                .count(),
            2,
            "the second attempt refetches the rolled-back object header"
        );
    }

    #[test]
    fn v1_object_header_reads_symbol_table_from_continuation_block() {
        fn put(buf: &mut Vec<u8>, at: usize, bytes: &[u8]) {
            if buf.len() < at + bytes.len() {
                buf.resize(at + bytes.len(), 0);
            }
            buf[at..at + bytes.len()].copy_from_slice(bytes);
        }
        fn v1_msg(typ: u16, data: Vec<u8>) -> Vec<u8> {
            let mut m = Vec::new();
            m.extend_from_slice(&typ.to_le_bytes());
            m.extend_from_slice(&(data.len() as u16).to_le_bytes());
            m.push(0);
            m.extend_from_slice(&[0u8; 3]);
            m.extend_from_slice(&data);
            m
        }
        fn v1_header(total: u16, messages: Vec<Vec<u8>>) -> Vec<u8> {
            let mut h = Vec::new();
            h.push(1);
            h.push(0);
            h.extend_from_slice(&total.to_le_bytes());
            h.extend_from_slice(&1u32.to_le_bytes());
            let area: usize = messages.iter().map(|m| m.len()).sum();
            h.extend_from_slice(&(area as u32).to_le_bytes());
            h.extend_from_slice(&[0u8; 4]);
            for m in messages {
                h.extend_from_slice(&m);
            }
            h
        }
        fn addr_bytes(v: u64) -> Vec<u8> {
            v.to_le_bytes().to_vec()
        }

        const ROOT: usize = 128;
        const CONT: usize = 256;
        const HEAP: usize = 320;
        const SEG: usize = 400;
        const SNOD: usize = 480;
        const DATASET: usize = 560;
        const PAYLOAD: usize = 700;

        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        buf.push(0);
        buf.resize(72, 0);
        buf[13] = 8;
        buf[14] = 8;
        buf[64..72].copy_from_slice(&(ROOT as u64).to_le_bytes());

        let cont_data = [addr_bytes(CONT as u64), addr_bytes(24)].concat();
        put(
            &mut buf,
            ROOT,
            &v1_header(2, vec![v1_msg(MSG_CONT as u16, cont_data)]),
        );

        let symtab_data = [addr_bytes(SNOD as u64), addr_bytes(HEAP as u64)].concat();
        put(
            &mut buf,
            CONT,
            &v1_msg(MSG_SYMBOL_TABLE as u16, symtab_data),
        );

        let mut heap = vec![b'H', b'E', b'A', b'P', 0, 0, 0, 0];
        heap.extend_from_slice(&8u64.to_le_bytes());
        heap.extend_from_slice(&u64::MAX.to_le_bytes());
        heap.extend_from_slice(&(SEG as u64).to_le_bytes());
        put(&mut buf, HEAP, &heap);

        put(&mut buf, SEG, b"d\0");

        let mut snod = vec![b'S', b'N', b'O', b'D', 0, 0];
        snod.extend_from_slice(&1u16.to_le_bytes());
        snod.extend_from_slice(&0u64.to_le_bytes());
        snod.extend_from_slice(&(DATASET as u64).to_le_bytes());
        snod.extend_from_slice(&0u32.to_le_bytes());
        snod.extend_from_slice(&0u32.to_le_bytes());
        snod.extend_from_slice(&[0u8; 16]);
        put(&mut buf, SNOD, &snod);

        let ds_data = [vec![2u8, 1, 0, 0], addr_bytes(4)].concat();
        let dt_data = [
            vec![0x11u8, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0x40, 0],
            vec![0u8; 8],
        ]
        .concat();
        let layout_data = [vec![3u8, 1], addr_bytes(PAYLOAD as u64), addr_bytes(32)].concat();
        put(
            &mut buf,
            DATASET,
            &v1_header(
                3,
                vec![
                    v1_msg(MSG_DATASPACE as u16, ds_data),
                    v1_msg(MSG_DATATYPE as u16, dt_data),
                    v1_msg(MSG_LAYOUT as u16, layout_data),
                ],
            ),
        );

        let payload: Vec<u8> = [1.0f64, 2.0, 3.0, 4.0]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        put(&mut buf, PAYLOAD, &payload);
        buf.resize(1024, 0);

        let file = Hdf5File::parse(&buf).expect("the v0 superblock and v1 headers parse");
        let (_, ds, dt) = file
            .dataset("d")
            .expect("the symbol table message in the v1 continuation block resolves");
        assert_eq!(ds.dims, vec![4]);
        assert_eq!(dt.class, 1);
        assert_eq!(dt.size, 8);
        assert_eq!(
            file.read_f64_dataset("d")
                .expect("the contiguous payload reads"),
            vec![1.0, 2.0, 3.0, 4.0]
        );
        let d = file
            .root_header_diag()
            .expect("the root header diag stands");
        assert_eq!(d.version, 1);
        assert_eq!(d.msgs_initial, 1);
        assert_eq!(d.cont_blocks, 1);
        assert_eq!(d.msgs_cont, 1);
        assert_eq!(d.declared, Some(2));
        assert!(d.symtab_found);
    }

    #[test]
    fn v2_continuation_self_cycle_terminates() {
        fn put(buf: &mut Vec<u8>, at: usize, bytes: &[u8]) {
            if buf.len() < at + bytes.len() {
                buf.resize(at + bytes.len(), 0);
            }
            buf[at..at + bytes.len()].copy_from_slice(bytes);
        }
        fn v2_cont(addr: u64, len: u64) -> Vec<u8> {
            let mut m = vec![MSG_CONT, 16, 0, 0];
            m.extend_from_slice(&addr.to_le_bytes());
            m.extend_from_slice(&len.to_le_bytes());
            m
        }

        const ROOT: usize = 128;
        const CONT: usize = 256;
        const CONT_LEN: u64 = 28;

        let mut buf: Vec<u8> = vec![0u8; 512];
        buf[..8].copy_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        buf[8] = 2;
        buf[9] = 8;
        buf[10] = 8;
        buf[36..44].copy_from_slice(&(ROOT as u64).to_le_bytes());
        let sb_ck = jenkins_lookup3(&buf[..44]);
        buf[44..48].copy_from_slice(&sb_ck.to_le_bytes());

        put(&mut buf, ROOT, b"OHDR");
        buf[ROOT + 4] = 2;
        buf[ROOT + 5] = 0;
        buf[ROOT + 6] = 20;
        put(&mut buf, ROOT + 7, &v2_cont(CONT as u64, CONT_LEN));
        let root_end = ROOT + 7 + 20;
        let root_ck = jenkins_lookup3(&buf[ROOT..root_end]);
        put(&mut buf, root_end, &root_ck.to_le_bytes());

        put(&mut buf, CONT, b"OCHK");
        put(&mut buf, CONT + 4, &v2_cont(CONT as u64, CONT_LEN));
        let cont_ck = jenkins_lookup3(&buf[CONT..CONT + 24]);
        put(&mut buf, CONT + 24, &cont_ck.to_le_bytes());

        let file = Hdf5File::parse(&buf)
            .expect("the v2 object header resolves through its self-cyclic continuation block");
        let d = file
            .root_header_diag()
            .expect("the root header diag stands");
        assert_eq!(d.version, 2);
        assert_eq!(d.cont_blocks, 1);
        assert!(d.cont_blocks <= MAX_CONT_BLOCKS);
    }

    #[test]
    fn continuation_length_beyond_read_cap_terminates() {
        fn put(buf: &mut Vec<u8>, at: usize, bytes: &[u8]) {
            if buf.len() < at + bytes.len() {
                buf.resize(at + bytes.len(), 0);
            }
            buf[at..at + bytes.len()].copy_from_slice(bytes);
        }
        fn v2_cont(addr: u64, len: u64) -> Vec<u8> {
            let mut m = vec![MSG_CONT, 16, 0, 0];
            m.extend_from_slice(&addr.to_le_bytes());
            m.extend_from_slice(&len.to_le_bytes());
            m
        }

        const ROOT: usize = 128;
        const CONT: u64 = 1 << 30;
        const CONT_LEN: u64 = 1 << 40;

        let mut buf: Vec<u8> = vec![0u8; 512];
        buf[..8].copy_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        buf[8] = 2;
        buf[9] = 8;
        buf[10] = 8;
        buf[36..44].copy_from_slice(&(ROOT as u64).to_le_bytes());
        let sb_ck = jenkins_lookup3(&buf[..44]);
        buf[44..48].copy_from_slice(&sb_ck.to_le_bytes());

        put(&mut buf, ROOT, b"OHDR");
        buf[ROOT + 4] = 2;
        buf[ROOT + 5] = 0;
        buf[ROOT + 6] = 20;
        put(&mut buf, ROOT + 7, &v2_cont(CONT, CONT_LEN));
        let root_end = ROOT + 7 + 20;
        let root_ck = jenkins_lookup3(&buf[ROOT..root_end]);
        put(&mut buf, root_end, &root_ck.to_le_bytes());

        let note = Hdf5File::parse(&buf)
            .err()
            .expect("a continuation length beyond the read cap ends the parse");
        match note {
            Hdf5Note::ReadLength { len, .. } => assert_eq!(len, CONT_LEN),
            other => panic!("the parse reports the read length, found {other:?}"),
        }
    }

    #[test]
    fn window_reader_stops_at_the_fetch_budget() {
        let base: Vec<u8> = Vec::new();
        let mut served = 0usize;
        let mut reader = Hdf5WindowReader::new(&base, |_off: u64, len: u64| {
            served += 1;
            Some(vec![0u8; len as usize])
        });
        let mut last: Result<Vec<u8>, Hdf5Note> = Ok(Vec::new());
        for i in 0..=MAX_FETCHES {
            last = reader.read((1 << 30) | i as u64, 16);
            if last.is_err() {
                break;
            }
        }
        drop(reader);
        assert_eq!(served, MAX_FETCHES);
        match last {
            Err(Hdf5Note::FetchBudget { reads, .. }) => assert_eq!(reads, MAX_FETCHES),
            other => panic!("the reader reports its exhausted fetch budget, found {other:?}"),
        }
    }

    #[test]
    fn object_header_traversal_is_range_bounded() {
        const BLOCK: u64 = 1 << 15;
        const ROOT: usize = 128;
        const CONT_START: u64 = 1 << 20;

        let mut base: Vec<u8> = vec![0u8; 512];
        base[..8].copy_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        base[8] = 0;
        base[13] = 8;
        base[14] = 8;
        base[64..72].copy_from_slice(&(ROOT as u64).to_le_bytes());

        base[ROOT] = 1;
        base[ROOT + 2..ROOT + 4].copy_from_slice(&0xFFFFu16.to_le_bytes());
        base[ROOT + 8..ROOT + 12].copy_from_slice(&24u32.to_le_bytes());
        let m = ROOT + 16;
        base[m..m + 2].copy_from_slice(&(MSG_CONT as u16).to_le_bytes());
        base[m + 2..m + 4].copy_from_slice(&16u16.to_le_bytes());
        base[m + 8..m + 16].copy_from_slice(&CONT_START.to_le_bytes());
        base[m + 16..m + 24].copy_from_slice(&BLOCK.to_le_bytes());

        let fetch = |off: u64, len: u64| -> Option<Vec<u8>> {
            let mut block = vec![0u8; len as usize];
            block[..2].copy_from_slice(&(MSG_CONT as u16).to_le_bytes());
            block[2..4].copy_from_slice(&(len.saturating_sub(8) as u16).to_le_bytes());
            block[8..16].copy_from_slice(&(off + len).to_le_bytes());
            block[16..24].copy_from_slice(&len.to_le_bytes());
            Some(block)
        };

        let note = Hdf5File::parse_fetch(&base, fetch)
            .err()
            .expect("the over-long continuation chain is bounded by the traversal budget");
        match note {
            Hdf5Note::TraversalBudget { bytes, .. } => {
                assert!(bytes <= MAX_TRAVERSAL_BYTES);
                assert!(bytes + BLOCK > MAX_TRAVERSAL_BYTES);
                assert!(
                    bytes / BLOCK < MAX_CONT_BLOCKS as u64,
                    "the global traversal budget fires before the per-block guard"
                );
            }
            other => panic!("the traversal budget note is expected, found {other:?}"),
        }
    }
}
