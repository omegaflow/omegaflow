use omegaflow::cdn::upload_asset;
use omegaflow::lsk::{LeapSeconds, parse as parse_lsk};
use omegaflow::pioneer_telemetry::{FILES, parse_bin, parse_series, write_bin};
use std::process::Command;

const BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/pioneer/pioneer10/radio/Turyshev20170327_Pioneer-10/TELEMETRY";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
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
    let out = arg_value(&args, "--out").unwrap_or_else(|| "pioneer10_telemetry.bin".to_string());
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch grid)");
            std::process::exit(1);
        }
    };
    let lsk: LeapSeconds = match parse_lsk(&lsk_text) {
        Some(l) => l,
        None => {
            eprintln!("--lsk parses void — the leap-second table stays unreadable");
            std::process::exit(1);
        }
    };
    let mut raw: Vec<(f64, f64, u32)> = Vec::new();
    let mut pre_lsk_skip = 0usize;
    for f in &FILES {
        let url = format!("{BASE}/{}.txt", f.stem);
        let Some(text) = fetch(&url) else {
            eprintln!(
                "{}: fetch void ({url}) — the file stays unharvested",
                f.stem
            );
            continue;
        };
        let rows = parse_series(&text, f.id);
        let mut n = 0usize;
        for (t_unix, chan, v) in rows {
            match lsk.unix_to_tdb(t_unix) {
                Some(t) => {
                    raw.push((t, v, chan));
                    n += 1;
                }
                None => pre_lsk_skip += 1,
            }
        }
        eprintln!("{}: {} Records", f.stem, n);
    }
    if raw.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    raw.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    eprintln!(
        "Telemetrie: {} Records ({} pre-1972 ungeerntet), {}..{} unix",
        raw.len(),
        pre_lsk_skip,
        raw[0].0,
        raw[raw.len() - 1].0
    );
    let bytes = write_bin(&raw);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!("{out}: {} Records, roundtrip parses", parsed.len());
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
