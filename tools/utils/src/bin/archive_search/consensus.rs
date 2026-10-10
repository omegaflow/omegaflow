use crate::json;
use crate::net::{get, urlencode};

pub fn search_url(query: &str, fulltext: bool) -> String {
    let mut url = format!(
        "https://api.consensus.app/v1/search?query={}",
        urlencode(query)
    );
    if fulltext {
        url.push_str("&include_full_text_chunks=true");
    }
    url
}

pub fn consensus_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — CONSENSUS_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let (text, refine) = crate::refine::split_refine(query, &["fulltext"]);
    let fulltext = matches!(
        crate::refine::value_of(&refine, "fulltext"),
        Some("1") | Some("true")
    );
    let url = search_url(&text, fulltext);
    let auth = format!("x-api-key: {}", token);
    let headers = ["-H", auth.as_str(), "-H", "Accept: application/json"];
    match get(&url, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = consensus_results(&v, max);
                if out.is_empty() {
                    out.push(format!("absent — Consensus carries no entry: {}", text));
                }
                out
            }
            None => vec!["pending — the Consensus response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — Consensus HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field_str(row: &json::Json, key: &str) -> Option<String> {
    row.get(key)
        .and_then(|v| v.as_scalar_string())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn cap(s: &str) -> String {
    let flat = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= 2000 {
        return flat;
    }
    let mut out: String = flat.chars().take(2000).collect();
    out.push_str(" …");
    out
}

fn consensus_results(v: &json::Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for row in results {
        let parts: Vec<String> = [
            "title",
            "doi",
            "publish_year",
            "citation_count",
            "study_type",
            "takeaway",
        ]
        .iter()
        .filter_map(|key| field_str(row, key))
        .collect();
        let mut line = parts.join(" · ");
        let mut carried = !parts.is_empty();
        if let Some(abstract_text) = field_str(row, "abstract") {
            line.push_str(&format!("\tabstract: {}", cap(&abstract_text)));
            carried = true;
        }
        if let Some(chunks) = row.get("full_text_chunks").and_then(|c| c.as_arr()) {
            for chunk in chunks {
                if let Some(chunk) = chunk.as_str() {
                    let chunk = cap(chunk);
                    if !chunk.is_empty() {
                        line.push_str(&format!("\tchunk: {}", chunk));
                        carried = true;
                    }
                }
            }
        }
        if !carried {
            continue;
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_url_encodes_the_query() {
        assert_eq!(
            search_url("transfer entropy", false),
            "https://api.consensus.app/v1/search?query=transfer%20entropy"
        );
    }

    #[test]
    fn a_fulltext_request_names_the_flag() {
        assert_eq!(
            search_url("transfer entropy", true),
            "https://api.consensus.app/v1/search?query=transfer%20entropy&include_full_text_chunks=true"
        );
    }

    #[test]
    fn reads_the_paper_fields_and_omits_absent() {
        let body = r#"{"results":[{"title":"A measured field","doi":"10.1/x","publish_year":2021,"citation_count":7,"study_type":"rct","takeaway":"It holds."},{"title":"Only a title"}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            consensus_results(&v, 10),
            vec![
                "A measured field · 10.1/x · 2021 · 7 · rct · It holds.".to_string(),
                "Only a title".to_string(),
            ]
        );
    }
}
