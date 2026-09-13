use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROOTS: &[&str] = &["docs", "phi"];
const VOCAB_PATH: &str = "tools/utils/src/bin/giveup_vocab.txt";
const SKIP_DIRS: &[&str] = &[".git", "target", "node_modules", ".opencode", "tool-output"];

struct Indicator {
    word: String,
    class: String,
}

fn load_vocab(path: &str) -> Vec<Indicator> {
    let mut out = Vec::new();
    if let Ok(text) = fs::read_to_string(path) {
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some((word, class)) = trimmed.split_once('\t') {
                let word = word.trim();
                let class = class.trim();
                if !word.is_empty() && !class.is_empty() {
                    out.push(Indicator {
                        word: word.to_lowercase(),
                        class: class.to_string(),
                    });
                }
            }
        }
    }
    out
}

fn classify<'a>(line: &str, vocab: &'a [Indicator]) -> Vec<&'a Indicator> {
    let lower = line.to_lowercase();
    vocab.iter().filter(|i| lower.contains(&i.word)).collect()
}

fn snippet(line: &str, max: usize) -> String {
    let trimmed = line.trim();
    if trimmed.chars().count() <= max {
        return trimmed.to_string();
    }
    let mut out: String = trimmed.chars().take(max).collect();
    out.push('\u{2026}');
    out
}

struct Site {
    path: String,
    line: usize,
    classes: Vec<String>,
    words: Vec<String>,
    snippet: String,
}

struct Scan {
    sites: Vec<Site>,
    by_class: BTreeMap<String, usize>,
    by_file: BTreeMap<String, usize>,
}

fn scan_text(path: &str, text: &str, vocab: &[Indicator], scan: &mut Scan) {
    for (idx, line) in text.lines().enumerate() {
        let hits = classify(line, vocab);
        if hits.is_empty() {
            continue;
        }
        let classes: Vec<String> = hits.iter().map(|h| h.class.clone()).collect();
        let words: Vec<String> = hits.iter().map(|h| h.word.clone()).collect();
        for class in &classes {
            *scan.by_class.entry(class.clone()).or_insert(0) += 1;
        }
        *scan.by_file.entry(path.to_string()).or_insert(0) += 1;
        scan.sites.push(Site {
            path: path.to_string(),
            line: idx + 1,
            classes,
            words,
            snippet: snippet(line, 160),
        });
    }
}

fn walk(dir: &Path, vocab: &[Indicator], scan: &mut Scan) {
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
            walk(&path, vocab, scan);
        } else if name.ends_with(".md") || name.ends_with(".rs") || name.ends_with('\u{3c6}') {
            if let Ok(text) = fs::read_to_string(&path) {
                scan_text(&path.to_string_lossy(), &text, vocab, scan);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let summary = args.iter().any(|a| a == "--summary");
    let mut roots: Vec<String> = Vec::new();
    let mut vocab_path = VOCAB_PATH.to_string();
    let mut class_filter: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                if let Some(r) = args.get(i + 1) {
                    roots.push(r.clone());
                }
                i += 2;
                continue;
            }
            "--vocab" => {
                if let Some(v) = args.get(i + 1) {
                    vocab_path = v.clone();
                }
                i += 2;
                continue;
            }
            "--class" => {
                if let Some(c) = args.get(i + 1) {
                    class_filter = Some(c.clone());
                }
                i += 2;
                continue;
            }
            _ => i += 1,
        }
    }
    if roots.is_empty() {
        roots = ROOTS.iter().map(|s| s.to_string()).collect();
    }

    let vocab = load_vocab(&vocab_path);
    let mut scan = Scan {
        sites: Vec::new(),
        by_class: BTreeMap::new(),
        by_file: BTreeMap::new(),
    };
    for root in &roots {
        walk(Path::new(root), &vocab, &mut scan);
    }

    if summary {
        println!(
            "giving-up vocabulary — {} words, {} sites",
            vocab.len(),
            scan.sites.len()
        );
        println!("-- by class --");
        for (class, n) in &scan.by_class {
            println!("{:>5}  {}", n, class);
        }
        println!("-- by file (top) --");
        let mut files: Vec<(&String, &usize)> = scan.by_file.iter().collect();
        files.sort_by(|a, b| b.1.cmp(a.1));
        for (file, n) in files.iter().take(30) {
            println!("{:>5}  {}", n, file);
        }
    } else {
        let mut shown = 0usize;
        for site in &scan.sites {
            if let Some(filter) = &class_filter {
                if !site.classes.iter().any(|c| c == filter) {
                    continue;
                }
            }
            println!(
                "{}:{}\t[{}]\t{}\t{}",
                site.path,
                site.line,
                site.classes.join(","),
                site.words.join(","),
                site.snippet
            );
            shown += 1;
        }
        println!("giving-up vocabulary — {} sites shown", shown);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vocab() -> Vec<Indicator> {
        vec![
            Indicator {
                word: "alpha".to_string(),
                class: "honest-face".to_string(),
            },
            Indicator {
                word: "beta".to_string(),
                class: "request-only".to_string(),
            },
        ]
    }

    #[test]
    fn classify_matches_a_word_to_its_class() {
        let v = vocab();
        let hits = classify("this line carries alpha and beta", &v);
        assert!(hits.iter().any(|h| h.class == "honest-face"));
        assert!(hits.iter().any(|h| h.class == "request-only"));
    }

    #[test]
    fn classify_is_case_insensitive() {
        let v = vocab();
        let hits = classify("ALPHA here", &v);
        assert!(hits.iter().any(|h| h.class == "honest-face"));
    }

    #[test]
    fn classify_leaves_plain_text_alone() {
        let v = vocab();
        assert!(classify("the measurement flows as 0.0", &v).is_empty());
    }

    #[test]
    fn scan_text_counts_sites_and_classes() {
        let text = "alpha line\nplain line\nbeta line\n";
        let mut scan = Scan {
            sites: Vec::new(),
            by_class: BTreeMap::new(),
            by_file: BTreeMap::new(),
        };
        scan_text("x.md", text, &vocab(), &mut scan);
        assert_eq!(scan.sites.len(), 2);
        assert_eq!(scan.by_class.get("honest-face"), Some(&1));
        assert_eq!(scan.by_class.get("request-only"), Some(&1));
    }

    #[test]
    fn vocab_lines_read_word_and_class() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("giveup_vocab_{}.txt", std::process::id()));
        fs::write(&path, "# comment\nalpha\thonest-face\nbeta\trequest-only\n").unwrap();
        let loaded = load_vocab(&path.to_string_lossy());
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].word, "alpha");
        assert_eq!(loaded[1].class, "request-only");
        let _ = fs::remove_file(&path);
    }
}
