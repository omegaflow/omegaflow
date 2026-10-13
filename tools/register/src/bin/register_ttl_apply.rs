use std::env;
use std::fs;
use std::process::exit;

const DEFAULT_REGISTER: &str = "phi/sources.φ";

fn usage() {
    eprintln!("usage: register_ttl_apply <rules.tsv> [--write] [--path <register>]");
    eprintln!("  TSV columns: MATCH<TAB>VALUE");
    eprintln!("  MATCH = substring of the block's `url`; VALUE = `no-cadence` or seconds");
    eprintln!("  first matching rule wins; rewrites the block's ttl/no-cadence line (or adds one)");
    eprintln!("  without --write the register is only reported (source-first cadence verdict stays the owner's)");
}

fn apply_block(block: &[String], rules: &[(String, String)]) -> (Vec<String>, bool, bool) {
    let Some(url) = block
        .iter()
        .find_map(|l| l.strip_prefix("url "))
        .map(|s| s.trim().to_string())
    else {
        return (block.to_vec(), false, false);
    };
    let value = rules
        .iter()
        .find(|(m, _)| !m.is_empty() && url.contains(m.as_str()))
        .map(|(_, v)| v.clone());
    let mut lines = block.to_vec();
    let mut matched = false;
    let mut changed = false;
    if let Some(v) = value {
        matched = true;
        let replacement = if v == "no-cadence" {
            "no-cadence".to_string()
        } else {
            format!("ttl {v}")
        };
        if let Some(pos) = lines
            .iter()
            .position(|l| l.starts_with("ttl ") || l == "no-cadence")
        {
            if lines[pos] != replacement {
                lines[pos] = replacement;
                changed = true;
            }
        } else {
            lines.push(replacement);
            changed = true;
        }
    }
    (lines, matched, changed)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut write = false;
    let mut path = String::from(DEFAULT_REGISTER);
    let mut rules_path: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--write" => write = true,
            "--path" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("register_ttl_apply: --path needs a value");
                    exit(2);
                }
                path = args[i].clone();
            }
            other if other.starts_with('-') => {
                eprintln!("register_ttl_apply: unknown flag '{}'", other);
                usage();
                exit(2);
            }
            other => {
                if rules_path.is_none() {
                    rules_path = Some(other.to_string());
                } else {
                    eprintln!("register_ttl_apply: multiple rule arguments");
                    usage();
                    exit(2);
                }
            }
        }
        i += 1;
    }
    let rules_path = match rules_path {
        Some(r) => r,
        None => {
            usage();
            exit(2);
        }
    };
    let rules_text = match fs::read_to_string(&rules_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("register_ttl_apply: read {}: {}", rules_path, e);
            exit(2);
        }
    };
    let mut rules: Vec<(String, String)> = Vec::new();
    for line in rules_text.lines() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((m, v)) = line.split_once('\t') {
            let m = m.trim();
            let v = v.trim();
            if !m.is_empty() && !v.is_empty() {
                rules.push((m.to_string(), v.to_string()));
            }
        }
    }
    eprintln!("register_ttl_apply: {} rules", rules.len());

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("register_ttl_apply: read {}: {}", path, e);
            exit(2);
        }
    };

    let mut out = String::new();
    let mut block: Vec<String> = Vec::new();
    let mut n_blocks = 0usize;
    let mut n_matched = 0usize;
    let mut n_changed = 0usize;

    for line in content.lines() {
        if line.trim().is_empty() {
            if !block.is_empty() {
                let (new_block, matched, changed) = apply_block(&block, &rules);
                n_blocks += 1;
                if matched {
                    n_matched += 1;
                }
                if changed {
                    n_changed += 1;
                }
                for l in &new_block {
                    out.push_str(l);
                    out.push('\n');
                }
                block.clear();
            }
            out.push('\n');
        } else {
            block.push(line.to_string());
        }
    }
    if !block.is_empty() {
        let (new_block, matched, changed) = apply_block(&block, &rules);
        n_blocks += 1;
        if matched {
            n_matched += 1;
        }
        if changed {
            n_changed += 1;
        }
        for l in &new_block {
            out.push_str(l);
            out.push('\n');
        }
    }

    eprintln!(
        "register_ttl_apply: {} blocks, {} rule-matched, {} changed",
        n_blocks, n_matched, n_changed
    );

    if write {
        match fs::write(&path, out) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("register_ttl_apply: write {}: {}", path, e);
                exit(2);
            }
        }
        eprintln!("register_ttl_apply: wrote {}", path);
    } else {
        eprintln!("register_ttl_apply: report only (no --write)");
    }
}
