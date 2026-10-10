pub const MAGIC: [u8; 4] = *b"root";

const K_BYTE_COUNT_MASK: u32 = 0x4000_0000; // TBuffer::kByteCountMask
const K_BYTE_COUNT_V_MASK: i16 = 0x4000; // kByteCountVMask: high bit of the first Version_t short
const K_NEW_CLASS_TAG: u32 = 0xffff_ffff; // TBuffer::kNewClassTag

#[derive(Clone, Debug, PartialEq)]
pub struct RootHeader {
    pub version: i32,
    pub begin: i64,
    pub end: i64,
    pub seek_free: i64,
    pub nbytes_free: i32,
    pub nfree: i32,
    pub nbytes_name: i32,
    pub units: u8,
    pub compress: i32,
    pub seek_info: i64,
    pub nbytes_info: i32,
    pub uuid: [u8; 18],
    pub big: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RootKey {
    pub name: String,
    pub class: String,
    pub nbytes: i32,
    pub objlen: i32,
    pub keylen: i32,
    pub cycle: i16,
    pub seek_key: i64,
    pub seek_pdir: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RootFile {
    pub header: RootHeader,
    pub keys: Vec<RootKey>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StreamerInfoHeader {
    pub version: i16,
    pub byte_count: i32,
    pub class_name: String,
    pub checksum: u32,
    pub n_members: i32,
}

struct Cursor<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(buf: &'a [u8], pos: usize) -> Cursor<'a> {
        Cursor { buf, pos }
    }

    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.pos.checked_add(n)?;
        let slice = self.buf.get(self.pos..end)?;
        self.pos = end;
        Some(slice)
    }

    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }

    fn i16(&mut self) -> Option<i16> {
        Some(i16::from_be_bytes(self.take(2)?.try_into().ok()?))
    }

    fn u16(&mut self) -> Option<u16> {
        Some(u16::from_be_bytes(self.take(2)?.try_into().ok()?))
    }

    fn i32(&mut self) -> Option<i32> {
        Some(i32::from_be_bytes(self.take(4)?.try_into().ok()?))
    }

    fn i64(&mut self) -> Option<i64> {
        Some(i64::from_be_bytes(self.take(8)?.try_into().ok()?))
    }

    fn tstring(&mut self) -> Option<String> {
        let first = self.u8()?;
        let len = if first == 255 {
            self.i32()?
        } else {
            first as i32
        };
        if len < 0 {
            return None;
        }
        let raw = self.take(len as usize)?;
        String::from_utf8(raw.to_vec()).ok()
    }

    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_be_bytes(self.take(4)?.try_into().ok()?))
    }

    fn class_tag_name(&mut self) -> Option<&'a str> {
        let rest = self.buf.get(self.pos..)?;
        let end = rest.iter().position(|&b| b == 0)?;
        let name = std::str::from_utf8(rest.get(..end)?).ok()?;
        self.pos += end + 1;
        Some(name)
    }

    fn read_version(&mut self) -> Option<(i16, i32)> {
        let count_pos = self.pos;
        let count = self.u32()?;
        if count & K_BYTE_COUNT_MASK == 0 {
            self.pos = count_pos;
            Some((self.i16()?, 0))
        } else {
            Some((self.i16()?, (count & !K_BYTE_COUNT_MASK) as i32))
        }
    }

    fn skip_version(&mut self) -> Option<i16> {
        let first = self.i16()?;
        if first & K_BYTE_COUNT_V_MASK != 0 {
            self.i16()?;
            self.i16()
        } else {
            Some(first)
        }
    }
}

struct Huffman {
    count: [u16; 16],
    symbol: [u16; 288],
}

impl Huffman {
    fn new() -> Huffman {
        Huffman {
            count: [0; 16],
            symbol: [0; 288],
        }
    }
}

struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
    bitbuf: u32,
    bitcnt: u32,
}

impl<'a> Bits<'a> {
    fn new(data: &'a [u8]) -> Bits<'a> {
        Bits {
            data,
            pos: 0,
            bitbuf: 0,
            bitcnt: 0,
        }
    }

    fn bits(&mut self, n: u32) -> Option<u32> {
        if n == 0 {
            return Some(0);
        }
        while self.bitcnt < n {
            let byte = *self.data.get(self.pos)?;
            self.pos += 1;
            self.bitbuf |= (byte as u32) << self.bitcnt;
            self.bitcnt += 8;
        }
        let value = self.bitbuf & ((1u32 << n) - 1);
        self.bitbuf >>= n;
        self.bitcnt -= n;
        Some(value)
    }

    fn align_byte(&mut self) -> Option<()> {
        let drop = self.bitcnt % 8;
        self.bits(drop)?;
        Some(())
    }
}

fn huffman_construct(h: &mut Huffman, lengths: &[u8]) -> Option<()> {
    if lengths.is_empty() || lengths.len() > h.symbol.len() {
        return None;
    }
    for c in h.count.iter_mut() {
        *c = 0;
    }
    for &len in lengths {
        if len as usize >= h.count.len() {
            return None;
        }
        h.count[len as usize] += 1;
    }
    if h.count[0] as usize == lengths.len() {
        return Some(());
    }
    let mut left: i32 = 1;
    for len in 1..16 {
        left <<= 1;
        left -= h.count[len] as i32;
        if left < 0 {
            return None;
        }
    }
    let mut offsets = [0u16; 16];
    for len in 1..15 {
        offsets[len + 1] = offsets[len] + h.count[len];
    }
    for (symbol, &len) in lengths.iter().enumerate() {
        if len != 0 {
            let slot = offsets[len as usize] as usize;
            *h.symbol.get_mut(slot)? = symbol as u16;
            offsets[len as usize] += 1;
        }
    }
    Some(())
}

fn huffman_decode(bits: &mut Bits<'_>, h: &Huffman) -> Option<usize> {
    let mut code: i32 = 0;
    let mut first: i32 = 0;
    let mut index: i32 = 0;
    for len in 1..16 {
        code |= bits.bits(1)? as i32;
        let count = h.count[len] as i32;
        if code - count < first {
            let slot = (index + (code - first)) as usize;
            return Some(*h.symbol.get(slot)? as usize);
        }
        index += count;
        first += count;
        first <<= 1;
        code <<= 1;
    }
    None
}

const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EXTRA: [u32; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u32; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];
const CODE_LENGTH_ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

fn inflate_stored(bits: &mut Bits<'_>, out: &mut Vec<u8>) -> Option<()> {
    bits.align_byte()?;
    let len = bits.bits(16)? as usize;
    let nlen = bits.bits(16)? as usize;
    if len != (!nlen & 0xffff) {
        return None;
    }
    for _ in 0..len {
        out.push(bits.bits(8)? as u8);
    }
    Some(())
}

fn inflate_codes(
    bits: &mut Bits<'_>,
    out: &mut Vec<u8>,
    lencode: &Huffman,
    distcode: &Huffman,
) -> Option<()> {
    loop {
        let symbol = huffman_decode(bits, lencode)?;
        if symbol == 256 {
            return Some(());
        }
        if symbol < 256 {
            out.push(symbol as u8);
            continue;
        }
        let index = symbol - 257;
        let len = *LEN_BASE.get(index)? as usize + bits.bits(*LEN_EXTRA.get(index)?)? as usize;
        let dsym = huffman_decode(bits, distcode)?;
        let dist = *DIST_BASE.get(dsym)? as usize + bits.bits(*DIST_EXTRA.get(dsym)?)? as usize;
        if dist > out.len() {
            return None;
        }
        let start = out.len() - dist;
        for i in 0..len {
            let byte = *out.get(start + i)?;
            out.push(byte);
        }
    }
}

fn fixed_tables() -> Option<(Huffman, Huffman)> {
    let mut lit = [0u8; 288];
    for (i, slot) in lit.iter_mut().enumerate() {
        *slot = if i < 144 {
            8
        } else if i < 256 {
            9
        } else if i < 280 {
            7
        } else {
            8
        };
    }
    let dist = [5u8; 30];
    let mut lencode = Huffman::new();
    huffman_construct(&mut lencode, &lit)?;
    let mut distcode = Huffman::new();
    huffman_construct(&mut distcode, &dist)?;
    Some((lencode, distcode))
}

fn dynamic_tables(
    bits: &mut Bits<'_>,
    lencode: &mut Huffman,
    distcode: &mut Huffman,
) -> Option<()> {
    let hlit = bits.bits(5)? as usize + 257;
    let hdist = bits.bits(5)? as usize + 1;
    let hclen = bits.bits(4)? as usize + 4;
    let mut lengths = [0u8; 320];
    for i in 0..hclen {
        lengths[*CODE_LENGTH_ORDER.get(i)?] = bits.bits(3)? as u8;
    }
    let mut clcode = Huffman::new();
    huffman_construct(&mut clcode, &lengths[..19])?;
    let mut index = 0;
    while index < hlit + hdist {
        let symbol = huffman_decode(bits, &clcode)?;
        match symbol {
            0..=15 => {
                *lengths.get_mut(index)? = symbol as u8;
                index += 1;
            }
            16 => {
                if index == 0 {
                    return None;
                }
                let prev = lengths[index - 1];
                let repeat = bits.bits(2)? as usize + 3;
                for _ in 0..repeat {
                    *lengths.get_mut(index)? = prev;
                    index += 1;
                }
            }
            17 => {
                let repeat = bits.bits(3)? as usize + 3;
                for _ in 0..repeat {
                    *lengths.get_mut(index)? = 0;
                    index += 1;
                }
            }
            18 => {
                let repeat = bits.bits(7)? as usize + 11;
                for _ in 0..repeat {
                    *lengths.get_mut(index)? = 0;
                    index += 1;
                }
            }
            _ => return None,
        }
        if index > hlit + hdist {
            return None;
        }
    }
    if lengths[hlit - 1] == 0 {
        return None;
    }
    huffman_construct(lencode, &lengths[..hlit])?;
    huffman_construct(distcode, &lengths[hlit..hlit + hdist])?;
    Some(())
}

fn inflate_zlib(data: &[u8], out_len: usize) -> Option<Vec<u8>> {
    if data.len() < 2 {
        return None;
    }
    let cmf = data[0];
    let flg = data[1];
    if cmf & 0x0f != 8 {
        return None;
    }
    if (((cmf as u16) << 8) | flg as u16) % 31 != 0 {
        return None;
    }
    let mut bits = Bits::new(data.get(2..)?);
    let mut out = Vec::with_capacity(out_len);
    loop {
        let last = bits.bits(1)?;
        let btype = bits.bits(2)?;
        match btype {
            0 => inflate_stored(&mut bits, &mut out)?,
            1 => {
                let (lencode, distcode) = fixed_tables()?;
                inflate_codes(&mut bits, &mut out, &lencode, &distcode)?;
            }
            2 => {
                let mut lencode = Huffman::new();
                let mut distcode = Huffman::new();
                dynamic_tables(&mut bits, &mut lencode, &mut distcode)?;
                inflate_codes(&mut bits, &mut out, &lencode, &distcode)?;
            }
            _ => return None,
        }
        if last == 1 {
            return Some(out);
        }
    }
}

fn u24_le(bytes: &[u8]) -> Option<usize> {
    let b = bytes.get(..3)?;
    Some(b[0] as usize | (b[1] as usize) << 8 | (b[2] as usize) << 16)
}

pub fn is_root(bytes: &[u8]) -> bool {
    bytes.len() >= MAGIC.len() && bytes[0..MAGIC.len()] == MAGIC
}

pub fn parse_header(bytes: &[u8]) -> Option<RootHeader> {
    if !is_root(bytes) {
        return None;
    }
    let mut c = Cursor::new(bytes, MAGIC.len());
    let version = c.i32()?;
    let big = version >= 1_000_000;
    let (begin, end, seek_free) = if big {
        (c.i64()?, c.i64()?, c.i64()?)
    } else {
        (c.i32()? as i64, c.i32()? as i64, c.i32()? as i64)
    };
    let nbytes_free = c.i32()?;
    let nfree = c.i32()?;
    let nbytes_name = c.i32()?;
    let units = c.u8()?;
    let compress = c.i32()?;
    let seek_info = if big { c.i64()? } else { c.i32()? as i64 };
    let nbytes_info = c.i32()?;
    let mut uuid = [0u8; 18];
    uuid.copy_from_slice(c.take(18)?);
    Some(RootHeader {
        version,
        begin,
        end,
        seek_free,
        nbytes_free,
        nfree,
        nbytes_name,
        units,
        compress,
        seek_info,
        nbytes_info,
        uuid,
        big,
    })
}

fn parse_key(c: &mut Cursor<'_>) -> Option<RootKey> {
    let nbytes = c.i32()?;
    let version = c.i16()?;
    let objlen = c.i32()?;
    let _datime = c.i32()?;
    let keylen = c.i16()?;
    let cycle = c.i16()?;
    let (seek_key, seek_pdir) = if version > 1000 {
        (c.i64()?, c.i64()?)
    } else {
        (c.i32()? as i64, c.i32()? as i64)
    };
    let class = c.tstring()?;
    let name = c.tstring()?;
    let _title = c.tstring()?;
    Some(RootKey {
        name,
        class,
        nbytes,
        objlen,
        keylen: keylen as i32,
        cycle,
        seek_key,
        seek_pdir,
    })
}

fn parse_dir_fields(c: &mut Cursor<'_>) -> Option<(i32, i32, i64)> {
    let version = c.u16()?;
    let _datime_c = c.i32()?;
    let _datime_m = c.i32()?;
    let nbytes_keys = c.i32()?;
    let _nbytes_name = c.i32()?;
    let (_seek_dir, _seek_parent, seek_keys) = if version > 1000 {
        (c.i64()?, c.i64()?, c.i64()?)
    } else {
        (c.i32()? as i64, c.i32()? as i64, c.i32()? as i64)
    };
    Some((nbytes_keys, _nbytes_name, seek_keys))
}

pub fn read_file(bytes: &[u8]) -> Result<RootFile, &'static str> {
    let header = parse_header(bytes).ok_or("root header absent or truncated")?;
    let begin = header.begin;
    if begin < 0 {
        return Err("root fBEGIN negative");
    }
    let mut dir = Cursor::new(bytes, begin as usize);
    let dir_key = parse_key(&mut dir).ok_or("directory key absent at fBEGIN")?;
    if dir_key.keylen < 0 {
        return Err("directory key length negative");
    }
    let payload = (begin as usize)
        .checked_add(dir_key.keylen as usize)
        .ok_or("directory key offset out of range")?;
    let mut dc = Cursor::new(bytes, payload);
    let _dir_name = dc.tstring().ok_or("directory name absent")?;
    let _dir_title = dc.tstring().ok_or("directory title absent")?;
    let (nbytes_keys, _nbytes_name, seek_keys) =
        parse_dir_fields(&mut dc).ok_or("directory header absent")?;
    if seek_keys <= 0 {
        return Ok(RootFile {
            header,
            keys: Vec::new(),
        });
    }
    let list_off = seek_keys as usize;
    let mut hc = Cursor::new(bytes, list_off);
    let header_key = parse_key(&mut hc).ok_or("key-list header key absent")?;
    if header_key.nbytes != nbytes_keys {
        return Err("key-list byte count diverges from directory header");
    }
    if header_key.keylen < 0 {
        return Err("key-list header length negative");
    }
    let records = list_off
        .checked_add(header_key.keylen as usize)
        .ok_or("key-list offset out of range")?;
    let mut lc = Cursor::new(bytes, records);
    let nkeys = lc.i32().ok_or("key-list count absent")?;
    if nkeys < 0 {
        return Err("key-list count negative");
    }
    let mut keys = Vec::with_capacity(nkeys as usize);
    for _ in 0..nkeys {
        keys.push(parse_key(&mut lc).ok_or("key-list record truncated")?);
    }
    Ok(RootFile { header, keys })
}

pub fn decompress_object(bytes: &[u8], key: &RootKey) -> Option<Vec<u8>> {
    if key.seek_key < 0 || key.keylen < 0 || key.objlen < 0 || key.nbytes < key.keylen {
        return None;
    }
    let start = (key.seek_key as usize).checked_add(key.keylen as usize)?;
    let span = (key.nbytes as usize).checked_sub(key.keylen as usize)?;
    let payload = bytes.get(start..start.checked_add(span)?)?;
    if payload.len() == key.objlen as usize {
        return Some(payload.to_vec());
    }
    if payload.len() < 9 || payload[0] != 0x5a || payload[1] != 0x4c {
        return None;
    }
    let stream_len = u24_le(payload.get(3..6)?)?;
    let raw_len = u24_le(payload.get(6..9)?)?;
    if raw_len != key.objlen as usize || 9usize.checked_add(stream_len)? != payload.len() {
        return None;
    }
    let stream = payload.get(9..)?;
    let out = inflate_zlib(stream, raw_len)?;
    if out.len() != raw_len {
        return None;
    }
    Some(out)
}

const BRANCH_VERSION: i16 = 12;

#[derive(Clone, Debug, PartialEq)]
pub struct BranchMeta {
    pub name: String,
    pub title: String,
    pub leaf_type: String,
    pub offset: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TreeIndex {
    pub branches: Vec<BranchMeta>,
}

fn read_u32_at(buf: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_be_bytes(buf.get(off..off + 4)?.try_into().ok()?))
}

fn find_new_class_tag(buf: &[u8], name: &str) -> Option<usize> {
    let tag = K_NEW_CLASS_TAG.to_be_bytes();
    let needle = name.as_bytes();
    let mut i = 0;
    while i + 4 + needle.len() + 1 <= buf.len() {
        if buf[i..i + 4] == tag {
            let rest = &buf[i + 4..];
            if rest.len() > needle.len()
                && rest[..needle.len()] == *needle
                && rest[needle.len()] == 0
            {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn leaf_type_from_title(title: &str) -> String {
    match title.rsplit_once('/') {
        Some((_, suffix)) => suffix.to_string(),
        None => String::new(),
    }
}

fn parse_branch_frame(obj: &[u8], start: usize) -> Option<(BranchMeta, u32, usize)> {
    let outer = read_u32_at(obj, start)?;
    if outer & K_BYTE_COUNT_MASK == 0 {
        return None;
    }
    let outer_len = (outer & !K_BYTE_COUNT_MASK) as usize;
    let frame_end = start.checked_add(4)?.checked_add(outer_len)?;
    if frame_end > obj.len() {
        return None;
    }
    let mut c = Cursor::new(obj, start);
    c.u32()?;
    let tag = c.u32()?;
    if tag == K_NEW_CLASS_TAG {
        c.class_tag_name()?;
    }
    let prefix_len = c.pos - start;
    let (version, inner_len) = c.read_version()?;
    if version != BRANCH_VERSION {
        return None;
    }
    if inner_len < 0 || inner_len as usize != outer_len.checked_sub(prefix_len)? {
        return None;
    }
    c.read_version()?;
    c.skip_version()?;
    c.u32()?;
    c.u32()?;
    let name = c.tstring()?;
    let title = c.tstring()?;
    let leaf_type = leaf_type_from_title(&title);
    Some((
        BranchMeta {
            name,
            title,
            leaf_type,
            offset: start as i64,
        },
        tag,
        frame_end,
    ))
}

pub fn parse_tree(bytes: &[u8], tree: &RootKey) -> Result<TreeIndex, &'static str> {
    if tree.class != "TTree" {
        return Err("key is not a TTree");
    }
    let obj = decompress_object(bytes, tree).ok_or("TTree object decompression failed")?;
    let tag_offset =
        find_new_class_tag(&obj, "TBranch").ok_or("fBranches absent in TTree object")?;
    let mut pos = tag_offset
        .checked_sub(4)
        .ok_or("TBranch frame offset underflow")?;
    let mut branch_ref: Option<u32> = None;
    let mut branches = Vec::new();
    while pos + 8 <= obj.len() {
        let outer = read_u32_at(&obj, pos).ok_or("branch frame count absent")?;
        if outer & K_BYTE_COUNT_MASK == 0 {
            break;
        }
        let tag = read_u32_at(&obj, pos + 4).ok_or("branch frame tag absent")?;
        let is_branch_ref = tag == K_NEW_CLASS_TAG || branch_ref == Some(tag);
        match parse_branch_frame(&obj, pos) {
            Some((meta, frame_tag, end)) => {
                if frame_tag != K_NEW_CLASS_TAG {
                    branch_ref = Some(frame_tag);
                }
                branches.push(meta);
                pos = end;
            }
            None => {
                if is_branch_ref {
                    return Err("TBranch frame inconsistent in fBranches index");
                }
                break;
            }
        }
    }
    if branches.is_empty() {
        return Err("no TBranch object parsed in fBranches index");
    }
    Ok(TreeIndex { branches })
}

pub const TSTREAMER_INFO_LAYOUT: &str = "TStreamerInfo header layout, measured from ROOT v6-36 docs (dobject.html/streamerinfo.html/tobject.html), TStreamerInfo::Streamer (io/io/src/TStreamerInfo.cxx:5616), TBufferFile::ReadVersion/ReadObjectAny/SkipVersion (io/io/src/TBufferFile.cxx:2933/:2519/:2862), TObject::Streamer (core/base/src/TObject.cxx:995); Version_t = Short_t (i16), byte order big-endian; object prefix: [byte count|kByteCountMask][class tag=0xffffffff][class name NUL], the tag is the second u32 only when the mask is present; TStreamerInfo ReadVersion [byte count|mask][version i16]; TNamed ReadVersion [byte count|mask][version i16]; TObject SkipVersion [version i16][fUniqueID u32][fBits u32] (10 bytes, no count); TNamed fName,fTitle TStrings; fCheckSum u32, fClassVersion i32; fElements TObjArray* [byte count|mask][tag=0xffffffff][name NUL][byte count|mask][version i16][TObject 10 bytes][fName TString][fSize i32 = n_members][fLowerBound i32]; the older 4-byte version form predates the byte-count mask and is not read here";

fn read_object_tag(c: &mut Cursor<'_>) -> Option<u32> {
    let first = c.u32()?;
    if first & K_BYTE_COUNT_MASK == 0 || first == K_NEW_CLASS_TAG {
        Some(first)
    } else {
        c.u32()
    }
}

pub fn parse_streamer_info_header(bytes: &[u8]) -> Option<StreamerInfoHeader> {
    let mut c = Cursor::new(bytes, 0);

    if read_object_tag(&mut c)? != K_NEW_CLASS_TAG {
        return None;
    }
    c.class_tag_name()?;

    let object_start = c.pos;
    let (version, byte_count) = c.read_version()?;
    let object_end = object_start
        .checked_add(4)?
        .checked_add(byte_count as usize)?;
    if object_end > bytes.len() {
        return None;
    }

    c.read_version()?;
    c.skip_version()?;
    c.u32()?;
    c.u32()?;

    let class_name = c.tstring()?;
    c.tstring()?;

    let checksum = c.u32()?;
    c.i32()?;

    if read_object_tag(&mut c)? != K_NEW_CLASS_TAG {
        return None;
    }
    c.class_tag_name()?;
    c.read_version()?;
    c.skip_version()?;
    c.u32()?;
    c.u32()?;
    c.tstring()?;
    let n_members = c.i32()?;
    c.i32()?;
    if n_members < 0 {
        return None;
    }

    Some(StreamerInfoHeader {
        version,
        byte_count,
        class_name,
        checksum,
        n_members,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &[u8] = &[
        0x72, 0x6f, 0x6f, 0x74, 0x00, 0x00, 0xcf, 0x0e, 0x00, 0x00, 0x00, 0x64, 0x02, 0x1b, 0xdb,
        0x13, 0x01, 0x05, 0xef, 0x49, 0x00, 0x00, 0x02, 0x6d, 0x00, 0x00, 0x00, 0x35, 0x00, 0x00,
        0x00, 0x96, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x8d, 0x75, 0x67, 0x00, 0x00, 0x15, 0x79,
        0x00, 0x01, 0x45, 0x2f, 0xb9, 0x28, 0x61, 0x92, 0x11, 0xe1, 0x97, 0x17, 0x01, 0x00, 0x00,
        0x7f, 0xbe, 0xef,
    ];

    #[test]
    fn magic_is_four_bytes() {
        assert!(is_root(b"root"));
        assert!(is_root(b"root\x00\x00\xcf\x0e"));
        assert!(!is_root(b"roo"));
        assert!(!is_root(b"Root"));
        assert!(!is_root(b""));
    }

    #[test]
    fn header_fixture_parses_measured_fields() {
        let h = parse_header(HEADER).expect("measured header parses");
        assert_eq!(h.version, 53006);
        assert_eq!(h.begin, 100);
        assert_eq!(h.end, 35379987);
        assert_eq!(h.seek_free, 17166153);
        assert_eq!(h.nbytes_free, 621);
        assert_eq!(h.nfree, 53);
        assert_eq!(h.nbytes_name, 150);
        assert_eq!(h.units, 4);
        assert_eq!(h.compress, 1);
        assert_eq!(h.seek_info, 9270631);
        assert_eq!(h.nbytes_info, 5497);
        assert!(!h.big);
        assert_eq!(
            h.uuid,
            [
                0x00, 0x01, 0x45, 0x2f, 0xb9, 0x28, 0x61, 0x92, 0x11, 0xe1, 0x97, 0x17, 0x01, 0x00,
                0x00, 0x7f, 0xbe, 0xef
            ]
        );
    }

    #[test]
    fn non_root_is_rejected() {
        assert!(parse_header(b"not a root file at all").is_none());
        assert!(parse_header(&HEADER[1..]).is_none());
    }

    #[test]
    fn key_list_absence_is_named_not_zero() {
        assert_eq!(read_file(HEADER), Err("directory key absent at fBEGIN"));
    }

    #[test]
    fn tree_index_refuses_without_a_readable_object() {
        let tree = RootKey {
            name: "AliVSD".to_string(),
            class: "TTree".to_string(),
            nbytes: 0,
            objlen: 0,
            keylen: 8,
            cycle: 1,
            seek_key: 0,
            seek_pdir: 0,
        };
        assert_eq!(
            parse_tree(&[0u8; 64], &tree),
            Err("TTree object decompression failed")
        );

        let mut directory = tree.clone();
        directory.class = "TDirectoryFile".to_string();
        assert_eq!(
            parse_tree(&[0u8; 64], &directory),
            Err("key is not a TTree")
        );
    }

    #[test]
    fn inflate_zlib_reads_a_stored_deflate_block() {
        let stream = [
            0x78, 0x01, 0x01, 0x05, 0x00, 0xfa, 0xff, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x06, 0x2c,
            0x02, 0x15,
        ];
        assert_eq!(inflate_zlib(&stream, 5).as_deref(), Some(&b"hello"[..]));
        assert_eq!(inflate_zlib(&stream, 4).map(|v| v.len()), Some(5));
    }

    #[test]
    fn decompress_object_reads_a_measured_root_compressed_frame() {
        const FRAME: &[u8] = &[
            0x5a, 0x4c, 0x08, 0x7c, 0x00, 0x00, 0x12, 0x01, 0x00, 0x78, 0x01, 0x73, 0x60, 0x60,
            0xe4, 0x63, 0x10, 0x76, 0x60, 0x60, 0x50, 0x60, 0x60, 0x04, 0x42, 0x20, 0x60, 0x66,
            0x60, 0xe0, 0xe0, 0x74, 0x49, 0x4d, 0x4e, 0xac, 0x0c, 0x29, 0x4a, 0x4d, 0x45, 0xb0,
            0x80, 0x6a, 0x38, 0xc0, 0x6a, 0x80, 0xea, 0x80, 0x6c, 0x36, 0x90, 0x6a, 0xe6, 0x97,
            0x40, 0x16, 0x17, 0x03, 0x13, 0x90, 0x2d, 0x6e, 0xef, 0x73, 0xe6, 0x2c, 0x50, 0x3b,
            0x51, 0xc0, 0xfe, 0x03, 0x8a, 0x32, 0x49, 0x08, 0x8f, 0xf9, 0x05, 0x54, 0xf4, 0xc5,
            0x95, 0xa5, 0x02, 0x20, 0x26, 0x8c, 0x06, 0x0b, 0xff, 0x07, 0x82, 0x77, 0x72, 0xb1,
            0x0c, 0x20, 0xfa, 0x9f, 0x99, 0x4d, 0x03, 0x58, 0x90, 0xdf, 0xc9, 0x81, 0x81, 0x01,
            0x88, 0x44, 0x81, 0xce, 0x86, 0xb9, 0x1e, 0x2c, 0x01, 0x26, 0x70, 0x4a, 0x20, 0x94,
            0x60, 0xb2, 0x00, 0xe7, 0x2a, 0x1f, 0x84,
        ];
        let key = RootKey {
            name: String::new(),
            class: "TTree".to_string(),
            nbytes: FRAME.len() as i32,
            objlen: 274,
            keylen: 0,
            cycle: 1,
            seek_key: 0,
            seek_pdir: 0,
        };
        let out = decompress_object(FRAME, &key).expect("measured frame decompresses");
        assert_eq!(out.len(), 274);
    }

    fn push_u32(b: &mut Vec<u8>, v: u32) {
        b.extend_from_slice(&v.to_be_bytes());
    }

    fn push_i32(b: &mut Vec<u8>, v: i32) {
        b.extend_from_slice(&v.to_be_bytes());
    }

    fn push_i16(b: &mut Vec<u8>, v: i16) {
        b.extend_from_slice(&v.to_be_bytes());
    }

    const SI_OBJECT_START: usize = 4 + 4 + "TStreamerInfo".len() + 1;

    fn streamer_info_fixture() -> (Vec<u8>, i32) {
        let mut b = Vec::new();
        push_u32(&mut b, K_BYTE_COUNT_MASK);
        push_u32(&mut b, K_NEW_CLASS_TAG);
        b.extend_from_slice(b"TStreamerInfo\0");

        let object_start = b.len();
        push_u32(&mut b, K_BYTE_COUNT_MASK);
        push_i16(&mut b, 9);
        push_u32(&mut b, K_BYTE_COUNT_MASK);
        push_i16(&mut b, 1);
        push_i16(&mut b, 1);
        push_u32(&mut b, 0);
        push_u32(&mut b, 0x0300_0000);
        b.push(6);
        b.extend_from_slice(b"AliVSD");
        b.push(0);
        push_u32(&mut b, 0xdead_beef);
        push_i32(&mut b, 5);
        push_u32(&mut b, K_BYTE_COUNT_MASK);
        push_u32(&mut b, K_NEW_CLASS_TAG);
        b.extend_from_slice(b"TObjArray\0");
        push_u32(&mut b, K_BYTE_COUNT_MASK);
        push_i16(&mut b, 1);
        push_i16(&mut b, 1);
        push_u32(&mut b, 0);
        push_u32(&mut b, 0x0300_0000);
        b.push(0);
        push_i32(&mut b, 12);
        push_i32(&mut b, 0);

        let byte_count = (b.len() - object_start - 4) as u32;
        b[object_start..object_start + 4]
            .copy_from_slice(&(byte_count | K_BYTE_COUNT_MASK).to_be_bytes());
        (b, byte_count as i32)
    }

    #[test]
    fn streamer_info_header_fixture_parses_measured_fields() {
        let (bytes, byte_count) = streamer_info_fixture();
        let h = parse_streamer_info_header(&bytes).expect("measured header parses");
        assert_eq!(h.version, 9);
        assert_eq!(h.byte_count, byte_count);
        assert_eq!(h.class_name, "AliVSD");
        assert_eq!(h.checksum, 0xdead_beef);
        assert_eq!(h.n_members, 12);
    }

    #[test]
    fn streamer_info_header_rejects_empty_truncated_and_corrupt_slices() {
        assert_eq!(parse_streamer_info_header(&[]), None);

        let (bytes, _) = streamer_info_fixture();
        assert_eq!(parse_streamer_info_header(&bytes[..bytes.len() / 2]), None);

        let mut absurd = bytes.clone();
        absurd[SI_OBJECT_START..SI_OBJECT_START + 4]
            .copy_from_slice(&(K_BYTE_COUNT_MASK | 0x3fff_ffff).to_be_bytes());
        assert_eq!(parse_streamer_info_header(&absurd), None);

        let mut reference = bytes.clone();
        reference[0..4].copy_from_slice(&2u32.to_be_bytes());
        assert_eq!(parse_streamer_info_header(&reference), None);
    }
}
