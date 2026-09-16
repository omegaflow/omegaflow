use omegaflow::archivar::{LeapSeconds, embedded_lsk, fetch_raw_bytes, http_code};
use omegaflow::cdn::upload_release;
use omegaflow::odf;
use std::sync::atomic::{AtomicUsize, Ordering};

const BASE: &str = "https://pds-geosciences.wustl.edu/ody/ody-m-rss-1-raw-v1/";
const UNIX_1950_OFFSET: f64 = 631152000.0;
const REQUEST_TTL_S: u64 = 1 << 9;
const WORKERS: usize = 1 << 3;
const PREFIX: &str = "odyssey_odf";

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

fn volumes() -> Vec<String> {
    let Some(bytes) = fetch_listing(BASE) else {
        eprintln!("volume listing fetch void ({BASE})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("volume listing not utf8");
        return Vec::new();
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
        .filter(|n| n.starts_with("odrs_"))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn files_of(vol: &str) -> Vec<String> {
    let dir = format!("{BASE}{vol}/odf/");
    let Some(bytes) = fetch_listing(&dir) else {
        eprintln!("{vol}: odf listing fetch void ({dir})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{vol}: odf listing not utf8");
        return Vec::new();
    };
    let mut out: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.to_ascii_lowercase().ends_with(".odf"))
        .map(|h| h.rsplit('/').next().unwrap_or("").to_string())
        .collect();
    out.sort();
    out.dedup();
    out
}

fn files_by_volume(vols: &[String]) -> Vec<(String, String)> {
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(WORKERS);
        for _ in 0..WORKERS {
            let next = &next;
            handles.push(s.spawn(move || {
                let mut found: Vec<(String, String)> = Vec::new();
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    let Some(vol) = vols.get(i) else {
                        break;
                    };
                    found.extend(files_of(vol).into_iter().map(|rel| (vol.clone(), rel)));
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
        all
    })
}

fn harvest(url: &str, lsk: &LeapSeconds) -> Vec<[f64; 9]> {
    let Some(bytes) = fetch_raw_bytes(url, REQUEST_TTL_S) else {
        eprintln!("{url}: fetch void");
        return Vec::new();
    };
    let Some(recs) = odf::parse_odf(&bytes) else {
        eprintln!("{url}: parse void — {} B", bytes.len());
        return Vec::new();
    };
    let mut rows: Vec<[f64; 9]> = Vec::new();
    let mut skipped = 0usize;
    for r in &recs {
        let doppler = (11..=14).contains(&r.data_type);
        if !r.valid || !doppler {
            skipped += 1;
            continue;
        }
        let unix = r.t_since_1950 - UNIX_1950_OFFSET;
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            skipped += 1;
            continue;
        };
        rows.push([
            tdb,
            r.observable_hz,
            r.ref_hz,
            r.dss_rx as f64,
            r.dss_tx as f64,
            r.data_type as f64,
            r.downlink_band as f64,
            r.scid as f64,
            r.compression_s,
        ]);
    }
    eprintln!(
        "{url}: {} orbit records, {} kept ({skipped} discarded)",
        recs.len(),
        rows.len()
    );
    rows
}

fn harvest_all(urls: &[String], lsk: &LeapSeconds) -> Vec<[f64; 9]> {
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(WORKERS);
        for _ in 0..WORKERS {
            let next = &next;
            handles.push(s.spawn(move || {
                let mut rows: Vec<[f64; 9]> = Vec::new();
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    let Some(url) = urls.get(i) else {
                        break;
                    };
                    rows.extend(harvest(url, lsk));
                }
                rows
            }));
        }
        let mut merged: Vec<[f64; 9]> = Vec::new();
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
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let vols = volumes();
    eprintln!("ody-m-rss-1-raw-v1: {} volumes", vols.len());
    let files = files_by_volume(&vols);
    let urls: Vec<String> = files
        .iter()
        .map(|(vol, rel)| format!("{BASE}{vol}/odf/{rel}"))
        .collect();
    eprintln!("ody-m-rss-1-raw-v1: {} odf files", urls.len());
    let mut merged = harvest_all(&urls, &lsk);
    if merged.is_empty() {
        eprintln!("no Mars Odyssey ODF orbit samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    std::fs::create_dir_all("data/pds-geosciences.wustl.edu").ok();
    let ranges = odf::podf_shard_ranges(merged.len(), odf::PODF_SHARD_BUDGET);
    if ranges.len() == 1 {
        let out = format!("data/pds-geosciences.wustl.edu/{PREFIX}.bin");
        let bin = odf::write_podf_bin(&merged);
        if std::fs::write(&out, &bin).is_err() {
            eprintln!("write {out} void");
            return;
        }
        match odf::parse_podf_bin(&bin) {
            Some(parsed) => {
                let d0 = parsed[0];
                let d1 = parsed[parsed.len() - 1];
                let mut stations: Vec<i64> = parsed.iter().map(|r| r[3] as i64).collect();
                stations.sort_unstable();
                stations.dedup();
                let mut dts: Vec<i64> = parsed.iter().map(|r| r[5] as i64).collect();
                dts.sort_unstable();
                dts.dedup();
                eprintln!(
                    "{out}: {} orbit samples (tdb {}..{}), stations {stations:?}, data_type {dts:?}, {} B — roundtrip parses",
                    parsed.len(),
                    d0[0],
                    d1[0],
                    bin.len()
                );
            }
            None => eprintln!("{out}: roundtrip parse void — the series stays unverified"),
        }
        if ci_mode && !upload_release("pds-geosciences.wustl.edu", &out) {
            std::process::exit(1);
        }
        return;
    }
    let mut names: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();
    for (ord, &(lo, hi)) in ranges.iter().enumerate() {
        let t_lo = merged[lo][0];
        let t_hi = merged[hi - 1][0];
        let mut name = odf::podf_shard_name(PREFIX, t_lo, t_hi);
        if names.contains(&name) {
            name = odf::podf_shard_name_ord(PREFIX, t_lo, t_hi, ord);
        }
        names.push(name.clone());
        let bin = odf::write_podf_bin(&merged[lo..hi]);
        assert!(bin.len() <= odf::PODF_SHARD_LIMIT);
        let parsed = match odf::parse_podf_bin(&bin) {
            Some(p) => p,
            None => {
                eprintln!("{name}: roundtrip parse void — the series stays unverified");
                std::process::exit(1);
            }
        };
        let d0 = parsed[0];
        let d1 = parsed[parsed.len() - 1];
        eprintln!(
            "{name}: {} orbit samples (tdb {}..{}), {} B — roundtrip parses",
            parsed.len(),
            d0[0],
            d1[0],
            bin.len()
        );
        let path = format!("data/pds-geosciences.wustl.edu/{name}");
        if std::fs::write(&path, &bin).is_err() {
            eprintln!("write {path} void");
            std::process::exit(1);
        }
        paths.push(path);
    }
    for name in &names {
        println!(
            "url https://github.com/omegaflow/sources/releases/download/pds-geosciences.wustl.edu/{name}"
        );
        println!("format {PREFIX}");
        println!("at earth");
        println!("ttl 604800");
        println!("field observable_hz {PREFIX}_observable_hz inverse-square em Hz 604800 0.0 0.0");
        println!();
    }
    if ci_mode {
        for path in &paths {
            if !upload_release("pds-geosciences.wustl.edu", path) {
                std::process::exit(1);
            }
        }
    }
}
