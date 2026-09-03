const INBUFSIZ: usize = 0x8000;
const INBUF_EXTRA: usize = 64;
const INIT_BITS: usize = 9;
const CLEAR: u32 = 256;
const FIRST: u32 = 257;
const BIT_MASK: u8 = 0x1f;
const BLOCK_MODE: u8 = 0x80;
const CODE_TABLE: usize = 1 << 16;

pub fn uncompress_z(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 3 || data[0] != 0x1f || data[1] != 0x9d {
        return None;
    }
    let flags = data[2];
    let block_mode = flags & BLOCK_MODE != 0;
    let maxbits = (flags & BIT_MASK) as usize;
    if maxbits > 16 {
        return None;
    }
    let maxmaxcode = 1usize << maxbits;

    let mut tab_prefix = vec![0u32; CODE_TABLE];
    let mut tab_suffix = vec![0u8; CODE_TABLE];
    for code in 0..=255u32 {
        tab_suffix[code as usize] = code as u8;
    }

    let mut buf: Vec<u8> = Vec::with_capacity(INBUFSIZ);
    let mut read_pos = 3usize;
    let mut posbits = 0usize;
    let mut rsize = 0usize;
    let mut n_bits = INIT_BITS;
    let mut maxcode = (1usize << n_bits) - 1;
    let mut free_ent = if block_mode { FIRST as usize } else { 256 };
    let mut oldcode: i64 = -1;
    let mut finchar = 0u32;
    let mut out: Vec<u8> = Vec::new();

    loop {
        let o = posbits >> 3;
        if o > 0 {
            buf.drain(..o.min(buf.len()));
            posbits = 0;
        }
        if buf.len() < INBUF_EXTRA {
            let start = read_pos;
            let end = (start + INBUFSIZ).min(data.len());
            buf.extend_from_slice(&data[start..end]);
            read_pos = end;
            rsize = end - start;
        }
        let insize = buf.len();
        let inbits = if rsize != 0 {
            (insize - insize % n_bits) << 3
        } else {
            (insize << 3).saturating_sub(n_bits - 1)
        };
        while inbits > posbits {
            if free_ent > maxcode {
                posbits =
                    (posbits - 1) + ((n_bits << 3) - (posbits - 1 + (n_bits << 3)) % (n_bits << 3));
                n_bits += 1;
                maxcode = if n_bits == maxbits {
                    maxmaxcode
                } else {
                    (1usize << n_bits) - 1
                };
                break;
            }
            let mut code = 0u32;
            for i in 0..n_bits {
                let idx = posbits >> 3;
                let b = if idx < insize { buf[idx] } else { 0 };
                code |= (((b >> (posbits & 7)) & 1) as u32) << i;
                posbits += 1;
            }
            if oldcode == -1 {
                if code >= 256 {
                    return None;
                }
                out.push(code as u8);
                finchar = code;
                oldcode = code as i64;
                continue;
            }
            if code == CLEAR && block_mode {
                free_ent = FIRST as usize - 1;
                posbits =
                    (posbits - 1) + ((n_bits << 3) - (posbits - 1 + (n_bits << 3)) % (n_bits << 3));
                n_bits = INIT_BITS;
                maxcode = (1usize << n_bits) - 1;
                break;
            }
            let incode = code;
            let mut stack: Vec<u8> = Vec::new();
            if code as usize >= free_ent {
                if code as usize > free_ent {
                    return None;
                }
                stack.push(finchar as u8);
                code = oldcode as u32;
            }
            while code >= 256 {
                stack.push(tab_suffix[code as usize]);
                code = tab_prefix[code as usize];
            }
            finchar = code;
            stack.push(code as u8);
            for &ch in stack.iter().rev() {
                out.push(ch);
            }
            if free_ent < maxmaxcode {
                tab_prefix[free_ent] = oldcode as u32;
                tab_suffix[free_ent] = finchar as u8;
                free_ent += 1;
            }
            oldcode = incode as i64;
        }
        if rsize == 0 {
            break;
        }
    }
    Some(out)
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
    fn decodes_plain() {
        let blob = from_hex("1f9d1061c400141890e040");
        assert_eq!(uncompress_z(&blob).unwrap(), b"abababababababab");
    }

    #[test]
    fn decodes_block_mode() {
        let blob = from_hex("1f9d9061c4041c28b02041");
        assert_eq!(uncompress_z(&blob).unwrap(), b"abababababababab");
    }

    #[test]
    fn decodes_phrase() {
        let blob = from_hex("1f9d1074d0940131260c1d1069dc800028104d418501073a44f8906141");
        assert_eq!(
            uncompress_z(&blob).unwrap(),
            b"the cat in the hat the cat in the hat"
        );
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(uncompress_z(b"").is_none());
        assert!(uncompress_z(b"PK\x03\x04").is_none());
        assert_eq!(uncompress_z(b"\x1f\x9d\x10"), Some(Vec::new()));
    }
}
