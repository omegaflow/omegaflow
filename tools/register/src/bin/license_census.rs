use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process::exit;

const SOURCES: &str = "phi/sources.φ";
const CENSUS: &str = "state/river/license-census.tsv";

const TERMS: &[&str] = &[
    "CC-BY-4.0",
    "CC-BY-NC-SA-4.0",
    "CC0",
    "ODC-BY-1.0",
    "ODbL-1.0",
    "PD",
    "PDDL-1.0",
    "free-open",
    "own-work",
];

fn release_tag(url: &str) -> Option<&str> {
    let marker = "/releases/download/";
    let rest = url.split_once(marker)?.1;
    let tag = rest.split('/').next()?;
    if tag.is_empty() { None } else { Some(tag) }
}

fn parse_terms(content: &str) -> HashSet<(String, String)> {
    let mut out = HashSet::new();
    for block in content.split("\n\n") {
        let mut netloc: Option<&str> = None;
        let mut class: Option<&str> = None;
        for line in block.lines() {
            let mut fields = line.split_whitespace();
            match fields.next() {
                Some("url") => {
                    if let Some(raw) = fields.next() {
                        netloc = release_tag(raw);
                    }
                }
                Some("terms") => class = fields.next(),
                _ => {}
            }
        }
        if let (Some(netloc), Some(class)) = (netloc, class) {
            out.insert((netloc.to_string(), class.to_string()));
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
        .map(|(netloc, class, _)| (netloc.clone(), class.clone()))
        .collect();

    let mut unmeasured: Vec<(&String, &String)> = terms
        .iter()
        .filter(|pair| !measured.contains(*pair))
        .map(|(netloc, class)| (netloc, class))
        .collect();
    unmeasured.sort();

    let mut stale: Vec<&(String, String, String)> = census
        .iter()
        .filter(|(netloc, class, _)| !terms.contains(&(netloc.clone(), class.clone())))
        .collect();
    stale.sort();

    let mut lines = Vec::new();
    for (netloc, class) in unmeasured {
        lines.push(format!("UNMEASURED {} {}", netloc, class));
    }
    for (netloc, class, url) in stale {
        lines.push(format!("STALE {} {} {}", netloc, class, url));
    }
    lines
}

fn main() {
    let count_only = env::args().any(|a| a == "--count");

    if !Path::new(SOURCES).exists() {
        println!("license_census: {} absent", SOURCES);
        exit(2);
    }
    if !Path::new(CENSUS).exists() {
        println!("license_census: {} absent", CENSUS);
        exit(2);
    }

    let sources = match std::fs::read_to_string(SOURCES) {
        Ok(text) => text,
        Err(_) => {
            println!("license_census: {} absent", SOURCES);
            exit(2);
        }
    };
    let census = match std::fs::read_to_string(CENSUS) {
        Ok(text) => text,
        Err(_) => {
            println!("license_census: {} absent", CENSUS);
            exit(2);
        }
    };

    let lines = drift(&parse_terms(&sources), &parse_census(&census));

    if count_only {
        println!("{}", lines.len());
    } else {
        for line in &lines {
            println!("{}", line);
        }
    }

    let entries = terms_entries(&sources);
    let violations = closed_vocab_violations(&sources);
    for (line, value) in &violations {
        println!("terms-vocab VIOLATION {} {}", line, value);
    }
    let distinct: HashSet<&str> = entries.iter().map(|(_, value)| value.as_str()).collect();
    println!(
        "license_census: terms-vocab {} violation(s), {} terms lines, {} distinct",
        violations.len(),
        entries.len(),
        distinct.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_register_netloc_against_census() {
        let src = "url https://github.com/omegaflow/sources/releases/download/a.org/x.bin\n\
                   terms CC0 https://a\n\n\
                   url https://github.com/omegaflow/sources/releases/download/b.org/y.bin\n\
                   terms MIT https://b\n\n\
                   url https://github.com/omegaflow/sources/releases/download/x.org/z.bin\n\
                   foo terms X https://c\n";
        let tsv = "# netloc\tclass\tterms-url\tmeasured\n\
                   a.org\tCC0\thttps://a\t2026-10-07\n\
                   c.org\tGPL\thttps://c\t2026-10-07\n";
        let terms = parse_terms(src);
        assert_eq!(terms.len(), 2, "{:?}", terms);
        let census = parse_census(tsv);
        assert_eq!(census.len(), 2, "{:?}", census);
        let lines = drift(&terms, &census);
        assert!(
            lines.contains(&"UNMEASURED b.org MIT".to_string()),
            "{:?}",
            lines
        );
        assert!(
            lines.contains(&"STALE c.org GPL https://c".to_string()),
            "{:?}",
            lines
        );
        assert_eq!(lines.len(), 2, "{:?}", lines);
    }

    #[test]
    fn closed_vocab_accepts_the_nine_and_refuses_others() {
        let src = "terms CC-BY-4.0\n\n\
                   terms CC-BY-NC-SA-4.0\n\n\
                   terms CC0\n\n\
                   terms ODC-BY-1.0\n\n\
                   terms ODbL-1.0\n\n\
                   terms PD\n\n\
                   terms PDDL-1.0\n\n\
                   terms free-open\n\n\
                   terms own-work\n\n\
                   terms unbestimmt\n\n\
                   terms keine\n\n\
                   terms unknown\n\n\
                   terms cc0\n\n\
                   url https://example.org/x.bin\n";
        let violations = closed_vocab_violations(src);
        let values: Vec<&str> = violations.iter().map(|(_, v)| v.as_str()).collect();
        assert_eq!(
            values,
            vec!["unbestimmt", "keine", "unknown", "cc0"],
            "{:?}",
            violations
        );
        assert_eq!(violations[0].0, 19, "{:?}", violations);
        assert_eq!(terms_entries(src).len(), 13, "nine accepted plus four refused");
    }
}
