use std::collections::BTreeSet;
use std::env;
use std::io::{stdin, stdout, IsTerminal, Write};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use omegaflow_service::jobdata::{
    ci_runs, clean_unit, job_keys, loadavg, log_hint, mem_frac, n_cpus, proc_metrics, ps_jobs,
    systemd_jobs, term_width, truncate, unix_now_secs, CiRun, ProcJob, UnitJob,
};

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
        print!("{}\n", dashboard(interval, false).join("\n"));
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
        let frame = dashboard(interval, true);
        let width = term_width();
        print!("\x1b[H");
        let mut buf = String::new();
        for line in &frame {
            buf.push_str(&fit_visible(line, width));
            buf.push('\n');
        }
        print!("{}", buf);
        print!("\x1b[J");
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

fn dashboard(interval: u64, color: bool) -> Vec<String> {
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
    lines.extend(system_panel(color));
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
    lines
}

fn system_panel(color: bool) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(paint(color, "1;33", "▌ SYSTEM ▐"));
    let load = loadavg();
    let ncpu = n_cpus();
    let load = match load {
        Some(l) => l,
        None => 0.0,
    };
    let (used_gb, total_gb, frac) = mem_frac();
    let ncpu_f = ncpu as f64;
    let loadbar = meter_bar(load / ncpu_f, 12);
    let membar = meter_bar(frac, 12);
    let load_pct = ((load / ncpu_f) * 100.0).round() as u64;
    let mem_pct = (frac * 100.0).round() as u64;
    let load_col = if load / ncpu_f < 0.7 {
        "1;32"
    } else if load / ncpu_f < 1.3 {
        "1;33"
    } else {
        "1;31"
    };
    let mem_col = if frac < 0.7 {
        "1;32"
    } else if frac < 0.85 {
        "1;33"
    } else {
        "1;31"
    };
    lines.push(format!(
        "  {} load {}/{} {}  {:>3}%   {} mem {}/{} {}  {:>3}%",
        paint(color, load_col, "CPU"),
        format!("{:.2}", load),
        ncpu,
        loadbar,
        load_pct,
        paint(color, mem_col, "RAM"),
        format!("{:.1}G", used_gb),
        format!("{:.1}G", total_gb),
        membar,
        mem_pct,
    ));
    lines
}

fn meter_bar(frac: f64, width: usize) -> String {
    let frac = frac.clamp(0.0, 1.0);
    let filled = (frac * width as f64).round() as usize;
    let filled = filled.min(width);
    let empty = width - filled;
    format!("[{}|{}]", "█".repeat(filled), "─".repeat(empty))
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
            if procs.len() == 1 { "" } else { "es" },
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
    let metrics = match u.pid {
        Some(pid) => proc_metrics(pid),
        None => None,
    };
    let metric_str = match &metrics {
        Some((cpu, rss, etime)) => format!(
            "  {} cpu {:<6} {:<10} {:<16}",
            paint(color, "1;33", &spin_char().to_string()),
            format!("{:.0}%", cpu),
            fit(&format!("mem {}", fmt_size(*rss)), 10),
            fit(&format!("elapsed {}", etime), 16),
        ),
        None => String::new(),
    };
    let pid_str = match u.pid {
        Some(p) => format!("pid {}", p),
        None => "no main pid".to_string(),
    };
    lines.push(format!(
        "  {} {}  {}  {}",
        paint(color, code, icon),
        fit(&name, 42),
        paint(color, code, &state),
        paint(color, "2", &pid_str),
    ));
    if !metric_str.is_empty() {
        lines.push(metric_str);
    }
    if let Some(exec) = &u.exec {
        lines.push(paint(
            color,
            "2",
            &format!(
                "      cmd {}",
                truncate(exec, term_width().saturating_sub(10))
            ),
        ));
    }
    let keys = job_keys(u.exec.as_deref(), &u.name);
    if let Some(hint) = log_hint(&keys, Some(&u.name)) {
        lines.push(paint(color, "2", &format!("      log {}", hint)));
    }
    lines
}

fn spin_char() -> char {
    let frames = ['⣾', '⣽', '⣻', '⢿', '⡿', '⣟', '⣯', '⣷'];
    let ms = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_millis() as usize,
        Err(_) => 0,
    };
    frames[(ms / 180) % frames.len()]
}

fn fmt_size(kb: u64) -> String {
    if kb >= 1024 * 1024 {
        format!("{:.1}G", kb as f64 / (1024.0 * 1024.0))
    } else if kb >= 1024 {
        format!("{:.0}M", kb as f64 / 1024.0)
    } else {
        format!("{}K", kb)
    }
}

fn proc_lines(p: &ProcJob, color: bool) -> Vec<String> {
    let mut lines = Vec::new();
    let cpu = format!("{:.0}%", p.cpu);
    lines.push(format!(
        "  {} {} pid {:<6} {:<9} {:<11} {:<16} {}",
        paint(color, "36", "+"),
        paint(color, "1;33", &spin_char().to_string()),
        p.pid,
        format!("cpu {}", paint(color, "1;33", &cpu)),
        fit(&format!("mem {}", fmt_size(p.rss_kb)), 11),
        fit(&format!("elapsed {}", p.etime), 16),
        fit(&p.name, 40),
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

fn paint(color: bool, code: &str, s: &str) -> String {
    if color {
        format!("\x1b[{}m{}\x1b[0m", code, s)
    } else {
        s.to_string()
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

fn visible_len(s: &str) -> usize {
    let mut n = 0usize;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&nxt) = chars.peek() {
                    chars.next();
                    if (nxt as char).is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            n += 1;
        }
    }
    n
}

fn fit_visible(s: &str, width: usize) -> String {
    let vis = visible_len(s);
    if vis > width {
        truncate_visible(s, width)
    } else {
        let mut out = s.to_string();
        while visible_len(&out) < width {
            out.push(' ');
        }
        out
    }
}

fn truncate_visible(s: &str, width: usize) -> String {
    let mut out = String::new();
    let mut n = 0usize;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            out.push(c);
            if chars.peek() == Some(&'[') {
                out.push(chars.next().unwrap());
                while let Some(&nxt) = chars.peek() {
                    out.push(nxt);
                    chars.next();
                    if (nxt as char).is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            if n >= width {
                break;
            }
            out.push(c);
            n += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn age_units() {
        assert_eq!(age_label(1000, Some(910)), "1m 30s");
        assert_eq!(age_label(6400, Some(1000)), "1h 30m");
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
    }
}
