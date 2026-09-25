use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::voyager_occlt::{
    MED_RECORD_BYTES, pack, parse_mediumband, parse_packed, split_records,
};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "spdf.gsfc.nasa.gov";

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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    eprintln!(
        "voyager_occlt: the PSPA tar list is pending (unmeasured) — the tar mode stays silent; \
         the measured payload path runs via --input <payload.DAT> or --url <url>"
    );
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "data/spdf.gsfc.nasa.gov/voyager_occlt.bin".to_string(),
    };
    let (bytes, name) = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => (b, name_of(&path)),
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => match arg_value(&args, "--url") {
            Some(url) => match fetch_raw_bytes(&url, 604800) {
                Some(b) => (b, name_of(&url)),
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("--input <payload.DAT> or --url <url> required");
                std::process::exit(2);
            }
        },
    };
    let (complete, trailing) = split_records(bytes.len());
    if complete == 0 {
        eprintln!(
            "{} byte(s) — shorter than one {MED_RECORD_BYTES}-byte mediumband record; the series stays unwritten (0 honored)",
            bytes.len()
        );
        std::process::exit(1);
    }
    let raw = &bytes[..complete * MED_RECORD_BYTES];
    let Some(records) = parse_mediumband(raw) else {
        eprintln!("mediumband decode void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let bin = pack(raw, &name);
    let Some(parsed) = parse_packed(&bin) else {
        eprintln!("{out}: packed read void — the series stays unverified (0 honored)");
        std::process::exit(1);
    };
    if parsed.files.len() != 1 || parsed.files[0].name != name || parsed.files[0].records != records
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
        "{out}: {} mediumband record(s) packed ({} complete + {} trailing byte(s)), sha256 {}, roundtrip holds; \
         the sample decode is pending (data-value stride and word byte order unmeasured)",
        records.len(),
        complete,
        trailing,
        sha256_hex(raw)
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::voyager_occlt::MED_RECORD_BYTES;

    fn sample_payload(records: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; records * MED_RECORD_BYTES];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (i % 251) as u8;
        }
        bytes
    }

    #[test]
    fn name_of_strips_path_and_query() {
        assert_eq!(
            name_of(
                "https://spdf.gsfc.nasa.gov/pub/data/voyager/voyager1/radio_science_rss/saturn_occultation_narrow_band/S0A.DAT"
            ),
            "S0A.DAT"
        );
        assert_eq!(name_of("/tmp/S0A.DAT"), "S0A.DAT");
        assert_eq!(name_of("S0A.DAT"), "S0A.DAT");
    }

    #[test]
    fn pack_and_roundtrip_hold_for_framed_payloads() {
        let raw = sample_payload(2);
        let records = parse_mediumband(&raw).unwrap();
        let bin = pack(&raw, "S0A.DAT");
        let parsed = parse_packed(&bin).unwrap();
        assert_eq!(parsed.files.len(), 1);
        assert_eq!(parsed.files[0].records, records);
        assert_eq!(parsed.files[0].name, "S0A.DAT");
        assert_eq!(split_records(raw.len()), (2, 0));
    }
}
