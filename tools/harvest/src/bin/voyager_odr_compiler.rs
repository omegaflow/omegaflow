use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::voyager_odr::{RECORD_BYTES, pack, parse_odr, parse_packed, split_records};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "pds-ppi.igpp.ucla.edu";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn name_of(source: &str) -> String {
    let mut base = source;
    if let Some((_, tail)) = source.rsplit_once('/') {
        base = tail;
    }
    match base.split_once('?') {
        Some((head, _)) => head,
        None => base,
    }
    .to_string()
}

fn year_of_label(bytes: &[u8]) -> Option<u16> {
    let text = std::str::from_utf8(bytes).ok()?;
    for line in text.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("START_TIME") else {
            continue;
        };
        let eq = rest.find('=')?;
        let value = rest[eq + 1..].trim();
        let year = value.split('-').next()?.trim();
        return year.parse().ok();
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "data/pds-ppi.igpp.ucla.edu/voyager_odr.bin".to_string(),
    };
    let (bytes, source) = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => (b, path),
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => match arg_value(&args, "--url") {
            Some(url) => match fetch_raw_bytes(&url, 604800) {
                Some(b) => (b, url),
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("--input <path.ODR> or --url <url> required");
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
        None => match arg_value(&args, "--label-url") {
            Some(url) => match fetch_raw_bytes(&url, 604800) {
                Some(lb) => match year_of_label(&lb) {
                    Some(v) => v,
                    None => {
                        eprintln!("{url}: START_TIME year void");
                        std::process::exit(1);
                    }
                },
                None => {
                    eprintln!("{url}: label fetch void");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("--year <yyyy> or --label-url <url> required");
                std::process::exit(1);
            }
        },
    };
    let (complete, trailing) = split_records(bytes.len());
    if complete == 0 {
        eprintln!(
            "{} byte(s) — shorter than one {RECORD_BYTES}-byte ODR record; the series stays unwritten (0 honored)",
            bytes.len()
        );
        std::process::exit(1);
    }
    let Some(decoded) = parse_odr(&bytes) else {
        eprintln!("ODR decode void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let name = name_of(&source);
    let raw = &bytes[..complete * RECORD_BYTES];
    let bin = pack(raw, &name, year);
    let Some(parsed) = parse_packed(&bin) else {
        eprintln!("{out}: packed read void — the series stays unverified (0 honored)");
        std::process::exit(1);
    };
    if parsed.records != decoded || parsed.year != year || parsed.name != name {
        eprintln!("{out}: roundtrip void — the series stays unverified (0 honored)");
        std::process::exit(1);
    }
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    eprintln!(
        "{out}: {} ODR record(s) packed ({complete} complete + {trailing} trailing byte(s)), year {year}, sha256 {}, roundtrip holds",
        decoded.len(),
        sha256_hex(raw)
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::voyager_odr::{HEADER_BYTES, SAMPLE_BYTES};

    fn sample_record(record_number: u16, word1: u16) -> Vec<u8> {
        let mut bytes = vec![0u8; RECORD_BYTES];
        bytes[..HEADER_BYTES].copy_from_slice(&[
            0x90, 0x06, 0x00, 0x01, 0x09, 0xE0, 0x20, 0x2B, 0x00, 0x1D, 0x23, 0x72, 0x03, 0x55,
            0x9F, 0x41, 0x5E, 0x25, 0x00, 0x60, 0x00, 0xA2, 0x50, 0xFE, 0xDB, 0x08, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x8B, 0x81, 0x4C, 0x59, 0x00, 0xA2, 0x50, 0x1D, 0xFF, 0xFB, 0x6C, 0x4E,
        ]);
        bytes[0..2].copy_from_slice(&word1.to_be_bytes());
        bytes[2..4].copy_from_slice(&record_number.to_be_bytes());
        for i in 0..SAMPLE_BYTES {
            bytes[HEADER_BYTES + i] = i as u8;
        }
        bytes
    }

    #[test]
    fn year_of_label_reads_start_time_year() {
        let label = b"PDS_VERSION_ID = PDS3\nSTART_TIME = 1981-08-26T04:05:00.000\nEND\n";
        assert_eq!(year_of_label(label), Some(1981));
        assert_eq!(
            year_of_label(b"START_TIME = 1979-01-01T00:00:00.000\n"),
            Some(1979)
        );
        assert_eq!(year_of_label(b"no start time here"), None);
    }

    #[test]
    fn name_of_strips_path_and_query() {
        assert_eq!(
            name_of("https://pds-ppi.igpp.ucla.edu/data/VG2-S-RSS-1-ROCC-V1.0/DATA/C0XR13AA.ODR"),
            "C0XR13AA.ODR"
        );
        assert_eq!(name_of("/tmp/C0XR13AA.ODR"), "C0XR13AA.ODR");
        assert_eq!(name_of("C0XR13AA.ODR"), "C0XR13AA.ODR");
    }

    #[test]
    fn pack_and_roundtrip_hold_for_measured_records() {
        let mut raw = Vec::new();
        raw.extend_from_slice(&sample_record(1, 0x9006));
        raw.extend_from_slice(&sample_record(2, 0x0006));
        let records = parse_odr(&raw).unwrap();
        let bin = pack(&raw, "C0XR13AA.ODR", 1981);
        let parsed = parse_packed(&bin).unwrap();
        assert_eq!(parsed.records, records);
        assert_eq!(parsed.year, 1981);
        assert_eq!(parsed.name, "C0XR13AA.ODR");
    }
}
