use omegaflow::commit_gate::{
    Gate, canon_diff, declared_canon, json_write, prose_violation, register_classes,
};
use omegaflow::json::JsonVal;
use std::collections::HashMap;
use std::process::Command;

fn main() {
    let out = Command::new("git")
        .args([
            "-c",
            "core.quotePath=false",
            "diff",
            "--cached",
            "--name-only",
            "--diff-filter=ACM",
        ])
        .output()
        .expect("git");
    let files = String::from_utf8_lossy(&out.stdout).to_string();
    let mut gate = Gate::new("commit", "");
    let mut fail = false;
    for path in files.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if !path.ends_with(".rs") {
            continue;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let mut obj = HashMap::new();
        obj.insert("filePath".to_string(), JsonVal::Str(path.to_string()));
        obj.insert("content".to_string(), JsonVal::Str(content));
        let args = json_write(&JsonVal::Obj(obj));
        if let Some(v) = gate.check_tool_call("write", &args) {
            let loc = if v.line > 0 {
                format!("{path}:{}", v.line)
            } else {
                path.to_string()
            };
            eprintln!("commit_check: {loc}: {} - {}", v.rule, v.feedback);
            fail = true;
        }
    }
    let staged: Vec<&str> = files.lines().map(str::trim).collect();
    for path in register_classes()
        .iter()
        .filter(|p| staged.contains(&p.as_str()))
    {
        let out = Command::new("git")
            .args(["diff", "--cached", "-U0", "--", path.as_str()])
            .output()
            .expect("git");
        let diff = String::from_utf8_lossy(&out.stdout).to_string();
        for line in diff.lines() {
            let t = line.trim_end_matches('\r');
            if !t.starts_with('+') || t.starts_with("+++") {
                continue;
            }
            if let Some(kind) = prose_violation(&t[1..]) {
                eprintln!("commit_check: phi-register-prose: {kind}: {path}");
                fail = true;
            }
        }
    }
    let canon_files = Command::new("git")
        .args([
            "-c",
            "core.quotePath=false",
            "ls-files",
            "phi/*.φ",
            "phi/**/*.φ",
        ])
        .output()
        .expect("git");
    let tracked: Vec<String> = String::from_utf8_lossy(&canon_files.stdout)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect();
    let declared = declared_canon();
    let (tracked_not_declared, declared_not_tracked) = canon_diff(&tracked, &declared);
    for path in tracked_not_declared {
        eprintln!("commit_check: tracked-not-declared: {path}");
        fail = true;
    }
    for path in declared_not_tracked {
        eprintln!("commit_check: declared-not-tracked: {path}");
        fail = true;
    }
    if fail {
        std::process::exit(1);
    }
}
