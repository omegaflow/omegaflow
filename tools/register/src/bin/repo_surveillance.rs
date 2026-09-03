use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let dirty = git(&["status", "--porcelain"]);
    let untracked = dirty.lines().filter(|l| l.starts_with("??")).count();
    let modified = dirty
        .lines()
        .filter(|l| l.starts_with(" M") || l.starts_with("M ") || l.starts_with("A "))
        .count();
    let last = git(&["log", "-1", "--oneline"]);
    let ahead = git(&["rev-list", "--count", "@{u}..HEAD"]);
    let check = git_verdict();
    let line = report_line(
        epoch(),
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
    std::path::PathBuf::from(".")
}

fn git_verdict() -> String {
    let mut v: Vec<&str> = Vec::new();
    let remote = git(&["remote", "get-url", "origin"]);
    if !remote.contains("omegaflow/omegaflow") {
        v.push("remote");
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

fn epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
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
