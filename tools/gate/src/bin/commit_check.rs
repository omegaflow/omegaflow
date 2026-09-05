use omegaflow::commit_gate::{json_write, Gate};
use omegaflow::json::JsonVal;
use std::collections::HashMap;
use std::process::Command;

fn main() {
    let out = Command::new("git")
        .args(["diff", "--cached", "--name-only", "--diff-filter=ACM"])
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
            eprintln!("commit_check: {path}: {} - {}", v.rule, v.feedback);
            fail = true;
        }
    }
    if fail {
        std::process::exit(1);
    }
}
