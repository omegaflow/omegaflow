use crate::json;
use crate::net::{get, urlencode};

pub fn erddap_lines(query: &str, max: usize) -> Vec<String> {
    let mut server = "";
    let mut searchfor = "";
    let mut page = max.min(50);
    for tok in query.split_whitespace() {
        if let Some(v) = tok.strip_prefix("server=") {
            server = v;
        } else if let Some(v) = tok.strip_prefix("searchFor=") {
            searchfor = v;
        } else if let Some(v) = tok.strip_prefix("itemsPerPage=") {
            if let Ok(n) = v.parse::<usize>() {
                page = n;
            }
        }
    }
    if server.is_empty() || searchfor.is_empty() {
        return vec![
            "usage — archive_search --erddap \"server=<url> searchFor=<kw> [itemsPerPage=N]\""
                .to_string(),
        ];
    }
    let base = server.trim_end_matches('/');
    let url = format!(
        "{}/erddap/search/advanced.json?searchFor={}&itemsPerPage={}&page=1",
        base,
        urlencode(searchfor),
        page
    );
    match get(&url, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the erddap response carries no JSON".to_string()];
            };
            let out = parse_erddap(&v, max);
            if out.is_empty() {
                vec![format!("absent — erddap carries no dataset: {}", searchfor)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — erddap HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn column_index(names: &[json::Json], key: &str) -> Option<usize> {
    names.iter().position(|n| n.as_str() == Some(key))
}

fn cell(row: &json::Json, idx: Option<usize>) -> String {
    let Some(i) = idx else {
        return String::new();
    };
    row.as_arr()
        .and_then(|a| a.get(i))
        .and_then(|c| c.as_str())
        .unwrap_or("")
        .to_string()
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_erddap(v: &json::Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(table) = v.get("table") else {
        return out;
    };
    let Some(names) = table.get("columnNames").and_then(|n| n.as_arr()) else {
        return out;
    };
    let data = column_index(names, "tabledap");
    let grid = column_index(names, "griddap");
    let title = column_index(names, "Title");
    let inst = column_index(names, "Institution");
    let id = column_index(names, "Dataset ID");
    let Some(rows) = table.get("rows").and_then(|r| r.as_arr()) else {
        return out;
    };
    for row in rows {
        let tabledap = cell(row, data);
        let data_url = if tabledap.is_empty() {
            cell(row, grid)
        } else {
            tabledap
        };
        if data_url.is_empty() {
            continue;
        }
        let title_s = cell(row, title);
        let inst_s = cell(row, inst);
        let id_s = cell(row, id);
        let mut line = format!("url {data_url}");
        if !title_s.is_empty() {
            line.push_str(&format!("\ttitle: {}", collapse(&title_s)));
        }
        if !inst_s.is_empty() {
            line.push_str(&format!("\tinstitution: {}", collapse(&inst_s)));
        }
        if !id_s.is_empty() {
            line.push_str(&format!("\tid: {id_s}"));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_table_rows() {
        let body = r#"{"table":{"columnNames":["griddap","tabledap","Title","Institution","Dataset ID"],"rows":[["https://s/griddap/A","","Salinity anomaly","EMODnet Physics","ERD_EP"]]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_erddap(&v, 10),
            vec![
                "url https://s/griddap/A\ttitle: Salinity anomaly\tinstitution: EMODnet Physics\tid: ERD_EP"
                    .to_string()
            ]
        );
    }

    #[test]
    fn prefers_tabledap_over_griddap() {
        let body = r#"{"table":{"columnNames":["griddap","tabledap","Dataset ID"],"rows":[["https://s/griddap/A","https://s/tabledap/A","A"]]}}"#;
        let v = json::parse(body).unwrap();
        assert!(parse_erddap(&v, 10)[0].contains("tabledap/A"));
    }

    #[test]
    fn caps_at_max() {
        let body = r#"{"table":{"columnNames":["griddap","Dataset ID"],"rows":[["https://s/griddap/A","A"],["https://s/griddap/B","B"]]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(parse_erddap(&v, 1).len(), 1);
    }
}
