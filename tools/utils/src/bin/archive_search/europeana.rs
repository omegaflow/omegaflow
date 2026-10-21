use crate::json;
use crate::net::{get, urlencode};

pub fn europeana_lines(query: &str, key: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://api.europeana.eu/record/v2/search.json?wskey={}&query={}&rows={}",
        urlencode(key),
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let lines = parse_europeana(&f.body);
            if lines.is_empty() {
                vec![format!("absent — Europeana carries no item: {}", query)]
            } else {
                lines
            }
        }
        Some(f) => vec![format!("pending — europeana HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn first(value: Option<&json::Json>) -> Option<String> {
    match value? {
        json::Json::Str(s) if !s.trim().is_empty() => Some(s.trim().to_string()),
        json::Json::Arr(items) => items
            .iter()
            .find_map(|i| i.as_str().filter(|s| !s.trim().is_empty()))
            .map(|s| s.trim().to_string()),
        _ => None,
    }
}

fn parse_europeana(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(items) = v.get("items").and_then(|i| i.as_arr()) else {
        return out;
    };
    for item in items {
        let Some(url) = first(item.get("guid")).or_else(|| first(item.get("id"))) else {
            continue;
        };
        let mut line = format!("url {url}");
        if let Some(title) = first(item.get("title")) {
            line.push_str(&format!("\ttitle: {title}"));
        }
        if let Some(provider) = first(item.get("dataProvider")) {
            line.push_str(&format!("\tprovider: {provider}"));
        }
        if let Some(kind) = first(item.get("type")) {
            line.push_str(&format!("\ttype: {kind}"));
        }
        if let Some(rights) = first(item.get("rights")) {
            line.push_str(&format!("\trights: {rights}"));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_item_fields() {
        let body = r#"{"success":true,"items":[{"guid":"https://www.europeana.eu/item/1","title":["Solar wind"],"dataProvider":["NASA"],"type":"IMAGE","rights":["http://creativecommons.org/licenses/by/4.0/"]}]}"#;
        assert_eq!(
            parse_europeana(body),
            vec!["url https://www.europeana.eu/item/1\ttitle: Solar wind\tprovider: NASA\ttype: IMAGE\trights: http://creativecommons.org/licenses/by/4.0/".to_string()]
        );
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_europeana(r#"{"success":true,"items":[]}"#).is_empty());
    }
}
