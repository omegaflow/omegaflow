use std::env;
use std::fs;
use std::process::exit;

const DEFAULT_REGISTER: &str = "phi/sources.φ";

fn usage() {
    eprintln!("usage: register_block_remove <pattern> [--write] [--path <register>]");
    eprintln!("  drops every source block whose url or origin contains <pattern>");
    eprintln!(
        "  phi/sources.φ is sources-only — a removed source leaves no tombstone (git carries it)"
    );
    eprintln!("  without --write it reports only");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut write = false;
    let mut path = String::from(DEFAULT_REGISTER);
    let mut pattern: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--write" => write = true,
            "--path" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("register_block_remove: --path needs a value");
                    exit(2);
                }
                path = args[i].clone();
            }
            other if other.starts_with('-') => {
                eprintln!("register_block_remove: unknown flag '{other}'");
                usage();
                exit(2);
            }
            other => {
                if pattern.is_none() {
                    pattern = Some(other.to_string());
                } else {
                    eprintln!("register_block_remove: multiple patterns");
                    usage();
                    exit(2);
                }
            }
        }
        i += 1;
    }
    let pattern = match pattern {
        Some(p) => p,
        None => {
            usage();
            exit(2);
        }
    };

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("register_block_remove: read {path}: {e}");
            exit(2);
        }
    };

    let mut out = String::new();
    let mut block: Vec<&str> = Vec::new();
    let mut n_blocks = 0usize;
    let mut removed = 0usize;

    let flush =
        |block: &mut Vec<&str>, out: &mut String, n_blocks: &mut usize, removed: &mut usize| {
            if block.is_empty() {
                return;
            }
            *n_blocks += 1;
            let hit = block.iter().any(|line| {
                line.strip_prefix("url ")
                    .or_else(|| line.strip_prefix("origin "))
                    .is_some_and(|v| v.contains(pattern.as_str()))
            });
            if hit {
                *removed += 1;
            } else {
                for line in block.iter() {
                    out.push_str(line);
                    out.push('\n');
                }
                out.push('\n');
            }
            block.clear();
        };

    for line in content.lines() {
        let trimmed = line.trim_end();
        if trimmed.trim().is_empty() {
            flush(&mut block, &mut out, &mut n_blocks, &mut removed);
            continue;
        }
        block.push(trimmed);
    }
    flush(&mut block, &mut out, &mut n_blocks, &mut removed);

    eprintln!("register_block_remove: {n_blocks} blocks, {removed} removed (pattern '{pattern}')");

    if write {
        match fs::write(&path, out) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("register_block_remove: write {path}: {e}");
                exit(2);
            }
        }
        eprintln!("register_block_remove: wrote {path}");
    } else {
        eprintln!("register_block_remove: report only (no --write)");
    }
}
