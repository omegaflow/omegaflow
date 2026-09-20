use crate::json;
use crate::net::{get, urlencode};

const ENDPOINT: &str = "https://api.materialsproject.org/materials/summary/search";

pub fn materialsproject_lines(query: &str, max: usize) -> Vec<String> {
    let Some(key) = crate::token::secret("MP_API_KEY") else {
        return vec![
            "pending — MP_API_KEY absent from .secrets.local/.env (Materials Project dashboard key)"
                .to_string(),
        ];
    };
    let url = format!("{}?q={}&_limit={}", ENDPOINT, urlencode(query), max);
    let header = format!("X-API-KEY: {}", key);
    match get(&url, &["-H", &header], "40") {
        Some(f) if f.status == Some(200) => {
            let out = parse_materialsproject(&f.body);
            if out.is_empty() {
                vec![format!("absent — materialsproject carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) if f.status == Some(401) || f.status == Some(403) => vec![
            "pending — materialsproject refuses the key (HTTP 401/403); check MP_API_KEY".to_string(),
        ],
        Some(f) => vec![format!("pending — materialsproject HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn field(v: &json::Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_materialsproject(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(data) = v.get("data").and_then(|d| d.as_arr()) else {
        return out;
    };
    for material in data {
        let Some(id) = field(material, "material_id") else {
            continue;
        };
        let mut line = format!("url https://next-gen.materialsproject.org/materials/{}", id);
        if let Some(formula) = field(material, "formula_pretty") {
            line.push_str(&format!("\tformula: {}", formula));
        }
        if let Some(gap) = material.get("band_gap").and_then(|g| g.as_scalar_string()) {
            line.push_str(&format!("\tband_gap: {}", gap));
        }
        if let Some(json::Json::Bool(stable)) = material.get("is_stable") {
            line.push_str(&format!("\tis_stable: {}", stable));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_summary_fields() {
        let body = r#"{"data":[{"material_id":"mp-149","formula_pretty":"Si","band_gap":0.61,"is_stable":true}]}"#;
        assert_eq!(
            parse_materialsproject(body),
            vec!["url https://next-gen.materialsproject.org/materials/mp-149\tformula: Si\tband_gap: 0.61\tis_stable: true".to_string()]
        );
    }

    #[test]
    fn a_material_without_an_id_carries_nothing() {
        assert!(parse_materialsproject(r#"{"data":[{"formula_pretty":"Si"}]}"#).is_empty());
    }
}
