use omegaflow::archivar::bzip2;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::hsd::{parse_hsd, parse_segment, write_segment, AhiSegment, HsdFile};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::geostationary_lat_lon;
use omegaflow::lsk::days_from_civil;
use std::env;
use std::fs;

const BLOCK_PROJECTION: u8 = 3;
const BLOCK_NAVIGATION: u8 = 4;
const BLOCK_CALIBRATION: u8 = 5;

struct ProjectionBlock {
    sub_lon_deg: f64,
    cfac: u32,
    lfac: u32,
    coff: f32,
    loff: f32,
    distance_earth_center_km: f64,
    equatorial_radius_km: f64,
    polar_radius_km: f64,
}

struct NavigationBlock {
    ssp_lon_deg: f64,
    ssp_lat_deg: f64,
    nadir_lon_deg: f64,
    nadir_lat_deg: f64,
}

struct CalibrationBlock {
    band: u16,
    central_wavelength_um: f64,
    valid_bits: u16,
    error_pixels: u16,
    outside_scan_pixels: u16,
    gain: f64,
    offset: f64,
}

struct RadianceStats {
    min: f64,
    max: f64,
    mean: f64,
    valid: usize,
}

fn block_contents(bytes: &[u8]) -> Option<Vec<(u8, Vec<u8>)>> {
    let decompressed;
    let b: &[u8] = if bytes.starts_with(b"BZh") {
        decompressed = bzip2::decompress(bytes)?;
        &decompressed
    } else {
        bytes
    };
    let mut out = Vec::new();
    let mut offset = 0usize;
    loop {
        if offset + 3 > b.len() {
            break;
        }
        let block_type = b[offset];
        let block_len =
            u16::from_le_bytes(b.get(offset + 1..offset + 3)?.try_into().ok()?) as usize;
        if block_len < 3 || offset + block_len > b.len() {
            break;
        }
        out.push((block_type, b[offset + 3..offset + block_len].to_vec()));
        offset += block_len;
        if block_type >= 11 {
            break;
        }
    }
    Some(out)
}

fn f64_le(content: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(
        content.get(off..off + 8)?.try_into().ok()?,
    ))
}

fn u32_le(content: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        content.get(off..off + 4)?.try_into().ok()?,
    ))
}

fn u16_le(content: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_le_bytes(
        content.get(off..off + 2)?.try_into().ok()?,
    ))
}

fn f32_le(content: &[u8], off: usize) -> Option<f32> {
    Some(f32::from_le_bytes(
        content.get(off..off + 4)?.try_into().ok()?,
    ))
}

fn projection_block(blocks: &[(u8, Vec<u8>)]) -> Option<ProjectionBlock> {
    let (_, c) = blocks.iter().find(|(t, _)| *t == BLOCK_PROJECTION)?;
    let sub_lon_deg = f64_le(c, 0)?;
    let cfac = u32_le(c, 8)?;
    let lfac = u32_le(c, 12)?;
    let coff = f32_le(c, 16)?;
    let loff = f32_le(c, 20)?;
    let distance_earth_center_km = f64_le(c, 24)?;
    let equatorial_radius_km = f64_le(c, 32)?;
    let polar_radius_km = f64_le(c, 40)?;
    Some(ProjectionBlock {
        sub_lon_deg,
        cfac,
        lfac,
        coff,
        loff,
        distance_earth_center_km,
        equatorial_radius_km,
        polar_radius_km,
    })
}

fn navigation_block(blocks: &[(u8, Vec<u8>)]) -> Option<NavigationBlock> {
    let (_, c) = blocks.iter().find(|(t, _)| *t == BLOCK_NAVIGATION)?;
    let ssp_lon_deg = f64_le(c, 8)?;
    let ssp_lat_deg = f64_le(c, 16)?;
    let nadir_lon_deg = f64_le(c, 32)?;
    let nadir_lat_deg = f64_le(c, 40)?;
    Some(NavigationBlock {
        ssp_lon_deg,
        ssp_lat_deg,
        nadir_lon_deg,
        nadir_lat_deg,
    })
}

fn calibration_block(blocks: &[(u8, Vec<u8>)]) -> Option<CalibrationBlock> {
    let (_, c) = blocks.iter().find(|(t, _)| *t == BLOCK_CALIBRATION)?;
    let band = u16_le(c, 0)?;
    let central_wavelength_um = f64_le(c, 2)?;
    let valid_bits = u16_le(c, 10)?;
    let error_pixels = u16_le(c, 12)?;
    let outside_scan_pixels = u16_le(c, 14)?;
    let gain = f64_le(c, 16)?;
    let offset = f64_le(c, 24)?;
    Some(CalibrationBlock {
        band,
        central_wavelength_um,
        valid_bits,
        error_pixels,
        outside_scan_pixels,
        gain,
        offset,
    })
}

fn radiance_stats(counts: &[u16], cal: &CalibrationBlock) -> RadianceStats {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0;
    let mut valid = 0usize;
    for &count in counts {
        if count == cal.error_pixels || count == cal.outside_scan_pixels {
            continue;
        }
        let rad = cal.gain * count as f64 + cal.offset;
        if !rad.is_finite() {
            continue;
        }
        if rad < min {
            min = rad;
        }
        if rad > max {
            max = rad;
        }
        sum += rad;
        valid += 1;
    }
    RadianceStats {
        min,
        max,
        mean: if valid > 0 { sum / valid as f64 } else { 0.0 },
        valid,
    }
}

fn center_lat_lon(proj: &ProjectionBlock, columns: u16, lines: u16) -> Option<(f64, f64)> {
    let x = (columns as f64 / 2.0 - proj.coff as f64) / ((1u64 << 16) as f64 * proj.cfac as f64);
    let y = (lines as f64 / 2.0 - proj.loff as f64) / ((1u64 << 16) as f64 * proj.lfac as f64);
    let altitude_m = (proj.distance_earth_center_km - proj.equatorial_radius_km) * 1000.0;
    if !x.is_finite() || !y.is_finite() || !altitude_m.is_finite() || altitude_m <= 0.0 {
        return None;
    }
    Some(geostationary_lat_lon(x, y, proj.sub_lon_deg, altitude_m))
}

const NETLOC: &str = "noaa-himawari8.s3.amazonaws.com";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn obs_unix(date: &str, time: &str) -> Option<f64> {
    let y: i64 = date[0..4].parse().ok()?;
    let m: i64 = date[4..6].parse().ok()?;
    let d: i64 = date[6..8].parse().ok()?;
    let h: i64 = time[0..2].parse().ok()?;
    let mi: i64 = time[2..4].parse().ok()?;
    let days = days_from_civil(y, m, d)?;
    Some(days as f64 * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0)
}

struct FilenameMeta {
    satellite: u8,
    band: u8,
    segment: u8,
    resolution_m: u16,
    obs_sec: f64,
    obs_present: u8,
}

fn satellite(tok: &str) -> Option<u8> {
    let body = tok.strip_prefix('H')?;
    if body.len() < 2 || !body.as_bytes()[0..2].iter().all(|b| b.is_ascii_digit()) {
        return None;
    }
    body[0..2].parse().ok()
}

fn band(tok: &str) -> Option<u8> {
    let body = tok.strip_prefix('B')?;
    if body.len() < 2 || !body.as_bytes()[0..2].iter().all(|b| b.is_ascii_digit()) {
        return None;
    }
    body[0..2].parse().ok()
}

fn resolution_m(tok: &str) -> Option<u16> {
    let body = tok.strip_prefix('R')?;
    if body.len() < 2 || !body.as_bytes()[0..2].iter().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let v: u16 = body[0..2].parse().ok()?;
    Some(v * 100)
}

fn segment_index(tok: &str) -> Option<u8> {
    let body = tok.strip_prefix('S')?;
    if body.len() < 4 || !body.as_bytes()[0..4].iter().all(|b| b.is_ascii_digit()) {
        return None;
    }
    body[0..2].parse().ok()
}

fn filename_meta(name: &str) -> FilenameMeta {
    let base = name.rsplit('/').next().unwrap_or(name);
    let mut meta = FilenameMeta {
        satellite: 0,
        band: 0,
        segment: 0,
        resolution_m: 0,
        obs_sec: 0.0,
        obs_present: 0,
    };
    let mut date: Option<&str> = None;
    let mut time: Option<&str> = None;
    for tok in base.split(|c| c == '_' || c == '.') {
        match tok.as_bytes().first().copied() {
            Some(b'H') => {
                if let Some(s) = satellite(tok) {
                    meta.satellite = s;
                }
            }
            Some(b'B') => {
                if let Some(b) = band(tok) {
                    meta.band = b;
                }
            }
            Some(b'R') => {
                if let Some(r) = resolution_m(tok) {
                    meta.resolution_m = r;
                }
            }
            Some(b'S') => {
                if let Some(s) = segment_index(tok) {
                    meta.segment = s;
                }
            }
            _ => {}
        }
        if tok.len() == 8 && tok.bytes().all(|b| b.is_ascii_digit()) {
            date = Some(tok);
        } else if tok.len() == 4 && tok.bytes().all(|b| b.is_ascii_digit()) {
            time = Some(tok);
        }
    }
    if let (Some(d), Some(t)) = (date, time) {
        if let Some(unix) = obs_unix(d, t) {
            meta.obs_sec = unix;
            meta.obs_present = 1;
        }
    }
    meta
}

fn report_decoded(name: &str, bytes: &[u8], hsd: &HsdFile) {
    let Some(blocks) = block_contents(bytes) else {
        println!("{name}: block enumeration returned void");
        return;
    };
    if let Some(proj) = projection_block(&blocks) {
        println!(
            "  projection: sub_lon {:.4} deg, CFAC {} LFAC {} COFF {} LOFF {}",
            proj.sub_lon_deg, proj.cfac, proj.lfac, proj.coff, proj.loff
        );
        println!(
            "  projection: earth-center distance {:.1} km, equatorial radius {:.3} km, polar radius {:.3} km",
            proj.distance_earth_center_km, proj.equatorial_radius_km, proj.polar_radius_km
        );
        if let Some((lat, lon)) = center_lat_lon(&proj, hsd.columns, hsd.lines) {
            println!("  center pixel -> lat {:.4} lon {:.4}", lat, lon);
        }
    } else {
        println!("  projection: absent (block 3)");
    }
    if let Some(nav) = navigation_block(&blocks) {
        println!(
            "  navigation: SSP lon {:.4} lat {:.4}, nadir lon {:.4} lat {:.4}",
            nav.ssp_lon_deg, nav.ssp_lat_deg, nav.nadir_lon_deg, nav.nadir_lat_deg
        );
    } else {
        println!("  navigation: absent (block 4)");
    }
    if let Some(cal) = calibration_block(&blocks) {
        println!(
            "  calibration: band {}, central wavelength {:.4} um, valid bits {}, gain {:.6} offset {:.4}",
            cal.band, cal.central_wavelength_um, cal.valid_bits, cal.gain, cal.offset
        );
        let stats = radiance_stats(&hsd.pixel_values, &cal);
        if stats.valid > 0 {
            println!(
                "  radiance: mean {:.6} min {:.6} max {:.6} W m-2 sr-1 um-1, {} valid / {} counts",
                stats.mean,
                stats.min,
                stats.max,
                stats.valid,
                hsd.pixel_values.len()
            );
        } else {
            println!(
                "  radiance: no valid count — every count is an error or outside-scan sentinel (0 honored)"
            );
        }
    } else {
        println!("  calibration: absent (block 5)");
    }
}

fn probe(name: &str, bytes: &[u8], hsd: &HsdFile) {
    println!("{name}: {} B", bytes.len());
    println!(
        "  columns {} lines {} bits_per_pixel {}",
        hsd.columns,
        hsd.lines,
        match hsd.bits_per_pixel {
            Some(b) => b.to_string(),
            None => "absent".to_string(),
        }
    );
    println!("  blocks {}", hsd.blocks.len());
    for (t, l) in &hsd.blocks {
        println!("    block_type {} len {}", t, l);
    }
    println!(
        "  pixel_values {} (raw counts, u16)",
        hsd.pixel_values.len()
    );
    report_decoded(name, bytes, hsd);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "himawari_ahi_counts.bin".to_string(),
    };

    let in_path = arg_value(&args, "--in");
    let (bytes, name) = match in_path.as_deref().and_then(|p| fs::read(p).ok()) {
        Some(b) => {
            let name = match in_path
                .as_deref()
                .map(|p| p.rsplit('/').next().unwrap_or(p).to_string())
            {
                Some(v) => v,
                None => "segment".to_string(),
            };
            (b, name)
        }
        None => {
            let url = match arg_value(&args, "--url") {
                Some(u) => u,
                None => {
                    eprintln!("--url <HSD .DAT.bz2> or --in <file> required");
                    std::process::exit(1);
                }
            };
            let name = url.rsplit('/').next().unwrap_or("segment").to_string();
            match fetch_raw_bytes(&url, 3600) {
                Some(b) => (b, name),
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            }
        }
    };

    let hsd = match parse_hsd(&bytes) {
        Some(h) => h,
        None => {
            eprintln!("{name}: parse_hsd returned void");
            std::process::exit(1);
        }
    };

    if args.iter().any(|a| a == "--probe") {
        probe(&name, &bytes, &hsd);
        return;
    }

    report_decoded(&name, &bytes, &hsd);

    let columns = hsd.columns;
    let lines = hsd.lines;
    let bits_per_pixel = match hsd.bits_per_pixel {
        Some(v) => v,
        None => 0,
    };
    let counts = hsd.pixel_values;
    if counts.is_empty() {
        eprintln!("{name}: no pixel values — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    if counts.len() != columns as usize * lines as usize {
        eprintln!(
            "{name}: {} counts disagree with the {}x{} grid — the bin stays unwritten",
            counts.len(),
            columns,
            lines
        );
        std::process::exit(1);
    }

    let meta = filename_meta(&name);
    let seg = AhiSegment {
        columns,
        lines,
        bits_per_pixel,
        band: meta.band,
        segment: meta.segment,
        satellite: meta.satellite,
        resolution_m: meta.resolution_m,
        obs_sec: meta.obs_sec,
        obs_present: meta.obs_present,
        counts,
    };

    let bin = write_segment(&seg);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = fs::create_dir_all(parent);
    }
    if fs::write(&out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_segment(&bin) {
        Some(parsed) => {
            eprintln!(
                "{name}: {} raw counts written ({}, unit counts, {} B), roundtrip parses",
                parsed.counts.len(),
                out,
                bin.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calibration_content(gain: f64, offset: f64) -> Vec<u8> {
        let mut c = vec![0u8; 32];
        c[0..2].copy_from_slice(&7u16.to_le_bytes());
        c[2..10].copy_from_slice(&3.9f64.to_le_bytes());
        c[10..12].copy_from_slice(&10u16.to_le_bytes());
        c[12..14].copy_from_slice(&0u16.to_le_bytes());
        c[14..16].copy_from_slice(&65535u16.to_le_bytes());
        c[16..24].copy_from_slice(&gain.to_le_bytes());
        c[24..32].copy_from_slice(&offset.to_le_bytes());
        c
    }

    #[test]
    fn decodes_calibration_block_5() {
        let blocks = vec![(BLOCK_CALIBRATION, calibration_content(0.001_5, -1.0))];
        let cal = calibration_block(&blocks).unwrap();
        assert_eq!(cal.band, 7);
        assert_eq!(cal.central_wavelength_um, 3.9);
        assert_eq!(cal.valid_bits, 10);
        assert_eq!(cal.error_pixels, 0);
        assert_eq!(cal.outside_scan_pixels, 65535);
        assert_eq!(cal.gain, 0.001_5);
        assert_eq!(cal.offset, -1.0);
    }

    #[test]
    fn radiance_skips_error_and_outside_scan_sentinels() {
        let cal = CalibrationBlock {
            band: 7,
            central_wavelength_um: 3.9,
            valid_bits: 10,
            error_pixels: 0,
            outside_scan_pixels: 65535,
            gain: 0.01,
            offset: 0.5,
        };
        let stats = radiance_stats(&[100, 200, 0, 65535], &cal);
        assert_eq!(stats.valid, 2);
        assert_eq!(stats.min, 1.5);
        assert_eq!(stats.max, 2.5);
        assert!((stats.mean - 2.0).abs() < 1e-9);
    }

    #[test]
    fn decodes_projection_block_3() {
        let mut c = vec![0u8; 48];
        c[0..8].copy_from_slice(&140.7f64.to_le_bytes());
        c[8..12].copy_from_slice(&40932549u32.to_le_bytes());
        c[12..16].copy_from_slice(&40932549u32.to_le_bytes());
        c[16..20].copy_from_slice(&5500.5f32.to_le_bytes());
        c[20..24].copy_from_slice(&5500.5f32.to_le_bytes());
        c[24..32].copy_from_slice(&42164.0f64.to_le_bytes());
        c[32..40].copy_from_slice(&6378.137f64.to_le_bytes());
        c[40..48].copy_from_slice(&6356.7523f64.to_le_bytes());
        let blocks = vec![(BLOCK_PROJECTION, c)];
        let proj = projection_block(&blocks).unwrap();
        assert_eq!(proj.sub_lon_deg, 140.7);
        assert_eq!(proj.cfac, 40932549);
        assert_eq!(proj.lfac, 40932549);
        assert_eq!(proj.coff, 5500.5);
        assert_eq!(proj.loff, 5500.5);
        assert_eq!(proj.distance_earth_center_km, 42164.0);
        assert_eq!(proj.equatorial_radius_km, 6378.137);
        assert_eq!(proj.polar_radius_km, 6356.7523);
    }

    #[test]
    fn decodes_navigation_block_4() {
        let mut c = vec![0u8; 48];
        c[8..16].copy_from_slice(&140.7f64.to_le_bytes());
        c[16..24].copy_from_slice(&0.0f64.to_le_bytes());
        c[32..40].copy_from_slice(&140.7f64.to_le_bytes());
        c[40..48].copy_from_slice(&0.0f64.to_le_bytes());
        let blocks = vec![(BLOCK_NAVIGATION, c)];
        let nav = navigation_block(&blocks).unwrap();
        assert_eq!(nav.ssp_lon_deg, 140.7);
        assert_eq!(nav.ssp_lat_deg, 0.0);
        assert_eq!(nav.nadir_lon_deg, 140.7);
        assert_eq!(nav.nadir_lat_deg, 0.0);
    }

    #[test]
    fn center_pixel_maps_near_the_subsatellite_point() {
        let proj = ProjectionBlock {
            sub_lon_deg: 140.7,
            cfac: 40932549,
            lfac: 40932549,
            coff: 5500.5,
            loff: 5500.5,
            distance_earth_center_km: 42164.0,
            equatorial_radius_km: 6378.137,
            polar_radius_km: 6356.7523,
        };
        let (lat, lon) = center_lat_lon(&proj, 11000, 11000).unwrap();
        assert!(lat.abs() < 1e-3, "center lat {lat}");
        assert!((lon - 140.7).abs() < 1e-3, "center lon {lon}");
    }
}
