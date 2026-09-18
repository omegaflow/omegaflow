use std::collections::BTreeMap;

use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/pioneer/pioneer10/radio/Turyshev20170327_Pioneer-10/DOPPLER";
const FILES: &[&str] = &["73288o74360_bj_sc23.odf", "86334o97343_sc23.odf"];
const NETLOC: &str = "spdf.gsfc.nasa.gov";
const UNIX_1950_OFFSET: f64 = 631152000.0;
const SCID_P10: i64 = 23;

fn year_of(tdb_s: f64) -> Option<u32> {
    let jd = 2451545.0 + tdb_s / 86400.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, _m, _d)) => Some(y),
        None => None,
    }
}

fn jd_date(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / 86400.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb_s:.0} s"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut merged: Vec<[f64; 9]> = Vec::new();
    for rel in FILES {
        let url = format!("{BASE}/{rel}");
        let Some(bytes) = fetch_raw_bytes(&url, 31536000) else {
            eprintln!("{rel}: fetch void ({url})");
            continue;
        };
        let Some(recs) = odf::parse_odf(&bytes) else {
            eprintln!("{rel}: parse void — {} B", bytes.len());
            continue;
        };
        let mut kept = 0usize;
        let mut skipped = 0usize;
        for r in &recs {
            if !r.valid || r.scid != SCID_P10 || !(11..=14).contains(&r.data_type) {
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
        eprintln!(
            "{rel}: {} orbit records, {kept} kept ({skipped} discarded)",
            recs.len()
        );
    }
    if merged.is_empty() {
        eprintln!("no P10-ODF samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/spdf.gsfc.nasa.gov/pioneer10_odf.bin";
    std::fs::create_dir_all("data/spdf.gsfc.nasa.gov").ok();
    let bin = odf::write_podf_bin(&merged);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    let verified = match odf::parse_podf_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            eprintln!(
                "{out}: {} Samples, {}..{} (tdb), {:.0} B — roundtrip parses",
                parsed.len(),
                jd_date(d0[0]),
                jd_date(d1[0]),
                bin.len()
            );
            true
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
            false
        }
    };
    if ci_mode {
        if !verified {
            std::process::exit(1);
        }
        if !upload_release(NETLOC, out) {
            std::process::exit(1);
        }
    }

    let mut per_year: BTreeMap<i64, usize> = BTreeMap::new();
    for r in &merged {
        if let Some(y) = year_of(r[0]) {
            if (1985..=2000u32).contains(&y) {
                *per_year.entry(y as i64).or_insert(0) += 1;
            }
        }
    }
    eprintln!("p10-odf per-year census (1985–2000): {per_year:?}");

    let mut per_mode: BTreeMap<i64, usize> = BTreeMap::new();
    for r in &merged {
        *per_mode.entry(r[5] as i64).or_insert(0) += 1;
    }
    eprintln!("p10-odf data_type census: {per_mode:?}");
}
