use crate::json;
use crate::net::{get, json_escape, urlencode};

const RESOLVER: &str = "https://mast.stsci.edu/api/v0/invoke";

pub fn mast_lines(query: &str, max: usize) -> Vec<String> {
    let url = resolver_url(query.trim());
    match get(&url, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the mast response carries no JSON".to_string()];
            };
            let out = parse_mast(&v, max);
            if out.is_empty() {
                vec![format!("absent — mast carries no result: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — mast HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn resolver_url(target: &str) -> String {
    let request = format!(
        "{{\"service\":\"Mast.Name.Lookup\",\"params\":{{\"input\":\"{}\",\"format\":\"json\"}}}}",
        json_escape(target)
    );
    format!("{}?request={}", RESOLVER, urlencode(&request))
}

fn numeric_or_text(row: &json::Json, key: &str) -> Option<String> {
    match row.get(key) {
        Some(v) => v.as_scalar_string(),
        None => None,
    }
}

fn mast_line(row: &json::Json) -> Option<String> {
    let canonical = row
        .get("canonicalName")
        .and_then(|n| n.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())?;
    let mut line = format!("url {}\tid: {}", resolver_url(canonical), canonical);
    if let Some(searched) = row.get("searchString").and_then(|s| s.as_str()) {
        let searched = searched.trim();
        if !searched.is_empty() {
            line.push_str(&format!("\tname: {}", searched));
        }
    }
    if let Some(ra) = numeric_or_text(row, "ra") {
        line.push_str(&format!("\tra: {}", ra));
    }
    if let Some(decl) = numeric_or_text(row, "decl") {
        line.push_str(&format!("\tdec: {}", decl));
    }
    if let Some(kind) = row.get("objectType").and_then(|o| o.as_str()) {
        let kind = kind.trim();
        if !kind.is_empty() {
            line.push_str(&format!("\ttype: {}", kind));
        }
    }
    Some(line)
}

fn parse_mast(v: &json::Json, max: usize) -> Vec<String> {
    let Some(rows) = v.get("resolvedCoordinate").and_then(|r| r.as_arr()) else {
        return Vec::new();
    };
    rows.iter().take(max).filter_map(mast_line).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const M31: &str = r#"{"resolvedCoordinate":[{"searchString":"m31","resolver":"SIMBAD","cached":true,"resolverTime":113,"cacheDate":"Jul 24, 2026, 12:31:45 PM","searchRadius":-1.0,"canonicalName":"M  31","ra":10.684708,"decl":41.26875,"radius":1.66275,"majorAxis":3.3255,"minorAxis":1.1798333333333335,"positionAngle":35.0,"objectType":"AGN"}],"status":""}"#;

    #[test]
    fn resolves_a_name_record() {
        let v = json::parse(M31).unwrap();
        let out = parse_mast(&v, 10);
        assert_eq!(out.len(), 1);
        let parts: Vec<&str> = out[0].split('\t').collect();
        assert!(parts[0].starts_with("url https://mast.stsci.edu/api/v0/invoke?request="));
        assert_eq!(
            &parts[1..],
            &[
                "id: M  31",
                "name: m31",
                "ra: 10.684708",
                "dec: 41.26875",
                "type: AGN"
            ]
        );
    }

    #[test]
    fn an_empty_resolution_carries_no_line() {
        let v = json::parse(r#"{"resolvedCoordinate":[],"status":""}"#).unwrap();
        assert!(parse_mast(&v, 10).is_empty());
    }

    #[test]
    fn caps_at_max_and_omits_absent_fields() {
        let v = json::parse(
            r#"{"resolvedCoordinate":[{"searchString":"a","canonicalName":"A"},{"searchString":"b","canonicalName":"B"}]}"#,
        )
        .unwrap();
        let out = parse_mast(&v, 1);
        assert_eq!(out.len(), 1);
        let parts: Vec<&str> = out[0].split('\t').collect();
        assert_eq!(&parts[1..], &["id: A", "name: a"]);
    }
}
