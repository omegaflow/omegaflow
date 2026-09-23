use crate::json;
use crate::net::{get, urlencode};

const RESULT: &str = "https://www.crystallography.net/cod/result";
const ENTRY: &str = "https://www.crystallography.net/cod";

const DEFAULT_FIELDS: &[&str] = &[
    "file", "chemname", "formula", "sg", "a", "b", "c", "alpha", "beta", "gamma", "vol", "year",
    "doi",
];

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
        if name == "text" {
            has_text = true;
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
            out.push(("text".to_string(), term));
        }
    }
    out
}

fn cod_url(params: &[(String, String)]) -> String {
    let mut url = format!("{}?format=json", RESULT);
    for (key, value) in params {
        url.push_str(&format!("&{}={}", urlencode(key), urlencode(value)));
    }
    url
}

fn entry_field(fields: &[String]) -> Option<&str> {
    fields
        .iter()
        .find(|f| f.as_str() == "file")
        .map(String::as_str)
}

fn scalar(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_scalar_string())
        .filter(|s| !s.is_empty())
}

pub fn cod_lines(query: &str, max: usize) -> Vec<String> {
    let params = query_params(query);
    if params.is_empty() {
        return vec![
            "usage — cod needs a query: text=<free text> [el1=.. el2=.. nel=..] [fields=<comma-list>] [max=<n>]"
                .to_string(),
        ];
    }
    let fields = requested_fields(query);
    let cap = requested_max(query, max);
    let url = cod_url(&params);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match parse_cod(&f.body, &fields, cap) {
            None => vec!["pending — the cod response carries no JSON".to_string()],
            Some(lines) if lines.is_empty() => {
                vec![format!(
                    "absent — the COD register carries no entry: {query}"
                )]
            }
            Some(lines) => lines,
        },
        Some(f) => vec![format!("pending — cod HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_cod(body: &str, fields: &[String], max: usize) -> Option<Vec<String>> {
    let v = json::parse(body)?;
    let items = v.as_arr()?;
    let mut out = Vec::new();
    for item in items.iter().take(max) {
        let Some(file) = entry_field(fields).and_then(|f| scalar(item, f)) else {
            continue;
        };
        let mut parts: Vec<String> = vec![format!("url {}/{}.html", ENTRY, file)];
        for field in fields {
            if let Some(value) = scalar(item, field) {
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
        let body = r#"[{"file":"1009000","chemname":"Gallium arsenate(V)","a":"4.994","year":"1999"},{"file":"1009001","chemname":"","a":"4.9942"}]"#;
        let fields = vec!["file".to_string(), "chemname".to_string(), "a".to_string()];
        assert_eq!(
            parse_cod(body, &fields, 10).unwrap(),
            vec![
                "url https://www.crystallography.net/cod/1009000.html\tfile: 1009000\tchemname: Gallium arsenate(V)\ta: 4.994".to_string(),
                "url https://www.crystallography.net/cod/1009001.html\tfile: 1009001\ta: 4.9942".to_string(),
            ]
        );
    }

    #[test]
    fn the_records_are_capped_at_max() {
        let body = r#"[{"file":"1"},{"file":"2"},{"file":"3"}]"#;
        let fields = vec!["file".to_string()];
        let out = parse_cod(body, &fields, 2).unwrap();
        assert_eq!(out.len(), 2);
        assert!(out[1].contains("2"));
    }

    #[test]
    fn an_empty_search_carries_nothing() {
        let fields = vec!["file".to_string()];
        assert!(parse_cod("[]", &fields, 10).unwrap().is_empty());
    }

    #[test]
    fn a_body_that_is_not_an_array_is_a_format_gap() {
        let fields = vec!["file".to_string()];
        assert!(parse_cod("{}", &fields, 10).is_none());
        assert!(parse_cod("", &fields, 10).is_none());
        assert!(parse_cod("<html>", &fields, 10).is_none());
    }

    #[test]
    fn a_record_without_the_entry_field_carries_nothing() {
        let fields = vec!["a".to_string()];
        assert!(
            parse_cod(r#"[{"a":"4.994"}]"#, &fields, 10)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn reads_key_value_params_and_free_text() {
        assert_eq!(
            parameter("text=quartz el1=Ga", "el1").as_deref(),
            Some("Ga")
        );
        assert_eq!(
            query_params("text=quartz el1=Ga max=2"),
            vec![
                ("text".to_string(), "quartz".to_string()),
                ("el1".to_string(), "Ga".to_string()),
            ]
        );
        assert_eq!(
            query_params("quartz"),
            vec![("text".to_string(), "quartz".to_string())]
        );
    }

    #[test]
    fn the_default_field_set_names_the_cell() {
        let fields = requested_fields("text=quartz");
        assert!(fields.contains(&"file".to_string()));
        assert!(fields.contains(&"vol".to_string()));
        assert!(fields.contains(&"sg".to_string()));
    }

    #[test]
    fn an_explicit_field_list_replaces_the_default() {
        assert_eq!(
            requested_fields("text=quartz fields=file,doi"),
            vec!["file".to_string(), "doi".to_string()]
        );
    }

    #[test]
    fn a_query_without_a_term_is_usage_not_network() {
        let out = cod_lines("fields=file", 10);
        assert_eq!(out.len(), 1);
        assert!(out[0].starts_with("usage — cod needs a query"));
    }
}
