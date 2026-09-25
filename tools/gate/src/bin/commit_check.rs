use omegaflow::commit_gate::{
    Gate, canon_diff, declared_canon, doc_open_marker_line, json_write, prose_violation_for,
    register_classes, status_proof_violations,
};
use omegaflow::json::JsonVal;
use std::collections::HashMap;
use std::process::Command;

const DOC_DIRS: [&str; 6] = [
    "docs/surveys/",
    "docs/specs/",
    "docs/auftrag/",
    "docs/blatt/",
    "docs/concepts/",
    "docs/paper/",
];

fn live_handover_carrier() -> String {
    let mut out = String::new();
    for dir in ["docs/handover", "state/funding/handover"] {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let name = match path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => continue,
            };
            if !name.ends_with(".md") || name.starts_with('_') || !name.starts_with("handover-") {
                continue;
            }
            if let Ok(text) = std::fs::read_to_string(&path) {
                out.push_str(&text.to_lowercase());
                out.push('\n');
            }
        }
    }
    out
}

fn doc_carried(carrier: &str, path: &str) -> bool {
    let base = match std::path::Path::new(path).file_name() {
        Some(f) => f.to_string_lossy().to_lowercase(),
        None => return false,
    };
    let stem = base.strip_suffix(".md").unwrap_or(&base);
    (base.len() >= 8 && carrier.contains(&base))
        || (stem.chars().count() >= 8 && carrier.contains(stem))
}

fn doc_closed(content: &str) -> bool {
    let (Some(open), Some(close)) = (content.find("<!--"), content.find("-->")) else {
        return false;
    };
    if close <= open {
        return false;
    }
    for line in content[open..close].lines() {
        if let Some(v) = line.trim().strip_prefix("status:") {
            let v = v.trim();
            return v == "consumed" || v == "archived" || v == "done";
        }
    }
    false
}

fn doc_has_open_marker(content: &str) -> bool {
    let start = match content.find("-->") {
        Some(i) => i + 3,
        None => 0,
    };
    content[start..].lines().any(doc_open_marker_line)
}

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
            if let Some(kind) = prose_violation_for(path, &t[1..]) {
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
    let carrier = live_handover_carrier();
    for path in files.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if !path.ends_with(".md") || !DOC_DIRS.iter().any(|d| path.starts_with(d)) {
            continue;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        if doc_closed(&content) {
            continue;
        }
        if doc_has_open_marker(&content) && !doc_carried(&carrier, path) {
            eprintln!(
                "commit_check: doc-carrier: {path} carries open markers but no live handover names it - carry it in its owner's handover or release it (descoped)"
            );
            fail = true;
        }
    }
    for path in files.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if !path.starts_with("docs/handover/")
            || !path.ends_with(".md")
            || path.contains("/archiv/")
        {
            continue;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for (line, rule, feedback) in status_proof_violations(&content) {
            eprintln!("commit_check: {path}:{line}: {rule} - {feedback}");
            fail = true;
        }
    }
    if fail {
        std::process::exit(1);
    }
}
