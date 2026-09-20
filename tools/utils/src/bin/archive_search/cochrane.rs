use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://www.ebi.ac.uk/europepmc/webservices/rest/search";
const JOURNAL: &str = "Cochrane Database Syst Rev";

fn composed(query: &str) -> String {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        format!("JOURNAL:\"{}\"", JOURNAL)
    } else {
        format!("(JOURNAL:\"{}\") AND ({})", JOURNAL, trimmed)
    }
}

pub fn cochrane_lines(query: &str, max: usize) -> Vec<String> {
    let search = composed(query);
    let mut cursor: Option<String> = Some("*".to_string());
    let (mut lines, stop) = crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |_| {
        let mut url = format!(
            "{}?query={}&format=json&pageSize={}",
            ENDPOINT,
            urlencode(&search),
            max
        );
        if let Some(c) = &cursor {
            url.push_str("&cursorMark=");
            url.push_str(&urlencode(c));
        }
        match get(&url, &[], "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let out = parse_cochrane(&v);
                    let next = v
                        .get("nextCursorMark")
                        .and_then(|c| c.as_str())
                        .filter(|c| !c.is_empty() && Some(*c) != cursor.as_deref())
                        .map(str::to_string);
                    let has_more = next.is_some();
                    cursor = next;
                    (out, has_more)
                }
                None => (
                    vec!["pending — the cochrane response carries no JSON".to_string()],
                    false,
                ),
            },
            Some(f) => (
                vec![format!("pending — cochrane HTTP {}", f.status_text())],
                false,
            ),
            None => (vec!["pending — no network".to_string()], false),
        }
    });
    if lines.is_empty() {
        vec![format!("absent — cochrane carries no entry: {}", query)]
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

fn review_url(record: &json::Json) -> Option<String> {
    if let Some(doi) = field(record, "doi") {
        if doi.starts_with("10.1002/14651858") {
            return Some(format!(
                "https://www.cochranelibrary.com/cdsr/doi/{}/full",
                doi
            ));
        }
    }
    let pmid = field(record, "pmid")?;
    Some(format!("https://europepmc.org/article/MED/{}", pmid))
}

fn parse_cochrane(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(results) = v
        .get("resultList")
        .and_then(|r| r.get("result"))
        .and_then(|r| r.as_arr())
    else {
        return out;
    };
    for record in results {
        let Some(url) = review_url(record) else {
            continue;
        };
        let mut line = format!("url {}", url);
        if let Some(title) = field(record, "title") {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(doi) = field(record, "doi") {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        if let Some(year) = field(record, "pubYear") {
            line.push_str(&format!("\tyear: {}", year));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_the_journal_filter() {
        assert_eq!(
            composed("asthma"),
            "(JOURNAL:\"Cochrane Database Syst Rev\") AND (asthma)"
        );
        assert_eq!(composed("  "), "JOURNAL:\"Cochrane Database Syst Rev\"");
    }

    #[test]
    fn a_cochrane_doi_reads_the_library_url() {
        let body = r#"{"resultList":{"result":[{"doi":"10.1002/14651858.CD015136.pub2","title":"Telepharmacy","pubYear":"2026"}]}}"#;
        let v = json::parse(body).expect("the fixture parses");
        assert_eq!(
            parse_cochrane(&v),
            vec!["url https://www.cochranelibrary.com/cdsr/doi/10.1002/14651858.CD015136.pub2/full\ttitle: Telepharmacy\tdoi: 10.1002/14651858.CD015136.pub2\tyear: 2026".to_string()]
        );
    }

    #[test]
    fn a_non_cochrane_record_reads_the_europepmc_url() {
        let body = r#"{"resultList":{"result":[{"pmid":"42283624","title":"x"}]}}"#;
        let v = json::parse(body).expect("the fixture parses");
        assert_eq!(
            parse_cochrane(&v),
            vec!["url https://europepmc.org/article/MED/42283624\ttitle: x".to_string()]
        );
    }
}
