use crate::json::{JsonVal, jpath_val, json_num, jstr, parse_json};

pub struct ZarrArray {
    pub shape: Vec<usize>,
    pub chunks: Vec<usize>,
    pub dtype: String,
    pub values: Vec<f64>,
}

pub enum Blosc {
    Decompressed(Vec<u8>),
    Unhandled(&'static str),
}

struct Meta {
    shape: Vec<usize>,
    chunks: Vec<usize>,
    dtype: String,
    order: String,
    compressed: bool,
}

enum Dtype {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}

pub fn blosc_codec_name(codec: u8) -> Option<&'static str> {
    match codec {
        0 => Some("blosclz"),
        1 => Some("lz4"),
        2 => Some("snappy"),
        3 => Some("zlib"),
        4 => Some("zstd"),
        _ => None,
    }
}

pub fn chunk_key(index: &[usize]) -> String {
    index
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("/")
}

pub fn chunk_grid(shape: &[usize], chunks: &[usize]) -> Vec<usize> {
    shape
        .iter()
        .zip(chunks.iter())
        .map(|(&s, &c)| s.div_ceil(c))
        .collect()
}

fn u32_at(b: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_le_bytes([
        *b.get(i)?,
        *b.get(i + 1)?,
        *b.get(i + 2)?,
        *b.get(i + 3)?,
    ]))
}

fn trans_bit_8x8(mut x: u64) -> u64 {
    let mut t = (x ^ (x >> 7)) & 0x00AA00AA00AA00AAu64;
    x ^= t ^ (t << 7);
    t = (x ^ (x >> 14)) & 0x0000CCCC0000CCCCu64;
    x ^= t ^ (t << 14);
    t = (x ^ (x >> 28)) & 0x00000000F0F0F0F0u64;
    x ^= t ^ (t << 28);
    x
}

fn trans_byte_bitrow_scal(src: &[u8], dst: &mut [u8], size: usize, elem_size: usize) {
    let nbyte_row = size / 8;
    for jj in 0..elem_size {
        for ii in 0..nbyte_row {
            for kk in 0..8 {
                dst[ii * 8 * elem_size + jj * 8 + kk] = src[(jj * 8 + kk) * nbyte_row + ii];
            }
        }
    }
}

fn shuffle_bit_eightelem_scal(src: &[u8], dst: &mut [u8], size: usize, elem_size: usize) {
    let nbyte = elem_size * size;
    let mut jj = 0;
    while jj < 8 * elem_size {
        let mut ii = 0;
        while ii + 8 * elem_size <= nbyte {
            let mut x = u64::from_le_bytes([
                src[ii + jj],
                src[ii + jj + 1],
                src[ii + jj + 2],
                src[ii + jj + 3],
                src[ii + jj + 4],
                src[ii + jj + 5],
                src[ii + jj + 6],
                src[ii + jj + 7],
            ]);
            x = trans_bit_8x8(x);
            for kk in 0..8 {
                dst[ii + jj / 8 + kk * elem_size] = (x & 0xff) as u8;
                x >>= 8;
            }
            ii += 8 * elem_size;
        }
        jj += 8;
    }
}

fn bshuf_untrans_bit_elem_scal(src: &[u8], dst: &mut [u8], size: usize, elem_size: usize) {
    let nbyte = size * elem_size;
    let mut tmp = vec![0u8; nbyte];
    trans_byte_bitrow_scal(src, &mut tmp, size, elem_size);
    shuffle_bit_eightelem_scal(&tmp, dst, size, elem_size);
}

fn bitunshuffle(typesize: usize, blocksize: usize, src: &[u8], dst: &mut [u8]) {
    let size = blocksize / typesize;
    if size.is_multiple_of(8) {
        bshuf_untrans_bit_elem_scal(src, dst, size, typesize);
        let offset = size * typesize;
        let remaining = blocksize - offset;
        if remaining != 0 {
            dst[offset..].copy_from_slice(&src[offset..]);
        }
    } else {
        dst.copy_from_slice(src);
    }
}

fn unshuffle(typesize: usize, blocksize: usize, src: &[u8], dst: &mut [u8]) {
    let neblock_quot = blocksize / typesize;
    let neblock_rem = blocksize % typesize;
    for i in 0..neblock_quot {
        for j in 0..typesize {
            dst[i * typesize + j] = src[j * neblock_quot + i];
        }
    }
    if neblock_rem != 0 {
        let start = blocksize - neblock_rem;
        dst[start..].copy_from_slice(&src[start..]);
    }
}

fn lz4_decompress(input: &[u8], output_len: usize) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(output_len);
    let mut ip = 0usize;
    let iend = input.len();
    while ip < iend {
        let token = *input.get(ip)?;
        ip += 1;
        let mut lit_len = (token >> 4) as usize;
        if lit_len == 15 {
            loop {
                let s = *input.get(ip)? as usize;
                ip += 1;
                lit_len += s;
                if s != 255 {
                    break;
                }
            }
        }
        let lit_end = ip.checked_add(lit_len)?;
        if lit_end > iend {
            return None;
        }
        out.extend_from_slice(&input[ip..lit_end]);
        ip = lit_end;
        if ip >= iend {
            break;
        }
        let off_lo = *input.get(ip)? as usize;
        let off_hi = *input.get(ip + 1)? as usize;
        ip += 2;
        let offset = off_lo | (off_hi << 8);
        if offset == 0 || offset > out.len() {
            return None;
        }
        let mut match_len = (token & 0x0f) as usize;
        if match_len == 15 {
            loop {
                let s = *input.get(ip)? as usize;
                ip += 1;
                match_len += s;
                if s != 255 {
                    break;
                }
            }
        }
        match_len += 4;
        let mut src = out.len().checked_sub(offset)?;
        for _ in 0..match_len {
            let b = *out.get(src)?;
            out.push(b);
            src += 1;
        }
    }
    if out.len() != output_len {
        return None;
    }
    Some(out)
}

struct BlockDecodeParams {
    block_off: usize,
    bsize: usize,
    flags: u8,
    typesize: usize,
    leftoverblock: bool,
    codec: u8,
}

fn decode_block<F: Fn(&[u8]) -> Option<Vec<u8>>>(
    bytes: &[u8],
    p: BlockDecodeParams,
    block_decoder: &F,
) -> Option<Vec<u8>> {
    let dont_split = (p.flags & 0x10) != 0;
    let doshuffle = (p.flags & 0x01 != 0) && p.typesize > 1;
    let dobitshuffle = (p.flags & 0x04 != 0) && p.bsize >= p.typesize;

    let nsplits =
        if !dont_split && p.typesize <= 16 && (p.bsize / p.typesize) >= 128 && !p.leftoverblock {
            p.typesize
        } else {
            1
        };
    let neblock = p.bsize / nsplits;

    let mut tmp = Vec::with_capacity(p.bsize);
    let mut off = p.block_off;
    for _ in 0..nsplits {
        let cbytes = u32_at(bytes, off)? as usize;
        off += 4;
        let end = off.checked_add(cbytes)?;
        if end > bytes.len() {
            return None;
        }
        let split = &bytes[off..end];
        off = end;
        if cbytes == neblock {
            tmp.extend_from_slice(split);
        } else if p.codec == 1 {
            tmp.extend_from_slice(&lz4_decompress(split, neblock)?);
        } else {
            tmp.extend_from_slice(&block_decoder(split)?);
        }
    }

    if doshuffle {
        let mut dst = vec![0u8; p.bsize];
        unshuffle(p.typesize, p.bsize, &tmp, &mut dst);
        Some(dst)
    } else if dobitshuffle {
        let mut dst = vec![0u8; p.bsize];
        bitunshuffle(p.typesize, p.bsize, &tmp, &mut dst);
        Some(dst)
    } else {
        Some(tmp)
    }
}

pub fn blosc_decompress_with<F: Fn(&[u8]) -> Option<Vec<u8>>>(
    bytes: &[u8],
    block_decoder: F,
) -> Option<Blosc> {
    let version = *bytes.first()?;
    let _versionlz = *bytes.get(1)?;
    let flags = *bytes.get(2)?;
    let typesize = *bytes.get(3)? as usize;
    let nbytes = u32_at(bytes, 4)? as usize;
    let blocksize = u32_at(bytes, 8)? as usize;
    let cbytes = u32_at(bytes, 12)? as usize;

    if version != 2 {
        return None;
    }

    if flags & 0x02 != 0 {
        let end = 16usize.checked_add(nbytes)?;
        if end > bytes.len() || nbytes + 16 != cbytes {
            return None;
        }
        return Some(Blosc::Decompressed(bytes[16..end].to_vec()));
    }

    if typesize == 0 {
        return None;
    }

    let codec = (flags >> 5) & 0x07;
    match codec {
        1 | 4 => {}
        0 | 2 | 3 => return Some(Blosc::Unhandled(blosc_codec_name(codec)?)),
        _ => return None,
    }

    let nblocks = if blocksize == 0 {
        return None;
    } else {
        nbytes.div_ceil(blocksize)
    };
    if nblocks == 0 {
        return Some(Blosc::Decompressed(Vec::new()));
    }

    let mut out = vec![0u8; nbytes];
    let leftover = nbytes % blocksize;
    for j in 0..nblocks {
        let block_off = u32_at(bytes, 16 + 4 * j)? as usize;
        let is_last = j == nblocks - 1;
        let bsize = if is_last && leftover != 0 {
            leftover
        } else {
            blocksize
        };
        let leftoverblock = is_last && leftover != 0;
        let block = decode_block(
            bytes,
            BlockDecodeParams {
                block_off,
                bsize,
                flags,
                typesize,
                leftoverblock,
                codec,
            },
            &block_decoder,
        )?;
        if block.len() != bsize {
            return None;
        }
        out[j * blocksize..j * blocksize + bsize].copy_from_slice(&block);
    }

    Some(Blosc::Decompressed(out))
}

pub fn blosc_decompress(bytes: &[u8]) -> Option<Blosc> {
    let codec = (*bytes.get(2)? >> 5) & 0x07;
    if codec == 4 {
        return Some(Blosc::Unhandled(blosc_codec_name(codec)?));
    }
    blosc_decompress_with(bytes, |_| None)
}

fn dtype_parse(dtype: &str) -> Option<Dtype> {
    let b = dtype.as_bytes();
    if b.len() < 2 {
        return None;
    }
    let endian = b[0];
    let kind = b[1];
    let size: usize = std::str::from_utf8(&b[2..]).ok()?.parse().ok()?;
    match (endian, kind, size) {
        (b'|', b'b', 1) => Some(Dtype::Bool),
        (b'|', b'i', 1) => Some(Dtype::I8),
        (b'|', b'u', 1) => Some(Dtype::U8),
        (b'<', b'i', 2) => Some(Dtype::I16),
        (b'<', b'u', 2) => Some(Dtype::U16),
        (b'<', b'i', 4) => Some(Dtype::I32),
        (b'<', b'u', 4) => Some(Dtype::U32),
        (b'<', b'i', 8) => Some(Dtype::I64),
        (b'<', b'u', 8) => Some(Dtype::U64),
        (b'<', b'f', 4) => Some(Dtype::F32),
        (b'<', b'f', 8) => Some(Dtype::F64),
        _ => None,
    }
}

fn dtype_size(dt: &Dtype) -> usize {
    match dt {
        Dtype::Bool | Dtype::I8 | Dtype::U8 => 1,
        Dtype::I16 | Dtype::U16 => 2,
        Dtype::I32 | Dtype::U32 | Dtype::F32 => 4,
        Dtype::I64 | Dtype::U64 | Dtype::F64 => 8,
    }
}

fn dtype_read(dt: &Dtype, b: &[u8]) -> f64 {
    match dt {
        Dtype::Bool => {
            if b[0] == 0 {
                0.0
            } else {
                1.0
            }
        }
        Dtype::I8 => b[0] as i8 as f64,
        Dtype::U8 => b[0] as f64,
        Dtype::I16 => i16::from_le_bytes([b[0], b[1]]) as f64,
        Dtype::U16 => u16::from_le_bytes([b[0], b[1]]) as f64,
        Dtype::I32 => i32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f64,
        Dtype::U32 => u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f64,
        Dtype::I64 => i64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]) as f64,
        Dtype::U64 => u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]) as f64,
        Dtype::F32 => f32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f64,
        Dtype::F64 => f64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
    }
}

fn decode_dtype(dtype: &str, data: &[u8]) -> Option<Vec<f64>> {
    let dt = dtype_parse(dtype)?;
    let sz = dtype_size(&dt);
    let n = data.len() / sz;
    if n * sz != data.len() {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(dtype_read(&dt, &data[i * sz..(i + 1) * sz]));
    }
    Some(out)
}

fn json_usize_array(v: &JsonVal) -> Option<Vec<usize>> {
    match v {
        JsonVal::Arr(arr) => arr
            .iter()
            .map(|e| {
                let n = json_num(e)?;
                if n < 0.0 || n.fract() != 0.0 {
                    None
                } else {
                    Some(n as usize)
                }
            })
            .collect(),
        _ => None,
    }
}

fn flat_c(coord: &[usize], shape: &[usize]) -> usize {
    let mut idx = 0usize;
    for (&c, &s) in coord.iter().zip(shape.iter()) {
        idx = idx * s + c;
    }
    idx
}

fn flat_f(coord: &[usize], shape: &[usize]) -> usize {
    let mut idx = 0usize;
    for (&c, &s) in coord.iter().zip(shape.iter()).rev() {
        idx = idx * s + c;
    }
    idx
}

fn place_chunk(
    buf: &mut [Option<f64>],
    shape: &[usize],
    chunks: &[usize],
    corner: &[usize],
    elems: &[f64],
    order: &str,
) -> Option<()> {
    let ndim = shape.len();
    let mut local_shape = Vec::with_capacity(ndim);
    for d in 0..ndim {
        let hi = (corner[d] + chunks[d]).min(shape[d]);
        if hi <= corner[d] {
            return None;
        }
        local_shape.push(hi - corner[d]);
    }
    let local_total: usize = local_shape.iter().product();
    if local_total != elems.len() {
        return None;
    }
    let mut local_idx = vec![0usize; ndim];
    let mut abs = vec![0usize; ndim];
    for e in elems {
        for d in 0..ndim {
            abs[d] = corner[d] + local_idx[d];
        }
        let flat = if order == "F" {
            flat_f(&abs, shape)
        } else {
            flat_c(&abs, shape)
        };
        *buf.get_mut(flat)? = Some(*e);
        for d in (0..ndim).rev() {
            local_idx[d] += 1;
            if local_idx[d] < local_shape[d] {
                break;
            }
            local_idx[d] = 0;
        }
    }
    Some(())
}

fn parse_meta(meta_json: &str) -> Option<Meta> {
    let meta = parse_json(meta_json)?;
    let shape = json_usize_array(jpath_val(&meta, "shape")?)?;
    let chunks = json_usize_array(jpath_val(&meta, "chunks")?)?;
    let dtype = jstr(&meta, "dtype")?;
    let order = match jstr(&meta, "order") {
        Some(o) => o,
        None => "C".to_string(),
    };

    let compressed = match jpath_val(&meta, "compressor") {
        None | Some(JsonVal::Null) => false,
        Some(JsonVal::Obj(_)) => match jstr(&meta, "compressor.id") {
            Some(id) if id == "blosc" => true,
            _ => return None,
        },
        Some(_) => return None,
    };

    Some(Meta {
        shape,
        chunks,
        dtype,
        order,
        compressed,
    })
}

pub fn parse_zarr(meta_json: &str, chunks: &[(Vec<usize>, Vec<u8>)]) -> Option<ZarrArray> {
    let meta = parse_meta(meta_json)?;
    let total: usize = meta.shape.iter().product();
    let mut buf: Vec<Option<f64>> = (0..total).map(|_| None).collect();

    for (chunk_index, chunk_bytes) in chunks {
        let raw = if meta.compressed {
            match blosc_decompress(chunk_bytes)? {
                Blosc::Decompressed(v) => v,
                Blosc::Unhandled(_) => return None,
            }
        } else {
            chunk_bytes.clone()
        };
        let elems = decode_dtype(&meta.dtype, &raw)?;
        let corner: Vec<usize> = chunk_index
            .iter()
            .zip(meta.chunks.iter())
            .map(|(&i, &c)| i * c)
            .collect();
        place_chunk(
            &mut buf,
            &meta.shape,
            &meta.chunks,
            &corner,
            &elems,
            &meta.order,
        )?;
    }

    let values: Vec<f64> = buf.into_iter().collect::<Option<Vec<_>>>()?;
    Some(ZarrArray {
        shape: meta.shape,
        chunks: meta.chunks,
        dtype: meta.dtype,
        values,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Vec<u8> {
        let mut out = Vec::with_capacity(s.len() / 2);
        let b = s.as_bytes();
        let mut i = 0;
        while i + 1 < b.len() {
            let hi = (b[i] as char).to_digit(16).unwrap() as u8;
            let lo = (b[i + 1] as char).to_digit(16).unwrap() as u8;
            out.push((hi << 4) | lo);
            i += 2;
        }
        out
    }

    const FIXTURE_SHUFFLE: &str = concat!(
        "02013104a00f000000010000b405000050000000a40000005101000062030000",
        "fe010000f800000057020000a5010000b6030000b0020000090300005b050000",
        "02050000a90400000f0400006804000050000000ff3600010203040506070809",
        "0a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20212223242526272829",
        "2a2b2c2d2e2f303132333435363738393a3b3c3d3e3f00000000000500a35000",
        "0000000050000000ff36404142434445464748494a4b4c4d4e4f505152535455",
        "565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475",
        "767778797a7b7c7d7e7f00000000000500a350000000000055000000ff364041",
        "42434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f6061",
        "62636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f0101",
        "0101010500281f0001006750000000000050000000ff36808182838485868788",
        "898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8",
        "a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf00000000000500a350",
        "000000000055000000ff36c0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4",
        "d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4",
        "f5f6f7f8f9fafbfcfdfeff01010101010500281f000100675000000000005500",
        "0000ff36000102030405060708090a0b0c0d0e0f101112131415161718191a1b",
        "1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b",
        "3c3d3e3f01010101010500281f0001006750000000000055000000ff36808182",
        "838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2",
        "a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf010101",
        "01010500281f0001006750000000000055000000ff3640414243444546474849",
        "4a4b4c4d4e4f505152535455565758595a5b5c5d5e5f60616263646566676869",
        "6a6b6c6d6e6f707172737475767778797a7b7c7d7e7f02020202020500281f00",
        "01006750000000000055000000ff36808182838485868788898a8b8c8d8e8f90",
        "9192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0",
        "b1b2b3b4b5b6b7b8b9babbbcbdbebf02020202020500281f0001006750000000",
        "000050000000ff36c0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7",
        "d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7",
        "f8f9fafbfcfdfeff00000000000500a350000000000055000000ff3600010203",
        "0405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20212223",
        "2425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f02020202",
        "020500281f0001006750000000000055000000ff36808182838485868788898a",
        "8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aa",
        "abacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf03030303030500281f0001",
        "00675000000000003d000000ff1ec0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1",
        "d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e703030303030500101f00",
        "01003750000000000055000000ff36404142434445464748494a4b4c4d4e4f50",
        "5152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f70",
        "7172737475767778797a7b7c7d7e7f03030303030500281f0001006750000000",
        "000055000000ff36000102030405060708090a0b0c0d0e0f1011121314151617",
        "18191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f3031323334353637",
        "38393a3b3c3d3e3f03030303030500281f0001006750000000000055000000ff",
        "36c0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcddde",
        "dfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfe",
        "ff02020202020500281f00010067500000000000"
    );

    const FIXTURE_BITSHUFFLE: &str = concat!(
        "02013404a00f00000001000096030000500000007b000000d6000000a8000000",
        "7a0100000d01000043010000b1010000df01000060030000860200004f020000",
        "1802000027030000c0020000f80200002700000013aa010013cc010013f00100",
        "2300ff02002300ff0400400000ffff0c000f0200b65000000000002900000013",
        "aa010013cc010013f001002300ff02002300ff0400370000ff01000010000f02",
        "00ac5000000000002a00000013aa010013cc010013f001002300ff02002300ff",
        "04003f0000ff0100000018000f0200a45000000000003300000013aa010013cc",
        "010013f001002300ff02002300ff0400400000ffff0c00020200000c00000200",
        "020e000f0200a25000000000003200000013aa010013cc010013f001002300ff",
        "02002300ff0400370000ff0100001000000200041300000c000f02009c500000",
        "0000003300000013aa010013cc010013f001002300ff02002300ff0400400000",
        "ffff0c00020200000c000802000216000f02009a5000000000003300000013aa",
        "010013cc010013f001002300ff02002300ff0400400000ffff0c000a02000014",
        "000002000a16000f0200925000000000002a00000013aa010013cc010013f001",
        "002300ff02002300ff04003f0000ff0100080020000f02009c50000000000035",
        "00000013aa010013cc010013f001002300ff02002300ff0400400000ffff0c00",
        "0f020003001c000002000f1e00030f0200825000000000003300000013aa0100",
        "13cc010013f001002300ff02002300ff0400400000ffff0c000a020000140008",
        "02000a1e000f02008a5000000000003300000013aa010013cc010013f0010023",
        "00ff02002300ff04003f0000ff010000001800000200041b00000c000f020094",
        "5000000000003600000013aa010013cc010013f001002300ff02002300ff0400",
        "400000ffff0c00020200000c00000200020e000e10000f020090500000000000",
        "3400000013aa010013cc010013f001002300ff02002300ff0400400000ffff0c",
        "00020200000c000f020001021e000f0200925000000000002b00000010aa0100",
        "10cc010010f001008000ff00ff000000ff05000007000f0200000019000f0200",
        "525000000000003500000013aa010013cc010013f001002300ff02002300ff04",
        "00370000ff01000010000002000713000102000014000f020094500000000000",
        "3200000013aa010013cc010013f001002300ff02002300ff0400370000ff0100",
        "001000080200041b000814000f02008c500000000000"
    );

    fn fixture_values() -> Vec<f64> {
        (0..1000i32).map(|i| i as f64).collect()
    }

    #[test]
    fn blosc_decodes_lz4_known_sequence() {
        let chunk = [
            2u8, 1, 0x30, 1, 0x80, 0, 0, 0, 0x80, 0, 0, 0, 0x1d, 0, 0, 0, 0x14, 0, 0, 0, 0x05, 0,
            0, 0, 0x1f, 0x61, 0x01, 0x00, 0x6c,
        ];
        match blosc_decompress(&chunk) {
            Some(Blosc::Decompressed(out)) => {
                assert_eq!(out.len(), 128);
                assert!(out.iter().all(|&b| b == 0x61));
            }
            _ => panic!("the chunk carries no decompressed blosc"),
        }
    }

    #[test]
    fn blosc_decodes_memcpyed_raw() {
        let chunk = [
            2u8, 1, 0x02, 1, 0x04, 0, 0, 0, 0x04, 0, 0, 0, 0x14, 0, 0, 0, 1, 2, 3, 4,
        ];
        match blosc_decompress(&chunk) {
            Some(Blosc::Decompressed(out)) => assert_eq!(out, vec![1, 2, 3, 4]),
            _ => panic!("the chunk carries no decompressed blosc"),
        }
    }

    #[test]
    fn blosc_names_unhandled_codecs() {
        let zlib = [2u8, 1, 0x60, 1, 0x04, 0, 0, 0, 0x04, 0, 0, 0, 0x14, 0, 0, 0];
        match blosc_decompress(&zlib) {
            Some(Blosc::Unhandled(name)) => assert_eq!(name, "zlib"),
            _ => panic!("the codec carries no unhandled variant"),
        }
        let blosclz = [2u8, 1, 0x00, 1, 0x04, 0, 0, 0, 0x04, 0, 0, 0, 0x14, 0, 0, 0];
        match blosc_decompress(&blosclz) {
            Some(Blosc::Unhandled(name)) => assert_eq!(name, "blosclz"),
            _ => panic!("the codec carries no unhandled variant"),
        }
        let future = [2u8, 1, 0xA0, 1, 0x04, 0, 0, 0, 0x04, 0, 0, 0, 0x14, 0, 0, 0];
        assert!(blosc_decompress(&future).is_none());
    }

    #[test]
    fn blosc_decodes_byte_shuffle_fixture() {
        let bytes = hex(FIXTURE_SHUFFLE);
        match blosc_decompress(&bytes) {
            Some(Blosc::Decompressed(out)) => {
                assert_eq!(out.len(), 4000);
                let vals = decode_dtype("<i4", &out).expect("dtype");
                assert_eq!(vals, fixture_values());
            }
            _ => panic!("the chunk carries no decompressed blosc"),
        }
    }

    #[test]
    fn blosc_decodes_bitshuffle_fixture() {
        let bytes = hex(FIXTURE_BITSHUFFLE);
        match blosc_decompress(&bytes) {
            Some(Blosc::Decompressed(out)) => {
                assert_eq!(out.len(), 4000);
                let vals = decode_dtype("<i4", &out).expect("dtype");
                assert_eq!(vals, fixture_values());
            }
            _ => panic!("the chunk carries no decompressed blosc"),
        }
    }

    #[test]
    fn parse_zarr_assembles_array() {
        let meta = r#"{"zarr_format":2,"shape":[1000],"chunks":[1000],"dtype":"<i4","compressor":{"id":"blosc","cname":"lz4","clevel":5,"shuffle":1},"fill_value":0,"order":"C"}"#;
        let arr = parse_zarr(meta, &[(vec![0], hex(FIXTURE_SHUFFLE))]).expect("array");
        assert_eq!(arr.shape, vec![1000]);
        assert_eq!(arr.dtype, "<i4");
        assert_eq!(arr.values, fixture_values());
    }

    #[test]
    fn parse_zarr_reads_raw_uncompressed_array() {
        let meta = r#"{"shape":[2,3],"chunks":[2,3],"dtype":"<f8","compressor":null,"order":"C"}"#;
        let raw = [1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<u8>>();
        let arr = parse_zarr(meta, &[(vec![0, 0], raw)]).expect("array");
        assert_eq!(arr.values, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn chunk_keys_and_grid() {
        assert_eq!(chunk_key(&[0, 0, 0]), "0/0/0");
        assert_eq!(chunk_key(&[3, 1, 2]), "3/1/2");
        assert_eq!(chunk_grid(&[1000], &[256]), vec![4]);
        assert_eq!(chunk_grid(&[10, 10], &[6, 4]), vec![2, 3]);
    }

    #[test]
    fn dtype_parses_little_endian_kinds() {
        assert_eq!(decode_dtype("<i4", &[0, 0, 0, 0]).unwrap(), vec![0.0]);
        assert_eq!(
            decode_dtype("<i4", &[1, 0, 0, 0, 0xff, 0xff, 0xff, 0xff]).unwrap(),
            vec![1.0, -1.0]
        );
        assert_eq!(
            decode_dtype("<f8", &1.5f64.to_le_bytes()).unwrap(),
            vec![1.5]
        );
        assert!(decode_dtype(">i4", &[0, 0, 0, 0]).is_none());
        assert!(decode_dtype("|S3", &[0, 0, 0]).is_none());
    }
}
