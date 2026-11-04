use std::collections::HashMap;

const TRACKING_PREFIXES: &[&str] = &["utm_"];
const TRACKING_KEYS: &[&str] = &[
    "gclid", "fbclid", "msclkid", "mc_cid", "mc_eid", "igshid", "yclid", "dclid", "srsltid", "si",
];

struct Entry {
    prefix: String,
    mode: Option<String>,
    url: String,
    segments: Vec<String>,
    origins: Vec<String>,
}

impl Entry {
    fn parse(line: &str) -> Option<Entry> {
        let (prefix, mode, body) = match line.strip_prefix('[') {
            Some(rest) => match rest.find("] ") {
                Some(end) => {
                    let mode = &rest[..end];
                    (
                        format!("[{}] ", mode),
                        Some(mode.to_string()),
                        &rest[end + 2..],
                    )
                }
                None => (String::new(), None, line),
            },
            None => (String::new(), None, line),
        };
        if !body.starts_with("url ") {
            return None;
        }
        let mut segments: Vec<String> = body.split('\t').map(str::to_string).collect();
        if segments.is_empty() {
            return None;
        }
        let url = segments[0].strip_prefix("url ")?.trim().to_string();
        if url.is_empty() {
            return None;
        }
        let mut origins = Vec::new();
        for segment in segments.iter().skip(1) {
            if let Some(value) = segment.strip_prefix("also: ") {
                push_origins(&mut origins, value);
            }
        }
        if let Some(mode) = &mode {
            push_origins(&mut origins, mode);
        }
        segments.retain(|segment| !segment.starts_with("also: "));
        Some(Entry {
            prefix,
            mode,
            url,
            segments,
            origins,
        })
    }

    fn key(&self) -> Option<String> {
        if let Some(doi) = self.field("doi") {
            let normalized = normalize_doi(doi);
            if !normalized.is_empty() {
                return Some(format!("doi:{normalized}"));
            }
        }
        if let Some(id) = arxiv_id(&self.url) {
            return Some(format!("arxiv:{id}"));
        }
        let normalized = normalize_url(&self.url);
        if normalized.is_empty() {
            None
        } else {
            Some(format!("url:{normalized}"))
        }
    }

    fn field(&self, name: &str) -> Option<&str> {
        self.segments.iter().skip(1).find_map(|segment| {
            let (key, value) = segment.split_once(": ")?;
            if key.trim().eq_ignore_ascii_case(name) && !value.trim().is_empty() {
                Some(value.trim())
            } else {
                None
            }
        })
    }

    fn merge(&mut self, other: &Entry) {
        for segment in other.segments.iter().skip(1) {
            let key = segment
                .split_once(": ")
                .map(|(key, _)| key.trim().to_ascii_lowercase());
            let exists = key.as_deref().is_some_and(|key| {
                self.segments.iter().skip(1).any(|existing| {
                    existing
                        .split_once(": ")
                        .map(|(name, _)| name.trim().eq_ignore_ascii_case(key))
                        .unwrap_or(false)
                })
            });
            if !exists {
                self.segments.push(segment.clone());
            }
        }
        for origin in &other.origins {
            push_origins(&mut self.origins, origin);
        }
        if self.mode.is_none() {
            self.mode = other.mode.clone();
        }
    }

    fn render(&self) -> String {
        let mut body = self.segments.join("\t");
        if self.origins.len() > 1 {
            body.push_str(&format!("\talso: {}", self.origins.join(",")));
        }
        format!("{}{}", self.prefix, body)
    }
}

fn push_origins(origins: &mut Vec<String>, value: &str) {
    for token in value.split(',') {
        let token = token.trim();
        if !token.is_empty() && !origins.iter().any(|origin| origin == token) {
            origins.push(token.to_string());
        }
    }
}

pub fn apply(lines: &[String], enabled: bool) -> Vec<String> {
    if !enabled {
        return lines.to_vec();
    }
    let mut out: Vec<String> = Vec::new();
    let mut entries: Vec<Option<Entry>> = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();
    for line in lines {
        let Some(entry) = Entry::parse(line) else {
            out.push(line.clone());
            entries.push(None);
            continue;
        };
        let Some(key) = entry.key() else {
            out.push(line.clone());
            entries.push(None);
            continue;
        };
        match seen.get(&key).copied() {
            Some(index) => {
                if let Some(target) = entries[index].as_mut() {
                    target.merge(&entry);
                    out[index] = target.render();
                }
            }
            None => {
                seen.insert(key, out.len());
                out.push(entry.render());
                entries.push(Some(entry));
            }
        }
    }
    out
}

pub fn normalize_doi(raw: &str) -> String {
    let mut value = raw.trim().to_ascii_lowercase();
    loop {
        let stripped = [
            "https://doi.org/",
            "http://doi.org/",
            "https://dx.doi.org/",
            "http://dx.doi.org/",
            "doi.org/",
            "dx.doi.org/",
            "doi:",
            "doi ",
        ]
        .iter()
        .find_map(|prefix| value.strip_prefix(prefix));
        match stripped {
            Some(rest) => value = rest.trim().to_string(),
            None => break,
        }
    }
    value
}

pub fn normalize_url(raw: &str) -> String {
    let raw = raw.trim();
    let raw = raw.split('#').next().unwrap_or(raw);
    let rest = raw
        .strip_prefix("https://")
        .or_else(|| raw.strip_prefix("http://"))
        .or_else(|| raw.strip_prefix("HTTPS://"))
        .or_else(|| raw.strip_prefix("HTTP://"))
        .unwrap_or(raw);
    let (authority, tail) = match rest.split_once('/') {
        Some((authority, tail)) => (authority, Some(tail)),
        None => (rest, None),
    };
    let authority = authority.to_ascii_lowercase();
    let tail = tail.unwrap_or("");
    let (path, query) = match tail.split_once('?') {
        Some((path, query)) => (path, query),
        None => (tail, ""),
    };
    let path = path.trim_end_matches('/');
    let params: Vec<&str> = query
        .split('&')
        .filter(|param| {
            let key = param.split_once('=').map(|(key, _)| key).unwrap_or(param);
            let key = key.to_ascii_lowercase();
            !key.is_empty()
                && !TRACKING_KEYS.contains(&key.as_str())
                && !TRACKING_PREFIXES
                    .iter()
                    .any(|prefix| key.starts_with(prefix))
        })
        .collect();
    let mut out = format!("https://{authority}");
    if !path.is_empty() {
        out.push('/');
        out.push_str(path);
    }
    if !params.is_empty() {
        out.push('?');
        out.push_str(&params.join("&"));
    }
    out
}

pub fn arxiv_id(raw: &str) -> Option<String> {
    let lower = raw.to_ascii_lowercase();
    let marker = ["arxiv.org/abs/", "arxiv.org/pdf/"]
        .iter()
        .find_map(|marker| lower.find(marker).map(|pos| (pos, marker.len())))?;
    let rest = &raw[marker.0 + marker.1..];
    let rest = rest.split(['?', '#']).next().unwrap_or(rest);
    let rest = rest.trim_end_matches('/');
    let rest = rest.strip_suffix(".pdf").unwrap_or(rest);
    let id = strip_version(rest);
    if id.is_empty() {
        None
    } else {
        Some(id.to_ascii_lowercase())
    }
}

fn strip_version(id: &str) -> &str {
    if let Some(pos) = id.rfind('v') {
        let suffix = &id[pos + 1..];
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
            return &id[..pos];
        }
    }
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(mode: &str, url: &str, fields: &[&str]) -> String {
        let mut body = format!("url {url}");
        for field in fields {
            body.push('\t');
            body.push_str(field);
        }
        format!("[{mode}] {body}")
    }

    #[test]
    fn doi_normalization_strips_prefixes_and_lowercases() {
        assert_eq!(normalize_doi("https://doi.org/10.1/ABC"), "10.1/abc");
        assert_eq!(normalize_doi("http://dx.doi.org/10.1/ABC"), "10.1/abc");
        assert_eq!(normalize_doi("doi:10.1/ABC"), "10.1/abc");
        assert_eq!(normalize_doi("  DOI 10.1/ABC "), "10.1/abc");
    }

    #[test]
    fn url_normalization_unifies_scheme_slash_and_tracking() {
        assert_eq!(
            normalize_url("http://Example.COM/path/?utm_source=x&a=1&fbclid=y#frag"),
            "https://example.com/path?a=1"
        );
        assert_eq!(
            normalize_url("https://example.com/path"),
            normalize_url("http://example.com/path/")
        );
    }

    #[test]
    fn arxiv_id_drops_version_and_pdf_suffix() {
        assert_eq!(
            arxiv_id("https://arxiv.org/abs/2101.12345v3"),
            Some("2101.12345".to_string())
        );
        assert_eq!(
            arxiv_id("https://arxiv.org/pdf/2101.12345v2.pdf"),
            Some("2101.12345".to_string())
        );
        assert_eq!(
            arxiv_id("https://arxiv.org/abs/hep-th/9901001"),
            Some("hep-th/9901001".to_string())
        );
    }

    #[test]
    fn duplicate_doi_merges_fields_and_marks_origins() {
        let lines = vec![
            line(
                "openalex",
                "https://example.org/a",
                &["title: A", "doi: 10.1/ABC"],
            ),
            "=== crossref (1) ===".to_string(),
            line(
                "crossref",
                "https://doi.org/10.1/abc",
                &["doi: https://doi.org/10.1/ABC", "year: 2020"],
            ),
        ];
        let out = apply(&lines, true);
        assert_eq!(out.len(), 2);
        assert_eq!(
            out[0],
            "[openalex] url https://example.org/a\ttitle: A\tdoi: 10.1/ABC\tyear: 2020\talso: openalex,crossref"
        );
        assert_eq!(out[1], "=== crossref (1) ===");
    }

    #[test]
    fn duplicate_url_keeps_one_line_but_preserves_other_lines() {
        let lines = vec![
            "=== coverage: 2 keyword modes ===".to_string(),
            line("a", "https://example.org/x/?utm_source=1", &["title: X"]),
            line("b", "http://example.org/x", &[]),
            "end: 2".to_string(),
            "total: 2".to_string(),
            "full: 4 lines -> /tmp/x".to_string(),
        ];
        let out = apply(&lines, true);
        assert_eq!(out.len(), 5);
        assert!(out[1].contains("also: a,b"));
        assert_eq!(out[0], "=== coverage: 2 keyword modes ===");
        assert_eq!(out[2], "end: 2");
        assert_eq!(out[3], "total: 2");
        assert_eq!(out[4], "full: 4 lines -> /tmp/x");
    }

    #[test]
    fn arxiv_versions_share_one_key() {
        let lines = vec![
            line("arxiv", "https://arxiv.org/abs/2101.12345v1", &["title: A"]),
            line("arxiv", "https://arxiv.org/abs/2101.12345v2", &["title: A"]),
        ];
        let out = apply(&lines, true);
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("title: A"));
        assert!(!out[0].contains("also:"));
    }

    #[test]
    fn disabled_dedupe_returns_the_old_lines_exactly() {
        let lines = vec![
            line("openalex", "https://example.org/a", &["doi: 10.1/x"]),
            line("crossref", "https://doi.org/10.1/x", &["doi: 10.1/x"]),
        ];
        assert_eq!(apply(&lines, false), lines);
    }
}
