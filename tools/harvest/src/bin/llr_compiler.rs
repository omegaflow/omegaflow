use omegaflow::archivar::llr::{self, NormalPoint};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::collections::BTreeSet;

const NETLOC: &str = "zenodo.org";
const COMPILER: &str = "tools/harvest/src/bin/llr_compiler.rs";
const FORMAT: &str = "llr";

const SAMPLE: &str = "h1 CRD  2 2023  4  5 21\n\
h2       APOL 7045 95  1  4       ILRS\n\
h3 apollo15        103  103       na 0 1  3\n\
h4  1 2006  4  7  6 24 27 2006  4  7  6 28 50  4 0 0 0 1 0 2 0\n\
11 23190.0000000 2.6494326499837 std1 2  518.0     44      79.2      na      na       na    na 0   3.5\n\
h8\n";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn measured(records: Vec<NormalPoint>) -> (Vec<NormalPoint>, usize) {
    let total = records.len();
    let named: Vec<NormalPoint> = records
        .into_iter()
        .filter(|r| r.epoch_utc.is_some() && r.time_of_flight.is_some())
        .collect();
    let held = total - named.len();
    (named, held)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let input = arg_value(args, "--input")
        .ok_or_else(|| "--input <crd.txt> absent — the file stays unread".to_string())?;
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{NETLOC}/{FORMAT}.bin"),
    };

    let bytes = std::fs::read(&input).map_err(|e| format!("{input}: read void ({e})"))?;
    let records = llr::parse_crd(&bytes)
        .ok_or_else(|| format!("{input}: carries no CRD normal point — refused"))?;
    let (mut named, held) = measured(records);
    if held > 0 {
        eprintln!(
            "{input}: {held} normal points carry no finite epoch/round-trip time — held out, named"
        );
    }
    if named.is_empty() {
        return Err(format!(
            "{input}: no normal point carries a finite epoch and round-trip time — the bin stays unwritten (0 honored)"
        ));
    }
    named.sort_by(|a, b| match (a.epoch_utc, b.epoch_utc) {
        (Some(x), Some(y)) => x.total_cmp(&y),
        _ => std::cmp::Ordering::Equal,
    });

    let bin = llr::write_bin(&named)
        .ok_or_else(|| "a held value is not finite — the bin stays unwritten".to_string())?;
    match llr::parse_bin(&bin) {
        Some(parsed) if parsed == named => {}
        _ => return Err("the roundtrip does not read back — the bin stays unwritten".to_string()),
    }

    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;

    let reflectors: BTreeSet<u32> = named.iter().map(|r| r.reflector).collect();
    let stations: BTreeSet<u32> = named.iter().map(|r| r.station).collect();
    let marked = named.iter().filter(|r| r.flags != 0).count();

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    println!("origin {input}");
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} normal points, reflectors {reflectors:?}, stations {stations:?}, {marked} marked, {} B, roundtrip parses",
        named.len(),
        bin.len()
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: the CDN upload returned void"));
    }
    Ok(())
}

fn selftest() {
    let records = llr::parse_crd(SAMPLE.as_bytes());
    let Some(records) = records else {
        eprintln!("selftest: the sample CRD does not parse");
        std::process::exit(1);
    };
    if records.len() != 1 {
        eprintln!("selftest: the sample does not carry exactly one normal point");
        std::process::exit(1);
    }
    let Some(bin) = llr::write_bin(&records) else {
        eprintln!("selftest: write_bin void");
        std::process::exit(1);
    };
    if bin.len() != llr::HEADER_BYTES + llr::RECORD_BYTES {
        eprintln!("selftest: the bin does not carry the measured record stride");
        std::process::exit(1);
    }
    if llr::parse_bin(&bin) != Some(records.clone()) {
        eprintln!("selftest: the roundtrip does not read back");
        std::process::exit(1);
    }
    if llr::parse_bin(&bin[..bin.len() - 1]).is_some() {
        eprintln!("selftest: a truncated bin reads back");
        std::process::exit(1);
    }
    eprintln!("llr_compiler: selftest passes (CRD normal point → t/round-trip/reflector series)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: llr_compiler --input <crd.txt> [--out <file.bin>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("llr_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_roundtrips_the_measured_stride() {
        let records = llr::parse_crd(SAMPLE.as_bytes()).expect("the sample parses");
        let bin = llr::write_bin(&records).expect("finite records encode");
        assert_eq!(bin.len(), llr::HEADER_BYTES + llr::RECORD_BYTES);
        assert_eq!(llr::parse_bin(&bin), Some(records));
    }

    #[test]
    fn parse_crd_refuses_foreign_bytes() {
        assert_eq!(llr::parse_crd(b"not a crd file"), None);
    }
}
