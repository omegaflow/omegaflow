use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;

const DEFAULT_REGISTER: &str = "phi/sources.φ";

fn usage() {
    eprintln!("usage: register_terms_apply <proposal.tsv> [--write] [--path <register>]");
    eprintln!("  TSV columns: TAG CAT HOST PROPOSED METHOD URL TERMS_REF (tab-separated)");
    eprintln!(
        "  sets each block's terms line whose `url` matches a row: `terms <proposed> <terms_ref>`"
    );
    eprintln!("  without --write it reports only; the register verdict stays Mountain's");
}

fn normalize(token: &str) -> String {
    let mut out = String::new();
    for c in token.chars() {
        match c {
            '(' | ')' => {}
            ' ' | '\t' => out.push('-'),
            _ => out.push(c),
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut write = false;
    let mut path = String::from(DEFAULT_REGISTER);
    let mut tsv: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--write" => write = true,
            "--path" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("register_terms_apply: --path needs a value");
                    exit(2);
                }
                path = args[i].clone();
            }
            other if other.starts_with('-') => {
                eprintln!("register_terms_apply: unknown flag '{}'", other);
                usage();
                exit(2);
            }
            other => {
                if tsv.is_none() {
                    tsv = Some(other.to_string());
                } else {
                    eprintln!("register_terms_apply: multiple tsv arguments");
                    usage();
                    exit(2);
                }
            }
        }
        i += 1;
    }
    let tsv = match tsv {
        Some(t) => t,
        None => {
            usage();
            exit(2);
        }
    };
    let tsv_text = match fs::read_to_string(&tsv) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("register_terms_apply: read {}: {}", tsv, e);
            exit(2);
        }
    };

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in tsv_text.lines() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') || line.starts_with("TAG\t") {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 7 {
            continue;
        }
        map.insert(
            f[5].trim().to_string(),
            (normalize(f[3]), f[6].trim().to_string()),
        );
    }
    eprintln!("register_terms_apply: {} proposal rows", map.len());

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("register_terms_apply: read {}: {}", path, e);
            exit(2);
        }
    };

    let mut out = String::new();
    let mut block_url: Option<String> = None;
    let mut replaced_this_block = false;
    let mut matched_block = false;
    let mut skip_clear = false;
    let mut n_replaced = 0usize;
    let mut n_matched = 0usize;
    let mut n_skipped_clear = 0usize;
    let mut n_matched_no_terms = 0usize;
    let mut n_blocks = 0usize;

    for line in content.lines() {
        let trimmed = line.trim_end();
        if trimmed.trim().is_empty() {
            if matched_block && !replaced_this_block && !skip_clear {
                n_matched_no_terms += 1;
            }
            out.push('\n');
            block_url = None;
            replaced_this_block = false;
            matched_block = false;
            skip_clear = false;
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("url ") {
            block_url = Some(rest.trim().to_string());
            n_blocks += 1;
            matched_block = map.contains_key(block_url.as_deref().unwrap_or(""));
            if matched_block {
                n_matched += 1;
            }
        }
        if matched_block && !replaced_this_block && !skip_clear && trimmed.starts_with("terms ") {
            let current = trimmed.split_whitespace().nth(1).unwrap_or("");
            if current == "unknown" || current == "no-license" {
                let clause = &map[block_url.as_deref().unwrap_or("")];
                out.push_str(&format!("terms {} {}", clause.0, clause.1));
                out.push('\n');
                replaced_this_block = true;
                n_replaced += 1;
                continue;
            }
            skip_clear = true;
            n_skipped_clear += 1;
        }
        out.push_str(trimmed);
        out.push('\n');
    }
    if matched_block && !replaced_this_block && !skip_clear {
        n_matched_no_terms += 1;
    }

    eprintln!(
        "register_terms_apply: {} blocks, {} url-matched, {} terms replaced (unknown/no-license), {} skipped (already clear), {} matched-without-terms",
        n_blocks, n_matched, n_replaced, n_skipped_clear, n_matched_no_terms
    );

    if write {
        match fs::write(&path, out) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("register_terms_apply: write {}: {}", path, e);
                exit(2);
            }
        }
        eprintln!("register_terms_apply: wrote {}", path);
    } else {
        eprintln!("register_terms_apply: report only (no --write)");
    }
}
