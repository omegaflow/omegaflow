use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use omegaflow::archivar::naming::extract_netloc;

const SOURCES: &str = "phi/sources.φ";
const DEFAULT_OUT_DIR: &str = "state/mountain/sources-repo-license/";

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

fn block_terms(block: &[&str]) -> Option<(String, Option<String>)> {
    for line in block {
        let mut fields = line.split_whitespace();
        if fields.next() == Some("terms") {
            let token = fields.next()?.to_string();
            let url = fields.next().map(|value| value.to_string());
            return Some((token, url));
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

struct Attribution {
    lines: Vec<String>,
    netlocs: usize,
    terms: usize,
    no_terms: usize,
}

fn attribute(content: &str) -> Attribution {
    let mut lines: BTreeSet<String> = BTreeSet::new();
    let mut netlocs: BTreeSet<String> = BTreeSet::new();
    let mut terms: usize = 0;
    let mut no_terms: usize = 0;
    for block in blocks(content) {
        let netloc = block_token(&block, "url")
            .and_then(extract_netloc)
            .map(|value| value.to_string());
        if let Some(netloc) = &netloc {
            netlocs.insert(netloc.clone());
        }
        match block_terms(&block) {
            Some((token, terms_url)) => {
                terms += 1;
                let source = match block_token(&block, "url") {
                    Some(url) => url.to_string(),
                    None => match block_identity(&block) {
                        Some(identity) => identity,
                        None => continue,
                    },
                };
                match terms_url {
                    Some(url) => lines.insert(format!("{source} | {token} | {url}")),
                    None => lines.insert(format!("{source} | {token}")),
                };
            }
            None => {
                if netloc.is_some() {
                    no_terms += 1;
                }
            }
        }
    }
    Attribution {
        lines: lines.into_iter().collect(),
        netlocs: netlocs.len(),
        terms,
        no_terms,
    }
}

fn license_text(attribution: &Attribution) -> String {
    let mut out = String::new();
    out.push_str("# omegaflow/sources — per-source attribution\n");
    out.push_str("# Generated from the `terms` directives in phi/sources.φ.\n");
    out.push_str("# Each line: <source-url> | <terms-token> | <terms-url>\n");
    out.push_str(
        "# One line per source; a source repeated carries disagreeing terms — never smoothed.\n",
    );
    out.push('\n');
    for line in &attribution.lines {
        out.push_str(line);
        out.push('\n');
    }
    out.push('\n');
    out.push_str(&format!("{} blocks carry no terms\n", attribution.no_terms));
    out
}

fn readme_text(attribution: &Attribution) -> String {
    format!(
        "# omegaflow/sources\n\n\
         Flattened CDN assets referenced by `phi/sources.φ`; the register is `phi/sources.φ`.\n\
         Source attribution is generated into `LICENSE` from the `terms` lines.\n\
         Measured: {} netlocs, {} terms directives, {} blocks carry no terms.\n",
        attribution.netlocs, attribution.terms, attribution.no_terms
    )
}

fn out_dir_from_args() -> PathBuf {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if let Some(value) = arg.strip_prefix("--out-dir=") {
            return PathBuf::from(value);
        }
        if arg == "--out-dir" {
            if let Some(value) = args.next() {
                return PathBuf::from(value);
            }
        }
    }
    PathBuf::from(DEFAULT_OUT_DIR)
}

fn main() {
    let out_dir = out_dir_from_args();
    let sources = match fs::read_to_string(SOURCES) {
        Ok(text) => text,
        Err(_) => {
            println!("sources_repo_license: {SOURCES} absent");
            exit(2);
        }
    };
    let attribution = attribute(&sources);
    if fs::create_dir_all(&out_dir).is_err() {
        println!("sources_repo_license: {} absent", out_dir.display());
        exit(2);
    }
    let license_path = out_dir.join("LICENSE");
    if fs::write(&license_path, license_text(&attribution)).is_err() {
        println!("sources_repo_license: {} absent", license_path.display());
        exit(2);
    }
    let readme_path = out_dir.join("README.md");
    if fs::write(&readme_path, readme_text(&attribution)).is_err() {
        println!("sources_repo_license: {} absent", readme_path.display());
        exit(2);
    }
    println!(
        "sources_repo_license: netlocs {}, terms {}, no-terms {} -> {}",
        attribution.netlocs,
        attribution.terms,
        attribution.no_terms,
        out_dir.display()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributes_one_line_per_source_terms_pair_and_counts_the_remainder() {
        let src = "url https://www.example.org/data/a.csv\n\
                   terms CC-BY-4.0 https://example.org/licence\n\n\
                   url https://other.example/tap/sync?QUERY=x\n";
        let attribution = attribute(src);
        assert_eq!(
            attribution.lines,
            vec![
                "https://www.example.org/data/a.csv | CC-BY-4.0 | https://example.org/licence"
                    .to_string()
            ],
            "{:?}",
            attribution.lines
        );
        assert_eq!(attribution.netlocs, 1);
        assert_eq!(attribution.terms, 1);
        assert_eq!(attribution.no_terms, 1);
    }

    #[test]
    fn keeps_one_line_per_source_under_one_netloc() {
        let src = "url https://example.org/data/a.csv\n\
                   terms CC-BY-4.0 https://example.org/licence\n\n\
                   url https://example.org/data/b.csv\n\
                   terms CC-BY-SA-4.0 https://example.org/licence\n";
        let attribution = attribute(src);
        assert_eq!(
            attribution.lines,
            vec![
                "https://example.org/data/a.csv | CC-BY-4.0 | https://example.org/licence"
                    .to_string(),
                "https://example.org/data/b.csv | CC-BY-SA-4.0 | https://example.org/licence"
                    .to_string(),
            ],
            "{:?}",
            attribution.lines
        );
        assert_eq!(attribution.netlocs, 1);
        assert_eq!(attribution.terms, 2);
        assert_eq!(attribution.no_terms, 0);
    }
}
