const BZ_MAX_CODE_LEN: usize = 23;

trait ByteSource {
    fn next(&mut self) -> Option<u8>;
}

struct SliceSource<'a> {
    data: &'a [u8],
    pos: usize,
}

impl ByteSource for SliceSource<'_> {
    fn next(&mut self) -> Option<u8> {
        let b = *self.data.get(self.pos)?;
        self.pos += 1;
        Some(b)
    }
}

struct ReaderSource<R> {
    reader: R,
}

impl<R: std::io::Read> ByteSource for ReaderSource<R> {
    fn next(&mut self) -> Option<u8> {
        let mut b = [0u8; 1];
        self.reader.read_exact(&mut b).ok().map(|()| b[0])
    }
}

struct Bits<S: ByteSource> {
    src: S,
    pos: usize,
    buf: u64,
    live: u32,
}

impl<S: ByteSource> Bits<S> {
    fn new(src: S) -> Self {
        Bits {
            src,
            pos: 0,
            buf: 0,
            live: 0,
        }
    }

    fn read(&mut self, n: u32) -> Option<u64> {
        while self.live < n {
            let b = self.src.next()? as u64;
            self.pos += 1;
            self.buf = (self.buf << 8) | b;
            self.live += 8;
        }
        let shift = self.live - n;
        let v = (self.buf >> shift) & ((1u64 << n) - 1);
        self.live = shift;
        Some(v)
    }
}

struct DecodeTable {
    limit: [i32; BZ_MAX_CODE_LEN + 1],
    base: [i32; BZ_MAX_CODE_LEN + 1],
    perm: Vec<u16>,
    min_len: u8,
}

fn build_table(lengths: &[u8]) -> Option<DecodeTable> {
    let alpha = lengths.len();
    if alpha == 0 {
        return None;
    }
    let mut min_len = BZ_MAX_CODE_LEN as u8;
    let mut max_len = 0u8;
    for &l in lengths {
        if !(1..=20).contains(&l) {
            return None;
        }
        if l < min_len {
            min_len = l;
        }
        if l > max_len {
            max_len = l;
        }
    }
    let mut perm = Vec::with_capacity(alpha);
    for len in min_len..=max_len {
        for (j, &l) in lengths.iter().enumerate() {
            if l == len {
                perm.push(j as u16);
            }
        }
    }
    let mut base = [0i32; BZ_MAX_CODE_LEN + 1];
    for &l in lengths {
        base[(l + 1) as usize] += 1;
    }
    for i in 1..=BZ_MAX_CODE_LEN {
        base[i] += base[i - 1];
    }
    let mut limit = [0i32; BZ_MAX_CODE_LEN + 1];
    let mut vec = 0i32;
    for len in min_len..=max_len {
        vec += base[(len + 1) as usize] - base[len as usize];
        limit[len as usize] = vec - 1;
        vec <<= 1;
    }
    for len in (min_len + 1)..=max_len {
        base[len as usize] = ((limit[(len - 1) as usize] + 1) << 1) - base[len as usize];
    }
    Some(DecodeTable {
        limit,
        base,
        perm,
        min_len,
    })
}

fn decode_sym<S: ByteSource>(bits: &mut Bits<S>, tab: &DecodeTable) -> Option<u16> {
    let mut zn = tab.min_len as u32;
    let mut zvec = bits.read(zn)? as i32;
    loop {
        if zn > 20 {
            return None;
        }
        if zvec <= tab.limit[zn as usize] {
            break;
        }
        zn += 1;
        zvec = (zvec << 1) | (bits.read(1)? as i32);
    }
    let idx = (zvec - tab.base[zn as usize]) as usize;
    tab.perm.get(idx).copied()
}

fn next_sym<S: ByteSource>(
    bits: &mut Bits<S>,
    selector: &[u8],
    tabs: &[DecodeTable],
    group_no: &mut i32,
    group_pos: &mut u32,
) -> Option<u16> {
    if *group_pos == 0 {
        *group_no += 1;
        if *group_no >= selector.len() as i32 {
            return None;
        }
        *group_pos = 50;
    }
    *group_pos -= 1;
    decode_sym(bits, &tabs[selector[*group_no as usize] as usize])
}

fn bw_inverse(block: &[u8], orig_ptr: usize) -> Option<Vec<u8>> {
    let n = block.len();
    if orig_ptr >= n {
        return None;
    }
    let mut count = [0u32; 256];
    for &b in block {
        count[b as usize] += 1;
    }
    let mut c = [0u32; 256];
    let mut sum = 0u32;
    for i in 0..256 {
        c[i] = sum;
        sum += count[i];
    }
    let mut rank = c;
    let mut lf = vec![0u32; n];
    for (i, &b) in block.iter().enumerate() {
        lf[i] = rank[b as usize];
        rank[b as usize] += 1;
    }
    let mut out = vec![0u8; n];
    let mut idx = orig_ptr;
    for k in (0..n).rev() {
        out[k] = block[idx];
        idx = lf[idx] as usize;
    }
    Some(out)
}

fn rle_expand(block: &[u8]) -> Option<Vec<u8>> {
    let n = block.len();
    let mut out = Vec::with_capacity(n);
    let mut i = 0usize;
    while i < n {
        let c = block[i];
        i += 1;
        let mut run = 1usize;
        if i < n && block[i] == c {
            run = 2;
            i += 1;
            if i < n && block[i] == c {
                run = 3;
                i += 1;
                if i < n && block[i] == c {
                    i += 1;
                    let count = *block.get(i)? as usize;
                    i += 1;
                    run = 4 + count;
                }
            }
        }
        out.resize(out.len() + run, c);
    }
    Some(out)
}

fn decode_block<S: ByteSource>(bits: &mut Bits<S>, orig_ptr: usize) -> Option<Vec<u8>> {
    let used_map = bits.read(16)? as u16;
    let mut in_use = [false; 256];
    let mut n_in_use = 0usize;
    for i in 0..16 {
        if used_map & (1 << (15 - i)) != 0 {
            let group = bits.read(16)? as u16;
            for j in 0..16 {
                if group & (1 << (15 - j)) != 0 {
                    in_use[i * 16 + j] = true;
                    n_in_use += 1;
                }
            }
        }
    }
    if n_in_use == 0 {
        return None;
    }
    let alpha_size = n_in_use + 2;

    let mut seq_to_unseq = [0u8; 256];
    let mut k = 0usize;
    for (b, &iu) in in_use.iter().enumerate() {
        if iu {
            seq_to_unseq[k] = b as u8;
            k += 1;
        }
    }

    let n_groups = bits.read(3)? as usize;
    if !(2..=6).contains(&n_groups) {
        return None;
    }
    let n_selectors = bits.read(15)? as usize;
    if n_selectors < 1 {
        return None;
    }

    let mut selector_mtf = vec![0u8; n_selectors];
    for slot in selector_mtf.iter_mut() {
        let mut j = 0u8;
        loop {
            if bits.read(1)? == 0 {
                break;
            }
            j += 1;
            if j as usize >= n_groups {
                return None;
            }
        }
        *slot = j;
    }

    let mut pos = [0u8; 6];
    for (v, p) in pos.iter_mut().enumerate().take(n_groups) {
        *p = v as u8;
    }
    let mut selector = vec![0u8; n_selectors];
    for i in 0..n_selectors {
        let mut v = selector_mtf[i] as usize;
        let tmp = pos[v];
        while v > 0 {
            pos[v] = pos[v - 1];
            v -= 1;
        }
        pos[0] = tmp;
        selector[i] = tmp;
    }

    let mut lengths = vec![vec![0u8; alpha_size]; n_groups];
    for row in lengths.iter_mut().take(n_groups) {
        let mut curr = bits.read(5)? as i32;
        for slot in row.iter_mut() {
            loop {
                if !(1..=20).contains(&curr) {
                    return None;
                }
                if bits.read(1)? == 0 {
                    break;
                }
                if bits.read(1)? == 0 {
                    curr += 1;
                } else {
                    curr -= 1;
                }
            }
            *slot = curr as u8;
        }
    }

    let mut tabs = Vec::with_capacity(n_groups);
    for row in lengths.iter().take(n_groups) {
        tabs.push(build_table(row)?);
    }

    let eob = n_in_use + 1;
    let mut mtf: Vec<u8> = (0..n_in_use).map(|i| i as u8).collect();
    let mut block: Vec<u8> = Vec::new();

    let mut group_no = -1i32;
    let mut group_pos = 0u32;
    let mut sym = next_sym(bits, &selector, &tabs, &mut group_no, &mut group_pos)? as usize;

    loop {
        if sym == eob {
            break;
        }
        if sym == 0 || sym == 1 {
            let mut es: i32 = -1;
            let mut n: i32 = 1;
            loop {
                if n >= 2 * 1024 * 1024 {
                    return None;
                }
                if sym == 0 {
                    es += n;
                } else {
                    es += 2 * n;
                }
                n *= 2;
                sym = next_sym(bits, &selector, &tabs, &mut group_no, &mut group_pos)? as usize;
                if sym != 0 && sym != 1 {
                    break;
                }
            }
            es += 1;
            let byte = seq_to_unseq[mtf[0] as usize];
            for _ in 0..es {
                block.push(byte);
            }
            continue;
        }
        let pos = sym - 1;
        let m = mtf[pos];
        for i in (1..=pos).rev() {
            mtf[i] = mtf[i - 1];
        }
        mtf[0] = m;
        block.push(seq_to_unseq[m as usize]);
        sym = next_sym(bits, &selector, &tabs, &mut group_no, &mut group_pos)? as usize;
    }

    let l = bw_inverse(&block, orig_ptr)?;
    rle_expand(&l)
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        let idx = ((crc >> 24) as u8 ^ b) as usize;
        crc = (crc << 8) ^ BZ_CRC_TABLE[idx];
    }
    !crc
}

const BZ_CRC_TABLE: [u32; 256] = build_crc_table();

const fn build_crc_table() -> [u32; 256] {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut c = (i as u32) << 24;
        let mut k = 0;
        while k < 8 {
            if c & 0x8000_0000 != 0 {
                c = (c << 1) ^ 0x04C1_1DB7;
            } else {
                c <<= 1;
            }
            k += 1;
        }
        t[i] = c;
        i += 1;
    }
    t
}

pub fn decompress(data: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut stream_start = 0usize;
    loop {
        let stream = data.get(stream_start..)?;
        if stream.len() < 4 || &stream[0..3] != b"BZh" {
            break;
        }
        if !(b'1'..=b'9').contains(&stream[3]) {
            break;
        }
        let mut bits = Bits::new(SliceSource {
            data: &stream[4..],
            pos: 0,
        });
        let mut combined_crc = 0u32;
        loop {
            let magic = bits.read(48)?;
            if magic == 0x177245385090 {
                let stored_combined = bits.read(32)? as u32;
                if stored_combined != combined_crc {
                    return None;
                }
                break;
            }
            if magic != 0x314159265359 {
                return None;
            }
            let stored_block_crc = bits.read(32)? as u32;
            let randomised = bits.read(1)?;
            if randomised != 0 {
                return None;
            }
            let orig_ptr = bits.read(24)? as usize;
            let block = decode_block(&mut bits, orig_ptr)?;
            let crc = crc32(&block);
            if crc != stored_block_crc {
                return None;
            }
            combined_crc = combined_crc.rotate_left(1);
            combined_crc ^= stored_block_crc;
            out.extend_from_slice(&block);
        }
        stream_start += 4 + bits.pos;
        if bits.pos == 0 {
            break;
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

pub struct Bz2Decoder<R: std::io::Read> {
    bits: Bits<ReaderSource<R>>,
    block: Vec<u8>,
    block_pos: usize,
    combined_crc: u32,
    done: bool,
}

impl<R: std::io::Read> Bz2Decoder<R> {
    pub fn new(reader: R) -> std::io::Result<Self> {
        let mut bits = Bits::new(ReaderSource { reader });
        let mut header = [0u8; 4];
        for slot in header.iter_mut() {
            *slot = bits.src.next().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "bzip2 header absent")
            })?;
        }
        if &header[0..3] != b"BZh" || !(b'1'..=b'9').contains(&header[3]) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "bzip2 header malformed",
            ));
        }
        Ok(Bz2Decoder {
            bits,
            block: Vec::new(),
            block_pos: 0,
            combined_crc: 0,
            done: false,
        })
    }

    fn next_block(&mut self) -> std::io::Result<()> {
        let magic = self.bits.read(48).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "bzip2 stream ended early",
            )
        })?;
        if magic == 0x177245385090 {
            let stored = self.bits.read(32).ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "bzip2 stream CRC absent")
            })? as u32;
            if stored != self.combined_crc {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "bzip2 combined CRC mismatch",
                ));
            }
            self.done = true;
            return Ok(());
        }
        if magic != 0x314159265359 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "bzip2 block magic absent",
            ));
        }
        let stored_block_crc = self.bits.read(32).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "bzip2 block CRC absent")
        })? as u32;
        let randomised = self.bits.read(1).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "bzip2 randomised bit absent",
            )
        })?;
        if randomised != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "bzip2 randomised block refused",
            ));
        }
        let orig_ptr = self.bits.read(24).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "bzip2 orig pointer absent",
            )
        })? as usize;
        let block = decode_block(&mut self.bits, orig_ptr).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "bzip2 block decode returned void",
            )
        })?;
        if crc32(&block) != stored_block_crc {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "bzip2 block CRC mismatch",
            ));
        }
        self.combined_crc = self.combined_crc.rotate_left(1);
        self.combined_crc ^= stored_block_crc;
        self.block = block;
        self.block_pos = 0;
        Ok(())
    }
}

impl<R: std::io::Read> std::io::Read for Bz2Decoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            if self.block_pos < self.block.len() {
                let n = buf.len().min(self.block.len() - self.block_pos);
                buf[..n].copy_from_slice(&self.block[self.block_pos..self.block_pos + n]);
                self.block_pos += n;
                return Ok(n);
            }
            if self.done {
                return Ok(0);
            }
            self.next_block()?;
        }
    }
}

pub fn decompress_reader<R: std::io::Read, W: std::io::Write>(
    reader: R,
    out: &mut W,
) -> std::io::Result<u64> {
    let mut dec = Bz2Decoder::new(reader)?;
    std::io::copy(&mut dec, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn decompress_sample() {
        let bz = from_hex(
            "425a6839314159265359e400c03c000054d980001040017fe03ffffff02000890ca8f500320034d01a34055290f5340d\
             34001a6869e9a91f8a1a0402178d83c6425343820704d428acf6458566d371bce0718e91acf6d9117aa5e7c315c6660e\
             64cc5484982a4cac92e4d245a5cadba16a4b13796589d145a86f570e2fc5dc914e14243900300f00",
        );
        let expected = b"the quick brown fox jumps over the lazy dog. the quick brown fox jumps over the lazy dog. omegaflow omegaflow omegaflow omegaflow 0123456789 0123456789 0123456789 bzip2 bzip2 bzip2 bzip2 bzip2\n";
        assert_eq!(decompress(&bz).as_deref(), Some(&expected[..]));
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(decompress(b"").is_none());
        assert!(decompress(b"PK\x03\x04").is_none());
        assert!(decompress(b"BZh9").is_none());
    }

    #[test]
    fn decompress_reader_matches_decompress() {
        let bz = from_hex(
            "425a6839314159265359e400c03c000054d980001040017fe03ffffff02000890ca8f500320034d01a34055290f5340d\
             34001a6869e9a91f8a1a0402178d83c6425343820704d428acf6458566d371bce0718e91acf6d9117aa5e7c315c6660e\
             64cc5484982a4cac92e4d245a5cadba16a4b13796589d145a86f570e2fc5dc914e14243900300f00",
        );
        let expected = decompress(&bz).unwrap();
        let mut out = Vec::new();
        let n = decompress_reader(std::io::Cursor::new(&bz), &mut out).unwrap();
        assert_eq!(n as usize, expected.len());
        assert_eq!(out, expected);
    }

    #[test]
    fn decompress_reader_rejects_foreign_bytes() {
        let mut out = Vec::new();
        assert!(decompress_reader(std::io::Cursor::new(b"PK\x03\x04"), &mut out).is_err());
    }
}
