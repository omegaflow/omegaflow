use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::hsd::{AhiSegment, HsdFile, parse_hsd, parse_segment, write_segment};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::env;
use std::fs;

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
    println!("  calibration absent — block 5 stays undecoded by the parser");
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
            eprintln!("calibration absent — the record carries counts, never radiance (0 honored)");
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
