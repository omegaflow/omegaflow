use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://nominatim.openstreetmap.org/search";
const UA: &str = "User-Agent: omegaflow-archive_search/1.0 (https://omegaflow.space)";
const PLACE: &str = "https://www.openstreetmap.org";

pub fn nominatim_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?q={}&format=jsonv2&limit={}",
        ENDPOINT,
        urlencode(query),
        max
    );
    match get(&url, &["-H", UA], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the nominatim response carries no JSON".to_string()];
            };
            let out = parse_nominatim(&v);
            if out.is_empty() {
                vec![format!("absent — nominatim carries no place: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — nominatim HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_nominatim(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.as_arr() else {
        return out;
    };
    for item in items {
        let Some(typ) = item
            .get("osm_type")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let Some(id) = item.get("osm_id").and_then(|i| i.as_scalar_string()) else {
            continue;
        };
        let id = id.trim();
        if id.is_empty() {
            continue;
        }
        let mut line = format!("url {}/{}/{}", PLACE, typ, id);
        if let Some(name) = item
            .get("display_name")
            .and_then(|n| n.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tname: {}", name));
        }
        if let Some(lat) = item
            .get("lat")
            .and_then(|l| l.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tlat: {}", lat));
        }
        if let Some(lon) = item
            .get("lon")
            .and_then(|l| l.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tlon: {}", lon));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"[{"place_id":144564737,"osm_type":"relation","osm_id":62369,"lat":"52.4009309","lon":"13.0591397","type":"administrative","display_name":"Potsdam, Brandenburg, Deutschland"},{"place_id":366939620,"osm_type":"relation","osm_id":175967,"lat":"44.6697996","lon":"-74.9813349","type":"administrative","display_name":"Village of Potsdam, Town of Potsdam, Saint Lawrence County, New York, 13699, United States"}]"#;

    #[test]
    fn reads_places() {
        let v = json::parse(FIXTURE).unwrap();
        let out = parse_nominatim(&v);
        assert_eq!(out.len(), 2);
        assert_eq!(
            out[0],
            "url https://www.openstreetmap.org/relation/62369\tname: Potsdam, Brandenburg, Deutschland\tlat: 52.4009309\tlon: 13.0591397"
                .to_string()
        );
        assert_eq!(
            out[1],
            "url https://www.openstreetmap.org/relation/175967\tname: Village of Potsdam, Town of Potsdam, Saint Lawrence County, New York, 13699, United States\tlat: 44.6697996\tlon: -74.9813349"
                .to_string()
        );
    }

    #[test]
    fn skips_places_without_osm_identity() {
        let v = json::parse(r#"[{"display_name":"Nowhere","lat":"1.0","lon":"2.0"}]"#).unwrap();
        assert!(parse_nominatim(&v).is_empty());
    }

    #[test]
    fn empty_array_carries_nothing() {
        let v = json::parse("[]").unwrap();
        assert!(parse_nominatim(&v).is_empty());
    }
}
