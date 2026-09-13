use std::path::Path;
use std::process::Command;

fn run_git(repo: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .current_dir(repo)
        .args(args)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

pub fn run_lines(repo: &Path, query: &str) -> Vec<String> {
    let mut lines = Vec::new();
    if query.is_empty() {
        return vec!["pending — the git search carries no query".to_string()];
    }
    match run_git(
        repo,
        &[
            "log",
            "--all",
            "--oneline",
            "--regexp-ignore-case",
            "-S",
            query,
            "--",
        ],
    ) {
        Some(text) => {
            for l in text.lines().take(40) {
                lines.push(format!("commit {}", l));
            }
        }
        None => lines.push("pending — git log carries no reading".to_string()),
    }
    if let Some(text) = run_git(
        repo,
        &["grep", "-n", "-i", "--no-color", query, "--", ":!target"],
    ) {
        for l in text.lines().take(40) {
            lines.push(format!("grep {}", l));
        }
    }
    if lines.is_empty() {
        lines.push(format!(
            "absent — the git archive carries no line for: {}",
            query
        ));
    }
    lines
}
