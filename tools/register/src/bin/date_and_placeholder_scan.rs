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

fn scan(text: &str, env: &HashMap<String, String>) -> Vec<String> {
    let markers: Vec<(String, String)> = live_markers();
    let mut findings = Vec::new();
    let mut probes = 0usize;
    let mut block_line = 0usize;
    let mut has_compiler = false;

    let mut flush = |url: &str, line: usize, has_compiler: bool| {
        if has_compiler {
            return;
        }
        probes += 1;
        let coords = COORD_MARKERS
            .iter()
            .map(|m| (m.to_string(), "0.0".to_string()))
            .collect::<Vec<_>>();
        let rendered = apply(&apply(url, &markers), &coords);
        if let Some(key) = unresolved_key(&rendered, env) {
            findings.push(format!("line {line}: {{{key}}} is neither a marker nor an environment key"));
        }
    };

    for (i, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("source ") || trimmed.starts_with("block ") {
            block_line = i + 1;
            has_compiler = false;
            continue;
        }
        if trimmed.starts_with("compiler ") || trimmed.starts_with("format ") {
            has_compiler = true;
            continue;
        }
        if let Some(url) = trimmed.strip_prefix("url ") {
            flush(url.trim(), if block_line > 0 { block_line } else { i + 1 }, has_compiler);
        }
    }
    println!(
        "date_and_placeholder_scan: {probes} probe url(s), {} finding(s)",
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

    fn markers() -> Vec<(String, String)> {
        live_markers()
    }

    #[test]
    fn a_probe_with_a_marker_passes_after_the_markers_are_applied() {
        let text = "block a\nurl https://example.org/data?start={now}&stop={today}\n";
        let findings = scan(text, &HashMap::new());
        assert!(findings.is_empty(), "{findings:?}");
    }

    #[test]
    fn a_probe_with_an_unknown_placeholder_is_named() {
        let text = "block a\nurl https://example.org/{nwo}\n";
        let findings = scan(text, &HashMap::new());
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert!(findings[0].contains("nwo"), "{findings:?}");
    }

    #[test]
    fn a_compiler_block_is_not_checked() {
        let text = "block a\ncompiler euvi\nurl https://example.org/{YYYY}/{DDD}/\n";
        assert!(scan(text, &HashMap::new()).is_empty());
    }
}
