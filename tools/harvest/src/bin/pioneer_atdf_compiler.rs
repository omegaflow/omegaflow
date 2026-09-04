use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::atdf::{parse_bin, reduce_skyfreq, write_bin, S_BAND_REF_HI, S_BAND_REF_LO};
use omegaflow::cdn::upload_release;

const BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/pioneer/pioneer10/radio/Data";
const FILES: &[&str] = &[
    "ATDF_Data-Files_CMarkwardt_Readable/SC23-87361172500-88078042912.TDF",
    "ATDF_Data-Files_CMarkwardt_Readable/SC23-88063171500-88169011500.TDF",
    "ATDF_Data-Files_CMarkwardt_Readable/SC23-88168163000-88263042950.TDF",
    "ATDF_Data-Files_CMarkwardt_Readable/SC23-88334163000-88337043000.TDF",
    "ATDF_Data-Files_CMarkwardt_Readable/pioneer10.fl1",
    "ATDF_Data-Files_CMarkwardt_Readable/pioneer10.fl2",
];

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
    let mut merged: Vec<[f64; 14]> = Vec::new();
    for (fid, rel) in FILES.iter().enumerate() {
        let url = format!("{BASE}/{rel}");
        let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
            eprintln!("{rel}: fetch void ({url})");
            continue;
        };
        if let Some(samples) =
            reduce_skyfreq(rel, fid as f64, &bytes, &lsk, S_BAND_REF_LO, S_BAND_REF_HI)
        {
            merged.extend(samples);
        }
    }
    if merged.is_empty() {
        eprintln!("no fsky samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/pioneer10_skyfreq.bin";
    std::fs::create_dir_all("data").ok();
    let bin = write_bin(&merged);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match parse_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            let mut f: Vec<f64> = parsed.iter().map(|r| r[1]).collect();
            f.sort_by(f64::total_cmp);
            let fmed = f[f.len() / 2];
            eprintln!(
                "{out}: {} Samples ({}..{}), fsky {:.3e}..{:.3e} Hz (Median {:.6e} Hz), {} B — roundtrip parses",
                parsed.len(),
                jd_date(d0[0]),
                jd_date(d1[0]),
                d1[1],
                d0[1],
                fmed,
                bin.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
        }
    }
    if ci_mode && !upload_release("spdf.gsfc.nasa.gov", out) {
        std::process::exit(1);
    }
}
