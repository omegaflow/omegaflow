use crate::json;
use crate::net::{get, urlencode};

pub fn datacite_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://api.datacite.org/dois?query={}&page[size]={}",
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_datacite(&f.body);
            if out.is_empty() {
                vec![format!("absent — datacite carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => {
            let code = match f.status {
                Some(s) => s.to_string(),
                None => "absent".to_string(),
            };
            vec![format!("pending — datacite HTTP {}", code)]
        }
        None => vec!["pending — no network".to_string()],
    }
}

fn nested_str(v: &json::Json, arr_key: &str, inner: &str) -> Option<String> {
    v.get(arr_key)
        .and_then(|a| a.as_arr())
        .and_then(|a| a.first())
        .and_then(|o| o.get(inner))
        .and_then(|s| s.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn authors_field(attrs: &json::Json) -> Option<String> {
    let creators = attrs.get("creators").and_then(|a| a.as_arr())?;
    let names: Vec<String> = creators
        .iter()
        .filter_map(|c| c.get("name").and_then(|s| s.as_str()))
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect();
    if names.is_empty() {
        return None;
    }
    if names.len() > 8 {
        Some(format!("{}, et al.", names[..8].join(", ")))
    } else {
        Some(names.join(", "))
    }
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

fn abstract_field(attrs: &json::Json) -> Option<String> {
    let descriptions = attrs.get("descriptions").and_then(|d| d.as_arr())?;
    let picked = descriptions
        .iter()
        .find(|d| {
            d.get("descriptionType")
                .and_then(|t| t.as_str())
                .map(|t| t == "Abstract")
                .unwrap_or(false)
        })
        .or_else(|| descriptions.first())?;
    let raw = picked
        .get("description")
        .and_then(|d| d.as_str())
        .filter(|s| !s.is_empty())?;
    let collapsed = strip_tags(raw)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if collapsed.is_empty() {
        return None;
    }
    if collapsed.chars().count() > 1200 {
        let capped: String = collapsed.chars().take(1200).collect();
        Some(format!("{}…", capped))
    } else {
        Some(collapsed)
    }
}

fn parse_datacite(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(items) = v.get("data").and_then(|d| d.as_arr()) else {
        return out;
    };
    for item in items {
        let Some(attrs) = item.get("attributes") else {
            continue;
        };
        let Some(doi) = attrs
            .get("doi")
            .and_then(|d| d.as_str())
            .filter(|d| !d.is_empty())
        else {
            continue;
        };
        let mut line = match attrs
            .get("url")
            .and_then(|u| u.as_str())
            .filter(|u| !u.is_empty())
        {
            Some(url) => format!("url {}\tdoi: {}", url, doi),
            None => format!("doi: {}", doi),
        };
        if let Some(title) = nested_str(attrs, "titles", "title") {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(rights) = nested_str(attrs, "rightsList", "rights") {
            line.push_str(&format!("\trights: {}", rights));
        }
        if let Some(authors) = authors_field(attrs) {
            line.push_str(&format!("\tauthors: {}", authors));
        }
        if let Some(abstract_) = abstract_field(attrs) {
            line.push_str(&format!("\tabstract: {}", abstract_));
        }
        if let Some(published) = attrs
            .get("published")
            .and_then(|p| p.as_str())
            .filter(|p| !p.is_empty())
        {
            line.push_str(&format!("\tpublished: {}", published));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_entry_fields() {
        let body = r#"{"data":[{"attributes":{"doi":"10.1234/abc","titles":[{"title":"A measured field"}],"url":"https://example.org/paper","rightsList":[{"rights":"CC BY 4.0"}]}}]}"#;
        assert_eq!(
            parse_datacite(body),
            vec!["url https://example.org/paper\tdoi: 10.1234/abc\ttitle: A measured field\trights: CC BY 4.0".to_string()]
        );
    }

    #[test]
    fn keeps_the_doi_without_url_and_rights() {
        let body = r#"{"data":[{"attributes":{"doi":"10.1234/abc","titles":[{"title":"A measured field"}]}}]}"#;
        assert_eq!(
            parse_datacite(body),
            vec!["doi: 10.1234/abc\ttitle: A measured field".to_string()]
        );
    }

    #[test]
    fn skips_the_element_without_doi() {
        let body = r#"{"data":[{"attributes":{"titles":[{"title":"No doi"}]}},{"attributes":{"doi":"10.1234/keep","titles":[{"title":"Kept"}]}}]}"#;
        assert_eq!(
            parse_datacite(body),
            vec!["doi: 10.1234/keep\ttitle: Kept".to_string()]
        );
    }

    #[test]
    fn empty_data_carries_nothing() {
        assert!(parse_datacite(r#"{"data":[]}"#).is_empty());
    }

    #[test]
    fn reads_authors_abstract_and_published() {
        let body = r#"{"data":[{"attributes":{"doi":"10.1234/full","titles":[{"title":"A measured field"}],"creators":[{"name":"Doe, Jane"},{"name":"Roe, Rick"}],"descriptions":[{"description":"<p>A short   abstract.</p>","descriptionType":"Abstract"}],"published":"2023-01-01"}}]}"#;
        assert_eq!(
            parse_datacite(body),
            vec!["doi: 10.1234/full\ttitle: A measured field\tauthors: Doe, Jane, Roe, Rick\tabstract: A short abstract.\tpublished: 2023-01-01".to_string()]
        );
    }

    #[test]
    fn omits_absent_new_fields() {
        let body = r#"{"data":[{"attributes":{"doi":"10.1234/plain","titles":[{"title":"A measured field"}]}}]}"#;
        assert_eq!(
            parse_datacite(body),
            vec!["doi: 10.1234/plain\ttitle: A measured field".to_string()]
        );
    }

    #[test]
    fn caps_authors_at_eight_with_et_al() {
        let body = r#"{"data":[{"attributes":{"doi":"10.1234/many","creators":[{"name":"A, A"},{"name":"B, B"},{"name":"C, C"},{"name":"D, D"},{"name":"E, E"},{"name":"F, F"},{"name":"G, G"},{"name":"H, H"},{"name":"I, I"},{"name":"J, J"}]}}]}"#;
        assert_eq!(
            parse_datacite(body),
            vec!["doi: 10.1234/many\tauthors: A, A, B, B, C, C, D, D, E, E, F, F, G, G, H, H, et al.".to_string()]
        );
    }
}
