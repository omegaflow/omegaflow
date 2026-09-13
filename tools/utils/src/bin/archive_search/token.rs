use crate::json;
use std::process::Command;

pub fn is_unauthorized(status: Option<i32>) -> bool {
    status == Some(401)
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
    let user = std::env::var("EARTHDATA_USER").ok()?;
    let pass = std::env::var("EARTHDATA_PASS").ok()?;
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
    fn unauthorized_is_only_the_401() {
        assert!(!is_unauthorized(None));
        assert!(is_unauthorized(Some(401)));
        assert!(!is_unauthorized(Some(403)));
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
