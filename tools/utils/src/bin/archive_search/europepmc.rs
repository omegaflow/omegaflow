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

fn strip_markup(s: &str) -> Option<String> {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for ch in s.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    let collapsed = out.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        None
    } else {
        Some(collapsed)
    }
}

fn cap_abstract(s: &str, max: usize) -> String {
    if s.chars().count() > max {
        let mut truncated: String = s.chars().take(max).collect();
        truncated.push('…');
        truncated
    } else {
        s.to_string()
    }
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
        if let Some(authors) = field(record, "authorString") {
            line.push_str(&format!("\tauthors: {}", authors));
        }
        if let Some(abstract_raw) = field(record, "abstractText") {
            if let Some(abstract_text) = strip_markup(&abstract_raw) {
                line.push_str(&format!(
                    "\tabstract: {}",
                    cap_abstract(&abstract_text, 1500)
                ));
            }
        }
        if let Some(journal) = field(record, "journalTitle") {
            line.push_str(&format!("\tjournal: {}", journal));
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

    #[test]
    fn reads_authors_abstract_and_journal() {
        let body = r#"{"resultList":{"result":[{"id":"1","source":"MED","pmid":"1","authorString":"Smith J, Doe A","abstractText":"<p>Some <b>text</b> here</p>","journalTitle":"Journal of Tests"}]}}"#;
        assert_eq!(
            parse_europepmc(body),
            vec!["url https://europepmc.org/article/MED/1\tauthors: Smith J, Doe A\tabstract: Some text here\tjournal: Journal of Tests".to_string()]
        );
    }

    #[test]
    fn absent_fields_are_omitted() {
        let body =
            r#"{"resultList":{"result":[{"id":"1","source":"MED","pmid":"1","title":"T"}]}}"#;
        assert_eq!(
            parse_europepmc(body),
            vec!["url https://europepmc.org/article/MED/1\ttitle: T".to_string()]
        );
    }

    #[test]
    fn strips_tags_and_caps_the_abstract() {
        let long = "a".repeat(1600);
        let body = format!(
            r#"{{"resultList":{{"result":[{{"id":"1","source":"MED","pmid":"1","abstractText":"<p>hello   <b>world</b></p>{}"}}]}}}}"#,
            long
        );
        let lines = parse_europepmc(&body);
        let line = &lines[0];
        let abstract_field = line.split("\tabstract: ").nth(1).unwrap();
        assert!(abstract_field.starts_with("hello world"));
        assert!(!abstract_field.contains('<'));
        assert_eq!(abstract_field.chars().count(), 1501);
        assert!(abstract_field.ends_with('…'));
    }
}
