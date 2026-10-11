use omegaflow::archivar::{LeapSeconds, embedded_lsk, fetch_raw_bytes, http_code};
use omegaflow::cdn::upload_release;
use omegaflow::odf;
use std::sync::atomic::{AtomicUsize, Ordering};

const BASE: &str = "https://archives.esac.esa.int/psa/ftp/MARS-EXPRESS/MRS/";
const ODF_DIR: &str = "DATA/LEVEL1A/CLOSED_LOOP/DSN/ODF/";
const UNIX_1950_OFFSET: f64 = 631152000.0;
const WORKERS: usize = 1 << 3;

fn fetch_listing(dir: &str) -> Option<Vec<u8>> {
    match http_code(dir, &[]) {
        Some(code) if (200..300).contains(&code) => fetch_raw_bytes(dir),
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
        .filter(|n| n.to_ascii_lowercase().starts_with("mex-m-mrs-"))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn files_of(bundle: &str) -> Vec<String> {
    let dir = format!("{BASE}{bundle}/{ODF_DIR}");
    let Some(bytes) = fetch_listing(&dir) else {
        eprintln!("{bundle}: odf listing fetch void ({dir})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{bundle}: odf listing not utf8");
        return Vec::new();
    };
    let mut out: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.to_ascii_lowercase().ends_with(".dat"))
        .map(|h| name_of(&h))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn files_by_bundle(bundles: &[String]) -> Vec<(String, String)> {
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(WORKERS);
        for _ in 0..WORKERS {
            let next = &next;
            handles.push(s.spawn(move || {
                let mut found: Vec<(String, String)> = Vec::new();
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    let Some(bundle) = bundles.get(i) else {
                        break;
                    };
                    found.extend(
                        files_of(bundle)
                            .into_iter()
                            .map(|rel| (bundle.clone(), rel)),
                    );
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

fn harvest(url: &str, lsk: &LeapSeconds) -> (Vec<[f64; 9]>, Vec<Option<i64>>) {
    let Some(bytes) = fetch_raw_bytes(url) else {
        eprintln!("{url}: fetch void");
        return (Vec::new(), Vec::new());
    };
    let Some(recs) = odf::parse_odf(&bytes) else {
        eprintln!("{url}: parse void — {} B", bytes.len());
        return (Vec::new(), Vec::new());
    };
    let mut rows: Vec<[f64; 9]> = Vec::new();
    let mut uplink_bands: Vec<Option<i64>> = Vec::new();
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
        uplink_bands.push(if r.uplink_band == 0 {
            None
        } else {
            Some(r.uplink_band)
        });
    }
    eprintln!(
        "{url}: {} orbit records, {} kept ({skipped} discarded)",
        recs.len(),
        rows.len()
    );
    (rows, uplink_bands)
}

fn harvest_all(urls: &[String], lsk: &LeapSeconds) -> (Vec<[f64; 9]>, Vec<Option<i64>>) {
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(WORKERS);
        for _ in 0..WORKERS {
            let next = &next;
            handles.push(s.spawn(move || {
                let mut rows: Vec<[f64; 9]> = Vec::new();
                let mut uplink_bands: Vec<Option<i64>> = Vec::new();
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    let Some(url) = urls.get(i) else {
                        break;
                    };
                    let (mut found, mut found_bands) = harvest(url, lsk);
                    rows.append(&mut found);
                    uplink_bands.append(&mut found_bands);
                }
                (rows, uplink_bands)
            }));
        }
        let mut merged: Vec<[f64; 9]> = Vec::new();
        let mut uplink_bands: Vec<Option<i64>> = Vec::new();
        for h in handles {
            match h.join() {
                Ok((mut rows, mut bands)) => {
                    merged.append(&mut rows);
                    uplink_bands.append(&mut bands);
                }
                Err(_) => eprintln!("a harvest worker stayed unjoined"),
            }
        }
        (merged, uplink_bands)
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let bundles = bundles();
    eprintln!("MARS-EXPRESS/MRS: {} bundles", bundles.len());
    let files = files_by_bundle(&bundles);
    let urls: Vec<String> = files
        .iter()
        .map(|(bundle, rel)| format!("{BASE}{bundle}/{ODF_DIR}{rel}"))
        .collect();
    eprintln!("MARS-EXPRESS/MRS: {} odf files", urls.len());
    let (merged, uplink_bands) = harvest_all(&urls, &lsk);
    if merged.is_empty() {
        eprintln!(
            "no Mars Express MaRS ODF orbit samples — the series stays unwritten (0 honored)"
        );
        return;
    }
    let mut paired: Vec<([f64; 9], Option<i64>)> = merged.into_iter().zip(uplink_bands).collect();
    paired.sort_by(|a, b| a.0[0].total_cmp(&b.0[0]));
    let (merged, uplink_bands): (Vec<[f64; 9]>, Vec<Option<i64>>) = paired.into_iter().unzip();
    let out = "data/archives.esac.esa.int/mars_express_odf.bin";
    std::fs::create_dir_all("data/archives.esac.esa.int").ok();
    let bin = odf::write_podf_bin_bands(&merged, &uplink_bands);
    if std::fs::write(out, &bin).is_err() {
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
    if ci_mode && !upload_release("archives.esac.esa.int", out) {
        std::process::exit(1);
    }
}
