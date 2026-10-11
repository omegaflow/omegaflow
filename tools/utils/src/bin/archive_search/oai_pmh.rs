use crate::net::get;

pub fn oai_pmh_lines(query: &str, max: usize) -> Vec<String> {
    let (_rest, refine) = crate::refine::split_refine(query, &["endpoint", "set"]);
    let Some(endpoint) = crate::refine::value_of(&refine, "endpoint") else {
        return vec![
            "pending — no endpoint: pass `endpoint=<oai-url> [set=<set>]` (e.g. https://doidb.wdc-terra.org/oaip/oai set=DOIDB.IGETS)"
                .to_string(),
        ];
    };
    let set = crate::refine::value_of(&refine, "set");
    let mut url = format!(
        "{}?verb=ListRecords&metadataPrefix=oai_dc",
        endpoint.trim_end_matches('?')
    );
    if let Some(set) = set {
        url.push_str(&format!("&set={}", crate::net::urlencode(set)));
    }
    match get(&url, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let out = parse_oai(&f.body, max);
            if out.is_empty() {
                vec![format!(
                    "absent — oai carries no record: {}",
                    set.unwrap_or("")
                )]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — oai HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn extract(block: &str, tag: &str) -> Option<String> {
    for variant in [format!("<{}>", tag), format!(":{}>", tag)] {
        if let Some(i) = block.find(&variant) {
            let rest = &block[i + variant.len()..];
            if let Some(close) = rest.find("</") {
                let value = rest[..close].trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

fn parse_oai(body: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    for block in body.split("<record").skip(1) {
        if out.len() >= max {
            break;
        }
        let mut line = String::new();
        if let Some(id) = extract(block, "identifier") {
            line.push_str(&format!("id {}", id));
        }
        if let Some(title) = extract(block, "title") {
            if line.is_empty() {
                line.push_str(&format!("title: {}", title));
            } else {
                line.push_str(&format!("\ttitle: {}", title));
            }
        }
        if !line.is_empty() {
            out.push(line);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_identifier_and_title() {
        let body = r#"<OAI-PMH><ListRecords><record><header><identifier>oai:doidb.wdc-terra.org:6238</identifier></header><metadata><oai_dc:dc><dc:title>Superconducting Gravimeter Data from Sutherland</dc:title></oai_dc:dc></metadata></record></ListRecords></OAI-PMH>"#;
        assert_eq!(
            parse_oai(body, 10),
            vec!["id oai:doidb.wdc-terra.org:6238\ttitle: Superconducting Gravimeter Data from Sutherland".to_string()]
        );
    }

    #[test]
    fn caps_at_max() {
        let one = r#"<record><header><identifier>i</identifier></header></record>"#;
        let body = format!("{}{}", one, one);
        assert_eq!(parse_oai(&body, 1).len(), 1);
    }
}
