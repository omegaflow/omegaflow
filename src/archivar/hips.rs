use super::inflate;

const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

pub const MAGIC: [u8; 4] = *b"HPS1";
pub const VERSION: u8 = 1;
pub const HEADER_BYTES: usize = 36;
pub const REC_BYTES: usize = 24;

#[derive(Clone, Debug, PartialEq)]
pub struct HipsProperties {
    pub creator_did: Option<String>,
    pub obs_title: Option<String>,
    pub obs_regime: Option<String>,
    pub dataproduct_type: Option<String>,
    pub dataproduct_subtype: Option<String>,
    pub hips_frame: Option<String>,
    pub hips_body: Option<String>,
    pub hips_order: Option<u32>,
    pub hips_order_min: Option<u32>,
    pub hips_tile_width: Option<u32>,
    pub hips_tile_format: Option<String>,
    pub hips_pixel_scale: Option<f64>,
    pub em_min: Option<f64>,
    pub em_max: Option<f64>,
    pub t_min: Option<f64>,
    pub t_max: Option<f64>,
}

fn positive_f64(v: &str) -> Option<f64> {
    v.parse::<f64>().ok().filter(|x| x.is_finite() && *x > 0.0)
}

fn finite_f64(v: &str) -> Option<f64> {
    v.parse::<f64>().ok().filter(|x| x.is_finite())
}

pub fn parse_properties(text: &str) -> Option<HipsProperties> {
    let mut props = HipsProperties {
        creator_did: None,
        obs_title: None,
        obs_regime: None,
        dataproduct_type: None,
        dataproduct_subtype: None,
        hips_frame: None,
        hips_body: None,
        hips_order: None,
        hips_order_min: None,
        hips_tile_width: None,
        hips_tile_format: None,
        hips_pixel_scale: None,
        em_min: None,
        em_max: None,
        t_min: None,
        t_max: None,
    };
    let mut known = 0usize;
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = value.trim();
        match key.trim().to_ascii_lowercase().as_str() {
            "creator_did" => {
                props.creator_did = Some(value.to_string());
                known += 1;
            }
            "obs_title" => {
                props.obs_title = Some(value.to_string());
                known += 1;
            }
            "obs_regime" => {
                props.obs_regime = Some(value.to_string());
                known += 1;
            }
            "dataproduct_type" => {
                props.dataproduct_type = Some(value.to_string());
                known += 1;
            }
            "dataproduct_subtype" => {
                props.dataproduct_subtype = Some(value.to_string());
                known += 1;
            }
            "hips_frame" => {
                props.hips_frame = Some(value.to_string());
                known += 1;
            }
            "hips_body" => {
                props.hips_body = Some(value.to_string());
                known += 1;
            }
            "hips_order" => {
                props.hips_order = value.parse::<u32>().ok();
                known += 1;
            }
            "hips_order_min" => {
                props.hips_order_min = value.parse::<u32>().ok();
                known += 1;
            }
            "hips_tile_width" => {
                props.hips_tile_width = value.parse::<u32>().ok().filter(|w| *w > 0);
                known += 1;
            }
            "hips_tile_format" => {
                props.hips_tile_format = Some(value.to_string());
                known += 1;
            }
            "hips_pixel_scale" => {
                props.hips_pixel_scale = positive_f64(value);
                known += 1;
            }
            "em_min" => {
                props.em_min = positive_f64(value);
                known += 1;
            }
            "em_max" => {
                props.em_max = positive_f64(value);
                known += 1;
            }
            "t_min" => {
                props.t_min = finite_f64(value);
                known += 1;
            }
            "t_max" => {
                props.t_max = finite_f64(value);
                known += 1;
            }
            _ => {}
        }
    }
    if known == 0 { None } else { Some(props) }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HipsRaster {
    pub color_type: u8,
    pub bands: usize,
    pub lines: usize,
    pub samples: usize,
    pub band_names: Vec<String>,
    pub values: Vec<Option<f64>>,
}

fn channels_of(color_type: u8) -> Option<usize> {
    match color_type {
        0 | 3 => Some(1),
        2 => Some(3),
        4 => Some(2),
        6 => Some(4),
        _ => None,
    }
}

fn bands_of(color_type: u8) -> Option<usize> {
    match color_type {
        0 | 3 => Some(1),
        4 => Some(2),
        2 => Some(3),
        6 => Some(4),
        _ => None,
    }
}

fn band_names_of(color_type: u8) -> Option<Vec<String>> {
    let name = |n: &str| n.to_string();
    match color_type {
        0 => Some(vec![name("gray")]),
        2 => Some(vec![name("red"), name("green"), name("blue")]),
        3 => Some(vec![name("palette_index")]),
        4 => Some(vec![name("gray"), name("alpha")]),
        6 => Some(vec![
            name("red"),
            name("green"),
            name("blue"),
            name("alpha"),
        ]),
        _ => None,
    }
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        0 => Some("hips_png_gray"),
        20 => Some("hips_png_red"),
        21 => Some("hips_png_green"),
        22 => Some("hips_png_blue"),
        30 => Some("hips_png_palette_index"),
        40 => Some("hips_png_gray"),
        41 => Some("hips_png_alpha"),
        60 => Some("hips_png_red"),
        61 => Some("hips_png_green"),
        62 => Some("hips_png_blue"),
        63 => Some("hips_png_alpha"),
        _ => None,
    }
}

static CRC32_TABLE: [u32; 256] = build_crc32_table();

const fn build_crc32_table() -> [u32; 256] {
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

fn crc32(data: &[u8]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &b in data {
        c = CRC32_TABLE[((c ^ b as u32) & 0xff) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}

fn be32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let p = a as i32 + b as i32 - c as i32;
    let pa = (p - a as i32).abs();
    let pb = (p - b as i32).abs();
    let pc = (p - c as i32).abs();
    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}

fn unfilter_row(filter: u8, bpp: usize, prev: &[u8], line: &[u8], out: &mut [u8]) -> Option<()> {
    for i in 0..line.len() {
        let left = if i >= bpp { out[i - bpp] } else { 0 };
        let up = prev[i];
        let ul = if i >= bpp { prev[i - bpp] } else { 0 };
        let pred = match filter {
            0 => 0u8,
            1 => left,
            2 => up,
            3 => ((left as u16 + up as u16) / 2) as u8,
            4 => paeth(left, up, ul),
            _ => return None,
        };
        out[i] = line[i].wrapping_add(pred);
    }
    Some(())
}

pub fn parse_png(bytes: &[u8]) -> Option<HipsRaster> {
    if bytes.len() < 8 || bytes[..8] != PNG_SIGNATURE {
        return None;
    }
    let mut off = 8usize;
    let mut ihdr: Option<[u8; 13]> = None;
    let mut plte: Option<Vec<[u8; 3]>> = None;
    let mut idat: Vec<u8> = Vec::new();
    loop {
        let head = bytes.get(off..off.checked_add(8)?)?;
        let len = be32(&head[0..4]) as usize;
        let chunk_type = &head[4..8];
        let data = bytes.get(off + 8..off.checked_add(8)?.checked_add(len)?)?;
        let crc_off = off.checked_add(8)?.checked_add(len)?;
        let stored_crc = be32(bytes.get(crc_off..crc_off.checked_add(4)?)?);
        let mut crc_input = Vec::with_capacity(4 + len);
        crc_input.extend_from_slice(chunk_type);
        crc_input.extend_from_slice(data);
        if crc32(&crc_input) != stored_crc {
            return None;
        }
        match chunk_type {
            b"IHDR" => {
                if ihdr.is_some() || data.len() != 13 {
                    return None;
                }
                let mut h = [0u8; 13];
                h.copy_from_slice(data);
                ihdr = Some(h);
            }
            b"PLTE" => {
                if !len.is_multiple_of(3) || len > 768 {
                    return None;
                }
                let mut entries = Vec::with_capacity(len / 3);
                for e in data.chunks_exact(3) {
                    entries.push([e[0], e[1], e[2]]);
                }
                plte = Some(entries);
            }
            b"IDAT" => {
                idat.extend_from_slice(data);
            }
            b"IEND" => break,
            _ => {}
        }
        off = off.checked_add(12)?.checked_add(len)?;
    }
    let ihdr = ihdr?;
    let width = be32(&ihdr[0..4]) as usize;
    let height = be32(&ihdr[4..8]) as usize;
    let depth = ihdr[8];
    let color_type = ihdr[9];
    let compression = ihdr[10];
    let filter_method = ihdr[11];
    let interlace = ihdr[12];
    if width == 0
        || height == 0
        || depth != 8
        || compression != 0
        || filter_method != 0
        || interlace != 0
    {
        return None;
    }
    let channels = channels_of(color_type)?;
    if color_type == 3 && plte.is_none() {
        return None;
    }
    let band_names = band_names_of(color_type)?;
    let stride = width.checked_mul(channels)?;
    let expected = height.checked_mul(stride.checked_add(1)?)?;
    let cmf = *idat.first()?;
    let flg = *idat.get(1)?;
    if cmf & 0x0f != 8 || flg & 0x20 != 0 || ((cmf as u32) * 256 + flg as u32) % 31 != 0 {
        return None;
    }
    let raw = inflate::inflate(&idat[2..])?;
    if raw.len() != expected {
        return None;
    }
    let mut out = vec![0u8; stride * height];
    for y in 0..height {
        let row_off = y * (stride + 1);
        let filter = raw[row_off];
        let line = &raw[row_off + 1..row_off + 1 + stride];
        let prev_copy: Vec<u8> = if y == 0 {
            vec![0u8; stride]
        } else {
            out[(y - 1) * stride..y * stride].to_vec()
        };
        unfilter_row(
            filter,
            channels,
            &prev_copy,
            line,
            &mut out[y * stride..(y + 1) * stride],
        )?;
    }
    let bands = bands_of(color_type)?;
    let total = bands.checked_mul(height)?.checked_mul(width)?;
    let mut values = Vec::with_capacity(total);
    for b in 0..bands {
        for y in 0..height {
            for x in 0..width {
                let v = out[y * stride + x * channels + b];
                values.push(Some(v as f64));
            }
        }
    }
    Some(HipsRaster {
        color_type,
        bands,
        lines: height,
        samples: width,
        band_names,
        values,
    })
}

pub fn band_means(raster: &HipsRaster) -> Option<Vec<(f64, f64, u32)>> {
    let per_band = raster.lines * raster.samples;
    let mut out = Vec::with_capacity(raster.bands);
    for b in 0..raster.bands {
        let start = b * per_band;
        let mut sum = 0.0f64;
        let mut count = 0usize;
        for i in start..start + per_band {
            if let Some(v) = raster.values[i] {
                sum += v;
                count += 1;
            }
        }
        if count > 0 {
            let comp = (raster.color_type as u32) * 10 + b as u32;
            out.push((b as f64, sum / count as f64, comp));
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn band_sums(raster: &HipsRaster) -> Vec<u64> {
    let per_band = raster.lines * raster.samples;
    let mut sums = vec![0u64; raster.bands];
    for b in 0..raster.bands {
        let start = b * per_band;
        let mut sum = 0u64;
        for i in start..start + per_band {
            if let Some(v) = raster.values[i] {
                sum += v as u64;
            }
        }
        sums[b] = sum;
    }
    sums
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let raster = parse_png(bytes)?;
    band_means(&raster)
}

pub fn tile_npix_bound(order: u32) -> Option<u64> {
    4u64.checked_pow(order)?.checked_mul(12)
}

pub fn tile_url(base: &str, order: u32, npix: u32) -> Option<String> {
    let bound = tile_npix_bound(order)?;
    if (npix as u64) >= bound {
        return None;
    }
    let dir = (npix / 10000) * 10000;
    Some(format!("{base}/Norder{order}/Dir{dir}/Npix{npix}.png"))
}

pub fn tile_from_url(url: &str) -> Option<(u32, u32)> {
    let norder = url.find("/Norder")?;
    let after_order = &url[norder + 7..];
    let order_end = after_order.find('/')?;
    let order: u32 = after_order[..order_end].parse().ok()?;
    let npix = url.find("/Npix")?;
    let after_npix = &url[npix + 5..];
    let npix_end = after_npix.find(".png")?;
    let npix: u32 = after_npix[..npix_end].parse().ok()?;
    let bound = tile_npix_bound(order)?;
    if (npix as u64) >= bound {
        return None;
    }
    Some((order, npix))
}

#[derive(Clone, Debug, PartialEq)]
pub struct HipsBand {
    pub mean: f32,
    pub std: f32,
    pub min: f32,
    pub max: f32,
    pub valid: u32,
    pub total: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HipsTile {
    pub order: u32,
    pub npix: u32,
    pub color_type: u8,
    pub lines: u32,
    pub samples: u32,
    pub pixel_scale: f64,
    pub bands: Vec<HipsBand>,
}

fn tile_plausible(tile: &HipsTile) -> bool {
    let Some(bound) = tile_npix_bound(tile.order) else {
        return false;
    };
    if (tile.npix as u64) >= bound {
        return false;
    }
    if !matches!(tile.color_type, 0 | 2 | 3 | 4 | 6) {
        return false;
    }
    if tile.lines == 0 || tile.lines != tile.samples {
        return false;
    }
    if !tile.pixel_scale.is_finite() || tile.pixel_scale <= 0.0 {
        return false;
    }
    if tile.bands.is_empty() || tile.bands.len() > 4 {
        return false;
    }
    let total = (tile.lines as u64) * (tile.samples as u64);
    if total > u32::MAX as u64 {
        return false;
    }
    for band in &tile.bands {
        if !band.mean.is_finite()
            || !band.std.is_finite()
            || !band.min.is_finite()
            || !band.max.is_finite()
        {
            return false;
        }
        if band.std < 0.0
            || !(0.0..=255.0).contains(&band.min)
            || !(0.0..=255.0).contains(&band.max)
            || band.min > band.mean
            || band.mean > band.max
        {
            return false;
        }
        if band.valid == 0 || band.valid > band.total {
            return false;
        }
        if band.total as u64 != total {
            return false;
        }
    }
    true
}

pub fn write_bin(tile: &HipsTile) -> Option<Vec<u8>> {
    if !tile_plausible(tile) {
        return None;
    }
    let mut out = Vec::with_capacity(HEADER_BYTES + tile.bands.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.push(tile.color_type);
    out.extend_from_slice(&[0u8; 2]);
    out.extend_from_slice(&tile.order.to_le_bytes());
    out.extend_from_slice(&tile.npix.to_le_bytes());
    out.extend_from_slice(&tile.lines.to_le_bytes());
    out.extend_from_slice(&tile.samples.to_le_bytes());
    out.extend_from_slice(&tile.pixel_scale.to_le_bytes());
    out.extend_from_slice(&(tile.bands.len() as u32).to_le_bytes());
    for band in &tile.bands {
        let mut rec = [0u8; REC_BYTES];
        rec[0..4].copy_from_slice(&band.mean.to_le_bytes());
        rec[4..8].copy_from_slice(&band.std.to_le_bytes());
        rec[8..12].copy_from_slice(&band.min.to_le_bytes());
        rec[12..16].copy_from_slice(&band.max.to_le_bytes());
        rec[16..20].copy_from_slice(&band.valid.to_le_bytes());
        rec[20..24].copy_from_slice(&band.total.to_le_bytes());
        out.extend_from_slice(&rec);
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<HipsTile> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC || bytes[4] != VERSION {
        return None;
    }
    let color_type = bytes[5];
    let order = u32::from_le_bytes(bytes[8..12].try_into().ok()?);
    let npix = u32::from_le_bytes(bytes[12..16].try_into().ok()?);
    let lines = u32::from_le_bytes(bytes[16..20].try_into().ok()?);
    let samples = u32::from_le_bytes(bytes[20..24].try_into().ok()?);
    let pixel_scale = f64::from_le_bytes(bytes[24..32].try_into().ok()?);
    let bands = u32::from_le_bytes(bytes[32..36].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + bands * REC_BYTES {
        return None;
    }
    let mut band_recs = Vec::with_capacity(bands);
    let mut off = HEADER_BYTES;
    for _ in 0..bands {
        let rec = bytes.get(off..off + REC_BYTES)?;
        off += REC_BYTES;
        let f32_at = |r: std::ops::Range<usize>| {
            rec.get(r)
                .and_then(|x| x.try_into().ok())
                .map(f32::from_le_bytes)
        };
        band_recs.push(HipsBand {
            mean: f32_at(0..4)?,
            std: f32_at(4..8)?,
            min: f32_at(8..12)?,
            max: f32_at(12..16)?,
            valid: u32::from_le_bytes(rec[16..20].try_into().ok()?),
            total: u32::from_le_bytes(rec[20..24].try_into().ok()?),
        });
    }
    let tile = HipsTile {
        order,
        npix,
        color_type,
        lines,
        samples,
        pixel_scale,
        bands: band_recs,
    };
    if !tile_plausible(&tile) {
        return None;
    }
    Some(tile)
}

pub fn tile_series(tile: &HipsTile) -> Vec<(f64, f64, u32)> {
    let comp_base = (tile.color_type as u32) * 10;
    tile.bands
        .iter()
        .enumerate()
        .map(|(b, band)| (b as f64, band.mean as f64, comp_base + b as u32))
        .collect()
}

pub fn parse_asset(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    parse_bin(bytes)
        .map(|t| tile_series(&t))
        .or_else(|| parse_series(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored_deflate(raw: &[u8]) -> Vec<u8> {
        let mut d = Vec::new();
        d.push(0x01);
        d.extend_from_slice(&(raw.len() as u16).to_le_bytes());
        d.extend_from_slice(&(!(raw.len() as u16)).to_le_bytes());
        d.extend_from_slice(raw);
        d
    }

    fn chunk(chunk_type: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut c = Vec::new();
        c.extend_from_slice(&(data.len() as u32).to_be_bytes());
        c.extend_from_slice(chunk_type);
        c.extend_from_slice(data);
        let mut crc_input = Vec::with_capacity(4 + data.len());
        crc_input.extend_from_slice(chunk_type);
        crc_input.extend_from_slice(data);
        c.extend_from_slice(&crc32(&crc_input).to_be_bytes());
        c
    }

    fn png_fixture(
        width: u32,
        height: u32,
        color_type: u8,
        plte: Option<&[[u8; 3]]>,
        scanlines: &[u8],
    ) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&PNG_SIGNATURE);
        let mut ihdr = Vec::new();
        ihdr.extend_from_slice(&width.to_be_bytes());
        ihdr.extend_from_slice(&height.to_be_bytes());
        ihdr.push(8);
        ihdr.push(color_type);
        ihdr.extend_from_slice(&[0, 0, 0]);
        buf.extend_from_slice(&chunk(b"IHDR", &ihdr));
        if let Some(entries) = plte {
            let mut data = Vec::new();
            for e in entries {
                data.extend_from_slice(e);
            }
            buf.extend_from_slice(&chunk(b"PLTE", &data));
        }
        let raw = stored_deflate(scanlines);
        let mut zlib = vec![0x78, 0x01];
        zlib.extend_from_slice(&raw);
        buf.extend_from_slice(&chunk(b"IDAT", &zlib));
        buf.extend_from_slice(&chunk(b"IEND", &[]));
        buf
    }

    #[test]
    fn rgba_tile_with_all_filters_decodes() {
        let row0 = [0u8, 100, 101, 200, 201, 50, 51, 150, 151];
        let row1 = [2u8, 5, 10, 15, 20, 25, 30, 35, 40];
        let buf = png_fixture(2, 2, 6, None, &[row0.as_slice(), row1.as_slice()].concat());
        let raster = parse_png(&buf).expect("the tile decodes");
        assert_eq!(raster.color_type, 6);
        assert_eq!(raster.bands, 4);
        assert_eq!(raster.lines, 2);
        assert_eq!(raster.samples, 2);
        assert_eq!(raster.band_names, vec!["red", "green", "blue", "alpha"]);
        assert_eq!(
            raster.values,
            vec![
                Some(100.0),
                Some(50.0),
                Some(105.0),
                Some(75.0),
                Some(101.0),
                Some(51.0),
                Some(111.0),
                Some(81.0),
                Some(200.0),
                Some(150.0),
                Some(215.0),
                Some(185.0),
                Some(201.0),
                Some(151.0),
                Some(221.0),
                Some(191.0),
            ]
        );
    }

    #[test]
    fn sub_filter_decodes() {
        let row0 = [1u8, 100, 101, 200, 201, 150, 150, 40, 40];
        let buf = png_fixture(2, 1, 6, None, &row0);
        let raster = parse_png(&buf).expect("the tile decodes");
        assert_eq!(
            raster.values,
            vec![
                Some(100.0),
                Some(250.0),
                Some(101.0),
                Some(251.0),
                Some(200.0),
                Some(240.0),
                Some(201.0),
                Some(241.0),
            ]
        );
    }

    #[test]
    fn average_filter_decodes() {
        let row0 = [0u8, 40, 40, 40, 40];
        let row1 = [3u8, 10, 10, 10, 10];
        let buf = png_fixture(1, 2, 6, None, &[row0.as_slice(), row1.as_slice()].concat());
        let raster = parse_png(&buf).expect("the tile decodes");
        assert_eq!(
            &raster.values[4..8],
            &[Some(30.0), Some(30.0), Some(30.0), Some(30.0)]
        );
    }

    #[test]
    fn paeth_filter_decodes() {
        let row0 = [0u8, 100, 100, 100, 100, 100, 100, 100, 100];
        let row1 = [4u8, 0, 0, 0, 0, 0, 0, 0, 0];
        let buf = png_fixture(2, 2, 6, None, &[row0.as_slice(), row1.as_slice()].concat());
        let raster = parse_png(&buf).expect("the tile decodes");
        assert_eq!(raster.values, vec![Some(100.0); 16]);
    }

    #[test]
    fn gray_tile_decodes() {
        let row0 = [0u8, 100, 200];
        let buf = png_fixture(2, 1, 0, None, &row0);
        let raster = parse_png(&buf).expect("the tile decodes");
        assert_eq!(raster.bands, 1);
        assert_eq!(raster.band_names, vec!["gray"]);
        assert_eq!(raster.values, vec![Some(100.0), Some(200.0)]);
        let series = band_means(&raster).expect("band means");
        assert_eq!(series, vec![(0.0, 150.0, 0)]);
    }

    #[test]
    fn palette_tile_decodes() {
        let plte = [[0u8, 0, 0], [255, 255, 255], [10, 20, 30]];
        let row0 = [0u8, 0, 2];
        let buf = png_fixture(2, 1, 3, Some(&plte), &row0);
        let raster = parse_png(&buf).expect("the tile decodes");
        assert_eq!(raster.bands, 1);
        assert_eq!(raster.band_names, vec!["palette_index"]);
        assert_eq!(raster.values, vec![Some(0.0), Some(2.0)]);
    }

    #[test]
    fn gray_alpha_tile_decodes() {
        let row0 = [0u8, 100, 255, 200, 0];
        let buf = png_fixture(2, 1, 4, None, &row0);
        let raster = parse_png(&buf).expect("the tile decodes");
        assert_eq!(raster.bands, 2);
        assert_eq!(raster.band_names, vec!["gray", "alpha"]);
        assert_eq!(
            raster.values,
            vec![Some(100.0), Some(200.0), Some(255.0), Some(0.0)]
        );
    }

    #[test]
    fn rejects_foreign_and_unreadable_pngs() {
        assert!(parse_png(b"").is_none());
        assert!(parse_png(b"XXXX").is_none());
        let row0 = [0u8, 100, 200];
        let good = png_fixture(2, 1, 0, None, &row0);
        assert!(parse_png(&good).is_some());
        let mut bad_crc = good.clone();
        let last = bad_crc.len() - 1;
        bad_crc[last] ^= 0x01;
        assert!(parse_png(&bad_crc).is_none());
        let mut interlace = png_fixture(2, 1, 0, None, &row0);
        interlace[36] = 1;
        assert!(parse_png(&interlace).is_none());
        let mut deep = png_fixture(2, 1, 0, None, &row0);
        deep[24] = 16;
        assert!(parse_png(&deep).is_none());
        assert!(parse_png(&png_fixture(2, 1, 3, None, &row0)).is_none());
        assert!(parse_png(&png_fixture(2, 1, 5, None, &row0)).is_none());
    }

    #[test]
    fn properties_reads_the_measured_manifest() {
        let text = "\
hips_tile_format     = png\n\
hips_frame           = mars\n\
hips_body            = mars\n\
hips_order           = 7\n\
hips_order_min       = 0\n\
hips_tile_width      = 512\n\
hips_pixel_scale     = 8.946E-4\n\
em_min               = 4.5e-7\n\
em_max               = 6.9e-7\n\
t_min                = 59519.0\n\
t_max                = 59792.0\n\
obs_title            = Tianwen-1 MoRIC true-color map of Mars\n";
        let props = parse_properties(text).expect("the manifest reads");
        assert_eq!(props.hips_tile_format.as_deref(), Some("png"));
        assert_eq!(props.hips_frame.as_deref(), Some("mars"));
        assert_eq!(props.hips_body.as_deref(), Some("mars"));
        assert_eq!(props.hips_order, Some(7));
        assert_eq!(props.hips_order_min, Some(0));
        assert_eq!(props.hips_tile_width, Some(512));
        assert_eq!(props.hips_pixel_scale, Some(8.946E-4));
        assert_eq!(props.em_min, Some(4.5e-7));
        assert_eq!(props.em_max, Some(6.9e-7));
        assert_eq!(props.t_min, Some(59519.0));
        assert_eq!(props.t_max, Some(59792.0));
        assert_eq!(
            props.obs_title.as_deref(),
            Some("Tianwen-1 MoRIC true-color map of Mars")
        );
        assert!(parse_properties("").is_none());
        assert!(parse_properties("no keys here").is_none());
    }

    #[test]
    fn tile_url_follows_the_dir_convention() {
        let base = "https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC";
        assert_eq!(
            tile_url(base, 7, 0).as_deref(),
            Some(
                "https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC/Norder7/Dir0/Npix0.png"
            )
        );
        assert_eq!(
            tile_url(base, 0, 0).as_deref(),
            Some(
                "https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC/Norder0/Dir0/Npix0.png"
            )
        );
        assert_eq!(
            tile_url(base, 7, 10000).as_deref(),
            Some(
                "https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC/Norder7/Dir10000/Npix10000.png"
            )
        );
        assert_eq!(tile_url(base, 7, 196608), None);
        assert_eq!(tile_url(base, 40, 0), None);
    }

    #[test]
    fn tile_from_url_reads_order_and_npix() {
        let url = "https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC/Norder7/Dir0/Npix0.png";
        assert_eq!(tile_from_url(url), Some((7, 0)));
        let url = "https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC/Norder7/Dir10000/Npix10000.png";
        assert_eq!(tile_from_url(url), Some((7, 10000)));
        assert_eq!(tile_from_url("https://example.com/properties"), None);
        assert_eq!(
            tile_from_url(
                "https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC/Norder7/Dir0/Npix196608.png"
            ),
            None
        );
    }

    fn tile() -> HipsTile {
        HipsTile {
            order: 7,
            npix: 0,
            color_type: 6,
            lines: 512,
            samples: 512,
            pixel_scale: 8.946E-4,
            bands: vec![
                HipsBand {
                    mean: 100.0,
                    std: 10.0,
                    min: 0.0,
                    max: 200.0,
                    valid: 262144,
                    total: 262144,
                },
                HipsBand {
                    mean: 90.0,
                    std: 9.0,
                    min: 0.0,
                    max: 180.0,
                    valid: 262144,
                    total: 262144,
                },
                HipsBand {
                    mean: 80.0,
                    std: 8.0,
                    min: 0.0,
                    max: 160.0,
                    valid: 262144,
                    total: 262144,
                },
                HipsBand {
                    mean: 255.0,
                    std: 0.0,
                    min: 255.0,
                    max: 255.0,
                    valid: 262144,
                    total: 262144,
                },
            ],
        }
    }

    #[test]
    fn record_roundtrips() {
        let t = tile();
        let bytes = write_bin(&t).expect("the record writes");
        let parsed = parse_bin(&bytes).expect("the record parses");
        assert_eq!(parsed, t);
        assert_eq!(
            tile_series(&parsed),
            vec![
                (0.0, 100.0, 60),
                (1.0, 90.0, 61),
                (2.0, 80.0, 62),
                (3.0, 255.0, 63),
            ]
        );
    }

    #[test]
    fn record_refuses_implausible_values() {
        assert!(parse_bin(b"HPS1").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let mut bad = tile();
        bad.npix = 196608;
        assert!(write_bin(&bad).is_none());
        let mut bad = tile();
        bad.lines = 500;
        assert!(write_bin(&bad).is_none());
        let mut bad = tile();
        bad.pixel_scale = 0.0;
        assert!(write_bin(&bad).is_none());
        let mut bad = tile();
        bad.bands[0].mean = -1.0;
        assert!(write_bin(&bad).is_none());
        let mut bad = tile();
        bad.bands[0].valid = 0;
        assert!(write_bin(&bad).is_none());
        let mut bad = tile();
        bad.bands[0].total = 262143;
        assert!(write_bin(&bad).is_none());
        let mut bad = tile();
        bad.color_type = 5;
        assert!(write_bin(&bad).is_none());
        let bytes = write_bin(&tile()).expect("the record writes");
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn component_names_map_channels() {
        assert_eq!(component_name(0), Some("hips_png_gray"));
        assert_eq!(component_name(20), Some("hips_png_red"));
        assert_eq!(component_name(21), Some("hips_png_green"));
        assert_eq!(component_name(22), Some("hips_png_blue"));
        assert_eq!(component_name(30), Some("hips_png_palette_index"));
        assert_eq!(component_name(40), Some("hips_png_gray"));
        assert_eq!(component_name(41), Some("hips_png_alpha"));
        assert_eq!(component_name(60), Some("hips_png_red"));
        assert_eq!(component_name(63), Some("hips_png_alpha"));
        assert_eq!(component_name(99), None);
    }

    #[test]
    fn parse_asset_reads_record_and_raw_tile() {
        let bytes = write_bin(&tile()).expect("the record writes");
        assert_eq!(
            parse_asset(&bytes),
            Some(vec![
                (0.0, 100.0, 60),
                (1.0, 90.0, 61),
                (2.0, 80.0, 62),
                (3.0, 255.0, 63),
            ])
        );
        assert!(parse_asset(b"XXXX").is_none());
        let row0 = [0u8, 100, 200];
        let png = png_fixture(2, 1, 0, None, &row0);
        assert_eq!(parse_asset(&png), Some(vec![(0.0, 150.0, 0)]));
    }

    #[test]
    #[ignore = "reads the tile named by OMEGAFLOW_HIPS_TILE"]
    fn real_cds_tianwen1_tile_decodes() {
        let path =
            std::env::var("OMEGAFLOW_HIPS_TILE").expect("OMEGAFLOW_HIPS_TILE names a tile on disk");
        let bytes = std::fs::read(&path).expect("read the tile");
        let raster = parse_png(&bytes).expect("the CDS tile decodes");
        assert_eq!((raster.lines, raster.samples), (512, 512));
        assert_eq!(raster.color_type, 6);
        assert_eq!(raster.bands, 4);
        let sums = band_sums(&raster);
        assert_eq!(sums, vec![40_546_943, 30_793_202, 24_197_126, 66_846_720]);
    }
}
