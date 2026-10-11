use crate::json;
use crate::net::get;

const ENDPOINT: &str = "https://vires.services/hapi/catalog";
const BASE: &str = "https://vires.services/hapi/data?datasetId=";

pub fn vires_lines(query: &str, max: usize) -> Vec<String> {
    match get(ENDPOINT, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the vires response carries no JSON".to_string()];
            };
            let out = parse_vires(&v, query, max);
            if out.is_empty() {
                vec![format!("absent — vires carries no dataset: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — vires HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_vires(v: &json::Json, query: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("catalog").and_then(|c| c.as_arr()) else {
        return out;
    };
    let needle = query.trim().to_lowercase();
    for item in items {
        let Some(id) = item
            .get("id")
            .and_then(|i| i.as_str())
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        if !needle.is_empty() && !id.to_lowercase().contains(&needle) {
            continue;
        }
        out.push(format!("url {}{}", BASE, id));
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
    fn filters_ids() {
        let body = r#"{"HAPI":"3.0","catalog":[{"id":"SW_OPER_MAG"},{"id":"GF1_MAG_ACAL_CORR"}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_vires(&v, "mag", 10),
            vec![
                "url https://vires.services/hapi/data?datasetId=SW_OPER_MAG".to_string(),
                "url https://vires.services/hapi/data?datasetId=GF1_MAG_ACAL_CORR".to_string()
            ]
        );
    }

    #[test]
    fn caps_at_max() {
        let v = json::parse(r#"{"catalog":[{"id":"a"},{"id":"b"}]}"#).unwrap();
        assert_eq!(parse_vires(&v, "", 1).len(), 1);
    }
}
