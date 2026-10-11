use omegaflow::archivar::{RetryPolicy, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::path::Path;

const MANIFEST_NAME: &str = "noaa_eri.manifest";
const COMPILER: &str = "tools/harvest/src/bin/noaa_eri_compiler.rs";
const NETLOC: &str = "noaa-eri-pds.s3.amazonaws.com";
const BASE: &str = "https://noaa-eri-pds.s3.amazonaws.com";
const MAX_KEYS: usize = 1000;
const PAGE_ATTEMPTS: usize = 5;
const PAGE_TRANSFER_BOUND_S: u64 = 1 << 7;
const PAGE_RETRY_DELAY_S: u64 = 2;

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

fn decode_entities(s: &str) -> String {
    if !s.contains('&') {
        return s.to_string();
    }
    let mut out = String::new();
    let mut rest = s;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp..];
        if let Some(semi) = rest.find(';') {
            if semi <= 8 {
                let ent = &rest[..=semi];
                let named = match ent {
                    "&amp;" => Some('&'),
                    "&lt;" => Some('<'),
                    "&gt;" => Some('>'),
                    "&quot;" => Some('"'),
                    "&apos;" => Some('\''),
                    _ => None,
                };
                let numeric = if named.is_none() {
                    if let Some(hex) = ent.strip_prefix("&#x").or_else(|| ent.strip_prefix("&#X")) {
                        hex.strip_suffix(';')
                            .and_then(|h| u32::from_str_radix(h, 16).ok())
                            .and_then(char::from_u32)
                    } else if let Some(dec) = ent.strip_prefix("&#") {
                        dec.strip_suffix(';')
                            .and_then(|d| d.parse::<u32>().ok())
                            .and_then(char::from_u32)
                    } else {
                        None
                    }
                } else {
                    None
                };
                if let Some(c) = named.or(numeric) {
                    out.push(c);
                    rest = &rest[semi + 1..];
                    continue;
                }
            }
        }
        out.push('&');
        rest = &rest[1..];
    }
    out.push_str(rest);
    out
}

fn xml_tag(body: &str, name: &str) -> Option<String> {
    let open = format!("<{name}>");
    let close = format!("</{name}>");
    let start = body.find(&open)? + open.len();
    let end = body[start..].find(&close)? + start;
    Some(decode_entities(&body[start..end]))
}

fn parse_keys(body: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut rest = body;
    while let Some(p) = rest.find("<Key>") {
        let tail = &rest[p + 5..];
        let Some(e) = tail.find("</Key>") else {
            break;
        };
        keys.push(decode_entities(&tail[..e]));
        rest = &tail[e + 6..];
    }
    keys
}

struct Listing {
    keys: Vec<String>,
    truncated: bool,
    next_token: Option<String>,
}

fn parse_page(body: &str) -> Listing {
    let next_token = xml_tag(body, "NextContinuationToken").filter(|t| !t.is_empty());
    let truncated = match xml_tag(body, "IsTruncated") {
        Some(v) => v.eq_ignore_ascii_case("true"),
        None => next_token.is_some(),
    };
    Listing {
        keys: parse_keys(body),
        truncated,
        next_token,
    }
}

fn page_url(base: &str, prefix: &str, token: Option<&str>, max_keys: usize) -> String {
    let mut url = format!("{base}/?list-type=2&max-keys={max_keys}");
    if !prefix.is_empty() {
        url.push_str("&prefix=");
        url.push_str(&enc(prefix));
    }
    if let Some(t) = token {
        url.push_str("&continuation-token=");
        url.push_str(&enc(t));
    }
    url
}

struct Coverage {
    keys: Vec<String>,
    pages: usize,
    keys_seen: usize,
    partial: bool,
}

fn fetch_page(url: &str) -> Option<String> {
    let mut attempt = 0usize;
    while attempt < PAGE_ATTEMPTS {
        if let Some(body) = fetch_raw_with(url, None, &[], RetryPolicy::All, PAGE_TRANSFER_BOUND_S)
        {
            return Some(body);
        }
        attempt += 1;
        if attempt < PAGE_ATTEMPTS {
            std::thread::sleep(std::time::Duration::from_secs(PAGE_RETRY_DELAY_S));
        }
    }
    None
}

fn enumerate(base: &str, prefix: &str, max_files: Option<usize>) -> Result<Coverage, String> {
    let mut keys = Vec::new();
    let mut pages = 0usize;
    let mut keys_seen = 0usize;
    let mut token: Option<String> = None;
    loop {
        let url = page_url(base, prefix, token.as_deref(), MAX_KEYS);
        let Some(body) = fetch_page(&url) else {
            return Err(format!(
                "{url}: the listing returned void after {PAGE_ATTEMPTS} attempts — the walk stays incomplete, no manifest is written"
            ));
        };
        pages += 1;
        let page = parse_page(&body);
        keys_seen += page.keys.len();
        for key in page.keys {
            if let Some(m) = max_files {
                if keys.len() >= m {
                    return Ok(Coverage {
                        keys,
                        pages,
                        keys_seen,
                        partial: true,
                    });
                }
            }
            keys.push(key);
        }
        if !page.truncated {
            break;
        }
        match page.next_token {
            Some(t) => token = Some(t),
            None => {
                return Err(format!(
                    "{url}: IsTruncated carries no NextContinuationToken — the walk stays incomplete"
                ));
            }
        }
    }
    let partial = max_files.is_some_and(|m| keys.len() >= m);
    Ok(Coverage {
        keys,
        pages,
        keys_seen,
        partial,
    })
}

fn run(args: &[String]) -> Result<(), String> {
    let prefix = match arg_value(args, "--prefix") {
        Some(p) => p,
        None => String::new(),
    };
    let max_files: Option<usize> = arg_value(args, "--max-files").and_then(|v| v.parse().ok());
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(o) => o,
        None => format!("data/{NETLOC}/{MANIFEST_NAME}"),
    };
    let out_name = match Path::new(&out).file_name().and_then(|n| n.to_str()) {
        Some(n) => n.to_string(),
        None => MANIFEST_NAME.to_string(),
    };

    let cov = enumerate(BASE, &prefix, max_files)?;
    if cov.keys.is_empty() {
        return Err(format!(
            "{NETLOC} prefix '{prefix}': the listing carried no key — the manifest stays unwritten (0 honored)"
        ));
    }

    let mut text = String::with_capacity(cov.keys.len() * 96);
    for key in &cov.keys {
        text.push_str("shard ");
        text.push_str(BASE);
        text.push('/');
        text.push_str(key);
        text.push('\n');
    }
    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &text).map_err(|e| format!("write {out} returned void: {e}"))?;
    eprintln!(
        "{NETLOC}: {} pages, {} keys read, {} shard lines -> {out}",
        cov.pages,
        cov.keys_seen,
        cov.keys.len()
    );
    if cov.partial {
        eprintln!(
            "{NETLOC}: the walk stands at the --max-files bound — the manifest is partial by word"
        );
    }

    println!("url {CDN_BASE}/{NETLOC}/{out_name}");
    println!("origin {BASE}/?list-type=2");
    println!("compiler {COMPILER}");
    println!("format noaa_eri");

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("noaa_eri_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LISTING_PAGE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/"><Name>noaa-eri-pds</Name><Prefix></Prefix><NextContinuationToken>TOKEN==</NextContinuationToken><KeyCount>2</KeyCount><MaxKeys>2</MaxKeys><IsTruncated>true</IsTruncated><Contents><Key>2005_Hurricane_Katrina/20050829a_2/m1.jpg</Key><LastModified>2006-01-01T00:00:00.000Z</LastModified><ETag>&quot;abc&quot;</ETag><Size>42</Size></Contents><Contents><Key>2005_Hurricane_Katrina/legacy_a&amp;b.jpg</Key><LastModified>2006-01-01T00:00:00.000Z</LastModified><ETag>&quot;def&quot;</ETag><Size>7</Size></Contents></ListBucketResult>"#;

    #[test]
    fn listing_page_reads_keys_and_the_token() {
        let page = parse_page(LISTING_PAGE);
        assert!(page.truncated);
        assert_eq!(page.next_token.as_deref(), Some("TOKEN=="));
        assert_eq!(page.keys.len(), 2);
        assert_eq!(page.keys[0], "2005_Hurricane_Katrina/20050829a_2/m1.jpg");
        assert_eq!(page.keys[1], "2005_Hurricane_Katrina/legacy_a&b.jpg");
    }

    #[test]
    fn page_url_carries_prefix_and_token() {
        let url = page_url(BASE, "2005_Hurricane_Katrina/", Some("a/b=="), 1000);
        assert_eq!(
            url,
            "https://noaa-eri-pds.s3.amazonaws.com/?list-type=2&max-keys=1000&prefix=2005_Hurricane_Katrina%2F&continuation-token=a%2Fb%3D%3D"
        );
    }
}
