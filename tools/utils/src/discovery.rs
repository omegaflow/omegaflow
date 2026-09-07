use omegaflow::force::gate_weigh;
use std::collections::HashSet;
use std::io::Write;

pub const LIBRARY_PATH: &str = "phi/pipeline/library.φ";
pub const CATALOG_DIR: &str = "phi/pipeline/catalog";
pub const MASTER_URLS_PATH: &str = "phi/pipeline/master_urls.txt";
pub const CANDIDATES_PATH: &str = "phi/pipeline/probe_url_candidates.txt";

const CANDIDATE_WEIGHT_FLOOR: i32 = 1;

pub fn source_url_candidates_run() -> i32 {
    let library = match std::fs::read_to_string(LIBRARY_PATH) {
        Ok(c) => omegaflow::force::parse_library(&c),
        Err(_) => {
            eprintln!("lens: library unreadable: {}", LIBRARY_PATH);
            return 1;
        }
    };

    let mut catalog_paths: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(CATALOG_DIR) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().map(|e| e == "φ").unwrap_or(false) {
                catalog_paths.push(p.to_string_lossy().into_owned());
            }
        }
    }
    catalog_paths.sort();

    let mut candidates: Vec<String> = Vec::new();

    for path in &catalog_paths {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("lens: catalog unreadable: {}", path);
                continue;
            }
        };
        for line in content.lines() {
            let t = line.trim();
            let stripped = if let Some(r) = t.strip_prefix("url ") {
                r
            } else if let Some(r) = t.strip_prefix("candidate ") {
                r
            } else {
                continue;
            };
            let Some(token) = stripped
                .split_whitespace()
                .find(|tok| tok.starts_with("http://") || tok.starts_with("https://"))
            else {
                continue;
            };
            let g = gate_weigh(token, &library);
            if g.weight >= CANDIDATE_WEIGHT_FLOOR {
                candidates.push(token.to_string());
            }
        }
    }

    candidates.sort();
    candidates.dedup();

    let mut known: HashSet<String> = HashSet::new();
    if let Ok(c) = std::fs::read_to_string(MASTER_URLS_PATH) {
        known = c.lines().map(|l| l.trim().to_string()).collect();
    }
    let known_count = candidates.iter().filter(|u| known.contains(*u)).count();
    let new_count = candidates.len() - known_count;

    match write_lines(CANDIDATES_PATH, candidates.iter()) {
        Ok(()) => {
            eprintln!(
                "lens: {} candidates ({} known to master_urls, {} new)",
                candidates.len(),
                known_count,
                new_count
            );
            0
        }
        Err(_) => {
            eprintln!("lens: output unwritable: {}", CANDIDATES_PATH);
            1
        }
    }
}

fn write_lines<'a, I, S>(path: &str, lines: I) -> std::io::Result<()>
where
    I: Iterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut f = std::fs::File::create(path)?;
    for line in lines {
        f.write_all(line.as_ref().as_bytes())?;
        f.write_all(b"\n")?;
    }
    Ok(())
}
