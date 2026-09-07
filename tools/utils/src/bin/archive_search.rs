use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const SKIP_DIRS: &[&str] = &[
    ".git",
    "target",
    "node_modules",
    ".opencode",
    "__pycache__",
    ".cache",
    ".venv",
    "venv",
    ".local",
    ".config",
    "Library",
    "log",
    "storage",
    "session_diff",
    "tool-output",
    "tmp",
    "Trash",
];
const SNIPPET_CHARS: usize = 200;

struct State {
    scanned: u64,
    matched: u64,
    results: Vec<(PathBuf, usize, Vec<String>)>,
}

struct LeadScan {
    scanned: u64,
    prose: u64,
    skipped: HashSet<String>,
    leads: HashMap<String, HostLead>,
}

struct HostLead {
    matches: usize,
    lines: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut roots: Vec<String> = Vec::new();
    let mut lines_per_file = 2usize;
    let mut max_files = 40usize;
    let mut max_mb = 100u64;
    let mut keywords: Vec<String> = Vec::new();
    let mut leads_mode = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--leads" => leads_mode = true,
            "--root" => {
                i += 1;
                if let Some(r) = args.get(i) {
                    roots.push(r.clone());
                }
            }
            "--lines" => {
                i += 1;
                if let Some(n) = args.get(i).and_then(|s| s.parse().ok()) {
                    lines_per_file = n;
                }
            }
            "--files" => {
                i += 1;
                if let Some(n) = args.get(i).and_then(|s| s.parse().ok()) {
                    max_files = n;
                }
            }
            "--max-mb" => {
                i += 1;
                if let Some(n) = args.get(i).and_then(|s| s.parse().ok()) {
                    max_mb = n;
                }
            }
            other => keywords.push(other.to_string()),
        }
        i += 1;
    }

    if keywords.is_empty() {
        eprintln!(
            "usage: archive_search <keyword> [<keyword>...] [--root <dir>]... [--lines <n>] [--files <n>] [--max-mb <n>]"
        );
        eprintln!(
            "       archive_search --leads <keyword> [<keyword>...] [--root <dir>]... [--lines <n>] [--files <n>] [--max-mb <n>]"
        );
        eprintln!(
            "       --leads scans the un-curated candidate homes only and subtracts hosts already registered in phi/sources.\u{3c6}, phi/blocked_sources.\u{3c6}, phi/dead_sources.\u{3c6}"
        );
        std::process::exit(2);
    }

    if leads_mode {
        run_leads(&roots, &keywords, lines_per_file, max_files, max_mb);
    } else {
        run_plain(&roots, &keywords, lines_per_file, max_files, max_mb);
    }
}

fn run_plain(
    roots_given: &[String],
    keywords: &[String],
    lines_per_file: usize,
    max_files: usize,
    max_mb: u64,
) {
    let mut roots: Vec<String> = Vec::new();
    if roots_given.is_empty() {
        match env::var("HOME") {
            Ok(h) if !h.is_empty() => roots.push(h),
            _ => roots.push(".".to_string()),
        }
    } else {
        roots.extend_from_slice(roots_given);
    }

    let needle: Vec<String> = keywords.iter().map(|k| k.to_lowercase()).collect();

    let mut state = State {
        scanned: 0,
        matched: 0,
        results: Vec::new(),
    };
    for root in &roots {
        walk(Path::new(root), &needle, lines_per_file, max_mb, &mut state);
    }
    state.results.sort_by(|a, b| b.1.cmp(&a.1));
    let shown = state.results.len().min(max_files);
    for (path, _count, hits) in state.results.iter().take(shown) {
        println!("{}", path.display());
        for line in hits {
            println!("  {}", line);
        }
    }
    eprintln!(
        "archive_search: scanned {} files | matched {} | shown {}",
        state.scanned, state.matched, shown
    );
}

fn run_leads(
    roots_given: &[String],
    keywords: &[String],
    lines_per_file: usize,
    max_files: usize,
    max_mb: u64,
) {
    let repo = match find_repo_root() {
        Some(root) => root,
        None => {
            match env::current_dir() {
                Ok(cwd) => eprintln!(
                    "archive_search --leads: phi/sources.\u{3c6} absent in {} and its parents",
                    cwd.display()
                ),
                Err(_) => {
                    eprintln!(
                        "archive_search --leads: phi/sources.\u{3c6} absent, current dir unknown"
                    )
                }
            }
            std::process::exit(2);
        }
    };
    let sources_path = repo.join("phi").join("sources.\u{3c6}");
    let sources = match fs::read_to_string(&sources_path) {
        Ok(text) => text,
        Err(_) => {
            eprintln!(
                "archive_search --leads: {} unreadable",
                sources_path.display()
            );
            std::process::exit(2);
        }
    };
    let mut register_docs: Vec<String> = Vec::with_capacity(4);
    register_docs.push(sources);
    if let Some(text) = optional_text(&repo.join("phi").join("blocked_sources.\u{3c6}")) {
        register_docs.push(text);
    }
    if let Some(text) = optional_text(&repo.join("phi").join("dead_sources.\u{3c6}")) {
        register_docs.push(text);
    }
    if let Some(text) = optional_text(&repo.join("phi").join("witnesses.\u{3c6}")) {
        register_docs.push(text);
    }
    let curated = curated_hosts(&register_docs);

    let mut roots: Vec<PathBuf> = Vec::new();
    if roots_given.is_empty() {
        roots.push(repo.join("phi").join("pipeline").join("queue"));
        roots.push(repo.join("phi").join("pipeline").join("stage"));
        roots.push(repo.join("phi").join("pipeline").join("catalog"));
        if let Ok(home) = env::var("HOME") {
            if !home.is_empty() {
                let home_path = PathBuf::from(home);
                roots.push(
                    home_path
                        .join("backup")
                        .join("archive")
                        .join("apis")
                        .join("harvester-leads.txt"),
                );
                roots.push(
                    home_path
                        .join("backup")
                        .join("archive")
                        .join("apis")
                        .join("APIs"),
                );
            }
        }
    } else {
        for root in roots_given {
            roots.push(PathBuf::from(root));
        }
    }

    let needle: Vec<String> = keywords.iter().map(|k| k.to_lowercase()).collect();
    let mut scan = LeadScan {
        scanned: 0,
        prose: 0,
        skipped: HashSet::new(),
        leads: HashMap::new(),
    };
    for root in &roots {
        if root.is_dir() {
            walk_leads(root, &needle, &curated, lines_per_file, max_mb, &mut scan);
        } else if root.is_file() {
            scan_leads_file(root, &needle, &curated, lines_per_file, max_mb, &mut scan);
        }
    }

    let mut ranked: Vec<(&String, &HostLead)> = scan.leads.iter().collect();
    ranked.sort_by(|a, b| b.1.matches.cmp(&a.1.matches).then_with(|| a.0.cmp(b.0)));
    for (host, lead) in ranked.iter().take(max_files) {
        println!("{}", host);
        for line in &lead.lines {
            println!("  {}", line);
        }
    }
    if scan.scanned == 0 {
        eprintln!("archive_search --leads: no files found under the lead roots");
    }
    eprintln!(
        "archive_search --leads: scanned {} files | {} new leads (hosts not in sources.\u{3c6}), {} skipped (already curated)",
        scan.scanned,
        scan.leads.len(),
        scan.skipped.len()
    );
    if scan.prose > 0 {
        eprintln!(
            "archive_search --leads: {} matched lines without a url (prose lead, host absent)",
            scan.prose
        );
    }
}

fn find_repo_root() -> Option<PathBuf> {
    if let Ok(env_root) = env::var("OMEGAFLOW_REPO") {
        if !env_root.is_empty() {
            let root = PathBuf::from(env_root);
            if root.join("phi").join("sources.\u{3c6}").is_file() {
                return Some(root);
            }
            return None;
        }
    }
    let mut dir = env::current_dir().ok()?;
    loop {
        if dir.join("phi").join("sources.\u{3c6}").is_file() {
            return Some(dir);
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => return None,
        }
    }
}

fn optional_text(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn curated_hosts(register_docs: &[String]) -> HashSet<String> {
    let mut hosts: HashSet<String> = HashSet::new();
    for doc in register_docs {
        for line in doc.lines() {
            if !line.trim_start().starts_with("url ") {
                continue;
            }
            for host in url_hosts(line) {
                hosts.insert(host);
            }
        }
    }
    hosts
}

fn split_known_hosts(line: &str, curated: &HashSet<String>) -> (Vec<String>, Vec<String>) {
    let mut new_hosts: Vec<String> = Vec::new();
    let mut known_hosts: Vec<String> = Vec::new();
    for host in url_hosts(line) {
        if curated.contains(&host) {
            known_hosts.push(host);
        } else {
            new_hosts.push(host);
        }
    }
    (new_hosts, known_hosts)
}

fn url_hosts(text: &str) -> Vec<String> {
    let mut hosts: Vec<String> = Vec::new();
    let mut cursor = text;
    loop {
        let http = cursor.find("http://");
        let https = cursor.find("https://");
        let pos = match (http, https) {
            (Some(a), Some(b)) => a.min(b),
            (Some(a), None) => a,
            (None, Some(b)) => b,
            (None, None) => break,
        };
        let rest = &cursor[pos..];
        let after_scheme = if rest.starts_with("https://") {
            &rest[8..]
        } else {
            &rest[7..]
        };
        if let Some(host) = host_of(after_scheme) {
            if !hosts.contains(&host) {
                hosts.push(host);
            }
        }
        cursor = after_scheme;
    }
    hosts
}

fn host_of(after_scheme: &str) -> Option<String> {
    let end = after_scheme.find(|c: char| {
        matches!(
            c,
            '/' | '?'
                | '#'
                | ' '
                | '\t'
                | '\n'
                | '\r'
                | '"'
                | '\''
                | '<'
                | '>'
                | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | ','
                | ';'
        )
    });
    let raw = match end {
        Some(position) => &after_scheme[..position],
        None => after_scheme,
    };
    let raw = match raw.find('@') {
        Some(position) => &raw[position + 1..],
        None => raw,
    };
    if raw.is_empty() {
        return None;
    }
    let raw = match raw.find(':') {
        Some(position) => &raw[..position],
        None => raw,
    };
    if raw.is_empty() {
        return None;
    }
    let host = raw.trim_end_matches('.').to_lowercase();
    if host.is_empty() {
        None
    } else {
        Some(host)
    }
}

fn walk_leads(
    dir: &Path,
    needle: &[String],
    curated: &HashSet<String>,
    lines_per_file: usize,
    max_mb: u64,
    scan: &mut LeadScan,
) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        paths.push(entry.path());
    }
    paths.sort();
    for path in paths {
        let name = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => continue,
        };
        if path.is_dir() {
            if !SKIP_DIRS.contains(&name.as_str()) {
                walk_leads(&path, needle, curated, lines_per_file, max_mb, scan);
            }
        } else {
            scan_leads_file(&path, needle, curated, lines_per_file, max_mb, scan);
        }
    }
}

fn scan_leads_file(
    path: &Path,
    needle: &[String],
    curated: &HashSet<String>,
    lines_per_file: usize,
    max_mb: u64,
    scan: &mut LeadScan,
) {
    scan.scanned += 1;
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return,
    };
    if bytes.len() > max_mb as usize * 1024 * 1024 {
        return;
    }
    if is_binary(&bytes) {
        return;
    }
    let text = String::from_utf8_lossy(&bytes);
    for (idx, line) in text.lines().enumerate() {
        if !line_matches(line, needle) {
            continue;
        }
        let (new_hosts, known_hosts) = split_known_hosts(line, curated);
        if new_hosts.is_empty() && known_hosts.is_empty() {
            scan.prose += 1;
            continue;
        }
        for host in known_hosts {
            scan.skipped.insert(host);
        }
        for host in new_hosts {
            let lead = scan.leads.entry(host).or_insert_with(|| HostLead {
                matches: 0,
                lines: Vec::new(),
            });
            lead.matches += 1;
            let shown = format!(
                "{}:{}: {}",
                path.display(),
                idx + 1,
                truncate(line.trim(), SNIPPET_CHARS)
            );
            if lead.lines.len() < lines_per_file && !lead.lines.contains(&shown) {
                lead.lines.push(shown);
            }
        }
    }
}

fn walk(dir: &Path, needle: &[String], lines_per_file: usize, max_mb: u64, state: &mut State) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        paths.push(entry.path());
    }
    paths.sort();
    for path in paths {
        let name = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => continue,
        };
        if path.is_dir() {
            if SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            walk(&path, needle, lines_per_file, max_mb, state);
        } else {
            state.scanned += 1;
            if let Some((count, hits)) = search_file(&path, needle, lines_per_file, max_mb) {
                state.matched += 1;
                state.results.push((path, count, hits));
            }
        }
    }
}

fn search_file(
    path: &Path,
    needle: &[String],
    max_lines: usize,
    max_mb: u64,
) -> Option<(usize, Vec<String>)> {
    let bytes = fs::read(path).ok()?;
    if bytes.len() > max_mb as usize * 1024 * 1024 {
        return None;
    }
    if is_binary(&bytes) {
        return None;
    }
    let text = String::from_utf8_lossy(&bytes);
    let mut count = 0usize;
    let mut hits: Vec<String> = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        if !line_matches(line, needle) {
            continue;
        }
        count += 1;
        if hits.len() < max_lines {
            hits.push(format!(
                "{}: {}",
                idx + 1,
                truncate(line.trim(), SNIPPET_CHARS)
            ));
        }
    }
    if count == 0 {
        None
    } else {
        Some((count, hits))
    }
}

fn line_matches(line: &str, needle: &[String]) -> bool {
    let lower = line.to_lowercase();
    needle.iter().any(|n| lower.contains(n))
}

fn is_binary(bytes: &[u8]) -> bool {
    bytes[..bytes.len().min(8192)].contains(&0u8)
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max).collect();
        out.push('\u{2026}');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_leaves_short_strings() {
        assert_eq!(truncate("short", 200), "short");
    }

    #[test]
    fn truncate_cuts_on_char_boundary() {
        let out = truncate(&"中".repeat(300), 10);
        assert_eq!(out.chars().count(), 11);
        assert!(out.ends_with('\u{2026}'));
    }

    #[test]
    fn line_matches_is_case_insensitive_and_any_keyword() {
        let needle = vec!["icecube".to_string(), "telescope".to_string()];
        assert!(line_matches("an ICECUBE alert", &needle));
        assert!(line_matches("the Telescope Array", &needle));
        assert!(!line_matches("a plain line", &needle));
    }

    #[test]
    fn is_binary_detects_nul_byte() {
        assert!(is_binary(&[0x41, 0x00, 0x42]));
        assert!(!is_binary(b"plain text"));
    }

    #[test]
    fn search_file_finds_keyword_in_temp_file() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("archive_search_test_{}.txt", std::process::id()));
        fs::write(
            &path,
            "first line\nICECUBE alert here\nICECUBE again\nthird line\n",
        )
        .unwrap();
        let needle = vec!["icecube".to_string()];
        let (count, hits) = search_file(&path, &needle, 2, 100).unwrap();
        assert_eq!(count, 2);
        assert_eq!(hits.len(), 2);
        assert!(hits[0].contains("ICECUBE"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn search_file_skips_binary() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("archive_search_bin_{}.dat", std::process::id()));
        fs::write(&path, [0x41, 0x00, 0x42]).unwrap();
        let needle = vec!["a".to_string()];
        assert!(search_file(&path, &needle, 2, 100).is_none());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn relevance_counts_all_matches_beyond_display() {
        let body: String = "needle line\nplain\n".repeat(50);
        let dir = std::env::temp_dir();
        let path = dir.join(format!("archive_search_rel_{}.txt", std::process::id()));
        fs::write(&path, &body).unwrap();
        let needle = vec!["needle".to_string()];
        let (count, hits) = search_file(&path, &needle, 2, 100).unwrap();
        assert_eq!(count, 50);
        assert_eq!(hits.len(), 2);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn url_hosts_extracts_normalizes_and_dedups() {
        let hosts = url_hosts(
            "see https://Host.Example.COM:8443/a?b=c and http://Example.ORG/x) plus https://host.example.com/dup",
        );
        assert_eq!(
            hosts,
            vec!["host.example.com".to_string(), "example.org".to_string()]
        );
        assert!(url_hosts("no scheme here").is_empty());
        assert!(url_hosts("ftp://ftp.example.net/x").is_empty());
    }

    #[test]
    fn leads_exclude_hosts_registered_in_sources() {
        let register = vec![
            "url https://dataverse.harvard.edu/api/datasets/:persistentId?persistentId=doi:10.7910/DVN/Y4D8PA\n".to_string(),
            "url http://example.org/data\n".to_string(),
            "url ftp://ftp.example.net/x\n".to_string(),
            "reg https://urs.earthdata.nasa.gov/users/new\n".to_string(),
        ];
        let curated = curated_hosts(&register);
        assert_eq!(curated.len(), 2);
        assert!(curated.contains("dataverse.harvard.edu"));
        assert!(curated.contains("example.org"));
        assert!(!curated.contains("ftp.example.net"));
        assert!(!curated.contains("urs.earthdata.nasa.gov"));

        let known_line = "url https://dataverse.harvard.edu/api/datasets/:persistentId?persistentId=doi:10.7910/DVN/RJLBOQ";
        let (new1, known1) = split_known_hosts(known_line, &curated);
        assert!(new1.is_empty());
        assert_eq!(known1, vec!["dataverse.harvard.edu".to_string()]);

        let fresh_line = "url https://skyview.gsfc.nasa.gov/current/cgi/query.pl";
        let (new2, known2) = split_known_hosts(fresh_line, &curated);
        assert_eq!(new2, vec!["skyview.gsfc.nasa.gov".to_string()]);
        assert!(known2.is_empty());

        let prose_line = "note probe-open dataverse: optical photometry via the SQL API";
        let (new3, known3) = split_known_hosts(prose_line, &curated);
        assert!(new3.is_empty());
        assert!(known3.is_empty());
    }
}
