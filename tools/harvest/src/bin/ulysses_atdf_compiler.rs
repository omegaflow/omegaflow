use std::collections::HashMap;

use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::atdf::{parse_bin, reduce_uly_skyfreq, write_bin};
use omegaflow::cdn::upload_release;
use omegaflow::odf;
use omegaflow::spectral::civil_from_days;

const LISTING: &str = "https://pds-ppi.igpp.ucla.edu/data/ULY-J-SCE-1-TDF-V1.0/DATA/";
const NETLOC: &str = "pds-ppi.igpp.ucla.edu";
const PREFIX: &str = "ulysses_atdf";
const PREFIX_X: &str = "ulysses_atdf_x";
const DIR: &str = "data/pds-ppi.igpp.ucla.edu";
const ROW_BYTES: usize = 112;

fn listing_files(html: &[u8]) -> Vec<String> {
    let text = String::from_utf8_lossy(html);
    let mut names: Vec<String> = Vec::new();
    let mut rest = text.as_ref();
    while let Some(pos) = rest.find("href=\"") {
        rest = &rest[pos + 6..];
        let Some(end) = rest.find('"') else { break };
        let href = &rest[..end];
        rest = &rest[end + 1..];
        if !href.ends_with(".TDF") || href.starts_with('?') || href.starts_with('/') {
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

fn manifest_family(
    prefix: &str,
    field_key: &str,
    component: &str,
    records: &[[f64; 14]],
    ci_mode: bool,
) {
    let budget = 8 + (odf::PODF_SHARD_BUDGET - 8) * 72 / ROW_BYTES;
    let ranges = odf::podf_shard_ranges(records.len(), budget);
    if ranges.len() == 1 {
        let out = format!("{DIR}/{prefix}.bin");
        write_and_verify(records, &out);
        if ci_mode && !upload_release(NETLOC, &out) {
            std::process::exit(1);
        }
        return;
    }

    let mut names: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();
    for (ord, &(lo, hi)) in ranges.iter().enumerate() {
        let t_lo = records[lo][0];
        let t_hi = records[hi - 1][0];
        let mut name = odf::podf_shard_name(prefix, t_lo, t_hi);
        if names.contains(&name) {
            name = odf::podf_shard_name_ord(prefix, t_lo, t_hi, ord);
        }
        names.push(name.clone());
        let path = format!("{DIR}/{name}");
        let bin = write_and_verify(&records[lo..hi], &path);
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
        println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
        println!("format {prefix}");
        println!("at earth");
        println!("ttl 604800");
        println!("field {field_key} {component} inverse-square em Hz 3600 0.0 0.0");
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
        eprintln!("{LISTING}: no .TDF files in listing — the series stays unwritten (0 honored)");
        return;
    }
    if list_mode {
        eprintln!("{LISTING}: {} .TDF files", files.len());
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
    let mut merged_x: Vec<[f64; 14]> = Vec::new();
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
        if let Some(res) = reduce_uly_skyfreq(name, fid as f64, &bytes, &lsk) {
            merged.extend(res.sband);
            merged_x.extend(res.xband);
        }
    }
    if merged.is_empty() && merged_x.is_empty() {
        eprintln!("no fsky samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    merged_x.sort_by(|a, b| a[0].total_cmp(&b[0]));
    std::fs::create_dir_all(DIR).ok();

    if !merged.is_empty() {
        manifest_family(
            PREFIX,
            "sky_frequency_hz",
            "ulysses_sky_frequency_hz",
            &merged,
            ci_mode,
        );
    }
    if !merged_x.is_empty() {
        manifest_family(
            PREFIX_X,
            "sky_frequency_x_hz",
            "ulysses_sky_frequency_x_hz",
            &merged_x,
            ci_mode,
        );
    }
}
