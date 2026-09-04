use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::atdf::{parse_resid_bin, reduce_resid, write_resid_bin};
use omegaflow::cdn::upload_release;

const BASE: &str = "https://pds-ppi.igpp.ucla.edu/annex/";
const VOLUMES: &[&str] = &[
    "GO-J-RSS-1-TDF-V1.0",
    "GO-JS-RSS-1-TDF-V1.0",
    "GO-JG-RSS-1-TDF-V1.0",
    "GO-SS-RSS-1-TDF-V1.0",
    "GO-SUN-RSS-1-TDF-V1.0",
];

fn files_of(volume: &str, ext: &str) -> Vec<String> {
    let url = format!("{BASE}{volume}/INDEX/INDEX.TAB");
    let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
        eprintln!("{volume}: index fetch void ({url})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{volume}: index not utf8");
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in text.lines() {
        let Some(lbl) = line
            .split(',')
            .find(|f| f.trim_matches('"').ends_with(".LBL"))
        else {
            continue;
        };
        let lbl = lbl.trim_matches('"');
        if let Some(stem) = lbl.strip_suffix(".LBL") {
            out.push(format!("{stem}.{ext}"));
        }
    }
    out
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
    let limit: Option<usize> = args
        .iter()
        .position(|a| a == "--limit")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok());
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut merged: Vec<[f64; 8]> = Vec::new();
    let mut fetched = 0usize;
    'outer: for volume in VOLUMES {
        for rel in files_of(volume, "TDF") {
            if let Some(l) = limit {
                if fetched >= l {
                    break 'outer;
                }
            }
            fetched += 1;
            let url = format!("{BASE}{volume}/{rel}");
            let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
                eprintln!("{rel}: fetch void ({url})");
                continue;
            };
            if let Some(samples) = reduce_resid(&rel, &bytes, &lsk) {
                merged.extend(samples);
            }
        }
    }
    if merged.is_empty() {
        eprintln!("no galileo resid samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/galileo_resid.bin";
    std::fs::create_dir_all("data").ok();
    let bin = write_resid_bin(&merged);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match parse_resid_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            let rmin = parsed.iter().map(|r| r[1]).fold(f64::INFINITY, f64::min);
            let rmax = parsed
                .iter()
                .map(|r| r[1])
                .fold(f64::NEG_INFINITY, f64::max);
            let mut stations: Vec<i64> = parsed.iter().map(|r| r[2] as i64).collect();
            stations.sort_unstable();
            stations.dedup();
            let mut modes: Vec<i64> = parsed.iter().map(|r| r[3] as i64).collect();
            modes.sort_unstable();
            modes.dedup();
            eprintln!(
                "{out}: {} resid samples ({}..{}), resid {rmin:.3}..{rmax:.3} Hz, stations {stations:?}, modes {modes:?}, {} B — roundtrip parses",
                parsed.len(),
                jd_date(d0[0]),
                jd_date(d1[0]),
                bin.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
        }
    }
    if ci_mode && !upload_release("pds-ppi.igpp.ucla.edu", out) {
        std::process::exit(1);
    }
}
