use crate::json::{self, Json};
use crate::net::{json_escape, post};
use crate::secrets::{Secret, resolve_key};
use std::collections::HashMap;

const PERPLEXITY_KEY: &str = "PERPLEXITY_API_KEY";
const PERPLEXITY_URL: &str = "https://api.perplexity.ai/v1/agent";

pub fn perplexity_lines(query: &str) -> Vec<String> {
    let env_map: HashMap<String, String> = match crate::find_repo_root() {
        Some(repo) => crate::secrets::load_env(&repo),
        None => std::env::vars().collect(),
    };
    let key = resolve_key(
        env_map
            .get(PERPLEXITY_KEY)
            .map(String::as_str)
            .unwrap_or(""),
        &env_map,
    );
    match key {
        Secret::Value(token) => perplexity_request(query, &token),
        Secret::Absent(_) => vec![format!(
            "pending — {} absent from .secrets.local/.env",
            PERPLEXITY_KEY
        )],
    }
}

pub fn perplexity_body(query: &str) -> String {
    format!(
        "{{\"preset\":\"medium\",\"input\":\"{}\"}}",
        json_escape(query)
    )
}

fn perplexity_request(query: &str, token: &str) -> Vec<String> {
    let body = perplexity_body(query);
    let auth = format!("Authorization: Bearer {}", token);
    let headers = [auth.as_str()];
    match post(PERPLEXITY_URL, &body, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let out = perplexity_results(&v);
                if out.is_empty() {
                    vec![format!("absent — Perplexity carries no answer: {}", query)]
                } else {
                    out
                }
            }
            None => vec!["pending — the Perplexity response carries no JSON".to_string()],
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
                "pending — perplexity HTTP {}: {}",
                f.status_text(),
                message
            )]
        }
        None => vec!["pending — no network".to_string()],
    }
}

fn perplexity_results(v: &Json) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(items) = v.get("output").and_then(|o| o.as_arr()) {
        for item in items {
            if let Some(content) = item.get("content").and_then(|c| c.as_arr()) {
                for part in content {
                    if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                        for line in text.lines() {
                            let trimmed = line.trim_end();
                            if !trimmed.trim().is_empty() {
                                out.push(trimmed.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    if out.is_empty() {
        if let Some(content) = v
            .get("choices")
            .and_then(|c| c.as_arr())
            .and_then(|a| a.first())
            .and_then(|c| c.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
        {
            for line in content.lines() {
                let trimmed = line.trim_end();
                if !trimmed.trim().is_empty() {
                    out.push(trimmed.to_string());
                }
            }
        }
    }
    if let Some(citations) = v.get("citations").and_then(|c| c.as_arr()) {
        for citation in citations {
            if let Some(url) = citation.as_str() {
                if !url.is_empty() {
                    out.push(format!("source {}", url));
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_the_agent_body_with_json_escaped_input() {
        assert_eq!(
            perplexity_body("transfer entropy"),
            "{\"preset\":\"medium\",\"input\":\"transfer entropy\"}"
        );
        assert_eq!(
            perplexity_body("a\"b\nc"),
            "{\"preset\":\"medium\",\"input\":\"a\\\"b\\nc\"}"
        );
    }

    #[test]
    fn reads_the_answer_text_and_appends_citations() {
        let body = r#"{"output":[{"type":"message","content":[{"type":"output_text","text":"It holds.\nA = A."}]}],"citations":["https://a.example/x","https://b.example/y"]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            perplexity_results(&v),
            vec![
                "It holds.".to_string(),
                "A = A.".to_string(),
                "source https://a.example/x".to_string(),
                "source https://b.example/y".to_string(),
            ]
        );
    }
}
