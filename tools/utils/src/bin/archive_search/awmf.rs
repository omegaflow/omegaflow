use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://leitlinien-api.awmf.org/v1/search";
const PUBLIC_API_KEY: &str = "MkI5Y1VIOEJ0ZGpoelNBVXRNM1E6WVFld0pBUF9RLVdJa012UHVPTmRQUQ==";

pub fn awmf_lines(query: &str, max: usize) -> Vec<String> {
    let key = PUBLIC_API_KEY;
    let mut offset = 0usize;
    let mut total: Option<u64> = None;
    let (mut lines, stop) = crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |_| {
        let url = format!(
            "{}?keywords={}&lang=de&limit={}&offset={}",
            ENDPOINT,
            urlencode(query),
            max,
            offset
        );
        let header = format!("Api-Key: {}", key);
        match get(&url, &["-H", &header], "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let hits = v
                        .get("meta")
                        .and_then(|m| m.get("hits"))
                        .and_then(|h| h.as_scalar_string())
                        .and_then(|h| h.parse::<u64>().ok());
                    let out = parse_awmf(&v);
                    if total.is_none() {
                        total = hits;
                    }
                    let count = out.len();
                    offset += count;
                    let has_more = count >= max && total.is_some_and(|t| (offset as u64) < t);
                    (out, has_more)
                }
                None => (
                    vec!["pending — the awmf response carries no JSON".to_string()],
                    false,
                ),
            },
            Some(f) if f.status == Some(401) || f.status == Some(403) => (
                vec![format!(
                    "pending — awmf refuses the public register key (HTTP {})",
                    f.status_text()
                )],
                false,
            ),
            Some(f) => (
                vec![format!("pending — awmf HTTP {}", f.status_text())],
                false,
            ),
            None => (vec!["pending — no network".to_string()], false),
        }
    });
    if lines.is_empty() {
        vec![format!("absent — awmf carries no entry: {}", query)]
    } else {
        lines.push(format!("end: {}", stop.label()));
        lines
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_awmf(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(records) = v.get("records").and_then(|r| r.as_arr()) else {
        return out;
    };
    for record in records {
        let Some(url) = field(record, "AWMFDetailPage") else {
            continue;
        };
        let mut line = format!("url {}", url);
        if let Some(name) = field(record, "name") {
            line.push_str(&format!("\ttitle: {}", name));
        }
        if let Some(class) = field(record, "AWMFGuidelineClass") {
            line.push_str(&format!("\tclass: {}", class));
        }
        if let Some(release) = field(record, "AWMFReleaseType") {
            line.push_str(&format!("\trelease: {}", release));
        }
        if let Some(description) = field(record, "description") {
            line.push_str(&format!("\tdescription: {}", description));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_guideline_fields() {
        let body = r#"{"meta":{"hits":212,"status":"success"},"records":[{"AWMFDetailPage":"https://register.awmf.org/de/leitlinien/detail/nvl-002","AWMFGuidelineClass":"S3","AWMFReleaseType":"Revision","name":"National Asthma Guideline","description":"A measured asthma guideline description."}]}"#;
        let v = json::parse(body).expect("the fixture parses");
        assert_eq!(
            parse_awmf(&v),
            vec!["url https://register.awmf.org/de/leitlinien/detail/nvl-002\ttitle: National Asthma Guideline\tclass: S3\trelease: Revision\tdescription: A measured asthma guideline description.".to_string()]
        );
    }

    #[test]
    fn an_error_body_carries_no_record() {
        let body = r#"{"meta":{"message":"Unauthorized or no request auth token provided","status":"error"}}"#;
        let v = json::parse(body).expect("the fixture parses");
        assert!(parse_awmf(&v).is_empty());
    }

    #[test]
    fn a_record_without_a_detail_page_is_skipped() {
        let body = r#"{"records":[{"name":"No page"}]}"#;
        let v = json::parse(body).expect("the fixture parses");
        assert!(parse_awmf(&v).is_empty());
    }
}
