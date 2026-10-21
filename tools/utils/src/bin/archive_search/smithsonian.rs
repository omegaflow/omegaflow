use crate::json;
use crate::net::{get, urlencode};

pub fn smithsonian_lines(query: &str, key: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://api.si.edu/openaccess/api/v1.0/search?api_key={}&q={}&rows={}&start=0",
        urlencode(key),
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let lines = parse_smithsonian(&f.body);
            if lines.is_empty() {
                vec![format!("absent — Smithsonian carries no item: {}", query)]
            } else {
                lines
            }
        }
        Some(f) => vec![format!("pending — smithsonian HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn text(v: Option<&json::Json>) -> Option<String> {
    v.and_then(|x| x.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
}

fn parse_smithsonian(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(rows) = v
        .get("response")
        .and_then(|r| r.get("rows"))
        .and_then(|r| r.as_arr())
    else {
        return out;
    };
    for row in rows {
        let link = row
            .get("content")
            .and_then(|c| c.get("descriptiveNonRepeating"))
            .and_then(|d| {
                text(d.get("record_link")).or_else(|| text(d.get("guid")))
            });
        let Some(url) = link else {
            continue;
        };
        let mut line = format!("url {url}");
        if let Some(title) = text(row.get("title")) {
            line.push_str(&format!("\ttitle: {title}"));
        }
        if let Some(unit) = text(row.get("unitCode")) {
            line.push_str(&format!("\tunit: {unit}"));
        }
        if let Some(kind) = text(row.get("type")) {
            line.push_str(&format!("\ttype: {kind}"));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_row_fields() {
        let body = r#"{"status":200,"response":{"rows":[{"title":"Solar wind model","unitCode":"NASM","type":"image","content":{"descriptiveNonRepeating":{"record_link":"https://collections.si.edu/search/detail/1","guid":"si:1"}}}]}}"#;
        assert_eq!(
            parse_smithsonian(body),
            vec!["url https://collections.si.edu/search/detail/1\ttitle: Solar wind model\tunit: NASM\ttype: image".to_string()]
        );
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_smithsonian(r#"{"response":{"rows":[]}}"#).is_empty());
    }
}
