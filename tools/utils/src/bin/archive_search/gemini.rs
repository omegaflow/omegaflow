use crate::json::{self, Json};
use crate::net::{json_escape, post};
use crate::secrets::{Secret, resolve_key};
use std::collections::HashMap;

const GEMINI_KEY: &str = "GEMINI_API_KEY";
const GEMINI_BASE: &str = "https://generativelanguage.googleapis.com/v1beta/models";
const DEFAULT_MODEL: &str = "gemini-flash-latest";

pub fn gemini_lines(query: &str) -> Vec<String> {
    let env_map: HashMap<String, String> = match crate::find_repo_root() {
        Some(repo) => crate::secrets::load_env(&repo),
        None => std::env::vars().collect(),
    };
    let key = resolve_key(
        env_map.get(GEMINI_KEY).map(String::as_str).unwrap_or(""),
        &env_map,
    );
    match key {
        Secret::Value(token) => {
            let (text, refine) = crate::refine::split_refine(query, &["model"]);
            let model = crate::refine::value_of(&refine, "model").unwrap_or(DEFAULT_MODEL);
            gemini_request(&text, model, &token)
        }
        Secret::Absent(_) => vec![format!(
            "pending — {} absent from .secrets.local/.env",
            GEMINI_KEY
        )],
    }
}

pub fn gemini_body(query: &str, grounded: bool) -> String {
    let tools = if grounded {
        ",\"tools\":[{\"google_search\":{}}]"
    } else {
        ""
    };
    format!(
        "{{\"contents\":[{{\"parts\":[{{\"text\":\"{}\"}}]}}]{}}}",
        json_escape(query),
        tools
    )
}

fn gemini_request(query: &str, model: &str, token: &str) -> Vec<String> {
    let body = gemini_body(query, true);
    let url = format!("{}/{}:generateContent", GEMINI_BASE, model);
    let auth = format!("x-goog-api-key: {}", token);
    let headers = [auth.as_str()];
    match post(&url, &body, &headers, "60") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let out = gemini_results(&v);
                if out.is_empty() {
                    vec![format!("absent — gemini carries no answer: {}", query)]
                } else {
                    out
                }
            }
            None => vec!["pending — the gemini response carries no JSON".to_string()],
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
                "pending — gemini HTTP {}: {}",
                f.status_text(),
                message
            )]
        }
        None => vec!["pending — no network".to_string()],
    }
}

fn gemini_results(v: &Json) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(candidates) = v.get("candidates").and_then(|c| c.as_arr()) {
        for candidate in candidates {
            if let Some(parts) = candidate
                .get("content")
                .and_then(|c| c.get("parts"))
                .and_then(|p| p.as_arr())
            {
                for part in parts {
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
    if let Some(chunks) = v
        .get("candidates")
        .and_then(|c| c.as_arr())
        .and_then(|a| a.first())
        .and_then(|c| c.get("groundingMetadata"))
        .and_then(|g| g.get("groundingChunks"))
        .and_then(|g| g.as_arr())
    {
        for chunk in chunks {
            if let Some(uri) = chunk
                .get("web")
                .and_then(|w| w.get("uri"))
                .and_then(|u| u.as_str())
                .filter(|s| !s.is_empty())
            {
                out.push(format!("source {}", uri));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_the_grounded_body_with_json_escaped_input() {
        assert_eq!(
            gemini_body("transfer entropy", true),
            "{\"contents\":[{\"parts\":[{\"text\":\"transfer entropy\"}]}],\"tools\":[{\"google_search\":{}}]}"
        );
        assert_eq!(
            gemini_body("a\"b\nc", true),
            "{\"contents\":[{\"parts\":[{\"text\":\"a\\\"b\\nc\"}]}],\"tools\":[{\"google_search\":{}}]}"
        );
    }

    #[test]
    fn an_ungrounded_body_carries_no_tools() {
        assert_eq!(
            gemini_body("transfer entropy", false),
            "{\"contents\":[{\"parts\":[{\"text\":\"transfer entropy\"}]}]}"
        );
    }

    #[test]
    fn reads_the_answer_text_and_appends_grounding_sources() {
        let body = r#"{"candidates":[{"content":{"parts":[{"text":"It holds.\nA = A."}]},"groundingMetadata":{"groundingChunks":[{"web":{"uri":"https://a.example/x"}},{"web":{"uri":"https://b.example/y"}}]}}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            gemini_results(&v),
            vec![
                "It holds.".to_string(),
                "A = A.".to_string(),
                "source https://a.example/x".to_string(),
                "source https://b.example/y".to_string(),
            ]
        );
    }
}
