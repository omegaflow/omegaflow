use crate::json;
use crate::net::{get, urlencode};

pub fn artic_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://api.artic.edu/api/v1/artworks/search?q={}&limit={}&fields=id,title,artist_display,date_display,department_title",
        urlencode(query),
        max
    );
    match get(&url, &[], "30") {
        Some(f) if f.status == Some(200) => {
            let lines = parse_artic(&f.body);
            if lines.is_empty() {
                vec![format!("absent — Art Institute carries no work: {}", query)]
            } else {
                lines
            }
        }
        Some(f) => vec![format!("pending — artic HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
}

fn parse_artic(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(works) = v.get("data").and_then(|d| d.as_arr()) else {
        return out;
    };
    for work in works {
        let Some(id) = work.get("id").and_then(|i| i.as_scalar_string()) else {
            continue;
        };
        let Some(title) = field(work, "title") else {
            continue;
        };
        let mut line = format!("url https://www.artic.edu/artworks/{id}\ttitle: {title}");
        if let Some(artist) = field(work, "artist_display") {
            let first = artist.lines().next().unwrap_or("").trim();
            if !first.is_empty() {
                line.push_str(&format!("\tartist: {first}"));
            }
        }
        if let Some(date) = field(work, "date_display") {
            line.push_str(&format!("\tdate: {date}"));
        }
        if let Some(department) = field(work, "department_title") {
            line.push_str(&format!("\tdepartment: {department}"));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_work_fields() {
        let body = r#"{"data":[{"id":12345,"title":"The Starry Night","artist_display":"Vincent van Gogh (Dutch, 1853-1890)\nDutch","date_display":"1889","department_title":"Painting and Sculpture of Europe"}]}"#;
        assert_eq!(
            parse_artic(body),
            vec!["url https://www.artic.edu/artworks/12345\ttitle: The Starry Night\tartist: Vincent van Gogh (Dutch, 1853-1890)\tdate: 1889\tdepartment: Painting and Sculpture of Europe".to_string()]
        );
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_artic(r#"{"data":[]}"#).is_empty());
    }
}
