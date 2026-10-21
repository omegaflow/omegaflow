use crate::cache;
use crate::json;
use crate::net::{get, urlencode};

pub const DEFAULT_LIMIT: usize = 5;

const CACHE_MODE: &str = "fulltext";
const CACHE_MAX: usize = 1;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Texts {
    pub pdf: Option<String>,
    pub fulltext: Option<String>,
}

impl Texts {
    fn is_empty(&self) -> bool {
        self.pdf.is_none() && self.fulltext.is_none()
    }

    fn merge(&mut self, other: Texts) {
        if self.pdf.is_none() {
            self.pdf = other.pdf;
        }
        if self.fulltext.is_none() {
            self.fulltext = other.fulltext;
        }
    }
}

enum Cached {
    Found(Texts),
    Absent,
}

pub fn run_budget(enabled: bool, lines: &mut [String], limit: usize, used: &mut usize) {
    run_with_budget(enabled, lines, limit, used, resolve);
}

#[cfg(test)]
pub fn run_with<F>(enabled: bool, lines: &mut [String], limit: usize, resolve: F)
where
    F: FnMut(&str) -> Option<Texts>,
{
    let mut used = 0usize;
    run_with_budget(enabled, lines, limit, &mut used, resolve);
}

pub fn run_with_budget<F>(
    enabled: bool,
    lines: &mut [String],
    limit: usize,
    used: &mut usize,
    mut resolve: F,
) where
    F: FnMut(&str) -> Option<Texts>,
{
    if !enabled || limit == 0 {
        return;
    }
    for line in lines.iter_mut() {
        if *used >= limit {
            break;
        }
        let Some(doi) = doi_of(line) else {
            continue;
        };
        *used += 1;
        let Some(texts) = resolve(&doi) else {
            continue;
        };
        if texts.is_empty() {
            continue;
        }
        annotate(line, &texts);
    }
}

pub fn doi_of(line: &str) -> Option<String> {
    for segment in line.split('\t') {
        let Some(value) = segment.strip_prefix("doi: ") else {
            continue;
        };
        if let Some(doi) = clean_doi(value) {
            return Some(doi);
        }
    }
    None
}

fn clean_doi(value: &str) -> Option<String> {
    let doi = value
        .trim()
        .trim_start_matches("https://doi.org/")
        .trim_start_matches("http://doi.org/")
        .trim_start_matches("https://dx.doi.org/")
        .trim_start_matches("http://dx.doi.org/")
        .trim_start_matches("doi:")
        .trim()
        .trim_end_matches(['.', ',', ';', ')', ']', '}']);
    if !doi.starts_with("10.") || !doi.contains('/') {
        return None;
    }
    Some(doi.to_ascii_lowercase())
}

pub fn annotate(line: &mut String, texts: &Texts) {
    if let Some(pdf) = &texts.pdf {
        if !has_field(line, "pdf") {
            line.push_str("\tpdf: ");
            line.push_str(pdf);
        }
    }
    if let Some(fulltext) = &texts.fulltext {
        if !has_field(line, "fulltext") {
            line.push_str("\tfulltext: ");
            line.push_str(fulltext);
        }
    }
}

fn has_field(line: &str, name: &str) -> bool {
    let prefix = format!("{name}: ");
    line.split('\t').any(|segment| segment.starts_with(&prefix))
}

pub fn unpaywall_url(doi: &str, email: &str) -> String {
    format!(
        "https://api.unpaywall.org/v2/{}?email={}",
        urlencode(doi),
        urlencode(email)
    )
}

pub fn openalex_url(doi: &str) -> String {
    format!("https://api.openalex.org/works/https://doi.org/{doi}")
}

pub fn parse_unpaywall(body: &str) -> Texts {
    let mut texts = Texts::default();
    let Some(v) = json::parse(body) else {
        return texts;
    };
    let Some(location) = v.get("best_oa_location") else {
        return texts;
    };
    texts.pdf = location
        .get("url_for_pdf")
        .and_then(|u| u.as_str())
        .filter(|u| !u.is_empty())
        .or_else(|| {
            location
                .get("pdf_url")
                .and_then(|u| u.as_str())
                .filter(|u| !u.is_empty())
        })
        .map(str::to_string);
    texts.fulltext = location
        .get("url")
        .and_then(|u| u.as_str())
        .filter(|u| !u.is_empty())
        .map(str::to_string);
    texts
}

pub fn parse_openalex(body: &str) -> Texts {
    let mut texts = Texts::default();
    let Some(v) = json::parse(body) else {
        return texts;
    };
    let Some(location) = v.get("best_oa_location") else {
        return texts;
    };
    texts.pdf = location
        .get("pdf_url")
        .and_then(|u| u.as_str())
        .filter(|u| !u.is_empty())
        .map(str::to_string);
    texts.fulltext = location
        .get("landing_page_url")
        .and_then(|u| u.as_str())
        .filter(|u| !u.is_empty())
        .map(str::to_string);
    texts
}

fn email() -> Option<String> {
    crate::token::secret("UNPAYWALL_EMAIL")
        .or_else(|| crate::token::secret("OPENALEX_MAILTO"))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn fetch(url: &str) -> Option<String> {
    match get(url, &[], "40") {
        Some(f) if f.status == Some(200) => Some(f.body),
        _ => None,
    }
}

fn resolve(doi: &str) -> Option<Texts> {
    if let Some(lines) = cache::load(CACHE_MODE, doi, CACHE_MAX) {
        match decode(&lines) {
            Some(Cached::Found(texts)) => return Some(texts),
            Some(Cached::Absent) => return None,
            None => {}
        }
    }
    let mut texts = Texts::default();
    let mut complete = true;
    let mut attempted = false;
    if let Some(mail) = email() {
        attempted = true;
        match fetch(&unpaywall_url(doi, &mail)) {
            Some(body) => texts.merge(parse_unpaywall(&body)),
            None => complete = false,
        }
    }
    if (texts.pdf.is_none() || texts.fulltext.is_none()) && complete {
        attempted = true;
        match fetch(&openalex_url(doi)) {
            Some(body) => texts.merge(parse_openalex(&body)),
            None => complete = false,
        }
    }
    if texts.is_empty() {
        if attempted && complete {
            cache::store(
                CACHE_MODE,
                doi,
                CACHE_MAX,
                &[format!("absent — no open fulltext: {doi}")],
            );
        }
        return None;
    }
    cache::store(CACHE_MODE, doi, CACHE_MAX, &encode(&texts));
    Some(texts)
}

fn encode(texts: &Texts) -> Vec<String> {
    let mut lines = Vec::new();
    if let Some(pdf) = &texts.pdf {
        lines.push(format!("pdf: {pdf}"));
    }
    if let Some(fulltext) = &texts.fulltext {
        lines.push(format!("fulltext: {fulltext}"));
    }
    lines
}

fn decode(lines: &[String]) -> Option<Cached> {
    let mut texts = Texts::default();
    for line in lines {
        if line.starts_with("absent") {
            return Some(Cached::Absent);
        }
        if let Some(pdf) = line.strip_prefix("pdf: ") {
            if !pdf.is_empty() {
                texts.pdf = Some(pdf.to_string());
            }
        } else if let Some(fulltext) = line.strip_prefix("fulltext: ") {
            if !fulltext.is_empty() {
                texts.fulltext = Some(fulltext.to_string());
            }
        }
    }
    if texts.is_empty() {
        None
    } else {
        Some(Cached::Found(texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_doi_field() {
        let line =
            "url https://example.org/x\ttitle: A field\tdoi: https://doi.org/10.1038/NATURE12373";
        assert_eq!(doi_of(line).as_deref(), Some("10.1038/nature12373"));
    }

    #[test]
    fn reads_the_bare_doi_field() {
        let line = "doi: 10.1234/abc\ttitle: A field";
        assert_eq!(doi_of(line).as_deref(), Some("10.1234/abc"));
    }

    #[test]
    fn ignores_lines_without_a_doi_field() {
        assert_eq!(
            doi_of("url https://example.org/x\ttitle: doi: 10.1234/abc"),
            None
        );
        assert_eq!(doi_of("doi: arXiv:2101.00001"), None);
        assert_eq!(doi_of(""), None);
    }

    #[test]
    fn builds_the_unpaywall_url() {
        assert_eq!(
            unpaywall_url("10.1234/abc", "ada@example.org"),
            "https://api.unpaywall.org/v2/10.1234%2Fabc?email=ada%40example.org"
        );
    }

    #[test]
    fn builds_the_openalex_url() {
        assert_eq!(
            openalex_url("10.1234/abc"),
            "https://api.openalex.org/works/https://doi.org/10.1234/abc"
        );
    }

    #[test]
    fn reads_the_unpaywall_location() {
        let body = r#"{"doi":"10.1234/abc","best_oa_location":{"url":"https://example.org/landing","url_for_pdf":"https://example.org/paper.pdf"}}"#;
        assert_eq!(
            parse_unpaywall(body),
            Texts {
                pdf: Some("https://example.org/paper.pdf".to_string()),
                fulltext: Some("https://example.org/landing".to_string()),
            }
        );
    }

    #[test]
    fn reads_the_openalex_location() {
        let body = r#"{"best_oa_location":{"pdf_url":"https://example.org/paper.pdf","landing_page_url":"https://example.org/landing"}}"#;
        assert_eq!(
            parse_openalex(body),
            Texts {
                pdf: Some("https://example.org/paper.pdf".to_string()),
                fulltext: Some("https://example.org/landing".to_string()),
            }
        );
    }

    #[test]
    fn missing_locations_carry_no_values() {
        assert_eq!(
            parse_unpaywall(r#"{"doi":"10.1234/abc","is_oa":false}"#),
            Texts::default()
        );
        assert_eq!(
            parse_openalex(r#"{"id":"https://openalex.org/W1"}"#),
            Texts::default()
        );
        assert_eq!(parse_unpaywall("not json"), Texts::default());
    }

    #[test]
    fn appends_the_fields_to_the_hit() {
        let mut line = "url https://example.org/x\ttitle: A field".to_string();
        annotate(
            &mut line,
            &Texts {
                pdf: Some("https://example.org/paper.pdf".to_string()),
                fulltext: Some("https://example.org/landing".to_string()),
            },
        );
        assert_eq!(
            line,
            "url https://example.org/x\ttitle: A field\tpdf: https://example.org/paper.pdf\tfulltext: https://example.org/landing"
        );
    }

    #[test]
    fn keeps_the_existing_pdf_field() {
        let mut line = "url https://example.org/x\tpdf: https://example.org/old.pdf".to_string();
        annotate(
            &mut line,
            &Texts {
                pdf: Some("https://example.org/new.pdf".to_string()),
                fulltext: None,
            },
        );
        assert_eq!(
            line,
            "url https://example.org/x\tpdf: https://example.org/old.pdf"
        );
    }

    #[test]
    fn appends_only_the_present_field() {
        let mut line = "url https://example.org/x\ttitle: A field".to_string();
        annotate(
            &mut line,
            &Texts {
                pdf: None,
                fulltext: Some("https://example.org/landing".to_string()),
            },
        );
        assert_eq!(
            line,
            "url https://example.org/x\ttitle: A field\tfulltext: https://example.org/landing"
        );
    }

    #[test]
    fn the_limit_counts_the_enriched_hits() {
        let mut lines = vec![
            "url https://example.org/1\tdoi: 10.1/a".to_string(),
            "url https://example.org/2\tdoi: 10.1/b".to_string(),
            "url https://example.org/3\tdoi: 10.1/c".to_string(),
        ];
        let mut seen = Vec::new();
        run_with(true, &mut lines, 2, |doi| {
            seen.push(doi.to_string());
            Some(Texts {
                pdf: Some(format!("https://example.org/{doi}.pdf")),
                fulltext: None,
            })
        });
        assert_eq!(seen, vec!["10.1/a".to_string(), "10.1/b".to_string()]);
        assert!(lines[0].contains("\tpdf: "));
        assert!(lines[1].contains("\tpdf: "));
        assert!(!lines[2].contains("\tpdf: "));
    }

    #[test]
    fn the_limit_holds_across_two_batches() {
        let mut used = 0usize;
        let mut first = vec!["url https://example.org/1\tdoi: 10.1/a".to_string()];
        let mut second = vec!["url https://example.org/2\tdoi: 10.1/b".to_string()];
        let answer = |_doi: &str| {
            Some(Texts {
                pdf: Some("https://example.org/paper.pdf".to_string()),
                fulltext: None,
            })
        };
        run_with_budget(true, &mut first, 1, &mut used, answer);
        run_with_budget(true, &mut second, 1, &mut used, answer);
        assert_eq!(used, 1);
        assert!(first[0].contains("\tpdf: "));
        assert!(!second[0].contains("\tpdf: "));
    }

    #[test]
    fn hits_without_a_doi_do_not_use_the_limit() {
        let mut lines = vec![
            "url https://example.org/1\ttitle: no doi".to_string(),
            "url https://example.org/2\tdoi: 10.1/b".to_string(),
        ];
        let mut seen = Vec::new();
        run_with(true, &mut lines, 1, |doi| {
            seen.push(doi.to_string());
            Some(Texts {
                pdf: Some("https://example.org/b.pdf".to_string()),
                fulltext: None,
            })
        });
        assert_eq!(seen, vec!["10.1/b".to_string()]);
        assert!(lines[1].contains("\tpdf: "));
    }

    #[test]
    fn without_the_flag_no_resolver_runs() {
        let mut lines = vec!["url https://example.org/x\tdoi: 10.1/a".to_string()];
        let mut calls = 0usize;
        run_with(false, &mut lines, 5, |_| {
            calls += 1;
            Some(Texts {
                pdf: Some("https://example.org/a.pdf".to_string()),
                fulltext: None,
            })
        });
        assert_eq!(calls, 0);
        assert_eq!(
            lines,
            vec!["url https://example.org/x\tdoi: 10.1/a".to_string()]
        );
    }

    #[test]
    fn without_a_result_the_hit_stays_unchanged() {
        let mut lines = vec!["url https://example.org/x\tdoi: 10.1/a".to_string()];
        run_with(true, &mut lines, 5, |_| None);
        assert_eq!(
            lines,
            vec!["url https://example.org/x\tdoi: 10.1/a".to_string()]
        );
    }

    #[test]
    fn the_cache_encoding_round_trips() {
        let texts = Texts {
            pdf: Some("https://example.org/paper.pdf".to_string()),
            fulltext: Some("https://example.org/landing".to_string()),
        };
        match decode(&encode(&texts)) {
            Some(Cached::Found(read)) => assert_eq!(read, texts),
            _ => panic!("the encoded texts do not decode"),
        }
    }

    #[test]
    fn the_cached_absence_decodes_as_absent() {
        match decode(&["absent — no open fulltext: 10.1/a".to_string()]) {
            Some(Cached::Absent) => {}
            _ => panic!("the absent marker does not decode"),
        }
    }
}
