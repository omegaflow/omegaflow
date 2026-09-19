use crate::json;
use crate::net::get;

const ENDPOINT: &str = "https://psychporta.org/api/search";

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn request_body(query: &str, from: usize, size: usize) -> String {
    format!(
        "{{\"index\":[\"psyndex\",\"psycharchives\",\"persons\",\"tests\"],\"from\":{},\"size\":{},\"track_total_hits\":true,\"query\":{{\"bool\":{{\"must\":[{{\"multi_match\":{{\"query\":{},\"fields\":[\"hasInstanceBundle.title.label^5\",\"testname^5\",\"summary.label.val^5\",\"subject.label^3\"]}}}}]}}}}}}",
        from,
        size,
        json_escape(query)
    )
}

pub fn psychporta_lines(query: &str, max: usize) -> Vec<String> {
    let mut from = 0usize;
    let mut total: Option<u64> = None;
    let (mut lines, stop) = crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |_| {
        let body = request_body(query, from, max);
        let extra = [
            "-X",
            "POST",
            "-H",
            "Content-Type: application/json",
            "--data",
            body.as_str(),
        ];
        match get(ENDPOINT, &extra, "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let (page_total, out) = parse_psychporta(&v);
                    if total.is_none() {
                        total = page_total;
                    }
                    let count = out.len();
                    from += count;
                    let has_more =
                        count >= max && total.is_some_and(|t| (from as u64) < t);
                    (out, has_more)
                }
                None => (
                    vec!["pending — the psychporta response carries no JSON".to_string()],
                    false,
                ),
            },
            Some(f) => (
                vec![format!("pending — psychporta HTTP {}", f.status_text())],
                false,
            ),
            None => (vec!["pending — no network".to_string()], false),
        }
    });
    if lines.is_empty() {
        return vec![format!("absent — psychporta carries no entry: {}", query)];
    }
    if let Some(t) = total {
        lines.insert(0, format!("total: {}", t));
    }
    lines.push(format!("end: {}", stop.label()));
    lines
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn index_label(index: &str) -> String {
    index.split('_').nth(2).unwrap_or(index).to_string()
}

fn title_of(source: &json::Json) -> Option<String> {
    let label = source
        .get("hasInstanceBundle")
        .and_then(|b| b.as_arr())
        .and_then(|b| b.first())
        .and_then(|b| b.get("title"))
        .and_then(|t| t.as_arr())
        .and_then(|t| t.first())
        .and_then(|t| t.get("label"))
        .and_then(|l| l.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    label.or_else(|| field(source, "testname"))
}

fn parse_psychporta(v: &json::Json) -> (Option<u64>, Vec<String>) {
    let mut out = Vec::new();
    let Some(hits) = v.get("hits") else {
        return (None, out);
    };
    let total = hits
        .get("total")
        .and_then(|t| t.get("value"))
        .and_then(|n| n.as_scalar_string())
        .and_then(|s| s.parse().ok());
    let Some(arr) = hits.get("hits").and_then(|h| h.as_arr()) else {
        return (total, out);
    };
    for hit in arr {
        let Some(id) = field(hit, "_id") else {
            continue;
        };
        let mut line = format!("url https://psychporta.org/works/{}", id);
        if let Some(source) = hit.get("_source") {
            if let Some(title) = title_of(source) {
                line.push_str(&format!("\ttitle: {}", title));
            }
        }
        if let Some(index) = field(hit, "_index") {
            line.push_str(&format!("\tindex: {}", index_label(&index)));
        }
        out.push(line);
    }
    (total, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_total_and_the_hit_fields() {
        let v = json::parse(
            r#"{"hits":{"total":{"value":3169,"relation":"eq"},"hits":[{"_id":"pa_abc","_index":"prod_psychporta_psycharchives_2026-09-19_05-00-03","_source":{"id":"https://w3id.org/zpid/resources/works/pa_abc","hasInstanceBundle":[{"title":[{"label":"A measured review of psychological interventions"}]}]}}]}}"#,
        )
        .unwrap();
        let (total, lines) = parse_psychporta(&v);
        assert_eq!(total, Some(3169));
        assert_eq!(
            lines,
            vec!["url https://psychporta.org/works/pa_abc\ttitle: A measured review of psychological interventions\tindex: psycharchives".to_string()]
        );
    }

    #[test]
    fn falls_back_to_the_testname() {
        let v = json::parse(
            r#"{"hits":{"total":{"value":1},"hits":[{"_id":"pt_x","_index":"prod_psychporta_tests_2026-09-19_05-00-03","_source":{"testname":"A measured test"}}]}}"#,
        )
        .unwrap();
        let (_, lines) = parse_psychporta(&v);
        assert_eq!(
            lines,
            vec!["url https://psychporta.org/works/pt_x\ttitle: A measured test\tindex: tests"
                .to_string()]
        );
    }

    #[test]
    fn an_empty_result_carries_the_total_only() {
        let v = json::parse(r#"{"hits":{"total":{"value":0},"hits":[]}}"#).unwrap();
        let (total, lines) = parse_psychporta(&v);
        assert_eq!(total, Some(0));
        assert!(lines.is_empty());
    }

    #[test]
    fn escapes_the_query_into_the_body() {
        let body = request_body(r#"a "b" \ c"#, 0, 10);
        assert!(body.contains(r#""query":"a \"b\" \\ c""#));
    }
}
