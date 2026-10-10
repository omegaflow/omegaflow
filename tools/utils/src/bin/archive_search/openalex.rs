use crate::json;
use crate::net::{get, urlencode};

pub fn openalex_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(query, &["filter", "select", "sort"]);
    let mailto = crate::token::secret("OPENALEX_MAILTO");
    let api_key = crate::token::secret("OPENALEX_API_KEY");
    let mut cursor: Option<String> = Some("*".to_string());
    let (mut lines, stop) = crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |_| {
        let mut url = format!("https://api.openalex.org/works?per-page={}", max);
        if !text.is_empty() {
            url.push_str("&search=");
            url.push_str(&urlencode(&text));
        }
        for (key, value) in &refine {
            url.push('&');
            url.push_str(key);
            url.push('=');
            url.push_str(&urlencode(value));
        }
        if let Some(mail) = &mailto {
            url.push_str("&mailto=");
            url.push_str(&urlencode(mail));
        }
        if let Some(key) = &api_key {
            url.push_str("&api_key=");
            url.push_str(&urlencode(key));
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

fn authors_line(work: &json::Json) -> Option<String> {
    let authorships = work.get("authorships").and_then(|a| a.as_arr())?;
    let mut names: Vec<String> = Vec::new();
    for authorship in authorships {
        if let Some(name) = authorship
            .get("author")
            .and_then(|a| a.get("display_name"))
            .and_then(|n| n.as_str())
            .filter(|s| !s.is_empty())
        {
            names.push(name.to_string());
        }
    }
    if names.is_empty() {
        return None;
    }
    if names.len() > 8 {
        let mut capped = names[..8].join(", ");
        capped.push_str(", et al.");
        Some(capped)
    } else {
        Some(names.join(", "))
    }
}

fn abstract_line(work: &json::Json) -> Option<String> {
    let index = match work.get("abstract_inverted_index") {
        Some(json::Json::Obj(map)) => map,
        _ => return None,
    };
    let mut positions: Vec<(usize, String)> = Vec::new();
    for (word, pos_json) in index {
        let Some(pos_list) = pos_json.as_arr() else {
            continue;
        };
        for pos in pos_list {
            if let Some(parsed) = pos.as_scalar_string().and_then(|s| s.parse::<usize>().ok()) {
                positions.push((parsed, word.clone()));
            }
        }
    }
    if positions.is_empty() {
        return None;
    }
    positions.sort();
    let text = positions
        .into_iter()
        .map(|(_, word)| word)
        .collect::<Vec<_>>()
        .join(" ");
    if text.chars().count() > 1200 {
        let mut capped: String = text.chars().take(1200).collect();
        capped.push('…');
        Some(capped)
    } else {
        Some(text)
    }
}

fn oa_pdf_line(work: &json::Json) -> Option<String> {
    work.get("best_oa_location")
        .and_then(|location| location.get("pdf_url"))
        .and_then(|url| url.as_str())
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
        if let Some(authors) = authors_line(work) {
            line.push_str(&format!("\tauthors: {}", authors));
        }
        if let Some(abstract_text) = abstract_line(work) {
            line.push_str(&format!("\tabstract: {}", abstract_text));
        }
        if let Some(pdf) = oa_pdf_line(work) {
            line.push_str(&format!("\toa_pdf: {}", pdf));
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

    #[test]
    fn reads_the_new_work_fields() {
        let body = r#"{"results":[{"id":"https://openalex.org/W123","title":"A measured field","authorships":[{"author":{"display_name":"Ada Lovelace"}},{"author":{"display_name":"Alan Turing"}}],"abstract_inverted_index":{"field":[0],"a":[1],"measured":[2]},"best_oa_location":{"pdf_url":"https://example.org/w123.pdf"}}]}"#;
        assert_eq!(
            parse_openalex(body),
            vec!["url https://openalex.org/W123\ttitle: A measured field\tauthors: Ada Lovelace, Alan Turing\tabstract: field a measured\toa_pdf: https://example.org/w123.pdf".to_string()]
        );
    }

    #[test]
    fn omits_the_absent_new_fields() {
        let body = r#"{"results":[{"id":"https://openalex.org/W123","title":"A measured field"}]}"#;
        assert_eq!(
            parse_openalex(body),
            vec!["url https://openalex.org/W123\ttitle: A measured field".to_string()]
        );
    }

    #[test]
    fn reconstructs_the_inverted_index_in_positional_order() {
        let body = r#"{"results":[{"id":"https://openalex.org/W123","abstract_inverted_index":{"world":[1],"hello":[0],"again":[2]}}]}"#;
        assert_eq!(
            parse_openalex(body),
            vec!["url https://openalex.org/W123\tabstract: hello world again".to_string()]
        );
    }

    #[test]
    fn caps_the_authors_to_eight_with_et_al() {
        let names = (1..=9)
            .map(|n| format!(r#"{{"author":{{"display_name":"Author {n}"}}}}"#))
            .collect::<Vec<_>>()
            .join(",");
        let body = format!(
            r#"{{"results":[{{"id":"https://openalex.org/W123","authorships":[{names}]}}]}}"#
        );
        assert_eq!(
            parse_openalex(&body),
            vec!["url https://openalex.org/W123\tauthors: Author 1, Author 2, Author 3, Author 4, Author 5, Author 6, Author 7, Author 8, et al.".to_string()]
        );
    }
}
