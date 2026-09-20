use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://www.ebi.ac.uk/interpro/api/entry/all/protein/reviewed/";

pub fn interpro_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?search={}", ENDPOINT, urlencode(query));
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_interpro(&f.body, max);
            if out.is_empty() {
                vec![format!("absent — interpro carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — interpro HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_interpro(body: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for record in results {
        if out.len() >= max {
            return out;
        }
        let Some(metadata) = record.get("metadata") else {
            continue;
        };
        let Some(accession) = field(metadata, "accession") else {
            continue;
        };
        let Some(source) = field(metadata, "source_database") else {
            continue;
        };
        let mut line = format!(
            "url https://www.ebi.ac.uk/interpro/entry/{}/{}",
            source, accession
        );
        if let Some(name) = field(metadata, "name") {
            line.push_str(&format!("\tname: {}", name));
        }
        if let Some(kind) = field(metadata, "type") {
            line.push_str(&format!("\ttype: {}", kind));
        }
        line.push_str(&format!("\tsource: {}", source));
        if let Some(integrated) = field(metadata, "integrated") {
            line.push_str(&format!("\tintegrated: {}", integrated));
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
        let body = r#"{"count":1,"results":[{"metadata":{"accession":"IPR001697","name":"Pyruvate kinase","source_database":"interpro","type":"domain","integrated":null}}]}"#;
        assert_eq!(
            parse_interpro(body, 10),
            vec!["url https://www.ebi.ac.uk/interpro/entry/interpro/IPR001697\tname: Pyruvate kinase\ttype: domain\tsource: interpro".to_string()]
        );
    }

    #[test]
    fn a_member_database_entry_carries_its_own_source_in_the_url() {
        let body = r#"{"count":1,"results":[{"metadata":{"accession":"cd00029","name":"C1 domain","source_database":"cdd","type":"domain","integrated":"IPR000159"}}]}"#;
        assert_eq!(
            parse_interpro(body, 10),
            vec!["url https://www.ebi.ac.uk/interpro/entry/cdd/cd00029\tname: C1 domain\ttype: domain\tsource: cdd\tintegrated: IPR000159".to_string()]
        );
    }

    #[test]
    fn an_entry_without_an_accession_carries_nothing() {
        assert!(parse_interpro(r#"{"results":[{"metadata":{"name":"x"}}]}"#, 10).is_empty());
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_interpro(r#"{"count":0,"results":[]}"#, 10).is_empty());
    }
}
