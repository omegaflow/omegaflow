use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://opencitations.net/index/api/v1/citation-count";

pub fn opencitations_lines(query: &str, max: usize) -> Vec<String> {
    if max == 0 {
        return Vec::new();
    }
    let doi = normalize_doi(query);
    if !doi.starts_with("10.") {
        return vec![format!(
            "pending — opencitations indexes by DOI; give a DOI like 10.1038/nature12373: {}",
            query
        )];
    }
    let url = format!("{}/{}", ENDPOINT, urlencode(&doi));
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the opencitations response carries no JSON".to_string()];
            };
            let out = parse_opencitations(&v, &doi, &url);
            if out.is_empty() {
                vec![format!(
                    "absent — opencitations carries no record: {}",
                    query
                )]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — opencitations HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn normalize_doi(query: &str) -> String {
    query
        .trim()
        .trim_start_matches("https://doi.org/")
        .trim_start_matches("http://doi.org/")
        .trim_start_matches("https://dx.doi.org/")
        .trim_start_matches("http://dx.doi.org/")
        .trim_start_matches("doi:")
        .trim()
        .to_string()
}

fn parse_opencitations(v: &json::Json, doi: &str, url: &str) -> Vec<String> {
    let Some(rows) = v.as_arr() else {
        return Vec::new();
    };
    let Some(row) = rows.first() else {
        return Vec::new();
    };
    let Some(count) = row.get("count").and_then(|c| c.as_scalar_string()) else {
        return Vec::new();
    };
    vec![format!("url {}\tdoi: {}\tcitations: {}", url, doi, count)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_citation_count() {
        let v = json::parse(r#"[{"count":"1806"}]"#).unwrap();
        let url = "https://opencitations.net/index/api/v1/citation-count/10.1038/nature12373";
        assert_eq!(
            parse_opencitations(&v, "10.1038/nature12373", url),
            vec![format!(
                "url {}\tdoi: 10.1038/nature12373\tcitations: 1806",
                url
            )]
        );
    }

    #[test]
    fn keeps_a_zero_count() {
        let v = json::parse(r#"[{"count":"0"}]"#).unwrap();
        let url = "https://opencitations.net/index/api/v1/citation-count/10.9999/doesnotexist";
        assert_eq!(
            parse_opencitations(&v, "10.9999/doesnotexist", url),
            vec![format!(
                "url {}\tdoi: 10.9999/doesnotexist\tcitations: 0",
                url
            )]
        );
    }

    #[test]
    fn empty_or_shapeless_response_carries_nothing() {
        let empty = json::parse("[]").unwrap();
        assert!(parse_opencitations(&empty, "10.1/x", "u").is_empty());
        let shapeless = json::parse(r#"[{"foo":"bar"}]"#).unwrap();
        assert!(parse_opencitations(&shapeless, "10.1/x", "u").is_empty());
    }

    #[test]
    fn strips_the_doi_prefix() {
        assert_eq!(
            normalize_doi("https://doi.org/10.1038/nature12373"),
            "10.1038/nature12373"
        );
        assert_eq!(normalize_doi("doi:10.1/x"), "10.1/x");
    }
}
