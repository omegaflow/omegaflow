use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.core.ac.uk/v3/search/works/";

pub fn core_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?q={}&limit={}", ENDPOINT, urlencode(query), max);
    let header = crate::token::secret("CORE_API_KEY").map(|k| format!("Authorization: Bearer {}", k));
    let extra: Vec<&str> = match &header {
        Some(h) => vec!["-H", h.as_str()],
        None => Vec::new(),
    };
    match get(&url, &extra, "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_core(&f.body);
            if out.is_empty() {
                vec![format!("absent — core carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) if f.status == Some(401) || f.status == Some(403) => vec![
            "pending — core refuses the request; the keyless search runs at 1 batch / 5 requests per 10 s, CORE_API_KEY raises it"
                .to_string(),
        ],
        Some(f) => vec![format!("pending — core HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_core(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for record in results {
        let Some(id) = record.get("id").and_then(|i| i.as_scalar_string()) else {
            continue;
        };
        let mut line = format!("url https://core.ac.uk/works/{}", id);
        if let Some(title) = field(record, "title") {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(doi) = field(record, "doi") {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        if let Some(year) = record.get("yearPublished").and_then(|y| y.as_scalar_string()) {
            line.push_str(&format!("\tyear: {}", year));
        }
        if let Some(download) = field(record, "downloadUrl") {
            line.push_str(&format!("\tfulltext: {}", download));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_work_fields() {
        let body = r#"{"totalHits":209053,"results":[{"id":19213398,"title":"The global Asthma report 2014","doi":null,"yearPublished":2014,"downloadUrl":"https://core.ac.uk/download/30673284.pdf"}]}"#;
        assert_eq!(
            parse_core(body),
            vec!["url https://core.ac.uk/works/19213398\ttitle: The global Asthma report 2014\tyear: 2014\tfulltext: https://core.ac.uk/download/30673284.pdf".to_string()]
        );
    }

    #[test]
    fn a_work_without_an_id_carries_nothing() {
        assert!(parse_core(r#"{"results":[{"title":"x"}]}"#).is_empty());
    }
}
