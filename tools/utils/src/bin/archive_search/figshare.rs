use crate::json;
use crate::net::{json_escape, post};

const ENDPOINT: &str = "https://api.figshare.com/v2/articles/search";
const HTML_BASE: &str = "https://figshare.com/articles/";

pub fn figshare_lines(query: &str, max: usize) -> Vec<String> {
    let body = figshare_body(query, max);
    match post(ENDPOINT, &body, &[], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let out = parse_figshare(&v, max);
                if out.is_empty() {
                    vec![format!("absent — figshare carries no result: {}", query)]
                } else {
                    out
                }
            }
            None => vec!["pending — the figshare response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — figshare HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn figshare_body(query: &str, max: usize) -> String {
    let page_size = max.clamp(1, 100);
    format!(
        "{{\"search_for\":\"{}\",\"page_size\":{}}}",
        json_escape(query),
        page_size
    )
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_figshare(v: &json::Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.as_arr() else {
        return out;
    };
    for item in items {
        let public_html = item
            .get("url_public_html")
            .and_then(|u| u.as_str())
            .filter(|s| !s.is_empty());
        let url = match public_html {
            Some(u) => u.to_string(),
            None => match item.get("id").and_then(|i| i.as_scalar_string()) {
                Some(id) => format!("{}{}", HTML_BASE, id),
                None => continue,
            },
        };
        let mut line = format!("url {}", url);
        if let Some(title) = item
            .get("title")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", collapse(title)));
        }
        if let Some(doi) = item
            .get("doi")
            .and_then(|d| d.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tdoi: {}", doi));
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
    fn carries_url_title_and_doi() {
        let body = r#"[{"id":33065816,"title":"Constraining neutron-star properties with ensembles of thermonuclear bursts","doi":"10.26180/33065816.v2","url_public_html":"https://bridges.monash.edu/articles/dataset/x/33065816"}]"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_figshare(&v, 10),
            vec![
                "url https://bridges.monash.edu/articles/dataset/x/33065816\ttitle: Constraining neutron-star properties with ensembles of thermonuclear bursts\tdoi: 10.26180/33065816.v2"
                    .to_string()
            ]
        );
    }

    #[test]
    fn falls_back_to_figshare_article_url() {
        let body = r#"[{"id":42,"title":"No html url"}]"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_figshare(&v, 10),
            vec!["url https://figshare.com/articles/42\ttitle: No html url".to_string()]
        );
    }

    #[test]
    fn caps_at_max() {
        let body = r#"[{"id":1,"title":"A"},{"id":2,"title":"B"}]"#;
        let v = json::parse(body).unwrap();
        assert_eq!(parse_figshare(&v, 1).len(), 1);
    }
}
