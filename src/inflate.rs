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
}
