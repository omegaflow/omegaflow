use crate::json;
use crate::net::{post, urlencode};

const ENDPOINT: &str = "http://overpass-api.de/api/interpreter";
const PLACE: &str = "https://www.openstreetmap.org";
const FORM: &str = "Content-Type: application/x-www-form-urlencoded";

pub fn overpass_lines(query: &str, max: usize) -> Vec<String> {
    let ql = format!(
        "[out:json][timeout:25];nwr[\"name\"~\"{}\",i];out center {};",
        regex_escape(query),
        max
    );
    let body = format!("data={}", urlencode(&ql));
    match post(ENDPOINT, &body, &[FORM], "60") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the overpass response carries no JSON".to_string()];
            };
            let mut out = parse_overpass(&v);
            out.truncate(max);
            if out.is_empty() {
                vec![format!("absent — overpass carries no element: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — overpass HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn regex_escape(q: &str) -> String {
    let mut out = String::with_capacity(q.len());
    for c in q.chars() {
        match c {
            '\\' | '"' | '.' | '^' | '$' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}'
            | '|' => {
                out.push('\\');
                out.push(c);
            }
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn parse_overpass(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(elements) = v.get("elements").and_then(|e| e.as_arr()) else {
        return out;
    };
    for el in elements {
        let Some(typ) = el
            .get("type")
            .and_then(|t| t.as_str())
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let Some(id) = el.get("id").and_then(|i| i.as_scalar_string()) else {
            continue;
        };
        let id = id.trim();
        if id.is_empty() {
            continue;
        }
        let mut line = format!("url {}/{}/{}", PLACE, typ, id);
        if let Some(name) = el
            .get("tags")
            .and_then(|t| t.get("name"))
            .and_then(|n| n.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tname: {}", name));
        }
        line.push_str(&format!("\ttype: {}", typ));
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"{"version":0.6,"generator":"Overpass API 0.7.62.11","elements":[
{"type":"node","id":989065359,"lat":48.8603413,"lon":2.2958124,"tags":{"highway":"bus_stop","name":"Tour Eiffel","public_transport":"platform"}},
{"type":"node","id":1168595158,"lat":48.8584294,"lon":2.2957348,"tags":{"amenity":"toilets","name":"Toilette Tour Eiffel"}},
{"type":"way","id":5013364,"center":{"lat":48.8582603,"lon":2.2945008}}
]}"#;

    #[test]
    fn reads_elements() {
        let v = json::parse(FIXTURE).unwrap();
        let out = parse_overpass(&v);
        assert_eq!(out.len(), 3);
        assert_eq!(
            out[0],
            "url https://www.openstreetmap.org/node/989065359\tname: Tour Eiffel\ttype: node"
                .to_string()
        );
        assert_eq!(
            out[1],
            "url https://www.openstreetmap.org/node/1168595158\tname: Toilette Tour Eiffel\ttype: node"
                .to_string()
        );
        assert_eq!(
            out[2],
            "url https://www.openstreetmap.org/way/5013364\ttype: way".to_string()
        );
    }

    #[test]
    fn skips_elements_without_identity() {
        let v = json::parse(
            r#"{"elements":[{"tags":{"name":"Nowhere"}},{"type":"node"},{"type":"node","id":1,"tags":{"name":"Kept"}}]}"#,
        )
        .unwrap();
        let out = parse_overpass(&v);
        assert_eq!(
            out,
            vec!["url https://www.openstreetmap.org/node/1\tname: Kept\ttype: node".to_string()]
        );
    }

    #[test]
    fn empty_elements_carries_nothing() {
        let v = json::parse(r#"{"version":0.6,"elements":[]}"#).unwrap();
        assert!(parse_overpass(&v).is_empty());
        let bare = json::parse(r#"{"version":0.6}"#).unwrap();
        assert!(parse_overpass(&bare).is_empty());
    }
}
