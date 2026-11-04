use crate::json;
use crate::net::{get, urlencode};

pub fn mendeley_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://data.mendeley.com/api/datasets?query={}&limit={}",
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match parse_mendeley(&f.body) {
            Some(out) if out.is_empty() => {
                vec![format!("absent — mendeley carries no dataset: {}", query)]
            }
            Some(out) => out,
            None => vec!["pending — the mendeley response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — mendeley HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_mendeley(body: &str) -> Option<Vec<String>> {
    let v = json::parse(body)?;
    let mut out = Vec::new();
    let Some(items) = v
        .get("data")
        .and_then(|d| d.get("results"))
        .and_then(|r| r.as_arr())
    else {
        return Some(out);
    };
    for item in items {
        let Some(id) = item
            .get("id")
            .and_then(|i| i.as_str())
            .filter(|i| !i.is_empty())
        else {
            continue;
        };
        let mut line = format!("url https://data.mendeley.com/datasets/{}", id);
        if let Some(title) = item
            .get("name")
            .and_then(|n| n.as_str())
            .filter(|n| !n.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(doi) = item
            .get("doi")
            .and_then(|d| d.get("id"))
            .and_then(|i| i.as_str())
            .filter(|d| !d.is_empty())
        {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        if let Some(version) = item.get("version").and_then(|v| v.as_scalar_string()) {
            line.push_str(&format!("\tversion: {}", version));
        }
        if let Some(date) = item
            .get("publish_date")
            .and_then(|d| d.as_str())
            .filter(|d| !d.is_empty())
        {
            line.push_str(&format!("\tdate: {}", date));
        }
        out.push(line);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_entry_fields() {
        let body = r#"{"data":{"results":[{"id":"jxj8bn2vyf","doi":{"id":"10.17632/jxj8bn2vyf.3","status":"allocated"},"name":"Paleomagnetic study of a volcanic sequence","version":3,"publish_date":"2020-05-01T00:00:00Z"}]}}"#;
        assert_eq!(
            parse_mendeley(body),
            Some(vec!["url https://data.mendeley.com/datasets/jxj8bn2vyf\ttitle: Paleomagnetic study of a volcanic sequence\tdoi: 10.17632/jxj8bn2vyf.3\tversion: 3\tdate: 2020-05-01T00:00:00Z".to_string()])
        );
    }

    #[test]
    fn keeps_the_id_without_title_and_doi() {
        let body = r#"{"data":{"results":[{"id":"abcd1234"}]}}"#;
        assert_eq!(
            parse_mendeley(body),
            Some(vec![
                "url https://data.mendeley.com/datasets/abcd1234".to_string()
            ])
        );
    }

    #[test]
    fn skips_the_element_without_id() {
        let body = r#"{"data":{"results":[{"name":"No id"},{"id":"keep01","name":"Kept"}]}}"#;
        assert_eq!(
            parse_mendeley(body),
            Some(vec![
                "url https://data.mendeley.com/datasets/keep01\ttitle: Kept".to_string()
            ])
        );
    }

    #[test]
    fn empty_results_carry_nothing() {
        assert_eq!(
            parse_mendeley(r#"{"data":{"results":[]}}"#),
            Some(Vec::new())
        );
    }

    #[test]
    fn absent_data_carries_nothing() {
        assert_eq!(parse_mendeley(r#"{"links":{}}"#), Some(Vec::new()));
    }

    #[test]
    fn a_non_json_body_is_absent() {
        assert_eq!(parse_mendeley("<html>not json</html>"), None);
    }
}
