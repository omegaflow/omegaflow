use crate::json;
use crate::net::{get, urlencode};

pub fn nasa_images_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://images-api.nasa.gov/search?q={}&page_size={}",
        urlencode(query),
        max
    );
    match get(&url, &[], "30") {
        Some(f) if f.status == Some(200) => {
            let lines = parse_nasa_images(&f.body);
            if lines.is_empty() {
                vec![format!("absent — NASA Images carries no item: {}", query)]
            } else {
                lines
            }
        }
        Some(f) => vec![format!("pending — nasa-images HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
}

fn parse_nasa_images(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(items) = v
        .get("collection")
        .and_then(|c| c.get("items"))
        .and_then(|i| i.as_arr())
    else {
        return out;
    };
    for item in items {
        let Some(data) = item.get("data").and_then(|d| d.as_arr()) else {
            continue;
        };
        let Some(meta) = data.first() else {
            continue;
        };
        let nasa_id = field(meta, "nasa_id");
        let url = nasa_id
            .as_deref()
            .map(|id| format!("https://images.nasa.gov/details/{id}"))
            .or_else(|| field(item, "href"));
        let Some(url) = url else {
            continue;
        };
        let mut line = format!("url {url}");
        if let Some(title) = field(meta, "title") {
            line.push_str(&format!("\ttitle: {title}"));
        }
        if let Some(description) = field(meta, "description") {
            line.push_str(&format!("\tdescription: {description}"));
        }
        if let Some(date) = field(meta, "date_created") {
            line.push_str(&format!("\tdate: {}", &date[..date.len().min(10)]));
        }
        if let Some(center) = field(meta, "center") {
            line.push_str(&format!("\tcenter: {center}"));
        }
        if let Some(media) = field(meta, "media_type") {
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
    fn reads_the_item_metadata() {
        let body = r#"{"collection":{"items":[{"href":"https://images-api.nasa.gov/search?q=solar","data":[{"nasa_id":"PIA12345","title":"Solar Wind","description":"An artist view.","date_created":"2026-01-02T00:00:00Z","center":"JPL","media_type":"image"}],"links":[{"href":"https://images-assets.nasa.gov/image/PIA12345/PIA12345.jpg"}]}]}}"#;
        assert_eq!(
            parse_nasa_images(body),
            vec!["url https://images.nasa.gov/details/PIA12345\ttitle: Solar Wind\tdescription: An artist view.\tdate: 2026-01-02\tcenter: JPL\tmedia: image".to_string()]
        );
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_nasa_images(r#"{"collection":{"items":[]}}"#).is_empty());
    }
}
