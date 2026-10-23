use crate::json;
use crate::net::get;
use crate::token_cmd;

const PRODUCTS: &str = "https://catalogue.dataspace.copernicus.eu/odata/v1/Products";
const DOWNLOAD: &str = "https://download.dataspace.copernicus.eu/odata/v1/Products";

fn credentials() -> Option<(String, String)> {
    let user = std::env::var("CDSE_USER")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .or_else(|| token_cmd::cached("CDSE_USER"))?;
    let pass = std::env::var("CDSE_PASS")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .or_else(|| token_cmd::cached("CDSE_PASS"))?;
    Some((user, pass))
}

fn auth_args(creds: &Option<(String, String)>) -> Vec<String> {
    match creds {
        Some((user, pass)) => vec!["-u".to_string(), format!("{user}:{pass}")],
        None => Vec::new(),
    }
}

fn encode(text: &str) -> String {
    let mut out = String::new();
    for b in text.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn collapse(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn cdse_lines(query: &str, max: usize) -> Vec<String> {
    let creds = credentials();
    let args = auth_args(&creds);
    let extra: Vec<&str> = args.iter().map(String::as_str).collect();
    let top = max.clamp(1, 100);
    let needle = query.trim().replace('\'', "''");
    let mut url = format!("{PRODUCTS}?$top={top}");
    if !needle.is_empty() {
        url.push_str(&format!("&$filter=contains(Name,'{}')", encode(&needle)));
    }
    let mut fetched = get(&url, &[], "60");
    if matches!(fetched.as_ref().and_then(|f| f.status), Some(401) | Some(403)) && !extra.is_empty() {
        fetched = get(&url, &extra, "60");
    }
    match fetched {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the CDSE OData response carries no JSON".to_string()];
            };
            let out = parse_cdse(&v, max);
            if out.is_empty() {
                vec![format!("absent — CDSE carries no product: {query}")]
            } else {
                out
            }
        }
        Some(f) if f.status == Some(401) => vec![
            "pending — CDSE HTTP 401: credentials set? CDSE_USER/CDSE_PASS or OMEGAFLOW_CDSE_USER_CMD/OMEGAFLOW_CDSE_PASS_CMD".to_string(),
        ],
        Some(f) => vec![format!("pending — CDSE HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_cdse(v: &json::Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("value").and_then(|value| value.as_arr()) else {
        return out;
    };
    for item in items {
        let Some(id) = item.get("Id").and_then(|id| id.as_str()).filter(|s| !s.is_empty()) else {
            continue;
        };
        let mut line = format!("url {PRODUCTS}({id})");
        if let Some(name) = item.get("Name").and_then(|name| name.as_str()) {
            line.push_str(&format!("\tname: {}", collapse(name)));
        }
        if let Some(collection) = item
            .get("Collection")
            .and_then(|collection| collection.get("Name"))
            .and_then(|name| name.as_str())
        {
            line.push_str(&format!("\tcollection: {collection}"));
        }
        if let Some(size) = item
            .get("ContentLength")
            .and_then(|size| size.as_scalar_string())
        {
            line.push_str(&format!("\tsize: {size}"));
        }
        if let Some(online) = item.get("Online").and_then(|online| online.as_scalar_string()) {
            line.push_str(&format!("\tonline: {online}"));
        }
        line.push_str(&format!("\tdownload: {DOWNLOAD}({id})/$value"));
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
    fn auth_args_are_curl_basic_auth_and_absent_without_credentials() {
        assert_eq!(
            auth_args(&Some(("user".to_string(), "pass".to_string()))),
            vec!["-u".to_string(), "user:pass".to_string()]
        );
        assert!(auth_args(&None).is_empty());
    }

    #[test]
    fn products_parse_to_url_name_collection_size_and_download() {
        let body = r#"{"value":[{"Id":"abc","Name":"S5P O3","ContentLength":123,"Online":true,"Collection":{"Name":"SENTINEL-5P"}}]}"#;
        let v = json::parse(body).unwrap();
        let out = parse_cdse(&v, 10);
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("url https://catalogue.dataspace.copernicus.eu/odata/v1/Products(abc)"));
        assert!(out[0].contains("name: S5P O3"));
        assert!(out[0].contains("collection: SENTINEL-5P"));
        assert!(out[0].contains("size: 123"));
        assert!(out[0].contains("download: https://download.dataspace.copernicus.eu/odata/v1/Products(abc)/$value"));
    }

    #[test]
    fn caps_at_max() {
        let body = r#"{"value":[{"Id":"a"},{"Id":"b"}]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(parse_cdse(&v, 1).len(), 1);
    }
}
