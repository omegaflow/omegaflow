use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://ssd-api.jpl.nasa.gov/sbdb.api";

fn element(elements: &json::Json, name: &str) -> Option<String> {
    elements
        .as_arr()?
        .iter()
        .find(|e| e.get("name").and_then(|n| n.as_str()) == Some(name))
        .and_then(|e| e.get("value"))
        .and_then(|v| v.as_scalar_string())
}

fn parse_sbdb(v: &json::Json, query: &str, max: usize) -> Vec<String> {
    if max == 0 {
        return Vec::new();
    }
    let Some(object) = v.get("object") else {
        return Vec::new();
    };
    let Some(fullname) = object.get("fullname").and_then(|n| n.as_str()) else {
        return Vec::new();
    };
    let mut line = format!(
        "url https://ssd.jpl.nasa.gov/tools/sbdb_lookup.html#/?sstr={}\tdes: {}",
        urlencode(query),
        fullname
    );
    if let Some(elements) = v.get("orbit").and_then(|o| o.get("elements")) {
        for key in ["a", "e", "i"] {
            if let Some(value) = element(elements, key) {
                line.push_str(&format!("\t{}: {}", key, value));
            }
        }
    }
    vec![line]
}

pub fn horizons_lines(query: &str, max: usize) -> Vec<String> {
    let needle = query.trim();
    if needle.is_empty() {
        return vec![format!("absent — horizons carries no object: {}", needle)];
    }
    let url = format!("{}?sstr={}&phys-par=1", ENDPOINT, urlencode(needle));
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the horizons response carries no JSON".to_string()];
            };
            let out = parse_sbdb(&v, needle, max);
            if out.is_empty() {
                vec![format!("absent — horizons carries no object: {}", needle)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — horizons HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{"object":{"neo":true,"fullname":"433 Eros (A898 PA)","orbit_class":{"code":"AMO","name":"Amor"},"kind":"an","spkid":"20000433","prefix":null,"shortname":"433 Eros","pha":false,"orbit_id":"659","des":"433"},"orbit":{"elements":[{"sigma":"9.4e-09","title":"eccentricity","value":"0.223","name":"e","label":"e","units":null},{"units":"au","name":"a","label":"a","value":"1.46","title":"semi-major axis","sigma":"1.5e-10"},{"value":"10.8","title":"inclination; angle with respect to x-y ecliptic plane","sigma":"1.2e-06","units":"deg","name":"i","label":"i"},{"label":"node","name":"om","units":"deg","sigma":"3.6e-06","value":"304","title":"longitude of the ascending node"}]},"signature":{"source":"NASA/JPL Small-Body Database (SBDB) API","version":"1.3"}}"#;

    #[test]
    fn reads_object_into_url_line() {
        let v = json::parse(SAMPLE).unwrap();
        assert_eq!(
            parse_sbdb(&v, "433", 10),
            vec![
                "url https://ssd.jpl.nasa.gov/tools/sbdb_lookup.html#/?sstr=433\tdes: 433 Eros (A898 PA)\ta: 1.46\te: 0.223\ti: 10.8"
                    .to_string()
            ]
        );
    }

    #[test]
    fn an_unknown_object_carries_no_line() {
        let body = r#"{"moreInfo":"https://ssd-api.jpl.nasa.gov/doc/sbdb.html","message":"specified object was not found","code":"200"}"#;
        let v = json::parse(body).unwrap();
        assert!(parse_sbdb(&v, "xyzzy", 10).is_empty());
    }

    #[test]
    fn a_missing_element_is_omitted_and_max_zero_is_empty() {
        let body = r#"{"object":{"fullname":"433 Eros"},"orbit":{"elements":[{"name":"a","value":"1.46"}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_sbdb(&v, "433", 10),
            vec![
                "url https://ssd.jpl.nasa.gov/tools/sbdb_lookup.html#/?sstr=433\tdes: 433 Eros\ta: 1.46"
                    .to_string()
            ]
        );
        assert!(parse_sbdb(&v, "433", 0).is_empty());
    }
}
