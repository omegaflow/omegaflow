use crate::json;
use crate::net::{get, urlencode};

const XREFS: &str = "https://rest.ensembl.org/xrefs/symbol/homo_sapiens";
const LOOKUP: &str = "https://rest.ensembl.org/lookup/id";

pub fn ensembl_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{XREFS}/{}?content-type=application/json", urlencode(query));
    let mut out = match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => parse_xrefs(&f.body),
        Some(f) => return vec![format!("pending — ensembl HTTP {}", f.status_text())],
        None => return vec!["pending — no network".to_string()],
    };
    if out.is_empty() {
        out = lookup_lines(query);
    }
    if out.is_empty() {
        return vec![format!("absent — ensembl carries no entry: {}", query)];
    }
    out.truncate(max);
    out
}

fn lookup_lines(query: &str) -> Vec<String> {
    let url = format!(
        "{LOOKUP}/{}?content-type=application/json",
        urlencode(query)
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => parse_lookup(&f.body),
        _ => Vec::new(),
    }
}

fn parse_xrefs(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(entries) = v.as_arr() else {
        return out;
    };
    for entry in entries {
        let Some(id) = entry.get("id").and_then(|i| i.as_str()) else {
            continue;
        };
        let mut line = format!("url https://www.ensembl.org/id/{}", id);
        if let Some(source) = entry.get("type").and_then(|s| s.as_str()) {
            line.push_str(&format!("\tsource: {}", source));
        }
        out.push(line);
    }
    out
}

fn parse_lookup(body: &str) -> Vec<String> {
    let Some(v) = json::parse(body) else {
        return Vec::new();
    };
    let Some(id) = v.get("id").and_then(|i| i.as_str()) else {
        return Vec::new();
    };
    let mut line = format!("url https://www.ensembl.org/id/{}", id);
    if let Some(source) = v.get("biotype").and_then(|s| s.as_str()) {
        line.push_str(&format!("\tsource: {}", source));
    }
    vec![line]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_xref_fields() {
        let body = r#"[{"id":"ENSG00000141510","type":"gene"},{"id":"LRG_321","type":"gene"}]"#;
        assert_eq!(
            parse_xrefs(body),
            vec![
                "url https://www.ensembl.org/id/ENSG00000141510\tsource: gene".to_string(),
                "url https://www.ensembl.org/id/LRG_321\tsource: gene".to_string()
            ]
        );
    }

    #[test]
    fn an_xref_without_an_id_carries_nothing() {
        assert!(parse_xrefs(r#"[{"type":"gene"}]"#).is_empty());
    }

    #[test]
    fn reads_the_lookup_id_and_biotype() {
        let body = r#"{"id":"ENSG00000141510","display_name":"TP53","biotype":"protein_coding"}"#;
        assert_eq!(
            parse_lookup(body),
            vec![
                "url https://www.ensembl.org/id/ENSG00000141510\tsource: protein_coding"
                    .to_string()
            ]
        );
    }
}
