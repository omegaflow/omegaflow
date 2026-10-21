use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Filters {
    pub since: Option<String>,
    pub until: Option<String>,
    pub kind: Option<String>,
    pub language: Option<String>,
}

impl Filters {
    pub fn is_empty(&self) -> bool {
        self.since.is_none()
            && self.until.is_none()
            && self.kind.is_none()
            && self.language.is_none()
    }

    pub fn has_date(&self) -> bool {
        self.since.is_some() || self.until.is_some()
    }

    fn signature(&self) -> String {
        format!(
            "since={};until={};type={};language={}",
            self.since.as_deref().unwrap_or("-"),
            self.until.as_deref().unwrap_or("-"),
            self.kind.as_deref().unwrap_or("-"),
            self.language.as_deref().unwrap_or("-")
        )
    }
}

static FILTERS: Mutex<Option<Filters>> = Mutex::new(None);
static EXPLAIN: AtomicBool = AtomicBool::new(false);

pub fn set(filters: Filters) {
    if let Ok(mut slot) = FILTERS.lock() {
        *slot = Some(filters);
    }
}

pub fn current() -> Filters {
    match FILTERS.lock() {
        Ok(slot) => match slot.as_ref() {
            Some(filters) => filters.clone(),
            None => Filters::default(),
        },
        Err(_) => Filters::default(),
    }
}

pub fn set_explain(on: bool) {
    EXPLAIN.store(on, Ordering::Relaxed);
}

pub fn explain_enabled() -> bool {
    EXPLAIN.load(Ordering::Relaxed)
}

pub fn cache_query(query: &str) -> String {
    cache_query_with(&current(), query)
}

fn cache_query_with(filters: &Filters, query: &str) -> String {
    if filters.is_empty() {
        query.to_string()
    } else {
        format!("{} |filters:{}", query, filters.signature())
    }
}

pub fn parse_since(raw: &str) -> Option<String> {
    parse_bound(raw, false)
}

pub fn parse_until(raw: &str) -> Option<String> {
    parse_bound(raw, true)
}

fn parse_bound(raw: &str, closing: bool) -> Option<String> {
    let raw = raw.trim();
    if raw.len() == 4 && raw.bytes().all(|b| b.is_ascii_digit()) {
        return Some(format!(
            "{}-{}-{}",
            raw,
            if closing { "12" } else { "01" },
            if closing { "31" } else { "01" }
        ));
    }
    if raw.len() == 10 && valid_date(raw) {
        return Some(raw.to_string());
    }
    None
}

fn valid_date(raw: &str) -> bool {
    if raw.len() != 10 {
        return false;
    }
    let bytes = raw.as_bytes();
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    if !bytes
        .iter()
        .enumerate()
        .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit())
    {
        return false;
    }
    let Ok(month) = raw[5..7].parse::<u8>() else {
        return false;
    };
    let Ok(day) = raw[8..10].parse::<u8>() else {
        return false;
    };
    (1..=12).contains(&month) && (1..=31).contains(&day)
}

pub fn valid_language(raw: &str) -> bool {
    raw.len() == 2 && raw.bytes().all(|b| b.is_ascii_alphabetic())
}

fn year(date: &str) -> &str {
    &date[..4]
}

fn lower(kind: &str) -> String {
    kind.trim().to_ascii_lowercase()
}

fn openalex_type(kind: &str) -> String {
    match lower(kind).as_str() {
        "thesis" | "dissertation" => "dissertation".to_string(),
        "journal-article" => "article".to_string(),
        "posted-content" => "preprint".to_string(),
        other => other.to_string(),
    }
}

fn crossref_type(kind: &str) -> String {
    match lower(kind).as_str() {
        "article" | "journal-article" => "journal-article".to_string(),
        "preprint" | "posted-content" => "posted-content".to_string(),
        "thesis" | "dissertation" => "dissertation".to_string(),
        "monograph" | "book" => "book".to_string(),
        other => other.to_string(),
    }
}

fn ads_doctype(kind: &str) -> String {
    match lower(kind).as_str() {
        "article" | "journal-article" => "article".to_string(),
        "preprint" | "posted-content" => "preprint".to_string(),
        "thesis" | "dissertation" => "thesis".to_string(),
        "monograph" | "book" => "book".to_string(),
        other => other.to_string(),
    }
}

fn s2_type(kind: &str) -> Option<&'static str> {
    match lower(kind).as_str() {
        "article" | "journal-article" | "journalarticle" => Some("JournalArticle"),
        "preprint" => Some("Preprint"),
        "dataset" => Some("Dataset"),
        "review" => Some("Review"),
        "conference" => Some("Conference"),
        "editorial" => Some("Editorial"),
        "case-report" | "casereport" => Some("CaseReport"),
        "clinical-trial" | "clinicaltrial" => Some("ClinicalTrial"),
        "magazine-article" | "magazinearticle" => Some("MagazineArticle"),
        "short-survey" | "shortsurvey" => Some("ShortSurvey"),
        _ => None,
    }
}

pub fn openalex_filter(filters: &Filters) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    if let Some(date) = &filters.since {
        parts.push(format!("from_publication_date:{}", date));
    }
    if let Some(date) = &filters.until {
        parts.push(format!("to_publication_date:{}", date));
    }
    if let Some(kind) = &filters.kind {
        parts.push(format!("type:{}", openalex_type(kind)));
    }
    if let Some(language) = &filters.language {
        parts.push(format!("language:{}", language.to_ascii_lowercase()));
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(","))
    }
}

pub fn crossref_filter(filters: &Filters) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    if let Some(date) = &filters.since {
        parts.push(format!("from-pub-date:{}", date));
    }
    if let Some(date) = &filters.until {
        parts.push(format!("until-pub-date:{}", date));
    }
    if let Some(kind) = &filters.kind {
        parts.push(format!("type:{}", crossref_type(kind)));
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(","))
    }
}

pub fn semanticscholar_year(filters: &Filters) -> Option<String> {
    match (&filters.since, &filters.until) {
        (Some(since), Some(until)) => Some(format!("{}-{}", year(since), year(until))),
        (Some(since), None) => Some(format!("{}-", year(since))),
        (None, Some(until)) => Some(format!("-{}", year(until))),
        (None, None) => None,
    }
}

pub fn semanticscholar_type(filters: &Filters) -> Option<&'static str> {
    filters.kind.as_deref().and_then(s2_type)
}

pub fn arxiv_submitted_range(filters: &Filters) -> Option<String> {
    let (since, until) = match (&filters.since, &filters.until) {
        (Some(since), Some(until)) => (since, until),
        _ => return None,
    };
    Some(format!(
        "submittedDate:[{}{}{} TO {}{}{}]",
        year(since),
        &since[5..7],
        &since[8..10],
        year(until),
        &until[5..7],
        &until[8..10]
    ))
}

pub fn pubmed_clause(filters: &Filters) -> Option<String> {
    let (since, until) = match (&filters.since, &filters.until) {
        (Some(since), Some(until)) => (since, until),
        _ => return None,
    };
    Some(format!(
        "({}:{}[pdat])",
        since.replace('-', "/"),
        until.replace('-', "/")
    ))
}

pub fn europepmc_clause(filters: &Filters) -> Option<String> {
    let (since, until) = match (&filters.since, &filters.until) {
        (Some(since), Some(until)) => (since, until),
        _ => return None,
    };
    Some(format!("(PUB_YEAR:[{} TO {}])", year(since), year(until)))
}

pub fn ads_fq(filters: &Filters) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if filters.has_date() {
        let from = filters.since.as_deref().map(year).unwrap_or("*");
        let to = filters.until.as_deref().map(year).unwrap_or("*");
        out.push(format!("pubdate:[{} TO {}]", from, to));
    }
    if let Some(kind) = &filters.kind {
        out.push(format!("doctype:{}", ads_doctype(kind)));
    }
    if let Some(language) = &filters.language {
        out.push(format!("lang:{}", language.to_ascii_lowercase()));
    }
    out
}

pub fn decorate(mode: &str, query: &str) -> String {
    decorate_with(&current(), mode, query)
}

pub fn decorate_with(filters: &Filters, mode: &str, query: &str) -> String {
    if filters.is_empty() {
        return query.to_string();
    }
    match mode {
        "openalex" => match openalex_filter(filters) {
            Some(filter) => merge_key(query, "filter", &filter),
            None => query.to_string(),
        },
        "crossref" => match crossref_filter(filters) {
            Some(filter) => merge_key(query, "filter", &filter),
            None => query.to_string(),
        },
        "semanticscholar" => {
            let mut out = query.to_string();
            if let Some(value) = semanticscholar_year(filters) {
                out = add_if_absent(&out, "year", &value);
            }
            if let Some(value) = semanticscholar_type(filters) {
                out = add_if_absent(&out, "publicationTypes", value);
            }
            out
        }
        "pubmed" => add_term(query, pubmed_clause(filters)),
        "europepmc" => add_term(query, europepmc_clause(filters)),
        _ => query.to_string(),
    }
}

fn merge_key(query: &str, key: &str, addition: &str) -> String {
    let mut parts: Vec<String> = query
        .split_whitespace()
        .map(str::to_string)
        .collect();
    let needle = format!("{}=", key);
    match parts.iter().position(|part| part.starts_with(&needle)) {
        Some(index) => {
            parts[index].push(',');
            parts[index].push_str(addition);
        }
        None => parts.push(format!("{}={}", key, addition)),
    }
    parts.join(" ")
}

fn add_if_absent(query: &str, key: &str, value: &str) -> String {
    let needle = format!("{}=", key);
    if query.split_whitespace().any(|part| part.starts_with(&needle)) {
        return query.to_string();
    }
    if query.trim().is_empty() {
        format!("{}={}", key, value)
    } else {
        format!("{} {}={}", query, key, value)
    }
}

fn add_term(query: &str, term: Option<String>) -> String {
    match term {
        Some(term) if query.trim().is_empty() => term,
        Some(term) => format!("{} AND {}", query, term),
        None => query.to_string(),
    }
}

const SUPPORTED: &[&str] = &[
    "openalex",
    "crossref",
    "semanticscholar",
    "arxiv",
    "pubmed",
    "europepmc",
    "ads",
];

fn date_expressible(filters: &Filters, mode: &str) -> bool {
    match mode {
        "arxiv" | "pubmed" | "europepmc" => filters.since.is_some() && filters.until.is_some(),
        _ => true,
    }
}

fn type_supported(filters: &Filters, mode: &str) -> bool {
    match mode {
        "openalex" | "crossref" | "ads" => true,
        "semanticscholar" => semanticscholar_type(filters).is_some(),
        _ => false,
    }
}

fn language_supported(mode: &str) -> bool {
    matches!(mode, "openalex" | "ads")
}

pub fn explain_lines(mode: &str) -> Vec<String> {
    explain_lines_with(&current(), explain_enabled(), mode)
}

pub fn explain_into(lines: &mut Vec<String>, mode: &str) {
    let notes = explain_lines(mode);
    if !notes.is_empty() {
        lines.splice(0..0, notes);
    }
}

pub fn explain_lines_with(filters: &Filters, explain: bool, mode: &str) -> Vec<String> {
    if !explain || filters.is_empty() || mode == "all" {
        return Vec::new();
    }
    if !SUPPORTED.contains(&mode) {
        return vec![format!("note: filter unsupported by {}", mode)];
    }
    let mut out: Vec<String> = Vec::new();
    if filters.has_date() && !date_expressible(filters, mode) {
        out.push(format!("note: date filter unsupported by {}", mode));
    }
    if filters.kind.is_some() && !type_supported(filters, mode) {
        out.push(format!("note: type filter unsupported by {}", mode));
    }
    if filters.language.is_some() && !language_supported(mode) {
        out.push(format!("note: language filter unsupported by {}", mode));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filters() -> Filters {
        Filters {
            since: parse_since("2020"),
            until: parse_until("2021"),
            kind: Some("article".to_string()),
            language: Some("en".to_string()),
        }
    }

    #[test]
    fn a_year_opens_at_the_first_day_and_closes_at_the_last() {
        assert_eq!(parse_since("2020").as_deref(), Some("2020-01-01"));
        assert_eq!(parse_until("2020").as_deref(), Some("2020-12-31"));
    }

    #[test]
    fn a_full_date_is_kept_and_malformed_dates_are_refused() {
        assert_eq!(parse_since("2020-02-29").as_deref(), Some("2020-02-29"));
        assert_eq!(parse_since("2020-13-01"), None);
        assert_eq!(parse_until("20-01-01"), None);
        assert_eq!(parse_until("2020/01/01"), None);
    }

    #[test]
    fn a_language_is_two_letters() {
        assert!(valid_language("en"));
        assert!(valid_language("DE"));
        assert!(!valid_language("eng"));
        assert!(!valid_language("e1"));
    }

    #[test]
    fn openalex_carries_all_four_filters() {
        assert_eq!(
            openalex_filter(&filters()).as_deref(),
            Some(
                "from_publication_date:2020-01-01,to_publication_date:2021-12-31,type:article,language:en"
            )
        );
    }

    #[test]
    fn openalex_adds_to_an_existing_filter_without_dropping_it() {
        let decorated = decorate_with(
            &filters(),
            "openalex",
            "solar wind filter=is_oa:true select=id",
        );
        assert_eq!(
            decorated,
            "solar wind filter=is_oa:true,from_publication_date:2020-01-01,to_publication_date:2021-12-31,type:article,language:en select=id"
        );
    }

    #[test]
    fn crossref_carries_dates_and_type_and_keeps_select_and_sort() {
        let decorated = decorate_with(
            &filters(),
            "crossref",
            "solar wind sort=published select=DOI,title filter=is-referenced-by-count:10",
        );
        assert_eq!(
            decorated,
            "solar wind sort=published select=DOI,title filter=is-referenced-by-count:10,from-pub-date:2020-01-01,until-pub-date:2021-12-31,type:journal-article"
        );
    }

    #[test]
    fn crossref_does_not_take_a_language() {
        let only_language = Filters {
            language: Some("en".to_string()),
            ..Filters::default()
        };
        assert_eq!(
            decorate_with(&only_language, "crossref", "solar wind"),
            "solar wind"
        );
    }

    #[test]
    fn semanticscholar_takes_a_year_range_and_a_publication_type() {
        let decorated = decorate_with(&filters(), "semanticscholar", "solar wind");
        assert_eq!(
            decorated,
            "solar wind year=2020-2021 publicationTypes=JournalArticle"
        );
    }

    #[test]
    fn semanticscholar_keeps_a_caller_supplied_year() {
        let decorated = decorate_with(&filters(), "semanticscholar", "solar wind year=2019");
        assert_eq!(
            decorated,
            "solar wind year=2019 publicationTypes=JournalArticle"
        );
    }

    #[test]
    fn semanticscholar_leaves_an_unknown_type_without_a_parameter() {
        let thesis = Filters {
            kind: Some("thesis".to_string()),
            since: Some("2020-01-01".to_string()),
            ..Filters::default()
        };
        assert_eq!(
            decorate_with(&thesis, "semanticscholar", "solar wind"),
            "solar wind year=2020-"
        );
        assert!(explain_lines_with(&thesis, true, "semanticscholar")
            .contains(&"note: type filter unsupported by semanticscholar".to_string()));
    }

    #[test]
    fn arxiv_submits_a_date_range_only_when_both_ends_are_known() {
        assert_eq!(
            arxiv_submitted_range(&filters()).as_deref(),
            Some("submittedDate:[20200101 TO 20211231]")
        );
        let open_end = Filters {
            since: parse_since("2020"),
            ..Filters::default()
        };
        assert_eq!(arxiv_submitted_range(&open_end), None);
        assert!(explain_lines_with(&open_end, true, "arxiv")
            .contains(&"note: date filter unsupported by arxiv".to_string()));
    }

    #[test]
    fn pubmed_uses_the_publication_date_range() {
        assert_eq!(
            pubmed_clause(&filters()).as_deref(),
            Some("(2020/01/01:2021/12/31[pdat])")
        );
        assert_eq!(
            decorate_with(&filters(), "pubmed", "solar wind"),
            "solar wind AND (2020/01/01:2021/12/31[pdat])"
        );
    }

    #[test]
    fn europepmc_uses_the_pub_year_range_and_keeps_the_query() {
        assert_eq!(
            europepmc_clause(&filters()).as_deref(),
            Some("(PUB_YEAR:[2020 TO 2021])")
        );
        assert_eq!(
            decorate_with(&filters(), "europepmc", "solar wind"),
            "solar wind AND (PUB_YEAR:[2020 TO 2021])"
        );
    }

    #[test]
    fn ads_builds_pubdate_doctype_and_lang_fq() {
        assert_eq!(
            ads_fq(&filters()),
            vec![
                "pubdate:[2020 TO 2021]".to_string(),
                "doctype:article".to_string(),
                "lang:en".to_string(),
            ]
        );
        let open_end = Filters {
            since: parse_since("2020"),
            ..Filters::default()
        };
        assert_eq!(ads_fq(&open_end), vec!["pubdate:[2020 TO *]".to_string()]);
    }

    #[test]
    fn a_mode_outside_the_seven_reports_one_unsupported_note() {
        assert_eq!(
            explain_lines_with(&filters(), true, "zenodo"),
            vec!["note: filter unsupported by zenodo".to_string()]
        );
    }

    #[test]
    fn notes_stay_silent_without_explain() {
        assert!(explain_lines_with(&filters(), false, "zenodo").is_empty());
        assert!(explain_lines_with(&filters(), true, "openalex").is_empty());
    }

    #[test]
    fn a_partial_arm_names_only_the_unsupported_part() {
        let notes = explain_lines_with(&filters(), true, "crossref");
        assert_eq!(notes, vec!["note: language filter unsupported by crossref".to_string()]);
    }

    #[test]
    fn decoration_leaves_a_filterless_query_untouched() {
        assert_eq!(
            decorate_with(&Filters::default(), "openalex", "solar wind"),
            "solar wind"
        );
    }

    #[test]
    fn the_cache_key_moves_only_when_a_filter_is_set() {
        assert_eq!(
            cache_query_with(&Filters::default(), "solar wind"),
            "solar wind"
        );
        assert_eq!(
            cache_query_with(&filters(), "solar wind"),
            "solar wind |filters:since=2020-01-01;until=2021-12-31;type=article;language=en"
        );
    }
}
