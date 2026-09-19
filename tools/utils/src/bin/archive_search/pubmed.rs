use crate::json;
use crate::net::{get, urlencode};

pub fn pubmed_lines(query: &str, max: usize) -> Vec<String> {
    let mut retstart = 0usize;
    let (mut lines, stop) = crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |_| {
        let url = format!(
            "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=pubmed&term={}&retmax={}&retstart={}&retmode=json&tool=omegaflow",
            urlencode(query),
            max,
            retstart
        );
        match get(&url, &[], "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let ids = search_ids(&v);
                    let count = ids.len();
                    let out = if ids.is_empty() {
                        Vec::new()
                    } else {
                        fetch_summaries(&ids)
                    };
                    retstart += count;
                    (out, count >= max)
                }
                None => (
                    vec!["pending — the pubmed response carries no JSON".to_string()],
                    false,
                ),
            },
            Some(f) => (
                vec![format!("pending — pubmed HTTP {}", f.status_text())],
                false,
            ),
            None => (vec!["pending — no network".to_string()], false),
        }
    });
    if lines.is_empty() {
        vec![format!("absent — pubmed carries no entry: {}", query)]
    } else {
        lines.push(format!("end: {}", stop.label()));
        lines
    }
}

fn search_ids(v: &json::Json) -> Vec<String> {
    let Some(a) = v
        .get("esearchresult")
        .and_then(|r| r.get("idlist"))
        .and_then(|l| l.as_arr())
    else {
        return Vec::new();
    };
    a.iter()
        .filter_map(|id| id.as_str().map(str::to_string))
        .collect()
}

fn fetch_summaries(ids: &[String]) -> Vec<String> {
    let url = format!(
        "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esummary.fcgi?db=pubmed&id={}&retmode=json&tool=omegaflow",
        ids.join(",")
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => parse_summaries(&f.body),
        Some(f) => vec![format!("pending — pubmed esummary HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn doc_doi(doc: &json::Json) -> Option<String> {
    doc.get("articleids")
        .and_then(|a| a.as_arr())?
        .iter()
        .find(|id| id.get("idtype").and_then(|t| t.as_str()) == Some("doi"))
        .and_then(|id| id.get("value").and_then(|v| v.as_str()))
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_summaries(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(result) = v.get("result") else {
        return out;
    };
    let Some(uids) = result.get("uids").and_then(|u| u.as_arr()) else {
        return out;
    };
    for uid in uids {
        let Some(id) = uid.as_str() else {
            continue;
        };
        let Some(doc) = result.get(id) else {
            continue;
        };
        let mut line = format!("url https://pubmed.ncbi.nlm.nih.gov/{}/", id);
        if let Some(title) = field(doc, "title") {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(journal) = field(doc, "source") {
            line.push_str(&format!("\tjournal: {}", journal));
        }
        if let Some(date) = field(doc, "pubdate") {
            line.push_str(&format!("\tdate: {}", date));
        }
        if let Some(doi) = doc_doi(doc) {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_search_id_list() {
        let v = json::parse(r#"{"esearchresult":{"idlist":["31452104","27142500"]}}"#).unwrap();
        assert_eq!(search_ids(&v), vec!["31452104", "27142500"]);
    }

    #[test]
    fn an_empty_search_carries_no_ids() {
        let v = json::parse(r#"{"esearchresult":{"idlist":[]}}"#).unwrap();
        assert!(search_ids(&v).is_empty());
    }

    #[test]
    fn reads_the_summary_fields() {
        let body = r#"{"result":{"uids":["31452104"],"31452104":{"title":"Heart rate variability","source":"Psychophysiology","pubdate":"2019","articleids":[{"idtype":"pubmed","value":"31452104"},{"idtype":"doi","value":"10.1111/psyp.13456"}]}}}"#;
        assert_eq!(
            parse_summaries(body),
            vec!["url https://pubmed.ncbi.nlm.nih.gov/31452104/\ttitle: Heart rate variability\tjournal: Psychophysiology\tdate: 2019\tdoi: 10.1111/psyp.13456".to_string()]
        );
    }

    #[test]
    fn omits_the_absent_doi() {
        let body = r#"{"result":{"uids":["1"],"1":{"title":"No doi","source":"J","pubdate":"2000"}}}"#;
        assert_eq!(
            parse_summaries(body),
            vec!["url https://pubmed.ncbi.nlm.nih.gov/1/\ttitle: No doi\tjournal: J\tdate: 2000"
                .to_string()]
        );
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_summaries(r#"{"result":{"uids":[]}}"#).is_empty());
    }
}
