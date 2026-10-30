use std::path::PathBuf;

fn work_root() -> PathBuf {
    match std::env::var_os("OMEGAFLOW_WORK") {
        Some(w) => PathBuf::from(w),
        None => match std::env::var_os("HOME") {
            Some(h) => PathBuf::from(h).join("projects/omegaflow-work"),
            None => PathBuf::new(),
        },
    }
}

fn main() {
    let out = match std::env::args().nth(1) {
        Some(p) => PathBuf::from(p),
        None => work_root().join("build/sources.\u{03c6}"),
    };
    let work = work_root().join("research/sources/sources.tsv");
    let text = match std::fs::read_to_string(&work) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("register_release: read {}: {e}", work.display());
            std::process::exit(2);
        }
    };
    let mut lines: Vec<String> = Vec::new();
    let mut applied = 0usize;
    let mut rows = 0usize;
    for line in text.lines() {
        if line.starts_with('#') {
            continue;
        }
        if line.trim().is_empty() {
            lines.push(String::new());
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 2 {
            continue;
        }
        let original = cols.first().copied().unwrap_or("");
        if original == "original" {
            continue;
        }
        if original.trim().is_empty() && cols.get(1).map(|c| c.trim()).unwrap_or("").is_empty() {
            lines.push(String::new());
            continue;
        }
        rows += 1;
        let correction = cols.get(1).map(|c| c.trim()).unwrap_or("");
        if correction.is_empty() {
            lines.push(original.to_string());
        } else {
            applied += 1;
            lines.push(correction.to_string());
        }
    }
    if let Some(parent) = out.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match std::fs::write(&out, lines.join("\n") + "\n") {
        Ok(()) => {}
        Err(e) => {
            eprintln!("register_release: write {}: {e}", out.display());
            std::process::exit(2);
        }
    }
    println!(
        "register_release: {rows} rows, {applied} corrections applied -> {}",
        out.display()
    );
}
