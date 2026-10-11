use std::env;
use std::fs;

const DEFAULT_REGISTER: &str = "phi/sources.φ";
const PDS3_FORMAT: &str = "format pds3_fixed_width";
const PDS3_COMPILER: &str = "compiler tools/harvest/src/bin/pds3_fixed_width_compiler.rs";

fn usage() {
    eprintln!("usage: register_add_compiler [path] [--write]");
    eprintln!("  report only, or --write to insert the pds3_fixed_width compiler directive");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = String::from(DEFAULT_REGISTER);
    let mut write = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--write" => write = true,
            "-h" | "--help" => {
                usage();
                return;
            }
            other if other.starts_with('-') => {
                eprintln!("register_add_compiler: unknown flag '{}'", other);
                usage();
                std::process::exit(2);
            }
            other => {
                if path == DEFAULT_REGISTER {
                    path = other.to_string();
                } else {
                    eprintln!("register_add_compiler: multiple paths");
                    std::process::exit(2);
                }
            }
        }
        i += 1;
    }
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("register_add_compiler: read {}: {}", path, e);
            std::process::exit(2);
        }
    };

    let lines: Vec<&str> = content.lines().collect();
    let mut out = String::with_capacity(content.len());
    let mut inserted = 0usize;
    let mut i = 0usize;
    while i < lines.len() {
        if lines[i].trim().is_empty() {
            out.push('\n');
            i += 1;
            continue;
        }
        let start = i;
        while i < lines.len() && !lines[i].trim().is_empty() {
            i += 1;
        }
        let block = &lines[start..i];
        let has_format = block.iter().any(|l| l.trim() == PDS3_FORMAT);
        let has_compiler = block
            .iter()
            .any(|l| l.trim_start().starts_with("compiler "));
        for line in block {
            out.push_str(line);
            out.push('\n');
            if has_format && !has_compiler && line.trim() == PDS3_FORMAT {
                out.push_str(PDS3_COMPILER);
                out.push('\n');
                inserted += 1;
            }
        }
    }

    println!(
        "register_add_compiler {path}: {inserted} pds3_fixed_width block(s) without a compiler directive"
    );
    if write {
        if let Err(e) = fs::write(&path, &out) {
            eprintln!("register_add_compiler: write {}: {}", path, e);
            std::process::exit(2);
        }
        println!("register_add_compiler: {path} rewritten");
    }
}
