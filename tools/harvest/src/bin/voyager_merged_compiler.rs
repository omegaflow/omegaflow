use omegaflow::archivar::voyager_merged;
use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "spdf.gsfc.nasa.gov";
const BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/voyager";
const HOUSES: &[(&str, &str)] = &[
    (
        "voyager1_daily.asc",
        "data/spdf.gsfc.nasa.gov/voyager1_merged.bin",
    ),
    (
        "voyager2_daily.asc",
        "data/spdf.gsfc.nasa.gov/voyager2_merged.bin",
    ),
];

fn main() {
    let ci_mode = std::env::args().any(|a| a == "--ci-mode");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    std::fs::create_dir_all("data/spdf.gsfc.nasa.gov").ok();
    let mut all_ok = true;
    for (rel, out) in HOUSES {
        let sc = rel.split('_').next().unwrap_or("voyager");
        let url = format!("{BASE}/{sc}/merged/{rel}");
        let Some(bytes) = fetch_raw_bytes(&url) else {
            eprintln!("{sc}: fetch void ({url})");
            all_ok = false;
            continue;
        };
        let Some(rows) = voyager_merged::parse_text(&bytes) else {
            eprintln!("{sc}: parse void — {} B", bytes.len());
            all_ok = false;
            continue;
        };
        let mut merged: Vec<(f64, f64, u32)> = Vec::with_capacity(rows.len());
        let mut skipped = 0usize;
        for (unix, value, comp) in rows {
            let Some(tdb) = lsk.unix_to_tdb(unix) else {
                skipped += 1;
                continue;
            };
            merged.push((tdb, value, comp));
        }
        if merged.is_empty() {
            eprintln!("{sc}: no samples — the series stays unwritten (0 honored)");
            all_ok = false;
            continue;
        }
        merged.sort_by(|a, b| a.0.total_cmp(&b.0));
        let bin = voyager_merged::write_series(&merged);
        if std::fs::write(out, &bin).is_err() {
            eprintln!("{sc}: write {out} void");
            all_ok = false;
            continue;
        }
        match voyager_merged::parse_series(&bin) {
            Some(parsed) => eprintln!(
                "{sc}: {out}: {} samples, tdb {:.0}..{:.0}, {:.0} B — roundtrip parses ({skipped} without TDB)",
                parsed.len(),
                parsed[0].0,
                parsed[parsed.len() - 1].0,
                bin.len()
            ),
            None => {
                eprintln!("{sc}: roundtrip parse void — the series stays unverified");
                all_ok = false;
            }
        }
    }
    if ci_mode {
        if !all_ok {
            std::process::exit(2);
        }
        for (_, out) in HOUSES {
            if !upload_release(NETLOC, out) {
                std::process::exit(2);
            }
        }
    }
}
