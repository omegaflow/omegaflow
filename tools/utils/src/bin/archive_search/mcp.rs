use crate::json;
use std::process::Command;

const PROTOCOL: &str = "2025-03-26";

pub fn mcp_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(
        query,
        &["endpoint", "tool", "query_arg", "count_arg"],
    );
    let Some(endpoint) = crate::refine::value_of(&refine, "endpoint") else {
        return vec![
            "pending — no endpoint: pass `endpoint=<mcp-url> tool=<name> [query_arg=query] [count_arg=count] <query>`"
                .to_string(),
        ];
    };
    let Some(tool) = crate::refine::value_of(&refine, "tool") else {
        return vec!["pending — no tool: pass `tool=<name>`".to_string()];
    };
    let query_arg = crate::refine::value_of(&refine, "query_arg").unwrap_or("query");
    let count_arg = crate::refine::value_of(&refine, "count_arg").unwrap_or("count");
    call(&endpoint, &tool, query_arg, count_arg, text.trim(), max)
}

pub fn youmcp_lines(query: &str, max: usize) -> Vec<String> {
    call(
        "https://api.you.com/mcp?profile=free",
        "you-search",
        "query",
        "count",
        query,
        max,
    )
}

pub fn examcp_lines(query: &str, max: usize) -> Vec<String> {
    call(
        "https://mcp.exa.ai/mcp",
        "web_search_exa",
        "query",
        "numResults",
        query,
        max,
    )
}

pub fn earthmcp_lines(query: &str, max: usize) -> Vec<String> {
    call(
        "https://cmr.earthdata.nasa.gov/mcp/v1",
        "get_collections",
        "keyword",
        "limit",
        query,
        max.min(50),
    )
}

fn call(
    endpoint: &str,
    tool: &str,
    query_arg: &str,
    count_arg: &str,
    query: &str,
    max: usize,
) -> Vec<String> {
    if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
        return vec!["pending — endpoint must be http(s)".to_string()];
    }
    let init = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{{\"protocolVersion\":\"{PROTOCOL}\",\"capabilities\":{{}},\"clientInfo\":{{\"name\":\"omegaflow\",\"version\":\"1\"}}}}}}"
    );
    let Some(first) = post(endpoint, &[], &init) else {
        return vec!["pending — no network".to_string()];
    };
    if first.status != Some(200) {
        return vec![format!("pending — mcp initialize HTTP {}", status_text(first.status))];
    }
    let mut headers: Vec<(String, String)> = Vec::new();
    if let Some(session) = &first.session {
        headers.push(("Mcp-Session-Id".to_string(), session.clone()));
    }
    let body = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/call\",\"params\":{{\"name\":\"{}\",\"arguments\":{{\"{}\":\"{}\",\"{}\":{}}}}}}}",
        escape(tool),
        escape(query_arg),
        escape(query),
        escape(count_arg),
        max.clamp(1, 100)
    );
    let Some(second) = post(endpoint, &headers, &body) else {
        return vec!["pending — no answer from mcp tools/call".to_string()];
    };
    if second.status != Some(200) {
        return vec![format!("pending — mcp tools/call HTTP {}", status_text(second.status))];
    }
    let Some(v) = jsonrpc(&second.body) else {
        return vec!["pending — mcp response carries no JSON-RPC".to_string()];
    };
    if let Some(error) = v.get("error") {
        let message = error
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("unknown error");
        return vec![format!("pending — mcp error: {message}")];
    }
    let Some(result) = v.get("result") else {
        return vec!["pending — mcp response carries no result".to_string()];
    };
    if result
        .get("isError")
        .and_then(|e| match e {
            json::Json::Bool(b) => Some(*b),
            _ => None,
        })
        .unwrap_or(false)
    {
        return vec!["pending — mcp tool reported an error".to_string()];
    }
    let text = result_text(result);
    if text.trim().is_empty() {
        return vec!["pending — mcp result carries no text".to_string()];
    }
    let lines = parse_content(&text, max);
    if lines.is_empty() {
        vec![format!("pending — mcp tool returned no URL for: {query}")]
    } else {
        lines
    }
}

struct Fetch {
    status: Option<i32>,
    session: Option<String>,
    body: String,
}

fn post(url: &str, headers: &[(String, String)], body: &str) -> Option<Fetch> {
    let mut cmd = Command::new("curl");
    cmd.args([
        "-sS",
        "-i",
        "--max-time",
        "40",
        "-X",
        "POST",
        url,
        "-H",
        "Content-Type: application/json",
        "-H",
        "Accept: application/json, text/event-stream",
    ]);
    for (name, value) in headers {
        cmd.arg("-H").arg(format!("{name}: {value}"));
    }
    cmd.args(["-d", body]);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let split = find(&stdout, b"\r\n\r\n").or_else(|| find(&stdout, b"\n\n"))?;
    let head = String::from_utf8_lossy(&stdout[..split.0]).to_string();
    let body = String::from_utf8_lossy(&stdout[split.1..]).to_string();
    let status = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|s| s.parse::<i32>().ok());
    let session = head.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        if name.eq_ignore_ascii_case("mcp-session-id") {
            Some(value.trim().to_string())
        } else {
            None
        }
    });
    Some(Fetch {
        status,
        session,
        body,
    })
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<(usize, usize)> {
    haystack
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|i| (i, i + needle.len()))
}

fn status_text(status: Option<i32>) -> String {
    match status {
        Some(s) => s.to_string(),
        None => "absent".to_string(),
    }
}

fn jsonrpc(body: &str) -> Option<json::Json> {
    let trimmed = body.trim_start();
    if trimmed.starts_with('{') {
        return json::parse(trimmed);
    }
    for line in body.lines() {
        let Some(payload) = line.strip_prefix("data:") else {
            continue;
        };
        if let Some(v) = json::parse(payload.trim()) {
            if v.get("result").is_some() || v.get("error").is_some() {
                return Some(v);
            }
        }
    }
    None
}

fn result_text(result: &json::Json) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(items) = result.get("content").and_then(|c| c.as_arr()) {
        for item in items {
            if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                parts.push(text.to_string());
            }
        }
    }
    parts.join("\n")
}

fn parse_content(text: &str, max: usize) -> Vec<String> {
    if let Some(v) = json::parse(text) {
        return json_results(&v, max);
    }
    let lines = text_results(text, max);
    if lines.is_empty() {
        return parse_urls(text, max);
    }
    lines
}

fn json_results(v: &json::Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut items: Vec<&json::Json> = Vec::new();
    if let Some(results) = v.get("results") {
        if let Some(web) = results.get("web").and_then(|w| w.as_arr()) {
            items.extend(web.iter());
        }
        if let Some(news) = results.get("news").and_then(|n| n.as_arr()) {
            items.extend(news.iter());
        }
        if let Some(all) = results.as_arr() {
            items.extend(all.iter());
        }
    }
    for key in ["data", "hits", "items"] {
        if let Some(list) = v.get(key).and_then(|x| x.as_arr()) {
            items.extend(list.iter());
        }
    }
    for item in items {
        let Some(url) = item.get("url").and_then(|u| u.as_str()) else {
            continue;
        };
        if !url.starts_with("http") {
            continue;
        }
        let mut line = format!("url {url}");
        if let Some(title) = item.get("title").and_then(|t| t.as_str()) {
            line.push_str(&format!("\ttitle: {title}"));
        }
        let description = item
            .get("contents")
            .and_then(|c| c.get("highlights"))
            .and_then(|h| h.as_arr())
            .and_then(|h| h.first())
            .and_then(|x| x.as_str())
            .or_else(|| item.get("description").and_then(|d| d.as_str()))
            .or_else(|| item.get("snippet").and_then(|d| d.as_str()));
        if let Some(description) = description {
            line.push_str(&format!("\tdescription: {}", clean(description)));
        }
        if let Some(date) = item
            .get("page_age")
            .or_else(|| item.get("published"))
            .or_else(|| item.get("publishedDate"))
            .and_then(|d| d.as_str())
        {
            line.push_str(&format!("\tdate: {}", &date[..date.len().min(10)]));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    if out.len() < max {
        if let Some(collections) = v.get("collections").and_then(|c| c.as_arr()) {
            for collection in collections {
                if out.len() >= max {
                    break;
                }
                let title = collection
                    .get("entry_title")
                    .or_else(|| collection.get("title"))
                    .and_then(|t| t.as_str());
                let url = collection
                    .get("url")
                    .and_then(|u| u.as_str())
                    .map(str::to_string)
                    .or_else(|| {
                        collection
                            .get("doi")
                            .and_then(|d| d.as_str())
                            .filter(|d| !d.trim().is_empty())
                            .map(|d| {
                                let doi = d
                                    .trim()
                                    .trim_start_matches("https://doi.org/")
                                    .trim_start_matches("http://doi.org/");
                                format!("https://doi.org/{doi}")
                            })
                    })
                    .or_else(|| {
                        collection
                            .get("concept_id")
                            .and_then(|c| c.as_str())
                            .map(|id| {
                                format!("https://search.earthdata.nasa.gov/search?q={id}")
                            })
                    });
                let Some(url) = url else {
                    continue;
                };
                if !url.starts_with("http") {
                    continue;
                }
                let mut line = format!("url {url}");
                if let Some(title) = title {
                    line.push_str(&format!("\ttitle: {}", clean(title)));
                }
                if let Some(description) = collection
                    .get("abstract")
                    .or_else(|| collection.get("description"))
                    .and_then(|d| d.as_str())
                {
                    line.push_str(&format!("\tdescription: {}", clean(description)));
                }
                if let Some(date) = collection
                    .get("time_start")
                    .or_else(|| collection.get("published"))
                    .and_then(|d| d.as_str())
                {
                    line.push_str(&format!("\tdate: {}", &date[..date.len().min(10)]));
                }
                out.push(line);
            }
        }
    }
    out
}

fn text_results(text: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut title = String::new();
    let mut url = String::new();
    let mut date = String::new();
    let mut author = String::new();
    let mut description = String::new();
    let mut in_highlights = false;
    for raw in text.lines().chain(std::iter::once("")) {
        let line = raw.trim();
        if let Some(rest) = line.strip_prefix("Title:") {
            if !url.is_empty() {
                push_text(&mut out, &url, &title, &author, &date, &description, max);
                title.clear();
                url.clear();
                date.clear();
                author.clear();
                description.clear();
            }
            title = rest.trim().to_string();
            in_highlights = false;
        } else if let Some(rest) = line.strip_prefix("URL:") {
            url = rest.trim().to_string();
            in_highlights = false;
        } else if let Some(rest) = line.strip_prefix("Published:") {
            date = rest.trim().to_string();
            in_highlights = false;
        } else if let Some(rest) = line.strip_prefix("Author:") {
            author = rest.trim().to_string();
            in_highlights = false;
        } else if line == "Highlights:" {
            in_highlights = true;
        } else if in_highlights && !line.is_empty() && description.is_empty() {
            description = line.to_string();
        }
    }
    if !url.is_empty() {
        push_text(&mut out, &url, &title, &author, &date, &description, max);
    }
    out
}

fn push_text(
    out: &mut Vec<String>,
    url: &str,
    title: &str,
    author: &str,
    date: &str,
    description: &str,
    max: usize,
) {
    if !url.starts_with("http") || out.len() >= max {
        return;
    }
    let mut line = format!("url {url}");
    if !title.is_empty() {
        line.push_str(&format!("\ttitle: {title}"));
    }
    if !author.is_empty() {
        line.push_str(&format!("\tauthor: {author}"));
    }
    if !date.is_empty() {
        line.push_str(&format!("\tdate: {}", &date[..date.len().min(10)]));
    }
    if !description.is_empty() {
        line.push_str(&format!("\tdescription: {}", clean(description)));
    }
    out.push(line);
}

fn parse_urls(text: &str, max: usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for token in text.split(|c: char| c.is_whitespace() || c == ')' || c == ']' || c == '"') {
        let url = token.trim_start_matches('(').trim_end_matches(['.', ',', ';']);
        if !url.starts_with("http") || out.iter().any(|u| u == url) {
            continue;
        }
        out.push(format!("url {url}"));
        if out.len() >= max {
            break;
        }
    }
    out
}

fn clean(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' | '\r' | '\t' => out.push(' '),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_youcom_json_content() {
        let text = r#"{"results":{"web":[{"url":"https://example.org/a","title":"A","page_age":"2026-10-01T00:00:00","contents":{"highlights":["one two"]}}]}}"#;
        assert_eq!(
            parse_content(text, 10),
            vec!["url https://example.org/a\ttitle: A\tdescription: one two\tdate: 2026-10-01".to_string()]
        );
    }

    #[test]
    fn reads_exa_text_blocks() {
        let text = "Title: What Is the Solar Wind?\nURL: https://science.nasa.gov/sun/what-is-the-solar-wind/\nPublished: 2024-11-19T22:12:30.000Z\nAuthor: Abbey Interrante\nHighlights:\n# What Is the Solar Wind?\n\nTitle: Second\nURL: https://example.org/b\n";
        assert_eq!(
            parse_content(text, 10),
            vec![
                "url https://science.nasa.gov/sun/what-is-the-solar-wind/\ttitle: What Is the Solar Wind?\tauthor: Abbey Interrante\tdate: 2024-11-19\tdescription: # What Is the Solar Wind?".to_string(),
                "url https://example.org/b\ttitle: Second".to_string(),
            ]
        );
    }

    #[test]
    fn reads_earthdata_collections_by_doi() {
        let text = r#"{"status":"success","collections":[{"entry_title":"Sea Surface Temperature","doi":"10.5067/TEST","abstract":"daily global field","time_start":"2020-01-01T00:00:00Z"}]}"#;
        assert_eq!(
            parse_content(text, 10),
            vec!["url https://doi.org/10.5067/TEST\ttitle: Sea Surface Temperature\tdescription: daily global field\tdate: 2020-01-01".to_string()]
        );
    }

    #[test]
    fn a_json_answer_without_a_known_result_shape_is_pending() {
        let text = r#"{"status":"success","links":[{"url":"https://example.org/about"}]}"#;
        assert!(parse_content(text, 10).is_empty());
    }

    #[test]
    fn falls_back_to_bare_urls() {
        assert_eq!(
            parse_urls("see https://example.org/a and https://example.org/b", 10),
            vec!["url https://example.org/a".to_string(), "url https://example.org/b".to_string()]
        );
    }

    #[test]
    fn a_tool_without_url_is_pending() {
        let lines = parse_content("no links here", 10);
        assert!(lines.is_empty());
    }
}
