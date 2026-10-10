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

pub fn parse_tree(bytes: &[u8], tree: &RootKey) -> Result<Vec<(f64, f64, u32)>, &'static str> {
    if tree.class != "TTree" {
        return Err("key is not a TTree");
    }
    if tree.seek_key < 0 || tree.keylen < 0 {
        return Err("TTree key offset negative");
    }
    let payload = (tree.seek_key as usize)
        .checked_add(tree.keylen as usize)
        .ok_or("TTree payload offset out of range")?;
    if payload >= bytes.len() {
        return Err("TTree payload absent");
    }
    Err(
        "TTree scalar decode pending: TStreamerInfo streamers (fBranches/fLeaves), TBasket free-segment (fEntryOffsetLen)",
    )
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
    fn tree_scalar_leaf_decode_names_the_missing_substructures() {
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
            Err(
                "TTree scalar decode pending: TStreamerInfo streamers (fBranches/fLeaves), TBasket free-segment (fEntryOffsetLen)"
            )
        );

        let mut directory = tree.clone();
        directory.class = "TDirectoryFile".to_string();
        assert_eq!(
            parse_tree(&[0u8; 64], &directory),
            Err("key is not a TTree")
        );
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
