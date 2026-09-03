use std::str::from_utf8;

#[derive(Debug)]
pub struct FitsHeader {
    cards: Vec<(String, String)>,
}

impl FitsHeader {
    pub fn parse(buf: &[u8], hdu_start: usize) -> Option<(Self, usize)> {
        let mut cards = Vec::new();
        let mut off = hdu_start;
        let mut end_found = false;
        while off + 80 <= buf.len() {
            let card = &buf[off..off + 80];
            let kw = from_utf8(&card[0..8]).ok()?.trim().to_string();
            if kw == "END" {
                end_found = true;
                off += 80;
                break;
            }
            if !kw.is_empty() {
                let raw = from_utf8(&card[10..]).ok()?.trim().to_string();
                let value = if let Some(q) = raw.strip_prefix('\'') {
                    let mut v = String::new();
                    let mut chars = q.chars().peekable();
                    while let Some(c) = chars.next() {
                        if c == '\'' {
                            if chars.peek() == Some(&'\'') {
                                v.push('\'');
                                chars.next();
                            } else {
                                break;
                            }
                        } else {
                            v.push(c);
                        }
                    }
                    format!("'{}'", v)
                } else {
                    raw.split('/').next().unwrap_or("").trim().to_string()
                };
                cards.push((kw, value));
            }
            off += 80;
        }
        if !end_found {
            return None;
        }
        let rel = off - hdu_start;
        let aligned = hdu_start + rel.div_ceil(2880) * 2880;
        Some((Self { cards }, aligned))
    }

    pub fn value(&self, key: &str) -> Option<&str> {
        self.cards
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    pub fn int(&self, key: &str) -> Option<i64> {
        self.value(key).and_then(|v| v.parse().ok())
    }

    pub fn f64(&self, key: &str) -> Option<f64> {
        self.value(key).and_then(|v| v.parse().ok())
    }

    pub fn str_unescaped(&self, key: &str) -> Option<String> {
        self.value(key).map(|v| {
            let inner = v
                .strip_prefix('\'')
                .unwrap_or(v)
                .strip_suffix('\'')
                .unwrap_or(v);
            inner.replace("''", "'")
        })
    }
}

#[derive(Debug)]
pub struct FitsColumn {
    pub name: String,
    pub code: char,
    pub repeat: usize,
    pub width: usize,
    pub tbcol: usize,
    pub tscal: f64,
    pub tzero: f64,
}

#[derive(Debug)]
pub struct FitsTable {
    pub n_rows: usize,
    pub row_bytes: usize,
    pub heap_bytes: usize,
    pub columns: Vec<FitsColumn>,
    pub data_start: usize,
}

fn code_width(code: char) -> Option<usize> {
    match code {
        'E' => Some(4),
        'D' => Some(8),
        'J' => Some(4),
        'I' => Some(2),
        'K' => Some(8),
        'B' => Some(1),
        'L' => Some(1),
        'A' => Some(1),
        'P' => Some(8),
        'Q' => Some(16),
        _ => None,
    }
}

fn parse_tform(tform: &str) -> Option<(char, usize, Option<char>)> {
    let inner = match tform.strip_suffix(')') {
        Some(i) => i.rsplit_once('(')?.0,
        None => tform,
    };
    let bytes = inner.as_bytes();
    let last = *bytes.last()?;
    if bytes.len() >= 2 && (bytes[bytes.len() - 2] == b'P' || bytes[bytes.len() - 2] == b'Q') {
        let repeat: usize = if bytes.len() >= 3 {
            from_utf8(&bytes[..bytes.len() - 2]).ok()?.parse().ok()?
        } else {
            1
        };
        return Some((bytes[bytes.len() - 2] as char, repeat, Some(last as char)));
    }
    let repeat: usize = if bytes.len() == 1 {
        1
    } else {
        from_utf8(&bytes[..bytes.len() - 1]).ok()?.parse().ok()?
    };
    Some((last as char, repeat, None))
}

impl FitsTable {
    pub fn parse(buf: &[u8], hdu_start: usize) -> Option<(Self, usize)> {
        let (header, data_start) = FitsHeader::parse(buf, hdu_start)?;
        if header.value("XTENSION") != Some("'BINTABLE'") {
            return None;
        }
        let row_bytes = header.int("NAXIS1")? as usize;
        let n_rows = header.int("NAXIS2")? as usize;
        let heap_bytes = header.int("PCOUNT").unwrap_or(0) as usize;
        let tfields = header.int("TFIELDS").unwrap_or(0) as usize;
        let mut columns = Vec::new();
        let mut next_tbcol = 1usize;
        for i in 1..=tfields {
            let name = header
                .str_unescaped(&format!("TTYPE{}", i))
                .unwrap_or_default()
                .trim()
                .to_string();
            let tform = header
                .value(&format!("TFORM{}", i))?
                .trim_matches('\'')
                .trim()
                .to_string();
            let (code, repeat, _elem) = parse_tform(&tform)?;
            let width = match code {
                'P' => 8,
                'Q' => 16,
                _ => code_width(code)? * repeat,
            };
            let tbcol = match header.int(&format!("TBCOL{}", i)) {
                Some(t) if t > 0 => t as usize,
                _ => next_tbcol,
            };
            if tbcol == 0 || tbcol + width > row_bytes + 1 {
                return None;
            }
            next_tbcol = tbcol + width;
            let tscal = header.f64(&format!("TSCAL{}", i)).unwrap_or(1.0);
            let tzero = header.f64(&format!("TZERO{}", i)).unwrap_or(0.0);
            columns.push(FitsColumn {
                name,
                code,
                repeat,
                width,
                tbcol,
                tscal,
                tzero,
            });
        }
        if columns.is_empty() {
            return None;
        }
        let table_bytes = row_bytes * n_rows;
        if data_start + table_bytes > buf.len() {
            return None;
        }
        let next = data_start + table_bytes + heap_bytes;
        let next_aligned = hdu_start + (next - hdu_start).div_ceil(2880) * 2880;
        Some((
            Self {
                n_rows,
                row_bytes,
                heap_bytes,
                columns,
                data_start,
            },
            next_aligned,
        ))
    }

    pub fn column(&self, name: &str) -> Option<&FitsColumn> {
        self.columns.iter().find(|c| c.name == name)
    }

    fn cell_offset(&self, row: usize, col: &FitsColumn) -> Option<usize> {
        if row >= self.n_rows {
            return None;
        }
        let base = self.data_start + row * self.row_bytes + col.tbcol - 1;
        if base + col.width > self.data_start + self.n_rows * self.row_bytes {
            return None;
        }
        Some(base)
    }

    pub fn cell_f64(&self, buf: &[u8], row: usize, col: &FitsColumn) -> Option<f64> {
        let off = self.cell_offset(row, col)?;
        let end = off + col.width;
        let raw = buf.get(off..end)?;
        let v = match col.code {
            'D' => f64::from_be_bytes(raw.try_into().ok()?),
            'E' => f32::from_be_bytes(raw[..4].try_into().ok()?) as f64,
            'J' => i32::from_be_bytes(raw[..4].try_into().ok()?) as f64,
            'I' => i16::from_be_bytes(raw[..2].try_into().ok()?) as f64,
            'K' => i64::from_be_bytes(raw.try_into().ok()?) as f64,
            'B' => raw[0] as f64,
            _ => return None,
        };
        Some(v * col.tscal + col.tzero)
    }

    pub fn cell_i64(&self, buf: &[u8], row: usize, col: &FitsColumn) -> Option<i64> {
        let off = self.cell_offset(row, col)?;
        let end = off + col.width;
        let raw = buf.get(off..end)?;
        match col.code {
            'J' => Some(i32::from_be_bytes(raw[..4].try_into().ok()?) as i64),
            'I' => Some(i16::from_be_bytes(raw[..2].try_into().ok()?) as i64),
            'K' => Some(i64::from_be_bytes(raw.try_into().ok()?)),
            'B' => Some(raw[0] as i64),
            _ => None,
        }
    }

    pub fn cell_array_f64(&self, buf: &[u8], row: usize, col: &FitsColumn) -> Option<Vec<f64>> {
        let off = self.cell_offset(row, col)?;
        let elem = code_width(col.code)?;
        let n = col.repeat;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let start = off + i * elem;
            let raw = buf.get(start..start + elem)?;
            let v = match col.code {
                'D' => f64::from_be_bytes(raw.try_into().ok()?),
                'E' => f32::from_be_bytes(raw[..4].try_into().ok()?) as f64,
                'J' => i32::from_be_bytes(raw[..4].try_into().ok()?) as f64,
                'I' => i16::from_be_bytes(raw[..2].try_into().ok()?) as f64,
                'K' => i64::from_be_bytes(raw.try_into().ok()?) as f64,
                'B' => raw[0] as f64,
                _ => return None,
            };
            out.push(v * col.tscal + col.tzero);
        }
        Some(out)
    }

    pub fn cell_array_i64(&self, buf: &[u8], row: usize, col: &FitsColumn) -> Option<Vec<i64>> {
        let off = self.cell_offset(row, col)?;
        let elem = code_width(col.code)?;
        let n = col.repeat;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let start = off + i * elem;
            let raw = buf.get(start..start + elem)?;
            let v = match col.code {
                'J' => i32::from_be_bytes(raw[..4].try_into().ok()?) as i64,
                'I' => i16::from_be_bytes(raw[..2].try_into().ok()?) as i64,
                'K' => i64::from_be_bytes(raw.try_into().ok()?),
                'B' => raw[0] as i64,
                _ => return None,
            };
            out.push(v);
        }
        Some(out)
    }

    pub fn heap_start(&self) -> usize {
        self.data_start + self.n_rows * self.row_bytes
    }

    pub fn cell_varlen<'a>(&self, buf: &'a [u8], row: usize, col: &FitsColumn) -> Option<&'a [u8]> {
        let off = self.cell_offset(row, col)?;
        let raw = buf.get(off..off + col.width)?;
        let (count, hoff): (usize, usize) = match col.code {
            'P' => {
                let n = u32::from_be_bytes(raw[..4].try_into().ok()?) as usize;
                let o = u32::from_be_bytes(raw[4..8].try_into().ok()?) as usize;
                (n, o)
            }
            'Q' => {
                let n = i64::from_be_bytes(raw[..8].try_into().ok()?) as usize;
                let o = i64::from_be_bytes(raw[8..16].try_into().ok()?) as usize;
                (n, o)
            }
            _ => return None,
        };
        let start = self.heap_start() + hoff;
        buf.get(start..start.checked_add(count)?)
    }
}

#[derive(Debug)]
pub struct FitsWcs {
    ctype1: String,
    ctype2: String,
    crval1: f64,
    crval2: f64,
    crpix1: f64,
    crpix2: f64,
    cd: [[f64; 2]; 2],
}

impl FitsWcs {
    pub fn from_header(h: &FitsHeader, naxis1: usize, naxis2: usize) -> Option<Self> {
        let ctype1 = h.str_unescaped("CTYPE1")?;
        let ctype2 = h.str_unescaped("CTYPE2")?;
        let crval1 = h.f64("CRVAL1")?;
        let crval2 = h.f64("CRVAL2")?;
        let crpix1 = h.f64("CRPIX1").unwrap_or((naxis1 + 1) as f64 / 2.0);
        let crpix2 = h.f64("CRPIX2").unwrap_or((naxis2 + 1) as f64 / 2.0);
        let cd = match (
            h.f64("CD1_1"),
            h.f64("CD1_2"),
            h.f64("CD2_1"),
            h.f64("CD2_2"),
        ) {
            (Some(a), Some(b), Some(c), Some(d)) => [[a, b], [c, d]],
            _ => {
                let cdelt1 = h.f64("CDELT1").unwrap_or(1.0);
                let cdelt2 = h.f64("CDELT2").unwrap_or(1.0);
                [[cdelt1, 0.0], [0.0, cdelt2]]
            }
        };
        Some(Self {
            ctype1,
            ctype2,
            crval1,
            crval2,
            crpix1,
            crpix2,
            cd,
        })
    }

    pub fn world(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        let dx = x - self.crpix1;
        let dy = y - self.crpix2;
        let xi = self.cd[0][0] * dx + self.cd[0][1] * dy;
        let eta = self.cd[1][0] * dx + self.cd[1][1] * dy;
        if !self.ctype1.contains("TAN") && !self.ctype2.contains("TAN") {
            return Some((self.crval1 + xi, self.crval2 + eta));
        }
        let xi = xi.to_radians();
        let eta = eta.to_radians();
        let rho2 = xi * xi + eta * eta;
        let ra0 = self.crval1.to_radians();
        let dec0 = self.crval2.to_radians();
        if rho2 == 0.0 {
            return Some((self.crval1, self.crval2));
        }
        let rho = rho2.sqrt();
        let c = rho.atan();
        let dec = (c.cos() * dec0.sin() + eta * c.sin() * dec0.cos() / rho).asin();
        let ra =
            ra0 + (xi * c.sin()).atan2(rho * dec0.cos() * c.cos() - eta * dec0.sin() * c.sin());
        Some((ra.to_degrees(), dec.to_degrees()))
    }
}

#[derive(Debug)]
pub struct FitsImage {
    pub bitpix: i32,
    pub dims: [usize; 3],
    pub data_start: usize,
    bscale: f64,
    bzero: f64,
    wcs: Option<FitsWcs>,
}

impl FitsImage {
    pub fn parse(buf: &[u8], hdu_start: usize) -> Option<(Self, usize)> {
        let (header, header_end) = FitsHeader::parse(buf, hdu_start)?;
        let bitpix = header.int("BITPIX")? as i32;
        let bytes_per = match bitpix {
            8 => 1,
            16 => 2,
            32 => 4,
            64 => 8,
            -32 => 4,
            -64 => 8,
            _ => return None,
        };
        let naxis = header.int("NAXIS")? as usize;
        if naxis == 0 || naxis > 3 {
            return None;
        }
        let mut dims = [1usize; 3];
        let mut data_bytes: usize = 1;
        for i in 1..=naxis {
            let d = header.int(&format!("NAXIS{}", i))?;
            if d <= 0 {
                return None;
            }
            dims[i - 1] = d as usize;
            data_bytes = data_bytes.checked_mul(d as usize)?;
        }
        data_bytes = data_bytes.checked_mul(bytes_per)?;
        let data_start = header_end;
        if data_start + data_bytes > buf.len() {
            return None;
        }
        let next = hdu_start + (data_start - hdu_start + data_bytes).div_ceil(2880) * 2880;
        let bscale = header.f64("BSCALE").unwrap_or(1.0);
        let bzero = header.f64("BZERO").unwrap_or(0.0);
        let wcs = FitsWcs::from_header(&header, dims[0], dims[1]);
        Some((
            Self {
                bitpix,
                dims,
                data_start,
                bscale,
                bzero,
                wcs,
            },
            next,
        ))
    }

    pub fn value_f64(&self, buf: &[u8], idx: [usize; 3]) -> Option<f64> {
        if idx[0] >= self.dims[0] || idx[1] >= self.dims[1] || idx[2] >= self.dims[2] {
            return None;
        }
        let linear = (idx[2] * self.dims[1] + idx[1]) * self.dims[0] + idx[0];
        let bytes_per = match self.bitpix {
            8 => 1,
            16 => 2,
            32 => 4,
            64 => 8,
            -32 => 4,
            _ => 8,
        };
        let off = self.data_start + linear * bytes_per;
        let raw = buf.get(off..off + bytes_per)?;
        let v = match self.bitpix {
            8 => raw[0] as f64,
            16 => i16::from_be_bytes(raw.try_into().ok()?) as f64,
            32 => i32::from_be_bytes(raw.try_into().ok()?) as f64,
            64 => i64::from_be_bytes(raw.try_into().ok()?) as f64,
            -32 => f32::from_be_bytes(raw.try_into().ok()?) as f64,
            _ => f64::from_be_bytes(raw.try_into().ok()?),
        };
        Some(v * self.bscale + self.bzero)
    }

    pub fn world(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        self.wcs.as_ref().and_then(|w| w.world(x, y))
    }
}

struct RiceBits<'a> {
    buf: &'a [u8],
    pos: usize,
    acc: u64,
    nbits: u32,
}

impl<'a> RiceBits<'a> {
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
}

pub fn rice_decompress(
    data: &[u8],
    n_pixels: usize,
    bytepix: usize,
    block: usize,
) -> Option<Vec<u8>> {
    let (fsbits, fsmax, bbits, first_bytes, mask): (u32, u64, u32, usize, u64) = match bytepix {
        1 => (3, 6, 8, 1, 0xFF),
        2 => (4, 14, 16, 2, 0xFFFF),
        4 => (5, 25, 32, 4, 0xFFFF_FFFF),
        _ => return None,
    };
    if block == 0 {
        return None;
    }
    let mut r = RiceBits::new(data);
    let first: u64 = r.read(first_bytes as u32 * 8)?;
    let mut lastpix = first;
    let mut out = Vec::with_capacity(n_pixels * bytepix);
    let push = |v: u64, out: &mut Vec<u8>| match bytepix {
        1 => out.push(v as u8),
        2 => out.extend_from_slice(&(v as u16).to_be_bytes()),
        _ => out.extend_from_slice(&(v as u32).to_be_bytes()),
    };
    let mut i = 0usize;
    while i < n_pixels {
        let thisblock = block.min(n_pixels - i);
        let fs_val = r.read(fsbits)?;
        let fs = fs_val as i64 - 1;
        if fs < 0 {
            for _ in 0..thisblock {
                push(lastpix, &mut out);
                i += 1;
            }
        } else if fs == fsmax as i64 {
            for _ in 0..thisblock {
                let diff = r.read(bbits)?;
                let m = if diff & 1 == 0 {
                    diff >> 1
                } else {
                    !(diff >> 1)
                };
                lastpix = lastpix.wrapping_add(m) & mask;
                push(lastpix, &mut out);
                i += 1;
            }
        } else {
            let fs = fs as u32;
            for _ in 0..thisblock {
                let nzero = r.unary()?;
                let rem = r.read(fs)?;
                let diff = ((nzero as u64) << fs) | rem;
                let m = if diff & 1 == 0 {
                    diff >> 1
                } else {
                    !(diff >> 1)
                };
                lastpix = lastpix.wrapping_add(m) & mask;
                push(lastpix, &mut out);
                i += 1;
            }
        }
    }
    Some(out)
}

#[derive(Debug)]
pub struct FitsCompressedImage {
    pub zbitpix: i32,
    pub dims: [usize; 3],
    pub tile: [usize; 3],
    pub cmptype: String,
    pub zscale: f64,
    pub zzero: f64,
    pub blank: Option<i64>,
    pub block: usize,
    pub bytepix: usize,
    pub crpix1: f64,
    pub crpix2: f64,
    pub r_sun: f64,
    pub datamean: f64,
    pub totvals: usize,
    table: FitsTable,
}

impl FitsCompressedImage {
    pub fn parse(buf: &[u8], hdu_start: usize) -> Option<(Self, usize)> {
        let (header, _) = FitsHeader::parse(buf, hdu_start)?;
        if header.value("ZIMAGE") != Some("T") {
            return None;
        }
        let (table, next) = FitsTable::parse(buf, hdu_start)?;
        let zbitpix = header.int("ZBITPIX")? as i32;
        let znaxis = header.int("ZNAXIS")? as usize;
        if znaxis == 0 || znaxis > 3 {
            return None;
        }
        let mut dims = [1usize; 3];
        let mut tile = [1usize; 3];
        for i in 1..=znaxis {
            let d = header.int(&format!("ZNAXIS{}", i))?;
            if d <= 0 {
                return None;
            }
            dims[i - 1] = d as usize;
            tile[i - 1] = header.int(&format!("ZTILE{}", i)).unwrap_or(d) as usize;
        }
        let cmptype = header.str_unescaped("ZCMPTYPE").unwrap_or_default();
        let mut block = 32usize;
        let mut bytepix = 0usize;
        for i in 1..=16 {
            let Some(name) = header.str_unescaped(&format!("ZNAME{}", i)) else {
                break;
            };
            let Some(zval) = header.f64(&format!("ZVAL{}", i)) else {
                break;
            };
            match name.trim() {
                "BLOCKSIZE" => block = zval as usize,
                "BYTEPIX" => bytepix = zval as usize,
                _ => {}
            }
        }
        let zscale = header.f64("ZSCALE").unwrap_or(1.0);
        let zzero = header.f64("ZZERO").unwrap_or(0.0);
        let blank = header.int("BLANK");
        let crpix1 = header.f64("CRPIX1").unwrap_or(f64::NAN);
        let crpix2 = header.f64("CRPIX2").unwrap_or(f64::NAN);
        let r_sun = header.f64("R_SUN").unwrap_or(f64::NAN);
        let datamean = header.f64("DATAMEAN").unwrap_or(f64::NAN);
        let totvals = header.int("TOTVALS").unwrap_or(0) as usize;
        Some((
            Self {
                zbitpix,
                dims,
                tile,
                cmptype,
                zscale,
                zzero,
                blank,
                block,
                bytepix,
                crpix1,
                crpix2,
                r_sun,
                datamean,
                totvals,
                table,
            },
            next,
        ))
    }

    pub fn tiles_per_axis(&self, axis: usize) -> usize {
        self.dims[axis].div_ceil(self.tile[axis].max(1))
    }

    pub fn tile_pixels(&self, buf: &[u8], t: [usize; 3]) -> Option<Vec<i64>> {
        for a in 0..3 {
            if t[a] >= self.tiles_per_axis(a) {
                return None;
            }
        }
        let n1 = self.tiles_per_axis(0);
        let n2 = self.tiles_per_axis(1);
        let row = t[0] + n1 * (t[1] + n2 * t[2]);
        let col = self.table.column("COMPRESSED_DATA")?;
        let raw = self.table.cell_varlen(buf, row, col)?;
        if self.cmptype.trim() != "RICE_1" {
            return None;
        }
        let nvals = self.tile[0] * self.tile[1] * self.tile[2];
        let bytes = rice_decompress(raw, nvals, self.bytepix, self.block)?;
        if bytes.len() != nvals * self.bytepix {
            return None;
        }
        let mut out = Vec::with_capacity(nvals);
        match self.bytepix {
            1 => {
                for i in 0..nvals {
                    out.push(bytes[i] as i64);
                }
            }
            2 => {
                for i in 0..nvals {
                    out.push(i16::from_be_bytes([bytes[i * 2], bytes[i * 2 + 1]]) as i64);
                }
            }
            4 => {
                for i in 0..nvals {
                    out.push(i32::from_be_bytes(bytes[i * 4..i * 4 + 4].try_into().ok()?) as i64);
                }
            }
            _ => return None,
        }
        Some(out)
    }

    pub fn pixel_value(&self, raw: i64) -> Option<f64> {
        if let Some(b) = self.blank {
            if raw == b {
                return None;
            }
        }
        Some(raw as f64 * self.zscale + self.zzero)
    }
}

#[cfg(test)]
mod tests {
    use super::{FitsHeader, FitsImage, FitsTable};

    fn pad_card(kw: &str, value: &str) -> [u8; 80] {
        let mut card = [b' '; 80];
        let k = kw.as_bytes();
        card[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        card[8] = b'=';
        let v = value.as_bytes();
        card[10..10 + v.len().min(20)].copy_from_slice(&v[..v.len().min(20)]);
        card
    }

    fn synth() -> Vec<u8> {
        let mut buf = Vec::new();
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&pad_card("SIMPLE", "T"));
        header.extend_from_slice(&pad_card("BITPIX", "8"));
        header.extend_from_slice(&pad_card("NAXIS", "0"));
        header.extend_from_slice(&pad_card("END", ""));
        while header.len() % 2880 != 0 {
            header.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&header);

        let mut ext: Vec<u8> = Vec::new();
        ext.extend_from_slice(&pad_card("XTENSION", "'BINTABLE'"));
        ext.extend_from_slice(&pad_card("BITPIX", "8"));
        ext.extend_from_slice(&pad_card("NAXIS", "2"));
        ext.extend_from_slice(&pad_card("NAXIS1", "12"));
        ext.extend_from_slice(&pad_card("NAXIS2", "3"));
        ext.extend_from_slice(&pad_card("PCOUNT", "0"));
        ext.extend_from_slice(&pad_card("GCOUNT", "1"));
        ext.extend_from_slice(&pad_card("TFIELDS", "2"));
        ext.extend_from_slice(&pad_card("TTYPE1", "'FLUX'"));
        ext.extend_from_slice(&pad_card("TFORM1", "E"));
        ext.extend_from_slice(&pad_card("TBCOL1", "1"));
        ext.extend_from_slice(&pad_card("TTYPE2", "'TIME'"));
        ext.extend_from_slice(&pad_card("TFORM2", "D"));
        ext.extend_from_slice(&pad_card("TBCOL2", "5"));
        ext.extend_from_slice(&pad_card("END", ""));
        while ext.len() % 2880 != 0 {
            ext.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&ext);

        let rows: [[u8; 12]; 3] = [
            [4, 3, 2, 1, 63, 240, 0, 0, 0, 0, 0, 0],
            [8, 7, 6, 5, 64, 0, 0, 0, 0, 0, 0, 0],
            [12, 11, 10, 9, 64, 8, 0, 0, 0, 0, 0, 0],
        ];
        for row in rows {
            buf.extend_from_slice(&row);
        }
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn header_roundtrip_and_alignment() {
        let buf = synth();
        let (h, data) = FitsHeader::parse(&buf, 0).unwrap();
        assert_eq!(h.value("SIMPLE"), Some("T"));
        assert_eq!(data, 2880);
    }

    fn synth_implicit() -> Vec<u8> {
        let mut buf = Vec::new();
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&pad_card("SIMPLE", "T"));
        header.extend_from_slice(&pad_card("BITPIX", "8"));
        header.extend_from_slice(&pad_card("NAXIS", "0"));
        header.extend_from_slice(&pad_card("END", ""));
        while header.len() % 2880 != 0 {
            header.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&header);

        let mut ext: Vec<u8> = Vec::new();
        ext.extend_from_slice(&pad_card("XTENSION", "'BINTABLE'"));
        ext.extend_from_slice(&pad_card("BITPIX", "8"));
        ext.extend_from_slice(&pad_card("NAXIS", "2"));
        ext.extend_from_slice(&pad_card("NAXIS1", "12"));
        ext.extend_from_slice(&pad_card("NAXIS2", "2"));
        ext.extend_from_slice(&pad_card("PCOUNT", "0"));
        ext.extend_from_slice(&pad_card("GCOUNT", "1"));
        ext.extend_from_slice(&pad_card("TFIELDS", "2"));
        ext.extend_from_slice(&pad_card("TTYPE1", "'FLUX'"));
        ext.extend_from_slice(&pad_card("TFORM1", "E"));
        ext.extend_from_slice(&pad_card("TTYPE2", "'TIME'"));
        ext.extend_from_slice(&pad_card("TFORM2", "D"));
        ext.extend_from_slice(&pad_card("END", ""));
        while ext.len() % 2880 != 0 {
            ext.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&ext);

        let rows: [[u8; 12]; 2] = [
            [4, 3, 2, 1, 63, 240, 0, 0, 0, 0, 0, 0],
            [8, 7, 6, 5, 64, 0, 0, 0, 0, 0, 0, 0],
        ];
        for row in rows {
            buf.extend_from_slice(&row);
        }
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn bintable_reads_columns_by_tbcol() {
        let buf = synth();
        let (t, _next) = FitsTable::parse(&buf, 2880).unwrap();
        assert_eq!(t.n_rows, 3);
        assert_eq!(t.row_bytes, 12);
        let flux = t.column("FLUX").unwrap();
        let time = t.column("TIME").unwrap();
        assert_eq!(flux.tbcol, 1);
        assert_eq!(time.tbcol, 5);
        let f0 = t.cell_f64(&buf, 0, flux).unwrap();
        assert_eq!(f0, f32::from_le_bytes([1, 2, 3, 4]) as f64);
        let t0 = t.cell_f64(&buf, 0, time).unwrap();
        assert_eq!(t0, f64::from_le_bytes([0, 0, 0, 0, 0, 0, 240, 63]));
        assert_eq!(t.cell_f64(&buf, 2, time).unwrap(), 3.0);
    }

    #[test]
    fn bintable_implicit_packing_without_tbcol() {
        let buf = synth_implicit();
        let (t, _next) = FitsTable::parse(&buf, 2880).unwrap();
        assert_eq!(t.n_rows, 2);
        let flux = t.column("FLUX").unwrap();
        let time = t.column("TIME").unwrap();
        assert_eq!(flux.tbcol, 1);
        assert_eq!(time.tbcol, 5);
        assert_eq!(
            t.cell_f64(&buf, 0, flux).unwrap(),
            f32::from_le_bytes([1, 2, 3, 4]) as f64
        );
        assert_eq!(t.cell_f64(&buf, 1, time).unwrap(), 2.0);
    }

    #[test]
    fn bintable_rejects_wrong_extension() {
        let buf = synth();
        assert!(FitsTable::parse(&buf, 0).is_none());
    }

    fn synth_array_cols() -> Vec<u8> {
        let mut buf = Vec::new();
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&pad_card("SIMPLE", "T"));
        header.extend_from_slice(&pad_card("BITPIX", "8"));
        header.extend_from_slice(&pad_card("NAXIS", "0"));
        header.extend_from_slice(&pad_card("END", ""));
        while header.len() % 2880 != 0 {
            header.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&header);

        let mut ext: Vec<u8> = Vec::new();
        ext.extend_from_slice(&pad_card("XTENSION", "'BINTABLE'"));
        ext.extend_from_slice(&pad_card("BITPIX", "8"));
        ext.extend_from_slice(&pad_card("NAXIS", "2"));
        ext.extend_from_slice(&pad_card("NAXIS1", "24"));
        ext.extend_from_slice(&pad_card("NAXIS2", "2"));
        ext.extend_from_slice(&pad_card("PCOUNT", "0"));
        ext.extend_from_slice(&pad_card("GCOUNT", "1"));
        ext.extend_from_slice(&pad_card("TFIELDS", "2"));
        ext.extend_from_slice(&pad_card("TTYPE1", "'SPEC'"));
        ext.extend_from_slice(&pad_card("TFORM1", "2D"));
        ext.extend_from_slice(&pad_card("TBCOL1", "1"));
        ext.extend_from_slice(&pad_card("TTYPE2", "'DQ'"));
        ext.extend_from_slice(&pad_card("TFORM2", "2J"));
        ext.extend_from_slice(&pad_card("TBCOL2", "17"));
        ext.extend_from_slice(&pad_card("END", ""));
        while ext.len() % 2880 != 0 {
            ext.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&ext);

        let rows: [[u8; 24]; 2] = [
            [
                63, 248, 0, 0, 0, 0, 0, 0, 64, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2,
            ],
            [
                64, 12, 0, 0, 0, 0, 0, 0, 64, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 4,
            ],
        ];
        for row in rows {
            buf.extend_from_slice(&row);
        }
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn bintable_reads_fixed_array_cells() {
        let buf = synth_array_cols();
        let (t, _next) = FitsTable::parse(&buf, 2880).unwrap();
        assert_eq!(t.n_rows, 2);
        assert_eq!(t.row_bytes, 24);
        let spec = t.column("SPEC").unwrap();
        let dq = t.column("DQ").unwrap();
        assert_eq!(spec.repeat, 2);
        assert_eq!(t.cell_array_f64(&buf, 0, spec).unwrap(), vec![1.5, 2.5]);
        assert_eq!(t.cell_array_f64(&buf, 1, spec).unwrap(), vec![3.5, 4.5]);
        assert_eq!(t.cell_array_i64(&buf, 0, dq).unwrap(), vec![1, 2]);
        assert_eq!(t.cell_array_i64(&buf, 1, dq).unwrap(), vec![3, 4]);
        assert!(t.cell_array_f64(&buf, 2, spec).is_none());
    }

    #[test]
    fn bintable_bounds_check() {
        let buf = synth();
        let (t, _) = FitsTable::parse(&buf, 2880).unwrap();
        let flux = t.column("FLUX").unwrap();
        assert!(t.cell_f64(&buf, 3, flux).is_none());
    }

    #[test]
    fn image_reads_f32_cube() {
        let mut buf: Vec<u8> = Vec::new();
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&pad_card("SIMPLE", "T"));
        header.extend_from_slice(&pad_card("BITPIX", "-32"));
        header.extend_from_slice(&pad_card("NAXIS", "3"));
        header.extend_from_slice(&pad_card("NAXIS1", "4"));
        header.extend_from_slice(&pad_card("NAXIS2", "2"));
        header.extend_from_slice(&pad_card("NAXIS3", "2"));
        header.extend_from_slice(&pad_card("GHISTSEQ", "0"));
        header.extend_from_slice(&pad_card("END", ""));
        while header.len() % 2880 != 0 {
            header.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&header);
        let vals: Vec<f32> = (0..16).map(|i| i as f32 + 0.5).collect();
        for v in vals {
            buf.extend_from_slice(&v.to_be_bytes());
        }
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        let (img, next) = FitsImage::parse(&buf, 0).unwrap();
        assert_eq!(img.dims, [4, 2, 2]);
        assert_eq!(next, buf.len());
        assert_eq!(img.value_f64(&buf, [0, 0, 0]).unwrap(), 0.5);
        assert_eq!(img.value_f64(&buf, [3, 1, 1]).unwrap(), 15.5);
        assert!(img.value_f64(&buf, [4, 0, 0]).is_none());
    }

    #[test]
    fn image_reads_past_multiblock_header() {
        let mut buf: Vec<u8> = Vec::new();
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&pad_card("SIMPLE", "T"));
        header.extend_from_slice(&pad_card("BITPIX", "-32"));
        header.extend_from_slice(&pad_card("NAXIS", "2"));
        header.extend_from_slice(&pad_card("NAXIS1", "2"));
        header.extend_from_slice(&pad_card("NAXIS2", "1"));
        for i in 0..40 {
            header.extend_from_slice(&pad_card(&format!("GHIST{:03}", i), "x"));
        }
        header.extend_from_slice(&pad_card("END", ""));
        while header.len() % 2880 != 0 {
            header.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&header);
        for v in [1.0f32, 2.0f32] {
            buf.extend_from_slice(&v.to_be_bytes());
        }
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        let (img, _) = FitsImage::parse(&buf, 0).unwrap();
        assert_eq!(img.value_f64(&buf, [0, 0, 0]).unwrap(), 1.0);
        assert_eq!(img.value_f64(&buf, [1, 0, 0]).unwrap(), 2.0);
    }

    fn img_with(cards: &[(&str, &str)], raw: &[u8]) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&pad_card("SIMPLE", "T"));
        for (k, v) in cards {
            header.extend_from_slice(&pad_card(k, v));
        }
        header.extend_from_slice(&pad_card("END", ""));
        while header.len() % 2880 != 0 {
            header.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&header);
        buf.extend_from_slice(raw);
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn image_bscale_bzero_int16() {
        let mut raw = Vec::new();
        for v in [0i16, 2i16] {
            raw.extend_from_slice(&v.to_be_bytes());
        }
        let buf = img_with(
            &[
                ("BITPIX", "16"),
                ("NAXIS", "2"),
                ("NAXIS1", "2"),
                ("NAXIS2", "1"),
                ("BSCALE", "0.5"),
                ("BZERO", "100"),
            ],
            &raw,
        );
        let (img, _) = FitsImage::parse(&buf, 0).unwrap();
        assert_eq!(img.bitpix, 16);
        assert_eq!(img.value_f64(&buf, [0, 0, 0]).unwrap(), 100.0);
        assert_eq!(img.value_f64(&buf, [1, 0, 0]).unwrap(), 101.0);
    }

    #[test]
    fn image_bitpix8_unsigned() {
        let buf = img_with(
            &[
                ("BITPIX", "8"),
                ("NAXIS", "2"),
                ("NAXIS1", "2"),
                ("NAXIS2", "1"),
            ],
            &[7, 255],
        );
        let (img, _) = FitsImage::parse(&buf, 0).unwrap();
        assert_eq!(img.value_f64(&buf, [0, 0, 0]).unwrap(), 7.0);
        assert_eq!(img.value_f64(&buf, [1, 0, 0]).unwrap(), 255.0);
    }

    #[test]
    fn wcs_tan_center_and_offsets() {
        let buf = img_with(
            &[
                ("BITPIX", "-32"),
                ("NAXIS", "2"),
                ("NAXIS1", "100"),
                ("NAXIS2", "100"),
                ("CTYPE1", "'RA---TAN'"),
                ("CTYPE2", "'DEC--TAN'"),
                ("CRVAL1", "200.0"),
                ("CRVAL2", "45.0"),
                ("CRPIX1", "50.5"),
                ("CRPIX2", "50.5"),
                ("CDELT1", "0.001"),
                ("CDELT2", "0.001"),
            ],
            &[0u8; 40000],
        );
        let (img, _) = FitsImage::parse(&buf, 0).unwrap();
        let (ra, dec) = img.world(50.5, 50.5).unwrap();
        assert!((ra - 200.0).abs() < 1e-9);
        assert!((dec - 45.0).abs() < 1e-9);
        let (ra, dec) = img.world(51.5, 50.5).unwrap();
        assert!((ra - 200.001414213).abs() < 1e-6);
        assert!((dec - 45.0).abs() < 1e-5);
        let (ra, dec) = img.world(50.5, 51.5).unwrap();
        assert!((ra - 200.0).abs() < 1e-9);
        assert!((dec - 45.001).abs() < 1e-6);
    }

    #[test]
    fn wcs_linear_axes() {
        let buf = img_with(
            &[
                ("BITPIX", "-32"),
                ("NAXIS", "2"),
                ("NAXIS1", "8"),
                ("NAXIS2", "8"),
                ("CTYPE1", "'FREQ'"),
                ("CTYPE2", "'TIME'"),
                ("CRVAL1", "100.0"),
                ("CRVAL2", "2000.0"),
                ("CRPIX1", "1.0"),
                ("CRPIX2", "1.0"),
                ("CDELT1", "2.0"),
                ("CDELT2", "-0.5"),
            ],
            &[0u8; 256],
        );
        let (img, _) = FitsImage::parse(&buf, 0).unwrap();
        assert_eq!(img.world(1.0, 1.0).unwrap(), (100.0, 2000.0));
        assert_eq!(img.world(3.0, 1.0).unwrap(), (104.0, 2000.0));
        assert_eq!(img.world(1.0, 3.0).unwrap(), (100.0, 1999.0));
    }

    #[test]
    fn wcs_cd_matrix_overrides_cdelt() {
        let buf = img_with(
            &[
                ("BITPIX", "-32"),
                ("NAXIS", "2"),
                ("NAXIS1", "4"),
                ("NAXIS2", "4"),
                ("CTYPE1", "'RA---TAN'"),
                ("CTYPE2", "'DEC--TAN'"),
                ("CRVAL1", "10.0"),
                ("CRVAL2", "0.0"),
                ("CRPIX1", "2.0"),
                ("CRPIX2", "2.0"),
                ("CDELT1", "9.9"),
                ("CDELT2", "9.9"),
                ("CD1_1", "0.001"),
                ("CD1_2", "0.0"),
                ("CD2_1", "0.0"),
                ("CD2_2", "0.001"),
            ],
            &[0u8; 64],
        );
        let (img, _) = FitsImage::parse(&buf, 0).unwrap();
        let (ra, dec) = img.world(3.0, 2.0).unwrap();
        assert!((ra - 10.001).abs() < 1e-6);
        assert!(dec.abs() < 1e-6);
    }

    fn table_with(cards: &[(&str, &str)], rows: &[[u8; 4]]) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&pad_card("SIMPLE", "T"));
        header.extend_from_slice(&pad_card("BITPIX", "8"));
        header.extend_from_slice(&pad_card("NAXIS", "0"));
        header.extend_from_slice(&pad_card("END", ""));
        while header.len() % 2880 != 0 {
            header.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&header);
        let mut ext: Vec<u8> = Vec::new();
        ext.extend_from_slice(&pad_card("XTENSION", "'BINTABLE'"));
        ext.extend_from_slice(&pad_card("BITPIX", "8"));
        ext.extend_from_slice(&pad_card("NAXIS", "2"));
        ext.extend_from_slice(&pad_card("NAXIS1", "4"));
        ext.extend_from_slice(&pad_card("NAXIS2", &rows.len().to_string()));
        ext.extend_from_slice(&pad_card("PCOUNT", "0"));
        ext.extend_from_slice(&pad_card("GCOUNT", "1"));
        ext.extend_from_slice(&pad_card("TFIELDS", "1"));
        ext.extend_from_slice(&pad_card("TTYPE1", "'FLUX'"));
        ext.extend_from_slice(&pad_card("TFORM1", "J"));
        ext.extend_from_slice(&pad_card("TBCOL1", "1"));
        for (k, v) in cards {
            ext.extend_from_slice(&pad_card(k, v));
        }
        ext.extend_from_slice(&pad_card("END", ""));
        while ext.len() % 2880 != 0 {
            ext.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&ext);
        for row in rows {
            buf.extend_from_slice(row);
        }
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn table_tscal_tzero_scales_integer() {
        let buf = table_with(&[("TSCAL1", "2.0"), ("TZERO1", "10.0")], &[[0, 0, 0, 5]]);
        let (t, _) = FitsTable::parse(&buf, 2880).unwrap();
        let flux = t.column("FLUX").unwrap();
        assert_eq!(t.cell_i64(&buf, 0, flux).unwrap(), 5);
        assert_eq!(t.cell_f64(&buf, 0, flux).unwrap(), 20.0);
    }

    #[test]
    fn table_unscaled_defaults_are_identity() {
        let buf = table_with(&[], &[[0, 0, 0, 5]]);
        let (t, _) = FitsTable::parse(&buf, 2880).unwrap();
        let flux = t.column("FLUX").unwrap();
        assert_eq!(t.cell_f64(&buf, 0, flux).unwrap(), 5.0);
    }
}
