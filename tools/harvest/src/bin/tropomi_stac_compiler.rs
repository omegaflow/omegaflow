use omegaflow::cdn::upload_release;
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "meeo-s5p.s3.amazonaws.com";
const BASE: &str = "https://meeo-s5p.s3.amazonaws.com/";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("-m")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "tropomi_stac_compiler: {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn tag<'a>(chunk: &'a str, name: &str) -> Option<&'a str> {
    let open = format!("<{name}>");
    let close = format!("</{name}>");
    let start = chunk.find(&open)? + open.len();
    let rest = &chunk[start..];
    let end = rest.find(&close)?;
    Some(&rest[..end])
}

fn unescape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(pos) = rest.find('&') {
        out.push_str(&rest[..pos]);
        let tail = &rest[pos..];
        let Some(end) = tail.find(';') else {
            out.push_str(tail);
            return out;
        };
        let entity = &tail[1..end];
        let decoded = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ => None,
        };
        match decoded {
            Some(c) => out.push(c),
            None => out.push_str(&tail[..=end]),
        }
        rest = &tail[end + 1..];
    }
    out.push_str(rest);
    out
}

fn encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

fn json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn page_url(prefix: &str, max_keys: usize, token: Option<&str>) -> String {
    let mut url = format!("{BASE}?list-type=2&max-keys={max_keys}");
    if !prefix.is_empty() {
        url.push_str(&format!("&prefix={}", encode(prefix)));
    }
    if let Some(token) = token {
        url.push_str(&format!("&continuation-token={}", encode(token)));
    }
    url
}

struct Entry {
    key: String,
    last_modified: String,
    size: u64,
}

fn parse_page(xml: &str) -> Vec<Entry> {
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(start) = rest.find("<Contents>") {
        let after = &rest[start + "<Contents>".len()..];
        let Some(end) = after.find("</Contents>") else {
            break;
        };
        let block = &after[..end];
        let key = match tag(block, "Key") {
            Some(k) => unescape(k),
            None => continue,
        };
        if !key.is_empty() {
            let last_modified = match tag(block, "LastModified") {
                Some(v) => unescape(v),
                None => "absent".to_string(),
            };
            let size = match tag(block, "Size").and_then(|s| s.trim().parse::<u64>().ok()) {
                Some(v) => v,
                None => continue,
            };
            out.push(Entry {
                key,
                last_modified,
                size,
            });
        }
        rest = &after[end + "</Contents>".len()..];
    }
    out
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let prefix = match arg_value(args, "--prefix") {
        Some(v) => v,
        None => String::new(),
    };
    let max_keys: usize = match arg_value(args, "--max-keys") {
        Some(v) => v
            .parse()
            .map_err(|_| "--max-keys is not an integer".to_string())?,
        None => 1000,
    };
    if max_keys == 0 || max_keys > 1000 {
        return Err("--max-keys must lie in 1..=1000".to_string());
    }
    let max_pages: usize = match arg_value(args, "--max-pages") {
        Some(v) => v
            .parse()
            .map_err(|_| "--max-pages is not an integer".to_string())?,
        None => usize::MAX,
    };
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/s5p_index.json"),
    };

    let mut entries: Vec<Entry> = Vec::new();
    let mut token: Option<String> = None;
    let mut page = 0usize;
    loop {
        let url = page_url(&prefix, max_keys, token.as_deref());
        let xml = curl(&url).ok_or_else(|| format!("{url}: fetch void"))?;
        let part = parse_page(&xml);
        let page_keys = part.len();
        entries.extend(part);
        page += 1;
        let truncated = tag(&xml, "IsTruncated")
            .map(|t| t.trim().eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        eprintln!(
            "tropomi_stac_compiler: page {page} carries {page_keys} keys ({} total)",
            entries.len()
        );
        if !truncated {
            break;
        }
        if page >= max_pages {
            return Err(format!(
                "listing still truncated after {page} pages — the index would be partial, the asset stays unwritten"
            ));
        }
        match tag(&xml, "NextContinuationToken") {
            Some(next) if !next.trim().is_empty() => token = Some(next.trim().to_string()),
            _ => {
                return Err(format!(
                    "{url}: truncated page carries no NextContinuationToken — the asset stays unwritten"
                ));
            }
        }
    }
    if entries.is_empty() {
        return Err(format!(
            "prefix {prefix:?} carries no key — the asset stays unwritten"
        ));
    }

    let mut text = String::from("[\n");
    for (i, entry) in entries.iter().enumerate() {
        text.push_str(&format!(
            "{{\"key\":{},\"last_modified\":{},\"size\":{}}}",
            json_string(&entry.key),
            json_string(&entry.last_modified),
            entry.size
        ));
        text.push_str(if i + 1 == entries.len() { "\n" } else { ",\n" });
    }
    text.push_str("]\n");

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, text.as_bytes()).map_err(|e| format!("write {out} returned {e}"))?;

    eprintln!(
        "tropomi_stac_compiler: {} keys over {page} pages ({} B) written to {out}",
        entries.len(),
        text.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("tropomi_stac_compiler: {msg}");
        std::process::exit(2);
    }
}
