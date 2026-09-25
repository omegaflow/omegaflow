use omegaflow::force::gate_weigh;
use std::collections::HashSet;
use std::io::Write;

pub const LIBRARY_PATH: &str = "phi/pipeline/library.φ";
pub const CATALOG_DIR: &str = "phi/pipeline/catalog";
pub const MASTER_URLS_PATH: &str = "phi/pipeline/master_urls.txt";
pub const CANDIDATES_PATH: &str = "phi/pipeline/probe_url_candidates.txt";

const CANDIDATE_WEIGHT_FLOOR: i32 = 1;

fn strip_scheme(url: &str) -> &str {
    for scheme in ["https://", "http://"] {
        if let Some(head) = url.get(..scheme.len()) {
            if head.eq_ignore_ascii_case(scheme) {
                return &url[scheme.len()..];
            }
        }
    }
    url
}

fn normalize_url(raw: &str) -> String {
    let bare = strip_scheme(raw.trim());
    let mut out = bare.replace("/?", "?");
    while out.ends_with('/') {
        out.pop();
    }
    out
}

fn fold_placeholders(normalized: &str) -> String {
    let mut out = String::with_capacity(normalized.len());
    let mut rest = normalized;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        match rest[open + 1..].find('}') {
            Some(close) => {
                out.push('*');
                rest = &rest[open + 1 + close + 1..];
            }
            None => {
                out.push('{');
                rest = &rest[open + 1..];
            }
        }
    }
    out.push_str(rest);
    out
}

fn catalog_variant_key(folded: &str) -> String {
    match folded.find('?') {
        Some(pos) => folded[..pos].to_string(),
        None => match folded.rfind('/') {
            Some(pos) => folded[..=pos].to_string(),
            None => folded.to_string(),
        },
    }
}

fn fold_key(url: &str) -> String {
    catalog_variant_key(&fold_placeholders(&normalize_url(url)))
}

pub fn source_url_candidates_run() -> i32 {
    let library = match std::fs::read_to_string(LIBRARY_PATH) {
        Ok(c) => omegaflow::force::parse_library(&c),
        Err(_) => {
            eprintln!("lens: library unreadable: {}", LIBRARY_PATH);
            return 1;
        }
    };

    let mut catalog_paths: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(CATALOG_DIR) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().map(|e| e == "φ").unwrap_or(false) {
                catalog_paths.push(p.to_string_lossy().into_owned());
            }
        }
    }
    catalog_paths.sort();

    let mut candidates: Vec<String> = Vec::new();

    for path in &catalog_paths {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("lens: catalog unreadable: {}", path);
                continue;
            }
        };
        for line in content.lines() {
            let t = line.trim();
            let stripped = if let Some(r) = t.strip_prefix("url ") {
                r
            } else if let Some(r) = t.strip_prefix("candidate ") {
                r
            } else {
                continue;
            };
            let Some(token) = stripped
                .split_whitespace()
                .find(|tok| tok.starts_with("http://") || tok.starts_with("https://"))
            else {
                continue;
            };
            let g = gate_weigh(token, &library);
            if g.weight >= CANDIDATE_WEIGHT_FLOOR {
                candidates.push(token.to_string());
            }
        }
    }

    candidates.sort();
    let mut seen: HashSet<String> = HashSet::new();
    let mut folded: Vec<String> = Vec::with_capacity(candidates.len());
    for c in candidates {
        if seen.insert(fold_key(&c)) {
            folded.push(c);
        }
    }
    candidates = folded;

    let mut known: HashSet<String> = HashSet::new();
    if let Ok(c) = std::fs::read_to_string(MASTER_URLS_PATH) {
        for line in c.lines() {
            let t = line.trim();
            if !t.is_empty() {
                known.insert(fold_key(t));
            }
        }
    }
    let known_count = candidates
        .iter()
        .filter(|u| known.contains(&fold_key(u)))
        .count();
    let new_count = candidates.len() - known_count;

    match write_lines(CANDIDATES_PATH, candidates.iter()) {
        Ok(()) => {
            eprintln!(
                "lens: {} candidates ({} known to master_urls, {} new)",
                candidates.len(),
                known_count,
                new_count
            );
            0
        }
        Err(_) => {
            eprintln!("lens: output unwritable: {}", CANDIDATES_PATH);
            1
        }
    }
}

fn write_lines<'a, I, S>(path: &str, lines: I) -> std::io::Result<()>
where
    I: Iterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut f = std::fs::File::create(path)?;
    for line in lines {
        f.write_all(line.as_ref().as_bytes())?;
        f.write_all(b"\n")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_strips_scheme_and_trailing_slash() {
        assert_eq!(normalize_url("https://host/path/"), "host/path");
        assert_eq!(normalize_url("http://host/path"), "host/path");
        assert_eq!(normalize_url("HTTPS://Host/A/"), "Host/A");
    }

    #[test]
    fn normalize_collapses_slash_question() {
        assert_eq!(normalize_url("https://host/path/?x=1"), "host/path?x=1");
        assert_eq!(normalize_url("https://host/path/?"), "host/path?");
    }

    #[test]
    fn fold_replaces_placeholder_with_wildcard() {
        assert_eq!(
            fold_placeholders("host/data?id={station}"),
            "host/data?id=*"
        );
        assert_eq!(fold_placeholders("host/{a}/b"), "host/*/b");
        assert_eq!(fold_placeholders("host/{x}y{z}"), "host*y*");
        assert_eq!(fold_placeholders("host/{unclosed"), "host/{unclosed");
    }

    #[test]
    fn catalog_key_strips_query() {
        assert_eq!(catalog_variant_key("host/path?a=1&b=2"), "host/path");
        assert_eq!(catalog_variant_key("host/path?"), "host/path");
    }

    #[test]
    fn catalog_key_takes_parent_directory() {
        assert_eq!(catalog_variant_key("host/dir/file.csv"), "host/dir/");
        assert_eq!(catalog_variant_key("host/file.csv"), "host/");
        assert_eq!(catalog_variant_key("host"), "host");
    }

    #[test]
    fn fold_key_collapses_fanout_template_and_concrete() {
        let template =
            "https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id={station}&start={week_ago}T00:00:00Z";
        let concrete =
            "https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=ABK&start=2026-01-01T00:00:00Z";
        assert_eq!(fold_key(template), fold_key(concrete));
        assert_eq!(fold_key(template), "imag-data.bgs.ac.uk/GIN_V1/hapi/data");
    }

    #[test]
    fn fold_key_collapses_table_as_file_variants() {
        let a = "https://host/catalog/gaia.csv";
        let b = "https://host/catalog/hipparcos.csv";
        assert_eq!(fold_key(a), fold_key(b));
    }
}
