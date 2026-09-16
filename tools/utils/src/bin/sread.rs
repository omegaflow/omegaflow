use std::env;
use std::fs;
use std::io::{self, Write};

const DEFAULT_LIMIT: usize = 80;

fn usage() {
    eprintln!("sread — print a text file with line numbers (the fast local read)");
    eprintln!("usage: sread <file> [--offset <n>] [--limit <n>]");
    eprintln!("  --offset <n>  first line to print, 1-based (default 1)");
    eprintln!("  --limit <n>   lines to print (default {DEFAULT_LIMIT})");
    eprintln!("a binary file reports its size and stops; images/PDFs stay with the read tool");
}

fn valid_start(offset: usize) -> Option<usize> {
    if offset == 0 { None } else { Some(offset) }
}

fn window(text: &str, start: usize, limit: usize) -> Vec<String> {
    let end = start.saturating_add(limit);
    let mut out = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let ln = idx + 1;
        if ln < start {
            continue;
        }
        if ln >= end {
            break;
        }
        out.push(format!("{ln}: {line}"));
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut file: Option<String> = None;
    let mut offset = 1usize;
    let mut limit = DEFAULT_LIMIT;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                usage();
                return;
            }
            "--offset" => {
                i += 1;
                if let Some(v) = args.get(i).and_then(|s| s.parse().ok()) {
                    offset = v;
                }
            }
            "--limit" => {
                i += 1;
                if let Some(v) = args.get(i).and_then(|s| s.parse().ok()) {
                    limit = v;
                }
            }
            other => {
                if file.is_none() {
                    file = Some(other.to_string());
                }
            }
        }
        i += 1;
    }
    let Some(path) = file else {
        usage();
        std::process::exit(2);
    };
    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("sread: {path}: {e}");
            std::process::exit(2);
        }
    };
    if bytes[..bytes.len().min(8192)].contains(&0u8) {
        eprintln!(
            "sread: {path} is binary ({} bytes) — the read tool carries images and PDFs",
            bytes.len()
        );
        std::process::exit(0);
    }
    let text = String::from_utf8_lossy(&bytes);
    let total = text.lines().count();
    let Some(start) = valid_start(offset) else {
        eprintln!("sread: --offset is 1-based; 0 names no line");
        std::process::exit(2);
    };
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for line in window(&text, start, limit) {
        let _ = writeln!(out, "{line}");
    }
    let _ = writeln!(out, "(total {total} lines)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_is_one_based_and_bounded() {
        let text = "a\nb\nc\nd\ne\n";
        assert_eq!(
            window(text, 2, 2),
            vec!["2: b".to_string(), "3: c".to_string()]
        );
        assert_eq!(window(text, 1, 1), vec!["1: a".to_string()]);
        assert!(window(text, 9, 5).is_empty());
    }

    #[test]
    fn a_zero_offset_names_no_line() {
        assert_eq!(valid_start(0), None);
        assert_eq!(valid_start(1), Some(1));
        assert_eq!(valid_start(7), Some(7));
    }
}
