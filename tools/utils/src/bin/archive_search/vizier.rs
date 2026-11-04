use crate::json;
use crate::net::{get, urlencode};

pub fn vizier_lines(query: &str, max: usize) -> Vec<String> {
    let adql = format!(
        "SELECT TOP {} table_name, description FROM TAP_SCHEMA.tables WHERE description LIKE '%{}%'",
        max,
        query.replace('\'', "''")
    );
    let url = format!(
        "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync?request=doQuery&lang=adql&format=json&query={}",
        urlencode(&adql)
    );
    match get(&url, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the vizier response carries no JSON".to_string()];
            };
            let out = parse_vizier(&v, max);
            if out.is_empty() {
                vec![format!("absent — vizier carries no table: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — vizier HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn clean_table(name: &str) -> String {
    name.trim_matches('"').to_string()
}

fn row_line(row: &json::Json) -> Option<String> {
    let cells = row.as_arr()?;
    let table = cells.first().and_then(|c| c.as_str()).map(clean_table)?;
    if table.is_empty() {
        return None;
    }
    match cells.get(1).and_then(|c| c.as_str()) {
        Some(description) if !description.is_empty() => Some(format!(
            "url https://vizier.cds.unistra.fr/viz-bin/VizieR?-source={}\ttable: {}\tdescription: {}",
            table, table, description
        )),
        _ => Some(format!(
            "url https://vizier.cds.unistra.fr/viz-bin/VizieR?-source={}\ttable: {}",
            table, table
        )),
    }
}

fn parse_vizier(v: &json::Json, max: usize) -> Vec<String> {
    let Some(rows) = v.get("data").and_then(|d| d.as_arr()) else {
        return Vec::new();
    };
    rows.iter().take(max).filter_map(row_line).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"{"metadata":[{"name":"table_name","description":"the table name","datatype":"CHAR","arraysize":"64*","ucd":"Metadata"},{"name":"description","description":"description","datatype":"CHAR","arraysize":"255*","ucd":"Metadata"}],"data":[["\"J/A+A/481/593/table1\"","Velocity measurements for 512 objects in the galaxy cluster Abell 2163 ( Maurogordato S., Cappi A.)"],["\"J/A+A/587/A153/galaxy\"","New galaxy cluster candidates in VISION ( Meingast S., Alves J.)"]]}"#;

    #[test]
    fn reads_table_and_description() {
        let v = json::parse(FIXTURE).unwrap();
        assert_eq!(
            parse_vizier(&v, 10),
            vec![
                "url https://vizier.cds.unistra.fr/viz-bin/VizieR?-source=J/A+A/481/593/table1\ttable: J/A+A/481/593/table1\tdescription: Velocity measurements for 512 objects in the galaxy cluster Abell 2163 ( Maurogordato S., Cappi A.)".to_string(),
                "url https://vizier.cds.unistra.fr/viz-bin/VizieR?-source=J/A+A/587/A153/galaxy\ttable: J/A+A/587/A153/galaxy\tdescription: New galaxy cluster candidates in VISION ( Meingast S., Alves J.)".to_string()
            ]
        );
    }

    #[test]
    fn caps_at_max() {
        let v = json::parse(FIXTURE).unwrap();
        assert_eq!(parse_vizier(&v, 1).len(), 1);
    }

    #[test]
    fn a_missing_description_is_not_invented() {
        let v = json::parse(r#"{"data":[["\"B/astorb/astorb\""]]}"#).unwrap();
        assert_eq!(
            parse_vizier(&v, 10),
            vec![
                "url https://vizier.cds.unistra.fr/viz-bin/VizieR?-source=B/astorb/astorb\ttable: B/astorb/astorb"
                    .to_string()
            ]
        );
    }
}
