use crate::net::get;

const ENDPOINT: &str = "https://www.re3data.org/api/v1/repositories";
const REPO: &str = "https://www.re3data.org/repository/";

pub fn re3data_lines(query: &str, max: usize) -> Vec<String> {
    match get(ENDPOINT, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = filter_re3data(parse_re3data(&f.body), query, max);
            if out.is_empty() {
                vec![format!("absent — re3data carries no repository: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — re3data HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn filter_re3data(all: Vec<(String, String)>, query: &str, max: usize) -> Vec<String> {
    let terms: Vec<String> = query.split_whitespace().map(|t| t.to_lowercase()).collect();
    all.into_iter()
        .filter(|(name, _)| {
            terms.is_empty()
                || terms
                    .iter()
                    .all(|t| name.to_lowercase().contains(t.as_str()))
        })
        .map(|(_, line)| line)
        .take(max)
        .collect()
}

fn tag<'a>(block: &'a str, name: &str) -> Option<&'a str> {
    let open = format!("<{}>", name);
    let close = format!("</{}>", name);
    let start = block.find(&open)? + open.len();
    let end = block[start..].find(&close)? + start;
    let value = block[start..end].trim();
    if value.is_empty() { None } else { Some(value) }
}

fn parse_re3data(body: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for block in body.split("<repository>").skip(1) {
        let Some(id) = tag(block, "id") else {
            continue;
        };
        let name = tag(block, "name").unwrap_or("").to_string();
        let mut line = format!("url {}{}", REPO, id);
        if !name.is_empty() {
            line.push_str(&format!("\tname: {}", name));
        }
        if let Some(doi) = tag(block, "doi") {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        out.push((name, line));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_id_name_doi() {
        let body = "<?xml version=\"1.0\"?><list><repository><id>r3d100000001</id><doi>https://doi.org/10.17616/R31NJCHT</doi><name>Odum Institute Archive Dataverse</name><link href=\"/x\" rel=\"self\" /></repository></list>";
        assert_eq!(
            parse_re3data(body),
            vec![(
                "Odum Institute Archive Dataverse".to_string(),
                "url https://www.re3data.org/repository/r3d100000001\tname: Odum Institute Archive Dataverse\tdoi: https://doi.org/10.17616/R31NJCHT".to_string()
            )]
        );
    }

    #[test]
    fn filters_by_name_substring() {
        let one = "<repository><id>r3d1</id><name>EMODnet Physics</name></repository>";
        let two =
            "<repository><id>r3d2</id><name>Odum Institute Archive Dataverse</name></repository>";
        let body = format!("<list>{}{}</list>", one, two);
        let out = filter_re3data(parse_re3data(&body), "emodnet", 10);
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("r3d1"));
    }

    #[test]
    fn all_terms_must_match() {
        let one = "<repository><id>r3d1</id><name>EMODnet Physics Data</name></repository>";
        let two = "<repository><id>r3d2</id><name>EMODnet Chemistry</name></repository>";
        let body = format!("<list>{}{}</list>", one, two);
        let out = filter_re3data(parse_re3data(&body), "emodnet data", 10);
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("r3d1"));
    }

    #[test]
    fn caps_at_max() {
        let one = "<repository><id>r3d1</id><name>A</name></repository>";
        let body = format!("<list>{}{}</list>", one, one);
        assert_eq!(filter_re3data(parse_re3data(&body), "", 1).len(), 1);
    }

    #[test]
    fn empty_list_carries_nothing() {
        assert!(filter_re3data(parse_re3data("<list></list>"), "x", 10).is_empty());
    }
}
