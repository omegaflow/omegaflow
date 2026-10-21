use crate::json;
use crate::net::{get, urlencode};

pub fn gbooks_lines(query: &str, max: usize) -> Vec<String> {
    let key = crate::token::secret("GOOGLE_BOOKS_API_KEY")
        .or_else(|| crate::token::secret("GOOGLE_API_KEY"));
    let mut url = format!(
        "https://www.googleapis.com/books/v1/volumes?q={}&maxResults={}",
        urlencode(query),
        max.min(40)
    );
    if let Some(key) = &key {
        url.push_str("&key=");
        url.push_str(&urlencode(key));
    }
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let lines = parse_gbooks(&f.body);
            if lines.is_empty() {
                vec![format!("absent — Google Books carries no volume: {}", query)]
            } else {
                lines
            }
        }
        Some(f) if f.status == Some(429) || f.status == Some(403) => {
            let mut alt = crate::openlibrary::openlibrary_lines(query, max);
            if alt.iter().any(|l| l.starts_with("url ")) {
                for line in &mut alt {
                    if line.starts_with("url ") {
                        line.push_str(&format!(
                            "\tvia: openlibrary (gbooks {})",
                            f.status_text()
                        ));
                    }
                }
                alt
            } else {
                vec![format!(
                    "pending — gbooks HTTP {} (quota) and openlibrary carries nothing",
                    f.status_text()
                )]
            }
        }
        Some(f) => vec![format!("pending — gbooks HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn text(v: Option<&json::Json>) -> Option<String> {
    v.and_then(|x| x.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
}

fn authors(info: &json::Json) -> Option<String> {
    let list = info.get("authors")?.as_arr()?;
    let names: Vec<String> = list
        .iter()
        .filter_map(|a| a.as_str().filter(|s| !s.trim().is_empty()).map(str::to_string))
        .collect();
    if names.is_empty() {
        None
    } else {
        Some(names.join(", "))
    }
}

fn isbn(info: &json::Json) -> Option<String> {
    for id in info.get("industryIdentifiers")?.as_arr()? {
        if let Some(value) = text(id.get("identifier")) {
            return Some(value);
        }
    }
    None
}

fn parse_gbooks(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(items) = v.get("items").and_then(|i| i.as_arr()) else {
        return out;
    };
    for item in items {
        let Some(info) = item.get("volumeInfo") else {
            continue;
        };
        let url = text(info.get("infoLink")).or_else(|| {
            text(item.get("id")).map(|id| format!("https://books.google.com/books?id={id}"))
        });
        let Some(url) = url else {
            continue;
        };
        let mut line = format!("url {url}");
        if let Some(title) = text(info.get("title")) {
            line.push_str(&format!("\ttitle: {title}"));
        }
        if let Some(authors) = authors(info) {
            line.push_str(&format!("\tauthors: {authors}"));
        }
        if let Some(date) = text(info.get("publishedDate")) {
            line.push_str(&format!("\tyear: {}", &date[..date.len().min(10)]));
        }
        if let Some(publisher) = text(info.get("publisher")) {
            line.push_str(&format!("\tpublisher: {publisher}"));
        }
        if let Some(isbn) = isbn(info) {
            line.push_str(&format!("\tisbn: {isbn}"));
        }
        if let Some(pages) = info.get("pageCount").and_then(|p| p.as_scalar_string()) {
            line.push_str(&format!("\tpages: {pages}"));
        }
        if let Some(categories) = info.get("categories").and_then(|c| c.as_arr()) {
            let names: Vec<&str> = categories.iter().filter_map(|c| c.as_str()).collect();
            if !names.is_empty() {
                line.push_str(&format!("\tcategories: {}", names.join(", ")));
            }
        }
        if let Some(description) = text(info.get("description")) {
            line.push_str(&format!("\tdescription: {description}"));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_volume_fields() {
        let body = r#"{"totalItems":1,"items":[{"id":"abc","volumeInfo":{"title":"Solar Wind","authors":["A. Author","B. Author"],"publishedDate":"2020-05-01","publisher":"Example Press","industryIdentifiers":[{"type":"ISBN_13","identifier":"9781234567890"}],"pageCount":321,"categories":["Science"],"infoLink":"https://books.google.com/books?id=abc","description":"A book."}}]}"#;
        assert_eq!(
            parse_gbooks(body),
            vec!["url https://books.google.com/books?id=abc\ttitle: Solar Wind\tauthors: A. Author, B. Author\tyear: 2020-05-01\tpublisher: Example Press\tisbn: 9781234567890\tpages: 321\tcategories: Science\tdescription: A book.".to_string()]
        );
    }

    #[test]
    fn builds_the_id_url_without_info_link() {
        let body = r#"{"items":[{"id":"xyz","volumeInfo":{"title":"T"}}]}"#;
        assert_eq!(
            parse_gbooks(body),
            vec!["url https://books.google.com/books?id=xyz\ttitle: T".to_string()]
        );
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_gbooks(r#"{"items":[]}"#).is_empty());
    }
}
