use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process::exit;

const SOURCES: &str = "phi/sources.φ";
const CENSUS: &str = "state/river/license-census.tsv";

fn parse_terms(content: &str) -> HashSet<(String, String)> {
    let mut out = HashSet::new();
    for line in content.lines() {
        let mut fields = line.split_whitespace();
        if fields.next() != Some("terms") {
            continue;
        }
        if let (Some(class), Some(url)) = (fields.next(), fields.next()) {
            out.insert((class.to_string(), url.to_string()));
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
        .map(|(_, class, url)| (class.clone(), url.clone()))
        .collect();

    let mut unmeasured: Vec<(&String, &String)> = terms
        .iter()
        .filter(|pair| !measured.contains(*pair))
        .map(|(class, url)| (class, url))
        .collect();
    unmeasured.sort();

    let mut stale: Vec<&(String, String, String)> = census
        .iter()
        .filter(|(_, class, url)| !terms.contains(&(class.clone(), url.clone())))
        .collect();
    stale.sort();

    let mut lines = Vec::new();
    for (class, url) in unmeasured {
        lines.push(format!("UNMEASURED {} {}", class, url));
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
    fn joins_terms_against_census() {
        let src = "terms CC0 https://a\nterms MIT https://b\nfoo terms X https://c\n";
        let tsv = "# netloc\tclass\tterms-url\tmeasured\na.org\tCC0\thttps://a\t2026-10-07\nc.org\tGPL\thttps://c\t2026-10-07\n";
        let terms = parse_terms(src);
        assert_eq!(terms.len(), 2, "{:?}", terms);
        let census = parse_census(tsv);
        assert_eq!(census.len(), 2, "{:?}", census);
        let lines = drift(&terms, &census);
        assert!(
            lines.contains(&"UNMEASURED MIT https://b".to_string()),
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
