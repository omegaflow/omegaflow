use crate::json;
use crate::net::{get, urlencode};

pub fn hn_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://hn.algolia.com/api/v1/search?query={}&hitsPerPage={}",
        urlencode(query),
        max
    );
    match get(&url, &[], "30") {
        Some(f) if f.status == Some(200) => {
            let lines = parse_hn(&f.body);
            if lines.is_empty() {
                vec![format!("absent — Hacker News carries no item: {}", query)]
            } else {
                lines
            }
        }
        Some(f) => vec![format!("pending — hn HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
}

fn parse_hn(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(hits) = v.get("hits").and_then(|h| h.as_arr()) else {
        return out;
    };
    for hit in hits {
        let Some(id) = hit.get("objectID").and_then(|o| o.as_scalar_string()) else {
            continue;
        };
        let url = match field(hit, "url").or_else(|| field(hit, "story_url")) {
            Some(url) => url,
            None => format!("https://news.ycombinator.com/item?id={id}"),
        };
        let mut line = format!("url {url}");
        if let Some(title) = field(hit, "title").or_else(|| field(hit, "story_title")) {
            line.push_str(&format!("\ttitle: {title}"));
        }
        if let Some(author) = field(hit, "author") {
            line.push_str(&format!("\tauthor: {author}"));
        }
        if let Some(points) = hit.get("points").and_then(|p| p.as_scalar_string()) {
            line.push_str(&format!("\tpoints: {points}"));
        }
        if let Some(comments) = hit.get("num_comments").and_then(|c| c.as_scalar_string()) {
            line.push_str(&format!("\tcomments: {comments}"));
        }
        if let Some(created) = field(hit, "created_at") {
            line.push_str(&format!("\tdate: {}", &created[..created.len().min(10)]));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_hits_with_story_fallback() {
        let body = r#"{"hits":[{"objectID":"41234567","title":"Solar wind hits Earth","url":"https://example.org/sw","author":"pg","points":120,"num_comments":42,"created_at":"2026-10-20T11:22:33Z"},{"objectID":"89","story_title":"Ask HN: solar wind","story_url":"https://example.org/ask","author":"alice","created_at":"2026-10-19T00:00:00Z"}]}"#;
        assert_eq!(
            parse_hn(body),
            vec![
                "url https://example.org/sw\ttitle: Solar wind hits Earth\tauthor: pg\tpoints: 120\tcomments: 42\tdate: 2026-10-20".to_string(),
                "url https://example.org/ask\ttitle: Ask HN: solar wind\tauthor: alice\tdate: 2026-10-19".to_string(),
            ]
        );
    }

    #[test]
    fn falls_back_to_the_item_page() {
        let body = r#"{"hits":[{"objectID":"7","title":"No link"}]}"#;
        assert_eq!(
            parse_hn(body),
            vec!["url https://news.ycombinator.com/item?id=7\ttitle: No link".to_string()]
        );
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_hn(r#"{"hits":[]}"#).is_empty());
    }
}
