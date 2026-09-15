use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut case_insensitive = false;
    let mut files_only = false;
    let mut count_only = false;
    let mut glob: Option<String> = None;
    let mut pattern: Option<String> = None;
    let mut root = String::from(".");
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-i" => case_insensitive = true,
            "-l" => files_only = true,
            "-c" => count_only = true,
            "-g" => {
                i += 1;
                if i < args.len() {
                    glob = Some(args[i].clone());
                }
            }
            _ => {
                if pattern.is_none() {
                    pattern = Some(args[i].clone());
                } else {
                    root = args[i].clone();
                }
            }
        }
        i += 1;
    }
    let Some(pattern) = pattern else {
        eprintln!("usage: sgrep [-i] [-l] [-c] [-g <glob>] <pattern> [dir|file]");
        std::process::exit(2);
    };
    let needle = if case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern
    };
    let mut matches: u64 = 0;
    grep_root(
        &root,
        &needle,
        case_insensitive,
        files_only,
        count_only,
        glob.as_deref(),
        &mut matches,
    );
    if count_only {
        println!("{}", matches);
    }
}

fn grep_root(
    root: &str,
    needle: &str,
    ci: bool,
    files_only: bool,
    count_only: bool,
    glob: Option<&str>,
    matches: &mut u64,
) {
    if Path::new(root).is_file() {
        grep_file(root, needle, ci, files_only, count_only, matches);
        return;
    }
    let Some(top) = repo_root() else {
        return;
    };
    let Some(files) = repo_files(&top) else {
        return;
    };
    let root_prefix = root.trim_matches('/');
    for f in files {
        if !root_prefix.is_empty() && root_prefix != "." && !f.starts_with(root_prefix) {
            continue;
        }
        if let Some(g) = glob {
            let Some(file_name) = Path::new(&f).file_name() else {
                continue;
            };
            if !glob_match(&file_name.to_string_lossy(), g) {
                continue;
            }
        }
        let full = Path::new(&top).join(&f);
        grep_file(
            &full.to_string_lossy(),
            needle,
            ci,
            files_only,
            count_only,
            matches,
        );
    }
}

fn repo_root() -> Option<String> {
    let out = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn repo_files(top: &str) -> Option<Vec<String>> {
    let out = Command::new("git")
        .args([
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ])
        .current_dir(top)
        .output()
        .ok()?;
    Some(
        out.stdout
            .split(|&b| b == 0)
            .filter_map(|b| std::str::from_utf8(b).ok())
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect(),
    )
}

fn grep_file(
    path: &str,
    needle: &str,
    ci: bool,
    files_only: bool,
    count_only: bool,
    matches: &mut u64,
) {
    let Ok(file) = File::open(path) else {
        return;
    };
    let reader = BufReader::new(file);
    let mut file_matches: u64 = 0;
    for (idx, line) in reader.lines().enumerate() {
        let Ok(line) = line else {
            continue;
        };
        let hit = if ci {
            line.to_lowercase().contains(needle)
        } else {
            line.contains(needle)
        };
        if !hit {
            continue;
        }
        file_matches += 1;
        *matches += 1;
        if !count_only && !files_only {
            println!("{}:{}:{}", path, idx + 1, line);
        }
    }
    if files_only && file_matches > 0 {
        println!("{}", path);
    }
}

fn glob_match(name: &str, glob: &str) -> bool {
    if !glob.contains('*') && !glob.contains('?') {
        return name == glob;
    }
    match_star(name.as_bytes(), glob.as_bytes())
}

fn match_star(text: &[u8], pattern: &[u8]) -> bool {
    if pattern.is_empty() {
        return text.is_empty();
    }
    match pattern[0] {
        b'*' => (0..=text.len()).any(|i| match_star(&text[i..], &pattern[1..])),
        b'?' => !text.is_empty() && match_star(&text[1..], &pattern[1..]),
        c => !text.is_empty() && text[0] == c && match_star(&text[1..], &pattern[1..]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glob_suffix() {
        assert!(glob_match("foo.rs", "*.rs"));
        assert!(!glob_match("foo.rs", "*.toml"));
    }

    #[test]
    fn glob_middle_star() {
        assert!(glob_match("a/b/foo.rs", "*foo*"));
        assert!(glob_match("foo.rs", "f*.rs"));
        assert!(!glob_match("foo.rs", "f?.toml"));
    }

    #[test]
    fn glob_exact() {
        assert!(glob_match("foo.rs", "foo.rs"));
        assert!(!glob_match("foo.rs", "bar.rs"));
    }

    #[test]
    fn wildcard_question() {
        assert!(glob_match("foo.rs", "???.rs"));
        assert!(!glob_match("fo.rs", "???.rs"));
    }
}
