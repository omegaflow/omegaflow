use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const REGISTER: &[(&str, &str)] = &[
    ("phi/sources.\u{3c6}", "live"),
    ("phi/dead_sources.\u{3c6}", "declined"),
    ("phi/blocked_sources.\u{3c6}", "blocked"),
    ("phi/witnesses.\u{3c6}", "witness"),
    ("phi/pipeline/ledger.\u{3c6}", "ledger"),
];

const REGISTER_DIRS: &[(&str, &str)] =
    &[("docs/handover", "handover"), ("docs/concepts", "concept")];

fn snippet(line: &str, max: usize) -> String {
    let trimmed = line.trim();
    if trimmed.chars().count() <= max {
        return trimmed.to_string();
    }
    let mut out: String = trimmed.chars().take(max).collect();
    out.push('\u{2026}');
    out
}

fn scan_file(path: &Path, class: &str, terms: &[String], out: &mut Vec<String>) -> usize {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    let mut n = 0;
    for (idx, line) in text.lines().enumerate() {
        let lower = line.to_lowercase();
        if terms.iter().any(|t| lower.contains(t)) {
            out.push(format!(
                "{}\t{}:{}\t{}",
                class,
                path.display(),
                idx + 1,
                snippet(line, 160)
            ));
            n += 1;
        }
    }
    n
}

fn scan_dir(dir: &Path, class: &str, terms: &[String], out: &mut Vec<String>) -> usize {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    let mut n = 0;
    for path in paths {
        if !path.is_file() {
            continue;
        }
        let name = match path.file_name() {
            Some(f) => f.to_string_lossy().to_string(),
            None => continue,
        };
        if name.ends_with(".md") {
            n += scan_file(&path, class, terms, out);
        }
    }
    n
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let terms: Vec<String> = args
        .iter()
        .filter(|a| !a.starts_with("--"))
        .map(|a| a.to_lowercase())
        .collect();
    if terms.is_empty() {
        eprintln!(
            "usage: register_lookup <term>...   (queries the live register: is X already measured/registered?)"
        );
        std::process::exit(2);
    }

    let mut out = Vec::new();
    let mut by_class: Vec<(&str, usize)> = Vec::new();
    for (path, class) in REGISTER {
        let n = scan_file(Path::new(path), class, &terms, &mut out);
        if n > 0 {
            by_class.push((class, n));
        }
    }
    for (dir, class) in REGISTER_DIRS {
        let n = scan_dir(Path::new(dir), class, &terms, &mut out);
        if n > 0 {
            by_class.push((class, n));
        }
    }

    for line in &out {
        println!("{}", line);
    }
    let summary: Vec<String> = by_class
        .iter()
        .map(|(class, n)| format!("{} {}", class, n))
        .collect();
    println!(
        "register_lookup: {} hits for {:?} [{}]",
        out.len(),
        terms,
        summary.join(", ")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snippet_leaves_short_lines_alone() {
        assert_eq!(snippet("  a short line  ", 200), "a short line");
    }

    #[test]
    fn snippet_cuts_on_char_boundary() {
        let out = snippet(&"\u{4e2d}".repeat(300), 10);
        assert_eq!(out.chars().count(), 11);
        assert!(out.ends_with('\u{2026}'));
    }

    #[test]
    fn scan_file_matches_a_term_case_insensitively() {
        let dir = env::temp_dir();
        let path = dir.join(format!("register_lookup_{}.md", std::process::id()));
        fs::write(&path, "the Hilbert transform\nplain line\n").unwrap();
        let mut out = Vec::new();
        let n = scan_file(&path, "test", &["hilbert".to_string()], &mut out);
        assert_eq!(n, 1);
        assert!(out[0].starts_with("test\t"));
        assert!(out[0].contains("Hilbert"));
        let _ = fs::remove_file(&path);
    }
}
