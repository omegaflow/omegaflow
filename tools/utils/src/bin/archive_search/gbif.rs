use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.gbif.org/v1/species/search";

pub fn gbif_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?q={}&limit={}", ENDPOINT, urlencode(query), max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_gbif(&f.body);
            if out.is_empty() {
                vec![format!("absent — gbif carries no taxon: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — gbif HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_gbif(body: &str) -> Vec<String> {
    let Some(v) = json::parse(body) else {
        return vec!["pending — the gbif response carries no JSON".to_string()];
    };
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return vec!["pending — the gbif response carries no JSON".to_string()];
    };
    let mut out = Vec::new();
    for entry in results {
        let Some(key) = entry.get("key").and_then(|k| k.as_scalar_string()) else {
            continue;
        };
        let mut line = format!("url https://www.gbif.org/species/{}", key);
        if let Some(name) = entry.get("scientificName").and_then(|s| s.as_str()) {
            line.push_str(&format!("\tscientific_name: {}", name));
        }
        if let Some(rank) = entry.get("rank").and_then(|r| r.as_str()) {
            line.push_str(&format!("\trank: {}", rank));
        }
        if let Some(status) = entry.get("taxonomicStatus").and_then(|s| s.as_str()) {
            line.push_str(&format!("\tstatus: {}", status));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_taxon_fields() {
        let body = r#"{"results":[{"key":335094696,"scientificName":"Quercusia quercus","canonicalName":"Quercusia quercus","rank":"SPECIES","taxonomicStatus":"ACCEPTED"}]}"#;
        assert_eq!(
            parse_gbif(body),
            vec!["url https://www.gbif.org/species/335094696\tscientific_name: Quercusia quercus\trank: SPECIES\tstatus: ACCEPTED".to_string()]
        );
    }

    #[test]
    fn a_taxon_without_a_rank_carries_no_rank() {
        let body = r#"{"results":[{"key":176557110,"scientificName":"Quercusia quercus","taxonomicStatus":"SYNONYM"}]}"#;
        assert_eq!(
            parse_gbif(body),
            vec!["url https://www.gbif.org/species/176557110\tscientific_name: Quercusia quercus\tstatus: SYNONYM".to_string()]
        );
    }

    #[test]
    fn a_taxon_without_a_key_carries_nothing() {
        let body = r#"{"results":[{"scientificName":"Quercus"}]}"#;
        assert!(parse_gbif(body).is_empty());
    }
}
