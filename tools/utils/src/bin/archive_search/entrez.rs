use crate::json;
use crate::net::{get, urlencode};
use std::collections::HashMap;

const ENDPOINT: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi";
const SUMMARY_ENDPOINT: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esummary.fcgi";
const MAX_AUTHORS: usize = 8;

fn parameter(query: &str, key: &str) -> Option<String> {
    query.split_whitespace().find_map(|token| {
        let (name, value) = token.split_once('=')?;
        if name == key && !value.is_empty() {
            Some(value.to_string())
        } else {
            None
        }
    })
}

fn term(query: &str) -> String {
    if let Some((_, rest)) = query.split_once("term=") {
        let explicit = rest.trim();
        if !explicit.is_empty() {
            return explicit.to_string();
        }
    }
    query
        .split_whitespace()
        .filter(|token| !token.starts_with("db=") && !token.starts_with("term="))
        .collect::<Vec<_>>()
        .join(" ")
}

fn entry_url(db: &str, id: &str) -> String {
    match db {
        "pubmed" => format!("https://pubmed.ncbi.nlm.nih.gov/{}/", id),
        "gds" => format!("https://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc={}", id),
        _ => format!("https://www.ncbi.nlm.nih.gov/{}/{}", db, id),
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

pub fn entrez_lines(query: &str, max: usize) -> Vec<String> {
    let Some(db) = parameter(query, "db") else {
        return vec!["usage — entrez needs db=<database>: db=nuccore|sra|gds <term>".to_string()];
    };
    let term = term(query);
    if term.is_empty() {
        return vec!["usage — entrez needs a term: db=<database> <term>".to_string()];
    }
    let url = format!(
        "{}?db={}&term={}&retmax={}&retmode=json&tool=omegaflow",
        ENDPOINT,
        urlencode(&db),
        urlencode(&term),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = if db == "pubmed" {
                let summaries = fetch_pubmed_summaries(&f.body, max);
                render_entrez(&f.body, &db, max, &summaries)
            } else {
                parse_entrez(&f.body, &db, max)
            };
            if out.is_empty() {
                vec![format!("absent — entrez carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — entrez HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn esearch_ids(body: &str, max: usize) -> Vec<String> {
    let Some(v) = json::parse(body) else {
        return Vec::new();
    };
    let Some(ids) = v
        .get("esearchresult")
        .and_then(|r| r.get("idlist"))
        .and_then(|l| l.as_arr())
    else {
        return Vec::new();
    };
    ids.iter()
        .filter_map(|id| id.as_str())
        .take(max)
        .map(str::to_string)
        .collect()
}

fn fetch_pubmed_summaries(esearch_body: &str, max: usize) -> HashMap<String, PubmedSummary> {
    let ids = esearch_ids(esearch_body, max);
    if ids.is_empty() {
        return HashMap::new();
    }
    let url = format!(
        "{}?db=pubmed&id={}&retmode=json&tool=omegaflow",
        SUMMARY_ENDPOINT,
        ids.join(",")
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => parse_pubmed_summaries(&f.body),
        _ => HashMap::new(),
    }
}

#[derive(Debug, Default, PartialEq)]
struct PubmedSummary {
    title: Option<String>,
    authors: Option<String>,
    year: Option<String>,
}

impl PubmedSummary {
    fn render_fields(&self) -> String {
        let mut line = String::new();
        if let Some(title) = &self.title {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(authors) = &self.authors {
            line.push_str(&format!("\tauthors: {}", authors));
        }
        if let Some(year) = &self.year {
            line.push_str(&format!("\tyear: {}", year));
        }
        line
    }
}

fn format_authors(entry: &json::Json) -> Option<String> {
    let authors = entry.get("authors").and_then(|a| a.as_arr())?;
    let names: Vec<&str> = authors
        .iter()
        .filter_map(|a| a.get("name").and_then(|n| n.as_str()))
        .filter(|n| !n.is_empty())
        .collect();
    if names.is_empty() {
        return None;
    }
    if names.len() > MAX_AUTHORS {
        Some(format!("{}, et al.", names[..MAX_AUTHORS].join(", ")))
    } else {
        Some(names.join(", "))
    }
}

fn parse_pubmed_summaries(body: &str) -> HashMap<String, PubmedSummary> {
    let mut out = HashMap::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(result) = v.get("result") else {
        return out;
    };
    let json::Json::Obj(map) = result else {
        return out;
    };
    for (uid, entry) in map {
        if !matches!(entry, json::Json::Obj(_)) {
            continue;
        }
        out.insert(
            uid.clone(),
            PubmedSummary {
                title: field(entry, "title"),
                authors: format_authors(entry),
                year: field(entry, "pubdate").and_then(|d| {
                    d.split_whitespace()
                        .next()
                        .filter(|t| !t.is_empty())
                        .map(str::to_string)
                }),
            },
        );
    }
    out
}

fn parse_entrez(body: &str, db: &str, max: usize) -> Vec<String> {
    render_entrez(body, db, max, &HashMap::new())
}

fn render_entrez(
    body: &str,
    db: &str,
    max: usize,
    summaries: &HashMap<String, PubmedSummary>,
) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(result) = v.get("esearchresult") else {
        return out;
    };
    let Some(ids) = result.get("idlist").and_then(|l| l.as_arr()) else {
        return out;
    };
    if ids.is_empty() {
        return out;
    }
    if let Some(count) = result.get("count").and_then(|c| c.as_scalar_string()) {
        let mut header = format!("count {}", count);
        if let Some(translation) = field(result, "querytranslation") {
            header.push_str(&format!("\tquery: {}", translation));
        }
        out.push(header);
    } else if let Some(translation) = field(result, "querytranslation") {
        out.push(format!("query: {}", translation));
    }
    for id in ids.iter().filter_map(|id| id.as_str()).take(max) {
        let mut line = format!("url {}\tdb: {}\tid: {}", entry_url(db, id), db, id);
        if let Some(summary) = summaries.get(id) {
            line.push_str(&summary.render_fields());
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_search_id_list_and_renders_urls() {
        let body = r#"{"header":{"type":"esearch","version":"0.3"},"esearchresult":{"count":"2","retmax":"2","retstart":"0","idlist":["NM_007294.4","NM_000059.4"],"translationset":[],"querytranslation":"BRCA1[All Fields]"}}"#;
        assert_eq!(
            parse_entrez(body, "nuccore", 10),
            vec![
                "count 2\tquery: BRCA1[All Fields]".to_string(),
                "url https://www.ncbi.nlm.nih.gov/nuccore/NM_007294.4\tdb: nuccore\tid: NM_007294.4".to_string(),
                "url https://www.ncbi.nlm.nih.gov/nuccore/NM_000059.4\tdb: nuccore\tid: NM_000059.4".to_string(),
            ]
        );
    }

    #[test]
    fn reads_a_numeric_count_and_the_geo_url() {
        let body =
            r#"{"esearchresult":{"count":1,"idlist":["GSE123"],"querytranslation":"cancer"}}"#;
        assert_eq!(
            parse_entrez(body, "gds", 10),
            vec![
                "count 1\tquery: cancer".to_string(),
                "url https://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc=GSE123\tdb: gds\tid: GSE123"
                    .to_string(),
            ]
        );
    }

    #[test]
    fn the_id_list_is_capped_at_max() {
        let body = r#"{"esearchresult":{"count":"3","idlist":["1","2","3"]}}"#;
        let out = parse_entrez(body, "sra", 2);
        assert_eq!(out.len(), 3);
        assert!(out[2].ends_with("id: 2"));
    }

    #[test]
    fn an_empty_search_carries_nothing() {
        assert!(
            parse_entrez(
                r#"{"esearchresult":{"count":"0","idlist":[]}}"#,
                "nuccore",
                10
            )
            .is_empty()
        );
    }

    #[test]
    fn a_body_without_esearchresult_carries_nothing() {
        assert!(parse_entrez("{}", "nuccore", 10).is_empty());
    }

    #[test]
    fn reads_db_and_term_tokens() {
        assert_eq!(parameter("db=sra SRR123", "db").as_deref(), Some("sra"));
        assert_eq!(term("db=sra SRR123"), "SRR123");
        assert_eq!(
            term("db=nuccore term=BRCA1[All Fields]"),
            "BRCA1[All Fields]"
        );
        assert_eq!(
            term("db=nuccore BRCA1 breast cancer"),
            "BRCA1 breast cancer"
        );
    }

    #[test]
    fn a_bare_query_is_usage_not_network() {
        let out = entrez_lines("BRCA1", 10);
        assert_eq!(out.len(), 1);
        assert!(out[0].starts_with("usage — entrez needs db="));
    }

    #[test]
    fn a_db_without_a_term_is_usage_not_network() {
        let out = entrez_lines("db=nuccore", 10);
        assert_eq!(out.len(), 1);
        assert!(out[0].starts_with("usage — entrez needs a term"));
    }

    #[test]
    fn pubmed_summaries_carry_title_authors_and_year() {
        let body = r#"{"header":{"type":"esummary","version":"0.3"},"result":{"uids":["123","456"],"123":{"uid":"123","pubdate":"2019 Jan","title":"A study of things","authors":[{"name":"A One"},{"name":"B Two"},{"name":"C Three"},{"name":"D Four"},{"name":"E Five"},{"name":"F Six"},{"name":"G Seven"},{"name":"H Eight"},{"name":"I Nine"}]},"456":{"uid":"456"}}}"#;
        let summaries = parse_pubmed_summaries(body);
        let doc = summaries.get("123").unwrap();
        assert_eq!(doc.title.as_deref(), Some("A study of things"));
        assert_eq!(
            doc.authors.as_deref(),
            Some("A One, B Two, C Three, D Four, E Five, F Six, G Seven, H Eight, et al.")
        );
        assert_eq!(doc.year.as_deref(), Some("2019"));
        assert_eq!(summaries.get("456").unwrap(), &PubmedSummary::default());
    }

    #[test]
    fn a_pubmed_line_carries_its_summary_fields() {
        let esearch =
            r#"{"esearchresult":{"count":"2","idlist":["123","456"],"querytranslation":"cancer"}}"#;
        let esummary = r#"{"result":{"123":{"uid":"123","title":"A study","pubdate":"2019 Jan","authors":[{"name":"A One"}]},"456":{"uid":"456"}}}"#;
        let summaries = parse_pubmed_summaries(esummary);
        assert_eq!(
            render_entrez(esearch, "pubmed", 10, &summaries),
            vec![
                "count 2\tquery: cancer".to_string(),
                "url https://pubmed.ncbi.nlm.nih.gov/123/\tdb: pubmed\tid: 123\ttitle: A study\tauthors: A One\tyear: 2019".to_string(),
                "url https://pubmed.ncbi.nlm.nih.gov/456/\tdb: pubmed\tid: 456".to_string(),
            ]
        );
    }

    #[test]
    fn a_pubmed_line_without_a_summary_keeps_its_id() {
        let esearch = r#"{"esearchresult":{"idlist":["789"]}}"#;
        assert_eq!(
            render_entrez(esearch, "pubmed", 10, &HashMap::new()),
            vec!["url https://pubmed.ncbi.nlm.nih.gov/789/\tdb: pubmed\tid: 789".to_string()]
        );
    }

    #[test]
    fn a_summary_body_without_result_carries_nothing() {
        assert!(parse_pubmed_summaries("{}").is_empty());
        assert!(parse_pubmed_summaries("not json").is_empty());
    }
}
