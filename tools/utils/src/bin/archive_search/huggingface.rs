use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://huggingface.co/api/datasets";

pub fn huggingface_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?search={}&limit={}", ENDPOINT, urlencode(query), max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the huggingface response carries no JSON".to_string()];
            };
            let out = parse_huggingface(&v, max);
            if out.is_empty() {
                vec![format!(
                    "absent — huggingface carries no dataset: {}",
                    query
                )]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — huggingface HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_huggingface(v: &json::Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.as_arr() else {
        return out;
    };
    for item in items {
        let Some(id) = item
            .get("id")
            .and_then(|i| i.as_str())
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let mut line = format!("url https://huggingface.co/datasets/{}", id);
        if let Some(author) = item
            .get("author")
            .and_then(|a| a.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tauthor: {}", author));
        }
        if let Some(downloads) = item.get("downloads").and_then(|d| d.as_scalar_string()) {
            line.push_str(&format!("\tdownloads: {}", downloads));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_url_author_and_downloads() {
        let body = r#"[{"id":"DataForGood/climate-misinformation-RCoT","author":"DataForGood","downloads":673,"likes":1}]"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_huggingface(&v, 10),
            vec!["url https://huggingface.co/datasets/DataForGood/climate-misinformation-RCoT\tauthor: DataForGood\tdownloads: 673".to_string()]
        );
    }

    #[test]
    fn skips_items_without_an_id() {
        let v = json::parse(r#"[{"author":"nobody"},{"id":"ok/one"}]"#).unwrap();
        assert_eq!(
            parse_huggingface(&v, 10),
            vec!["url https://huggingface.co/datasets/ok/one".to_string()]
        );
    }

    #[test]
    fn caps_at_max_and_carries_nothing_for_an_empty_array() {
        let v = json::parse(r#"[{"id":"a/one"},{"id":"b/two"}]"#).unwrap();
        assert_eq!(parse_huggingface(&v, 1).len(), 1);
        let empty = json::parse(r#"[]"#).unwrap();
        assert!(parse_huggingface(&empty, 10).is_empty());
    }
}
