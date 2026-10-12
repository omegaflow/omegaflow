use crate::json;
use crate::net::{get, urlencode};

pub fn grepapp_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("https://grep.app/api/search?q={}", urlencode(query));
    match get(&url, &[], "30") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(_) => {
                let lines = parse_grepapp(&f.body, query, max);
                if lines.is_empty() {
                    vec![format!("absent — grepapp carries no hit: {}", query)]
                } else {
                    lines
                }
            }
            None => vec!["pending — the grepapp response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — grepapp HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn text(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_grepapp(body: &str, query: &str, max: usize) -> Vec<String> {
    let Some(v) = json::parse(body) else {
        return Vec::new();
    };
    let Some(entries) = v
        .get("hits")
        .and_then(|h| h.get("hits"))
        .and_then(|h| h.as_arr())
    else {
        return Vec::new();
    };
    let url = format!("https://grep.app/search?q={}", urlencode(query));
    let mut out = Vec::new();
    for entry in entries {
        if out.len() >= max {
            break;
        }
        let (Some(repo), Some(path)) = (text(entry, "repo"), text(entry, "path")) else {
            continue;
        };
        out.push(format!("url {}\trepo: {}\tpath: {}", url, repo, path));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_captured_hits() {
        let body = r#"{"time":130,"facets":{},"hits":{"total":108,"hits":[{"owner_id":"79945230","repo":"Nixtla/nixtla","branch":"main","path":"timegpt-docs/scripts/generate_simulate_explain_assets.py","content":{"snippet":"<table class=\"highlight-table\"><tr data-line=\"360\"><td><div class=\"lineno\">360</div></td><td><div class=\"highlight\"><pre><span class=\"s2\">&quot;<mark>transfer_entropy</mark>&quot;</span></pre></div></td></tr></table>"}},{"owner_id":"49326130","repo":"cbueth/infomeasure","branch":"main","path":"infomeasure/estimators/transfer_entropy.py","content":{"snippet":"x"}}]}}"#;
        assert_eq!(
            parse_grepapp(body, "transfer_entropy", 10),
            vec![
                "url https://grep.app/search?q=transfer_entropy\trepo: Nixtla/nixtla\tpath: timegpt-docs/scripts/generate_simulate_explain_assets.py".to_string(),
                "url https://grep.app/search?q=transfer_entropy\trepo: cbueth/infomeasure\tpath: infomeasure/estimators/transfer_entropy.py".to_string(),
            ]
        );
    }

    #[test]
    fn empty_and_hitless_carry_nothing() {
        assert!(parse_grepapp(r#"{"hits":{"total":0,"hits":[]}}"#, "x", 10).is_empty());
        assert!(
            parse_grepapp(r#"{"hits":{"total":1,"hits":[{"repo":"a/b"}]}}"#, "x", 10).is_empty()
        );
    }

    #[test]
    fn caps_at_max() {
        let body = r#"{"hits":{"hits":[{"repo":"a/1","path":"a"},{"repo":"a/2","path":"b"},{"repo":"a/3","path":"c"}]}}"#;
        assert_eq!(parse_grepapp(body, "x", 2).len(), 2);
    }
}
