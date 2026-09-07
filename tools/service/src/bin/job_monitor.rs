use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{stdin, stdout, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::{id, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const JOB_KEYWORDS: [&str; 11] = [
    "solar", "matrix", "measure", "seconds", "galileo", "corona", "watchdog", "omega", "harvest",
    "probe", "monitor",
];

const CI_REPO: &str = "omegaflow/omegaflow";
const LOG_DIRS: [&str; 3] = ["data/reports", "state/reports", "/tmp/opencode"];

struct UnitJob {
    name: String,
    load: String,
    active: String,
    sub: String,
    pid: Option<u64>,
    exec: Option<String>,
}

struct ProcJob {
    pid: u64,
    etime: String,
    cpu: String,
    name: String,
}

struct CiRun {
    name: String,
    workflow: String,
    branch: String,
    status: String,
    conclusion: String,
    created: Option<u64>,
    done_steps: Option<usize>,
    total_steps: Option<usize>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interval = 5u64;
    let mut i = 1;
    while i + 1 < args.len() {
        if args[i] == "--interval" {
            if let Ok(n) = args[i + 1].parse::<u64>() {
                interval = n;
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    if interval == 0 {
        interval = 1;
    }

    let tty = stdout().is_terminal();
    if !tty {
        print!("{}", dashboard(interval, false));
        return;
    }

    let quit = Arc::new(AtomicBool::new(false));
    if stdin().is_terminal() {
        let q = Arc::clone(&quit);
        thread::spawn(move || {
            let mut line = String::new();
            loop {
                line.clear();
                match stdin().read_line(&mut line) {
                    Ok(0) => {
                        q.store(true, Ordering::Relaxed);
                        break;
                    }
                    Ok(_) => {
                        let word = line.trim();
                        if word == "q" || word == "quit" {
                            q.store(true, Ordering::Relaxed);
                            break;
                        }
                    }
                    Err(_) => {
                        q.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            }
        });
    }

    print!("\x1b[?25l");
    loop {
        let text = dashboard(interval, true);
        print!("\x1b[2J\x1b[H{}", text);
        let _ = stdout().flush();
        let ticks = interval * 4;
        let mut stop = false;
        for _ in 0..ticks {
            thread::sleep(Duration::from_millis(250));
            if quit.load(Ordering::Relaxed) {
                stop = true;
                break;
            }
        }
        if stop {
            break;
        }
    }
    print!("\x1b[?25h\x1b[0m");
}

fn dashboard(interval: u64, color: bool) -> String {
    let mut lines = Vec::new();
    lines.push(paint(
        color,
        "1;7",
        &format!(
            " {}  omegaflow job monitor | local jobs + GitHub Actions | refresh {}s | q quits ",
            clock_label(),
            interval
        ),
    ));
    lines.push(String::new());
    lines.extend(local_panel(color));
    lines.push(String::new());
    lines.extend(ci_panel(color));
    lines.push(String::new());
    lines.push(paint(
        color,
        "2",
        "◉ running jobs    ✓ success    ✗ failure    ▶ in-progress    − cancelled",
    ));
    lines.join("\n")
}

fn local_panel(color: bool) -> Vec<String> {
    let mut lines = Vec::new();
    let units = systemd_jobs();
    let active: BTreeSet<u64> = units.iter().filter_map(|u| u.pid).collect();
    let procs = ps_jobs(&active);
    lines.push(paint(
        color,
        "1;36",
        &format!(
            "▌ LOCAL JOBS ▐  {} systemd unit{} · {} process{}",
            units.len(),
            if units.len() == 1 { "" } else { "s" },
            procs.len(),
            if procs.len() == 1 { "" } else { "s" },
        ),
    ));
    if units.is_empty() && procs.is_empty() {
        lines.push(paint(color, "2", "   no omegaflow jobs running"));
    }
    for u in &units {
        lines.extend(unit_lines(u, color));
    }
    for p in &procs {
        lines.extend(proc_lines(p, color));
    }
    lines
}

fn unit_lines(u: &UnitJob, color: bool) -> Vec<String> {
    let mut lines = Vec::new();
    let (code, icon) = unit_style(&u.sub);
    let name = clean_unit(&u.name);
    let state = format!("{}/{}/{}", u.load, u.active, u.sub);
    let pid = match u.pid {
        Some(p) => format!("pid {}", p),
        None => "no main pid".to_string(),
    };
    lines.push(format!(
        "  {} {}  {}  {}",
        paint(color, code, icon),
        fit(&name, 42),
        paint(color, code, &state),
        paint(color, "2", &pid),
    ));
    if let Some(exec) = &u.exec {
        lines.push(paint(
            color,
            "2",
            &format!("      cmd {}", truncate(exec, 118)),
        ));
    }
    let keys = job_keys(u.exec.as_deref(), &u.name);
    if let Some(hint) = log_hint(&keys, Some(&u.name)) {
        lines.push(paint(color, "2", &format!("      log {}", hint)));
    }
    lines
}

fn proc_lines(p: &ProcJob, color: bool) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "  {} {} {} {} {}",
        paint(color, "36", "+"),
        format!("pid {:<6}", p.pid),
        paint(color, "2", &fit(&format!("elapsed {}", p.etime), 18)),
        paint(color, "2", &fit(&format!("cpu {}%", p.cpu), 12)),
        fit(&p.name, 50),
    ));
    let keys = vec![p.name.clone()];
    if let Some(hint) = log_hint(&keys, None) {
        lines.push(paint(color, "2", &format!("      log {}", hint)));
    }
    lines
}

fn ci_panel(color: bool) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(paint(
        color,
        "1;35",
        "▌ CI RUNS ▐  github.com/omegaflow/omegaflow · newest first · in-progress on top",
    ));
    let now = unix_now_secs();
    let runs = ci_runs();
    if runs.is_empty() {
        lines.push(paint(
            color,
            "2",
            "  no ci runs visible (gh unreachable or no runs)",
        ));
        return lines;
    }
    for r in &runs {
        lines.push(ci_run_line(r, now, color));
    }
    lines
}

fn ci_run_line(r: &CiRun, now: u64, color: bool) -> String {
    let sc = state_color(&r.status, &r.conclusion);
    let mark = ci_mark(&r.status, &r.conclusion);
    let name_part = format!(
        "{} {} {} {} {}",
        paint(color, sc, mark),
        paint(color, "0", &fit(&r.name, 40)),
        paint(color, "36", &fit(&r.workflow, 18)),
        fit(&r.branch, 16),
        fit(&age_label(now, r.created), 8),
    );
    if r.status == "in_progress" {
        let bar = progress_bar(r.done_steps, r.total_steps, 16);
        let pct = progress_pct(r.done_steps, r.total_steps);
        format!(
            "  {}{} {}",
            name_part,
            paint(color, "33", &bar),
            paint(color, "33", &pct),
        )
    } else {
        format!(
            "  {}{}",
            name_part,
            paint(color, sc, &fit(&verdict_word(&r.status, &r.conclusion), 10)),
        )
    }
}

fn progress_bar(done: Option<usize>, total: Option<usize>, width: usize) -> String {
    let (d, t) = match (done, total) {
        (Some(d), Some(t)) if t > 0 => (d.min(t), t),
        _ => return "[ waiting ]".to_string(),
    };
    let filled = (d as f64 / t as f64 * width as f64).round() as usize;
    let filled = filled.min(width);
    let empty = width - filled;
    format!("[{}>{}]", "█".repeat(filled), "░".repeat(empty))
}

fn progress_pct(done: Option<usize>, total: Option<usize>) -> String {
    match (done, total) {
        (Some(d), Some(t)) if t > 0 => {
            format!("{:>4}%", (d as f64 / t as f64 * 100.0).round() as u64)
        }
        _ => "  --  ".to_string(),
    }
}

fn systemd_jobs() -> Vec<UnitJob> {
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

fn ps_jobs(active: &BTreeSet<u64>) -> Vec<ProcJob> {
    let mut jobs = Vec::new();
    let Some(text) = run_cmd("ps", &["-eo", "pid=,etime=,pcpu=,args="]) else {
        return jobs;
    };
    let me = id() as u64;
    for line in text.lines() {
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 4 {
            continue;
        }
        let pid: u64 = match toks[0].parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        if pid == me || active.contains(&pid) {
            continue;
        }
        let argv0 = toks[3];
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
            cpu: toks[2].to_string(),
            name: bin,
        });
    }
    jobs
}

fn ci_runs() -> Vec<CiRun> {
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

fn log_hint(keys: &[String], journal: Option<&str>) -> Option<String> {
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

fn journal_tail(unit: &str) -> Option<String> {
    let text = run_cmd("journalctl", &["--unit", unit, "--no-pager", "-n", "1"])?;
    let s = text.trim();
    if s.is_empty() {
        None
    } else {
        Some(truncate(s, 160).to_string())
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
        Some(truncate(l, 140).to_string())
    }
}

fn job_keys(exec: Option<&str>, unit: &str) -> Vec<String> {
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

fn clean_unit(name: &str) -> String {
    match name.strip_suffix(".service") {
        Some(s) => s.to_string(),
        None => name.to_string(),
    }
}

fn hits_keyword(s: &str) -> bool {
    for k in JOB_KEYWORDS {
        if s.contains(k) {
            return true;
        }
    }
    false
}

fn unit_style(sub: &str) -> (&'static str, &'static str) {
    if sub == "running" {
        ("32", "●")
    } else if sub == "activating" {
        ("33", "◐")
    } else {
        ("33", "◑")
    }
}

fn state_color(status: &str, conclusion: &str) -> &'static str {
    if status == "in_progress" {
        "1;33"
    } else if conclusion == "success" {
        "1;32"
    } else if conclusion == "failure" {
        "1;31"
    } else if conclusion == "cancelled" || conclusion == "skipped" {
        "2"
    } else {
        "0"
    }
}

fn ci_mark(status: &str, conclusion: &str) -> &'static str {
    if status == "in_progress" {
        "▶"
    } else if conclusion == "success" {
        "✓"
    } else if conclusion == "failure" {
        "✗"
    } else if conclusion == "cancelled" {
        "−"
    } else if conclusion == "skipped" {
        "·"
    } else {
        "?"
    }
}

fn verdict_word(status: &str, conclusion: &str) -> String {
    if status == "in_progress" {
        return "running".to_string();
    }
    if conclusion.is_empty() {
        return status.to_string();
    }
    conclusion.to_string()
}

fn age_label(now: u64, created: Option<u64>) -> String {
    let created = match created {
        Some(c) => c,
        None => return "–".to_string(),
    };
    if created >= now {
        return "now".to_string();
    }
    let d = now - created;
    let days = d / 86400;
    let hours = (d % 86400) / 3600;
    let minutes = (d % 3600) / 60;
    let seconds = d % 60;
    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
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

fn clock_label() -> String {
    let now = SystemTime::now();
    let secs = match now.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => 0,
    };
    let local = secs as i64 + local_utc_offset();
    let s = local.rem_euclid(86400);
    format!("{:02}:{:02}:{:02} local", s / 3600, (s % 3600) / 60, s % 60)
}

fn local_utc_offset() -> i64 {
    let tz = env::var("TZ").ok();
    if tz.as_deref() == Some("UTC") || tz.as_deref() == Some("Etc/UTC") {
        return 0;
    }
    let out = Command::new("date")
        .args(["+%z"])
        .output()
        .ok()
        .filter(|o| o.status.success());
    match out {
        Some(o) => {
            let s = String::from_utf8_lossy(&o.stdout);
            let s = s.trim();
            if s.len() == 5 && (s.starts_with('+') || s.starts_with('-')) {
                let sign: i64 = if s.starts_with('-') { -1 } else { 1 };
                let h: i64 = match s[1..3].parse() {
                    Ok(v) => v,
                    Err(_) => 0,
                };
                let m: i64 = match s[3..5].parse() {
                    Ok(v) => v,
                    Err(_) => 0,
                };
                sign * (h * 3600 + m * 60)
            } else {
                0
            }
        }
        None => 0,
    }
}

fn unix_now_secs() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => 0,
    }
}

fn paint(color: bool, code: &str, s: &str) -> String {
    if color {
        format!("\x1b[{}m{}\x1b[0m", code, s)
    } else {
        s.to_string()
    }
}

fn truncate(s: &str, n: usize) -> String {
    let count = s.chars().count();
    if count <= n {
        s.to_string()
    } else {
        s.chars().take(n).collect()
    }
}

fn fit(s: &str, width: usize) -> String {
    let t = truncate(s, width);
    let mut out = t;
    while out.chars().count() < width {
        out.push(' ');
    }
    out
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
    fn age_units() {
        assert_eq!(age_label(1000, Some(910)), "1m 30s");
        assert_eq!(age_label(1000, Some(6400)), "1h 30m");
        assert_eq!(age_label(1000, Some(1000)), "now");
        assert_eq!(age_label(1000, None), "–");
    }

    #[test]
    fn verdict_words() {
        assert_eq!(verdict_word("in_progress", ""), "running");
        assert_eq!(verdict_word("completed", "success"), "success");
        assert_eq!(verdict_word("completed", ""), "completed");
    }

    #[test]
    fn text_widths() {
        assert_eq!(fit("abc", 5), "abc  ");
        assert_eq!(truncate("abcdef", 3), "abc");
    }
}
