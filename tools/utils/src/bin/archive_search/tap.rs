use crate::json;
use crate::net::{get, urlencode};

pub fn tap_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(query, &["endpoint"]);
    let adql = text.trim();
    let Some(endpoint) = crate::refine::value_of(&refine, "endpoint") else {
        return vec![
            "pending — no endpoint: pass `endpoint=<tap-sync-url> <adql>` (find a service with `archive_search --regtap`)"
                .to_string(),
        ];
    };
    if adql.is_empty() {
        return vec![format!("pending — no adql query for endpoint {}", endpoint)];
    }
    let url = format!(
        "{}?REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY={}",
        endpoint.trim_end_matches('?'),
        urlencode(adql)
    );
    match get(&url, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the tap response carries no JSON".to_string()];
            };
            let out = parse_tap(&v, max);
            if out.is_empty() {
                vec![format!("absent — tap carries no row: {}", adql)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — tap HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn cell_str(cell: &json::Json) -> Option<String> {
    match cell {
        json::Json::Null => Some("null".to_string()),
        json::Json::Str(s) => Some(s.clone()),
        json::Json::Num(n) if n.is_finite() => Some(n.to_string()),
        json::Json::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn row_line(row: &json::Json, columns: &[&str]) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    match row {
        json::Json::Obj(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for k in keys {
                if let Some(v) = map.get(k).and_then(cell_str) {
                    if !v.is_empty() {
                        parts.push(format!("{}={}", k, v));
                    }
                }
            }
        }
        json::Json::Arr(cells) => {
            for (i, cell) in cells.iter().enumerate() {
                let name = match columns.get(i) {
                    Some(n) => *n,
                    None => "?",
                };
                if let Some(v) = cell_str(cell) {
                    if !v.is_empty() {
                        parts.push(format!("{}={}", name, v));
                    }
                }
            }
        }
        _ => return None,
    }
    if parts.is_empty() {
        None
    } else {
        Some(format!("row {}", parts.join(" ")))
    }
}

fn parse_tap(v: &json::Json, max: usize) -> Vec<String> {
    if let Some(rows) = v.as_arr() {
        return rows
            .iter()
            .take(max)
            .filter_map(|r| row_line(r, &[]))
            .collect();
    }
    let columns: Vec<&str> = match v.get("metadata").and_then(|m| m.as_arr()) {
        Some(m) => m
            .iter()
            .filter_map(|c| c.get("name").and_then(|n| n.as_str()))
            .collect(),
        None => Vec::new(),
    };
    let Some(rows) = v.get("data").and_then(|d| d.as_arr()) else {
        return Vec::new();
    };
    rows.iter()
        .take(max)
        .filter_map(|r| row_line(r, &columns))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_metadata_and_rows() {
        let body = r#"{"metadata":[{"name":"ra"},{"name":"dec"},{"name":"name"}],"data":[[12.5,-3.2,"X"],[1.0,2.0,"Y"]]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_tap(&v, 10),
            vec![
                "row ra=12.5 dec=-3.2 name=X".to_string(),
                "row ra=1 dec=2 name=Y".to_string()
            ]
        );
    }

    #[test]
    fn a_bare_row_object_array_is_read() {
        let body = r#"[{"pl_name":"HD 2039 b"},{"pl_name":"HAT-P-8 b"}]"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_tap(&v, 10),
            vec![
                "row pl_name=HD 2039 b".to_string(),
                "row pl_name=HAT-P-8 b".to_string()
            ]
        );
    }

    #[test]
    fn caps_at_max() {
        let body = r#"{"metadata":[{"name":"a"}],"data":[[1],[2],[3]]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(parse_tap(&v, 2).len(), 2);
    }

    #[test]
    fn a_null_cell_is_named_null_never_empty() {
        let body = r#"{"metadata":[{"name":"a"},{"name":"b"}],"data":[[1,null]]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(parse_tap(&v, 10), vec!["row a=1 b=null".to_string()]);
    }
}
