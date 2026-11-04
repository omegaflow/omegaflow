use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://www.wikidata.org/w/api.php";
const WIKI: &str = "https://www.wikidata.org/wiki/";

pub fn wikidata_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?action=wbsearchentities&search={}&language=en&format=json&limit={}",
        ENDPOINT,
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the wikidata response carries no JSON".to_string()];
            };
            let out = parse_wikidata(&v);
            if out.is_empty() {
                vec![format!("absent — wikidata carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — wikidata HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_wikidata(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("search").and_then(|s| s.as_arr()) else {
        return out;
    };
    for item in items {
        let Some(id) = item.get("id").and_then(|i| i.as_str()) else {
            continue;
        };
        let mut line = format!("url {}{}", WIKI, id);
        if let Some(label) = item
            .get("label")
            .and_then(|l| l.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tlabel: {}", label));
        }
        if let Some(description) = item
            .get("description")
            .and_then(|d| d.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tdescription: {}", description));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(body: &str) -> Vec<String> {
        parse_wikidata(&json::parse(body).expect("test body is JSON"))
    }

    #[test]
    fn reads_the_full_item() {
        let body = r#"{"search":[{"id":"Q17147155","label":"Transfer entropy","description":"measure the amount of directed transfer of information"}]}"#;
        assert_eq!(
            parse(body),
            vec!["url https://www.wikidata.org/wiki/Q17147155\tlabel: Transfer entropy\tdescription: measure the amount of directed transfer of information".to_string()]
        );
    }

    #[test]
    fn omits_absent_description() {
        let body = r#"{"search":[{"id":"Q1","label":"Only a label"}]}"#;
        assert_eq!(
            parse(body),
            vec!["url https://www.wikidata.org/wiki/Q1\tlabel: Only a label".to_string()]
        );
    }

    #[test]
    fn empty_search_carries_nothing() {
        assert!(parse(r#"{"search":[]}"#).is_empty());
    }
}
