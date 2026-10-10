use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://cmr.earthdata.nasa.gov/search/collections.json";
const CONCEPT: &str = "https://cmr.earthdata.nasa.gov/search/concepts/";

pub fn cmr_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?keyword={}&page_size={}",
        ENDPOINT,
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the cmr response carries no JSON".to_string()];
            };
            let out = parse_cmr(&v);
            if out.is_empty() {
                vec![format!("absent — cmr carries no collection: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — cmr HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_cmr(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(entry) = v.get("feed").and_then(|f| f.get("entry")) else {
        return out;
    };
    let entries: Vec<&json::Json> = match entry.as_arr() {
        Some(arr) => arr.iter().collect(),
        None => vec![entry],
    };
    for entry in entries {
        let Some(id) = entry.get("id").and_then(|i| i.as_str()) else {
            continue;
        };
        let mut line = format!("url {}{}", CONCEPT, id);
        if let Some(title) = entry
            .get("title")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(short) = entry
            .get("short_name")
            .and_then(|s| s.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tshort_name: {}", short));
        }
        if let Some(provider) = entry
            .get("provider_id")
            .and_then(|p| p.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tprovider: {}", provider));
        }
        if let Some(summary) = entry
            .get("summary")
            .and_then(|s| s.as_str())
            .filter(|s| !s.is_empty())
        {
            let c = collapse(summary);
            let capped: String = c.chars().take(300).collect();
            line.push_str(&format!("\tsummary: {}", capped));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_entries() {
        let body = r#"{"feed":{"entry":[{"id":"C123-X","title":"Soil Moisture","short_name":"SMAP","provider_id":"NASA","summary":"A short  summary."}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_cmr(&v),
            vec!["url https://cmr.earthdata.nasa.gov/search/concepts/C123-X\ttitle: Soil Moisture\tshort_name: SMAP\tprovider: NASA\tsummary: A short summary.".to_string()]
        );
    }

    #[test]
    fn a_single_entry_object_is_read_as_one_collection() {
        let body = r#"{"feed":{"entry":{"id":"C1","title":"One"}}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_cmr(&v),
            vec!["url https://cmr.earthdata.nasa.gov/search/concepts/C1\ttitle: One".to_string()]
        );
    }

    #[test]
    fn empty_feed_carries_nothing() {
        let v = json::parse(r#"{"feed":{"entry":[]}}"#).unwrap();
        assert!(parse_cmr(&v).is_empty());
    }
}
