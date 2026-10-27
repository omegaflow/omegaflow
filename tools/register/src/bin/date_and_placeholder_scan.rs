use std::collections::HashMap;
use std::fs;
use std::path::Path;

use omegaflow::archivar::fetch::{live_markers, unresolved_key};
use omegaflow::archivar::render::COORD_MARKERS;

fn apply(template: &str, markers: &[(String, String)]) -> String {
    let mut out = template.to_string();
    for (key, value) in markers {
        out = out.replace(key.as_str(), value);
    }
    out
}

struct Block {
    line: usize,
    url: String,
    has_compiler: bool,
}

fn blocks(text: &str) -> Vec<Block> {
    let mut out: Vec<Block> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("url ") {
            out.push(Block {
                line: i + 1,
                url: rest.trim().to_string(),
                has_compiler: false,
            });
            continue;
        }
        if let Some(last) = out.last_mut()
            && trimmed.starts_with("compiler ")
        {
            last.has_compiler = true;
        }
    }
    out
}

fn scan(text: &str, env: &HashMap<String, String>) -> Vec<String> {
    let markers = live_markers();
    let coords: Vec<(String, String)> = COORD_MARKERS
        .iter()
        .map(|m| (m.to_string(), "0.0".to_string()))
        .collect();
    let mut findings = Vec::new();
    let mut probes = 0usize;
    for block in blocks(text) {
        if block.has_compiler {
            continue;
        }
        probes += 1;
        let rendered = apply(&apply(&block.url, &markers), &coords);
        let mut unknown: Vec<String> = Vec::new();
        let mut rest = rendered.as_str();
        while let Some(start) = rest.find('{') {
            let Some(end) = rest[start..].find('}') else {
                break;
            };
            let name = &rest[start + 1..start + end];
            let is_secret = name
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
            if !is_secret {
                unknown.push(name.to_string());
            }
            rest = &rest[start + end + 1..];
        }
        for name in unknown {
            findings.push(format!(
                "line {}: {{{name}}} is neither a marker nor an environment key",
                block.line
            ));
        }
    }
    println!(
        "date_and_placeholder_scan: {probes} probe block(s), {} finding(s)",
        findings.len()
    );
    findings
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let register = match args.iter().position(|a| a == "--register") {
        Some(i) => match args.get(i + 1) {
            Some(p) => p.clone(),
            None => {
                eprintln!("date_and_placeholder_scan: --register needs a path");
                std::process::exit(2);
            }
        },
        None => "phi/sources.φ".to_string(),
    };
    let Ok(text) = fs::read_to_string(Path::new(&register)) else {
        eprintln!("date_and_placeholder_scan: {register} is unreadable");
        std::process::exit(2);
    };
    let env: HashMap<String, String> = std::env::vars().collect();
    let findings = scan(&text, &env);
    for f in &findings {
        println!("{f}");
    }
    if !findings.is_empty() {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_probe_with_markers_passes_once_they_are_applied() {
        let text = "url https://example.org/data?start={now}&stop={today}\nterms x https://e.org\n";
        assert!(scan(text, &HashMap::new()).is_empty());
    }

    #[test]
    fn a_probe_with_an_unknown_placeholder_is_named() {
        let text = "url https://example.org/{nwo}\nterms x https://e.org\n";
        let findings = scan(text, &HashMap::new());
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert!(findings[0].contains("nwo"), "{findings:?}");
    }

    #[test]
    fn a_compiler_block_is_not_checked() {
        let text = "url https://example.org/{YYYY}/{DDD}/\nformat euvi\ncompiler tools/harvest/src/bin/euvi_compiler.rs\n";
        assert!(scan(text, &HashMap::new()).is_empty());
    }

    #[test]
    fn two_blocks_are_two_probes() {
        let text = "url https://a.example/{now}\nurl https://b.example/{today}\n";
        let found = blocks(text);
        assert_eq!(found.len(), 2, "{}", found.len());
    }
}
