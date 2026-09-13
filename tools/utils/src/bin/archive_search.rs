#[path = "archive_search/datacite.rs"]
mod datacite;
#[path = "archive_search/git.rs"]
mod git;
#[path = "archive_search/index.rs"]
mod index;
#[path = "archive_search/json.rs"]
mod json;
#[path = "archive_search/magic.rs"]
mod magic;
#[path = "archive_search/net.rs"]
mod net;
#[path = "archive_search/ntfs.rs"]
mod ntfs;
#[path = "archive_search/pdf.rs"]
mod pdf;
#[path = "archive_search/zenodo.rs"]
mod zenodo;
#[path = "archive_search/isc.rs"]
mod isc;
#[path = "archive_search/openalex.rs"]
mod openalex;
#[path = "archive_search/playwright.rs"]
mod playwright;
#[path = "archive_search/secrets.rs"]
mod secrets;
#[path = "archive_search/server.rs"]
mod server;
#[path = "archive_search/web.rs"]
mod web;

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{BufRead, IsTerminal};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

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

struct PlainResult {
    scanned: u64,
    matched: u64,
    lines: Vec<String>,
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

struct LeadsSummary {
    scanned: u64,
    leads: usize,
    skipped: usize,
    prose: u64,
}

#[derive(Clone, Copy)]
enum Mode {
    Plain,
    Leads,
    Mft,
    Index,
    Serve,
    Git,
    Verdict,
    Playwright,
    Net(&'static str),
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut roots: Vec<String> = Vec::new();
    let mut lines_per_file = 2usize;
    let mut max_files = 40usize;
    let mut max_mb = 100u64;
    let mut skip = 0usize;
    let mut binary = false;
    let mut content = false;
    let mut kind = index::Kind::Any;
    let mut sort = index::Sort::Name;
    let mut keywords: Vec<String> = Vec::new();
    let mut mode = Mode::Plain;
    let mut mft_path: Option<String> = None;
    let mut serve_addr: Option<String> = None;
    let mut verdict_url: Option<String> = None;
    let mut playwright_input: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--leads" => mode = Mode::Leads,
            "--index" => mode = Mode::Index,
            "--serve" => {
                mode = Mode::Serve;
                if let Some(a) = args.get(i + 1) {
                    if a.contains(':') && !a.starts_with('-') {
                        serve_addr = Some(a.clone());
                        i += 1;
                    }
                }
            }
            "--git" => mode = Mode::Git,
            "--mft" => {
                i += 1;
                if let Some(p) = args.get(i) {
                    mft_path = Some(p.clone());
                }
                mode = Mode::Mft;
            }
            "--verdict" => {
                i += 1;
                if let Some(u) = args.get(i) {
                    verdict_url = Some(u.clone());
                }
                mode = Mode::Verdict;
            }
            "--cacert" => {
                i += 1;
                if let Some(p) = args.get(i) {
                    net::set_ca_bundle(p);
                }
            }
            "--playwright" => {
                i += 1;
                if let Some(u) = args.get(i) {
                    playwright_input = Some(u.clone());
                }
                mode = Mode::Playwright;
            }
            "--arxiv" => mode = Mode::Net("arxiv"),
            "--ads" => mode = Mode::Net("ads"),
            "--ntrs" => mode = Mode::Net("ntrs"),
            "--wayback" => mode = Mode::Net("wayback"),
            "--crossref" => mode = Mode::Net("crossref"),
            "--wiki" => mode = Mode::Net("wiki"),
            "--github" => mode = Mode::Net("github"),
            "--crates" => mode = Mode::Net("crates"),
            "--librs" => mode = Mode::Net("librs"),
            "--brave" => mode = Mode::Net("brave"),
            "--datacite" => mode = Mode::Net("datacite"),
            "--sniff" => mode = Mode::Net("sniff"),
            "--zenodo" => mode = Mode::Net("zenodo"),
            "--isc" => mode = Mode::Net("isc"),
            "--openalex" => mode = Mode::Net("openalex"),
            "--kind" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    kind = match v.as_str() {
                        "file" => index::Kind::File,
                        "dir" => index::Kind::Dir,
                        _ => index::Kind::Any,
                    };
                }
            }
            "--sort" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    sort = match v.as_str() {
                        "size" => index::Sort::Size,
                        "mtime" => index::Sort::Mtime,
                        _ => index::Sort::Name,
                    };
                }
            }
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
            "--skip" => {
                i += 1;
                if let Some(n) = args.get(i).and_then(|s| s.parse().ok()) {
                    skip = n;
                }
            }
            "--binary" => binary = true,
            "--content" => content = true,
            other => keywords.push(other.to_string()),
        }
        i += 1;
    }

    match mode {
        Mode::Plain => {
            if keywords.is_empty() {
                if std::io::stdin().is_terminal() {
                    usage();
                    std::process::exit(2);
                }
                let stdin = std::io::stdin();
                let mut reader = stdin.lock();
                let mut line = String::new();
                loop {
                    line.clear();
                    let n = match reader.read_line(&mut line) {
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("archive_search: stdin read ends: {e}");
                            break;
                        }
                    };
                    if n == 0 {
                        break;
                    }
                    let query = line.trim();
                    if query.is_empty() {
                        continue;
                    }
                    let kws: Vec<String> =
                        query.split_whitespace().map(|s| s.to_string()).collect();
                    run_plain(
                        &roots,
                        &kws,
                        lines_per_file,
                        max_files,
                        max_mb,
                        skip,
                        binary,
                    );
                }
            } else {
                run_plain(
                    &roots,
                    &keywords,
                    lines_per_file,
                    max_files,
                    max_mb,
                    skip,
                    binary,
                );
            }
        }
        Mode::Leads => run_leads(&roots, &keywords, lines_per_file, max_files, max_mb),
        Mode::Mft => {
            let device = match &mft_path {
                Some(p) => p.clone(),
                None => {
                    eprintln!("archive_search --mft: the mode carries no device path");
                    std::process::exit(2);
                }
            };
            let lines = ntfs::run_lines(Path::new(&device), &keywords, max_files, max_mb, content);
            print_lines(&lines);
        }
        Mode::Index => {
            let lines = run_index(&roots, &keywords, max_files, kind, sort);
            print_lines(&lines);
        }
        Mode::Serve => run_serve(serve_addr, &roots, mft_path),
        Mode::Git => {
            let repo = match find_repo_root() {
                Some(r) => r,
                None => PathBuf::from("."),
            };
            let query = keywords.join(" ");
            let lines = git::run_lines(&repo, &query);
            print_lines(&lines);
        }
        Mode::Verdict => {
            let url = match verdict_url {
                Some(u) => u,
                None => {
                    eprintln!("archive_search --verdict: the mode carries no url");
                    std::process::exit(2);
                }
            };
            let lines = net::verdict_lines(&url);
            print_lines(&lines);
        }
        Mode::Playwright => {
            let input = match playwright_input {
                Some(u) => u,
                None => {
                    eprintln!("archive_search --playwright: the mode carries no url");
                    std::process::exit(2);
                }
            };
            let lines = playwright::run_lines(&input);
            print_lines(&lines);
        }
        Mode::Net(name) => {
            let query = keywords.join(" ");
            let env_map = match find_repo_root() {
                Some(repo) => secrets::load_env(&repo),
                None => env::vars().collect(),
            };
            let lines = net::run_lines(name, &query, &env_map);
            print_lines(&lines);
        }
    }
}

fn usage() {
    eprintln!(
        "usage: archive_search <keyword>... [--root <dir>]... [--lines <n>] [--files <n>] [--max-mb <n>] [--skip <n>] [--binary]"
    );
    eprintln!(
        "       archive_search --leads <keyword>... | --git <query> | --index [<query>...] | --mft <device> [<query>...] [--content] [--kind any|file|dir] [--sort name|size|mtime]   (--index matches paths, not file content)"
    );
    eprintln!(
        "       archive_search --verdict <url> | --sniff <url> | --arxiv|--ads|--ntrs|--wayback|--crossref|--wiki|--github|--crates|--librs|--brave|--datacite|--zenodo|--isc|--openalex <query> [--cacert <pem>]   (--ntrs: a bare citation id resolves via the citation path, any other query searches; --sniff reports magic bytes + sha256; --isc takes key=value: start/end/minmag/minlat/maxlat/minlon/maxlon)"
    );
    eprintln!(
        "       archive_search --playwright <url|query>   (real browser render: title, headings, links, text; a bare query searches)"
    );
    eprintln!(
        "       archive_search --serve [addr]   (foreground display, no writes, keys never cross the page)"
    );
    eprintln!(
        "       --leads scans the un-curated candidate homes only and subtracts hosts already registered in phi/sources.\u{3c6}, phi/blocked_sources.\u{3c6}, phi/dead_sources.\u{3c6}"
    );
}

fn print_lines(lines: &[String]) {
    for line in lines {
        println!("{}", line);
    }
}

fn now_unix() -> i64 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(neg) => -(neg.duration().as_secs() as i64),
    }
}

fn collect_plain(
    roots_given: &[String],
    keywords: &[String],
    lines_per_file: usize,
    max_files: usize,
    max_mb: u64,
    skip: usize,
    include_binary: bool,
) -> PlainResult {
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
        walk(
            Path::new(root),
            &needle,
            lines_per_file,
            max_mb,
            include_binary,
            &mut state,
        );
    }
    state.results.sort_by(|a, b| b.1.cmp(&a.1));
    let mut lines = Vec::new();
    for (path, _count, hits) in state.results.iter().skip(skip).take(max_files) {
        lines.push(path.display().to_string());
        for line in hits {
            lines.push(format!("  {}", line));
        }
    }
    PlainResult {
        scanned: state.scanned,
        matched: state.matched,
        lines,
    }
}

fn run_plain(
    roots_given: &[String],
    keywords: &[String],
    lines_per_file: usize,
    max_files: usize,
    max_mb: u64,
    skip: usize,
    include_binary: bool,
) {
    let result = collect_plain(
        roots_given,
        keywords,
        lines_per_file,
        max_files,
        max_mb,
        skip,
        include_binary,
    );
    for line in &result.lines {
        println!("{}", line);
    }
    let shown = result.lines.iter().filter(|l| !l.starts_with("  ")).count();
    eprintln!(
        "archive_search: scanned {} files | matched {} | shown {}",
        result.scanned, result.matched, shown
    );
}

fn collect_leads(
    repo: &Path,
    roots_given: &[String],
    keywords: &[String],
    lines_per_file: usize,
    max_files: usize,
    max_mb: u64,
) -> Result<(Vec<String>, LeadsSummary), String> {
    let sources_path = repo.join("phi").join("sources.\u{3c6}");
    let sources = fs::read_to_string(&sources_path).map_err(|_| {
        format!(
            "archive_search --leads: {} unreadable",
            sources_path.display()
        )
    })?;
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
    let mut lines = Vec::new();
    for (host, lead) in ranked.iter().take(max_files) {
        lines.push(host.to_string());
        for line in &lead.lines {
            lines.push(format!("  {}", line));
        }
    }
    let summary = LeadsSummary {
        scanned: scan.scanned,
        leads: scan.leads.len(),
        skipped: scan.skipped.len(),
        prose: scan.prose,
    };
    Ok((lines, summary))
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
    match collect_leads(
        &repo,
        roots_given,
        keywords,
        lines_per_file,
        max_files,
        max_mb,
    ) {
        Ok((lines, summary)) => {
            for line in &lines {
                println!("{}", line);
            }
            if summary.scanned == 0 {
                eprintln!("archive_search --leads: no files found under the lead roots");
            }
            eprintln!(
                "archive_search --leads: scanned {} files | {} new leads (hosts not in sources.\u{3c6}), {} skipped (already curated)",
                summary.scanned, summary.leads, summary.skipped
            );
            if summary.prose > 0 {
                eprintln!(
                    "archive_search --leads: {} matched lines without a url (prose lead, host absent)",
                    summary.prose
                );
            }
        }
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(2);
        }
    }
}

fn build_index(roots_given: &[String], repo: &Path) -> index::Index {
    let mut roots: Vec<PathBuf> = Vec::new();
    if roots_given.is_empty() {
        roots.push(repo.to_path_buf());
    } else {
        for root in roots_given {
            roots.push(PathBuf::from(root));
        }
    }
    let mut entries: Vec<index::Entry> = Vec::new();
    for root in &roots {
        if root.is_dir() {
            walk_index(root, u32::MAX, &mut entries, 0);
        }
    }
    index::Index {
        entries,
        labels: roots.iter().map(|r| r.display().to_string()).collect(),
        scanned_at: now_unix(),
    }
}

fn walk_index(dir: &Path, parent: u32, entries: &mut Vec<index::Entry>, depth: u32) {
    if depth > 64 {
        return;
    }
    let parent_index = entries.len() as u32;
    let name = match dir.file_name() {
        Some(n) => n.to_string_lossy().to_string(),
        None => dir.display().to_string(),
    };
    entries.push(index::Entry {
        parent,
        is_dir: true,
        size: 0,
        mtime: dir_mtime(dir),
        name,
    });
    let read = match fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return,
    };
    let mut paths: Vec<PathBuf> = read.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        let fname = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => continue,
        };
        if path.is_dir() {
            if SKIP_DIRS.contains(&fname.as_str()) {
                continue;
            }
            walk_index(&path, parent_index, entries, depth + 1);
        } else {
            let meta = match fs::metadata(&path) {
                Ok(m) => m,
                Err(_) => continue,
            };
            let mtime = match meta.modified() {
                Ok(t) => match t.duration_since(std::time::UNIX_EPOCH) {
                    Ok(d) => d.as_secs() as i64,
                    Err(neg) => -(neg.duration().as_secs() as i64),
                },
                Err(_) => continue,
            };
            entries.push(index::Entry {
                parent: parent_index,
                is_dir: false,
                size: meta.len(),
                mtime: Some(mtime),
                name: fname,
            });
        }
    }
}

fn dir_mtime(dir: &Path) -> Option<i64> {
    let m = fs::metadata(dir).ok()?;
    let t = m.modified().ok()?;
    match t.duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => Some(d.as_secs() as i64),
        Err(neg) => Some(-(neg.duration().as_secs() as i64)),
    }
}

fn run_index(
    roots_given: &[String],
    keywords: &[String],
    max_files: usize,
    kind: index::Kind,
    sort: index::Sort,
) -> Vec<String> {
    let repo = match find_repo_root() {
        Some(r) => r,
        None => PathBuf::from("."),
    };
    let idx = build_index(roots_given, &repo);
    let query = index::Query {
        text: keywords.join(" "),
        ci: true,
        path: false,
        kind,
        sort,
        limit: max_files,
    };
    let mut lines = Vec::new();
    for hit in idx.search(&query) {
        let mtext = match hit.mtime {
            Some(m) => m.to_string(),
            None => "absent".to_string(),
        };
        lines.push(format!(
            "{} ({} bytes, mtime {})",
            hit.path, hit.size, mtext
        ));
    }
    if lines.is_empty() {
        lines.push(format!(
            "absent — the index carries no path for: {}",
            keywords.join(" ")
        ));
    }
    lines
}

fn run_serve(addr: Option<String>, roots_given: &[String], mft_path: Option<String>) {
    let repo = match find_repo_root() {
        Some(r) => r,
        None => match env::current_dir() {
            Ok(d) => d,
            Err(_) => PathBuf::from("."),
        },
    };
    let bind = match addr {
        Some(a) => a,
        None => "127.0.0.1:1789".to_string(),
    };
    let env_map = secrets::load_env(&repo);
    let roots: Vec<PathBuf> = if roots_given.is_empty() {
        vec![repo.clone()]
    } else {
        roots_given.iter().map(PathBuf::from).collect()
    };
    let root_strings: Vec<String> = roots.iter().map(|p| p.display().to_string()).collect();
    let idx = build_index(&root_strings, &repo);
    let state = Arc::new(server::AppState {
        index: RwLock::new(idx),
        roots,
        mft: mft_path.map(PathBuf::from),
        repo,
        env: env_map,
    });
    if let Err(msg) = server::run(state, &bind) {
        eprintln!("{}", msg);
        std::process::exit(1);
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
    let text = match readable_text(&bytes, false) {
        Some(t) => t,
        None => return,
    };
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

fn walk(
    dir: &Path,
    needle: &[String],
    lines_per_file: usize,
    max_mb: u64,
    include_binary: bool,
    state: &mut State,
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
            if SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            walk(&path, needle, lines_per_file, max_mb, include_binary, state);
        } else {
            state.scanned += 1;
            if let Some((count, hits)) =
                search_file(&path, needle, lines_per_file, max_mb, include_binary)
            {
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
    include_binary: bool,
) -> Option<(usize, Vec<String>)> {
    let bytes = fs::read(path).ok()?;
    if bytes.len() > max_mb as usize * 1024 * 1024 {
        return None;
    }
    let text = match readable_text(&bytes, include_binary) {
        Some(t) => t,
        None => return None,
    };
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

fn readable_text(bytes: &[u8], include_binary: bool) -> Option<String> {
    match magic::magic_identity(bytes) {
        magic::Magic::Pdf => pdf::pdf_text(bytes),
        _ if is_binary(bytes) && !include_binary => None,
        _ => Some(String::from_utf8_lossy(bytes).to_string()),
    }
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
        let (count, hits) = search_file(&path, &needle, 2, 100, false).unwrap();
        assert_eq!(count, 2);
        assert_eq!(hits.len(), 2);
        assert!(hits[0].contains("ICECUBE"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn search_file_skips_binary_unless_requested() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("archive_search_bin_{}.dat", std::process::id()));
        fs::write(&path, [0x41, 0x00, 0x42]).unwrap();
        let needle = vec!["a".to_string()];
        assert!(search_file(&path, &needle, 2, 100, false).is_none());
        assert!(search_file(&path, &needle, 2, 100, true).is_some());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn relevance_counts_all_matches_beyond_display() {
        let body: String = "needle line\nplain\n".repeat(50);
        let dir = std::env::temp_dir();
        let path = dir.join(format!("archive_search_rel_{}.txt", std::process::id()));
        fs::write(&path, &body).unwrap();
        let needle = vec!["needle".to_string()];
        let (count, hits) = search_file(&path, &needle, 2, 100, false).unwrap();
        assert_eq!(count, 50);
        assert_eq!(hits.len(), 2);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn plain_search_skip_and_binary_flags_apply() {
        let dir = std::env::temp_dir().join(format!("archive_skip_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        for i in 0..3 {
            fs::write(dir.join(format!("f{}.txt", i)), "needle here\n").unwrap();
        }
        let roots = vec![dir.display().to_string()];
        let keywords = vec!["needle".to_string()];
        let all = collect_plain(&roots, &keywords, 2, 40, 100, 0, false);
        assert_eq!(all.matched, 3);
        let skipped = collect_plain(&roots, &keywords, 2, 40, 100, 2, false);
        let shown = skipped
            .lines
            .iter()
            .filter(|l| !l.starts_with("  "))
            .count();
        assert_eq!(shown, 1);
        let _ = fs::remove_dir_all(&dir);
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
