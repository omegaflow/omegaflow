use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process::exit;

const SOURCES: &str = "phi/sources.φ";
const CENSUS: &str = "state/mountain/license-census.tsv";

const TERMS: &[&str] = &[
    "CC-BY-4.0",
    "CC-BY-NC-3.0-IGO",
    "CC-BY-NC-SA-4.0",
    "CC0-1.0",
    "ODC-By-1.0",
    "ODbL-1.0",
    "OGL-Canada-2.0",
    "PD",
    "PDDL-1.0",
    "free-open",
    "own-work",
    "unbestimmt",
    "ohne-lizenz",
];

fn blocks(content: &str) -> Vec<Vec<&str>> {
    let mut out: Vec<Vec<&str>> = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
        } else {
            cur.push(line);
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn block_token<'a>(block: &[&'a str], name: &str) -> Option<&'a str> {
    for line in block {
        let mut fields = line.split_whitespace();
        if fields.next() == Some(name) {
            return fields.next();
        }
    }
    None
}

fn url_basename(url: &str) -> Option<&str> {
    let cut = url
        .find(|c| c == '?' || c == '#')
        .map(|i| &url[..i])
        .unwrap_or(url);
    let seg = cut.rsplit('/').next()?;
    if seg.is_empty() { None } else { Some(seg) }
}

fn block_identity(block: &[&str]) -> Option<String> {
    if let Some(url) = block_token(block, "url") {
        if let Some(base) = url_basename(url) {
            return Some(base.to_string());
        }
    }
    if let Some(format) = block_token(block, "format") {
        return Some(format.to_string());
    }
    block
        .first()
        .and_then(|line| line.split_whitespace().next())
        .map(|token| token.to_string())
}

fn parse_terms(content: &str) -> HashSet<(String, String)> {
    let mut out = HashSet::new();
    for block in blocks(content) {
        let value = match block_token(&block, "terms") {
            Some(value) => value,
            None => continue,
        };
        if let Some(identity) = block_identity(&block) {
            out.insert((identity, value.to_string()));
        }
    }
    out
}

fn terms_entries(sources: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut block_start: usize = 0;
    let mut in_block = false;
    let mut terms_value: Option<String> = None;
    for (idx, line) in sources.lines().enumerate() {
        let lineno = idx + 1;
        if line.trim().is_empty() {
            if in_block {
                if let Some(value) = terms_value.take() {
                    out.push((block_start, value));
                }
                in_block = false;
            }
            continue;
        }
        if !in_block {
            in_block = true;
            block_start = lineno;
            terms_value = None;
        }
        let mut fields = line.split_whitespace();
        if fields.next() == Some("terms") {
            if let Some(value) = fields.next() {
                terms_value = Some(value.to_string());
            }
        }
    }
    if in_block {
        if let Some(value) = terms_value.take() {
            out.push((block_start, value));
        }
    }
    out
}

fn closed_vocab_violations(sources: &str) -> Vec<(usize, String)> {
    terms_entries(sources)
        .into_iter()
        .filter(|(_, value)| !TERMS.contains(&value.as_str()))
        .collect()
}

fn parse_census(content: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    for line in content.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 3 {
            continue;
        }
        out.push((
            fields[0].to_string(),
            fields[1].to_string(),
            fields[2].to_string(),
        ));
    }
    out
}

fn drift(terms: &HashSet<(String, String)>, census: &[(String, String, String)]) -> Vec<String> {
    let measured: HashSet<(String, String)> = census
        .iter()
        .map(|(key, class, _)| (key.clone(), class.clone()))
        .collect();

    let mut unmeasured: Vec<(&String, &String)> = terms
        .iter()
        .filter(|pair| !measured.contains(*pair))
        .map(|(key, class)| (key, class))
        .collect();
    unmeasured.sort();

    let mut stale: Vec<&(String, String, String)> = census
        .iter()
        .filter(|(key, class, _)| !terms.contains(&(key.clone(), class.clone())))
        .collect();
    stale.sort();

    let mut lines = Vec::new();
    for (key, class) in unmeasured {
        lines.push(format!("UNMEASURED {} {}", key, class));
    }
    for (key, class, url) in stale {
        lines.push(format!("STALE {} {} {}", key, class, url));
    }
    lines
}

struct Counts {
    blocks: usize,
    with_terms: usize,
    distinct_terms: usize,
    pending: usize,
}

fn counts(content: &str) -> Counts {
    let mut total: usize = 0;
    let mut with_terms: usize = 0;
    let mut with_compiler_or_url: usize = 0;
    let mut values: HashSet<String> = HashSet::new();
    for block in blocks(content) {
        total += 1;
        if let Some(value) = block_token(&block, "terms") {
            with_terms += 1;
            values.insert(value.to_string());
        }
        if block_token(&block, "compiler").is_some() || block_token(&block, "url").is_some() {
            with_compiler_or_url += 1;
        }
    }
    Counts {
        blocks: total,
        with_terms,
        distinct_terms: values.len(),
        pending: with_compiler_or_url.saturating_sub(with_terms),
    }
}

fn no_terms_identities(content: &str) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out: Vec<String> = Vec::new();
    for block in blocks(content) {
        if block_token(&block, "terms").is_some() {
            continue;
        }
        if block_token(&block, "compiler").is_none() && block_token(&block, "url").is_none() {
            continue;
        }
        if let Some(identity) = block_identity(&block) {
            if seen.insert(identity.clone()) {
                out.push(identity);
            }
        }
    }
    out.sort();
    out
}

fn main() {
    let count_only = env::args().any(|a| a == "--count");
    let fail = env::args().any(|a| a == "--fail");

    if !Path::new(SOURCES).exists() {
        println!("license_census: {} absent", SOURCES);
        exit(2);
    }

    let sources = match std::fs::read_to_string(SOURCES) {
        Ok(text) => text,
        Err(_) => {
            println!("license_census: {} absent", SOURCES);
            exit(2);
        }
    };

    match std::fs::read_to_string(CENSUS) {
        Ok(census) => {
            let lines = drift(&parse_terms(&sources), &parse_census(&census));
            if count_only {
                println!("{}", lines.len());
            } else {
                for line in &lines {
                    println!("{}", line);
                }
            }
        }
        Err(_) => println!(
            "license_census: {} absent — state-census complement skipped (tracked gate stands)",
            CENSUS
        ),
    }

    let violations = closed_vocab_violations(&sources);
    for (line, value) in &violations {
        println!("terms-vocab VIOLATION {} {}", line, value);
    }
    let no_terms = no_terms_identities(&sources);
    if !count_only {
        for identity in &no_terms {
            println!("NO-TERMS {}", identity);
        }
    }
    let census_counts = counts(&sources);
    println!(
        "license_census: blocks {} | terms {} | distinct {} | no-terms {} | pending {} | terms-vocab {} violation(s)",
        census_counts.blocks,
        census_counts.with_terms,
        census_counts.distinct_terms,
        no_terms.len(),
        census_counts.pending,
        violations.len()
    );
    if fail && !violations.is_empty() {
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drift_classifies_unmeasured_and_stale() {
        let src = "url https://github.com/omegaflow/sources/releases/download/a.org/x.bin\n\
                   terms CC0 https://a\n\n\
                   url https://github.com/omegaflow/sources/releases/download/b.org/y.bin\n\
                   terms MIT https://b\n\n\
                   url https://github.com/omegaflow/sources/releases/download/x.org/z.bin\n\
                   foo terms X https://c\n";
        let tsv = "# key\tclass\tterms-url\tmeasured\n\
                   x.bin\tCC0\thttps://a\t2026-10-07\n\
                   c.bin\tGPL\thttps://c\t2026-10-07\n";
        let terms = parse_terms(src);
        assert_eq!(terms.len(), 2, "{:?}", terms);
        let census = parse_census(tsv);
        assert_eq!(census.len(), 2, "{:?}", census);
        let lines = drift(&terms, &census);
        assert!(
            lines.contains(&"UNMEASURED y.bin MIT".to_string()),
            "{:?}",
            lines
        );
        assert!(
            lines.contains(&"STALE c.bin GPL https://c".to_string()),
            "{:?}",
            lines
        );
        assert_eq!(lines.len(), 2, "{:?}", lines);
    }

    #[test]
    fn join_keys_on_the_source_block() {
        let src = "url https://example.org/data/observations.csv?token=abc\n\
                   terms CC-BY-4.0 https://example.org/licence\n\n\
                   format tap\n\
                   url https://other.example/tap/sync?QUERY=x\n\
                   terms ODbL-1.0 https://other.example/terms\n\n\
                   format openneuro_pd_eeg\n\
                   terms CC0 https://openneuro.org/datasets/ds007822\n";
        let terms = parse_terms(src);
        assert!(
            terms.contains(&("observations.csv".to_string(), "CC-BY-4.0".to_string())),
            "{:?}",
            terms
        );
        assert!(
            terms.contains(&("sync".to_string(), "ODbL-1.0".to_string())),
            "{:?}",
            terms
        );
        assert!(
            terms.contains(&("openneuro_pd_eeg".to_string(), "CC0".to_string())),
            "{:?}",
            terms
        );
        assert_eq!(terms.len(), 3, "{:?}", terms);
    }

    #[test]
    fn closed_vocab_accepts_the_twelve_and_refuses_others() {
        let src = "terms CC-BY-4.0\n\n\
                   terms CC-BY-NC-SA-4.0\n\n\
                   terms CC0\n\n\
                   terms ODC-BY-1.0\n\n\
                   terms ODbL-1.0\n\n\
                   terms OGL-Canada-2.0\n\n\
                   terms PD\n\n\
                   terms PDDL-1.0\n\n\
                   terms free-open\n\n\
                   terms own-work\n\n\
                   terms unbestimmt\n\n\
                   terms ohne-lizenz\n\n\
                   terms unknown\n\n\
                   terms cc0\n\n\
                   url https://example.org/x.bin\n";
        let violations = closed_vocab_violations(src);
        let values: Vec<&str> = violations.iter().map(|(_, v)| v.as_str()).collect();
        assert_eq!(values, vec!["unknown", "cc0"], "{:?}", violations);
        assert_eq!(violations[0].0, 25, "{:?}", violations);
        assert_eq!(
            terms_entries(src).len(),
            14,
            "twelve accepted plus two refused"
        );
    }
}
