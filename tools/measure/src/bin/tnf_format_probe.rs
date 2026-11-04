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
        if args.iter().any(|a| a == "--ranging") {
            let mut ranging: BTreeMap<u8, usize> = BTreeMap::new();
            let mut printed = 0usize;
            for f in &frames {
                let end = f.offset + f.total_len;
                if end > bytes.len() {
                    continue;
                }
                let slice = &bytes[f.offset..end];
                let (first, last, cycle) = match f.format_code {
                    odf::TNF_FORMAT_UL_SEQ_RANGING_PHASE => {
                        let Some(d) = odf::tnf_dt2(f, slice) else {
                            continue;
                        };
                        (d.first_comp_num, d.last_comp_num, d.rng_cycle_time)
                    }
                    odf::TNF_FORMAT_DL_SEQ_RANGING_PHASE => {
                        let Some(d) = odf::tnf_dt3(f, slice) else {
                            continue;
                        };
                        (d.first_comp_num, d.last_comp_num, d.rng_cycle_time)
                    }
                    _ => continue,
                };
                *ranging.entry(f.format_code).or_insert(0) += 1;
                if printed < 5 {
                    let frq_up = odf::tnf_pair_ul_freq(&frames, &bytes, f);
                    let res = match frq_up {
                        Some(v) => odf::tnf_ranging_resolution_with_frq_up(f, slice, v),
                        None => odf::tnf_ranging_resolution(f, slice),
                    };
                    eprintln!(
                        "{url}: ranging fmt {} first {first} last {last} cycle {cycle:.6}s -> {res:?} m",
                        f.format_code
                    );
                    printed += 1;
                }
            }
            eprintln!("{url}: ranging frames {ranging:?}");
        }
        eprintln!(
            "{url}: {} B, {} frames, frame codes {frame_hist:?}, {} rows, row codes {row_hist:?}",
            bytes.len(),
            frames.len(),
            rows.len()
        );
    }
}
