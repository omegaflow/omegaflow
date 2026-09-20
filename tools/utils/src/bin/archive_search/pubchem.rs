use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name";

pub fn pubchem_lines(query: &str) -> Vec<String> {
    let url = format!(
        "{}/{}/property/MolecularFormula,MolecularWeight,IUPACName/JSON",
        ENDPOINT,
        urlencode(query)
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_pubchem(&f.body);
            if out.is_empty() {
                vec![format!("absent — pubchem carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) if f.status == Some(404) => {
            vec![format!("absent — pubchem carries no entry: {}", query)]
        }
        Some(f) => vec![format!("pending — pubchem HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_pubchem(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(props) = v
        .get("PropertyTable")
        .and_then(|p| p.get("Properties"))
        .and_then(|p| p.as_arr())
    else {
        return out;
    };
    for prop in props {
        let Some(cid) = prop.get("CID").and_then(|c| c.as_scalar_string()) else {
            continue;
        };
        let mut line = format!("url https://pubchem.ncbi.nlm.nih.gov/compound/{}", cid);
        if let Some(name) = prop.get("IUPACName").and_then(|n| n.as_str()) {
            line.push_str(&format!("\ttitle: {}", name));
        }
        if let Some(formula) = prop.get("MolecularFormula").and_then(|f| f.as_str()) {
            line.push_str(&format!("\tformula: {}", formula));
        }
        if let Some(weight) = prop.get("MolecularWeight").and_then(|w| w.as_str()) {
            line.push_str(&format!("\tweight: {}", weight));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_property_table() {
        let body = r#"{"PropertyTable":{"Properties":[{"CID":2244,"MolecularFormula":"C9H8O4","MolecularWeight":"180.16","IUPACName":"acetylsalicylic acid"}]}}"#;
        assert_eq!(
            parse_pubchem(body),
            vec!["url https://pubchem.ncbi.nlm.nih.gov/compound/2244\ttitle: acetylsalicylic acid\tformula: C9H8O4\tweight: 180.16".to_string()]
        );
    }

    #[test]
    fn an_empty_property_table_carries_nothing() {
        assert!(parse_pubchem(r#"{"PropertyTable":{"Properties":[]}}"#).is_empty());
    }
}
