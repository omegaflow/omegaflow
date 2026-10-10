use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://www.pangaea.de/advanced/search.php";

pub fn pangaea_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?q={}&count={}&format=json",
        ENDPOINT,
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the pangaea response carries no JSON".to_string()];
            };
            let out = parse_pangaea(&v, max);
            if out.is_empty() {
                vec![format!("absent — pangaea carries no dataset: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — pangaea HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_pangaea(v: &json::Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for item in items.iter().take(max) {
        let Some(uri) = item
            .get("URI")
            .and_then(|u| u.as_str())
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let doi = uri.strip_prefix("doi:").unwrap_or(uri);
        let mut line = format!("url https://doi.org/{}", doi);
        if let Some(html) = item.get("html").and_then(|h| h.as_str()) {
            let text = collapse(&strip_tags(html)).replace(" […]", "");
            let text = text.trim();
            if !text.is_empty() {
                let capped: String = text.chars().take(300).collect();
                line.push_str(&format!("\t{}", capped));
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
    fn reads_uri_and_html() {
        let body = r#"{"results":[{"URI":"doi:10.1594/PANGAEA.940004","html":"<strong>Chen (2022):</strong> A soil moisture dataset"}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_pangaea(&v, 10),
            vec![
                "url https://doi.org/10.1594/PANGAEA.940004\tChen (2022): A soil moisture dataset"
                    .to_string()
            ]
        );
    }

    #[test]
    fn empty_results_carry_nothing() {
        assert!(parse_pangaea(&json::parse(r#"{"results":[]}"#).unwrap(), 10).is_empty());
    }
}
