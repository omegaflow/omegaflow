use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::voyager_odr::{
    RECORD_BYTES, SHARD_BUDGET, SHARD_LIMIT, pack, pack_many, parse_odr, parse_packed, shard_name,
    shard_ranges, split_records,
};
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

fn index_rows(text: &str) -> Option<Vec<(String, String, u16)>> {
    let mut rows = Vec::new();
    for line in text.lines() {
        let fields: Vec<String> = line
            .split(',')
            .map(|f| f.trim().trim_matches('"').trim().to_string())
            .collect();
        if fields.len() < 7 || !fields[2].ends_with(".ODR") {
            continue;
        }
        let year = fields[6].split('-').next()?.parse().ok()?;
        rows.push((fields[1].clone(), fields[2].clone(), year));
    }
    Some(rows)
}

fn pack_and_verify(entries: &[(Vec<u8>, String, u16)], out: &str) -> Vec<u8> {
    let refs: Vec<(&[u8], &str, u16)> = entries
        .iter()
        .map(|(b, n, y)| (b.as_slice(), n.as_str(), *y))
        .collect();
    let bin = pack_many(&refs);
    let Some(parsed) = parse_packed(&bin) else {
        eprintln!("{out}: packed read void — the series stays unverified (0 honored)");
        std::process::exit(1);
    };
    if parsed.files.len() != entries.len() {
        eprintln!("{out}: entry count void — the series stays unverified (0 honored)");
        std::process::exit(1);
    }
    for (f, (b, n, y)) in parsed.files.iter().zip(entries.iter()) {
        if f.name != *n || f.year != *y || f.records.len() != b.len() / RECORD_BYTES {
            eprintln!("{out}: {n} roundtrip void — the series stays unverified (0 honored)");
            std::process::exit(1);
        }
    }
    if let Some(parent) = std::path::Path::new(out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    bin
}

fn run_index(args: &[String]) {
    let Some(index_path) = arg_value(args, "--index") else {
        return;
    };
    let Some(base) = arg_value(args, "--base") else {
        eprintln!("--index <INDEX.TAB> requires --base <dataset-url>");
        std::process::exit(1);
    };
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => "data/pds-ppi.igpp.ucla.edu/voyager_odr.bin".to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let dir = arg_value(args, "--dir");
    let text = match std::fs::read_to_string(&index_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("read {index_path} returned void: {e}");
            std::process::exit(1);
        }
    };
    let Some(rows) = index_rows(&text) else {
        eprintln!("{index_path}: index parse void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    if rows.is_empty() {
        eprintln!("{index_path}: no .ODR rows");
        std::process::exit(1);
    }
    let base_trim = base.trim_end_matches('/');
    let mut entries: Vec<(Vec<u8>, String, u16)> = Vec::new();
    let mut trailing_total = 0usize;
    for (path, name, year) in rows {
        let dir_of = match path.rsplit_once('/') {
            Some((d, _)) => d,
            None => "",
        };
        let url = format!("{base_trim}{dir_of}/{name}");
        let bytes = match &dir {
            Some(d) => std::fs::read(format!("{d}/{name}")).ok(),
            None => fetch_raw_bytes(&url, 604800),
        };
        let Some(bytes) = bytes else {
            eprintln!("{name}: read void at {url}");
            std::process::exit(1);
        };
        let (complete, trailing) = split_records(bytes.len());
        if complete == 0 {
            eprintln!(
                "{name}: {} byte(s) — shorter than one {RECORD_BYTES}-byte ODR record; the series stays unwritten (0 honored)",
                bytes.len()
            );
            std::process::exit(1);
        }
        trailing_total += trailing;
        entries.push((bytes[..complete * RECORD_BYTES].to_vec(), name, year));
    }
    let lengths: Vec<usize> = entries.iter().map(|(b, _, _)| b.len()).collect();
    let ranges = shard_ranges(&lengths, SHARD_BUDGET);
    if ranges.len() <= 1 {
        pack_and_verify(&entries, &out);
        let total_bytes: usize = entries.iter().map(|(b, _, _)| b.len()).sum();
        eprintln!(
            "{out}: {} .ODR file(s) packed ({} bytes, {} trailing byte(s) dropped), roundtrip holds",
            entries.len(),
            total_bytes,
            trailing_total
        );
        if ci_mode && !upload_release(NETLOC, &out) {
            std::process::exit(1);
        }
        std::process::exit(0);
    }
    let mut paths: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for (ord, &(lo, hi)) in ranges.iter().enumerate() {
        let name = shard_name("voyager_odr", ord);
        let path = format!("data/pds-ppi.igpp.ucla.edu/{name}");
        let bin = pack_and_verify(&entries[lo..hi], &path);
        if bin.len() > SHARD_LIMIT {
            eprintln!(
                "{path}: {}-byte shard exceeds the {SHARD_LIMIT}-byte CDN asset limit — the series stays unwritten (0 honored)",
                bin.len()
            );
            std::process::exit(1);
        }
        eprintln!(
            "{path}: {} .ODR file(s) packed ({} bytes), roundtrip holds",
            hi - lo,
            bin.len()
        );
        names.push(name);
        paths.push(path);
    }
    for name in &names {
        println!(
            "url https://github.com/omegaflow/sources/releases/download/pds-ppi.igpp.ucla.edu/{name}"
        );
        println!("format voyager_odr");
        println!("at earth");
        println!("ttl 604800");
        println!("field sample voyager_odr_sample_count inverse-square em count 604800 0.0 0.0");
        println!();
    }
    if ci_mode {
        for path in &paths {
            if !upload_release(NETLOC, path) {
                std::process::exit(1);
            }
        }
    }
    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    run_index(&args);
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
    if parsed.files.len() != 1
        || parsed.files[0].records != decoded
        || parsed.files[0].year != year
        || parsed.files[0].name != name
    {
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
        assert_eq!(parsed.files.len(), 1);
        assert_eq!(parsed.files[0].records, records);
        assert_eq!(parsed.files[0].year, 1981);
        assert_eq!(parsed.files[0].name, "C0XR13AA.ODR");
    }

    #[test]
    fn index_rows_reads_measured_index_tab_format() {
        let tab = "DATA_SET_ID            ,FILE_SPECIFICATION_NAME  ,PRODUCT_ID    ,VOLUME_ID  ,PRODUCT_CREATION_TIME  ,TARGET_NAME  ,START_TIME             ,STOP_TIME              \n\
\"VG2-S-RSS-1-ROCC-V1.0\",\"/CALIB/VG2SPOC1.LBL    \",\"VG2SPOC1.DAT\",\"VG2_9065 \",1999-06-30T00:00:00Z   ,\"SATURN     \",1981-08-26T03:44:18Z   ,1981-08-26T07:59:57Z    \n\
\"VG2-S-RSS-1-ROCC-V1.0\",\"/DATA/C0SR01AA.LBL     \",\"C0SR01AA.ODR\",\"VG2_9065 \",1999-06-30T00:00:00Z   ,\"SATURN     \",1981-08-26T03:45:00.000,1981-08-26T03:49:57.000\n\
\"VG2-S-RSS-1-ROCC-V1.0\",\"/DATA/C1SR04AA.LBL     \",\"C1SR04AA.ODR\",\"VG2_9065 \",1999-06-30T00:00:00Z   ,\"SATURN     \",1981-08-26T04:30:00.000,1981-08-26T04:34:57.000\n";
        let rows = index_rows(tab).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "/DATA/C0SR01AA.LBL");
        assert_eq!(rows[0].1, "C0SR01AA.ODR");
        assert_eq!(rows[0].2, 1981);
        assert_eq!(rows[1].1, "C1SR04AA.ODR");
        assert_eq!(rows[1].2, 1981);
        assert!(index_rows("no rows here").unwrap().is_empty());
        assert!(
            index_rows(
                "\"a\",\"/DATA/X.LBL\",\"X.ODR\",\"v\",1999-01-01Z,\"t\",\"BAD\",1999-01-01Z"
            )
            .is_none()
        );
    }
}
