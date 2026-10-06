use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::geo::{COMP_GHSL_BUILT, GeoRec, magic_of, parse_bin, write_bin};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::tiff::{GeoTransform, TiffImage, parse_tiff};
use omegaflow::cdn::upload_release;
use omegaflow::inflate::inflate;
use omegaflow::lsk::days_from_civil;
use std::process::Command;

const NETLOC: &str = "jeodpp.jrc.ec.europa.eu";
const URL: &str = "https://jeodpp.jrc.ec.europa.eu/ftp/jrc-opendata/GHSL/GHS_BUILT_S_GLOBE_R2023A/GHS_BUILT_S_E2020_GLOBE_R2023A_4326_3ss/V1-0/GHS_BUILT_S_E2020_GLOBE_R2023A_4326_3ss_V1_0.zip";
const TILES_DIR: &str = "https://jeodpp.jrc.ec.europa.eu/ftp/jrc-opendata/GHSL/GHS_BUILT_S_GLOBE_R2023A/GHS_BUILT_S_E2020_GLOBE_R2023A_4326_3ss/V1-0/tiles/";
const TILE_PREFIX: &str = "GHS_BUILT_S_E2020_GLOBE_R2023A_4326_3ss_V1_0_";
const FORMAT: &str = "ghsl_built_s";
const DEFAULT_STRIDE: usize = 1;
const EPOCH_YEAR: i64 = 2020;
const EPOCH_MONTH: i64 = 1;
const EPOCH_DAY: i64 = 1;

const SIG_LOCAL: [u8; 4] = *b"PK\x03\x04";
const SIG_CENTRAL: [u8; 4] = *b"PK\x01\x02";
const SIG_EOCD: [u8; 4] = *b"PK\x05\x06";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl_range(url: &str, range: &str, max_time: u32) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("--max-time")
        .arg(max_time.to_string())
        .arg("-r")
        .arg(range)
        .arg(url)
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{url}: range {range} reads no bytes — {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    if out.stdout.is_empty() {
        return Err(format!("{url}: range {range} carries no bytes"));
    }
    Ok(out.stdout)
}

fn le16(b: &[u8], off: usize) -> Option<u16> {
    let s = b.get(off..off + 2)?;
    Some(u16::from_le_bytes([s[0], s[1]]))
}

fn le32(b: &[u8], off: usize) -> Option<u32> {
    let s = b.get(off..off + 4)?;
    Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

fn eocd(tail: &[u8]) -> Result<(usize, u32, u32), String> {
    let mut found = None;
    if tail.len() >= 4 {
        let mut i = tail.len() - 4;
        loop {
            if tail.get(i..i + 4) == Some(&SIG_EOCD[..]) {
                found = Some(i);
                break;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
    let i = found.ok_or("the archive tail carries no EOCD signature")?;
    let entries = le16(tail, i + 10).ok_or("the EOCD carries no entry count")? as usize;
    let cd_size = le32(tail, i + 12).ok_or("the EOCD carries no central-directory size")?;
    let cd_off = le32(tail, i + 16).ok_or("the EOCD carries no central-directory offset")?;
    Ok((entries, cd_off, cd_size))
}

struct ZipMember {
    name: String,
    method: u16,
    compressed: u32,
    uncompressed: u32,
    local_offset: u32,
}

fn zip_members(cd: &[u8], expected: usize) -> Result<Vec<ZipMember>, String> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while off + 46 <= cd.len() {
        if cd.get(off..off + 4) != Some(&SIG_CENTRAL[..]) {
            break;
        }
        let method = le16(cd, off + 10).ok_or("a central-directory entry ends early")?;
        let compressed = le32(cd, off + 20).ok_or("a central-directory entry ends early")?;
        let uncompressed = le32(cd, off + 24).ok_or("a central-directory entry ends early")?;
        let name_len = le16(cd, off + 28).ok_or("a central-directory entry ends early")? as usize;
        let extra_len = le16(cd, off + 30).ok_or("a central-directory entry ends early")? as usize;
        let comment_len =
            le16(cd, off + 32).ok_or("a central-directory entry ends early")? as usize;
        let local_offset = le32(cd, off + 42).ok_or("a central-directory entry ends early")?;
        let name_bytes = cd
            .get(off + 46..off + 46 + name_len)
            .ok_or("a central-directory name ends early")?;
        out.push(ZipMember {
            name: String::from_utf8_lossy(name_bytes).to_string(),
            method,
            compressed,
            uncompressed,
            local_offset,
        });
        off += 46 + name_len + extra_len + comment_len;
    }
    if out.len() != expected {
        return Err(format!(
            "the central directory reads {} members against the EOCD count {expected}",
            out.len()
        ));
    }
    Ok(out)
}

fn local_data_start(header: &[u8], at: usize) -> Result<usize, String> {
    if header.get(0..4) != Some(&SIG_LOCAL[..]) {
        return Err(format!("member at {at} carries no local signature"));
    }
    let name_len = le16(header, 26).ok_or("a local header ends early")? as usize;
    let extra_len = le16(header, 28).ok_or("a local header ends early")? as usize;
    Ok(at + 30 + name_len + extra_len)
}

fn remote_data_start(url: &str, member: &ZipMember) -> Result<usize, String> {
    let at = member.local_offset as usize;
    let end = at.saturating_add(255);
    let head = curl_range(url, &format!("{at}-{end}"), 60)?;
    local_data_start(&head, at)
}

fn inflate_member(member: &ZipMember, payload: &[u8]) -> Result<Vec<u8>, String> {
    let out = match member.method {
        0 => payload.to_vec(),
        8 => inflate(payload).ok_or_else(|| {
            format!(
                "{}: the deflate member ({} B compressed) stays unread",
                member.name, member.compressed
            )
        })?,
        other => {
            return Err(format!(
                "{}: method {other} carries no decoder",
                member.name
            ));
        }
    };
    if member.uncompressed != 0xFFFF_FFFF && out.len() != member.uncompressed as usize {
        return Err(format!(
            "{}: {} bytes against the central directory's {} — the member stays unread",
            member.name,
            out.len(),
            member.uncompressed
        ));
    }
    Ok(out)
}

fn tif_member(members: &[ZipMember]) -> Result<&ZipMember, String> {
    members
        .iter()
        .find(|m| m.name.ends_with(".tif"))
        .ok_or_else(|| "the archive carries no .tif member".to_string())
}

fn read_remote_tif(url: &str) -> Result<(Vec<u8>, String), String> {
    let tail = curl_range(url, "-65557", 60)?;
    let (entries, cd_off, cd_size) = eocd(&tail)?;
    let cd_end = cd_off.saturating_add(cd_size.saturating_sub(1));
    let cd = curl_range(url, &format!("{cd_off}-{cd_end}"), 60)?;
    let members = zip_members(&cd, entries)?;
    let member = tif_member(&members)?;
    let start = remote_data_start(url, member)?;
    let end = start as u64 + member.compressed as u64 - 1;
    let payload = curl_range(url, &format!("{start}-{end}"), 7200)?;
    let tif = inflate_member(member, &payload)?;
    Ok((tif, member.name.clone()))
}

fn read_local_zip(path: &str) -> Result<(Vec<u8>, String), String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    let tail_start = bytes.len().saturating_sub(65557);
    let tail = bytes
        .get(tail_start..)
        .ok_or("the archive is shorter than its tail")?;
    let (entries, cd_off, cd_size) = eocd(tail)?;
    let cd_end = cd_off.saturating_add(cd_size);
    let cd = bytes
        .get(cd_off as usize..cd_end as usize)
        .ok_or("the central directory ends beyond the archive")?;
    let members = zip_members(cd, entries)?;
    let member = tif_member(&members)?;
    let at = member.local_offset as usize;
    let head = bytes
        .get(at..at.saturating_add(255))
        .ok_or("the local header ends beyond the archive")?;
    let start = local_data_start(head, at)?;
    let end = start
        .checked_add(member.compressed as usize)
        .ok_or("the member length overflows")?;
    let payload = bytes
        .get(start..end)
        .ok_or("the member ends beyond the archive")?;
    let tif = inflate_member(member, payload)?;
    Ok((tif, member.name.clone()))
}

fn read_local_tif(path: &str) -> Result<(Vec<u8>, String), String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    Ok((bytes, path.to_string()))
}

fn read_source(args: &[String]) -> Result<(Vec<u8>, String), String> {
    if let Some(path) = arg_value(args, "--tif") {
        return read_local_tif(&path);
    }
    let source = match positional_source(args) {
        Some(s) => s,
        None => URL.to_string(),
    };
    if source.starts_with("http://") || source.starts_with("https://") {
        read_remote_tif(&source)
    } else {
        read_local_zip(&source)
    }
}

fn epoch_tdb() -> Result<f64, String> {
    let lsk: LeapSeconds = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no epoch".to_string())?;
    let days = days_from_civil(EPOCH_YEAR, EPOCH_MONTH, EPOCH_DAY)
        .ok_or_else(|| "the 2020-01-01 epoch stays uncompiled".to_string())?;
    let unix = days as f64 * 86400.0;
    lsk.unix_to_tdb(unix)
        .ok_or_else(|| "the 2020-01-01 epoch stays untranslated".to_string())
}

fn sample_at(pixels: &[u8], off: usize, bits: u16, little: bool) -> Option<f64> {
    match bits {
        8 => pixels.get(off).map(|&b| b as f64),
        16 => {
            let s = pixels.get(off..off + 2)?;
            let v = if little {
                u16::from_le_bytes([s[0], s[1]])
            } else {
                u16::from_be_bytes([s[0], s[1]])
            };
            Some(v as f64)
        }
        32 => {
            let s = pixels.get(off..off + 4)?;
            let v = if little {
                u32::from_le_bytes([s[0], s[1], s[2], s[3]])
            } else {
                u32::from_be_bytes([s[0], s[1], s[2], s[3]])
            };
            Some(v as f64)
        }
        _ => None,
    }
}

fn build_records(
    tif: &[u8],
    source: &str,
    t: f64,
    stride: usize,
    limit: usize,
) -> Result<Vec<GeoRec>, String> {
    let little = match tif.get(0..2) {
        Some(b"II") => true,
        Some(b"MM") => false,
        _ => {
            return Err(format!(
                "{source}: carries no TIFF byte-order mark — the raster stays unread"
            ));
        }
    };
    let img: TiffImage = parse_tiff(tif).ok_or_else(|| {
        format!(
            "{source}: the TIFF arm reads no raster (internal compression or tiling outside its domain) — the per-cell grid stays unwritten"
        )
    })?;
    let geo = img
        .geo
        .as_ref()
        .ok_or_else(|| format!("{source}: carries no geotransform — no cell centre"))?;
    if img.samples_per_pixel != 1 {
        return Err(format!(
            "{source}: {} bands per pixel — a single-band raster is expected",
            img.samples_per_pixel
        ));
    }
    let bits = match img.bits_per_sample.first() {
        Some(b) if *b == 8 || *b == 16 || *b == 32 => *b,
        Some(b) => {
            return Err(format!(
                "{source}: {b} bits per sample carries no integer decoder"
            ));
        }
        None => return Err(format!("{source}: bits_per_sample absent")),
    };
    let bytes_per_sample = (bits / 8) as usize;
    let width = img.width as usize;
    let height = img.height as usize;
    let expected = width
        .checked_mul(height)
        .and_then(|n| n.checked_mul(bytes_per_sample))
        .ok_or_else(|| format!("{source}: the {width}x{height} grid overflows"))?;
    if img.pixels.len() != expected {
        return Err(format!(
            "{source}: {} raster bytes against the {width}x{height}x{bytes_per_sample} grid — the arm reads no common grid",
            img.pixels.len()
        ));
    }
    let nodata = if bits == 8 {
        255.0
    } else if bits == 16 {
        65535.0
    } else {
        4294967295.0
    };
    let mut records = Vec::new();
    let mut row = 0usize;
    while row < height {
        let mut col = 0usize;
        while col < width {
            let off = (row * width + col) * bytes_per_sample;
            if let Some(val) = sample_at(&img.pixels, off, bits, little) {
                if val != nodata {
                    let lat = geo.y0 + (row as f64 + 0.5) * geo.dy;
                    let lon = geo.x0 + (col as f64 + 0.5) * geo.dx;
                    if lat.is_finite()
                        && lon.is_finite()
                        && (-90.0..=90.0).contains(&lat)
                        && (-360.0..=360.0).contains(&lon)
                    {
                        records.push(GeoRec {
                            t,
                            lat,
                            lon,
                            alt: 0.0,
                            freq: 0.0,
                            bin_width: 0.0,
                            val,
                            comp: COMP_GHSL_BUILT,
                            station: 0,
                        });
                        if records.len() >= limit {
                            return Ok(records);
                        }
                    }
                }
            }
            col += stride;
        }
        row += stride;
    }
    Ok(records)
}

fn bt_u16(b: &[u8], off: usize, little: bool) -> Option<u16> {
    let s = b.get(off..off + 2)?;
    Some(if little {
        u16::from_le_bytes([s[0], s[1]])
    } else {
        u16::from_be_bytes([s[0], s[1]])
    })
}

fn bt_u32(b: &[u8], off: usize, little: bool) -> Option<u32> {
    let s = b.get(off..off + 4)?;
    Some(if little {
        u32::from_le_bytes([s[0], s[1], s[2], s[3]])
    } else {
        u32::from_be_bytes([s[0], s[1], s[2], s[3]])
    })
}

fn bt_u64(b: &[u8], off: usize, little: bool) -> Option<u64> {
    let s = b.get(off..off + 8)?;
    let mut a = [0u8; 8];
    a.copy_from_slice(s);
    Some(if little {
        u64::from_le_bytes(a)
    } else {
        u64::from_be_bytes(a)
    })
}

fn bt_f64(b: &[u8], off: usize, little: bool) -> Option<f64> {
    let s = b.get(off..off + 8)?;
    let mut a = [0u8; 8];
    a.copy_from_slice(s);
    Some(if little {
        f64::from_le_bytes(a)
    } else {
        f64::from_be_bytes(a)
    })
}

struct BigTiff {
    little: bool,
    width: u64,
    height: u64,
    bits: u16,
    compression: u16,
    samples: u16,
    planar: u16,
    tile_w: u64,
    tile_h: u64,
    tile_offsets: Vec<u64>,
    tile_counts: Vec<u64>,
    geo: Option<GeoTransform>,
}

fn bt_scalar(tif: &[u8], p: usize, ftype: u16, little: bool) -> Option<u64> {
    match ftype {
        3 => bt_u16(tif, p + 12, little).map(u64::from),
        4 => bt_u32(tif, p + 12, little).map(u64::from),
        16 | 18 => bt_u64(tif, p + 12, little),
        _ => None,
    }
}

fn bt_offset_array(tif: &[u8], p: usize, ftype: u16, count: u64, little: bool) -> Option<Vec<u64>> {
    let size = match ftype {
        4 => 4usize,
        16 | 18 => 8,
        _ => return None,
    };
    let total = size.checked_mul(count as usize)?;
    let mut out = Vec::with_capacity(count as usize);
    if total <= 8 {
        for i in 0..count as usize {
            out.push(bt_scalar(tif, p + i * size, ftype, little)?);
        }
        return Some(out);
    }
    let base = bt_u64(tif, p + 12, little)? as usize;
    for i in 0..count as usize {
        out.push(bt_scalar(tif, base + i * size - 12, ftype, little)?);
    }
    Some(out)
}

fn bt_double_vec(tif: &[u8], p: usize, count: u64, little: bool) -> Option<Vec<f64>> {
    let base = bt_u64(tif, p + 12, little)? as usize;
    let mut out = Vec::with_capacity(count as usize);
    for i in 0..count as usize {
        out.push(bt_f64(tif, base + i * 8, little)?);
    }
    Some(out)
}

fn parse_bigtiff(tif: &[u8]) -> Option<BigTiff> {
    let little = match tif.get(0..2)? {
        b"II" => true,
        b"MM" => false,
        _ => return None,
    };
    if bt_u16(tif, 2, little)? != 43 {
        return None;
    }
    let ifd = bt_u64(tif, 8, little)? as usize;
    let count = bt_u64(tif, ifd, little)?;
    let mut width = None;
    let mut height = None;
    let mut bits = None;
    let mut compression = None;
    let mut samples = None;
    let mut planar = None;
    let mut tile_w = None;
    let mut tile_h = None;
    let mut tile_offsets = None;
    let mut tile_counts = None;
    let mut scale = None;
    let mut tie = None;
    for i in 0..count as usize {
        let p = ifd + 8 + i * 20;
        let tag = bt_u16(tif, p, little)?;
        let ftype = bt_u16(tif, p + 2, little)?;
        let cnt = bt_u64(tif, p + 4, little)?;
        match tag {
            256 => width = bt_scalar(tif, p, ftype, little),
            257 => height = bt_scalar(tif, p, ftype, little),
            258 => bits = bt_scalar(tif, p, ftype, little).map(|v| v as u16),
            259 => compression = bt_scalar(tif, p, ftype, little).map(|v| v as u16),
            277 => samples = bt_scalar(tif, p, ftype, little).map(|v| v as u16),
            284 => planar = bt_scalar(tif, p, ftype, little).map(|v| v as u16),
            322 => tile_w = bt_scalar(tif, p, ftype, little),
            323 => tile_h = bt_scalar(tif, p, ftype, little),
            324 => tile_offsets = bt_offset_array(tif, p, ftype, cnt, little),
            325 => tile_counts = bt_offset_array(tif, p, ftype, cnt, little),
            33550 => scale = bt_double_vec(tif, p, cnt, little),
            33922 => tie = bt_double_vec(tif, p, cnt, little),
            _ => {}
        }
    }
    let (sx, sy) = {
        let s = scale?;
        (s[0], *s.get(1)?)
    };
    let geo = tie.as_ref().and_then(|t| {
        if t.len() < 6 {
            return None;
        }
        Some(GeoTransform {
            x0: t[3] - t[0] * sx,
            y0: t[4] + t[1] * sy,
            dx: sx,
            dy: -sy,
        })
    });
    Some(BigTiff {
        little,
        width: width?,
        height: height?,
        bits: bits?,
        compression: compression.unwrap_or(1),
        samples: samples.unwrap_or(1),
        planar: planar.unwrap_or(1),
        tile_w: tile_w?,
        tile_h: tile_h?,
        tile_offsets: tile_offsets?,
        tile_counts: tile_counts?,
        geo,
    })
}

const LZW_TABLE_MAX: usize = 4096;
const LZW_CLEAR: u16 = 256;
const LZW_EOI: u16 = 257;
const LZW_FIRST: u16 = 258;

fn lzw_code(data: &[u8], bit_pos: &mut usize, width: u32) -> Option<u16> {
    let mut code = 0u32;
    for _ in 0..width {
        let byte = *data.get(*bit_pos >> 3)?;
        let bit = (byte >> (7 - (*bit_pos & 7))) & 1;
        code = (code << 1) | (bit as u32);
        *bit_pos += 1;
    }
    Some(code as u16)
}

fn lzw_decode(data: &[u8], expected: usize) -> Option<Vec<u8>> {
    let mut prefix = vec![0u16; LZW_TABLE_MAX];
    let mut suffix = vec![0u8; LZW_TABLE_MAX];
    for i in 0..256usize {
        suffix[i] = i as u8;
    }
    let mut out = Vec::with_capacity(expected);
    let mut bit_pos = 0usize;
    let mut width = 9u32;
    let mut free = LZW_FIRST;
    let mut prev: Option<u16> = None;
    let mut prev_str: Vec<u8> = Vec::new();
    loop {
        let code = lzw_code(data, &mut bit_pos, width)?;
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

fn sample_tile_value(block: &[u8], off: usize, bits: u16, little: bool) -> Option<f64> {
    match bits {
        8 => block.get(off).map(|&b| b as f64),
        16 => {
            let s = block.get(off..off + 2)?;
            Some(if little {
                u16::from_le_bytes([s[0], s[1]]) as f64
            } else {
                u16::from_be_bytes([s[0], s[1]]) as f64
            })
        }
        32 => {
            let s = block.get(off..off + 4)?;
            Some(if little {
                u32::from_le_bytes([s[0], s[1], s[2], s[3]]) as f64
            } else {
                u32::from_be_bytes([s[0], s[1], s[2], s[3]]) as f64
            })
        }
        _ => None,
    }
}

fn records_from_tile(
    tif: &[u8],
    source: &str,
    t: f64,
    stride: usize,
    limit: usize,
) -> Result<Vec<GeoRec>, String> {
    let bt = parse_bigtiff(tif).ok_or_else(|| {
        format!(
            "{source}: the tile arm reads no BigTIFF raster — the per-cell grid stays unwritten"
        )
    })?;
    if bt.samples != 1 || bt.planar != 1 {
        return Err(format!(
            "{source}: {} bands, planar {} — a single-band chunky raster is expected",
            bt.samples, bt.planar
        ));
    }
    if bt.compression != 5 {
        return Err(format!(
            "{source}: compression {} carries no tile decoder (LZW 5 is the measured arm)",
            bt.compression
        ));
    }
    if bt.bits != 8 && bt.bits != 16 && bt.bits != 32 {
        return Err(format!(
            "{source}: {} bits per sample carries no integer decoder",
            bt.bits
        ));
    }
    let geo = bt
        .geo
        .as_ref()
        .ok_or_else(|| format!("{source}: carries no geotransform — no cell centre"))?;
    let bps = (bt.bits / 8) as usize;
    let across = (bt.width + bt.tile_w - 1) / bt.tile_w;
    let down = (bt.height + bt.tile_h - 1) / bt.tile_h;
    let tiles = (across * down) as usize;
    if bt.tile_offsets.len() < tiles {
        return Err(format!(
            "{source}: {} tile offsets against the {tiles} tile grid",
            bt.tile_offsets.len()
        ));
    }
    let block_bytes = (bt.tile_w as usize)
        .checked_mul(bt.tile_h as usize)
        .and_then(|n| n.checked_mul(bps))
        .ok_or_else(|| format!("{source}: the {}x{} tile overflows", bt.tile_w, bt.tile_h))?;
    let nodata = match bt.bits {
        8 => 255.0,
        16 => 65535.0,
        _ => 4294967295.0,
    };
    let mut records = Vec::new();
    for k in 0..tiles {
        let count = *bt
            .tile_counts
            .get(k)
            .ok_or_else(|| format!("{source}: tile {k} carries no byte count"))?
            as usize;
        let start = bt.tile_offsets[k] as usize;
        let end = start
            .checked_add(count)
            .ok_or_else(|| format!("{source}: tile {k} overflows"))?;
        let strip = tif
            .get(start..end)
            .ok_or_else(|| format!("{source}: tile {k} ends beyond the raster"))?;
        let block = lzw_decode(strip, block_bytes)
            .ok_or_else(|| format!("{source}: tile {k} stays undecoded"))?;
        let tx = (k as u64 % across) * bt.tile_w;
        let ty = (k as u64 / across) * bt.tile_h;
        let gy0 = ty;
        let gy1 = (ty + bt.tile_h).min(bt.height);
        let gx0 = tx;
        let gx1 = (tx + bt.tile_w).min(bt.width);
        let mut gy = gy0.div_ceil(stride as u64) * stride as u64;
        while gy < gy1 {
            let row = (gy - ty) as usize;
            let mut gx = gx0.div_ceil(stride as u64) * stride as u64;
            while gx < gx1 {
                let col = (gx - tx) as usize;
                let off = (row * bt.tile_w as usize + col) * bps;
                let val = sample_tile_value(&block, off, bt.bits, bt.little)
                    .ok_or_else(|| format!("{source}: tile {k} cell {gx},{gy} stays unread"))?;
                if val != nodata {
                    let lat = geo.y0 + (gy as f64 + 0.5) * geo.dy;
                    let lon = geo.x0 + (gx as f64 + 0.5) * geo.dx;
                    if lat.is_finite()
                        && lon.is_finite()
                        && (-90.0..=90.0).contains(&lat)
                        && (-360.0..=360.0).contains(&lon)
                    {
                        records.push(GeoRec {
                            t,
                            lat,
                            lon,
                            alt: 0.0,
                            freq: 0.0,
                            bin_width: 0.0,
                            val,
                            comp: COMP_GHSL_BUILT,
                            station: 0,
                        });
                        if records.len() >= limit {
                            return Ok(records);
                        }
                    }
                }
                gx += stride as u64;
            }
            gy += stride as u64;
        }
    }
    Ok(records)
}

fn curl_get(url: &str, max_time: u32) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("--max-time")
        .arg(max_time.to_string())
        .arg(url)
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{url}: reads no bytes — {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    if out.stdout.is_empty() {
        return Err(format!("{url}: carries no bytes"));
    }
    Ok(out.stdout)
}

fn tile_url_from_rc(spec: &str) -> Option<String> {
    let nums: Vec<u32> = spec
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();
    if nums.len() >= 2 {
        Some(format!(
            "{TILES_DIR}{TILE_PREFIX}R{}_C{}.zip",
            nums[0], nums[1]
        ))
    } else {
        None
    }
}

fn tile_sources(args: &[String]) -> Result<Vec<String>, String> {
    let mut urls: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < args.len() {
        if args[i] == "--tile" {
            match args.get(i + 1).and_then(|v| tile_url_from_rc(v)) {
                Some(u) => urls.push(u),
                None => return Err("--tile carries no R<row>_C<col> cell".to_string()),
            }
            i += 2;
            continue;
        }
        i += 1;
    }
    if let Some(path) = arg_value(args, "--tile-list") {
        let text = std::fs::read_to_string(&path)
            .map_err(|e| format!("read {path} returned void: {e}"))?;
        for line in text.lines() {
            let l = line.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            if l.starts_with("http") {
                urls.push(l.to_string());
            } else if l.ends_with(".zip") && std::path::Path::new(l).exists() {
                urls.push(l.to_string());
            } else if let Some(u) = tile_url_from_rc(l) {
                urls.push(u);
            } else {
                return Err(format!("{path}: {l} carries no tile cell or URL"));
            }
        }
    }
    if let Some(dir) = arg_value(args, "--tiles-dir") {
        let dir = if dir.ends_with('/') {
            dir
        } else {
            format!("{dir}/")
        };
        let html = curl_get(&dir, 120)?;
        let text = String::from_utf8_lossy(&html);
        for tok in text
            .split(|c: char| matches!(c, '"' | '\'' | '<' | '>' | ' ' | '\n' | '\r' | '\t' | '='))
        {
            if let Some(idx) = tok.find(TILE_PREFIX) {
                let name = &tok[idx..];
                if name.ends_with(".zip") {
                    urls.push(format!("{dir}{name}"));
                }
            }
        }
    }
    if urls.is_empty() {
        let html = curl_get(TILES_DIR, 120)?;
        let text = String::from_utf8_lossy(&html);
        for tok in text
            .split(|c: char| matches!(c, '"' | '\'' | '<' | '>' | ' ' | '\n' | '\r' | '\t' | '='))
        {
            if let Some(idx) = tok.find(TILE_PREFIX) {
                let name = &tok[idx..];
                if name.ends_with(".zip") {
                    urls.push(format!("{TILES_DIR}{name}"));
                }
            }
        }
    }
    let mut seen = std::collections::HashSet::new();
    urls.retain(|u| seen.insert(u.clone()));
    if urls.is_empty() {
        return Err(
            "the tiles directory carries no tile name — the harvest stays unwritten".to_string(),
        );
    }
    Ok(urls)
}

fn harvest_tiles(
    args: &[String],
    t: f64,
    stride: usize,
    limit: usize,
) -> Result<Vec<GeoRec>, String> {
    let sources = tile_sources(args)?;
    let mut records = Vec::new();
    for src in &sources {
        let (tif, name) = if src.starts_with("http") {
            read_remote_tif(src)?
        } else {
            read_local_zip(src)?
        };
        let remaining = limit.saturating_sub(records.len());
        let mut tile_records = records_from_tile(&tif, &name, t, stride, remaining)?;
        records.append(&mut tile_records);
        if records.len() >= limit {
            break;
        }
    }
    Ok(records)
}

fn inspect(tif: &[u8]) {
    match parse_tiff(tif) {
        Some(img) => {
            eprintln!(
                "tif {}x{} bits {:?} samples {} compression {}",
                img.width, img.height, img.bits_per_sample, img.samples_per_pixel, img.compression
            );
            match &img.geo {
                Some(g) => eprintln!("geo x0={} y0={} dx={} dy={}", g.x0, g.y0, g.dx, g.dy),
                None => eprintln!("the raster carries no geotransform"),
            }
        }
        None => eprintln!("ghsl: the TIFF arm reads no raster"),
    }
}

fn out_path(args: &[String], netloc: &str) -> String {
    match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{netloc}/ghsl_built_s.bin"),
    }
}

fn netloc_of(args: &[String], source: &str) -> String {
    match arg_value(args, "--netloc") {
        Some(n) if !n.is_empty() => n,
        _ => match source.split("//").nth(1).and_then(|r| r.split('/').next()) {
            Some(h) if !h.is_empty() && source.starts_with("http") => h.to_string(),
            _ => NETLOC.to_string(),
        },
    }
}

fn ensure_parent(out: &str) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    Ok(())
}

fn parse_step(args: &[String], name: &str, default: usize) -> Result<usize, String> {
    match arg_value(args, name) {
        Some(v) => {
            let n = v
                .parse::<usize>()
                .map_err(|_| format!("{name} {v} carries no step"))?;
            if n == 0 {
                return Err(format!("{name} carries no positive step"));
            }
            Ok(n)
        }
        None => Ok(default),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let inspect_mode = args.iter().any(|a| a == "--inspect");
    let tiles_mode = args.iter().any(|a| a == "--tiles");
    let stride = parse_step(args, "--stride", DEFAULT_STRIDE)?;
    let limit = parse_step(args, "--limit", usize::MAX)?;
    let t = epoch_tdb()?;
    if tiles_mode {
        let records = harvest_tiles(args, t, stride, limit)?;
        if records.is_empty() {
            return Err(
                "the tile harvest carried no measured GHSL built-up cell — the bin stays unwritten (0 honored)"
                    .to_string(),
            );
        }
        return emit(args, &records, TILES_DIR, ci_mode);
    }
    let (tif, source) = read_source(args)?;
    if inspect_mode {
        inspect(&tif);
        return Ok(());
    }
    let records = build_records(&tif, &source, t, stride, limit)?;
    if records.is_empty() {
        return Err(format!(
            "{source}: no measured GHSL built-up cell left the harvest — the bin stays unwritten (0 honored)"
        ));
    }
    emit(args, &records, &source, ci_mode)
}

fn emit(args: &[String], records: &[GeoRec], source: &str, ci_mode: bool) -> Result<(), String> {
    let magic = magic_of(FORMAT).ok_or_else(|| {
        format!("{source}: {FORMAT} carries no geo magic — the per-cell arm stays unwritten")
    })?;
    let netloc = netloc_of(args, source);
    let out = out_path(args, &netloc);
    ensure_parent(&out)?;
    let bytes = write_bin(magic, records);
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} void: {e}"))?;
    match parse_bin(magic, &bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            println!(
                "url https://github.com/omegaflow/sources/releases/download/{netloc}/{}",
                last_seg(&out)
            );
            println!("origin {source}");
            println!("compiler tools/harvest/src/bin/ghsl_compiler.rs");
            println!("format {FORMAT}");
            println!("sha256 {}", sha256_hex(&bytes));
            eprintln!(
                "{out}: {} records from the GHSL built-up raster, {} B, roundtrip parses",
                parsed.len(),
                bytes.len()
            );
        }
        Some(parsed) => {
            return Err(format!(
                "{out}: {} parsed vs {} written — the asset stays unverified",
                parsed.len(),
                records.len()
            ));
        }
        None => {
            return Err(format!(
                "{out}: roundtrip parse void — the asset stays unverified"
            ));
        }
    }
    if ci_mode && !upload_release(&netloc, &out) {
        return Err(format!(
            "{out}: CDN upload did not reach the {netloc} release"
        ));
    }
    Ok(())
}

fn last_seg(path: &str) -> &str {
    match path.rsplit('/').next() {
        Some(s) => s,
        None => path,
    }
}

fn positional_source(args: &[String]) -> Option<String> {
    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        if matches!(
            a.as_str(),
            "--tif"
                | "--netloc"
                | "--stride"
                | "--limit"
                | "--out"
                | "--tile"
                | "--tile-list"
                | "--tiles-dir"
        ) {
            i += 2;
            continue;
        }
        if a.starts_with("--") || a == "-h" {
            i += 1;
            continue;
        }
        return Some(a.clone());
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: ghsl_compiler [<zip-url|zip-path>] [--tif <path>] [--tiles] [--tile R_C] [--tile-list <path>] [--tiles-dir <url>] [--netloc <netloc>] [--stride N] [--limit N] [--out <path>] [--ci-mode] [--inspect]"
        );
        eprintln!("  reads the JRC GHSL GHS-BUILT-S E2020 GLOBE R2023A 4326 3ss GeoTIFF");
        eprintln!("  unit: built square metres in the grid cell (GHSL Data Package 2023)");
        eprintln!("  epoch: the dataset's own year, 2020-01-01 TDB");
        eprintln!("  --inspect prints the raster's shape and geotransform and stops");
        eprintln!(
            "  --tiles harvests the per-10-degree BigTIFF tile archives instead of the global raster"
        );
        eprintln!(
            "    (--tile R2_C2 repeats, --tile-list <path> or the tiles directory by default)"
        );
        eprintln!("  --limit bounds the emitted cells; --stride samples every Nth cell");
        eprintln!("  --ci-mode uploads the verified asset to the <netloc> CDN release");
        std::process::exit(1);
    }
    if let Err(msg) = run(&args) {
        eprintln!("ghsl_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eocd_bytes(entries: u16, cd_size: u32, cd_off: u32) -> Vec<u8> {
        let mut t = Vec::new();
        t.extend_from_slice(&SIG_EOCD);
        t.extend_from_slice(&0u16.to_le_bytes());
        t.extend_from_slice(&0u16.to_le_bytes());
        t.extend_from_slice(&entries.to_le_bytes());
        t.extend_from_slice(&entries.to_le_bytes());
        t.extend_from_slice(&cd_size.to_le_bytes());
        t.extend_from_slice(&cd_off.to_le_bytes());
        t.extend_from_slice(&0u16.to_le_bytes());
        t
    }

    #[test]
    fn eocd_reads_the_trailer() {
        let t = eocd_bytes(3, 336, 12345);
        let (entries, cd_off, cd_size) = eocd(&t).unwrap();
        assert_eq!(entries, 3);
        assert_eq!(cd_off, 12345);
        assert_eq!(cd_size, 336);
    }

    #[test]
    fn zip_member_reads_name_and_method() {
        let mut cd = Vec::new();
        cd.extend_from_slice(&SIG_CENTRAL);
        cd.extend_from_slice(&[0u8; 6]);
        cd.extend_from_slice(&8u16.to_le_bytes());
        cd.extend_from_slice(&[0u8; 4]);
        cd.extend_from_slice(&[0u8; 4]);
        cd.extend_from_slice(&100u32.to_le_bytes());
        cd.extend_from_slice(&200u32.to_le_bytes());
        cd.extend_from_slice(&5u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&[0u8; 8]);
        cd.extend_from_slice(&7u32.to_le_bytes());
        cd.extend_from_slice(b"x.tif");
        let m = zip_members(&cd, 1).unwrap();
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].name, "x.tif");
        assert_eq!(m[0].method, 8);
        assert_eq!(m[0].compressed, 100);
        assert_eq!(m[0].uncompressed, 200);
        assert_eq!(m[0].local_offset, 7);
    }

    #[test]
    fn eocd_refuses_a_tail_without_signature() {
        assert!(eocd(b"no archive here").is_err());
    }

    fn entry(out: &mut Vec<u8>, tag: u16, ftype: u16, count: u32, value: u32) {
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&ftype.to_le_bytes());
        out.extend_from_slice(&count.to_le_bytes());
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn synth_tif(width: u16, height: u16, samples: &[u16]) -> Vec<u8> {
        let ifd_count = 11u16;
        let ifd_off = 8usize;
        let ifd_len = 2 + ifd_count as usize * 12 + 4;
        let scale_off = ifd_off + ifd_len;
        let tie_off = scale_off + 24;
        let pix_off = tie_off + 48;
        let row_bytes = width as u32 * 2;
        let strip_bytes = row_bytes * height as u32;

        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        out.extend_from_slice(&42u16.to_le_bytes());
        out.extend_from_slice(&(ifd_off as u32).to_le_bytes());
        out.extend_from_slice(&ifd_count.to_le_bytes());
        entry(&mut out, 256, 3, 1, width as u32);
        entry(&mut out, 257, 3, 1, height as u32);
        entry(&mut out, 258, 3, 1, 16);
        entry(&mut out, 259, 3, 1, 1);
        entry(&mut out, 262, 3, 1, 1);
        entry(&mut out, 273, 4, 1, pix_off as u32);
        entry(&mut out, 277, 3, 1, 1);
        entry(&mut out, 278, 4, 1, height as u32);
        entry(&mut out, 279, 4, 1, strip_bytes);
        entry(&mut out, 33550, 12, 3, scale_off as u32);
        entry(&mut out, 33922, 12, 6, tie_off as u32);
        out.extend_from_slice(&0u32.to_le_bytes());
        for v in [1.0f64, 1.0, 0.0] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in [0.0f64, 0.0, 0.0, 10.0, 20.0, 0.0] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for &s in samples {
            out.extend_from_slice(&s.to_le_bytes());
        }
        assert_eq!(out.len(), pix_off + samples.len() * 2);
        out
    }

    #[test]
    fn build_records_reads_cell_centres_and_nodata() {
        let tif = synth_tif(2, 2, &[10, 65535, 20, 0]);
        let records = build_records(&tif, "synth", 1577836800.0, 1, usize::MAX).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].lat, 19.5);
        assert_eq!(records[0].lon, 10.5);
        assert_eq!(records[0].val, 10.0);
        assert_eq!(records[1].val, 20.0);
        assert_eq!(records[2].val, 0.0);
        assert_eq!(records[0].comp, COMP_GHSL_BUILT);
        let strided = build_records(&tif, "synth", 1577836800.0, 2, usize::MAX).unwrap();
        assert_eq!(strided.len(), 1);
        let limited = build_records(&tif, "synth", 1577836800.0, 1, 1).unwrap();
        assert_eq!(limited.len(), 1);
    }

    #[test]
    fn sample_at_reads_both_ends() {
        assert_eq!(sample_at(&[0x01, 0x00], 0, 16, true), Some(1.0));
        assert_eq!(sample_at(&[0x01, 0x00], 0, 16, false), Some(256.0));
        assert_eq!(sample_at(&[3], 0, 8, true), Some(3.0));
        assert_eq!(sample_at(&[0], 0, 16, true), None);
    }

    #[test]
    fn tile_url_from_rc_reads_the_cell() {
        assert_eq!(
            tile_url_from_rc("R2_C2").unwrap(),
            format!("{TILES_DIR}{TILE_PREFIX}R2_C2.zip")
        );
        assert_eq!(
            tile_url_from_rc("15 34").unwrap(),
            format!("{TILES_DIR}{TILE_PREFIX}R15_C34.zip")
        );
        assert!(tile_url_from_rc("junk").is_none());
    }

    #[test]
    fn epoch_is_the_dataset_year() {
        let t = epoch_tdb().unwrap();
        assert!(t > 1577836800.0);
        assert!(t < 1577836800.0 + 86400.0);
    }
}
