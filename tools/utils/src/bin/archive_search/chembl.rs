use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://www.ebi.ac.uk/chembl/api/data/molecule/search";

pub fn chembl_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?q={}&format=json&limit={}",
        ENDPOINT,
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_chembl(&f.body);
            if out.is_empty() {
                vec![format!("absent — chembl carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — chembl HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_chembl(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(molecules) = v.get("molecules").and_then(|m| m.as_arr()) else {
        return out;
    };
    for molecule in molecules {
        let Some(id) = molecule.get("molecule_chembl_id").and_then(|m| m.as_str()) else {
            continue;
        };
        let mut line = format!(
            "url https://www.ebi.ac.uk/chembl/compound_report_card/{}/",
            id
        );
        if let Some(name) = molecule.get("pref_name").and_then(|p| p.as_str()) {
            line.push_str(&format!("\ttitle: {}", name));
        }
        if let Some(phase) = molecule.get("max_phase").and_then(|p| p.as_str()) {
            line.push_str(&format!("\tmax_phase: {}", phase));
        }
        if let Some(props) = molecule.get("molecule_properties") {
            if let Some(formula) = props.get("full_molformula").and_then(|f| f.as_str()) {
                line.push_str(&format!("\tformula: {}", formula));
            }
            if let Some(weight) = props.get("full_mwt").and_then(|w| w.as_str()) {
                line.push_str(&format!("\tweight: {}", weight));
            }
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_molecule_fields() {
        let body = r#"{"molecules":[{"molecule_chembl_id":"CHEMBL25","pref_name":"ASPIRIN","max_phase":"4.0","molecule_properties":{"full_molformula":"C9H8O4","full_mwt":"180.16"}}]}"#;
        assert_eq!(
            parse_chembl(body),
            vec!["url https://www.ebi.ac.uk/chembl/compound_report_card/CHEMBL25/\ttitle: ASPIRIN\tmax_phase: 4.0\tformula: C9H8O4\tweight: 180.16".to_string()]
        );
    }

    #[test]
    fn a_molecule_without_an_id_carries_nothing() {
        assert!(parse_chembl(r#"{"molecules":[{"pref_name":"x"}]}"#).is_empty());
    }
}
