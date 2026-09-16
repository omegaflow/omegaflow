use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const BASE: &str = "https://pds-ppi.igpp.ucla.edu/data/maven-rose-raw/data/tnf/";
const COLLECTION: &str = "https://pds-ppi.igpp.ucla.edu/data/maven-rose-raw/data/tnf/collection_maven_rose_raw_data_l0_tnf_1.33.csv";
const NETLOC: &str = "pds-ppi.igpp.ucla.edu";

fn products(text: &str) -> Vec<(String, String, String)> {
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
        let month = local[base + 4..base + 6].to_string();
        let mut name = String::with_capacity(local.len() + 8);
        name.push_str(&local[..base + 8]);
        name.push('T');
        name.push_str(&local[base + 9..]);
        name.push_str("_v01_r00.dat");
        out.push((name, year, month));
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
    let Some(csv) = fetch_raw_bytes(COLLECTION, 604800) else {
        eprintln!("maven tnf collection fetch void ({COLLECTION})");
        return;
    };
    let Ok(text) = std::str::from_utf8(&csv) else {
        eprintln!("maven tnf collection not utf8");
        return;
    };
    let prods = products(text);
    eprintln!("maven tnf collection: {} products", prods.len());
    let mut merged: Vec<[f64; 9]> = Vec::new();
    for (name, year, month) in &prods {
        let url = format!("{BASE}{year}/{month}/{name}");
        let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
            eprintln!("{name}: fetch void ({url})");
            continue;
        };
        let Some(recs) = odf::tnf_rows(&bytes, &lsk) else {
            eprintln!("{name}: tnf scan void — {} B", bytes.len());
            continue;
        };
        merged.extend_from_slice(&recs);
    }
    if merged.is_empty() {
        eprintln!("no maven TNF DT0 samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/pds-ppi.igpp.ucla.edu/maven_tnf.bin";
    std::fs::create_dir_all("data/pds-ppi.igpp.ucla.edu").ok();
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
