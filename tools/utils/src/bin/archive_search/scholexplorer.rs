use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.scholexplorer.openaire.eu/v3/Links";
const SCHEMES: [&str; 4] = ["doi", "pmid", "pmc", "arxiv"];

pub fn scholexplorer_lines(query: &str, max: usize) -> Vec<String> {
    let Some(pid) = identifier(query) else {
        return vec![format!(
            "usage — scholexplorer links by DOI or PMID, not by keyword: {}",
            query
        )];
    };
    let url = format!("{}?sourcePid={}", ENDPOINT, urlencode(&pid));
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match parse_scholexplorer(&f.body, max) {
            Some(out) if out.is_empty() => {
                vec![format!("absent — scholexplorer carries no link: {}", query)]
            }
            Some(out) => out,
            None => vec!["pending — the scholexplorer response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — scholexplorer HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn identifier(query: &str) -> Option<String> {
    let stripped = query
        .trim()
        .trim_start_matches("https://doi.org/")
        .trim_start_matches("http://doi.org/")
        .trim_start_matches("https://dx.doi.org/")
        .trim_start_matches("http://dx.doi.org/")
        .trim_start_matches("doi:")
        .trim_start_matches("pmid:");
    let s = stripped.trim();
    if s.starts_with("10.") && s.contains('/') {
        return Some(s.to_string());
    }
    if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) {
        return Some(s.to_string());
    }
    None
}

fn pick(node: &json::Json) -> Option<(String, Option<String>)> {
    let ids = node.get("Identifier").and_then(|i| i.as_arr())?;
    let chosen = SCHEMES
        .iter()
        .find_map(|scheme| {
            ids.iter().find(|e| {
                e.get("IDScheme").and_then(|s| s.as_str()) == Some(*scheme)
                    && e.get("ID")
                        .and_then(|v| v.as_str())
                        .filter(|s| !s.is_empty())
                        .is_some()
            })
        })
        .or_else(|| {
            ids.iter().find(|e| {
                e.get("ID")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                    .is_some()
            })
        })?;
    let pid = chosen.get("ID").and_then(|v| v.as_str())?.to_string();
    let url = chosen
        .get("IDURL")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    Some((pid, url))
}

fn parse_scholexplorer(body: &str, max: usize) -> Option<Vec<String>> {
    let v = json::parse(body)?;
    let mut out = Vec::new();
    let items = v.get("result").and_then(|r| r.as_arr())?;
    for item in items {
        if out.len() >= max {
            break;
        }
        let Some(target) = item.get("target") else {
            continue;
        };
        let Some((pid, url)) = pick(target) else {
            continue;
        };
        let rel = item.get("RelationshipType");
        let name = rel.and_then(|r| r.get("Name")).and_then(|n| n.as_str());
        let sub = rel.and_then(|r| r.get("SubType")).and_then(|n| n.as_str());
        let mut line = match &url {
            Some(u) => format!("url {}\ttarget: {}", u, pid),
            None => format!("target: {}", pid),
        };
        match (
            name.filter(|s| !s.is_empty()),
            sub.filter(|s| !s.is_empty()),
        ) {
            (Some(name), Some(sub)) => line.push_str(&format!("\ttype: {} ({})", name, sub)),
            (Some(name), None) => line.push_str(&format!("\ttype: {}", name)),
            (None, Some(sub)) => line.push_str(&format!("\ttype: {}", sub)),
            (None, None) => {}
        }
        out.push(line);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_doi_target_link() {
        let body = r#"{"result":[{"RelationshipType":{"Name":"IsRelatedTo","SubType":"cites"},"target":{"Identifier":[{"ID":"50|x","IDScheme":"openaireIdentifier","IDURL":"https://explore.openaire.eu/x"},{"ID":"10.1021/jp073938o","IDScheme":"doi","IDURL":"https://doi.org/10.1021/jp073938o"}]}}]}"#;
        assert_eq!(
            parse_scholexplorer(body, 10).unwrap(),
            vec!["url https://doi.org/10.1021/jp073938o\ttarget: 10.1021/jp073938o\ttype: IsRelatedTo (cites)".to_string()]
        );
    }

    #[test]
    fn skips_a_target_without_identifier() {
        let body = r#"{"result":[{"RelationshipType":{"Name":"IsRelatedTo"},"target":{"Identifier":[]}},{"RelationshipType":{"Name":"Cites"},"target":{"Identifier":[{"ID":"10.1/x","IDScheme":"doi","IDURL":"https://doi.org/10.1/x"}]}}]}"#;
        assert_eq!(
            parse_scholexplorer(body, 10).unwrap(),
            vec!["url https://doi.org/10.1/x\ttarget: 10.1/x\ttype: Cites".to_string()]
        );
    }

    #[test]
    fn caps_the_links_at_max() {
        let body = r#"{"result":[{"RelationshipType":{"Name":"Cites"},"target":{"Identifier":[{"ID":"10.1/a","IDScheme":"doi","IDURL":"https://doi.org/10.1/a"}]}},{"RelationshipType":{"Name":"Cites"},"target":{"Identifier":[{"ID":"10.1/b","IDScheme":"doi","IDURL":"https://doi.org/10.1/b"}]}}]}"#;
        assert_eq!(parse_scholexplorer(body, 1).unwrap().len(), 1);
    }
}
