use crate::json;
use crate::net::{get, urlencode};

pub fn geonames_lines(query: &str, max: usize) -> Vec<String> {
    let Some(user) = crate::token::secret("GEONAMES_USER") else {
        return vec![
            "pending — geonames needs GEONAMES_USER (free account at geonames.org/login)"
                .to_string(),
        ];
    };
    let url = format!(
        "http://api.geonames.org/searchJSON?q={}&maxRows={}&username={}",
        urlencode(query),
        max,
        urlencode(&user)
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_geonames(&f.body);
            if out.is_empty() {
                vec![format!("absent — geonames carries no place: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — geonames HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn scalar(v: &json::Json, key: &str) -> Option<String> {
    v.get(key).and_then(|x| x.as_scalar_string())
}

fn parse_geonames(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(items) = v.get("geonames").and_then(|g| g.as_arr()) else {
        return out;
    };
    for item in items {
        let Some(id) = scalar(item, "geonameId") else {
            continue;
        };
        let mut line = format!("url https://www.geonames.org/{}", id);
        if let Some(name) = field(item, "name") {
            line.push_str(&format!("\tname: {}", name));
        }
        if let Some(country) = field(item, "countryName") {
            line.push_str(&format!("\tcountry: {}", country));
        }
        if let Some(lat) = scalar(item, "lat") {
            line.push_str(&format!("\tlat: {}", lat));
        }
        if let Some(lon) = scalar(item, "lng") {
            line.push_str(&format!("\tlon: {}", lon));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_place_fields() {
        let body = r#"{"totalResultsCount":1,"geonames":[{"geonameId":2950159,"name":"Berlin","countryName":"Germany","lat":"52.52437","lng":"13.41053"}]}"#;
        assert_eq!(
            parse_geonames(body),
            vec![
                "url https://www.geonames.org/2950159\tname: Berlin\tcountry: Germany\tlat: 52.52437\tlon: 13.41053"
                    .to_string()
            ]
        );
    }

    #[test]
    fn skips_the_element_without_a_geonameid() {
        let body = r#"{"geonames":[{"name":"No id"},{"geonameId":1,"name":"Kept"}]}"#;
        assert_eq!(
            parse_geonames(body),
            vec!["url https://www.geonames.org/1\tname: Kept".to_string()]
        );
    }

    #[test]
    fn empty_geonames_carries_nothing() {
        assert!(parse_geonames(r#"{"geonames":[]}"#).is_empty());
    }
}
