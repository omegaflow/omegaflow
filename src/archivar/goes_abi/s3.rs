use crate::archivar::fetch_raw_bytes;

const REQUEST_TTL_S: u64 = 1 << 9;
const PAGE_CAP: usize = 1 << 6;

#[derive(Clone, Debug)]
pub struct S3Key {
    pub key: String,
    pub size: u64,
}

#[derive(Clone, Debug)]
pub struct S3List {
    pub keys: Vec<S3Key>,
    pub prefixes: Vec<String>,
}

#[derive(Debug)]
struct Page {
    keys: Vec<S3Key>,
    prefixes: Vec<String>,
    truncated: bool,
    next_marker: Option<String>,
}

fn block_ranges<'a>(body: &'a str, tag: &str) -> Vec<&'a str> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(s) = body[from..].find(&open) {
        let start = from + s + open.len();
        let Some(e) = body[start..].find(&close) else {
            break;
        };
        out.push(&body[start..start + e]);
        from = start + e + close.len();
    }
    out
}

fn value_of(block: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let s = block.find(&open)? + open.len();
    let e = block[s..].find(&close)?;
    Some(block[s..s + e].to_string())
}

fn first_value(body: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let s = body.find(&open)? + open.len();
    let e = body[s..].find(&close)?;
    Some(body[s..s + e].to_string())
}

fn parse_list_body(body: &str) -> Option<Page> {
    let mut keys = Vec::new();
    for c in block_ranges(body, "Contents") {
        let Some(key) = value_of(c, "Key") else {
            continue;
        };
        let Some(size) = value_of(c, "Size").and_then(|s| s.parse::<u64>().ok()) else {
            continue;
        };
        keys.push(S3Key { key, size });
    }
    let prefixes: Vec<String> = block_ranges(body, "CommonPrefixes")
        .iter()
        .filter_map(|b| value_of(b, "Prefix"))
        .collect();
    let truncated = first_value(body, "IsTruncated").as_deref() == Some("true");
    let next_marker = first_value(body, "NextMarker").filter(|s| !s.is_empty());
    Some(Page {
        keys,
        prefixes,
        truncated,
        next_marker,
    })
}

pub fn list(bucket: &str, prefix: &str) -> Option<S3List> {
    let mut keys: Vec<S3Key> = Vec::new();
    let mut prefixes: Vec<String> = Vec::new();
    let mut marker = String::new();
    let mut pages = 0usize;
    loop {
        let mut url = format!("https://{bucket}/?prefix={prefix}&delimiter=/&max-keys=1000");
        if !marker.is_empty() {
            url.push_str("&marker=");
            url.push_str(&marker);
        }
        let body = fetch_raw_bytes(&url, REQUEST_TTL_S)?;
        let text = std::str::from_utf8(&body).ok()?;
        let page = parse_list_body(text)?;
        keys.extend(page.keys);
        prefixes.extend(page.prefixes);
        if !page.truncated {
            break;
        }
        match page.next_marker {
            Some(m) if m != marker => {
                pages += 1;
                if pages > PAGE_CAP {
                    break;
                }
                marker = m;
            }
            _ => break,
        }
    }
    keys.sort_by(|a, b| a.key.cmp(&b.key));
    keys.dedup_by(|a, b| a.key == b.key);
    prefixes.sort();
    prefixes.dedup();
    Some(S3List { keys, prefixes })
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/"><Name>noaa-goes16</Name><Prefix>ABI-L1b-RadC/2025/097/18/</Prefix><Marker></Marker><MaxKeys>1000</MaxKeys><Delimiter>/</Delimiter><IsTruncated>false</IsTruncated><Contents><Key>ABI-L1b-RadC/2025/097/18/OR_ABI-L1b-RadC-M6C01_G16_s20250971801174_e20250971803547_c20250971803585.nc</Key><LastModified>2025-04-07T18:04:18.000Z</LastModified><Size>12280502</Size></Contents><Contents><Key>ABI-L1b-RadC/2025/097/18/OR_ABI-L1b-RadC-M6C02_G16_s20250971801174_e20250971803546_c20250971803586.nc</Key><LastModified>2025-04-07T18:04:18.000Z</LastModified><Size>9037330</Size></Contents><CommonPrefixes><Prefix>ABI-L1b-RadC/2025/097/19/</Prefix></CommonPrefixes></ListBucketResult>"#;

    #[test]
    fn parse_list_body_reads_keys_sizes_and_prefixes() {
        let page = parse_list_body(BODY).expect("listing parses");
        assert_eq!(page.keys.len(), 2);
        assert_eq!(page.keys[0].size, 12280502);
        assert!(
            page.keys[0]
                .key
                .ends_with("M6C01_G16_s20250971801174_e20250971803547_c20250971803585.nc")
        );
        assert_eq!(page.keys[1].size, 9037330);
        assert!(
            page.keys[1]
                .key
                .ends_with("M6C02_G16_s20250971801174_e20250971803546_c20250971803586.nc")
        );
        assert_eq!(page.prefixes, vec!["ABI-L1b-RadC/2025/097/19/".to_string()]);
        assert!(!page.truncated);
        assert_eq!(page.next_marker, None);
    }

    #[test]
    fn parse_list_body_reads_truncation_marker() {
        let body = r#"<ListBucketResult><Name>b</Name><Prefix>p</Prefix><IsTruncated>true</IsTruncated><NextMarker>ABI-L1b-RadC/2025/097/19/</NextMarker><Contents><Key>k</Key><Size>1</Size></Contents></ListBucketResult>"#;
        let page = parse_list_body(body).expect("listing parses");
        assert!(page.truncated);
        assert_eq!(
            page.next_marker.as_deref(),
            Some("ABI-L1b-RadC/2025/097/19/")
        );
    }

    #[test]
    fn parse_list_body_skips_contents_without_size() {
        let body = r#"<ListBucketResult><Name>b</Name><Prefix>p</Prefix><IsTruncated>false</IsTruncated><Contents><Key>k</Key></Contents><Contents><Key>m</Key><Size>7</Size></Contents></ListBucketResult>"#;
        let page = parse_list_body(body).expect("listing parses");
        assert_eq!(page.keys.len(), 1);
        assert_eq!(page.keys[0].key, "m");
        assert_eq!(page.keys[0].size, 7);
    }
}
