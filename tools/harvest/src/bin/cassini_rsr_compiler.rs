use std::collections::HashSet;

use omegaflow::archivar::cassini_rsr;
use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;

const DATA: &str = "https://atmos.nmsu.edu/pdsd/archive/data/";
const NETLOC: &str = "atmos.nmsu.edu";
const PREFIX: &str = "cassini_rsr";
const CRAWL_THREADS: usize = 16;
const CRAWL_DEPTH: u32 = 8;

fn hrefs(text: &str) -> Vec<String> {
    let low = text.to_ascii_lowercase();
    let mut out: Vec<String> = Vec::new();
    let mut from = 0usize;
    while let Some(p) = low[from..].find("href=\"") {
        let start = from + p + 6;
        let Some(end) = low[start..].find('"') else {
            break;
        };
        out.push(text[start..start + end].to_string());
        from = start + end;
    }
    out
}

fn volumes() -> Option<Vec<String>> {
    let Some(bytes) = fetch_raw_bytes(DATA) else {
        eprintln!("rss volume listing fetch void ({DATA})");
        return None;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("rss volume listing not utf8");
        return None;
    };
    let mut out: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.ends_with('/'))
        .map(|h| {
            h.trim_end_matches('/')
                .rsplit('/')
                .next()
                .unwrap_or("")
                .to_string()
        })
        .filter(|n| n.starts_with("co-ss-rss-1-"))
        .collect();
    out.sort();
    out.dedup();
    Some(out)
}

fn skip_dir(dir: &str) -> bool {
    matches!(
        dir,
        "index" | "calib" | "catalog" | "document" | "errata" | "tlm" | "158" | "odf"
    )
}

fn crawl_wave(urls: &[String]) -> (Vec<String>, Vec<String>, usize) {
    if urls.is_empty() {
        return (Vec::new(), Vec::new(), 0);
    }
    let threads = CRAWL_THREADS.min(urls.len());
    let chunk = urls.len().div_ceil(threads);
    let results: Vec<(Vec<String>, Vec<String>, usize)> = std::thread::scope(|s| {
        let handles: Vec<_> = urls
            .chunks(chunk)
            .map(|slice| {
                s.spawn(move || {
                    let mut dirs: Vec<String> = Vec::new();
                    let mut files: Vec<String> = Vec::new();
                    let mut failures = 0usize;
                    for url in slice {
                        let in_rsr = url.to_ascii_lowercase().ends_with("/rsr/");
                        let Some(bytes) = fetch_raw_bytes(url) else {
                            eprintln!("{url}: listing fetch void");
                            failures += 1;
                            continue;
                        };
                        let Ok(text) = std::str::from_utf8(&bytes) else {
                            eprintln!("{url}: listing not utf8");
                            failures += 1;
                            continue;
                        };
                        for name in hrefs(text) {
                            if name.starts_with('?')
                                || name.starts_with('/')
                                || name == "../"
                                || name.contains("://")
                            {
                                continue;
                            }
                            if name.ends_with('/') {
                                let dir = name.trim_end_matches('/').to_ascii_lowercase();
                                if !skip_dir(&dir) {
                                    dirs.push(format!("{url}{name}"));
                                }
                                continue;
                            }
                            if in_rsr && !name.to_ascii_lowercase().ends_with(".lbl") {
                                files.push(format!("{url}{name}"));
                            }
                        }
                    }
                    (dirs, files, failures)
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let mut failures = 0usize;
    for (d, f, x) in results {
        dirs.extend(d);
        files.extend(f);
        failures += x;
    }
    (dirs, files, failures)
}

fn crawl_all(vols: &[String]) -> (Vec<String>, usize) {
    let mut visited: HashSet<String> = HashSet::new();
    let mut frontier: Vec<String> = Vec::new();
    for vol in vols {
        let url = format!("{DATA}{vol}/");
        if visited.insert(url.clone()) {
            frontier.push(url);
        }
    }
    let mut files: Vec<String> = Vec::new();
    let mut failures = 0usize;
    for _ in 0..=CRAWL_DEPTH {
        if frontier.is_empty() {
            break;
        }
        let (dirs, mut found, fail) = crawl_wave(&frontier);
        files.append(&mut found);
        failures += fail;
        let mut next: Vec<String> = Vec::new();
        for dir in dirs {
            if visited.insert(dir.clone()) {
                next.push(dir);
            }
        }
        frontier = next;
    }
    files.sort();
    files.dedup();
    (files, failures)
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let files_limit: usize = arg_value(&args, "--files")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let explicit = arg_value(&args, "--file");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };

    let mut failures = 0usize;
    let files: Vec<String> = match explicit {
        Some(url) => vec![url],
        None => {
            let Some(vols) = volumes() else {
                eprintln!("rss volume listing unavailable — the series stays unwritten");
                std::process::exit(1);
            };
            eprintln!("cassini rss rsr volumes: {}", vols.len());
            let (mut files, fail) = crawl_all(&vols);
            failures = fail;
            files.truncate(files_limit);
            files
        }
    };
    eprintln!("cassini rss rsr files: {}", files.len());

    let mut rows: Vec<(f64, f64, u32)> = Vec::new();
    for url in &files {
        let Some(bytes) = fetch_raw_bytes(url) else {
            eprintln!("{url}: fetch void");
            failures += 1;
            continue;
        };
        let Some(records) = cassini_rsr::parse_records(&bytes) else {
            eprintln!("{url}: record scan void — {} B", bytes.len());
            failures += 1;
            continue;
        };
        let before = rows.len();
        rows.extend(cassini_rsr::series(&records, &lsk));
        eprintln!(
            "{url}: {} records, {} series rows",
            records.len(),
            rows.len() - before
        );
    }
    if rows.is_empty() {
        if failures > 0 {
            eprintln!(
                "no Cassini RSR samples — {failures} fetch/parse failures, the series stays unwritten"
            );
            std::process::exit(1);
        }
        eprintln!("no Cassini RSR samples — the series stays unwritten (0 honored)");
        return;
    }
    if failures > 0 {
        eprintln!(
            "{failures} listing/fetch/parse failures — the series stays unwritten (a partial harvest is not the measurement)"
        );
        std::process::exit(1);
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));

    std::fs::create_dir_all("data/atmos.nmsu.edu").ok();
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
        let path = format!("data/atmos.nmsu.edu/{name}");
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
            println!("origin procedure: {DATA} (Live-Listing co-ss-rss-1-* Volumes /rsr/)");
            println!("compiler tools/harvest/src/bin/cassini_rsr_compiler.rs");
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
