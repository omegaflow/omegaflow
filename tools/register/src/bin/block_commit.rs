use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit};
use std::time::{SystemTime, UNIX_EPOCH};

fn usage() -> ! {
    eprintln!(
        "usage: block_commit --path <file> --key <substring> [--key <substring>...] -m <message>"
    );
    exit(2);
}

fn run_git(args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| format!("git {}: {}", args.join(" "), e))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

fn run_git_index(index: &PathBuf, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .env("GIT_INDEX_FILE", index)
        .args(args)
        .output()
        .map_err(|e| format!("git {}: {}", args.join(" "), e))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

fn temp_path(kind: &str) -> PathBuf {
    let nanos = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_nanos(),
        Err(_) => 0,
    };
    env::temp_dir().join(format!(
        "block-commit-{}-{}-{}",
        kind,
        std::process::id(),
        nanos
    ))
}

fn split_blocks(text: &str) -> Vec<Vec<String>> {
    let mut blocks: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                blocks.push(std::mem::take(&mut current));
            }
        } else {
            current.push(line.to_string());
        }
    }
    if !current.is_empty() {
        blocks.push(current);
    }
    blocks
}

fn block_matches(block: &[String], keys: &[String]) -> bool {
    block
        .iter()
        .any(|line| keys.iter().any(|key| line.contains(key.as_str())))
}

fn build_target(
    wt_blocks: &[Vec<String>],
    base_blocks: &[Vec<String>],
    keys: &[String],
) -> (Vec<Vec<String>>, usize) {
    let mut target: Vec<Vec<String>> = Vec::new();
    let mut matched = 0usize;
    for block in wt_blocks {
        if block_matches(block, keys) {
            matched += 1;
            target.push(block.clone());
            continue;
        }
        if let Some(base_block) = base_blocks.iter().find(|b| b.first() == block.first()) {
            target.push(base_block.clone());
        }
    }
    (target, matched)
}

fn join_blocks(blocks: &[Vec<String>]) -> String {
    let mut out = String::new();
    for (i, block) in blocks.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(&block.join("\n"));
        out.push('\n');
    }
    out
}

fn read_base(path: &str) -> String {
    match run_git(&["show", &format!("HEAD:{}", path)]) {
        Ok(text) => text,
        Err(_) => String::new(),
    }
}

fn hash_object(text: &str) -> Result<String, String> {
    let tmp = temp_path("blob");
    fs::write(&tmp, text).map_err(|e| format!("write {}: {}", tmp.display(), e))?;
    let tmp_arg = tmp.to_string_lossy().to_string();
    let result = run_git(&["hash-object", "-w", &tmp_arg]);
    let _ = fs::remove_file(&tmp);
    result.map(|s| s.trim().to_string())
}

fn write_tree(path: &str, blob: &str) -> Result<String, String> {
    let index = temp_path("index");
    let _ = fs::remove_file(&index);
    let entry = format!("100644,{},{}", blob, path);
    if let Err(e) = run_git_index(&index, &["read-tree", "HEAD"]) {
        let _ = fs::remove_file(&index);
        return Err(e);
    }
    if let Err(e) = run_git_index(&index, &["update-index", "--add", "--cacheinfo", &entry]) {
        let _ = fs::remove_file(&index);
        return Err(e);
    }
    let tree = run_git_index(&index, &["write-tree"]);
    let _ = fs::remove_file(&index);
    tree.map(|t| t.trim().to_string())
}

fn build_commit(path: &str, text: &str, message: &str) -> Result<(String, String), String> {
    let blob = hash_object(text)?;
    let tree = write_tree(path, &blob)?;
    let old = run_git(&["rev-parse", "HEAD"])?.trim().to_string();
    let new = run_git(&["commit-tree", &tree, "-p", &old, "-m", message])?
        .trim()
        .to_string();
    Ok((old, new))
}

fn sync_index(path: &str) {
    let head_path = format!("HEAD:{}", path);
    if let Ok(blob) = run_git(&["rev-parse", &head_path]) {
        let entry = format!("100644,{},{}", blob.trim(), path);
        let _ = run_git(&["update-index", "--add", "--cacheinfo", &entry]);
    }
}

fn report() {
    if let Ok(head) = run_git(&["rev-parse", "HEAD"]) {
        let head = head.trim();
        let short = if head.len() >= 7 { &head[..7] } else { head };
        eprintln!("block_commit: committed {}", short);
    }
    if let Ok(stat) = run_git(&["show", "--stat", "HEAD"]) {
        eprint!("{}", stat);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path: Option<String> = None;
    let mut keys: Vec<String> = Vec::new();
    let mut message: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--path" => {
                i += 1;
                path = args.get(i).cloned();
            }
            "--key" => {
                i += 1;
                if let Some(key) = args.get(i) {
                    keys.push(key.clone());
                }
            }
            "-m" => {
                i += 1;
                message = args.get(i).cloned();
            }
            _ => {}
        }
        i += 1;
    }

    let path = match path {
        Some(p) if !p.is_empty() => p,
        _ => usage(),
    };
    if keys.is_empty() {
        eprintln!("block_commit: at least one --key is required");
        exit(2);
    }
    let message = match message {
        Some(m) if !m.is_empty() => m,
        _ => usage(),
    };

    for attempt in 0..2 {
        let base_blocks = split_blocks(&read_base(&path));
        let wt = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("block_commit: read {}: {}", path, e);
                exit(2);
            }
        };
        let wt_blocks = split_blocks(&wt);
        let (target, matched) = build_target(&wt_blocks, &base_blocks, &keys);

        if matched == 0 {
            eprintln!("block_commit: no block matched key — nothing committed");
            exit(2);
        }
        if target == base_blocks {
            eprintln!("no selected block changed — nothing committed");
            exit(0);
        }

        let text = join_blocks(&target);
        let (old, new) = match build_commit(&path, &text, &message) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!("block_commit: {}", e);
                exit(2);
            }
        };

        match run_git(&["update-ref", "refs/heads/main", &new, &old]) {
            Ok(_) => {
                sync_index(&path);
                report();
                exit(0);
            }
            Err(reason) => {
                if attempt == 1 {
                    eprintln!(
                        "block_commit: update-ref refs/heads/main {} <- {} refused: {}",
                        old, new, reason
                    );
                    exit(2);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_split_blocks_separates_on_blank_runs() {
        let blocks = split_blocks("\n\nurl a\nx\n\n\nurl b\ny\n\n");
        assert_eq!(
            blocks,
            vec![
                vec!["url a".to_string(), "x".to_string()],
                vec!["url b".to_string(), "y".to_string()],
            ]
        );
    }

    fn fixture() -> (&'static str, &'static str) {
        let base = "url b1\na=1\n\nurl b2\nb=2\n";
        let wt = "url b1\na=2\n\nurl b2\nb=2\n\nurl b3\nc=3\n";
        (base, wt)
    }

    #[test]
    fn b_target_keeps_base_blocks_and_takes_selected_new_block() {
        let (base, wt) = fixture();
        let keys = vec!["url b3".to_string()];
        let (target, matched) = build_target(&split_blocks(wt), &split_blocks(base), &keys);
        assert_eq!(matched, 1);
        assert_eq!(
            target,
            vec![
                vec!["url b1".to_string(), "a=1".to_string()],
                vec!["url b2".to_string(), "b=2".to_string()],
                vec!["url b3".to_string(), "c=3".to_string()],
            ]
        );
    }

    #[test]
    fn c_target_takes_selected_modified_block_and_drops_unselected_new_block() {
        let (base, wt) = fixture();
        let keys = vec!["url b1".to_string()];
        let (target, matched) = build_target(&split_blocks(wt), &split_blocks(base), &keys);
        assert_eq!(matched, 1);
        assert_eq!(
            target,
            vec![
                vec!["url b1".to_string(), "a=2".to_string()],
                vec!["url b2".to_string(), "b=2".to_string()],
            ]
        );
    }

    #[test]
    fn d_target_is_base_when_no_block_changed() {
        let base = "url b1\na=1\n\nurl b2\nb=2\n";
        let keys = vec!["url b1".to_string()];
        let (target, matched) = build_target(&split_blocks(base), &split_blocks(base), &keys);
        assert_eq!(matched, 1);
        assert_eq!(target, split_blocks(base));
    }
}
