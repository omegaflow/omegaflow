use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.stackexchange.com/2.3/search/advanced";
const DEFAULT_SITE: &str = "stackoverflow";

pub fn stackexchange_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?order=desc&sort=relevance&q={}&site={}&pagesize={}",
        ENDPOINT,
        urlencode(query),
        DEFAULT_SITE,
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the stackexchange response carries no JSON".to_string()];
            };
            let out = parse_stackexchange(&v);
            if out.is_empty() {
                vec![format!(
                    "absent — stackexchange carries no entry: {}",
                    query
                )]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — stackexchange HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn tag_list(item: &json::Json) -> Option<String> {
    let tags: Vec<&str> = item
        .get("tags")?
        .as_arr()?
        .iter()
        .filter_map(|t| t.as_str())
        .filter(|s| !s.is_empty())
        .collect();
    if tags.is_empty() {
        return None;
    }
    Some(tags.join(", "))
}

fn parse_stackexchange(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("items").and_then(|d| d.as_arr()) else {
        return out;
    };
    for item in items {
        let url = item
            .get("link")
            .and_then(|l| l.as_str())
            .filter(|s| !s.is_empty());
        let Some(url) = url else {
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
        if let Some(score) = item.get("score").and_then(|s| s.as_scalar_string()) {
            line.push_str(&format!("\tscore: {}", score));
        }
        if let Some(json::Json::Bool(answered)) = item.get("is_answered") {
            line.push_str(&format!("\tanswered: {}", answered));
        }
        if let Some(count) = item.get("answer_count").and_then(|c| c.as_scalar_string()) {
            line.push_str(&format!("\tanswers: {}", count));
        }
        if let Some(tags) = tag_list(item) {
            line.push_str(&format!("\ttags: {}", tags));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(body: &str) -> Vec<String> {
        parse_stackexchange(&json::parse(body).expect("test body is JSON"))
    }

    #[test]
    fn reads_the_full_item() {
        let body = r#"{"items":[{"title":"What is transfer entropy?","link":"https://physics.stackexchange.com/questions/1/x","score":4,"is_answered":true,"answer_count":2,"tags":["information-theory","entropy"]}]}"#;
        assert_eq!(
            parse(body),
            vec![
                "url https://physics.stackexchange.com/questions/1/x\ttitle: What is transfer entropy?\tscore: 4\tanswered: true\tanswers: 2\ttags: information-theory, entropy".to_string()
            ]
        );
    }

    #[test]
    fn omits_absent_fields() {
        let body = r#"{"items":[{"title":"Bare","link":"https://example.stackexchange.com/q/1"}]}"#;
        assert_eq!(
            parse(body),
            vec!["url https://example.stackexchange.com/q/1\ttitle: Bare".to_string()]
        );
    }

    #[test]
    fn empty_items_carries_nothing() {
        assert!(parse(r#"{"items":[]}"#).is_empty());
    }

    #[test]
    fn a_missing_link_is_skipped() {
        assert!(parse(r#"{"items":[{"title":"No link"}]}"#).is_empty());
    }
}
