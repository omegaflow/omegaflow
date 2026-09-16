use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

struct Options {
    case_insensitive: bool,
    files_only: bool,
    count_only: bool,
    glob: Option<String>,
    pattern: String,
    root: String,
}

enum Parsed {
    Run(Options),
    Help,
}

fn parse_args(args: &[String]) -> Result<Parsed, String> {
    let mut case_insensitive = false;
    let mut files_only = false;
    let mut count_only = false;
    let mut glob: Option<String> = None;
    let mut pattern: Option<String> = None;
    let mut root = String::from(".");
    let mut root_given = false;
    let mut positional_only = false;
    let mut i = 1;
    while i < args.len() {
        let a = args[i].as_str();
        if positional_only {
            push_positional(&mut pattern, &mut root, &mut root_given, a)?;
        } else {
            match a {
                "--" => positional_only = true,
                "-i" => case_insensitive = true,
                "-l" => files_only = true,
                "-c" => count_only = true,
                "-g" => {
                    i += 1;
                    let Some(v) = args.get(i) else {
                        return Err("flag -g needs a <glob> value".to_string());
                    };
                    glob = Some(v.clone());
                }
                "-h" | "--help" => return Ok(Parsed::Help),
                _ if a.starts_with('-') && a.len() > 1 => {
                    return Err(format!("unknown flag: {a}"));
                }
                _ => push_positional(&mut pattern, &mut root, &mut root_given, a)?,
            }
        }
        i += 1;
    }
    let Some(pattern) = pattern else {
        return Err("no <pattern> given".to_string());
    };
    Ok(Parsed::Run(Options {
        case_insensitive,
        files_only,
        count_only,
        glob,
        pattern,
        root,
    }))
}

fn push_positional(
    pattern: &mut Option<String>,
    root: &mut String,
    root_given: &mut bool,
    a: &str,
) -> Result<(), String> {
    if pattern.is_none() {
        *pattern = Some(a.to_string());
        Ok(())
    } else if !*root_given {
        *root = a.to_string();
        *root_given = true;
        Ok(())
    } else {
        Err(format!("a second [dir|file] was given: {a}"))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = match parse_args(&args) {
        Ok(Parsed::Help) => {
            usage();
            std::process::exit(0);
        }
        Ok(Parsed::Run(o)) => o,
        Err(e) => {
            eprintln!("sgrep: {e}");
            usage();
            std::process::exit(2);
        }
    };
    let needle = if opts.case_insensitive {
        opts.pattern.to_lowercase()
    } else {
        opts.pattern
    };
    let mut matches: u64 = 0;
    grep_root(
        &opts.root,
        &needle,
        opts.case_insensitive,
        opts.files_only,
        opts.count_only,
        opts.glob.as_deref(),
        &mut matches,
    );
    if opts.count_only {
        println!("{}", matches);
    }
}

fn usage() {
    eprintln!("sgrep — content search over the live tree (git ls-files, no target/, no .git)");
    eprintln!("usage: sgrep [-i] [-l] [-c] [-g <glob>] <pattern> [dir|file]");
    eprintln!("flags:");
    eprintln!("  -i        case-insensitive (default is case-sensitive)");
    eprintln!("  -l        file paths only, one per file with a match");
    eprintln!("  -c        print only the total match count");
    eprintln!("  -g <glob> restrict to filenames matching the glob (e.g. '*.rs')");
    eprintln!("  -h|--help this text");
    eprintln!("  --        end of flags (a <pattern> starting with '-' needs this)");
    eprintln!("output: `path:line:text` — line numbers are in the output (there is no -n flag).");
    eprintln!("an unknown flag or a second [dir|file] is rejected, never a silent empty result.");
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
    if s.is_empty() { None } else { Some(s) }
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

    fn argv(parts: &[&str]) -> Vec<String> {
        std::iter::once("sgrep")
            .chain(parts.iter().copied())
            .map(String::from)
            .collect()
    }

    fn run(parts: &[&str]) -> Options {
        match parse_args(&argv(parts)) {
            Ok(Parsed::Run(o)) => o,
            Ok(Parsed::Help) => panic!("Help"),
            Err(e) => panic!("the args were rejected: {e}"),
        }
    }

    #[test]
    fn unknown_flag_is_rejected() {
        assert!(parse_args(&argv(&["-n", "pat", "file"])).is_err());
        assert!(parse_args(&argv(&["-r", "pat"])).is_err());
        assert!(parse_args(&argv(&["--line-number", "pat", "file"])).is_err());
    }

    #[test]
    fn second_path_is_rejected() {
        assert!(parse_args(&argv(&["pat", "a", "b"])).is_err());
    }

    #[test]
    fn flags_and_single_path_parse() {
        let o = run(&["-i", "pat", "dir"]);
        assert!(o.case_insensitive);
        assert_eq!(o.pattern, "pat");
        assert_eq!(o.root, "dir");
    }

    #[test]
    fn default_root_is_dot() {
        assert_eq!(run(&["pat"]).root, ".");
    }

    #[test]
    fn double_dash_allows_dash_pattern() {
        assert_eq!(run(&["--", "-n"]).pattern, "-n");
    }

    #[test]
    fn missing_glob_value_is_rejected() {
        assert!(parse_args(&argv(&["-g"])).is_err());
    }

    #[test]
    fn missing_pattern_is_rejected() {
        assert!(parse_args(&argv(&[])).is_err());
    }

    #[test]
    fn help_is_accepted() {
        assert!(matches!(parse_args(&argv(&["-h"])), Ok(Parsed::Help)));
    }

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
