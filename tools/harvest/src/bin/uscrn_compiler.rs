use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::geo::{MAGIC_USCRN, parse_bin, write_bin};
use omegaflow::archivar::noaa_nodd::{
    filter_window, parse_uscrn, parse_uscrn_stations, tdb_window, uscrn_wban,
};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::parse as parse_lsk;

const NETLOC: &str = "www.ncei.noaa.gov";
const BUCKET: &str = "https://www.ncei.noaa.gov/pub/data/uscrn/products";
const STATIONS: &str = "https://www.ncei.noaa.gov/pub/data/uscrn/products/stations.tsv";

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
            eprintln!("--station (hourly02 file stem, e.g. AK_Aleknagik_1_NNE) required");
            std::process::exit(1);
        }
    };
    let year: i64 = match arg_value(&args, "--year").and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => {
            eprintln!("--year (e.g. 2026) required");
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

    let stations_text = match fetch_raw(STATIONS, None, &[], 3600) {
        Some(t) => t,
        None => {
            eprintln!("stations.tsv fetch void");
            std::process::exit(1);
        }
    };
    let anchors = parse_uscrn_stations(&stations_text);
    let url = format!("{BUCKET}/hourly02/{year}/CRNH0203-{year}-{station}.txt");
    let text = match fetch_raw(&url, None, &[], 3600) {
        Some(t) => t,
        None => {
            eprintln!("{url}: hourly02 fetch void");
            std::process::exit(1);
        }
    };
    let wban = match uscrn_wban(&text) {
        Some(w) => w,
        None => {
            eprintln!("{station} {year}: no WBAN in the first row — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    let anchor = match anchors.get(&wban) {
        Some(a) => a,
        None => {
            eprintln!(
                "{station}: WBAN {wban} carries no anchor in stations.tsv — the bin stays unwritten"
            );
            std::process::exit(1);
        }
    };
    let records = parse_uscrn(&text, anchor.2, &lsk);
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
    let bytes = write_bin(MAGIC_USCRN, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_USCRN, &bytes) {
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
