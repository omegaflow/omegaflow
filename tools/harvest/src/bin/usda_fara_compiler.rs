use omegaflow::archivar::quaoar_occlt::{ZipEntry, zip_entries, zip_extract};
use std::process::Command;

const NETLOC: &str = "ers.usda.gov";
const ZIP_URL: &str = "https://www.ers.usda.gov/media/5627/2019-large-retailer-access-map-lram-formerly-known-as-the-food-access-research-atlas-fara-data.zip";
const KEY_FIELDS: [&str; 4] = ["CensusTract", "State", "County", "Urban"];

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("900")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn split_csv(line: &str) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for c in line.chars() {
        match c {
            '"' if in_quotes => in_quotes = false,
            '"' => in_quotes = true,
            ',' if !in_quotes => {
                fields.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    fields.push(cur.trim().to_string());
    fields
}

fn cell_kind(cell: &str) -> &'static str {
    let t = cell.trim();
    if t.is_empty() || t.eq_ignore_ascii_case("null") {
        "null"
    } else if t.parse::<f64>().is_ok() {
        "numeric"
    } else {
        "text"
    }
}

fn find_data_member(entries: &[ZipEntry]) -> Option<&ZipEntry> {
    entries
        .iter()
        .filter(|e| e.name.to_ascii_lowercase().ends_with(".csv"))
        .max_by_key(|e| e.comp_size)
}

struct Schema {
    fields: Vec<String>,
    sample: Vec<String>,
    rows: usize,
    numeric_cells: usize,
    null_cells: usize,
    text_cells: usize,
}

fn parse_schema(body: &str) -> Option<Schema> {
    let mut lines = body.lines();
    let header = lines.find(|l| !l.trim().is_empty())?;
    let fields = split_csv(header);
    if fields.is_empty() {
        return None;
    }
    let mut sample: Vec<String> = Vec::new();
    let mut rows = 0usize;
    let mut numeric_cells = 0usize;
    let mut null_cells = 0usize;
    let mut text_cells = 0usize;
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cells = split_csv(line);
        if sample.is_empty() {
            sample = cells.clone();
        }
        rows += 1;
        for cell in &cells {
            match cell_kind(cell) {
                "numeric" => numeric_cells += 1,
                "null" => null_cells += 1,
                _ => text_cells += 1,
            }
        }
    }
    Some(Schema {
        fields,
        sample,
        rows,
        numeric_cells,
        null_cells,
        text_cells,
    })
}

fn name_tokens(name: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();
    let mut prev_lower = false;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            if prev_lower && c.is_ascii_uppercase() && !token.is_empty() {
                tokens.push(token.to_ascii_lowercase());
                token.clear();
            }
            prev_lower = c.is_ascii_lowercase();
            token.push(c);
        } else {
            if !token.is_empty() {
                tokens.push(token.to_ascii_lowercase());
                token.clear();
            }
            prev_lower = false;
        }
    }
    if !token.is_empty() {
        tokens.push(token.to_ascii_lowercase());
    }
    tokens
}

fn has_coordinate_field(fields: &[String]) -> bool {
    fields.iter().any(|f| {
        name_tokens(f)
            .iter()
            .any(|t| matches!(t.as_str(), "lat" | "latitude" | "lon" | "longitude"))
    })
}

fn has_time_field(fields: &[String]) -> bool {
    fields.iter().any(|f| {
        name_tokens(f).iter().any(|t| {
            matches!(
                t.as_str(),
                "year" | "date" | "epoch" | "time" | "month" | "day"
            )
        })
    })
}

fn field_index(fields: &[String], name: &str) -> Option<usize> {
    fields.iter().position(|f| f == name)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let (zip, source) = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => (b, path),
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => {
            let url = match arg_value(&args, "--url") {
                Some(u) => u,
                None => ZIP_URL.to_string(),
            };
            match fetch(&url) {
                Some(b) => (b, url),
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            }
        }
    };
    let entries = match zip_entries(&zip) {
        Some(e) => e,
        None => {
            eprintln!("{source}: central directory void — the zip stays unread (0 honored)");
            std::process::exit(1);
        }
    };
    let member = match find_data_member(&entries) {
        Some(m) => m,
        None => {
            eprintln!("{source}: no .csv member in the zip — the table stays unread (0 honored)");
            std::process::exit(1);
        }
    };
    let raw = match zip_extract(&zip, member) {
        Some(r) => r,
        None => {
            eprintln!("{}: entry extract void (0 honored)", member.name);
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&raw);
    let schema = match parse_schema(&text) {
        Some(s) => s,
        None => {
            eprintln!(
                "{}: header void — the table stays unread (0 honored)",
                member.name
            );
            std::process::exit(1);
        }
    };
    if schema.rows == 0 {
        eprintln!(
            "{}: header only, no data rows — the table stays unread (0 honored)",
            member.name
        );
        std::process::exit(1);
    }

    eprintln!("{source} -> {} ({} B)", member.name, raw.len());
    eprintln!(
        "{} fields, {} data rows ({} numeric cells, {} null cells, {} text cells)",
        schema.fields.len(),
        schema.rows,
        schema.numeric_cells,
        schema.null_cells,
        schema.text_cells
    );
    eprintln!("key fields:");
    for key in KEY_FIELDS {
        match field_index(&schema.fields, key) {
            Some(i) => {
                let cell = match schema.sample.get(i) {
                    Some(v) => v.as_str(),
                    None => "<absent>",
                };
                eprintln!("  [{i}] {key} = {cell}");
            }
            None => eprintln!("  {key}: absent"),
        }
    }
    eprintln!("fields:");
    for (i, field) in schema.fields.iter().enumerate() {
        let cell = match schema.sample.get(i) {
            Some(v) => v.as_str(),
            None => "",
        };
        eprintln!("  [{i}] {field}: {cell} ({})", cell_kind(cell));
    }
    eprintln!(
        "coordinate field present: {} | time field present: {}",
        has_coordinate_field(&schema.fields),
        has_time_field(&schema.fields)
    );
    eprintln!(
        "STOP — static geography table: the geography key is CensusTract (11-digit FIPS) with State/County/Urban; no latitude/longitude column and no year/date/epoch column are present."
    );
    eprintln!(
        "missing design: no epoch value stream can be emitted (the ReadMe names a single April 2021 release, no time axis), and no (lat, lon, value) catalog can be emitted without a registered census-tract centroid catalog arm (none exists). No epoch is invented. 0 honored — the schema dump is the deliverable of this atom."
    );
    if ci_mode {
        eprintln!(
            "{NETLOC}: --ci-mode set, but there is no epoch value stream to manifest; no upload_release (0 honored)."
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(name: &str, comp_size: u64) -> ZipEntry {
        ZipEntry {
            name: name.to_string(),
            method: 0,
            comp_size,
            local_offset: 0,
        }
    }

    #[test]
    fn split_csv_keeps_quoted_commas() {
        let fields = split_csv("a,\"b,c\",d");
        assert_eq!(fields, vec!["a", "b,c", "d"]);
    }

    #[test]
    fn cell_kind_reads_numeric_null_and_text() {
        assert_eq!(cell_kind("11.3"), "numeric");
        assert_eq!(cell_kind("NULL"), "null");
        assert_eq!(cell_kind(""), "null");
        assert_eq!(cell_kind("Alabama"), "text");
    }

    #[test]
    fn parse_schema_reads_header_sample_and_counts() {
        let body = "CensusTract,State,Urban,lapophalfshare\n\
 1001020100,Alabama,1,24.42\n\
 1001020200,Alabama,1,NULL\n";
        let s = parse_schema(body).unwrap();
        assert_eq!(s.fields.len(), 4);
        assert_eq!(s.rows, 2);
        assert_eq!(s.sample[0], "1001020100");
        assert_eq!(s.numeric_cells, 5);
        assert_eq!(s.null_cells, 1);
        assert_eq!(s.text_cells, 2);
        assert_eq!(field_index(&s.fields, "lapophalfshare"), Some(3));
    }

    #[test]
    fn parse_schema_refuses_an_empty_body() {
        assert!(parse_schema("").is_none());
        assert!(parse_schema("\n\n").is_none());
    }

    #[test]
    fn find_data_member_picks_the_largest_csv() {
        let entries = vec![
            entry("ReadMe.csv", 512),
            entry("VariableLookup.csv", 21_531),
            entry("Food Access Research Atlas.csv", 47_053_488),
        ];
        let member = find_data_member(&entries).unwrap();
        assert_eq!(member.name, "Food Access Research Atlas.csv");
    }

    #[test]
    fn find_data_member_refuses_a_zip_without_a_csv() {
        let entries = vec![entry("data.xlsx", 1_000_000), entry("ReadMe.txt", 10)];
        assert!(find_data_member(&entries).is_none());
    }

    #[test]
    fn coordinate_and_time_names_are_measured() {
        let fields: Vec<String> = [
            "LILATracts_1And10",
            "CensusTract",
            "State",
            "PovertyRate",
            "Update",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert!(!has_coordinate_field(&fields));
        assert!(!has_time_field(&fields));
        let with_coord: Vec<String> = ["Latitude", "lon"].iter().map(|s| s.to_string()).collect();
        assert!(has_coordinate_field(&with_coord));
        let with_time: Vec<String> = ["Year", "ObservationDate"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert!(has_time_field(&with_time));
    }

    #[test]
    fn name_tokens_split_on_camel_and_separator() {
        assert_eq!(
            name_tokens("LILATracts_1And10"),
            vec!["lilatracts", "1and10"]
        );
        assert_eq!(name_tokens("ObservationDate"), vec!["observation", "date"]);
    }
}
