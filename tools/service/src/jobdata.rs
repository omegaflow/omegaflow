use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{id, Command};
use std::time::{SystemTime, UNIX_EPOCH};

const JOB_KEYWORDS: [&str; 12] = [
    "solar", "matrix", "measure", "seconds", "galileo", "corona", "watchdog", "omega", "harvest",
    "probe", "monitor", "compiler",
];

const CI_REPO: &str = "omegaflow/omegaflow";
const LOG_DIRS: [&str; 3] = ["data/reports", "state/reports", "/tmp/opencode"];

pub struct UnitJob {
    pub name: String,
    pub load: String,
    pub active: String,
    pub sub: String,
    pub pid: Option<u64>,
    pub exec: Option<String>,
}

pub struct ProcJob {
    pub pid: u64,
    pub etime: String,
    pub cpu: f64,
    pub rss_kb: u64,
    pub name: String,
    pub exec: Option<String>,
}

pub struct Progress {
    pub done: u64,
    pub total: u64,
}

pub struct CiRun {
    pub name: String,
    pub workflow: String,
    pub branch: String,
    pub status: String,
    pub conclusion: String,
    pub created: Option<u64>,
    pub done_steps: Option<usize>,
    pub total_steps: Option<usize>,
}

pub fn loadavg() -> Option<f64> {
    let text = fs::read_to_string("/proc/loadavg").ok()?;
    let first = text.split_whitespace().next()?;
    first.parse().ok()
}

pub fn n_cpus() -> usize {
    fs::read_to_string("/proc/cpuinfo")
        .map(|s| s.matches("processor").count())
        .unwrap_or(1)
}

pub fn mem_frac() -> (f64, f64, f64) {
    let text = match fs::read_to_string("/proc/meminfo") {
        Ok(t) => t,
        Err(_) => String::new(),
    };
    let mut total = 0.0f64;
    let mut avail = 0.0f64;
    let parse_kb = |s: &str| -> f64 {
        match s.split_whitespace().nth(1).and_then(|v| v.parse().ok()) {
            Some(v) => v,
            None => 0.0,
        }
    };
    for line in text.lines() {
        if line.starts_with("MemTotal:") {
            total = parse_kb(line);
        } else if line.starts_with("MemAvailable:") {
            avail = parse_kb(line);
        }
    }
    let total_gb = total / 1048576.0;
    let used_gb = (total - avail) / 1048576.0;
    let frac = if total > 0.0 {
        (total - avail) / total
    } else {
        0.0
    };
    (used_gb, total_gb, frac)
}

pub fn proc_metrics(pid: u64) -> Option<(f64, u64, String)> {
    let text = run_cmd("ps", &["-o", "pcpu=,rss=", "-p", &pid.to_string()])?;
    let toks: Vec<&str> = text.split_whitespace().collect();
    if toks.len() < 2 {
        return None;
    }
    let cpu = toks[0].parse().ok()?;
    let rss = toks[1].parse().ok()?;
    let etime = match run_cmd("ps", &["-o", "etime=", "-p", &pid.to_string()]) {
        Some(s) => s.trim().to_string(),
        None => String::new(),
    };
    Some((cpu, rss, etime))
}

pub fn systemd_jobs() -> Vec<UnitJob> {
    let mut jobs = Vec::new();
    let Some(text) = run_cmd(
        "systemctl",
        &["list-units", "--type=service", "--all", "--no-legend"],
    ) else {
        return jobs;
    };
    for line in text.lines() {
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 5 {
            continue;
        }
        let name = toks[0];
        if !name.ends_with(".service") || !hits_keyword(name) {
            continue;
        }
        let sub = toks[3];
        if sub != "running" && sub != "activating" && sub != "deactivating" {
            continue;
        }
        let pid = main_pid_of(name);
        let exec = exec_start_of(name);
        jobs.push(UnitJob {
            name: name.to_string(),
            load: toks[1].to_string(),
            active: toks[2].to_string(),
            sub: sub.to_string(),
            pid,
            exec,
        });
    }
    jobs
}

pub fn ps_jobs(active: &BTreeSet<u64>) -> Vec<ProcJob> {
    let mut jobs = Vec::new();
    let Some(text) = run_cmd("ps", &["-eo", "pid=,etime=,pcpu=,rss=,args="]) else {
        return jobs;
    };
    let me = id() as u64;
    for line in text.lines() {
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 5 {
            continue;
        }
        let pid: u64 = match toks[0].parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        if pid == me || active.contains(&pid) {
            continue;
        }
        let argv0 = toks[4];
        let lower = argv0.to_lowercase();
        let has_target = lower.contains("target/release/") || lower.contains("target/debug/");
        if !has_target {
            continue;
        }
        let bin = match Path::new(argv0).file_name() {
            Some(f) => f.to_string_lossy().to_string(),
            None => continue,
        };
        if bin == "job_monitor" {
            continue;
        }
        if !hits_keyword(&bin) && !bin.contains("omegaflow") {
            continue;
        }
        jobs.push(ProcJob {
            pid,
            etime: toks[1].to_string(),
            cpu: match toks[2].parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            },
            rss_kb: match toks[3].parse() {
                Ok(v) => v,
                Err(_) => 0,
            },
            name: bin,
            exec: Some(toks[4..].join(" ")),
        });
    }
    jobs
}

pub fn ci_runs() -> Vec<CiRun> {
    let jq = ".[] | [.databaseId, .name, .headBranch, .status, .conclusion, .createdAt, .workflowName, .event] | @tsv";
    let mut runs = Vec::new();
    let Some(text) = run_cmd(
        "gh",
        &[
            "run",
            "list",
            "--repo",
            CI_REPO,
            "--limit",
            "12",
            "--json",
            "databaseId,name,headBranch,status,conclusion,createdAt,workflowName,event",
            "--jq",
            jq,
        ],
    ) else {
        return runs;
    };
    for line in text.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 8 {
            continue;
        }
        let field = |idx: usize| -> String {
            if f[idx] == "null" {
                String::new()
            } else {
                f[idx].to_string()
            }
        };
        let id = field(0);
        let status = field(3);
        let (done, total) = if status == "in_progress" {
            run_progress(&id)
        } else {
            (None, None)
        };
        runs.push(CiRun {
            name: field(1),
            branch: field(2),
            status,
            conclusion: field(4),
            created: parse_rfc3339(&field(5)),
            workflow: field(6),
            done_steps: done,
            total_steps: total,
        });
    }
    let mut open = Vec::new();
    let mut closed = Vec::new();
    for r in runs {
        if r.status == "in_progress" {
            open.push(r);
        } else {
            closed.push(r);
        }
    }
    open.extend(closed);
    open
}

pub fn log_hint(keys: &[String], journal: Option<&str>) -> Option<String> {
    for key in keys {
        let exact = format!("data/reports/{}.log", key);
        if let Some(line) = tail_of(Path::new(&exact)) {
            return Some(line);
        }
    }
    for key in keys {
        let exact = format!("state/reports/{}.φ", key);
        if let Some(line) = tail_of(Path::new(&exact)) {
            return Some(line);
        }
    }
    if let Some(unit) = journal {
        if let Some(line) = journal_tail(unit) {
            return Some(line);
        }
    }
    for key in keys {
        for dir in LOG_DIRS {
            if let Some(line) = newest_dir_match(dir, key) {
                return Some(line);
            }
        }
    }
    None
}

pub fn proc_progress(exec: Option<&str>, name: &str) -> Option<Progress> {
    let year = exec.and_then(extract_year);
    let has_c2 = exec.map(|e| e.contains("--confound2")).unwrap_or(false);
    let variants = key_variants(name);
    for dir in LOG_DIRS {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        let mut best: Option<(u64, PathBuf)> = None;
        for ent in entries.flatten() {
            let path = ent.path();
            if !path.is_file() {
                continue;
            }
            let fname = ent.file_name().to_string_lossy().to_string();
            if !variants.iter().any(|k| fname.contains(k)) {
                continue;
            }
            if let Some(y) = &year {
                if !fname.contains(y) {
                    continue;
                }
            }
            if has_c2 {
                if !fname.contains("335") {
                    continue;
                }
            } else if fname.contains("335") {
                continue;
            }
            let meta = match ent.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            if let Some(s) = mtime_secs(&meta) {
                if best.as_ref().map(|(b, _)| s > *b).unwrap_or(true) {
                    best = Some((s, path));
                }
            }
        }
        if let Some((_, path)) = best {
            if let Some(p) = progress_of(&path) {
                return Some(p);
            }
        }
    }
    None
}

fn progress_of(path: &Path) -> Option<Progress> {
    let text = fs::read_to_string(path).ok()?;
    let mut last: Option<(u64, u64)> = None;
    for line in text.lines() {
        if let Some(p) = parse_progress_line(line) {
            last = Some(p);
        }
    }
    last.map(|(done, total)| Progress { done, total })
}

fn parse_progress_line(line: &str) -> Option<(u64, u64)> {
    let l = line.trim();
    let rest = l.strip_prefix("progress ")?;
    let mut it = rest.split_whitespace();
    let done: u64 = it.next()?.parse().ok()?;
    let slash = it.next()?;
    if slash != "/" {
        return None;
    }
    let total: u64 = it.next()?.parse().ok()?;
    Some((done, total))
}

fn extract_year(s: &str) -> Option<String> {
    let b = s.as_bytes();
    for i in 0..b.len().saturating_sub(3) {
        let w = &b[i..i + 4];
        if w.iter().all(|c| c.is_ascii_digit())
            && ((w[0] == b'1' && w[1] == b'9') || (w[0] == b'2' && w[1] == b'0'))
        {
            return Some(String::from_utf8_lossy(w).to_string());
        }
    }
    None
}

pub fn job_keys(exec: Option<&str>, unit: &str) -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();
    if let Some(exec) = exec {
        if let Some(bin) = exec_target_bin(exec) {
            for v in key_variants(&bin) {
                push_key(&mut keys, v);
            }
        }
    }
    let clean = clean_unit(unit);
    for v in key_variants(&clean) {
        push_key(&mut keys, v);
    }
    keys
}

pub fn clean_unit(name: &str) -> String {
    match name.strip_suffix(".service") {
        Some(s) => s.to_string(),
        None => name.to_string(),
    }
}

pub fn truncate(s: &str, n: usize) -> String {
    let count = s.chars().count();
    if count <= n {
        s.to_string()
    } else {
        s.chars().take(n).collect()
    }
}

pub fn term_width() -> usize {
    #[repr(C)]
    struct Winsize {
        ws_row: u16,
        ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }
    unsafe extern "C" {
        fn ioctl(fd: i32, request: u64, ...) -> i32;
    }
    const TIOCGWINSZ: u64 = 0x5413;
    let mut ws = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let ok = unsafe { ioctl(1, TIOCGWINSZ, &mut ws as *mut Winsize) };
    if ok == 0 && ws.ws_col > 0 {
        ws.ws_col as usize
    } else {
        100
    }
}

pub fn unix_now_secs() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => 0,
    }
}

fn run_progress(id: &str) -> (Option<usize>, Option<usize>) {
    let jq = "[.jobs[].steps[]] | {total:length, done:(map(select(.status==\"completed\"))|length)} | [.done,.total] | @tsv";
    let args = [
        "run", "view", id, "--repo", CI_REPO, "--json", "jobs", "--jq", jq,
    ];
    let Some(text) = run_cmd("gh", &args) else {
        return (None, None);
    };
    let f: Vec<&str> = text.split('\t').collect();
    if f.len() < 2 {
        return (None, None);
    }
    let done = f[0].trim().parse().ok();
    let total = f[1].trim().parse().ok();
    (done, total)
}

fn run_cmd(program: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(program).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn main_pid_of(unit: &str) -> Option<u64> {
    let text = run_cmd("systemctl", &["show", "-p", "MainPID", "--value", unit])?;
    let s = text.trim();
    if s.is_empty() {
        return None;
    }
    match s.parse::<u64>() {
        Ok(n) if n > 0 => Some(n),
        _ => None,
    }
}

fn exec_start_of(unit: &str) -> Option<String> {
    let text = run_cmd("systemctl", &["show", "-p", "ExecStart", "--value", unit])?;
    let s = text.trim();
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

fn journal_tail(unit: &str) -> Option<String> {
    let text = run_cmd("journalctl", &["--unit", unit, "--no-pager", "-n", "1"])?;
    let s = text.trim();
    if s.is_empty() {
        None
    } else {
        Some(truncate(s, term_width().saturating_sub(10)).to_string())
    }
}

fn newest_dir_match(dir: &str, key: &str) -> Option<String> {
    let entries = fs::read_dir(dir).ok()?;
    let mut best: Option<(u64, PathBuf)> = None;
    for ent in entries.flatten() {
        let path = ent.path();
        if !path.is_file() {
            continue;
        }
        let fname = ent.file_name().to_string_lossy().to_string();
        if !fname.contains(key) {
            continue;
        }
        let meta = match ent.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        let secs = match mtime_secs(&meta) {
            Some(s) => s,
            None => continue,
        };
        let replace = match &best {
            Some((b, _)) => secs > *b,
            None => true,
        };
        if replace {
            best = Some((secs, path));
        }
    }
    match best {
        Some((_, path)) => tail_of(&path),
        None => None,
    }
}

fn mtime_secs(meta: &fs::Metadata) -> Option<u64> {
    match meta.modified() {
        Ok(t) => match t.duration_since(UNIX_EPOCH) {
            Ok(d) => Some(d.as_secs()),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

fn tail_of(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    let text = String::from_utf8_lossy(&bytes);
    let trimmed = text.trim_end();
    let last = match trimmed.rsplit('\n').next() {
        Some(l) => l,
        None => return None,
    };
    let l = last.trim();
    if l.is_empty() {
        None
    } else {
        Some(truncate(l, term_width().saturating_sub(10)).to_string())
    }
}

fn key_variants(name: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = name.to_string();
    loop {
        if out.is_empty() || out.last() != Some(&cur) {
            out.push(cur.clone());
        }
        match cur.rfind('_') {
            Some(pos) if pos > 0 => {
                cur = cur[..pos].to_string();
            }
            _ => break,
        }
    }
    out
}

fn push_key(keys: &mut Vec<String>, key: String) {
    if !key.is_empty() && !keys.contains(&key) {
        keys.push(key);
    }
}

fn exec_target_bin(exec: &str) -> Option<String> {
    for marker in ["target/release/", "target/debug/"] {
        if let Some(pos) = exec.find(marker) {
            let tail = &exec[pos..];
            let token = match tail.split_whitespace().next() {
                Some(t) => t,
                None => marker,
            };
            if let Some(file) = Path::new(token).file_name() {
                return Some(file.to_string_lossy().to_string());
            }
        }
    }
    None
}

fn hits_keyword(s: &str) -> bool {
    for k in JOB_KEYWORDS {
        if s.contains(k) {
            return true;
        }
    }
    false
}

fn parse_rfc3339(s: &str) -> Option<u64> {
    if s.len() < 20 {
        return None;
    }
    if s.get(4..5)? != "-" || s.get(7..8)? != "-" || s.get(10..11)? != "T" {
        return None;
    }
    if s.get(13..14)? != ":" || s.get(16..17)? != ":" {
        return None;
    }
    let year: i64 = s.get(0..4)?.parse().ok()?;
    let month: u32 = s.get(5..7)?.parse().ok()?;
    let day: u32 = s.get(8..10)?.parse().ok()?;
    let hour: i64 = s.get(11..13)?.parse().ok()?;
    let minute: i64 = s.get(14..16)?.parse().ok()?;
    let second: i64 = s.get(17..19)?.parse().ok()?;
    if month == 0 || month > 12 || day == 0 || day > 31 {
        return None;
    }
    if hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    let rest = &s[19..];
    let rest = match rest.strip_prefix('.') {
        Some(fraction) => {
            let digits = fraction.chars().take_while(|c| c.is_ascii_digit()).count();
            &fraction[digits..]
        }
        None => rest,
    };
    let offset: i64 = if rest == "Z" {
        0
    } else if let Some(off) = rest.strip_prefix('+') {
        parse_offset(off)?
    } else if let Some(off) = rest.strip_prefix('-') {
        -parse_offset(off)?
    } else {
        return None;
    };
    let days = days_from_civil(year, month, day);
    let epoch = days * 86400 + hour * 3600 + minute * 60 + second - offset;
    if epoch < 0 {
        None
    } else {
        Some(epoch as u64)
    }
}

fn parse_offset(s: &str) -> Option<i64> {
    if s.len() != 5 || s.get(2..3)? != ":" {
        return None;
    }
    let h: i64 = s.get(0..2)?.parse().ok()?;
    let m: i64 = s.get(3..5)?.parse().ok()?;
    if h > 23 || m > 59 {
        return None;
    }
    Some(h * 3600 + m * 60)
}

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = ((m as i64) + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc3339_epoch() {
        assert_eq!(parse_rfc3339("1970-01-01T00:00:00Z"), Some(0));
    }

    #[test]
    fn rfc3339_leap_day() {
        assert_eq!(parse_rfc3339("2020-02-29T12:34:56Z"), Some(1582979696));
    }

    #[test]
    fn rfc3339_offset() {
        assert_eq!(parse_rfc3339("2020-02-29T10:04:56-02:30"), Some(1582979696));
    }

    #[test]
    fn rfc3339_fraction() {
        assert_eq!(parse_rfc3339("2020-02-29T12:34:56.789Z"), Some(1582979696));
    }

    #[test]
    fn unit_name_clean() {
        assert_eq!(clean_unit("omegaflow-x.service"), "omegaflow-x");
        assert_eq!(clean_unit("omegaflow-x"), "omegaflow-x");
    }

    #[test]
    fn keyword_match() {
        assert!(hits_keyword("omegaflow-solar_seconds_matrix_probe.service"));
        assert!(!hits_keyword("systemd-logind.service"));
    }

    #[test]
    fn exec_bin_extraction() {
        let exec = "/home/o/omegaflow/target/release/solar_seconds_matrix_probe --scan";
        assert_eq!(
            exec_target_bin(exec),
            Some("solar_seconds_matrix_probe".to_string())
        );
    }

    #[test]
    fn key_drop_suffix() {
        let variants = key_variants("solar_seconds_matrix_probe");
        assert!(variants.contains(&"solar_seconds_matrix_probe".to_string()));
        assert!(variants.contains(&"solar_seconds_matrix".to_string()));
        assert!(variants.contains(&"solar".to_string()));
    }

    #[test]
    fn truncate_clips() {
        assert_eq!(truncate("abcdef", 3), "abc");
        assert_eq!(truncate("abc", 5), "abc");
    }

    #[test]
    fn parse_progress_line_forms() {
        assert_eq!(parse_progress_line("progress 400 / 522"), Some((400, 522)));
        assert_eq!(
            parse_progress_line("  progress 0   /   10  "),
            Some((0, 10))
        );
        assert_eq!(parse_progress_line("progress 400/522"), None);
        assert_eq!(parse_progress_line("304A->131A | 0s | 1.19e-2"), None);
    }

    #[test]
    fn progress_of_scans_last_line() {
        let path = Path::new("/tmp/opencode/progress_scan_test.φ");
        let _ = std::fs::remove_file(path);
        std::fs::write(path, "progress 100 / 900\nprogress 250 / 900\n").ok();
        let p = progress_of(path).unwrap();
        assert_eq!((p.done, p.total), (250, 900));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn year_extraction() {
        assert_eq!(
            extract_year(
                "/home/omegaflow/data/jsoc.stanford.edu/aia2013_fullyear.bin --confound goes"
            ),
            Some("2013".to_string())
        );
        assert_eq!(extract_year("dispersion --write-shelf"), None);
    }

    #[test]
    fn missing_year_gives_void() {
        assert_eq!(extract_year("  nope 22"), None);
    }
}
