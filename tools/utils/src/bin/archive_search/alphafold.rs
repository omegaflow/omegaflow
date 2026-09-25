use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://alphafold.ebi.ac.uk/api/prediction/";

pub fn alphafold_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}{}", ENDPOINT, urlencode(query));
    match get(&url, &[], "40") {
        Some(f) => alphafold_from_fetch(&f, query, max),
        None => vec!["pending — no network".to_string()],
    }
}

fn alphafold_from_fetch(f: &crate::net::Fetch, query: &str, max: usize) -> Vec<String> {
    match f.status {
        Some(200) => {
            let out = parse_alphafold(&f.body, max);
            if out.is_empty() {
                vec![format!("absent — alphafold carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(400) => vec![format!(
            "absent — alphafold needs a UniProt accession (e.g. P00533): {}",
            query
        )],
        _ => vec![format!("pending — alphafold HTTP {}", f.status_text())],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn number(v: &json::Json, key: &str) -> Option<String> {
    match v.get(key) {
        Some(json::Json::Num(n)) if n.is_finite() => Some(format!("{}", n)),
        _ => None,
    }
}

fn parse_alphafold(body: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(entries) = v.as_arr() else {
        return out;
    };
    for entry in entries {
        if out.len() >= max {
            return out;
        }
        let Some(entry_id) = field(entry, "entryId") else {
            continue;
        };
        let Some(pdb) = field(entry, "pdbUrl") else {
            continue;
        };
        let mut line = format!("url {}", pdb);
        line.push_str(&format!("\tentry: {}", entry_id));
        if let Some(accession) = field(entry, "uniprotAccession") {
            line.push_str(&format!("\tuniprot: {}", accession));
        }
        if let Some(uniprot_id) = field(entry, "uniprotId") {
            line.push_str(&format!("\tuniprot_id: {}", uniprot_id));
        }
        if let Some(description) = field(entry, "uniprotDescription") {
            line.push_str(&format!("\tdescription: {}", description));
        }
        if let Some(gene) = field(entry, "gene") {
            line.push_str(&format!("\tgene: {}", gene));
        }
        if let Some(organism) = field(entry, "organismScientificName") {
            line.push_str(&format!("\torganism: {}", organism));
        }
        if let Some(metric) = number(entry, "globalMetricValue") {
            line.push_str(&format!("\tplddt: {}", metric));
        }
        if let Some(date) = field(entry, "sequenceVersionDate") {
            line.push_str(&format!("\tsequence_date: {}", date));
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
        let body = r#"[{"entryId":"AF-P00533-F1","uniprotAccession":"P00533","uniprotId":"EGFR_HUMAN","uniprotDescription":"Epidermal growth factor receptor","gene":"EGFR","organismScientificName":"Homo sapiens","sequenceVersionDate":"1997-11-01T00:00:00Z","pdbUrl":"https://alphafold.ebi.ac.uk/files/AF-P00533-F1-model_v6.pdb","globalMetricValue":75.94}]"#;
        assert_eq!(
            parse_alphafold(body, 10),
            vec!["url https://alphafold.ebi.ac.uk/files/AF-P00533-F1-model_v6.pdb\tentry: AF-P00533-F1\tuniprot: P00533\tuniprot_id: EGFR_HUMAN\tdescription: Epidermal growth factor receptor\tgene: EGFR\torganism: Homo sapiens\tplddt: 75.94\tsequence_date: 1997-11-01T00:00:00Z".to_string()]
        );
    }

    #[test]
    fn an_entry_without_a_pdb_url_carries_nothing() {
        assert!(parse_alphafold(r#"[{"entryId":"AF-X-F1"}]"#, 10).is_empty());
    }

    #[test]
    fn a_non_array_body_carries_nothing() {
        assert!(parse_alphafold("{}", 10).is_empty());
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_alphafold("[]", 10).is_empty());
    }

    #[test]
    fn http_400_names_the_accession_requirement() {
        let f = crate::net::Fetch {
            status: Some(400),
            body: String::new(),
            raw: Vec::new(),
            retry_after: None,
            complete: true,
        };
        assert_eq!(
            alphafold_from_fetch(&f, "solar wind", 10),
            vec![
                "absent — alphafold needs a UniProt accession (e.g. P00533): solar wind"
                    .to_string()
            ]
        );
    }
}
