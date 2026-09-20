use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.fda.gov/drug/label.json";

pub fn openfda_lines(query: &str, max: usize) -> Vec<String> {
    let search = format!("openfda.generic_name:\"{}\"", query);
    let url = format!("{}?search={}&limit={}", ENDPOINT, urlencode(&search), max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_openfda(&f.body);
            if out.is_empty() {
                vec![format!("absent — openfda carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) if f.status == Some(404) => {
            vec![format!("absent — openfda carries no entry: {}", query)]
        }
        Some(f) => vec![format!("pending — openfda HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn first(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|a| a.as_arr())
        .and_then(|a| a.first())
        .and_then(|s| s.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_openfda(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for record in results {
        let mut line = String::from("url ");
        let mut url = None;
        if let Some(set_id) = record.get("set_id").and_then(|s| s.as_str()) {
            url = Some(format!(
                "https://dailymed.nlm.nih.gov/dailymed/drugInfo.cfm?setid={}",
                set_id
            ));
        }
        if let Some(u) = url {
            line.push_str(&u);
        } else {
            line.push_str(ENDPOINT);
        }
        if let Some(openfda) = record.get("openfda") {
            if let Some(name) =
                first(openfda, "brand_name").or_else(|| first(openfda, "generic_name"))
            {
                line.push_str(&format!("\ttitle: {}", name));
            }
            if let Some(manufacturer) = first(openfda, "manufacturer_name") {
                line.push_str(&format!("\tmanufacturer: {}", manufacturer));
            }
        }
        if let Some(date) = record.get("effective_time").and_then(|d| d.as_str()) {
            line.push_str(&format!("\tdate: {}", date));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_label_fields() {
        let body = r#"{"results":[{"set_id":"abc-123","effective_time":"20240416","openfda":{"brand_name":["Aspirin"],"manufacturer_name":["Bayer"]}}]}"#;
        assert_eq!(
            parse_openfda(body),
            vec!["url https://dailymed.nlm.nih.gov/dailymed/drugInfo.cfm?setid=abc-123\ttitle: Aspirin\tmanufacturer: Bayer\tdate: 20240416".to_string()]
        );
    }

    #[test]
    fn a_label_without_a_set_id_reads_the_api_url() {
        let body = r#"{"results":[{"openfda":{"generic_name":["Ibuprofen"]}}]}"#;
        assert_eq!(
            parse_openfda(body),
            vec!["url https://api.fda.gov/drug/label.json\ttitle: Ibuprofen".to_string()]
        );
    }
}
