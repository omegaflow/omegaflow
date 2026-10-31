use crate::net::get;
use std::collections::VecDeque;

const DEFAULT_ROOT: &str = "https://archives.esac.esa.int/psa/ftp/";

fn number(refine: &[(String, String)], key: &str, fallback: usize) -> usize {
    match crate::refine::value_of(refine, key) {
        Some(value) => value.parse().unwrap_or(fallback),
        None => fallback,
    }
}

fn links(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find("href=\"") {
        rest = &rest[start + 6..];
        let Some(end) = rest.find('"') else {
            break;
        };
        let target = &rest[..end];
        rest = &rest[end..];
        if target.is_empty() || target.starts_with('?') || target.starts_with("http") {
            continue;
        }
        out.push(target.to_string());
    }
    out
}

pub fn psa_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(query, &["root", "depth", "pages"]);
    let needle = text.trim().to_lowercase().replace(['-', ' '], "_");
    if needle.is_empty() {
        return vec![
            "pending — psa needs a keyword: archive_search --psa \"<name>\" [root=<url> depth=<n> pages=<n>]"
                .to_string(),
        ];
    }
    let root = match crate::refine::value_of(&refine, "root") {
        Some(value) => value.to_string(),
        None => DEFAULT_ROOT.to_string(),
    };
    let max_depth = number(&refine, "depth", 6);
    let page_cap = number(&refine, "pages", 300);
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    queue.push_back((root.clone(), 0));
    let mut pages = 0usize;
    let mut out: Vec<String> = Vec::new();
    let mut capped = false;
    while let Some((url, depth)) = queue.pop_front() {
        if pages >= page_cap {
            capped = true;
            break;
        }
        pages += 1;
        let Some(fetch) = get(&url, &[], "30") else {
            continue;
        };
        if fetch.status != Some(200) {
            continue;
        }
        for target in links(&fetch.body) {
            if target.starts_with("..") || target.starts_with('/') {
                continue;
            }
            let full = format!("{url}{target}");
            if target.ends_with('/') {
                if depth < max_depth {
                    queue.push_back((full, depth + 1));
                }
                continue;
            }
            let normalised = target.to_lowercase().replace(['-', ' '], "_");
            if !normalised.contains(&needle) {
                continue;
            }
            out.push(format!("url {full}\tname {target}"));
            if out.len() >= max {
                return out;
            }
        }
    }
    if out.is_empty() {
        out.push(format!(
            "absent — psa carries no file named like '{needle}' in {pages} listing(s) under {root}"
        ));
    }
    if capped {
        out.push(format!(
            "pending — page cap {page_cap} reached after {pages} listing(s); raise pages= or narrow root="
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_read_relative_targets_only() {
        let body = "<a href=\"DATA/\">DATA</a><a href=\"file.PDF\">f</a><a href=\"http://x/y\">x</a><a href=\"?C=N\">s</a>";
        let got = links(body);
        assert_eq!(got, vec!["DATA/".to_string(), "file.PDF".to_string()]);
    }

    #[test]
    fn a_query_without_keyword_is_pending() {
        let lines = psa_lines("   ", 5);
        assert!(lines[0].starts_with("pending"));
    }
}
