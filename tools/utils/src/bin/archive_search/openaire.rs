use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.openaire.eu/graph/v1/researchProducts";

pub fn openaire_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?search={}&pageSize={}", ENDPOINT, urlencode(query), max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the openaire response carries no JSON".to_string()];
            };
            let out = parse_openaire(&v, max);
            if out.is_empty() {
                vec![format!("absent — openaire carries no result: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — openaire HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn first_doi(pids: &json::Json) -> Option<String> {
    let arr = pids.as_arr()?;
    for p in arr {
        if p.get("scheme").and_then(|s| s.as_str()) == Some("doi") {
            if let Some(v) = p
                .get("value")
                .and_then(|s| s.as_str())
                .filter(|s| !s.is_empty())
            {
                return Some(v.to_string());
            }
        }
    }
    None
}

fn item_doi(item: &json::Json) -> Option<String> {
    if let Some(doi) = item.get("pids").and_then(first_doi) {
        return Some(doi);
    }
    let instances = item.get("instances").and_then(|i| i.as_arr())?;
    for inst in instances {
        if let Some(doi) = inst.get("pids").and_then(first_doi) {
            return Some(doi);
        }
    }
    None
}

fn item_url(item: &json::Json) -> Option<String> {
    let instances = item.get("instances").and_then(|i| i.as_arr())?;
    for inst in instances {
        if let Some(urls) = inst.get("urls").and_then(|u| u.as_arr()) {
            if let Some(url) = urls
                .first()
                .and_then(|u| u.as_str())
                .filter(|s| !s.is_empty())
            {
                return Some(url.to_string());
            }
        }
    }
    None
}

fn parse_openaire(v: &json::Json, max: usize) -> Vec<String> {
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
        let url = match item_doi(item) {
            Some(doi) => format!("https://doi.org/{}", doi),
            None => match item_url(item) {
                Some(u) => u,
                None => id.to_string(),
            },
        };
        let mut line = format!("url {}", url);
        if let Some(title) = item
            .get("mainTitle")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", collapse(title)));
        }
        if let Some(date) = item
            .get("publicationDate")
            .and_then(|d| d.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tdate: {}", date));
        }
        if let Some(publisher) = item
            .get("publisher")
            .and_then(|p| p.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tpublisher: {}", collapse(publisher)));
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

    fn first_fixture() -> json::Json {
        let body = r#"{"header":{"numFound":1,"page":1,"pageSize":10},"results":[{"id":"doi_dedup___::673434ddc10e3f684fc966065606f18e","mainTitle":"Neutron Star Mass Distribution in Binaries","type":"publication","publicationDate":"2016-05-01","publisher":"IOP Publishing","pids":[{"scheme":"doi","value":"10.1088/1742-6596/716/1/012021"}],"instances":[{"urls":["https://doi.org/10.1088/1742-6596/716/1/012021"]}]}]}"#;
        json::parse(body).unwrap()
    }

    #[test]
    fn reads_the_doi_and_fields() {
        assert_eq!(
            parse_openaire(&first_fixture(), 10),
            vec!["url https://doi.org/10.1088/1742-6596/716/1/012021\ttitle: Neutron Star Mass Distribution in Binaries\tdate: 2016-05-01\tpublisher: IOP Publishing".to_string()]
        );
    }

    #[test]
    fn falls_back_to_instance_url_without_a_doi() {
        let body = r#"{"results":[{"id":"openaire____::x","mainTitle":"No DOI here","instances":[{"urls":["https://example.org/x"]}]}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_openaire(&v, 10),
            vec!["url https://example.org/x\ttitle: No DOI here".to_string()]
        );
    }

    #[test]
    fn falls_back_to_id_and_caps_at_max() {
        let body = r#"{"results":[{"id":"openaire____::a","mainTitle":"First"},{"id":"openaire____::b","mainTitle":"Second"}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_openaire(&v, 1),
            vec!["url openaire____::a\ttitle: First".to_string()]
        );
    }
}
