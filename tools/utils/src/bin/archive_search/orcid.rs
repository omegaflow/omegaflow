use crate::json;
use crate::net::{get, urlencode};

pub fn orcid_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://pub.orcid.org/v3.0/expanded-search/?q={}&rows={}",
        urlencode(query),
        max
    );
    match get(&url, &["-H", "Accept: application/json"], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(_) => {
                let out = parse_orcid(&f.body);
                if out.is_empty() {
                    vec![format!("absent — orcid carries no record: {}", query)]
                } else {
                    out
                }
            }
            None => vec!["pending — the orcid response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — orcid HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn orcid_line(record: &json::Json) -> Option<String> {
    let id = record
        .get("orcid-id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())?;
    let mut line = format!("url https://orcid.org/{}", id);
    let given = record
        .get("given-names")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty());
    let family = record
        .get("family-names")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty());
    match (given, family) {
        (Some(g), Some(f)) => line.push_str(&format!("\tname: {} {}", g, f)),
        (Some(g), None) => line.push_str(&format!("\tname: {}", g)),
        (None, Some(f)) => line.push_str(&format!("\tname: {}", f)),
        (None, None) => {}
    }
    if let Some(inst) = record
        .get("institution-name")
        .and_then(|v| v.as_arr())
        .and_then(|a| a.first())
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
    {
        line.push_str(&format!("\tinstitution: {}", inst));
    }
    Some(line)
}

fn parse_orcid(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(records) = v.get("expanded-result").and_then(|r| r.as_arr()) else {
        return out;
    };
    for record in records {
        if let Some(line) = orcid_line(record) {
            out.push(line);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"{"num-found":1591,"expanded-result":[{"orcid-id":"0000-0002-3704-5398","given-names":"David","family-names":"Hawking","institution-name":["Australian National University","Microsoft Corp","University of Melbourne"]}]}"#;

    #[test]
    fn reads_a_researcher_line() {
        assert_eq!(
            parse_orcid(FIXTURE),
            vec!["url https://orcid.org/0000-0002-3704-5398\tname: David Hawking\tinstitution: Australian National University".to_string()]
        );
    }

    #[test]
    fn omits_an_absent_name_and_institution() {
        let body = r#"{"expanded-result":[{"orcid-id":"0000-0001-0000-0000"}]}"#;
        assert_eq!(
            parse_orcid(body),
            vec!["url https://orcid.org/0000-0001-0000-0000".to_string()]
        );
    }

    #[test]
    fn an_empty_or_non_json_result_carries_nothing() {
        assert!(parse_orcid(r#"{"expanded-result":[]}"#).is_empty());
        assert!(parse_orcid("<xml/>").is_empty());
        assert!(parse_orcid("").is_empty());
    }
}
