use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pvo_orse::{
    SHARD_BUDGET, SHARD_LIMIT, complete_bytes, pack, pack_many, parse_odr, parse_packed,
    record_count, shard_name, shard_ranges,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "pds-ppi.igpp.ucla.edu";
const SHARD_PREFIX: &str = "pvo_orse_odr";

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

fn index_rows(text: &str) -> (Vec<(String, String, u16)>, usize) {
    let mut rows = Vec::new();
    let mut undated = 0usize;
    for line in text.lines() {
        let fields: Vec<String> = line
            .split(',')
            .map(|f| f.trim().trim_matches('"').trim().to_string())
            .collect();
        if fields.len() < 7 || !fields[2].ends_with(".ODR") {
            continue;
        }
        let start = fields[6]
            .split('-')
            .next()
            .and_then(|y| y.parse::<u16>().ok());
        let stop = fields
            .get(7)
            .and_then(|s| s.split('-').next())
            .and_then(|y| y.parse::<u16>().ok());
        match start.or(stop) {
            Some(year) => rows.push((fields[1].clone(), fields[2].clone(), year)),
            None => undated += 1,
        }
    }
    (rows, undated)
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
        if f.name != *n || f.year != *y || f.record_count != record_count(b) {
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
        None => "data/pds-ppi.igpp.ucla.edu/pvo_orse_odr.bin".to_string(),
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
    let (rows, undated) = index_rows(&text);
    if undated > 0 {
        eprintln!(
            "{index_path}: {undated} row(s) carry no measurable START_TIME/STOP_TIME year — skipped (0 honored)"
        );
    }
    if rows.is_empty() {
        eprintln!("{index_path}: no dated .ODR rows");
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
            None => fetch_raw_bytes(&url),
        };
        let Some(bytes) = bytes else {
            eprintln!("{name}: read void at {url}");
            std::process::exit(1);
        };
        let complete = complete_bytes(&bytes);
        if record_count(&bytes) == 0 {
            eprintln!(
                "{name}: {} byte(s) — no complete ODR record frame; the series stays unwritten (0 honored)",
                bytes.len()
            );
            std::process::exit(1);
        }
        trailing_total += bytes.len() - complete;
        entries.push((bytes[..complete].to_vec(), name, year));
    }
    let lengths: Vec<usize> = entries.iter().map(|(b, _, _)| b.len()).collect();
    let ranges = shard_ranges(&lengths, SHARD_BUDGET);
    if ranges.len() <= 1 {
        pack_and_verify(&entries, &out);
        let total_bytes: usize = entries.iter().map(|(b, _, _)| b.len()).sum();
        let total_records: usize = entries.iter().map(|(b, _, _)| record_count(b)).sum();
        eprintln!(
            "{out}: {} ODR file(s) packed ({total_records} records, {total_bytes} bytes, {trailing_total} trailing byte(s) dropped), roundtrip holds",
            entries.len()
        );
        if ci_mode && !upload_release(NETLOC, &out) {
            std::process::exit(1);
        }
        std::process::exit(0);
    }
    let mut paths: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for (ord, &(lo, hi)) in ranges.iter().enumerate() {
        let name = shard_name(SHARD_PREFIX, ord);
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
            "{path}: {} ODR file(s) packed ({} bytes), roundtrip holds",
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
        println!("format pvo_orse");
        println!("at earth");
        println!("ttl 604800");
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
        None => "data/pds-ppi.igpp.ucla.edu/pvo_orse_odr.bin".to_string(),
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
            Some(url) => match fetch_raw_bytes(&url) {
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
            Some(url) => match fetch_raw_bytes(&url) {
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
    let complete = complete_bytes(&bytes);
    if record_count(&bytes) == 0 {
        eprintln!(
            "{} byte(s) — no complete ODR record frame; the series stays unwritten (0 honored)",
            bytes.len()
        );
        std::process::exit(1);
    }
    let Some(decoded) = parse_odr(&bytes) else {
        eprintln!("ODR decode void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let name = name_of(&source);
    let raw = &bytes[..complete];
    let bin = pack(raw, &name, year);
    let Some(parsed) = parse_packed(&bin) else {
        eprintln!("{out}: packed read void — the series stays unverified (0 honored)");
        std::process::exit(1);
    };
    if parsed.files.len() != 1
        || parsed.files[0].record_count != decoded.len()
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
        "{out}: {} ODR record(s) packed ({} complete bytes + {} trailing byte(s)), year {year}, sha256 {}, roundtrip holds",
        decoded.len(),
        complete,
        bytes.len() - complete,
        sha256_hex(raw)
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::pvo_orse::WORD_BYTES;

    fn sample_record(len_words: usize, record_number: u16, word1: u16) -> Vec<u8> {
        let mut bytes = vec![0u8; len_words * WORD_BYTES];
        bytes[0..2].copy_from_slice(&word1.to_be_bytes());
        bytes[2..4].copy_from_slice(&record_number.to_be_bytes());
        bytes[4..6].copy_from_slice(&(len_words as u16).to_be_bytes());
        bytes[6] = 12;
        bytes[7] = 42;
        bytes
    }

    #[test]
    fn year_of_label_reads_start_time_year() {
        let label = b"PDS_VERSION_ID = PDS3\nSTART_TIME = 1984-08-18T23:22:47.000Z\nEND\n";
        assert_eq!(year_of_label(label), Some(1984));
        assert_eq!(
            year_of_label(b"START_TIME = 1986-07-20T23:26:00.000Z\n"),
            Some(1986)
        );
        assert_eq!(year_of_label(b"no start time here"), None);
    }

    #[test]
    fn name_of_strips_path_and_query() {
        assert_eq!(
            name_of(
                "https://pds-ppi.igpp.ucla.edu/data/PVO-V-ORSE-1-ODR-OPENLOOP--V1.0/DATA/19840818_19841101/ODR8231.ODR"
            ),
            "ODR8231.ODR"
        );
        assert_eq!(name_of("/tmp/ODR8231.ODR"), "ODR8231.ODR");
        assert_eq!(name_of("ODR8231.ODR"), "ODR8231.ODR");
    }

    #[test]
    fn pack_and_roundtrip_hold_for_measured_records() {
        let mut raw = Vec::new();
        raw.extend_from_slice(&sample_record(2045, 1, 0xC101));
        raw.extend_from_slice(&sample_record(2045, 2, 0x0101));
        let records = parse_odr(&raw).unwrap();
        let bin = pack(&raw, "ODR8231.ODR", 1984);
        let parsed = parse_packed(&bin).unwrap();
        assert_eq!(parsed.files.len(), 1);
        assert_eq!(parsed.files[0].record_count, records.len());
        assert_eq!(parsed.files[0].year, 1984);
        assert_eq!(parsed.files[0].name, "ODR8231.ODR");
    }

    #[test]
    fn index_rows_reads_measured_index_tab_format() {
        let tab = "DATA_SET_ID                      ,FILE_SPECIFICATION_NAME                     ,PRODUCT_ID       ,VOLUME_ID  ,PRODUCT_CREATION_TIME,TARGET_NAME  ,START_TIME              ,STOP_TIME               \n\
\"PVO-V-ORSE-1-ODR-OPENLOOP--V1.0\",\"/DATA/19840818_19841101/ODR8231.LBL       \",\"ODR8231.ODR    \",\"PVRSE_ODR\",1993-11-01           ,\"VENUS      \",1984-08-18T23:22:47.000Z,1984-08-18T23:58:22.000Z\n\
\"PVO-V-ORSE-1-ODR-OPENLOOP--V1.0\",\"/DATA/19860720_19870118/PXC0001.LBL       \",\"PXC0001.ODR    \",\"PVRSE_ODR\",1993-09-01           ,\"VENUS      \",1986-07-20T23:26:00.000Z,1986-07-21T00:40:09.000Z\n\
\"PVO-V-ORSE-1-ODR-OPENLOOP--V1.0\",\"/DATA/19870910_19871116/SEASON11_INDEX    \",\"SEASON11_INDEX \",\"PVRSE_ODR\",1993-09-01           ,\"VENUS      \",1987-09-10T00:00:00.000Z,1987-11-16T00:00:00.000Z\n";
        let (rows, undated) = index_rows(tab);
        assert_eq!(undated, 0);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "/DATA/19840818_19841101/ODR8231.LBL");
        assert_eq!(rows[0].1, "ODR8231.ODR");
        assert_eq!(rows[0].2, 1984);
        assert_eq!(rows[1].1, "PXC0001.ODR");
        assert_eq!(rows[1].2, 1986);
        assert!(index_rows("no rows here").0.is_empty());
    }

    #[test]
    fn index_rows_falls_back_to_stop_time_and_skips_undated() {
        let tab = "\"S\",\"/D/ODR001.LBL\",\"ODR001.ODR\",\"V\",1994-07-01,\"VENUS\",UNK,1978-05-13T18:30:00.000Z\n\
\"S\",\"/D/ODR002.LBL\",\"ODR002.ODR\",\"V\",1994-07-01,\"VENUS\",UNK,UNK\n";
        let (rows, undated) = index_rows(tab);
        assert_eq!(undated, 1);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].1, "ODR001.ODR");
        assert_eq!(rows[0].2, 1978);
    }

    #[test]
    fn complete_bytes_and_record_count_follow_measured_framing() {
        let raw = sample_record(2045, 1, 0xC101);
        assert_eq!(complete_bytes(&raw), raw.len());
        assert_eq!(record_count(&raw), 1);
    }
}
