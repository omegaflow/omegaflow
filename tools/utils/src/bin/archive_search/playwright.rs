use crate::json::{self, Json};
use std::path::PathBuf;
use std::process::Command;

const FETCH_JS: &str = include_str!("playwright_fetch.cjs");

fn playwright_node_modules() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let npx = PathBuf::from(&home).join(".npm/_npx");
    if let Ok(entries) = std::fs::read_dir(&npx) {
        for entry in entries.flatten() {
            let nm = entry.path().join("node_modules");
            if nm.join("playwright").is_dir() {
                return Some(nm);
            }
        }
    }
    let stable = PathBuf::from(&home).join(".cache/omegaflow/playwright/node_modules");
    if stable.join("playwright").is_dir() {
        return Some(stable);
    }
    None
}

fn write_helper() -> Option<PathBuf> {
    let path = std::env::temp_dir().join("omegaflow_playwright_fetch.cjs");
    std::fs::write(&path, FETCH_JS).ok()?;
    Some(path)
}

pub fn run_lines(input: &str, headed: bool) -> Vec<String> {
    if input.starts_with("http://") || input.starts_with("https://") {
        render_lines(input, headed)
    } else {
        search_lines(input)
    }
}

fn render_lines(url: &str, headed: bool) -> Vec<String> {
    let Some(node_modules) = playwright_node_modules() else {
        return vec![
            "pending — playwright carries no module (run `npx -y playwright --version` once)"
                .to_string(),
        ];
    };
    let Some(helper) = write_helper() else {
        return vec!["pending — the helper carries no temp home".to_string()];
    };
    let mut cmd = Command::new("node");
    cmd.arg(&helper).arg(url).env("NODE_PATH", &node_modules);
    if headed {
        cmd.env("OMEGAFLOW_HEADED", "1");
    }
    if let Some(proxy) = crate::net::socks_proxy() {
        cmd.env("OMEGAFLOW_PROXY", proxy);
    }
    let out = match cmd.output() {
        Ok(o) => o,
        Err(_) => return vec!["pending — node carries no response".to_string()],
    };
    let body = String::from_utf8_lossy(&out.stdout);
    if body.trim().is_empty() {
        let note = String::from_utf8_lossy(&out.stderr);
        let first = note.lines().next().unwrap_or("").trim();
        return vec![format!(
            "pending — playwright rendered nothing for {} {}",
            url, first
        )];
    }
    let Some(v) = json::parse(&body) else {
        return vec!["pending — the playwright response carries no JSON".to_string()];
    };
    page_lines(&v, url)
}

fn status_code(v: &Json) -> Option<i64> {
    match v.get("status") {
        Some(Json::Num(n)) => Some(*n as i64),
        _ => None,
    }
}

fn page_lines(v: &Json, input: &str) -> Vec<String> {
    let url = v.get("url").and_then(|u| u.as_str()).unwrap_or(input);
    let title = v.get("title").and_then(|t| t.as_str()).unwrap_or("");
    let mut lines = Vec::new();
    lines.push(format!(
        "url {}\ttitle: {}",
        url,
        if title.is_empty() { "absent" } else { title }
    ));
    if let Some(s) = status_code(v) {
        lines.push(format!("status {}", s));
    }
    if matches!(v.get("challenge"), Some(Json::Bool(true))) {
        lines.push(
            "bridge: the Cloudflare interstitial did not clear — use the browser bridge (the operator's profile) or `bin/proton-wg.sh suggest <host>` for a country exit (operator consent)"
                .to_string(),
        );
    }
    if let Some(d) = v.get("description").and_then(|d| d.as_str()) {
        if !d.is_empty() {
            lines.push(format!("description: {}", d));
        }
    }
    if let Some(hs) = v.get("headings").and_then(|h| h.as_arr()) {
        for h in hs {
            if let Some(s) = h.as_str() {
                lines.push(format!("heading: {}", s));
            }
        }
    }
    if let Some(ls) = v.get("links").and_then(|l| l.as_arr()) {
        for l in ls {
            let t = l.get("text").and_then(|x| x.as_str()).unwrap_or("");
            let h = l.get("href").and_then(|x| x.as_str()).unwrap_or("");
            if h.is_empty() {
                continue;
            }
            lines.push(format!("link: {} -> {}", t, h));
        }
    }
    if let Some(t) = v.get("text").and_then(|t| t.as_str()) {
        if !t.is_empty() {
            lines.push(format!("text: {}", t));
        }
    }
    lines.push(format!("measurement {}", crate::net::today()));
    lines
}

fn search_lines(query: &str) -> Vec<String> {
    let url = format!(
        "https://www.bing.com/search?format=rss&q={}",
        crate::net::urlencode(query)
    );
    let Some(f) = crate::net::get(&url, &["-H", "User-Agent: Mozilla/5.0"], "30") else {
        return vec!["pending — no network".to_string()];
    };
    if f.status != Some(200) {
        let code = match f.status {
            Some(s) => s.to_string(),
            None => "absent".to_string(),
        };
        return vec![format!("pending — the search carries HTTP {}", code)];
    }
    let items = parse_rss_items(&f.body);
    if items.is_empty() {
        return vec![format!("absent — the search carries no result: {}", query)];
    }
    let mut lines = vec![format!("search {}\tresults: {}", query, items.len())];
    for (title, link, desc) in items.iter().take(15) {
        lines.push(format!("result: {}\t{}", title, link));
        if !desc.is_empty() {
            lines.push(format!("  snippet: {}", desc));
        }
    }
    lines.push(format!("measurement {}", crate::net::today()));
    lines
}

fn extract_between<'a>(s: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = s.find(open)? + open.len();
    let rest = &s[start..];
    let end = rest.find(close)?;
    Some(&rest[..end])
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        if in_tag {
            if c == '>' {
                in_tag = false;
            }
            continue;
        }
        if c == '<' {
            in_tag = true;
            continue;
        }
        out.push(c);
    }
    out
}

fn flatten(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&nbsp;", " ")
}

fn rss_field(item: &str, name: &str) -> String {
    let open = format!("<{}>", name);
    let close = format!("</{}>", name);
    match extract_between(item, &open, &close) {
        Some(t) => unescape(&flatten(&strip_tags(t))),
        None => String::new(),
    }
}

fn parse_rss_items(xml: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(open) = rest.find("<item>") {
        let close = match rest[open..].find("</item>") {
            Some(c) => open + c,
            None => break,
        };
        let item = &rest[open..close];
        let title = rss_field(item, "title");
        let link = rss_field(item, "link");
        let desc = rss_field(item, "description");
        if !link.is_empty() {
            out.push((title, link, desc));
        }
        rest = &rest[close..];
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_lines_carry_title_headings_links_and_measurement() {
        let body = r#"{"kind":"page","status":200,"url":"https://example.com/","title":"Example Domain",
            "description":"An example.","headings":["h1: Example Domain"],
            "links":[{"text":"Learn more","href":"https://iana.org/domains/example"}],
            "text":"This domain is for use in documentation."}"#;
        let v = json::parse(body).unwrap();
        let lines = page_lines(&v, "https://example.com/");
        assert!(lines[0].starts_with("url https://example.com/\ttitle: Example Domain"));
        assert!(lines.contains(&"status 200".to_string()));
        assert!(lines.contains(&"heading: h1: Example Domain".to_string()));
        assert!(
            lines.contains(&"link: Learn more -> https://iana.org/domains/example".to_string())
        );
        assert!(lines.iter().any(|l| l.starts_with("measurement ")));
    }

    #[test]
    fn rss_parser_reads_items() {
        let xml = "<rss><channel>\
            <item><title>Transfer entropy - Wikipedia</title>\
            <link>https://en.wikipedia.org/wiki/Transfer_entropy</link>\
            <description>Transfer entropy &amp; information theory</description></item>\
            <item><title>Second</title><link>https://example.com/</link></item>\
            </channel></rss>";
        let items = parse_rss_items(xml);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].0, "Transfer entropy - Wikipedia");
        assert_eq!(items[0].1, "https://en.wikipedia.org/wiki/Transfer_entropy");
        assert_eq!(items[0].2, "Transfer entropy & information theory");
    }
}
