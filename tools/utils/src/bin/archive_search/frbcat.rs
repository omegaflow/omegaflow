use crate::net::{get, urlencode};
use std::time::{SystemTime, UNIX_EPOCH};

const LIVE_CSV: &str = "https://docs.google.com/spreadsheets/d/1W27KNa6yJzYA_b8HLSz4hxtWEZQtxUhGTXfQjlXgpzY/gviz/tq?tqx=out:csv&sheet=Catalogue";
const RAW_CSV: &str = "https://raw.githubusercontent.com/HeRTA/FRBSTATS/main/catalogue.csv";

const COLUMNS: &[&str] = &["frb", "ra", "dec", "redshift"];

struct Row {
    frb: String,
    ra: String,
    dec: String,
    redshift: String,
}

pub fn frbcat_lines(query: &str, max: usize) -> Vec<String> {
    let body = match fetch_catalogue() {
        Ok(body) => body,
        Err(lines) => return lines,
    };
    let rows = parse_catalogue(&body);
    let needle = query.trim().to_lowercase();
    let hits: Vec<Row> = rows
        .into_iter()
        .filter(|row| matches_row(row, &needle))
        .collect();
    if hits.is_empty() {
        return vec![format!(
            "absent — the FRB catalogue carries no entry: {}",
            query
        )];
    }
    let full: Vec<String> = hits.iter().map(row_line).collect();
    let preview = max;
    let mut out: Vec<String> = full.iter().take(preview).cloned().collect();
    if let Some(path) = write_full(query, &full) {
        out.push(format!(
            "full: {} lines -> {}  [read the full file to its end — the previewed {} are not the result]",
            full.len(),
            path,
            out.len()
        ));
    }
    out
}

fn fetch_catalogue() -> Result<String, Vec<String>> {
    let mut http: Option<String> = None;
    for url in [LIVE_CSV, RAW_CSV] {
        if let Some(f) = get(url, &[], "60") {
            if f.status == Some(200) && !parse_catalogue(&f.body).is_empty() {
                return Ok(f.body);
            }
            http = Some(f.status_text());
        }
    }
    match http {
        Some(status) => Err(vec![format!(
            "pending — the FRB catalogue carries no rows beyond the header (FRBSTATS gviz + raw catalogue.csv; HTTP {})",
            status
        )]),
        None => Err(vec!["pending — no network".to_string()]),
    }
}

fn parse_catalogue(body: &str) -> Vec<Row> {
    let mut records = parse_csv(body).into_iter();
    let header = match records.next() {
        Some(h) => h,
        None => return Vec::new(),
    };
    let index: Vec<Option<usize>> = COLUMNS
        .iter()
        .map(|name| {
            header
                .iter()
                .position(|h| h.trim().eq_ignore_ascii_case(name))
        })
        .collect();
    let mut rows = Vec::new();
    for record in records {
        if record.iter().all(|f| f.trim().is_empty()) {
            continue;
        }
        let pick = |i: usize| -> String {
            match index.get(i).copied().flatten() {
                Some(pos) => match record.get(pos) {
                    Some(field) => field.clone(),
                    None => String::new(),
                },
                None => String::new(),
            }
        };
        rows.push(Row {
            frb: pick(0),
            ra: pick(1),
            dec: pick(2),
            redshift: pick(3),
        });
    }
    rows
}

fn parse_csv(body: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = body.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    field.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(c);
            }
            continue;
        }
        match c {
            '"' => in_quotes = true,
            ',' => row.push(std::mem::take(&mut field)),
            '\r' => {}
            '\n' => {
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
            }
            _ => field.push(c),
        }
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows
}

fn matches_row(row: &Row, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    [&row.frb, &row.ra, &row.dec, &row.redshift]
        .iter()
        .any(|f| f.to_lowercase().contains(needle))
}

fn row_line(row: &Row) -> String {
    let name = row.frb.trim();
    let url = if name.is_empty() {
        "https://github.com/HeRTA/FRBSTATS".to_string()
    } else {
        format!(
            "https://ui.adsabs.harvard.edu/search/q=%22{}%22",
            urlencode(name)
        )
    };
    let mut line = format!("url {}", url);
    for (key, value) in [
        ("frb", name),
        ("ra", row.ra.trim()),
        ("dec", row.dec.trim()),
        ("redshift", row.redshift.trim()),
    ] {
        if !value.is_empty() {
            line.push_str(&format!("\t{}: {}", key, value));
        }
    }
    line
}

fn write_full(query: &str, lines: &[String]) -> Option<String> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let slug: String = query
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .take(40)
        .collect();
    let path = std::env::temp_dir().join(format!("omegaflow_frbcat_{stamp}_{slug}.txt"));
    std::fs::write(&path, lines.join("\n")).ok()?;
    Some(path.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\"frb\",\"ra\",\"dec\",\"redshift\"\n\"FRB 19920428A\",\"15:03:00.000\",\"-05:12:00.00\",\"2.8319\"\n\"FRB 20010125A\",\"19:06:53.000\",\"-40:37:14.00\",\"0.7083\"\n";

    #[test]
    fn parses_the_header_and_rows() {
        let rows = parse_catalogue(SAMPLE);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1].frb, "FRB 20010125A");
        assert_eq!(rows[1].ra, "19:06:53.000");
        assert_eq!(rows[1].dec, "-40:37:14.00");
        assert_eq!(rows[1].redshift, "0.7083");
    }

    #[test]
    fn csv_keeps_commas_inside_quotes() {
        let rows = parse_csv("\"a\",\"b,c\",\"d\"\n");
        assert_eq!(
            rows,
            vec![vec!["a".to_string(), "b,c".to_string(), "d".to_string()]]
        );
    }

    #[test]
    fn csv_unescapes_doubled_quotes() {
        let rows = parse_csv("\"a\"\"b\",\"c\"\n");
        assert_eq!(rows, vec![vec!["a\"b".to_string(), "c".to_string()]]);
    }

    #[test]
    fn matches_the_name_case_insensitively() {
        let rows = parse_catalogue(SAMPLE);
        assert!(matches_row(&rows[0], "frb 19920428a"));
        assert!(!matches_row(&rows[0], "20010125a"));
    }

    #[test]
    fn matches_coordinates_and_redshift() {
        let rows = parse_catalogue(SAMPLE);
        assert!(matches_row(&rows[0], "-05:12"));
        assert!(matches_row(&rows[1], "0.7083"));
        assert!(!matches_row(&rows[1], "2.8319"));
    }

    #[test]
    fn row_line_carries_the_url_and_the_measured_fields() {
        let rows = parse_catalogue(SAMPLE);
        assert_eq!(
            row_line(&rows[0]),
            "url https://ui.adsabs.harvard.edu/search/q=%22FRB%2019920428A%22\tfrb: FRB 19920428A\tra: 15:03:00.000\tdec: -05:12:00.00\tredshift: 2.8319"
                .to_string()
        );
    }

    #[test]
    fn empty_optional_fields_are_omitted() {
        let row = Row {
            frb: "FRB 20010305A".to_string(),
            ra: "4:57:20".to_string(),
            dec: "-52:36:25".to_string(),
            redshift: String::new(),
        };
        assert_eq!(
            row_line(&row),
            "url https://ui.adsabs.harvard.edu/search/q=%22FRB%2020010305A%22\tfrb: FRB 20010305A\tra: 4:57:20\tdec: -52:36:25"
                .to_string()
        );
    }

    #[test]
    fn a_header_only_body_carries_no_rows() {
        assert!(parse_catalogue("\"frb\",\"ra\",\"dec\",\"redshift\"\n").is_empty());
    }
}
