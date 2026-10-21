use crate::json;
use std::process::Command;

const ENDPOINT: &str = "https://ydc-index.io/v1/search";
const KEY: &str = "YOUCOM_API_KEY";

pub fn youcom_lines(query: &str, max: usize) -> Vec<String> {
    let Some(key) = crate::token::secret(KEY) else {
        return vec![format!("pending — {KEY} absent from .secrets.local/.env")];
    };
    let count = max.clamp(1, 100);
    let body = format!(
        "{{\"query\":\"{}\",\"count\":{count}}}",
        escape(query)
    );
    let out = Command::new("curl")
        .args([
            "-sS",
            "--max-time",
            "40",
            "-X",
            "POST",
            ENDPOINT,
            "-H",
            "Content-Type: application/json",
            "-H",
        ])
        .arg(format!("X-API-Key: {key}"))
        .args(["-d", &body, "-w", "\n%{http_code}"])
        .output()
        .ok();
    let Some(out) = out else {
        return vec!["pending — no network".to_string()];
    };
    let stdout = out.stdout;
    let Some(idx) = stdout.iter().rposition(|&b| b == b'\n') else {
        return vec!["pending — you.com response carries no status".to_string()];
    };
    let status = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .parse::<i32>()
        .ok();
    let body = String::from_utf8_lossy(&stdout[..idx]).to_string();
    match status {
        Some(200) => {
            let lines = parse_youcom(&body);
            if lines.is_empty() {
                vec![format!("absent — You.com carries no web result: {query}")]
            } else {
                lines
            }
        }
        Some(402) => vec!["pending — You.com HTTP 402 (prepaid credit balance depleted)".to_string()],
        Some(code) => vec![format!("pending — You.com HTTP {code}")],
        None => vec!["pending — you.com status unreadable".to_string()],
    }
}

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push(' '),
            '\r' => {}
            '\t' => out.push(' '),
            _ => out.push(c),
        }
    }
    out
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
}

fn joined(v: Option<&json::Json>) -> Option<String> {
    let values = v?.as_arr()?;
    let parts: Vec<&str> = values
        .iter()
        .filter_map(|x| x.as_str())
        .filter(|s| !s.trim().is_empty())
        .take(2)
        .collect();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn parse_youcom(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v.get("results") else {
        return out;
    };
    let mut items: Vec<&json::Json> = Vec::new();
    if let Some(web) = results.get("web").and_then(|w| w.as_arr()) {
        items.extend(web.iter());
    }
    if let Some(news) = results.get("news").and_then(|n| n.as_arr()) {
        items.extend(news.iter());
    }
    if items.is_empty() {
        if let Some(all) = results.as_arr() {
            items.extend(all.iter());
        }
    }
    for item in items {
        let Some(url) = field(item, "url") else {
            continue;
        };
        let mut line = format!("url {url}");
        if let Some(title) = field(item, "title") {
            line.push_str(&format!("\ttitle: {title}"));
        }
        let description = item
            .get("contents")
            .and_then(|c| joined(c.get("highlights")))
            .or_else(|| joined(item.get("snippets")))
            .or_else(|| field(item, "description"));
        if let Some(description) = description {
            line.push_str(&format!("\tdescription: {description}"));
        }
        if let Some(age) = field(item, "page_age") {
            line.push_str(&format!("\tdate: {}", &age[..age.len().min(10)]));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_web_results() {
        let body = r#"{"results":{"web":[{"url":"https://example.org/a","title":"A","page_age":"2026-10-01T00:00:00","contents":{"highlights":["one","two","three"]}},{"url":"https://example.org/b","title":"B","description":"plain"}]},"metadata":{"query":"x"}}"#;
        assert_eq!(
            parse_youcom(body),
            vec![
                "url https://example.org/a\ttitle: A\tdescription: one two\tdate: 2026-10-01".to_string(),
                "url https://example.org/b\ttitle: B\tdescription: plain".to_string(),
            ]
        );
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_youcom(r#"{"results":{"web":[]}}"#).is_empty());
    }

    #[test]
    fn escapes_the_query() {
        assert_eq!(escape("a \"b\"\\c\n"), "a \\\"b\\\"\\\\c ");
    }
}
