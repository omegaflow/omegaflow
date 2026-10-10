use crate::json;
use crate::net::{get, urlencode};

pub fn zenodo_lines(query: &str, max: usize) -> Vec<String> {
    let (mut lines, stop) = crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |page| {
        let url = format!(
            "https://zenodo.org/api/records?q={}&size={}&page={}",
            urlencode(query),
            max,
            page + 1
        );
        match get(&url, &[], "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let out = parse_zenodo(&f.body);
                    let has_more = match v
                        .get("hits")
                        .and_then(|h| h.get("total"))
                        .and_then(|t| t.as_scalar_string())
                        .and_then(|t| t.parse::<usize>().ok())
                    {
                        Some(total) => (page + 1) * max < total,
                        None => false,
                    };
                    (out, has_more)
                }
                None => (
                    vec!["pending — the zenodo response carries no JSON".to_string()],
                    false,
                ),
            },
            Some(f) => (
                vec![format!("pending — zenodo HTTP {}", f.status_text())],
                false,
            ),
            None => (vec!["pending — no network".to_string()], false),
        }
    });
    if lines.is_empty() {
        vec![format!("absent — zenodo carries no entry: {}", query)]
    } else {
        lines.push(format!("end: {}", stop.label()));
        lines
    }
}

fn first_file(hit: &json::Json) -> Option<String> {
    hit.get("files")
        .and_then(|f| f.as_arr())
        .and_then(|a| a.first())
        .and_then(|f| f.get("links"))
        .and_then(|l| l.get("self"))
        .and_then(|l| l.as_str())
        .filter(|l| !l.is_empty())
        .map(str::to_string)
}

fn authors_field(metadata: &json::Json) -> Option<String> {
    let creators = metadata.get("creators").and_then(|a| a.as_arr())?;
    let names: Vec<String> = creators
        .iter()
        .filter_map(|c| c.get("name").and_then(|s| s.as_str()))
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect();
    if names.is_empty() {
        return None;
    }
    if names.len() > 8 {
        Some(format!("{}, et al.", names[..8].join(", ")))
    } else {
        Some(names.join(", "))
    }
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}

fn abstract_field(metadata: &json::Json) -> Option<String> {
    let raw = metadata
        .get("description")
        .and_then(|d| d.as_str())
        .filter(|s| !s.is_empty())?;
    let collapsed = strip_tags(raw)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if collapsed.is_empty() {
        return None;
    }
    if collapsed.chars().count() > 1200 {
        let capped: String = collapsed.chars().take(1200).collect();
        Some(format!("{}…", capped))
    } else {
        Some(collapsed)
    }
}

fn parse_zenodo(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(hits) = v
        .get("hits")
        .and_then(|h| h.get("hits"))
        .and_then(|h| h.as_arr())
    else {
        return out;
    };
    for hit in hits {
        let Some(doi) = hit
            .get("doi")
            .and_then(|d| d.as_str())
            .filter(|d| !d.is_empty())
        else {
            continue;
        };
        let mut line = match first_file(hit) {
            Some(f) => format!("url {}\tdoi: {}", f, doi),
            None => format!("doi: {}", doi),
        };
        if let Some(title) = hit
            .get("title")
            .and_then(|t| t.as_str())
            .filter(|t| !t.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(date) = hit
            .get("metadata")
            .and_then(|m| m.get("publication_date"))
            .and_then(|d| d.as_str())
            .filter(|d| !d.is_empty())
        {
            line.push_str(&format!("\tdate: {}", date));
        }
        if let Some(license) = hit
            .get("metadata")
            .and_then(|m| m.get("license"))
            .and_then(|l| l.get("id"))
            .and_then(|l| l.as_str())
            .filter(|l| !l.is_empty())
        {
            line.push_str(&format!("\tlicense: {}", license));
        }
        if let Some(metadata) = hit.get("metadata") {
            if let Some(authors) = authors_field(metadata) {
                line.push_str(&format!("\tauthors: {}", authors));
            }
            if let Some(abstract_) = abstract_field(metadata) {
                line.push_str(&format!("\tabstract: {}", abstract_));
            }
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_doi_title_date_license_and_file() {
        let body = r#"{"hits":{"hits":[{"doi":"10.5281/zenodo.1","title":"A dataset","metadata":{"publication_date":"2023-03-07","license":{"id":"cc-by-4.0"}},"files":[{"links":{"self":"https://zenodo.org/api/records/1/files/x/content"}}]}]}}"#;
        assert_eq!(
            parse_zenodo(body),
            vec!["url https://zenodo.org/api/records/1/files/x/content\tdoi: 10.5281/zenodo.1\ttitle: A dataset\tdate: 2023-03-07\tlicense: cc-by-4.0".to_string()]
        );
    }

    #[test]
    fn keeps_the_doi_without_file_or_license() {
        let body = r#"{"hits":{"hits":[{"doi":"10.5281/zenodo.2","title":"B","metadata":{"publication_date":"2020-01-01"}}]}}"#;
        assert_eq!(
            parse_zenodo(body),
            vec!["doi: 10.5281/zenodo.2\ttitle: B\tdate: 2020-01-01".to_string()]
        );
    }

    #[test]
    fn skips_the_hit_without_doi() {
        let body =
            r#"{"hits":{"hits":[{"title":"no doi"},{"doi":"10.5281/zenodo.3","title":"C"}]}}"#;
        assert_eq!(
            parse_zenodo(body),
            vec!["doi: 10.5281/zenodo.3\ttitle: C".to_string()]
        );
    }

    #[test]
    fn empty_hits_carry_nothing() {
        assert!(parse_zenodo(r#"{"hits":{"hits":[]}}"#).is_empty());
    }

    #[test]
    fn reads_the_full_record_with_authors_and_abstract() {
        let body = r#"{"hits":{"hits":[{"doi":"10.5281/zenodo.1","title":"A dataset","metadata":{"publication_date":"2023-03-07","license":{"id":"cc-by-4.0"},"creators":[{"name":"Lovelace, Ada"},{"name":"Turing, Alan"}],"description":"<p>A <b>measured</b> field</p>"},"files":[{"links":{"self":"https://zenodo.org/api/records/1/files/x/content"}}]}]}}"#;
        assert_eq!(
            parse_zenodo(body),
            vec!["url https://zenodo.org/api/records/1/files/x/content\tdoi: 10.5281/zenodo.1\ttitle: A dataset\tdate: 2023-03-07\tlicense: cc-by-4.0\tauthors: Lovelace, Ada, Turing, Alan\tabstract: A measured field".to_string()]
        );
    }

    #[test]
    fn omits_the_absent_authors_and_abstract() {
        let body = r#"{"hits":{"hits":[{"doi":"10.5281/zenodo.2","title":"B","metadata":{"publication_date":"2020-01-01"}}]}}"#;
        assert_eq!(
            parse_zenodo(body),
            vec!["doi: 10.5281/zenodo.2\ttitle: B\tdate: 2020-01-01".to_string()]
        );
    }

    #[test]
    fn caps_the_authors_to_eight_with_et_al() {
        let names = (1..=9)
            .map(|n| format!(r#"{{"name":"Author {n}"}}"#))
            .collect::<Vec<_>>()
            .join(",");
        let body = format!(
            r#"{{"hits":{{"hits":[{{"doi":"10.5281/zenodo.9","metadata":{{"creators":[{names}]}}}}]}}}}"#
        );
        assert_eq!(
            parse_zenodo(&body),
            vec!["doi: 10.5281/zenodo.9\tauthors: Author 1, Author 2, Author 3, Author 4, Author 5, Author 6, Author 7, Author 8, et al.".to_string()]
        );
    }

    #[test]
    fn strips_html_tags_and_collapses_whitespace_in_the_abstract() {
        let body = r#"{"hits":{"hits":[{"doi":"10.5281/zenodo.4","metadata":{"description":"<p>Hello   <em>world</em>\n</p>"}}]}}"#;
        assert_eq!(
            parse_zenodo(body),
            vec!["doi: 10.5281/zenodo.4\tabstract: Hello world".to_string()]
        );
    }
}
