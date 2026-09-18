use std::collections::{HashMap, HashSet};

use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::lro_utf::{parse_bin, reduce_lro_trk, write_bin};
use omegaflow::odf;
use omegaflow::spectral::civil_from_days;

const LISTING: &str = "http://imbrium.mit.edu/LRORS/DATA/TRK/";
const NETLOC: &str = "imbrium.mit.edu";
const PREFIX: &str = "lro_trk";
const DIR: &str = "data/imbrium.mit.edu";
const ROW_BYTES: usize = 112;
const MAX_DEPTH: usize = 4;
const FILE_CAP: usize = 100_000;

fn listing_hrefs(html: &[u8]) -> Vec<String> {
    let text = String::from_utf8_lossy(html);
    let mut names: Vec<String> = Vec::new();
    let mut rest = text.as_ref();
    while let Some(pos) = rest.find("href=\"") {
        rest = &rest[pos + 6..];
        let Some(end) = rest.find('"') else { break };
        let href = &rest[..end];
        rest = &rest[end + 1..];
        if href.starts_with('?') || href.starts_with('/') {
            continue;
        }
        names.push(href.to_string());
    }
    names.sort();
    names.dedup();
    names
}

fn year_dir_selected(name: &str, year: &str) -> bool {
    if name.len() == 7 && name.bytes().all(|b| b.is_ascii_digit()) {
        return name.starts_with(year);
    }
    true
}

fn collect_trk_files(
    seed: &str,
    depth: usize,
    year: &str,
    out: &mut Vec<String>,
    visited: &mut HashSet<String>,
) {
    if depth == 0 || out.len() >= FILE_CAP {
        return;
    }
    let Some(html) = fetch_raw_bytes(seed, 86400) else {
        eprintln!("{seed}: listing fetch void");
        return;
    };
    let mut dirs: Vec<String> = Vec::new();
    for href in listing_hrefs(&html) {
        if href.to_uppercase().ends_with(".TRK") {
            out.push(format!("{seed}{href}"));
            continue;
        }
        if href.ends_with('/') && !href.starts_with("../") {
            let name = &href[..href.len() - 1];
            if year_dir_selected(name, year) {
                dirs.push(format!("{seed}{href}"));
            }
        }
    }
    for d in dirs {
        if visited.insert(d.clone()) {
            collect_trk_files(&d, depth - 1, year, out, visited);
        }
    }
}

fn jd_date(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / 86400.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb_s:.0} s"),
    }
}

fn write_and_verify(records: &[[f64; 14]], out: &str) -> Vec<u8> {
    let bin = write_bin(records);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        std::process::exit(1);
    }
    match parse_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            let mut f: Vec<f64> = parsed.iter().map(|r| r[1]).collect();
            f.sort_by(f64::total_cmp);
            let fmed = f[f.len() / 2];
            eprintln!(
                "{out}: {} sky-frequency samples ({}..{}), fsky {:.3e}..{:.3e} Hz (median {:.6e} Hz), {} B — roundtrip parses",
                parsed.len(),
                jd_date(d0[0]),
                jd_date(d1[0]),
                d1[1],
                d0[1],
                fmed,
                bin.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
            std::process::exit(1);
        }
    }
    bin
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let list_mode = args.iter().any(|a| a == "--list");
    let year = args
        .iter()
        .position(|a| a == "--year")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str());
    if !list_mode && year.is_none() {
        eprintln!("usage: lro_trk_compiler --year <YYYY> [--list] [--ci-mode]");
        std::process::exit(2);
    }
    let year = year.unwrap_or("");
    let mut files: Vec<String> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(LISTING.to_string());
    collect_trk_files(LISTING, MAX_DEPTH, year, &mut files, &mut visited);
    if files.is_empty() {
        eprintln!("{LISTING}: no .TRK files in listing tree — the series stays unwritten (0 honored)");
        return;
    }
    files.sort();
    if list_mode {
        eprintln!("{LISTING}: {} .TRK files", files.len());
        for name in &files {
            eprintln!("  {name}");
        }
        return;
    }
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut merged: Vec<[f64; 14]> = Vec::new();
    let mut seen: HashMap<String, String> = HashMap::new();
    for (fid, url) in files.iter().enumerate() {
        let name = url.rsplit('/').next().unwrap_or("track").to_string();
        let Some(bytes) = fetch_raw_bytes(url, 604800) else {
            eprintln!("{name}: fetch void ({url})");
            continue;
        };
        let digest = sha256_hex(&bytes);
        if let Some(first) = seen.get(&digest) {
            eprintln!("{name}: sha256 {digest} — alias of {first} (counted once)");
            continue;
        }
        seen.insert(digest, name.clone());
        if let Some(samples) = reduce_lro_trk(&name, fid as f64, &bytes, &lsk) {
            merged.extend(samples);
        }
    }
    if merged.is_empty() {
        eprintln!("no fsky samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    std::fs::create_dir_all(DIR).ok();

    let asset_prefix = format!("{PREFIX}_{year}");
    let budget = 8 + (odf::PODF_SHARD_BUDGET - 8) * 72 / ROW_BYTES;
    let ranges = odf::podf_shard_ranges(merged.len(), budget);
    if ranges.len() == 1 {
        let out = format!("{DIR}/{asset_prefix}.bin");
        write_and_verify(&merged, &out);
        if ci_mode && !upload_release(NETLOC, &out) {
            std::process::exit(1);
        }
        return;
    }

    let mut names: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();
    for (ord, &(lo, hi)) in ranges.iter().enumerate() {
        let t_lo = merged[lo][0];
        let t_hi = merged[hi - 1][0];
        let mut name = odf::podf_shard_name(&asset_prefix, t_lo, t_hi);
        if names.contains(&name) {
            name = odf::podf_shard_name_ord(&asset_prefix, t_lo, t_hi, ord);
        }
        names.push(name.clone());
        let path = format!("{DIR}/{name}");
        let bin = write_and_verify(&merged[lo..hi], &path);
        if bin.len() > odf::PODF_SHARD_LIMIT {
            eprintln!(
                "{path}: {}-byte shard exceeds the {}-byte CDN asset limit — the series stays unwritten (0 honored)",
                bin.len(),
                odf::PODF_SHARD_LIMIT
            );
            std::process::exit(1);
        }
        paths.push(path);
    }
    for name in &names {
        println!(
            "url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}"
        );
        println!("format {PREFIX}");
        println!("at earth");
        println!("ttl 604800");
        println!("field sky_frequency_hz lro_sky_frequency_hz inverse-square em Hz 3600 0.0 0.0");
        println!();
    }
    if ci_mode {
        for path in &paths {
            if !upload_release(NETLOC, path) {
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::year_dir_selected;

    #[test]
    fn year_dir_selection() {
        assert!(year_dir_selected("2009169", "2009"));
        assert!(!year_dir_selected("2009169", "2012"));
        assert!(year_dir_selected("LRO_ES_01", "2009"));
        assert!(year_dir_selected("../", "2009"));
    }
}
