use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://ned.ipac.caltech.edu/tap/sync";
const BYNAME_URL: &str = "https://ned.ipac.caltech.edu/byname?objname=";

pub fn ned_lines(query: &str, max: usize) -> Vec<String> {
    let needle = query.trim();
    if needle.is_empty() {
        return vec!["pending — no ned query".to_string()];
    }
    let escaped = needle.to_uppercase().replace('\'', "''");
    let adql = format!(
        "SELECT TOP {} prefname, ra, dec FROM NEDTAP.objdir WHERE prefname LIKE '{}%'",
        max, escaped
    );
    let url = format!(
        "{}?request=doQuery&lang=adql&format=json&query={}",
        ENDPOINT,
        urlencode(&adql)
    );
    match get(&url, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the ned response carries no JSON".to_string()];
            };
            let out = parse_ned(&v, max);
            if out.is_empty() {
                vec![format!("absent — ned carries no object: {}", needle)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — ned HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn cell_str(cell: &json::Json) -> Option<String> {
    match cell {
        json::Json::Null => None,
        json::Json::Str(s) => Some(s.clone()),
        json::Json::Num(n) if n.is_finite() => Some(n.to_string()),
        json::Json::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn row_line(row: &json::Json, columns: &[&str]) -> Option<String> {
    let cells = row.as_arr()?;
    let field = |name: &str| -> Option<String> {
        let i = columns.iter().position(|c| *c == name)?;
        cells.get(i).and_then(cell_str)
    };
    let main_id = field("prefname")?;
    if main_id.is_empty() {
        return None;
    }
    let mut line = format!("url {}{}", BYNAME_URL, urlencode(&main_id));
    line.push_str(&format!("\tmain_id: {}", main_id));
    if let Some(ra) = field("ra") {
        line.push_str(&format!("\tra: {}", ra));
    }
    if let Some(dec) = field("dec") {
        line.push_str(&format!("\tdec: {}", dec));
    }
    Some(line)
}

fn parse_ned(v: &json::Json, max: usize) -> Vec<String> {
    let columns: Vec<&str> = match v.get("metadata").and_then(|m| m.as_arr()) {
        Some(m) => m
            .iter()
            .filter_map(|c| c.get("name").and_then(|n| n.as_str()))
            .collect(),
        None => Vec::new(),
    };
    let Some(rows) = v.get("data").and_then(|d| d.as_arr()) else {
        return Vec::new();
    };
    rows.iter()
        .take(max)
        .filter_map(|r| row_line(r, &columns))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{"metadata":[{"name":"prefname","description":"preferred object name","datatype":"CHAR","arraysize":"30","ucd":"meta.id.cross"},{"name":"ra","description":"RA in decimal degrees of the object Equatorial Coordinate System, J2000.0","datatype":"DOUBLE","unit":"deg","ucd":"pos.eq.ra"},{"name":"dec","description":"DEC in decimal degrees of the object Equatorial Coordinate System, J2000.0","datatype":"DOUBLE","unit":"deg","ucd":"pos.eq.dec"}],"data":[["NGC 7812",0.7268228,-34.2356685],["NGC 7814",0.8121263,16.1453085],["NGC 7808",0.8838602,-10.7446759]]}"#;

    #[test]
    fn reads_metadata_and_rows_into_url_lines() {
        let v = json::parse(SAMPLE).unwrap();
        let out = parse_ned(&v, 10);
        assert_eq!(out.len(), 3);
        assert!(out[0].starts_with("url https://ned.ipac.caltech.edu/byname?objname=NGC%207812"));
        assert!(out[0].contains("\tmain_id: NGC 7812"));
        assert!(out[0].contains("\tra: 0.7268228"));
        assert!(out[0].contains("\tdec: -34.2356685"));
        assert!(out[2].starts_with("url https://ned.ipac.caltech.edu/byname?objname=NGC%207808"));
    }

    #[test]
    fn caps_at_max() {
        let v = json::parse(SAMPLE).unwrap();
        assert_eq!(parse_ned(&v, 2).len(), 2);
    }

    #[test]
    fn a_null_coordinate_is_omitted_never_written() {
        let body = r#"{"metadata":[{"name":"prefname"},{"name":"ra"},{"name":"dec"}],"data":[["MESSIER 031",null,41.269]]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_ned(&v, 10),
            vec![
                "url https://ned.ipac.caltech.edu/byname?objname=MESSIER%20031\tmain_id: MESSIER 031\tdec: 41.269"
                    .to_string()
            ]
        );
    }
}
