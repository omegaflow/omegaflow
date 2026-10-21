use std::sync::{Mutex, OnceLock};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arm {
    pub mode: String,
    pub status: String,
    pub ms: u64,
    pub results: usize,
    pub cached: bool,
    pub retry_after: Option<u64>,
    pub via: Vec<String>,
}

static ARMS: OnceLock<Mutex<Vec<Arm>>> = OnceLock::new();

fn arms() -> &'static Mutex<Vec<Arm>> {
    ARMS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn observe(
    mode: &str,
    ms: u64,
    lines: &[String],
    cached: bool,
    http: Option<i32>,
    retry_after: Option<u64>,
) {
    if mode == "all" {
        return;
    }
    let arm = Arm {
        mode: mode.to_string(),
        status: status_of(cached, http, lines),
        ms,
        results: results_of(lines),
        cached,
        retry_after,
        via: via_of(lines),
    };
    if let Ok(mut list) = arms().lock() {
        list.push(arm);
    }
}

pub fn snapshot() -> Vec<Arm> {
    match arms().lock() {
        Ok(list) => list.clone(),
        Err(_) => Vec::new(),
    }
}

fn body_of(line: &str) -> &str {
    if line.starts_with('[') {
        if let Some((_, body)) = line.split_once("] ") {
            return body;
        }
    }
    line
}

fn http_of(line: &str) -> Option<i32> {
    let rest = line.split("HTTP ").nth(1)?;
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn result_line(body: &str) -> bool {
    body.starts_with("url ") || body.starts_with("identifier ")
}

fn results_of(lines: &[String]) -> usize {
    lines.iter().filter(|line| result_line(body_of(line))).count()
}

fn status_of(cached: bool, http: Option<i32>, lines: &[String]) -> String {
    if cached {
        return "cache".to_string();
    }
    if let Some(code) = http {
        return code.to_string();
    }
    if let Some(code) = lines.iter().find_map(|line| http_of(body_of(line))) {
        return code.to_string();
    }
    let bodies: Vec<&str> = lines.iter().map(|line| body_of(line)).collect();
    if bodies.iter().any(|body| body.starts_with("pending")) {
        return "pending".to_string();
    }
    if bodies.iter().any(|body| body.starts_with("absent")) {
        return "absent".to_string();
    }
    if bodies.iter().any(|body| result_line(body)) {
        return "ok".to_string();
    }
    "pending".to_string()
}

fn via_of(lines: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in lines {
        for segment in line.split('\t') {
            let Some(value) = segment.strip_prefix("via: ") else {
                continue;
            };
            let value = value.trim();
            if !value.is_empty() && !out.iter().any(|seen| seen == value) {
                out.push(value.to_string());
            }
        }
    }
    out
}

fn json_escape(value: &str) -> String {
    crate::net::json_escape(value)
}

fn arm_json(arm: &Arm) -> String {
    let via: Vec<String> = arm
        .via
        .iter()
        .map(|value| format!("\"{}\"", json_escape(value)))
        .collect();
    let retry_after = match arm.retry_after {
        Some(secs) => secs.to_string(),
        None => "null".to_string(),
    };
    format!(
        "{{\"mode\":\"{}\",\"status\":\"{}\",\"ms\":{},\"results\":{},\"cached\":{},\"retry_after\":{},\"via\":[{}]}}",
        json_escape(&arm.mode),
        json_escape(&arm.status),
        arm.ms,
        arm.results,
        arm.cached,
        retry_after,
        via.join(",")
    )
}

pub fn json(arms: &[Arm]) -> String {
    let rows: Vec<String> = arms.iter().map(arm_json).collect();
    format!("[{}]", rows.join(","))
}

pub fn write(path: &str) -> std::io::Result<()> {
    let body = format!("{}\n", json(&snapshot()));
    std::fs::write(path, body)
}

pub fn timing_lines(arms: &[Arm]) -> Vec<String> {
    let mut out = Vec::with_capacity(arms.len() + 1);
    out.push("mode\tstatus\tms\tresults\tcached\tretry_after\tvia".to_string());
    for arm in arms {
        let retry = match arm.retry_after {
            Some(secs) => secs.to_string(),
            None => "-".to_string(),
        };
        out.push(format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            arm.mode,
            arm.status,
            arm.ms,
            arm.results,
            if arm.cached { "yes" } else { "no" },
            retry,
            arm.via.join(",")
        ));
    }
    out
}

pub fn print_timings() {
    for line in timing_lines(&snapshot()) {
        eprintln!("{line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json;

    fn lines(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| item.to_string()).collect()
    }

    #[test]
    fn report_json_carries_the_selected_fields_only() {
        let arm = Arm {
            mode: "brave".to_string(),
            status: "402".to_string(),
            ms: 42,
            results: 1,
            cached: false,
            retry_after: Some(7),
            via: vec!["4get (brave 402)".to_string()],
        };
        let parsed = json::parse(&json(&[arm])).expect("valid json");
        let row = &parsed.as_arr().expect("array")[0];
        let keys: Vec<&str> = match row {
            json::Json::Obj(map) => map.keys().map(String::as_str).collect(),
            _ => panic!("object"),
        };
        assert_eq!(keys.len(), 7);
        for key in [
            "mode",
            "status",
            "ms",
            "results",
            "cached",
            "retry_after",
            "via",
        ] {
            assert!(keys.contains(&key), "missing {key}");
        }
        assert_eq!(row.get("mode").and_then(|v| v.as_str()), Some("brave"));
        assert_eq!(row.get("status").and_then(|v| v.as_str()), Some("402"));
        assert_eq!(row.get("ms").and_then(|v| v.as_scalar_string()), Some("42".to_string()));
        assert_eq!(
            row.get("results").and_then(|v| v.as_scalar_string()),
            Some("1".to_string())
        );
        assert_eq!(row.get("cached"), Some(&json::Json::Bool(false)));
        assert_eq!(
            row.get("retry_after").and_then(|v| v.as_scalar_string()),
            Some("7".to_string())
        );
        assert_eq!(
            row.get("via").and_then(|v| v.as_arr()).map(|v| v.len()),
            Some(1)
        );
    }

    #[test]
    fn cached_arms_report_the_cache_status_and_no_retry() {
        let arm = Arm {
            mode: "brave".to_string(),
            status: status_of(true, Some(402), &lines(&["pending — HTTP 402"])),
            ms: 1,
            results: 0,
            cached: true,
            retry_after: None,
            via: Vec::new(),
        };
        let parsed = json::parse(&json(&[arm])).expect("valid json");
        let row = &parsed.as_arr().expect("array")[0];
        assert_eq!(row.get("status").and_then(|v| v.as_str()), Some("cache"));
        assert_eq!(
            row.get("cached"),
            Some(&json::Json::Bool(true))
        );
        assert_eq!(row.get("retry_after"), Some(&json::Json::Null));
    }

    #[test]
    fn status_comes_from_http_then_pending_then_absent_then_ok() {
        assert_eq!(
            status_of(false, Some(402), &lines(&["pending — HTTP 402"])),
            "402"
        );
        assert_eq!(
            status_of(false, None, &lines(&["pending — Brave HTTP 402"])),
            "402"
        );
        assert_eq!(
            status_of(false, None, &lines(&["pending — no key"])),
            "pending"
        );
        assert_eq!(
            status_of(false, None, &lines(&["absent — nothing there"])),
            "absent"
        );
        assert_eq!(
            status_of(false, None, &lines(&["url https://example.org"])),
            "ok"
        );
        assert_eq!(status_of(false, None, &[]), "pending");
    }

    #[test]
    fn via_labels_are_collected_once_in_order() {
        let watched = lines(&[
            "url https://a\ttitle: A\tvia: 4get (brave 402)",
            "url https://b\ttitle: B\tvia: arxiv-html (api 429)\tvia: 4get (brave 402)",
        ]);
        assert_eq!(
            via_of(&watched),
            vec![
                "4get (brave 402)".to_string(),
                "arxiv-html (api 429)".to_string()
            ]
        );
    }

    #[test]
    fn results_count_only_url_and_identifier_lines() {
        let watched = lines(&[
            "[brave] url https://a\ttitle: A",
            "pending — HTTP 402",
            "identifier 2026-01-01 | title",
            "note: held",
        ]);
        assert_eq!(results_of(&watched), 2);
    }

    #[test]
    fn timing_table_is_text_with_one_row_per_arm() {
        let arm = Arm {
            mode: "brave".to_string(),
            status: "402".to_string(),
            ms: 12,
            results: 0,
            cached: false,
            retry_after: Some(3),
            via: vec!["4get (brave 402)".to_string()],
        };
        let table = timing_lines(&[arm]);
        assert_eq!(table[0], "mode\tstatus\tms\tresults\tcached\tretry_after\tvia");
        assert!(table[1].contains("brave\t402\t12\t0\tno\t3\t4get (brave 402)"));
        assert_eq!(table.len(), 2);
    }
}
