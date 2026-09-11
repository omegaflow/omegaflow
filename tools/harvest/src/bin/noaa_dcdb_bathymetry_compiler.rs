use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::geo::{parse_bin, write_bin, MAGIC_DCDB};
use omegaflow::archivar::noaa_nodd::parse_dcdb;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::parse as parse_lsk;

const NETLOC: &str = "noaa-dcdb-bathymetry-pds.s3.amazonaws.com";
const BUCKET: &str = "https://noaa-dcdb-bathymetry-pds.s3.amazonaws.com";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn enc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn tag(body: &str, name: &str) -> Option<String> {
    let open = format!("<{name}>");
    let close = format!("</{name}>");
    let start = body.find(&open)? + open.len();
    let end = body[start..].find(&close)? + start;
    Some(body[start..end].to_string())
}

fn list_keys(prefix: &str, max_files: Option<usize>) -> Vec<String> {
    let mut keys = Vec::new();
    let mut token: Option<String> = None;
    loop {
        let mut url = format!("{BUCKET}/?list-type=2&max-keys=1000&prefix={}", enc(prefix));
        if let Some(t) = &token {
            url.push_str("&continuation-token=");
            url.push_str(&enc(t));
        }
        let Some(body) = fetch_raw(&url, None, &[], 3600) else {
            break;
        };
        let mut rest = body.as_str();
        while let Some(p) = rest.find("<Key>") {
            let tail = &rest[p + 5..];
            let Some(e) = tail.find("</Key>") else {
                break;
            };
            let key = &tail[..e];
            rest = &tail[e + 6..];
            if key.ends_with("_pointData.csv") {
                keys.push(key.to_string());
                if let Some(m) = max_files {
                    if keys.len() >= m {
                        return keys;
                    }
                }
            }
        }
        if !body.contains("<IsTruncated>true</IsTruncated>") {
            break;
        }
        token = tag(&body, "NextContinuationToken");
        if token.is_none() {
            break;
        }
    }
    keys
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let prefix = match arg_value(&args, "--date") {
        Some(v) => {
            let mut it = v.split('-');
            let (Some(y), Some(m), Some(d)) = (it.next(), it.next(), it.next()) else {
                eprintln!("--date reads {v}, not YYYY-MM-DD");
                std::process::exit(1);
            };
            format!("csb/csv/{y}/{m}/{d}/")
        }
        None => match arg_value(&args, "--prefix") {
            Some(v) => v,
            None => {
                eprintln!("--date YYYY-MM-DD or --prefix <s3-prefix> required");
                std::process::exit(1);
            }
        },
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("--out <path> required");
            std::process::exit(1);
        }
    };
    let max_files: Option<usize> = arg_value(&args, "--max-files").and_then(|v| v.parse().ok());
    let lsk = match arg_value(&args, "--lsk")
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|t| parse_lsk(&t))
    {
        Some(l) => l,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void");
            std::process::exit(1);
        }
    };

    let keys = list_keys(&prefix, max_files);
    if keys.is_empty() {
        eprintln!(
            "{prefix}: no _pointData.csv under the prefix — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let mut records = Vec::new();
    let mut files = 0usize;
    for key in &keys {
        let Some(text) = fetch_raw(&format!("{BUCKET}/{key}"), None, &[], 3600) else {
            eprintln!("{key}: fetch void — the file stays unharvested");
            continue;
        };
        let rows = parse_dcdb(&text, &lsk);
        files += 1;
        records.extend(rows);
    }
    if records.is_empty() {
        eprintln!("{prefix}: {files} files carried no measured sounding — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t));
    let bytes = write_bin(MAGIC_DCDB, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_DCDB, &bytes) {
        Some(parsed) => eprintln!(
            "{prefix}: {} files, {} sounding records written, roundtrip parses",
            files,
            parsed.len()
        ),
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
