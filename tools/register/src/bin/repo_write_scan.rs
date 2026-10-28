use std::path::Path;

const REPO_PREFIXES: &[&str] = &[
    "phi/", "tools/", "docs/", "data/", "state/", "cache/", "queue/", "stage/", "research/", "src/",
    "bin/",
];

const WRITE_CALLS: &[&str] = &[
    "fs::write(",
    "std::fs::write(",
    "File::create(",
    "OpenOptions::new(",
    "create_dir_all(",
    "fs::copy(",
    "fs::rename(",
];

const WORK_HINTS: &[&str] = &[
    "base.join",
    "temp_dir",
    "temp_path",
    "asset_path",
    "work_root",
    "pipeline_path",
    "pipeline_root",
    "work.join",
];

struct Hit {
    line: usize,
    literal: String,
}

fn first_literal(line: &str) -> Option<String> {
    let start = line.find('"')?;
    let rest = &line[start + 1..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn scan_text(text: &str) -> Vec<Hit> {
    let mut hits = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with("#") {
            continue;
        }
        let call_at = WRITE_CALLS
            .iter()
            .filter_map(|c| line.find(c).map(|at| at + c.len()))
            .min();
        let Some(after_open) = call_at else {
            continue;
        };
        if WORK_HINTS.iter().any(|h| line.contains(h)) || line.contains(".join(") {
            continue;
        }
        let argument = line[after_open..].trim_start();
        if !argument.starts_with('"') {
            continue;
        }
        let Some(literal) = first_literal(argument) else {
            continue;
        };
        if !REPO_PREFIXES.iter().any(|p| literal.starts_with(p)) {
            continue;
        }
        let _ = &lines;
        hits.push(Hit {
            line: idx + 1,
            literal,
        });
    }
    hits
}

fn walk(dir: &Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().and_then(|n| n.to_str()) == Some("target") {
                continue;
            }
            walk(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path.to_string_lossy().into_owned());
        }
    }
}

fn main() {
    let root = match std::env::args().nth(1) {
        Some(p) => p,
        None => ".".to_string(),
    };
    let mut files = Vec::new();
    for dir in ["src", "tools"] {
        walk(&Path::new(&root).join(dir), &mut files);
    }
    files.sort();
    let mut total = 0usize;
    for file in &files {
        let Ok(text) = std::fs::read_to_string(file) else {
            continue;
        };
        for hit in scan_text(&text) {
            total += 1;
            println!("{}:{}  {}", file, hit.line, hit.literal);
        }
    }
    println!("repo_write_scan: {total} Schreibstellen in den Repo-Baum");
    if total > 0 {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::scan_text;

    #[test]
    fn a_repo_literal_in_a_write_call_counts() {
        let text = "    std::fs::write(\"phi/sources.φ\", out).ok();\n";
        let hits = scan_text(text);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].literal, "phi/sources.φ");
    }

    #[test]
    fn a_temp_base_fixture_does_not_count() {
        let text = "    fs::create_dir_all(base.join(\"docs/handover\")).unwrap();\n";
        assert!(scan_text(text).is_empty());
    }

    #[test]
    fn a_resolved_work_path_does_not_count() {
        let text = "    std::fs::write(asset_path::raw(NETLOC, \"rtsw.txt\"), body);\n";
        assert!(scan_text(text).is_empty());
    }

    #[test]
    fn a_message_that_names_a_path_is_not_a_write() {
        let text = "    eprintln!(\"write phi/sources.φ: the register does not remember\");\n";
        assert!(scan_text(text).is_empty());
    }
}
