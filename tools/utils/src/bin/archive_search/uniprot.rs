use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://rest.uniprot.org/uniprotkb/search";

pub fn uniprot_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?query={}&format=json&size={}&fields=accession,protein_name,organism_name",
        ENDPOINT,
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_uniprot(&f.body);
            if out.is_empty() {
                vec![format!("absent — uniprot carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — uniprot HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_uniprot(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for entry in results {
        let Some(acc) = entry.get("primaryAccession").and_then(|a| a.as_str()) else {
            continue;
        };
        let mut line = format!("url https://www.uniprot.org/uniprotkb/{}/entry", acc);
        if let Some(name) = entry
            .get("proteinDescription")
            .and_then(|p| p.get("recommendedName"))
            .and_then(|r| r.get("fullName"))
            .and_then(|f| f.get("value"))
            .and_then(|v| v.as_str())
        {
            line.push_str(&format!("\ttitle: {}", name));
        }
        if let Some(organism) = entry
            .get("organism")
            .and_then(|o| o.get("scientificName"))
            .and_then(|s| s.as_str())
        {
            line.push_str(&format!("\torganism: {}", organism));
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
        let body = r#"{"results":[{"primaryAccession":"Q6W5P4","proteinDescription":{"recommendedName":{"fullName":{"value":"Interleukin-13"}}},"organism":{"scientificName":"Homo sapiens"}}]}"#;
        assert_eq!(
            parse_uniprot(body),
            vec!["url https://www.uniprot.org/uniprotkb/Q6W5P4/entry\ttitle: Interleukin-13\torganism: Homo sapiens".to_string()]
        );
    }

    #[test]
    fn an_entry_without_an_accession_carries_nothing() {
        assert!(parse_uniprot(r#"{"results":[{"organism":{}}]}"#).is_empty());
    }
}
