use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.semanticscholar.org/graph/v1/paper/search";

pub fn semanticscholar_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?query={}&limit={}&fields=title,year,externalIds,url,citationCount",
        ENDPOINT,
        urlencode(query),
        max
    );
    let header = crate::token::secret("S2_API_KEY").map(|k| format!("x-api-key: {}", k));
    let extra: Vec<&str> = match &header {
        Some(h) => vec!["-H", h.as_str()],
        None => Vec::new(),
    };
    match get(&url, &extra, "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_semanticscholar(&f.body);
            if out.is_empty() {
                vec![format!("absent — semanticscholar carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) if f.status == Some(429) => vec![
            "pending — semanticscholar rate limit (keyless shared pool); S2_API_KEY in .secrets.local lifts it"
                .to_string(),
        ],
        Some(f) if f.status == Some(403) => vec![
            "pending — semanticscholar refuses the key (HTTP 403); check S2_API_KEY".to_string(),
        ],
        Some(f) => vec![format!("pending — semanticscholar HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_semanticscholar(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(data) = v.get("data").and_then(|d| d.as_arr()) else {
        return out;
    };
    for paper in data {
        let Some(id) = field(paper, "paperId") else {
            continue;
        };
        let url = match field(paper, "url") {
            Some(u) => u,
            None => format!("https://www.semanticscholar.org/paper/{}", id),
        };
        let mut line = format!("url {}", url);
        if let Some(title) = field(paper, "title") {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(ids) = paper.get("externalIds") {
            if let Some(doi) = field(ids, "DOI") {
                line.push_str(&format!("\tdoi: {}", doi));
            }
            if let Some(arxiv) = field(ids, "ArXiv") {
                line.push_str(&format!("\tarxiv: {}", arxiv));
            }
        }
        if let Some(year) = paper.get("year").and_then(|y| y.as_scalar_string()) {
            line.push_str(&format!("\tyear: {}", year));
        }
        if let Some(cites) = paper.get("citationCount").and_then(|c| c.as_scalar_string()) {
            line.push_str(&format!("\tcites: {}", cites));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_paper_fields() {
        let body = r#"{"total":1,"data":[{"paperId":"abc","title":"Asthma","year":2019,"externalIds":{"DOI":"10.1/x","ArXiv":"1901.00001"},"url":"https://www.semanticscholar.org/paper/abc","citationCount":5}]}"#;
        assert_eq!(
            parse_semanticscholar(body),
            vec!["url https://www.semanticscholar.org/paper/abc\ttitle: Asthma\tdoi: 10.1/x\tarxiv: 1901.00001\tyear: 2019\tcites: 5".to_string()]
        );
    }

    #[test]
    fn a_paper_without_an_id_carries_nothing() {
        assert!(parse_semanticscholar(r#"{"data":[{"title":"x"}]}"#).is_empty());
    }
}
