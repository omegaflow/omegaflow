use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    if std::env::args().any(|a| a == "--push") {
        print_push_order();
        return;
    }
    let dirty = git(&["status", "--porcelain"]);
    let untracked = dirty.lines().filter(|l| l.starts_with("??")).count();
    let modified = dirty
        .lines()
        .filter(|l| l.starts_with(" M") || l.starts_with("M ") || l.starts_with("A "))
        .count();
    let last = git(&["log", "-1", "--oneline"]);
    let ahead = git(&["rev-list", "--count", "@{u}..HEAD"]);
    let check = git_verdict();
    let Some(epoch) = epoch() else {
        return;
    };
    let line = report_line(
        epoch,
        modified,
        untracked,
        last.trim(),
        ahead.trim(),
        &check,
    );
    append_report(&state_dir().join("reports/repo_surveillance.φ"), &line);
    println!("{}", line.trim_end());
}

fn state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    std::path::PathBuf::from("state")
}

fn print_push_order() {
    let up = upstream();
    let ahead = git(&["rev-list", "--count", &format!("{}..HEAD", up)]);
    let ff = if git_ok(&["merge-base", "--is-ancestor", "origin/main", "HEAD"]) {
        "yes"
    } else {
        "no"
    };
    println!("push_order | ahead={} ff={}", ahead.trim(), ff);
    for line in git(&["log", "--reverse", "--oneline", &format!("{}..HEAD", up)]).lines() {
        if !line.trim().is_empty() {
            println!("commit {}", line.trim());
        }
    }
    for (name, head) in divergent_lines() {
        let n = git(&["rev-list", "--count", &format!("main..{}", head)]);
        let patches = git(&["cherry", "main", &head])
            .lines()
            .filter(|l| l.starts_with('+'))
            .count();
        println!("line {} {} +{} patches={}", name, head, n.trim(), patches);
    }
}

fn upstream() -> String {
    if git_ok(&["rev-parse", "--verify", "--quiet", "@{u}"]) {
        "@{u}".to_string()
    } else {
        "origin/main".to_string()
    }
}

fn divergent_lines() -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let refs = git(&[
        "for-each-ref",
        "--format=%(refname:short) %(objectname:short)",
        "refs/heads",
    ]);
    for line in refs.lines() {
        let mut it = line.split_whitespace();
        let (Some(name), Some(head)) = (it.next(), it.next()) else {
            continue;
        };
        if name == "main" || name == "origin/main" {
            continue;
        }
        let n = git(&["rev-list", "--count", &format!("main..{head}")]);
        if !n.trim().is_empty() && n.trim() != "0" {
            out.push((name.to_string(), head.to_string()));
        }
    }
    out
}

fn git_ok(args: &[&str]) -> bool {
    match Command::new("git").args(args).output() {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

fn git_verdict() -> String {
    let mut v: Vec<&str> = Vec::new();
    let remote = git(&["remote", "get-url", "origin"]);
    if !remote.contains("omegaflow/omegaflow") {
        v.push("remote");
    }
    let branch = git(&["branch", "--show-current"]);
    if branch.trim() != "main" {
        v.push("branch");
    }
    if v.is_empty() {
        "check=ok".to_string()
    } else {
        format!("check={}", v.join(","))
    }
}

fn git(args: &[&str]) -> String {
    match Command::new("git").args(args).output() {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => String::from("void"),
    }
}

fn report_line(
    epoch: u64,
    modified: usize,
    untracked: usize,
    last: &str,
    ahead: &str,
    check: &str,
) -> String {
    format!(
        "repo_surveillance | {} | dirty={} untracked={} | ahead={} | {} | last={}\n",
        epoch, modified, untracked, ahead, check, last
    )
}

fn epoch() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs())
}

fn append_report<P: AsRef<std::path::Path>>(path: P, line: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        use std::io::Write;
        let _ = f.write_all(line.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_format() {
        let line = report_line(1787600000, 3, 1, "abcd123 The gate", "0", "check=main");
        assert!(line.starts_with("repo_surveillance | 1787600000 | dirty=3 untracked=1"));
        assert!(line.contains("check=main"));
        assert!(line.contains("last=abcd123 The gate"));
    }

    #[test]
    fn append_creates_file() {
        let path = "/tmp/surv_test_report.φ";
        let _ = std::fs::remove_file(path);
        append_report(
            path,
            "repo_surveillance | 1 | dirty=0 untracked=0 | check=main | last=x\n",
        );
        let text = std::fs::read_to_string(path).unwrap();
        assert!(text.contains("repo_surveillance | 1 |"));
        let _ = std::fs::remove_file(path);
    }
}
