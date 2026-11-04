use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.ror.org/organizations";
const UA: &str = "User-Agent: omegaflow-archive_search/1.0 (https://omegaflow.space)";

pub fn ror_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?query={}", ENDPOINT, urlencode(query));
    match get(&url, &["-H", UA], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the ror response carries no JSON".to_string()];
            };
            let out = parse_ror(&v);
            if out.is_empty() {
                vec![format!("absent — ror carries no organisation: {}", query)]
            } else {
                out.into_iter().take(max).collect()
            }
        }
        Some(f) => vec![format!("pending — ror HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_ror(v: &json::Json) -> Vec<String> {
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
        let mut line = format!("url {}", ror_url(id));
        if let Some(name) = pick_name(item.get("names")) {
            line.push_str(&format!("\tname: {}", name));
        }
        if let Some(country) = pick_country(item.get("locations")) {
            line.push_str(&format!("\tcountry: {}", country));
        }
        out.push(line);
    }
    out
}

fn ror_url(id: &str) -> String {
    if id.starts_with("http://") || id.starts_with("https://") {
        id.to_string()
    } else {
        format!("https://ror.org/{}", id)
    }
}

fn pick_name(names: Option<&json::Json>) -> Option<String> {
    let list = names.and_then(|n| n.as_arr())?;
    for token in ["ror_display", "label"] {
        for entry in list {
            let carries = entry
                .get("types")
                .and_then(|t| t.as_arr())
                .map(|t| t.iter().any(|x| x.as_str() == Some(token)))
                .is_some_and(|b| b);
            if !carries {
                continue;
            }
            if let Some(value) = entry
                .get("value")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
            {
                return Some(value.to_string());
            }
        }
    }
    for entry in list {
        if let Some(value) = entry
            .get("value")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            return Some(value.to_string());
        }
    }
    None
}

fn pick_country(locations: Option<&json::Json>) -> Option<String> {
    locations
        .and_then(|l| l.as_arr())
        .and_then(|l| l.first())
        .and_then(|l| l.get("geonames_details"))
        .and_then(|g| g.get("country_name"))
        .and_then(|c| c.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"{"number_of_results":147,"time_taken":6,"items":[{"id":"https://ror.org/05tc5g775","names":[{"lang":null,"types":["acronym"],"value":"MPF"},{"lang":"en","types":["label"],"value":"Max Planck Foundation"},{"lang":"de","types":["ror_display","label"],"value":"Max-Planck-Förderstiftung"}],"locations":[{"geonames_details":{"country_code":"DE","country_name":"Germany"},"geonames_id":2867714}]},{"id":"https://ror.org/00d0v0049","names":[{"lang":"en","types":["ror_display","label"],"value":"Max Planck Innovation"}],"locations":[{"geonames_details":{"country_code":"DE","country_name":"Germany"},"geonames_id":2867714}]}]}"#;

    #[test]
    fn reads_ror_display_name_and_country() {
        let v = json::parse(FIXTURE).unwrap();
        let out = parse_ror(&v);
        assert_eq!(out.len(), 2);
        assert_eq!(
            out[0],
            "url https://ror.org/05tc5g775\tname: Max-Planck-Förderstiftung\tcountry: Germany"
                .to_string()
        );
        assert_eq!(
            out[1],
            "url https://ror.org/00d0v0049\tname: Max Planck Innovation\tcountry: Germany"
                .to_string()
        );
    }

    #[test]
    fn falls_back_to_label_and_bare_id() {
        let fixture = r#"{"items":[{"id":"abc123","names":[{"lang":"en","types":["label"],"value":"Fallback Org"}],"locations":[]}]}"#;
        let v = json::parse(fixture).unwrap();
        let out = parse_ror(&v);
        assert_eq!(
            out[0],
            "url https://ror.org/abc123\tname: Fallback Org".to_string()
        );
    }

    #[test]
    fn skips_items_without_id() {
        let v = json::parse(r#"{"items":[{"names":[{"types":["ror_display"],"value":"X"}]}]}"#)
            .unwrap();
        assert!(parse_ror(&v).is_empty());
    }

    #[test]
    fn no_items_key_carries_nothing() {
        let v = json::parse(r#"{"number_of_results":0}"#).unwrap();
        assert!(parse_ror(&v).is_empty());
    }
}
