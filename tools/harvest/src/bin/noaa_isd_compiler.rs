use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::geo::{parse_bin, write_bin, MAGIC_ISD};
use omegaflow::archivar::noaa_nodd::{filter_window, parse_isd, tdb_window};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::parse as parse_lsk;

const NETLOC: &str = "noaa-global-hourly-pds.s3.amazonaws.com";
const BUCKET: &str = "https://noaa-global-hourly-pds.s3.amazonaws.com";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = match arg_value(&args, "--station") {
        Some(v) => v,
        None => {
            eprintln!("--station (WMO id, e.g. 01001099999) required");
            std::process::exit(1);
        }
    };
    let year: i64 = match arg_value(&args, "--year").and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => {
            eprintln!("--year (e.g. 2025) required");
            std::process::exit(1);
        }
    };
    let month: Option<i64> = arg_value(&args, "--month").and_then(|v| v.parse().ok());
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("--out <path> required");
            std::process::exit(1);
        }
    };
    let lsk = match arg_value(&args, "--lsk")
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|t| parse_lsk(&t))
    {
        Some(l) => l,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void");
            std::process::exit(1);
        }
    };

    let text = match fetch_raw(&format!("{BUCKET}/{year}/{station}.csv"), None, &[], 3600) {
        Some(t) => t,
        None => {
            eprintln!("{station} {year}: csv fetch void");
            std::process::exit(1);
        }
    };
    let records = parse_isd(&text, &lsk);
    let (start, end) = match tdb_window(year, month, &lsk) {
        Some(w) => w,
        None => {
            eprintln!("{station}: window reads void — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    let records = filter_window(records, start, end);
    if records.is_empty() {
        eprintln!(
            "{station}: no measured records in the window — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let bytes = write_bin(MAGIC_ISD, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_ISD, &bytes) {
        Some(parsed) => eprintln!(
            "{station} {year}: {} geo records written, roundtrip parses",
            parsed.len()
        ),
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
