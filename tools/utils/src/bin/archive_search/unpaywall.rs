use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.unpaywall.org/v2";

pub fn unpaywall_lines(query: &str) -> Vec<String> {
    let Some(email) = crate::token::secret("UNPAYWALL_EMAIL") else {
        return vec![
            "pending — UNPAYWALL_EMAIL absent from .secrets.local/.env (Unpaywall requires a contact email)"
                .to_string(),
        ];
    };
    let doi = query
        .trim()
        .trim_start_matches("https://doi.org/")
        .trim_start_matches("http://doi.org/")
        .trim_start_matches("doi:");
    let url = format!(
        "{}/{}?email={}",
        ENDPOINT,
        urlencode(doi),
        urlencode(&email)
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_unpaywall(&f.body);
            if out.is_empty() {
                vec![format!("absent — unpaywall carries no entry: {}", doi)]
            } else {
                out
            }
        }
        Some(f) if f.status == Some(404) || f.status == Some(422) => {
            vec![format!("absent — unpaywall carries no entry: {}", doi)]
        }
        Some(f) => vec![format!("pending — unpaywall HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_unpaywall(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(doi) = v.get("doi").and_then(|d| d.as_str()) else {
        return out;
    };
    let oa_url = v
        .get("best_oa_location")
        .and_then(|l| l.get("url"))
        .and_then(|u| u.as_str())
        .filter(|u| !u.is_empty());
    let mut line = match oa_url {
        Some(url) => format!("url {}", url),
        None => format!("url https://doi.org/{}", doi),
    };
    if let Some(title) = v.get("title").and_then(|t| t.as_str()) {
        line.push_str(&format!("\ttitle: {}", title));
    }
    line.push_str(&format!("\tdoi: {}", doi));
    if let Some(json::Json::Bool(oa)) = v.get("is_oa") {
        line.push_str(&format!("\tis_oa: {}", oa));
    }
    if let Some(host) = v
        .get("best_oa_location")
        .and_then(|l| l.get("host_type"))
        .and_then(|h| h.as_str())
    {
        line.push_str(&format!("\thost: {}", host));
    }
    out.push(line);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_oa_location() {
        let body = r#"{"doi":"10.1038/nature12373","title":"Nanometre-scale","is_oa":true,"best_oa_location":{"url":"https://www.nature.com/articles/nature12373","host_type":"publisher"}}"#;
        assert_eq!(
            parse_unpaywall(body),
            vec!["url https://www.nature.com/articles/nature12373\ttitle: Nanometre-scale\tdoi: 10.1038/nature12373\tis_oa: true\thost: publisher".to_string()]
        );
    }

    #[test]
    fn without_an_oa_location_reads_the_doi_url() {
        let body = r#"{"doi":"10.1/x","is_oa":false}"#;
        assert_eq!(
            parse_unpaywall(body),
            vec!["url https://doi.org/10.1/x\tdoi: 10.1/x\tis_oa: false".to_string()]
        );
    }
}
