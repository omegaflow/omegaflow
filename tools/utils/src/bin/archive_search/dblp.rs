use crate::json;
use crate::net::{get, urlencode};

pub fn dblp_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://dblp.org/search/publ/api?q={}&format=json&h={}",
        urlencode(query),
        max
    );
    match get(&url, &[], "30") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(_) => {
                let lines = parse_dblp(&f.body);
                if lines.is_empty() {
                    vec![format!("absent — dblp carries no publication: {}", query)]
                } else {
                    lines
                }
            }
            None => vec!["pending — the dblp response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — dblp HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn author_text(author: &json::Json) -> Option<String> {
    if let Some(text) = author
        .get("text")
        .and_then(|t| t.as_str())
        .filter(|s| !s.is_empty())
    {
        return Some(text.to_string());
    }
    author
        .as_str()
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn authors_line(info: &json::Json) -> Option<String> {
    let author = info.get("authors")?.get("author")?;
    let mut names: Vec<String> = Vec::new();
    match author {
        json::Json::Arr(list) => {
            for entry in list {
                if let Some(text) = author_text(entry) {
                    names.push(text);
                }
            }
        }
        single => {
            if let Some(text) = author_text(single) {
                names.push(text);
            }
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

fn parse_dblp(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(hits) = v.get("result").and_then(|r| r.get("hits")) else {
        return out;
    };
    let Some(entries) = hits.get("hit").and_then(|h| h.as_arr()) else {
        return out;
    };
    for entry in entries {
        let Some(info) = entry.get("info") else {
            continue;
        };
        let Some(url) = field(info, "url") else {
            continue;
        };
        let mut line = format!("url {}", url);
        if let Some(title) = field(info, "title") {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(venue) = field(info, "venue") {
            line.push_str(&format!("\tvenue: {}", venue));
        }
        if let Some(year) = info.get("year").and_then(|y| y.as_scalar_string()) {
            line.push_str(&format!("\tyear: {}", year));
        }
        if let Some(doi) = field(info, "doi") {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        if let Some(authors) = authors_line(info) {
            line.push_str(&format!("\tauthors: {}", authors));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_publication_fields() {
        let body = r#"{"result":{"hits":{"@total":"1","@sent":"1","hit":[{"@score":"5","@id":"1","info":{"title":"A measured field.","venue":"J. Test","year":"2020","doi":"10.1234/abc","url":"https://dblp.org/rec/journals/test/abc","authors":{"author":[{"@pid":"1","text":"Ada Lovelace"},{"@pid":"2","text":"Alan Turing"}]}}}]}}}"#;
        assert_eq!(
            parse_dblp(body),
            vec!["url https://dblp.org/rec/journals/test/abc\ttitle: A measured field.\tvenue: J. Test\tyear: 2020\tdoi: 10.1234/abc\tauthors: Ada Lovelace, Alan Turing".to_string()]
        );
    }

    #[test]
    fn a_single_author_object_and_absent_fields_are_read() {
        let body = r#"{"result":{"hits":{"hit":[{"info":{"title":"Only a title.","url":"https://dblp.org/rec/conf/x/y","authors":{"author":{"text":"Grace Hopper"}}}}]}}}"#;
        assert_eq!(
            parse_dblp(body),
            vec![
                "url https://dblp.org/rec/conf/x/y\ttitle: Only a title.\tauthors: Grace Hopper"
                    .to_string()
            ]
        );
    }

    #[test]
    fn empty_hits_and_a_hit_without_url_carry_nothing() {
        assert!(parse_dblp(r#"{"result":{"hits":{"@total":"0","hit":[]}}}"#).is_empty());
        assert!(
            parse_dblp(r#"{"result":{"hits":{"hit":[{"info":{"title":"No url."}}]}}}"#).is_empty()
        );
    }
}
