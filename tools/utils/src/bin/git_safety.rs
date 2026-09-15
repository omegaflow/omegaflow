use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn usage() -> ! {
    eprintln!(
        "usage: git_safety --snapshot            (record the working tree under refs/safety/<epoch>)\n       git_safety --list                (the safety snapshots)\n       git_safety --restore <ref> [<path>]   (write the snapshot back; whole tree without a path)\n       git_safety --prune <keep>        (keep the newest <keep> snapshots)\n       git_safety --watch <secs>        (snapshot every <secs>, silent)"
    );
    std::process::exit(2);
}

fn run_git(root: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        None
    }
}

fn run_git_index(root: &Path, args: &[&str], index: &Path) -> Option<String> {
    let out = Command::new("git")
        .current_dir(root)
        .env("GIT_INDEX_FILE", index)
        .args(args)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        None
    }
}

fn run_git_commit(root: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .current_dir(root)
        .env("GIT_AUTHOR_NAME", "omegaflow-safety")
        .env("GIT_AUTHOR_EMAIL", "safety@omegaflow.space")
        .env("GIT_COMMITTER_NAME", "omegaflow-safety")
        .env("GIT_COMMITTER_EMAIL", "safety@omegaflow.space")
        .args(args)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        None
    }
}

fn repo_root() -> Option<PathBuf> {
    let out = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(PathBuf::from(text))
    }
}

fn now_epoch() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => 0,
    }
}

fn now_nanos() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_nanos(),
        Err(_) => 0,
    }
}

fn snapshot(root: &Path) -> Option<String> {
    let head_tree = run_git(root, &["rev-parse", "HEAD^{tree}"])?
        .trim()
        .to_string();
    let index = env::temp_dir().join(format!(
        "omegaflow-safety-index-{}-{}",
        std::process::id(),
        now_nanos()
    ));
    let _ = fs::remove_file(&index);
    run_git_index(root, &["read-tree", "HEAD"], &index)?;
    run_git_index(root, &["add", "-A"], &index)?;
    let tree = run_git_index(root, &["write-tree"], &index)?
        .trim()
        .to_string();
    let _ = fs::remove_file(&index);
    if tree == head_tree {
        return None;
    }
    let epoch = now_epoch();
    let message = format!("safety snapshot {}", epoch);
    let commit = run_git_commit(root, &["commit-tree", &tree, "-p", "HEAD", "-m", &message])?
        .trim()
        .to_string();
    let refname = format!("refs/safety/{}", epoch);
    run_git(root, &["update-ref", &refname, &commit])?;
    Some(refname)
}

fn list_snapshots(root: &Path) -> Vec<String> {
    let text = match run_git(
        root,
        &[
            "for-each-ref",
            "--sort=-creatordate",
            "--format=%(refname) %(objectname:short) %(creatordate:iso-strict)",
            "refs/safety",
        ],
    ) {
        Some(t) => t,
        None => String::new(),
    };
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let root = match repo_root() {
        Some(r) => r,
        None => {
            eprintln!("git_safety: no git repository here");
            std::process::exit(2);
        }
    };
    if args.is_empty() {
        usage();
    }
    match args[0].as_str() {
        "--snapshot" => match snapshot(&root) {
            Some(refname) => println!(
                "safety snapshot {} (recover: git_safety --restore {})",
                refname, refname
            ),
            None => println!("safety snapshot: the working tree equals HEAD — nothing to record"),
        },
        "--list" => {
            let rows = list_snapshots(&root);
            for row in &rows {
                println!("{}", row);
            }
            println!("git_safety --list: {} snapshots", rows.len());
        }
        "--restore" => {
            let reference = match args.get(1) {
                Some(r) => r.clone(),
                None => usage(),
            };
            let path = match args.get(2) {
                Some(p) => p.clone(),
                None => ".".to_string(),
            };
            match run_git(&root, &["checkout", &reference, "--", &path]) {
                Some(_) => println!("git_safety: restored {} from {}", path, reference),
                None => {
                    eprintln!("git_safety: {} absent in {}", path, reference);
                    std::process::exit(1);
                }
            }
        }
        "--prune" => {
            let keep = match args.get(1).and_then(|s| s.parse::<usize>().ok()) {
                Some(k) => k,
                None => usage(),
            };
            let refs: Vec<String> = list_snapshots(&root)
                .iter()
                .filter_map(|row| row.split_whitespace().next().map(|s| s.to_string()))
                .collect();
            let mut removed = 0;
            for r in refs.iter().skip(keep) {
                if run_git(&root, &["update-ref", "-d", r]).is_some() {
                    removed += 1;
                }
            }
            println!(
                "git_safety --prune: {} kept, {} removed",
                refs.len().min(keep),
                removed
            );
        }
        "--watch" => {
            let secs = match args.get(1).and_then(|s| s.parse::<u64>().ok()) {
                Some(s) if s > 0 => s,
                _ => usage(),
            };
            eprintln!(
                "git_safety: watching every {} s (silent); snapshots under refs/safety/",
                secs
            );
            loop {
                let _ = snapshot(&root);
                thread::sleep(Duration::from_secs(secs));
            }
        }
        _ => usage(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn git(dir: &Path, args: &[&str]) {
        let status = Command::new("git")
            .current_dir(dir)
            .args(args)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?}", args);
    }

    fn scratch(name: &str) -> PathBuf {
        let dir = env::temp_dir().join(format!("git_safety_{}_{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        git(&dir, &["init", "-q"]);
        git(&dir, &["config", "user.email", "t@t"]);
        git(&dir, &["config", "user.name", "t"]);
        dir
    }

    #[test]
    fn snapshot_of_a_clean_tree_records_nothing() {
        let dir = scratch("clean");
        fs::write(dir.join("a.txt"), "one\n").unwrap();
        git(&dir, &["add", "a.txt"]);
        git(&dir, &["commit", "-q", "-m", "base"]);
        assert!(snapshot(&dir).is_none());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn snapshot_records_and_restores_uncommitted_work() {
        let dir = scratch("restore");
        fs::write(dir.join("a.txt"), "one\n").unwrap();
        git(&dir, &["add", "a.txt"]);
        git(&dir, &["commit", "-q", "-m", "base"]);
        fs::write(dir.join("a.txt"), "two\n").unwrap();
        let refname = snapshot(&dir).unwrap();
        assert!(refname.starts_with("refs/safety/"));
        git(&dir, &["checkout", "--", "a.txt"]);
        assert_eq!(fs::read_to_string(dir.join("a.txt")).unwrap(), "one\n");
        git(&dir, &["checkout", &refname, "--", "a.txt"]);
        assert_eq!(fs::read_to_string(dir.join("a.txt")).unwrap(), "two\n");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn snapshot_carries_untracked_files() {
        let dir = scratch("untracked");
        fs::write(dir.join("base.txt"), "x\n").unwrap();
        git(&dir, &["add", "base.txt"]);
        git(&dir, &["commit", "-q", "-m", "base"]);
        fs::write(dir.join("new.txt"), "fresh\n").unwrap();
        let refname = snapshot(&dir).unwrap();
        fs::remove_file(dir.join("new.txt")).unwrap();
        assert!(!dir.join("new.txt").exists());
        git(&dir, &["checkout", &refname, "--", "new.txt"]);
        assert_eq!(fs::read_to_string(dir.join("new.txt")).unwrap(), "fresh\n");
        let _ = fs::remove_dir_all(&dir);
    }
}
