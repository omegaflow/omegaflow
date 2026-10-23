use std::io::Read;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const HISTORY_LIMIT: usize = 40;
const HISTORY_TIMEOUT: Duration = Duration::from_secs(10);
const CONTENT_LIMIT: usize = 40;
const POLL_INTERVAL: Duration = Duration::from_millis(10);

enum Run {
    Done(String),
    Failed,
    TimedOut,
}

fn run_command(repo: &Path, program: &str, args: &[&str], timeout: Option<Duration>) -> Run {
    let mut child = match Command::new(program)
        .current_dir(repo)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return Run::Failed,
    };
    let mut stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => return Run::Failed,
    };
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stdout.read_to_end(&mut buf);
        let _ = tx.send(buf);
    });
    let started = Instant::now();
    let mut timed_out = false;
    let status: Option<ExitStatus> = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) => {
                let limit = match timeout {
                    Some(limit) => limit,
                    None => {
                        thread::sleep(POLL_INTERVAL);
                        continue;
                    }
                };
                if started.elapsed() >= limit {
                    let _ = child.kill();
                    let _ = child.wait();
                    timed_out = true;
                    break None;
                }
                thread::sleep(POLL_INTERVAL);
            }
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
        }
    };
    if timed_out {
        let _ = rx.recv_timeout(Duration::from_millis(250));
        return Run::TimedOut;
    }
    let buf = match rx.recv() {
        Ok(buf) => buf,
        Err(_) => return Run::Failed,
    };
    match status {
        Some(status) if status.success() || status.code() == Some(1) => {
            Run::Done(String::from_utf8_lossy(&buf).to_string())
        }
        _ => Run::Failed,
    }
}

pub fn content_lines(repo: &Path, query: &str) -> Vec<String> {
    if query.is_empty() {
        return vec!["pending — the git search carries no query".to_string()];
    }
    match run_command(
        repo,
        "git",
        &["grep", "-n", "-i", "--no-color", query, "--", ":!target"],
        None,
    ) {
        Run::Done(text) => {
            let lines: Vec<String> = text
                .lines()
                .take(CONTENT_LIMIT)
                .map(|line| format!("grep {line}"))
                .collect();
            if lines.is_empty() {
                vec![format!(
                    "absent — the git content carries no line for: {query}"
                )]
            } else {
                lines
            }
        }
        Run::TimedOut => vec!["pending — git content timed out".to_string()],
        Run::Failed => vec!["pending — git content carries no reading".to_string()],
    }
}

pub fn history_lines(repo: &Path, query: &str) -> Vec<String> {
    if query.is_empty() {
        return vec!["pending — the git history carries no query".to_string()];
    }
    match run_command(
        repo,
        "git",
        &[
            "log",
            "--all",
            "--oneline",
            "--regexp-ignore-case",
            "-S",
            query,
            "--",
        ],
        Some(HISTORY_TIMEOUT),
    ) {
        Run::Done(text) => {
            let lines: Vec<String> = text
                .lines()
                .take(HISTORY_LIMIT)
                .map(|line| format!("commit {line}"))
                .collect();
            if lines.is_empty() {
                vec![format!(
                    "absent — the git history carries no commit for: {query}"
                )]
            } else {
                lines
            }
        }
        Run::TimedOut => vec!["pending — git history timed out".to_string()],
        Run::Failed => vec!["pending — git history carries no reading".to_string()],
    }
}

pub fn run_lines(repo: &Path, query: &str) -> Vec<String> {
    content_lines(repo, query)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    struct TempRepo {
        path: PathBuf,
    }

    impl TempRepo {
        fn new(name: &str) -> TempRepo {
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0);
            let path = std::env::temp_dir().join(format!(
                "omegaflow-git-{name}-{}-{nanos}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("temp repo");
            let repo = TempRepo { path };
            repo.git(&["init", "-q"]);
            repo.git(&["config", "user.email", "test@example.invalid"]);
            repo.git(&["config", "user.name", "omegaflow-test"]);
            repo
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn git(&self, args: &[&str]) {
            let status = Command::new("git")
                .current_dir(&self.path)
                .args(args)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .expect("git spawn");
            assert!(status.success(), "git {:?} failed", args);
        }

        fn write(&self, name: &str, text: &str) {
            fs::write(self.path.join(name), text).expect("write fixture");
        }

        fn commit(&self, message: &str) {
            self.git(&["add", "-A"]);
            self.git(&["commit", "-q", "-m", message]);
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn content_path_finds_a_line() {
        let repo = TempRepo::new("content");
        repo.write("main.rs", "fn main() {\n    let x = 1;\n}\n");
        repo.commit("add main");
        let lines = content_lines(repo.path(), "fn main");
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("grep main.rs:1:fn main"));
    }

    #[test]
    fn content_path_reports_absent() {
        let repo = TempRepo::new("content-absent");
        repo.write("main.rs", "fn main() {}\n");
        repo.commit("add main");
        let lines = content_lines(repo.path(), "zzz-no-such-token");
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("absent — the git content carries no line"));
    }

    #[test]
    fn content_path_carries_no_query() {
        let repo = TempRepo::new("content-empty");
        let lines = content_lines(repo.path(), "");
        assert_eq!(lines, vec!["pending — the git search carries no query"]);
    }

    #[test]
    fn content_path_never_reads_history() {
        let repo = TempRepo::new("content-no-history");
        repo.write("log.txt", "needle\n");
        repo.commit("add needle");
        repo.write("log.txt", "clean\n");
        repo.commit("remove needle");
        let lines = content_lines(repo.path(), "needle");
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("absent — the git content carries no line"));
        assert!(lines.iter().all(|line| !line.starts_with("commit ")));
    }

    #[test]
    fn history_path_finds_commit() {
        let repo = TempRepo::new("history");
        repo.write("main.rs", "fn main() {}\n");
        repo.commit("add main");
        repo.write("main.rs", "fn main() {\n    let needle = 1;\n}\n");
        repo.commit("add needle");
        let lines = history_lines(repo.path(), "needle");
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("commit "));
        assert!(lines[0].contains("add needle"));
    }

    #[test]
    fn history_path_reports_absent() {
        let repo = TempRepo::new("history-absent");
        repo.write("main.rs", "fn main() {}\n");
        repo.commit("add main");
        let lines = history_lines(repo.path(), "zzz-no-such-token");
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("absent — the git history carries no commit"));
    }

    #[test]
    fn history_path_caps_at_forty_lines() {
        let repo = TempRepo::new("history-cap");
        let mut text = String::new();
        for step in 0..45 {
            text.push_str(&format!("needle line {step}\n"));
            repo.write("log.txt", &text);
            repo.commit(&format!("step {step}"));
        }
        let lines = history_lines(repo.path(), "needle");
        assert_eq!(lines.len(), HISTORY_LIMIT);
        assert!(lines.iter().all(|line| line.starts_with("commit ")));
    }

    #[test]
    fn timeout_kills_child_and_reports_timed_out() {
        let repo = TempRepo::new("timeout");
        let started = Instant::now();
        let run = run_command(
            repo.path(),
            "sh",
            &["-c", "sleep 5"],
            Some(Duration::from_millis(100)),
        );
        assert!(matches!(run, Run::TimedOut));
        assert!(started.elapsed() < Duration::from_secs(2));
    }
}
