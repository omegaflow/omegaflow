use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://reactome.org/ContentService/search/query";

pub fn reactome_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?query={}&cluster=true", ENDPOINT, urlencode(query));
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_reactome(&f.body, max);
            if out.is_empty() {
                vec![format!("absent — reactome carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — reactome HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn first_species(entry: &json::Json) -> Option<String> {
    entry
        .get("species")
        .and_then(|s| s.as_arr())
        .and_then(|a| a.first())
        .and_then(|s| s.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}

fn clean_field(v: &json::Json, key: &str) -> Option<String> {
    field(v, key)
        .map(|s| strip_tags(&s))
        .filter(|s| !s.trim().is_empty())
}

fn parse_reactome(body: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(groups) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for group in groups {
        let Some(entries) = group.get("entries").and_then(|e| e.as_arr()) else {
            continue;
        };
        for entry in entries {
            if out.len() >= max {
                return out;
            }
            let Some(st_id) = field(entry, "stId") else {
                continue;
            };
            let mut line = format!("url https://reactome.org/content/detail/{}", st_id);
            if let Some(name) = clean_field(entry, "name") {
                line.push_str(&format!("\tname: {}", name));
            }
            if let Some(kind) = field(entry, "type") {
                line.push_str(&format!("\ttype: {}", kind));
            }
            if let Some(species) = first_species(entry) {
                line.push_str(&format!("\tspecies: {}", species));
            }
            if let Some(database) = field(entry, "databaseName") {
                line.push_str(&format!("\tdatabase: {}", database));
            }
            if let Some(reference) = field(entry, "referenceIdentifier") {
                line.push_str(&format!("\treference: {}", reference));
            }
            out.push(line);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_entry_fields() {
        let body = r#"{"results":[{"typeName":"Protein","entries":[{"dbId":"69488","stId":"R-HSA-69488","name":"<span class=\"highlighting\" >TP53</span>","type":"Protein","exactType":"ReferenceGeneProduct","species":["Homo sapiens"],"referenceIdentifier":"P04637","databaseName":"UniProt"}]}]}"#;
        assert_eq!(
            parse_reactome(body, 10),
            vec!["url https://reactome.org/content/detail/R-HSA-69488\tname: TP53\ttype: Protein\tspecies: Homo sapiens\tdatabase: UniProt\treference: P04637".to_string()]
        );
    }

    #[test]
    fn the_highlight_markup_falls() {
        assert_eq!(
            strip_tags("<span class=\"highlighting\" >TP53</span> Q5*"),
            "TP53 Q5*"
        );
    }

    #[test]
    fn an_entry_without_an_stid_carries_nothing() {
        assert!(parse_reactome(r#"{"results":[{"entries":[{"name":"x"}]}]}"#, 10).is_empty());
    }

    #[test]
    fn an_empty_result_carries_nothing() {
        assert!(parse_reactome(r#"{"results":[]}"#, 10).is_empty());
    }
}
