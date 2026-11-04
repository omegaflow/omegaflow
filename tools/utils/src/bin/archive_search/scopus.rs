use crate::json;
use crate::net::{get, urlencode};

pub fn scopus_lines(query: &str, max: usize) -> Vec<String> {
    let Some(key) = crate::token::secret("ELSEVIER_API_KEY") else {
        return vec![
            "pending — scopus needs ELSEVIER_API_KEY (dev.elsevier.com; the Meta-Info API key is free)"
                .to_string(),
        ];
    };
    let url = format!(
        "https://api.elsevier.com/content/search/scopus?query={}&count={}",
        urlencode(query),
        max
    );
    let key_header = format!("X-ELS-APIKey: {}", key);
    let headers = ["-H", key_header.as_str(), "-H", "Accept: application/json"];
    match get(&url, &headers, "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_scopus(&f.body);
            if out.is_empty() {
                vec![format!("absent — scopus carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — scopus HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_scopus(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(entries) = v
        .get("search-results")
        .and_then(|s| s.get("entry"))
        .and_then(|e| e.as_arr())
    else {
        return out;
    };
    for entry in entries {
        let eid = field(entry, "eid");
        let doi = field(entry, "prism:doi").or_else(|| field(entry, "doi"));
        let url = match (&eid, &doi) {
            (_, Some(d)) => format!("https://doi.org/{}", d),
            (Some(e), None) => {
                format!("https://www.scopus.com/record/display.uri?eid={}", e)
            }
            (None, None) => continue,
        };
        let mut line = format!("url {}", url);
        if let Some(title) = field(entry, "dc:title") {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(author) = field(entry, "dc:creator") {
            line.push_str(&format!("\tauthor: {}", author));
        }
        if let Some(journal) = field(entry, "prism:publicationName") {
            line.push_str(&format!("\tjournal: {}", journal));
        }
        if let Some(date) = field(entry, "prism:coverDate") {
            line.push_str(&format!("\tdate: {}", date));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_scopus_entries() {
        let body = r#"{"search-results":{"entry":[{"eid":"2-s2.0-1","prism:doi":"10.1234/abc","dc:title":"A measured field","dc:creator":"Lovelace A.","prism:publicationName":"J. Test","prism:coverDate":"2020-01-01"}]}}"#;
        assert_eq!(
            parse_scopus(body),
            vec![
                "url https://doi.org/10.1234/abc\ttitle: A measured field\tauthor: Lovelace A.\tjournal: J. Test\tdate: 2020-01-01"
                    .to_string()
            ]
        );
    }

    #[test]
    fn falls_back_to_the_eid_without_a_doi() {
        let body = r#"{"search-results":{"entry":[{"eid":"2-s2.0-2","dc:title":"No doi"}]}}"#;
        assert_eq!(
            parse_scopus(body),
            vec![
                "url https://www.scopus.com/record/display.uri?eid=2-s2.0-2\ttitle: No doi"
                    .to_string()
            ]
        );
    }

    #[test]
    fn skips_the_entry_without_eid_or_doi() {
        let body = r#"{"search-results":{"entry":[{"dc:title":"no id"},{"eid":"2-s2.0-3","dc:title":"Kept"}]}}"#;
        assert_eq!(
            parse_scopus(body),
            vec![
                "url https://www.scopus.com/record/display.uri?eid=2-s2.0-3\ttitle: Kept"
                    .to_string()
            ]
        );
    }

    #[test]
    fn empty_entries_carry_nothing() {
        assert!(parse_scopus(r#"{"search-results":{"entry":[]}}"#).is_empty());
    }
}
