use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://geocoding-api.open-meteo.com/v1/search";
const DOCS: &str = "https://open-meteo.com/en/docs/geocoding-api";

pub fn open_meteo_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?name={}&count={}", ENDPOINT, urlencode(query), max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the open-meteo response carries no JSON".to_string()];
            };
            let out = parse_open_meteo(&v);
            if out.is_empty() {
                vec![format!("absent — open-meteo carries no place: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — open-meteo HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_open_meteo(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for item in items {
        let Some(lat) = item.get("latitude").and_then(|x| x.as_scalar_string()) else {
            continue;
        };
        let Some(lon) = item.get("longitude").and_then(|x| x.as_scalar_string()) else {
            continue;
        };
        let url = format!("{}#latitude={}&longitude={}", DOCS, lat, lon);
        let mut line = format!("url {}", url);
        if let Some(name) = item
            .get("name")
            .and_then(|n| n.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tname: {}", name));
        }
        if let Some(country) = item
            .get("country")
            .and_then(|c| c.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tcountry: {}", country));
        }
        line.push_str(&format!("\tlat: {}", lat));
        line.push_str(&format!("\tlon: {}", lon));
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(body: &str) -> Vec<String> {
        parse_open_meteo(&json::parse(body).expect("test body is JSON"))
    }

    #[test]
    fn reads_a_place_with_coordinates() {
        let body = r#"{"results":[{"id":2950159,"name":"Berlin","latitude":52.52437,"longitude":13.41053,"elevation":74.0,"feature_code":"PPLC","country_code":"DE","timezone":"Europe/Berlin","population":3426354,"country":"Germany","admin1":"State of Berlin"}],"generationtime_ms":6.5449476}"#;
        assert_eq!(
            parse(body),
            vec!["url https://open-meteo.com/en/docs/geocoding-api#latitude=52.52437&longitude=13.41053\tname: Berlin\tcountry: Germany\tlat: 52.52437\tlon: 13.41053".to_string()]
        );
    }

    #[test]
    fn no_results_carries_nothing() {
        assert!(parse(r#"{"generationtime_ms":0.53703785}"#).is_empty());
    }

    #[test]
    fn skips_a_place_without_coordinates_and_keeps_a_nameless_one() {
        let body = r#"{"results":[{"name":"Nowhere","country":"Atlantis"},{"latitude":44.46867,"longitude":-71.18508}]}"#;
        assert_eq!(
            parse(body),
            vec!["url https://open-meteo.com/en/docs/geocoding-api#latitude=44.46867&longitude=-71.18508\tlat: 44.46867\tlon: -71.18508".to_string()]
        );
    }
}
