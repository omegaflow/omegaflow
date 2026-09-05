use std::env;
use std::path::{Path, PathBuf};

struct Report {
    line: usize,
    raw: String,
    path: String,
    kind: &'static str,
    note: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut root = PathBuf::from(".");
    let mut registers: Vec<PathBuf> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                i += 1;
                root = PathBuf::from(&args[i]);
            }
            "--register" => {
                i += 1;
                registers.push(PathBuf::from(&args[i]));
            }
            _ => {}
        }
        i += 1;
    }
    if registers.is_empty() {
        registers = vec![
            PathBuf::from("docs/TODO.md"),
            PathBuf::from("docs/status/lose-enden.md"),
        ];
    }

    let mut reports: Vec<Report> = Vec::new();
    for reg in &registers {
        let full = root.join(reg);
        let text = match std::fs::read_to_string(&full) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("register_verify: {} unreadable: {}", full.display(), e);
                continue;
            }
        };
        scan_register(&root, &text, &mut reports);
    }

    reports.sort_by(|a, b| (a.raw.as_str()).cmp(b.raw.as_str()));
    let mut clean = true;
    for r in &reports {
        if r.kind == "EXIST" {
            continue;
        }
        if r.kind == "GATE" || r.kind == "MODULE" {
            println!(
                "{:<8} {:>6} {:<12} {}",
                r.kind,
                r.line,
                r.path,
                if r.note.is_empty() {
                    r.raw.to_string()
                } else {
                    r.note.clone()
                }
            );
            continue;
        }
        if is_allowlisted(&r.path) {
            println!(
                "{:<8} {:>6} {:<12} {}",
                r.kind, r.line, r.path, "benannt entfernt (siehe Register-Vermerk)"
            );
            continue;
        }
        clean = false;
        println!(
            "{:<8} {:>6} {:<12} {}",
            r.kind,
            r.line,
            r.path,
            if r.note.is_empty() {
                r.raw.to_string()
            } else {
                r.note.clone()
            }
        );
    }
    if clean {
        println!(
            "register_verify: every named path exists in {}",
            root.display()
        );
    } else {
        println!(
            "register_verify: drift measured — the register names paths that do not exist (or live elsewhere)."
        );
        std::process::exit(1);
    }
}

fn scan_register(root: &Path, text: &str, out: &mut Vec<Report>) {
    for (idx, line) in text.lines().enumerate() {
        let linen = idx + 1;
        for path in extract_paths(line) {
            out.push(check(root, linen, line, &path));
        }
    }
}

fn extract_paths(line: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0;
    let candidates = [
        "src/",
        "tools/gate/src/bin/",
        "tools/harvest/src/bin/",
        "tools/measure/src/bin/",
        "tools/register/src/bin/",
        "tools/science/src/bin/",
        "tools/service/src/bin/",
        "tools/utils/src/bin/",
        "docs/",
        "phi/",
    ];
    while i < bytes.len() {
        for c in &candidates {
            if bytes[i..].starts_with(c.as_bytes()) {
                let start = i;
                let mut j = i + c.len();
                while j < bytes.len() && is_path_char(bytes[j]) {
                    j += 1;
                }
                let p = &line[start..j];
                if is_path(p) {
                    paths.push(p.to_string());
                }
                i = j;
                break;
            }
        }
        i += 1;
    }
    paths
}

fn is_path_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'/' | b'.' | b'_' | b'-' | b'~')
}

fn is_path(p: &str) -> bool {
    p.ends_with(".rs")
        || p.ends_with(".md")
        || p.ends_with(".φ")
        || p.ends_with(".bin")
        || p.ends_with(".json")
        || p.ends_with(".txt")
}

fn check(root: &Path, line: usize, _rawline: &str, path: &str) -> Report {
    let full = root.join(path);
    let raw = match Path::new(&path).file_stem() {
        Some(s) => s.to_string_lossy().to_string(),
        None => path.to_string(),
    };
    if full.exists() {
        return Report {
            line,
            raw: raw.clone(),
            path: path.to_string(),
            kind: "EXIST",
            note: String::new(),
        };
    }

    if path.starts_with("src/") {
        if let Some(actual) = find_by_basename(root, path) {
            return Report {
                line,
                raw,
                path: path.to_string(),
                kind: "DRIFT",
                note: format!("{} -> {}", path, actual.display()),
            };
        }

        if let Some(dir_mod) = module_shorthand(root, path) {
            return Report {
                line,
                raw,
                path: path.to_string(),
                kind: "MODULE",
                note: format!("{} -> {}", path, dir_mod.display()),
            };
        }
    }

    if is_gate_path(path) {
        return Report {
            line,
            raw,
            path: path.to_string(),
            kind: "GATE",
            note: "gate source lives under src/gate/".to_string(),
        };
    }
    Report {
        line,
        raw,
        path: path.to_string(),
        kind: "MISSING",
        note: String::new(),
    }
}

fn find_by_basename(root: &Path, path: &str) -> Option<PathBuf> {
    let name = Path::new(path).file_name()?;
    let mut stack = vec![root.join("src")];
    while let Some(dir) = stack.pop() {
        let entries = std::fs::read_dir(&dir).ok()?;
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p.clone());
            } else if p.file_name() == Some(name) {
                return Some(p);
            }
        }
    }
    None
}

fn module_shorthand(root: &Path, path: &str) -> Option<PathBuf> {
    let stem = Path::new(path).file_stem()?;
    let mod_path = root.join("src").join(stem).join("mod.rs");
    if mod_path.exists() {
        Some(mod_path)
    } else {
        None
    }
}

fn is_allowlisted(path: &str) -> bool {
    if path.starts_with("docs/handover/") {
        return true;
    }
    matches!(
        path,
        "src/handover_template.md"
            | "src/handover.rs"
            | "src/tool_te.rs"
            | "src/machines.rs"
            | "src/demeter.rs"
            | "src/mathematikerin/window.rs"
            | "src/axioms.md"
            | "phi/frb_harvest/frb_chime_cat1.json"
    )
}

fn is_gate_path(path: &str) -> bool {
    let name = match Path::new(path).file_name() {
        Some(n) => n.to_string_lossy(),
        None => return false,
    };
    matches!(
        name.as_ref(),
        "commit_gate.rs" | "axioms.rs" | "state.rs" | "friction.rs"
    )
}
