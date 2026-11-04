use std::fs;

use omegaflow::archivar::ae_ua::{parse, samples, write_bin};
use omegaflow::archivar::fetch::fetch_raw_bytes;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "spdf.gsfc.nasa.gov";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    omegaflow::archivar::cdn::require_ci();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let usage =
        "usage: ae_ua_compiler --url <day.dat> | --input <day.dat> --out <path> [--ci-mode]";
    let out_path = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            eprintln!("{usage}");
            std::process::exit(2);
        }
    };
    let text = match arg_value(&args, "--input") {
        Some(path) => match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("ae_ua_compiler: {path}: {e}");
                std::process::exit(2);
            }
        },
        None => match arg_value(&args, "--url") {
            Some(url) => match fetch_raw_bytes(&url) {
                Some(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                None => {
                    eprintln!("ae_ua_compiler: {url} gave no bytes");
                    std::process::exit(2);
                }
            },
            None => {
                eprintln!("{usage}");
                std::process::exit(2);
            }
        },
    };
    let day = match parse(&text) {
        Some(d) => d,
        None => {
            eprintln!("ae_ua_compiler: the day does not parse");
            std::process::exit(2);
        }
    };
    let rows = samples(&day);
    if rows.is_empty() {
        eprintln!(
            "ae_ua_compiler: day {} carries none of the three channels",
            day.year_day
        );
        std::process::exit(2);
    }
    let bytes = write_bin(&rows);
    if let Err(e) = fs::write(&out_path, &bytes) {
        eprintln!("ae_ua_compiler: write {out_path}: {e}");
        std::process::exit(2);
    }
    println!(
        "ae_ua_compiler: day {} gives {} rows to {out_path}",
        day.year_day,
        rows.len()
    );
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("ae_ua_compiler: upload of {out_path} refused — quota or network");
        std::process::exit(3);
    }
}
