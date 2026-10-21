use crate::json;
use crate::net::{get, urlencode};

pub fn openverse_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://api.openverse.org/v1/images/?q={}&page_size={}",
        urlencode(query),
        max
    );
    match get(&url, &[], "30") {
        Some(f) if f.status == Some(200) => {
            let lines = parse_openverse(&f.body);
            if lines.is_empty() {
                vec![format!("absent — Openverse carries no image: {}", query)]
            } else {
                lines
            }
        }
        Some(f) => vec![format!("pending — openverse HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
}

fn parse_openverse(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for item in results {
        let url = field(item, "foreign_landing_url").or_else(|| field(item, "url"));
        let Some(url) = url else {
            continue;
        };
        let mut line = format!("url {url}");
        if let Some(title) = field(item, "title") {
            line.push_str(&format!("\ttitle: {title}"));
        }
        if let Some(creator) = field(item, "creator") {
            line.push_str(&format!("\tcreator: {creator}"));
        }
        if let Some(license) = field(item, "license") {
            let label = match field(item, "license_version") {
                Some(version) => format!("{license} {version}"),
                None => license,
            };
            line.push_str(&format!("\tlicense: {label}"));
        }
        if let Some(source) = field(item, "source") {
            line.push_str(&format!("\tsource: {source}"));
        }
        if let Some(media) = field(item, "url") {
            line.push_str(&format!("\tmedia: {media}"));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_image_fields() {
        let body = r#"{"result_count":240,"results":[{"id":"8e67","title":"Solar wind","creator":"NASA","license":"by","license_version":"4.0","source":"flickr","url":"https://live.staticflickr.com/1.jpg","foreign_landing_url":"https://www.flickr.com/photos/1"}]}"#;
        assert_eq!(
            parse_openverse(body),
            vec!["url https://www.flickr.com/photos/1\ttitle: Solar wind\tcreator: NASA\tlicense: by 4.0\tsource: flickr\tmedia: https://live.staticflickr.com/1.jpg".to_string()]
        );
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_openverse(r#"{"results":[]}"#).is_empty());
    }
}
