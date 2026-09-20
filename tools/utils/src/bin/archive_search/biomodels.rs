use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://www.ebi.ac.uk/ebisearch/ws/rest/biomodels";
const ENTRY: &str = "https://www.ebi.ac.uk/biomodels";

const DEFAULT_FIELDS: &[&str] = &["name"];

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

fn requested_fields(query: &str) -> Vec<String> {
    match parameter(query, "fields") {
        Some(fields) => fields
            .split(',')
            .map(|f| f.trim().to_string())
            .filter(|f| !f.is_empty())
            .collect(),
        None => DEFAULT_FIELDS.iter().map(|f| f.to_string()).collect(),
    }
}

fn requested_max(query: &str, fallback: usize) -> usize {
    match parameter(query, "max").and_then(|v| v.parse::<usize>().ok()) {
        Some(n) if n > 0 => n,
        _ => fallback,
    }
}

fn query_params(query: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut has_text = false;
    for token in query.split_whitespace() {
        let Some((name, value)) = token.split_once('=') else {
            continue;
        };
        if name.is_empty() || value.is_empty() || name == "fields" || name == "max" {
            continue;
        }
        if name == "text" || name == "query" || name == "term" {
            has_text = true;
            out.push(("query".to_string(), value.to_string()));
            continue;
        }
        out.push((name.to_string(), value.to_string()));
    }
    if !has_text {
        let term = query
            .split_whitespace()
            .filter(|token| !token.contains('='))
            .collect::<Vec<_>>()
            .join(" ");
        if !term.is_empty() {
            out.push(("query".to_string(), term));
        }
    }
    out
}

fn biomodels_url(params: &[(String, String)], size: usize) -> String {
    let mut url = format!("{}?format=json&size={}", ENDPOINT, size);
    for (key, value) in params {
        url.push_str(&format!("&{}={}", urlencode(key), urlencode(value)));
    }
    url
}

fn entry_id(entry: &json::Json) -> Option<String> {
    entry
        .get("id")
        .and_then(|x| x.as_scalar_string())
        .filter(|s| !s.is_empty())
}

fn scalar(entry: &json::Json, key: &str) -> Option<String> {
    entry
        .get("fields")
        .and_then(|f| f.get(key))
        .and_then(|v| v.as_arr())
        .and_then(|a| a.first())
        .and_then(|x| x.as_scalar_string())
        .filter(|s| !s.is_empty())
}

pub fn biomodels_lines(query: &str, max: usize) -> Vec<String> {
    let params = query_params(query);
    if params.is_empty() {
        return vec![
            "usage — biomodels needs a query: text=<free text> [fields=<comma-list>] [max=<n>]"
                .to_string(),
        ];
    }
    let fields = requested_fields(query);
    let cap = requested_max(query, max);
    let url = biomodels_url(&params, cap);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match parse_biomodels(&f.body, &fields, cap) {
            None => vec!["pending — the biomodels response carries no JSON".to_string()],
            Some(lines) if lines.is_empty() => {
                vec![format!(
                    "absent — the BioModels register carries no entry: {query}"
                )]
            }
            Some(lines) => lines,
        },
        Some(f) => vec![format!("pending — biomodels HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_biomodels(body: &str, fields: &[String], max: usize) -> Option<Vec<String>> {
    let v = json::parse(body)?;
    let entries = v.get("entries")?.as_arr()?;
    let mut out = Vec::new();
    for entry in entries.iter().take(max) {
        let Some(id) = entry_id(entry) else {
            continue;
        };
        let mut parts: Vec<String> = vec![format!("url {}/{}", ENTRY, id)];
        for field in fields {
            if let Some(value) = scalar(entry, field) {
                parts.push(format!("{field}: {value}"));
            }
        }
        out.push(parts.join("\t"));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_records_and_renders_entry_urls() {
        let body = r#"{"hitCount":2,"entries":[{"id":"BIOMD0000000188","source":"biomodels","fields":{"name":["Proctor2008 - p53/Mdm2 circuit"]}},{"id":"BIOMD0000000189","source":"biomodels","fields":{"name":[""]}}]}"#;
        let fields = vec!["name".to_string()];
        assert_eq!(
            parse_biomodels(body, &fields, 10).unwrap(),
            vec![
                "url https://www.ebi.ac.uk/biomodels/BIOMD0000000188\tname: Proctor2008 - p53/Mdm2 circuit".to_string(),
                "url https://www.ebi.ac.uk/biomodels/BIOMD0000000189".to_string(),
            ]
        );
    }

    #[test]
    fn the_records_are_capped_at_max() {
        let body = r#"{"entries":[{"id":"A"},{"id":"B"},{"id":"C"}]}"#;
        let fields = vec!["name".to_string()];
        let out = parse_biomodels(body, &fields, 2).unwrap();
        assert_eq!(out.len(), 2);
        assert!(out[1].contains("B"));
    }

    #[test]
    fn an_empty_search_carries_nothing() {
        let fields = vec!["name".to_string()];
        assert!(
            parse_biomodels(r#"{"entries":[]}"#, &fields, 10)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn a_body_that_is_not_an_entries_object_is_a_format_gap() {
        let fields = vec!["name".to_string()];
        assert!(parse_biomodels("[]", &fields, 10).is_none());
        assert!(parse_biomodels("{}", &fields, 10).is_none());
        assert!(parse_biomodels("", &fields, 10).is_none());
        assert!(parse_biomodels("<html>", &fields, 10).is_none());
    }

    #[test]
    fn a_record_without_an_id_carries_nothing() {
        let fields = vec!["name".to_string()];
        assert!(
            parse_biomodels(r#"{"entries":[{"fields":{"name":["x"]}}]}"#, &fields, 10)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn reads_key_value_params_and_free_text() {
        assert_eq!(
            parameter("text=p53 fields=name", "text").as_deref(),
            Some("p53")
        );
        assert_eq!(
            query_params("text=p53 max=2"),
            vec![("query".to_string(), "p53".to_string())]
        );
        assert_eq!(
            query_params("p53"),
            vec![("query".to_string(), "p53".to_string())]
        );
    }

    #[test]
    fn the_default_field_set_names_the_model() {
        assert_eq!(requested_fields("text=p53"), vec!["name".to_string()]);
    }

    #[test]
    fn an_explicit_field_list_replaces_the_default() {
        assert_eq!(
            requested_fields("text=p53 fields=name,description"),
            vec!["name".to_string(), "description".to_string()]
        );
    }

    #[test]
    fn a_query_without_a_term_is_usage_not_network() {
        let out = biomodels_lines("fields=name", 10);
        assert_eq!(out.len(), 1);
        assert!(out[0].starts_with("usage — biomodels needs a query"));
    }
}
