use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const SKIP_DIRS: &[&str] = &[
    ".git",
    "target",
    "node_modules",
    ".opencode",
    "__pycache__",
    ".cache",
    ".venv",
    "venv",
    ".local",
    ".config",
    "Library",
    "log",
    "storage",
    "session_diff",
    "tool-output",
    "tmp",
    "Trash",
];
const SNIPPET_CHARS: usize = 200;

struct State {
    scanned: u64,
    matched: u64,
    shown: usize,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut roots: Vec<String> = Vec::new();
    let mut lines_per_file = 2usize;
    let mut max_files = 40usize;
    let mut max_mb = 100u64;
    let mut keywords: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                i += 1;
                if let Some(r) = args.get(i) {
                    roots.push(r.clone());
                }
            }
            "--lines" => {
                i += 1;
                if let Some(n) = args.get(i).and_then(|s| s.parse().ok()) {
                    lines_per_file = n;
                }
            }
            "--files" => {
                i += 1;
                if let Some(n) = args.get(i).and_then(|s| s.parse().ok()) {
                    max_files = n;
                }
            }
            "--max-mb" => {
                i += 1;
                if let Some(n) = args.get(i).and_then(|s| s.parse().ok()) {
                    max_mb = n;
                }
            }
            other => keywords.push(other.to_string()),
        }
        i += 1;
    }

    if keywords.is_empty() {
        eprintln!(
            "usage: archive_search [--root <dir>]... [--lines <n>] [--files <n>] [--max-mb <n>] <keyword> [<keyword>...]"
        );
        std::process::exit(2);
    }
    if roots.is_empty() {
        roots.push(".".to_string());
    }

    let needle: Vec<String> = keywords.iter().map(|k| k.to_lowercase()).collect();

    let mut state = State {
        scanned: 0,
        matched: 0,
        shown: 0,
    };
    for root in &roots {
        walk(
            Path::new(root),
            &needle,
            lines_per_file,
            max_files,
            max_mb,
            &mut state,
        );
        if state.shown >= max_files {
            break;
        }
    }
    eprintln!(
        "archive_search: scanned {} files | matched {} | shown {}",
        state.scanned, state.matched, state.shown
    );
}

fn walk(
    dir: &Path,
    needle: &[String],
    lines_per_file: usize,
    max_files: usize,
    max_mb: u64,
    state: &mut State,
) {
    if state.shown >= max_files {
        return;
    }
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        paths.push(entry.path());
    }
    paths.sort();
    for path in paths {
        if state.shown >= max_files {
            return;
        }
        let name = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => continue,
        };
        if path.is_dir() {
            if SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            walk(&path, needle, lines_per_file, max_files, max_mb, state);
        } else {
            state.scanned += 1;
            if let Some(hits) = search_file(&path, needle, lines_per_file, max_mb) {
                state.matched += 1;
                if state.shown < max_files {
                    state.shown += 1;
                    println!("{}", path.display());
                    for line in hits {
                        println!("  {}", line);
                    }
                }
            }
        }
    }
}

fn search_file(
    path: &Path,
    needle: &[String],
    max_lines: usize,
    max_mb: u64,
) -> Option<Vec<String>> {
    let bytes = fs::read(path).ok()?;
    if bytes.len() > max_mb as usize * 1024 * 1024 {
        return None;
    }
    if is_binary(&bytes) {
        return None;
    }
    let text = String::from_utf8_lossy(&bytes);
    let mut hits: Vec<String> = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        if !line_matches(line, needle) {
            continue;
        }
        if hits.len() >= max_lines {
            break;
        }
        hits.push(format!(
            "{}: {}",
            idx + 1,
            truncate(line.trim(), SNIPPET_CHARS)
        ));
    }
    if hits.is_empty() {
        None
    } else {
        Some(hits)
    }
}

fn line_matches(line: &str, needle: &[String]) -> bool {
    let lower = line.to_lowercase();
    needle.iter().any(|n| lower.contains(n))
}

fn is_binary(bytes: &[u8]) -> bool {
    bytes[..bytes.len().min(8192)].contains(&0u8)
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max).collect();
        out.push('…');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_leaves_short_strings() {
        assert_eq!(truncate("short", 200), "short");
    }

    #[test]
    fn truncate_cuts_on_char_boundary() {
        let out = truncate(&"中".repeat(300), 10);
        assert_eq!(out.chars().count(), 11);
        assert!(out.ends_with('…'));
    }

    #[test]
    fn line_matches_is_case_insensitive_and_any_keyword() {
        let needle = vec!["icecube".to_string(), "telescope".to_string()];
        assert!(line_matches("an ICECUBE alert", &needle));
        assert!(line_matches("the Telescope Array", &needle));
        assert!(!line_matches("a plain line", &needle));
    }

    #[test]
    fn is_binary_detects_nul_byte() {
        assert!(is_binary(&[0x41, 0x00, 0x42]));
        assert!(!is_binary(b"plain text"));
    }

    #[test]
    fn search_file_finds_keyword_in_temp_file() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("archive_search_test_{}.txt", std::process::id()));
        fs::write(&path, "first line\nICECUBE alert here\nthird line\n").unwrap();
        let needle = vec!["icecube".to_string()];
        let hits = search_file(&path, &needle, 2, 100).unwrap();
        assert_eq!(hits.len(), 1);
        assert!(hits[0].contains("ICECUBE"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn search_file_skips_binary() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("archive_search_bin_{}.dat", std::process::id()));
        fs::write(&path, [0x41, 0x00, 0x42]).unwrap();
        let needle = vec!["a".to_string()];
        assert!(search_file(&path, &needle, 2, 100).is_none());
        let _ = fs::remove_file(&path);
    }
}
