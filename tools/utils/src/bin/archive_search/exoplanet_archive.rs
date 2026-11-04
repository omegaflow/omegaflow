use crate::json;
use crate::net::{get, urlencode};

pub fn exoplanet_archive_lines(query: &str, max: usize) -> Vec<String> {
    let safe = query.replace('\'', "''");
    let adql = format!(
        "SELECT TOP {} pl_name, hostname, disc_year FROM pscomppars WHERE pl_name LIKE '%{}%' OR hostname LIKE '%{}%'",
        max, safe, safe
    );
    let url = format!(
        "https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query={}&format=json",
        urlencode(&adql)
    );
    match get(&url, &[], "60") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec![
                    "pending — the exoplanet-archive response carries no JSON".to_string(),
                ];
            };
            let out = parse_planets(&v, max);
            if out.is_empty() {
                vec![format!(
                    "absent — exoplanet-archive carries no planet: {}",
                    query
                )]
            } else {
                out
            }
        }
        Some(f) => vec![format!(
            "pending — exoplanet-archive HTTP {}",
            f.status_text()
        )],
        None => vec!["pending — no network".to_string()],
    }
}

fn planet_line(row: &json::Json) -> Option<String> {
    let pl_name = row.get("pl_name").and_then(|v| v.as_scalar_string())?;
    let url = format!(
        "https://exoplanetarchive.ipac.caltech.edu/overview/{}",
        pl_name
    );
    let mut parts = vec![format!("url {}", url), format!("pl_name: {}", pl_name)];
    if let Some(host) = row.get("hostname").and_then(|v| v.as_scalar_string()) {
        parts.push(format!("hostname: {}", host));
    }
    if let Some(year) = row.get("disc_year").and_then(|v| v.as_scalar_string()) {
        parts.push(format!("year: {}", year));
    }
    Some(parts.join("\t"))
}

fn parse_planets(v: &json::Json, max: usize) -> Vec<String> {
    let Some(rows) = v.as_arr() else {
        return Vec::new();
    };
    rows.iter().take(max).filter_map(planet_line).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> json::Json {
        json::parse(
            r#"[{"pl_name":"55 Cnc b","hostname":"55 Cnc","disc_year":1996},{"pl_name":"55 Cnc c","hostname":"55 Cnc","disc_year":2004}]"#,
        )
        .unwrap()
    }

    #[test]
    fn reads_planet_rows() {
        let v = fixture();
        assert_eq!(
            parse_planets(&v, 10),
            vec![
                "url https://exoplanetarchive.ipac.caltech.edu/overview/55 Cnc b\tpl_name: 55 Cnc b\thostname: 55 Cnc\tyear: 1996".to_string(),
                "url https://exoplanetarchive.ipac.caltech.edu/overview/55 Cnc c\tpl_name: 55 Cnc c\thostname: 55 Cnc\tyear: 2004".to_string(),
            ]
        );
    }

    #[test]
    fn caps_at_max() {
        let v = fixture();
        assert_eq!(parse_planets(&v, 1).len(), 1);
    }

    #[test]
    fn a_non_array_carries_no_planet() {
        let v = json::parse(r#"{"error":"no rows"}"#).unwrap();
        assert!(parse_planets(&v, 10).is_empty());
    }
}
