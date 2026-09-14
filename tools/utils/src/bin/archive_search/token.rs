use crate::json;
use std::collections::HashMap;
use std::process::Command;
use std::sync::OnceLock;

static SECRETS: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn set_secrets(env: HashMap<String, String>) {
    let _ = SECRETS.set(env);
}

fn secret(name: &str) -> Option<String> {
    if let Some(map) = SECRETS.get() {
        if let Some(value) = map.get(name) {
            if !value.is_empty() {
                return Some(value.clone());
            }
        }
    }
    std::env::var(name).ok().filter(|v| !v.is_empty())
}

pub fn is_unauthorized(status: Option<i32>) -> bool {
    matches!(status, Some(401) | Some(403) | Some(307))
}

pub fn is_earthdata_host(host: &str) -> bool {
    host == "urs.earthdata.nasa.gov" || host.ends_with(".earthdata.nasa.gov")
}

fn parse_token(body: &str) -> Option<String> {
    json::parse(body)?
        .get("access_token")?
        .as_str()
        .filter(|token| !token.is_empty())
        .map(str::to_string)
}

pub fn earthdata_token() -> Option<String> {
    if let Some(token) = secret("EARTHDATA_EDL_TOKEN") {
        return Some(token);
    }
    let user = secret("EARTHDATA_USER")?;
    let pass = secret("EARTHDATA_PASS")?;
    let credentials = format!("{}:{}", user, pass);
    let out = Command::new("curl")
        .args([
            "-sL",
            "--max-time",
            "30",
            "-u",
            &credentials,
            "https://urs.earthdata.nasa.gov/api/users/token",
        ])
        .output()
        .ok()?;
    parse_token(&String::from_utf8_lossy(&out.stdout))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthorized_reads_the_earthdata_codes() {
        assert!(!is_unauthorized(None));
        assert!(is_unauthorized(Some(401)));
        assert!(is_unauthorized(Some(403)));
        assert!(is_unauthorized(Some(307)));
        assert!(!is_unauthorized(Some(200)));
    }

    #[test]
    fn earthdata_host_reads_the_domain_and_subdomains() {
        assert!(is_earthdata_host("urs.earthdata.nasa.gov"));
        assert!(is_earthdata_host("data.gesdisc.earthdata.nasa.gov"));
        assert!(!is_earthdata_host("example.com"));
        assert!(!is_earthdata_host("notearthdata.nasa.gov"));
    }

    #[test]
    fn parse_token_reads_the_access_token() {
        assert_eq!(
            parse_token(r#"{"access_token":"abc"}"#),
            Some("abc".to_string())
        );
        assert_eq!(parse_token("{}"), None);
        assert_eq!(parse_token(r#"{"access_token":""}"#), None);
        assert_eq!(parse_token("not json"), None);
    }
}
