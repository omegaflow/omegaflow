use std::env;
use std::fs;
use std::path::Path;

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
        eprintln!("usage: sgrep [-i] [-l] [-c] [-g <glob>] <pattern> [dir]");
        std::process::exit(2);
    };
    let needle = if case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern
    };
    let mut matches: u64 = 0;
    let mut files_hit: u64 = 0;
    walk(
        Path::new(&root),
        &needle,
        case_insensitive,
        files_only,
        count_only,
        glob.as_deref(),
        &mut matches,
        &mut files_hit,
    );
    if count_only {
        println!("{}", matches);
    }
}

fn walk(
    dir: &Path,
    needle: &str,
    ci: bool,
    files_only: bool,
    count_only: bool,
    glob: Option<&str>,
    matches: &mut u64,
    files_hit: &mut u64,
) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut names: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        names.push(entry.path().to_string_lossy().to_string());
    }
    names.sort();
    for name in names {
        let path = Path::new(&name);
        let file_name = path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default();
        if path.is_dir() {
            if matches!(
                file_name.as_str(),
                ".git" | "target" | "node_modules" | ".opencode"
            ) {
                continue;
            }
            walk(
                path, needle, ci, files_only, count_only, glob, matches, files_hit,
            );
        } else if glob.is_none() || glob_match(&file_name, glob.unwrap()) {
            let before = *matches;
            grep_file(path, needle, ci, files_only, count_only, matches);
            if *matches > before {
                *files_hit += 1;
            }
        }
    }
}

fn grep_file(
    path: &Path,
    needle: &str,
    ci: bool,
    files_only: bool,
    count_only: bool,
    matches: &mut u64,
) {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return,
    };
    let mut file_matches: u64 = 0;
    for (idx, line) in text.lines().enumerate() {
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
            println!("{}:{}:{}", path.display(), idx + 1, line);
        }
    }
    if files_only && file_matches > 0 {
        println!("{}", path.display());
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
