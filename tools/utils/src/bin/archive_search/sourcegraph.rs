use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://sourcegraph.com/.api/search/stream";

pub fn sourcegraph_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?q={}&v=V3&t=literal", ENDPOINT, urlencode(query));
    match get(&url, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let out = parse_sourcegraph(&f.body, max);
            if out.is_empty() {
                vec![format!("absent — sourcegraph carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — sourcegraph HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_sourcegraph(body: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut event = String::new();
    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("event: ") {
            event = rest.trim().to_string();
            continue;
        }
        let Some(data) = line.strip_prefix("data: ") else {
            continue;
        };
        if event != "matches" {
            continue;
        }
        let Some(v) = json::parse(data) else {
            continue;
        };
        let Some(items) = v.as_arr() else {
            continue;
        };
        for item in items {
            if out.len() >= max {
                return out;
            }
            let repo = item
                .get("repository")
                .and_then(|r| r.as_str())
                .unwrap_or("");
            let path = item.get("path").and_then(|p| p.as_str()).unwrap_or("");
            if repo.is_empty() || path.is_empty() {
                continue;
            }
            let mut line = format!("url https://sourcegraph.com/{}/-/blob/{}", repo, path);
            line.push_str(&format!("\trepo: {}", repo));
            line.push_str(&format!("\tpath: {}", path));
            if let Some(lm) = item
                .get("lineMatches")
                .and_then(|m| m.as_arr())
                .and_then(|m| m.first())
            {
                if let Some(n) = lm.get("lineNumber").and_then(|n| n.as_scalar_string()) {
                    line.push_str(&format!("\tline: {}", n));
                }
                if let Some(text) = lm
                    .get("line")
                    .and_then(|p| p.as_str())
                    .filter(|s| !s.is_empty())
                {
                    let c = collapse(text);
                    let capped: String = c.chars().take(200).collect();
                    line.push_str(&format!("\tmatch: {}", capped));
                }
            }
            out.push(line);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_match_event() {
        let body = "event: matches\ndata: [{\"type\":\"content\",\"path\":\"src/te.rs\",\"repository\":\"github.com/omegaflow/omegaflow\",\"lineMatches\":[{\"lineNumber\":42,\"line\":\"fn transfer_entropy() -> f64\"}]}]\n\nevent: done\ndata: {}\n";
        assert_eq!(
            parse_sourcegraph(body, 10),
            vec!["url https://sourcegraph.com/github.com/omegaflow/omegaflow/-/blob/src/te.rs\trepo: github.com/omegaflow/omegaflow\tpath: src/te.rs\tline: 42\tmatch: fn transfer_entropy() -> f64".to_string()]
        );
    }

    #[test]
    fn ignores_non_match_events() {
        let body = "event: filters\ndata: [{\"value\":\"archived:yes\"}]\n\nevent: progress\ndata: {\"done\":false}\n";
        assert!(parse_sourcegraph(body, 10).is_empty());
    }

    #[test]
    fn caps_at_max() {
        let one =
            "event: matches\ndata: [{\"path\":\"a\",\"repository\":\"r\",\"lineMatches\":[]}]\n";
        let body = one.repeat(5);
        assert_eq!(parse_sourcegraph(&body, 2).len(), 2);
    }

    #[test]
    fn skips_items_without_repo_or_path() {
        let body = "event: matches\ndata: [{\"lineMatches\":[]},{\"path\":\"a\"}]\n";
        assert!(parse_sourcegraph(body, 10).is_empty());
    }
}
