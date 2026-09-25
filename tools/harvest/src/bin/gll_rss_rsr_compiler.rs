use omegaflow::archivar::cassini_rsr;
use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;

const BASE: &str = "https://pds-rings.seti.org/pds4/bundles/gll.rss/gll.rss.raw/data_0159_sci/";
const NETLOC: &str = "pds-rings.seti.org";
const PREFIX: &str = "gll_rss_rsr";
const DIR: &str = "data/pds-rings.seti.org";
const SUFFIX: &str = "_rsr.dat";
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
    let files_limit: usize = args
        .windows(2)
        .find(|w| w[0] == "--files")
        .and_then(|w| w[1].parse().ok())
        .unwrap_or(usize::MAX);
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut files: Vec<String> = Vec::new();
    crawl(BASE, 0, &mut files);
    files.sort();
    files.dedup();
    if files.is_empty() {
        eprintln!("{BASE}: no RSR files in listing tree — the series stays unwritten (0 honored)");
        return;
    }
    if list_mode {
        eprintln!("{BASE}: {} RSR files", files.len());
        for name in &files {
            eprintln!("  {name}");
        }
        return;
    }
    let mut rows: Vec<(f64, f64, u32)> = Vec::new();
    let mut failures = 0usize;
    for url in files.iter().take(files_limit) {
        let name = url.rsplit('/').next().unwrap_or("rsr").to_string();
        let Some(bytes) = fetch_raw_bytes(url) else {
            eprintln!("{name}: fetch void ({url})");
            failures += 1;
            continue;
        };
        let Some(records) = cassini_rsr::parse_records(&bytes) else {
            eprintln!("{name}: record scan void — {} B", bytes.len());
            failures += 1;
            continue;
        };
        let before = rows.len();
        rows.extend(cassini_rsr::series(&records, &lsk));
        eprintln!(
            "{name}: {} records, {} series rows",
            records.len(),
            rows.len() - before
        );
    }
    if rows.is_empty() {
        if failures > 0 {
            eprintln!(
                "no gll.rss RSR samples — {failures} fetch/parse failures, the series stays unwritten"
            );
            std::process::exit(1);
        }
        eprintln!("no gll.rss RSR samples — the series stays unwritten (0 honored)");
        return;
    }
    if failures > 0 {
        eprintln!(
            "{failures} fetch/parse failures — the series stays unwritten (a partial harvest is not the measurement)"
        );
        std::process::exit(1);
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));

    std::fs::create_dir_all(DIR).ok();
    let per_shard = cassini_rsr::SHARD_BUDGET / 20;
    let chunks: Vec<&[(f64, f64, u32)]> = rows.chunks(per_shard).collect();
    let multi = chunks.len() > 1;
    let mut names: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();
    for (ord, chunk) in chunks.iter().enumerate() {
        let name = if multi {
            cassini_rsr::shard_name(PREFIX, ord)
        } else {
            format!("{PREFIX}.bin")
        };
        let bin = cassini_rsr::write_series(chunk);
        let path = format!("{DIR}/{name}");
        if std::fs::write(&path, &bin).is_err() {
            eprintln!("write {path} void");
            std::process::exit(1);
        }
        match cassini_rsr::parse_series(&bin) {
            Some(parsed) => eprintln!(
                "{name}: {} series rows (tdb {}..{}), {} B — roundtrip parses",
                parsed.len(),
                chunk[0].0,
                chunk[chunk.len() - 1].0,
                bin.len()
            ),
            None => {
                eprintln!("{name}: roundtrip parse void — the series stays unverified");
                std::process::exit(1);
            }
        }
        names.push(name);
        paths.push(path);
    }

    if multi {
        for name in &names {
            println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
            println!("format {PREFIX}");
            println!("origin procedure: {BASE} (Live-Listing data_0159_sci subdirectories)");
            println!("compiler tools/harvest/src/bin/gll_rss_rsr_compiler.rs");
            println!("at earth");
            println!("ttl 604800");
            println!("field i_count {PREFIX}_i_count inverse-square em count 604800 0.0 0.0");
            println!("field q_count {PREFIX}_q_count inverse-square em count 604800 0.0 0.0");
            println!();
        }
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
    use super::*;

    #[test]
    fn rsr_series_roundtrips() {
        let rows = vec![
            (1.5e9, -3.0, cassini_rsr::COMP_I),
            (1.5e9, 7.0, cassini_rsr::COMP_Q),
        ];
        let bytes = cassini_rsr::write_series(&rows);
        assert_eq!(cassini_rsr::parse_series(&bytes).unwrap(), rows);
        assert!(cassini_rsr::parse_series(b"X").is_none());
    }
}
