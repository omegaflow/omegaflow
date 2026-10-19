use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::waldo_ab::{decode, to_csv};
use omegaflow::cdn::upload_release;
use std::io::Write;

const NETLOC: &str = "craam-files-bucket.s3.amazonaws.com";
const SAMPLE_URL: &str = "https://craam-files-bucket.s3.amazonaws.com/2015/01/01/narrowband/FE/FE150101000500NAA_008A.mat";
const ASSET: &str = "waldo_ab.csv";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "usage: waldo_ab_compiler [--input <file.mat>] [--url <file.mat>] [--out <waldo_ab.csv>] [--ci-mode]"
        );
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let url = match arg_value(&args, "--url") {
        Some(v) => v,
        None => SAMPLE_URL.to_string(),
    };
    let bytes = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("waldo_ab: read {path} returned void: {e} — the asset stays unwritten");
                std::process::exit(1);
            }
        },
        None => match fetch_raw_bytes(&url) {
            Some(b) => b,
            None => {
                eprintln!(
                    "waldo_ab: fetch {url} returned void — the harvest stays pending (0 honored)"
                );
                std::process::exit(1);
            }
        },
    };
    let series = match decode(&bytes) {
        Ok(s) => s,
        Err(note) => {
            eprintln!(
                "waldo_ab: {url} ({}) {note} — the asset stays unwritten (0 honored)",
                bytes.len()
            );
            std::process::exit(1);
        }
    };
    let csv = to_csv(&series);
    let rows = series.samples.len();
    if rows < omegaflow::archivar::waldo_ab::MIN_SAMPLES {
        eprintln!("waldo_ab: {rows} sample(s) survive — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/waldo_ab/{ASSET}"),
    };
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut file = match std::fs::File::create(&out_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("waldo_ab: create {out_path} returned void: {e}");
            std::process::exit(1);
        }
    };
    if file.write_all(csv.as_bytes()).is_err() || file.flush().is_err() {
        eprintln!("waldo_ab: write {out_path} returned void — the asset stays unwritten");
        std::process::exit(1);
    }
    let checksum = sha256_hex(csv.as_bytes());
    eprintln!(
        "waldo_ab: {rows} sample(s), order {}, call_sign {}, station {}, Fc {}, Fs {} — {out_path}, sha256 {checksum}",
        series.meta.order.label(),
        series.meta.call_sign,
        series.meta.station,
        match series.meta.frequency_hz {
            Some(v) => v.to_string(),
            None => String::from("absent"),
        },
        match series.meta.sample_rate_hz {
            Some(v) => v.to_string(),
            None => String::from("absent"),
        },
    );
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!(
            "waldo_ab: {out_path} did not reach the CDN — the local asset stands, the manifest is pending"
        );
        std::process::exit(1);
    }
}
