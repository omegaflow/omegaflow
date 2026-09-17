use omegaflow::archivar::geo::{COMP_LISOTD_FLASH_RAD, GeoRec, MAGIC_LISOTD, parse_bin, write_bin};
use omegaflow::cdn::upload_release;
use omegaflow::hdf4::{DFTAG_VH, Hdf4, field_value, field_values};
use omegaflow::lsk::{LeapSeconds, days_from_civil, parse as parse_lsk};
use omegaflow::lzw::uncompress_z;
use std::process::Command;

const NETLOC: &str = "ghrc.nasa.gov";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_secret(text: &str, key: &str) -> Option<String> {
    let mut found = None;
    for line in text.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key && !v.trim().is_empty() {
                found = Some(v.trim().to_string());
            }
        }
    }
    found
}

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    parse_secret(&body, name)
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sSLf")
        .arg("--retry")
        .arg("3")
        .arg("--max-time")
        .arg("240");
    if let Some(token) = secret("EARTHDATA_EDL_TOKEN") {
        cmd.arg("-H").arg(format!("Authorization: Bearer {token}"));
    }
    let out = cmd.arg(url).output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch {}: {}",
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn granule_bytes(path_or_url: &str) -> Option<Vec<u8>> {
    if let Ok(b) = std::fs::read(path_or_url) {
        return Some(b);
    }
    fetch(path_or_url)
}

fn is_tar(bytes: &[u8]) -> bool {
    bytes.len() > 262 && &bytes[257..262] == b"ustar"
}

fn tar_members(bytes: &[u8]) -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while off + 512 <= bytes.len() {
        let hdr = &bytes[off..off + 512];
        if hdr.iter().all(|&c| c == 0) {
            break;
        }
        let name_end = hdr[0..100].iter().position(|&c| c == 0).unwrap_or(100);
        let name = String::from_utf8_lossy(&hdr[0..name_end]).to_string();
        let size_field = String::from_utf8_lossy(&hdr[124..136]);
        let digits = size_field.trim_matches(|c: char| c == '\0' || c == ' ');
        let size = match usize::from_str_radix(digits, 8) {
            Ok(s) => s,
            Err(_) => break,
        };
        let start = off + 512;
        let end = match start.checked_add(size) {
            Some(e) if e <= bytes.len() => e,
            _ => break,
        };
        if !name.is_empty() {
            out.push((name, bytes[start..end].to_vec()));
        }
        let padded = size.div_ceil(512) * 512;
        off = start + padded;
    }
    out
}

fn tai93_to_tdb(lsk: &LeapSeconds, tai93: f64) -> Option<f64> {
    if !tai93.is_finite() {
        return None;
    }
    let epoch_days = days_from_civil(1993, 1, 1)?;
    let c = tai93 + epoch_days as f64 * 86400.0;
    let mut u = c - 37.0;
    for _ in 0..3 {
        u = c - lsk.leap_at(u)?;
    }
    if u < 0.0 {
        return None;
    }
    lsk.unix_to_tdb(u)
}

fn hdf4_flashes(raw: &[u8], lsk: &LeapSeconds) -> Vec<GeoRec> {
    let hdf = match Hdf4::parse(raw) {
        Some(h) => h,
        None => return Vec::new(),
    };
    let mut out = Vec::new();
    for dd in hdf.dds() {
        if dd.tag != DFTAG_VH {
            continue;
        }
        let (vh, data) = match hdf.vdata(dd.ref_) {
            Some(v) => v,
            None => continue,
        };
        if vh.vsname != "flash" {
            continue;
        }
        for r in 0..vh.nrecords as usize {
            let tai = match field_value(&vh, &data, r, "TAI93") {
                Some(v) => v,
                None => continue,
            };
            let cent = match field_values(&vh, &data, r, "cent") {
                Some(v) if v.len() >= 2 => v,
                _ => continue,
            };
            let rad = match field_value(&vh, &data, r, "rad") {
                Some(v) => v,
                None => continue,
            };
            let (lat, lon) = (cent[0], cent[1]);
            if !rad.is_finite() || rad < 0.0 {
                continue;
            }
            if !(lat.abs() <= 90.0) || !(lon.abs() <= 180.0) {
                continue;
            }
            let tdb = match tai93_to_tdb(lsk, tai) {
                Some(t) => t,
                None => continue,
            };
            out.push(GeoRec {
                t: tdb,
                lat,
                lon,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val: rad,
                comp: COMP_LISOTD_FLASH_RAD,
                station: 0,
            });
        }
    }
    out
}

fn granule_records(bytes: &[u8], lsk: &LeapSeconds, src: &str) -> Vec<GeoRec> {
    let mut out = Vec::new();
    if is_tar(bytes) {
        for (name, member) in tar_members(bytes) {
            if !name.ends_with(".Z") {
                continue;
            }
            match uncompress_z(&member) {
                Some(raw) => out.extend(hdf4_flashes(&raw, lsk)),
                None => eprintln!("lis_otd: {src}:{name} stays compressed (LZW void)"),
            }
        }
    } else if bytes.starts_with(&[0x1f, 0x9d]) {
        match uncompress_z(bytes) {
            Some(raw) => out.extend(hdf4_flashes(&raw, lsk)),
            None => eprintln!("lis_otd: {src} stays compressed (LZW void)"),
        }
    } else {
        out.extend(hdf4_flashes(bytes, lsk));
    }
    out
}

fn run(out_path: &str, granules: &[String], lsk: &LeapSeconds, ci: bool) -> Result<(), String> {
    let mut records: Vec<GeoRec> = Vec::new();
    for src in granules {
        let bytes = granule_bytes(src).ok_or(format!("{src} stayed unreadable"))?;
        let recs = granule_records(&bytes, lsk, src);
        eprintln!("lis_otd: {} → {} flashes", src, recs.len());
        records.extend(recs);
    }
    if records.is_empty() {
        return Err("no flashes harvested — the bin stays unwritten (0 honored)".into());
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
    let bytes = write_bin(MAGIC_LISOTD, &records);
    std::fs::write(out_path, &bytes).map_err(|e| format!("{out_path}: {e}"))?;
    match parse_bin(MAGIC_LISOTD, &bytes) {
        Some(parsed) => eprintln!(
            "{}: {} flashes, {} B, roundtrip parses",
            out_path,
            parsed.len(),
            bytes.len()
        ),
        None => {
            return Err(format!(
                "{out_path}: roundtrip parse void — the bin stays unverified"
            ));
        }
    }
    if ci && !upload_release(NETLOC, out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => {
            eprintln!(
                "lis_otd_compiler: --out <file.bin> absent — the output path is never silent"
            );
            std::process::exit(1);
        }
    };
    let ci = args.iter().any(|a| a == "--ci-mode");
    let granules: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(_, a)| a.as_str() == "--granule")
        .filter_map(|(i, _)| args.get(i + 1))
        .cloned()
        .collect();
    if granules.is_empty() {
        eprintln!("lis_otd_compiler: --granule <url|path> absent — refused");
        std::process::exit(1);
    }
    let lsk_path = match arg_value(&args, "--lsk") {
        Some(p) => p,
        None => {
            eprintln!("lis_otd_compiler: --lsk <naif0012.tls> absent — the TDB clock stays unread");
            std::process::exit(1);
        }
    };
    let lsk_text = match std::fs::read_to_string(&lsk_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "lis_otd_compiler: {} unreadable: {e} — the TDB clock stays unread",
                lsk_path
            );
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(&lsk_text) {
        Some(l) => l,
        None => {
            eprintln!(
                "lis_otd_compiler: {} parses void — the leap table stays unread",
                lsk_path
            );
            std::process::exit(1);
        }
    };
    if let Err(msg) = run(&out, &granules, &lsk, ci) {
        eprintln!("lis_otd_compiler: {msg}");
        std::process::exit(1);
    }
}
