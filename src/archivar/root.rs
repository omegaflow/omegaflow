pub const MAGIC: [u8; 4] = *b"root";

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
}
