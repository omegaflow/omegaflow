use omegaflow::archivar::viking_text;
use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;

const RANGE_URL: &str = "https://ssd.jpl.nasa.gov/dat/planets/vikingrange.txt";
const DIFFERENCED_URL: &str = "https://ssd.jpl.nasa.gov/dat/planets/vikingdoppler.txt";
const NETLOC: &str = "ssd.jpl.nasa.gov";
const OUT: &str = "data/ssd.jpl.nasa.gov/viking_lander_tracking.bin";

fn main() {
    let ci_mode = std::env::args().any(|a| a == "--ci-mode");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut merged: Vec<(f64, f64, u32)> = Vec::new();
    for (label, url) in [("range", RANGE_URL), ("differenced", DIFFERENCED_URL)] {
        let Some(bytes) = fetch_raw_bytes(url) else {
            eprintln!("{label}: fetch void ({url})");
            continue;
        };
        let Some(rows) = viking_text::parse_text(&bytes) else {
            eprintln!("{label}: parse void — {} B", bytes.len());
            continue;
        };
        let mut kept = 0usize;
        let mut skipped = 0usize;
        for (unix, value, comp) in rows {
            let Some(tdb) = lsk.unix_to_tdb(unix) else {
                skipped += 1;
                continue;
            };
            merged.push((tdb, value, comp));
            kept += 1;
        }
        eprintln!("{label}: {kept} samples kept ({skipped} without TDB)");
    }
    if merged.is_empty() {
        eprintln!("no Viking tracking samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a.0.total_cmp(&b.0));
    std::fs::create_dir_all("data/ssd.jpl.nasa.gov").ok();
    let bin = viking_text::write_series(&merged);
    if std::fs::write(OUT, &bin).is_err() {
        eprintln!("write {OUT} void");
        return;
    }
    let verified = match viking_text::parse_series(&bin) {
        Some(parsed) => {
            eprintln!(
                "{OUT}: {} samples, tdb {:.0}..{:.0}, {:.0} B — roundtrip parses",
                parsed.len(),
                parsed[0].0,
                parsed[parsed.len() - 1].0,
                bin.len()
            );
            true
        }
        None => {
            eprintln!("{OUT}: roundtrip parse void — the series stays unverified");
            false
        }
    };
    if ci_mode {
        if !verified {
            std::process::exit(2);
        }
        if !upload_release(NETLOC, OUT) {
            std::process::exit(2);
        }
    }
}
