use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process::exit;

const SOURCES: &str = "phi/sources.φ";
const CENSUS: &str = "state/river/license-census.tsv";

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
}
