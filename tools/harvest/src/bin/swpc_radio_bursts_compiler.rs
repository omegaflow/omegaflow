use omegaflow::archivar::radio_burst::{parse_bin, parse_text, write_bin};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::process::Command;

const DEFAULT_URL: &str = "https://services.swpc.noaa.gov/json/edited_events.json";
const DEFAULT_OUT: &str = "swpc_radio_bursts.bin";
const CDN_TAG: &str = "services.swpc.noaa.gov";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch_measured(url: &str) -> Option<(u16, Vec<u8>)> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("120")
        .arg("-w")
        .arg("\n%{http_code}")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    let split = out.stdout.iter().rposition(|&b| b == b'\n')?;
    let status = std::str::from_utf8(&out.stdout[split + 1..])
        .ok()?
        .trim()
        .parse::<u16>()
        .ok()?;
    let mut body = out.stdout;
    body.truncate(split);
    Some((status, body))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let url = match arg_value(&args, "--url") {
        Some(u) => u,
        None => DEFAULT_URL.to_string(),
    };
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => DEFAULT_OUT.to_string(),
    };

    let Some((status, body)) = fetch_measured(&url) else {
        eprintln!("{url} fetch void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    eprintln!("{url}: HTTP {status}, {} B", body.len());
    if !(200..300).contains(&status) {
        eprintln!("{url}: HTTP {status} is not a 2xx body — the bin stays unwritten");
        std::process::exit(1);
    }
    let Some(text) = std::str::from_utf8(&body).ok() else {
        eprintln!("{url}: body is not UTF-8 — the bin stays unwritten");
        std::process::exit(1);
    };
    let mut records = parse_text(text);
    records.sort_by(|a, b| {
        a.t_unix
            .partial_cmp(&b.t_unix)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    if records.is_empty() {
        eprintln!(
            "{url}: no RSP (type II/III sweep) records in the window — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let first = records[0].t_unix;
    let last = records[records.len() - 1].t_unix;
    eprintln!(
        "SWPC radio sweeps: {} records, unix {:.0}..{:.0} ({} sweeps with type, {} with frequency)",
        records.len(),
        first,
        last,
        records.iter().filter(|r| r.type_code.is_some()).count(),
        records.iter().filter(|r| r.freq_low_mhz.is_some()).count()
    );
    let bytes = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("create {} void: {e}", parent.display());
                std::process::exit(1);
            }
        }
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} records, roundtrip parses ({} B), sha256 {}",
                parsed.len(),
                bytes.len(),
                sha256_hex(&bytes)
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(CDN_TAG, &out) {
        std::process::exit(1);
    }
}
