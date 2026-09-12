use std::collections::HashMap;

use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::atdf::{parse_bin, reduce_skyfreq, write_bin, S_BAND_REF_HI, S_BAND_REF_LO};
use omegaflow::cdn::upload_release;

const LISTING: &str = "https://spdf.gsfc.nasa.gov/pub/data/pioneer/pioneer10/radio/Data/ATDF_Data-Files_CMarkwardt_Readable/";
const NETLOC: &str = "spdf.gsfc.nasa.gov";

fn listing_files(html: &[u8]) -> Vec<String> {
    let text = String::from_utf8_lossy(html);
    let mut names: Vec<String> = Vec::new();
    let mut rest = text.as_ref();
    while let Some(pos) = rest.find("href=\"") {
        rest = &rest[pos + 6..];
        let Some(end) = rest.find('"') else { break };
        let href = &rest[..end];
        rest = &rest[end + 1..];
        if href.is_empty()
            || href.starts_with('?')
            || href.starts_with('#')
            || href.starts_with('/')
            || href.ends_with('/')
            || href == "SHA1SUM"
        {
            continue;
        }
        names.push(href.to_string());
    }
    names.sort();
    names.dedup();
    names
}

fn jd_date(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / 86400.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb_s:.0} s"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let list_mode = args.iter().any(|a| a == "--list");
    let Some(index) = fetch_raw_bytes(LISTING, 86400) else {
        eprintln!("{LISTING}: fetch void — the series stays unwritten (0 honored)");
        return;
    };
    let files = listing_files(&index);
    if files.is_empty() {
        eprintln!("{LISTING}: no ATDF files in listing — the series stays unwritten (0 honored)");
        return;
    }
    if list_mode {
        eprintln!("{LISTING}: {} ATDF files", files.len());
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
    for (fid, name) in files.iter().enumerate() {
        let url = format!("{LISTING}{name}");
        let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
            eprintln!("{name}: fetch void ({url})");
            continue;
        };
        let digest = sha256_hex(&bytes);
        if let Some(first) = seen.get(&digest) {
            eprintln!("{name}: sha256 {digest} — alias of {first} (counted once)");
            continue;
        }
        seen.insert(digest, name.clone());
        if let Some(samples) =
            reduce_skyfreq(name, fid as f64, &bytes, &lsk, S_BAND_REF_LO, S_BAND_REF_HI)
        {
            merged.extend(samples);
        }
    }
    if merged.is_empty() {
        eprintln!("no fsky samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/spdf.gsfc.nasa.gov/pioneer10_skyfreq.bin";
    std::fs::create_dir_all("data/spdf.gsfc.nasa.gov").ok();
    let bin = write_bin(&merged);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match parse_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            let mut f: Vec<f64> = parsed.iter().map(|r| r[1]).collect();
            f.sort_by(f64::total_cmp);
            let fmed = f[f.len() / 2];
            eprintln!(
                "{out}: {} Samples ({}..{}), fsky {:.3e}..{:.3e} Hz (Median {:.6e} Hz), {} B — roundtrip parses",
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
        }
    }
    if ci_mode && !upload_release(NETLOC, out) {
        std::process::exit(1);
    }
}
