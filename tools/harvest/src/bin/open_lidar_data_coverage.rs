use omegaflow::archivar::fetch_raw;
use omegaflow::cdn::upload_release;
use std::collections::BTreeSet;
use std::io::Write;

const BUCKET: &str = "open-lidar-data";
const NETLOC: &str = "open-lidar-data.s3.amazonaws.com";
const BASE: &str = "https://open-lidar-data.s3.amazonaws.com";
const COMPILER: &str = "tools/harvest/src/bin/open_lidar_data_coverage.rs";
const DEFAULT_PREFIX: &str = "data/";
const MANIFEST_NAME: &str = "open_lidar_data_coverage.manifest";
const MAX_KEYS: usize = 1000;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
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

fn xml_unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}

fn xml_text(doc: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = doc.find(&open)? + open.len();
    let end = doc[start..].find(&close)? + start;
    Some(xml_unescape(&doc[start..end]))
}

fn parse_keys(doc: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut rest = doc;
    while let Some(p) = rest.find("<Key>") {
        let tail = &rest[p + "<Key>".len()..];
        let Some(e) = tail.find("</Key>") else {
            break;
        };
        keys.push(xml_unescape(&tail[..e]));
        rest = &tail[e + "</Key>".len()..];
    }
    keys
}

fn list_keys(prefix: &str) -> Result<Vec<String>, String> {
    let mut keys = Vec::new();
    let mut token: Option<String> = None;
    let mut page = 0usize;
    loop {
        page += 1;
        let mut url = format!(
            "{BASE}/?list-type=2&max-keys={MAX_KEYS}&prefix={}",
            enc(prefix)
        );
        if let Some(t) = &token {
            url.push_str("&continuation-token=");
            url.push_str(&enc(t));
        }
        let body = fetch_raw(&url, None, &[])
            .ok_or_else(|| format!("{url}: the listing returned void"))?;
        let page_keys = parse_keys(&body);
        let page_count = page_keys.len();
        keys.extend(page_keys);
        let truncated = xml_text(&body, "IsTruncated").as_deref() == Some("true");
        if page == 1 || !truncated || page % 100 == 0 {
            eprintln!(
                "{BUCKET}?prefix={prefix}: page {page}, {page_count} keys this page, {} keys so far, truncated {truncated}",
                keys.len()
            );
        }
        if !truncated {
            break;
        }
        match xml_text(&body, "NextContinuationToken") {
            Some(t) if !t.is_empty() => token = Some(t),
            _ => break,
        }
    }
    Ok(keys)
}

fn country_of(key: &str) -> Option<&str> {
    let mut parts = key.split('/');
    if parts.next() != Some("data") {
        return None;
    }
    parts.next().filter(|c| !c.is_empty())
}

fn is_copc_object(key: &str) -> bool {
    key.ends_with(".copc.laz")
}

fn object_url(key: &str) -> String {
    format!("{BASE}/{key}")
}

fn coverage(keys: &[String]) -> (Vec<String>, Vec<String>) {
    let mut urls = Vec::new();
    let mut countries: BTreeSet<String> = BTreeSet::new();
    for key in keys {
        if !is_copc_object(key) {
            continue;
        }
        let Some(country) = country_of(key) else {
            continue;
        };
        countries.insert(country.to_string());
        urls.push(object_url(key));
    }
    urls.sort();
    urls.dedup();
    (urls, countries.into_iter().collect())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: open_lidar_data_coverage [--prefix <data/>] [--out <manifest>] [--ci-mode]";
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let prefix = match arg_value(&args, "--prefix") {
        Some(p) if !p.is_empty() => p,
        _ => DEFAULT_PREFIX.to_string(),
    };
    let out_path = match arg_value(&args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{NETLOC}/{MANIFEST_NAME}"),
    };
    if args.iter().any(|a| a == "--help") {
        eprintln!("{usage}");
        std::process::exit(1);
    }

    let keys = match list_keys(&prefix) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("open_lidar_data_coverage: {e}");
            std::process::exit(1);
        }
    };
    eprintln!("{BUCKET}?prefix={prefix}: {} keys listed", keys.len());

    let (urls, countries) = coverage(&keys);
    if urls.is_empty() {
        eprintln!(
            "open_lidar_data_coverage: no COPC object under {prefix} — the manifest stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }

    let mut text = String::new();
    for u in &urls {
        text.push_str(u);
        text.push('\n');
    }
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    let mut out = match std::fs::File::create(&out_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("create {out_path}: {e}");
            std::process::exit(1);
        }
    };
    if let Err(e) = out.write_all(text.as_bytes()) {
        eprintln!("write {out_path}: {e}");
        std::process::exit(1);
    }
    if let Err(e) = out.flush() {
        eprintln!("flush {out_path}: {e}");
        std::process::exit(1);
    }
    eprintln!(
        "{out_path}: {} COPC objects across {} countries, {} B",
        urls.len(),
        countries.len(),
        text.len()
    );

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{MANIFEST_NAME}");
    println!("origin {BASE}/?list-type=2&prefix={prefix}");
    println!("compiler {COMPILER}");
    println!("format open_lidar_data_coverage");
    println!("countries {}", countries.len());
    println!("objects {}", urls.len());

    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("upload: {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage_groups_copc_objects_by_country() {
        let keys = vec![
            "data/BE/EODaS/LiDAR_DHMV_II-2013-2015/copc/b.copc.laz".to_string(),
            "data/US/USGS/foo/copc/a.copc.laz".to_string(),
            "data/BE/EODaS/LiDAR_DHMV_II-2013-2015/copc/readme.txt".to_string(),
            "data/US/USGS/foo/copc/a.copc.laz".to_string(),
            "other/NO/x.copc.laz".to_string(),
        ];
        let (urls, countries) = coverage(&keys);
        assert_eq!(countries, vec!["BE".to_string(), "US".to_string()]);
        assert_eq!(
            urls,
            vec![
                "https://open-lidar-data.s3.amazonaws.com/data/BE/EODaS/LiDAR_DHMV_II-2013-2015/copc/b.copc.laz".to_string(),
                "https://open-lidar-data.s3.amazonaws.com/data/US/USGS/foo/copc/a.copc.laz".to_string(),
            ]
        );
        assert_eq!(country_of("data/BE/x"), Some("BE"));
        assert_eq!(country_of("data/"), None);
        assert_eq!(country_of("other/BE/x"), None);
    }
}
