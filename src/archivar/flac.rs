const MAGIC: [u8; 4] = *b"fLaC";
const BIN_MAGIC: [u8; 4] = *b"FLCS";
const STREAMINFO_TYPE: u8 = 0;

pub const COMP_PCM: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StreamInfo {
    pub min_block_size: u16,
    pub max_block_size: u16,
    pub sample_rate: u32,
    pub channels: u8,
    pub bits_per_sample: u8,
    pub total_samples: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlacDecoded {
    pub sample_rate: u32,
    pub channels: u8,
    pub bits_per_sample: u8,
    pub samples: Vec<i32>,
}

struct Bits<'a> {
    buf: &'a [u8],
    pos: usize,
    acc: u64,
    nbits: u32,
}

impl<'a> Bits<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            acc: 0,
            nbits: 0,
        }
    }

    fn ensure(&mut self, n: u32) -> Option<()> {
        while self.nbits < n {
            if self.nbits >= 56 {
                return None;
            }
            let b = *self.buf.get(self.pos)? as u64;
            self.pos += 1;
            self.acc = (self.acc << 8) | b;
            self.nbits += 8;
        }
        Some(())
    }

    fn read(&mut self, n: u32) -> Option<u64> {
        self.ensure(n)?;
        let v = (self.acc >> (self.nbits - n)) & ((1u64 << n) - 1);
        self.nbits -= n;
        self.acc &= (1u64 << self.nbits) - 1;
        Some(v)
    }

    fn read_signed(&mut self, n: u32) -> Option<i64> {
        let u = self.read(n)?;
        let sign = 1u64 << (n - 1);
        Some(((u ^ sign) as i64) - sign as i64)
    }

    fn unary(&mut self) -> Option<u32> {
        let mut nzero = 0u32;
        loop {
            self.ensure(1)?;
            let top = (self.acc >> (self.nbits - 1)) & 1;
            self.nbits -= 1;
            self.acc &= (1u64 << self.nbits) - 1;
            if top == 1 {
                return Some(nzero);
            }
            nzero += 1;
        }
    }

    fn align_byte(&mut self) {
        self.nbits -= self.nbits % 8;
        self.acc &= if self.nbits == 0 {
            0
        } else {
            (1u64 << self.nbits) - 1
        };
    }
}

fn parse_streaminfo(b: &[u8]) -> Option<StreamInfo> {
    if b.len() < 34 {
        return None;
    }
    let min_block_size = ((b[0] as u16) << 8) | (b[1] as u16);
    let max_block_size = ((b[2] as u16) << 8) | (b[3] as u16);
    let sample_rate = (((b[10] as u32) << 12) | ((b[11] as u32) << 4) | ((b[12] as u32) >> 4))
        & 0xFFFFF;
    let channels = (((b[12] as u32) >> 1) & 0x07) + 1;
    let bits_per_sample = (((b[12] as u32) & 0x01) << 4 | ((b[13] as u32) >> 4)) + 1;
    let total_samples = (((b[13] as u64) & 0x0F) << 32)
        | ((b[14] as u64) << 24)
        | ((b[15] as u64) << 16)
        | ((b[16] as u64) << 8)
        | (b[17] as u64);
    if sample_rate == 0 || bits_per_sample < 4 || bits_per_sample > 32 {
        return None;
    }
    Some(StreamInfo {
        min_block_size,
        max_block_size,
        sample_rate,
        channels: channels as u8,
        bits_per_sample: bits_per_sample as u8,
        total_samples,
    })
}

fn read_sample_rate(code: u8, bits: &mut Bits) -> Option<u32> {
    match code {
        0 => None,
        1 => Some(88_200),
        2 => Some(176_400),
        3 => Some(192_000),
        4 => Some(8_000),
        5 => Some(16_000),
        6 => Some(22_050),
        7 => Some(24_000),
        8 => Some(32_000),
        9 => Some(44_100),
        10 => Some(48_000),
        11 => Some(96_000),
        12 => Some(bits.read(8)? as u32 * 1000),
        13 => Some(bits.read(16)? as u32),
        14 => Some(bits.read(16)? as u32 * 10),
        15 => None,
        _ => None,
    }
}

fn read_sample_size(code: u8, _bits: &mut Bits) -> Option<u8> {
    match code {
        0 => None,
        1 => Some(8),
        2 => Some(12),
        3 => None,
        4 => Some(16),
        5 => Some(20),
        6 => Some(24),
        7 => Some(32),
        _ => None,
    }
}

fn read_utf8_coded(bits: &mut Bits) -> Option<u64> {
    let b0 = bits.read(8)? as u8;
    if b0 & 0x80 == 0 {
        return Some(b0 as u64);
    }
    let (bytes, top_mask) = if b0 & 0xC0 == 0x80 {
        return None;
    } else if b0 & 0xE0 == 0xC0 {
        (2, 0x1F)
    } else if b0 & 0xF0 == 0xE0 {
        (3, 0x0F)
    } else if b0 & 0xF8 == 0xF0 {
        (4, 0x07)
    } else if b0 & 0xFC == 0xF8 {
        (5, 0x03)
    } else if b0 & 0xFE == 0xFC {
        (6, 0x01)
    } else if b0 & 0xFE == 0xFE {
        (7, 0x00)
    } else {
        return None;
    };
    let mut v = (b0 & top_mask) as u64;
    for _ in 1..bytes {
        let cont = bits.read(8)? as u8;
        if cont & 0xC0 != 0x80 {
            return None;
        }
        v = (v << 6) | (cont & 0x3F) as u64;
    }
    Some(v)
}

fn read_rice(bits: &mut Bits, k: u32) -> Option<i32> {
    let q = bits.unary()? as u64;
    let rem = bits.read(k)?;
    let zig = (q << k) | rem;
    let val = if zig & 1 == 0 {
        (zig >> 1) as i32
    } else {
        -((zig >> 1) as i32) - 1
    };
    Some(val)
}

struct SubframeHeader {
    sample_count: usize,
    bps: u8,
}

fn decode_subframe(
    bits: &mut Bits,
    header: &SubframeHeader,
    out: &mut [i32],
) -> Option<()> {
    let pad = bits.read(1)?;
    if pad != 0 {
        return None;
    }
    let stype = bits.read(6)? as u8;
    let wasted_flag = bits.read(1)?;
    let mut wasted = 0u8;
    if wasted_flag == 1 {
        wasted = bits.read(1)? as u8;
    }
    let shift = wasted as u32;
    let n = header.sample_count;
    let bps = header.bps;
    match stype {
        0 => {
            let v = bits.read(bps as u32)? as i32 >> shift;
            for s in out.iter_mut().take(n) {
                *s = v;
            }
        }
        1 => {
            for s in out.iter_mut().take(n) {
                let raw = bits.read(bps as u32)? as i32;
                *s = raw >> shift;
            }
        }
        8..=12 => {
            let order = (stype - 8) as usize;
            let mut warm: [i64; 4] = [0; 4];
            for i in 0..order {
                warm[i] = bits.read(bps as u32)? as i64;
            }
            for (i, s) in out.iter_mut().take(n).enumerate() {
                if i < order {
                    *s = (warm[i] >> shift) as i32;
                    continue;
                }
                let res = read_rice(bits, bps as u32)? as i64;
                let pred: i64 = match order {
                    0 => 0,
                    1 => warm[0],
                    2 => 2 * warm[0] - warm[1],
                    3 => 3 * warm[0] - 3 * warm[1] + warm[2],
                    _ => 4 * warm[0] - 6 * warm[1] + 4 * warm[2] - warm[3],
                };
                let v = pred + res;
                *s = (v >> shift) as i32;
                for j in (1..order).rev() {
                    warm[j] = warm[j - 1];
                }
                warm[0] = v;
            }
        }
        32..=63 => {
            let order = (stype - 31) as usize;
            let mut warm: Vec<i64> = Vec::with_capacity(order);
            for _ in 0..order {
                warm.push(bits.read(bps as u32)? as i64);
            }
            let precision = bits.read(4)? as u32 + 1;
            let shift_raw = bits.read_signed(5)?;
            let mut coeff: Vec<i64> = Vec::with_capacity(order);
            for _ in 0..order {
                coeff.push(bits.read_signed(precision)?);
            }
            for (i, s) in out.iter_mut().take(n).enumerate() {
                if i < order {
                    *s = (warm[i] >> shift) as i32;
                    continue;
                }
                let res = read_rice(bits, bps as u32)? as i64;
                let mut pred: i64 = 0;
                for j in 0..order {
                    pred += coeff[j] * warm[j];
                }
                pred >>= shift_raw;
                let v = pred + res;
                *s = (v >> shift) as i32;
                for j in (1..order).rev() {
                    warm[j] = warm[j - 1];
                }
                warm[0] = v;
            }
        }
        _ => return None,
    }
    Some(())
}

fn decode_frame(
    bits: &mut Bits,
    si: &StreamInfo,
    out: &mut Vec<i32>,
) -> Option<()> {
    let _sync = bits.read(14)?;
    let _reserved = bits.read(1)?;
    let blocking = bits.read(1)?;
    let bs_code = bits.read(4)? as u8;
    let sr_code = bits.read(4)? as u8;
    let ch_assign = bits.read(4)? as u8;
    let ss_code = bits.read(3)? as u8;
    let _reserved2 = bits.read(1)?;
    let _frame_or_sample = read_utf8_coded(bits)?;

    let block_size = match bs_code {
        1 => 192,
        6 => bits.read(8)? as usize + 1,
        7 => bits.read(16)? as usize + 1,
        c @ 2..=5 => 576 * (1usize << (c - 2)),
        c @ 8..=15 => 256 * (1usize << (c - 8)),
        0 => si.min_block_size as usize,
        _ => return None,
    };
    let _frame_sample_rate = match sr_code {
        0 => si.sample_rate,
        c => read_sample_rate(c, bits)?,
    };
    let bps = match ss_code {
        0 => si.bits_per_sample,
        c => read_sample_size(c, bits)?,
    };
    if bps < 4 || bps > 32 {
        return None;
    }

    let _crc = bits.read(8)?;

    let (channels, decorr) = match ch_assign {
        0..=7 => (ch_assign as usize + 1, 0u8),
        8 => (2, 8),
        9 => (2, 9),
        10 => (2, 10),
        _ => return None,
    };

    let hdr = SubframeHeader {
        sample_count: block_size,
        bps,
    };
    let mut per_channel: Vec<Vec<i32>> = Vec::with_capacity(channels);
    for _ in 0..channels {
        let mut sub = vec![0i32; block_size];
        decode_subframe(bits, &hdr, &mut sub)?;
        per_channel.push(sub);
    }

    for i in 0..block_size {
        let (l, r) = if channels == 2 {
            (per_channel[0][i], per_channel[1][i])
        } else {
            (per_channel[0][i], 0)
        };
        let (left, right) = match decorr {
            8 => (l, l - r),
            9 => (l + r, r),
            10 => {
                let mid = l;
                let side = r;
                let left = mid + ((side + 1) >> 1);
                let right = mid - (side >> 1);
                (left, right)
            }
            _ => (l, r),
        };
        for c in 0..channels {
            let v = if c == 0 { left } else { right };
            out.push(v);
        }
    }
    bits.align_byte();
    let _ = bits.read(16)?;
    let _ = blocking;
    Some(())
}

pub fn decode(buf: &[u8]) -> Option<FlacDecoded> {
    if buf.len() < 4 || buf[0..4] != MAGIC {
        return None;
    }
    let mut off = 4usize;
    let mut si: Option<StreamInfo> = None;
    loop {
        let hdr = *buf.get(off)?;
        let last = hdr & 0x80 != 0;
        let btype = hdr & 0x7F;
        let len = ((*buf.get(off + 1)? as usize) << 16)
            | ((*buf.get(off + 2)? as usize) << 8)
            | (*buf.get(off + 3)? as usize);
        let body = buf.get(off + 4..off + 4 + len)?;
        if btype == STREAMINFO_TYPE {
            si = parse_streaminfo(body);
        }
        off += 4 + len;
        if last {
            break;
        }
    }
    let si = si?;
    let mut bits = Bits::new(buf);
    bits.pos = off;
    bits.nbits = 0;
    bits.acc = 0;
    let mut samples = Vec::new();
    let mut guard = 0usize;
    while bits.pos < buf.len() {
        let start = bits.pos;
        match decode_frame(&mut bits, &si, &mut samples) {
            Some(()) => {}
            None => {
                break;
            }
        }
        if bits.pos <= start {
            break;
        }
        guard += 1;
        if guard > 1_000_000 {
            break;
        }
    }
    if samples.is_empty() {
        return None;
    }
    let expected = si.total_samples as usize * si.channels as usize;
    if expected > 0 && samples.len() > expected {
        samples.truncate(expected);
    }
    Some(FlacDecoded {
        sample_rate: si.sample_rate,
        channels: si.channels,
        bits_per_sample: si.bits_per_sample,
        samples,
    })
}

pub const SERIES_HEADER_BYTES: usize = 4 + 4 + 8 + 8;
pub const SERIES_REC_BYTES: usize = 8;

pub fn write_series(samples: &[f64], epoch: f64, sample_rate: f64) -> Vec<u8> {
    let mut out = Vec::with_capacity(SERIES_HEADER_BYTES + samples.len() * SERIES_REC_BYTES);
    out.extend_from_slice(&BIN_MAGIC);
    out.extend_from_slice(&(samples.len() as u32).to_le_bytes());
    out.extend_from_slice(&epoch.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    for s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

pub fn parse_series(data: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if data.len() < SERIES_HEADER_BYTES || data[0..4] != BIN_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    let epoch = f64::from_le_bytes(data[8..16].try_into().ok()?);
    let sample_rate = f64::from_le_bytes(data[16..24].try_into().ok()?);
    if !epoch.is_finite() || epoch <= 0.0 || !sample_rate.is_finite() || sample_rate <= 0.0 {
        return None;
    }
    if data.len() != SERIES_HEADER_BYTES + count * SERIES_REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = SERIES_HEADER_BYTES + i * SERIES_REC_BYTES;
        let v = f64::from_le_bytes(data.get(base..base + 8)?.try_into().ok()?);
        if !v.is_finite() {
            return None;
        }
        let t = epoch + i as f64 / sample_rate;
        out.push((t, v, COMP_PCM));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn streaminfo_bytes(sample_rate: u32, channels: u8, bps: u8, total: u64) -> Vec<u8> {
        let mut b = vec![0u8; 34];
        let sr = sample_rate & 0xFFFFF;
        b[10] = (sr >> 12) as u8;
        b[11] = (sr >> 4) as u8;
        b[12] = ((sr & 0xF) as u8) << 4;
        b[12] |= ((channels - 1) << 1) as u8;
        b[12] |= ((bps - 1) >> 4) as u8;
        b[13] = ((bps - 1) << 4) as u8;
        b[13] |= ((total >> 32) & 0xF) as u8;
        b[14] = (total >> 24) as u8;
        b[15] = (total >> 16) as u8;
        b[16] = (total >> 8) as u8;
        b[17] = total as u8;
        b
    }

    fn header_streaminfo(si: &[u8], last: bool) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&MAGIC);
        out.push(if last { 0x80 } else { 0x00 });
        let len = si.len();
        out.push((len >> 16) as u8);
        out.push((len >> 8) as u8);
        out.push(len as u8);
        out.extend_from_slice(si);
        out
    }

    #[test]
    fn streaminfo_parses_sample_rate_channels_bps() {
        let si = streaminfo_bytes(88_752, 1, 20, 36_522_951_734);
        let parsed = parse_streaminfo(&si).unwrap();
        assert_eq!(parsed.sample_rate, 88_752);
        assert_eq!(parsed.channels, 1);
        assert_eq!(parsed.bits_per_sample, 20);
        assert_eq!(parsed.total_samples, 36_522_951_734);
    }

    #[test]
    fn decode_rejects_foreign_magic_and_empty() {
        assert!(decode(b"X").is_none());
        assert!(decode(b"fLaC").is_none());
        assert!(parse_series(b"X").is_none());
    }

    #[test]
    fn series_roundtrip_derives_time() {
        let bytes = write_series(&[1.0, -2.0, 3.0], 1.5e9, 1000.0);
        let series = parse_series(&bytes).unwrap();
        assert_eq!(series.len(), 3);
        assert_eq!(series[0], (1.5e9, 1.0, COMP_PCM));
        assert_eq!(series[1], (1.5e9 + 0.001, -2.0, COMP_PCM));
        assert_eq!(series[2], (1.5e9 + 0.002, 3.0, COMP_PCM));
    }

    #[test]
    fn series_rejects_truncated_and_bad_epoch() {
        let bytes = write_series(&[1.0, 2.0], 1.5e9, 1000.0);
        assert!(parse_series(&bytes[..bytes.len() - 1]).is_none());
        let mut bad = write_series(&[1.0], 0.0, 1000.0);
        bad[8..16].copy_from_slice(&0.0f64.to_le_bytes());
        assert!(parse_series(&bad).is_none());
        let mut nan = write_series(&[1.0], 1.5e9, 1000.0);
        nan[8..16].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_series(&nan).is_none());
    }

    #[test]
    fn decode_constant_frame_reconstructs_samples() {
        let si = streaminfo_bytes(44_100, 1, 16, 192);
        let mut buf = header_streaminfo(&si, true);
        let mut bits: Vec<u8> = Vec::new();
        let mut wr = BitWriter::new(&mut bits);
        wr.write(0x3FFE, 14);
        wr.write(0, 1);
        wr.write(0, 1);
        wr.write(1, 4);
        wr.write(0, 4);
        wr.write(0, 4);
        wr.write(0, 3);
        wr.write(0, 1);
        wr.write(0, 8);
        wr.write(0, 8);
        wr.write(0, 1);
        wr.write(0, 6);
        wr.write(0, 1);
        wr.write(0x7FFF, 16);
        wr.align();
        wr.write(0, 16);
        buf.extend_from_slice(&bits);
        let decoded = decode(&buf).unwrap();
        assert_eq!(decoded.channels, 1);
        assert_eq!(decoded.bits_per_sample, 16);
        assert_eq!(decoded.samples.len(), 192);
        assert!(decoded.samples.iter().all(|&s| s == 0x7FFF));
    }

    struct BitWriter<'a> {
        out: &'a mut Vec<u8>,
        acc: u64,
        nbits: u32,
    }

    impl<'a> BitWriter<'a> {
        fn new(out: &'a mut Vec<u8>) -> Self {
            Self {
                out,
                acc: 0,
                nbits: 0,
            }
        }

        fn write(&mut self, v: u64, n: u32) {
            self.acc = (self.acc << n) | (v & ((1u64 << n) - 1));
            self.nbits += n;
            while self.nbits >= 8 {
                self.nbits -= 8;
                self.out.push((self.acc >> self.nbits) as u8);
                self.acc &= (1u64 << self.nbits) - 1;
            }
        }

        fn align(&mut self) {
            if self.nbits > 0 {
                self.out.push((self.acc << (8 - self.nbits)) as u8);
                self.acc = 0;
                self.nbits = 0;
            }
        }
    }
}
