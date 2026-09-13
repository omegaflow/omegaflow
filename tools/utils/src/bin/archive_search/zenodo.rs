use crate::json;
use crate::net::{get, urlencode};

pub fn zenodo_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://zenodo.org/api/records?q={}&size={}",
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_zenodo(&f.body);
            if out.is_empty() {
                vec![format!("absent — zenodo carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => {
            let code = match f.status {
                Some(s) => s.to_string(),
                None => "absent".to_string(),
            };
            vec![format!("pending — zenodo HTTP {}", code)]
        }
        None => vec!["pending — no network".to_string()],
    }
}

fn first_file(hit: &json::Json) -> Option<String> {
    hit.get("files")
        .and_then(|f| f.as_arr())
        .and_then(|a| a.first())
        .and_then(|f| f.get("links"))
        .and_then(|l| l.get("self"))
        .and_then(|l| l.as_str())
        .filter(|l| !l.is_empty())
        .map(str::to_string)
}

fn parse_zenodo(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(hits) = v
        .get("hits")
        .and_then(|h| h.get("hits"))
        .and_then(|h| h.as_arr())
    else {
        return out;
    };
    for hit in hits {
        let Some(doi) = hit
            .get("doi")
            .and_then(|d| d.as_str())
            .filter(|d| !d.is_empty())
        else {
            continue;
        };
        let mut line = match first_file(hit) {
            Some(f) => format!("url {}\tdoi: {}", f, doi),
            None => format!("doi: {}", doi),
        };
        if let Some(title) = hit
            .get("title")
            .and_then(|t| t.as_str())
            .filter(|t| !t.is_empty())
        {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(date) = hit
            .get("metadata")
            .and_then(|m| m.get("publication_date"))
            .and_then(|d| d.as_str())
            .filter(|d| !d.is_empty())
        {
            line.push_str(&format!("\tdate: {}", date));
        }
        if let Some(license) = hit
            .get("metadata")
            .and_then(|m| m.get("license"))
            .and_then(|l| l.get("id"))
            .and_then(|l| l.as_str())
            .filter(|l| !l.is_empty())
        {
            line.push_str(&format!("\tlicense: {}", license));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_doi_title_date_license_and_file() {
        let body = r#"{"hits":{"hits":[{"doi":"10.5281/zenodo.1","title":"A dataset","metadata":{"publication_date":"2023-03-07","license":{"id":"cc-by-4.0"}},"files":[{"links":{"self":"https://zenodo.org/api/records/1/files/x/content"}}]}]}}"#;
        assert_eq!(
            parse_zenodo(body),
            vec!["url https://zenodo.org/api/records/1/files/x/content\tdoi: 10.5281/zenodo.1\ttitle: A dataset\tdate: 2023-03-07\tlicense: cc-by-4.0".to_string()]
        );
    }

    #[test]
    fn keeps_the_doi_without_file_or_license() {
        let body = r#"{"hits":{"hits":[{"doi":"10.5281/zenodo.2","title":"B","metadata":{"publication_date":"2020-01-01"}}]}}"#;
        assert_eq!(
            parse_zenodo(body),
            vec!["doi: 10.5281/zenodo.2\ttitle: B\tdate: 2020-01-01".to_string()]
        );
    }

    #[test]
    fn skips_the_hit_without_doi() {
        let body =
            r#"{"hits":{"hits":[{"title":"no doi"},{"doi":"10.5281/zenodo.3","title":"C"}]}}"#;
        assert_eq!(
            parse_zenodo(body),
            vec!["doi: 10.5281/zenodo.3\ttitle: C".to_string()]
        );
    }

    #[test]
    fn empty_hits_carry_nothing() {
        assert!(parse_zenodo(r#"{"hits":{"hits":[]}}"#).is_empty());
    }
}
