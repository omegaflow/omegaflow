use omegaflow::commit_gate::{
    Gate, addressed_origin_violations, canon_diff, check_handover_burn,
    declared_canon, doc_open_marker_line, ereignis_folge_violations, handover_dupe_violations,
    integrated_twin, is_test_file_path, json_write, prose_violation_for,
    register_field_unit_issues, status_proof_violations, unbacked_mirror_violations,
    word_register_origin_violations,
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
    for dir in ["docs/handover", "state/future/handover"] {
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

fn own_handover_missing(doc_paths: &[String], own_handover_text: &str) -> Vec<String> {
    doc_paths
        .iter()
        .filter(|p| !doc_carried(own_handover_text, p))
        .cloned()
        .collect()
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

fn handover_line_of(path: &str) -> Option<String> {
    let rel = path.strip_prefix("docs/handover/")?;
    if rel.contains('/') {
        return None;
    }
    let stem = rel.strip_suffix(".md")?;
    let rest = stem.strip_prefix("handover-")?;
    if rest.len() < 12 {
        return None;
    }
    let date = &rest[..10];
    let date_field = date.len() == 10
        && date.as_bytes()[4] == b'-'
        && date.as_bytes()[7] == b'-'
        && date.bytes().all(|b| b.is_ascii_digit() || b == b'-');
    if !date_field || rest.as_bytes()[10] != b'-' {
        return None;
    }
    let tail = &rest[11..];
    let at = tail.rfind("-folge")?;
    let line = &tail[..at];
    let digits = &tail[at + 6..];
    if line.is_empty() || digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some(line.to_lowercase())
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
    let mut test_file_exempt = 0usize;
    for path in files.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if !path.ends_with(".rs") {
            continue;
        }
        if is_test_file_path(path) {
            eprintln!(
                "commit_check: test-file-exempt {path} (name-based; production markers are never exempt)"
            );
            test_file_exempt += 1;
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
    let sources = match std::fs::read_to_string("phi/sources.φ") {
        Ok(s) => s,
        Err(_) => String::new(),
    };
    for register in ["phi/blocked_sources.φ", "phi/declined_sources.φ"] {
        if !staged.contains(&register) {
            continue;
        }
        let out = Command::new("git")
            .args(["show", &format!(":{register}")])
            .output()
            .expect("git");
        let content = String::from_utf8_lossy(&out.stdout).to_string();
        if let Some(v) = integrated_twin(register, &content, &sources) {
            let loc = if v.line > 0 {
                format!("{register}:{}", v.line)
            } else {
                register.to_string()
            };
            eprintln!("commit_check: {loc}: {} - {}", v.rule, v.feedback);
            fail = true;
        }
    }
    if staged.contains(&"phi/sources.φ") {
        match std::fs::read_to_string("docs/specs/force-unit-baseline.txt") {
            Ok(baseline) => {
                let known: std::collections::HashSet<(String, String)> = baseline
                    .lines()
                    .map(str::trim)
                    .filter(|l| !l.is_empty() && !l.starts_with('#'))
                    .filter_map(|l| {
                        let mut it = l.split_whitespace();
                        Some((it.next()?.to_string(), it.next()?.to_string()))
                    })
                    .collect();
                let out = Command::new("git")
                    .args(["show", ":phi/sources.φ"])
                    .output()
                    .expect("git");
                let register = String::from_utf8_lossy(&out.stdout).to_string();
                for issue in register_field_unit_issues(&register) {
                    if !known.contains(&(issue.force.clone(), issue.unit.clone())) {
                        eprintln!(
                            "commit_check: phi/sources.φ:{}: force-unit-ratchet - the pair \"{}\" / \"{}\" is outside the registry and not in docs/specs/force-unit-baseline.txt",
                            issue.line, issue.force, issue.unit
                        );
                        fail = true;
                    }
                }
            }
            Err(_) => {
                eprintln!(
                    "commit_check: docs/specs/force-unit-baseline.txt absent - the force-unit ratchet did not run for phi/sources.φ"
                );
                fail = true;
            }
        }
    }
    let session = std::env::var("OMEGAFLOW_SESSION")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    if let Ok(ereignisse) = std::fs::read_to_string("state/zustand/ereignisse.φ") {
        let wartend = std::fs::read_to_string("state/zustand/wartend.φ").ok();
        let blocked = std::fs::read_to_string("phi/blocked_sources.φ").ok();
        match session.as_deref() {
            Some(name) => {
                for (line, rule, feedback) in ereignis_folge_violations(
                    &ereignisse,
                    wartend.as_deref(),
                    blocked.as_deref(),
                    Some(name),
                ) {
                    eprintln!(
                        "commit_check: state/zustand/ereignisse.φ:{line}: {rule} - {feedback}"
                    );
                    fail = true;
                }
            }
            None => {
                eprintln!(
                    "commit_check: state/zustand/ereignisse.φ: ereignis-ohne-folge: session name missing (OMEGAFLOW_SESSION unset) - check skipped by name"
                );
            }
        }
    }
    let gated: Vec<String> = declared_canon();
    for path in gated.iter().filter(|p| staged.contains(&p.as_str())) {
        let out = Command::new("git")
            .args(["diff", "--cached", "-U0", "--", path.as_str()])
            .output()
            .expect("git");
        let diff = String::from_utf8_lossy(&out.stdout).to_string();
        let mut added: Vec<&str> = Vec::new();
        for line in diff.lines() {
            let t = line.trim_end_matches('\r');
            if !t.starts_with('+') || t.starts_with("+++") {
                continue;
            }
            added.push(&t[1..]);
            if let Some(kind) = prose_violation_for(path, &t[1..]) {
                eprintln!("commit_check: phi-register-prose: {kind}: {path}");
                fail = true;
            }
        }
        if path == "phi/sources.φ" {
            for (line, message) in unbacked_mirror_violations(&added) {
                eprintln!("commit_check: phi/sources.φ:{line}: unbacked_mirror - {message}");
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
        .filter(|l| !l.is_empty() && *l != "phi/canon.φ")
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
    let added = Command::new("git")
        .args([
            "-c",
            "core.quotePath=false",
            "diff",
            "--cached",
            "--name-only",
            "--diff-filter=A",
        ])
        .output()
        .expect("git");
    let added_files = String::from_utf8_lossy(&added.stdout).to_string();
    let added_docs: Vec<String> = added_files
        .lines()
        .map(str::trim)
        .filter(|l| {
            !l.is_empty() && l.ends_with(".md") && DOC_DIRS.iter().any(|d| l.starts_with(d))
        })
        .filter(|l| {
            std::fs::read_to_string(l)
                .map(|c| !doc_closed(&c))
                .unwrap_or(true)
        })
        .map(str::to_string)
        .collect();
    if !added_docs.is_empty() {
        let mut own_carrier = String::new();
        for path in files
            .lines()
            .map(str::trim)
            .filter(|l| handover_line_of(l).is_some())
        {
            if let Ok(text) = std::fs::read_to_string(path) {
                own_carrier.push_str(&text.to_lowercase());
                own_carrier.push('\n');
            }
        }
        for path in own_handover_missing(&added_docs, &own_carrier) {
            eprintln!(
                "commit_check: own-handover: new document {path} is not named in this commit's handover - name it in the handover point so the next line finds it"
            );
            fail = true;
        }
    }
    for path in files.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if !path.contains("handover/") || !path.ends_with(".md") || path.contains("/archiv/") {
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
        for (line, rule, feedback) in word_register_origin_violations(&content) {
            eprintln!("commit_check: {path}:{line}: {rule} - {feedback}");
            fail = true;
        }
        for (line, rule, feedback) in handover_dupe_violations(&content) {
            eprintln!("commit_check: {path}:{line}: {rule} - {feedback}");
            fail = true;
        }
        for (line, rule, feedback) in addressed_origin_violations(&content) {
            eprintln!("commit_check: {path}:{line}: {rule} - {feedback}");
            fail = true;
        }
        if let Some(v) = check_handover_burn(path, &content) {
            eprintln!(
                "commit_check: {path}:{}: {} - {}",
                v.line, v.rule, v.feedback
            );
            fail = true;
        }
    }
    let mut handover_lines: Vec<String> = Vec::new();
    for path in files.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(line) = handover_line_of(path) {
            if !handover_lines.contains(&line) {
                handover_lines.push(line);
            }
        }
    }
    for line in handover_lines {
        let out = Command::new("register_lookup")
            .args(["--orphans", "--owner", &line, "--fail"])
            .output();
        let output = match out {
            Ok(o) => o,
            Err(_) => {
                eprintln!(
                    "commit_check: register_lookup absent from PATH - the handover orphan gate did not run for {line}"
                );
                continue;
            }
        };
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let reported = stdout.contains("register_lookup --orphans:");
        if output.status.success() {
            continue;
        }
        if !reported {
            let code = match output.status.code() {
                Some(c) => c.to_string(),
                None => "signal".to_string(),
            };
            eprintln!(
                "commit_check: register_lookup exited {code} without an orphan report - the handover orphan gate did not run for {line}"
            );
            continue;
        }
        eprintln!(
            "commit_check: handover-orphan-gate: {line}: staged handover, open register entries without a carrier remain"
        );
        for report_line in stdout.lines() {
            eprintln!("commit_check: {report_line}");
        }
        fail = true;
    }
    eprintln!("commit_check: {test_file_exempt} test-file(s) exempted by path");
    if fail {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn own_handover_requires_the_new_document_named() {
        let doc = "docs/surveys/survey-2026-10-10-foo.md".to_string();
        assert_eq!(
            own_handover_missing(std::slice::from_ref(&doc), "kein treffer"),
            vec![doc.clone()]
        );
        assert!(
            own_handover_missing(std::slice::from_ref(&doc), "siehe survey-2026-10-10-foo.md")
                .is_empty()
        );
    }
}
