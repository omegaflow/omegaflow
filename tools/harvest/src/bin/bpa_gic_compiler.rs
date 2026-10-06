use omegaflow::archivar::bpa_gic::{CHANNELS, parse_bin, parse_text, write_bin};
use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::sha256::sha256_hex;

const URL: &str = "https://transmission.bpa.gov/business/operations/gic/gic.txt";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => "bpa_gic.bin".to_string(),
    };

    let local_in = arg_value(&args, "--in").or_else(|| arg_value(&args, "--input"));
    let text = match local_in {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(_) => {
                eprintln!("{path} unreadable — the bin stays unwritten (0 honored)");
                std::process::exit(1);
            }
        },
        None => {
            let url = match arg_value(&args, "--url") {
                Some(u) => u,
                None => URL.to_string(),
            };
            match fetch_raw(&url, None, &[]) {
                Some(t) => t,
                None => {
                    eprintln!("{url} fetch void — the bin stays unwritten (0 honored)");
                    std::process::exit(1);
                }
            }
        }
    };

    let mut records = parse_text(&text);
    records.sort_by(|a, b| a.t_unix.total_cmp(&b.t_unix));
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let present: usize = records
        .iter()
        .map(|r| r.values().iter().filter(|v| v.is_some()).count())
        .sum();
    eprintln!(
        "source: {} B, sha256 {}",
        text.len(),
        sha256_hex(text.as_bytes())
    );
    eprintln!(
        "BPA GIC: {} records, {} present values, {} channels",
        records.len(),
        present,
        CHANNELS
    );

    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} records, {} B, sha256 {}, roundtrip parses",
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
    if ci_mode && !omegaflow::cdn::upload_release("transmission.bpa.gov", &out) {
        std::process::exit(1);
    }
}
