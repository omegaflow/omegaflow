use std::env;
use std::fs;
use std::process::Command;

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        None
    }
}

fn extract(block: &str, tag: &str) -> String {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    match block.find(&open) {
        Some(p) => {
            let a = &block[p + open.len()..];
            match a.find(&close) {
                Some(e) => a[..e].trim().to_string(),
                None => String::new(),
            }
        }
        None => String::new(),
    }
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
                let r = match ent {
                    "&amp;" => Some("&"),
                    "&lt;" => Some("<"),
                    "&gt;" => Some(">"),
                    "&quot;" => Some("\""),
                    "&apos;" => Some("'"),
                    _ => None,
                };
                if let Some(r) = r {
                    out.push_str(r);
                    rest = &rest[semi + 1..];
                    continue;
                }
                if ent.len() > 3 && (ent.starts_with("&#x") || ent.starts_with("&#X")) {
                    if let Ok(code) = u32::from_str_radix(&ent[3..ent.len() - 1], 16) {
                        if let Some(c) = char::from_u32(code) {
                            out.push(c);
                            rest = &rest[semi + 1..];
                            continue;
                        }
                    }
                } else if ent.len() > 2 && ent.starts_with("&#") {
                    if let Ok(code) = ent[2..ent.len() - 1].parse::<u32>() {
                        if let Some(c) = char::from_u32(code) {
                            out.push(c);
                            rest = &rest[semi + 1..];
                            continue;
                        }
                    }
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

fn root_value(s: &str, local: &str) -> String {
    child_value(s, local)
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

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_usize(args: &[String], name: &str) -> Option<usize> {
    arg_value(args, name).and_then(|v| v.parse::<usize>().ok())
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

struct S3Object {
    key: String,
    size: Option<u64>,
    last_modified: String,
    etag: String,
}

struct Page {
    objects: Vec<S3Object>,
    dirs: Vec<String>,
    truncated: bool,
    next_token: String,
}

fn parse_page(body: &str) -> Option<Page> {
    let mut objects = Vec::new();
    for blk in extract_blocks(body, "Contents") {
        let key = child_value(blk, "Key");
        if key.is_empty() {
            continue;
        }
        objects.push(S3Object {
            key,
            size: child_value(blk, "Size").parse::<u64>().ok(),
            last_modified: child_value(blk, "LastModified"),
            etag: child_value(blk, "ETag"),
        });
    }
    let dirs: Vec<String> = extract_blocks(body, "CommonPrefixes")
        .iter()
        .map(|b| child_value(b, "Prefix"))
        .filter(|p| !p.is_empty())
        .collect();
    let truncated = root_value(body, "IsTruncated").eq_ignore_ascii_case("true");
    let mut next_token = root_value(body, "NextContinuationToken");
    if next_token.is_empty() {
        next_token = root_value(body, "NextMarker");
    }
    Some(Page {
        objects,
        dirs,
        truncated,
        next_token,
    })
}

fn page(base: &str, prefix: &str, delimiter: &str, token: &str, max_keys: usize) -> Option<Page> {
    let mut url = format!("{}?list-type=2&max-keys={}", base, max_keys);
    if !prefix.is_empty() {
        url.push_str("&prefix=");
        url.push_str(prefix);
    }
    if !delimiter.is_empty() {
        url.push_str("&delimiter=");
        url.push_str(delimiter);
    }
    if !token.is_empty() {
        url.push_str("&continuation-token=");
        url.push_str(&percent_encode(token));
    }
    let body = curl(&url)?;
    parse_page(&body)
}

struct Walk {
    objects: usize,
    dirs: usize,
}

fn walk(
    base: &str,
    prefix: &str,
    delimiter: &str,
    depth: usize,
    cap: usize,
    max_keys: usize,
    out: &mut String,
) -> Walk {
    let mut w = Walk {
        objects: 0,
        dirs: 0,
    };
    let mut token = String::new();
    loop {
        let p = match page(base, prefix, delimiter, &token, max_keys) {
            Some(p) => p,
            None => break,
        };
        w.dirs += p.dirs.len();
        for o in p.objects {
            if cap > 0 && w.objects >= cap {
                return w;
            }
            let size = match o.size {
                Some(s) => s.to_string(),
                None => String::new(),
            };
            out.push_str(&format!(
                "{}|{}|{}|{}\n",
                o.key, size, o.last_modified, o.etag
            ));
            w.objects += 1;
        }
        if depth > 0 {
            for dir in &p.dirs {
                if *dir == prefix {
                    continue;
                }
                let sub = walk(
                    base,
                    dir,
                    delimiter,
                    depth - 1,
                    cap.saturating_sub(w.objects),
                    max_keys,
                    out,
                );
                w.objects += sub.objects;
                w.dirs += sub.dirs;
                if cap > 0 && w.objects >= cap {
                    return w;
                }
            }
        }
        if !p.truncated {
            break;
        }
        if p.next_token.is_empty() {
            break;
        }
        token = p.next_token;
    }
    w
}

fn run_s3(args: &[String]) {
    let Some(base) = arg_value(args, "--s3") else {
        eprintln!("xml: --s3 <bucket-listing-URL> absent");
        std::process::exit(1);
    };
    let prefix = match arg_value(args, "--prefix") {
        Some(v) => v,
        None => String::new(),
    };
    let delimiter = match arg_value(args, "--delimiter") {
        Some(v) => v,
        None => String::from("/"),
    };
    let depth = match arg_usize(args, "--depth") {
        Some(v) => v,
        None => 0,
    };
    let cap = match arg_usize(args, "--cap") {
        Some(v) => v,
        None => 0,
    };
    let max_keys = match arg_usize(args, "--max-keys") {
        Some(v) => v,
        None => 1000,
    };
    let out = arg_value(args, "--out");
    let mut buf = String::from("#key|size_bytes|last_modified|etag\n");
    let w = walk(&base, &prefix, &delimiter, depth, cap, max_keys, &mut buf);
    if let Some(p) = out.as_deref() {
        if fs::write(p, &buf).is_err() {
            eprintln!("write {} returned void", p);
            std::process::exit(1);
        }
    }
    eprintln!(
        "xml: {} objects, {} common prefixes under {} prefix {} delimiter {}",
        w.objects, w.dirs, base, prefix, delimiter
    );
    if w.objects == 0 && w.dirs == 0 {
        eprintln!(
            "xml: the bucket listing carried no objects at prefix {} — nothing fabricated",
            prefix
        );
        std::process::exit(1);
    }
    if out.is_none() {
        print!("{}", buf);
    } else {
        for line in buf.lines().skip(1).take(10) {
            println!("{}", line);
        }
    }
    if cap > 0 && w.objects >= cap {
        eprintln!("xml: cap {} reached — the walk is partial", cap);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--s3") {
        run_s3(&args);
        return;
    }
    let mut root: Option<String> = None;
    let mut record = String::from("repository");
    let mut id_tag = String::from("id");
    let mut title_tag = String::from("name");
    let mut out: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                root = args.get(i + 1).cloned();
                i += 1;
            }
            "--record" => {
                record = args.get(i + 1).cloned().unwrap_or(record);
                i += 1;
            }
            "--id" => {
                id_tag = args.get(i + 1).cloned().unwrap_or(id_tag);
                i += 1;
            }
            "--title" => {
                title_tag = args.get(i + 1).cloned().unwrap_or(title_tag);
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let Some(root) = root else {
        eprintln!("--root absent");
        std::process::exit(1);
    };
    let Some(body) = curl(&root) else {
        eprintln!("xml: {} returned void", root);
        std::process::exit(1);
    };
    let open = format!("<{}>", record);
    let close = format!("</{}>", record);
    let mut buf = String::new();
    let mut total = 0usize;
    let mut rest = body.as_str();
    while let Some(rp) = rest.find(&open) {
        let after = &rest[rp + open.len()..];
        let Some(re) = after.find(&close) else {
            break;
        };
        let block = &after[..re];
        let id = extract(block, &id_tag);
        let title = extract(block, &title_tag);
        if !id.is_empty() {
            buf.push_str(&format!("{} | {}\n", id, title));
            total += 1;
        }
        rest = &after[re..];
    }
    if let Some(path) = out {
        let _ = fs::write(&path, &buf);
        eprintln!("xml: {} records → {}", total, path);
    } else {
        eprintln!("xml: {} records (--out absent)", total);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const S3_PAGE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/"><Name>noaa-cdr-gridsat-b1-pds</Name><Prefix></Prefix><NextContinuationToken>1kCcAYvwLqSi+UgU+YF+HFT5Ld0FCdAWvsneLm3qYT/QmMOHdluseshBA08G+Xutwha09yoK0Z8GoZFAaFbCRfEQ/c+rS0pC4OiPTWDrw5+Y=</NextContinuationToken><KeyCount>3</KeyCount><MaxKeys>3</MaxKeys><IsTruncated>true</IsTruncated><Contents><Key>data/1980/GRIDSAT-B1.1980.01.01.00.v02r01.nc</Key><LastModified>2024-03-30T03:07:25.000Z</LastModified><ETag>&quot;dbaba783d1a506460781524582e7da05&quot;</ETag><Size>3836905</Size><StorageClass>INTELLIGENT_TIERING</StorageClass></Contents><Contents><Key>data/1980/GRIDSAT-B1.1980.01.01.03.v02r01.nc</Key><LastModified>2024-03-29T22:44:42.000Z</LastModified><ETag>&quot;eb97ace7503101cbd4ff5ab42d1aa43f&quot;</ETag><Size>18147782</Size><StorageClass>INTELLIGENT_TIERING</StorageClass></Contents><Contents><Key>data/1980/a&amp;b.nc</Key><LastModified>2024-03-30T09:11:59.000Z</LastModified><ETag>&quot;1d90dc56f107d46bd21dbaa7b63ac1f8&quot;</ETag><Size>17969468</Size><StorageClass>INTELLIGENT_TIERING</StorageClass></Contents></ListBucketResult>"#;

    const S3_DIRS: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/"><Name>noaa-goes16</Name><Prefix></Prefix><KeyCount>2</KeyCount><MaxKeys>6</MaxKeys><Delimiter>/</Delimiter><IsTruncated>false</IsTruncated><CommonPrefixes><Prefix>ABI-L1b-RadC/</Prefix></CommonPrefixes><CommonPrefixes><Prefix>ABI-L1b-RadF/</Prefix></CommonPrefixes></ListBucketResult>"#;

    #[test]
    fn s3_page_objects_sizes_tokens() {
        let p = parse_page(S3_PAGE).unwrap();
        assert_eq!(p.objects.len(), 3);
        assert_eq!(
            p.objects[0].key,
            "data/1980/GRIDSAT-B1.1980.01.01.00.v02r01.nc"
        );
        assert_eq!(p.objects[0].size, Some(3836905));
        assert_eq!(p.objects[0].last_modified, "2024-03-30T03:07:25.000Z");
        assert_eq!(p.objects[0].etag, "\"dbaba783d1a506460781524582e7da05\"");
        assert!(p.truncated);
        assert!(p.next_token.starts_with("1kCcAYvw"));
        assert_eq!(p.dirs.len(), 0);
    }

    #[test]
    fn s3_key_entities_decode() {
        let p = parse_page(S3_PAGE).unwrap();
        assert_eq!(p.objects[2].key, "data/1980/a&b.nc");
        assert_eq!(p.objects[2].size, Some(17969468));
    }

    #[test]
    fn s3_common_prefixes_list_dirs() {
        let p = parse_page(S3_DIRS).unwrap();
        assert_eq!(p.objects.len(), 0);
        assert_eq!(p.dirs, vec!["ABI-L1b-RadC/", "ABI-L1b-RadF/"]);
        assert!(!p.truncated);
        assert!(p.next_token.is_empty());
    }

    #[test]
    fn s3_page_with_namespace_prefix_parses_local_names() {
        let body = r#"<s3:ListBucketResult xmlns:s3="http://s3.amazonaws.com/doc/2006-03-01/"><s3:IsTruncated>true</s3:IsTruncated><s3:NextContinuationToken>tok==</s3:NextContinuationToken><s3:Contents><s3:Key>a.nc</s3:Key><s3:Size>42</s3:Size></s3:Contents></s3:ListBucketResult>"#;
        let p = parse_page(body).unwrap();
        assert_eq!(p.objects.len(), 1);
        assert_eq!(p.objects[0].key, "a.nc");
        assert_eq!(p.objects[0].size, Some(42));
        assert!(p.truncated);
        assert_eq!(p.next_token, "tok==");
    }
}
