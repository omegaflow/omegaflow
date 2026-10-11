use crate::json;
use crate::net::{get, urlencode};

pub fn dataeuropa_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://data.europa.eu/api/hub/search/search?q={}&limit={}",
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the data.europa response carries no JSON".to_string()];
            };
            let out = parse_dataeuropa(&v);
            if out.is_empty() {
                vec![format!(
                    "absent — data.europa carries no dataset: {}",
                    query
                )]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — data.europa HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn pick_lang(o: &json::Json) -> Option<String> {
    for key in ["en", "de", "fr", "es"] {
        if let Some(s) = o
            .get(key)
            .and_then(|x| x.as_str())
            .filter(|s| !s.is_empty())
        {
            return Some(s.to_string());
        }
    }
    o.as_str().map(|s| s.to_string())
}

fn parse_dataeuropa(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v
        .get("result")
        .and_then(|r| r.get("results"))
        .and_then(|r| r.as_arr())
    else {
        return out;
    };
    for item in items {
        let id = item.get("id").and_then(|i| i.as_str()).unwrap_or("");
        let landing = item
            .get("landing_page")
            .and_then(|l| l.as_arr())
            .and_then(|a| a.first())
            .and_then(|e| e.get("resource"))
            .and_then(|r| r.as_str())
            .filter(|s| !s.is_empty());
        let url = match landing {
            Some(u) => u.to_string(),
            None if id.is_empty() => "https://data.europa.eu/".to_string(),
            None => format!("https://data.europa.eu/data/datasets/{}", id),
        };
        let mut line = format!("url {}", url);
        if let Some(title) = item
            .get("title")
            .and_then(pick_lang)
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(publisher) = item
            .get("publisher")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tpublisher: {}", publisher));
        }
        if let Some(country) = item
            .get("country")
            .and_then(|c| c.get("label"))
            .and_then(|l| l.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tcountry: {}", country));
        }
        if let Some(issued) = item
            .get("issued")
            .and_then(|i| i.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tissued: {}", issued));
        }
        if let Some(desc) = item.get("description").and_then(pick_lang) {
            let c = collapse(&desc);
            if !c.is_empty() {
                let capped: String = c.chars().take(300).collect();
                line.push_str(&format!("\tdescription: {}", capped));
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
    fn reads_the_items_with_the_english_title_and_landing_page() {
        let body = r#"{"result":{"count":1,"results":[{"id":"abc-1","title":{"en":"Energy data","de":"Energiedaten"},"description":{"en":"A short  note."},"publisher":{"name":"Eurostat"},"country":{"label":"Luxembourg"},"issued":"2024-04-23T06:30:00Z","landing_page":[{"resource":"https://example.org/dataset/abc-1"}]}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_dataeuropa(&v),
            vec!["url https://example.org/dataset/abc-1\ttitle: Energy data\tpublisher: Eurostat\tcountry: Luxembourg\tissued: 2024-04-23T06:30:00Z\tdescription: A short note.".to_string()]
        );
    }

    #[test]
    fn a_missing_landing_page_reads_the_dataset_path() {
        let body = r#"{"result":{"results":[{"id":"co2-2","title":{"en":"CO2"}}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_dataeuropa(&v),
            vec!["url https://data.europa.eu/data/datasets/co2-2\ttitle: CO2".to_string()]
        );
    }

    #[test]
    fn empty_results_carry_nothing() {
        let v = json::parse(r#"{"result":{"results":[]}}"#).unwrap();
        assert!(parse_dataeuropa(&v).is_empty());
    }
}
