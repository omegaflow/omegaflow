use crate::json;
use crate::net::{get, urlencode};
use crate::refine::{split_refine, value_of};

const LABEL: &str = "https://api.fda.gov/drug/label.json";
const EVENT: &str = "https://api.fda.gov/drug/event.json";
const DRUGSFDA: &str = "https://api.fda.gov/drug/drugsfda.json";

/// openFDA search with an endpoint switch:
///
///   --openfda "finerenone"                       drug label, generic name
///   --openfda "finerenone field=brand"           drug label, brand name
///   --openfda "finerenone endpoint=event"        adverse-event reports (FAERS)
///   --openfda "finerenone endpoint=drug"         Drugs@FDA applications
///   --openfda "finerenone endpoint=all"          label + event + Drugs@FDA
pub fn openfda_lines(query: &str, max: usize) -> Vec<String> {
    let (text, opts) = split_refine(query, &["endpoint", "field"]);
    let text = text.trim();
    if text.is_empty() {
        return vec!["absent — openfda needs a query, e.g. finerenone endpoint=event".to_string()];
    }
    let endpoint = value_of(&opts, "endpoint").unwrap_or("label");
    let field = match value_of(&opts, "field") {
        Some(f @ ("generic" | "brand")) => f,
        _ => "generic",
    };
    let out = match endpoint {
        "event" => event_lines(text, field, max),
        "drug" | "drugsfda" => drugsfda_lines(text, field, max),
        "all" => {
            let mut all = label_lines(text, field, max);
            all.extend(with_source(event_lines(text, field, max), "event"));
            all.extend(with_source(drugsfda_lines(text, field, max), "drugsfda"));
            all
        }
        _ => label_lines(text, field, max),
    };
    if out.is_empty() {
        vec![format!("absent — openfda carries no entry: {}", text)]
    } else {
        out
    }
}

fn with_source(lines: Vec<String>, source: &str) -> Vec<String> {
    lines
        .into_iter()
        .map(|line| format!("{}\tsource: {}", line, source))
        .collect()
}

fn search_url(base: &str, search: &str, max: usize) -> String {
    format!("{}?search={}&limit={}", base, urlencode(search), max)
}

fn query_label(query: &str, field: &str, max: usize) -> Vec<String> {
    let search = format!("openfda.{}_name:\"{}\"", field, query);
    let url = search_url(LABEL, &search, max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => parse_label(&f.body),
        Some(f) if f.status == Some(404) => Vec::new(),
        Some(f) => vec![format!("pending — openfda HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn label_lines(query: &str, field: &str, max: usize) -> Vec<String> {
    let out = query_label(query, field, max);
    if out.is_empty() && field == "generic" {
        return query_label(query, "brand", max);
    }
    out
}

fn event_lines(query: &str, field: &str, max: usize) -> Vec<String> {
    let out = query_event(query, field, max);
    if out.is_empty() && field == "generic" {
        return query_event(query, "brand", max);
    }
    out
}

fn query_event(query: &str, field: &str, max: usize) -> Vec<String> {
    let search = format!("patient.drug.openfda.{}_name:\"{}\"", field, query);
    let url = search_url(EVENT, &search, max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => parse_event(&f.body, &url, query),
        Some(f) if f.status == Some(404) => Vec::new(),
        Some(f) => vec![format!("pending — openfda HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn drugsfda_lines(query: &str, field: &str, max: usize) -> Vec<String> {
    let out = query_drugsfda(query, field, max);
    if out.is_empty() && field == "generic" {
        return query_drugsfda(query, "brand", max);
    }
    out
}

fn query_drugsfda(query: &str, field: &str, max: usize) -> Vec<String> {
    let search = format!("openfda.{}_name:\"{}\"", field, query);
    let url = search_url(DRUGSFDA, &search, max);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => parse_drugsfda(&f.body, &url),
        Some(f) if f.status == Some(404) => Vec::new(),
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

fn text(v: Option<&json::Json>, key: &str) -> Option<String> {
    v.and_then(|v| v.get(key))
        .and_then(|s| s.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn scalar(v: Option<&json::Json>, key: &str) -> Option<String> {
    v.and_then(|v| v.get(key))
        .and_then(|s| s.as_scalar_string())
        .filter(|s| !s.is_empty())
}

fn joined(list: Option<&json::Json>, key: &str, cap: usize) -> Option<String> {
    let arr = list?.as_arr()?;
    let mut values: Vec<String> = Vec::new();
    for item in arr {
        if let Some(value) = text(Some(item), key) {
            if !values.iter().any(|v| v == &value) {
                values.push(value);
            }
            if values.len() >= cap {
                break;
            }
        }
    }
    if values.is_empty() {
        None
    } else {
        Some(values.join(", "))
    }
}

fn parse_label(body: &str) -> Vec<String> {
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
            line.push_str(LABEL);
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

fn drug_names(patient: Option<&json::Json>, query: &str, cap: usize) -> Option<String> {
    let arr = patient?.get("drug")?.as_arr()?;
    let needle = query.to_lowercase();
    let mut matching: Vec<String> = Vec::new();
    let mut others: Vec<String> = Vec::new();
    for item in arr {
        let openfda = item.get("openfda");
        let candidates: Vec<Option<String>> = vec![
            text(Some(item), "medicinalproduct"),
            openfda.and_then(|o| first(o, "brand_name")),
            openfda.and_then(|o| first(o, "generic_name")),
        ];
        let hit = candidates
            .iter()
            .flatten()
            .any(|name| name.to_lowercase().contains(&needle));
        let Some(name) = candidates.into_iter().flatten().next() else {
            continue;
        };
        if hit {
            matching.push(name);
        } else {
            others.push(name);
        }
    }
    matching.extend(others);
    let mut values: Vec<String> = Vec::new();
    for value in matching {
        if !values.iter().any(|v| v == &value) {
            values.push(value);
        }
        if values.len() >= cap {
            break;
        }
    }
    if values.is_empty() {
        None
    } else {
        Some(values.join(", "))
    }
}

fn parse_event(body: &str, url: &str, query: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for record in results {
        let patient = record.get("patient");
        let mut line = format!("url {}", url);
        if let Some(serious) = scalar(Some(record), "serious") {
            line.push_str(&format!(
                "\tserious: {}",
                if serious == "1" { "yes" } else { "no" }
            ));
        }
        if let Some(date) = scalar(Some(record), "receivedate") {
            line.push_str(&format!("\tdate: {}", date));
        }
        if let Some(country) = text(Some(record), "occurcountry")
            .or_else(|| text(Some(record), "primarysourcecountry"))
        {
            line.push_str(&format!("\tcountry: {}", country));
        }
        if let Some(reactions) = joined(
            patient.and_then(|p| p.get("reaction")),
            "reactionmeddrapt",
            5,
        ) {
            line.push_str(&format!("\treactions: {}", reactions));
        }
        if let Some(drugs) = drug_names(patient, query, 3) {
            line.push_str(&format!("\tdrugs: {}", drugs));
        }
        out.push(line);
    }
    out
}

fn parse_drugsfda(body: &str, url: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for record in results {
        let mut line = format!("url {}", url);
        if let Some(application) = text(Some(record), "application_number") {
            line.push_str(&format!("\tapplication: {}", application));
        }
        if let Some(sponsor) = text(Some(record), "sponsor_name") {
            line.push_str(&format!("\tmanufacturer: {}", sponsor));
        }
        if let Some(brands) = joined(record.get("products"), "brand_name", 3) {
            line.push_str(&format!("\ttitle: {}", brands));
        }
        if let Some(status) = joined(record.get("products"), "marketing_status", 2) {
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
    fn reads_the_label_fields() {
        let body = r#"{"results":[{"set_id":"abc-123","effective_time":"20240416","openfda":{"brand_name":["Aspirin"],"manufacturer_name":["Bayer"]}}]}"#;
        assert_eq!(
            parse_label(body),
            vec!["url https://dailymed.nlm.nih.gov/dailymed/drugInfo.cfm?setid=abc-123\ttitle: Aspirin\tmanufacturer: Bayer\tdate: 20240416".to_string()]
        );
    }

    #[test]
    fn a_label_without_a_set_id_reads_the_api_url() {
        let body = r#"{"results":[{"openfda":{"generic_name":["Ibuprofen"]}}]}"#;
        assert_eq!(
            parse_label(body),
            vec!["url https://api.fda.gov/drug/label.json\ttitle: Ibuprofen".to_string()]
        );
    }

    #[test]
    fn reads_an_event_report() {
        let body = r#"{"results":[{"serious":"1","receivedate":"20140714","occurcountry":"US","patient":{"reaction":[{"reactionmeddrapt":"Dyspnoea"},{"reactionmeddrapt":"Dyspnoea"}],"drug":[{"medicinalproduct":"ENTRESTO"},{"medicinalproduct":"FUROSEMIDE"}]}}]}"#;
        assert_eq!(
            parse_event(body, EVENT, "entresto"),
            vec![format!(
                "url {}\tserious: yes\tdate: 20140714\tcountry: US\treactions: Dyspnoea\tdrugs: ENTRESTO, FUROSEMIDE",
                EVENT
            )]
        );
    }

    #[test]
    fn reads_a_drugsfda_application() {
        let body = r#"{"results":[{"application_number":"ANDA213605","sponsor_name":"CRYSTAL","products":[{"brand_name":"SACUBITRIL AND VALSARTAN","marketing_status":"Prescription"}]}]}"#;
        assert_eq!(
            parse_drugsfda(body, DRUGSFDA),
            vec![format!(
                "url {}\tapplication: ANDA213605\tmanufacturer: CRYSTAL\ttitle: SACUBITRIL AND VALSARTAN\tstatus: Prescription",
                DRUGSFDA
            )]
        );
    }

    #[test]
    fn empty_results_carry_nothing() {
        assert!(parse_event(r#"{"results":[]}"#, EVENT, "x").is_empty());
        assert!(parse_drugsfda(r#"{"results":[]}"#, DRUGSFDA).is_empty());
    }
}
