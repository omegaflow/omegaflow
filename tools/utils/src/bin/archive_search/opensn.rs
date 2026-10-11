use crate::json::{self, Json};
use crate::net::urlencode;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const OSC_REPOS: &[&str] = &[
    "sne-pre-1990",
    "sne-1990-1999",
    "sne-2000-2004",
    "sne-2005-2009",
    "sne-2010-2014",
    "sne-2015-2019",
    "sne-2020-2024",
];

const USER_AGENT: &str = "User-Agent: omegaflow-archive-search";

pub fn opensn_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    let needle = query.trim().to_lowercase();
    if needle.is_empty() {
        return vec!["absent — opensn carries no query".to_string()];
    }
    let mut hits: Vec<(String, String)> = Vec::new();
    let mut pending: Vec<String> = Vec::new();
    for repo in OSC_REPOS {
        let url =
            format!("https://api.github.com/repos/astrocatalogs/{repo}/git/trees/HEAD?recursive=1");
        match curl_get(&url, token) {
            Some((200, body)) => {
                for path in tree_paths(&body) {
                    if !object_path(&path) {
                        continue;
                    }
                    if name_of(&path).to_lowercase().contains(&needle) {
                        hits.push(((*repo).to_string(), path));
                    }
                }
                if tree_truncated(&body) {
                    pending.push(format!(
                        "pending — {repo}: the GitHub tree API truncated the index"
                    ));
                }
            }
            Some((code, _)) => pending.push(format!("pending — opensn index {repo} HTTP {code}")),
            None => pending.push(format!("pending — opensn index {repo}: no network")),
        }
    }
    if hits.is_empty() {
        let mut out = vec![format!(
            "absent — the Open Supernova Catalog carries no object: {query}"
        )];
        out.extend(pending);
        return out;
    }
    hits.sort();
    let mut full: Vec<String> = Vec::new();
    for (repo, path) in &hits {
        full.push(format!(
            "url {}\tname: {}",
            blob_url(repo, path),
            name_of(path)
        ));
    }
    let mut out: Vec<String> = Vec::new();
    for (repo, path) in hits.iter().take(max) {
        let mut line = format!("url {}\tname: {}", blob_url(repo, path), name_of(path));
        match curl_get(&raw_url(repo, path), "") {
            Some((200, body)) => match json::parse(&body) {
                Some(v) => match object_detail(&v) {
                    Some(detail) => line.push_str(&detail),
                    None => line.push_str("\tdetail: absent — the OSC object carries no fields"),
                },
                None => line.push_str("\tdetail: pending — the OSC object carries no JSON"),
            },
            Some((code, _)) => line.push_str(&format!("\tdetail: pending HTTP {code}")),
            None => line.push_str("\tdetail: pending — no network"),
        }
        out.push(line);
    }
    if hits.len() > max {
        out.push(format!("end: {} of {} hits shown", max, hits.len()));
    }
    if let Some(path) = write_full_file(&needle, &full) {
        out.push(format!(
            "full: {} lines -> {}  [read the full file to its end — the inline hits are the first {max}]",
            full.len(),
            path
        ));
    }
    out.extend(pending);
    out
}

fn object_path(path: &str) -> bool {
    path.ends_with(".json") && !path.contains('/') && !path.starts_with('.')
}

fn name_of(path: &str) -> &str {
    path.strip_suffix(".json").unwrap_or(path)
}

fn blob_url(repo: &str, path: &str) -> String {
    format!(
        "https://github.com/astrocatalogs/{repo}/blob/HEAD/{}",
        urlencode(path)
    )
}

fn raw_url(repo: &str, path: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/astrocatalogs/{repo}/HEAD/{}",
        urlencode(path)
    )
}

fn curl_get(url: &str, token: &str) -> Option<(i32, String)> {
    let mut cmd = Command::new("curl");
    cmd.args(["-sS", "-g", "-L", "--compressed", "--max-time", "60"]);
    cmd.args(["-H", USER_AGENT]);
    if !token.is_empty() {
        cmd.arg("-H").arg(format!("Authorization: Bearer {token}"));
    }
    cmd.args(["-o", "-", "-w", "\n%{http_code}"]);
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .parse::<i32>()
        .ok()?;
    Some((code, String::from_utf8_lossy(&stdout[..idx]).to_string()))
}

fn tree_paths(body: &str) -> Vec<String> {
    let b = body.as_bytes();
    let needle = b"\"path\"";
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + needle.len() <= b.len() {
        if &b[i..i + needle.len()] == needle {
            if let Some((value, next)) = quoted_after_colon(b, i + needle.len()) {
                out.push(value);
                i = next;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn quoted_after_colon(b: &[u8], from: usize) -> Option<(String, usize)> {
    let mut j = from;
    while j < b.len() && matches!(b[j], b' ' | b'\t' | b'\n' | b'\r') {
        j += 1;
    }
    if b.get(j) != Some(&b':') {
        return None;
    }
    j += 1;
    while j < b.len() && matches!(b[j], b' ' | b'\t' | b'\n' | b'\r') {
        j += 1;
    }
    if b.get(j) != Some(&b'"') {
        return None;
    }
    j += 1;
    let mut value = String::new();
    while j < b.len() {
        match b[j] {
            b'"' => return Some((value, j + 1)),
            b'\\' => {
                j += 1;
                match b.get(j)? {
                    b'"' => value.push('"'),
                    b'\\' => value.push('\\'),
                    b'/' => value.push('/'),
                    b'n' => value.push('\n'),
                    b't' => value.push('\t'),
                    b'r' => value.push('\r'),
                    b'b' => value.push('\u{8}'),
                    b'f' => value.push('\u{c}'),
                    b'u' => {
                        let hex = std::str::from_utf8(b.get(j + 1..j + 5)?).ok()?;
                        let cp = u16::from_str_radix(hex, 16).ok()?;
                        value.push(char::from_u32(cp as u32)?);
                        j += 4;
                    }
                    other => value.push(*other as char),
                }
                j += 1;
            }
            other => {
                let width = utf8_width(other);
                if width == 0 || j + width > b.len() {
                    return None;
                }
                value.push_str(std::str::from_utf8(&b[j..j + width]).ok()?);
                j += width;
            }
        }
    }
    None
}

fn tree_truncated(body: &str) -> bool {
    let b = body.as_bytes();
    let needle = b"\"truncated\"";
    let mut i = 0usize;
    while i + needle.len() <= b.len() {
        if &b[i..i + needle.len()] == needle {
            let mut j = i + needle.len();
            while j < b.len() && matches!(b[j], b' ' | b'\t' | b'\n' | b'\r') {
                j += 1;
            }
            if b.get(j) != Some(&b':') {
                i += needle.len();
                continue;
            }
            j += 1;
            while j < b.len() && matches!(b[j], b' ' | b'\t' | b'\n' | b'\r') {
                j += 1;
            }
            return b[j..].starts_with(b"true");
        }
        i += 1;
    }
    false
}

fn utf8_width(b: u8) -> usize {
    if b < 0x80 {
        1
    } else if b >> 5 == 0b110 {
        2
    } else if b >> 4 == 0b1110 {
        3
    } else if b >> 3 == 0b11110 {
        4
    } else {
        0
    }
}

fn values(obj: &Json, field: &str) -> Vec<String> {
    let Some(entries) = obj.get(field).and_then(|f| f.as_arr()) else {
        return Vec::new();
    };
    let mut out: Vec<String> = Vec::new();
    for entry in entries {
        let Some(value) = entry
            .get("value")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        if !out.iter().any(|v| v == value) {
            out.push(value.to_string());
        }
    }
    out
}

fn object_detail(v: &Json) -> Option<String> {
    let inner = match v {
        Json::Obj(map) => map.values().next()?,
        _ => return None,
    };
    let mut parts: Vec<String> = Vec::new();
    for (field, label) in [
        ("claimedtype", "type"),
        ("redshift", "redshift"),
        ("ra", "ra"),
        ("dec", "dec"),
        ("discoverdate", "discoverdate"),
    ] {
        let found = values(inner, field);
        if !found.is_empty() {
            parts.push(format!("{label}: {}", found.join(", ")));
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(format!("\t{}", parts.join("\t")))
    }
}

fn write_full_file(query: &str, lines: &[String]) -> Option<String> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let slug: String = query
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .take(40)
        .collect();
    let path = std::env::temp_dir().join(format!("omegaflow_opensn_{stamp}_{slug}.txt"));
    std::fs::write(&path, lines.join("\n")).ok()?;
    Some(path.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_paths_reads_compact_and_spaced_paths() {
        let body = r#"{"sha":"x","tree":[{"path":"ASASSN-20ao.json","type":"blob"},{"path": "SN2020a.json","type":"blob"}],"truncated":false}"#;
        assert_eq!(
            tree_paths(body),
            vec!["ASASSN-20ao.json".to_string(), "SN2020a.json".to_string()]
        );
    }

    #[test]
    fn tree_paths_unescapes_a_quoted_path() {
        let body = r#"{"tree":[{"path":"a\"b.json"}]}"#;
        assert_eq!(tree_paths(body), vec!["a\"b.json".to_string()]);
    }

    #[test]
    fn tree_truncated_reads_the_flag() {
        assert!(tree_truncated(r#"{"tree":[],"truncated":true}"#));
        assert!(!tree_truncated(r#"{"tree":[],"truncated":false}"#));
        assert!(!tree_truncated(r#"{"tree":[]}"#));
    }

    #[test]
    fn object_path_keeps_root_json_only() {
        assert!(object_path("ASASSN-20ao.json"));
        assert!(!object_path(".gitignore"));
        assert!(!object_path("sub/x.json"));
        assert!(!object_path("README.md"));
    }

    #[test]
    fn object_detail_reads_type_and_redshift() {
        let body = r#"{"ASASSN-20ao":{"name":"ASASSN-20ao","claimedtype":[{"value":"Candidate"}],"redshift":[{"value":"0.024934"}],"discoverdate":[{"value":"2020/01/15"}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            object_detail(&v),
            Some("\ttype: Candidate\tredshift: 0.024934\tdiscoverdate: 2020/01/15".to_string())
        );
    }

    #[test]
    fn object_detail_joins_distinct_values() {
        let body = r#"{"X":{"claimedtype":[{"value":"Ia"},{"value":"Ia"},{"value":"SN Ia"}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(object_detail(&v), Some("\ttype: Ia, SN Ia".to_string()));
    }

    #[test]
    fn object_detail_is_absent_without_fields() {
        let v = json::parse(r#"{"X":{}}"#).unwrap();
        assert_eq!(object_detail(&v), None);
    }
}
