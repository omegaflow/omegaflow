use crate::json;
use crate::net::{get, urlencode};

pub fn search_url(query: &str) -> String {
    format!(
        "https://api.consensus.app/v1/search?query={}",
        urlencode(query)
    )
}

pub fn consensus_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — CONSENSUS_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let url = search_url(query);
    let auth = format!("x-api-key: {}", token);
    let headers = ["-H", auth.as_str(), "-H", "Accept: application/json"];
    match get(&url, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = consensus_results(&v, max);
                if out.is_empty() {
                    out.push(format!("absent — Consensus carries no entry: {}", query));
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
        if parts.is_empty() {
            continue;
        }
        out.push(parts.join(" · "));
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
            search_url("transfer entropy"),
            "https://api.consensus.app/v1/search?query=transfer%20entropy"
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
