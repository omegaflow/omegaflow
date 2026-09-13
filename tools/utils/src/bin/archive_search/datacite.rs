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
}
