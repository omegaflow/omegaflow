use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROOTS: &[&str] = &["docs"];
const SKIP_DIRS: &[&str] = &["archiv", "reference", "target"];

struct Doc {
    path: String,
    lines: usize,
    words: usize,
    bytes: usize,
}

fn measure(path: &Path) -> Option<Doc> {
    let text = fs::read_to_string(path).ok()?;
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    Some(Doc {
        path: path.to_string_lossy().to_string(),
        lines,
        words,
        bytes: text.len(),
    })
}

fn walk(dir: &Path, docs: &mut Vec<Doc>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        let name = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => continue,
        };
        if path.is_dir() {
            if SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            walk(&path, docs);
        } else if name.ends_with(".md") {
            if let Some(doc) = measure(&path) {
                docs.push(doc);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let top = args
        .iter()
        .position(|a| a == "--top")
        .and_then(|i| args.get(i + 1))
        .and_then(|n| n.parse::<usize>().ok())
        .unwrap_or(30);
    let mut roots: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--root" {
            if let Some(r) = args.get(i + 1) {
                roots.push(r.clone());
            }
            i += 2;
            continue;
        }
        i += 1;
    }
    if roots.is_empty() {
        roots = ROOTS.iter().map(|s| s.to_string()).collect();
    }

    let mut docs: Vec<Doc> = Vec::new();
    for root in &roots {
        walk(Path::new(root), &mut docs);
    }
    docs.sort_by(|a, b| b.words.cmp(&a.words));

    let total_words: usize = docs.iter().map(|d| d.words).sum();
    let total_lines: usize = docs.iter().map(|d| d.lines).sum();
    for doc in docs.iter().take(top) {
        println!(
            "{:>7} words\t{:>6} lines\t{:>8} bytes\t{}",
            doc.words, doc.lines, doc.bytes, doc.path
        );
    }
    if docs.is_empty() {
        println!("bloat_scan: 0 docs");
    } else {
        println!(
            "bloat_scan: {} docs, {} words, {} lines (avg {:.0} words/doc)",
            docs.len(),
            total_words,
            total_lines,
            total_words as f64 / docs.len() as f64
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure_counts_lines_words_bytes() {
        let dir = env::temp_dir();
        let path = dir.join(format!("bloat_scan_{}.md", std::process::id()));
        fs::write(&path, "one two three\nfour five\n").unwrap();
        let doc = measure(&path).unwrap();
        assert_eq!(doc.lines, 2);
        assert_eq!(doc.words, 5);
        assert_eq!(doc.bytes, 24);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn measure_reads_a_void_path_as_none() {
        assert!(measure(Path::new("/nonexistent/bloat_scan.md")).is_none());
    }
}
