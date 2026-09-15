use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::tiff::{parse_tiff, GeoTransform, TiffImage};
use omegaflow::cdn::upload_release;
use std::io::{BufWriter, Write};

const NETLOC: &str = "noaa-eri-pds.s3.amazonaws.com";
const MAGIC: [u8; 4] = *b"ERI1";
const REC_BYTES: usize = 24;

const GEOTIFF_MODEL_TYPE: u16 = 1024;
const GEOTIFF_PROJECTED_CS: u16 = 3072;
const WEB_MERCATOR: u16 = 3857;

struct EriSample {
    lat: f64,
    lon: f64,
    value: f64,
}

#[derive(Clone, Copy, Debug)]
enum Georef {
    Geographic,
    WebMercator,
    Unsupported(u16),
}

fn geokey_value(keys: &[u16], wanted: u16) -> Option<u16> {
    if keys.len() < 4 {
        return None;
    }
    let n = keys[3] as usize;
    for k in 0..n {
        let base = 4 + k * 4;
        let id = *keys.get(base)?;
        let tag_loc = *keys.get(base + 1)?;
        let count = *keys.get(base + 2)?;
        let value = *keys.get(base + 3)?;
        if id == wanted && tag_loc == 0 && count == 1 {
            return Some(value);
        }
    }
    None
}

fn georef_of(keys: Option<&[u16]>) -> Georef {
    let Some(keys) = keys else {
        return Georef::Geographic;
    };
    match geokey_value(keys, GEOTIFF_MODEL_TYPE) {
        Some(1) => match geokey_value(keys, GEOTIFF_PROJECTED_CS) {
            Some(WEB_MERCATOR) => Georef::WebMercator,
            Some(code) => Georef::Unsupported(code),
            None => Georef::Unsupported(0),
        },
        _ => Georef::Geographic,
    }
}

fn web_mercator_to_latlon(x: f64, y: f64) -> Option<(f64, f64)> {
    const EARTH_RADIUS_M: f64 = 6_378_137.0;
    let max = std::f64::consts::PI * EARTH_RADIUS_M;
    if !x.is_finite() || !y.is_finite() || x.abs() > max + 1e-3 || y.abs() > max + 1e-3 {
        return None;
    }
    let deg = 180.0 / std::f64::consts::PI;
    let lon = x / EARTH_RADIUS_M * deg;
    let lat = (2.0 * (y / EARTH_RADIUS_M).exp().atan() - std::f64::consts::FRAC_PI_2) * deg;
    Some((lat, lon))
}

fn lonlat_of(geo: &GeoTransform, georef: &Georef, col: usize, row: usize) -> Option<(f64, f64)> {
    let x = geo.x0 + (col as f64 + 0.5) * geo.dx;
    let y = geo.y0 + (row as f64 + 0.5) * geo.dy;
    match georef {
        Georef::Geographic => {
            const EPS: f64 = 1e-9;
            if x.is_finite()
                && y.is_finite()
                && x >= -180.0 - EPS
                && x <= 180.0 + EPS
                && y >= -90.0 - EPS
                && y <= 90.0 + EPS
            {
                Some((y, x))
            } else {
                None
            }
        }
        Georef::WebMercator => web_mercator_to_latlon(x, y),
        Georef::Unsupported(_) => None,
    }
}

fn collect(img: &TiffImage, georef: &Georef) -> Result<Vec<EriSample>, String> {
    if img.pixels.is_empty() {
        return Err(
            "no decoded pixels — the strip/tile decode returned void, the bin stays unwritten (0 honored)"
                .to_string(),
        );
    }
    let first_band_bits = img.bits_per_sample.first().copied().unwrap_or(8);
    if first_band_bits != 8 {
        return Err(format!(
            "the first band is {first_band_bits} bits per sample — the ERI value path reads 8-bit bands only"
        ));
    }
    let geo = match &img.geo {
        Some(g) => g,
        None => {
            return Err(
                "the GeoTIFF carries no georeferencing (no ModelPixelScale/Tiepoint/Transformation) — lat/lon absent, the bin stays unwritten"
                    .to_string(),
            );
        }
    };
    if let Georef::Unsupported(code) = georef {
        return Err(format!(
            "projected CRS {code} is not Web Mercator (3857) — pixel coordinates are meters, not lat/lon; the bin stays unwritten"
        ));
    }
    let spp = img.samples_per_pixel as usize;
    if spp == 0 {
        return Err("samples per pixel is zero".to_string());
    }
    let width = img.width as usize;
    let height = img.height as usize;
    let pixel_count = width.checked_mul(height).and_then(|v| v.checked_mul(spp));
    let Some(pixel_count) = pixel_count else {
        return Err("width × height × samples-per-pixel overflowed".to_string());
    };
    if img.pixels.len() < pixel_count {
        return Err(
            "the pixel buffer is shorter than width × height × samples-per-pixel".to_string(),
        );
    }
    let mut samples = Vec::new();
    for row in 0..height {
        for col in 0..width {
            let Some((lat, lon)) = lonlat_of(geo, georef, col, row) else {
                continue;
            };
            let value = img.pixels[(row * width + col) * spp] as f64;
            samples.push(EriSample { lat, lon, value });
        }
    }
    if samples.is_empty() {
        return Err(
            "no pixel yielded a finite lat/lon — the field stays unwritten (0 honored)".to_string(),
        );
    }
    Ok(samples)
}

fn report_image(img: &TiffImage, georef: &Georef) {
    eprintln!(
        "image {}×{}, samples/pixel {}, bits {:?}, compression {}, photometric {:?}",
        img.width,
        img.height,
        img.samples_per_pixel,
        img.bits_per_sample,
        img.compression,
        img.photometric
    );
    if let Some(g) = &img.geo {
        eprintln!(
            "geotransform x0 {:.6} y0 {:.6} dx {:.6} dy {:.6}",
            g.x0, g.y0, g.dx, g.dy
        );
    } else {
        eprintln!("geotransform absent");
    }
    match georef {
        Georef::Geographic => {
            eprintln!("crs geographic — lon reads as x, lat reads as y from the geotransform")
        }
        Georef::WebMercator => {
            eprintln!("crs Web Mercator (EPSG:3857) — inverted to lat/lon")
        }
        Georef::Unsupported(code) => eprintln!("crs projected {code} — unsupported"),
    }
}

fn report_value_ranges(samples: &[EriSample]) {
    let min = samples
        .iter()
        .map(|s| s.value)
        .fold(f64::INFINITY, f64::min);
    let max = samples
        .iter()
        .map(|s| s.value)
        .fold(f64::NEG_INFINITY, f64::max);
    let mean = samples.iter().map(|s| s.value).sum::<f64>() / samples.len() as f64;
    eprintln!("first band: value min {min:.3} max {max:.3} mean {mean:.3}");
}

fn write_asset(samples: &[EriSample], out_path: &str) -> Result<usize, String> {
    let file = std::fs::File::create(out_path).map_err(|e| format!("create {out_path}: {e}"))?;
    let mut out = BufWriter::new(file);
    out.write_all(&MAGIC)
        .map_err(|e| format!("write {out_path}: {e}"))?;
    out.write_all(&(samples.len() as u32).to_le_bytes())
        .map_err(|e| format!("write {out_path}: {e}"))?;
    let mut rec = [0u8; REC_BYTES];
    for s in samples {
        rec[0..8].copy_from_slice(&s.lat.to_le_bytes());
        rec[8..16].copy_from_slice(&s.lon.to_le_bytes());
        rec[16..24].copy_from_slice(&s.value.to_le_bytes());
        out.write_all(&rec)
            .map_err(|e| format!("write {out_path}: {e}"))?;
    }
    out.flush().map_err(|e| format!("flush {out_path}: {e}"))?;
    let expect = 8 + samples.len() * REC_BYTES;
    let actual = std::fs::metadata(out_path)
        .map_err(|e| format!("stat {out_path}: {e}"))?
        .len() as usize;
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} bytes written, {expect} expected — the asset stays unwritten"
        ));
    }
    Ok(expect)
}

fn parse_asset(bytes: &[u8]) -> Option<Vec<EriSample>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    let f64_of = |b: &[u8], r: std::ops::Range<usize>| {
        b.get(r)
            .and_then(|x| x.try_into().ok())
            .map(f64::from_le_bytes)
    };
    for _ in 0..n {
        let s = bytes.get(off..off + REC_BYTES)?;
        let lat = f64_of(s, 0..8)?;
        let lon = f64_of(s, 8..16)?;
        let value = f64_of(s, 16..24)?;
        if !lat.is_finite() || !lon.is_finite() || !value.is_finite() {
            return None;
        }
        out.push(EriSample { lat, lon, value });
        off += REC_BYTES;
    }
    Some(out)
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_values(args: &[String], name: &str) -> Vec<String> {
    args.iter()
        .enumerate()
        .filter(|(_, a)| a.as_str() == name)
        .filter_map(|(i, _)| args.get(i + 1))
        .cloned()
        .collect()
}

fn percent_encode(s: &str) -> String {
    let mut out = String::new();
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn xml_blocks<'a>(s: &'a str, tag: &str) -> Vec<&'a str> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut out = Vec::new();
    let mut rest = s;
    while let Some(p) = rest.find(&open) {
        let body = &rest[p + open.len()..];
        match body.find(&close) {
            Some(e) => {
                out.push(&body[..e]);
                rest = &body[e + close.len()..];
            }
            None => break,
        }
    }
    out
}

fn xml_child(block: &str, tag: &str) -> String {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let Some(start) = block.find(&open) else {
        return String::new();
    };
    let body = &block[start + open.len()..];
    match body.find(&close) {
        Some(e) => body[..e].trim().to_string(),
        None => String::new(),
    }
}

fn s3_base(url: &str) -> Option<String> {
    let scheme_end = url.find("://")? + 3;
    let rest = &url[scheme_end..];
    let host_end = rest.find('/').unwrap_or(rest.len());
    Some(format!("{}{}", &url[..scheme_end], &rest[..host_end]))
}

fn s3_page_tiles(body: &str) -> Vec<String> {
    xml_blocks(body, "Contents")
        .into_iter()
        .map(|b| xml_child(b, "Key"))
        .filter(|k| k.ends_with(".tif"))
        .collect()
}

fn s3_continuation(body: &str) -> String {
    if !xml_child(body, "IsTruncated").eq_ignore_ascii_case("true") {
        return String::new();
    }
    let token = xml_child(body, "NextContinuationToken");
    if token.is_empty() {
        xml_child(body, "NextMarker")
    } else {
        token
    }
}

fn strip_continuation(url: &str) -> String {
    let Some(q) = url.find('?') else {
        return url.to_string();
    };
    let base = &url[..q];
    let mut params = String::new();
    for part in url[q + 1..].split('&') {
        if part.starts_with("continuation-token=") || part.starts_with("marker=") {
            continue;
        }
        if !params.is_empty() {
            params.push('&');
        }
        params.push_str(part);
    }
    if params.is_empty() {
        base.to_string()
    } else {
        format!("{base}?{params}")
    }
}

fn continuation_url(listing_url: &str, token: &str) -> String {
    let base = strip_continuation(listing_url);
    let sep = if base.contains('?') { '&' } else { '?' };
    format!("{base}{sep}continuation-token={}", percent_encode(token))
}

fn catalog_tiles(body: &str) -> Vec<String> {
    body.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.to_string())
        .collect()
}

fn s3_tiles(listing_url: &str, first_body: &str) -> Vec<String> {
    let Some(base) = s3_base(listing_url) else {
        eprintln!("eri_compiler: {listing_url}: no scheme/host — the tile URLs stay unresolvable");
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut body = first_body.to_string();
    loop {
        for key in s3_page_tiles(&body) {
            out.push(format!("{base}/{key}"));
        }
        let token = s3_continuation(&body);
        if token.is_empty() {
            break;
        }
        let next = continuation_url(listing_url, &token);
        match fetch_raw_bytes(&next, 600) {
            Some(b) => body = String::from_utf8_lossy(&b).into_owned(),
            None => {
                eprintln!("eri_compiler: {next}: listing page returned void");
                break;
            }
        }
    }
    out
}

fn discover_index(url: &str) -> Vec<String> {
    match fetch_raw_bytes(url, 600) {
        Some(b) => {
            let text = String::from_utf8_lossy(&b);
            if text.trim_start().starts_with('<') {
                s3_tiles(url, &text)
            } else {
                catalog_tiles(&text)
            }
        }
        None => {
            eprintln!("eri_compiler: {url}: index fetch returned void");
            Vec::new()
        }
    }
}

fn source_bytes(src: &str) -> Option<(Vec<u8>, String)> {
    let leaf = src.rsplit('/').next().unwrap_or(src).to_string();
    if let Ok(b) = std::fs::read(src) {
        return Some((b, leaf));
    }
    fetch_raw_bytes(src, 600).map(|b| (b, leaf))
}

fn compile_one(bytes: &[u8], name: &str) -> Option<Vec<EriSample>> {
    let img = match parse_tiff(bytes) {
        Some(v) => v,
        None => {
            eprintln!("{name}: not a TIFF/GeoTIFF the reader parses — the tile stays unwritten");
            return None;
        }
    };
    let georef = georef_of(img.geo_keys.as_deref());
    report_image(&img, &georef);
    match collect(&img, &georef) {
        Ok(samples) => Some(samples),
        Err(e) => {
            eprintln!("eri_compiler: {name}: {e}");
            None
        }
    }
}

fn compile_many(sources: &[String], out_path: &str, ci_mode: bool) {
    let mut samples: Vec<EriSample> = Vec::new();
    let mut decoded = 0usize;
    for src in sources {
        let Some((bytes, name)) = source_bytes(src) else {
            eprintln!("eri_compiler: {src}: fetch returned void");
            continue;
        };
        let Some(tile) = compile_one(&bytes, &name) else {
            continue;
        };
        eprintln!("eri: {name} -> {} samples", tile.len());
        decoded += 1;
        samples.extend(tile);
    }
    eprintln!("eri: {} of {} tiles decoded", decoded, sources.len());
    if samples.is_empty() {
        eprintln!("eri_compiler: no tile yielded a sample — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    report_value_ranges(&samples);
    finish(&samples, out_path, ci_mode);
}

fn finish(samples: &[EriSample], out_path: &str, ci_mode: bool) {
    if let Err(e) = write_asset(samples, out_path) {
        eprintln!("eri_compiler: {e}");
        std::process::exit(1);
    }
    let written = match std::fs::read(out_path) {
        Ok(v) => v,
        Err(_) => {
            eprintln!(
                "eri_compiler: {out_path} read returned void — the roundtrip stays unverified"
            );
            std::process::exit(1);
        }
    };
    match parse_asset(&written) {
        Some(parsed) if parsed.len() == samples.len() => {
            let last = parsed.last().unwrap();
            eprintln!(
                "eri: {} samples, {} B -> {out_path}, roundtrip parses; last lat {:.6} lon {:.6} value {:.3}",
                parsed.len(),
                written.len(),
                last.lat,
                last.lon,
                last.value
            );
        }
        _ => {
            eprintln!("{out_path}: roundtrip parse returned void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, out_path) {
        eprintln!("upload: {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: eri_compiler --input <geotiff> | --url <url> --out <path> [--ci-mode]\n\
                 usage: eri_compiler --index <list-url|catalog> [--granule <url|path> ...] --out <path> [--ci-mode]";
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            eprintln!("{usage}");
            std::process::exit(1);
        }
    };

    let index = arg_value(&args, "--index");
    let granules = arg_values(&args, "--granule");
    if index.is_some() || !granules.is_empty() {
        let mut sources: Vec<String> = Vec::new();
        if let Some(idx) = &index {
            let tiles = discover_index(idx);
            eprintln!("eri: {idx} -> {} tiles", tiles.len());
            sources.extend(tiles);
        }
        sources.extend(granules);
        if sources.is_empty() {
            eprintln!(
                "eri_compiler: the enumeration carried no tile — the bin stays unwritten (0 honored)"
            );
            std::process::exit(1);
        }
        compile_many(&sources, &out_path, ci_mode);
        return;
    }

    let (bytes, name) = match arg_value(&args, "--input") {
        Some(path) => {
            let leaf = path.rsplit('/').next().unwrap_or(&path).to_string();
            match std::fs::read(&path) {
                Ok(b) => (b, leaf),
                Err(e) => {
                    eprintln!("{path}: read returned void ({e})");
                    std::process::exit(1);
                }
            }
        }
        None => match arg_value(&args, "--url") {
            Some(url) => {
                let leaf = url.rsplit('/').next().unwrap_or(&url).to_string();
                match fetch_raw_bytes(&url, 600) {
                    Some(b) => (b, leaf),
                    None => {
                        eprintln!("{url}: fetch returned void");
                        std::process::exit(1);
                    }
                }
            }
            None => {
                eprintln!("{usage}");
                std::process::exit(1);
            }
        },
    };

    let img = match parse_tiff(&bytes) {
        Some(v) => v,
        None => {
            eprintln!("{name}: not a TIFF/GeoTIFF the reader parses — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    let georef = georef_of(img.geo_keys.as_deref());
    report_image(&img, &georef);
    let samples = match collect(&img, &georef) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("eri_compiler: {e}");
            std::process::exit(1);
        }
    };
    report_value_ranges(&samples);
    finish(&samples, &out_path, ci_mode);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn web_mercator_inverts_to_latlon() {
        let (lat0, lon0) = web_mercator_to_latlon(0.0, 0.0).unwrap();
        assert_eq!(lat0, 0.0);
        assert_eq!(lon0, 0.0);
        let (lat, lon) = web_mercator_to_latlon(6_378_137.0 * std::f64::consts::PI, 0.0).unwrap();
        assert!(lon > 179.99 && lon < 180.01);
        assert!(lat.abs() < 0.01);
        let phi = 60.0f64.to_radians();
        let y = 6_378_137.0 * (std::f64::consts::FRAC_PI_4 + phi / 2.0).tan().ln();
        let (north, _) = web_mercator_to_latlon(0.0, y).unwrap();
        assert!((north - 60.0).abs() < 1e-6);
        assert!(web_mercator_to_latlon(0.0, f64::NAN).is_none());
        assert!(web_mercator_to_latlon(0.0, 1e9).is_none());
    }

    #[test]
    fn georef_reads_model_type_and_projected_cs() {
        let keys = [1u16, 1, 0, 2, 1024, 0, 1, 1, 3072, 0, 1, 3857];
        assert!(matches!(georef_of(Some(&keys)), Georef::WebMercator));

        let geographic = [1u16, 1, 0, 1, 1024, 0, 1, 2];
        assert!(matches!(georef_of(Some(&geographic)), Georef::Geographic));

        let other = [1u16, 1, 0, 2, 1024, 0, 1, 1, 3072, 0, 1, 32633];
        assert!(matches!(
            georef_of(Some(&other)),
            Georef::Unsupported(32633)
        ));

        assert!(matches!(georef_of(None), Georef::Geographic));
    }

    #[test]
    fn geographic_lonlat_reads_from_geotransform() {
        let geo = GeoTransform {
            x0: 10.0,
            y0: 20.0,
            dx: 0.5,
            dy: -0.5,
        };
        let georef = Georef::Geographic;
        let (lat, lon) = lonlat_of(&geo, &georef, 0, 0).unwrap();
        assert_eq!(lon, 10.25);
        assert_eq!(lat, 19.75);
        assert!(lonlat_of(&geo, &georef, 1000, 0).is_none());
    }

    #[test]
    fn asset_roundtrip_and_rejections() {
        let samples = vec![
            EriSample {
                lat: 30.253,
                lon: -88.343,
                value: 128.0,
            },
            EriSample {
                lat: 30.253,
                lon: -88.342,
                value: 255.0,
            },
        ];
        let out = "/tmp/opencode/eri_test_asset.bin";
        let bytes = write_asset(&samples, out).unwrap();
        assert_eq!(bytes, 8 + samples.len() * REC_BYTES);
        let read = std::fs::read(out).unwrap();
        let parsed = parse_asset(&read).unwrap();
        assert_eq!(parsed.len(), samples.len());
        assert_eq!(parsed[0].lat, 30.253);
        assert_eq!(parsed[0].lon, -88.343);
        assert_eq!(parsed[1].value, 255.0);

        assert!(parse_asset(b"X").is_none());
        assert!(parse_asset(b"ERI1abc").is_none());
        assert!(parse_asset(&read[..read.len() - 1]).is_none());
        let mut nan = read.clone();
        nan[8..16].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_asset(&nan).is_none());
    }

    fn ifd_entry(out: &mut Vec<u8>, tag: u16, field_type: u16, count: u32, value: u32) {
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&field_type.to_le_bytes());
        out.extend_from_slice(&count.to_le_bytes());
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn build_gray_geotiff(
        width: u32,
        height: u32,
        pixels: &[u8],
        scale: [f64; 3],
        tie: [f64; 6],
    ) -> Vec<u8> {
        let entry_count = 11u16;
        let pixel_offset = (8 + 2 + entry_count as usize * 12 + 4) as u32;
        let scale_offset = pixel_offset + pixels.len() as u32;
        let tie_offset = scale_offset + 24;
        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        out.extend_from_slice(&42u16.to_le_bytes());
        out.extend_from_slice(&8u32.to_le_bytes());
        out.extend_from_slice(&entry_count.to_le_bytes());
        ifd_entry(&mut out, 256, 4, 1, width);
        ifd_entry(&mut out, 257, 4, 1, height);
        ifd_entry(&mut out, 258, 3, 1, 8);
        ifd_entry(&mut out, 259, 3, 1, 1);
        ifd_entry(&mut out, 262, 3, 1, 1);
        ifd_entry(&mut out, 273, 4, 1, pixel_offset);
        ifd_entry(&mut out, 277, 3, 1, 1);
        ifd_entry(&mut out, 278, 4, 1, height);
        ifd_entry(&mut out, 279, 4, 1, pixels.len() as u32);
        ifd_entry(&mut out, 33550, 12, 3, scale_offset);
        ifd_entry(&mut out, 33922, 12, 6, tie_offset);
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(pixels);
        for v in scale {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in tie {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }

    #[test]
    fn collect_emits_lat_lon_value_from_geotiff() {
        let data = build_gray_geotiff(
            2,
            2,
            &[0x10, 0x20, 0x30, 0x40],
            [0.5, 0.5, 0.0],
            [0.0, 0.0, 0.0, 10.0, 20.0, 0.0],
        );
        let img = parse_tiff(&data).unwrap();
        assert_eq!(img.compression, 1);
        let georef = georef_of(img.geo_keys.as_deref());
        let samples = collect(&img, &georef).unwrap();
        assert_eq!(samples.len(), 4);
        assert_eq!(samples[0].lon, 10.25);
        assert_eq!(samples[0].lat, 19.75);
        assert_eq!(samples[0].value, 0x10 as f64);
        assert_eq!(samples[3].lon, 10.75);
        assert_eq!(samples[3].lat, 19.25);
        assert_eq!(samples[3].value, 0x40 as f64);
    }

    fn build_gray_jpeg_8x8() -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&[0xFF, 0xD8]);
        out.extend_from_slice(&[0xFF, 0xDB, 0x00, 0x43, 0x00]);
        out.extend_from_slice(&[1u8; 64]);
        out.extend_from_slice(&[
            0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x08, 0x00, 0x08, 0x01, 0x01, 0x11, 0x00,
        ]);
        out.extend_from_slice(&[0xFF, 0xC4, 0x00, 0x14, 0x00]);
        out.extend_from_slice(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        out.push(0x00);
        out.extend_from_slice(&[0xFF, 0xC4, 0x00, 0x14, 0x10]);
        out.extend_from_slice(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        out.push(0x00);
        out.extend_from_slice(&[0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00]);
        out.push(0x3F);
        out.extend_from_slice(&[0xFF, 0xD9]);
        out
    }

    fn build_jpeg_strip_tiff(jpeg: &[u8]) -> Vec<u8> {
        let entry_count = 9u16;
        let strip_offset = (8 + 2 + entry_count as usize * 12 + 4) as u32;
        let mut out = Vec::new();
        out.extend_from_slice(b"II");
        out.extend_from_slice(&42u16.to_le_bytes());
        out.extend_from_slice(&8u32.to_le_bytes());
        out.extend_from_slice(&entry_count.to_le_bytes());
        ifd_entry(&mut out, 256, 4, 1, 8);
        ifd_entry(&mut out, 257, 4, 1, 8);
        ifd_entry(&mut out, 258, 3, 1, 8);
        ifd_entry(&mut out, 259, 3, 1, 7);
        ifd_entry(&mut out, 262, 3, 1, 1);
        ifd_entry(&mut out, 273, 4, 1, strip_offset);
        ifd_entry(&mut out, 277, 3, 1, 1);
        ifd_entry(&mut out, 278, 4, 1, 8);
        ifd_entry(&mut out, 279, 4, 1, jpeg.len() as u32);
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(jpeg);
        out
    }

    #[test]
    fn compression_7_strip_decodes() {
        let jpeg = build_gray_jpeg_8x8();
        let data = build_jpeg_strip_tiff(&jpeg);
        let img = parse_tiff(&data).unwrap();
        assert_eq!(img.compression, 7);
        assert_eq!(img.width, 8);
        assert_eq!(img.height, 8);
        assert_eq!(img.samples_per_pixel, 1);
        assert_eq!(img.pixels.len(), 64);
        assert!(img.pixels.iter().all(|&b| b == 128));
    }

    #[test]
    fn arg_values_collects_repeated_flags() {
        let args = vec![
            "--granule".to_string(),
            "a.tif".to_string(),
            "--granule".to_string(),
            "b.tif".to_string(),
        ];
        assert_eq!(
            arg_values(&args, "--granule"),
            vec!["a.tif".to_string(), "b.tif".to_string()]
        );
        assert!(arg_values(&args, "--url").is_empty());
    }

    #[test]
    fn s3_listing_extracts_tif_keys_and_token() {
        let body = r#"<?xml version="1.0"?><ListBucketResult><IsTruncated>true</IsTruncated><NextContinuationToken>abc+/def=</NextContinuationToken><Contents><Key>2005_Hurricane_Katrina/aug30JpegTiles_GCS_NAD83/aug30C0883430w302530n.tif</Key><Size>1</Size></Contents><Contents><Key>2005_Hurricane_Katrina/aug30JpegTiles_GCS_NAD83/readme.txt</Key></Contents><CommonPrefixes><Prefix>other/</Prefix></CommonPrefixes></ListBucketResult>"#;
        let tiles = s3_page_tiles(body);
        assert_eq!(tiles.len(), 1);
        assert!(tiles[0].ends_with("aug30C0883430w302530n.tif"));
        assert_eq!(s3_continuation(body), "abc+/def=");
    }

    #[test]
    fn s3_listing_not_truncated_stops() {
        let body = r#"<ListBucketResult><IsTruncated>false</IsTruncated><Contents><Key>a.tif</Key></Contents></ListBucketResult>"#;
        assert_eq!(s3_page_tiles(body).len(), 1);
        assert_eq!(s3_continuation(body), "");
    }

    #[test]
    fn catalog_lines_become_tile_urls() {
        let body = "# a comment\n\n  https://host/a.tif  \n\nhttps://host/b.tif\n";
        assert_eq!(
            catalog_tiles(body),
            vec![
                "https://host/a.tif".to_string(),
                "https://host/b.tif".to_string()
            ]
        );
    }

    #[test]
    fn s3_base_reads_scheme_and_host() {
        assert_eq!(
            s3_base("https://noaa-eri-pds.s3.amazonaws.com/?list-type=2").as_deref(),
            Some("https://noaa-eri-pds.s3.amazonaws.com")
        );
        assert_eq!(
            s3_base(
                "https://noaa-eri-pds.s3.amazonaws.com/2005_Hurricane_Katrina/aug30JpegTiles_GCS_NAD83/?list-type=2&prefix=p"
            )
            .as_deref(),
            Some("https://noaa-eri-pds.s3.amazonaws.com")
        );
    }

    #[test]
    fn continuation_url_strips_and_encodes() {
        let url = "https://host/bucket?list-type=2&prefix=p/";
        let next = continuation_url(url, "a+/b=c");
        assert_eq!(
            next,
            "https://host/bucket?list-type=2&prefix=p/&continuation-token=a%2B%2Fb%3Dc"
        );
        assert_eq!(
            continuation_url(&next, "x"),
            "https://host/bucket?list-type=2&prefix=p/&continuation-token=x"
        );
    }
}
