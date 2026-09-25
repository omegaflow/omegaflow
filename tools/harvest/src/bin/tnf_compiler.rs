use std::collections::BTreeMap;

use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const NETLOC: &str = "pds-smallbodies.astro.umd.edu";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("--out <path> required");
            std::process::exit(1);
        }
    };
    let bytes = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => match arg_value(&args, "--url") {
            Some(url) => match fetch_raw_bytes(&url) {
                Some(b) => b,
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("--input <path.tnf> or --url <url> required");
                std::process::exit(1);
            }
        },
    };
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let rows = match odf::tnf_rows(&bytes, &lsk) {
        Some(r) => r,
        None => {
            eprintln!("TNF SFDU scan void — the series stays unwritten (0 honored)");
            std::process::exit(1);
        }
    };
    if rows.is_empty() {
        eprintln!("no TNF rows decoded — the series stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let mut codes: BTreeMap<i64, usize> = BTreeMap::new();
    for r in &rows {
        *codes.entry(r[odf::TNF_ROW_FORMAT] as i64).or_insert(0) += 1;
    }
    eprintln!("{} TNF rows by format code: {codes:?}", rows.len());
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let bin = odf::write_podf_bin(&rows);
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match odf::parse_podf_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            eprintln!(
                "{out}: {} TNF rows (tdb {}..{}), {} B — roundtrip parses",
                parsed.len(),
                d0[odf::TNF_ROW_TDB],
                d1[odf::TNF_ROW_TDB],
                bin.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
