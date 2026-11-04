use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::redr::{self, RECORD_BYTES};
use omegaflow::archivar::voyager_odr::{SHARD_BUDGET, SHARD_LIMIT, shard_ranges};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "pds-rings.seti.org";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn name_of(source: &str) -> String {
    source
        .rsplit('/')
        .next()
        .map(|s| s.split('?').next().unwrap_or(s))
        .unwrap_or("redr")
        .to_string()
}

fn main() {
    omegaflow::archivar::cdn::require_ci();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let (bytes, source) = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => (b, path),
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => match arg_value(&args, "--url") {
            Some(url) => match fetch_raw_bytes(&url) {
                Some(b) => (b, url),
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("--input <path.odr> or --url <url> required");
                std::process::exit(1);
            }
        },
    };
    let year = match arg_value(&args, "--year") {
        Some(y) => match y.parse::<u16>() {
            Ok(v) if v > 0 => v,
            _ => {
                eprintln!("--year {y}: not a positive year");
                std::process::exit(1);
            }
        },
        None => match redr::record(&bytes) {
            Some(first) if first.header.year >= 70 => 1900 + first.header.year as u16,
            Some(first) => 2000 + first.header.year as u16,
            None => {
                eprintln!(
                    "{source}: no record carries a year; the series stays unwritten (0 honored)"
                );
                std::process::exit(1);
            }
        },
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => format!("sources/{NETLOC}/redr.bin"),
    };
    let (complete, trailing) = (bytes.len() / RECORD_BYTES, bytes.len() % RECORD_BYTES);
    if complete == 0 {
        eprintln!(
            "{source}: {} byte(s) — shorter than one {RECORD_BYTES}-byte REDR record; the series stays unwritten (0 honored)",
            bytes.len()
        );
        std::process::exit(1);
    }
    if record_check(&bytes).is_none() {
        eprintln!("{source}: a record does not parse; the series stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let records = &bytes[..complete * RECORD_BYTES];
    let name = name_of(&source);
    let lengths = [records.len()];
    let ranges = shard_ranges(&lengths, SHARD_BUDGET);
    if ranges.len() > 1 {
        eprintln!(
            "one input file became {} shards; a REDR input is one file, not a set",
            ranges.len()
        );
        std::process::exit(1);
    }
    let bin = omegaflow::archivar::voyager_odr::pack(records, &name, year);
    if bin.len() > SHARD_LIMIT {
        eprintln!(
            "{out}: {}-byte shard exceeds the {SHARD_LIMIT}-byte CDN asset limit — the series stays unwritten (0 honored)",
            bin.len()
        );
        std::process::exit(1);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("{out}: write void");
        std::process::exit(1);
    }
    eprintln!(
        "{out}: {complete} REDR record(s) of {RECORD_BYTES} bytes packed ({trailing} trailing byte(s) dropped), roundtrip holds"
    );
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{out}");
    println!("format redr");
    println!("at earth");
    println!("ttl 604800");
    println!("field sample redr_sample_count inverse-square em count 604800 0.0 0.0");
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

fn record_check(bytes: &[u8]) -> Option<()> {
    let first = redr::record(bytes)?;
    if first.header.sample_rate == 0 {
        return None;
    }
    Some(())
}
