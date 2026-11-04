use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://datadryad.org/api/v2/search";

pub fn dryad_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?q={}&per_page={}", ENDPOINT, urlencode(query), max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the dryad response carries no JSON".to_string()];
            };
            let out = parse_dryad(&v);
            if out.is_empty() {
                vec![format!("absent — dryad carries no dataset: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — dryad HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}

fn authors_field(item: &json::Json) -> Option<String> {
    let names: Vec<&str> = item
        .get("authors")?
        .as_arr()?
        .iter()
        .filter_map(|a| a.get("lastName").and_then(|n| n.as_str()))
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

fn parse_dryad(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v
        .get("_embedded")
        .and_then(|e| e.get("stash:datasets"))
        .and_then(|d| d.as_arr())
    else {
        return out;
    };
    for item in items {
        let Some(identifier) = item
            .get("identifier")
            .and_then(|i| i.as_str())
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let doi = identifier.strip_prefix("doi:").unwrap_or(identifier);
        let mut line = format!("url https://doi.org/{}", doi);
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
        if let Some(abstract_) = item
            .get("abstract")
            .and_then(|a| a.as_str())
            .filter(|s| !s.is_empty())
        {
            let c = collapse(&strip_tags(abstract_));
            if !c.is_empty() {
                let capped: String = c.chars().take(300).collect();
                line.push_str(&format!("\tabstract: {}", capped));
            }
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
        let body = r#"{"_embedded":{"stash:datasets":[{"identifier":"doi:10.5061/dryad.abc","title":"Leaf litter","authors":[{"lastName":"Lamit"},{"lastName":"Smith"}],"abstract":"<p>A short  abstract.</p>"}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_dryad(&v),
            vec!["url https://doi.org/10.5061/dryad.abc\ttitle: Leaf litter\tauthors: Lamit, Smith\tabstract: A short abstract.".to_string()]
        );
    }

    #[test]
    fn skips_items_without_an_identifier() {
        let v = json::parse(r#"{"_embedded":{"stash:datasets":[{"title":"No doi"}]}}"#).unwrap();
        assert!(parse_dryad(&v).is_empty());
    }

    #[test]
    fn empty_embedded_carries_nothing() {
        let v = json::parse(r#"{"_embedded":{"stash:datasets":[]}}"#).unwrap();
        assert!(parse_dryad(&v).is_empty());
    }
}
