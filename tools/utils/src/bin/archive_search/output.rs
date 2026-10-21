use std::io::Write;

const CSV_HEADER: &str = "mode,url,title,doi,year,authors,venue,pdf,via";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Text,
    Jsonl,
    Csv,
    Bibtex,
    Ris,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    mode: String,
    url: String,
    fields: Vec<(String, String)>,
    via: Vec<String>,
}

impl Hit {
    fn field(&self, names: &[&str]) -> Option<&str> {
        self.fields
            .iter()
            .find(|(key, value)| names.contains(&key.as_str()) && !value.is_empty())
            .map(|(_, value)| value.as_str())
    }
}

pub struct Sink {
    format: Format,
    allow_empty: bool,
    hits: usize,
    csv_header: bool,
}

impl Sink {
    pub fn new(format: Format, allow_empty: bool) -> Self {
        Sink {
            format,
            allow_empty,
            hits: 0,
            csv_header: false,
        }
    }

    pub fn emit(&mut self, lines: &[String], default_mode: &str) {
        if self.format == Format::Text {
            let mut stdout = std::io::stdout();
            for line in lines {
                let _ = writeln!(stdout, "{line}");
            }
            return;
        }
        for line in lines {
            let Some(hit) = parse_hit(line, default_mode) else {
                continue;
            };
            self.hits += 1;
            match self.format {
                Format::Text => {}
                Format::Jsonl => {
                    let mut stdout = std::io::stdout();
                    let _ = writeln!(stdout, "{}", json_line(&hit));
                }
                Format::Csv => {
                    self.ensure_csv_header();
                    let mut stdout = std::io::stdout();
                    let _ = writeln!(stdout, "{}", csv_line(&hit));
                }
                Format::Bibtex => {
                    let mut stdout = std::io::stdout();
                    let _ = writeln!(stdout, "{}", bibtex_entry(&hit));
                }
                Format::Ris => {
                    let mut stdout = std::io::stdout();
                    let _ = writeln!(stdout, "{}", ris_entry(&hit));
                }
            }
        }
    }

    fn ensure_csv_header(&mut self) {
        if self.csv_header {
            return;
        }
        self.csv_header = true;
        let mut stdout = std::io::stdout();
        let _ = writeln!(stdout, "{CSV_HEADER}");
    }

    pub fn finish(&mut self) -> i32 {
        if self.format == Format::Csv {
            self.ensure_csv_header();
        }
        let _ = std::io::stdout().flush();
        exit_code(self.hits, self.allow_empty)
    }
}

pub fn exit_code(hits: usize, allow_empty: bool) -> i32 {
    if hits > 0 || allow_empty {
        0
    } else {
        3
    }
}

fn parse_hit(line: &str, default_mode: &str) -> Option<Hit> {
    let (mode, body) = split_mode(line, default_mode);
    if !is_hit(body) {
        return None;
    }
    let mut url = String::new();
    let mut fields: Vec<(String, String)> = Vec::new();
    let mut via: Vec<String> = Vec::new();
    for segment in body.split('\t') {
        if let Some(value) = segment.strip_prefix("url ") {
            url = value.trim().to_string();
            continue;
        }
        let Some((key, value)) = segment.split_once(": ") else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() {
            continue;
        }
        if key == "via" {
            for token in value.split(',') {
                if let Some(first) = token.split_whitespace().next() {
                    via.push(first.to_string());
                }
            }
        } else {
            fields.push((key.to_string(), value.to_string()));
        }
    }
    Some(Hit {
        mode: mode.to_string(),
        url,
        fields,
        via,
    })
}

fn split_mode<'a>(line: &'a str, default_mode: &'a str) -> (&'a str, &'a str) {
    if let Some(rest) = line.strip_prefix('[') {
        if let Some(end) = rest.find("] ") {
            let mode = &rest[..end];
            if !mode.is_empty() {
                return (mode, &rest[end + 2..]);
            }
        }
    }
    (default_mode, line)
}

fn is_hit(body: &str) -> bool {
    body.starts_with("url ")
        || body.starts_with("doi: ")
        || body.starts_with("title: ")
        || body.contains("\tdoi: ")
        || body.contains("\ttitle: ")
}

fn json_line(hit: &Hit) -> String {
    let fields: Vec<String> = hit
        .fields
        .iter()
        .map(|(key, value)| {
            format!(
                "\"{}\":\"{}\"",
                crate::net::json_escape(key),
                crate::net::json_escape(value)
            )
        })
        .collect();
    let via: Vec<String> = hit
        .via
        .iter()
        .map(|value| format!("\"{}\"", crate::net::json_escape(value)))
        .collect();
    format!(
        "{{\"mode\":\"{}\",\"url\":\"{}\",\"fields\":{{{}}},\"via\":[{}]}}",
        crate::net::json_escape(&hit.mode),
        crate::net::json_escape(&hit.url),
        fields.join(","),
        via.join(",")
    )
}

fn csv_line(hit: &Hit) -> String {
    let via = hit.via.join(",");
    let row = [
        hit.mode.as_str(),
        hit.url.as_str(),
        hit.field(&["title"]).unwrap_or(""),
        hit.field(&["doi"]).unwrap_or(""),
        hit.field(&["year"]).unwrap_or(""),
        hit.field(&["authors", "author"]).unwrap_or(""),
        hit.field(&["venue", "journal"]).unwrap_or(""),
        hit.field(&["pdf", "oa_pdf"]).unwrap_or(""),
        via.as_str(),
    ];
    row.iter()
        .map(|value| csv_quote(value))
        .collect::<Vec<String>>()
        .join(",")
}

fn csv_quote(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn bib_key(hit: &Hit) -> String {
    if let Some(doi) = hit.field(&["doi"]) {
        let key: String = doi
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | ':' | '/') {
                    c
                } else {
                    '-'
                }
            })
            .collect();
        if !key.is_empty() {
            return key;
        }
    }
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in hit.url.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("url-{hash:016x}")
}

fn bib_escape(value: &str) -> String {
    let mut out = String::new();
    for c in value.chars() {
        match c {
            '\\' => out.push_str("\\textbackslash{}"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '&' | '%' | '#' | '_' | '$' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

fn bibtex_entry(hit: &Hit) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(title) = hit.field(&["title"]) {
        parts.push(format!("  title = {{{}}},", bib_escape(title)));
    }
    if let Some(authors) = hit.field(&["authors", "author"]) {
        parts.push(format!("  author = {{{}}},", bib_escape(authors)));
    }
    if let Some(venue) = hit.field(&["venue", "journal"]) {
        parts.push(format!("  journal = {{{}}},", bib_escape(venue)));
    }
    if let Some(year) = hit.field(&["year"]) {
        parts.push(format!("  year = {{{}}},", bib_escape(year)));
    }
    if let Some(doi) = hit.field(&["doi"]) {
        parts.push(format!("  doi = {{{}}},", bib_escape(doi)));
    }
    let link = if hit.url.is_empty() {
        hit.field(&["pdf", "oa_pdf"])
    } else {
        Some(hit.url.as_str())
    };
    if let Some(url) = link {
        parts.push(format!("  url = {{{}}},", bib_escape(url)));
    }
    let mut out = format!("@misc{{{},\n", bib_key(hit));
    for part in parts {
        out.push_str(&part);
        out.push('\n');
    }
    out.push('}');
    out
}

fn ris_entry(hit: &Hit) -> String {
    let mut out = String::from("TY  - GEN\n");
    if let Some(title) = hit.field(&["title"]) {
        out.push_str(&format!("TI  - {title}\n"));
    }
    if let Some(authors) = hit.field(&["authors", "author"]) {
        out.push_str(&format!("AU  - {authors}\n"));
    }
    if let Some(year) = hit.field(&["year"]) {
        out.push_str(&format!("PY  - {year}\n"));
    }
    if let Some(venue) = hit.field(&["venue", "journal"]) {
        out.push_str(&format!("JO  - {venue}\n"));
    }
    if let Some(doi) = hit.field(&["doi"]) {
        out.push_str(&format!("DO  - {doi}\n"));
    }
    if !hit.url.is_empty() {
        out.push_str(&format!("UR  - {}\n", hit.url));
    }
    if let Some(pdf) = hit.field(&["pdf", "oa_pdf"]) {
        out.push_str(&format!("L1  - {pdf}\n"));
    }
    out.push_str("ER  -");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jsonl_carries_only_hits_and_keeps_the_mode_and_via() {
        let lines = vec![
            "[openalex] url https://openalex.org/W1\tdoi: 10.1/abc\ttitle: Solar wind\tvia: 4get (openalex 402)".to_string(),
            "[openalex] pending — no network".to_string(),
            "[openalex] absent — carries no entry".to_string(),
            "end: cursor".to_string(),
            "total: 0".to_string(),
        ];
        let hits: Vec<Hit> = lines
            .iter()
            .filter_map(|l| parse_hit(l, "openalex"))
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(
            json_line(&hits[0]),
            "{\"mode\":\"openalex\",\"url\":\"https://openalex.org/W1\",\"fields\":{\"doi\":\"10.1/abc\",\"title\":\"Solar wind\"},\"via\":[\"4get\"]}"
        );
    }

    #[test]
    fn a_line_without_a_prefix_takes_the_default_mode() {
        let hit = parse_hit("url https://example.org/a\ttitle: A study", "crossref")
            .expect("hit");
        assert_eq!(hit.mode, "crossref");
        assert_eq!(hit.field(&["title"]), Some("A study"));
    }

    #[test]
    fn csv_quotes_commas_quotes_and_newlines() {
        assert_eq!(csv_quote("plain"), "plain");
        assert_eq!(csv_quote("Doe, Jane"), "\"Doe, Jane\"");
        assert_eq!(csv_quote("say \"hi\""), "\"say \"\"hi\"\"\"");
        assert_eq!(csv_quote("a\nb"), "\"a\nb\"");
        let hit = parse_hit(
            "url https://example.org/a\ttitle: A, \"quoted\" study\tauthors: Doe, Jane\tyear: 2020",
            "openalex",
        )
        .expect("hit");
        assert_eq!(
            csv_line(&hit),
            "openalex,https://example.org/a,\"A, \"\"quoted\"\" study\",,2020,\"Doe, Jane\",,,"
        );
        assert_eq!(
            CSV_HEADER,
            "mode,url,title,doi,year,authors,venue,pdf,via"
        );
    }

    #[test]
    fn bibtex_uses_the_doi_as_key_and_omits_absent_fields() {
        let hit = parse_hit(
            "url https://doi.org/10.1/abc\ttitle: A study\tauthors: Doe, Jane\tyear: 2020\tdoi: 10.1/abc",
            "openalex",
        )
        .expect("hit");
        assert_eq!(
            bibtex_entry(&hit),
            "@misc{10.1/abc,\n  title = {A study},\n  author = {Doe, Jane},\n  year = {2020},\n  doi = {10.1/abc},\n  url = {https://doi.org/10.1/abc},\n}"
        );
        let bare = parse_hit("url https://example.org/b\ttitle: B", "openalex").expect("hit");
        let entry = bibtex_entry(&bare);
        assert!(entry.starts_with("@misc{url-"));
        assert!(entry.contains("  title = {B},"));
        assert!(!entry.contains("doi"));
        assert!(!entry.contains("author"));
    }

    #[test]
    fn bibtex_escapes_special_characters() {
        let hit = parse_hit(
            "url https://example.org/a\ttitle: A & B_100% {x}",
            "openalex",
        )
        .expect("hit");
        assert!(bibtex_entry(&hit).contains("title = {A \\& B\\_100\\% \\{x\\}}"));
    }

    #[test]
    fn ris_carries_only_present_fields() {
        let hit = parse_hit(
            "url https://example.org/a\ttitle: A study\tauthors: Doe, Jane\tyear: 2020\tdoi: 10.1/abc",
            "openalex",
        )
        .expect("hit");
        assert_eq!(
            ris_entry(&hit),
            "TY  - GEN\nTI  - A study\nAU  - Doe, Jane\nPY  - 2020\nDO  - 10.1/abc\nUR  - https://example.org/a\nER  -"
        );
        let bare = parse_hit("url https://example.org/b\tdoi: 10.1/x", "openalex").expect("hit");
        assert_eq!(
            ris_entry(&bare),
            "TY  - GEN\nDO  - 10.1/x\nUR  - https://example.org/b\nER  -"
        );
    }

    #[test]
    fn exit_classification_separates_hits_from_empty_runs() {
        assert_eq!(exit_code(2, false), 0);
        assert_eq!(exit_code(0, false), 3);
        assert_eq!(exit_code(0, true), 0);
        assert_eq!(exit_code(1, true), 0);
    }

    #[test]
    fn csv_and_bibtex_read_journal_and_oa_pdf_aliases() {
        let hit = parse_hit(
            "url https://example.org/a\ttitle: A\tjournal: J. Test\toa_pdf: https://example.org/a.pdf",
            "openalex",
        )
        .expect("hit");
        assert_eq!(
            csv_line(&hit),
            "openalex,https://example.org/a,A,,,,J. Test,https://example.org/a.pdf,"
        );
        assert!(bibtex_entry(&hit).contains("journal = {J. Test}"));
    }
}
