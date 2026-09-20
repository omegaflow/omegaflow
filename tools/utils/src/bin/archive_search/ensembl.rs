use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://www.ebi.ac.uk/ebisearch/ws/rest/ensembl";

pub fn ensembl_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?query={}&format=json&size={}",
        ENDPOINT,
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_ensembl(&f.body);
            if out.is_empty() {
                vec![format!("absent — ensembl carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — ensembl HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_ensembl(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(entries) = v.get("entries").and_then(|e| e.as_arr()) else {
        return out;
    };
    for entry in entries {
        let Some(id) = entry.get("id").and_then(|i| i.as_str()) else {
            continue;
        };
        let mut line = format!("url https://www.ensembl.org/id/{}", id);
        if let Some(source) = entry.get("source").and_then(|s| s.as_str()) {
            line.push_str(&format!("\tsource: {}", source));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_entry_fields() {
        let body =
            r#"{"hitCount":1979,"entries":[{"id":"ENSG00000139618","source":"ensembl_gene"}]}"#;
        assert_eq!(
            parse_ensembl(body),
            vec![
                "url https://www.ensembl.org/id/ENSG00000139618\tsource: ensembl_gene".to_string()
            ]
        );
    }

    #[test]
    fn an_entry_without_an_id_carries_nothing() {
        assert!(parse_ensembl(r#"{"entries":[{"source":"ensembl_gene"}]}"#).is_empty());
    }
}
