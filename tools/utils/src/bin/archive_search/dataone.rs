use crate::json;
use crate::net::{get, urlencode};

pub fn dataone_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://cn.dataone.org/cn/v2/query/solr/?q={}&rows={}&wt=json",
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the dataone response carries no JSON".to_string()];
            };
            let out = parse_dataone(&v);
            if out.is_empty() {
                vec![format!("absent — dataone carries no dataset: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — dataone HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_dataone(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(docs) = v
        .get("response")
        .and_then(|r| r.get("docs"))
        .and_then(|d| d.as_arr())
    else {
        return out;
    };
    for doc in docs {
        let id = match doc
            .get("id")
            .and_then(|i| i.as_str())
            .filter(|s| !s.is_empty())
        {
            Some(i) => i,
            None => continue,
        };
        let mut line = format!("url https://search.dataone.org/view/{}", id);
        if let Some(title) = doc
            .get("title")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", collapse(title)));
        }
        if let Some(shape) = doc
            .get("formatId")
            .and_then(|f| f.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tformat: {}", shape));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_docs() {
        let body = r#"{"response":{"numFound":2,"docs":[{"id":"sha256:abc","title":"Abiotic  stress maps","formatId":"science-on-schema.org/Dataset;ld+json","dateUploaded":"2018-03-30T00:00:00Z"},{"id":"sha256:def","title":"Chronic BMY7378","formatId":"text/csv"}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_dataone(&v),
            vec![
                "url https://search.dataone.org/view/sha256:abc\ttitle: Abiotic stress maps\tformat: science-on-schema.org/Dataset;ld+json".to_string(),
                "url https://search.dataone.org/view/sha256:def\ttitle: Chronic BMY7378\tformat: text/csv".to_string(),
            ]
        );
    }

    #[test]
    fn skips_docs_without_id() {
        let body = r#"{"response":{"docs":[{"title":"no id here","formatId":"text/csv"},{"id":"sha256:keep","title":"kept"}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_dataone(&v),
            vec!["url https://search.dataone.org/view/sha256:keep\ttitle: kept".to_string()]
        );
    }

    #[test]
    fn empty_docs_carry_nothing() {
        let v = json::parse(r#"{"response":{"numFound":0,"docs":[]}}"#).unwrap();
        assert!(parse_dataone(&v).is_empty());
    }
}
