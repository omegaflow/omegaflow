use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const BASE: &str = "https://pdssbn.astro.umd.edu/holdings/pds4-dart:data_trk234-v1.0/";
const COLLECTION: &str =
    "https://pdssbn.astro.umd.edu/holdings/pds4-dart:data_trk234-v1.0/collection_data_trk234.csv";
const NETLOC: &str = "pdssbn.astro.umd.edu";

fn products(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let Some(lid) = line.strip_prefix("P,") else {
            continue;
        };
        let Some(main) = lid.split("::").next() else {
            continue;
        };
        let Some(local) = main.rsplit(':').next() else {
            continue;
        };
        let Some(idx) = local.find("_tnf_") else {
            continue;
        };
        let base = idx + "_tnf_".len();
        if local.len() < base + 9 || local.as_bytes()[base + 8] != b't' {
            continue;
        }
        let year = local[base..base + 4].to_string();
        let mut name = String::with_capacity(local.len() + 4);
        name.push_str(&local[..base + 8]);
        name.push('T');
        name.push_str(&local[base + 9..]);
        name.push_str(".dat");
        out.push((name, year));
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
    let Some(csv) = fetch_raw_bytes(COLLECTION) else {
        eprintln!("dart tnf collection fetch void ({COLLECTION})");
        return;
    };
    let Ok(text) = std::str::from_utf8(&csv) else {
        eprintln!("dart tnf collection not utf8");
        return;
    };
    let prods = products(text);
    eprintln!("dart tnf collection: {} products", prods.len());
    let mut merged: Vec<[f64; 9]> = Vec::new();
    for (name, year) in &prods {
        let url = format!("{BASE}{year}/{name}");
        let Some(bytes) = fetch_raw_bytes(&url) else {
            eprintln!("{name}: fetch void ({url})");
            continue;
        };
        let Some(recs) = odf::tnf_rows(&bytes, &lsk) else {
            eprintln!("{name}: tnf scan void — {} B", bytes.len());
            continue;
        };
        merged.extend(recs.into_iter().filter(|r| r[odf::TNF_ROW_FORMAT] == 0.0));
    }
    if merged.is_empty() {
        eprintln!("no dart TNF DT0 samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/pdssbn.astro.umd.edu/dart_tnf.bin";
    std::fs::create_dir_all("data/pdssbn.astro.umd.edu").ok();
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
