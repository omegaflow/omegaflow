use super::*;

pub const SUPERDARN_ASCII_POST_URL: &str = "https://superdarn.ca/ascii-call";

pub const SUPERDARN_ASCII_GET_BASE: &str = "https://sdc-serv.usask.ca";

pub const SUPERDARN_ASCII_CONTENT_TYPE: &str = "application/json";

pub fn fetch_token_form(
    post_url: &str,
    post_body: &str,
    headers: &[(String, String)],
    get_base: &str,
) -> Option<String> {
    let token_response = fetch_raw(post_url, Some(post_body), headers)?;
    let token_path = extract_token_path(&token_response)?;
    let get_url = join_token_path(get_base, &token_path)?;
    fetch_raw(&get_url, None, &[])
}

pub fn fetch_superdarn_ascii(post_body: &str) -> Option<String> {
    let headers = vec![(
        "Content-Type".to_string(),
        SUPERDARN_ASCII_CONTENT_TYPE.to_string(),
    )];
    fetch_token_form(
        SUPERDARN_ASCII_POST_URL,
        post_body,
        &headers,
        SUPERDARN_ASCII_GET_BASE,
    )
}

pub fn extract_token_path(response: &str) -> Option<String> {
    let trimmed = response.trim();
    if trimmed.is_empty() || !trimmed.starts_with('/') {
        return None;
    }
    Some(trimmed.to_string())
}

pub fn join_token_path(get_base: &str, token_path: &str) -> Option<String> {
    let base = get_base.trim_end_matches('/');
    if base.is_empty() || token_path.is_empty() {
        return None;
    }
    Some(format!("{}{}", base, token_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_token_path_reads_the_bare_path_body() {
        assert_eq!(
            extract_token_path(
                "/../website_updating_tools/ascii_files/20260905T000000-20260905T060000_sas-bmall_K9RQD8VVCG.txt"
            )
            .as_deref(),
            Some(
                "/../website_updating_tools/ascii_files/20260905T000000-20260905T060000_sas-bmall_K9RQD8VVCG.txt"
            )
        );
        assert_eq!(
            extract_token_path("  /../website_updating_tools/x.txt  ").as_deref(),
            Some("/../website_updating_tools/x.txt")
        );
        assert!(extract_token_path("").is_none());
        assert!(extract_token_path("   ").is_none());
        assert!(extract_token_path("not-a-path").is_none());
    }

    #[test]
    fn join_token_path_prefixes_the_download_base() {
        assert_eq!(
            join_token_path(
                "https://sdc-serv.usask.ca",
                "/../website_updating_tools/x.txt"
            )
            .as_deref(),
            Some("https://sdc-serv.usask.ca/../website_updating_tools/x.txt")
        );
        assert_eq!(
            join_token_path(
                "https://sdc-serv.usask.ca/",
                "/../website_updating_tools/x.txt"
            )
            .as_deref(),
            Some("https://sdc-serv.usask.ca/../website_updating_tools/x.txt")
        );
        assert!(join_token_path("", "/x").is_none());
        assert!(join_token_path("https://sdc-serv.usask.ca", "").is_none());
    }
}
