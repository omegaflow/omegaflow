use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const MATRIX_WORKFLOWS: [&str; 6] = [
    "planetary-odf-cdn.yml",
    "gaia-xp-full-cdn.yml",
    "noaa-isd-allstations-cdn.yml",
    "noaa-gsod-allstations-cdn.yml",
    "noaa-ghcn-allstations-cdn.yml",
    "physionet-cdn.yml",
];

fn root() -> Option<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let candidate = manifest.parent().and_then(|p| p.parent())?;
    if candidate.join(".github/workflows").is_dir() {
        return Some(candidate.to_path_buf());
    }
    let cwd = env::current_dir().ok()?;
    if cwd.join(".github/workflows").is_dir() {
        return Some(cwd);
    }
    None
}

fn workflow_header(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with("jobs:") {
            break;
        }
        out.push(line);
    }
    out
}

fn workflow_concurrency_block<'a>(header: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut block: Option<Vec<&str>> = None;
    let mut in_block = false;
    for &line in header {
        if line.starts_with("concurrency:") {
            in_block = true;
            block = Some(vec![line]);
            continue;
        }
        if in_block {
            if line.starts_with(' ') || line.starts_with('\t') {
                if let Some(b) = block.as_mut() {
                    b.push(line);
                }
            } else {
                in_block = false;
            }
        }
    }
    block
}

fn contract_violations(root: &Path) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let ci = root.join(".github/workflows/ci-check.yml");
    match fs::read_to_string(&ci) {
        Ok(text) => {
            let header = workflow_header(&text);
            match workflow_concurrency_block(&header) {
                Some(block) if block.join("\n").contains("cancel-in-progress: false") => {}
                Some(_) => out.push(format!(
                    "{}: the measurement workflow carries cancel-in-progress: true — a run is cut mid-flight, no per-SHA verdict forms (the f8cdca96 lesson)",
                    ci.display()
                )),
                None => out.push(format!(
                    "{}: no workflow-level concurrency block — the per-SHA verdict contract is not declared",
                    ci.display()
                )),
            }
        }
        Err(_) => out.push(format!(
            "{}: unreadable — the contract is unmeasured (0 honored)",
            ci.display()
        )),
    }
    for name in MATRIX_WORKFLOWS {
        let path = root.join(".github/workflows").join(name);
        match fs::read_to_string(&path) {
            Ok(text) => {
                let header = workflow_header(&text);
                if workflow_concurrency_block(&header).is_some() {
                    out.push(format!(
                        "{}: workflow-level concurrency — a multi-asset matrix scopes concurrency to the job (release, asset), not the workflow (the 08cbdb74 lesson)",
                        path.display()
                    ));
                }
            }
            Err(_) => out.push(format!(
                "{}: unreadable — the contract is unmeasured (0 honored)",
                path.display()
            )),
        }
    }
    out
}

fn main() {
    let Some(root) = root() else {
        eprintln!(
            "no repository root carries .github/workflows — the contract is unmeasured (0 honored)"
        );
        std::process::exit(2);
    };
    let violations = contract_violations(&root);
    println!("=== concurrency-contract — the measured lesson, held per workflow ===");
    if violations.is_empty() {
        println!(
            "all concurrency contracts hold (ci-check: false; the six matrix workflows: job-level)"
        );
        return;
    }
    for v in &violations {
        println!("{v}");
    }
    println!("[{} contract violation(s)]", violations.len());
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrency_contracts_hold_in_the_tree() {
        let root = root().expect("a repository root carries .github/workflows");
        let v = contract_violations(&root);
        assert!(
            v.is_empty(),
            "concurrency contracts violated:\n{}",
            v.join("\n")
        );
    }

    #[test]
    fn workflow_concurrency_block_is_read() {
        let text = "name: x\nconcurrency:\n  group: g\n  cancel-in-progress: false\njobs:\n  a:\n";
        let header = workflow_header(text);
        let block = workflow_concurrency_block(&header).expect("a workflow-level block");
        assert!(block.join("\n").contains("cancel-in-progress: false"));
    }

    #[test]
    fn job_level_concurrency_is_not_workflow_level() {
        let text = "name: x\non:\n  workflow_dispatch:\njobs:\n  a:\n    concurrency:\n      group: g\n";
        let header = workflow_header(text);
        assert!(workflow_concurrency_block(&header).is_none());
    }

    #[test]
    fn a_workflow_level_block_without_false_is_named() {
        let text = "name: x\nconcurrency:\n  group: g\n  cancel-in-progress: true\njobs:\n  a:\n";
        let header = workflow_header(text);
        let block = workflow_concurrency_block(&header).expect("a workflow-level block");
        assert!(!block.join("\n").contains("cancel-in-progress: false"));
    }
}
