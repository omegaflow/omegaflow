use super::{decode_point, LasHeader, LasNote, LasPoint, LasVlr};

const BM_LENGTH_SHIFT: u32 = 13;
const DM_LENGTH_SHIFT: u32 = 15;
const AC_MIN_LENGTH: u32 = 0x0100_0000;
const AC_MAX_LENGTH: u32 = 0xFFFF_FFFF;
const BM_MAX_COUNT: u32 = 1 << 13;
const DM_MAX_COUNT: u32 = 1 << 15;

const LASZIP_GPSTIME_MULTI: i32 = 500;
const LASZIP_GPSTIME_MULTI_MINUS: i32 = -10;
const LASZIP_GPSTIME_MULTI_CODE_FULL: i32 = LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 1;
const LASZIP_GPSTIME_MULTI_TOTAL: u32 =
    (LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 5) as u32;

const LASZIP_COMPRESSOR_POINTWISE_CHUNKED: u16 = 2;
const LASZIP_COMPRESSOR_LAYERED_CHUNKED: u16 = 3;

const ITEM_POINT10: u16 = 6;
const ITEM_GPSTIME11: u16 = 7;
const ITEM_RGB12: u16 = 8;
const ITEM_WAVEPACKET13: u16 = 9;
const ITEM_POINT14: u16 = 10;
const ITEM_RGB14: u16 = 11;
const ITEM_RGBNIR14: u16 = 12;
const ITEM_WAVEPACKET14: u16 = 13;
const ITEM_BYTE14: u16 = 14;

const LASZIP_VLR_RECORD_ID: u16 = 22204;

fn le_i16(b: &[u8]) -> i16 {
    i16::from_le_bytes([b[0], b[1]])
}

fn le_i64(b: &[u8]) -> i64 {
    i64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

fn zero_bit_0(n: u32) -> u32 {
    n & 0xFFFF_FFFE
}

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
    base: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8], base: usize) -> Self {
        Cursor { data, pos: 0, base }
    }

    fn read_u32(&mut self) -> Result<u32, LasNote> {
        let b = self.read_bytes(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn read_bytes(&mut self, n: usize) -> Result<&'a [u8], LasNote> {
        if self.pos + n > self.data.len() {
            return Err(LasNote::EndAtByte {
                off: self.base + self.pos,
            });
        }
        let out = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(out)
    }
}

struct BitModel {
    bit_0_count: u32,
    bit_count: u32,
    bit_0_prob: u32,
    update_cycle: u32,
    bits_until_update: u32,
}

impl BitModel {
    fn new() -> Self {
        let mut m = BitModel {
            bit_0_count: 0,
            bit_count: 0,
            bit_0_prob: 0,
            update_cycle: 0,
            bits_until_update: 0,
        };
        m.init();
        m
    }

    fn init(&mut self) {
        self.bit_0_count = 1;
        self.bit_count = 2;
        self.bit_0_prob = 1 << (BM_LENGTH_SHIFT - 1);
        self.update_cycle = 4;
        self.bits_until_update = 4;
    }

    fn update(&mut self) {
        self.bit_count += self.update_cycle;
        if self.bit_count > BM_MAX_COUNT {
            self.bit_count = (self.bit_count + 1) >> 1;
            self.bit_0_count = (self.bit_0_count + 1) >> 1;
            if self.bit_0_count == self.bit_count {
                self.bit_count += 1;
            }
        }
        let scale = 0x8000_0000u64 / self.bit_count as u64;
        self.bit_0_prob = ((self.bit_0_count as u64 * scale) >> (31 - BM_LENGTH_SHIFT)) as u32;
        self.update_cycle = (5 * self.update_cycle) >> 2;
        if self.update_cycle > 64 {
            self.update_cycle = 64;
        }
        self.bits_until_update = self.update_cycle;
    }
}

struct SymbolModel {
    symbols: u32,
    symbol_count: Vec<u32>,
    distribution: Vec<u32>,
    total_count: u32,
    update_cycle: u32,
    symbols_until_update: u32,
}

impl SymbolModel {
    fn new(symbols: u32) -> Self {
        SymbolModel {
            symbols,
            symbol_count: vec![1; symbols as usize],
            distribution: vec![0; symbols as usize],
            total_count: 0,
            update_cycle: 0,
            symbols_until_update: 0,
        }
    }

    fn init(&mut self) {
        self.total_count = 0;
        self.update_cycle = self.symbols;
        for c in &mut self.symbol_count {
            *c = 1;
        }
        self.update();
        self.update_cycle = (self.symbols + 6) >> 1;
        self.symbols_until_update = self.update_cycle;
    }

    fn update(&mut self) {
        self.total_count += self.update_cycle;
        if self.total_count > DM_MAX_COUNT {
            self.total_count = 0;
            for c in &mut self.symbol_count {
                *c = (*c + 1) >> 1;
                self.total_count += *c;
            }
        }
        let scale = 0x8000_0000u64 / self.total_count as u64;
        let mut sum: u32 = 0;
        for k in 0..self.symbols as usize {
            self.distribution[k] = ((scale * sum as u64) >> (31 - DM_LENGTH_SHIFT)) as u32;
            sum += self.symbol_count[k];
        }
        self.update_cycle = (5 * self.update_cycle) >> 2;
        let max_cycle = (self.symbols + 6) << 3;
        if self.update_cycle > max_cycle {
            self.update_cycle = max_cycle;
        }
        self.symbols_until_update = self.update_cycle;
    }
}

struct AcDecoder {
    data: Vec<u8>,
    pos: usize,
    base: usize,
    value: u32,
    length: u32,
}

impl AcDecoder {
    fn init(data: &[u8], base: usize) -> Result<Self, LasNote> {
        let mut dec = AcDecoder {
            data: data.to_vec(),
            pos: 0,
            base,
            value: 0,
            length: AC_MAX_LENGTH,
        };
        let value = (dec.get_byte()? as u32) << 24
            | (dec.get_byte()? as u32) << 16
            | (dec.get_byte()? as u32) << 8
            | dec.get_byte()? as u32;
        dec.value = value;
        Ok(dec)
    }

    fn get_byte(&mut self) -> Result<u8, LasNote> {
        if self.pos >= self.data.len() {
            return Err(LasNote::LazCoderStall {
                off: self.base + self.pos,
            });
        }
        let b = self.data[self.pos];
        self.pos += 1;
        Ok(b)
    }

    fn renorm(&mut self) -> Result<(), LasNote> {
        loop {
            self.value = (self.value << 8) | self.get_byte()? as u32;
            self.length <<= 8;
            if self.length >= AC_MIN_LENGTH {
                break;
            }
        }
        Ok(())
    }

    fn decode_bit(&mut self, m: &mut BitModel) -> Result<u32, LasNote> {
        let x = (m.bit_0_prob as u64 * (self.length >> BM_LENGTH_SHIFT) as u64) as u32;
        let sym = if self.value >= x { 1 } else { 0 };
        if sym == 0 {
            self.length = x;
            m.bit_0_count += 1;
        } else {
            self.value -= x;
            self.length -= x;
        }
        if self.length < AC_MIN_LENGTH {
            self.renorm()?;
        }
        m.bits_until_update -= 1;
        if m.bits_until_update == 0 {
            m.update();
        }
        Ok(sym)
    }

    fn decode_symbol(&mut self, m: &mut SymbolModel) -> Result<u32, LasNote> {
        let mut y = self.length;
        let mut x = 0u32;
        let mut sym = 0u32;
        self.length >>= DM_LENGTH_SHIFT;
        let mut n = m.symbols;
        let mut k = m.symbols >> 1;
        loop {
            let z = (self.length as u64 * m.distribution[k as usize] as u64) as u32;
            if z > self.value {
                n = k;
                y = z;
            } else {
                sym = k;
                x = z;
            }
            k = (sym + n) >> 1;
            if k == sym {
                break;
            }
        }
        self.value -= x;
        self.length = y - x;
        if self.length < AC_MIN_LENGTH {
            self.renorm()?;
        }
        m.symbol_count[sym as usize] += 1;
        m.symbols_until_update -= 1;
        if m.symbols_until_update == 0 {
            m.update();
        }
        Ok(sym)
    }

    fn read_bits(&mut self, bits: u32) -> Result<u32, LasNote> {
        if bits > 19 {
            let tmp = self.read_short()?;
            let bits = bits - 16;
            let tmp1 = self.read_bits(bits)? << 16;
            return Ok(tmp1 | tmp);
        }
        self.length >>= bits;
        let sym = self.value / self.length;
        self.value -= self.length * sym;
        if self.length < AC_MIN_LENGTH {
            self.renorm()?;
        }
        Ok(sym)
    }

    fn read_short(&mut self) -> Result<u32, LasNote> {
        self.length >>= 16;
        let sym = self.value / self.length;
        self.value -= self.length * sym;
        if self.length < AC_MIN_LENGTH {
            self.renorm()?;
        }
        Ok(sym)
    }

    fn read_int(&mut self) -> Result<u32, LasNote> {
        let lo = self.read_short()?;
        let hi = self.read_short()?;
        Ok((hi << 16) | lo)
    }

    fn read_int64(&mut self) -> Result<u64, LasNote> {
        let lower = self.read_int()? as u64;
        let upper = self.read_int()? as u64;
        Ok((upper << 32) | lower)
    }
}

struct IntegerCompressor {
    corr_range: u32,
    corr_min: i32,
    bits_high: u32,
    k: u32,
    m_bits: Vec<SymbolModel>,
    m_corrector0: BitModel,
    m_corrector: Vec<SymbolModel>,
}

impl IntegerCompressor {
    fn new(bits: u32, contexts: u32, bits_high: u32) -> Self {
        let (corr_bits, corr_range, corr_min) = if bits < 32 {
            let range = 1u32 << bits;
            (bits, range, -((range / 2) as i32))
        } else {
            (32u32, 0u32, i32::MIN)
        };
        let m_bits = (0..contexts)
            .map(|_| SymbolModel::new(corr_bits + 1))
            .collect();
        let m_corrector = (1..=corr_bits)
            .map(|i| {
                if i <= bits_high {
                    SymbolModel::new(1 << i)
                } else {
                    SymbolModel::new(1 << bits_high)
                }
            })
            .collect();
        IntegerCompressor {
            corr_range,
            corr_min,
            bits_high,
            k: 0,
            m_bits,
            m_corrector0: BitModel::new(),
            m_corrector,
        }
    }

    fn init(&mut self) {
        for m in &mut self.m_bits {
            m.init();
        }
        self.m_corrector0.init();
        for m in &mut self.m_corrector {
            m.init();
        }
    }

    fn read_corrector(&mut self, dec: &mut AcDecoder, context: u32) -> Result<i32, LasNote> {
        let k = {
            let m = &mut self.m_bits[context as usize];
            dec.decode_symbol(m)?
        };
        self.k = k;
        let c: i64 = if k != 0 {
            if k < 32 {
                let mut c = if k <= self.bits_high {
                    let m = &mut self.m_corrector[k as usize - 1];
                    dec.decode_symbol(m)? as i64
                } else {
                    let k1 = k - self.bits_high;
                    let hi = {
                        let m = &mut self.m_corrector[k as usize - 1];
                        dec.decode_symbol(m)? as i64
                    };
                    let lo = dec.read_bits(k1)? as i64;
                    (hi << k1) | lo
                };
                if c >= (1i64 << (k - 1)) {
                    c += 1;
                } else {
                    c -= (1i64 << k) - 1;
                }
                c
            } else {
                self.corr_min as i64
            }
        } else {
            dec.decode_bit(&mut self.m_corrector0)? as i64
        };
        Ok(c as i32)
    }

    fn decompress(&mut self, dec: &mut AcDecoder, pred: i32, context: u32) -> Result<i32, LasNote> {
        let corr = self.read_corrector(dec, context)?;
        let real = pred.wrapping_add(corr);
        let real = if real < 0 {
            real.wrapping_add(self.corr_range as i32)
        } else if (real as u32) >= self.corr_range {
            real.wrapping_sub(self.corr_range as i32)
        } else {
            real
        };
        Ok(real)
    }

    fn get_k(&self) -> u32 {
        self.k
    }
}

#[derive(Clone, Copy)]
struct StreamingMedian5 {
    values: [i32; 5],
    high: bool,
}

impl StreamingMedian5 {
    fn new() -> Self {
        StreamingMedian5 {
            values: [0; 5],
            high: true,
        }
    }

    fn init(&mut self) {
        self.values = [0; 5];
        self.high = true;
    }

    fn add(&mut self, v: i32) {
        if self.high {
            if v < self.values[2] {
                self.values[4] = self.values[3];
                self.values[3] = self.values[2];
                if v < self.values[0] {
                    self.values[2] = self.values[1];
                    self.values[1] = self.values[0];
                    self.values[0] = v;
                } else if v < self.values[1] {
                    self.values[2] = self.values[1];
                    self.values[1] = v;
                } else {
                    self.values[2] = v;
                }
            } else {
                if v < self.values[3] {
                    self.values[4] = self.values[3];
                    self.values[3] = v;
                } else {
                    self.values[4] = v;
                }
                self.high = false;
            }
        } else if self.values[2] < v {
            self.values[0] = self.values[1];
            self.values[1] = self.values[2];
            if self.values[4] < v {
                self.values[2] = self.values[3];
                self.values[3] = self.values[4];
                self.values[4] = v;
            } else if self.values[3] < v {
                self.values[2] = self.values[3];
                self.values[3] = v;
            } else {
                self.values[2] = v;
            }
        } else {
            if self.values[1] < v {
                self.values[0] = self.values[1];
                self.values[1] = v;
            } else {
                self.values[0] = v;
            }
            self.high = true;
        }
    }

    fn get(&self) -> i32 {
        self.values[2]
    }
}

const NUMBER_RETURN_MAP_6CTX: [[u8; 16]; 16] = [
    [0, 1, 2, 3, 4, 5, 3, 4, 4, 5, 5, 5, 5, 5, 5, 5],
    [1, 0, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
    [2, 1, 2, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3],
    [3, 3, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
    [4, 3, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
    [5, 3, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
    [3, 3, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4],
    [4, 3, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4],
    [4, 3, 4, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4],
    [5, 3, 4, 4, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4],
    [5, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4],
    [5, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 4, 4, 4],
    [5, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4],
    [5, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4],
    [5, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5],
    [5, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5],
];

const NUMBER_RETURN_LEVEL_8CTX: [[u8; 16]; 16] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7],
    [1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7, 7, 7, 7],
    [2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7, 7, 7],
    [3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7, 7],
    [4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7],
    [5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7],
    [6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7],
    [7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7],
    [7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7],
    [7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6],
    [7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5],
    [7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4],
    [7, 7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3],
    [7, 7, 7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2],
    [7, 7, 7, 7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1],
    [7, 7, 7, 7, 7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0],
];

const NUMBER_RETURN_MAP_3BIT: [[u8; 8]; 8] = [
    [15, 14, 13, 12, 11, 10, 9, 8],
    [14, 0, 1, 3, 6, 10, 10, 9],
    [13, 1, 2, 4, 7, 11, 11, 10],
    [12, 3, 4, 5, 8, 12, 12, 11],
    [11, 6, 7, 8, 9, 13, 13, 12],
    [10, 10, 11, 12, 13, 14, 14, 13],
    [9, 10, 11, 12, 13, 14, 15, 14],
    [8, 9, 10, 11, 12, 13, 14, 15],
];

const NUMBER_RETURN_LEVEL_3BIT: [[u8; 8]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [1, 0, 1, 2, 3, 4, 5, 6],
    [2, 1, 0, 1, 2, 3, 4, 5],
    [3, 2, 1, 0, 1, 2, 3, 4],
    [4, 3, 2, 1, 0, 1, 2, 3],
    [5, 4, 3, 2, 1, 0, 1, 2],
    [6, 5, 4, 3, 2, 1, 0, 1],
    [7, 6, 5, 4, 3, 2, 1, 0],
];

#[derive(Clone, Copy)]
struct Point14State {
    x: i32,
    y: i32,
    z: i32,
    intensity: u16,
    return_number: u8,
    number_of_returns: u8,
    scan_direction_flag: bool,
    edge_of_flight_line: bool,
    classification_flags: u8,
    scanner_channel: u8,
    classification: u8,
    scan_angle: i16,
    user_data: u8,
    point_source_id: u16,
    gps_time: f64,
    gps_time_change: bool,
}

impl Point14State {
    fn to_raw(&self, out: &mut [u8]) {
        out[0..4].copy_from_slice(&self.x.to_le_bytes());
        out[4..8].copy_from_slice(&self.y.to_le_bytes());
        out[8..12].copy_from_slice(&self.z.to_le_bytes());
        out[12..14].copy_from_slice(&self.intensity.to_le_bytes());
        out[14] = self.return_number | (self.number_of_returns << 4);
        out[15] = self.classification_flags
            | (self.scanner_channel << 4)
            | ((self.scan_direction_flag as u8) << 6)
            | ((self.edge_of_flight_line as u8) << 7);
        out[16] = self.classification;
        out[17] = self.user_data;
        out[18..20].copy_from_slice(&self.scan_angle.to_le_bytes());
        out[20..22].copy_from_slice(&self.point_source_id.to_le_bytes());
        out[22..30].copy_from_slice(&self.gps_time.to_le_bytes());
    }
}

fn point14_from_raw(b: &[u8]) -> Point14State {
    Point14State {
        x: i32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        y: i32::from_le_bytes([b[4], b[5], b[6], b[7]]),
        z: i32::from_le_bytes([b[8], b[9], b[10], b[11]]),
        intensity: u16::from_le_bytes([b[12], b[13]]),
        return_number: b[14] & 0x0F,
        number_of_returns: b[14] >> 4,
        classification_flags: b[15] & 0x0F,
        scanner_channel: (b[15] >> 4) & 0x03,
        scan_direction_flag: (b[15] >> 6) & 1 != 0,
        edge_of_flight_line: (b[15] >> 7) & 1 != 0,
        classification: b[16],
        user_data: b[17],
        scan_angle: le_i16(&b[18..20]),
        point_source_id: u16::from_le_bytes([b[20], b[21]]),
        gps_time: f64::from_le_bytes([b[22], b[23], b[24], b[25], b[26], b[27], b[28], b[29]]),
        gps_time_change: false,
    }
}

struct Point14Context {
    unused: bool,
    last_item: Point14State,
    last_intensity: [u16; 8],
    last_x_diff_median5: [StreamingMedian5; 12],
    last_y_diff_median5: [StreamingMedian5; 12],
    last_z: [i32; 8],
    m_changed_values: Vec<SymbolModel>,
    m_scanner_channel: SymbolModel,
    m_number_of_returns: Vec<SymbolModel>,
    m_return_number: Vec<SymbolModel>,
    m_return_number_gps_same: SymbolModel,
    ic_dx: IntegerCompressor,
    ic_dy: IntegerCompressor,
    ic_z: IntegerCompressor,
    m_classification: Vec<SymbolModel>,
    m_flags: Vec<SymbolModel>,
    m_user_data: Vec<SymbolModel>,
    ic_intensity: IntegerCompressor,
    ic_scan_angle: IntegerCompressor,
    ic_point_source_id: IntegerCompressor,
    last: u32,
    next: u32,
    last_gpstime: [i64; 4],
    last_gpstime_diff: [i32; 4],
    multi_extreme_counter: [i32; 4],
    m_gpstime_multi: SymbolModel,
    m_gpstime_0diff: SymbolModel,
    ic_gpstime: IntegerCompressor,
}

impl Point14Context {
    fn new() -> Self {
        Point14Context {
            unused: true,
            last_item: Point14State {
                x: 0,
                y: 0,
                z: 0,
                intensity: 0,
                return_number: 1,
                number_of_returns: 1,
                scan_direction_flag: false,
                edge_of_flight_line: false,
                classification_flags: 0,
                scanner_channel: 0,
                classification: 0,
                scan_angle: 0,
                user_data: 0,
                point_source_id: 0,
                gps_time: 0.0,
                gps_time_change: false,
            },
            last_intensity: [0; 8],
            last_x_diff_median5: [StreamingMedian5::new(); 12],
            last_y_diff_median5: [StreamingMedian5::new(); 12],
            last_z: [0; 8],
            m_changed_values: (0..8).map(|_| SymbolModel::new(128)).collect(),
            m_scanner_channel: SymbolModel::new(3),
            m_number_of_returns: (0..16).map(|_| SymbolModel::new(16)).collect(),
            m_return_number: (0..16).map(|_| SymbolModel::new(16)).collect(),
            m_return_number_gps_same: SymbolModel::new(13),
            ic_dx: IntegerCompressor::new(32, 2, 8),
            ic_dy: IntegerCompressor::new(32, 22, 8),
            ic_z: IntegerCompressor::new(32, 20, 8),
            m_classification: (0..64).map(|_| SymbolModel::new(256)).collect(),
            m_flags: (0..64).map(|_| SymbolModel::new(64)).collect(),
            m_user_data: (0..64).map(|_| SymbolModel::new(256)).collect(),
            ic_intensity: IntegerCompressor::new(16, 4, 8),
            ic_scan_angle: IntegerCompressor::new(16, 2, 8),
            ic_point_source_id: IntegerCompressor::new(16, 1, 8),
            last: 0,
            next: 0,
            last_gpstime: [0; 4],
            last_gpstime_diff: [0; 4],
            multi_extreme_counter: [0; 4],
            m_gpstime_multi: SymbolModel::new(LASZIP_GPSTIME_MULTI_TOTAL),
            m_gpstime_0diff: SymbolModel::new(5),
            ic_gpstime: IntegerCompressor::new(32, 9, 8),
        }
    }

    fn init(&mut self, first: &Point14State) {
        for m in &mut self.m_changed_values {
            m.init();
        }
        self.m_scanner_channel.init();
        for m in &mut self.m_number_of_returns {
            m.init();
        }
        for m in &mut self.m_return_number {
            m.init();
        }
        self.m_return_number_gps_same.init();
        self.ic_dx.init();
        self.ic_dy.init();
        for m in &mut self.last_x_diff_median5 {
            m.init();
        }
        for m in &mut self.last_y_diff_median5 {
            m.init();
        }
        self.ic_z.init();
        for z in &mut self.last_z {
            *z = first.z;
        }
        for m in &mut self.m_classification {
            m.init();
        }
        for m in &mut self.m_flags {
            m.init();
        }
        for m in &mut self.m_user_data {
            m.init();
        }
        self.ic_intensity.init();
        for i in &mut self.last_intensity {
            *i = first.intensity;
        }
        self.ic_scan_angle.init();
        self.ic_point_source_id.init();
        self.m_gpstime_multi.init();
        self.m_gpstime_0diff.init();
        self.ic_gpstime.init();
        self.last = 0;
        self.next = 0;
        self.last_gpstime_diff = [0; 4];
        self.multi_extreme_counter = [0; 4];
        self.last_gpstime[0] = first.gps_time.to_bits() as i64;
        self.last_gpstime[1] = 0;
        self.last_gpstime[2] = 0;
        self.last_gpstime[3] = 0;
        self.last_item = *first;
        self.last_item.gps_time_change = false;
        self.unused = false;
    }
}

struct ChangedFlags {
    z: bool,
    classification: bool,
    flags: bool,
    intensity: bool,
    scan_angle: bool,
    user_data: bool,
    point_source: bool,
    gps_time: bool,
}

struct Point14Decoders {
    dec_xy: AcDecoder,
    dec_z: Option<AcDecoder>,
    dec_classification: Option<AcDecoder>,
    dec_flags: Option<AcDecoder>,
    dec_intensity: Option<AcDecoder>,
    dec_scan_angle: Option<AcDecoder>,
    dec_user_data: Option<AcDecoder>,
    dec_point_source: Option<AcDecoder>,
    dec_gps_time: Option<AcDecoder>,
}

struct Point14Reader {
    contexts: [Point14Context; 4],
    current_context: usize,
}

fn read_gps_time(ctx: &mut Point14Context, dec: &mut AcDecoder) -> Result<(), LasNote> {
    loop {
        if ctx.last_gpstime_diff[ctx.last as usize] == 0 {
            let multi = dec.decode_symbol(&mut ctx.m_gpstime_0diff)? as i32;
            if multi == 0 {
                let diff = ctx.ic_gpstime.decompress(dec, 0, 0)?;
                ctx.last_gpstime_diff[ctx.last as usize] = diff;
                ctx.last_gpstime[ctx.last as usize] =
                    ctx.last_gpstime[ctx.last as usize].wrapping_add(diff as i64);
                ctx.multi_extreme_counter[ctx.last as usize] = 0;
            } else if multi == 1 {
                ctx.next = (ctx.next + 1) & 3;
                let high = ctx.ic_gpstime.decompress(
                    dec,
                    (ctx.last_gpstime[ctx.last as usize] as u64 >> 32) as i32,
                    8,
                )?;
                let low = dec.read_int()?;
                ctx.last_gpstime[ctx.next as usize] = (((high as u64) << 32) | low as u64) as i64;
                ctx.last = ctx.next;
                ctx.last_gpstime_diff[ctx.last as usize] = 0;
                ctx.multi_extreme_counter[ctx.last as usize] = 0;
            } else {
                ctx.last = (ctx.last + multi as u32 - 1) & 3;
                continue;
            }
        } else {
            let multi = dec.decode_symbol(&mut ctx.m_gpstime_multi)? as i32;
            if multi == 1 {
                let diff =
                    ctx.ic_gpstime
                        .decompress(dec, ctx.last_gpstime_diff[ctx.last as usize], 1)?;
                ctx.last_gpstime[ctx.last as usize] =
                    ctx.last_gpstime[ctx.last as usize].wrapping_add(diff as i64);
                ctx.multi_extreme_counter[ctx.last as usize] = 0;
            } else if multi < LASZIP_GPSTIME_MULTI_CODE_FULL {
                let gpstime_diff;
                if multi == 0 {
                    gpstime_diff = ctx.ic_gpstime.decompress(dec, 0, 7)?;
                    ctx.multi_extreme_counter[ctx.last as usize] += 1;
                    if ctx.multi_extreme_counter[ctx.last as usize] > 3 {
                        ctx.last_gpstime_diff[ctx.last as usize] = gpstime_diff;
                        ctx.multi_extreme_counter[ctx.last as usize] = 0;
                    }
                } else if multi < LASZIP_GPSTIME_MULTI {
                    if multi < 10 {
                        gpstime_diff = ctx.ic_gpstime.decompress(
                            dec,
                            multi.wrapping_mul(ctx.last_gpstime_diff[ctx.last as usize]),
                            2,
                        )?;
                    } else {
                        gpstime_diff = ctx.ic_gpstime.decompress(
                            dec,
                            multi.wrapping_mul(ctx.last_gpstime_diff[ctx.last as usize]),
                            3,
                        )?;
                    }
                } else if multi == LASZIP_GPSTIME_MULTI {
                    gpstime_diff = ctx.ic_gpstime.decompress(
                        dec,
                        LASZIP_GPSTIME_MULTI.wrapping_mul(ctx.last_gpstime_diff[ctx.last as usize]),
                        4,
                    )?;
                    ctx.multi_extreme_counter[ctx.last as usize] += 1;
                    if ctx.multi_extreme_counter[ctx.last as usize] > 3 {
                        ctx.last_gpstime_diff[ctx.last as usize] = gpstime_diff;
                        ctx.multi_extreme_counter[ctx.last as usize] = 0;
                    }
                } else {
                    let m = LASZIP_GPSTIME_MULTI - multi;
                    if m > LASZIP_GPSTIME_MULTI_MINUS {
                        gpstime_diff = ctx.ic_gpstime.decompress(
                            dec,
                            m.wrapping_mul(ctx.last_gpstime_diff[ctx.last as usize]),
                            5,
                        )?;
                    } else {
                        gpstime_diff = ctx.ic_gpstime.decompress(
                            dec,
                            LASZIP_GPSTIME_MULTI_MINUS
                                .wrapping_mul(ctx.last_gpstime_diff[ctx.last as usize]),
                            6,
                        )?;
                        ctx.multi_extreme_counter[ctx.last as usize] += 1;
                        if ctx.multi_extreme_counter[ctx.last as usize] > 3 {
                            ctx.last_gpstime_diff[ctx.last as usize] = gpstime_diff;
                            ctx.multi_extreme_counter[ctx.last as usize] = 0;
                        }
                    }
                }
                ctx.last_gpstime[ctx.last as usize] =
                    ctx.last_gpstime[ctx.last as usize].wrapping_add(gpstime_diff as i64);
            } else if multi == LASZIP_GPSTIME_MULTI_CODE_FULL {
                ctx.next = (ctx.next + 1) & 3;
                let high = ctx.ic_gpstime.decompress(
                    dec,
                    (ctx.last_gpstime[ctx.last as usize] as u64 >> 32) as i32,
                    8,
                )?;
                let low = dec.read_int()?;
                ctx.last_gpstime[ctx.next as usize] = (((high as u64) << 32) | low as u64) as i64;
                ctx.last = ctx.next;
                ctx.last_gpstime_diff[ctx.last as usize] = 0;
                ctx.multi_extreme_counter[ctx.last as usize] = 0;
            } else {
                ctx.last = (ctx.last + multi as u32 - LASZIP_GPSTIME_MULTI_CODE_FULL as u32) & 3;
                continue;
            }
        }
        break;
    }
    Ok(())
}

impl Point14Reader {
    fn read(
        &mut self,
        dec: &mut Point14Decoders,
        changed: &ChangedFlags,
    ) -> Result<Point14State, LasNote> {
        let mut last = self.contexts[self.current_context].last_item;

        let mut lpr = if last.return_number == 1 { 1 } else { 0 };
        lpr += if last.return_number >= last.number_of_returns {
            2
        } else {
            0
        };
        lpr += if last.gps_time_change { 4 } else { 0 };

        let changed_values = {
            let ctx = &mut self.contexts[self.current_context];
            dec.dec_xy.decode_symbol(&mut ctx.m_changed_values[lpr])?
        };

        if changed_values & (1 << 6) != 0 {
            let diff = {
                let ctx = &mut self.contexts[self.current_context];
                dec.dec_xy.decode_symbol(&mut ctx.m_scanner_channel)?
            };
            let scanner_channel = ((self.current_context as u32 + diff + 1) % 4) as usize;
            if self.contexts[scanner_channel].unused {
                let first = self.contexts[self.current_context].last_item;
                self.contexts[scanner_channel].init(&first);
            }
            self.current_context = scanner_channel;
            last = self.contexts[self.current_context].last_item;
            last.scanner_channel = scanner_channel as u8;
        }

        let point_source_change = changed_values & (1 << 5) != 0;
        let gps_time_change = changed_values & (1 << 4) != 0;
        let scan_angle_change = changed_values & (1 << 3) != 0;

        let last_n = last.number_of_returns;
        let last_r = last.return_number;

        let n = if changed_values & (1 << 2) != 0 {
            let ctx = &mut self.contexts[self.current_context];
            let sym = dec
                .dec_xy
                .decode_symbol(&mut ctx.m_number_of_returns[last_n as usize])?;
            sym as u8
        } else {
            last_n
        };
        last.number_of_returns = n;

        let r = match changed_values & 3 {
            0 => last_r,
            1 => (last_r + 1) % 16,
            2 => (last_r + 15) % 16,
            _ => {
                if gps_time_change {
                    let ctx = &mut self.contexts[self.current_context];
                    let sym = dec
                        .dec_xy
                        .decode_symbol(&mut ctx.m_return_number[last_r as usize])?;
                    sym as u8
                } else {
                    let ctx = &mut self.contexts[self.current_context];
                    let sym = dec
                        .dec_xy
                        .decode_symbol(&mut ctx.m_return_number_gps_same)?;
                    ((last_r as u32 + (sym + 2)) % 16) as u8
                }
            }
        };
        last.return_number = r;

        let m = NUMBER_RETURN_MAP_6CTX[n as usize][r as usize] as usize;
        let l = NUMBER_RETURN_LEVEL_8CTX[n as usize][r as usize] as usize;

        let cpr = if r == 1 { 2 } else { 0 } + if r >= n { 1 } else { 0 };

        let gps_time_change_bit = gps_time_change as usize;

        {
            let ctx = &mut self.contexts[self.current_context];
            let median = ctx.last_x_diff_median5[(m << 1) | gps_time_change_bit].get();
            let diff = ctx
                .ic_dx
                .decompress(&mut dec.dec_xy, median, (n == 1) as u32)?;
            last.x = last.x.wrapping_add(diff);
            ctx.last_x_diff_median5[(m << 1) | gps_time_change_bit].add(diff);

            let median = ctx.last_y_diff_median5[(m << 1) | gps_time_change_bit].get();
            let k_bits = ctx.ic_dx.get_k();
            let ctx_y = (n == 1) as u32 + if k_bits < 20 { zero_bit_0(k_bits) } else { 20 };
            let diff = ctx.ic_dy.decompress(&mut dec.dec_xy, median, ctx_y)?;
            last.y = last.y.wrapping_add(diff);
            ctx.last_y_diff_median5[(m << 1) | gps_time_change_bit].add(diff);

            if changed.z {
                let k_bits = (ctx.ic_dx.get_k() + ctx.ic_dy.get_k()) / 2;
                let ctx_z = (n == 1) as u32 + if k_bits < 18 { zero_bit_0(k_bits) } else { 18 };
                let z = ctx
                    .ic_z
                    .decompress(dec.dec_z.as_mut().unwrap(), ctx.last_z[l], ctx_z)?;
                ctx.last_z[l] = z;
                last.z = z;
            }
        }

        if changed.classification {
            let ctx = &mut self.contexts[self.current_context];
            let last_classification = last.classification;
            let ccc = ((last_classification & 0x1F) as usize) << 1 | if cpr == 3 { 1 } else { 0 };
            let sym = dec
                .dec_classification
                .as_mut()
                .unwrap()
                .decode_symbol(&mut ctx.m_classification[ccc])?;
            last.classification = sym as u8;
        }

        if changed.flags {
            let ctx = &mut self.contexts[self.current_context];
            let last_flags = ((last.edge_of_flight_line as u8) << 5)
                | ((last.scan_direction_flag as u8) << 4)
                | last.classification_flags;
            let flags = dec
                .dec_flags
                .as_mut()
                .unwrap()
                .decode_symbol(&mut ctx.m_flags[last_flags as usize])?;
            last.edge_of_flight_line = flags & (1 << 5) != 0;
            last.scan_direction_flag = flags & (1 << 4) != 0;
            last.classification_flags = (flags & 0x0F) as u8;
        }

        if changed.intensity {
            let ctx = &mut self.contexts[self.current_context];
            let intensity = ctx.ic_intensity.decompress(
                dec.dec_intensity.as_mut().unwrap(),
                ctx.last_intensity[(cpr << 1) | gps_time_change_bit] as i32,
                cpr as u32,
            )?;
            ctx.last_intensity[(cpr << 1) | gps_time_change_bit] = intensity as u16;
            last.intensity = intensity as u16;
        }

        if changed.scan_angle && scan_angle_change {
            let ctx = &mut self.contexts[self.current_context];
            let angle = ctx.ic_scan_angle.decompress(
                dec.dec_scan_angle.as_mut().unwrap(),
                last.scan_angle as i32,
                gps_time_change as u32,
            )?;
            last.scan_angle = angle as i16;
        }

        if changed.user_data {
            let ctx = &mut self.contexts[self.current_context];
            let idx = (last.user_data / 4) as usize;
            let sym = dec
                .dec_user_data
                .as_mut()
                .unwrap()
                .decode_symbol(&mut ctx.m_user_data[idx])?;
            last.user_data = sym as u8;
        }

        if changed.point_source && point_source_change {
            let ctx = &mut self.contexts[self.current_context];
            let ps = ctx.ic_point_source_id.decompress(
                dec.dec_point_source.as_mut().unwrap(),
                last.point_source_id as i32,
                0,
            )?;
            last.point_source_id = ps as u16;
        }

        if changed.gps_time && gps_time_change {
            let ctx = &mut self.contexts[self.current_context];
            read_gps_time(ctx, dec.dec_gps_time.as_mut().unwrap())?;
            last.gps_time = f64::from_bits(ctx.last_gpstime[ctx.last as usize] as u64);
        }

        last.gps_time_change = gps_time_change;
        self.contexts[self.current_context].last_item = last;
        Ok(last)
    }
}

fn u8_fold(v: i32) -> u8 {
    if v < 0 {
        (v + 256) as u8
    } else if v > 255 {
        (v - 256) as u8
    } else {
        v as u8
    }
}

fn u8_clamp(v: i32) -> u8 {
    if v <= 0 {
        0
    } else if v >= 255 {
        255
    } else {
        v as u8
    }
}

const GPSTIME11_MULTI: i32 = 500;
const GPSTIME11_MULTI_MINUS: i32 = -10;
const GPSTIME11_MULTI_UNCHANGED: i32 = GPSTIME11_MULTI - GPSTIME11_MULTI_MINUS + 1;
const GPSTIME11_MULTI_CODE_FULL: i32 = GPSTIME11_MULTI - GPSTIME11_MULTI_MINUS + 2;
const GPSTIME11_MULTI_TOTAL: u32 = (GPSTIME11_MULTI - GPSTIME11_MULTI_MINUS + 6) as u32;

struct Point10Reader {
    last_item: [u8; 20],
    last_intensity: [u16; 16],
    last_x_diff_median5: [StreamingMedian5; 16],
    last_y_diff_median5: [StreamingMedian5; 16],
    last_height: [i32; 8],
    m_changed_values: SymbolModel,
    ic_intensity: IntegerCompressor,
    m_scan_angle_rank: [SymbolModel; 2],
    ic_point_source_id: IntegerCompressor,
    m_bit_byte: Vec<Option<SymbolModel>>,
    m_classification: Vec<Option<SymbolModel>>,
    m_user_data: Vec<Option<SymbolModel>>,
    ic_dx: IntegerCompressor,
    ic_dy: IntegerCompressor,
    ic_z: IntegerCompressor,
}

impl Point10Reader {
    fn new() -> Self {
        Point10Reader {
            last_item: [0; 20],
            last_intensity: [0; 16],
            last_x_diff_median5: [StreamingMedian5::new(); 16],
            last_y_diff_median5: [StreamingMedian5::new(); 16],
            last_height: [0; 8],
            m_changed_values: SymbolModel::new(64),
            ic_intensity: IntegerCompressor::new(16, 4, 8),
            m_scan_angle_rank: [SymbolModel::new(256), SymbolModel::new(256)],
            ic_point_source_id: IntegerCompressor::new(16, 1, 8),
            m_bit_byte: (0..256).map(|_| None).collect(),
            m_classification: (0..256).map(|_| None).collect(),
            m_user_data: (0..256).map(|_| None).collect(),
            ic_dx: IntegerCompressor::new(32, 2, 8),
            ic_dy: IntegerCompressor::new(32, 22, 8),
            ic_z: IntegerCompressor::new(32, 20, 8),
        }
    }

    fn init(&mut self, item: &[u8; 20]) {
        for m in &mut self.last_x_diff_median5 {
            m.init();
        }
        for m in &mut self.last_y_diff_median5 {
            m.init();
        }
        self.last_intensity = [0; 16];
        self.last_height = [0; 8];
        self.m_changed_values.init();
        self.ic_intensity.init();
        self.m_scan_angle_rank[0].init();
        self.m_scan_angle_rank[1].init();
        self.ic_point_source_id.init();
        for m in self.m_bit_byte.iter_mut().flatten() {
            m.init();
        }
        for m in self.m_classification.iter_mut().flatten() {
            m.init();
        }
        for m in self.m_user_data.iter_mut().flatten() {
            m.init();
        }
        self.ic_dx.init();
        self.ic_dy.init();
        self.ic_z.init();
        self.last_item = *item;
        self.last_item[12] = 0;
        self.last_item[13] = 0;
    }

    fn read(&mut self, dec: &mut AcDecoder) -> Result<[u8; 20], LasNote> {
        let changed = dec.decode_symbol(&mut self.m_changed_values)?;

        let (m, l) = if changed != 0 {
            if changed & 32 != 0 {
                let idx = self.last_item[14] as usize;
                if self.m_bit_byte[idx].is_none() {
                    self.m_bit_byte[idx] = Some(SymbolModel::new(256));
                    self.m_bit_byte[idx].as_mut().unwrap().init();
                }
                let sym = dec.decode_symbol(self.m_bit_byte[idx].as_mut().unwrap())?;
                self.last_item[14] = sym as u8;
            }
            let r = (self.last_item[14] & 0x07) as usize;
            let n = ((self.last_item[14] >> 3) & 0x07) as usize;
            let m = NUMBER_RETURN_MAP_3BIT[n][r] as usize;
            let l = NUMBER_RETURN_LEVEL_3BIT[n][r] as usize;

            if changed & 16 != 0 {
                let ctx = if m < 3 { m as u32 } else { 3 };
                let v = self
                    .ic_intensity
                    .decompress(dec, self.last_intensity[m] as i32, ctx)?
                    as u16;
                self.last_item[12] = (v & 0xFF) as u8;
                self.last_item[13] = (v >> 8) as u8;
                self.last_intensity[m] = v;
            } else {
                let v = self.last_intensity[m];
                self.last_item[12] = (v & 0xFF) as u8;
                self.last_item[13] = (v >> 8) as u8;
            }

            if changed & 8 != 0 {
                let idx = self.last_item[15] as usize;
                if self.m_classification[idx].is_none() {
                    self.m_classification[idx] = Some(SymbolModel::new(256));
                    self.m_classification[idx].as_mut().unwrap().init();
                }
                let sym = dec.decode_symbol(self.m_classification[idx].as_mut().unwrap())?;
                self.last_item[15] = sym as u8;
            }

            if changed & 4 != 0 {
                let scan_dir = (self.last_item[14] >> 6) & 1;
                let sym = dec.decode_symbol(&mut self.m_scan_angle_rank[scan_dir as usize])? as i32;
                self.last_item[16] = u8_fold(sym + self.last_item[16] as i32);
            }

            if changed & 2 != 0 {
                let idx = self.last_item[17] as usize;
                if self.m_user_data[idx].is_none() {
                    self.m_user_data[idx] = Some(SymbolModel::new(256));
                    self.m_user_data[idx].as_mut().unwrap().init();
                }
                let sym = dec.decode_symbol(self.m_user_data[idx].as_mut().unwrap())?;
                self.last_item[17] = sym as u8;
            }

            if changed & 1 != 0 {
                let pred = u16::from_le_bytes([self.last_item[18], self.last_item[19]]);
                let v = self.ic_point_source_id.decompress(dec, pred as i32, 0)? as u16;
                self.last_item[18] = (v & 0xFF) as u8;
                self.last_item[19] = (v >> 8) as u8;
            }
            (m, l)
        } else {
            let r = (self.last_item[14] & 0x07) as usize;
            let n = ((self.last_item[14] >> 3) & 0x07) as usize;
            (
                NUMBER_RETURN_MAP_3BIT[n][r] as usize,
                NUMBER_RETURN_LEVEL_3BIT[n][r] as usize,
            )
        };

        let n = ((self.last_item[14] >> 3) & 0x07) as usize;

        let median = self.last_x_diff_median5[m].get();
        let diff = self.ic_dx.decompress(dec, median, (n == 1) as u32)?;
        let x = i32::from_le_bytes([
            self.last_item[0],
            self.last_item[1],
            self.last_item[2],
            self.last_item[3],
        ])
        .wrapping_add(diff);
        self.last_item[0..4].copy_from_slice(&x.to_le_bytes());
        self.last_x_diff_median5[m].add(diff);

        let median = self.last_y_diff_median5[m].get();
        let k_bits = self.ic_dx.get_k();
        let ctx = (n == 1) as u32 + if k_bits < 20 { zero_bit_0(k_bits) } else { 20 };
        let diff = self.ic_dy.decompress(dec, median, ctx)?;
        let y = i32::from_le_bytes([
            self.last_item[4],
            self.last_item[5],
            self.last_item[6],
            self.last_item[7],
        ])
        .wrapping_add(diff);
        self.last_item[4..8].copy_from_slice(&y.to_le_bytes());
        self.last_y_diff_median5[m].add(diff);

        let k_bits = (self.ic_dx.get_k() + self.ic_dy.get_k()) / 2;
        let ctx = (n == 1) as u32 + if k_bits < 18 { zero_bit_0(k_bits) } else { 18 };
        let z = self.ic_z.decompress(dec, self.last_height[l], ctx)?;
        self.last_height[l] = z;
        self.last_item[8..12].copy_from_slice(&z.to_le_bytes());

        Ok(self.last_item)
    }
}

struct GpsTime11Reader {
    last: u32,
    next: u32,
    last_gpstime: [i64; 4],
    last_gpstime_diff: [i32; 4],
    multi_extreme_counter: [i32; 4],
    m_gpstime_multi: SymbolModel,
    m_gpstime_0diff: SymbolModel,
    ic_gpstime: IntegerCompressor,
}

impl GpsTime11Reader {
    fn new() -> Self {
        GpsTime11Reader {
            last: 0,
            next: 0,
            last_gpstime: [0; 4],
            last_gpstime_diff: [0; 4],
            multi_extreme_counter: [0; 4],
            m_gpstime_multi: SymbolModel::new(GPSTIME11_MULTI_TOTAL),
            m_gpstime_0diff: SymbolModel::new(6),
            ic_gpstime: IntegerCompressor::new(32, 9, 8),
        }
    }

    fn init(&mut self, item: &[u8; 8]) {
        self.last = 0;
        self.next = 0;
        self.last_gpstime_diff = [0; 4];
        self.multi_extreme_counter = [0; 4];
        self.m_gpstime_multi.init();
        self.m_gpstime_0diff.init();
        self.ic_gpstime.init();
        self.last_gpstime[0] = i64::from_le_bytes(*item);
        self.last_gpstime[1] = 0;
        self.last_gpstime[2] = 0;
        self.last_gpstime[3] = 0;
    }

    fn read(&mut self, dec: &mut AcDecoder) -> Result<i64, LasNote> {
        loop {
            if self.last_gpstime_diff[self.last as usize] == 0 {
                let multi = dec.decode_symbol(&mut self.m_gpstime_0diff)? as i32;
                if multi == 1 {
                    let diff = self.ic_gpstime.decompress(dec, 0, 0)?;
                    self.last_gpstime_diff[self.last as usize] = diff;
                    self.last_gpstime[self.last as usize] += diff as i64;
                    self.multi_extreme_counter[self.last as usize] = 0;
                } else if multi == 2 {
                    self.next = (self.next + 1) & 3;
                    let high = self.ic_gpstime.decompress(
                        dec,
                        (self.last_gpstime[self.last as usize] as u64 >> 32) as i32,
                        8,
                    )?;
                    let low = dec.read_int()?;
                    self.last_gpstime[self.next as usize] =
                        (((high as u64) << 32) | low as u64) as i64;
                    self.last = self.next;
                    self.last_gpstime_diff[self.last as usize] = 0;
                    self.multi_extreme_counter[self.last as usize] = 0;
                } else if multi > 2 {
                    self.last = (self.last + multi as u32 - 2) & 3;
                    continue;
                }
            } else {
                let multi = dec.decode_symbol(&mut self.m_gpstime_multi)? as i32;
                if multi == 1 {
                    let diff = self.ic_gpstime.decompress(
                        dec,
                        self.last_gpstime_diff[self.last as usize],
                        1,
                    )?;
                    self.last_gpstime[self.last as usize] += diff as i64;
                    self.multi_extreme_counter[self.last as usize] = 0;
                } else if multi < GPSTIME11_MULTI_UNCHANGED {
                    let gpstime_diff;
                    if multi == 0 {
                        gpstime_diff = self.ic_gpstime.decompress(dec, 0, 7)?;
                        self.multi_extreme_counter[self.last as usize] += 1;
                        if self.multi_extreme_counter[self.last as usize] > 3 {
                            self.last_gpstime_diff[self.last as usize] = gpstime_diff;
                            self.multi_extreme_counter[self.last as usize] = 0;
                        }
                    } else if multi < GPSTIME11_MULTI {
                        if multi < 10 {
                            gpstime_diff = self.ic_gpstime.decompress(
                                dec,
                                multi.wrapping_mul(self.last_gpstime_diff[self.last as usize]),
                                2,
                            )?;
                        } else {
                            gpstime_diff = self.ic_gpstime.decompress(
                                dec,
                                multi.wrapping_mul(self.last_gpstime_diff[self.last as usize]),
                                3,
                            )?;
                        }
                    } else if multi == GPSTIME11_MULTI {
                        gpstime_diff = self.ic_gpstime.decompress(
                            dec,
                            GPSTIME11_MULTI
                                .wrapping_mul(self.last_gpstime_diff[self.last as usize]),
                            4,
                        )?;
                        self.multi_extreme_counter[self.last as usize] += 1;
                        if self.multi_extreme_counter[self.last as usize] > 3 {
                            self.last_gpstime_diff[self.last as usize] = gpstime_diff;
                            self.multi_extreme_counter[self.last as usize] = 0;
                        }
                    } else {
                        let m = GPSTIME11_MULTI - multi;
                        if m > GPSTIME11_MULTI_MINUS {
                            gpstime_diff = self.ic_gpstime.decompress(
                                dec,
                                m.wrapping_mul(self.last_gpstime_diff[self.last as usize]),
                                5,
                            )?;
                        } else {
                            gpstime_diff = self.ic_gpstime.decompress(
                                dec,
                                GPSTIME11_MULTI_MINUS
                                    .wrapping_mul(self.last_gpstime_diff[self.last as usize]),
                                6,
                            )?;
                            self.multi_extreme_counter[self.last as usize] += 1;
                            if self.multi_extreme_counter[self.last as usize] > 3 {
                                self.last_gpstime_diff[self.last as usize] = gpstime_diff;
                                self.multi_extreme_counter[self.last as usize] = 0;
                            }
                        }
                    }
                    self.last_gpstime[self.last as usize] += gpstime_diff as i64;
                } else if multi == GPSTIME11_MULTI_CODE_FULL {
                    self.next = (self.next + 1) & 3;
                    let high = self.ic_gpstime.decompress(
                        dec,
                        (self.last_gpstime[self.last as usize] as u64 >> 32) as i32,
                        8,
                    )?;
                    let low = dec.read_int()?;
                    self.last_gpstime[self.next as usize] =
                        (((high as u64) << 32) | low as u64) as i64;
                    self.last = self.next;
                    self.last_gpstime_diff[self.last as usize] = 0;
                    self.multi_extreme_counter[self.last as usize] = 0;
                } else if multi >= GPSTIME11_MULTI_CODE_FULL {
                    self.last = (self.last + multi as u32 - GPSTIME11_MULTI_CODE_FULL as u32) & 3;
                    continue;
                }
            }
            break;
        }
        Ok(self.last_gpstime[self.last as usize])
    }
}

struct Rgb12Reader {
    last_item: [u16; 3],
    m_byte_used: SymbolModel,
    m_rgb_diff: [SymbolModel; 6],
}

impl Rgb12Reader {
    fn new() -> Self {
        Rgb12Reader {
            last_item: [0; 3],
            m_byte_used: SymbolModel::new(128),
            m_rgb_diff: [
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
            ],
        }
    }

    fn init(&mut self, item: &[u8; 6]) {
        self.m_byte_used.init();
        for m in &mut self.m_rgb_diff {
            m.init();
        }
        self.last_item = [
            u16::from_le_bytes([item[0], item[1]]),
            u16::from_le_bytes([item[2], item[3]]),
            u16::from_le_bytes([item[4], item[5]]),
        ];
    }

    fn read(&mut self, dec: &mut AcDecoder) -> Result<[u16; 3], LasNote> {
        let sym = dec.decode_symbol(&mut self.m_byte_used)?;
        let mut item = [0u16; 3];
        if sym & (1 << 0) != 0 {
            let corr = dec.decode_symbol(&mut self.m_rgb_diff[0])? as i32;
            item[0] = u8_fold(corr + (self.last_item[0] & 255) as i32) as u16;
        } else {
            item[0] = self.last_item[0] & 0xFF;
        }
        if sym & (1 << 1) != 0 {
            let corr = dec.decode_symbol(&mut self.m_rgb_diff[1])? as i32;
            item[0] |= (u8_fold(corr + (self.last_item[0] >> 8) as i32) as u16) << 8;
        } else {
            item[0] |= self.last_item[0] & 0xFF00;
        }
        if sym & (1 << 6) != 0 {
            let mut diff = (item[0] & 0x00FF) as i32 - (self.last_item[0] & 0x00FF) as i32;
            if sym & (1 << 2) != 0 {
                let corr = dec.decode_symbol(&mut self.m_rgb_diff[2])? as i32;
                item[1] =
                    u8_fold(corr + u8_clamp(diff + (self.last_item[1] & 255) as i32) as i32) as u16;
            } else {
                item[1] = self.last_item[1] & 0xFF;
            }
            if sym & (1 << 4) != 0 {
                let corr = dec.decode_symbol(&mut self.m_rgb_diff[4])? as i32;
                diff =
                    (diff + ((item[1] & 0x00FF) as i32 - (self.last_item[1] & 0x00FF) as i32)) / 2;
                item[2] =
                    u8_fold(corr + u8_clamp(diff + (self.last_item[2] & 255) as i32) as i32) as u16;
            } else {
                item[2] = self.last_item[2] & 0xFF;
            }
            diff = (item[0] >> 8) as i32 - (self.last_item[0] >> 8) as i32;
            if sym & (1 << 3) != 0 {
                let corr = dec.decode_symbol(&mut self.m_rgb_diff[3])? as i32;
                item[1] |= (u8_fold(corr + u8_clamp(diff + (self.last_item[1] >> 8) as i32) as i32)
                    as u16)
                    << 8;
            } else {
                item[1] |= self.last_item[1] & 0xFF00;
            }
            if sym & (1 << 5) != 0 {
                let corr = dec.decode_symbol(&mut self.m_rgb_diff[5])? as i32;
                diff = (diff + ((item[1] >> 8) as i32 - (self.last_item[1] >> 8) as i32)) / 2;
                item[2] |= (u8_fold(corr + u8_clamp(diff + (self.last_item[2] >> 8) as i32) as i32)
                    as u16)
                    << 8;
            } else {
                item[2] |= self.last_item[2] & 0xFF00;
            }
        } else {
            item[1] = item[0];
            item[2] = item[0];
        }
        self.last_item = item;
        Ok(item)
    }
}

#[derive(Clone, Copy)]
struct WavePacket13 {
    offset: u64,
    packet_size: u32,
    return_point: i32,
    x: i32,
    y: i32,
    z: i32,
}

fn wavepacket13_unpack(b: &[u8]) -> WavePacket13 {
    WavePacket13 {
        offset: u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
        packet_size: u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
        return_point: i32::from_le_bytes([b[12], b[13], b[14], b[15]]),
        x: i32::from_le_bytes([b[16], b[17], b[18], b[19]]),
        y: i32::from_le_bytes([b[20], b[21], b[22], b[23]]),
        z: i32::from_le_bytes([b[24], b[25], b[26], b[27]]),
    }
}

fn wavepacket13_pack(w: &WavePacket13, out: &mut [u8]) {
    out[0..8].copy_from_slice(&w.offset.to_le_bytes());
    out[8..12].copy_from_slice(&w.packet_size.to_le_bytes());
    out[12..16].copy_from_slice(&w.return_point.to_le_bytes());
    out[16..20].copy_from_slice(&w.x.to_le_bytes());
    out[20..24].copy_from_slice(&w.y.to_le_bytes());
    out[24..28].copy_from_slice(&w.z.to_le_bytes());
}

struct Wavepacket13Reader {
    last_item: [u8; 28],
    last_diff_32: i32,
    sym_last_offset_diff: u32,
    m_packet_index: SymbolModel,
    m_offset_diff: [SymbolModel; 4],
    ic_offset_diff: IntegerCompressor,
    ic_packet_size: IntegerCompressor,
    ic_return_point: IntegerCompressor,
    ic_xyz: IntegerCompressor,
}

impl Wavepacket13Reader {
    fn new() -> Self {
        Wavepacket13Reader {
            last_item: [0; 28],
            last_diff_32: 0,
            sym_last_offset_diff: 0,
            m_packet_index: SymbolModel::new(256),
            m_offset_diff: [
                SymbolModel::new(4),
                SymbolModel::new(4),
                SymbolModel::new(4),
                SymbolModel::new(4),
            ],
            ic_offset_diff: IntegerCompressor::new(32, 1, 8),
            ic_packet_size: IntegerCompressor::new(32, 1, 8),
            ic_return_point: IntegerCompressor::new(32, 1, 8),
            ic_xyz: IntegerCompressor::new(32, 3, 8),
        }
    }

    fn init(&mut self, item: &[u8; 29]) {
        self.last_diff_32 = 0;
        self.sym_last_offset_diff = 0;
        self.m_packet_index.init();
        for m in &mut self.m_offset_diff {
            m.init();
        }
        self.ic_offset_diff.init();
        self.ic_packet_size.init();
        self.ic_return_point.init();
        self.ic_xyz.init();
        self.last_item.copy_from_slice(&item[1..29]);
    }

    fn read(&mut self, dec: &mut AcDecoder) -> Result<[u8; 29], LasNote> {
        let mut out = [0u8; 29];
        out[0] = dec.decode_symbol(&mut self.m_packet_index)? as u8;
        let last_m = wavepacket13_unpack(&self.last_item);
        let mut this_m = WavePacket13 {
            offset: 0,
            packet_size: 0,
            return_point: 0,
            x: 0,
            y: 0,
            z: 0,
        };
        self.sym_last_offset_diff =
            dec.decode_symbol(&mut self.m_offset_diff[self.sym_last_offset_diff as usize])?;
        match self.sym_last_offset_diff {
            0 => this_m.offset = last_m.offset,
            1 => this_m.offset = last_m.offset + last_m.packet_size as u64,
            2 => {
                self.last_diff_32 = self.ic_offset_diff.decompress(dec, self.last_diff_32, 0)?;
                this_m.offset =
                    (last_m.offset as i64).wrapping_add(self.last_diff_32 as i64) as u64;
            }
            _ => this_m.offset = dec.read_int64()?,
        }
        this_m.packet_size = self
            .ic_packet_size
            .decompress(dec, last_m.packet_size as i32, 0)? as u32;
        this_m.return_point = self
            .ic_return_point
            .decompress(dec, last_m.return_point, 0)?;
        this_m.x = self.ic_xyz.decompress(dec, last_m.x, 0)?;
        this_m.y = self.ic_xyz.decompress(dec, last_m.y, 1)?;
        this_m.z = self.ic_xyz.decompress(dec, last_m.z, 2)?;
        wavepacket13_pack(&this_m, &mut out[1..29]);
        self.last_item.copy_from_slice(&out[1..29]);
        Ok(out)
    }
}

#[derive(Clone, Debug)]
pub struct LaszipItem {
    pub item_type: u16,
    pub size: u16,
    pub version: u16,
}

#[derive(Clone, Debug)]
pub struct LaszipLayout {
    pub compressor: u16,
    pub coder: u16,
    pub chunk_size: u32,
    pub items: Vec<LaszipItem>,
}

pub fn laszip_vlr(vlrs: &[LasVlr]) -> Option<LaszipLayout> {
    let vlr = vlrs
        .iter()
        .find(|v| v.user_id == "laszip encoded" && v.record_id == LASZIP_VLR_RECORD_ID)?;
    let p = &vlr.payload;
    if p.len() < 34 {
        return None;
    }
    if (p.len() - 34) % 6 != 0 {
        return None;
    }
    let num_items = (p.len() - 34) / 6;
    if num_items == 0 {
        return None;
    }
    let mut items = Vec::with_capacity(num_items);
    let mut off = 34usize;
    for _ in 0..num_items {
        let item_type = u16::from_le_bytes([p[off], p[off + 1]]);
        let size = u16::from_le_bytes([p[off + 2], p[off + 3]]);
        let version = u16::from_le_bytes([p[off + 4], p[off + 5]]);
        items.push(LaszipItem {
            item_type,
            size,
            version,
        });
        off += 6;
    }
    Some(LaszipLayout {
        compressor: u16::from_le_bytes([p[0], p[1]]),
        coder: u16::from_le_bytes([p[2], p[3]]),
        chunk_size: u32::from_le_bytes([p[12], p[13], p[14], p[15]]),
        items,
    })
}

pub fn has_laszip_vlr(vlrs: &[LasVlr]) -> bool {
    vlrs.iter()
        .any(|v| v.user_id == "laszip encoded" && v.record_id == LASZIP_VLR_RECORD_ID)
}

fn take_layer(
    cur: &mut Cursor,
    n: usize,
    base: usize,
    offset: &mut usize,
) -> Result<Option<AcDecoder>, LasNote> {
    if n == 0 {
        return Ok(None);
    }
    let data = cur.read_bytes(n)?;
    let dec = AcDecoder::init(data, base)?;
    *offset += n;
    Ok(Some(dec))
}

fn read_chunk_table(
    header: &LasHeader,
    bytes: &[u8],
    layout: &LaszipLayout,
) -> Result<(Vec<u64>, Option<Vec<u64>>, u32), LasNote> {
    let point_start = header.offset_to_points as usize;
    if point_start + 8 > bytes.len() {
        return Err(LasNote::LazChunkTable { off: point_start });
    }
    let mut table_pos = le_i64(&bytes[point_start..point_start + 8]);
    let chunks_start = point_start + 8;
    if table_pos == -1 {
        if bytes.len() < 8 {
            return Err(LasNote::LazChunkTable { off: point_start });
        }
        table_pos = le_i64(&bytes[bytes.len() - 8..]);
    }
    if table_pos < 0 {
        return Err(LasNote::LazChunkTable { off: point_start });
    }
    let table_pos = table_pos as usize;
    if table_pos + 8 > bytes.len() {
        return Err(LasNote::LazChunkTable { off: table_pos });
    }
    let mut cur = Cursor::new(&bytes[table_pos..], table_pos);
    let version = cur.read_u32()?;
    if version != 0 {
        return Err(LasNote::LazChunkTable { off: table_pos });
    }
    let number_chunks = cur.read_u32()? as usize;

    let variable = layout.chunk_size == 0 || layout.chunk_size == u32::MAX;

    let mut start_deltas = vec![0i64; number_chunks + 1];
    let mut total_deltas = if variable {
        Some(vec![0i64; number_chunks + 1])
    } else {
        None
    };

    if number_chunks > 0 {
        let mut dec = AcDecoder::init(&bytes[table_pos + 8..], table_pos + 8)?;
        let mut ic = IntegerCompressor::new(32, 2, 8);
        ic.init();
        for i in 1..=number_chunks {
            if let Some(td) = total_deltas.as_mut() {
                td[i] = ic.decompress(&mut dec, td[i - 1] as i32, 0)? as i64;
            }
            start_deltas[i] = ic.decompress(&mut dec, start_deltas[i - 1] as i32, 1)? as i64;
        }
    }

    let mut chunk_starts = vec![0u64; number_chunks + 1];
    chunk_starts[0] = chunks_start as u64;
    let mut chunk_totals = if variable {
        Some(vec![0u64; number_chunks + 1])
    } else {
        None
    };
    for i in 1..=number_chunks {
        chunk_starts[i] = chunk_starts[i - 1].wrapping_add(start_deltas[i] as u64);
        if let Some(t) = chunk_totals.as_mut() {
            t[i] = t[i - 1].wrapping_add(total_deltas.as_ref().unwrap()[i] as u64);
        }
    }

    Ok((chunk_starts, chunk_totals, layout.chunk_size))
}

fn decode_rgb(
    dec: &mut AcDecoder,
    last: [u16; 3],
    m_byte_used: &mut SymbolModel,
    m_rgb_diff: &mut [SymbolModel; 6],
) -> Result<[u16; 3], LasNote> {
    let sym = dec.decode_symbol(m_byte_used)?;
    let mut item = [0u16; 3];
    if sym & (1 << 0) != 0 {
        let corr = dec.decode_symbol(&mut m_rgb_diff[0])? as i32;
        item[0] = u8_fold(corr + (last[0] & 255) as i32) as u16;
    } else {
        item[0] = last[0] & 0xFF;
    }
    if sym & (1 << 1) != 0 {
        let corr = dec.decode_symbol(&mut m_rgb_diff[1])? as i32;
        item[0] |= (u8_fold(corr + (last[0] >> 8) as i32) as u16) << 8;
    } else {
        item[0] |= last[0] & 0xFF00;
    }
    if sym & (1 << 6) != 0 {
        let mut diff = (item[0] & 0x00FF) as i32 - (last[0] & 0x00FF) as i32;
        if sym & (1 << 2) != 0 {
            let corr = dec.decode_symbol(&mut m_rgb_diff[2])? as i32;
            item[1] = u8_fold(corr + u8_clamp(diff + (last[1] & 255) as i32) as i32) as u16;
        } else {
            item[1] = last[1] & 0xFF;
        }
        if sym & (1 << 4) != 0 {
            let corr = dec.decode_symbol(&mut m_rgb_diff[4])? as i32;
            diff = (diff + ((item[1] & 0x00FF) as i32 - (last[1] & 0x00FF) as i32)) / 2;
            item[2] = u8_fold(corr + u8_clamp(diff + (last[2] & 255) as i32) as i32) as u16;
        } else {
            item[2] = last[2] & 0xFF;
        }
        diff = (item[0] >> 8) as i32 - (last[0] >> 8) as i32;
        if sym & (1 << 3) != 0 {
            let corr = dec.decode_symbol(&mut m_rgb_diff[3])? as i32;
            item[1] |= (u8_fold(corr + u8_clamp(diff + (last[1] >> 8) as i32) as i32) as u16) << 8;
        } else {
            item[1] |= last[1] & 0xFF00;
        }
        if sym & (1 << 5) != 0 {
            let corr = dec.decode_symbol(&mut m_rgb_diff[5])? as i32;
            diff = (diff + ((item[1] >> 8) as i32 - (last[1] >> 8) as i32)) / 2;
            item[2] |= (u8_fold(corr + u8_clamp(diff + (last[2] >> 8) as i32) as i32) as u16) << 8;
        } else {
            item[2] |= last[2] & 0xFF00;
        }
    } else {
        item[1] = item[0];
        item[2] = item[0];
    }
    Ok(item)
}

fn u16x3_bytes(v: [u16; 3]) -> [u8; 6] {
    [
        (v[0] & 0xFF) as u8,
        (v[0] >> 8) as u8,
        (v[1] & 0xFF) as u8,
        (v[1] >> 8) as u8,
        (v[2] & 0xFF) as u8,
        (v[2] >> 8) as u8,
    ]
}

struct Rgb14Context {
    unused: bool,
    last_item: [u16; 3],
    m_byte_used: SymbolModel,
    m_rgb_diff: [SymbolModel; 6],
}

impl Rgb14Context {
    fn new() -> Self {
        Rgb14Context {
            unused: true,
            last_item: [0; 3],
            m_byte_used: SymbolModel::new(128),
            m_rgb_diff: [
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
            ],
        }
    }

    fn init(&mut self, last: [u16; 3]) {
        self.m_byte_used.init();
        for m in &mut self.m_rgb_diff {
            m.init();
        }
        self.last_item = last;
        self.unused = false;
    }
}

struct Rgb14Reader {
    contexts: [Rgb14Context; 4],
    current_context: usize,
}

impl Rgb14Reader {
    fn new() -> Self {
        Rgb14Reader {
            contexts: [
                Rgb14Context::new(),
                Rgb14Context::new(),
                Rgb14Context::new(),
                Rgb14Context::new(),
            ],
            current_context: 0,
        }
    }

    fn init(&mut self, context: usize, item: &[u8]) {
        for c in &mut self.contexts {
            c.unused = true;
        }
        self.current_context = context;
        let last = [
            u16::from_le_bytes([item[0], item[1]]),
            u16::from_le_bytes([item[2], item[3]]),
            u16::from_le_bytes([item[4], item[5]]),
        ];
        self.contexts[context].init(last);
    }

    fn read(
        &mut self,
        dec: Option<&mut AcDecoder>,
        context: usize,
        out: &mut [u8],
    ) -> Result<(), LasNote> {
        if self.current_context != context {
            let prev = self.contexts[self.current_context].last_item;
            self.current_context = context;
            if self.contexts[context].unused {
                self.contexts[context].init(prev);
            }
        }
        let ctx = &mut self.contexts[self.current_context];
        match dec {
            Some(d) => {
                let item = decode_rgb(d, ctx.last_item, &mut ctx.m_byte_used, &mut ctx.m_rgb_diff)?;
                ctx.last_item = item;
                out[0..6].copy_from_slice(&u16x3_bytes(item));
            }
            None => out[0..6].copy_from_slice(&u16x3_bytes(ctx.last_item)),
        }
        Ok(())
    }
}

struct RgbNir14Context {
    unused: bool,
    last_item: [u16; 4],
    m_rgb_bytes_used: SymbolModel,
    m_rgb_diff: [SymbolModel; 6],
    m_nir_bytes_used: SymbolModel,
    m_nir_diff: [SymbolModel; 2],
}

impl RgbNir14Context {
    fn new() -> Self {
        RgbNir14Context {
            unused: true,
            last_item: [0; 4],
            m_rgb_bytes_used: SymbolModel::new(128),
            m_rgb_diff: [
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
                SymbolModel::new(256),
            ],
            m_nir_bytes_used: SymbolModel::new(4),
            m_nir_diff: [SymbolModel::new(256), SymbolModel::new(256)],
        }
    }

    fn init(&mut self, last: [u16; 4]) {
        self.m_rgb_bytes_used.init();
        for m in &mut self.m_rgb_diff {
            m.init();
        }
        self.m_nir_bytes_used.init();
        for m in &mut self.m_nir_diff {
            m.init();
        }
        self.last_item = last;
        self.unused = false;
    }
}

struct RgbNir14Reader {
    contexts: [RgbNir14Context; 4],
    current_context: usize,
}

impl RgbNir14Reader {
    fn new() -> Self {
        RgbNir14Reader {
            contexts: [
                RgbNir14Context::new(),
                RgbNir14Context::new(),
                RgbNir14Context::new(),
                RgbNir14Context::new(),
            ],
            current_context: 0,
        }
    }

    fn init(&mut self, context: usize, item: &[u8]) {
        for c in &mut self.contexts {
            c.unused = true;
        }
        self.current_context = context;
        let last = [
            u16::from_le_bytes([item[0], item[1]]),
            u16::from_le_bytes([item[2], item[3]]),
            u16::from_le_bytes([item[4], item[5]]),
            u16::from_le_bytes([item[6], item[7]]),
        ];
        self.contexts[context].init(last);
    }

    fn read(
        &mut self,
        dec_rgb: Option<&mut AcDecoder>,
        dec_nir: Option<&mut AcDecoder>,
        context: usize,
        out: &mut [u8],
    ) -> Result<(), LasNote> {
        if self.current_context != context {
            let prev = self.contexts[self.current_context].last_item;
            self.current_context = context;
            if self.contexts[context].unused {
                self.contexts[context].init(prev);
            }
        }
        let ctx = &mut self.contexts[self.current_context];
        match dec_rgb {
            Some(d) => {
                let last3 = [ctx.last_item[0], ctx.last_item[1], ctx.last_item[2]];
                let item = decode_rgb(d, last3, &mut ctx.m_rgb_bytes_used, &mut ctx.m_rgb_diff)?;
                ctx.last_item[0] = item[0];
                ctx.last_item[1] = item[1];
                ctx.last_item[2] = item[2];
                out[0..2].copy_from_slice(&item[0].to_le_bytes());
                out[2..4].copy_from_slice(&item[1].to_le_bytes());
                out[4..6].copy_from_slice(&item[2].to_le_bytes());
            }
            None => {
                out[0..2].copy_from_slice(&ctx.last_item[0].to_le_bytes());
                out[2..4].copy_from_slice(&ctx.last_item[1].to_le_bytes());
                out[4..6].copy_from_slice(&ctx.last_item[2].to_le_bytes());
            }
        }
        match dec_nir {
            Some(d) => {
                let sym = d.decode_symbol(&mut ctx.m_nir_bytes_used)?;
                let mut nir;
                if sym & (1 << 0) != 0 {
                    let corr = d.decode_symbol(&mut ctx.m_nir_diff[0])? as i32;
                    nir = u8_fold(corr + (ctx.last_item[3] & 255) as i32) as u16;
                } else {
                    nir = ctx.last_item[3] & 0xFF;
                }
                if sym & (1 << 1) != 0 {
                    let corr = d.decode_symbol(&mut ctx.m_nir_diff[1])? as i32;
                    nir |= (u8_fold(corr + (ctx.last_item[3] >> 8) as i32) as u16) << 8;
                } else {
                    nir |= ctx.last_item[3] & 0xFF00;
                }
                ctx.last_item[3] = nir;
                out[6..8].copy_from_slice(&nir.to_le_bytes());
            }
            None => out[6..8].copy_from_slice(&ctx.last_item[3].to_le_bytes()),
        }
        Ok(())
    }
}

struct Wavepacket14Context {
    unused: bool,
    last_item: [u8; 29],
    last_diff_32: i32,
    sym_last_offset_diff: u32,
    m_packet_index: SymbolModel,
    m_offset_diff: [SymbolModel; 4],
    ic_offset_diff: IntegerCompressor,
    ic_packet_size: IntegerCompressor,
    ic_return_point: IntegerCompressor,
    ic_xyz: IntegerCompressor,
}

impl Wavepacket14Context {
    fn new() -> Self {
        Wavepacket14Context {
            unused: true,
            last_item: [0; 29],
            last_diff_32: 0,
            sym_last_offset_diff: 0,
            m_packet_index: SymbolModel::new(256),
            m_offset_diff: [
                SymbolModel::new(4),
                SymbolModel::new(4),
                SymbolModel::new(4),
                SymbolModel::new(4),
            ],
            ic_offset_diff: IntegerCompressor::new(32, 1, 8),
            ic_packet_size: IntegerCompressor::new(32, 1, 8),
            ic_return_point: IntegerCompressor::new(32, 1, 8),
            ic_xyz: IntegerCompressor::new(32, 3, 8),
        }
    }

    fn init(&mut self, item: &[u8]) {
        self.m_packet_index.init();
        for m in &mut self.m_offset_diff {
            m.init();
        }
        self.ic_offset_diff.init();
        self.ic_packet_size.init();
        self.ic_return_point.init();
        self.ic_xyz.init();
        self.last_diff_32 = 0;
        self.sym_last_offset_diff = 0;
        self.last_item.copy_from_slice(item);
        self.unused = false;
    }
}

struct Wavepacket14Reader {
    contexts: [Wavepacket14Context; 4],
    current_context: usize,
}

impl Wavepacket14Reader {
    fn new() -> Self {
        Wavepacket14Reader {
            contexts: [
                Wavepacket14Context::new(),
                Wavepacket14Context::new(),
                Wavepacket14Context::new(),
                Wavepacket14Context::new(),
            ],
            current_context: 0,
        }
    }

    fn init(&mut self, context: usize, item: &[u8]) {
        for c in &mut self.contexts {
            c.unused = true;
        }
        self.current_context = context;
        self.contexts[context].init(item);
    }

    fn read(
        &mut self,
        dec: Option<&mut AcDecoder>,
        context: usize,
        out: &mut [u8],
    ) -> Result<(), LasNote> {
        if self.current_context != context {
            let prev = self.contexts[self.current_context].last_item;
            self.current_context = context;
            if self.contexts[context].unused {
                self.contexts[context].init(&prev);
            }
        }
        if let Some(dec) = dec {
            let ctx = &mut self.contexts[self.current_context];
            out[0] = dec.decode_symbol(&mut ctx.m_packet_index)? as u8;
            let last_m = wavepacket13_unpack(&ctx.last_item[1..29]);
            let mut this_m = WavePacket13 {
                offset: 0,
                packet_size: 0,
                return_point: 0,
                x: 0,
                y: 0,
                z: 0,
            };
            ctx.sym_last_offset_diff =
                dec.decode_symbol(&mut ctx.m_offset_diff[ctx.sym_last_offset_diff as usize])?;
            match ctx.sym_last_offset_diff {
                0 => this_m.offset = last_m.offset,
                1 => this_m.offset = last_m.offset + last_m.packet_size as u64,
                2 => {
                    ctx.last_diff_32 = ctx.ic_offset_diff.decompress(dec, ctx.last_diff_32, 0)?;
                    this_m.offset =
                        (last_m.offset as i64).wrapping_add(ctx.last_diff_32 as i64) as u64;
                }
                _ => this_m.offset = dec.read_int64()?,
            }
            this_m.packet_size =
                ctx.ic_packet_size
                    .decompress(dec, last_m.packet_size as i32, 0)? as u32;
            this_m.return_point = ctx
                .ic_return_point
                .decompress(dec, last_m.return_point, 0)?;
            this_m.x = ctx.ic_xyz.decompress(dec, last_m.x, 0)?;
            this_m.y = ctx.ic_xyz.decompress(dec, last_m.y, 1)?;
            this_m.z = ctx.ic_xyz.decompress(dec, last_m.z, 2)?;
            wavepacket13_pack(&this_m, &mut out[1..29]);
            ctx.last_item.copy_from_slice(&out[0..29]);
        }
        Ok(())
    }
}

struct Byte14Context {
    unused: bool,
    last_item: Vec<u8>,
    m_bytes: Vec<SymbolModel>,
}

impl Byte14Context {
    fn new(number: usize) -> Self {
        Byte14Context {
            unused: true,
            last_item: vec![0; number],
            m_bytes: (0..number).map(|_| SymbolModel::new(256)).collect(),
        }
    }

    fn init(&mut self, item: &[u8]) {
        for m in &mut self.m_bytes {
            m.init();
        }
        self.last_item.copy_from_slice(item);
        self.unused = false;
    }
}

struct Byte14Reader {
    contexts: Vec<Byte14Context>,
    current_context: usize,
    number: usize,
}

impl Byte14Reader {
    fn new(number: usize) -> Self {
        Byte14Reader {
            contexts: (0..4).map(|_| Byte14Context::new(number)).collect(),
            current_context: 0,
            number,
        }
    }

    fn init(&mut self, context: usize, item: &[u8]) {
        for c in &mut self.contexts {
            c.unused = true;
        }
        self.current_context = context;
        self.contexts[context].init(item);
    }

    fn read(
        &mut self,
        decs: &mut [Option<AcDecoder>],
        context: usize,
        out: &mut [u8],
    ) -> Result<(), LasNote> {
        if self.current_context != context {
            let prev = self.contexts[self.current_context].last_item.clone();
            self.current_context = context;
            if self.contexts[context].unused {
                self.contexts[context].init(&prev);
            }
        }
        for i in 0..self.number {
            match decs[i].as_mut() {
                Some(d) => {
                    let ctx = &mut self.contexts[self.current_context];
                    let value =
                        ctx.last_item[i] as i32 + d.decode_symbol(&mut ctx.m_bytes[i])? as i32;
                    out[i] = u8_fold(value);
                    ctx.last_item[i] = out[i];
                }
                None => {
                    let ctx = &self.contexts[self.current_context];
                    out[i] = ctx.last_item[i];
                }
            }
        }
        Ok(())
    }
}

fn decode_chunk(
    header: &LasHeader,
    bytes: &[u8],
    start: usize,
    end: usize,
    layout: &LaszipLayout,
    point_count: u64,
) -> Result<Vec<LasPoint>, LasNote> {
    if start >= end || end > bytes.len() {
        return Err(LasNote::LazChunkOverrun { off: end });
    }
    match layout.compressor {
        LASZIP_COMPRESSOR_LAYERED_CHUNKED => {
            decode_chunk_layered(header, bytes, start, end, layout)
        }
        LASZIP_COMPRESSOR_POINTWISE_CHUNKED => {
            decode_chunk_pointwise(header, bytes, start, end, layout, point_count)
        }
        _ => Err(LasNote::LazItem {
            item: layout.compressor,
        }),
    }
}

enum LayeredExtra {
    Rgb14 {
        reader: Rgb14Reader,
        dec: Option<AcDecoder>,
    },
    RgbNir14 {
        reader: RgbNir14Reader,
        dec_rgb: Option<AcDecoder>,
        dec_nir: Option<AcDecoder>,
    },
    Wavepacket14 {
        reader: Wavepacket14Reader,
        dec: Option<AcDecoder>,
    },
    Byte14 {
        reader: Byte14Reader,
        decs: Vec<Option<AcDecoder>>,
    },
}

impl LayeredExtra {
    fn read(&mut self, context: usize, out: &mut [u8]) -> Result<(), LasNote> {
        match self {
            LayeredExtra::Rgb14 { reader, dec } => reader.read(dec.as_mut(), context, out),
            LayeredExtra::RgbNir14 {
                reader,
                dec_rgb,
                dec_nir,
            } => reader.read(dec_rgb.as_mut(), dec_nir.as_mut(), context, out),
            LayeredExtra::Wavepacket14 { reader, dec } => reader.read(dec.as_mut(), context, out),
            LayeredExtra::Byte14 { reader, decs } => reader.read(decs, context, out),
        }
    }
}

fn decode_chunk_layered(
    header: &LasHeader,
    bytes: &[u8],
    start: usize,
    end: usize,
    layout: &LaszipLayout,
) -> Result<Vec<LasPoint>, LasNote> {
    let first = layout.items.first().ok_or(LasNote::LazAbsent)?;
    if first.item_type != ITEM_POINT14 {
        return Err(LasNote::LazItem {
            item: first.item_type,
        });
    }
    if layout.items[0].version != 3 && layout.items[0].version != 4 {
        return Err(LasNote::LazItem {
            item: layout.items[0].version,
        });
    }
    for item in &layout.items[1..] {
        let ok = matches!(
            item.item_type,
            ITEM_RGB14 | ITEM_RGBNIR14 | ITEM_WAVEPACKET14 | ITEM_BYTE14
        ) && (item.version == 2 || item.version == 3);
        if !ok {
            return Err(LasNote::LazItem {
                item: item.item_type,
            });
        }
    }

    let point_length = header.point_length as usize;
    let mut offsets = Vec::with_capacity(layout.items.len());
    let mut total = 0usize;
    for item in &layout.items {
        offsets.push(total);
        total += item.size as usize;
    }
    if total > point_length {
        return Err(LasNote::LazChunkOverrun { off: start });
    }

    let mut cur = Cursor::new(&bytes[start..end], start);
    let mut raw_first = vec![0u8; point_length];
    for (i, item) in layout.items.iter().enumerate() {
        let b = cur.read_bytes(item.size as usize)?;
        raw_first[offsets[i]..offsets[i] + item.size as usize].copy_from_slice(b);
    }

    let first_state = point14_from_raw(&raw_first[0..30]);

    let count = cur.read_u32()? as usize;

    let n_xy = cur.read_u32()? as usize;
    let n_z = cur.read_u32()? as usize;
    let n_classification = cur.read_u32()? as usize;
    let n_flags = cur.read_u32()? as usize;
    let n_intensity = cur.read_u32()? as usize;
    let n_scan_angle = cur.read_u32()? as usize;
    let n_user_data = cur.read_u32()? as usize;
    let n_point_source = cur.read_u32()? as usize;
    let n_gps_time = cur.read_u32()? as usize;

    let mut extra_sizes: Vec<Vec<usize>> = Vec::with_capacity(layout.items.len() - 1);
    for item in &layout.items[1..] {
        let n_layers = match item.item_type {
            ITEM_RGB14 => 1,
            ITEM_RGBNIR14 => 2,
            ITEM_WAVEPACKET14 => 1,
            ITEM_BYTE14 => item.size as usize,
            _ => {
                return Err(LasNote::LazItem {
                    item: item.item_type,
                })
            }
        };
        let mut sizes = Vec::with_capacity(n_layers);
        for _ in 0..n_layers {
            sizes.push(cur.read_u32()? as usize);
        }
        extra_sizes.push(sizes);
    }

    let layer_base = start + cur.pos;

    let mut offset = 0usize;
    let dec_xy = {
        if n_xy == 0 {
            return Err(LasNote::LazChunkOverrun { off: layer_base });
        }
        let data = cur.read_bytes(n_xy)?;
        AcDecoder::init(data, layer_base)?
    };
    offset += n_xy;
    let dec_z = take_layer(&mut cur, n_z, layer_base + offset, &mut offset)?;
    let dec_classification =
        take_layer(&mut cur, n_classification, layer_base + offset, &mut offset)?;
    let dec_flags = take_layer(&mut cur, n_flags, layer_base + offset, &mut offset)?;
    let dec_intensity = take_layer(&mut cur, n_intensity, layer_base + offset, &mut offset)?;
    let dec_scan_angle = take_layer(&mut cur, n_scan_angle, layer_base + offset, &mut offset)?;
    let dec_user_data = take_layer(&mut cur, n_user_data, layer_base + offset, &mut offset)?;
    let dec_point_source = take_layer(&mut cur, n_point_source, layer_base + offset, &mut offset)?;
    let dec_gps_time = take_layer(&mut cur, n_gps_time, layer_base + offset, &mut offset)?;

    let scanner = first_state.scanner_channel as usize;
    let mut extras: Vec<(usize, usize, LayeredExtra)> = Vec::with_capacity(layout.items.len() - 1);
    for (i, item) in layout.items[1..].iter().enumerate() {
        let sizes = &extra_sizes[i];
        let item_off = offsets[i + 1];
        let size = item.size as usize;
        let extra = match item.item_type {
            ITEM_RGB14 => {
                let n = sizes[0];
                let mut reader = Rgb14Reader::new();
                let dec = if n > 0 {
                    let data = cur.read_bytes(n)?;
                    let d = AcDecoder::init(data, layer_base + offset)?;
                    offset += n;
                    Some(d)
                } else {
                    None
                };
                reader.init(scanner, &raw_first[item_off..item_off + size]);
                LayeredExtra::Rgb14 { reader, dec }
            }
            ITEM_RGBNIR14 => {
                let n_rgb = sizes[0];
                let n_nir = sizes[1];
                let mut reader = RgbNir14Reader::new();
                let dec_rgb = if n_rgb > 0 {
                    let data = cur.read_bytes(n_rgb)?;
                    let d = AcDecoder::init(data, layer_base + offset)?;
                    offset += n_rgb;
                    Some(d)
                } else {
                    None
                };
                let dec_nir = if n_nir > 0 {
                    let data = cur.read_bytes(n_nir)?;
                    let d = AcDecoder::init(data, layer_base + offset)?;
                    offset += n_nir;
                    Some(d)
                } else {
                    None
                };
                reader.init(scanner, &raw_first[item_off..item_off + size]);
                LayeredExtra::RgbNir14 {
                    reader,
                    dec_rgb,
                    dec_nir,
                }
            }
            ITEM_WAVEPACKET14 => {
                let n = sizes[0];
                let mut reader = Wavepacket14Reader::new();
                let dec = if n > 0 {
                    let data = cur.read_bytes(n)?;
                    let d = AcDecoder::init(data, layer_base + offset)?;
                    offset += n;
                    Some(d)
                } else {
                    None
                };
                reader.init(scanner, &raw_first[item_off..item_off + size]);
                LayeredExtra::Wavepacket14 { reader, dec }
            }
            ITEM_BYTE14 => {
                let number = size;
                let mut reader = Byte14Reader::new(number);
                let mut decs = Vec::with_capacity(number);
                for j in 0..number {
                    let n = sizes[j];
                    if n > 0 {
                        let data = cur.read_bytes(n)?;
                        let d = AcDecoder::init(data, layer_base + offset)?;
                        offset += n;
                        decs.push(Some(d));
                    } else {
                        decs.push(None);
                    }
                }
                reader.init(scanner, &raw_first[item_off..item_off + number]);
                LayeredExtra::Byte14 { reader, decs }
            }
            _ => {
                return Err(LasNote::LazItem {
                    item: item.item_type,
                })
            }
        };
        extras.push((item_off, size, extra));
    }

    let mut decoders = Point14Decoders {
        dec_xy,
        dec_z,
        dec_classification,
        dec_flags,
        dec_intensity,
        dec_scan_angle,
        dec_user_data,
        dec_point_source,
        dec_gps_time,
    };

    let changed = ChangedFlags {
        z: n_z > 0,
        classification: n_classification > 0,
        flags: n_flags > 0,
        intensity: n_intensity > 0,
        scan_angle: n_scan_angle > 0,
        user_data: n_user_data > 0,
        point_source: n_point_source > 0,
        gps_time: n_gps_time > 0,
    };

    let mut reader = Point14Reader {
        contexts: [
            Point14Context::new(),
            Point14Context::new(),
            Point14Context::new(),
            Point14Context::new(),
        ],
        current_context: 0,
    };
    reader.current_context = first_state.scanner_channel as usize;
    reader.contexts[reader.current_context].init(&first_state);

    let mut out = Vec::with_capacity(count);
    let mut raw = raw_first;
    out.push(decode_point(header, &raw, 0)?);

    for _ in 1..count {
        let p = reader.read(&mut decoders, &changed)?;
        let context = reader.current_context;
        p.to_raw(&mut raw[0..30]);
        for (off, size, extra) in extras.iter_mut() {
            extra.read(context, &mut raw[*off..*off + *size])?;
        }
        out.push(decode_point(header, &raw, 0)?);
    }

    Ok(out)
}

enum PointwiseItem {
    Point10(Point10Reader),
    GpsTime11(GpsTime11Reader),
    Rgb12(Rgb12Reader),
    Wavepacket13(Wavepacket13Reader),
}

impl PointwiseItem {
    fn read(&mut self, dec: &mut AcDecoder, out: &mut [u8]) -> Result<(), LasNote> {
        match self {
            PointwiseItem::Point10(r) => {
                let v = r.read(dec)?;
                out[0..20].copy_from_slice(&v);
            }
            PointwiseItem::GpsTime11(r) => {
                let v = r.read(dec)?;
                out[0..8].copy_from_slice(&v.to_le_bytes());
            }
            PointwiseItem::Rgb12(r) => {
                let v = r.read(dec)?;
                out[0..6].copy_from_slice(&u16x3_bytes(v));
            }
            PointwiseItem::Wavepacket13(r) => {
                let v = r.read(dec)?;
                out[0..29].copy_from_slice(&v);
            }
        }
        Ok(())
    }
}

fn decode_chunk_pointwise(
    header: &LasHeader,
    bytes: &[u8],
    start: usize,
    end: usize,
    layout: &LaszipLayout,
    point_count: u64,
) -> Result<Vec<LasPoint>, LasNote> {
    let point_length = header.point_length as usize;
    let mut offsets = Vec::with_capacity(layout.items.len());
    let mut total = 0usize;
    for item in &layout.items {
        offsets.push(total);
        total += item.size as usize;
    }
    if total > point_length {
        return Err(LasNote::LazChunkOverrun { off: start });
    }

    let mut cur = Cursor::new(&bytes[start..end], start);
    let mut raw_first = vec![0u8; point_length];
    for (i, item) in layout.items.iter().enumerate() {
        let b = cur.read_bytes(item.size as usize)?;
        raw_first[offsets[i]..offsets[i] + item.size as usize].copy_from_slice(b);
    }

    let mut items: Vec<(usize, usize, PointwiseItem)> = Vec::with_capacity(layout.items.len());
    for (i, item) in layout.items.iter().enumerate() {
        let off = offsets[i];
        let size = item.size as usize;
        let reader = match item.item_type {
            ITEM_POINT10 => {
                if size != 20 {
                    return Err(LasNote::LazItem {
                        item: item.item_type,
                    });
                }
                let mut a = [0u8; 20];
                a.copy_from_slice(&raw_first[off..off + 20]);
                let mut r = Point10Reader::new();
                r.init(&a);
                PointwiseItem::Point10(r)
            }
            ITEM_GPSTIME11 => {
                if size != 8 {
                    return Err(LasNote::LazItem {
                        item: item.item_type,
                    });
                }
                let mut a = [0u8; 8];
                a.copy_from_slice(&raw_first[off..off + 8]);
                let mut r = GpsTime11Reader::new();
                r.init(&a);
                PointwiseItem::GpsTime11(r)
            }
            ITEM_RGB12 => {
                if size != 6 {
                    return Err(LasNote::LazItem {
                        item: item.item_type,
                    });
                }
                let mut a = [0u8; 6];
                a.copy_from_slice(&raw_first[off..off + 6]);
                let mut r = Rgb12Reader::new();
                r.init(&a);
                PointwiseItem::Rgb12(r)
            }
            ITEM_WAVEPACKET13 => {
                if size != 29 {
                    return Err(LasNote::LazItem {
                        item: item.item_type,
                    });
                }
                let mut a = [0u8; 29];
                a.copy_from_slice(&raw_first[off..off + 29]);
                let mut r = Wavepacket13Reader::new();
                r.init(&a);
                PointwiseItem::Wavepacket13(r)
            }
            _ => {
                return Err(LasNote::LazItem {
                    item: item.item_type,
                })
            }
        };
        items.push((off, size, reader));
    }

    let stream_start = start + cur.pos;
    let mut dec = AcDecoder::init(&bytes[stream_start..end], stream_start)?;

    let count = point_count as usize;
    let mut out = Vec::with_capacity(count);
    out.push(decode_point(header, &raw_first, 0)?);

    let mut raw = raw_first;
    for _ in 1..count {
        for (off, size, item) in items.iter_mut() {
            item.read(&mut dec, &mut raw[*off..*off + *size])?;
        }
        out.push(decode_point(header, &raw, 0)?);
    }

    Ok(out)
}

impl super::LasHeader {
    pub fn laz_point_at(&self, bytes: &[u8], index: u64) -> Result<LasPoint, LasNote> {
        let mut dec = LazDecoder::new(self, bytes)?;
        dec.point_at(index)
    }
}

pub struct LazDecoder<'a> {
    header: &'a LasHeader,
    bytes: &'a [u8],
    layout: LaszipLayout,
    chunk_starts: Vec<u64>,
    chunk_totals: Option<Vec<u64>>,
    fixed_chunk_size: u32,
    cache: Option<(usize, Vec<LasPoint>)>,
}

impl<'a> LazDecoder<'a> {
    pub fn new(header: &'a LasHeader, bytes: &'a [u8]) -> Result<Self, LasNote> {
        let vlrs = header.vlrs(bytes)?;
        let layout = laszip_vlr(&vlrs).ok_or(LasNote::LazAbsent)?;
        let (chunk_starts, chunk_totals, fixed_chunk_size) =
            read_chunk_table(header, bytes, &layout)?;
        Ok(LazDecoder {
            header,
            bytes,
            layout,
            chunk_starts,
            chunk_totals,
            fixed_chunk_size,
            cache: None,
        })
    }

    pub fn point_at(&mut self, index: u64) -> Result<LasPoint, LasNote> {
        let (chunk_idx, within, point_count) = match &self.chunk_totals {
            Some(totals) => {
                let n = totals.len() - 1;
                if n == 0 {
                    return Err(LasNote::LazChunkTable { off: 0 });
                }
                let mut lo = 0usize;
                let mut hi = n;
                while lo + 1 < hi {
                    let mid = (lo + hi) >> 1;
                    if index >= totals[mid] {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                let chunk_idx = lo;
                let within = index - totals[chunk_idx];
                let point_count = totals[chunk_idx + 1] - totals[chunk_idx];
                (chunk_idx, within, point_count)
            }
            None => {
                let cs = self.fixed_chunk_size as u64;
                if cs == 0 {
                    return Err(LasNote::LazChunkTable { off: 0 });
                }
                let chunk_idx = (index / cs) as usize;
                let within = index % cs;
                let point_count = (self.header.point_count - chunk_idx as u64 * cs).min(cs);
                (chunk_idx, within, point_count)
            }
        };

        if chunk_idx + 1 >= self.chunk_starts.len() {
            return Err(LasNote::LazChunkOverrun { off: 0 });
        }

        let points = match &self.cache {
            Some((c, pts)) if *c == chunk_idx => pts,
            _ => {
                let start = self.chunk_starts[chunk_idx] as usize;
                let end = self.chunk_starts[chunk_idx + 1] as usize;
                let pts = decode_chunk(
                    self.header,
                    self.bytes,
                    start,
                    end,
                    &self.layout,
                    point_count,
                )?;
                self.cache = Some((chunk_idx, pts));
                &self.cache.as_ref().unwrap().1
            }
        };

        points
            .get(within as usize)
            .copied()
            .ok_or(LasNote::LazChunkOverrun { off: 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEncoder {
        base: u32,
        length: u32,
        out: Vec<u8>,
    }

    impl TestEncoder {
        fn new() -> Self {
            TestEncoder {
                base: 0,
                length: AC_MAX_LENGTH,
                out: Vec::new(),
            }
        }

        fn propagate_carry(&mut self) {
            let mut i = self.out.len();
            loop {
                i -= 1;
                if self.out[i] != 0xFF {
                    self.out[i] += 1;
                    break;
                }
                self.out[i] = 0;
            }
        }

        fn renorm(&mut self) {
            loop {
                self.out.push((self.base >> 24) as u8);
                self.base <<= 8;
                self.length <<= 8;
                if self.length >= AC_MIN_LENGTH {
                    break;
                }
            }
        }

        fn encode_bit(&mut self, m: &mut BitModel, sym: u32) {
            let x = (m.bit_0_prob as u64 * (self.length >> BM_LENGTH_SHIFT) as u64) as u32;
            if sym == 0 {
                self.length = x;
                m.bit_0_count += 1;
            } else {
                let init_base = self.base;
                self.base = self.base.wrapping_add(x);
                self.length -= x;
                if init_base > self.base {
                    self.propagate_carry();
                }
            }
            if self.length < AC_MIN_LENGTH {
                self.renorm();
            }
            m.bits_until_update -= 1;
            if m.bits_until_update == 0 {
                m.update();
            }
        }

        fn encode_symbol(&mut self, m: &mut SymbolModel, sym: u32) {
            let init_base = self.base;
            let full_length = self.length;
            self.length >>= DM_LENGTH_SHIFT;
            let x = (m.distribution[sym as usize] as u64 * self.length as u64) as u32;
            self.base = self.base.wrapping_add(x);
            let hi = if sym + 1 == m.symbols {
                full_length
            } else {
                (m.distribution[sym as usize + 1] as u64 * self.length as u64) as u32
            };
            self.length = hi - x;
            if init_base > self.base {
                self.propagate_carry();
            }
            if self.length < AC_MIN_LENGTH {
                self.renorm();
            }
            m.symbol_count[sym as usize] += 1;
            m.symbols_until_update -= 1;
            if m.symbols_until_update == 0 {
                m.update();
            }
        }

        fn write_bits(&mut self, bits: u32, sym: u32) {
            if bits > 19 {
                self.write_short(sym & 0xFFFF);
                self.write_bits(bits - 16, sym >> 16);
                return;
            }
            let init_base = self.base;
            self.length >>= bits;
            self.base = self.base.wrapping_add(sym * self.length);
            if init_base > self.base {
                self.propagate_carry();
            }
            if self.length < AC_MIN_LENGTH {
                self.renorm();
            }
        }

        fn write_short(&mut self, sym: u32) {
            let init_base = self.base;
            self.length >>= 16;
            self.base = self.base.wrapping_add(sym * self.length);
            if init_base > self.base {
                self.propagate_carry();
            }
            if self.length < AC_MIN_LENGTH {
                self.renorm();
            }
        }

        fn done(&mut self) {
            let init_base = self.base;
            let mut another_byte = true;
            if self.length > 2 * AC_MIN_LENGTH {
                self.base = self.base.wrapping_add(AC_MIN_LENGTH);
                self.length = AC_MIN_LENGTH >> 1;
            } else {
                self.base = self.base.wrapping_add(AC_MIN_LENGTH >> 1);
                self.length = AC_MIN_LENGTH >> 9;
                another_byte = false;
            }
            if init_base > self.base {
                self.propagate_carry();
            }
            self.renorm();
            self.out.push(0);
            self.out.push(0);
            if another_byte {
                self.out.push(0);
            }
        }
    }

    #[test]
    fn arithmetic_roundtrip_symbols() {
        let mut enc = TestEncoder::new();
        let mut m = SymbolModel::new(64);
        m.init();
        let values: Vec<u32> = vec![3, 17, 42, 0, 63, 8, 8, 55, 21, 3];
        for &v in &values {
            enc.encode_symbol(&mut m, v);
        }
        enc.done();

        let mut dec = AcDecoder::init(&enc.out, 0).unwrap();
        let mut m2 = SymbolModel::new(64);
        m2.init();
        for &v in &values {
            assert_eq!(dec.decode_symbol(&mut m2).unwrap(), v);
        }
    }

    #[test]
    fn arithmetic_roundtrip_bits() {
        let mut enc = TestEncoder::new();
        let mut m = BitModel::new();
        let bits: Vec<u32> = vec![0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0];
        for &b in &bits {
            enc.encode_bit(&mut m, b);
        }
        enc.done();

        let mut dec = AcDecoder::init(&enc.out, 0).unwrap();
        let mut m2 = BitModel::new();
        for &b in &bits {
            assert_eq!(dec.decode_bit(&mut m2).unwrap(), b);
        }
    }

    #[test]
    fn integer_compressor_roundtrip() {
        struct IcEncoder {
            corr_range: u32,
            corr_min: i32,
            corr_max: i32,
            bits_high: u32,
            k: u32,
            m_bits: Vec<SymbolModel>,
            m_corrector0: BitModel,
            m_corrector: Vec<SymbolModel>,
        }
        impl IcEncoder {
            fn new(bits: u32, contexts: u32, bits_high: u32) -> Self {
                let (corr_bits, corr_range, corr_min, corr_max) = if bits < 32 {
                    let r = 1u32 << bits;
                    (bits, r, -((r / 2) as i32), -((r / 2) as i32) + r as i32 - 1)
                } else {
                    (32u32, 0u32, i32::MIN, i32::MAX)
                };
                IcEncoder {
                    corr_range,
                    corr_min,
                    corr_max,
                    bits_high,
                    k: 0,
                    m_bits: (0..contexts)
                        .map(|_| SymbolModel::new(corr_bits + 1))
                        .collect(),
                    m_corrector0: BitModel::new(),
                    m_corrector: (1..=corr_bits)
                        .map(|i| {
                            if i <= bits_high {
                                SymbolModel::new(1 << i)
                            } else {
                                SymbolModel::new(1 << bits_high)
                            }
                        })
                        .collect(),
                }
            }
            fn init(&mut self) {
                for m in &mut self.m_bits {
                    m.init();
                }
                self.m_corrector0.init();
                for m in &mut self.m_corrector {
                    m.init();
                }
            }
            fn compress(&mut self, enc: &mut TestEncoder, pred: i32, real: i32, ctx: u32) {
                let mut corr = real.wrapping_sub(pred);
                if corr < self.corr_min {
                    corr = corr.wrapping_add(self.corr_range as i32);
                } else if corr > self.corr_max {
                    corr = corr.wrapping_sub(self.corr_range as i32);
                }
                self.write_corrector(enc, corr, ctx);
            }
            fn write_corrector(&mut self, enc: &mut TestEncoder, c: i32, ctx: u32) {
                let c1 = if c <= 0 {
                    (c as i64).wrapping_neg()
                } else {
                    c as i64 - 1
                };
                let mut k = 0u32;
                let mut c1u = c1 as u64;
                while c1u != 0 {
                    c1u >>= 1;
                    k += 1;
                }
                self.k = k;
                {
                    let m = &mut self.m_bits[ctx as usize];
                    enc.encode_symbol(m, k);
                }
                if k != 0 {
                    if k < 32 {
                        let mut cv = c as i64;
                        if c < 0 {
                            cv += (1i64 << k) - 1;
                        } else {
                            cv -= 1;
                        }
                        if k <= self.bits_high {
                            let m = &mut self.m_corrector[k as usize - 1];
                            enc.encode_symbol(m, cv as u32);
                        } else {
                            let k1 = k - self.bits_high;
                            let c1 = cv & ((1i64 << k1) - 1);
                            let hi = cv >> k1;
                            let m = &mut self.m_corrector[k as usize - 1];
                            enc.encode_symbol(m, hi as u32);
                            enc.write_bits(k1, c1 as u32);
                        }
                    }
                } else {
                    enc.encode_bit(&mut self.m_corrector0, c as u32);
                }
            }
        }

        let mut enc = TestEncoder::new();
        let mut ic = IcEncoder::new(32, 2, 8);
        ic.init();
        let pairs: Vec<(i32, i32, u32)> = vec![
            (100, 110, 0),
            (110, 95, 0),
            (200, 5000, 1),
            (5000, -12345, 1),
            (0, 1, 0),
        ];
        for &(p, r, c) in &pairs {
            ic.compress(&mut enc, p, r, c);
        }
        enc.done();

        let mut dec = AcDecoder::init(&enc.out, 0).unwrap();
        let mut icd = IntegerCompressor::new(32, 2, 8);
        icd.init();
        for &(p, r, c) in &pairs {
            assert_eq!(icd.decompress(&mut dec, p, c).unwrap(), r);
        }
    }

    #[test]
    fn symbol_model_uniform_distribution() {
        let mut m = SymbolModel::new(4);
        m.init();
        assert_eq!(m.distribution, vec![0, 8192, 16384, 24576]);
    }
}
