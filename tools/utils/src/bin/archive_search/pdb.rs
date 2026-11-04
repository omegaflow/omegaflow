use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://www.ebi.ac.uk/pdbe/search/pdb/select";

pub fn pdb_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?q={}&wt=json&rows={}&fl=pdb_id,title",
        ENDPOINT,
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_pdb(&f.body);
            if out.is_empty() {
                vec![format!("absent — pdb carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — pdb HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_pdb(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(docs) = v
        .get("response")
        .and_then(|r| r.get("docs"))
        .and_then(|d| d.as_arr())
    else {
        return out;
    };
    for doc in docs {
        let Some(id) = doc.get("pdb_id").and_then(|p| p.as_str()) else {
            continue;
        };
        let mut line = format!(
            "url https://www.ebi.ac.uk/pdbe/entry/pdb/{}",
            id.to_uppercase()
        );
        if let Some(title) = doc.get("title").and_then(|t| t.as_str()) {
            line.push_str(&format!("\ttitle: {}", title));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_doc_fields() {
        let body = r#"{"response":{"numFound":3823,"docs":[{"pdb_id":"3f6z","title":"Crystal structure of MliC"}]}}"#;
        assert_eq!(
            parse_pdb(body),
            vec![
                "url https://www.ebi.ac.uk/pdbe/entry/pdb/3F6Z\ttitle: Crystal structure of MliC"
                    .to_string()
            ]
        );
    }

    #[test]
    fn an_empty_response_carries_nothing() {
        assert!(parse_pdb(r#"{"response":{"docs":[]}}"#).is_empty());
    }
}
