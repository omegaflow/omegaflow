use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://www.ebi.ac.uk/QuickGO/services/ontology/go/search";

pub fn go_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?query={}&limit={}", ENDPOINT, urlencode(query), max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_go(&f.body);
            if out.is_empty() {
                vec![format!("absent — go carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — go HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_go(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for term in results {
        let Some(id) = term.get("id").and_then(|i| i.as_str()) else {
            continue;
        };
        let mut line = format!("url https://www.ebi.ac.uk/QuickGO/term/{}", id);
        if let Some(name) = term.get("name").and_then(|n| n.as_str()) {
            line.push_str(&format!("\ttitle: {}", name));
        }
        if let Some(definition) = term
            .get("definition")
            .and_then(|d| d.get("text"))
            .and_then(|t| t.as_str())
        {
            line.push_str(&format!("\tdefinition: {}", definition));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_term_fields() {
        let body = r#"{"numberOfHits":352,"results":[{"id":"GO:0097194","name":"execution phase of apoptosis","definition":{"text":"A stage of the apoptotic process."}}]}"#;
        assert_eq!(
            parse_go(body),
            vec!["url https://www.ebi.ac.uk/QuickGO/term/GO:0097194\ttitle: execution phase of apoptosis\tdefinition: A stage of the apoptotic process.".to_string()]
        );
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_go(r#"{"results":[]}"#).is_empty());
    }
}
