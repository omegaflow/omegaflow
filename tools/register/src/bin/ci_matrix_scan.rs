use std::fs;
use std::path::Path;

fn package_name(cargo_toml: &Path) -> Option<String> {
    let text = fs::read_to_string(cargo_toml).ok()?;
    text.lines()
        .find_map(|line| line.trim().strip_prefix("name = \""))
        .and_then(|rest| rest.strip_suffix('"'))
        .map(|name| name.to_string())
}

fn workspace_members(root_toml: &str) -> Vec<String> {
    let mut members = Vec::new();
    let mut inside = false;
    for line in root_toml.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("members = [") {
            inside = true;
        }
        if inside {
            let mut rest = trimmed;
            while let Some(start) = rest.find('"') {
                let after = &rest[start + 1..];
                let Some(end) = after.find('"') else {
                    break;
                };
                members.push(after[..end].to_string());
                rest = &after[end + 1..];
            }
            if trimmed.contains(']') {
                break;
            }
        }
    }
    members
}

fn nextest_lines(workflow: &str) -> Vec<String> {
    workflow
        .lines()
        .filter(|line| line.contains("nextest run"))
        .map(|line| line.trim().to_string())
        .collect()
}

fn packages_in(line: &str) -> Vec<String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "-p" || tokens[i] == "--package" {
            if let Some(name) = tokens.get(i + 1) {
                out.push(name.trim_matches('"').to_string());
            }
            i += 2;
            continue;
        }
        i += 1;
    }
    out
}

fn gaps(members: &[(String, String)], lines: &[String], root_package: &str) -> Vec<String> {
    let mut gaps = Vec::new();
    for (path, name) in members {
        let covered = lines.iter().any(|line| {
            if line.contains("--bin") {
                return false;
            }
            let named = packages_in(line);
            if named.is_empty() {
                return name == root_package;
            }
            named.iter().any(|p| p == name)
        });
        if !covered {
            gaps.push(format!("{path} ({name}) runs in no nextest step"));
        }
    }
    for line in lines {
        if line.contains("--bin") {
            gaps.push(format!("a --bin filter leaves binaries untested: {line}"));
        }
    }
    gaps
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = match args.iter().position(|a| a == "--root") {
        Some(i) => match args.get(i + 1) {
            Some(path) => path.clone(),
            None => {
                eprintln!("ci_matrix_scan: --root needs a path");
                std::process::exit(2);
            }
        },
        None => {
            let Ok(dir) = std::env::current_dir() else {
                eprintln!("ci_matrix_scan: the working directory is unreadable");
                std::process::exit(2);
            };
            dir.display().to_string()
        }
    };
    let root = Path::new(&root);

    let Ok(root_toml) = fs::read_to_string(root.join("Cargo.toml")) else {
        println!("ci_matrix_scan: {} carries no Cargo.toml", root.display());
        std::process::exit(2);
    };
    let Some(root_package) = package_name(&root.join("Cargo.toml")) else {
        println!("ci_matrix_scan: {} names no package", root.display());
        std::process::exit(2);
    };
    let mut members: Vec<(String, String)> = Vec::new();
    for path in workspace_members(&root_toml) {
        match package_name(&root.join(&path).join("Cargo.toml")) {
            Some(name) => members.push((path, name)),
            None => {
                println!("ci_matrix_scan: {path} carries no package name");
                std::process::exit(2);
            }
        }
    }
    let Ok(workflow) = fs::read_to_string(root.join(".github/workflows/ci-check.yml")) else {
        println!("ci_matrix_scan: ci-check.yml absent — the matrix cannot be checked");
        std::process::exit(1);
    };
    let lines = nextest_lines(&workflow);
    let gaps = gaps(&members, &lines, &root_package);
    for gap in &gaps {
        println!("{gap}");
    }
    println!(
        "ci_matrix_scan: {} member(s), {} nextest step(s), {} gap(s)",
        members.len(),
        lines.len(),
        gaps.len()
    );
    if !gaps.is_empty() {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_member_array_is_read_with_its_paths() {
        let toml = "[workspace]\nmembers = [\n    \"tools/app\",\n    \".\",\n]\ndefault-members = [\".\"]\n";
        assert_eq!(workspace_members(toml), vec!["tools/app", "."]);
    }

    #[test]
    fn a_member_without_a_step_is_a_gap() {
        let members = vec![("tools/app".to_string(), "omegaflow-app".to_string())];
        let lines = vec!["cargo nextest run --release -p omegaflow-register".to_string()];
        let gaps = gaps(&members, &lines, "omegaflow");
        assert_eq!(gaps.len(), 1, "{gaps:?}");
        assert!(gaps[0].contains("omegaflow-app"), "{gaps:?}");
    }

    #[test]
    fn the_root_package_is_covered_by_the_unqualified_step() {
        let members = vec![(".".to_string(), "omegaflow".to_string())];
        let lines = vec!["cargo nextest run --release --partition count:1/4".to_string()];
        assert!(gaps(&members, &lines, "omegaflow").is_empty());
    }

    #[test]
    fn a_bin_filter_is_reported_even_when_the_package_is_named() {
        let members = vec![("tools/harvest".to_string(), "omegaflow-harvest".to_string())];
        let lines =
            vec!["cargo nextest run --release -p omegaflow-harvest --bin a_probe".to_string()];
        let gaps = gaps(&members, &lines, "omegaflow");
        assert_eq!(gaps.len(), 2, "{gaps:?}");
        assert!(gaps.iter().any(|g| g.contains("runs in no nextest step")));
        assert!(gaps.iter().any(|g| g.contains("--bin filter")));
    }
}
