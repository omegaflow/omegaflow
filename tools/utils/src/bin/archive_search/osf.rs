use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.osf.io/v2/preprints/";
const FALLBACK_URL: &str = "https://osf.io/preprints/";

pub fn osf_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?filter[title]={}&page[size]={}&embed=contributors",
        ENDPOINT,
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the osf response carries no JSON".to_string()];
            };
            let out = parse_osf(&v);
            if out.is_empty() {
                vec![format!("absent — osf carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — osf HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
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

fn abstract_field(attrs: &json::Json) -> Option<String> {
    let raw = attrs
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
    if collapsed.chars().count() > 800 {
        let capped: String = collapsed.chars().take(800).collect();
        Some(format!("{}…", capped))
    } else {
        Some(collapsed)
    }
}

fn full_name_of(user: &json::Json) -> Option<&str> {
    user.get("attributes")
        .and_then(|a| a.get("full_name"))
        .and_then(|n| n.as_str())
        .filter(|s| !s.is_empty())
}

fn authors_field(item: &json::Json) -> Option<String> {
    let contributors = item
        .get("embeds")
        .and_then(|e| e.get("contributors"))
        .and_then(|c| c.get("data"))
        .and_then(|d| d.as_arr())?;
    let mut names: Vec<&str> = Vec::new();
    for contributor in contributors {
        let Some(users) = contributor
            .get("embeds")
            .and_then(|e| e.get("users"))
            .and_then(|u| u.get("data"))
        else {
            continue;
        };
        if let Some(arr) = users.as_arr() {
            names.extend(arr.iter().filter_map(full_name_of));
        } else if let Some(name) = full_name_of(users) {
            names.push(name);
        }
    }
    if names.is_empty() {
        return None;
    }
    let cap = 8usize;
    let mut joined = names
        .iter()
        .take(cap)
        .copied()
        .collect::<Vec<_>>()
        .join(", ");
    if names.len() > cap {
        joined.push_str(", et al.");
    }
    Some(joined)
}

fn parse_osf(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("data").and_then(|d| d.as_arr()) else {
        return out;
    };
    for item in items {
        let Some(attrs) = item.get("attributes") else {
            continue;
        };
        let url = match item
            .get("links")
            .and_then(|l| l.get("html"))
            .and_then(|h| h.as_str())
            .filter(|s| !s.is_empty())
        {
            Some(u) => u,
            None => FALLBACK_URL,
        };
        let mut line = format!("url {}", url);
        if let Some(title) = attrs
            .get("title")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(authors) = authors_field(item) {
            line.push_str(&format!("\tauthors: {}", authors));
        }
        if let Some(doi) = attrs
            .get("doi")
            .and_then(|d| d.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        if let Some(date) = attrs
            .get("date_published")
            .and_then(|d| d.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tdate: {}", date));
        }
        if let Some(abstract_) = abstract_field(attrs) {
            line.push_str(&format!("\tabstract: {}", abstract_));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(body: &str) -> Vec<String> {
        parse_osf(&json::parse(body).expect("test body is JSON"))
    }

    #[test]
    fn reads_the_full_item() {
        let body = r#"{"data":[{"attributes":{"title":"A measured field","date_published":"2023-01-01","doi":"10.31234/osf.io/abc","description":"<p>A short   abstract.</p>","is_published":true},"links":{"html":"https://osf.io/preprints/psyarxiv/abc","preprint_doi":"10.31234/osf.io/abc"}}]}"#;
        assert_eq!(
            parse(body),
            vec!["url https://osf.io/preprints/psyarxiv/abc\ttitle: A measured field\tdoi: 10.31234/osf.io/abc\tdate: 2023-01-01\tabstract: A short abstract.".to_string()]
        );
    }

    #[test]
    fn omits_absent_doi_and_description() {
        let body = r#"{"data":[{"attributes":{"title":"No extras"},"links":{"html":"https://osf.io/preprints/x"}}]}"#;
        assert_eq!(
            parse(body),
            vec!["url https://osf.io/preprints/x\ttitle: No extras".to_string()]
        );
    }

    #[test]
    fn empty_data_carries_nothing() {
        assert!(parse(r#"{"data":[]}"#).is_empty());
    }

    #[test]
    fn a_missing_link_reads_the_fallback() {
        let body = r#"{"data":[{"attributes":{"title":"No link"}}]}"#;
        assert_eq!(
            parse(body),
            vec!["url https://osf.io/preprints/\ttitle: No link".to_string()]
        );
    }

    #[test]
    fn reads_authors_from_the_contributor_embed() {
        let body = r#"{"data":[{"attributes":{"title":"Authored"},"links":{"html":"https://osf.io/preprints/psyarxiv/y"},"embeds":{"contributors":{"data":[{"embeds":{"users":{"data":{"attributes":{"full_name":"Ada Lovelace"}}}}},{"embeds":{"users":{"data":{"attributes":{"full_name":"Alan Turing"}}}}}]}}}]}"#;
        assert_eq!(
            parse(body),
            vec!["url https://osf.io/preprints/psyarxiv/y\ttitle: Authored\tauthors: Ada Lovelace, Alan Turing".to_string()]
        );
    }

    #[test]
    fn reads_authors_when_the_user_embed_is_an_array() {
        let body = r#"{"data":[{"attributes":{"title":"Grouped"},"embeds":{"contributors":{"data":[{"embeds":{"users":{"data":[{"attributes":{"full_name":"Grace Hopper"}}]}}}]}}}]}"#;
        assert_eq!(
            parse(body),
            vec![
                "url https://osf.io/preprints/\ttitle: Grouped\tauthors: Grace Hopper".to_string()
            ]
        );
    }

    #[test]
    fn caps_authors_at_eight_with_et_al() {
        let users: Vec<String> = (1..=9)
            .map(|i| {
                format!(
                    r#"{{"embeds":{{"users":{{"data":{{"attributes":{{"full_name":"Author {i}"}}}}}}}}}}"#
                )
            })
            .collect();
        let body = format!(
            r#"{{"data":[{{"attributes":{{"title":"Many"}},"embeds":{{"contributors":{{"data":[{}]}}}}}}]}}"#,
            users.join(",")
        );
        let out = parse(&body);
        assert_eq!(
            out,
            vec!["url https://osf.io/preprints/\ttitle: Many\tauthors: Author 1, Author 2, Author 3, Author 4, Author 5, Author 6, Author 7, Author 8, et al.".to_string()]
        );
    }
}
