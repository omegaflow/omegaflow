use crate::json;
use crate::net::{get, urlencode};

pub fn stac_lines(query: &str, max: usize) -> Vec<String> {
    let mut endpoint = "";
    let mut collection = "";
    let mut bbox = "";
    let mut datetime = "";
    for tok in query.split_whitespace() {
        if let Some(v) = tok.strip_prefix("endpoint=") {
            endpoint = v;
        } else if let Some(v) = tok.strip_prefix("collection=") {
            collection = v;
        } else if let Some(v) = tok.strip_prefix("bbox=") {
            bbox = v;
        } else if let Some(v) = tok.strip_prefix("datetime=") {
            datetime = v;
        }
    }
    if endpoint.is_empty() {
        return vec![
            "usage — archive_search --stac \"endpoint=<url> [collection=<id>] [bbox=w,s,e,n] [datetime=a/b]\""
                .to_string(),
        ];
    }
    let base = endpoint.trim_end_matches('/');
    let mut url = format!("{}/search?limit={}", base, max.min(50));
    if !collection.is_empty() {
        url.push_str(&format!("&collections={}", urlencode(collection)));
    }
    if !bbox.is_empty() {
        url.push_str(&format!("&bbox={}", urlencode(bbox)));
    }
    if !datetime.is_empty() {
        url.push_str(&format!("&datetime={}", urlencode(&stac_datetime(datetime))));
    }
    match get(&url, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the stac response carries no JSON".to_string()];
            };
            let out = parse_stac(&v, base, max);
            if out.is_empty() {
                vec!["absent — the stac endpoint carries no item".to_string()]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — stac HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn stac_datetime(value: &str) -> String {
    value
        .split('/')
        .map(|part| {
            let p = part.trim();
            if p.len() == 10 && p.as_bytes().get(4) == Some(&b'-') {
                format!("{p}T00:00:00Z")
            } else {
                p.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn str_at(v: &json::Json, key: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string()
}

fn self_href(f: &json::Json) -> Option<String> {
    for l in f.get("links").and_then(|l| l.as_arr())? {
        if l.get("rel").and_then(|r| r.as_str()) == Some("self") {
            if let Some(h) = l
                .get("href")
                .and_then(|h| h.as_str())
                .filter(|h| !h.is_empty())
            {
                return Some(h.to_string());
            }
        }
    }
    None
}

fn parse_stac(v: &json::Json, base: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("features").and_then(|f| f.as_arr()) else {
        return out;
    };
    for item in items {
        let id = str_at(item, "id");
        let collection = str_at(item, "collection");
        let datetime = item
            .get("properties")
            .map(|p| str_at(p, "datetime"))
            .filter(|d| !d.is_empty());
        let href = match self_href(item) {
            Some(h) => h,
            None => {
                if collection.is_empty() {
                    format!("{base}/search")
                } else {
                    format!("{base}/collections/{collection}/items/{id}")
                }
            }
        };
        let mut line = format!("url {href}");
        if !collection.is_empty() {
            line.push_str(&format!("\tcollection: {collection}"));
        }
        if !id.is_empty() {
            line.push_str(&format!("\tid: {id}"));
        }
        if let Some(dt) = datetime.as_deref() {
            line.push_str(&format!("\tdatetime: {dt}"));
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
    fn reads_features_with_self_link() {
        let body = r#"{"type":"FeatureCollection","features":[{"id":"S2A","collection":"s2","properties":{"datetime":"2026-01-01T00:00:00Z"},"links":[{"rel":"self","href":"https://x/items/S2A"}]}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_stac(&v, "https://x", 10),
            vec![
                "url https://x/items/S2A\tcollection: s2\tid: S2A\tdatetime: 2026-01-01T00:00:00Z"
                    .to_string()
            ]
        );
    }

    #[test]
    fn builds_item_url_without_self_link() {
        let body = r#"{"type":"FeatureCollection","features":[{"id":"A","collection":"c","properties":{"datetime":"t"}}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_stac(&v, "https://x", 10),
            vec![
                "url https://x/collections/c/items/A\tcollection: c\tid: A\tdatetime: t"
                    .to_string()
            ]
        );
    }

    #[test]
    fn date_only_bounds_become_rfc3339() {
        assert_eq!(
            stac_datetime("2026-10-01/2026-10-02"),
            "2026-10-01T00:00:00Z/2026-10-02T00:00:00Z"
        );
        assert_eq!(
            stac_datetime("2026-10-01T12:00:00Z/.."),
            "2026-10-01T12:00:00Z/.."
        );
    }

    #[test]
    fn caps_at_max() {
        let v = json::parse(r#"{"features":[{"id":"a"},{"id":"b"}]}"#).unwrap();
        assert_eq!(parse_stac(&v, "https://x", 1).len(), 1);
    }
}
