use omegaflow::archivar::geo::{GeoRec, parse_bin, verify_bin, write_bin};
use omegaflow::archivar::iaga::{parse_text, to_geo_rows};
use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::archivar::quaoar_occlt::{zip_entries, zip_extract};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::LeapSeconds;
use std::process::Command;

const NETLOC: &str = "zenodo.org";
const ZIP_URL: &str = "https://zenodo.org/api/records/10594301/files/Mag_Data.zip/content";
const MAGIC_IAGA: [u8; 4] = *b"IGA1";
const DEFAULT_OUT: &str = "data/zenodo.org/iaga_text.bin";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("900")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let lsk: LeapSeconds = match embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!("embedded naif0012 parses void — the epoch conversion stays unread");
            std::process::exit(1);
        }
    };
    let zip = match fetch(ZIP_URL) {
        Some(b) => b,
        None => {
            eprintln!("{ZIP_URL}: fetch void");
            std::process::exit(1);
        }
    };
    let entries = match zip_entries(&zip) {
        Some(e) => e,
        None => {
            eprintln!("{ZIP_URL}: central directory void — the zip stays unread");
            std::process::exit(1);
        }
    };
    let mut records: Vec<GeoRec> = Vec::new();
    let mut members = 0usize;
    let mut parsed = 0usize;
    for e in &entries {
        if !e.name.to_ascii_lowercase().ends_with(".sec") {
            continue;
        }
        members += 1;
        let Some(raw) = zip_extract(&zip, e) else {
            eprintln!("{}: entry extract void", e.name);
            continue;
        };
        let text = String::from_utf8_lossy(&raw);
        let Some(file) = parse_text(&text, &lsk) else {
            eprintln!(
                "{}: IAGA-2002 text parses void — the station stays unharvested",
                e.name
            );
            continue;
        };
        let rows = to_geo_rows(&file);
        if rows.is_empty() {
            eprintln!("{}: no present component — 0 honored", e.name);
            continue;
        }
        parsed += 1;
        records.extend(rows);
    }
    if records.is_empty() {
        eprintln!(
            "{ZIP_URL}: {members} .sec members carry no measured component — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
    let bin = write_bin(MAGIC_IAGA, &records);
    if let Some(parent) = std::path::Path::new(&out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).ok();
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_IAGA, &bin) {
        Some(roundtrip) if roundtrip.len() == records.len() => {
            eprintln!(
                "{out}: {} records ({} of {} .sec members parsed, {} B, {ZIP_URL}), roundtrip parses",
                records.len(),
                parsed,
                members,
                bin.len()
            );
        }
        _ => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    match verify_bin(MAGIC_IAGA, &bin) {
        Some(n) => eprintln!("{out}: verify {} records", n),
        None => {
            eprintln!("{out}: verify void — the asset stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
