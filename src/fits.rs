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
        _ => None,
    }
}

fn parse_tform(tform: &str) -> Option<(char, usize)> {
    let code = tform.chars().last()?;
    let repeat: usize = if tform.len() == 1 {
        1
    } else {
        tform[..tform.len() - 1].parse().ok()?
    };
    Some((code, repeat))
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
                .unwrap_or_default();
            let tform = header
                .value(&format!("TFORM{}", i))?
                .trim_start_matches('\'');
            let tform = tform.trim_end_matches('\'');
            let (code, repeat) = parse_tform(tform)?;
            let width = code_width(code)? * repeat;
            let tbcol = match header.int(&format!("TBCOL{}", i)) {
                Some(t) if t > 0 => t as usize,
                _ => next_tbcol,
            };
            if tbcol == 0 || tbcol + width > row_bytes + 1 {
                return None;
            }
            next_tbcol = tbcol + width;
            columns.push(FitsColumn {
                name,
                code,
                repeat,
                width,
                tbcol,
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
        match col.code {
            'D' => Some(f64::from_le_bytes(raw.try_into().ok()?)),
            'E' => Some(f32::from_le_bytes(raw[..4].try_into().ok()?) as f64),
            _ => None,
        }
    }

    pub fn cell_i64(&self, buf: &[u8], row: usize, col: &FitsColumn) -> Option<i64> {
        let off = self.cell_offset(row, col)?;
        let end = off + col.width;
        let raw = buf.get(off..end)?;
        match col.code {
            'J' => Some(i32::from_le_bytes(raw[..4].try_into().ok()?) as i64),
            'I' => Some(i16::from_le_bytes(raw[..2].try_into().ok()?) as i64),
            'K' => Some(i64::from_le_bytes(raw.try_into().ok()?)),
            'B' => Some(raw[0] as i64),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct FitsImage {
    pub bitpix: i32,
    pub dims: [usize; 3],
    pub data_start: usize,
}

impl FitsImage {
    pub fn parse(buf: &[u8], hdu_start: usize) -> Option<(Self, usize)> {
        let (header, header_end) = FitsHeader::parse(buf, hdu_start)?;
        let bitpix = header.int("BITPIX")? as i32;
        if bitpix != -32 && bitpix != -64 {
            return None;
        }
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
        let bytes_per = if bitpix == -32 { 4 } else { 8 };
        data_bytes = data_bytes.checked_mul(bytes_per)?;
        let data_start = header_end;
        if data_start + data_bytes > buf.len() {
            return None;
        }
        let next = hdu_start + (data_start - hdu_start + data_bytes).div_ceil(2880) * 2880;
        Some((
            Self {
                bitpix,
                dims,
                data_start,
            },
            next,
        ))
    }

    pub fn value_f64(&self, buf: &[u8], idx: [usize; 3]) -> Option<f64> {
        if idx[0] >= self.dims[0] || idx[1] >= self.dims[1] || idx[2] >= self.dims[2] {
            return None;
        }
        let linear = (idx[2] * self.dims[1] + idx[1]) * self.dims[0] + idx[0];
        let bytes_per = if self.bitpix == -32 { 4 } else { 8 };
        let off = self.data_start + linear * bytes_per;
        let raw = buf.get(off..off + bytes_per)?;
        match self.bitpix {
            -32 => Some(f32::from_be_bytes(raw.try_into().ok()?) as f64),
            _ => Some(f64::from_be_bytes(raw.try_into().ok()?)),
        }
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
            [1u8, 2, 3, 4, 0, 0, 0, 0, 0, 0, 240, 63],
            [5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 64],
            [9, 10, 11, 12, 0, 0, 0, 0, 0, 0, 8, 64],
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
            [1u8, 2, 3, 4, 0, 0, 0, 0, 0, 0, 240, 63],
            [5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 64],
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
}
