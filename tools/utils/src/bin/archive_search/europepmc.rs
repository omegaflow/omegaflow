use crate::json;
use crate::net::{get, urlencode};

pub fn europepmc_lines(query: &str, max: usize) -> Vec<String> {
    let mut cursor: Option<String> = Some("*".to_string());
    let (mut lines, stop) = crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |_| {
        let mut url = format!(
            "https://www.ebi.ac.uk/europepmc/webservices/rest/search?query={}&format=json&pageSize={}",
            urlencode(query),
            max
        );
        if let Some(c) = &cursor {
            url.push_str("&cursorMark=");
            url.push_str(&urlencode(c));
        }
        match get(&url, &[], "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let out = parse_europepmc(&f.body);
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
                    vec!["pending — the europepmc response carries no JSON".to_string()],
                    false,
                ),
            },
            Some(f) => (
                vec![format!("pending — europepmc HTTP {}", f.status_text())],
                false,
            ),
            None => (vec!["pending — no network".to_string()], false),
        }
    });
    if lines.is_empty() {
        vec![format!("absent — europepmc carries no entry: {}", query)]
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

fn article_url(record: &json::Json) -> Option<String> {
    if let Some(pmid) = field(record, "pmid") {
        return Some(format!("https://europepmc.org/article/MED/{}", pmid));
    }
    let id = field(record, "id")?;
    let source = field(record, "source")?;
    Some(format!("https://europepmc.org/article/{}/{}", source, id))
}

fn parse_europepmc(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v
        .get("resultList")
        .and_then(|r| r.get("result"))
        .and_then(|r| r.as_arr())
    else {
        return out;
    };
    for record in results {
        let Some(url) = article_url(record) else {
            continue;
        };
        let mut line = format!("url {}", url);
        if let Some(doi) = field(record, "doi") {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        if let Some(title) = field(record, "title") {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(year) = field(record, "pubYear") {
            line.push_str(&format!("\tyear: {}", year));
        }
        if let Some(cites) = record
            .get("citedByCount")
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
    fn reads_the_result_fields() {
        let body = r#"{"hitCount":1,"nextCursorMark":"AoE","resultList":{"result":[{"id":"31452104","source":"MED","pmid":"31452104","doi":"10.1111/psyp.13456","title":"Heart rate variability","pubYear":"2019","citedByCount":5}]}}"#;
        assert_eq!(
            parse_europepmc(body),
            vec!["url https://europepmc.org/article/MED/31452104\tdoi: 10.1111/psyp.13456\ttitle: Heart rate variability\tyear: 2019\tcites: 5".to_string()]
        );
    }

    #[test]
    fn falls_back_to_the_source_id_without_a_pmid() {
        let body =
            r#"{"resultList":{"result":[{"id":"PPR123","source":"PPR","title":"A preprint"}]}}"#;
        assert_eq!(
            parse_europepmc(body),
            vec!["url https://europepmc.org/article/PPR/PPR123\ttitle: A preprint".to_string()]
        );
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_europepmc(r#"{"resultList":{"result":[]}}"#).is_empty());
    }
}
