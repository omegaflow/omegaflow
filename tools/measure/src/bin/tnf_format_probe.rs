use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::odf;
use std::collections::BTreeMap;

fn urls(args: &[String]) -> Vec<String> {
    args.windows(2)
        .filter(|w| w[0] == "--url")
        .map(|w| w[1].clone())
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let targets = urls(&args);
    if targets.is_empty() {
        eprintln!("--url <url> required (repeatable)");
        std::process::exit(2);
    }
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the probe stays unwritten");
        std::process::exit(2);
    };
    for url in &targets {
        let Some(bytes) = fetch_raw_bytes(url) else {
            eprintln!("{url}: fetch void");
            continue;
        };
        let Some(frames) = odf::scan_tnf_sfdus(&bytes) else {
            eprintln!("{url}: SFDU scan void ({} B)", bytes.len());
            continue;
        };
        let mut frame_hist: BTreeMap<u8, usize> = BTreeMap::new();
        for f in &frames {
            *frame_hist.entry(f.format_code).or_insert(0) += 1;
        }
        let rows = match odf::tnf_rows(&bytes, &lsk) {
            Some(r) => r,
            None => {
                eprintln!("{url}: tnf_rows void");
                continue;
            }
        };
        let mut row_hist: BTreeMap<i64, usize> = BTreeMap::new();
        for r in &rows {
            *row_hist.entry(r[odf::TNF_ROW_FORMAT] as i64).or_insert(0) += 1;
        }
        eprintln!(
            "{url}: {} B, {} frames, frame codes {frame_hist:?}, {} rows, row codes {row_hist:?}",
            bytes.len(),
            frames.len(),
            rows.len()
        );
    }
}
