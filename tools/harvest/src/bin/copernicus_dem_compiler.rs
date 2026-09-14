use omegaflow::cdn::upload_release;
use omegaflow::zeuge::{FeldIdentitaet, ZeugeArt, magic_identity};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::process::Command;

const NETLOC: &str = "copernicus-dem-30m.s3.amazonaws.com";
const BASE: &str = "https://copernicus-dem-30m.s3.amazonaws.com";
const MAGIC: [u8; 4] = *b"GL30";
const HEADER_BYTES: usize = 48;

const TAG_IMAGE_WIDTH: u16 = 256;
const TAG_IMAGE_LENGTH: u16 = 257;
const TAG_BITS_PER_SAMPLE: u16 = 258;
const TAG_COMPRESSION: u16 = 259;
const TAG_STRIP_OFFSETS: u16 = 273;
const TAG_SAMPLES_PER_PIXEL: u16 = 277;
const TAG_ROWSPERSTRIP: u16 = 278;
const TAG_STRIP_BYTE_COUNTS: u16 = 279;
const TAG_PREDICTOR: u16 = 317;
const TAG_TILE_WIDTH: u16 = 322;
const TAG_TILE_LENGTH: u16 = 323;
const TAG_TILE_OFFSETS: u16 = 324;
const TAG_TILE_BYTE_COUNTS: u16 = 325;
const TAG_SAMPLE_FORMAT: u16 = 339;
const TAG_MODEL_PIXEL_SCALE: u16 = 33550;
const TAG_MODEL_TIEPOINT: u16 = 33922;
const TAG_GEO_KEY_DIRECTORY: u16 = 34735;
const GEO_KEY_GEOGRAPHIC_TYPE: u16 = 2048;

struct Tag {
    typ: u16,
    count: u32,
    value: [u8; 4],
}

struct Dem {
    width: usize,
    height: usize,
    epsg: Option<u16>,
    geo: Option<(f64, f64, f64, f64)>,
    samples: Vec<f32>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn tile_id(lat: i32, lon: i32) -> String {
    let ns = if lat >= 0 {
        format!("N{lat:02}")
    } else {
        format!("S{:02}", -lat)
    };
    let ew = if lon >= 0 {
        format!("E{lon:03}")
    } else {
        format!("W{:03}", -lon)
    };
    format!("{ns}_00_{ew}_00")
}

fn tile_url(tile: &str) -> String {
    format!("{BASE}/Copernicus_DSM_COG_10_{tile}_DEM/Copernicus_DSM_COG_10_{tile}_DEM.tif")
}

fn witness_gestalt_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FeldIdentitaet::Zeuge(ZeugeArt::Gestalt)) => {
            eprintln!(
                "{} reads as a gestalt witness record",
                String::from_utf8_lossy(&magic)
            );
            Ok(())
        }
        Some(other) => Err(format!(
            "{} reads {:?}, not gestalt — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn download(url: &str, path: &str) -> Result<(), String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("600")
        .arg("-o")
        .arg(path)
        .arg(url)
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{url}: {} — the tile stays unfetched",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let sz = std::fs::metadata(path)
        .map_err(|e| format!("metadata {path} returned void: {e}"))?
        .len();
    if sz == 0 {
        return Err(format!("{url}: the tile carries no bytes"));
    }
    eprintln!("{url}: {sz} B -> {path}");
    Ok(())
}

fn type_size(typ: u16) -> Option<usize> {
    Some(match typ {
        1 | 2 | 6 | 7 => 1,
        3 | 8 => 2,
        4 | 9 | 11 => 4,
        5 | 10 | 12 => 8,
        _ => return None,
    })
}

fn read_u16(data: &[u8], off: usize) -> Result<u16, String> {
    let b = data
        .get(off..off + 2)
        .ok_or("the container ends before its marker")?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}

fn parse_ifd(data: &[u8]) -> Result<HashMap<u16, Tag>, String> {
    if data.len() < 8 || &data[0..2] != b"II" {
        return Err("the container does not read little-endian TIFF".into());
    }
    let marker = read_u16(data, 2)?;
    if marker != 42 {
        return Err(format!("the TIFF marker reads {marker}, not 42"));
    }
    let ifd_off = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
    let n = read_u16(data, ifd_off)? as usize;
    let mut tags = HashMap::new();
    let mut cur = ifd_off + 2;
    for _ in 0..n {
        let e = data
            .get(cur..cur + 12)
            .ok_or("the IFD ends before its entry count")?;
        let tag = u16::from_le_bytes([e[0], e[1]]);
        let typ = u16::from_le_bytes([e[2], e[3]]);
        let count = u32::from_le_bytes([e[4], e[5], e[6], e[7]]);
        let value = [e[8], e[9], e[10], e[11]];
        tags.insert(tag, Tag { typ, count, value });
        cur += 12;
    }
    Ok(tags)
}

fn tag_uint(tag: &Tag) -> Option<u64> {
    if tag.count < 1 {
        return None;
    }
    match tag.typ {
        1 => Some(tag.value[0] as u64),
        3 => Some(u16::from_le_bytes([tag.value[0], tag.value[1]]) as u64),
        4 => Some(u32::from_le_bytes(tag.value) as u64),
        _ => None,
    }
}

fn tag_u32_vec(data: &[u8], tag: &Tag) -> Result<Vec<u32>, String> {
    let sz = type_size(tag.typ).ok_or_else(|| format!("tag type {} carries no width", tag.typ))?;
    let total = sz * tag.count as usize;
    let mut out = Vec::with_capacity(tag.count as usize);
    for k in 0..tag.count as usize {
        let inline = total <= 4;
        let off = if inline {
            k * sz
        } else {
            u32::from_le_bytes(tag.value) as usize + k * sz
        };
        let src: &[u8] = if inline { &tag.value } else { data };
        let s = src
            .get(off..off + sz)
            .ok_or_else(|| format!("tag {} reads past the container", tag.typ))?;
        out.push(match sz {
            1 => s[0] as u32,
            2 => u16::from_le_bytes([s[0], s[1]]) as u32,
            4 => u32::from_le_bytes([s[0], s[1], s[2], s[3]]),
            other => return Err(format!("tag element width {other} is not a u32")),
        });
    }
    Ok(out)
}

fn tag_u16_vec(data: &[u8], tag: &Tag) -> Result<Vec<u16>, String> {
    let sz = type_size(tag.typ).ok_or_else(|| format!("tag type {} carries no width", tag.typ))?;
    let total = sz * tag.count as usize;
    let mut out = Vec::with_capacity(tag.count as usize);
    for k in 0..tag.count as usize {
        let inline = total <= 4;
        let off = if inline {
            k * sz
        } else {
            u32::from_le_bytes(tag.value) as usize + k * sz
        };
        let src: &[u8] = if inline { &tag.value } else { data };
        let s = src
            .get(off..off + sz)
            .ok_or_else(|| format!("tag {} reads past the container", tag.typ))?;
        out.push(match sz {
            1 => s[0] as u16,
            2 => u16::from_le_bytes([s[0], s[1]]),
            other => return Err(format!("tag element width {other} is not a u16")),
        });
    }
    Ok(out)
}

fn tag_f64_vec(data: &[u8], tag: &Tag) -> Result<Vec<f64>, String> {
    let sz = type_size(tag.typ).ok_or_else(|| format!("tag type {} carries no width", tag.typ))?;
    let total = sz * tag.count as usize;
    let base = if total <= 4 {
        0
    } else {
        u32::from_le_bytes(tag.value) as usize
    };
    let mut out = Vec::with_capacity(tag.count as usize);
    for k in 0..tag.count as usize {
        let off = base + k * sz;
        let s = data
            .get(off..off + sz)
            .ok_or("the georeference reads past the container")?;
        out.push(match sz {
            8 => f64::from_le_bytes(s.try_into().map_err(|_| "double unread")?),
            4 => f32::from_le_bytes(s.try_into().map_err(|_| "float unread")?) as f64,
            other => return Err(format!("georeference width {other} is not a double")),
        });
    }
    Ok(out)
}

fn undo_predictor2(row: &[u8], bps: usize) -> Vec<u8> {
    let mut out = row.to_vec();
    let n = out.len() / bps;
    for i in 1..n {
        let mut carry = 0u16;
        for b in 0..bps {
            let s = out[i * bps + b] as u16 + out[(i - 1) * bps + b] as u16 + carry;
            out[i * bps + b] = (s & 0xff) as u8;
            carry = s >> 8;
        }
    }
    out
}

fn undo_predictor3(row: &[u8], bps: usize) -> Vec<u8> {
    let mut tmp = row.to_vec();
    for i in 1..tmp.len() {
        tmp[i] = tmp[i].wrapping_add(tmp[i - 1]);
    }
    let wc = tmp.len() / bps;
    let mut out = vec![0u8; tmp.len()];
    for count in 0..wc {
        for byte in 0..bps {
            out[bps * count + byte] = tmp[(bps - byte - 1) * wc + count];
        }
    }
    out
}

fn apply_predictor(dec: &[u8], predictor: u16, row_bytes: usize) -> Vec<u8> {
    if predictor == 1 || row_bytes == 0 {
        return dec.to_vec();
    }
    let mut out = Vec::with_capacity(dec.len());
    for row in dec.chunks(row_bytes) {
        let fixed = match predictor {
            2 => undo_predictor2(row, 4),
            3 => undo_predictor3(row, 4),
            _ => row.to_vec(),
        };
        out.extend_from_slice(&fixed);
    }
    out
}

fn decompress(raw: &[u8], compression: u16) -> Result<Vec<u8>, String> {
    match compression {
        1 => Ok(raw.to_vec()),
        8 | 32946 => {
            let body = if raw.len() >= 2 && raw[0] == 0x78 {
                &raw[2..]
            } else {
                raw
            };
            omegaflow::inflate::inflate(body)
                .ok_or_else(|| "the deflate stream stays unread".into())
        }
        other => Err(format!("compression {other} carries no decoder")),
    }
}

fn geokey(data: &[u8], tags: &HashMap<u16, Tag>, key_id: u16) -> Option<u16> {
    let tag = tags.get(&TAG_GEO_KEY_DIRECTORY)?;
    let vals = tag_u16_vec(data, tag).ok()?;
    let n = *vals.get(3)? as usize;
    for k in 0..n {
        let base = 4 + k * 4;
        if *vals.get(base)? == key_id && *vals.get(base + 1)? == 0 && *vals.get(base + 2)? == 1 {
            return vals.get(base + 3).copied();
        }
    }
    None
}

fn geotransform(
    data: &[u8],
    tags: &HashMap<u16, Tag>,
) -> Result<Option<(f64, f64, f64, f64)>, String> {
    let Some(scale_tag) = tags.get(&TAG_MODEL_PIXEL_SCALE) else {
        return Ok(None);
    };
    let Some(tie_tag) = tags.get(&TAG_MODEL_TIEPOINT) else {
        return Ok(None);
    };
    let scale = tag_f64_vec(data, scale_tag)?;
    let tie = tag_f64_vec(data, tie_tag)?;
    if scale.len() < 2 || tie.len() < 6 {
        return Ok(None);
    }
    let (i, j) = (tie[0], tie[1]);
    let (x, y) = (tie[3], tie[4]);
    let (sx, sy) = (scale[0], scale[1]);
    if !sx.is_finite() || !sy.is_finite() || !x.is_finite() || !y.is_finite() {
        return Ok(None);
    }
    Ok(Some((x - i * sx, y + j * sy, sx, sy)))
}

fn decode_tiff(data: &[u8]) -> Result<Dem, String> {
    let tags = parse_ifd(data)?;
    let width = tags
        .get(&TAG_IMAGE_WIDTH)
        .and_then(tag_uint)
        .ok_or("ImageWidth is absent")? as usize;
    let height = tags
        .get(&TAG_IMAGE_LENGTH)
        .and_then(tag_uint)
        .ok_or("ImageLength is absent")? as usize;
    let bits = tags
        .get(&TAG_BITS_PER_SAMPLE)
        .and_then(tag_uint)
        .ok_or("BitsPerSample is absent")?;
    if bits != 32 {
        return Err(format!(
            "BitsPerSample reads {bits}, not 32 — the samples stay unread"
        ));
    }
    let spp = tags
        .get(&TAG_SAMPLES_PER_PIXEL)
        .and_then(tag_uint)
        .unwrap_or(1);
    if spp != 1 {
        return Err(format!(
            "SamplesPerPixel reads {spp}, not 1 — the band stays unread"
        ));
    }
    let sample_format = tags.get(&TAG_SAMPLE_FORMAT).and_then(tag_uint).unwrap_or(1);
    if sample_format != 3 {
        return Err(format!(
            "SampleFormat reads {sample_format}, not IEEE float (3) — the samples stay unread"
        ));
    }
    let compression = tags.get(&TAG_COMPRESSION).and_then(tag_uint).unwrap_or(1) as u16;
    let predictor = tags.get(&TAG_PREDICTOR).and_then(tag_uint).unwrap_or(1) as u16;
    if predictor != 1 && predictor != 2 && predictor != 3 {
        return Err(format!("Predictor reads {predictor}, carrying no decoder"));
    }
    if width == 0 || height == 0 {
        return Err("the raster carries no extent".into());
    }
    let mut samples = vec![f32::NAN; width * height];
    if tags.contains_key(&TAG_TILE_OFFSETS) {
        let tw = tags
            .get(&TAG_TILE_WIDTH)
            .and_then(tag_uint)
            .ok_or("TileWidth is absent")? as usize;
        let th = tags
            .get(&TAG_TILE_LENGTH)
            .and_then(tag_uint)
            .ok_or("TileLength is absent")? as usize;
        if tw == 0 || th == 0 {
            return Err("the tile carries no extent".into());
        }
        let offs = tag_u32_vec(
            data,
            tags.get(&TAG_TILE_OFFSETS).ok_or("TileOffsets is absent")?,
        )?;
        let cnts = tag_u32_vec(
            data,
            tags.get(&TAG_TILE_BYTE_COUNTS)
                .ok_or("TileByteCounts is absent")?,
        )?;
        let across = (width + tw - 1) / tw;
        for ty in 0..(height + th - 1) / th {
            for tx in 0..across {
                let idx = ty * across + tx;
                let off = *offs.get(idx).ok_or("TileOffsets ends before the grid")? as usize;
                let len = *cnts.get(idx).ok_or("TileByteCounts ends before the grid")? as usize;
                let raw = data
                    .get(off..off + len)
                    .ok_or("a tile reads past the container")?;
                let dec = decompress(raw, compression)?;
                let dec = apply_predictor(&dec, predictor, tw * 4);
                let tile_w = tw.min(width - tx * tw);
                let tile_h = th.min(height - ty * th);
                for r in 0..tile_h {
                    for c in 0..tile_w {
                        let p = (r * tw + c) * 4;
                        let s = dec
                            .get(p..p + 4)
                            .ok_or("a tile sample reads past its bytes")?;
                        samples[(ty * th + r) * width + tx * tw + c] =
                            f32::from_le_bytes([s[0], s[1], s[2], s[3]]);
                    }
                }
            }
        }
    } else if tags.contains_key(&TAG_STRIP_OFFSETS) {
        let rows_per_strip = tags
            .get(&TAG_ROWSPERSTRIP)
            .and_then(tag_uint)
            .unwrap_or(height as u64) as usize;
        if rows_per_strip == 0 {
            return Err("RowsPerStrip reads 0".into());
        }
        let offs = tag_u32_vec(
            data,
            tags.get(&TAG_STRIP_OFFSETS)
                .ok_or("StripOffsets is absent")?,
        )?;
        let cnts = tag_u32_vec(
            data,
            tags.get(&TAG_STRIP_BYTE_COUNTS)
                .ok_or("StripByteCounts is absent")?,
        )?;
        for (s, (&off, &len)) in offs.iter().zip(cnts.iter()).enumerate() {
            let y0 = s * rows_per_strip;
            if y0 >= height {
                break;
            }
            let rows = rows_per_strip.min(height - y0);
            let raw = data
                .get(off as usize..off as usize + len as usize)
                .ok_or("a strip reads past the container")?;
            let dec = decompress(raw, compression)?;
            let dec = apply_predictor(&dec, predictor, width * 4);
            for r in 0..rows {
                for c in 0..width {
                    let p = (r * width + c) * 4;
                    let s = dec
                        .get(p..p + 4)
                        .ok_or("a strip sample reads past its bytes")?;
                    samples[(y0 + r) * width + c] = f32::from_le_bytes([s[0], s[1], s[2], s[3]]);
                }
            }
        }
    } else {
        return Err("the IFD carries neither tiles nor strips".into());
    }
    if samples.iter().any(|s| s.is_nan()) {
        return Err("the grid carries pixels the tiles do not reach".into());
    }
    let geo = geotransform(data, &tags)?;
    let epsg = geokey(data, &tags, GEO_KEY_GEOGRAPHIC_TYPE);
    Ok(Dem {
        width,
        height,
        epsg,
        geo,
        samples,
    })
}

fn write_bin(path: &str, dem: &Dem) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    let mut f =
        std::fs::File::create(path).map_err(|e| format!("create {path} returned void: {e}"))?;
    f.write_all(&MAGIC)
        .map_err(|e| format!("write {path} returned void: {e}"))?;
    let epsg: u16 = match dem.epsg {
        Some(e) => e,
        None => 0,
    };
    f.write_all(&(dem.width as u32).to_le_bytes())
        .and_then(|_| f.write_all(&(dem.height as u32).to_le_bytes()))
        .and_then(|_| f.write_all(&epsg.to_le_bytes()))
        .map_err(|e| format!("write {path} returned void: {e}"))?;
    let flags = if dem.geo.is_some() { 1u8 } else { 0u8 };
    f.write_all(&[flags, 0u8])
        .map_err(|e| format!("write {path} returned void: {e}"))?;
    let (west, north, plon, plat) = dem.geo.unwrap_or((0.0, 0.0, 0.0, 0.0));
    for v in [west, north, plon, plat] {
        f.write_all(&v.to_le_bytes())
            .map_err(|e| format!("write {path} returned void: {e}"))?;
    }
    for s in &dem.samples {
        f.write_all(&s.to_le_bytes())
            .map_err(|e| format!("write {path} returned void: {e}"))?;
    }
    let _ = f.flush();
    Ok(())
}

fn verify_bin(path: &str, dem: &Dem) -> Result<(), String> {
    let mut vf =
        std::fs::File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
    let mut head = [0u8; HEADER_BYTES];
    vf.read_exact(&mut head)
        .map_err(|e| format!("read {path} header returned void: {e}"))?;
    if head[0..4] != MAGIC {
        return Err(format!("{path}: the magic stays unread"));
    }
    let w = u32::from_le_bytes([head[4], head[5], head[6], head[7]]) as usize;
    let h = u32::from_le_bytes([head[8], head[9], head[10], head[11]]) as usize;
    if w != dem.width || h != dem.height {
        return Err(format!(
            "{path}: reads {w}x{h}, wrote {}x{} — the asset stays unwritten",
            dem.width, dem.height
        ));
    }
    let expected = HEADER_BYTES as u64 + (w as u64) * (h as u64) * 4;
    let actual = std::fs::metadata(path)
        .map_err(|e| format!("metadata {path} returned void: {e}"))?
        .len();
    if actual != expected {
        return Err(format!(
            "{path}: {actual} bytes read, {expected} written — the asset stays unwritten"
        ));
    }
    eprintln!("{path}: {}x{} height grid, roundtrip verified", w, h);
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    witness_gestalt_identity(MAGIC)?;
    let Some(lat) = arg_value(args, "--lat").and_then(|s| s.parse::<i32>().ok()) else {
        return Err(
            "usage: copernicus_dem_compiler --lat <deg> --lon <deg> [--out <grid.bin>] [--ci-mode] — refused"
                .into(),
        );
    };
    let Some(lon) = arg_value(args, "--lon").and_then(|s| s.parse::<i32>().ok()) else {
        return Err("--lon <deg>: the tile coordinate is never silent — refused".into());
    };
    if !(-90..90).contains(&lat) || !(-180..180).contains(&lon) {
        return Err(format!(
            "{lat},{lon} lies outside the 1° tile register — refused"
        ));
    }
    let tile = tile_id(lat, lon);
    let url = tile_url(&tile);
    let out_path = match arg_value(args, "--out") {
        Some(p) => p,
        None => format!("data/{NETLOC}/copernicus_dem_30m_{tile}.bin"),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let tif_path = format!("data/{NETLOC}/{tile}.tif");
    if let Some(parent) = std::path::Path::new(&tif_path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
    }
    download(&url, &tif_path)?;
    let bytes =
        std::fs::read(&tif_path).map_err(|e| format!("read {tif_path} returned void: {e}"))?;
    let dem = decode_tiff(&bytes)?;
    if let Some(epsg) = dem.epsg {
        eprintln!("{tile}: EPSG:{epsg}, {:?} geotransform", dem.geo);
    }
    write_bin(&out_path, &dem)?;
    verify_bin(&out_path, &dem)?;

    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("copernicus_dem_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tiff(mut tags: Vec<(u16, u16, Vec<u8>)>, payload: &[u8]) -> Vec<u8> {
        let total = tags.len() + 2;
        let ifd_end = 8 + 2 + 12 * total + 4;
        let payload_off = ifd_end;
        tags.push((
            TAG_STRIP_OFFSETS,
            4,
            (payload_off as u32).to_le_bytes().to_vec(),
        ));
        tags.push((
            TAG_STRIP_BYTE_COUNTS,
            4,
            (payload.len() as u32).to_le_bytes().to_vec(),
        ));
        tags.sort_by_key(|(t, _, _)| *t);
        let extra_base = payload_off + payload.len();
        let mut extra: Vec<u8> = Vec::new();
        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        out.extend_from_slice(&42u16.to_le_bytes());
        out.extend_from_slice(&8u32.to_le_bytes());
        out.extend_from_slice(&(total as u16).to_le_bytes());
        for (tag, typ, val) in &tags {
            let sz = type_size(*typ).unwrap();
            let count = val.len() / sz;
            let field = if val.len() <= 4 {
                let mut b = [0u8; 4];
                b[..val.len()].copy_from_slice(val);
                u32::from_le_bytes(b)
            } else {
                let off = (extra_base + extra.len()) as u32;
                extra.extend_from_slice(val);
                off
            };
            out.extend_from_slice(&tag.to_le_bytes());
            out.extend_from_slice(&typ.to_le_bytes());
            out.extend_from_slice(&(count as u32).to_le_bytes());
            out.extend_from_slice(&field.to_le_bytes());
        }
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(payload);
        out.extend_from_slice(&extra);
        out
    }

    fn base_tags(
        width: u16,
        height: u16,
        predictor: u16,
        compression: u16,
    ) -> Vec<(u16, u16, Vec<u8>)> {
        vec![
            (TAG_IMAGE_WIDTH, 3, width.to_le_bytes().to_vec()),
            (TAG_IMAGE_LENGTH, 3, height.to_le_bytes().to_vec()),
            (TAG_BITS_PER_SAMPLE, 3, 32u16.to_le_bytes().to_vec()),
            (TAG_COMPRESSION, 3, compression.to_le_bytes().to_vec()),
            (TAG_SAMPLES_PER_PIXEL, 3, 1u16.to_le_bytes().to_vec()),
            (TAG_ROWSPERSTRIP, 3, height.to_le_bytes().to_vec()),
            (TAG_PREDICTOR, 3, predictor.to_le_bytes().to_vec()),
            (TAG_SAMPLE_FORMAT, 3, 3u16.to_le_bytes().to_vec()),
        ]
    }

    fn encode_predictor3(samples: &[f32], width: usize) -> Vec<u8> {
        let bps = 4usize;
        let mut out = Vec::new();
        for row in samples.chunks(width) {
            let wc = row.len();
            let cc = wc * bps;
            let mut tmp = Vec::with_capacity(cc);
            for s in row {
                tmp.extend_from_slice(&s.to_le_bytes());
            }
            let mut cp = vec![0u8; cc];
            for count in 0..wc {
                for byte in 0..bps {
                    cp[(bps - byte - 1) * wc + count] = tmp[bps * count + byte];
                }
            }
            for i in (1..cc).rev() {
                cp[i] = cp[i].wrapping_sub(cp[i - 1]);
            }
            out.extend_from_slice(&cp);
        }
        out
    }

    fn adler32(data: &[u8]) -> u32 {
        let (mut a, mut b) = (1u32, 0u32);
        for &x in data {
            a = (a + x as u32) % 65521;
            b = (b + a) % 65521;
        }
        (b << 16) | a
    }

    fn zlib_stored(data: &[u8]) -> Vec<u8> {
        let mut out = vec![0x78, 0x01, 0x01];
        let len = data.len() as u16;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(data);
        out.extend_from_slice(&adler32(data).to_be_bytes());
        out
    }

    fn raw_samples(samples: &[f32]) -> Vec<u8> {
        let mut out = Vec::new();
        for s in samples {
            out.extend_from_slice(&s.to_le_bytes());
        }
        out
    }

    #[test]
    fn strips_without_predictor_decode_float32() {
        let samples = [1.5f32, -2.25, 0.0, 12345.678];
        let tiff = build_tiff(base_tags(2, 2, 1, 1), &raw_samples(&samples));
        let dem = decode_tiff(&tiff).unwrap();
        assert_eq!(dem.width, 2);
        assert_eq!(dem.height, 2);
        assert_eq!(dem.samples, samples);
        assert_eq!(dem.geo, None);
        assert_eq!(dem.epsg, None);
    }

    #[test]
    fn strips_with_float_predictor_decode_float32() {
        let samples = [1.5f32, -2.25, 0.0, 12345.678];
        let payload = encode_predictor3(&samples, 2);
        let tiff = build_tiff(base_tags(2, 2, 3, 1), &payload);
        let dem = decode_tiff(&tiff).unwrap();
        assert_eq!(dem.samples, samples);
    }

    #[test]
    fn strips_with_deflate_zlib_decode_float32() {
        let samples = [3.25f32, 4.5, -6.75, 8.0];
        let payload = zlib_stored(&encode_predictor3(&samples, 2));
        let tiff = build_tiff(base_tags(2, 2, 3, 8), &payload);
        let dem = decode_tiff(&tiff).unwrap();
        assert_eq!(dem.samples, samples);
    }

    #[test]
    fn geotransform_reads_pixel_scale_and_tiepoint() {
        let samples = [1.0f32; 4];
        let mut tags = base_tags(2, 2, 1, 1);
        tags.push((
            TAG_MODEL_PIXEL_SCALE,
            12,
            [0.0002777777777777778f64, 0.0002777777777777778, 0.0]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect(),
        ));
        tags.push((
            TAG_MODEL_TIEPOINT,
            12,
            [0.0f64, 0.0, 0.0, 10.0, 50.0, 0.0]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect(),
        ));
        let tiff = build_tiff(tags, &raw_samples(&samples));
        let dem = decode_tiff(&tiff).unwrap();
        let (west, north, plon, plat) = dem.geo.unwrap();
        assert_eq!(west, 10.0);
        assert_eq!(north, 50.0);
        assert!(plon > 0.0 && plat > 0.0);
    }

    #[test]
    fn refuses_non_float_sample_format() {
        let mut tags = base_tags(2, 2, 1, 1);
        for t in tags.iter_mut() {
            if t.0 == TAG_SAMPLE_FORMAT {
                t.2 = 1u16.to_le_bytes().to_vec();
            }
        }
        let tiff = build_tiff(tags, &raw_samples(&[1.0, 2.0, 3.0, 4.0]));
        assert!(decode_tiff(&tiff).is_err());
    }

    #[test]
    fn tile_id_carries_hemisphere_signs() {
        assert_eq!(tile_id(50, 10), "N50_00_E010_00");
        assert_eq!(tile_id(-34, 138), "S34_00_E138_00");
        assert_eq!(tile_id(0, -120), "N00_00_W120_00");
    }

    #[test]
    fn tile_url_reads_the_witness_template() {
        assert_eq!(
            tile_url("N50_00_E010_00"),
            "https://copernicus-dem-30m.s3.amazonaws.com/Copernicus_DSM_COG_10_N50_00_E010_00_DEM/Copernicus_DSM_COG_10_N50_00_E010_00_DEM.tif"
        );
    }
}
