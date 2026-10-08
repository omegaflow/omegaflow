use crate::json;
use crate::secrets::{Secret, resolve_key};
use std::collections::HashMap;
use std::process::Command;

const SEARCH_URL: &str = "https://product.gosat-gw.nies.go.jp/product_search/api/cui-search/";
const UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0 Safari/537.36";
const PRODUCT_DEFAULT: &str = "GWT3F_L1B";
const WINDOW_DAYS: i64 = 30;

pub struct GosatQuery {
    pub product: String,
    pub start: String,
    pub end: String,
}

pub fn gosat_lines(query: &str) -> Vec<String> {
    let Some((mail, pass)) = credentials() else {
        return vec![
            "pending — GOSAT_GW_MAIL/GOSAT_GW_PASS absent from .secrets.local".to_string(),
        ];
    };
    let Some(today) = epoch_days() else {
        return vec!["pending — the system clock reads no epoch".to_string()];
    };
    let parsed = parse_query(query, today);
    let url = search_url(&parsed);
    match search_fetch(&mail, &pass, &url) {
        Some((200, body)) => match json::parse(&body) {
            Some(v) => {
                let mut lines = product_lines(&v);
                if lines.is_empty() {
                    lines.push(format!(
                        "absent — GOSAT carries no {} for {}..{}",
                        parsed.product, parsed.start, parsed.end
                    ));
                }
                lines
            }
            None => vec!["pending — the GOSAT response carries no JSON".to_string()],
        },
        Some((code, _)) => vec![format!("pending — gosat HTTP {code}")],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn parse_query(query: &str, today: i64) -> GosatQuery {
    let mut words = query.split_whitespace();
    let product = words
        .next()
        .filter(|w| !w.is_empty())
        .unwrap_or(PRODUCT_DEFAULT)
        .to_string();
    match (words.next(), words.next()) {
        (Some(start), Some(end)) => GosatQuery {
            product,
            start: start.to_string(),
            end: end.to_string(),
        },
        _ => {
            let (start, end) = default_window(today);
            GosatQuery {
                product,
                start,
                end,
            }
        }
    }
}

pub fn search_url(q: &GosatQuery) -> String {
    let params: [(&str, &str); 8] = [
        ("level", "2"),
        ("mode", "2"),
        ("product", q.product.as_str()),
        ("format", "2"),
        ("version", "1"),
        ("area", "1"),
        ("start", q.start.as_str()),
        ("end", q.end.as_str()),
    ];
    let query = params
        .iter()
        .map(|(k, v)| format!("{}={}", uri_encode(k), uri_encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    format!("{SEARCH_URL}?{query}")
}

fn credentials() -> Option<(String, String)> {
    let env_map: HashMap<String, String> = match crate::find_repo_root() {
        Some(repo) => crate::secrets::load_env(&repo),
        None => std::env::vars().collect(),
    };
    let mail = resolve_key(
        env_map
            .get("GOSAT_GW_MAIL")
            .map(String::as_str)
            .unwrap_or(""),
        &env_map,
    );
    let pass = resolve_key(
        env_map
            .get("GOSAT_GW_PASS")
            .map(String::as_str)
            .unwrap_or(""),
        &env_map,
    );
    match (mail, pass) {
        (Secret::Value(m), Secret::Value(p)) => Some((m, p)),
        _ => None,
    }
}

fn uri_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn curl(args: &[&str]) -> Option<(i32, String)> {
    let out = Command::new("curl").args(args).output().ok()?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let cut = stdout.rfind('\n')?;
    let code: i32 = stdout[cut + 1..].trim().parse().ok()?;
    Some((code, stdout[..cut].to_string()))
}

fn search_fetch(mail: &str, pass: &str, url: &str) -> Option<(i32, String)> {
    let cookie = format!("mail={mail}; password={pass}");
    curl(&[
        "-sS",
        "-L",
        "-m",
        "60",
        "--connect-timeout",
        "15",
        "-A",
        UA,
        "-b",
        cookie.as_str(),
        "-H",
        "X-Requested-With: XMLHttpRequest",
        "-w",
        "\n%{http_code}",
        url,
    ])
}

fn product_lines(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(items) = v.get("result").and_then(|r| r.as_arr()) {
        for row in items {
            let Some(filename) = row
                .get("filename")
                .and_then(|f| f.as_str())
                .filter(|s| !s.is_empty())
            else {
                continue;
            };
            let mut parts = vec![filename.to_string()];
            if let Some(start) = row
                .get("obs_start_time")
                .and_then(|f| f.as_str())
                .filter(|s| !s.is_empty())
            {
                parts.push(start.to_string());
            }
            if let Some(end) = row
                .get("obs_end_time")
                .and_then(|f| f.as_str())
                .filter(|s| !s.is_empty())
            {
                parts.push(end.to_string());
            }
            if let Some(size) = row
                .get("filesize")
                .and_then(|f| f.as_scalar_string())
                .filter(|s| !s.is_empty())
            {
                parts.push(size);
            }
            out.push(parts.join(" · "));
        }
        return out;
    }
    let summary = v
        .get("messages")
        .and_then(|m| m.get("summary"))
        .and_then(|s| s.as_str())
        .unwrap_or("");
    if summary.contains("exceeds the maximum number") {
        out.push(format!("pending — GOSAT overflow: {}", summary.trim()));
    } else if summary.contains("search result is 0") {
        out.push("absent — GOSAT carries no product for the window".to_string());
    }
    out
}

fn epoch_days() -> Option<i64> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .map(|d| (d.as_secs() / 86400) as i64)
}

fn default_window(today: i64) -> (String, String) {
    (iso_date(today - WINDOW_DAYS + 1), iso_date(today))
}

fn iso_date(days: i64) -> String {
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_url_carries_the_measured_parameters() {
        let q = parse_query("GWT3F_L1B 2025-08-01 2025-08-31", 20000);
        assert_eq!(
            search_url(&q),
            "https://product.gosat-gw.nies.go.jp/product_search/api/cui-search/?level=2&mode=2&product=GWT3F_L1B&format=2&version=1&area=1&start=2025-08-01&end=2025-08-31"
        );
    }

    #[test]
    fn bare_query_falls_to_the_default_product_and_the_recent_window() {
        let q = parse_query("", 20000);
        assert_eq!(q.product, PRODUCT_DEFAULT);
        assert_eq!(q.start, iso_date(20000 - WINDOW_DAYS + 1));
        assert_eq!(q.end, iso_date(20000));
    }

    #[test]
    fn product_only_carries_the_recent_window() {
        let q = parse_query("GWT3F_L1B", 20000);
        assert_eq!(q.product, "GWT3F_L1B");
        assert_eq!(q.end, iso_date(20000));
    }

    #[test]
    fn iso_date_round_trips_known_epochs() {
        assert_eq!(iso_date(0), "1970-01-01");
        assert_eq!(iso_date(20000), "2024-10-04");
    }

    #[test]
    fn product_lines_read_the_measured_result_shape() {
        let body = r#"{"result": [
            {"filename": "TANSO3_202508151758008JO1F35004100_1BO00_101101.h5",
             "obs_start_time": "2025-08-15 17:58:07",
             "obs_end_time": "2025-08-15 17:58:21",
             "version": "101101", "path_no": "", "filesize": "17440691",
             "product_quality": "Good"},
            {"filename": "TANSO3_202512301358001NO1W35001400_1BO00_101101.h5",
             "obs_start_time": "2025-12-30 13:58:27",
             "obs_end_time": "2025-12-30 13:58:41",
             "version": "101101", "path_no": "", "filesize": "17276783",
             "product_quality": "Good"}
        ]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            product_lines(&v),
            vec![
                "TANSO3_202508151758008JO1F35004100_1BO00_101101.h5 · 2025-08-15 17:58:07 · 2025-08-15 17:58:21 · 17440691".to_string(),
                "TANSO3_202512301358001NO1W35001400_1BO00_101101.h5 · 2025-12-30 13:58:27 · 2025-12-30 13:58:41 · 17276783".to_string(),
            ]
        );
    }

    #[test]
    fn product_lines_name_the_server_overflow() {
        let body = r#"{"status": "success", "results": {}, "messages": {"level": "info", "summary": "The number of search results exceeds the maximum number (3000). Please change your search conditions and try again.", "details": ""}}"#;
        let v = json::parse(body).unwrap();
        let lines = product_lines(&v);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("pending — GOSAT overflow:"));
    }
}
