use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://doaj.org/api/search/articles";

pub fn doaj_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}/{}?pageSize={}", ENDPOINT, urlencode(query), max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_doaj(&f.body);
            if out.is_empty() {
                vec![format!("absent — doaj carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — doaj HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn doi_of(bibjson: &json::Json) -> Option<String> {
    bibjson
        .get("identifier")?
        .as_arr()?
        .iter()
        .find(|i| i.get("type").and_then(|t| t.as_str()) == Some("doi"))
        .and_then(|i| i.get("id").and_then(|v| v.as_str()))
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_doaj(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for record in results {
        let Some(bibjson) = record.get("bibjson") else {
            continue;
        };
        let doi = doi_of(bibjson);
        let mut line = String::from("url ");
        match (&doi, record.get("id").and_then(|i| i.as_str())) {
            (Some(doi), _) => line.push_str(&format!("https://doi.org/{}", doi)),
            (None, Some(id)) => line.push_str(&format!("https://doaj.org/article/{}", id)),
            (None, None) => continue,
        }
        if let Some(title) = bibjson.get("title").and_then(|t| t.as_str()) {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(journal) = bibjson
            .get("journal")
            .and_then(|j| j.get("title"))
            .and_then(|t| t.as_str())
        {
            line.push_str(&format!("\tjournal: {}", journal));
        }
        if let Some(doi) = doi {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_article_fields() {
        let body = r#"{"total":1,"results":[{"id":"abc","bibjson":{"title":"Asthma care","journal":{"title":"Frontiers"},"identifier":[{"type":"eissn","id":"2296-2360"},{"type":"doi","id":"10.3389/fped.2019.00372"}]}}]}"#;
        assert_eq!(
            parse_doaj(body),
            vec!["url https://doi.org/10.3389/fped.2019.00372\ttitle: Asthma care\tjournal: Frontiers\tdoi: 10.3389/fped.2019.00372".to_string()]
        );
    }

    #[test]
    fn an_article_without_doi_reads_the_doaj_url() {
        let body = r#"{"results":[{"id":"abc","bibjson":{"title":"x","identifier":[]}}]}"#;
        assert_eq!(
            parse_doaj(body),
            vec!["url https://doaj.org/article/abc\ttitle: x".to_string()]
        );
    }
}
