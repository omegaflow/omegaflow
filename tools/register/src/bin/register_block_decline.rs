use std::env;
use std::fs;
use std::process::exit;

const DEFAULT_REGISTER: &str = "phi/sources.φ";

fn usage() {
    eprintln!(
        "usage: register_block_decline <pattern> --class <class> [--write] [--path <register>]"
    );
    eprintln!("  drops every source block whose url or origin contains <pattern> and appends");
    eprintln!("  `decline <class> <origin-or-url>` to the disposition section (end of file)");
    eprintln!("  without --write it reports only");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut write = false;
    let mut path = String::from(DEFAULT_REGISTER);
    let mut class: Option<String> = None;
    let mut pattern: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--write" => write = true,
            "--class" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("register_block_decline: --class needs a value");
                    exit(2);
                }
                class = Some(args[i].clone());
            }
            "--path" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("register_block_decline: --path needs a value");
                    exit(2);
                }
                path = args[i].clone();
            }
            other if other.starts_with('-') => {
                eprintln!("register_block_decline: unknown flag '{}'", other);
                usage();
                exit(2);
            }
            other => {
                if pattern.is_none() {
                    pattern = Some(other.to_string());
                } else {
                    eprintln!("register_block_decline: multiple patterns");
                    usage();
                    exit(2);
                }
            }
        }
        i += 1;
    }
    let (pattern, class) = match (pattern, class) {
        (Some(p), Some(c)) => (p, c),
        _ => {
            usage();
            exit(2);
        }
    };

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("register_block_decline: read {}: {}", path, e);
            exit(2);
        }
    };

    let mut out = String::new();
    let mut block: Vec<&str> = Vec::new();
    let mut declined: Vec<String> = Vec::new();
    let mut n_blocks = 0usize;

    let flush = |block: &mut Vec<&str>,
                 out: &mut String,
                 declined: &mut Vec<String>,
                 n_blocks: &mut usize| {
        if block.is_empty() {
            return;
        }
        *n_blocks += 1;
        let mut hit = false;
        let mut origin: Option<String> = None;
        let mut url: Option<String> = None;
        for line in block.iter() {
            if let Some(rest) = line.strip_prefix("origin ") {
                origin = Some(rest.trim().to_string());
            }
            if let Some(rest) = line.strip_prefix("url ") {
                url = Some(rest.trim().to_string());
            }
            if let Some(v) = line
                .strip_prefix("origin ")
                .or_else(|| line.strip_prefix("url "))
            {
                if v.contains(pattern.as_str()) {
                    hit = true;
                }
            }
        }
        if hit {
            if let Some(key) = origin.or(url) {
                declined.push(format!("decline {} {}", class, key));
            }
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
            flush(&mut block, &mut out, &mut declined, &mut n_blocks);
            continue;
        }
        block.push(trimmed);
    }
    flush(&mut block, &mut out, &mut declined, &mut n_blocks);

    let mut seen = std::collections::HashSet::new();
    declined.retain(|d| seen.insert(d.clone()));
    for d in &declined {
        out.push_str(d);
        out.push('\n');
    }

    eprintln!(
        "register_block_decline: {} blocks, {} declined (pattern '{}', class '{}')",
        n_blocks,
        declined.len(),
        pattern,
        class
    );

    if write {
        match fs::write(&path, out) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("register_block_decline: write {}: {}", path, e);
                exit(2);
            }
        }
        eprintln!("register_block_decline: wrote {}", path);
    } else {
        eprintln!("register_block_decline: report only (no --write)");
    }
}
