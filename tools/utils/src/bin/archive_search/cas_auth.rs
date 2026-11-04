use crate::net::{get, urlencode};
use crate::secrets::resolve_secret;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const NSSDC_ORIGIN: &str = "https://umt.nssdc.ac.cn";
const NSSDC_LOGIN: &str =
    "https://umt.nssdc.ac.cn/login?service=https://www.nssdc.ac.cn:443/logindo";
const SSDC_ORIGIN: &str = "https://tools.ssdc.asi.it";

const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64(input: &[u8]) -> String {
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let mut buf = [0u8; 3];
        buf[..chunk.len()].copy_from_slice(chunk);
        let b0 = buf[0] as u32;
        let b1 = buf[1] as u32;
        let b2 = buf[2] as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64[((n >> 18) & 63) as usize] as char);
        out.push(B64[((n >> 12) & 63) as usize] as char);
        if chunk.len() > 1 {
            out.push(B64[((n >> 6) & 63) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(B64[(n & 63) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn attr_value(tag: &str, attr: &str) -> Option<String> {
    let needle = format!("{attr}=\"");
    let start = tag.find(&needle)? + needle.len();
    let rest = &tag[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn hidden_value(body: &str, name: &str) -> Option<String> {
    let mut rest = body;
    while let Some(i) = rest.find("<input") {
        let after = &rest[i..];
        let end = after.find('>')?;
        let tag = &after[..end];
        if attr_value(tag, "name").as_deref() == Some(name) {
            return attr_value(tag, "value");
        }
        rest = &after[end..];
    }
    None
}

fn form_action(body: &str) -> Option<String> {
    let i = body.find("<form")?;
    let after = &body[i..];
    let end = after.find('>')?;
    attr_value(&after[..end], "action")
}

fn absolute(origin: &str, target: &str) -> String {
    if target.starts_with("http://") || target.starts_with("https://") {
        target.to_string()
    } else {
        format!("{}/{}", origin, target.trim_start_matches('/'))
    }
}

fn ticket_present(headers: &str) -> bool {
    headers.contains("ticket=ST-")
}

fn page_marker(body: &str) -> &'static str {
    if body.contains("ticket=ST-") || body.contains("CURRENTCASUSER") {
        "authenticated (js redirect)"
    } else if body.contains("name=\"execution\"") || body.contains("name=\"lt\"") {
        "login form still present"
    } else if body.to_ascii_lowercase().contains("logout") || body.contains("退出") {
        "authenticated (logout marker)"
    } else {
        "no login form"
    }
}

const PAGE_MARKERS: &[&str] = &["logout", "登录", "验证", "error", "账号"];

fn marker_report(body: &str) -> String {
    let lower = body.to_ascii_lowercase();
    PAGE_MARKERS
        .iter()
        .map(|marker| {
            format!(
                "{marker}={}",
                if lower.contains(&marker.to_ascii_lowercase()) {
                    "yes"
                } else {
                    "no"
                }
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn page_title(body: &str) -> Option<String> {
    let start = body.find("<title")?;
    let open_end = body[start..].find('>')? + start + 1;
    let end = body[open_end..].find("</title>")? + open_end;
    let title = body[open_end..end].trim();
    if title.is_empty() {
        None
    } else {
        Some(title.chars().take(160).collect())
    }
}

fn header_statuses(headers: &str) -> Vec<String> {
    headers
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            if !parts.next()?.starts_with("HTTP/") {
                return None;
            }
            parts.next().map(str::to_string)
        })
        .collect()
}

fn header_values(headers: &str, name: &str) -> Vec<String> {
    headers
        .lines()
        .filter_map(|line| {
            let (key, value) = line.trim().split_once(':')?;
            if key.eq_ignore_ascii_case(name) {
                Some(value.trim().to_string())
            } else {
                None
            }
        })
        .collect()
}

fn redact_credentials(url: &str) -> String {
    let mut out = url.to_string();
    for key in ["ticket", "jsessionid"] {
        let needle = format!("{key}=");
        let mut search = 0;
        while let Some(pos) = out[search..].find(&needle) {
            let start = search + pos + needle.len();
            let end = match out[start..].find(|c| c == '&' || c == '#' || c == ' ' || c == '"') {
                Some(i) => start + i,
                None => out.len(),
            };
            out.replace_range(start..end, "<redacted>");
            search = start + "<redacted>".len();
        }
    }
    out
}

fn escape_js(input: &str) -> String {
    let mut out = String::new();
    for unit in input.encode_utf16() {
        let u = unit as u32;
        let bare = (0x30..=0x39).contains(&u)
            || (0x41..=0x5A).contains(&u)
            || (0x61..=0x7A).contains(&u)
            || matches!(u, 0x40 | 0x2A | 0x5F | 0x2B | 0x2D | 0x2E | 0x2F);
        if bare {
            if let Some(c) = char::from_u32(u) {
                out.push(c);
            }
        } else if u < 256 {
            out.push_str(&format!("%{u:02X}"));
        } else {
            out.push_str(&format!("%u{u:04X}"));
        }
    }
    out
}

fn nssdc_password(pass: &str) -> String {
    base64(escape_js(pass).as_bytes())
}

fn nssdc_form(user: &str, pass: &str, lt: &str, execution: &str) -> String {
    format!(
        "username={}&password={}&lt={}&execution={}&_eventId=submit&loginFlag=cas&phoneFlag=&lang=zh",
        urlencode(user),
        urlencode(&nssdc_password(pass)),
        urlencode(lt),
        urlencode(execution)
    )
}

fn ssdc_form(user: &str, pass: &str, lt: &str) -> String {
    format!(
        "username={}&password={}&lt={}&warn=true",
        urlencode(user),
        urlencode(pass),
        urlencode(lt)
    )
}

fn temp_file(tag: &str) -> PathBuf {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    std::env::temp_dir().join(format!("archive_search-{tag}-{}-{n}", std::process::id()))
}

fn session_args(jar: &Path, headers: &Path) -> Vec<String> {
    let jar = jar.to_string_lossy().to_string();
    let headers = headers.to_string_lossy().to_string();
    vec![
        "--cookie-jar".to_string(),
        jar.clone(),
        "--cookie".to_string(),
        jar,
        "--dump-header".to_string(),
        headers,
    ]
}

fn fetch(url: &str, extra: &[String]) -> Option<crate::net::Fetch> {
    let refs: Vec<&str> = extra.iter().map(String::as_str).collect();
    get(url, &refs, "40")
}

fn cleanup(paths: &[&Path]) {
    for path in paths {
        let _ = std::fs::remove_file(path);
    }
}

fn push_login_report(
    lines: &mut Vec<String>,
    headers: &Path,
    post_url: &str,
    f: &crate::net::Fetch,
) {
    lines.push(format!("login POST HTTP {}", f.status_text()));
    let header_text = match std::fs::read_to_string(headers) {
        Ok(text) => text,
        Err(_) => String::new(),
    };
    if let Some(status) = header_statuses(&header_text).first() {
        lines.push(format!("post status {status}"));
    }
    let locations = header_values(&header_text, "Location");
    match locations.first() {
        Some(location) => lines.push(format!("post location {}", redact_credentials(location))),
        None => lines.push("post location absent".to_string()),
    }
    let final_url = match locations.last() {
        Some(url) => url.as_str(),
        None => post_url,
    };
    lines.push(format!("final url {}", redact_credentials(final_url)));
    let ticket = ticket_present(&header_text) || ticket_present(&f.body);
    lines.push(format!(
        "ticket {}",
        if ticket { "present" } else { "absent" }
    ));
    lines.push(format!("page {}", page_marker(&f.body)));
    lines.push(format!("page markers {}", marker_report(&f.body)));
    if let Some(title) = page_title(&f.body) {
        lines.push(format!("page title {title}"));
    }
}

pub fn nssdc_auth_lines(env: &HashMap<String, String>) -> Vec<String> {
    let user = resolve_secret("{NSSDC_USER}", env);
    if user.is_empty() {
        return vec!["pending — NSSDC_USER unresolved".to_string()];
    }
    let pass = resolve_secret("{NSSDC_PASS}", env);
    if pass.is_empty() {
        return vec!["pending — NSSDC_PASS unresolved".to_string()];
    }
    let jar = temp_file("nssdc-jar");
    let headers = temp_file("nssdc-headers");
    let common = session_args(&jar, &headers);
    let mut lines = vec!["nssdc-auth — CAS login".to_string()];
    let Some(page) = fetch(NSSDC_LOGIN, &common) else {
        cleanup(&[&jar, &headers]);
        return vec!["pending — no network".to_string()];
    };
    lines.push(format!("login GET HTTP {}", page.status_text()));
    let (Some(lt), Some(execution)) = (
        hidden_value(&page.body, "lt"),
        hidden_value(&page.body, "execution"),
    ) else {
        lines.push("pending — the login form carries no lt/execution".to_string());
        cleanup(&[&jar, &headers]);
        return lines;
    };
    let post_url = match form_action(&page.body) {
        Some(action) if !action.is_empty() => absolute(NSSDC_ORIGIN, &action),
        _ => NSSDC_LOGIN.to_string(),
    };
    let mut extra = common.clone();
    extra.push("-X".to_string());
    extra.push("POST".to_string());
    extra.push("-H".to_string());
    extra.push("Content-Type: application/x-www-form-urlencoded".to_string());
    extra.push("--data".to_string());
    extra.push(nssdc_form(&user, &pass, &lt, &execution));
    match fetch(&post_url, &extra) {
        Some(f) => push_login_report(&mut lines, &headers, &post_url, &f),
        None => lines.push("pending — no network".to_string()),
    }
    cleanup(&[&jar, &headers]);
    lines
}

pub fn ssdc_auth_lines(service: &str, env: &HashMap<String, String>) -> Vec<String> {
    let service = service.trim();
    if service.is_empty() {
        return vec!["usage — archive_search --ssdc-auth <service-url>".to_string()];
    }
    let user = resolve_secret("{SSDC_USER}", env);
    if user.is_empty() {
        return vec!["pending — SSDC_USER unresolved".to_string()];
    }
    let pass = resolve_secret("{SSDC_PASS}", env);
    if pass.is_empty() {
        return vec!["pending — SSDC_PASS unresolved".to_string()];
    }
    let login = format!("{SSDC_ORIGIN}/cas/login?service={}", urlencode(service));
    let jar = temp_file("ssdc-jar");
    let headers = temp_file("ssdc-headers");
    let common = session_args(&jar, &headers);
    let mut lines = vec!["ssdc-auth — CAS login".to_string()];
    let Some(page) = fetch(&login, &common) else {
        cleanup(&[&jar, &headers]);
        return vec!["pending — no network".to_string()];
    };
    lines.push(format!("login GET HTTP {}", page.status_text()));
    let Some(lt) = hidden_value(&page.body, "lt") else {
        lines.push("pending — the login form carries no lt".to_string());
        cleanup(&[&jar, &headers]);
        return lines;
    };
    let mut extra = common.clone();
    extra.push("-X".to_string());
    extra.push("POST".to_string());
    extra.push("-H".to_string());
    extra.push("Content-Type: application/x-www-form-urlencoded".to_string());
    extra.push("--data".to_string());
    extra.push(ssdc_form(&user, &pass, &lt));
    match fetch(&login, &extra) {
        Some(f) => push_login_report(&mut lines, &headers, &login, &f),
        None => lines.push("pending — no network".to_string()),
    }
    cleanup(&[&jar, &headers]);
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_matches_rfc4648() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foob"), "Zm9vYg==");
        assert_eq!(base64(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn hidden_fields_are_read_from_the_form() {
        let body = "<form action=\"login;jsessionid=AB12?service=x\">\
<input type=\"hidden\" name=\"lt\" value=\"LT-42-abc\" />\
<input name=\"execution\" value=\"e1s1\" type=\"hidden\"/></form>";
        assert_eq!(hidden_value(body, "lt"), Some("LT-42-abc".to_string()));
        assert_eq!(hidden_value(body, "execution"), Some("e1s1".to_string()));
        assert_eq!(hidden_value(body, "miss"), None);
        assert_eq!(
            form_action(body),
            Some("login;jsessionid=AB12?service=x".to_string())
        );
    }

    #[test]
    fn relative_action_resolves_against_the_origin() {
        assert_eq!(
            absolute("https://umt.nssdc.ac.cn", "login;jsessionid=1?service=x"),
            "https://umt.nssdc.ac.cn/login;jsessionid=1?service=x"
        );
        assert_eq!(
            absolute("https://umt.nssdc.ac.cn", "https://other/path"),
            "https://other/path"
        );
    }

    #[test]
    fn nssdc_form_carries_base64_password_never_plain() {
        let form = nssdc_form("alice", "s3cr3t", "LT-1", "e1s1");
        assert!(form.contains("username=alice"));
        assert!(form.contains("password=czNjcjN0"));
        assert!(form.contains("lt=LT-1"));
        assert!(form.contains("_eventId=submit"));
        assert!(!form.contains("s3cr3t"));
    }

    #[test]
    fn nssdc_password_escapes_before_base64() {
        assert_eq!(escape_js("a b"), "a%20b");
        assert_eq!(escape_js("a@b_c-1.2/3"), "a@b_c-1.2/3");
        assert_eq!(escape_js("p@ss!"), "p@ss%21");
        assert_eq!(escape_js("ä"), "%E4");
        assert_eq!(nssdc_password("a b"), base64(b"a%20b"));
        assert_ne!(nssdc_password("a b"), base64(b"a b"));
    }

    #[test]
    fn ssdc_form_carries_the_cas_fields() {
        let form = ssdc_form("bob", "pw", "LT-9");
        assert_eq!(form, "username=bob&password=pw&lt=LT-9&warn=true");
    }

    #[test]
    fn ticket_is_read_from_the_location_header() {
        let headers = "HTTP/1.1 302 Found\r\nLocation: https://tools.ssdc.asi.it/app?ticket=ST-1234-xyz\r\n\r\n";
        assert!(ticket_present(headers));
        assert!(!ticket_present("HTTP/1.1 200 OK\r\n\r\n"));
    }

    #[test]
    fn the_js_redirect_body_carries_the_ticket() {
        let body = "href = 'https://www.nssdc.ac.cn:443/logindo?CURRENTCASUSER=x&ticket=ST-4329-abc-cas01.example.org';\nwindow.location = href;";
        assert!(ticket_present(body));
        assert_eq!(page_marker(body), "authenticated (js redirect)");
    }

    #[test]
    fn a_missing_secret_flows_as_pending_without_network() {
        let empty = HashMap::new();
        assert_eq!(
            nssdc_auth_lines(&empty),
            vec!["pending — NSSDC_USER unresolved".to_string()]
        );
        assert_eq!(
            ssdc_auth_lines("https://x/s", &empty),
            vec!["pending — SSDC_USER unresolved".to_string()]
        );
        assert_eq!(
            ssdc_auth_lines("", &empty),
            vec!["usage — archive_search --ssdc-auth <service-url>".to_string()]
        );
    }

    #[test]
    fn the_page_marker_names_what_is_visible() {
        assert_eq!(
            page_marker("<input name=\"execution\" value=\"e\">"),
            "login form still present"
        );
        assert_eq!(
            page_marker("<a href=\"/logout\">Logout</a>"),
            "authenticated (logout marker)"
        );
        assert_eq!(page_marker("<p>welcome</p>"), "no login form");
    }

    #[test]
    fn headers_reveal_status_and_redacted_location() {
        let headers = "HTTP/1.1 302 Found\r\nLocation: https://www.nssdc.ac.cn/logindo?ticket=ST-99-abcdef\r\n\r\nHTTP/1.1 200 OK\r\n\r\n";
        assert_eq!(
            header_statuses(headers),
            vec!["302".to_string(), "200".to_string()]
        );
        let locations = header_values(headers, "Location");
        assert_eq!(
            redact_credentials(&locations[0]),
            "https://www.nssdc.ac.cn/logindo?ticket=<redacted>"
        );
    }

    #[test]
    fn redaction_leaves_a_clean_url_untouched() {
        assert_eq!(
            redact_credentials("https://tools.ssdc.asi.it/"),
            "https://tools.ssdc.asi.it/"
        );
    }

    #[test]
    fn marker_report_names_the_present_strings() {
        let report = marker_report("<html>验证码 <a href=\"/logout\">Logout</a></html>");
        assert!(report.contains("logout=yes"));
        assert!(report.contains("验证=yes"));
        assert!(report.contains("error=no"));
    }

    #[test]
    fn title_is_read_from_the_page() {
        assert_eq!(
            page_title("<html><title>统一身份认证</title></html>"),
            Some("统一身份认证".to_string())
        );
        assert_eq!(page_title("<html></html>"), None);
    }
}
