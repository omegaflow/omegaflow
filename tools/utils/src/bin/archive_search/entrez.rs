use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi";

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
    if let Some(explicit) = parameter(query, "term") {
        return explicit;
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
            let out = parse_entrez(&f.body, &db, max);
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

fn parse_entrez(body: &str, db: &str, max: usize) -> Vec<String> {
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
        out.push(format!("url {}\tdb: {}\tid: {}", entry_url(db, id), db, id));
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
}
