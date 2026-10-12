use crate::json;
use crate::net::{get, urlencode};

const SCIENCEBASE: &str = "https://www.sciencebase.gov/catalog/items";
const SCIENCEBASE_ITEM: &str = "https://www.sciencebase.gov/catalog/item/";
const QUAKE: &str = "https://earthquake.usgs.gov/fdsnws/event/1/query";
const QUAKE_EVENT: &str = "https://earthquake.usgs.gov/earthquakes/eventpage/";

pub fn usgs_lines(query: &str, max: usize) -> Vec<String> {
    let sciencebase_url = format!(
        "{}?q={}&format=json&max={}",
        SCIENCEBASE,
        urlencode(query),
        max
    );
    if let Some(f) = get(&sciencebase_url, &[], "20") {
        if f.status == Some(200) {
            if let Some(v) = json::parse(&f.body) {
                let out = parse_sciencebase(&v, max);
                if !out.is_empty() {
                    return out;
                }
            }
        }
    }
    quake_lines(query, max)
}

fn quake_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?format=geojson&orderby=time&limit={}", QUAKE, max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_usgs(&f.body, query, max);
            if out.is_empty() {
                vec![format!("absent — usgs carries no item: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — usgs HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_sciencebase(v: &json::Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("items").and_then(|i| i.as_arr()) else {
        return out;
    };
    for item in items {
        let Some(id) = item
            .get("id")
            .and_then(|i| i.as_str())
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let mut line = format!("url {}{}", SCIENCEBASE_ITEM, id);
        if let Some(title) = item
            .get("title")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", collapse(title)));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

fn parse_usgs(body: &str, query: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(features) = v.get("features").and_then(|f| f.as_arr()) else {
        return out;
    };
    let needle = query.trim().to_lowercase();
    for feature in features {
        let Some(props) = feature.get("properties") else {
            continue;
        };
        let place = props
            .get("place")
            .and_then(|p| p.as_str())
            .filter(|s| !s.is_empty());
        if !needle.is_empty() {
            let hit = place.map(|p| p.to_lowercase().contains(&needle)) == Some(true);
            if !hit {
                continue;
            }
        }
        let url = props
            .get("url")
            .and_then(|u| u.as_str())
            .filter(|u| !u.is_empty())
            .map(str::to_string)
            .or_else(|| {
                feature
                    .get("id")
                    .and_then(|i| i.as_str())
                    .filter(|i| !i.is_empty())
                    .map(|id| format!("{}{}", QUAKE_EVENT, id))
            });
        let Some(url) = url else {
            continue;
        };
        let mut line = format!("url {}", url);
        if let Some(place) = place {
            line.push_str(&format!("\tplace: {}", collapse(place)));
        }
        if let Some(mag) = props.get("mag").and_then(|m| m.as_scalar_string()) {
            line.push_str(&format!("\tmag: {}", mag));
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

    const QUAKES: &str = r#"{"type":"FeatureCollection","metadata":{"status":200,"api":"2.8.1"},"features":[{"type":"Feature","id":"us6000u1rh","properties":{"mag":5.4,"place":"Mid-Indian Ridge","url":"https://earthquake.usgs.gov/earthquakes/eventpage/us6000u1rh"}},{"type":"Feature","id":"us6000u1pl","properties":{"mag":5.5,"place":"122 km W of Neiafu, Tonga","url":"https://earthquake.usgs.gov/earthquakes/eventpage/us6000u1pl"}}]}"#;

    #[test]
    fn reads_url_place_and_mag() {
        assert_eq!(
            parse_usgs(QUAKES, "", 10),
            vec![
                "url https://earthquake.usgs.gov/earthquakes/eventpage/us6000u1rh\tplace: Mid-Indian Ridge\tmag: 5.4".to_string(),
                "url https://earthquake.usgs.gov/earthquakes/eventpage/us6000u1pl\tplace: 122 km W of Neiafu, Tonga\tmag: 5.5".to_string(),
            ]
        );
    }

    #[test]
    fn filters_by_place() {
        assert_eq!(
            parse_usgs(QUAKES, "tonga", 10),
            vec![
                "url https://earthquake.usgs.gov/earthquakes/eventpage/us6000u1pl\tplace: 122 km W of Neiafu, Tonga\tmag: 5.5".to_string(),
            ]
        );
    }

    #[test]
    fn caps_at_max() {
        assert_eq!(parse_usgs(QUAKES, "", 1).len(), 1);
    }
}
