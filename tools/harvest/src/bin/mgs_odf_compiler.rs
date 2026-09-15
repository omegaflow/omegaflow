use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const BASE: &str = "https://pds-geosciences.wustl.edu/mgs/mgs-m-rss-1-moi-v1/";
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

fn volumes() -> Vec<String> {
    let Some(bytes) = fetch_raw_bytes(BASE, 604800) else {
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
        .filter(|n| n.starts_with("mors_"))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn files_of(vol: &str) -> Vec<String> {
    let dir = format!("{BASE}{vol}/odf/");
    let Some(bytes) = fetch_raw_bytes(&dir, 604800) else {
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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut merged: Vec<[f64; 9]> = Vec::new();
    let vols = volumes();
    eprintln!("mgs-m-rss-1-moi-v1: {} volumes", vols.len());
    for vol in vols {
        let mut kept_vol = 0usize;
        for rel in files_of(&vol) {
            let url = format!("{BASE}{vol}/odf/{rel}");
            let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
                eprintln!("{vol}/{rel}: fetch void ({url})");
                continue;
            };
            let Some(recs) = odf::parse_odf(&bytes) else {
                eprintln!("{vol}/{rel}: parse void — {} B", bytes.len());
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
            kept_vol += kept;
            eprintln!(
                "{vol}/{rel}: {} orbit records, {kept} kept ({skipped} discarded)",
                recs.len()
            );
        }
        eprintln!("{vol}: {kept_vol} samples merged");
    }
    if merged.is_empty() {
        eprintln!("no MGS ODF orbit samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/pds-geosciences.wustl.edu/mgs_odf.bin";
    std::fs::create_dir_all("data/pds-geosciences.wustl.edu").ok();
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
    if ci_mode && !upload_release("pds-geosciences.wustl.edu", out) {
        std::process::exit(1);
    }
}
