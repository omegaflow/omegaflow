use std::collections::BTreeMap;

use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const BASE: &str =
    "https://pds-rings.seti.org/pds4/bundles/gll.rss/gll.rss.raw/data_trk234_trknav/";
const NETLOC: &str = "pds-rings.seti.org";
const PREFIX: &str = "gll_rss_tnf";
const DIR: &str = "data/pds-rings.seti.org";
const SUFFIX: &str = "_tnf.dat";
const MAX_DEPTH: u32 = 3;

fn listing_names(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for token in text.split("href=\"") {
        let Some(end) = token.find('"') else {
            continue;
        };
        out.push(token[..end].to_string());
    }
    out
}

fn crawl(url: &str, depth: u32, files: &mut Vec<String>) {
    if depth > MAX_DEPTH {
        return;
    }
    let Some(bytes) = fetch_raw_bytes(url) else {
        eprintln!("{url}: listing fetch void");
        return;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{url}: listing not utf8");
        return;
    };
    for name in listing_names(text) {
        if name.starts_with('?') || name.starts_with('/') || name == "../" {
            continue;
        }
        if name.ends_with(SUFFIX) {
            files.push(format!("{url}{name}"));
            continue;
        }
        if name.ends_with('/') {
            crawl(&format!("{url}{name}"), depth + 1, files);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let list_mode = args.iter().any(|a| a == "--list");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut files: Vec<String> = Vec::new();
    crawl(BASE, 0, &mut files);
    files.sort();
    files.dedup();
    if files.is_empty() {
        eprintln!("{BASE}: no TNF files in listing tree — the series stays unwritten (0 honored)");
        return;
    }
    if list_mode {
        eprintln!("{BASE}: {} TNF files", files.len());
        for name in &files {
            eprintln!("  {name}");
        }
        return;
    }
    let mut merged: Vec<[f64; 9]> = Vec::new();
    for url in &files {
        let name = url.rsplit('/').next().unwrap_or("tnf").to_string();
        let Some(bytes) = fetch_raw_bytes(url) else {
            eprintln!("{name}: fetch void ({url})");
            continue;
        };
        let Some(recs) = odf::tnf_rows(&bytes, &lsk) else {
            eprintln!("{name}: tnf scan void — {} B", bytes.len());
            continue;
        };
        let mut codes: BTreeMap<i64, usize> = BTreeMap::new();
        for r in &recs {
            *codes.entry(r[odf::TNF_ROW_FORMAT] as i64).or_insert(0) += 1;
        }
        eprintln!("{name}: {} TNF rows by format code: {codes:?}", recs.len());
        merged.extend(recs.into_iter().filter(|r| r[odf::TNF_ROW_FORMAT] == 0.0));
    }
    if merged.is_empty() {
        eprintln!("no gll.rss TNF DT0 samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    std::fs::create_dir_all(DIR).ok();
    let out = format!("{DIR}/{PREFIX}.bin");
    let bin = odf::write_podf_bin(&merged);
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} void");
        std::process::exit(1);
    }
    match odf::parse_podf_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            eprintln!(
                "{out}: {} TNF DT0 samples (tdb {}..{}), {} B — roundtrip parses",
                parsed.len(),
                d0[odf::TNF_ROW_TDB],
                d1[odf::TNF_ROW_TDB],
                bin.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
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

    #[test]
    fn podf_bin_roundtrips() {
        let rows = vec![[1.0e9, 1234.5, 2.293e9, 43.0, 77.0, 0.0, 1.0, 1.0, 0.0]];
        let bin = odf::write_podf_bin(&rows);
        assert!(odf::parse_podf_bin(&bin).is_some());
        assert!(odf::parse_podf_bin(b"X").is_none());
    }
}
