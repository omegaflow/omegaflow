use crate::json;
use crate::net::{get, urlencode};

const DEFAULT_PORTAL: &str = "ckan.publishing.service.gov.uk";

pub fn ckan_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(query, &["portal"]);
    let portal = crate::refine::value_of(&refine, "portal").unwrap_or(DEFAULT_PORTAL);
    let url = format!(
        "https://{}/api/3/action/package_search?q={}&rows={}",
        portal,
        urlencode(&text),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the ckan response carries no JSON".to_string()];
            };
            let out = parse_ckan(&v, portal);
            if out.is_empty() {
                vec![format!("absent — ckan carries no dataset: {}", text)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — ckan HTTP {}", f.status_text())],
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

fn parse_ckan(v: &json::Json, portal: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v
        .get("result")
        .and_then(|r| r.get("results"))
        .and_then(|r| r.as_arr())
    else {
        return out;
    };
    for item in items {
        let name = item.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let url = match item
            .get("url")
            .and_then(|u| u.as_str())
            .filter(|s| !s.is_empty())
        {
            Some(u) => u.to_string(),
            None if name.is_empty() => format!("https://{}", portal),
            None => format!("https://{}/dataset/{}", portal, name),
        };
        let mut line = format!("url {}", url);
        if let Some(title) = item
            .get("title")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(org) = item
            .get("organization")
            .and_then(|o| o.get("title"))
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\torganization: {}", org));
        }
        if let Some(res) = item.get("num_resources").and_then(|n| n.as_scalar_string()) {
            line.push_str(&format!("\tresources: {}", res));
        }
        if let Some(notes) = item
            .get("notes")
            .and_then(|n| n.as_str())
            .filter(|s| !s.is_empty())
        {
            let c = collapse(&strip_tags(notes));
            if !c.is_empty() {
                let capped: String = c.chars().take(300).collect();
                line.push_str(&format!("\tnotes: {}", capped));
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
        let body = r#"{"result":{"results":[{"name":"climate-x","title":"Climate data","url":"https://example.org/dataset/climate-x","num_resources":6,"organization":{"title":"Met Office"},"notes":"<p>A short  note.</p>"}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_ckan(&v, "ckan.publishing.service.gov.uk"),
            vec!["url https://example.org/dataset/climate-x\ttitle: Climate data\torganization: Met Office\tresources: 6\tnotes: A short note.".to_string()]
        );
    }

    #[test]
    fn a_missing_url_reads_the_portal_dataset_path() {
        let body = r#"{"result":{"results":[{"name":"co2","title":"CO2"}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_ckan(&v, "data.gov.example"),
            vec!["url https://data.gov.example/dataset/co2\ttitle: CO2".to_string()]
        );
    }

    #[test]
    fn empty_results_carry_nothing() {
        let v = json::parse(r#"{"result":{"results":[]}}"#).unwrap();
        assert!(parse_ckan(&v, "p").is_empty());
    }
}
