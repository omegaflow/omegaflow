use omegaflow::llm_gate::{home_of, scan_home};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn walk_md(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if path.is_dir() {
            walk_md(&path, out);
        } else if path.extension().map_or(false, |e| e == "md") {
            out.push(path);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: home_scan <root-dir>");
        std::process::exit(1);
    }
    let root = PathBuf::from(&args[1]);
    let mut files = Vec::new();
    walk_md(&root, &mut files);
    files.sort();

    let mut drift = 0usize;
    let mut classified = 0usize;
    for f in &files {
        let path = f.to_string_lossy();
        let Some(home) = home_of(&path) else { continue };
        classified += 1;
        let content = fs::read_to_string(f).unwrap_or_default();
        match scan_home(&path, &content) {
            Some(v) => {
                drift += 1;
                println!("DRIFT  home={:<8} {}", home, f.display());
                for line in v.feedback.lines() {
                    println!("         {}", line.trim());
                }
            }
            None => {
                println!("OK     home={:<8} {}", home, f.display());
            }
        }
    }
    println!(
        "\nclassified={} drift={} clean={}",
        classified,
        drift,
        classified - drift
    );
}
