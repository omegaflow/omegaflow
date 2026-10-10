use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://openlibrary.org/search.json";
const BASE_URL: &str = "https://openlibrary.org";

pub fn openlibrary_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?q={}&limit={}&fields=key,title,author_name,first_publish_year,edition_count",
        ENDPOINT,
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the openlibrary response carries no JSON".to_string()];
            };
            let out = parse_openlibrary(&v);
            if out.is_empty() {
                vec![format!("absent — openlibrary carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — openlibrary HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn authors_field(doc: &json::Json) -> Option<String> {
    let names: Vec<&str> = doc
        .get("author_name")?
        .as_arr()?
        .iter()
        .filter_map(|a| a.as_str())
        .filter(|s| !s.is_empty())
        .collect();
    if names.is_empty() {
        return None;
    }
    let cap = 8usize;
    let mut joined = names
        .iter()
        .take(cap)
        .copied()
        .collect::<Vec<_>>()
        .join(", ");
    if names.len() > cap {
        joined.push_str(", et al.");
    }
    Some(joined)
}

fn parse_openlibrary(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("docs").and_then(|d| d.as_arr()) else {
        return out;
    };
    for item in items {
        let key = item
            .get("key")
            .and_then(|k| k.as_str())
            .filter(|s| !s.is_empty());
        let url = match key {
            Some(k) => format!("{}{}", BASE_URL, k),
            None => BASE_URL.to_string(),
        };
        let mut line = format!("url {}", url);
        if let Some(title) = item
            .get("title")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(authors) = authors_field(item) {
            line.push_str(&format!("\tauthors: {}", authors));
        }
        if let Some(year) = item
            .get("first_publish_year")
            .and_then(|y| y.as_scalar_string())
        {
            line.push_str(&format!("\tyear: {}", year));
        }
        if let Some(editions) = item.get("edition_count").and_then(|e| e.as_scalar_string()) {
            line.push_str(&format!("\teditions: {}", editions));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(body: &str) -> Vec<String> {
        parse_openlibrary(&json::parse(body).expect("test body is JSON"))
    }

    #[test]
    fn reads_the_full_item() {
        let body = r#"{"docs":[{"key":"/works/OL1W","title":"Spacetime and geometry","author_name":["Sean M. Carroll"],"first_publish_year":2003,"edition_count":5}]}"#;
        assert_eq!(
            parse(body),
            vec!["url https://openlibrary.org/works/OL1W\ttitle: Spacetime and geometry\tauthors: Sean M. Carroll\tyear: 2003\teditions: 5".to_string()]
        );
    }

    #[test]
    fn omits_absent_author_year_editions() {
        let body = r#"{"docs":[{"key":"/works/OL2W","title":"Only a title"}]}"#;
        assert_eq!(
            parse(body),
            vec!["url https://openlibrary.org/works/OL2W\ttitle: Only a title".to_string()]
        );
    }

    #[test]
    fn empty_docs_carries_nothing() {
        assert!(parse(r#"{"docs":[]}"#).is_empty());
    }

    #[test]
    fn a_missing_key_reads_the_base_url() {
        let body = r#"{"docs":[{"title":"No key"}]}"#;
        assert_eq!(
            parse(body),
            vec!["url https://openlibrary.org\ttitle: No key".to_string()]
        );
    }

    #[test]
    fn caps_authors_at_eight_with_et_al() {
        let names: Vec<String> = (1..=9).map(|i| format!("\"Author {i}\"")).collect();
        let body = format!(
            r#"{{"docs":[{{"key":"/works/OL3W","title":"Many","author_name":[{}]}}]}}"#,
            names.join(",")
        );
        assert_eq!(
            parse(&body),
            vec!["url https://openlibrary.org/works/OL3W\ttitle: Many\tauthors: Author 1, Author 2, Author 3, Author 4, Author 5, Author 6, Author 7, Author 8, et al.".to_string()]
        );
    }
}
