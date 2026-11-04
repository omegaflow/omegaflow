use crate::json;
use crate::net::get;

const ENDPOINT: &str = "https://cds.climate.copernicus.eu/api/catalogue/v1/collections";
const BASE: &str = "https://cds.climate.copernicus.eu/datasets/";

pub fn copernicus_lines(query: &str, max: usize) -> Vec<String> {
    match get(ENDPOINT, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the copernicus response carries no JSON".to_string()];
            };
            let out = parse_copernicus(&v, query, max);
            if out.is_empty() {
                vec![format!(
                    "absent — copernicus carries no collection: {}",
                    query
                )]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — copernicus HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_copernicus(v: &json::Json, query: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("collections").and_then(|c| c.as_arr()) else {
        return out;
    };
    let needle = query.trim().to_lowercase();
    for item in items {
        let Some(id) = item
            .get("id")
            .and_then(|i| i.as_str())
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let title = match item.get("title").and_then(|t| t.as_str()) {
            Some(t) => t,
            None => "",
        };
        if !needle.is_empty()
            && !id.to_lowercase().contains(&needle)
            && !title.to_lowercase().contains(&needle)
        {
            continue;
        }
        let mut line = format!("url {}{}", BASE, id);
        if !title.is_empty() {
            line.push_str(&format!("\ttitle: {}", collapse(title)));
        }
        if let Some(description) = item
            .get("description")
            .and_then(|d| d.as_str())
            .filter(|s| !s.is_empty())
        {
            let c = collapse(description);
            let capped: String = c.chars().take(300).collect();
            line.push_str(&format!("\tdescription: {}", capped));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_id_or_title() {
        let body = r#"{"collections":[{"id":"era5-land","title":"ERA5-Land"},{"id":"cams-ghg","title":"Greenhouse gas"}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_copernicus(&v, "era5", 10),
            vec![
                "url https://cds.climate.copernicus.eu/datasets/era5-land\ttitle: ERA5-Land"
                    .to_string()
            ]
        );
    }

    #[test]
    fn caps_at_max() {
        let body = r#"{"collections":[{"id":"a","title":"A"},{"id":"b","title":"B"}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(parse_copernicus(&v, "", 1).len(), 1);
    }
}
