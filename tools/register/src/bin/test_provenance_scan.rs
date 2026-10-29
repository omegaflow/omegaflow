use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const SOURCES: &str = include_str!("../test_sources.tsv");

struct Test {
    file: String,
    name: String,
    lines: usize,
    foreign: Vec<String>,
    numeric: bool,
}

fn markers() -> Vec<String> {
    SOURCES
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.to_lowercase())
        .collect()
}

fn rust_files(root: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(root) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
            out.push(path);
        }
    }
}

fn has_digit(text: &str) -> bool {
    text.chars().any(|c| c.is_ascii_digit())
}

fn carries_number(text: &str) -> bool {
    let mut found_assert = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("assert") {
            found_assert = true;
        }
        if found_assert && has_digit(t) {
            return true;
        }
    }
    false
}

fn scan_file(path: &Path, markers: &[String], out: &mut Vec<Test>) -> usize {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    let mut attributes = 0usize;
    let lines: Vec<&str> = text.lines().collect();
    let file = path.to_string_lossy().to_string();
    let mut index = 0usize;
    while index < lines.len() {
        if lines[index].trim() != "#[test]" {
            index += 1;
            continue;
        }
        attributes += 1;
        let mut fn_index = index + 1;
        while fn_index < lines.len() && !lines[fn_index].contains("fn ") {
            fn_index += 1;
        }
        if fn_index >= lines.len() {
            break;
        }
        let header = lines[fn_index].trim();
        let name = header
            .split("fn ")
            .nth(1)
            .and_then(|r| r.split('(').next())
            .unwrap_or("unnamed")
            .to_string();
        let indent = lines[fn_index].len() - lines[fn_index].trim_start().len();
        let mut end = fn_index + 1;
        while end < lines.len() {
            let t = lines[end].trim();
            let lead = lines[end].len() - lines[end].trim_start().len();
            if t == "}" && lead <= indent {
                break;
            }
            end += 1;
        }
        let body = lines[fn_index..=end.min(lines.len() - 1)].join("\n");
        let context_start = fn_index.saturating_sub(3);
        let context = format!(
            "{}\n{}",
            lines[context_start..fn_index].join("\n"),
            body
        );
        let foreign: Vec<String> = markers
            .iter()
            .filter(|m| context.to_lowercase().contains(m.as_str()))
            .cloned()
            .collect();
        out.push(Test {
            file: file.clone(),
            name,
            lines: end - fn_index,
            foreign,
            numeric: carries_number(&body),
        });
        index = end + 1;
    }
    attributes
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let roots: Vec<String> = if args.is_empty() {
        vec!["src".to_string(), "tools".to_string()]
    } else {
        args
    };
    let markers = markers();
    let mut tests: Vec<Test> = Vec::new();
    let mut attributes = 0usize;
    for root in &roots {
        let mut files = Vec::new();
        rust_files(Path::new(root), &mut files);
        for file in files {
            attributes += scan_file(&file, &markers, &mut tests);
        }
    }
    let mut by_file: BTreeMap<String, usize> = BTreeMap::new();
    for t in &tests {
        *by_file.entry(t.file.clone()).or_insert(0) += 1;
    }
    let foreign_number = tests
        .iter()
        .filter(|t| !t.foreign.is_empty() && t.numeric)
        .count();
    let foreign_name = tests
        .iter()
        .filter(|t| !t.foreign.is_empty() && !t.numeric)
        .count();
    let self_only = tests.iter().filter(|t| t.foreign.is_empty()).count();
    println!(
        "tests: {} in {} files ([#test] attributes seen: {attributes})",
        tests.len(),
        by_file.len()
    );
    println!("  foreign source and a number compared : {foreign_number}");
    println!("  foreign source named, no number      : {foreign_name}");
    println!("  no foreign source in reach           : {self_only}");
    let mut scope: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for t in &tests {
        let key = if t.file.starts_with("src/archivar") {
            "reader"
        } else if t.file.starts_with("src/mathematikerin") {
            "measure"
        } else if t.file.contains("_compiler.rs") {
            "compiler"
        } else if t.file.starts_with("tools/register") {
            "register"
        } else {
            "other"
        };
        let e = scope.entry(key).or_insert((0, 0));
        e.0 += 1;
        if !t.foreign.is_empty() && t.numeric {
            e.1 += 1;
        }
    }
    for (name, (all, foreign)) in &scope {
        println!("  {:<10} {:>5} tests, {:>3} against a foreign number", name, all, foreign);
    }
    let compilers: Vec<&Test> = tests
        .iter()
        .filter(|t| t.file.contains("_compiler.rs"))
        .collect();
    let cn = compilers
        .iter()
        .filter(|t| !t.foreign.is_empty() && t.numeric)
        .count();
    let compiler_files: std::collections::BTreeSet<&str> =
        compilers.iter().map(|t| t.file.as_str()).collect();
    let compiler_files_with_source: std::collections::BTreeSet<&str> = compilers
        .iter()
        .filter(|t| !t.foreign.is_empty())
        .map(|t| t.file.as_str())
        .collect();
    println!("compiler tests: {}", compilers.len());
    println!("  foreign source and a number compared : {cn}");
    println!(
        "  compiler files naming a foreign source : {} of {}",
        compiler_files_with_source.len(),
        compiler_files.len()
    );
    let mut per_file: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for t in &compilers {
        let e = per_file.entry(t.file.as_str()).or_insert((0, 0));
        e.0 += 1;
        if !t.foreign.is_empty() && t.numeric {
            e.1 += 1;
        }
    }
    for (file, (all, foreign)) in &per_file {
        if *foreign > 0 {
            println!("  {:<70} {:>3}/{:<3} {}", file, foreign, all, "");
        }
    }
    if std::env::var("OMEGAFLOW_LIST").is_ok() {
        for t in &tests {
            if !t.foreign.is_empty() && t.numeric {
                println!(
                    "{}\t{}\t{}\t{} lines",
                    t.file,
                    t.name,
                    t.foreign.join(","),
                    t.lines
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_number_inside_an_assertion_counts_and_a_name_alone_does_not() {
        assert!(carries_number("assert!((dpsi - 1.0e-6).abs() < 1.0e-13);"));
        assert!(!carries_number("assert!(path.ends_with(\"mocc\"));"));
        assert!(!carries_number("let rows = parse(&bytes);"));
    }

    #[test]
    fn the_marker_list_is_lowercase_and_carries_no_house_words() {
        let markers = markers();
        assert!(markers.iter().any(|m| m == "sofa"));
        assert!(markers.iter().any(|m| m == "iers"));
        assert!(!markers.iter().any(|m| m == "witness"));
        assert!(!markers.iter().any(|m| m == "reference"));
        assert!(markers.iter().all(|m| m == &m.to_lowercase()));
    }
}
