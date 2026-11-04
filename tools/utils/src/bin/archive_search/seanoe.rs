use crate::net::{get, urlencode};
use std::collections::HashSet;

const ENDPOINT: &str = "https://www.seanoe.org/";
const DATA: &str = "https://www.seanoe.org/data/";

pub fn seanoe_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?q={}", ENDPOINT, urlencode(query));
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_seanoe(&f.body, max);
            if out.is_empty() {
                vec![format!("absent — seanoe carries no dataset: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — seanoe HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_seanoe(html: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut rest = html;
    while let Some(i) = rest.find(DATA) {
        let after = &rest[i..];
        let end = after
            .find(|c| c == '"' || c == '\'' || c == ' ' || c == '<')
            .unwrap_or(after.len());
        let url = &after[..end];
        if url.ends_with('/') && seen.insert(url.to_string()) {
            out.push(format!("url {}", url));
            if out.len() >= max {
                break;
            }
        }
        rest = &after[end..];
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_data_links_and_dedupes() {
        let html = r#"<a href="https://www.seanoe.org/data/01091/120228/">A</a><a href="https://www.seanoe.org/data/01089/120058/">B</a><a href="https://www.seanoe.org/data/01091/120228/">A again</a>"#;
        assert_eq!(
            parse_seanoe(html, 10),
            vec![
                "url https://www.seanoe.org/data/01091/120228/".to_string(),
                "url https://www.seanoe.org/data/01089/120058/".to_string()
            ]
        );
    }

    #[test]
    fn caps_at_max() {
        let html = r#"href="https://www.seanoe.org/data/1/1/" href="https://www.seanoe.org/data/2/2/" href="https://www.seanoe.org/data/3/3/""#;
        assert_eq!(parse_seanoe(html, 2).len(), 2);
    }
}
