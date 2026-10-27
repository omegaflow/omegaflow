use std::fs;
use std::path::{Path, PathBuf};

const EXISTENCE: [&str; 4] = [".is_some()", ".is_none()", ".is_ok()", ".is_err()"];
const VALUE: [&str; 6] = ["==", "!=", "unwrap", "expect", ">=", "<="];

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn body_of(lines: &[&str], start: usize) -> Option<String> {
    let mut depth = 0i32;
    let mut opened = false;
    let mut body = String::new();
    for line in lines.iter().skip(start) {
        for ch in line.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    opened = true;
                }
                '}' => depth -= 1,
                _ => {}
            }
        }
        body.push_str(line);
        body.push('\n');
        if opened && depth == 0 {
            return Some(body);
        }
    }
    None
}

fn assert_arguments(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = body.as_bytes();
    let mut i = 0usize;
    while let Some(found) = body[i..].find("assert") {
        let mut j = i + found + "assert".len();
        while j < bytes.len() && bytes[j] != b'(' {
            if bytes[j] == b'\n' && !body[i + found..j].trim_end().ends_with('!') {
                break;
            }
            j += 1;
        }
        if j >= bytes.len() || bytes[j] != b'(' {
            i += found + "assert".len();
            continue;
        }
        let mut depth = 0i32;
        let start = j + 1;
        let mut end = start;
        while end < bytes.len() {
            match bytes[end] {
                b'(' => depth += 1,
                b')' => {
                    if depth == 0 {
                        break;
                    }
                    depth -= 1;
                }
                _ => {}
            }
            end += 1;
        }
        out.push(body[start..end].to_string());
        i = end.max(j + 1);
    }
    out
}

fn test_name(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let at = trimmed.find("fn ")?;
    if trimmed[..at].contains('(') {
        return None;
    }
    let rest = trimmed.get(at + "fn ".len()..)?;
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() { None } else { Some(name) }
}

fn scan(root: &Path, allows_mode: bool) -> Vec<String> {
    let mut files = Vec::new();
    rust_files(&root.join("src"), &mut files);
    if let Ok(entries) = fs::read_dir(root.join("tools")) {
        for entry in entries.flatten() {
            rust_files(&entry.path().join("src"), &mut files);
        }
    }
    files.sort();

    let mut findings = Vec::new();
    for path in &files {
        let Ok(text) = fs::read_to_string(path) else {
            continue;
        };
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            if allows_mode {
                if trimmed.starts_with("#[allow(") || trimmed.starts_with("#![allow(") {
                    findings.push(format!("{}:{}: allow", path.display(), i + 1));
                }
                continue;
            }
            if trimmed != "#[test]" && !trimmed.starts_with("#[tokio::test]") {
                continue;
            }
            let Some(name_line) = (i..lines.len().min(i + 4)).find(|n| {
                let l = lines[*n].trim_start();
                l.starts_with("fn ") || l.starts_with("async fn ")
            }) else {
                continue;
            };
            let Some(name) = test_name(lines[name_line]) else {
                continue;
            };
            let Some(body) = body_of(&lines, name_line) else {
                continue;
            };
            let args = assert_arguments(&body);
            if args.is_empty() {
                findings.push(format!(
                    "{}:{}: {} — no assertion",
                    path.display(),
                    name_line + 1,
                    name
                ));
                continue;
            }
            let existence_only = args.iter().all(|a| {
                EXISTENCE.iter().any(|e| a.contains(e)) && !VALUE.iter().any(|v| a.contains(v))
            });
            if existence_only {
                findings.push(format!(
                    "{}:{}: {} — asserts existence only",
                    path.display(),
                    name_line + 1,
                    name
                ));
            }
        }
    }
    findings
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = match args.iter().position(|a| a == "--root") {
        Some(i) => match args.get(i + 1) {
            Some(path) => path.clone(),
            None => {
                eprintln!("test_shape_scan: --root needs a path");
                std::process::exit(2);
            }
        },
        None => {
            let Ok(dir) = std::env::current_dir() else {
                eprintln!("test_shape_scan: the working directory is unreadable");
                std::process::exit(2);
            };
            dir.display().to_string()
        }
    };
    let allows_mode = args.iter().any(|a| a == "--allows");

    let findings = scan(Path::new(&root), allows_mode);
    for line in &findings {
        println!("{line}");
    }
    println!("test_shape_scan: {} finding(s)", findings.len());
    if !findings.is_empty() {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str, body: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("test_shape_scan_{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("src")).expect("fixture tree");
        fs::write(dir.join("src/lib.rs"), body).expect("fixture file");
        dir
    }

    #[test]
    fn an_existence_assertion_is_named() {
        let dir = fixture(
            "existence",
            "#[test]\nfn only_present() {\n    let v = Some(3.0);\n    assert!(v.is_some());\n}\n",
        );
        let findings = scan(&dir, false);
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert!(findings[0].contains("asserts existence only"), "{findings:?}");
    }

    #[test]
    fn a_value_assertion_passes() {
        let dir = fixture(
            "value",
            "#[test]\nfn carries_the_value() {\n    let v = Some(3.0);\n    assert_eq!(v, Some(3.0));\n}\n",
        );
        assert!(scan(&dir, false).is_empty());
    }

    #[test]
    fn a_test_without_any_assertion_is_named() {
        let dir = fixture("silent", "#[test]\nfn runs() {\n    let _ = 2 + 2;\n}\n");
        let findings = scan(&dir, false);
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert!(findings[0].contains("no assertion"), "{findings:?}");
    }

    #[test]
    fn an_allow_is_named_in_allows_mode() {
        let dir = fixture("allow", "#![allow(clippy::all)]\npub fn f() {}\n");
        let findings = scan(&dir, true);
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert!(findings[0].contains("allow"), "{findings:?}");
    }
}
