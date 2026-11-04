use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://www.arcgis.com/sharing/rest/search";
const ITEM: &str = "https://www.arcgis.com/home/item.html?id=";

pub fn arcgis_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?q={}&num={}&f=json", ENDPOINT, urlencode(query), max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the arcgis response carries no JSON".to_string()];
            };
            let out = parse_arcgis(&v);
            if out.is_empty() {
                vec![format!("absent — arcgis carries no item: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — arcgis HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_arcgis(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("results").and_then(|r| r.as_arr()) else {
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
        let mut line = format!("url {}{}", ITEM, id);
        if let Some(title) = item
            .get("title")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(kind) = item
            .get("type")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttype: {}", kind));
        }
        if let Some(owner) = item
            .get("owner")
            .and_then(|o| o.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\towner: {}", owner));
        }
        if let Some(snippet) = item
            .get("snippet")
            .and_then(|s| s.as_str())
            .filter(|s| !s.is_empty())
        {
            let c = collapse(snippet);
            let capped: String = c.chars().take(300).collect();
            line.push_str(&format!("\tsnippet: {}", capped));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_items() {
        let body = r#"{"total":1,"results":[{"id":"abc123","title":"Soil Moisture","type":"Feature Service","owner":"esri","snippet":"A dataset."}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_arcgis(&v),
            vec!["url https://www.arcgis.com/home/item.html?id=abc123\ttitle: Soil Moisture\ttype: Feature Service\towner: esri\tsnippet: A dataset.".to_string()]
        );
    }

    #[test]
    fn skips_items_without_id() {
        let v = json::parse(r#"{"results":[{"title":"No id"}]}"#).unwrap();
        assert!(parse_arcgis(&v).is_empty());
    }

    #[test]
    fn empty_results_carry_nothing() {
        let v = json::parse(r#"{"results":[]}"#).unwrap();
        assert!(parse_arcgis(&v).is_empty());
    }
}
