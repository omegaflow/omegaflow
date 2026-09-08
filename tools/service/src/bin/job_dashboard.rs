use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use omegaflow_service::jobdata::{
    ci_runs, clean_unit, job_keys, loadavg, log_hint, mem_frac, n_cpus, proc_metrics,
    proc_progress, ps_jobs, systemd_jobs, unix_now_secs, CiRun, ProcJob, UnitJob,
};

const INDEX_HTML: &str = include_str!("../assets/job_dashboard.html");

struct Snapshot {
    at: Instant,
    json: String,
}

fn main() {
    let port: u16 = env_u64("OMEGAFLOW_DASH_PORT", 1620) as u16;
    let interval = env_u64("OMEGAFLOW_DASH_INTERVAL", 5);
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("job-dashboard: bind 127.0.0.1:{} refused: {}", port, e);
            return;
        }
    };
    eprintln!(
        "job-dashboard: http://127.0.0.1:{} (data refresh {}s)",
        port, interval
    );
    let cache = Arc::new(Mutex::new(None::<Snapshot>));
    for conn in listener.incoming() {
        let Ok(mut stream) = conn else { continue };
        let cache = Arc::clone(&cache);
        std::thread::spawn(move || {
            let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
            let Some((method, path)) = read_request(&mut stream) else {
                return;
            };
            if method != "GET" {
                respond(&mut stream, 404, "text/plain", b"not found\n");
                return;
            }
            if path.starts_with("/health") {
                respond(&mut stream, 200, "text/plain", b"job-dashboard ok\n");
                return;
            }
            if path == "/data" {
                let body = data_json(&cache, interval);
                respond(&mut stream, 200, "application/json", body.as_bytes());
                return;
            }
            if path == "/" || path.starts_with("/index") {
                respond(
                    &mut stream,
                    200,
                    "text/html; charset=utf-8",
                    INDEX_HTML.as_bytes(),
                );
                return;
            }
            respond(&mut stream, 404, "text/plain", b"not found\n");
        });
    }
}

fn data_json(cache: &Arc<Mutex<Option<Snapshot>>>, interval: u64) -> String {
    let mut guard = match cache.lock() {
        Ok(g) => g,
        Err(_) => return "{}".to_string(),
    };
    if let Some(snap) = guard.as_ref() {
        if snap.at.elapsed().as_secs() < interval {
            return snap.json.clone();
        }
    }
    let json = build_json();
    *guard = Some(Snapshot {
        at: Instant::now(),
        json: json.clone(),
    });
    json
}

fn build_json() -> String {
    let now = unix_now_secs();
    let load = loadavg();
    let ncpu = n_cpus();
    let (used_gb, total_gb, _frac) = mem_frac();
    let units = systemd_jobs();
    let active: BTreeSet<u64> = units.iter().filter_map(|u| u.pid).collect();
    let procs = ps_jobs(&active);
    let runs = ci_runs();

    let mut s = String::new();
    s.push_str("{\"at\":");
    s.push_str(&now.to_string());
    s.push_str(",\"system\":{\"load\":");
    s.push_str(&opt_f2(load));
    s.push_str(",\"ncpu\":");
    s.push_str(&ncpu.to_string());
    s.push_str(",\"mem_used_gb\":");
    s.push_str(&format!("{:.1}", used_gb));
    s.push_str(",\"mem_total_gb\":");
    s.push_str(&format!("{:.1}", total_gb));
    s.push_str("}");
    s.push_str(",\"units\":[");
    for (i, u) in units.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push_str(&unit_json(u));
    }
    s.push_str("],\"procs\":[");
    for (i, p) in procs.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push_str(&proc_json(p));
    }
    s.push_str("],\"runs\":[");
    for (i, r) in runs.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push_str(&run_json(r));
    }
    s.push_str("]}");
    s
}

fn unit_json(u: &UnitJob) -> String {
    let name = clean_unit(&u.name);
    let metrics = u.pid.and_then(proc_metrics);
    let keys = job_keys(u.exec.as_deref(), &u.name);
    let log = log_hint(&keys, Some(&u.name));
    let mut s = String::new();
    s.push_str("{\"name\":");
    s.push_str(&q(&name));
    s.push_str(",\"load\":");
    s.push_str(&q(&u.load));
    s.push_str(",\"active\":");
    s.push_str(&q(&u.active));
    s.push_str(",\"sub\":");
    s.push_str(&q(&u.sub));
    s.push_str(",\"pid\":");
    match u.pid {
        Some(pid) => s.push_str(&pid.to_string()),
        None => s.push_str("null"),
    }
    s.push_str(",\"exec\":");
    match &u.exec {
        Some(e) => s.push_str(&q(e)),
        None => s.push_str("null"),
    }
    s.push_str(",\"cpu\":");
    match &metrics {
        Some((c, _, _)) => s.push_str(&format!("{:.2}", c)),
        None => s.push_str("null"),
    }
    s.push_str(",\"rss_kb\":");
    match &metrics {
        Some((_, r, _)) => s.push_str(&r.to_string()),
        None => s.push_str("null"),
    }
    s.push_str(",\"etime\":");
    match &metrics {
        Some((_, _, e)) => s.push_str(&q(e)),
        None => s.push_str("null"),
    }
    s.push_str(",\"log\":");
    match &log {
        Some(l) => s.push_str(&q(l)),
        None => s.push_str("null"),
    }
    s.push_str(",\"progress\":");
    match u.pid {
        Some(_) => match proc_progress(u.exec.as_deref(), &u.name) {
            Some(p) => s.push_str(&format!("{{\"done\":{},\"total\":{}}}", p.done, p.total)),
            None => s.push_str("null"),
        },
        None => s.push_str("null"),
    }
    s.push('}');
    s
}

fn proc_json(p: &ProcJob) -> String {
    let keys = vec![p.name.clone()];
    let log = log_hint(&keys, None);
    let mut s = String::new();
    s.push_str("{\"pid\":");
    s.push_str(&p.pid.to_string());
    s.push_str(",\"cpu\":");
    s.push_str(&format!("{:.2}", p.cpu));
    s.push_str(",\"rss_kb\":");
    s.push_str(&p.rss_kb.to_string());
    s.push_str(",\"etime\":");
    s.push_str(&q(&p.etime));
    s.push_str(",\"name\":");
    s.push_str(&q(&p.name));
    s.push_str(",\"log\":");
    match &log {
        Some(l) => s.push_str(&q(l)),
        None => s.push_str("null"),
    }
    s.push_str(",\"progress\":");
    match proc_progress(p.exec.as_deref(), &p.name) {
        Some(pr) => s.push_str(&format!("{{\"done\":{},\"total\":{}}}", pr.done, pr.total)),
        None => s.push_str("null"),
    }
    s.push('}');
    s
}

fn run_json(r: &CiRun) -> String {
    let mut s = String::new();
    s.push_str("{\"name\":");
    s.push_str(&q(&r.name));
    s.push_str(",\"workflow\":");
    s.push_str(&q(&r.workflow));
    s.push_str(",\"branch\":");
    s.push_str(&q(&r.branch));
    s.push_str(",\"status\":");
    s.push_str(&q(&r.status));
    s.push_str(",\"conclusion\":");
    s.push_str(&q(&r.conclusion));
    s.push_str(",\"created\":");
    match r.created {
        Some(c) => s.push_str(&c.to_string()),
        None => s.push_str("null"),
    }
    s.push_str(",\"done_steps\":");
    match r.done_steps {
        Some(d) => s.push_str(&d.to_string()),
        None => s.push_str("null"),
    }
    s.push_str(",\"total_steps\":");
    match r.total_steps {
        Some(t) => s.push_str(&t.to_string()),
        None => s.push_str("null"),
    }
    s.push('}');
    s
}

fn opt_f2(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{:.2}", x),
        None => "null".to_string(),
    }
}

fn q(s: &str) -> String {
    format!("\"{}\"", esc(s))
}

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn read_request(stream: &mut TcpStream) -> Option<(String, String)> {
    let mut buf: Vec<u8> = Vec::new();
    let mut tmp = [0u8; 4096];
    loop {
        match stream.read(&mut tmp) {
            Ok(0) => return None,
            Ok(n) => {
                buf.extend_from_slice(&tmp[..n]);
                if buf.windows(4).any(|w| w == b"\r\n\r\n") {
                    break;
                }
                if buf.len() > 8192 {
                    return None;
                }
            }
            Err(_) => return None,
        }
    }
    let head = String::from_utf8_lossy(&buf).to_string();
    let mut parts = head.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();
    Some((method, path))
}

fn respond(stream: &mut TcpStream, code: u16, ctype: &str, body: &[u8]) {
    let reason = match code {
        200 => "OK",
        404 => "Not Found",
        _ => "Error",
    };
    let head = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        code,
        reason,
        ctype,
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body);
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_escapes_quotes() {
        assert_eq!(esc("a\"b"), "a\\\"b");
        assert_eq!(esc("a\\b"), "a\\\\b");
        assert_eq!(esc("a\nb"), "a\\nb");
    }

    #[test]
    fn json_quote_wraps() {
        assert_eq!(q("ok"), "\"ok\"");
        assert_eq!(q(""), "\"\"");
    }

    #[test]
    fn optional_float_null() {
        assert_eq!(opt_f2(None), "null");
        assert_eq!(opt_f2(Some(1.0)), "1.00");
    }

    #[test]
    fn run_json_absent_progress_is_null() {
        let r = CiRun {
            name: "n".to_string(),
            workflow: "w".to_string(),
            branch: "main".to_string(),
            status: "completed".to_string(),
            conclusion: "success".to_string(),
            created: None,
            done_steps: None,
            total_steps: None,
        };
        let j = run_json(&r);
        assert!(j.contains("\"created\":null"));
        assert!(j.contains("\"done_steps\":null"));
        assert!(j.contains("\"conclusion\":\"success\""));
    }
}
