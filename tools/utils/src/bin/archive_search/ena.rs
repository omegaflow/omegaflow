use crate::json;
use crate::net::{get, urlencode};

const SEARCH: &str = "https://www.ebi.ac.uk/ena/portal/api/search";
const COUNT: &str = "https://www.ebi.ac.uk/ena/portal/api/count";

fn parameter(query: &str, key: &str) -> Option<String> {
    query.split_whitespace().find_map(|token| {
        let (name, value) = token.split_once('=')?;
        if name == key && !value.is_empty() {
            Some(value.to_string())
        } else {
            None
        }
    })
}

fn term(query: &str) -> String {
    if let Some(explicit) = parameter(query, "query").or_else(|| parameter(query, "term")) {
        return explicit;
    }
    query
        .split_whitespace()
        .filter(|token| {
            !token.starts_with("result=")
                && !token.starts_with("fields=")
                && !token.starts_with("query=")
                && !token.starts_with("term=")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn ena_count(result: &str, term: &str) -> Option<String> {
    let mut url = format!("{}?result={}", COUNT, urlencode(result));
    if !term.is_empty() {
        url.push_str(&format!("&query={}", urlencode(term)));
    }
    let f = get(&url, &[], "40")?;
    if f.status != Some(200) {
        return None;
    }
    let count = f.body.trim();
    if count.is_empty() || !count.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(count.to_string())
}

fn ena_search_url(result: &str, term: &str, fields: &str, max: usize) -> String {
    let mut url = format!(
        "{}?result={}&fields={}&format=json&limit={}",
        SEARCH,
        urlencode(result),
        urlencode(fields),
        max
    );
    if !term.is_empty() {
        url.push_str(&format!("&query={}", urlencode(term)));
    }
    url
}

fn accession_field(fields: &[String]) -> Option<&str> {
    fields
        .iter()
        .find(|f| f.as_str() == "accession" || f.ends_with("_accession"))
        .map(String::as_str)
}

fn scalar(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_scalar_string())
        .filter(|s| !s.is_empty())
}

pub fn ena_lines(query: &str, max: usize) -> Vec<String> {
    let Some(result) = parameter(query, "result") else {
        return vec![
            "usage — ena needs result=<type>: result=read_run|study|sample|analysis|assembly <query>"
                .to_string(),
        ];
    };
    let Some(fields) = parameter(query, "fields") else {
        return vec![
            "usage — ena needs fields=<comma-list>: fields=run_accession,country".to_string(),
        ];
    };
    let field_list: Vec<String> = fields
        .split(',')
        .map(|f| f.trim().to_string())
        .filter(|f| !f.is_empty())
        .collect();
    if field_list.is_empty() {
        return vec![
            "usage — ena needs fields=<comma-list>: fields=run_accession,country".to_string(),
        ];
    }
    let term = term(query);
    let mut out = Vec::new();
    if let Some(count) = ena_count(&result, &term) {
        out.push(format!("count {count}\tresult: {result}"));
    }
    let url = ena_search_url(&result, &term, &fields, max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let mut lines = parse_ena(&f.body, &field_list, max);
            if lines.is_empty() {
                if out.is_empty() {
                    out.push(format!("absent — ena carries no entry: {query}"));
                }
            } else {
                out.append(&mut lines);
            }
            out
        }
        Some(f) => vec![format!("pending — ena HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_ena(body: &str, fields: &[String], max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(items) = v.as_arr() else {
        return out;
    };
    for item in items.iter().take(max) {
        let mut parts: Vec<String> = Vec::new();
        if let Some(accession) = accession_field(fields).and_then(|f| scalar(item, f)) {
            parts.push(format!(
                "url https://www.ebi.ac.uk/ena/browser/view/{accession}"
            ));
        }
        for field in fields {
            if let Some(value) = scalar(item, field) {
                parts.push(format!("{field}: {value}"));
            }
        }
        if !parts.is_empty() {
            out.push(parts.join("\t"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_search_records_and_renders_urls() {
        let body = r#"[{"run_accession":"ERR10006159","country":"Sweden"},{"run_accession":"ERR10006164","country":""}]"#;
        let fields = vec!["run_accession".to_string(), "country".to_string()];
        assert_eq!(
            parse_ena(body, &fields, 10),
            vec![
                "url https://www.ebi.ac.uk/ena/browser/view/ERR10006159\trun_accession: ERR10006159\tcountry: Sweden".to_string(),
                "url https://www.ebi.ac.uk/ena/browser/view/ERR10006164\trun_accession: ERR10006164".to_string(),
            ]
        );
    }

    #[test]
    fn the_records_are_capped_at_max() {
        let body = r#"[{"run_accession":"A"},{"run_accession":"B"},{"run_accession":"C"}]"#;
        let fields = vec!["run_accession".to_string()];
        let out = parse_ena(body, &fields, 2);
        assert_eq!(out.len(), 2);
        assert!(out[1].contains("B"));
    }

    #[test]
    fn an_empty_search_carries_nothing() {
        let fields = vec!["run_accession".to_string()];
        assert!(parse_ena("[]", &fields, 10).is_empty());
    }

    #[test]
    fn a_body_that_is_not_an_array_carries_nothing() {
        let fields = vec!["run_accession".to_string()];
        assert!(parse_ena("{}", &fields, 10).is_empty());
        assert!(parse_ena("", &fields, 10).is_empty());
    }

    #[test]
    fn a_record_without_the_requested_fields_carries_nothing() {
        let fields = vec!["run_accession".to_string()];
        assert!(parse_ena(r#"[{"other":"x"}]"#, &fields, 10).is_empty());
    }

    #[test]
    fn reads_result_fields_and_query_tokens() {
        assert_eq!(
            parameter("result=read_run fields=run_accession", "result").as_deref(),
            Some("read_run")
        );
        assert_eq!(
            term("result=read_run fields=run_accession tax_tree(9606)"),
            "tax_tree(9606)"
        );
        assert_eq!(
            term("result=read_run query=tax_tree(9606)"),
            "tax_tree(9606)"
        );
        assert_eq!(
            term("result=read_run term=country=Germany"),
            "country=Germany"
        );
    }

    #[test]
    fn a_query_without_result_is_usage_not_network() {
        let out = ena_lines("read_run fields=run_accession", 10);
        assert_eq!(out.len(), 1);
        assert!(out[0].starts_with("usage — ena needs result="));
    }

    #[test]
    fn a_query_without_fields_is_usage_not_network() {
        let out = ena_lines("result=read_run", 10);
        assert_eq!(out.len(), 1);
        assert!(out[0].starts_with("usage — ena needs fields="));
    }
}
