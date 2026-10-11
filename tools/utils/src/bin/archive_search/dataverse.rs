use crate::json;
use crate::net::{get, urlencode};

const DEFAULT_HOST: &str = "entrepot.recherche.data.gouv.fr";

pub fn dataverse_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(query, &["host"]);
    let host = crate::refine::value_of(&refine, "host").unwrap_or(DEFAULT_HOST);
    let url = format!(
        "https://{}/api/search?q={}&type=dataset&per_page={}",
        host,
        urlencode(&text),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the dataverse response carries no JSON".to_string()];
            };
            let out = parse_dataverse(&v);
            if out.is_empty() {
                vec![format!("absent — dataverse carries no dataset: {}", text)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — dataverse HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_dataverse(v: &json::Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v
        .get("data")
        .and_then(|d| d.get("items"))
        .and_then(|i| i.as_arr())
    else {
        return out;
    };
    for item in items {
        let url = match item
            .get("url")
            .and_then(|u| u.as_str())
            .filter(|s| !s.is_empty())
        {
            Some(u) => u.to_string(),
            None => match item
                .get("global_id")
                .and_then(|g| g.as_str())
                .filter(|s| !s.is_empty())
            {
                Some(g) => format!("https://doi.org/{}", g.strip_prefix("doi:").unwrap_or(g)),
                None => continue,
            },
        };
        let mut line = format!("url {}", url);
        if let Some(name) = item
            .get("name")
            .and_then(|n| n.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", collapse(name)));
        }
        if let Some(publisher) = item
            .get("publisher")
            .and_then(|p| p.as_str())
            .filter(|s| !s.is_empty())
        {
            line.push_str(&format!("\tpublisher: {}", publisher));
        }
        if let Some(subjects) = item.get("subjects").and_then(|s| s.as_arr()) {
            let list: Vec<&str> = subjects.iter().filter_map(|s| s.as_str()).collect();
            if !list.is_empty() {
                line.push_str(&format!("\tsubjects: {}", list.join(", ")));
            }
        }
        if let Some(description) = item
            .get("description")
            .and_then(|d| d.as_str())
            .filter(|s| !s.is_empty())
        {
            let c = collapse(description);
            let capped: String = c.chars().take(300).collect();
            line.push_str(&format!("\tdescription: {}", capped));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_items() {
        let body = r#"{"data":{"items":[{"name":"Soil carbon","url":"https://doi.org/10.57745/DPAL0I","publisher":"EJP SOIL","subjects":["Earth and Environmental Sciences"],"description":"A  dataset."}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_dataverse(&v),
            vec!["url https://doi.org/10.57745/DPAL0I\ttitle: Soil carbon\tpublisher: EJP SOIL\tsubjects: Earth and Environmental Sciences\tdescription: A dataset.".to_string()]
        );
    }

    #[test]
    fn global_id_is_the_doi_fallback() {
        let body = r#"{"data":{"items":[{"name":"X","global_id":"doi:10.57745/Z"}]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_dataverse(&v),
            vec!["url https://doi.org/10.57745/Z\ttitle: X".to_string()]
        );
    }

    #[test]
    fn empty_items_carry_nothing() {
        let v = json::parse(r#"{"data":{"items":[]}}"#).unwrap();
        assert!(parse_dataverse(&v).is_empty());
    }
}
