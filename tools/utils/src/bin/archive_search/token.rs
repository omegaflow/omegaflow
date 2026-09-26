use std::collections::HashMap;
use std::sync::OnceLock;

static SECRETS: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn set_secrets(env: HashMap<String, String>) {
    let _ = SECRETS.set(env);
}

pub(crate) fn secret(name: &str) -> Option<String> {
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

pub fn earthdata_token() -> Option<String> {
    secret("EARTHDATA_EDL_TOKEN")
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
}
