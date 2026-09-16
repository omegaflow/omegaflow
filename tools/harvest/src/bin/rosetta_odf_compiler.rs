use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::http_code;
use omegaflow::archivar::ifms_agc::{parse_ifms_agc, parse_series, write_series};
use omegaflow::cdn::upload_release;
use std::sync::atomic::{AtomicUsize, Ordering};

const BASE: &str = "https://archives.esac.esa.int/psa/ftp/INTERNATIONAL-ROSETTA-MISSION/RSI/";
const ODF_DIR: &str = "DATA/LEVEL1A/CLOSED_LOOP/IFMS/";
const ODF_SUBDIRS: &[&str] = &["AG1", "AG2", "DP1", "DP2"];
const REQUEST_TTL_S: u64 = 1 << 9;
const WORKERS: usize = 1 << 3;

fn fetch_listing(dir: &str) -> Option<Vec<u8>> {
    match http_code(dir, &[]) {
        Some(code) if (200..300).contains(&code) => fetch_raw_bytes(dir, REQUEST_TTL_S),
        Some(code) => {
            eprintln!("{dir}: http {code} — no listing");
            None
        }
        None => {
            eprintln!("{dir}: unreachable — no listing");
            None
        }
    }
}

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

fn name_of(href: &str) -> String {
    href.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_string()
}

fn bundles() -> Vec<String> {
    let Some(bytes) = fetch_listing(BASE) else {
        eprintln!("bundle listing fetch void ({BASE})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("bundle listing not utf8");
        return Vec::new();
    };
    let mut out: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.ends_with('/'))
        .map(|h| name_of(&h))
        .filter(|n| {
            let l = n.to_ascii_lowercase();
            l.starts_with("ro-") && l.contains("-rsi-")
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

fn files_of(bundle: &str, sub: &str) -> Vec<String> {
    let dir = format!("{BASE}{bundle}/{ODF_DIR}{sub}/");
    let Some(bytes) = fetch_listing(&dir) else {
        eprintln!("{bundle}/{sub}: odf listing fetch void ({dir})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{bundle}/{sub}: odf listing not utf8");
        return Vec::new();
    };
    let mut rels: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.to_ascii_lowercase().ends_with(".raw"))
        .map(|h| format!("{sub}/{}", name_of(&h)))
        .collect();
    rels.sort();
    rels.dedup();
    rels
}

fn files_by_bundle(bundles: &[String]) -> Vec<(String, String)> {
    let mut work: Vec<(usize, &str)> = Vec::new();
    for (bi, _) in bundles.iter().enumerate() {
        for sub in ODF_SUBDIRS {
            work.push((bi, sub));
        }
    }
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(WORKERS);
        for _ in 0..WORKERS {
            let next = &next;
            let work = &work;
            handles.push(s.spawn(move || {
                let mut found: Vec<(String, String)> = Vec::new();
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    let Some(&(bi, sub)) = work.get(i) else {
                        break;
                    };
                    let bundle = &bundles[bi];
                    found.extend(files_of(bundle, sub).into_iter().map(|rel| (bundle.clone(), rel)));
                }
                found
            }));
        }
        let mut all: Vec<(String, String)> = Vec::new();
        for h in handles {
            match h.join() {
                Ok(mut found) => all.append(&mut found),
                Err(_) => eprintln!("a listing worker stayed unjoined"),
            }
        }
        all.sort();
        all.dedup();
        all
    })
}

fn harvest(url: &str) -> Vec<(f64, f64, f64)> {
    let Some(bytes) = fetch_raw_bytes(url, REQUEST_TTL_S) else {
        eprintln!("{url}: fetch void");
        return Vec::new();
    };
    let Some(file) = parse_ifms_agc(&bytes) else {
        eprintln!("{url}: parse void — {} B", bytes.len());
        return Vec::new();
    };
    eprintln!(
        "{url}: {} AGC samples ({} header fields)",
        file.samples.len(),
        file.fields.len()
    );
    file.samples
        .iter()
        .map(|s| (s.unix_time, s.carrier_level_dbm, s.polar_angle_cycles))
        .collect()
}

fn harvest_all(urls: &[String]) -> Vec<(f64, f64, f64)> {
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(WORKERS);
        for _ in 0..WORKERS {
            let next = &next;
            handles.push(s.spawn(move || {
                let mut rows: Vec<(f64, f64, f64)> = Vec::new();
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    let Some(url) = urls.get(i) else {
                        break;
                    };
                    rows.extend(harvest(url));
                }
                rows
            }));
        }
        let mut merged: Vec<(f64, f64, f64)> = Vec::new();
        for h in handles {
            match h.join() {
                Ok(mut rows) => merged.append(&mut rows),
                Err(_) => eprintln!("a harvest worker stayed unjoined"),
            }
        }
        merged
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let bundles = bundles();
    eprintln!(
        "INTERNATIONAL-ROSETTA-MISSION/RSI: {} bundles",
        bundles.len()
    );
    let files = files_by_bundle(&bundles);
    let urls: Vec<String> = files
        .iter()
        .map(|(bundle, rel)| format!("{BASE}{bundle}/{ODF_DIR}{rel}"))
        .collect();
    eprintln!("INTERNATIONAL-ROSETTA-MISSION/RSI: {} files", urls.len());
    let mut merged = harvest_all(&urls);
    if merged.is_empty() {
        eprintln!("no Rosetta RSI IFMS AGC samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a.0.total_cmp(&b.0));
    let out = "data/archives.esac.esa.int/rosetta_odf.bin";
    std::fs::create_dir_all("data/archives.esac.esa.int").ok();
    let bin = write_series(&merged);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    let (first, last) = match parse_series(&bin) {
        Some(parsed) => match (parsed.first(), parsed.last()) {
            (Some(a), Some(b)) => (a.0, b.0),
            _ => {
                eprintln!("{out}: roundtrip parse empty — the series stays unverified");
                return;
            }
        },
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
            return;
        }
    };
    eprintln!(
        "{out}: {} AGC samples (unix {first}..{last}), {} B — roundtrip parses",
        merged.len(),
        bin.len()
    );
    if ci_mode && !upload_release("archives.esac.esa.int", out) {
        std::process::exit(1);
    }
}
