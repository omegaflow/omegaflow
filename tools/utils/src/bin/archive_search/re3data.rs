use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://www.re3data.org/api/v1/repositories";
const REPO: &str = "https://www.re3data.org/repository/";

pub fn re3data_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("{}?query={}", ENDPOINT, urlencode(query));
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_re3data(&f.body, max);
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

fn tag<'a>(block: &'a str, name: &str) -> Option<&'a str> {
    let open = format!("<{}>", name);
    let close = format!("</{}>", name);
    let start = block.find(&open)? + open.len();
    let end = block[start..].find(&close)? + start;
    let value = block[start..end].trim();
    if value.is_empty() { None } else { Some(value) }
}

fn parse_re3data(body: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    for block in body.split("<repository>").skip(1) {
        if out.len() >= max {
            break;
        }
        let Some(id) = tag(block, "id") else {
            continue;
        };
        let mut line = format!("url {}{}", REPO, id);
        if let Some(name) = tag(block, "name") {
            line.push_str(&format!("\tname: {}", name));
        }
        if let Some(doi) = tag(block, "doi") {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        out.push(line);
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
            parse_re3data(body, 10),
            vec!["url https://www.re3data.org/repository/r3d100000001\tname: Odum Institute Archive Dataverse\tdoi: https://doi.org/10.17616/R31NJCHT".to_string()]
        );
    }

    #[test]
    fn caps_at_max() {
        let one = "<repository><id>r3d1</id><name>A</name></repository>";
        let body = format!("<list>{}{}</list>", one, one);
        assert_eq!(parse_re3data(&body, 1).len(), 1);
    }

    #[test]
    fn empty_list_carries_nothing() {
        assert!(parse_re3data("<list></list>", 10).is_empty());
    }
}
