use crate::json;
use crate::net::{get, urlencode};

pub fn openalex_lines(query: &str, max: usize) -> Vec<String> {
    let mailto = crate::token::secret("OPENALEX_MAILTO");
    let mut cursor: Option<String> = Some("*".to_string());
    let (mut lines, stop) = crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |_| {
        let mut url = format!(
            "https://api.openalex.org/works?search={}&per-page={}",
            urlencode(query),
            max
        );
        if let Some(mail) = &mailto {
            url.push_str("&mailto=");
            url.push_str(&urlencode(mail));
        }
        if let Some(c) = &cursor {
            url.push_str("&cursor=");
            url.push_str(&urlencode(c));
        }
        match get(&url, &[], "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let out = parse_openalex(&f.body);
                    let next = v
                        .get("meta")
                        .and_then(|m| m.get("next_cursor"))
                        .and_then(|c| c.as_str())
                        .filter(|c| !c.is_empty())
                        .map(str::to_string);
                    let has_more = next.is_some();
                    cursor = next;
                    (out, has_more)
                }
                None => (
                    vec!["pending — the openalex response carries no JSON".to_string()],
                    false,
                ),
            },
            Some(f) if f.status == Some(429) => (
                vec!["pending — openalex HTTP 429 (rate limit); the polite pool needs a mailto (OPENALEX_MAILTO in .secrets.local)".to_string()],
                false,
            ),
            Some(f) => (
                vec![format!("pending — openalex HTTP {}", f.status_text())],
                false,
            ),
            None => (vec!["pending — no network".to_string()], false),
        }
    });
    if lines.is_empty() {
        vec![format!("absent — openalex carries no entry: {}", query)]
    } else {
        lines.push(format!("end: {}", stop.label()));
        lines
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_openalex(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for work in results {
        let Some(id) = field(work, "id") else {
            continue;
        };
        let mut line = format!("url {}", id);
        if let Some(doi) = field(work, "doi") {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        if let Some(title) = field(work, "title") {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(year) = work
            .get("publication_year")
            .and_then(|y| y.as_scalar_string())
        {
            line.push_str(&format!("\tyear: {}", year));
        }
        if let Some(cites) = work
            .get("cited_by_count")
            .and_then(|c| c.as_scalar_string())
        {
            line.push_str(&format!("\tcites: {}", cites));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_work_fields() {
        let body = r#"{"results":[{"id":"https://openalex.org/W123","doi":"https://doi.org/10.1234/abc","title":"A measured field","publication_year":2020,"cited_by_count":123,"referenced_works":["https://openalex.org/W9"],"authorships":[]}],"meta":{"count":1}}"#;
        assert_eq!(
            parse_openalex(body),
            vec!["url https://openalex.org/W123\tdoi: https://doi.org/10.1234/abc\ttitle: A measured field\tyear: 2020\tcites: 123".to_string()]
        );
    }

    #[test]
    fn omits_the_absent_doi() {
        let body = r#"{"results":[{"id":"https://openalex.org/W123","doi":null,"title":"A measured field","publication_year":2020,"cited_by_count":123}]}"#;
        assert_eq!(
            parse_openalex(body),
            vec![
                "url https://openalex.org/W123\ttitle: A measured field\tyear: 2020\tcites: 123"
                    .to_string()
            ]
        );
    }

    #[test]
    fn skips_the_work_without_id() {
        let body = r#"{"results":[{"title":"No id","publication_year":2020},{"id":"https://openalex.org/W123","title":"Kept"}]}"#;
        assert_eq!(
            parse_openalex(body),
            vec!["url https://openalex.org/W123\ttitle: Kept".to_string()]
        );
    }

    #[test]
    fn empty_results_carry_nothing() {
        assert!(parse_openalex(r#"{"results":[],"meta":{"count":0}}"#).is_empty());
    }
}
