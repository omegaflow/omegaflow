use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

const TOKENS: &[&str] = &[
    concat!("\"ea", "rth\""),
    concat!("EARTH_RAD", "IUS"),
    concat!("637813", "7.0"),
    concat!("637813", "6.6"),
    concat!("force_const", "ants"),
];

fn strip_comment(line: &str) -> &str {
    match line.find("//") {
        Some(i) => &line[..i],
        None => line,
    }
}

fn brace_delta(line: &str) -> i32 {
    let code = strip_comment(line);
    let mut depth: i32 = 0;
    for ch in code.chars() {
        match ch {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
    }
    depth
}

fn scan_content(content: &str) -> Vec<(usize, String)> {
    let mut hits: Vec<(usize, String)> = Vec::new();
    let mut in_test = false;
    let mut awaiting_open = false;
    let mut depth: i32 = 0;
    for (idx, raw) in content.lines().enumerate() {
        let lineno = idx + 1;
        if in_test {
            depth += brace_delta(raw);
            if depth <= 0 {
                in_test = false;
                depth = 0;
            }
            continue;
        }
        if awaiting_open {
            if raw.contains('{') {
                awaiting_open = false;
                depth = brace_delta(raw);
                if depth > 0 {
                    in_test = true;
                } else {
                    depth = 0;
                }
            }
            continue;
        }
        if raw.trim_start().starts_with("//") {
            continue;
        }
        let code = strip_comment(raw);
        if code.contains("#[cfg(test)]") {
            if raw.contains('{') {
                depth = brace_delta(raw);
                if depth > 0 {
                    in_test = true;
                } else {
                    depth = 0;
                }
            } else {
                awaiting_open = true;
            }
            continue;
        }
        for token in TOKENS {
            if code.contains(token) {
                hits.push((lineno, (*token).to_string()));
            }
        }
    }
    hits
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if matches!(path.file_name().and_then(|n| n.to_str()), Some("tests")) {
                continue;
            }
            collect_rs(&path, out);
        } else if matches!(path.extension().and_then(|e| e.to_str()), Some("rs")) {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with("tests.rs") {
                    continue;
                }
            }
            out.push(path);
        }
    }
}

fn media_table_present() -> bool {
    let path = Path::new("src/mathematikerin/media.rs");
    match fs::read_to_string(path) {
        Ok(content) => content.contains("match body_name") && content.contains("=> Some(m("),
        Err(_) => false,
    }
}

fn main() {
    let count_only = env::args().any(|a| a == "--count");
    let fail = env::args().any(|a| a == "--fail");

    let mut files: Vec<PathBuf> = Vec::new();
    collect_rs(Path::new("src"), &mut files);
    files.sort();

    let mut lines: Vec<String> = Vec::new();
    for path in &files {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => continue,
        };
        for (lineno, token) in scan_content(&content) {
            lines.push(format!(
                "CLEAN-TREE {}:{}: {}",
                path.display(),
                lineno,
                token
            ));
        }
    }
    if media_table_present() {
        lines.push("MEDIA-TABLE src/mathematikerin/media.rs".to_string());
    }

    let hits = lines.len();
    if count_only {
        println!("{}", hits);
    } else {
        for line in &lines {
            println!("{}", line);
        }
        println!("clean_tree: files {} | hits {}", files.len(), hits);
    }

    if fail && hits > 0 {
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hit_in_code_is_flagged() {
        let hits = scan_content("let x = \"earth\";\n");
        assert_eq!(hits, vec![(1, "\"earth\"".to_string())]);
    }

    #[test]
    fn a_hit_inside_cfg_test_module_is_not_flagged() {
        let src = "#[cfg(test)]\nmod tests {\n    fn t() { let x = \"earth\"; }\n}\n";
        assert!(scan_content(src).is_empty());
    }

    #[test]
    fn a_hit_in_a_comment_is_not_flagged() {
        assert!(scan_content("// \"earth\" placeholder\n").is_empty());
    }
}
