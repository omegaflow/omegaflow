use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

const MANIFEST_NAME: &str = "linz_coverage.manifest";
const COMPILER: &str = "tools/harvest/src/bin/linz_coverage.rs";
const DEFAULT_BUCKET: &str = "nz-elevation";
const S3_REGION_SUFFIX: &str = "s3-ap-southeast-2.amazonaws.com";
const MAX_KEYS: usize = 1000;

struct S3Object {
    key: String,
}

struct Listing {
    objects: Vec<S3Object>,
    truncated: bool,
    next_token: String,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn listing_fetch(url: &str) -> Option<String> {
    fetch_raw_with(url, None, &[], RetryPolicy::All, TRANSFER_BOUND_S)
}

fn gt_after(s: &str, from: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = from;
    let mut in_q = false;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => in_q = !in_q,
            b'>' if !in_q => return Some(i),
            _ => {}
        }
        i += 1;
    }
    None
}

fn tag_local(s: &str, from: usize) -> Option<(String, bool, usize)> {
    let b = s.as_bytes();
    if b.get(from) != Some(&b'<') {
        return None;
    }
    let mut i = from + 1;
    let mut closing = false;
    if b.get(i) == Some(&b'/') {
        closing = true;
        i += 1;
    }
    let start = i;
    while i < s.len() {
        match b[i] {
            b' ' | b'>' | b'/' | b'\t' | b'\r' | b'\n' => break,
            _ => i += 1,
        }
    }
    if i == start {
        return None;
    }
    let raw = &s[start..i];
    let local = raw.rsplit(':').next().unwrap_or("").to_string();
    let gt = gt_after(s, i)?;
    Some((local, closing, gt))
}

fn skip_to_lt(s: &str, from: usize) -> usize {
    match s[from..].find('<') {
        Some(p) => from + p,
        None => s.len(),
    }
}

fn decode(s: &str) -> String {
    if !s.contains('&') {
        return s.to_string();
    }
    let mut out = String::new();
    let mut rest = s;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp..];
        match rest.find(';') {
            Some(semi) if semi <= 8 => {
                let ent = &rest[..=semi];
                let named = match ent {
                    "&amp;" => Some("&"),
                    "&lt;" => Some("<"),
                    "&gt;" => Some(">"),
                    "&quot;" => Some("\""),
                    "&apos;" => Some("'"),
                    _ => None,
                };
                if let Some(r) = named {
                    out.push_str(r);
                    rest = &rest[semi + 1..];
                    continue;
                }
                let numeric = if ent.len() > 3 && (ent.starts_with("&#x") || ent.starts_with("&#X"))
                {
                    u32::from_str_radix(&ent[3..ent.len() - 1], 16)
                        .ok()
                        .and_then(char::from_u32)
                } else if ent.len() > 2 && ent.starts_with("&#") {
                    ent[2..ent.len() - 1]
                        .parse::<u32>()
                        .ok()
                        .and_then(char::from_u32)
                } else {
                    None
                };
                if let Some(c) = numeric {
                    out.push(c);
                    rest = &rest[semi + 1..];
                    continue;
                }
            }
            _ => {}
        }
        out.push('&');
        rest = &rest[1..];
    }
    out.push_str(rest);
    out
}

fn child_value(block: &str, local: &str) -> String {
    let mut i = 0usize;
    while i < block.len() {
        i = skip_to_lt(block, i);
        if i >= block.len() {
            return String::new();
        }
        let Some((name, closing, gt)) = tag_local(block, i) else {
            return String::new();
        };
        if !closing && name == local {
            let start = gt + 1;
            let end = block[start..]
                .find('<')
                .map(|p| start + p)
                .unwrap_or(block.len());
            return decode(block[start..end].trim());
        }
        i = gt + 1;
    }
    String::new()
}

fn extract_blocks<'a>(s: &'a str, local: &'a str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < s.len() {
        i = skip_to_lt(s, i);
        if i >= s.len() {
            break;
        }
        let Some((name, closing, gt)) = tag_local(s, i) else {
            i += 1;
            continue;
        };
        if !closing && name == local {
            let start = gt + 1;
            let mut j = start;
            let mut close_gt: Option<usize> = None;
            while j < s.len() {
                j = skip_to_lt(s, j);
                if j >= s.len() {
                    break;
                }
                if let Some((cname, cclosing, cgt)) = tag_local(s, j) {
                    if cclosing && cname == local {
                        close_gt = Some(cgt);
                        break;
                    }
                    j = cgt + 1;
                } else {
                    j += 1;
                }
            }
            if let Some(cgt) = close_gt {
                out.push(&s[start..cgt]);
                i = cgt + 1;
                continue;
            }
        }
        i = gt + 1;
    }
    out
}

fn parse_listing(body: &str) -> Listing {
    let mut objects = Vec::new();
    for blk in extract_blocks(body, "Contents") {
        let key = child_value(blk, "Key");
        if key.is_empty() {
            continue;
        }
        objects.push(S3Object { key });
    }
    let truncated = child_value(body, "IsTruncated").eq_ignore_ascii_case("true");
    let next_token = child_value(body, "NextContinuationToken");
    Listing {
        objects,
        truncated,
        next_token,
    }
}

fn percent_encode(s: &str) -> String {
    let mut out = String::new();
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

fn listing_url(base: &str, prefix: &str, token: &str) -> String {
    let mut url = format!("{base}/?list-type=2&max-keys={MAX_KEYS}");
    if !prefix.is_empty() {
        url.push_str("&prefix=");
        url.push_str(&percent_encode(prefix));
    }
    if !token.is_empty() {
        url.push_str("&continuation-token=");
        url.push_str(&percent_encode(token));
    }
    url
}

fn is_lidar_tile(key: &str) -> bool {
    let k = key.to_ascii_lowercase();
    k.ends_with(".laz")
        || k.ends_with(".las")
        || k.ends_with(".copc.laz")
        || k.ends_with(".tif")
        || k.ends_with(".tiff")
}

fn manifest_lines(base: &str, objects: &[S3Object]) -> Vec<String> {
    let mut out = Vec::new();
    for o in objects {
        if !is_lidar_tile(&o.key) {
            continue;
        }
        out.push(format!("shard {base}/{}", o.key));
    }
    out
}

fn enumerate(base: &str, prefix: &str) -> Result<(Vec<S3Object>, usize), String> {
    let mut objects = Vec::new();
    let mut pages = 0usize;
    let mut token = String::new();
    loop {
        let url = listing_url(base, prefix, &token);
        let body = listing_fetch(&url).ok_or_else(|| {
            format!(
                "listing page {} under prefix '{prefix}' returned void ({url}) — the walk stays incomplete",
                pages + 1
            )
        })?;
        let listing = parse_listing(&body);
        pages += 1;
        objects.extend(listing.objects);
        if !listing.truncated {
            break;
        }
        if listing.next_token.is_empty() {
            return Err(format!(
                "page {pages} under prefix '{prefix}' is truncated without a NextContinuationToken — the walk stays incomplete"
            ));
        }
        token = listing.next_token;
    }
    Ok((objects, pages))
}

fn list_prefixes(base: &str) -> Result<Vec<String>, String> {
    let mut prefixes = Vec::new();
    let mut token = String::new();
    loop {
        let mut url = format!("{base}/?list-type=2&max-keys={MAX_KEYS}&delimiter=%2F");
        if !token.is_empty() {
            url.push_str("&continuation-token=");
            url.push_str(&percent_encode(&token));
        }
        let body = listing_fetch(&url)
            .ok_or_else(|| format!("the top-level prefix listing returned void ({url})"))?;
        for blk in extract_blocks(&body, "CommonPrefixes") {
            let p = child_value(blk, "Prefix");
            if !p.is_empty() {
                prefixes.push(p);
            }
        }
        let truncated = child_value(&body, "IsTruncated").eq_ignore_ascii_case("true");
        if !truncated {
            break;
        }
        let t = child_value(&body, "NextContinuationToken");
        if t.is_empty() {
            break;
        }
        token = t;
    }
    Ok(prefixes)
}

fn parallel_enumerate(
    base: &str,
    prefixes: &[String],
    jobs: usize,
) -> Result<(Vec<S3Object>, usize), String> {
    let prefixes = Arc::new(prefixes.to_vec());
    let cursor = Arc::new(AtomicUsize::new(0));
    let done = Arc::new(AtomicUsize::new(0));
    let objects = Arc::new(Mutex::new(Vec::<S3Object>::new()));
    let pages = Arc::new(AtomicUsize::new(0));
    let failures = Arc::new(Mutex::new(Vec::<String>::new()));
    let mut handles = Vec::new();
    for _ in 0..jobs {
        let prefixes = Arc::clone(&prefixes);
        let cursor = Arc::clone(&cursor);
        let done = Arc::clone(&done);
        let objects = Arc::clone(&objects);
        let pages = Arc::clone(&pages);
        let failures = Arc::clone(&failures);
        let base = base.to_string();
        handles.push(thread::spawn(move || loop {
            let i = cursor.fetch_add(1, Ordering::Relaxed);
            if i >= prefixes.len() {
                break;
            }
            match enumerate(&base, &prefixes[i]) {
                Ok((objs, pg)) => {
                    pages.fetch_add(pg, Ordering::Relaxed);
                    if let Ok(mut o) = objects.lock() {
                        o.extend(objs);
                    }
                }
                Err(e) => {
                    if let Ok(mut f) = failures.lock() {
                        f.push(e);
                    }
                }
            }
            let n = done.fetch_add(1, Ordering::Relaxed) + 1;
            if n % 128 == 0 {
                let p = pages.load(Ordering::Relaxed);
                let read = match objects.lock() {
                    Ok(o) => o.len().to_string(),
                    Err(_) => "unread".to_string(),
                };
                eprintln!(
                    "linz_coverage: {n}/{} prefixes, {p} pages, {read} objects read so far — the walk continues",
                    prefixes.len()
                );
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }
    if let Ok(f) = failures.lock() {
        if let Some(first) = f.first() {
            return Err(format!(
                "{} prefix walk(s) stayed incomplete, first: {first}",
                f.len()
            ));
        }
    }
    let objects = match objects.lock() {
        Ok(mut o) => std::mem::take(&mut *o),
        Err(_) => Vec::new(),
    };
    Ok((objects, pages.load(Ordering::Relaxed)))
}

fn run(args: &[String]) -> Result<(), String> {
    let bucket = match arg_value(args, "--bucket").filter(|b| !b.is_empty()) {
        Some(b) => b,
        None => DEFAULT_BUCKET.to_string(),
    };
    if bucket.contains('/') || bucket.contains(char::is_whitespace) {
        return Err(format!("--bucket {bucket}: not a bucket name — refused"));
    }
    let prefix = match arg_value(args, "--prefix") {
        Some(p) => p,
        None => String::new(),
    };
    let netloc = format!("{bucket}.{S3_REGION_SUFFIX}");
    let base = format!("https://{netloc}");
    let out = match arg_value(args, "--out").filter(|o| !o.is_empty()) {
        Some(o) => o,
        None => format!("data/{netloc}/{MANIFEST_NAME}"),
    };
    let out_name = match Path::new(&out).file_name().and_then(|n| n.to_str()) {
        Some(n) => n.to_string(),
        None => MANIFEST_NAME.to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let jobs = match arg_value(args, "--jobs").and_then(|v| v.parse::<usize>().ok()) {
        Some(j) if j > 0 => j,
        _ => 1,
    };

    let (objects, pages) = if prefix.is_empty() && jobs > 1 {
        let prefixes = list_prefixes(&base)?;
        if prefixes.is_empty() {
            return Err(format!(
                "{bucket}: the top-level listing carries no prefix — nothing to walk"
            ));
        }
        eprintln!(
            "linz_coverage: {} top-level prefixes, {jobs} workers",
            prefixes.len()
        );
        parallel_enumerate(&base, &prefixes, jobs)?
    } else {
        enumerate(&base, &prefix)?
    };

    if pages == 0 || objects.is_empty() {
        return Err(format!(
            "{bucket} prefix '{prefix}': the listing carries no object — nothing manifestiert (0 honored)"
        ));
    }
    let lines = manifest_lines(&base, &objects);
    if lines.is_empty() {
        return Err(format!(
            "{bucket} prefix '{prefix}': {} objects over {pages} pages, none a lidar tile — nothing manifestiert (0 honored)",
            objects.len()
        ));
    }

    let mut text = String::with_capacity(lines.len() * 96);
    for line in &lines {
        text.push_str(line);
        text.push('\n');
    }
    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    fs::write(&out, &text).map_err(|e| format!("write {out} returned void: {e}"))?;
    let back = fs::read_to_string(&out).map_err(|_| format!("{out} re-read returned void"))?;
    if back.lines().count() != lines.len() {
        return Err(format!(
            "{out} roundtrip lists {} of {} lines — the manifest stays unverified",
            back.lines().count(),
            lines.len()
        ));
    }
    eprintln!(
        "linz_coverage: {bucket} prefix '{prefix}': {pages} pages, {} objects, {} lidar tiles -> {out}",
        objects.len(),
        lines.len()
    );

    println!("url {CDN_BASE}/{netloc}/{out_name}");
    println!("origin {base}/?list-type=2");
    println!("compiler {COMPILER}");
    println!("format linz_coverage");

    if ci_mode && !upload_release(&netloc, &out) {
        return Err(format!("{out} did not reach the CDN"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("linz_coverage: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/"><Name>nz-elevation</Name><Prefix>taranaki/taranaki_2021/dem_1m/2193/</Prefix><NextContinuationToken>1G2c0Ar8GsGpr1HsPS3mU2y0LydDppu50c2ROz+/ADu0aY7ybXK2DXvL9LUG+5bAcu1K2d9hfWiMmdc6X/6qkLRJh52Bc1W/ay138Mz6guRc=</NextContinuationToken><KeyCount>6</KeyCount><MaxKeys>6</MaxKeys><IsTruncated>true</IsTruncated><Contents><Key>taranaki/taranaki_2021/dem_1m/2193/BG30_10000_0505.json</Key><LastModified>2024-11-28T21:20:11.000Z</LastModified><ETag>&quot;9189bf2b693ef88d284e72b41a8ef259&quot;</ETag><Size>1165</Size><StorageClass>STANDARD</StorageClass></Contents><Contents><Key>taranaki/taranaki_2021/dem_1m/2193/BG30_10000_0505.tiff</Key><LastModified>2024-02-28T02:25:34.000Z</LastModified><ETag>&quot;987404cce0d6f8e891878482cc5d058d&quot;</ETag><Size>695442</Size><StorageClass>INTELLIGENT_TIERING</StorageClass></Contents><Contents><Key>taranaki/taranaki_2021/dem_1m/2193/BG31_10000_0302.json</Key><LastModified>2024-11-28T21:20:11.000Z</LastModified><ETag>&quot;442f82341ce9af5ae4d2abc1e79af625&quot;</ETag><Size>1166</Size><StorageClass>STANDARD</StorageClass></Contents><Contents><Key>taranaki/taranaki_2021/dem_1m/2193/BG31_10000_0302.tiff</Key><LastModified>2024-02-28T02:25:35.000Z</LastModified><ETag>&quot;8d03f11e88286bc3bb27b59707db9ef0-2&quot;</ETag><Size>19752936</Size><StorageClass>INTELLIGENT_TIERING</StorageClass></Contents><Contents><Key>taranaki/taranaki_2021/dem_1m/2193/BG31_10000_0303.json</Key><LastModified>2024-11-28T21:20:11.000Z</LastModified><ETag>&quot;f0e89504676e5f849dddca844e5542c0&quot;</ETag><Size>1166</Size><StorageClass>STANDARD</StorageClass></Contents><Contents><Key>taranaki/taranaki_2021/dem_1m/2193/BG31_10000_0303.tiff</Key><LastModified>2024-02-28T02:25:35.000Z</LastModified><ETag>&quot;946ae24ca53c80bcc54e22f9f07aaf6d-4&quot;</ETag><Size>39441532</Size><StorageClass>INTELLIGENT_TIERING</StorageClass></Contents></ListBucketResult>"#;

    #[test]
    fn lidar_tiles_become_shard_lines_on_an_absolute_url() {
        let listing = parse_listing(PAGE);
        assert!(listing.truncated);
        assert_eq!(
            listing.next_token,
            "1G2c0Ar8GsGpr1HsPS3mU2y0LydDppu50c2ROz+/ADu0aY7ybXK2DXvL9LUG+5bAcu1K2d9hfWiMmdc6X/6qkLRJh52Bc1W/ay138Mz6guRc="
        );
        assert_eq!(listing.objects.len(), 6);
        let base = "https://nz-elevation.s3-ap-southeast-2.amazonaws.com";
        let lines = manifest_lines(base, &listing.objects);
        assert_eq!(lines.len(), 3);
        assert_eq!(
            lines[0],
            "shard https://nz-elevation.s3-ap-southeast-2.amazonaws.com/taranaki/taranaki_2021/dem_1m/2193/BG30_10000_0505.tiff"
        );
        assert_eq!(
            lines[1],
            "shard https://nz-elevation.s3-ap-southeast-2.amazonaws.com/taranaki/taranaki_2021/dem_1m/2193/BG31_10000_0302.tiff"
        );
        assert_eq!(
            lines[2],
            "shard https://nz-elevation.s3-ap-southeast-2.amazonaws.com/taranaki/taranaki_2021/dem_1m/2193/BG31_10000_0303.tiff"
        );
    }

    #[test]
    fn point_cloud_extensions_are_lidar_tiles() {
        assert!(is_lidar_tile("northland/BN32_0500_068098.laz"));
        assert!(is_lidar_tile("northland/BN32_0500_068098.copc.laz"));
        assert!(is_lidar_tile("northland/BN32_0500_068098.las"));
    }

    #[test]
    fn non_tile_objects_are_left_out() {
        let body = r#"<ListBucketResult><Contents><Key>taranaki/taranaki_2021/dem_1m/2193/BG30_10000_0505.json</Key><Size>1165</Size></Contents><Contents><Key>LICENSE</Key><Size>18657</Size></Contents><Contents><Key>catalog.json</Key><Size>66450</Size></Contents></ListBucketResult>"#;
        let listing = parse_listing(body);
        let lines = manifest_lines(
            "https://nz-elevation.s3-ap-southeast-2.amazonaws.com",
            &listing.objects,
        );
        assert!(lines.is_empty());
    }
}
