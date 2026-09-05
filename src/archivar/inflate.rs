use std::collections::HashMap;

const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];
const CL_ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    bit: u8,
}

impl<'a> BitReader<'a> {
    fn read_bit(&mut self) -> Option<u32> {
        let byte = *self.data.get(self.pos)?;
        let b = (byte >> self.bit) & 1;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.pos += 1;
        }
        Some(b as u32)
    }

    fn read_bits(&mut self, n: u8) -> Option<u32> {
        let mut v = 0u32;
        for i in 0..n {
            v |= self.read_bit()? << i;
        }
        Some(v)
    }

    fn align_byte(&mut self) {
        if self.bit != 0 {
            self.bit = 0;
            self.pos += 1;
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        let b = *self.data.get(self.pos)?;
        self.pos += 1;
        Some(b)
    }

    fn read_le16(&mut self) -> Option<u16> {
        let lo = self.read_byte()? as u16;
        let hi = self.read_byte()? as u16;
        Some(lo | (hi << 8))
    }
}

struct Huffman {
    map: HashMap<(u32, u8), u16>,
    max_len: u8,
}

fn canonical(lengths: &[u8]) -> Option<Huffman> {
    let mut count = [0u16; 16];
    let mut max_len = 0u8;
    for &l in lengths.iter() {
        if l > 15 {
            return None;
        }
        if l != 0 {
            count[l as usize] += 1;
            if l > max_len {
                max_len = l;
            }
        }
    }
    if max_len == 0 {
        return None;
    }
    let mut next_code = [0u32; 16];
    let mut code = 0u32;
    for bits in 1..=15 {
        code = (code + count[bits - 1] as u32) << 1;
        next_code[bits] = code;
        if code >= (1u32 << bits) && count[bits] > 0 {
            return None;
        }
    }
    let mut map = HashMap::new();
    for (sym, &l) in lengths.iter().enumerate() {
        if l == 0 {
            continue;
        }
        let c = next_code[l as usize];
        next_code[l as usize] += 1;
        map.insert((c, l), sym as u16);
    }
    Some(Huffman { map, max_len })
}

fn decode_symbol(br: &mut BitReader, huff: &Huffman) -> Option<u16> {
    let mut code = 0u32;
    for len in 1..=huff.max_len {
        code = (code << 1) | br.read_bit()?;
        if let Some(&sym) = huff.map.get(&(code, len)) {
            return Some(sym);
        }
    }
    None
}

fn fixed_lit() -> Huffman {
    let mut lengths = vec![0u8; 288];
    for l in lengths.iter_mut().take(144) {
        *l = 8;
    }
    for l in lengths.iter_mut().take(256).skip(144) {
        *l = 9;
    }
    for l in lengths.iter_mut().take(280).skip(256) {
        *l = 7;
    }
    for l in lengths.iter_mut().skip(280) {
        *l = 8;
    }
    canonical(&lengths).expect("fixed literal table")
}

fn fixed_dist() -> Huffman {
    let lengths = vec![5u8; 32];
    canonical(&lengths).expect("fixed distance table")
}

fn decode_block(
    br: &mut BitReader,
    lit: &Huffman,
    dist: &Huffman,
    out: &mut Vec<u8>,
) -> Option<()> {
    loop {
        let sym = decode_symbol(br, lit)?;
        if sym < 256 {
            out.push(sym as u8);
        } else if sym == 256 {
            return Some(());
        } else {
            let li = (sym - 257) as usize;
            if li >= LEN_BASE.len() {
                return None;
            }
            let len = LEN_BASE[li] as usize + br.read_bits(LEN_EXTRA[li])? as usize;
            let dcode = decode_symbol(br, dist)? as usize;
            if dcode >= DIST_BASE.len() {
                return None;
            }
            let d = DIST_BASE[dcode] as usize + br.read_bits(DIST_EXTRA[dcode])? as usize;
            if d > out.len() {
                return None;
            }
            for _ in 0..len {
                let idx = out.len() - d;
                let b = out[idx];
                out.push(b);
            }
        }
    }
}

pub fn gunzip(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 10 || data[0] != 0x1f || data[1] != 0x8b {
        return None;
    }
    let flags = data[3];
    let mut i = 10usize;
    if flags & 0x04 != 0 {
        let xlen = u16::from_le_bytes([data[i], data[i + 1]]) as usize;
        i += 2 + xlen;
    }
    if flags & 0x08 != 0 {
        while data.get(i).copied() != Some(0) {
            i += 1;
        }
        i += 1;
    }
    if flags & 0x10 != 0 {
        while data.get(i).copied() != Some(0) {
            i += 1;
        }
        i += 1;
    }
    if flags & 0x02 != 0 {
        i += 2;
    }
    inflate(&data[i..])
}

pub fn inflate(data: &[u8]) -> Option<Vec<u8>> {
    let mut br = BitReader {
        data,
        pos: 0,
        bit: 0,
    };
    let mut out = Vec::new();
    loop {
        let bfinal = br.read_bit()?;
        let btype = br.read_bits(2)?;
        match btype {
            0 => {
                br.align_byte();
                let len = br.read_le16()? as usize;
                let nlen = br.read_le16()? as usize;
                if len != (!nlen) & 0xffff {
                    return None;
                }
                for _ in 0..len {
                    out.push(br.read_byte()?);
                }
            }
            1 => {
                let lit = fixed_lit();
                let dist = fixed_dist();
                decode_block(&mut br, &lit, &dist, &mut out)?;
            }
            2 => {
                let hlit = br.read_bits(5)? as usize + 257;
                let hdist = br.read_bits(5)? as usize + 1;
                let hclen = br.read_bits(4)? as usize + 4;
                let mut cl_lengths = [0u8; 19];
                for &o in CL_ORDER.iter().take(hclen) {
                    cl_lengths[o] = br.read_bits(3)? as u8;
                }
                let cl_huff = canonical(&cl_lengths)?;
                let mut lengths = vec![0u8; hlit + hdist];
                let mut i = 0usize;
                while i < lengths.len() {
                    let sym = decode_symbol(&mut br, &cl_huff)?;
                    match sym {
                        0..=15 => {
                            lengths[i] = sym as u8;
                            i += 1;
                        }
                        16 => {
                            let prev = *lengths.get(i.wrapping_sub(1))?;
                            let n = 3 + br.read_bits(2)? as usize;
                            for _ in 0..n {
                                if i >= lengths.len() {
                                    return None;
                                }
                                lengths[i] = prev;
                                i += 1;
                            }
                        }
                        17 => {
                            let n = 3 + br.read_bits(3)? as usize;
                            for _ in 0..n {
                                if i >= lengths.len() {
                                    return None;
                                }
                                lengths[i] = 0;
                                i += 1;
                            }
                        }
                        18 => {
                            let n = 11 + br.read_bits(7)? as usize;
                            for _ in 0..n {
                                if i >= lengths.len() {
                                    return None;
                                }
                                lengths[i] = 0;
                                i += 1;
                            }
                        }
                        _ => return None,
                    }
                }
                let lit = canonical(&lengths[..hlit])?;
                let dist = canonical(&lengths[hlit..])?;
                decode_block(&mut br, &lit, &dist, &mut out)?;
            }
            _ => return None,
        }
        if bfinal == 1 {
            break;
        }
    }
    Some(out)
}

fn le16(b: &[u8], off: usize) -> u16 {
    b[off] as u16 | ((b[off + 1] as u16) << 8)
}

fn le32(b: &[u8], off: usize) -> u32 {
    b[off] as u32
        | ((b[off + 1] as u32) << 8)
        | ((b[off + 2] as u32) << 16)
        | ((b[off + 3] as u32) << 24)
}

pub fn unzip(data: &[u8]) -> Option<Vec<u8>> {
    let mut i = 0usize;
    while i + 30 <= data.len() {
        if &data[i..i + 4] != b"PK\x03\x04" {
            i += 1;
            continue;
        }
        let method = le16(data, i + 8);
        let comp_size = le32(data, i + 18) as usize;
        let name_len = le16(data, i + 26) as usize;
        let extra_len = le16(data, i + 28) as usize;
        let start = i + 30 + name_len + extra_len;
        if start > data.len() {
            return None;
        }
        return match method {
            0 => {
                if comp_size == 0 {
                    return None;
                }
                if start + comp_size > data.len() {
                    return None;
                }
                Some(data[start..start + comp_size].to_vec())
            }
            8 => {
                if comp_size == 0 {
                    inflate(&data[start..])
                } else {
                    if start + comp_size > data.len() {
                        return None;
                    }
                    inflate(&data[start..start + comp_size])
                }
            }
            _ => None,
        };
    }
    None
}

pub fn gunzip_stream<R: std::io::Read>(src: R, mut sink: impl FnMut(&[u8])) -> Result<u64, String> {
    let mut z = ZStream {
        src,
        inbuf: [0u8; 65536],
        in_len: 0,
        in_pos: 0,
        eof: false,
        bitbuf: 0,
        nbits: 0,
        win: Vec::new(),
        out_len: 0u64,
        crc: 0xFFFF_FFFFu32,
        emit: &mut sink,
    };
    let mut total = 0u64;
    loop {
        let mut header = [0u8; 2];
        match z.read_raw(&mut header) {
            Ok(0) => break,
            Ok(1) => return Err("gzip stream ends inside a member header".into()),
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        if header[0] != 0x1f || header[1] != 0x8b {
            return Err("gzip member header carries no 1f 8b magic".into());
        }
        total += z.decode_member()?;
    }
    Ok(total)
}

struct ZStream<'a, R: std::io::Read> {
    src: R,
    inbuf: [u8; 65536],
    in_len: usize,
    in_pos: usize,
    eof: bool,
    bitbuf: u64,
    nbits: u8,
    win: Vec<u8>,
    out_len: u64,
    crc: u32,
    emit: &'a mut dyn FnMut(&[u8]),
}

impl<R: std::io::Read> ZStream<'_, R> {
    fn read_raw(&mut self, out: &mut [u8]) -> Result<usize, String> {
        let mut got = 0usize;
        while got < out.len() {
            if self.in_pos >= self.in_len {
                if self.eof {
                    break;
                }
                self.in_len = self
                    .src
                    .read(&mut self.inbuf)
                    .map_err(|e| format!("compressed read returned void: {e}"))?;
                self.in_pos = 0;
                if self.in_len == 0 {
                    self.eof = true;
                    break;
                }
            }
            let n = (self.in_len - self.in_pos).min(out.len() - got);
            out[got..got + n].copy_from_slice(&self.inbuf[self.in_pos..self.in_pos + n]);
            self.in_pos += n;
            got += n;
        }
        Ok(got)
    }

    fn bit(&mut self) -> Option<u32> {
        if self.nbits == 0 {
            if self.eof {
                return None;
            }
            let mut b = [0u8; 1];
            match self.read_raw(&mut b) {
                Ok(1) => {
                    self.bitbuf |= (b[0] as u64) << self.nbits;
                    self.nbits += 8;
                }
                _ => return None,
            }
        }
        let v = (self.bitbuf & 1) as u32;
        self.bitbuf >>= 1;
        self.nbits -= 1;
        Some(v)
    }

    fn bits(&mut self, n: u8) -> Option<u32> {
        let mut v = 0u32;
        for i in 0..n {
            v |= self.bit()? << i;
        }
        Some(v)
    }

    fn align_byte(&mut self) {
        self.bitbuf = 0;
        self.nbits = 0;
    }

    fn push(&mut self, b: u8) {
        self.win.push(b);
        self.out_len += 1;
        self.crc = (self.crc >> 8) ^ CRC_TABLE[((self.crc ^ b as u32) & 0xff) as usize];
        if self.win.len() >= 1 << 20 {
            let keep = 1 << 15;
            let flush = self.win.len() - keep;
            (self.emit)(&self.win[..flush]);
            self.win.drain(..flush);
        }
    }

    fn flush_win(&mut self) {
        if !self.win.is_empty() {
            (self.emit)(&self.win);
            self.win.clear();
        }
    }

    fn decode_member(&mut self) -> Result<u64, String> {
        let mut rest = [0u8; 8];
        match self.read_raw(&mut rest[..8]) {
            Ok(8) => {}
            _ => return Err("gzip member header is shorter than 10 bytes".into()),
        }
        if rest[0] != 8 {
            return Err("gzip member header carries no deflate method".into());
        }
        let flags = rest[1];
        if flags & 0x04 != 0 {
            match self.read_raw(&mut rest[..2]) {
                Ok(2) => {}
                _ => return Err("gzip member header carries no FEXTRA length".into()),
            }
            let xlen = rest[0] as usize | ((rest[1] as usize) << 8);
            let mut skip = vec![0u8; xlen];
            if self.read_raw(&mut skip)? != xlen {
                return Err("gzip FEXTRA field is shorter than its length".into());
            }
            skip.clear();
        }
        if flags & 0x08 != 0 {
            loop {
                if self.read_raw(&mut rest[..1])? != 1 {
                    return Err("gzip FNAME field carries no terminator".into());
                }
                if rest[0] == 0 {
                    break;
                }
            }
        }
        if flags & 0x10 != 0 {
            loop {
                if self.read_raw(&mut rest[..1])? != 1 {
                    return Err("gzip FCOMMENT field carries no terminator".into());
                }
                if rest[0] == 0 {
                    break;
                }
            }
        }
        if flags & 0x02 != 0 {
            if self.read_raw(&mut rest[..2])? != 2 {
                return Err("gzip FHCRC field is shorter than 2 bytes".into());
            }
        }
        self.decode_deflate()?;
        self.align_byte();
        let mut trailer = [0u8; 8];
        match self.read_raw(&mut trailer) {
            Ok(8) => {}
            _ => return Err("gzip member trailer is shorter than 8 bytes".into()),
        }
        let stored_crc = u32::from_le_bytes([trailer[0], trailer[1], trailer[2], trailer[3]]);
        let stored_len =
            u32::from_le_bytes([trailer[4], trailer[5], trailer[6], trailer[7]]) as u64;
        let final_crc = self.crc ^ 0xFFFF_FFFF;
        let member_len = self.out_len;
        self.flush_win();
        if stored_crc != final_crc {
            return Err(format!(
                "gzip trailer crc32 {stored_crc:08x} differs from the stream crc32 {final_crc:08x}"
            ));
        }
        if (stored_len as u64) != (member_len & 0xFFFF_FFFF) {
            return Err(format!(
                "gzip trailer isize {stored_len} differs from the stream length {member_len}"
            ));
        }
        self.out_len = 0;
        self.crc = 0xFFFF_FFFF;
        Ok(member_len)
    }

    fn decode_deflate(&mut self) -> Result<(), String> {
        loop {
            let bfinal = self
                .bit()
                .ok_or("deflate stream ends inside a block header")?;
            let btype = self
                .bits(2)
                .ok_or("deflate stream ends inside a block header")?;
            match btype {
                0 => {
                    self.align_byte();
                    let mut hdr = [0u8; 4];
                    match self.read_raw(&mut hdr) {
                        Ok(4) => {}
                        _ => {
                            return Err("deflate stored block header is shorter than 4 bytes".into())
                        }
                    }
                    let len = hdr[0] as usize | ((hdr[1] as usize) << 8);
                    let nlen = hdr[2] as usize | ((hdr[3] as usize) << 8);
                    if len != (!nlen) & 0xffff {
                        return Err(
                            "deflate stored block length disagrees with its complement".into()
                        );
                    }
                    for _ in 0..len {
                        let mut b = [0u8; 1];
                        match self.read_raw(&mut b) {
                            Ok(1) => {}
                            _ => {
                                return Err(
                                    "deflate stored block data is shorter than its length".into()
                                )
                            }
                        }
                        self.push(b[0]);
                    }
                }
                1 => {
                    let lit = fixed_lit();
                    let dist = fixed_dist();
                    self.decode_huff_block(&lit, &dist)?;
                }
                2 => {
                    let hlit =
                        self.bits(5).ok_or("deflate block header ends early")? as usize + 257;
                    let hdist = self.bits(5).ok_or("deflate block header ends early")? as usize + 1;
                    let hclen = self.bits(4).ok_or("deflate block header ends early")? as usize + 4;
                    let mut cl_lengths = [0u8; 19];
                    for &o in CL_ORDER.iter().take(hclen) {
                        let v = self.bits(3).ok_or("deflate code-length table ends early")? as u8;
                        cl_lengths[o] = v;
                    }
                    let cl_huff = canonical(&cl_lengths)
                        .ok_or("deflate code-length table is not canonical")?;
                    let mut lengths = vec![0u8; hlit + hdist];
                    let mut k = 0usize;
                    while k < lengths.len() {
                        let sym = self
                            .decode_sym(&cl_huff)
                            .ok_or("deflate code-length stream ends early")?;
                        match sym {
                            0..=15 => {
                                lengths[k] = sym as u8;
                                k += 1;
                            }
                            16 => {
                                let prev = *lengths
                                    .get(k.wrapping_sub(1))
                                    .ok_or("deflate code-length 16 precedes any length")?;
                                let n =
                                    3 + self.bits(2).ok_or("deflate code-length 16 ends early")?
                                        as usize;
                                for _ in 0..n {
                                    if k >= lengths.len() {
                                        return Err(
                                            "deflate code-length 16 overruns the tables".into()
                                        );
                                    }
                                    lengths[k] = prev;
                                    k += 1;
                                }
                            }
                            17 => {
                                let n =
                                    3 + self.bits(3).ok_or("deflate code-length 17 ends early")?
                                        as usize;
                                for _ in 0..n {
                                    if k >= lengths.len() {
                                        return Err(
                                            "deflate code-length 17 overruns the tables".into()
                                        );
                                    }
                                    lengths[k] = 0;
                                    k += 1;
                                }
                            }
                            18 => {
                                let n = 11
                                    + self.bits(7).ok_or("deflate code-length 18 ends early")?
                                        as usize;
                                for _ in 0..n {
                                    if k >= lengths.len() {
                                        return Err(
                                            "deflate code-length 18 overruns the tables".into()
                                        );
                                    }
                                    lengths[k] = 0;
                                    k += 1;
                                }
                            }
                            _ => return Err("deflate code-length symbol above 18".into()),
                        }
                    }
                    let lit = canonical(&lengths[..hlit])
                        .ok_or("deflate literal table is not canonical")?;
                    let dist = canonical(&lengths[hlit..])
                        .ok_or("deflate distance table is not canonical")?;
                    self.decode_huff_block(&lit, &dist)?;
                }
                _ => return Err("deflate block header carries btype 3".into()),
            }
            if bfinal == 1 {
                return Ok(());
            }
        }
    }

    fn decode_sym(&mut self, huff: &Huffman) -> Option<u16> {
        let mut code = 0u32;
        for len in 1..=huff.max_len {
            code = (code << 1) | self.bit()?;
            if let Some(&sym) = huff.map.get(&(code, len)) {
                return Some(sym);
            }
        }
        None
    }

    fn decode_huff_block(&mut self, lit: &Huffman, dist: &Huffman) -> Result<(), String> {
        loop {
            let sym = self
                .decode_sym(lit)
                .ok_or("deflate block ends inside a symbol")?;
            if sym < 256 {
                self.push(sym as u8);
            } else if sym == 256 {
                return Ok(());
            } else {
                let li = (sym - 257) as usize;
                if li >= LEN_BASE.len() {
                    return Err("deflate length symbol above 285".into());
                }
                let extra = self
                    .bits(LEN_EXTRA[li])
                    .ok_or("deflate length extra ends early")? as usize;
                let len = LEN_BASE[li] as usize + extra;
                let dcode = self
                    .decode_sym(dist)
                    .ok_or("deflate block ends inside a distance")?
                    as usize;
                if dcode >= DIST_BASE.len() {
                    return Err("deflate distance symbol above 29".into());
                }
                let dextra =
                    self.bits(DIST_EXTRA[dcode])
                        .ok_or("deflate distance extra ends early")? as usize;
                let d = DIST_BASE[dcode] as usize + dextra;
                if d > self.win.len() {
                    return Err("deflate copy reaches before the emitted stream".into());
                }
                for _ in 0..len {
                    let idx = self.win.len() - d;
                    let b = self.win[idx];
                    self.push(b);
                }
            }
        }
    }
}

static CRC_TABLE: [u32; 256] = build_crc_table();

const fn build_crc_table() -> [u32; 256] {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut c = i as u32;
        let mut k = 0;
        while k < 8 {
            if c & 1 != 0 {
                c = 0xEDB8_8320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
            k += 1;
        }
        t[i] = c;
        i += 1;
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_code(buf: &mut Vec<u8>, bit_cursor: &mut usize, value: u32, n: u8) {
        for i in (0..n).rev() {
            let bit = ((value >> i) & 1) as u8;
            let byte = *bit_cursor / 8;
            if byte >= buf.len() {
                buf.push(0);
            }
            if bit == 1 {
                buf[byte] |= 1 << (*bit_cursor % 8);
            }
            *bit_cursor += 1;
        }
    }

    fn push_lsb(buf: &mut Vec<u8>, bit_cursor: &mut usize, value: u32, n: u8) {
        for i in 0..n {
            let bit = ((value >> i) & 1) as u8;
            let byte = *bit_cursor / 8;
            if byte >= buf.len() {
                buf.push(0);
            }
            if bit == 1 {
                buf[byte] |= 1 << (*bit_cursor % 8);
            }
            *bit_cursor += 1;
        }
    }

    #[test]
    fn inflate_stored_block() {
        let stream = [0x01, 0x05, 0x00, 0xFA, 0xFF, b'h', b'e', b'l', b'l', b'o'];
        assert_eq!(inflate(&stream).as_deref(), Some(&b"hello"[..]));
    }

    #[test]
    fn inflate_fixed_huffman_hello() {
        let mut buf = Vec::new();
        let mut cur = 0usize;
        push_lsb(&mut buf, &mut cur, 1, 1);
        push_lsb(&mut buf, &mut cur, 1, 2);
        push_code(&mut buf, &mut cur, 0x98, 8);
        push_code(&mut buf, &mut cur, 0x95, 8);
        push_code(&mut buf, &mut cur, 0x9c, 8);
        push_code(&mut buf, &mut cur, 0x9c, 8);
        push_code(&mut buf, &mut cur, 0x9f, 8);
        push_code(&mut buf, &mut cur, 0x00, 7);
        assert_eq!(inflate(&buf).as_deref(), Some(&b"hello"[..]));
    }

    #[test]
    fn canonical_fixed_code_points() {
        let mut lengths = vec![0u8; 288];
        for l in lengths.iter_mut().take(144) {
            *l = 8;
        }
        for l in lengths.iter_mut().take(256).skip(144) {
            *l = 9;
        }
        for l in lengths.iter_mut().take(280).skip(256) {
            *l = 7;
        }
        for l in lengths.iter_mut().skip(280) {
            *l = 8;
        }
        let h = canonical(&lengths).unwrap();
        assert_eq!(h.map.get(&(0x98, 8)), Some(&104));
        assert_eq!(h.map.get(&(0x00, 7)), Some(&256));
        assert_eq!(h.map.get(&(0xc7, 8)), Some(&287));
    }

    #[test]
    fn unzip_stored_entry() {
        let content = b"omegaflow tns csv";
        let mut zip = Vec::new();
        zip.extend_from_slice(b"PK\x03\x04");
        zip.extend_from_slice(&20u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u32.to_le_bytes());
        zip.extend_from_slice(&(content.len() as u32).to_le_bytes());
        zip.extend_from_slice(&(content.len() as u32).to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(content);
        assert_eq!(unzip(&zip).as_deref(), Some(&content[..]));
    }

    fn gzip_wrap(deflate: &[u8], content: &[u8]) -> Vec<u8> {
        let mut gz = Vec::new();
        gz.extend_from_slice(&[0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 0]);
        gz.extend_from_slice(deflate);
        let mut crc = 0xFFFF_FFFFu32;
        for &b in content {
            crc = (crc >> 8) ^ CRC_TABLE[((crc ^ b as u32) & 0xff) as usize];
        }
        gz.extend_from_slice(&(crc ^ 0xFFFF_FFFF).to_le_bytes());
        gz.extend_from_slice(&(content.len() as u32).to_le_bytes());
        gz
    }

    fn streamed(gz: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let n = gunzip_stream(gz, |chunk| out.extend_from_slice(chunk)).unwrap();
        assert_eq!(n as usize, out.len());
        out
    }

    #[test]
    fn gunzip_stream_stored_block() {
        let deflate = [0x01, 0x05, 0x00, 0xFA, 0xFF, b'h', b'e', b'l', b'l', b'o'];
        let gz = gzip_wrap(&deflate, b"hello");
        assert_eq!(streamed(&gz), b"hello");
    }

    #[test]
    fn gunzip_stream_fixed_huffman_hello() {
        let mut deflate = Vec::new();
        let mut cur = 0usize;
        push_lsb(&mut deflate, &mut cur, 1, 1);
        push_lsb(&mut deflate, &mut cur, 1, 2);
        push_code(&mut deflate, &mut cur, 0x98, 8);
        push_code(&mut deflate, &mut cur, 0x95, 8);
        push_code(&mut deflate, &mut cur, 0x9c, 8);
        push_code(&mut deflate, &mut cur, 0x9c, 8);
        push_code(&mut deflate, &mut cur, 0x9f, 8);
        push_code(&mut deflate, &mut cur, 0x00, 7);
        let gz = gzip_wrap(&deflate, b"hello");
        assert_eq!(streamed(&gz), b"hello");
    }

    #[test]
    fn gunzip_stream_rejects_corrupt_trailer() {
        let mut deflate = Vec::new();
        let mut cur = 0usize;
        push_lsb(&mut deflate, &mut cur, 1, 1);
        push_lsb(&mut deflate, &mut cur, 1, 2);
        push_code(&mut deflate, &mut cur, 0x98, 8);
        push_code(&mut deflate, &mut cur, 0x95, 8);
        push_code(&mut deflate, &mut cur, 0x9c, 8);
        push_code(&mut deflate, &mut cur, 0x9c, 8);
        push_code(&mut deflate, &mut cur, 0x9f, 8);
        push_code(&mut deflate, &mut cur, 0x00, 7);
        let mut gz = gzip_wrap(&deflate, b"hello");
        let last = gz.len() - 1;
        gz[last] ^= 0x01;
        assert!(gunzip_stream(&gz[..], |_| {}).is_err());
    }
}
