#[derive(Debug, Clone)]
pub struct GeoTransform {
    pub x0: f64,
    pub y0: f64,
    pub dx: f64,
    pub dy: f64,
}

#[derive(Debug, Clone)]
pub struct TiffImage {
    pub width: u32,
    pub height: u32,
    pub bits_per_sample: Vec<u16>,
    pub samples_per_pixel: u16,
    pub compression: u16,
    pub photometric: Option<u16>,
    pub pixels: Vec<u8>,
    pub geo: Option<GeoTransform>,
    pub geo_keys: Option<Vec<u16>>,
}

#[derive(Clone, Copy)]
struct Entry {
    tag: u16,
    field_type: u16,
    count: u32,
    value_pos: usize,
}

fn u16_at(b: &[u8], off: usize, little: bool) -> Option<u16> {
    let v: [u8; 2] = b.get(off..off + 2)?.try_into().ok()?;
    Some(if little {
        u16::from_le_bytes(v)
    } else {
        u16::from_be_bytes(v)
    })
}

fn u32_at(b: &[u8], off: usize, little: bool) -> Option<u32> {
    let v: [u8; 4] = b.get(off..off + 4)?.try_into().ok()?;
    Some(if little {
        u32::from_le_bytes(v)
    } else {
        u32::from_be_bytes(v)
    })
}

fn f64_at(b: &[u8], off: usize, little: bool) -> Option<f64> {
    let v: [u8; 8] = b.get(off..off + 8)?.try_into().ok()?;
    Some(if little {
        f64::from_le_bytes(v)
    } else {
        f64::from_be_bytes(v)
    })
}

fn type_size(t: u16) -> Option<usize> {
    Some(match t {
        1 | 2 | 6 | 7 => 1,
        3 | 8 => 2,
        4 | 9 | 11 => 4,
        5 | 10 | 12 => 8,
        _ => return None,
    })
}

fn entry_base(data: &[u8], e: &Entry, little: bool, size: usize) -> Option<usize> {
    let byte_len = (e.count as usize).checked_mul(size)?;
    if byte_len <= 4 {
        Some(e.value_pos)
    } else {
        u32_at(data, e.value_pos, little).map(|v| v as usize)
    }
}

fn entry_scalar(data: &[u8], e: &Entry, little: bool) -> Option<u32> {
    let base = entry_base(data, e, little, type_size(e.field_type)?)?;
    match e.field_type {
        3 => u16_at(data, base, little).map(u32::from),
        4 => u32_at(data, base, little),
        _ => None,
    }
}

fn entry_u16_vec(data: &[u8], e: &Entry, little: bool) -> Option<Vec<u16>> {
    let base = entry_base(data, e, little, 2)?;
    let mut out = Vec::with_capacity(e.count as usize);
    for i in 0..e.count as usize {
        out.push(u16_at(data, base + i * 2, little)?);
    }
    Some(out)
}

fn entry_u32_vec(data: &[u8], e: &Entry, little: bool) -> Option<Vec<u32>> {
    let base = entry_base(data, e, little, 4)?;
    let mut out = Vec::with_capacity(e.count as usize);
    for i in 0..e.count as usize {
        out.push(u32_at(data, base + i * 4, little)?);
    }
    Some(out)
}

fn entry_offsets(data: &[u8], e: &Entry, little: bool) -> Option<Vec<u32>> {
    match e.field_type {
        3 => entry_u16_vec(data, e, little).map(|v| v.into_iter().map(u32::from).collect()),
        4 => entry_u32_vec(data, e, little),
        _ => None,
    }
}

fn entry_double_vec(data: &[u8], e: &Entry, little: bool) -> Option<Vec<f64>> {
    let base = entry_base(data, e, little, 8)?;
    let mut out = Vec::with_capacity(e.count as usize);
    for i in 0..e.count as usize {
        out.push(f64_at(data, base + i * 8, little)?);
    }
    Some(out)
}

fn entry_raw(data: &[u8], e: &Entry, little: bool) -> Option<Vec<u8>> {
    let base = entry_base(data, e, little, 1)?;
    data.get(base..base + e.count as usize).map(|s| s.to_vec())
}

fn geotransform(
    pixel_scale: Option<Vec<f64>>,
    tiepoint: Option<Vec<f64>>,
    transformation: Option<Vec<f64>>,
) -> Option<GeoTransform> {
    if let Some(m) = transformation
        && m.len() >= 8
    {
        return Some(GeoTransform {
            x0: m[3],
            y0: m[7],
            dx: m[0],
            dy: m[5],
        });
    }
    if let (Some(scale), Some(tie)) = (pixel_scale, tiepoint)
        && scale.len() >= 2
        && tie.len() >= 6
    {
        let sx = scale[0];
        let sy = scale[1];
        let i0 = tie[0];
        let j0 = tie[1];
        return Some(GeoTransform {
            x0: tie[3] - i0 * sx,
            y0: tie[4] + j0 * sy,
            dx: sx,
            dy: -sy,
        });
    }
    None
}

fn raw_strip(strip: &[u8], need: usize) -> Option<Vec<u8>> {
    if strip.len() < need {
        return None;
    }
    Some(strip[..need].to_vec())
}

struct StripDecodeParams<'a> {
    width: u32,
    height: u32,
    bits_per_sample: &'a [u16],
    rows_per_strip: u32,
    strip_offsets: &'a [u32],
    strip_byte_counts: &'a [u32],
}

fn decode_strips(
    data: &[u8],
    p: StripDecodeParams<'_>,
    strip_decode: fn(&[u8], usize) -> Option<Vec<u8>>,
) -> Option<Vec<u8>> {
    let mut bytes_per_pixel = 0usize;
    for &b in p.bits_per_sample {
        if b % 8 != 0 {
            return None;
        }
        bytes_per_pixel += (b / 8) as usize;
    }
    if bytes_per_pixel == 0 {
        return None;
    }
    let row_bytes = (p.width as usize).checked_mul(bytes_per_pixel)?;
    let total = row_bytes.checked_mul(p.height as usize)?;
    let mut out = Vec::with_capacity(total);
    let mut row = 0usize;
    for (i, &off) in p.strip_offsets.iter().enumerate() {
        if row >= p.height as usize {
            break;
        }
        let count = *p.strip_byte_counts.get(i)? as usize;
        let start = off as usize;
        let end = start.checked_add(count)?;
        let strip = data.get(start..end)?;
        let remaining = p.height as usize - row;
        let rows = (p.rows_per_strip as usize).min(remaining);
        let need = rows.checked_mul(row_bytes)?;
        let decoded = strip_decode(strip, need)?;
        if decoded.len() != need {
            return None;
        }
        out.extend_from_slice(&decoded);
        row += rows;
    }
    if out.len() != total {
        return None;
    }
    Some(out)
}

const LZW_CLEAR: u16 = 256;
const LZW_EOI: u16 = 257;
const LZW_FIRST: u16 = 258;
const LZW_TABLE_MAX: usize = 4096;

fn read_lzw_code(data: &[u8], bit_pos: &mut usize, width: u32) -> Option<u16> {
    let mut code = 0u32;
    for _ in 0..width {
        let byte = *data.get(*bit_pos >> 3)?;
        let bit = (byte >> (7 - (*bit_pos & 7))) & 1;
        code = (code << 1) | (bit as u32);
        *bit_pos += 1;
    }
    Some(code as u16)
}

fn decode_lzw_strip(data: &[u8], expected: usize) -> Option<Vec<u8>> {
    let mut prefix = vec![0u16; LZW_TABLE_MAX];
    let mut suffix = vec![0u8; LZW_TABLE_MAX];
    for i in 0..256u16 {
        suffix[i as usize] = i as u8;
    }
    let mut out = Vec::with_capacity(expected);
    let mut bit_pos = 0usize;
    let mut width = 9u32;
    let mut free = LZW_FIRST;
    let mut prev: Option<u16> = None;
    let mut prev_str: Vec<u8> = Vec::new();

    loop {
        let code = read_lzw_code(data, &mut bit_pos, width)?;
        if code == LZW_EOI {
            break;
        }
        if code == LZW_CLEAR {
            width = 9;
            free = LZW_FIRST;
            prev = None;
            prev_str.clear();
            continue;
        }
        let entry: Vec<u8> = if code < free {
            let mut chain: Vec<u8> = Vec::new();
            let mut c = code;
            while c >= 256 {
                if (c as usize) >= free as usize {
                    return None;
                }
                chain.push(suffix[c as usize]);
                c = prefix[c as usize];
            }
            chain.push(c as u8);
            chain.reverse();
            chain
        } else if code == free {
            match prev {
                Some(_) => {
                    let mut s = prev_str.clone();
                    let first = *s.first()?;
                    s.push(first);
                    s
                }
                None => return None,
            }
        } else {
            return None;
        };
        out.extend_from_slice(&entry);
        if let Some(p) = prev
            && (free as usize) < LZW_TABLE_MAX
        {
            prefix[free as usize] = p;
            suffix[free as usize] = entry[0];
            free += 1;
            if free == (1u16 << width) - 1 && width < 12 {
                width += 1;
            }
        }
        prev = Some(code);
        prev_str = entry;
    }
    if out.len() != expected {
        return None;
    }
    Some(out)
}

const ZIGZAG: [usize; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59,
    52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];

#[derive(Clone)]
struct HuffTable {
    min_code: [i32; 16],
    max_code: [i32; 16],
    val_ptr: [usize; 16],
    values: Vec<u8>,
}

#[derive(Clone, Copy)]
struct JpegComponent {
    id: u8,
    h: u8,
    v: u8,
    qt_id: u8,
}

struct JpegFrame {
    width: u16,
    height: u16,
    components: Vec<JpegComponent>,
}

struct JpegImage {
    width: usize,
    height: usize,
    components: usize,
    pixels: Vec<u8>,
}

struct JpegDecoder {
    quant: [Option<[u16; 64]>; 4],
    dc: [Option<HuffTable>; 4],
    ac: [Option<HuffTable>; 4],
    frame: Option<JpegFrame>,
    scan: Vec<(u8, u8, u8)>,
    restart_interval: u16,
}

fn be16(b: &[u8], off: usize) -> Option<u16> {
    let v: [u8; 2] = b.get(off..off + 2)?.try_into().ok()?;
    Some(u16::from_be_bytes(v))
}

fn build_huffman(counts: &[u8; 16], values: &[u8]) -> Option<HuffTable> {
    let total: usize = counts.iter().map(|&c| c as usize).sum();
    if total != values.len() {
        return None;
    }
    let mut min_code = [0i32; 16];
    let mut max_code = [0i32; 16];
    let mut val_ptr = [0usize; 16];
    let mut code = 0i32;
    let mut ptr = 0usize;
    for i in 0..16 {
        min_code[i] = code;
        val_ptr[i] = ptr;
        code += counts[i] as i32;
        max_code[i] = code - 1;
        ptr += counts[i] as usize;
        code <<= 1;
    }
    Some(HuffTable {
        min_code,
        max_code,
        val_ptr,
        values: values.to_vec(),
    })
}

struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    cur: u8,
    bitpos: u8,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        BitReader {
            data,
            pos: 0,
            cur: 0,
            bitpos: 8,
        }
    }

    fn next_byte(&mut self) -> Option<u8> {
        let b = *self.data.get(self.pos)?;
        self.pos += 1;
        if b == 0xFF {
            let n = *self.data.get(self.pos)?;
            if n == 0x00 {
                self.pos += 1;
                return Some(0xFF);
            }
            self.pos -= 1;
            return None;
        }
        Some(b)
    }

    fn read_bit(&mut self) -> Option<u32> {
        if self.bitpos == 8 {
            self.cur = self.next_byte()?;
            self.bitpos = 0;
        }
        let bit = (self.cur >> (7 - self.bitpos)) & 1;
        self.bitpos += 1;
        Some(bit as u32)
    }

    fn read_bits(&mut self, n: u32) -> Option<u32> {
        let mut v = 0u32;
        for _ in 0..n {
            v = (v << 1) | self.read_bit()?;
        }
        Some(v)
    }

    fn align_byte(&mut self) {
        self.bitpos = 8;
    }

    fn read_raw(&mut self) -> Option<u8> {
        let b = *self.data.get(self.pos)?;
        self.pos += 1;
        Some(b)
    }
}

fn huff_decode(bits: &mut BitReader, table: &HuffTable) -> Option<u8> {
    let mut code = 0i32;
    for i in 0..16 {
        let b = bits.read_bit()? as i32;
        code = (code << 1) | b;
        if code <= table.max_code[i] {
            let idx = table.val_ptr[i] + (code - table.min_code[i]) as usize;
            return table.values.get(idx).copied();
        }
    }
    None
}

fn receive_extend(bits: &mut BitReader, s: u32) -> Option<i32> {
    if s == 0 {
        return Some(0);
    }
    let v = bits.read_bits(s)? as i32;
    let half = 1i32 << (s - 1);
    Some(if v < half { v - (1 << s) + 1 } else { v })
}

fn idct1d(p: &mut [f64]) {
    let mut tmp = [0.0f64; 8];
    for (x, slot) in tmp.iter_mut().enumerate() {
        let mut sum = 0.0;
        for (u, &pu) in p.iter().enumerate() {
            let cu = if u == 0 {
                std::f64::consts::FRAC_1_SQRT_2
            } else {
                1.0
            };
            let angle = (2.0 * x as f64 + 1.0) * u as f64 * std::f64::consts::PI / 16.0;
            sum += cu * pu * angle.cos();
        }
        *slot = sum * 0.5;
    }
    p.copy_from_slice(&tmp);
}

fn idct2d(block: &mut [f64; 64]) {
    for i in 0..8 {
        idct1d(&mut block[i * 8..i * 8 + 8]);
    }
    let mut col = [0.0f64; 8];
    for j in 0..8 {
        for i in 0..8 {
            col[i] = block[i * 8 + j];
        }
        idct1d(&mut col);
        for i in 0..8 {
            block[i * 8 + j] = col[i];
        }
    }
}

fn decode_block(
    bits: &mut BitReader,
    dc_t: &HuffTable,
    ac_t: &HuffTable,
    qt: &[u16; 64],
    pred: &mut i32,
) -> Option<[u8; 64]> {
    let mut zz = [0i32; 64];
    let t = huff_decode(bits, dc_t)? as i32;
    let diff = if t == 0 {
        0
    } else {
        receive_extend(bits, t as u32)?
    };
    *pred += diff;
    zz[0] = *pred;
    let mut k = 1;
    while k < 64 {
        let rs = huff_decode(bits, ac_t)?;
        if rs == 0x00 {
            break;
        }
        let r = (rs >> 4) as usize;
        let s = (rs & 0x0F) as u32;
        if s == 0 {
            if r == 15 {
                k += 16;
                continue;
            }
            return None;
        }
        k += r;
        if k >= 64 {
            return None;
        }
        zz[k] = receive_extend(bits, s)?;
        k += 1;
    }
    let mut natural = [0.0f64; 64];
    for i in 0..64 {
        natural[ZIGZAG[i]] = zz[i] as f64 * qt[i] as f64;
    }
    idct2d(&mut natural);
    let mut out = [0u8; 64];
    for i in 0..64 {
        out[i] = (natural[i] + 128.0).clamp(0.0, 255.0).round() as u8;
    }
    Some(out)
}

impl JpegDecoder {
    fn parse_sof(&mut self, seg: &[u8]) -> Option<()> {
        if *seg.first()? != 8 {
            return None;
        }
        let height = be16(seg, 1)?;
        let width = be16(seg, 3)?;
        let n = *seg.get(5)? as usize;
        let mut components = Vec::with_capacity(n);
        for i in 0..n {
            let off = 6 + i * 3;
            let id = *seg.get(off)?;
            let hv = *seg.get(off + 1)?;
            let qt_id = *seg.get(off + 2)?;
            let h = hv >> 4;
            let v = hv & 0x0F;
            if h == 0 || v == 0 {
                return None;
            }
            components.push(JpegComponent { id, h, v, qt_id });
        }
        self.frame = Some(JpegFrame {
            width,
            height,
            components,
        });
        Some(())
    }

    fn parse_dqt(&mut self, seg: &[u8]) -> Option<()> {
        let mut pos = 0usize;
        while pos < seg.len() {
            let info = *seg.get(pos)?;
            pos += 1;
            let precision = info >> 4;
            let id = (info & 0x0F) as usize;
            if id >= 4 {
                return None;
            }
            let mut table = [0u16; 64];
            if precision == 0 {
                for v in table.iter_mut() {
                    *v = *seg.get(pos)? as u16;
                    pos += 1;
                }
            } else {
                for v in table.iter_mut() {
                    *v = be16(seg, pos)?;
                    pos += 2;
                }
            }
            self.quant[id] = Some(table);
        }
        Some(())
    }

    fn parse_dht(&mut self, seg: &[u8]) -> Option<()> {
        let mut pos = 0usize;
        while pos < seg.len() {
            let info = *seg.get(pos)?;
            pos += 1;
            let class = info >> 4;
            let id = (info & 0x0F) as usize;
            if class > 1 || id >= 4 {
                return None;
            }
            let mut counts = [0u8; 16];
            for c in counts.iter_mut() {
                *c = *seg.get(pos)?;
                pos += 1;
            }
            let total: usize = counts.iter().map(|&c| c as usize).sum();
            let values = seg.get(pos..pos + total)?.to_vec();
            pos += total;
            let table = build_huffman(&counts, &values)?;
            if class == 0 {
                self.dc[id] = Some(table);
            } else {
                self.ac[id] = Some(table);
            }
        }
        Some(())
    }
}

fn parse_sos(dec: &mut JpegDecoder, data: &[u8], pos: usize) -> Option<usize> {
    let len = be16(data, pos)? as usize;
    if len < 2 {
        return None;
    }
    let seg_end = pos + len;
    let mut p = pos + 2;
    let ns = *data.get(p)? as usize;
    p += 1;
    if ns == 0 || ns > 4 {
        return None;
    }
    let mut scan = Vec::with_capacity(ns);
    for _ in 0..ns {
        let id = *data.get(p)?;
        let t = *data.get(p + 1)?;
        p += 2;
        scan.push((id, t >> 4, t & 0x0F));
    }
    let ss = *data.get(p)?;
    let se = *data.get(p + 1)?;
    let ahal = *data.get(p + 2)?;
    if ss != 0 || se != 63 || ahal != 0 {
        return None;
    }
    dec.scan = scan;
    Some(seg_end)
}

struct Plane {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

fn to_interleaved(planes: &[Plane], width: usize, height: usize, ids: &[u8]) -> Option<Vec<u8>> {
    if planes.len() == 1 {
        let p = &planes[0];
        let mut out = vec![0u8; width * height];
        for y in 0..height {
            let sy = y * p.height / height;
            for x in 0..width {
                let sx = x * p.width / width;
                out[y * width + x] = p.data[sy * p.width + sx];
            }
        }
        return Some(out);
    }
    if planes.len() != 3 {
        return None;
    }
    let is_ycbcr = ids == [1, 2, 3].as_slice();
    let mut out = vec![0u8; width * height * 3];
    for y in 0..height {
        for x in 0..width {
            let sample = |ci: usize| -> u8 {
                let p = &planes[ci];
                let sx = x * p.width / width;
                let sy = y * p.height / height;
                p.data[sy * p.width + sx]
            };
            let c0 = sample(0) as f64;
            let c1 = sample(1) as f64;
            let c2 = sample(2) as f64;
            let (r, g, b) = if is_ycbcr {
                (
                    c0 + 1.402 * (c2 - 128.0),
                    c0 - 0.344_136 * (c1 - 128.0) - 0.714_136 * (c2 - 128.0),
                    c0 + 1.772 * (c1 - 128.0),
                )
            } else {
                (c0, c1, c2)
            };
            let clamp = |v: f64| -> u8 {
                if v < 0.0 {
                    0
                } else if v > 255.0 {
                    255
                } else {
                    v.round() as u8
                }
            };
            let o = (y * width + x) * 3;
            out[o] = clamp(r);
            out[o + 1] = clamp(g);
            out[o + 2] = clamp(b);
        }
    }
    Some(out)
}

fn decode_scan(dec: &JpegDecoder, data: &[u8], start: usize) -> Option<JpegImage> {
    let frame = dec.frame.as_ref()?;
    let max_h = frame.components.iter().map(|c| c.h).max()? as usize;
    let max_v = frame.components.iter().map(|c| c.v).max()? as usize;
    let mcus_x = (frame.width as usize).div_ceil(8 * max_h);
    let mcus_y = (frame.height as usize).div_ceil(8 * max_v);

    let mut scan_info: Vec<(usize, JpegComponent, u8, u8)> = Vec::with_capacity(dec.scan.len());
    for &(id, dc_id, ac_id) in &dec.scan {
        let frame_idx = frame.components.iter().position(|c| c.id == id)?;
        let comp = frame.components[frame_idx];
        scan_info.push((frame_idx, comp, dc_id, ac_id));
    }
    let n = scan_info.len();

    let mut planes: Vec<Plane> = frame
        .components
        .iter()
        .map(|c| {
            let w = mcus_x * c.h as usize * 8;
            let h = mcus_y * c.v as usize * 8;
            Plane {
                width: w,
                height: h,
                data: vec![0u8; w * h],
            }
        })
        .collect();

    let mut dc_pred = vec![0i32; n];
    let mut bits = BitReader::new(&data[start..]);
    let mut mcu_count = 0usize;

    for mcu_y in 0..mcus_y {
        for mcu_x in 0..mcus_x {
            if dec.restart_interval > 0
                && mcu_count > 0
                && mcu_count.is_multiple_of(dec.restart_interval as usize)
            {
                bits.align_byte();
                if bits.read_raw()? != 0xFF {
                    return None;
                }
                let m = bits.read_raw()?;
                if !(0xD0..=0xD7).contains(&m) {
                    return None;
                }
                for p in dc_pred.iter_mut() {
                    *p = 0;
                }
            }
            mcu_count += 1;
            for si in 0..n {
                let (frame_idx, comp, dc_id, ac_id) = scan_info[si];
                let dc_t = dec.dc[dc_id as usize].as_ref()?;
                let ac_t = dec.ac[ac_id as usize].as_ref()?;
                let qt = dec.quant[comp.qt_id as usize].as_ref()?;
                for by in 0..comp.v as usize {
                    for bx in 0..comp.h as usize {
                        let block = decode_block(&mut bits, dc_t, ac_t, qt, &mut dc_pred[si])?;
                        let gx = mcu_x * comp.h as usize + bx;
                        let gy = mcu_y * comp.v as usize + by;
                        let plane = &mut planes[frame_idx];
                        for i in 0..8 {
                            let src = i * 8;
                            let dst = (gy * 8 + i) * plane.width + gx * 8;
                            plane.data[dst..dst + 8].copy_from_slice(&block[src..src + 8]);
                        }
                    }
                }
            }
        }
    }

    let width = frame.width as usize;
    let height = frame.height as usize;
    let comp_ids: Vec<u8> = frame.components.iter().map(|c| c.id).collect();
    let pixels = to_interleaved(&planes, width, height, &comp_ids)?;
    Some(JpegImage {
        width,
        height,
        components: planes.len(),
        pixels,
    })
}

fn decode_jpeg(data: &[u8]) -> Option<JpegImage> {
    let mut dec = JpegDecoder {
        quant: [None, None, None, None],
        dc: [None, None, None, None],
        ac: [None, None, None, None],
        frame: None,
        scan: Vec::new(),
        restart_interval: 0,
    };
    let mut pos = 0usize;
    loop {
        if *data.get(pos)? != 0xFF {
            return None;
        }
        while *data.get(pos)? == 0xFF {
            pos += 1;
        }
        let marker = *data.get(pos)?;
        pos += 1;
        match marker {
            0xD8 | 0xD9 => continue,
            0xDA => {
                let scan_start = parse_sos(&mut dec, data, pos)?;
                return decode_scan(&dec, data, scan_start);
            }
            _ if (0xD0..=0xD7).contains(&marker) => continue,
            _ => {
                let len = be16(data, pos)? as usize;
                pos += 2;
                if len < 2 {
                    return None;
                }
                let seg_end = pos + len - 2;
                let seg = data.get(pos..seg_end)?;
                match marker {
                    0xDB => dec.parse_dqt(seg)?,
                    0xC4 => dec.parse_dht(seg)?,
                    0xC0 | 0xC1 => dec.parse_sof(seg)?,
                    0xDD if seg.len() >= 2 => {
                        dec.restart_interval = be16(seg, 0)?;
                    }
                    _ => {}
                }
                pos = seg_end;
            }
        }
    }
}

fn decode_jpeg_with_tables(block: &[u8], tables: Option<&[u8]>) -> Option<JpegImage> {
    match tables {
        Some(t) if !t.is_empty() => {
            let mut combined = Vec::with_capacity(t.len() + block.len());
            combined.extend_from_slice(t);
            combined.extend_from_slice(block);
            decode_jpeg(&combined)
        }
        _ => decode_jpeg(block),
    }
}

fn jpeg_bytes_per_pixel(bits_per_sample: &[u16], samples_per_pixel: u16) -> Option<usize> {
    if bits_per_sample.is_empty() {
        return Some(samples_per_pixel as usize);
    }
    let mut bpp = 0usize;
    for &b in bits_per_sample {
        if b != 8 {
            return None;
        }
        bpp += 1;
    }
    if bpp == 0 { None } else { Some(bpp) }
}

struct JpegAssembleParams<'a> {
    width: u32,
    height: u32,
    bpp: usize,
    jpeg_tables: Option<&'a [u8]>,
    offsets: &'a [u32],
    counts: &'a [u32],
    tiled: bool,
    block_w: u32,
    block_h: u32,
}

fn assemble_jpeg(data: &[u8], p: JpegAssembleParams<'_>) -> Option<Vec<u8>> {
    let w = p.width as usize;
    let h = p.height as usize;
    let mut out = vec![0u8; w * h * p.bpp];
    let blocks_across = if p.tiled {
        w.div_ceil(p.block_w as usize)
    } else {
        1
    };
    let mut row = 0usize;
    for i in 0..p.offsets.len() {
        if row >= h {
            break;
        }
        let off = *p.offsets.get(i)? as usize;
        let cnt = *p.counts.get(i)? as usize;
        let block = data.get(off..off.checked_add(cnt)?)?;
        let img = decode_jpeg_with_tables(block, p.jpeg_tables)?;
        if img.components != p.bpp {
            return None;
        }
        let (bx, by) = if p.tiled {
            let tx = i % blocks_across;
            let ty = i / blocks_across;
            (tx * p.block_w as usize, ty * p.block_h as usize)
        } else {
            (0, row)
        };
        let iw = img.width;
        let ih = img.height;
        if bx >= w || by >= h {
            return None;
        }
        let copy_w = iw.min(w - bx);
        let copy_h = ih.min(h - by);
        for r in 0..copy_h {
            let src = r * iw * p.bpp;
            let dst = (by + r) * w * p.bpp + bx * p.bpp;
            out[dst..dst + copy_w * p.bpp].copy_from_slice(&img.pixels[src..src + copy_w * p.bpp]);
        }
        if !p.tiled {
            row += ih;
        }
    }
    Some(out)
}

pub fn parse_tiff(data: &[u8]) -> Option<TiffImage> {
    let header = data.get(0..8)?;
    let little = match &header[0..2] {
        b"II" => true,
        b"MM" => false,
        _ => return None,
    };
    if u16_at(header, 2, little)? != 42 {
        return None;
    }
    let ifd_offset = u32_at(header, 4, little)? as usize;
    let entry_count = u16_at(data, ifd_offset, little)? as usize;
    let ifd_base = ifd_offset + 2;
    if ifd_base + entry_count * 12 + 4 > data.len() {
        return None;
    }

    let mut entries = Vec::with_capacity(entry_count);
    for i in 0..entry_count {
        let pos = ifd_base + i * 12;
        entries.push(Entry {
            tag: u16_at(data, pos, little)?,
            field_type: u16_at(data, pos + 2, little)?,
            count: u32_at(data, pos + 4, little)?,
            value_pos: pos + 8,
        });
    }

    let mut width = None;
    let mut height = None;
    let mut bits_per_sample = None;
    let mut compression = None;
    let mut photometric = None;
    let mut strip_offsets = None;
    let mut samples_per_pixel = None;
    let mut rows_per_strip = None;
    let mut strip_byte_counts = None;
    let mut planar_configuration = None;
    let mut model_pixel_scale = None;
    let mut model_tiepoint = None;
    let mut model_transformation = None;
    let mut geo_keys = None;
    let mut tile_width = None;
    let mut tile_length = None;
    let mut tile_offsets = None;
    let mut tile_byte_counts = None;
    let mut jpeg_tables = None;

    for e in &entries {
        match e.tag {
            256 => width = entry_scalar(data, e, little),
            257 => height = entry_scalar(data, e, little),
            258 => bits_per_sample = entry_u16_vec(data, e, little),
            259 => compression = entry_scalar(data, e, little).map(|v| v as u16),
            262 => photometric = entry_scalar(data, e, little).map(|v| v as u16),
            273 => strip_offsets = entry_offsets(data, e, little),
            277 => samples_per_pixel = entry_scalar(data, e, little).map(|v| v as u16),
            278 => rows_per_strip = entry_scalar(data, e, little),
            279 => strip_byte_counts = entry_offsets(data, e, little),
            284 => planar_configuration = entry_scalar(data, e, little).map(|v| v as u16),
            322 => tile_width = entry_scalar(data, e, little),
            323 => tile_length = entry_scalar(data, e, little),
            324 => tile_offsets = entry_offsets(data, e, little),
            325 => tile_byte_counts = entry_offsets(data, e, little),
            347 => jpeg_tables = entry_raw(data, e, little),
            33550 => model_pixel_scale = entry_double_vec(data, e, little),
            33922 => model_tiepoint = entry_double_vec(data, e, little),
            34264 => model_transformation = entry_double_vec(data, e, little),
            34735 => geo_keys = entry_u16_vec(data, e, little),
            _ => {}
        }
    }

    let width = width?;
    let height = height?;
    let bits_per_sample = match bits_per_sample {
        Some(v) => v,
        None => vec![1],
    };
    let samples_per_pixel = samples_per_pixel.unwrap_or(1);
    let compression = compression.unwrap_or(1);
    let rows_per_strip = rows_per_strip.unwrap_or(u32::MAX);
    let planar_configuration = planar_configuration.unwrap_or(1);

    let geo = geotransform(model_pixel_scale, model_tiepoint, model_transformation);

    let pixels = if planar_configuration == 1 {
        let offsets: &[u32] = match &strip_offsets {
            Some(v) => v,
            None => &[],
        };
        let counts: &[u32] = match &strip_byte_counts {
            Some(v) => v,
            None => &[],
        };
        let decoded = if compression == 7 {
            match jpeg_bytes_per_pixel(&bits_per_sample, samples_per_pixel) {
                Some(bpp) => {
                    if let (Some(tw), Some(tl), Some(to), Some(tc)) = (
                        tile_width,
                        tile_length,
                        tile_offsets.as_deref(),
                        tile_byte_counts.as_deref(),
                    ) {
                        assemble_jpeg(
                            data,
                            JpegAssembleParams {
                                width,
                                height,
                                bpp,
                                jpeg_tables: jpeg_tables.as_deref(),
                                offsets: to,
                                counts: tc,
                                tiled: true,
                                block_w: tw,
                                block_h: tl,
                            },
                        )
                    } else {
                        assemble_jpeg(
                            data,
                            JpegAssembleParams {
                                width,
                                height,
                                bpp,
                                jpeg_tables: jpeg_tables.as_deref(),
                                offsets,
                                counts,
                                tiled: false,
                                block_w: width,
                                block_h: rows_per_strip,
                            },
                        )
                    }
                }
                None => None,
            }
        } else {
            match compression {
                1 => decode_strips(
                    data,
                    StripDecodeParams {
                        width,
                        height,
                        bits_per_sample: &bits_per_sample,
                        rows_per_strip,
                        strip_offsets: offsets,
                        strip_byte_counts: counts,
                    },
                    raw_strip,
                ),
                5 => decode_strips(
                    data,
                    StripDecodeParams {
                        width,
                        height,
                        bits_per_sample: &bits_per_sample,
                        rows_per_strip,
                        strip_offsets: offsets,
                        strip_byte_counts: counts,
                    },
                    decode_lzw_strip,
                ),
                _ => None,
            }
        };
        match decoded {
            Some(p) => p,
            None => return None,
        }
    } else {
        Vec::new()
    };

    Some(TiffImage {
        width,
        height,
        bits_per_sample,
        samples_per_pixel,
        compression,
        photometric,
        pixels,
        geo,
        geo_keys,
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    fn ifd_entry(out: &mut Vec<u8>, tag: u16, field_type: u16, count: u32, value: u32) {
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&field_type.to_le_bytes());
        out.extend_from_slice(&count.to_le_bytes());
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn build_tiff(
        width: u32,
        height: u32,
        compression: u16,
        pixels: &[u8],
        scale: Option<[f64; 3]>,
        tiepoint: Option<[f64; 6]>,
    ) -> Vec<u8> {
        let geo = scale.is_some() && tiepoint.is_some();
        let entry_count = if geo { 11u16 } else { 9u16 };
        let pixel_offset = (8 + 2 + entry_count as usize * 12 + 4) as u32;
        let scale_offset = pixel_offset + pixels.len() as u32;
        let tiepoint_offset = scale_offset + 24;

        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        out.extend_from_slice(&42u16.to_le_bytes());
        out.extend_from_slice(&8u32.to_le_bytes());
        out.extend_from_slice(&entry_count.to_le_bytes());

        ifd_entry(&mut out, 256, 4, 1, width);
        ifd_entry(&mut out, 257, 4, 1, height);
        ifd_entry(&mut out, 258, 3, 1, 8);
        ifd_entry(&mut out, 259, 3, 1, compression as u32);
        ifd_entry(&mut out, 262, 3, 1, 1);
        ifd_entry(&mut out, 273, 4, 1, pixel_offset);
        ifd_entry(&mut out, 277, 3, 1, 1);
        ifd_entry(&mut out, 278, 4, 1, height);
        ifd_entry(&mut out, 279, 4, 1, pixels.len() as u32);
        if geo {
            ifd_entry(&mut out, 33550, 12, 3, scale_offset);
            ifd_entry(&mut out, 33922, 12, 6, tiepoint_offset);
        }
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(pixels);
        if geo {
            for v in scale.unwrap() {
                out.extend_from_slice(&v.to_le_bytes());
            }
            for v in tiepoint.unwrap() {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        out
    }

    #[test]
    fn parses_uncompressed_strip() {
        let data = build_tiff(2, 2, 1, &[0x10, 0x20, 0x30, 0x40], None, None);
        let img = parse_tiff(&data).unwrap();
        assert_eq!(img.width, 2);
        assert_eq!(img.height, 2);
        assert_eq!(img.bits_per_sample, vec![8]);
        assert_eq!(img.samples_per_pixel, 1);
        assert_eq!(img.compression, 1);
        assert_eq!(img.photometric, Some(1));
        assert_eq!(img.pixels, vec![0x10, 0x20, 0x30, 0x40]);
        assert!(img.geo.is_none());
    }

    #[test]
    fn reads_pixel_scale_and_tiepoint() {
        let data = build_tiff(
            2,
            2,
            1,
            &[0x10, 0x20, 0x30, 0x40],
            Some([0.5, 0.5, 0.0]),
            Some([0.0, 0.0, 0.0, 10.0, 20.0, 0.0]),
        );
        let img = parse_tiff(&data).unwrap();
        let geo = img.geo.unwrap();
        assert_eq!(geo.dx, 0.5);
        assert_eq!(geo.dy, -0.5);
        assert_eq!(geo.x0, 10.0);
        assert_eq!(geo.y0, 20.0);
    }

    #[test]
    fn jpeg_without_pixel_data_parses_void() {
        let data = build_tiff(2, 2, 7, &[], None, None);
        assert!(parse_tiff(&data).is_none());
    }

    #[test]
    fn decodes_lzw_compressed_strip() {
        let lzw = [0x80u8, 0x18, 0x4C, 0x50, 0x10];
        let data = build_tiff(2, 1, 5, &lzw, None, None);
        let img = parse_tiff(&data).unwrap();
        assert_eq!(img.compression, 5);
        assert_eq!(img.pixels, vec![0x61, 0x62]);
    }

    #[test]
    fn lzw_roundtrip_through_encoder() {
        let input: Vec<u8> = (0..64u32)
            .flat_map(|i| [i as u8, (i.wrapping_mul(3)) as u8])
            .collect();
        let lzw = lzw_encode(&input);
        let data = build_tiff(input.len() as u32, 1, 5, &lzw, None, None);
        let img = parse_tiff(&data).unwrap();
        assert_eq!(img.pixels, input);
    }

    fn lzw_encode(input: &[u8]) -> Vec<u8> {
        use std::collections::HashMap;
        let mut dict: HashMap<Vec<u8>, u16> = HashMap::new();
        for i in 0..256u16 {
            dict.insert(vec![i as u8], i);
        }
        let mut free = LZW_FIRST;
        let mut width = 9u32;
        let mut encoded: Vec<(u16, u32)> = Vec::new();
        encoded.push((LZW_CLEAR, width));
        let mut w: Vec<u8> = Vec::new();
        for &k in input {
            let mut wk = w.clone();
            wk.push(k);
            if dict.contains_key(&wk) {
                w = wk;
            } else {
                encoded.push((dict[&w], width));
                if (free as usize) < LZW_TABLE_MAX {
                    dict.insert(wk, free);
                    free += 1;
                    if free == (1u16 << width) - 1 && width < 12 {
                        width += 1;
                    }
                }
                w = vec![k];
            }
        }
        encoded.push((dict[&w], width));
        encoded.push((LZW_EOI, width));
        let mut out: Vec<u8> = Vec::new();
        let mut acc = 0u32;
        let mut nbits = 0u32;
        for (code, w) in encoded {
            acc = (acc << w) | (code as u32);
            nbits += w;
            while nbits >= 8 {
                nbits -= 8;
                out.push((acc >> nbits) as u8);
                acc &= (1 << nbits) - 1;
            }
        }
        if nbits > 0 {
            out.push((acc << (8 - nbits)) as u8);
        }
        out
    }

    #[test]
    fn lzw_kwkwk_path_decodes() {
        let input = b"abababab".to_vec();
        let lzw = lzw_encode(&input);
        let data = build_tiff(input.len() as u32, 1, 5, &lzw, None, None);
        let img = parse_tiff(&data).unwrap();
        assert_eq!(img.pixels, input);
    }

    #[test]
    fn reads_big_endian() {
        let mut out = Vec::new();
        out.extend_from_slice(b"MM");
        out.extend_from_slice(&42u16.to_be_bytes());
        out.extend_from_slice(&8u32.to_be_bytes());
        out.extend_from_slice(&9u16.to_be_bytes());
        let be_entry = |e: &mut Vec<u8>, tag: u16, t: u16, count: u32, value: u32| {
            e.extend_from_slice(&tag.to_be_bytes());
            e.extend_from_slice(&t.to_be_bytes());
            e.extend_from_slice(&count.to_be_bytes());
            if t == 3 {
                e.extend_from_slice(&(value as u16).to_be_bytes());
                e.extend_from_slice(&[0, 0]);
            } else {
                e.extend_from_slice(&value.to_be_bytes());
            }
        };
        be_entry(&mut out, 256, 4, 1, 1);
        be_entry(&mut out, 257, 4, 1, 1);
        be_entry(&mut out, 258, 3, 1, 8);
        be_entry(&mut out, 259, 3, 1, 1);
        be_entry(&mut out, 262, 3, 1, 1);
        be_entry(&mut out, 273, 4, 1, 122);
        be_entry(&mut out, 277, 3, 1, 1);
        be_entry(&mut out, 278, 4, 1, 1);
        be_entry(&mut out, 279, 4, 1, 1);
        out.extend_from_slice(&0u32.to_be_bytes());
        out.push(0xAB);

        let img = parse_tiff(&out).unwrap();
        assert_eq!(img.width, 1);
        assert_eq!(img.height, 1);
        assert_eq!(img.pixels, vec![0xAB]);
    }

    #[test]
    fn reads_geo_key_directory() {
        let entry_count = 10u16;
        let pixel_offset = (8 + 2 + entry_count as usize * 12 + 4) as u32;
        let keys_offset = pixel_offset + 4;
        let keys: [u16; 16] = [1, 1, 0, 3, 1024, 0, 1, 2, 2048, 0, 1, 4326, 1025, 0, 1, 2];

        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        out.extend_from_slice(&42u16.to_le_bytes());
        out.extend_from_slice(&8u32.to_le_bytes());
        out.extend_from_slice(&entry_count.to_le_bytes());
        ifd_entry(&mut out, 256, 4, 1, 2);
        ifd_entry(&mut out, 257, 4, 1, 2);
        ifd_entry(&mut out, 258, 3, 1, 8);
        ifd_entry(&mut out, 259, 3, 1, 1);
        ifd_entry(&mut out, 262, 3, 1, 1);
        ifd_entry(&mut out, 273, 4, 1, pixel_offset);
        ifd_entry(&mut out, 277, 3, 1, 1);
        ifd_entry(&mut out, 278, 4, 1, 2);
        ifd_entry(&mut out, 279, 4, 1, 4);
        ifd_entry(&mut out, 34735, 3, 16, keys_offset);
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&[0x10, 0x20, 0x30, 0x40]);
        for &k in &keys {
            out.extend_from_slice(&k.to_le_bytes());
        }

        let img = parse_tiff(&out).unwrap();
        assert_eq!(img.geo_keys, Some(keys.to_vec()));
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_tiff(b"").is_none());
        assert!(parse_tiff(b"PK\x03\x04").is_none());
        assert!(parse_tiff(b"II\x00\x00").is_none());
        assert!(parse_tiff(b"MM\x00\x2a\x00\x00\x00\x08").is_none());
    }

    fn build_jpeg(
        width: u16,
        height: u16,
        components: &[(u8, u8, u8)],
        dc_counts: [u8; 16],
        dc_symbols: &[u8],
        entropy_bits: &[u8],
    ) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&[0xFF, 0xD8]);
        out.extend_from_slice(&[0xFF, 0xDB, 0x00, 0x43, 0x00]);
        out.extend_from_slice(&[1u8; 64]);
        let n = components.len() as u8;
        out.extend_from_slice(&[0xFF, 0xC0]);
        out.extend_from_slice(&((8 + 3 * n as usize) as u16).to_be_bytes());
        out.push(8);
        out.extend_from_slice(&height.to_be_bytes());
        out.extend_from_slice(&width.to_be_bytes());
        out.push(n);
        for &(id, h, v) in components {
            out.push(id);
            out.push((h << 4) | v);
            out.push(0);
        }
        out.extend_from_slice(&[0xFF, 0xC4]);
        out.extend_from_slice(&((19 + dc_symbols.len()) as u16).to_be_bytes());
        out.push(0x00);
        out.extend_from_slice(&dc_counts);
        out.extend_from_slice(dc_symbols);
        out.extend_from_slice(&[0xFF, 0xC4, 0x00, 0x14, 0x10]);
        out.extend_from_slice(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        out.push(0x00);
        out.extend_from_slice(&[0xFF, 0xDA]);
        out.extend_from_slice(&((6 + 2 * n as usize) as u16).to_be_bytes());
        out.push(n);
        for &(id, _, _) in components {
            out.push(id);
            out.push(0x00);
        }
        out.extend_from_slice(&[0x00, 0x3F, 0x00]);
        let total = entropy_bits.len().div_ceil(8) * 8;
        let mut byte = 0u8;
        let mut nbits = 0usize;
        for i in 0..total {
            let bit = if i < entropy_bits.len() {
                entropy_bits[i] & 1
            } else {
                1
            };
            byte = (byte << 1) | bit;
            nbits += 1;
            if nbits == 8 {
                out.push(byte);
                byte = 0;
                nbits = 0;
            }
        }
        out.extend_from_slice(&[0xFF, 0xD9]);
        out
    }

    fn build_tiled_tiff(width: u32, height: u32, tile: &[u8]) -> Vec<u8> {
        let n = 11u16;
        let tile_offset = (8 + 2 + n as usize * 12 + 4) as u32;
        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        out.extend_from_slice(&42u16.to_le_bytes());
        out.extend_from_slice(&8u32.to_le_bytes());
        out.extend_from_slice(&n.to_le_bytes());
        ifd_entry(&mut out, 256, 4, 1, width);
        ifd_entry(&mut out, 257, 4, 1, height);
        ifd_entry(&mut out, 258, 3, 1, 8);
        ifd_entry(&mut out, 259, 3, 1, 7);
        ifd_entry(&mut out, 262, 3, 1, 1);
        ifd_entry(&mut out, 277, 3, 1, 1);
        ifd_entry(&mut out, 284, 3, 1, 1);
        ifd_entry(&mut out, 322, 4, 1, 16);
        ifd_entry(&mut out, 323, 4, 1, 16);
        ifd_entry(&mut out, 324, 4, 1, tile_offset);
        ifd_entry(&mut out, 325, 4, 1, tile.len() as u32);
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(tile);
        out
    }

    #[test]
    fn decodes_baseline_jpeg_grayscale() {
        let jpeg = build_jpeg(
            8,
            8,
            &[(1, 1, 1)],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0],
            &[0, 0],
        );
        let img = decode_jpeg(&jpeg).unwrap();
        assert_eq!(img.width, 8);
        assert_eq!(img.height, 8);
        assert_eq!(img.components, 1);
        assert_eq!(img.pixels.len(), 64);
        assert!(img.pixels.iter().all(|&b| b == 128));
    }

    #[test]
    fn decodes_baseline_jpeg_rgb_passthrough() {
        let comps = [(82u8, 1u8, 1u8), (71, 1, 1), (66, 1, 1)];
        let jpeg = build_jpeg(
            8,
            8,
            &comps,
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0],
            &[0; 6],
        );
        let img = decode_jpeg(&jpeg).unwrap();
        assert_eq!(img.components, 3);
        assert_eq!(img.pixels.len(), 8 * 8 * 3);
        assert!(img.pixels.chunks(3).all(|px| px == [128, 128, 128]));
    }

    #[test]
    fn decodes_ycbcr_to_rgb() {
        let comps = [(1u8, 1u8, 1u8), (2, 1, 1), (3, 1, 1)];
        let dc_counts = [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let dc_symbols = [0x00u8, 0x04];
        let bits = [0u8, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0];
        let jpeg = build_jpeg(8, 8, &comps, dc_counts, &dc_symbols, &bits);
        let img = decode_jpeg(&jpeg).unwrap();
        assert_eq!(img.components, 3);
        assert!(img.pixels.chunks(3).all(|px| px == [129, 127, 128]));
    }

    #[test]
    fn parses_tiled_jpeg_tiff() {
        let jpeg = build_jpeg(
            16,
            16,
            &[(1, 1, 1)],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0],
            &[0; 8],
        );
        let data = build_tiled_tiff(16, 16, &jpeg);
        let img = parse_tiff(&data).unwrap();
        assert_eq!(img.width, 16);
        assert_eq!(img.height, 16);
        assert_eq!(img.compression, 7);
        assert_eq!(img.pixels.len(), 16 * 16);
        assert!(img.pixels.iter().all(|&b| b == 128));
    }

    #[test]
    fn crops_edge_tile() {
        let jpeg = build_jpeg(
            16,
            16,
            &[(1, 1, 1)],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0],
            &[0; 8],
        );
        let data = build_tiled_tiff(8, 8, &jpeg);
        let img = parse_tiff(&data).unwrap();
        assert_eq!(img.width, 8);
        assert_eq!(img.height, 8);
        assert_eq!(img.pixels.len(), 64);
        assert!(img.pixels.iter().all(|&b| b == 128));
    }

    #[test]
    #[ignore = "reads the GeoTIFF named by OMEGAFLOW_OCS_TIFF"]
    fn real_ocs_float32_lzw_geotiff_decodes() {
        let path = std::env::var("OMEGAFLOW_OCS_TIFF")
            .expect("OMEGAFLOW_OCS_TIFF names a GeoTIFF on disk");
        let bytes = std::fs::read(&path).expect("read the OCS GeoTIFF");
        let img = parse_tiff(&bytes).expect("the GeoTIFF parses");
        assert_eq!(img.compression, 5);
        assert_eq!(img.samples_per_pixel, 2);
        assert_eq!(img.bits_per_sample, vec![32, 32]);
        assert_eq!(
            img.pixels.len(),
            img.width as usize * img.height as usize * 8,
            "two float32 bands per pixel"
        );
        let floats: Vec<f32> = img
            .pixels
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();
        assert!(
            floats.iter().any(|v| v.is_finite()),
            "the decoded float32 band carries a real value"
        );
        fn min_valid(band: &[f32]) -> f32 {
            band.iter()
                .copied()
                .filter(|v| v.is_finite() && *v != f32::MAX)
                .fold(f32::INFINITY, f32::min)
        }
        let band0: Vec<f32> = floats.iter().step_by(2).copied().collect();
        let band1: Vec<f32> = floats.iter().skip(1).step_by(2).copied().collect();
        assert_eq!(min_valid(&band0), -15.85, "Elevation floor");
        assert_eq!(min_valid(&band1), 1.103_64, "Uncertainty floor");
        assert!(
            floats.contains(&f32::MAX),
            "the GDAL_NODATA 3.4028235e38 sentinel decodes"
        );
    }
}
