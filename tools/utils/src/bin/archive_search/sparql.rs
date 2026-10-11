use crate::json;
use crate::net::{get, urlencode};

pub fn sparql_lines(query: &str, max: usize) -> Vec<String> {
    let (q, refine) = crate::refine::split_refine(query, &["endpoint"]);
    let Some(endpoint) = crate::refine::value_of(&refine, "endpoint") else {
        return vec![
            "pending — no endpoint: pass `endpoint=<sparql-url> <SELECT ...>`".to_string(),
        ];
    };
    let q = q.trim();
    if q.is_empty() {
        return vec![format!("pending — no query for endpoint {}", endpoint)];
    }
    let url = format!(
        "{}?query={}&format=json",
        endpoint.trim_end_matches('?'),
        urlencode(q)
    );
    match get(&url, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the sparql response carries no JSON".to_string()];
            };
            let out = parse_sparql(&v, max);
            if out.is_empty() {
                vec![format!("absent — sparql carries no row: {}", q)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — sparql HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_sparql(v: &json::Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(bindings) = v
        .get("results")
        .and_then(|r| r.get("bindings"))
        .and_then(|b| b.as_arr())
    else {
        return out;
    };
    for row in bindings.iter().take(max) {
        let json::Json::Obj(obj) = row else {
            continue;
        };
        let mut keys: Vec<&String> = obj.keys().collect();
        keys.sort();
        let mut parts: Vec<String> = Vec::new();
        for k in keys {
            if let Some(value) = obj
                .get(k)
                .and_then(|cell| cell.get("value"))
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
            {
                parts.push(format!("{}={}", k, value));
            }
        }
        if !parts.is_empty() {
            out.push(format!("row {}", parts.join(" ")));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_bindings() {
        let body = r#"{"head":{"vars":["s","t"]},"results":{"bindings":[{"s":{"type":"uri","value":"https://x"},"t":{"type":"literal","value":"A"}}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_sparql(&v, 10),
            vec!["row s=https://x t=A".to_string()]
        );
    }

    #[test]
    fn empty_bindings_carry_nothing() {
        let v = json::parse(r#"{"results":{"bindings":[]}}"#).unwrap();
        assert!(parse_sparql(&v, 10).is_empty());
    }
}
