use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://simbad.u-strasbg.fr/simbad/sim-tap/sync";
const ID_URL: &str = "https://simbad.u-strasbg.fr/simbad/sim-id?Ident=";

pub fn simbad_lines(query: &str, max: usize) -> Vec<String> {
    let needle = query.trim();
    if needle.is_empty() {
        return vec!["pending — no simbad query".to_string()];
    }
    let escaped = needle.replace('\'', "''");
    let adql = format!(
        "SELECT TOP {} main_id, ra, dec FROM basic JOIN ident ON basic.oid=ident.oidref WHERE ident.id LIKE '%{}%'",
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
                return vec!["pending — the simbad response carries no JSON".to_string()];
            };
            let out = parse_simbad(&v, max);
            if out.is_empty() {
                vec![format!("absent — simbad carries no object: {}", needle)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — simbad HTTP {}", f.status_text())],
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
    let main_id = field("main_id")?;
    if main_id.is_empty() {
        return None;
    }
    let mut line = format!("url {}{}", ID_URL, urlencode(&main_id));
    line.push_str(&format!("\tmain_id: {}", main_id));
    if let Some(ra) = field("ra") {
        line.push_str(&format!("\tra: {}", ra));
    }
    if let Some(dec) = field("dec") {
        line.push_str(&format!("\tdec: {}", dec));
    }
    Some(line)
}

fn parse_simbad(v: &json::Json, max: usize) -> Vec<String> {
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

    const SAMPLE: &str = r#"{"metadata":[{"name":"main_id","description":"Main identifier for an object","datatype":"CHAR","arraysize":"*","ucd":"meta.id;meta.main","utype":"mango:MangoObject.identifier"},{"name":"ra","description":"Right ascension","datatype":"DOUBLE","unit":"deg","ucd":"pos.eq.ra;meta.main","utype":"mango:EpochPosition.longitude[CS.spaceSys=ICRS CT.epoch=J2000]"},{"name":"dec","description":"Declination","datatype":"DOUBLE","unit":"deg","ucd":"pos.eq.dec;meta.main","utype":"mango:EpochPosition.latitude[CS.spaceSys=ICRS CT.epoch=J2000]"}],"data":[["M31GC J004431+413005",11.130509988839998,41.50130294694],["M31N 2006-04a",10.805833333333334,41.28305555555556],["V* V500 And",10.610416666666666,42.26027777777778]]}"#;

    #[test]
    fn reads_metadata_and_rows_into_url_lines() {
        let v = json::parse(SAMPLE).unwrap();
        let out = parse_simbad(&v, 10);
        assert_eq!(out.len(), 3);
        assert!(out[0].starts_with(
            "url https://simbad.u-strasbg.fr/simbad/sim-id?Ident=M31GC%20J004431%2B413005"
        ));
        assert!(out[0].contains("\tmain_id: M31GC J004431+413005"));
        assert!(out[0].contains("\tra: 11.130509988839998"));
        assert!(out[0].contains("\tdec: 41.50130294694"));
        assert!(
            out[2].starts_with(
                "url https://simbad.u-strasbg.fr/simbad/sim-id?Ident=V%2A%20V500%20And"
            )
        );
    }

    #[test]
    fn caps_at_max() {
        let v = json::parse(SAMPLE).unwrap();
        assert_eq!(parse_simbad(&v, 2).len(), 2);
    }

    #[test]
    fn a_null_coordinate_is_omitted_never_written() {
        let body = r#"{"metadata":[{"name":"main_id"},{"name":"ra"},{"name":"dec"}],"data":[["NGC 224",null,41.269]]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_simbad(&v, 10),
            vec![
                "url https://simbad.u-strasbg.fr/simbad/sim-id?Ident=NGC%20224\tmain_id: NGC 224\tdec: 41.269"
                    .to_string()
            ]
        );
    }

    #[test]
    fn a_row_without_main_id_is_skipped() {
        let body = r#"{"metadata":[{"name":"main_id"},{"name":"ra"},{"name":"dec"}],"data":[["",1.0,2.0],["M31",3.0,4.0]]}"#;
        let v = json::parse(body).unwrap();
        let out = parse_simbad(&v, 10);
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("main_id: M31"));
    }
}
