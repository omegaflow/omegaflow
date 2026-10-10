use crate::json::{self, Json};
use crate::net::{get, urlencode};
use crate::secrets::{Secret, resolve_key};
use std::collections::HashMap;

const KEY_NAME: &str = "GOOGLE_CSE_API_KEY";
const CX_NAME: &str = "GOOGLE_CSE_CX";
const ENDPOINT: &str = "https://www.googleapis.com/customsearch/v1";

pub fn googlecse_lines(query: &str, max: usize) -> Vec<String> {
    let env_map: HashMap<String, String> = match crate::find_repo_root() {
        Some(repo) => crate::secrets::load_env(&repo),
        None => std::env::vars().collect(),
    };
    let key = resolve_key(
        env_map.get(KEY_NAME).map(String::as_str).unwrap_or(""),
        &env_map,
    );
    let cx = resolve_key(
        env_map.get(CX_NAME).map(String::as_str).unwrap_or(""),
        &env_map,
    );
    match (&key, &cx) {
        (Secret::Value(k), Secret::Value(c)) => googlecse_request(query, k, c, max),
        (Secret::Absent(_), _) => vec![format!(
            "pending — {} absent from .secrets.local/.env",
            KEY_NAME
        )],
        (_, Secret::Absent(_)) => vec![format!(
            "pending — {} absent from .secrets.local/.env",
            CX_NAME
        )],
    }
}

fn googlecse_request(query: &str, key: &str, cx: &str, max: usize) -> Vec<String> {
    let num = max.clamp(1, 10);
    let url = format!(
        "{}?key={}&cx={}&q={}&num={}",
        ENDPOINT,
        urlencode(key),
        urlencode(cx),
        urlencode(query),
        num
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let out = googlecse_results(&v);
                if out.is_empty() {
                    vec![format!("absent — google cse carries no entry: {}", query)]
                } else {
                    out
                }
            }
            None => vec!["pending — the google cse response carries no JSON".to_string()],
        },
        Some(f) => {
            let raw: String = f.body.chars().take(240).collect();
            let message = match json::parse(&f.body) {
                Some(v) => match v
                    .get("error")
                    .and_then(|e| e.get("message"))
                    .or_else(|| v.get("message"))
                    .and_then(|m| m.as_str())
                {
                    Some(s) if !s.is_empty() => s.to_string(),
                    _ => raw.trim().to_string(),
                },
                None => raw.trim().to_string(),
            };
            vec![format!(
                "pending — google cse HTTP {}: {}",
                f.status_text(),
                message
            )]
        }
        None => vec!["pending — no network".to_string()],
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn googlecse_results(v: &Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("items").and_then(|i| i.as_arr()) else {
        return out;
    };
    for item in items {
        let Some(url) = item
            .get("link")
            .and_then(|l| l.as_str())
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let mut line = format!("url {}", url);
        if let Some(title) = item
            .get("title")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(snippet) = item
            .get("snippet")
            .and_then(|s| s.as_str())
            .filter(|s| !s.is_empty())
        {
            let c = collapse(snippet);
            let capped: String = c.chars().take(400).collect();
            line.push_str(&format!("\tsnippet: {}", capped));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_items() {
        let body = r#"{"items":[{"title":"Transfer entropy — Wikipedia","link":"https://en.wikipedia.org/wiki/Transfer_entropy","snippet":"Transfer entropy is a   measure."}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            googlecse_results(&v),
            vec!["url https://en.wikipedia.org/wiki/Transfer_entropy\ttitle: Transfer entropy — Wikipedia\tsnippet: Transfer entropy is a measure.".to_string()]
        );
    }

    #[test]
    fn skips_items_without_a_link() {
        let v = json::parse(r#"{"items":[{"title":"No link"}]}"#).unwrap();
        assert!(googlecse_results(&v).is_empty());
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        let v = json::parse(r#"{"kind":"customsearch#search"}"#).unwrap();
        assert!(googlecse_results(&v).is_empty());
    }
}
