use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://gea.esac.esa.int/tap-server/tap/sync";
const SOURCE_URL: &str = "https://gea.esac.esa.int/archive/";

pub fn gaia_lines(query: &str, max: usize) -> Vec<String> {
    let needle = query.trim();
    if needle.is_empty() {
        return vec!["pending — no gaia query".to_string()];
    }
    let adql = if needle.chars().all(|c| c.is_ascii_digit()) {
        format!(
            "SELECT TOP {} source_id, ra, dec, phot_g_mean_mag FROM gaiadr3.gaia_source WHERE source_id = {}",
            max, needle
        )
    } else {
        let escaped = needle.replace('\'', "''");
        format!(
            "SELECT TOP {} source_id, ra, dec, phot_g_mean_mag FROM gaiadr3.gaia_source WHERE designation LIKE '%{}%'",
            max, escaped
        )
    };
    let url = format!(
        "{}?REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY={}",
        ENDPOINT,
        urlencode(&adql)
    );
    match get(&url, &[], "90") {
        Some(f) if f.status == Some(200) => {
            let Some(v) = json::parse(&f.body) else {
                return vec!["pending — the gaia response carries no JSON".to_string()];
            };
            let out = parse_gaia(&v, max);
            if out.is_empty() {
                vec![format!("absent — gaia carries no source: {}", needle)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — gaia HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn cell_str(cell: &json::Json) -> Option<String> {
    cell.as_scalar_string()
}

fn row_line(row: &json::Json, columns: &[&str]) -> Option<String> {
    let cells = row.as_arr()?;
    let field = |name: &str| -> Option<String> {
        let i = columns.iter().position(|c| *c == name)?;
        cells.get(i).and_then(cell_str)
    };
    let source_id = field("source_id")?;
    if source_id.is_empty() {
        return None;
    }
    let mut line = format!("url {} {}", SOURCE_URL, source_id);
    line.push_str(&format!("\tsource_id: {}", source_id));
    if let Some(ra) = field("ra") {
        line.push_str(&format!("\tra: {}", ra));
    }
    if let Some(dec) = field("dec") {
        line.push_str(&format!("\tdec: {}", dec));
    }
    if let Some(g_mag) = field("phot_g_mean_mag") {
        line.push_str(&format!("\tg_mag: {}", g_mag));
    }
    Some(line)
}

fn parse_gaia(v: &json::Json, max: usize) -> Vec<String> {
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

    const SAMPLE: &str = r#"{"metadata":[{"name":"source_id","datatype":"long"},{"name":"ra","datatype":"double"},{"name":"dec","datatype":"double"},{"name":"phot_g_mean_mag","datatype":"float"}],"data":[[137332953680255488,45.000006268957925,35.26727307493361,19.189445],[4294967296,10.5,-2.25,12.0]]}"#;

    #[test]
    fn reads_metadata_and_rows_into_url_lines() {
        let v = json::parse(SAMPLE).unwrap();
        let out = parse_gaia(&v, 10);
        assert_eq!(out.len(), 2);
        assert!(out[0].starts_with("url https://gea.esac.esa.int/archive/ 137332953680255488"));
        assert!(out[0].contains("\tsource_id: 137332953680255488"));
        assert!(out[0].contains("\tra: 45.000006268957925"));
        assert!(out[0].contains("\tdec: 35.26727307493361"));
        assert!(out[0].contains("\tg_mag: 19.189445"));
        assert_eq!(
            out[1],
            "url https://gea.esac.esa.int/archive/ 4294967296\tsource_id: 4294967296\tra: 10.5\tdec: -2.25\tg_mag: 12"
                .to_string()
        );
    }

    #[test]
    fn caps_at_max() {
        let v = json::parse(SAMPLE).unwrap();
        assert_eq!(parse_gaia(&v, 1).len(), 1);
    }

    #[test]
    fn a_null_magnitude_is_omitted_and_a_row_without_source_id_is_skipped() {
        let body = r#"{"metadata":[{"name":"source_id"},{"name":"ra"},{"name":"dec"},{"name":"phot_g_mean_mag"}],"data":[[123,1.0,2.0,null],["",3.0,4.0,5.0]]}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            parse_gaia(&v, 10),
            vec![
                "url https://gea.esac.esa.int/archive/ 123\tsource_id: 123\tra: 1\tdec: 2"
                    .to_string()
            ]
        );
    }
}
