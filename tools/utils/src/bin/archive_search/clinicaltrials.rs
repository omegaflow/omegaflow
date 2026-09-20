use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://clinicaltrials.gov/api/v2/studies";

pub fn clinicaltrials_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "{}?query.term={}&pageSize={}&fields=NCTId,BriefTitle,OverallStatus",
        ENDPOINT,
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_clinicaltrials(&f.body);
            if out.is_empty() {
                vec![format!(
                    "absent — clinicaltrials carries no entry: {}",
                    query
                )]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — clinicaltrials HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_clinicaltrials(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(studies) = v.get("studies").and_then(|s| s.as_arr()) else {
        return out;
    };
    for study in studies {
        let Some(protocol) = study.get("protocolSection") else {
            continue;
        };
        let Some(id) = protocol.get("identificationModule") else {
            continue;
        };
        let Some(nct) = id.get("nctId").and_then(|n| n.as_str()) else {
            continue;
        };
        let mut line = format!("url https://clinicaltrials.gov/study/{}", nct);
        if let Some(title) = id.get("briefTitle").and_then(|t| t.as_str()) {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(status) = protocol
            .get("statusModule")
            .and_then(|s| s.get("overallStatus"))
            .and_then(|s| s.as_str())
        {
            line.push_str(&format!("\tstatus: {}", status));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_study_fields() {
        let body = r#"{"studies":[{"protocolSection":{"identificationModule":{"nctId":"NCT02880982","briefTitle":"Vitamin D in schoolchildren"},"statusModule":{"overallStatus":"COMPLETED"}}}]}"#;
        assert_eq!(
            parse_clinicaltrials(body),
            vec!["url https://clinicaltrials.gov/study/NCT02880982\ttitle: Vitamin D in schoolchildren\tstatus: COMPLETED".to_string()]
        );
    }

    #[test]
    fn a_study_without_an_id_carries_nothing() {
        assert!(parse_clinicaltrials(r#"{"studies":[{"protocolSection":{}}]}"#).is_empty());
    }
}
