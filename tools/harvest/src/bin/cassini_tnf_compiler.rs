use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const DATA: &str = "https://atmos.nmsu.edu/pdsd/archive/data/";
const NETLOC: &str = "atmos.nmsu.edu";
const VOLUME_CODES: [&str; 8] = ["sroc", "enoc", "hygr", "tocc", "tbis", "iagr", "rhgr", "gwe"];

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

fn volumes() -> Vec<String> {
    let Some(bytes) = fetch_raw_bytes(DATA, 604800) else {
        eprintln!("rss volume listing fetch void ({DATA})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("rss volume listing not utf8");
        return Vec::new();
    };
    let mut out = Vec::new();
    for name in listing_names(text) {
        let Some(stem) = name.strip_suffix('/') else {
            continue;
        };
        let Some(code) = stem.strip_prefix("co-s-rss-1-") else {
            continue;
        };
        if VOLUME_CODES.iter().any(|c| code.starts_with(c)) {
            out.push(name.clone());
        }
    }
    out.sort();
    out.dedup();
    out
}

fn crawl(url: &str, depth: u32, files: &mut Vec<String>) {
    if depth > 6 {
        return;
    }
    let Some(bytes) = fetch_raw_bytes(url, 604800) else {
        return;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        return;
    };
    for name in listing_names(text) {
        if name.starts_with('?') || name.starts_with('/') || name == "../" {
            continue;
        }
        if name.ends_with(".tnf") {
            files.push(format!("{url}{name}"));
            continue;
        }
        if name.ends_with('/') {
            let dir = name.trim_end_matches('/').to_ascii_lowercase();
            if matches!(dir.as_str(), "index" | "calib" | "catalog" | "document" | "errata") {
                continue;
            }
            crawl(&format!("{url}{name}"), depth + 1, files);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let vols = volumes();
    eprintln!("cassini rss tnf volumes: {}", vols.len());
    let mut files: Vec<String> = Vec::new();
    for vol in &vols {
        crawl(&format!("{DATA}{vol}"), 0, &mut files);
    }
    files.sort();
    files.dedup();
    eprintln!("cassini rss tnf files: {}", files.len());
    let mut merged: Vec<[f64; 9]> = Vec::new();
    for url in &files {
        let Some(bytes) = fetch_raw_bytes(url, 604800) else {
            eprintln!("{url}: fetch void");
            continue;
        };
        let Some(recs) = odf::tnf_rows(&bytes, &lsk) else {
            eprintln!("{url}: tnf scan void — {} B", bytes.len());
            continue;
        };
        merged.extend_from_slice(&recs);
    }
    if merged.is_empty() {
        eprintln!("no cassini TNF DT0 samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/atmos.nmsu.edu/cassini_tnf.bin";
    std::fs::create_dir_all("data/atmos.nmsu.edu").ok();
    let bin = odf::write_podf_bin(&merged);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match odf::parse_podf_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            eprintln!(
                "{out}: {} TNF DT0 samples (tdb {}..{}), {} B — roundtrip parses",
                parsed.len(),
                d0[0],
                d1[0],
                bin.len()
            );
        }
        None => eprintln!("{out}: roundtrip parse void — the series stays unverified"),
    }
    if ci_mode && !upload_release(NETLOC, out) {
        std::process::exit(1);
    }
}
