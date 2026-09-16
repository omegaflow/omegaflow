use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const BASE: &str = "https://archives.esac.esa.int/psa/ftp/INTERNATIONAL-ROSETTA-MISSION/RSI/";
const ODF_DIR: &str = "DATA/LEVEL1A/CLOSED_LOOP/IFMS/";
const ODF_SUBDIRS: &[&str] = &["AG1", "AG2", "DP1", "DP2"];
const UNIX_1950_OFFSET: f64 = 631152000.0;

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
    let Some(bytes) = fetch_raw_bytes(BASE, 604800) else {
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

fn files_of(bundle: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for sub in ODF_SUBDIRS {
        let dir = format!("{BASE}{bundle}/{ODF_DIR}{sub}/");
        let Some(bytes) = fetch_raw_bytes(&dir, 604800) else {
            eprintln!("{bundle}/{sub}: odf listing fetch void ({dir})");
            continue;
        };
        let Ok(text) = std::str::from_utf8(&bytes) else {
            eprintln!("{bundle}/{sub}: odf listing not utf8");
            continue;
        };
        let mut rels: Vec<String> = hrefs(text)
            .into_iter()
            .filter(|h| h.to_ascii_lowercase().ends_with(".raw"))
            .map(|h| format!("{sub}/{}", name_of(&h)))
            .collect();
        out.append(&mut rels);
    }
    out.sort();
    out.dedup();
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut merged: Vec<[f64; 9]> = Vec::new();
    let bundles = bundles();
    eprintln!(
        "INTERNATIONAL-ROSETTA-MISSION/RSI: {} bundles",
        bundles.len()
    );
    for bundle in bundles {
        let mut kept_bundle = 0usize;
        for rel in files_of(&bundle) {
            let url = format!("{BASE}{bundle}/{ODF_DIR}{rel}");
            let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
                eprintln!("{bundle}/{rel}: fetch void ({url})");
                continue;
            };
            let Some(recs) = odf::parse_odf(&bytes) else {
                eprintln!("{bundle}/{rel}: parse void — {} B", bytes.len());
                continue;
            };
            let mut kept = 0usize;
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
                merged.push([
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
                kept += 1;
            }
            kept_bundle += kept;
            eprintln!(
                "{bundle}/{rel}: {} orbit records, {kept} kept ({skipped} discarded)",
                recs.len()
            );
        }
        eprintln!("{bundle}: {kept_bundle} samples merged");
    }
    if merged.is_empty() {
        eprintln!("no Rosetta RSI ODF orbit samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/archives.esac.esa.int/rosetta_odf.bin";
    std::fs::create_dir_all("data/archives.esac.esa.int").ok();
    let bin = odf::write_podf_bin(&merged);
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
