use crate::json::{self, Json};
use crate::net::{json_escape, post};
use crate::secrets::{Secret, resolve_key};
use std::collections::HashMap;

const PERPLEXITY_KEY: &str = "PERPLEXITY_API_KEY";
const PERPLEXITY_URL: &str = "https://api.perplexity.ai/chat/completions";

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
        "{{\"model\":\"sonar\",\"messages\":[{{\"role\":\"user\",\"content\":\"{}\"}}]}}",
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
        Some(f) => vec![format!("pending — perplexity HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn perplexity_results(v: &Json) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(content) = v
        .get("choices")
        .and_then(|c| c.as_arr())
        .and_then(|a| a.first())
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
    {
        for line in content.lines() {
            let text = line.trim_end();
            if !text.trim().is_empty() {
                out.push(text.to_string());
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
    fn builds_the_sonar_body_with_json_escaped_query() {
        assert_eq!(
            perplexity_body("transfer entropy"),
            "{\"model\":\"sonar\",\"messages\":[{\"role\":\"user\",\"content\":\"transfer entropy\"}]}"
        );
        assert_eq!(
            perplexity_body("a\"b\nc"),
            "{\"model\":\"sonar\",\"messages\":[{\"role\":\"user\",\"content\":\"a\\\"b\\nc\"}]}"
        );
    }

    #[test]
    fn reads_the_answer_text_and_appends_citations() {
        let body = r#"{"choices":[{"message":{"role":"assistant","content":"It holds.\nA = A."}}],"citations":["https://a.example/x","https://b.example/y"]}"#;
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
