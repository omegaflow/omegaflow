use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn work_root() -> PathBuf {
    if let Some(dir) = env::var_os("OMEGAFLOW_WORK").filter(|v| !v.is_empty()) {
        return PathBuf::from(dir);
    }
    if let Some(home) = env::var_os("HOME").filter(|v| !v.is_empty()) {
        return PathBuf::from(home).join("projects/omegaflow-work");
    }
    PathBuf::from("omegaflow-work")
}

fn code_root() -> PathBuf {
    if let Some(dir) = env::var_os("OMEGAFLOW_CODE").filter(|v| !v.is_empty()) {
        return PathBuf::from(dir);
    }
    if let Some(home) = env::var_os("HOME").filter(|v| !v.is_empty()) {
        return PathBuf::from(home).join("projects/omegaflow");
    }
    PathBuf::from(".")
}

fn bins_under(root: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for family in ["tools", "firmware"] {
        let Ok(crates) = fs::read_dir(root.join(family)) else {
            continue;
        };
        for krate in crates.flatten() {
            let dir = krate.path().join("src/bin");
            let Ok(files) = fs::read_dir(&dir) else {
                continue;
            };
            for file in files.flatten() {
                let path = file.path();
                if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                    continue;
                }
                let Some(name) = path.file_stem().and_then(|s| s.to_str()) else {
                    continue;
                };
                if name == "main" {
                    continue;
                }
                let krate_name = krate.file_name().to_string_lossy().to_string();
                out.push((format!("{family}/{krate_name}"), name.to_string()));
            }
        }
    }
    out.sort();
    out
}

fn words(text: &str) -> BTreeSet<&str> {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .filter(|w| w.len() >= 4)
        .collect()
}

fn read_tree(path: &Path, out: &mut String) {
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if matches!(name.as_ref(), ".git" | "target" | "sources" | "node_modules" | "build") {
                continue;
            }
            read_tree(&p, out);
        } else if file_type.is_file() && p.extension().and_then(|e| e.to_str()) == Some("md") {
            if let Ok(text) = fs::read_to_string(&p) {
                out.push_str(&text);
                out.push('\n');
            }
        }
    }
}

fn doc_words(root: &Path, work: &Path) -> BTreeSet<String> {
    let mut text = String::new();
    read_tree(root, &mut text);
    if work.exists() {
        read_tree(work, &mut text);
    }
    words(&text).into_iter().map(|w| w.to_string()).collect()
}

fn workflow_words(root: &Path) -> BTreeSet<String> {
    let mut text = String::new();
    let dir = root.join(".github/workflows");
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Ok(body) = fs::read_to_string(entry.path()) {
                text.push_str(&body);
                text.push('\n');
            }
        }
    }
    words(&text).into_iter().map(|w| w.to_string()).collect()
}

fn compiler_names(register: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in register.lines() {
        let Some(rest) = line.strip_prefix("compiler ") else {
            continue;
        };
        let path = rest.trim();
        if path == "none" {
            continue;
        }
        if let Some(name) = Path::new(path).file_stem().and_then(|s| s.to_str()) {
            out.insert(name.to_string());
        }
    }
    out
}

struct Row {
    krate: String,
    name: String,
    ci: bool,
    register: bool,
    docs: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut root = code_root();
    let mut list: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                i += 1;
                if let Some(r) = args.get(i) {
                    root = PathBuf::from(r);
                }
            }
            "--list" => {
                i += 1;
                match args.get(i) {
                    Some(kind) => list = Some(kind.clone()),
                    None => {
                        eprintln!("tool_census: --list needs ci, register, docs or none");
                        std::process::exit(2);
                    }
                }
            }
            "-h" | "--help" => {
                println!(
                    "usage: tool_census [--root <dir>] [--list ci|register|docs|none]\n\
                     \n\
                     Counts the binaries under tools/*/src/bin and firmware/*/src/bin and asks\n\
                     three callers: a workflow under .github/workflows, a `compiler` line in\n\
                     phi/measurements.phi, and any .md in the tree."
                );
                return;
            }
            other => {
                eprintln!("tool_census: unknown argument {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let register_path = root.join("phi/measurements.\u{03c6}");
    let register = match fs::read_to_string(&register_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!(
                "tool_census: {} reads not: {}",
                register_path.display(),
                e
            );
            std::process::exit(2);
        }
    };

    let ci = workflow_words(&root);
    let work = work_root();
    let docs = doc_words(&root, &work);
    let register_names = compiler_names(&register);

    let rows: Vec<Row> = bins_under(&root)
        .into_iter()
        .map(|(krate, name)| Row {
            ci: ci.contains(&name),
            register: register_names.contains(&name),
            docs: docs.contains(&name),
            krate,
            name,
        })
        .collect();

    if let Some(kind) = list {
        for row in &rows {
            let hit = match kind.as_str() {
                "ci" => row.ci,
                "register" => row.register,
                "docs" => row.docs,
                "none" => !row.ci && !row.register && !row.docs,
                other => {
                    eprintln!("tool_census: --list takes ci, register, docs or none, not {other}");
                    std::process::exit(2);
                }
            };
            if hit {
                println!("{}\t{}", row.krate, row.name);
            }
        }
        return;
    }

    let mut crates: BTreeMap<String, [usize; 5]> = BTreeMap::new();
    for row in &rows {
        let slot = crates.entry(row.krate.clone()).or_insert([0; 5]);
        slot[0] += 1;
        if row.ci {
            slot[1] += 1;
        }
        if row.register {
            slot[2] += 1;
        }
        if row.docs {
            slot[3] += 1;
        }
        if !row.ci && !row.register && !row.docs {
            slot[4] += 1;
        }
    }

    println!(
        "tool_census — {} binaries in {} crates (docs: {} + {})",
        rows.len(),
        crates.len(),
        root.display(),
        work.display()
    );
    println!("crate                              bins     ci  register    docs    none");
    for (krate, slot) in &crates {
        println!(
            "{krate:<32} {:>6} {:>6} {:>9} {:>7} {:>7}",
            slot[0], slot[1], slot[2], slot[3], slot[4]
        );
    }
    let mut sums = [0usize; 5];
    for slot in crates.values() {
        for (i, n) in slot.iter().enumerate() {
            sums[i] += n;
        }
    }
    println!(
        "{:<32} {:>6} {:>6} {:>9} {:>7} {:>7}",
        "total", sums[0], sums[1], sums[2], sums[3], sums[4]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_lines_yield_basenames() {
        let register = "compiler tools/harvest/src/bin/aodn_compiler.rs\ncompiler none\nblock x\n";
        let names = compiler_names(register);
        assert_eq!(names.len(), 1);
        assert!(names.contains("aodn_compiler"));
    }

    #[test]
    fn short_words_are_not_callers() {
        let words = words("uses xfer and aodn_compiler");
        assert!(words.contains("aodn_compiler"));
        assert!(!words.contains("xfer"));
    }
}
