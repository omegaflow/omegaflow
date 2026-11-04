use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.semanticscholar.org/graph/v1/paper/search";
const DEFAULT_FIELDS: &str = "title,year,externalIds,url,citationCount,abstract,authors";

fn semanticscholar_url(
    text: &str,
    refine: &[(String, String)],
    max: usize,
    offset: Option<&str>,
) -> String {
    let fields = crate::refine::value_of(refine, "fields").unwrap_or(DEFAULT_FIELDS);
    let mut url = format!(
        "{}?query={}&limit={}&fields={}",
        ENDPOINT,
        urlencode(text),
        max,
        urlencode(fields)
    );
    for (key, value) in refine {
        if key == "fields" {
            continue;
        }
        url.push('&');
        url.push_str(key);
        url.push('=');
        url.push_str(&urlencode(value));
    }
    if let Some(o) = offset {
        url.push_str("&offset=");
        url.push_str(&urlencode(o));
    }
    url
}

pub fn semanticscholar_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(
        query,
        &[
            "fields",
            "year",
            "venue",
            "publicationTypes",
            "minCitationCount",
            "sort",
        ],
    );
    let header = crate::token::secret("S2_API_KEY").map(|k| format!("x-api-key: {}", k));
    let extra: Vec<&str> = match &header {
        Some(h) => vec!["-H", h.as_str()],
        None => Vec::new(),
    };
    let mut next: Option<String> = Some("0".to_string());
    let (mut lines, stop) = crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |_| {
        let url = semanticscholar_url(&text, &refine, max, next.as_deref());
        match get(&url, &extra, "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let out = parse_semanticscholar(&f.body);
                    next = v.get("next").and_then(|n| n.as_scalar_string());
                    let has_more = next.is_some();
                    (out, has_more)
                }
                None => (
                    vec!["pending — the semanticscholar response carries no JSON".to_string()],
                    false,
                ),
            },
            Some(f) if f.status == Some(429) => match &header {
                Some(_) => (
                    vec!["pending — semanticscholar rate limit (HTTP 429) even with S2_API_KEY"
                        .to_string()],
                    false,
                ),
                None => (
                    vec!["pending — semanticscholar rate limit (keyless shared pool); S2_API_KEY in .secrets.local lifts it"
                        .to_string()],
                    false,
                ),
            },
            Some(f) if f.status == Some(403) => (
                vec!["pending — semanticscholar refuses the key (HTTP 403); check S2_API_KEY"
                    .to_string()],
                false,
            ),
            Some(f) => (
                vec![format!("pending — semanticscholar HTTP {}", f.status_text())],
                false,
            ),
            None => (vec!["pending — no network".to_string()], false),
        }
    });
    if lines.is_empty() {
        vec![format!(
            "absent — semanticscholar carries no entry: {}",
            query
        )]
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

fn authors_field(paper: &json::Json) -> Option<String> {
    let arr = paper.get("authors").and_then(|a| a.as_arr())?;
    let names: Vec<String> = arr.iter().filter_map(|a| field(a, "name")).collect();
    if names.is_empty() {
        return None;
    }
    let mut rendered = names.iter().take(8).cloned().collect::<Vec<_>>().join(", ");
    if names.len() > 8 {
        rendered.push_str(", et al.");
    }
    Some(rendered)
}

fn truncate_chars(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    s.chars().take(max).collect()
}

fn abstract_field(paper: &json::Json) -> Option<String> {
    let s = field(paper, "abstract")?;
    Some(truncate_chars(&s, 1200))
}

fn parse_semanticscholar(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(data) = v.get("data").and_then(|d| d.as_arr()) else {
        return out;
    };
    for paper in data {
        let Some(id) = field(paper, "paperId") else {
            continue;
        };
        let url = match field(paper, "url") {
            Some(u) => u,
            None => format!("https://www.semanticscholar.org/paper/{}", id),
        };
        let mut line = format!("url {}", url);
        if let Some(title) = field(paper, "title") {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(authors) = authors_field(paper) {
            line.push_str(&format!("\tauthors: {}", authors));
        }
        if let Some(ids) = paper.get("externalIds") {
            if let Some(doi) = field(ids, "DOI") {
                line.push_str(&format!("\tdoi: {}", doi));
            }
            if let Some(arxiv) = field(ids, "ArXiv") {
                line.push_str(&format!("\tarxiv: {}", arxiv));
            }
        }
        if let Some(year) = paper.get("year").and_then(|y| y.as_scalar_string()) {
            line.push_str(&format!("\tyear: {}", year));
        }
        if let Some(abstract_text) = abstract_field(paper) {
            line.push_str(&format!("\tabstract: {}", abstract_text));
        }
        if let Some(cites) = paper
            .get("citationCount")
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
    fn reads_the_paper_fields() {
        let body = r#"{"total":1,"data":[{"paperId":"abc","title":"Asthma","year":2019,"externalIds":{"DOI":"10.1/x","ArXiv":"1901.00001"},"url":"https://www.semanticscholar.org/paper/abc","citationCount":5}]}"#;
        assert_eq!(
            parse_semanticscholar(body),
            vec!["url https://www.semanticscholar.org/paper/abc\ttitle: Asthma\tdoi: 10.1/x\tarxiv: 1901.00001\tyear: 2019\tcites: 5".to_string()]
        );
    }

    #[test]
    fn a_paper_without_an_id_carries_nothing() {
        assert!(parse_semanticscholar(r#"{"data":[{"title":"x"}]}"#).is_empty());
    }

    #[test]
    fn the_authors_and_abstract_speak_when_the_paper_carries_them() {
        let body = r#"{"data":[{"paperId":"abc","title":"Asthma","year":2019,"abstract":"A study of asthma.","authors":[{"name":"A. One"},{"name":"B. Two"}],"url":"https://www.semanticscholar.org/paper/abc"}]}"#;
        assert_eq!(
            parse_semanticscholar(body),
            vec!["url https://www.semanticscholar.org/paper/abc\ttitle: Asthma\tauthors: A. One, B. Two\tyear: 2019\tabstract: A study of asthma.".to_string()]
        );
    }

    #[test]
    fn more_than_eight_authors_carry_et_al() {
        let names: Vec<String> = (1..=9)
            .map(|i| format!("{{\"name\":\"A. {}\"}}", i))
            .collect();
        let body = format!(
            "{{\"data\":[{{\"paperId\":\"abc\",\"authors\":[{}]}}]}}",
            names.join(",")
        );
        assert_eq!(
            parse_semanticscholar(&body),
            vec!["url https://www.semanticscholar.org/paper/abc\tauthors: A. 1, A. 2, A. 3, A. 4, A. 5, A. 6, A. 7, A. 8, et al.".to_string()]
        );
    }

    #[test]
    fn an_absent_or_empty_abstract_and_authors_are_omitted() {
        let body = r#"{"data":[{"paperId":"abc","title":"Asthma","abstract":"","authors":[]}]}"#;
        assert_eq!(
            parse_semanticscholar(body),
            vec!["url https://www.semanticscholar.org/paper/abc\ttitle: Asthma".to_string()]
        );
    }

    #[test]
    fn an_abstract_longer_than_1200_chars_is_cut_at_the_boundary() {
        let long = "é".repeat(1300);
        let body = format!(
            "{{\"data\":[{{\"paperId\":\"abc\",\"abstract\":\"{}\"}}]}}",
            long
        );
        let lines = parse_semanticscholar(&body);
        let abstract_text = lines[0]
            .split("\tabstract: ")
            .nth(1)
            .expect("abstract present");
        assert_eq!(abstract_text.chars().count(), 1200);
        assert!(abstract_text.chars().all(|c| c == 'é'));
    }

    #[test]
    fn the_url_carries_the_default_fields_and_the_offset() {
        assert_eq!(
            semanticscholar_url("asthma", &[], 10, Some("0")),
            "https://api.semanticscholar.org/graph/v1/paper/search?query=asthma&limit=10&fields=title%2Cyear%2CexternalIds%2Curl%2CcitationCount%2Cabstract%2Cauthors&offset=0"
        );
    }

    #[test]
    fn the_url_lifts_year_venue_and_a_custom_fields_projection() {
        let opts = vec![
            ("year".to_string(), "2019".to_string()),
            ("venue".to_string(), "NeurIPS".to_string()),
            ("fields".to_string(), "title,year".to_string()),
        ];
        assert_eq!(
            semanticscholar_url("transformers", &opts, 20, None),
            "https://api.semanticscholar.org/graph/v1/paper/search?query=transformers&limit=20&fields=title%2Cyear&year=2019&venue=NeurIPS"
        );
    }
}
