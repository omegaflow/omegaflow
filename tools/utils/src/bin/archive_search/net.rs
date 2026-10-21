use crate::json::{self, Json};
use crate::secrets::{Secret, resolve_key};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub struct Fetch {
    pub status: Option<i32>,
    pub body: String,
    pub raw: Vec<u8>,
    pub retry_after: Option<u64>,
    pub complete: bool,
}

impl Fetch {
    pub fn status_text(&self) -> String {
        match self.status {
            Some(s) => s.to_string(),
            None => "absent".to_string(),
        }
    }
}

static CA_BUNDLE: OnceLock<String> = OnceLock::new();

pub fn set_ca_bundle(path: &str) {
    let _ = CA_BUNDLE.set(path.to_string());
}

const DEFAULT_MIN_INTERVAL_MS: u64 = 1100;
const DEFAULT_RETRY_AFTER_SECS: u64 = 2;

const ARXIV_QUERY_WINDOW: usize = 2;

fn arxiv_window(max: usize) -> usize {
    max.clamp(1, ARXIV_QUERY_WINDOW)
}

fn split_curl_stdout(stdout: &[u8]) -> Option<(&[u8], i32, Option<u64>)> {
    let last = stdout.iter().rposition(|b| *b == b'\n')?;
    let prev = stdout[..last].iter().rposition(|b| *b == b'\n')?;
    let body = &stdout[..prev];
    let code = std::str::from_utf8(&stdout[prev + 1..last])
        .ok()?
        .trim()
        .parse::<i32>()
        .ok()?;
    let retry = std::str::from_utf8(&stdout[last + 1..])
        .ok()?
        .trim()
        .parse::<u64>()
        .ok();
    Some((body, code, retry))
}

fn curl_args(url: &str, extra: &[&str], timeout: &str, transport: &[String]) -> Vec<String> {
    let mut args: Vec<String> = vec![
        "-sL".to_string(),
        "-g".to_string(),
        "--max-time".to_string(),
        timeout.to_string(),
    ];
    if let Some(ca) = CA_BUNDLE.get() {
        args.push("--cacert".to_string());
        args.push(ca.clone());
    }
    for e in extra {
        args.push((*e).to_string());
    }
    for t in transport {
        args.push(t.clone());
    }
    args.push("-w".to_string());
    args.push("\n%{http_code}\n%header{retry-after}".to_string());
    args.push(url.to_string());
    args
}

fn curl_fetch(url: &str, extra: &[&str], timeout: &str, transport: &[String]) -> Option<Fetch> {
    let args = curl_args(url, extra, timeout, transport);
    let out = Command::new("curl").args(&args).output().ok()?;
    let (body, code, retry) = split_curl_stdout(&out.stdout)?;
    Some(Fetch {
        status: Some(code),
        body: String::from_utf8_lossy(body).to_string(),
        raw: body.to_vec(),
        retry_after: retry,
        complete: out.status.success(),
    })
}

#[derive(Clone)]
pub(crate) enum Exit {
    Direct,
    Iface(String),
    Socks(String),
}

impl Exit {
    fn transport_args(&self) -> Vec<String> {
        match self {
            Exit::Direct => Vec::new(),
            Exit::Iface(name) => vec!["--interface".to_string(), name.clone()],
            Exit::Socks(url) => vec!["--proxy".to_string(), url.clone()],
        }
    }

    fn label(&self) -> String {
        match self {
            Exit::Direct => "direct".to_string(),
            Exit::Iface(name) => name.clone(),
            Exit::Socks(url) => url.clone(),
        }
    }
}

fn get_once(url: &str, extra: &[&str], timeout: &str, exit: &Exit) -> Option<Fetch> {
    curl_fetch(url, extra, timeout, &exit.transport_args())
}

const VERDICT_ATTEMPTS: usize = 3;

fn is_transient(f: &Fetch) -> bool {
    matches!(f.status, None | Some(0))
}

fn is_found(f: &Fetch) -> bool {
    matches!(f.status, Some(200..=299))
}

fn retry_transient<F: FnMut() -> Option<Fetch>>(mut attempt: F) -> Option<Fetch> {
    let mut last = None;
    for _ in 0..VERDICT_ATTEMPTS {
        match attempt() {
            Some(f) if !is_transient(&f) => return Some(f),
            Some(f) => last = Some(f),
            None => {}
        }
    }
    last.filter(|f| !is_transient(f))
}

const RATE_LIMIT_ATTEMPTS: usize = 4;
const RATE_LIMIT_BACKOFF_SECS: u64 = 3;
const RATE_LIMIT_BACKOFF_MAX_SECS: u64 = 30;

fn is_rate_limited(f: &Fetch) -> bool {
    matches!(f.status, Some(429) | Some(503))
}

fn get_retrying(url: &str, extra: &[&str], timeout: &str) -> Option<Fetch> {
    let mut last: Option<Fetch> = None;
    for attempt in 0..RATE_LIMIT_ATTEMPTS {
        if attempt > 0 {
            let secs = last
                .as_ref()
                .and_then(|f| f.retry_after)
                .unwrap_or(RATE_LIMIT_BACKOFF_SECS)
                .min(RATE_LIMIT_BACKOFF_MAX_SECS);
            std::thread::sleep(Duration::from_secs(secs));
        }
        match get(url, extra, timeout) {
            Some(f) if !is_rate_limited(&f) => return Some(f),
            other => last = other,
        }
    }
    last
}

const VERDICT_RANGE: &[&str] = &["--range", "0-0"];

fn verdict_probe(url: &str, timeout: &str, exit: &Exit) -> Option<Fetch> {
    let result = retry_transient(|| get_once(url, VERDICT_RANGE, timeout, exit));
    if let Some(f) = &result {
        let host = url_host(url);
        if crate::token::is_earthdata_host(&host)
            && (crate::token::is_unauthorized(f.status) || is_edl_login_page(f))
        {
            if let Some(token) = crate::token::earthdata_token() {
                let auth = format!("Authorization: Bearer {}", token);
                let extra = [VERDICT_RANGE[0], VERDICT_RANGE[1], "-H", auth.as_str()];
                if let Some(retried) = retry_transient(|| get_once(url, &extra, timeout, exit)) {
                    return Some(retried);
                }
            }
        }
    }
    result
}

fn proton_interfaces() -> Vec<String> {
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("proton") {
                out.push(name);
            }
        }
    }
    out.sort();
    out
}

fn socks_bind(conf: &str) -> Option<String> {
    conf.lines()
        .find_map(|line| line.trim().strip_prefix("BindAddress"))
        .map(|rest| rest.trim_start_matches([' ', '=']).trim().to_string())
        .filter(|addr| !addr.is_empty())
}

fn socks_addr() -> Option<String> {
    let runtime = match std::env::var("XDG_RUNTIME_DIR") {
        Ok(dir) => dir,
        Err(_) => "/tmp".to_string(),
    };
    let mut dirs = vec![std::path::Path::new(&runtime).join("proton-wg")];
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(std::path::Path::new(&home).join(".config/proton-wg"));
    }
    dirs.into_iter().find_map(|dir| socks_addr_in(&dir))
}

fn socks_addr_in(dir: &std::path::Path) -> Option<String> {
    let conf = std::fs::read_to_string(dir.join("active.conf")).ok()?;
    let addr = socks_bind(&conf)?;
    let sock: SocketAddr = addr.parse().ok()?;
    TcpStream::connect_timeout(&sock, Duration::from_millis(500)).ok()?;
    Some(addr)
}

fn proton_socks() -> Option<String> {
    socks_addr().map(|addr| format!("socks5h://{addr}"))
}

pub(crate) fn socks_proxy() -> Option<String> {
    socks_addr().map(|addr| format!("socks5://{addr}"))
}

fn exits() -> Vec<Exit> {
    let mut out = vec![Exit::Direct];
    for name in proton_interfaces() {
        out.push(Exit::Iface(name));
    }
    if let Some(socks) = proton_socks() {
        out.push(Exit::Socks(socks));
    }
    out
}

fn is_block(status: Option<i32>) -> bool {
    matches!(status, None | Some(0) | Some(403) | Some(429))
}

fn first_answer<F: Fn(&Exit) -> Option<Fetch>>(ladder: &[Exit], attempt: F) -> Option<Fetch> {
    let mut last = None;
    for exit in ladder {
        if let Some(f) = attempt(exit) {
            if !is_block(f.status) {
                return Some(f);
            }
            last = Some(f);
        }
    }
    last
}

fn url_host(url: &str) -> String {
    let rest = url.split("://").nth(1).unwrap_or(url);
    rest.split(['/', '?', '#']).next().unwrap_or("").to_string()
}

fn is_edl_login_page(f: &Fetch) -> bool {
    f.body.contains("urs.earthdata.nasa.gov")
        || f.body.contains("Earthdata Login")
        || f.body.contains("oauth/authorize")
}

struct RateGate {
    next_allowed: HashMap<String, Instant>,
    blocked_until: HashMap<String, Instant>,
    interval: Duration,
}

const ARXIV_MIN_INTERVAL_MS: u64 = 3000;

impl RateGate {
    fn new() -> Self {
        let ms = std::env::var("ARCHIVE_SEARCH_MIN_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(DEFAULT_MIN_INTERVAL_MS);
        RateGate {
            next_allowed: HashMap::new(),
            blocked_until: HashMap::new(),
            interval: Duration::from_millis(ms),
        }
    }

    fn interval_for(&self, key: &str) -> Duration {
        let host = key.split('|').next().unwrap_or(key);
        if host == "export.arxiv.org" || host == "arxiv.org" {
            let ms = (self.interval.as_millis() as u64).max(ARXIV_MIN_INTERVAL_MS);
            Duration::from_millis(ms)
        } else {
            self.interval
        }
    }

    fn wait(&mut self, host: &str) {
        let now = Instant::now();
        if let Some(until) = self.next_allowed.get(host).copied() {
            if until > now {
                std::thread::sleep(until - now);
            }
        }
        self.next_allowed
            .insert(host.to_string(), Instant::now() + self.interval_for(host));
    }

    fn blocked(&self, host: &str) -> bool {
        matches!(self.blocked_until.get(host), Some(until) if *until > Instant::now())
    }

    fn back_off(&mut self, host: &str, secs: u64) {
        self.blocked_until
            .insert(host.to_string(), Instant::now() + Duration::from_secs(secs));
    }
}

static RATE_GATE: OnceLock<Mutex<RateGate>> = OnceLock::new();

pub(crate) fn get(url: &str, extra: &[&str], timeout: &str) -> Option<Fetch> {
    let host = url_host(url);
    let gate = RATE_GATE.get_or_init(|| Mutex::new(RateGate::new()));
    let ladder = exits();
    let result = first_answer(&ladder, |exit| {
        let key = format!("{}|{}", host, exit.label());
        if let Ok(g) = gate.lock() {
            if g.blocked(&key) {
                return None;
            }
        }
        if let Ok(mut g) = gate.lock() {
            g.wait(&key);
        }
        let result = get_once(url, extra, timeout, exit);
        if let Some(f) = &result {
            if f.status == Some(429) {
                if let Ok(mut g) = gate.lock() {
                    g.back_off(&key, f.retry_after.unwrap_or(DEFAULT_RETRY_AFTER_SECS));
                }
            }
        }
        result
    });
    if let Some(f) = &result {
        if crate::token::is_earthdata_host(&host)
            && (crate::token::is_unauthorized(f.status) || is_edl_login_page(f))
        {
            if let Some(token) = crate::token::earthdata_token() {
                let auth = format!("Authorization: Bearer {}", token);
                let mut merged: Vec<&str> = extra.to_vec();
                merged.push("-H");
                merged.push(auth.as_str());
                if let Ok(mut g) = gate.lock() {
                    g.wait(&host);
                }
                let retried = first_answer(&ladder, |exit| get_once(url, &merged, timeout, exit));
                if let Some(retried) = retried {
                    return Some(retried);
                }
            }
        }
    }
    result
}

fn post_args(body: &str, headers: &[&str]) -> Vec<String> {
    let mut extra: Vec<String> = vec![
        "-X".to_string(),
        "POST".to_string(),
        "-H".to_string(),
        "Content-Type: application/json".to_string(),
    ];
    for h in headers {
        extra.push("-H".to_string());
        extra.push((*h).to_string());
    }
    extra.push("--data".to_string());
    extra.push(body.to_string());
    extra
}

pub(crate) fn post(url: &str, body: &str, headers: &[&str], timeout: &str) -> Option<Fetch> {
    let extra = post_args(body, headers);
    let extra_refs: Vec<&str> = extra.iter().map(String::as_str).collect();
    get(url, &extra_refs, timeout)
}

pub fn urlencode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

pub(crate) fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
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

pub(crate) fn today() -> String {
    let secs = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(neg) => -(neg.duration().as_secs() as i64),
    };
    let (y, m, d) = civil_from_days(secs.div_euclid(86400));
    format!("{:04}-{:02}-{:02}", y, m, d)
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn flatten(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

const CONTENT_CAP: usize = 4000;

fn cap_content(s: &str) -> String {
    let flat = flatten(s);
    if flat.chars().count() <= CONTENT_CAP {
        return flat;
    }
    let mut out: String = flat.chars().take(CONTENT_CAP).collect();
    out.push_str(" …");
    out
}

fn stage(lines: &mut Vec<String>, n: u8, name: &str, url: &str, r: Option<Fetch>) {
    match r {
        Some(f) => stage_result(lines, n, name, url, f),
        None => lines.push(format!("  stage {} {}: pending — no response", n, name)),
    }
}

fn stage_result(lines: &mut Vec<String>, n: u8, name: &str, url: &str, f: Fetch) {
    if is_found(&f) {
        lines.push(format!(
            "  stage {} {}: HTTP {} ({} bytes) — found",
            n,
            name,
            f.status_text(),
            f.raw.len()
        ));
        lines.push(format!("url {}", url));
    } else {
        lines.push(format!(
            "  stage {} {}: HTTP {} ({} bytes) — absent",
            n,
            name,
            f.status_text(),
            f.raw.len()
        ));
    }
}

fn cdx_indices(header: &[Json]) -> (usize, usize, usize) {
    let mut ts = 0usize;
    let mut original = 1usize;
    let mut status = 2usize;
    for (i, cell) in header.iter().enumerate() {
        match cell.as_str() {
            Some("timestamp") => ts = i,
            Some("original") => original = i,
            Some("statuscode") => status = i,
            _ => {}
        }
    }
    (ts, original, status)
}

fn cdx_is_header(header: &[Json]) -> bool {
    header
        .iter()
        .any(|c| matches!(c.as_str(), Some("timestamp") | Some("original")))
}

fn first_snapshot(body: &str) -> Option<String> {
    let v = json::parse(body)?;
    let rows = v.as_arr()?;
    let header = rows.first()?.as_arr()?;
    let (ti, oi, _) = cdx_indices(header);
    let data = if cdx_is_header(header) {
        rows.get(1)?
    } else {
        rows.first()?
    };
    let cells = data.as_arr()?;
    let ts = cells.get(ti)?.as_str()?;
    let original = cells.get(oi)?.as_str()?;
    Some(format!("https://web.archive.org/web/{}/{}", ts, original))
}

fn cdx_snapshot_lines(v: &Json) -> Vec<String> {
    let mut out = Vec::new();
    let Some(rows) = v.as_arr() else {
        return out;
    };
    let header = rows.first().and_then(|r| r.as_arr());
    let (ti, oi, si) = match header {
        Some(h) => cdx_indices(h),
        None => (0, 1, 2),
    };
    let start = if header.is_some_and(cdx_is_header) {
        1
    } else {
        0
    };
    for row in rows.iter().skip(start) {
        let Some(cells) = row.as_arr() else {
            continue;
        };
        let ts = cells.get(ti).and_then(|c| c.as_str()).unwrap_or("");
        let original = cells.get(oi).and_then(|c| c.as_str()).unwrap_or("");
        let code = cells.get(si).and_then(|c| c.as_str()).unwrap_or("");
        if !ts.is_empty() && !original.is_empty() {
            out.push(format!(
                "url https://web.archive.org/web/{}/{} (status {})",
                ts, original, code
            ));
        }
    }
    out
}

pub fn verdict_lines(url: &str) -> Vec<String> {
    if let Some(repo) = crate::find_repo_root() {
        crate::token::set_secrets(crate::secrets::load_env(&repo));
    }
    let mut lines = Vec::new();
    lines.push(format!("verdict {} — three-stage ladder", url));
    let direct = verdict_probe(url, "30", &Exit::Direct);
    let direct_blocked = matches!(&direct, Some(f) if is_block(f.status));
    let direct_found = matches!(&direct, Some(f) if is_found(f));
    stage(&mut lines, 1, "direct", url, direct);
    if direct_found {
        lines.push("  stage 2/3 skipped — stage 1 answered".to_string());
        lines.push(format!("measurement {}", today()));
        return lines;
    }
    let proxies: Vec<Exit> = exits()
        .into_iter()
        .filter(|exit| !matches!(exit, Exit::Direct))
        .collect();
    let mut proton_found = false;
    if proxies.is_empty() {
        lines.push("  stage 2 proton: absent — no proton transport is up".to_string());
    } else {
        for exit in &proxies {
            let label = exit.label();
            match verdict_probe(url, "30", exit) {
                Some(f) => {
                    if is_found(&f) {
                        proton_found = true;
                    }
                    stage_result(&mut lines, 2, &label, url, f);
                }
                None => lines.push(format!("  stage 2 {}: pending — no response", label)),
            }
        }
    }
    let cdx = format!(
        "https://web.archive.org/cdx/search/cdx?url={}&output=json&limit=1",
        urlencode(url)
    );
    match get(&cdx, &[], "40") {
        Some(f) if is_transient(&f) => {
            lines.push("  stage 3 wayback: pending — no response".to_string())
        }
        Some(f) => match first_snapshot(&f.body) {
            Some(snapshot) => {
                lines.push(format!(
                    "  stage 3 wayback: HTTP {} — snapshot {}",
                    f.status_text(),
                    snapshot
                ));
                lines.push(format!("url {}", snapshot));
            }
            None => lines.push(format!(
                "  stage 3 wayback: HTTP {} — the CDX register carries no snapshot",
                f.status_text()
            )),
        },
        None => lines.push("  stage 3 wayback: pending — no response".to_string()),
    }
    if direct_blocked && !proton_found {
        lines.push(format!(
            "hint: the direct route is blocked — `bin/proton-wg.sh suggest {}` names a country exit (operator consent)",
            url_host(url)
        ));
    }
    lines.push(format!("measurement {}", today()));
    lines
}

struct AtomEntry {
    id: String,
    title: String,
    authors: Option<String>,
    published: Option<String>,
    pdf: Option<String>,
    summary: Option<String>,
}

fn extract_between<'a>(s: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = s.find(open)? + open.len();
    let rest = &s[start..];
    let end = rest.find(close)?;
    Some(&rest[..end])
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        if in_tag {
            if c == '>' {
                in_tag = false;
            }
            continue;
        }
        if c == '<' {
            in_tag = true;
            continue;
        }
        out.push(c);
    }
    out
}

fn pdf_link(entry: &str) -> Option<String> {
    let pos = entry.find("title=\"pdf\"")?;
    let before = &entry[..pos];
    let h = before.rfind("href=\"")? + 6;
    let after = &before[h..];
    let end = after.find('"')?;
    Some(after[..end].to_string())
}

fn field_text(s: &str, open: &str, close: &str) -> Option<String> {
    extract_between(s, open, close)
        .map(|t| flatten(&strip_tags(t)))
        .filter(|t| !t.is_empty())
}

fn parse_atom_entries(xml: &str) -> Vec<AtomEntry> {
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(open) = rest.find("<entry") {
        let close = match rest[open..].find("</entry>") {
            Some(c) => open + c,
            None => break,
        };
        let entry = &rest[open..close];
        let (Some(id), Some(title)) = (
            field_text(entry, "<id>", "</id>"),
            field_text(entry, "<title>", "</title>"),
        ) else {
            rest = &rest[close..];
            continue;
        };
        let mut names = Vec::new();
        let mut r = entry;
        while let Some(n) = extract_between(r, "<name>", "</name>") {
            let name = flatten(&strip_tags(n));
            if !name.is_empty() {
                names.push(name);
            }
            let off = r.find("<name>").unwrap() + 6;
            r = &r[off..];
        }
        out.push(AtomEntry {
            id,
            title,
            authors: if names.is_empty() {
                None
            } else {
                Some(names.join("; "))
            },
            published: field_text(entry, "<published>", "</published>"),
            pdf: pdf_link(entry).filter(|t| !t.is_empty()),
            summary: field_text(entry, "<summary>", "</summary>"),
        });
        rest = &rest[close..];
    }
    out
}

const ARXIV_QUERY_PREFIXES: &[&str] = &[
    "all:", "ti:", "au:", "abs:", "cat:", "co:", "jr:", "rn:", "id:",
];

fn arxiv_query(text: &str) -> String {
    if let Some((prefix, value)) = text.split_once(':') {
        if ARXIV_QUERY_PREFIXES.contains(&format!("{}:", prefix).as_str()) {
            return format!("{}:{}", prefix, urlencode(value));
        }
    }
    format!("all:{}", urlencode(text))
}

fn arxiv_url(text: &str, refine: &[(String, String)], max: usize, start: usize) -> String {
    let mut url = format!(
        "https://export.arxiv.org/api/query?search_query={}&start={}&max_results={}",
        arxiv_query(text),
        start,
        arxiv_window(max)
    );
    for (key, value) in refine {
        url.push('&');
        url.push_str(key);
        url.push('=');
        url.push_str(&urlencode(value));
    }
    url
}

fn arxiv_html_lines(query: &str, max: usize) -> Vec<String> {
    let size = max.clamp(1, 50);
    let url = format!(
        "https://arxiv.org/search/?query={}&searchtype=all&size={}",
        urlencode(query),
        size
    );
    let out = Command::new("curl")
        .args([
            "-sL",
            "--compressed",
            "--max-time",
            "30",
            "-A",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/131 Safari/537.36",
            "-H",
            "Accept: text/html",
            "-o",
            "-",
            "-w",
            "\n%{http_code}",
        ])
        .arg(&url)
        .output()
        .ok();
    let Some(out) = out else {
        return Vec::new();
    };
    let stdout = out.stdout;
    let Some(idx) = stdout.iter().rposition(|&b| b == b'\n') else {
        return Vec::new();
    };
    let status = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .parse::<i32>()
        .ok();
    if status != Some(200) {
        return Vec::new();
    }
    parse_arxiv_html(&String::from_utf8_lossy(&stdout[..idx]), max)
}

fn parse_arxiv_html(body: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(pos) = rest.find("<li class=\"arxiv-result\"") {
        rest = &rest[pos..];
        let end = rest.find("</li>").unwrap_or(rest.len());
        let block = &rest[..end];
        let url = match html_attr_after(block, "class=\"list-title") {
            Some(url) => url.to_string(),
            None => String::new(),
        };
        if !url.is_empty() {
            let title = html_text_after(block, "class=\"title is-5", "</p>");
            let authors = html_text_after(block, "class=\"authors\"", "</p>");
            let authors = authors
                .strip_prefix("Authors:")
                .unwrap_or(&authors)
                .trim()
                .to_string();
            let mut line = format!("url {url}\ttitle: {title}");
            if !authors.is_empty() {
                line.push_str(&format!("\tauthors: {authors}"));
            }
            line.push_str("\tvia: arxiv-html (api 429)");
            out.push(line);
            if out.len() >= max {
                break;
            }
        }
        rest = &rest[end..];
    }
    out
}

pub fn arxiv_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(query, &["sortBy", "sortOrder"]);
    let headers = [
        "-H",
        "User-Agent: omegaflow-archive-search (https://github.com/omegaflow/omegaflow)",
        "-H",
        "Accept: application/atom+xml",
    ];
    let window = arxiv_window(max);
    let (mut lines, stop) = crate::paged::follow_pages(
        crate::paged::DEFAULT_PAGE_BUDGET,
        |index| {
            let url = arxiv_url(&text, &refine, max, index * window);
            match get_retrying(&url, &headers, "40") {
                Some(f) if f.status == Some(200) => {
                    let entries = parse_atom_entries(&f.body);
                    let out: Vec<String> = entries
                        .iter()
                        .map(|e| {
                            let mut line = format!("url {}\ttitle: {}", e.id, e.title);
                            if let Some(a) = &e.authors {
                                line.push_str(&format!("\tauthors: {}", a));
                            }
                            if let Some(p) = &e.published {
                                line.push_str(&format!("\tpublished: {}", p));
                            }
                            if let Some(p) = &e.pdf {
                                line.push_str(&format!("\tpdf: {}", p));
                            }
                            if let Some(s) = &e.summary {
                                line.push_str(&format!("\tabstract: {}", s));
                            }
                            line
                        })
                        .collect();
                    let has_more = out.len() >= window;
                    (out, has_more)
                }
                Some(f) if f.status == Some(406) => (
                    vec![format!(
                        "pending — the arXiv /api/query edge answers HTTP 406 to every window (measured 2026-09-27: max_results=1 and no max_results both 406); the live route is `--arxiv-oai` (OAI-PMH)"
                    )],
                    false,
                ),
                Some(f) if f.status == Some(429) => {
                    let out = arxiv_html_lines(&text, max);
                    if out.is_empty() {
                        (
                            vec![format!(
                                "pending — arxiv HTTP 429 (edge rate limit; {}, the block outlasted the {}-second retry ladder) and the HTML search also carries nothing",
                                match f.retry_after {
                                    Some(s) => format!("Retry-After: {}s", s),
                                    None => "no Retry-After header".to_string(),
                                },
                                RATE_LIMIT_BACKOFF_SECS * (RATE_LIMIT_ATTEMPTS as u64 - 1)
                            )],
                            false,
                        )
                    } else {
                        (out, false)
                    }
                }
                Some(f) => (
                    vec![format!("pending — arxiv HTTP {}", f.status_text())],
                    false,
                ),
                None => (vec!["pending — no network".to_string()], false),
            }
        },
    );
    if lines.is_empty() {
        vec![format!(
            "absent — the arxiv register carries no entry: {}",
            query
        )]
    } else {
        lines.push(format!("end: {}", stop.label()));
        lines
    }
}

fn doc_title(doc: &Json) -> &str {
    doc.get("title")
        .and_then(|t| t.as_arr())
        .and_then(|a| a.first())
        .and_then(|s| s.as_str())
        .unwrap_or("")
}

fn ads_url(text: &str, refine: &[(String, String)], max: usize, start: usize) -> String {
    let fl = crate::refine::value_of(refine, "fl").unwrap_or("title,bibcode,author,year,abstract");
    let mut url = format!(
        "https://api.adsabs.harvard.edu/v1/search/query?q={}&fl={}&rows={}",
        urlencode(text),
        urlencode(fl),
        max
    );
    if start > 0 {
        url.push_str("&start=");
        url.push_str(&start.to_string());
    }
    for (key, value) in refine {
        if key == "fl" {
            continue;
        }
        url.push('&');
        url.push_str(key);
        url.push('=');
        url.push_str(&urlencode(value));
    }
    url
}

fn ads_authors(doc: &Json) -> Option<String> {
    let arr = doc.get("author").and_then(|a| a.as_arr())?;
    let mut names: Vec<String> = arr
        .iter()
        .filter_map(|entry| entry.as_str())
        .map(|s| s.to_string())
        .collect();
    if names.is_empty() {
        return None;
    }
    if names.len() > 8 {
        names.truncate(8);
        Some(format!("{}, et al.", names.join(", ")))
    } else {
        Some(names.join(", "))
    }
}

fn ads_year(doc: &Json) -> Option<String> {
    let year = doc.get("year").and_then(|y| y.as_scalar_string())?;
    if year.is_empty() { None } else { Some(year) }
}

fn ads_abstract(doc: &Json) -> Option<String> {
    let raw = doc.get("abstract").and_then(|a| a.as_str())?;
    if raw.is_empty() {
        return None;
    }
    Some(truncate_to_chars(raw, 1200))
}

fn ads_doc_line(doc: &Json) -> Option<String> {
    let bib = doc.get("bibcode").and_then(|b| b.as_str()).unwrap_or("");
    if bib.is_empty() {
        return None;
    }
    let mut line = format!(
        "url https://ui.adsabs.harvard.edu/abs/{}\ttitle: {}",
        bib,
        doc_title(doc)
    );
    if let Some(authors) = ads_authors(doc) {
        line.push_str(&format!("\tauthors: {}", authors));
    }
    if let Some(year) = ads_year(doc) {
        line.push_str(&format!("\tyear: {}", year));
    }
    if let Some(abstract_text) = ads_abstract(doc) {
        line.push_str(&format!("\tabstract: {}", abstract_text));
    }
    Some(line)
}

pub fn ads_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — NASA_ADS_TOKEN absent from .secrets.local/.env".to_string()];
    }
    let (text, refine) = crate::refine::split_refine(query, &["fl", "fq", "sort"]);
    let auth = format!("Authorization: Bearer {}", token);
    let (mut lines, stop) =
        crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |index| {
            let start = index * max;
            let url = ads_url(&text, &refine, max, start);
            match get(&url, &["-H", auth.as_str()], "40") {
                Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                    Some(v) => {
                        let mut out = Vec::new();
                        if let Some(docs) = v
                            .get("response")
                            .and_then(|r| r.get("docs"))
                            .and_then(|d| d.as_arr())
                        {
                            for doc in docs {
                                if let Some(line) = ads_doc_line(doc) {
                                    out.push(line);
                                }
                            }
                        }
                        let num_found = v
                            .get("response")
                            .and_then(|r| r.get("numFound"))
                            .and_then(|n| n.as_scalar_string())
                            .and_then(|s| s.parse::<usize>().ok());
                        let has_more = match num_found {
                            Some(n) => start + out.len() < n,
                            None => out.len() >= max,
                        };
                        (out, has_more)
                    }
                    None => (
                        vec!["pending — the ADS response carries no JSON".to_string()],
                        false,
                    ),
                },
                Some(f) => (
                    vec![format!("pending — ADS HTTP {}", f.status_text())],
                    false,
                ),
                None => (vec!["pending — no network".to_string()], false),
            }
        });
    if lines.is_empty() {
        vec![format!(
            "absent — the ADS register carries no entry: {}",
            query
        )]
    } else {
        lines.push(format!("end: {}", stop.label()));
        lines
    }
}

fn ntrs_id_form(query: &str) -> bool {
    let t = query.trim();
    t.len() >= 6 && t.chars().all(|c| c.is_ascii_digit())
}

const NTRS_ABSTRACT_MAX: usize = 1200;

fn ntrs_abstract_text(text: &str) -> String {
    text.chars().take(NTRS_ABSTRACT_MAX).collect()
}

fn ntrs_published(doc: &Json) -> Option<String> {
    for key in ["publicationDate", "distributionDate", "created"] {
        if let Some(value) = doc.get(key).and_then(|d| d.as_str()) {
            let value = value.trim();
            if !value.is_empty() {
                return Some(value.chars().take(10).collect());
            }
        }
    }
    None
}

fn ntrs_authors(doc: &Json) -> Option<String> {
    let mut names: Vec<String> = Vec::new();
    if let Some(arr) = doc.get("authorAffiliations").and_then(|a| a.as_arr()) {
        for entry in arr {
            if let Some(name) = entry.as_str() {
                let name = name.trim();
                if !name.is_empty() {
                    names.push(name.to_string());
                }
            }
        }
    }
    if names.is_empty() {
        if let Some(arr) = doc.get("authors").and_then(|a| a.as_arr()) {
            for entry in arr {
                if let Some(name) = entry.get("name").and_then(|n| n.as_str()) {
                    let name = name.trim();
                    if !name.is_empty() {
                        names.push(name.to_string());
                    }
                }
            }
        }
    }
    if names.is_empty() {
        return None;
    }
    let et_al = names.len() > 8;
    names.truncate(8);
    let mut joined = names.join(", ");
    if et_al {
        joined.push_str(", et al.");
    }
    Some(joined)
}

fn ntrs_doc_line(doc: &Json, rid: &str) -> String {
    let title = doc.get("title").and_then(|t| t.as_str()).unwrap_or("");
    let mut line = format!(
        "url https://ntrs.nasa.gov/citations/{}\ttitle: {}",
        rid, title
    );
    if let Some(abstract_text) = doc.get("abstract").and_then(|a| a.as_str()) {
        let abstract_text = abstract_text.trim();
        if !abstract_text.is_empty() {
            line.push_str("\tabstract: ");
            line.push_str(&ntrs_abstract_text(abstract_text));
        }
    }
    if let Some(published) = ntrs_published(doc) {
        line.push_str(&format!("\tpublished: {}", published));
    }
    if let Some(authors) = ntrs_authors(doc) {
        line.push_str(&format!("\tauthors: {}", authors));
    }
    line
}

pub fn ntrs_lines(query: &str, max: usize) -> Vec<String> {
    if ntrs_id_form(query) {
        let id = query.trim();
        let url = format!("https://ntrs.nasa.gov/api/citations/{}", id);
        return match get(&url, &[], "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let rid = match v.get("id").and_then(|i| i.as_scalar_string()) {
                        Some(s) => s,
                        None => id.to_string(),
                    };
                    vec![ntrs_doc_line(&v, &rid)]
                }
                None => vec!["pending — the NTRS response carries no JSON".to_string()],
            },
            Some(f) if f.status == Some(404) => vec![format!(
                "absent — the NTRS register carries no entry: {}",
                query
            )],
            Some(f) => vec![format!("pending — NTRS HTTP {}", f.status_text())],
            None => vec!["pending — no network".to_string()],
        };
    }
    let url = format!(
        "https://ntrs.nasa.gov/api/citations/search?q={}&page.size={}",
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = Vec::new();
                if let Some(results) = v.get("results").and_then(|r| r.as_arr()) {
                    for doc in results {
                        if let Some(id) = doc.get("id").and_then(|i| i.as_scalar_string()) {
                            if id.is_empty() {
                                continue;
                            }
                            out.push(ntrs_doc_line(doc, &id));
                        }
                    }
                }
                if out.is_empty() {
                    vec![format!(
                        "absent — the NTRS register carries no entry: {}",
                        query
                    )]
                } else {
                    out
                }
            }
            None => vec!["pending — the NTRS response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — NTRS HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn wayback_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://web.archive.org/cdx/search/cdx?url={}&output=json&limit={}&fl=timestamp,original,statuscode",
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let out = cdx_snapshot_lines(&v);
                if out.is_empty() {
                    vec![format!(
                        "absent — the CDX register carries no snapshot: {}",
                        query
                    )]
                } else {
                    out
                }
            }
            None => vec!["pending — the CDX response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — CDX HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

pub const CC_DEFAULT_INDEX: &str = "CC-MAIN-2026-39";

const CC_COLLINFO_URL: &str = "https://index.commoncrawl.org/collinfo.json";

static CC_LATEST_INDEX: OnceLock<Option<String>> = OnceLock::new();

fn cc_index(query: &str) -> Option<String> {
    query.split_whitespace().find_map(|token| {
        let (name, value) = token.split_once('=')?;
        if name == "index" && !value.is_empty() {
            Some(value.to_string())
        } else {
            None
        }
    })
}

fn cc_latest_index() -> Option<String> {
    CC_LATEST_INDEX
        .get_or_init(|| {
            let body = match get(CC_COLLINFO_URL, &[], "20") {
                Some(f) if f.status == Some(200) => f.body,
                Some(f) => {
                    eprintln!(
                        "common crawl: collinfo unavailable (HTTP {}), using fallback index {}",
                        f.status_text(),
                        CC_DEFAULT_INDEX
                    );
                    return None;
                }
                None => {
                    eprintln!(
                        "common crawl: collinfo unavailable (no network), using fallback index {}",
                        CC_DEFAULT_INDEX
                    );
                    return None;
                }
            };
            let latest = json::parse(&body)
                .as_ref()
                .and_then(|v| v.as_arr())
                .and_then(|arr| arr.first())
                .and_then(|first| first.get("id"))
                .and_then(|id| id.as_str())
                .filter(|id| !id.is_empty())
                .map(str::to_string);
            if latest.is_none() {
                eprintln!(
                    "common crawl: collinfo carried no index id, using fallback index {}",
                    CC_DEFAULT_INDEX
                );
            }
            latest
        })
        .clone()
}

fn cc_pattern(target: &str) -> String {
    if target.contains('*') || target.contains('/') || target.contains(char::is_whitespace) {
        target.to_string()
    } else {
        format!("*.{target}")
    }
}

fn cc_target(query: &str) -> String {
    let target = query
        .split_whitespace()
        .filter(|token| !token.starts_with("index="))
        .map(|token| match token.strip_prefix("url=") {
            Some(target) => target,
            None => token,
        })
        .collect::<Vec<_>>()
        .join(" ");
    cc_pattern(&target)
}

pub fn log_dir() -> Option<std::path::PathBuf> {
    if let Ok(d) = std::env::var("OMEGAFLOW_TMP") {
        if !d.trim().is_empty() {
            return Some(std::path::PathBuf::from(d));
        }
    }
    if let Ok(w) = std::env::var("OMEGAFLOW_WORK") {
        return Some(std::path::PathBuf::from(w).join("scratch"));
    }
    std::env::var("HOME")
        .ok()
        .map(|h| std::path::PathBuf::from(h).join("projects/omegaflow-work/scratch"))
}

fn cc_record_lines(body: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    for raw in body.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let Some(v) = json::parse(line) else {
            continue;
        };
        let Some(url) = v.get("url").and_then(|u| u.as_str()) else {
            continue;
        };
        if url.is_empty() {
            continue;
        }
        let mut line = format!("url {url}");
        if let Some(ts) = v.get("timestamp").and_then(|t| t.as_scalar_string()) {
            line.push_str(&format!("\ttimestamp: {ts}"));
        }
        if let Some(status) = v.get("status").and_then(|s| s.as_scalar_string()) {
            line.push_str(&format!("\tstatus: {status}"));
        }
        if let Some(mime) = v.get("mime").and_then(|m| m.as_str()) {
            line.push_str(&format!("\tmime: {mime}"));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

pub fn cc_lines(query: &str, max: usize) -> Vec<String> {
    let target = cc_target(query);
    let index = match cc_index(query) {
        Some(index) => index,
        None => match cc_latest_index() {
            Some(idx) => idx,
            None => CC_DEFAULT_INDEX.to_string(),
        },
    };
    let url = format!(
        "https://index.commoncrawl.org/{}-index?url={}&output=json&limit={}",
        index,
        urlencode(&target),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let out = cc_record_lines(&f.body, max);
            if out.is_empty() {
                vec![format!(
                    "absent — the Common Crawl index carries no record: {}",
                    target
                )]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — Common Crawl HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn wayback_available_lines(url: &str) -> Vec<String> {
    let endpoint = format!(
        "https://archive.org/wayback/available?url={}",
        urlencode(url)
    );
    match get(&endpoint, &[], "40") {
        Some(f) if f.status == Some(200) => availability_lines_from(&f.body, url),
        Some(f) => vec![format!("pending — availability HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn availability_lines_from(body: &str, url: &str) -> Vec<String> {
    match json::parse(body) {
        Some(v) => {
            let closest = v.get("archived_snapshots").and_then(|s| s.get("closest"));
            let (snapshot, ts, status) = match closest {
                Some(c) => (
                    c.get("url").and_then(|u| u.as_str()).unwrap_or(""),
                    c.get("timestamp").and_then(|t| t.as_str()).unwrap_or(""),
                    c.get("status").and_then(|s| s.as_str()).unwrap_or(""),
                ),
                None => ("", "", ""),
            };
            if snapshot.is_empty() {
                vec![format!(
                    "absent — the availability register carries no snapshot: {}",
                    url
                )]
            } else {
                vec![format!(
                    "url {}\ttimestamp: {}\tstatus: {}",
                    snapshot, ts, status
                )]
            }
        }
        None => vec!["pending — the availability response carries no JSON".to_string()],
    }
}

pub fn wayback_timemap_lines(url: &str) -> Vec<String> {
    let endpoint = format!("https://web.archive.org/web/timemap/json/{}", url);
    match get(&endpoint, &[], "120") {
        Some(f) if f.status == Some(200) && f.complete => timemap_lines_from(&f.body, url),
        Some(f) if f.status == Some(200) => {
            vec!["pending — the timemap download is incomplete (truncated)".to_string()]
        }
        Some(f) => vec![format!("pending — timemap HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn timemap_lines_from(body: &str, url: &str) -> Vec<String> {
    match json::parse(body) {
        Some(v) => {
            let out = cdx_snapshot_lines(&v);
            if out.is_empty() {
                vec![format!(
                    "absent — the timemap register carries no snapshot: {}",
                    url
                )]
            } else {
                out
            }
        }
        None => vec!["pending — the timemap response carries no JSON".to_string()],
    }
}

fn crossref_url(
    text: &str,
    refine: &[(String, String)],
    max: usize,
    cursor: Option<&str>,
) -> String {
    let select = crate::refine::value_of(refine, "select")
        .unwrap_or("DOI,title,issued,published,author,is-referenced-by-count,abstract");
    let mut url = format!("https://api.crossref.org/works?rows={}", max);
    if !text.is_empty() {
        url.push_str("&query=");
        url.push_str(&urlencode(text));
    }
    url.push_str("&select=");
    url.push_str(&urlencode(select));
    for (key, value) in refine {
        if key == "select" {
            continue;
        }
        url.push('&');
        url.push_str(key);
        url.push('=');
        url.push_str(&urlencode(value));
    }
    if let Some(c) = cursor {
        url.push_str("&cursor=");
        url.push_str(&urlencode(c));
    }
    url
}

fn crossref_authors(item: &Json) -> Option<String> {
    let arr = item.get("author").and_then(|a| a.as_arr())?;
    let mut names: Vec<String> = Vec::new();
    for entry in arr {
        let given = entry.get("given").and_then(|g| g.as_str()).unwrap_or("");
        let family = entry.get("family").and_then(|f| f.as_str()).unwrap_or("");
        match (given.is_empty(), family.is_empty()) {
            (true, true) => continue,
            (true, false) => names.push(family.to_string()),
            (false, true) => names.push(given.to_string()),
            (false, false) => names.push(format!("{} {}", given, family)),
        }
    }
    if names.len() > 8 {
        names.truncate(8);
        Some(format!("{}, et al.", names.join(", ")))
    } else if names.is_empty() {
        None
    } else {
        Some(names.join(", "))
    }
}

fn crossref_abstract(item: &Json) -> Option<String> {
    let raw = item.get("abstract").and_then(|a| a.as_str())?;
    let mut stripped = String::new();
    let mut in_tag = false;
    for ch in raw.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => stripped.push(ch),
            _ => {}
        }
    }
    let collapsed: String = stripped.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        return None;
    }
    if collapsed.chars().count() > 1200 {
        let head: String = collapsed.chars().take(1200).collect();
        Some(format!("{}…", head))
    } else {
        Some(collapsed)
    }
}

fn crossref_year(item: &Json) -> Option<String> {
    for field in ["issued", "published", "published-online", "published-print"] {
        let Some(parts) = item
            .get(field)
            .and_then(|p| p.get("date-parts"))
            .and_then(|d| d.as_arr())
        else {
            continue;
        };
        let year = parts
            .first()
            .and_then(|p| p.as_arr())
            .and_then(|p| p.first())
            .and_then(|y| y.as_scalar_string());
        if let Some(year) = year {
            let year = year.trim();
            if !year.is_empty() {
                return Some(year.to_string());
            }
        }
    }
    None
}

fn crossref_line(item: &Json) -> Option<String> {
    let doi = item.get("DOI").and_then(|d| d.as_str()).unwrap_or("");
    if doi.is_empty() {
        return None;
    }
    let mut line = format!("url https://doi.org/{}\ttitle: {}", doi, doc_title(item));
    if let Some(year) = crossref_year(item) {
        line.push_str(&format!("\tyear: {}", year));
    }
    if let Some(authors) = crossref_authors(item) {
        line.push_str(&format!("\tauthors: {}", authors));
    }
    if let Some(cites) = item
        .get("is-referenced-by-count")
        .and_then(|c| c.as_scalar_string())
    {
        line.push_str(&format!("\tcites: {}", cites));
    }
    if let Some(abstract_) = crossref_abstract(item) {
        line.push_str(&format!("\tabstract: {}", abstract_));
    }
    Some(line)
}

pub fn crossref_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(query, &["filter", "sort", "order", "select"]);
    let mut cursor: Option<String> = None;
    let (mut lines, stop) = crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |_| {
        let url = crossref_url(&text, &refine, max, cursor.as_deref());
        match get(&url, &[], "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let mut out = Vec::new();
                    if let Some(items) = v
                        .get("message")
                        .and_then(|m| m.get("items"))
                        .and_then(|i| i.as_arr())
                    {
                        for item in items {
                            if let Some(line) = crossref_line(item) {
                                out.push(line);
                            }
                        }
                    }
                    let next = v
                        .get("message")
                        .and_then(|m| m.get("next-cursor"))
                        .and_then(|c| c.as_str())
                        .filter(|c| !c.is_empty())
                        .map(str::to_string);
                    let has_more = next.is_some();
                    cursor = next;
                    (out, has_more)
                }
                None => (
                    vec!["pending — the crossref response carries no JSON".to_string()],
                    false,
                ),
            },
            Some(f) => (
                vec![format!("pending — crossref HTTP {}", f.status_text())],
                false,
            ),
            None => (vec!["pending — no network".to_string()], false),
        }
    });
    if lines.is_empty() {
        vec![format!("absent — crossref carries no entry: {}", query)]
    } else {
        lines.push(format!("end: {}", stop.label()));
        lines
    }
}

pub fn wiki_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json&srlimit={}",
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = Vec::new();
                if let Some(hits) = v
                    .get("query")
                    .and_then(|q| q.get("search"))
                    .and_then(|s| s.as_arr())
                {
                    for hit in hits {
                        if let Some(line) = wiki_hit_line(hit) {
                            out.push(line);
                        }
                    }
                }
                if out.is_empty() {
                    vec![format!("absent — wikipedia carries no entry: {}", query)]
                } else {
                    out
                }
            }
            None => vec!["pending — the wikipedia response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — wikipedia HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

const WIKI_SNIPPET_CAP: usize = 400;

fn wiki_hit_line(hit: &Json) -> Option<String> {
    let title = hit.get("title").and_then(|t| t.as_str())?;
    if title.is_empty() {
        return None;
    }
    let mut line = format!(
        "url https://en.wikipedia.org/wiki/{}\ttitle: {}",
        title.replace(' ', "_"),
        title
    );
    if let Some(snippet) = hit.get("snippet").and_then(|s| s.as_str()) {
        let flat = flatten(&strip_tags(snippet));
        if !flat.is_empty() {
            let capped: String = flat.chars().take(WIKI_SNIPPET_CAP).collect();
            line.push_str(&format!("\tsnippet: {}", capped));
        }
    }
    Some(line)
}

fn duckduckgo_instant_related(topics: &[Json], out: &mut Vec<String>, max: usize) {
    for topic in topics {
        if out.len() >= max {
            return;
        }
        if let Some(first_url) = topic.get("FirstURL").and_then(|u| u.as_str()) {
            if !first_url.is_empty() {
                let text = topic.get("Text").and_then(|t| t.as_str()).unwrap_or("");
                out.push(format!("url {first_url}\t{text}"));
            }
        }
        if let Some(nested) = topic.get("Topics").and_then(|t| t.as_arr()) {
            duckduckgo_instant_related(nested, out, max);
        }
    }
}

fn duckduckgo_instant_entries(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(answer) = v.get("Answer").and_then(|a| a.as_str()) {
        if !answer.is_empty() {
            out.push(format!("answer: {answer}"));
        }
    }
    if let Some(abstract_text) = v.get("AbstractText").and_then(|a| a.as_str()) {
        if !abstract_text.is_empty() {
            let url = v.get("AbstractURL").and_then(|u| u.as_str()).unwrap_or("");
            out.push(format!("abstract: {abstract_text}\t{url}"));
        }
    }
    if let Some(topics) = v.get("RelatedTopics").and_then(|t| t.as_arr()) {
        duckduckgo_instant_related(topics, &mut out, max);
    }
    out
}

pub fn duckduckgo_instant_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
        urlencode(query)
    );
    match get(&url, &["-H", "User-Agent: omegaflow-archive-search"], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let out = duckduckgo_instant_entries(&v, max);
                if out.is_empty() {
                    vec![format!(
                        "absent — duckduckgo-instant carries no entry: {query}"
                    )]
                } else {
                    out
                }
            }
            None => {
                vec!["pending — the duckduckgo-instant response carries no JSON".to_string()]
            }
        },
        Some(f) => vec![format!(
            "pending — duckduckgo-instant HTTP {}",
            f.status_text()
        )],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn github_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://api.github.com/search/repositories?q={}&per_page={}",
        urlencode(query),
        max
    );
    let ua = "User-Agent: omegaflow-archive-search";
    let auth = format!("Authorization: Bearer {}", token);
    let mut extra: Vec<&str> = vec!["-H", ua];
    if !token.is_empty() {
        extra.push("-H");
        extra.push(auth.as_str());
    }
    match get(&url, &extra, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = Vec::new();
                if let Some(items) = v.get("items").and_then(|i| i.as_arr()) {
                    for item in items {
                        let full = item.get("full_name").and_then(|n| n.as_str()).unwrap_or("");
                        let html = item.get("html_url").and_then(|h| h.as_str()).unwrap_or("");
                        if !html.is_empty() {
                            out.push(format!("url {}\trepo: {}", html, full));
                        }
                    }
                }
                if out.is_empty() {
                    vec![format!("absent — github carries no entry: {}", query)]
                } else {
                    out
                }
            }
            None => vec!["pending — the github response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — github HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn crates_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://crates.io/api/v1/crates?q={}&per_page={}",
        urlencode(query),
        max
    );
    match get(&url, &["-H", "User-Agent: omegaflow-archive-search"], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = Vec::new();
                if let Some(crates) = v.get("crates").and_then(|c| c.as_arr()) {
                    for krate in crates {
                        let name = krate.get("name").and_then(|n| n.as_str()).unwrap_or("");
                        let version = krate
                            .get("max_version")
                            .and_then(|n| n.as_str())
                            .unwrap_or("");
                        if !name.is_empty() {
                            out.push(format!(
                                "url https://crates.io/crates/{}\tversion: {}",
                                name, version
                            ));
                        }
                    }
                }
                if out.is_empty() {
                    vec![format!("absent — crates.io carries no entry: {}", query)]
                } else {
                    out
                }
            }
            None => vec!["pending — the crates.io response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — crates.io HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn brave_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — BRAVE_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let url = format!(
        "https://api.search.brave.com/res/v1/web/search?q={}&count={}",
        urlencode(query),
        max
    );
    let auth = format!("X-Subscription-Token: {}", token);
    let extra = ["-H", auth.as_str(), "-H", "Accept: application/json"];
    match get(&url, &extra, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = Vec::new();
                if let Some(results) = v
                    .get("web")
                    .and_then(|w| w.get("results"))
                    .and_then(|r| r.as_arr())
                {
                    for r in results {
                        let title = r.get("title").and_then(|t| t.as_str()).unwrap_or("");
                        let link = r.get("url").and_then(|u| u.as_str()).unwrap_or("");
                        let desc = r
                            .get("description")
                            .and_then(|d| d.as_str())
                            .map(|d| flatten(&strip_tags(d)));
                        if link.is_empty() {
                            continue;
                        }
                        let mut line = format!("url {}\ttitle: {}", link, title);
                        if let Some(desc) = &desc {
                            if !desc.is_empty() {
                                line.push_str(&format!("\tdescription: {}", desc));
                            }
                        }
                        out.push(line);
                    }
                }
                if out.is_empty() {
                    vec![format!("absent — Brave carries no entry: {}", query)]
                } else {
                    out
                }
            }
            None => vec!["pending — the Brave response carries no JSON".to_string()],
        },
        Some(f) => vec![format!(
            "pending — Brave HTTP {} (the keyless path is --mwmbl)",
            f.status_text()
        )],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn mwmbl_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("https://mwmbl.org/api/v1/search/?s={}", urlencode(query));
    let headers = [
        "-H",
        "Accept: application/json",
        "-H",
        "User-Agent: omegaflow-archive-search",
    ];
    match get(&url, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = mwmbl_results(&v, max);
                if out.is_empty() {
                    out.push(format!("absent — Mwmbl carries no entry: {}", query));
                }
                out
            }
            None => vec!["pending — the Mwmbl response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — Mwmbl HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn mwmbl_text(v: Option<&Json>) -> String {
    let Some(parts) = v.and_then(|x| x.as_arr()) else {
        return String::new();
    };
    let mut out = String::new();
    for p in parts {
        if let Some(s) = p.get("value").and_then(|x| x.as_str()) {
            out.push_str(s);
        }
    }
    out
}

fn mwmbl_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(results) = v.as_arr() else {
        return out;
    };
    for r in results {
        let link = r.get("url").and_then(|u| u.as_str()).unwrap_or("");
        if link.is_empty() {
            continue;
        }
        let title = flatten(&mwmbl_text(r.get("title")));
        let extract = flatten(&mwmbl_text(r.get("extract")));
        let source = r.get("source").and_then(|s| s.as_str()).unwrap_or("");
        let mut line = format!("url {}\ttitle: {}", link, title);
        if !source.is_empty() {
            line.push_str(&format!("\tsource: {}", source));
        }
        if !extract.is_empty() {
            line.push_str(&format!("\tdescription: {}", extract));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

pub fn marginalia_lines(query: &str, max: usize) -> Vec<String> {
    let base = format!(
        "https://old-search.marginalia.nu/search?query={}",
        urlencode(query)
    );
    let jar = std::env::temp_dir().join(format!(
        "omegaflow-marginalia-{}.jar",
        std::process::id()
    ));
    let jar = jar.to_string_lossy().to_string();
    let first = match marginalia_curl(&base, &jar) {
        Some(r) => r,
        None => return vec!["pending — Marginalia: no answer (timeout or no network)".to_string()],
    };
    let (status, mut body) = first;
    if body.contains("Wait For A Moment") {
        let Some(sst) = marginalia_sst(&body) else {
            let _ = std::fs::remove_file(&jar);
            return vec!["pending — Marginalia: the wait page carries no sst token".to_string()];
        };
        std::thread::sleep(Duration::from_millis(1300));
        match marginalia_curl(&format!("{base}&{sst}"), &jar) {
            Some((_, second)) => body = second,
            None => {
                let _ = std::fs::remove_file(&jar);
                return vec!["pending — Marginalia: no answer after the wait".to_string()];
            }
        }
    }
    let _ = std::fs::remove_file(&jar);
    if body.contains("Wait For A Moment") {
        return vec!["pending — Marginalia: bot wait did not clear".to_string()];
    }
    if status != 200 {
        return vec![format!("pending — Marginalia HTTP {status}")];
    }
    let mut out = marginalia_html_results(&body, max);
    if out.is_empty() {
        out.push(format!("absent — Marginalia carries no entry: {}", query));
    }
    out
}

fn marginalia_curl(url: &str, jar: &str) -> Option<(i32, String)> {
    let out = Command::new("curl")
        .args([
            "-sL",
            "--compressed",
            "--max-time",
            "40",
            "-A",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/131 Safari/537.36",
            "-c",
            jar,
            "-b",
            jar,
            "-o",
            "-",
            "-w",
            "\n%{http_code}",
        ])
        .arg(url)
        .output()
        .ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let status = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .parse::<i32>()
        .ok()?;
    let body = String::from_utf8_lossy(&stdout[..idx]).to_string();
    Some((status, body))
}

fn marginalia_sst(body: &str) -> Option<String> {
    let pos = body.find("sst=")?;
    let rest = &body[pos..];
    let end = rest
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '=' || c == '-' || c == '_'))
        .unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

fn html_attr_after<'a>(card: &'a str, class: &str) -> Option<&'a str> {
    let pos = card.find(class)?;
    let rest = &card[pos..];
    let href = rest.find("href=\"")? + 6;
    let rest = &rest[href..];
    let end = rest.find('"')?;
    Some(&rest[..end])
}

fn html_text_after(card: &str, marker: &str, close: &str) -> String {
    let Some(pos) = card.find(marker) else {
        return String::new();
    };
    let rest = &card[pos..];
    let Some(tag_end) = rest.find('>') else {
        return String::new();
    };
    let rest = &rest[tag_end + 1..];
    let end = rest.find(close).unwrap_or(rest.len());
    flatten(&unescape_html(&strip_tags(&rest[..end])))
}

fn marginalia_html_results(body: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(pos) = rest.find("class=\"card search-result\"") {
        rest = &rest[pos..];
        let end = rest.find("</section>").unwrap_or(rest.len());
        let card = &rest[..end];
        if let Some(link) = html_attr_after(card, "class=\"url\"") {
            let url = if link.starts_with("http") {
                link.to_string()
            } else {
                format!("https://old-search.marginalia.nu{link}")
            };
            let title = html_text_after(card, "class=\"title\"", "</a>");
            let description = html_text_after(card, "<p class=\"description\">", "</p>");
            let mut line = format!("url {url}\ttitle: {title}");
            if !description.is_empty() {
                line.push_str(&format!("\tdescription: {description}"));
            }
            out.push(line);
            if out.len() >= max {
                break;
            }
        }
        rest = &rest[end..];
    }
    out
}

fn unescape_html(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&nbsp;", " ")
}

const SEARXNG_INSTANCES: &[&str] = &["https://search.lumy.live"];

fn searxng_bases() -> Vec<String> {
    match std::env::var("OMEGAFLOW_SEARXNG") {
        Ok(v) if !v.trim().is_empty() => v
            .split(',')
            .map(|s| s.trim().trim_end_matches('/').to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        _ => SEARXNG_INSTANCES.iter().map(|s| s.to_string()).collect(),
    }
}

pub fn searxng_lines(query: &str, max: usize) -> Vec<String> {
    let headers = [
        "-H",
        "Accept: application/json",
        "-H",
        "User-Agent: omegaflow-archive-search",
    ];
    let mut last = "pending — no SearXNG instance answered".to_string();
    let mut absent = false;
    for base in searxng_bases() {
        let url = format!("{}/search?q={}&format=json", base, urlencode(query));
        match get(&url, &headers, "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let out = searxng_results(&v, max);
                    if out.is_empty() {
                        last = format!("absent — {} carries no entry: {}", base, query);
                        absent = true;
                        continue;
                    }
                    return out;
                }
                None => {
                    last = format!("pending — {} carries no JSON", base);
                    continue;
                }
            },
            Some(f) => {
                last = format!("pending — {} HTTP {}", base, f.status_text());
                continue;
            }
            None => {
                last = format!("pending — no network to {}", base);
                continue;
            }
        }
    }
    if absent {
        last = format!("absent — SearXNG carries no entry: {}", query);
    }
    vec![last]
}

fn searxng_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for r in results {
        let link = r.get("url").and_then(|u| u.as_str()).unwrap_or("");
        if link.is_empty() {
            continue;
        }
        let title = flatten(r.get("title").and_then(|t| t.as_str()).unwrap_or(""));
        let content = flatten(r.get("content").and_then(|d| d.as_str()).unwrap_or(""));
        let engine = r.get("engine").and_then(|e| e.as_str()).unwrap_or("");
        let mut line = format!("url {}\ttitle: {}", link, title);
        if !engine.is_empty() {
            line.push_str(&format!("\tengine: {}", engine));
        }
        if !content.is_empty() {
            line.push_str(&format!("\tcontent: {}", content));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

pub fn tavily_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — TAVILY_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let (text, refine) = crate::refine::split_refine(query, &["answer", "raw"]);
    let body = tavily_body(&text, max, token, &refine);
    let auth = format!("Authorization: Bearer {}", token);
    let headers = [auth.as_str()];
    match post("https://api.tavily.com/search", &body, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = tavily_results(&v, max);
                if let Some(answer) = v.get("answer").and_then(|a| a.as_str()) {
                    let answer = flatten(answer);
                    if !answer.is_empty() {
                        out.insert(0, format!("answer: {}", answer));
                    }
                }
                if out.is_empty() {
                    out.push(format!("absent — Tavily carries no entry: {}", text));
                }
                out
            }
            None => vec!["pending — the Tavily response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — Tavily HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn tavily_body(query: &str, max: usize, token: &str, refine: &[(String, String)]) -> String {
    let mut body = format!(
        "{{\"query\":\"{}\",\"max_results\":{},\"api_key\":\"{}\"",
        json_escape(query),
        max,
        json_escape(token)
    );
    if let Some(value) = crate::refine::value_of(refine, "answer") {
        body.push_str(&format!(
            ",\"include_answer\":{}",
            tavily_content_value(value)
        ));
    }
    if let Some(value) = crate::refine::value_of(refine, "raw") {
        body.push_str(&format!(
            ",\"include_raw_content\":{}",
            tavily_content_value(value)
        ));
    }
    body.push('}');
    body
}

fn tavily_content_value(value: &str) -> String {
    match value {
        "1" | "true" => "true".to_string(),
        other => format!("\"{}\"", json_escape(other)),
    }
}

fn tavily_score(r: &Json) -> Option<String> {
    match r.get("score") {
        Some(Json::Num(n)) if n.is_finite() => Some(format!("{n}")),
        Some(Json::Str(s)) if !s.is_empty() => Some(s.clone()),
        _ => None,
    }
}

fn tavily_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for r in results {
        let link = r.get("url").and_then(|u| u.as_str()).unwrap_or("");
        if link.is_empty() {
            continue;
        }
        let title = flatten(r.get("title").and_then(|t| t.as_str()).unwrap_or(""));
        let content = flatten(r.get("content").and_then(|c| c.as_str()).unwrap_or(""));
        let mut line = format!("url {}\ttitle: {}", link, title);
        if let Some(score) = tavily_score(r) {
            line.push_str(&format!("\tscore: {}", score));
        }
        if !content.is_empty() {
            line.push_str(&format!("\tdescription: {}", content));
        }
        let raw = cap_content(r.get("raw_content").and_then(|c| c.as_str()).unwrap_or(""));
        if !raw.is_empty() {
            line.push_str(&format!("\tcontent: {}", raw));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

pub fn exa_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — EXA_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let (text, refine) = crate::refine::split_refine(query, &["contents"]);
    let body = exa_body(&text, max, crate::refine::value_of(&refine, "contents"));
    let auth = format!("x-api-key: {}", token);
    let headers = [auth.as_str()];
    match post("https://api.exa.ai/search", &body, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = exa_results(&v, max);
                if out.is_empty() {
                    out.push(format!("absent — Exa carries no entry: {}", text));
                }
                out
            }
            None => vec!["pending — the Exa response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — Exa HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn exa_body(query: &str, max: usize, contents: Option<&str>) -> String {
    let mut body = format!(
        "{{\"query\":\"{}\",\"numResults\":{}",
        json_escape(query),
        max
    );
    match contents {
        Some("highlights") => body.push_str(",\"contents\":{\"highlights\":true}"),
        Some("summary") => body.push_str(",\"contents\":{\"summary\":true}"),
        Some(_) => body.push_str(",\"contents\":{\"text\":true}"),
        None => {}
    }
    body.push('}');
    body
}

fn exa_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for r in results {
        let link = r.get("url").and_then(|u| u.as_str()).unwrap_or("");
        if link.is_empty() {
            continue;
        }
        let title = flatten(r.get("title").and_then(|t| t.as_str()).unwrap_or(""));
        let mut line = format!("url {}\ttitle: {}", link, title);
        let author = flatten(r.get("author").and_then(|a| a.as_str()).unwrap_or(""));
        if !author.is_empty() {
            line.push_str(&format!("\tauthor: {}", author));
        }
        let published = flatten(
            r.get("publishedDate")
                .and_then(|p| p.as_str())
                .unwrap_or(""),
        );
        if !published.is_empty() {
            line.push_str(&format!("\tpublished: {}", published));
        }
        let text = cap_content(r.get("text").and_then(|t| t.as_str()).unwrap_or(""));
        if !text.is_empty() {
            line.push_str(&format!("\tdescription: {}", text));
        }
        if let Some(highlights) = r.get("highlights").and_then(|h| h.as_arr()) {
            for highlight in highlights {
                if let Some(highlight) = highlight.as_str() {
                    let highlight = cap_content(highlight);
                    if !highlight.is_empty() {
                        line.push_str(&format!("\thighlight: {}", highlight));
                    }
                }
            }
        }
        let summary = cap_content(r.get("summary").and_then(|s| s.as_str()).unwrap_or(""));
        if !summary.is_empty() {
            line.push_str(&format!("\tsummary: {}", summary));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

fn organic_lines(v: &Json, key: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(results) = v.get(key).and_then(|r| r.as_arr()) else {
        return out;
    };
    for r in results {
        let link = r.get("link").and_then(|u| u.as_str()).unwrap_or("");
        if link.is_empty() {
            continue;
        }
        let title = flatten(r.get("title").and_then(|t| t.as_str()).unwrap_or(""));
        let mut line = format!("url {}\ttitle: {}", link, title);
        if let Some(p) = r.get("position") {
            let pv = match p {
                Json::Num(n) if n.is_finite() => format!("{n}"),
                Json::Str(s) => s.clone(),
                _ => String::new(),
            };
            if !pv.is_empty() {
                line.push_str(&format!("\tposition: {}", pv));
            }
        }
        let snippet = flatten(r.get("snippet").and_then(|s| s.as_str()).unwrap_or(""));
        if !snippet.is_empty() {
            line.push_str(&format!("\tsnippet: {}", snippet));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

pub fn serper_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — SERPER_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let body = format!("{{\"q\":\"{}\",\"num\":{}}}", json_escape(query), max);
    let auth = format!("X-API-KEY: {}", token);
    let headers = [auth.as_str()];
    match post("https://google.serper.dev/search", &body, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = organic_lines(&v, "organic", max);
                if out.is_empty() {
                    out.push(format!("absent — Serper carries no entry: {}", query));
                }
                out
            }
            None => vec!["pending — the Serper response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — Serper HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn firecrawl_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — FIRECRAWL_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let body = format!("{{\"query\":\"{}\",\"limit\":{}}}", json_escape(query), max);
    let auth = format!("Authorization: Bearer {}", token);
    let headers = [auth.as_str()];
    match post("https://api.firecrawl.dev/v1/search", &body, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = firecrawl_results(&v, max);
                if out.is_empty() {
                    out.push(format!("absent — Firecrawl carries no entry: {}", query));
                }
                out
            }
            None => vec!["pending — the Firecrawl response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — Firecrawl HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn firecrawl_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(results) = v.get("data").and_then(|r| r.as_arr()) else {
        return out;
    };
    for r in results {
        let link = r.get("url").and_then(|u| u.as_str()).unwrap_or("");
        if link.is_empty() {
            continue;
        }
        let title = flatten(r.get("title").and_then(|t| t.as_str()).unwrap_or(""));
        let mut line = format!("url {}\ttitle: {}", link, title);
        let description = flatten(r.get("description").and_then(|d| d.as_str()).unwrap_or(""));
        if !description.is_empty() {
            let clipped: String = description.chars().take(240).collect();
            line.push_str(&format!("\tdescription: {}", clipped));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

pub fn searchapi_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — SEARCHAPI_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let url = format!(
        "https://www.searchapi.io/api/v1/search?engine=google&q={}&num={}&api_key={}",
        urlencode(query),
        max,
        urlencode(token)
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = organic_lines(&v, "organic_results", max);
                if out.is_empty() {
                    out.push(format!("absent — SearchApi carries no entry: {}", query));
                }
                out
            }
            None => vec!["pending — the SearchApi response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — SearchApi HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn serpapi_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — SERPAPI_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let url = format!(
        "https://serpapi.com/search.json?engine=google&q={}&num={}&api_key={}",
        urlencode(query),
        max,
        urlencode(token)
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = organic_lines(&v, "organic_results", max);
                if out.is_empty() {
                    out.push(format!("absent — SerpApi carries no entry: {}", query));
                }
                out
            }
            None => vec!["pending — the SerpApi response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — SerpApi HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn oeis_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("https://oeis.org/search?q={}&fmt=json", urlencode(query));
    let headers = ["-H", "Accept: application/json"];
    match get(&url, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = oeis_results(&v, max);
                if out.is_empty() {
                    out.push(format!("absent — OEIS carries no entry: {}", query));
                }
                out
            }
            None => vec!["pending — the OEIS response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — OEIS HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn oeis_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(rows) = v.as_arr() else {
        return out;
    };
    for r in rows {
        let Some(Json::Num(number)) = r.get("number") else {
            continue;
        };
        let name = flatten(r.get("name").and_then(|n| n.as_str()).unwrap_or(""));
        let data = flatten(r.get("data").and_then(|d| d.as_str()).unwrap_or(""));
        let mut line = format!(
            "url https://oeis.org/A{:06}\ttitle: {}",
            *number as i64, name
        );
        if !data.is_empty() {
            let clipped: String = data.chars().take(120).collect();
            line.push_str(&format!("\tdata: {}", clipped));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

fn hal_url(text: &str, refine: &[(String, String)], max: usize, start: usize) -> String {
    let fl =
        crate::refine::value_of(refine, "fl").unwrap_or("title_s,uri_s,doiId_s,publicationDate_s");
    let mut url = format!(
        "https://api.archives-ouvertes.fr/search/?q={}&wt=json&rows={}&fl={}",
        urlencode(text),
        max,
        urlencode(fl)
    );
    if start > 0 {
        url.push_str("&start=");
        url.push_str(&start.to_string());
    }
    for (key, value) in refine {
        if key == "fl" {
            continue;
        }
        url.push('&');
        url.push_str(key);
        url.push('=');
        url.push_str(&urlencode(value));
    }
    url
}

pub fn hal_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(query, &["fl", "fq", "sort"]);
    let headers = ["-H", "Accept: application/json"];
    let (mut lines, stop) =
        crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |index| {
            let start = index * max;
            let url = hal_url(&text, &refine, max, start);
            match get(&url, &headers, "40") {
                Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                    Some(v) => {
                        let out = hal_results(&v, max);
                        let num_found = v
                            .get("response")
                            .and_then(|r| r.get("numFound"))
                            .and_then(|n| n.as_scalar_string())
                            .and_then(|s| s.parse::<usize>().ok());
                        let has_more = match num_found {
                            Some(n) => start + out.len() < n,
                            None => out.len() >= max,
                        };
                        (out, has_more)
                    }
                    None => (
                        vec!["pending — the HAL response carries no JSON".to_string()],
                        false,
                    ),
                },
                Some(f) => (
                    vec![format!("pending — HAL HTTP {}", f.status_text())],
                    false,
                ),
                None => (vec!["pending — no network".to_string()], false),
            }
        });
    if lines.is_empty() {
        vec![format!("absent — HAL carries no entry: {}", query)]
    } else {
        lines.push(format!("end: {}", stop.label()));
        lines
    }
}

fn hal_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(docs) = v
        .get("response")
        .and_then(|r| r.get("docs"))
        .and_then(|d| d.as_arr())
    else {
        return out;
    };
    for r in docs {
        let link = r.get("uri_s").and_then(|u| u.as_str()).unwrap_or("");
        if link.is_empty() {
            continue;
        }
        let title = match r
            .get("title_s")
            .and_then(|t| t.as_arr())
            .and_then(|a| a.first())
        {
            Some(Json::Str(s)) => flatten(s),
            _ => String::new(),
        };
        let mut line = format!("url {}\ttitle: {}", link, title);
        let doi = r.get("doiId_s").and_then(|d| d.as_str()).unwrap_or("");
        if !doi.is_empty() {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        let published = r
            .get("publicationDate_s")
            .and_then(|p| p.as_str())
            .unwrap_or("");
        if !published.is_empty() {
            line.push_str(&format!("\tpublished: {}", published));
        }
        let authors: Vec<&str> = match r.get("authFullName_s").and_then(|a| a.as_arr()) {
            Some(arr) => arr.iter().filter_map(|n| n.as_str()).collect(),
            None => Vec::new(),
        };
        if !authors.is_empty() {
            let shown = if authors.len() > 8 {
                &authors[..8]
            } else {
                &authors[..]
            };
            let mut names = shown.join(", ");
            if authors.len() > 8 {
                names.push_str(", et al.");
            }
            line.push_str(&format!("\tauthors: {}", names));
        }
        let abstract_text = match r
            .get("abstract_s")
            .and_then(|a| a.as_arr())
            .and_then(|a| a.first())
            .and_then(|s| s.as_str())
        {
            Some(s) if !s.is_empty() => flatten(s),
            _ => String::new(),
        };
        if !abstract_text.is_empty() {
            line.push_str(&format!(
                "\tabstract: {}",
                truncate_to_chars(&abstract_text, 1200)
            ));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

fn truncate_to_chars(s: &str, max: usize) -> String {
    if let Some((i, _)) = s.char_indices().nth(max) {
        s[..i].to_string()
    } else {
        s.to_string()
    }
}

pub fn wiby_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!("https://wiby.me/json/?q={}", urlencode(query));
    let headers = ["-H", "Accept: application/json"];
    match get(&url, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = wiby_results(&v, max);
                if out.is_empty() {
                    out.push(format!("absent — Wiby carries no entry: {}", query));
                }
                out
            }
            None => vec!["pending — the Wiby response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — Wiby HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn wiby_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(rows) = v.as_arr() else {
        return out;
    };
    for r in rows {
        let link = r.get("URL").and_then(|u| u.as_str()).unwrap_or("");
        if link.is_empty() {
            continue;
        }
        let title = flatten(r.get("Title").and_then(|t| t.as_str()).unwrap_or(""));
        let mut line = format!("url {}\ttitle: {}", link, title);
        let desc = flatten(r.get("Description").and_then(|d| d.as_str()).unwrap_or(""));
        if !desc.is_empty() {
            line.push_str(&format!("\tdescription: {}", desc));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

fn ia_search_url(text: &str, refine: &[(String, String)], max: usize, page: usize) -> String {
    let fl =
        crate::refine::value_of(refine, "fl").unwrap_or("identifier,title,mediatype,creator,year");
    let mut url = format!(
        "https://archive.org/advancedsearch.php?q={}&rows={}&output=json",
        urlencode(text),
        max
    );
    for field in fl.split(',') {
        if field.is_empty() {
            continue;
        }
        url.push_str("&fl%5B%5D=");
        url.push_str(&urlencode(field));
    }
    if page > 1 {
        url.push_str("&page=");
        url.push_str(&page.to_string());
    }
    for (key, value) in refine {
        if key == "fl" {
            continue;
        }
        url.push_str("&sort%5B%5D=");
        url.push_str(&urlencode(value));
    }
    url
}

pub fn ia_search_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(query, &["fl", "sort"]);
    let headers = ["-H", "Accept: application/json"];
    let (mut lines, stop) =
        crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |index| {
            let page = index + 1;
            let url = ia_search_url(&text, &refine, max, page);
            match get(&url, &headers, "40") {
                Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                    Some(v) => {
                        let out = ia_search_results(&v, max);
                        let num_found = v
                            .get("response")
                            .and_then(|r| r.get("numFound"))
                            .and_then(|n| n.as_scalar_string())
                            .and_then(|s| s.parse::<usize>().ok());
                        let has_more = match num_found {
                            Some(n) => page * max < n,
                            None => out.len() >= max,
                        };
                        (out, has_more)
                    }
                    None => (
                        vec!["pending — the Internet Archive response carries no JSON".to_string()],
                        false,
                    ),
                },
                Some(f) => (
                    vec![format!(
                        "pending — Internet Archive HTTP {}",
                        f.status_text()
                    )],
                    false,
                ),
                None => (vec!["pending — no network".to_string()], false),
            }
        });
    if lines.is_empty() {
        vec![format!(
            "absent — the Internet Archive carries no entry: {}",
            query
        )]
    } else {
        lines.push(format!("end: {}", stop.label()));
        lines
    }
}

fn ia_search_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(docs) = v
        .get("response")
        .and_then(|r| r.get("docs"))
        .and_then(|d| d.as_arr())
    else {
        return out;
    };
    for r in docs {
        let id = r.get("identifier").and_then(|i| i.as_str()).unwrap_or("");
        if id.is_empty() {
            continue;
        }
        let title = flatten(r.get("title").and_then(|t| t.as_str()).unwrap_or(""));
        let mediatype = r.get("mediatype").and_then(|m| m.as_str()).unwrap_or("");
        let mut line = format!("url https://archive.org/details/{}\ttitle: {}", id, title);
        if !mediatype.is_empty() {
            line.push_str(&format!("\tmediatype: {}", mediatype));
        }
        if let Some(creator) = r
            .get("creator")
            .map(ia_creator_names)
            .filter(|c| !c.is_empty())
        {
            line.push_str(&format!("\tcreator: {}", creator));
        }
        let year = r
            .get("year")
            .and_then(|y| y.as_scalar_string())
            .map(|y| flatten(&y))
            .filter(|y| !y.is_empty())
            .or_else(|| {
                r.get("date")
                    .and_then(|d| d.as_str())
                    .map(|d| d.chars().take(4).collect::<String>())
                    .filter(|y| !y.is_empty())
            });
        if let Some(year) = year {
            line.push_str(&format!("\tyear: {}", year));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

fn ia_creator_names(creator: &Json) -> String {
    if let Some(arr) = creator.as_arr() {
        let names: Vec<String> = arr
            .iter()
            .filter_map(|n| n.as_str())
            .map(flatten)
            .filter(|n| !n.is_empty())
            .collect();
        if names.is_empty() {
            return String::new();
        }
        if names.len() > 8 {
            let mut out = names[..8].join(", ");
            out.push_str(", et al.");
            out
        } else {
            names.join(", ")
        }
    } else {
        flatten(creator.as_str().unwrap_or(""))
    }
}

fn ngmdb_url(text: &str, refine: &[(String, String)], max: usize) -> String {
    let state = crate::refine::value_of(refine, "state")
        .map(|s| s.trim().to_uppercase())
        .filter(|s| !s.is_empty());
    let mut conditions = Vec::new();
    if !(text.is_empty() && state.is_some()) {
        conditions.push(format!("map_name+LIKE+%27%25{}%27", urlencode(text)));
    }
    if let Some(state) = &state {
        conditions.push(format!("primary_state%3D%27{}%27", urlencode(state)));
    }
    format!(
        "https://ngmdb.usgs.gov/arcgis/rest/services/topoview/ustOverlay/MapServer/0/query?where={}&outFields=map_name,primary_state,imprint_year,scan_id&f=json&resultRecordCount={}",
        conditions.join("+AND+"),
        max
    )
}

pub fn ngmdb_lines(query: &str, max: usize) -> Vec<String> {
    let (text, refine) = crate::refine::split_refine(query, &["state"]);
    let url = ngmdb_url(&text, &refine, max);
    let headers = [
        "-H",
        "User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0 Safari/537.36",
        "-H",
        "Accept: application/json",
        "-H",
        "Range: bytes=0-131071",
    ];
    match get(&url, &headers, "40") {
        Some(f) if f.status == Some(200) || f.status == Some(206) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = ngmdb_results(&v, max);
                if out.is_empty() {
                    out.push(format!("absent — NGMDB carries no map: {}", query));
                }
                out
            }
            None => vec!["pending — the NGMDB response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — NGMDB HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn ngmdb_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(features) = v.get("features").and_then(|f| f.as_arr()) else {
        return out;
    };
    for feature in features {
        let Some(attrs) = feature.get("attributes") else {
            continue;
        };
        let Some(Json::Num(scan)) = attrs.get("scan_id") else {
            continue;
        };
        if !scan.is_finite() {
            continue;
        }
        let map_name = flatten(attrs.get("map_name").and_then(|m| m.as_str()).unwrap_or(""));
        let state = attrs
            .get("primary_state")
            .and_then(|s| s.as_str())
            .unwrap_or("");
        let year = match attrs.get("imprint_year") {
            Some(Json::Num(y)) if y.is_finite() => format!("{}", *y as i64),
            _ => String::new(),
        };
        out.push(format!(
            "url https://ngmdb.usgs.gov/topoview/viewer/#{}\ttitle: {}\tstate: {}\tyear: {}",
            *scan as i64, map_name, state, year
        ));
        if out.len() >= max {
            break;
        }
    }
    out
}

pub fn rss_bridge_lines(query: &str, max: usize) -> Vec<String> {
    if kv_token(query, "bridge").is_none() {
        return vec!["pending — rss-bridge needs bridge=<name>".to_string()];
    }
    let mut url = String::from("https://rss-bridge.org/bridge01/?action=display&format=Json");
    for token in query.split_whitespace() {
        if let Some((k, v)) = token.split_once('=') {
            url.push_str(&format!("&{}={}", urlencode(k), urlencode(v)));
        }
    }
    let headers = ["-H", "Accept: application/json"];
    match get(&url, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = rss_bridge_results(&v, max);
                if out.is_empty() {
                    out.push(format!("absent — rss-bridge carries no item: {}", query));
                }
                out
            }
            None => vec!["pending — the rss-bridge response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — rss-bridge HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn rss_bridge_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(items) = v.get("items").and_then(|i| i.as_arr()) else {
        return out;
    };
    for item in items {
        let link = item.get("url").and_then(|u| u.as_str()).unwrap_or("");
        if link.is_empty() {
            continue;
        }
        let title = flatten(item.get("title").and_then(|t| t.as_str()).unwrap_or(""));
        let mut line = format!("url {}\ttitle: {}", link, title);
        let content = flatten(
            item.get("content_text")
                .and_then(|c| c.as_str())
                .unwrap_or(""),
        );
        if !content.is_empty() {
            let clipped: String = content.chars().take(120).collect();
            line.push_str(&format!("\tcontent: {}", clipped));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

fn acquisition_link(entry: &str) -> Option<String> {
    let pos = entry.find("acquisition")?;
    let tag_start = entry[..pos].rfind("<link")?;
    let tag_end = entry[pos..].find('>')? + pos;
    let tag = &entry[tag_start..tag_end];
    let h = tag.find("href=\"")? + 6;
    let after = &tag[h..];
    let end = after.find('"')?;
    Some(after[..end].to_string())
}

fn entry_blocks(xml: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(open) = rest.find("<entry") {
        let close = match rest[open..].find("</entry>") {
            Some(c) => open + c,
            None => break,
        };
        out.push(&rest[open..close]);
        rest = &rest[close..];
    }
    out
}

pub fn kiwix_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "https://opds.library.kiwix.org/catalog/v2/entries?q={}&lang=eng&count={}",
        urlencode(query),
        max
    );
    let headers = ["-H", "Accept: application/atom+xml"];
    match get(&url, &headers, "40") {
        Some(f) if f.status == Some(200) => {
            let entries = parse_atom_entries(&f.body);
            let mut out = Vec::new();
            if entries.is_empty() {
                if f.body.contains("<entry") {
                    out = kiwix_fallback_results(&f.body, max);
                }
            } else {
                let blocks = entry_blocks(&f.body);
                for (i, e) in entries.iter().enumerate() {
                    let link = match blocks.get(i).and_then(|b| acquisition_link(b)) {
                        Some(l) => l,
                        None => e.id.clone(),
                    };
                    let mut line = format!("url {}\ttitle: {}", link, e.title);
                    if let Some(s) = &e.summary {
                        line.push_str(&format!("\tsummary: {}", s));
                    }
                    out.push(line);
                    if out.len() >= max {
                        break;
                    }
                }
            }
            if out.is_empty() {
                out.push(format!("absent — Kiwix carries no entry: {}", query));
            }
            out
        }
        Some(f) => vec![format!("pending — Kiwix HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn kiwix_fallback_results(xml: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    for block in entry_blocks(xml) {
        let Some(title) = extract_between(block, "<title>", "</title>") else {
            continue;
        };
        let title = flatten(&strip_tags(title));
        if title.is_empty() {
            continue;
        }
        let id = extract_between(block, "<id>", "</id>")
            .map(|t| flatten(&strip_tags(t)))
            .filter(|t| !t.is_empty());
        let Some(link) = acquisition_link(block).or(id) else {
            continue;
        };
        let mut line = format!("url {}\ttitle: {}", link, title);
        if let Some(s) = field_text(block, "<summary>", "</summary>") {
            line.push_str(&format!("\tsummary: {}", s));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

fn record_blocks(xml: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(open) = rest.find("<record") {
        let close = match rest[open..].find("</record>") {
            Some(c) => open + c,
            None => break,
        };
        out.push(&rest[open..close]);
        rest = &rest[close..];
    }
    out
}

fn oapen_set_blocks(xml: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(open) = rest.find("<set>") {
        let close = match rest[open..].find("</set>") {
            Some(c) => open + c,
            None => break,
        };
        out.push(&rest[open..close]);
        rest = &rest[close..];
    }
    out
}

fn oapen_title(block: &str) -> Option<String> {
    let mut rest = block;
    while let Some(open) = rest.find("<dc:title>") {
        let start = open + "<dc:title>".len();
        let tail = &rest[start..];
        let Some(end) = tail.find("</dc:title>") else {
            break;
        };
        let title = flatten(&strip_tags(&tail[..end]));
        if !title.is_empty() {
            return Some(title);
        }
        rest = &tail[end..];
    }
    None
}

pub fn oapen_lines(query: &str, max: usize) -> Vec<String> {
    let base = "https://library.oapen.org/oai/request";
    let headers = ["-H", "Accept: application/xml"];
    if query.trim().is_empty() {
        let url = format!("{base}?verb=ListSets");
        return match get(&url, &headers, "40") {
            Some(f) if f.status == Some(200) => {
                let mut out = Vec::new();
                for block in oapen_set_blocks(&f.body) {
                    let Some(spec) = field_text(block, "<setSpec>", "</setSpec>") else {
                        continue;
                    };
                    let name = field_text(block, "<setName>", "</setName>");
                    let mut line = format!("set {spec}");
                    if let Some(name) = name {
                        line.push_str(&format!("\tname: {name}"));
                    }
                    out.push(line);
                    if out.len() >= max {
                        break;
                    }
                }
                if out.is_empty() {
                    out.push("absent — OAPEN OAI carries no set".to_string());
                }
                out
            }
            Some(f) => vec![format!("pending — OAPEN OAI HTTP {}", f.status_text())],
            None => vec!["pending — no network".to_string()],
        };
    }
    let Some(set) = kv_token(query, "set") else {
        return vec![
            "pending — OAPEN OAI carries no free-text search; give set=<setSpec> (--oapen with no argument lists the sets)"
                .to_string(),
        ];
    };
    let mut url = format!(
        "{base}?verb=ListRecords&metadataPrefix=oai_dc&set={}",
        urlencode(&set)
    );
    if let Some(from) = kv_token(query, "from") {
        url.push_str(&format!("&from={}", urlencode(&from)));
    }
    if let Some(until) = kv_token(query, "until") {
        url.push_str(&format!("&until={}", urlencode(&until)));
    }
    match get(&url, &headers, "40") {
        Some(f) if f.status == Some(200) => {
            if f.body.contains("noRecordsMatch") {
                return vec![format!(
                    "absent — OAPEN OAI: noRecordsMatch for set {}",
                    set
                )];
            }
            let mut out = Vec::new();
            for block in record_blocks(&f.body) {
                let Some(ident) = field_text(block, "<identifier>", "</identifier>") else {
                    continue;
                };
                let handle = ident
                    .strip_prefix("oai:library.oapen.org:")
                    .unwrap_or(&ident)
                    .to_string();
                let Some(title) = oapen_title(block) else {
                    continue;
                };
                out.push(format!(
                    "url https://library.oapen.org/handle/{}\ttitle: {}",
                    handle, title
                ));
                if out.len() >= max {
                    break;
                }
            }
            if out.is_empty() {
                return vec![format!(
                    "absent — OAPEN OAI carries no record for set {}",
                    set
                )];
            }
            if f.body.contains("<resumptionToken") {
                out.push("more — resumptionToken present".to_string());
            }
            out
        }
        Some(f) => vec![format!("pending — OAPEN OAI HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn regtap_lines(query: &str, max: usize) -> Vec<String> {
    let keyword = query.trim();
    if keyword.is_empty() {
        return vec!["pending — regtap needs a keyword".to_string()];
    }
    let sanitized = keyword.replace('\'', "''");
    let adql = format!(
        "SELECT TOP {max} ivoid, res_title, res_description FROM rr.resource WHERE 1=ivo_nocasematch(res_title, '%{sanitized}%') OR 1=ivo_nocasematch(res_description, '%{sanitized}%')"
    );
    let path = format!(
        "/sync?request=doQuery&lang=ADQL&format=json&query={}",
        urlencode(&adql)
    );
    let headers = ["-H", "Accept: application/json"];
    let primary = get(&format!("http://dc.g-vo.org/tap{path}"), &headers, "40");
    let primary_status = primary.as_ref().and_then(|f| f.status);
    if let Some(f) = &primary {
        if f.status == Some(200) {
            if let Some(out) = regtap_body(&f.body, max) {
                return regtap_result(out, query);
            }
        }
    }
    let mirror = get(
        &format!("https://registry.euro-vo.org/eurovo/regtap/tap{path}"),
        &headers,
        "40",
    );
    if let Some(f) = &mirror {
        if f.status == Some(200) {
            if let Some(out) = regtap_body(&f.body, max) {
                return regtap_result(out, query);
            }
        }
    }
    match primary_status {
        Some(s) => vec![format!("pending — RegTAP HTTP {s}")],
        None => vec!["pending — no network".to_string()],
    }
}

fn regtap_body(body: &str, max: usize) -> Option<Vec<String>> {
    let v = json::parse(body)?;
    let rows = v.get("data").and_then(|d| d.as_arr())?;
    let mut out = Vec::new();
    for row in rows {
        let Some(cols) = row.as_arr() else {
            continue;
        };
        let ivoid = cols.first().and_then(|c| c.as_str()).unwrap_or("");
        if ivoid.is_empty() {
            continue;
        }
        let title = cols.get(1).and_then(|c| c.as_str()).unwrap_or("");
        let mut line = format!("url {ivoid}\ttitle: {title}");
        if let Some(desc) = cols.get(2).and_then(|c| c.as_str()) {
            if !desc.trim().is_empty() {
                line.push_str(&format!("\tdescription: {}", regtap_clip(desc)));
            }
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    Some(out)
}

fn regtap_result(out: Vec<String>, query: &str) -> Vec<String> {
    if out.is_empty() {
        vec![format!("absent — RegTAP carries no resource: {query}")]
    } else {
        out
    }
}

fn regtap_clip(s: &str) -> String {
    let flat = flatten(s);
    if flat.chars().count() <= 160 {
        return flat;
    }
    let mut out: String = flat.chars().take(160).collect();
    out.push('…');
    out
}

pub fn apis_lines(query: &str, max: usize) -> Vec<String> {
    let keyword = query.trim();
    if keyword.is_empty() {
        return vec!["pending — apis needs a keyword".to_string()];
    }
    let headers = ["-H", "Accept: application/json"];
    let url = format!("https://apis.io/api/v1/apis?q={}", urlencode(keyword));
    if let Some(f) = get(&url, &headers, "40") {
        if f.status == Some(200) {
            if let Some(out) = apis_io_body(&f.body, max) {
                if !out.is_empty() {
                    return out;
                }
            }
        }
    }
    apis_guru_lines(keyword, max)
}

fn apis_io_body(body: &str, max: usize) -> Option<Vec<String>> {
    let v = json::parse(body)?;
    let data = v.get("data").and_then(|d| d.as_arr())?;
    let mut out = Vec::new();
    for item in data {
        let name = item.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let url = item
            .get("humanURL")
            .and_then(|u| u.as_str())
            .or_else(|| item.get("baseURL").and_then(|u| u.as_str()))
            .unwrap_or("");
        let provider = item
            .get("provider_name")
            .and_then(|p| p.as_str())
            .unwrap_or("");
        let mut line = format!("url {url}\ttitle: {name}\tprovider: {provider}");
        if let Some(d) = item.get("description").and_then(|d| d.as_str()) {
            if !d.trim().is_empty() {
                line.push_str(&format!("\tdescription: {}", regtap_clip(d)));
            }
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    Some(out)
}

fn apis_guru_lines(keyword: &str, max: usize) -> Vec<String> {
    let query = keyword;
    let needle = keyword.to_lowercase();
    let headers = ["-H", "Accept: application/json"];
    match get("https://api.apis.guru/v2/list.json", &headers, "60") {
        Some(f) if f.status == Some(200) => match apis_body(&f.body, &needle, max) {
            Some(out) if !out.is_empty() => out,
            Some(_) => vec![format!("absent — apis.guru carries no API: {query}")],
            None => vec!["pending — the apis.guru response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — apis.guru HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn apis_body(body: &str, needle: &str, max: usize) -> Option<Vec<String>> {
    let Json::Obj(providers) = json::parse(body)? else {
        return None;
    };
    let mut keys: Vec<&String> = providers.keys().collect();
    keys.sort();
    let mut out = Vec::new();
    for provider in keys {
        let Some(pv) = providers.get(provider) else {
            continue;
        };
        let versions = pv.get("versions");
        let entry = match pv.get("preferred").and_then(|p| p.as_str()) {
            Some(pref) => versions.and_then(|vs| vs.get(pref)),
            None => versions.and_then(|vs| match vs {
                Json::Obj(map) => map.values().next(),
                _ => None,
            }),
        };
        let Some(entry) = entry else {
            continue;
        };
        let info = entry.get("info");
        let title = info
            .and_then(|i| i.get("title"))
            .and_then(|t| t.as_str())
            .unwrap_or("");
        let desc = info
            .and_then(|i| i.get("description"))
            .and_then(|d| d.as_str());
        let in_desc = desc
            .map(|d| d.to_lowercase().contains(needle))
            .unwrap_or(false);
        if !provider.to_lowercase().contains(needle)
            && !title.to_lowercase().contains(needle)
            && !in_desc
        {
            continue;
        }
        let swagger = entry
            .get("swaggerUrl")
            .and_then(|s| s.as_str())
            .unwrap_or("");
        let mut line = format!("url {swagger}\ttitle: {title}\tprovider: {provider}");
        if let Some(d) = desc {
            if !d.trim().is_empty() {
                line.push_str(&format!("\tdescription: {}", regtap_clip(d)));
            }
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    Some(out)
}

pub fn scrape_lines(query: &str, max: usize) -> Vec<String> {
    let Some(target) = query.split_whitespace().next() else {
        return vec!["pending — scrape needs a target url".to_string()];
    };
    let selector = match kv_token(query, "selector") {
        Some(s) => s,
        None => "a".to_string(),
    };
    let url = format!(
        "https://web.scraper.workers.dev/?url={}&selector={}&pretty=true",
        urlencode(target),
        urlencode(&selector)
    );
    let headers = ["-H", "Accept: application/json"];
    match get(&url, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = Vec::new();
                if let Some(Json::Obj(map)) = v.get("result") {
                    let mut keys: Vec<&String> = map.keys().collect();
                    keys.sort();
                    for k in keys {
                        let Some(arr) = map[k].as_arr() else {
                            continue;
                        };
                        for item in arr {
                            let Some(s) = item.as_str() else {
                                continue;
                            };
                            out.push(format!("url {}\ttext: {}", target, s));
                            if out.len() >= max {
                                break;
                            }
                        }
                        if out.len() >= max {
                            break;
                        }
                    }
                }
                if out.is_empty() {
                    out.push(format!(
                        "absent — the scraped page carries no {}: {}",
                        selector, target
                    ));
                }
                out
            }
            None => vec!["pending — the scraper response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — scraper HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn kv_token(query: &str, key: &str) -> Option<String> {
    query.split_whitespace().find_map(|t| {
        t.split_once('=')
            .and_then(|(k, v)| if k == key { Some(v.to_string()) } else { None })
    })
}

fn error_detail(body: &str) -> String {
    if let Some(v) = json::parse(body) {
        for k in ["error", "detail", "message", "msg"] {
            if let Some(s) = v.get(k).and_then(|m| m.as_str()) {
                if !s.is_empty() {
                    return s.to_string();
                }
            }
        }
    }
    let clipped: String = body.chars().take(160).collect();
    clipped
}

fn json_compact(v: &Json) -> String {
    match v {
        Json::Null => "null".to_string(),
        Json::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        Json::Num(n) => {
            if n.is_finite() {
                format!("{n}")
            } else {
                "null".to_string()
            }
        }
        Json::Str(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        Json::Arr(a) => {
            let parts: Vec<String> = a.iter().map(json_compact).collect();
            format!("[{}]", parts.join(","))
        }
        Json::Obj(m) => {
            let mut keys: Vec<&String> = m.keys().collect();
            keys.sort();
            let parts: Vec<String> = keys
                .iter()
                .map(|k| format!("\"{}\":{}", k, json_compact(&m[*k])))
                .collect();
            format!("{{{}}}", parts.join(","))
        }
    }
}

pub fn shodan_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — SHODAN_API_KEY absent from .secrets.local/.env".to_string()];
    }
    if query.split_whitespace().count() != 1 || query.trim().is_empty() {
        return vec![format!(
            "pending — shodan uses the free-tier host lookup: pass a single ip (got: {})",
            query
        )];
    }
    let target = query.trim();
    let url = format!(
        "https://api.shodan.io/shodan/host/{}?key={}",
        urlencode(target),
        urlencode(token)
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = shodan_host_results(&v, max);
                if out.is_empty() {
                    out.push(format!("absent — Shodan carries no host for {}", target));
                }
                out
            }
            None => vec!["pending — the Shodan response carries no JSON".to_string()],
        },
        Some(f) => vec![format!(
            "pending — Shodan HTTP {} {}",
            f.status_text(),
            error_detail(&f.body)
        )],
        None => vec!["pending — no network".to_string()],
    }
}

fn shodan_host_results(v: &Json, max: usize) -> Vec<String> {
    let ip = v.get("ip_str").and_then(|i| i.as_str()).unwrap_or("");
    if ip.is_empty() {
        return Vec::new();
    }
    let mut line = format!("url https://www.shodan.io/host/{}\tip: {}", ip, ip);
    if let Some(ports) = v.get("ports").and_then(|p| p.as_arr()) {
        let list: Vec<String> = ports.iter().filter_map(|p| p.as_scalar_string()).collect();
        if !list.is_empty() {
            line.push_str(&format!("\tports: {}", list.join(",")));
        }
    }
    if let Some(org) = v.get("org").and_then(|o| o.as_str()) {
        if !org.is_empty() {
            line.push_str(&format!("\torg: {}", org));
        }
    }
    if let Some(country) = v.get("country_name").and_then(|c| c.as_str()) {
        if !country.is_empty() {
            line.push_str(&format!("\tcountry: {}", country));
        }
    }
    let mut out = vec![line];
    if let Some(data) = v.get("data").and_then(|d| d.as_arr()) {
        for banner in data.iter().take(max.saturating_sub(1)) {
            let text = banner.get("data").and_then(|d| d.as_str()).map(flatten);
            if let Some(text) = text {
                if !text.is_empty() {
                    let clipped: String = text.chars().take(120).collect();
                    out.push(format!("  banner: {}", clipped));
                }
            }
        }
    }
    out
}

pub fn opencellid_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — OPENCELLID_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let (Some(mcc), Some(mnc), Some(lac), Some(cellid)) = (
        kv_token(query, "mcc"),
        kv_token(query, "mnc"),
        kv_token(query, "lac"),
        kv_token(query, "cellid"),
    ) else {
        return vec![format!(
            "pending — OpenCelliD needs mcc=<..> mnc=<..> lac=<..> cellid=<..> [radio=..] (got: {})",
            query
        )];
    };
    let mut url = format!(
        "https://opencellid.org/cell/get?key={}&mcc={}&mnc={}&lac={}&cellid={}&format=json",
        urlencode(token),
        urlencode(&mcc),
        urlencode(&mnc),
        urlencode(&lac),
        urlencode(&cellid)
    );
    if let Some(radio) = kv_token(query, "radio") {
        url.push_str(&format!("&radio={}", urlencode(&radio)));
    }
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => opencellid_results(&v, &query)
                .into_iter()
                .take(max)
                .collect(),
            None => vec!["pending — the OpenCelliD response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — OpenCelliD HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn opencellid_results(v: &Json, query: &str) -> Vec<String> {
    for key in ["error", "err", "message"] {
        if let Some(msg) = v.get(key).and_then(|m| m.as_str()) {
            return vec![format!("pending — OpenCelliD: {}", msg)];
        }
    }
    let lat = v.get("lat").and_then(|x| x.as_scalar_string());
    let lon = v.get("lon").and_then(|x| x.as_scalar_string());
    match (lat, lon) {
        (Some(lat), Some(lon)) => {
            let mut line = format!(
                "url https://www.opencellid.org/#zoom=16&lat={}&lon={}\tlat: {}\tlon: {}",
                lat, lon, lat, lon
            );
            if let Some(range) = v.get("range").and_then(|r| r.as_scalar_string()) {
                line.push_str(&format!("\trange: {}", range));
            }
            if let Some(samples) = v.get("samples").and_then(|s| s.as_scalar_string()) {
                line.push_str(&format!("\tsamples: {}", samples));
            }
            vec![line]
        }
        _ => vec![format!(
            "absent — OpenCelliD carries no position for {}",
            query
        )],
    }
}

pub fn gfw_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — GFW_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let dataset = kv_token(query, "dataset")
        .or_else(|| query.split_whitespace().next().map(|s| s.to_string()));
    let Some(dataset) = dataset else {
        return vec!["pending — gfw needs dataset=<name> [sql=<select>]".to_string()];
    };
    let sql = match query.split_once("sql=") {
        Some((_, rest)) if !rest.trim().is_empty() => rest.trim().to_string(),
        _ => format!("SELECT * FROM {dataset} LIMIT {max}"),
    };
    let base = format!(
        "https://data-api.globalforestwatch.org/dataset/{}/latest/query/json",
        urlencode(&dataset)
    );
    let auth = format!("x-api-key: {}", token);
    let bearer = format!("Authorization: Bearer {}", token);
    let headers = [auth.as_str(), bearer.as_str()];
    let fetched = match kv_token(query, "geometry") {
        Some(geom) if !geom.trim().is_empty() => {
            let body = format!(
                "{{\"sql\":\"{}\",\"geometry\":{}}}",
                json_escape(&sql),
                geom.trim()
            );
            post(&base, &body, &headers, "40")
        }
        _ => {
            let mut url = format!(
                "{}?sql={}&x-api-key={}",
                base,
                urlencode(&sql),
                urlencode(token)
            );
            if let Some(geostore) = kv_token(query, "geostore_id") {
                url.push_str(&format!("&geostore_id={}", urlencode(&geostore)));
            }
            get(&url, &headers, "40")
        }
    };
    match fetched {
        Some(f) if f.status == Some(200) => {
            let body = match f.body.find(|c| c == '{' || c == '[') {
                Some(start) => &f.body[start..],
                None => &f.body[..],
            };
            match json::parse(body) {
                Some(v) => gfw_results(&v, max),
                None => {
                    let snip: String = body.chars().take(160).collect();
                    vec![format!(
                        "pending — the GFW response carries no JSON: {snip}"
                    )]
                }
            }
        }
        Some(f) => vec![format!(
            "pending — GFW HTTP {} {}",
            f.status_text(),
            error_detail(&f.body)
        )],
        None => vec!["pending — no network".to_string()],
    }
}

fn gfw_results(v: &Json, max: usize) -> Vec<String> {
    let Some(data) = v.get("data").and_then(|d| d.as_arr()) else {
        let status = v.get("status").and_then(|s| s.as_str()).unwrap_or("?");
        let msg = v.get("message").and_then(|m| m.as_str()).unwrap_or("");
        return vec![format!("pending — GFW status {} {}", status, msg)];
    };
    let mut out = Vec::new();
    for row in data {
        let compact = json_compact(row);
        let clipped: String = compact.chars().take(300).collect();
        out.push(clipped);
        if out.len() >= max {
            break;
        }
    }
    if out.is_empty() {
        out.push("absent — GFW query carried no rows".to_string());
    }
    out
}

pub fn linkup_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — LINKUP_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let (text, refine) = crate::refine::split_refine(query, &["output", "depth"]);
    let output = crate::refine::value_of(&refine, "output").unwrap_or("searchResults");
    let depth = crate::refine::value_of(&refine, "depth").unwrap_or("standard");
    let body = linkup_body(&text, output, depth);
    let auth = format!("Authorization: Bearer {}", token);
    let headers = [auth.as_str()];
    match post("https://api.linkup.so/v1/search", &body, &headers, "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                let mut out = linkup_output(&v, max);
                if out.is_empty() {
                    out.push(format!("absent — Linkup carries no entry: {}", text));
                }
                out
            }
            None => vec!["pending — the Linkup response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — Linkup HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn linkup_body(query: &str, output: &str, depth: &str) -> String {
    let output = match output {
        "sourcedAnswer" => "sourcedAnswer",
        _ => "searchResults",
    };
    format!(
        "{{\"q\":\"{}\",\"depth\":\"{}\",\"outputType\":\"{}\"}}",
        json_escape(query),
        json_escape(depth),
        output
    )
}

fn linkup_output(v: &Json, max: usize) -> Vec<String> {
    let mut out = linkup_results(v, max);
    if out.is_empty() {
        if let Some(answer) = v.get("answer").and_then(|a| a.as_str()) {
            let answer = cap_content(answer);
            if !answer.is_empty() {
                out.push(format!("answer: {}", answer));
            }
        }
    }
    if let Some(sources) = v.get("sources").and_then(|s| s.as_arr()) {
        for source in sources {
            let link = source.get("url").and_then(|u| u.as_str()).unwrap_or("");
            if link.is_empty() {
                continue;
            }
            let name = flatten(source.get("name").and_then(|n| n.as_str()).unwrap_or(""));
            let mut line = format!("url {}\ttitle: {}", link, name);
            let snippet = cap_content(source.get("snippet").and_then(|s| s.as_str()).unwrap_or(""));
            if !snippet.is_empty() {
                line.push_str(&format!("\tdescription: {}", snippet));
            }
            out.push(line);
            if out.len() >= max {
                break;
            }
        }
    }
    out
}

fn linkup_results(v: &Json, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let Some(results) = v.get("results").and_then(|r| r.as_arr()) else {
        return out;
    };
    for r in results {
        let link = r.get("url").and_then(|u| u.as_str()).unwrap_or("");
        if link.is_empty() {
            continue;
        }
        let title = flatten(r.get("name").and_then(|n| n.as_str()).unwrap_or(""));
        let content = cap_content(r.get("content").and_then(|c| c.as_str()).unwrap_or(""));
        let mut line = format!("url {}\ttitle: {}", link, title);
        if !content.is_empty() {
            line.push_str(&format!("\tdescription: {}", content));
        }
        out.push(line);
        if out.len() >= max {
            break;
        }
    }
    out
}

fn value_bait_lines(bytes: &[u8]) -> Vec<String> {
    let text = String::from_utf8_lossy(bytes);
    let mut vals: Vec<String> = Vec::new();
    let mut cur = String::new();
    let flush = |cur: &mut String, vals: &mut Vec<String>| {
        let long_enough = cur.len() >= 6 || (cur.len() >= 4 && cur.contains('.'));
        if long_enough
            && cur.chars().any(|c| c.is_ascii_digit())
            && let Ok(v) = cur.parse::<f64>()
            && v.is_finite()
            && v != 0.0
        {
            vals.push(cur.clone());
        }
        cur.clear();
    };
    for ch in text.chars() {
        if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == 'e' || ch == 'E' || ch == '+' {
            cur.push(ch);
        } else {
            flush(&mut cur, &mut vals);
        }
        if vals.len() > 5000 {
            break;
        }
    }
    flush(&mut cur, &mut vals);
    let chars: Vec<char> = text.chars().collect();
    let mut pairs: Vec<String> = Vec::new();
    for (i, ch) in chars.iter().enumerate() {
        if *ch != '±' {
            continue;
        }
        let mut a = i;
        while a > 0 && chars[a - 1] == ' ' {
            a -= 1;
        }
        while a > 0 && (chars[a - 1].is_ascii_digit() || chars[a - 1] == '.') {
            a -= 1;
        }
        let mut b = i + 1;
        while b < chars.len() && chars[b] == ' ' {
            b += 1;
        }
        while b < chars.len() && (chars[b].is_ascii_digit() || chars[b] == '.') {
            b += 1;
        }
        let before: String = chars[a..i].iter().collect::<String>().trim().to_string();
        let after: String = chars[i + 1..b]
            .iter()
            .collect::<String>()
            .trim()
            .to_string();
        let dotted = before.contains('.') || after.contains('.');
        if !before.is_empty() && !after.is_empty() && dotted {
            let line = format!("bait value {before}±{after}");
            if !pairs.contains(&line) {
                pairs.push(line);
            }
            if pairs.len() >= 3 {
                break;
            }
        }
    }

    let mut seen = std::collections::HashSet::new();
    vals.retain(|v| seen.insert(v.clone()));
    if vals.is_empty() && pairs.is_empty() {
        return vec!["bait absent — no distinctive number in the prefix".to_string()];
    }
    let n = vals.len();
    let mut out: Vec<String> = pairs.clone();
    if vals.is_empty() {
        return out;
    }
    for i in [0, n / 2, n - 1] {
        out.push(format!("bait value {}", vals[i]));
        if let Ok(f) = vals[i].parse::<f64>() {
            out.push(format!(
                "bait hex-f64 {}",
                f.to_le_bytes()
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<String>()
            ));
        }
    }
    let mut x = (n as u64).wrapping_mul(6364136223846793005).wrapping_add(1);
    x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
    out.push(format!("bait value {}", vals[(x >> 33) as usize % n]));
    out
}

const CHUNK_BYTES: usize = 4096;

const BAIT_TEXT_MAX: usize = 8;

const BAIT_STOP_WORDS: &[&str] = &[
    "the", "and", "for", "with", "from", "that", "this", "these", "those", "are", "was", "were",
    "has", "have", "had", "not", "but", "all", "any", "can", "may", "our", "their", "its", "into",
    "than", "then", "there", "where", "which", "while", "been", "being", "also", "such", "using",
    "used", "use", "per", "via", "of", "in", "on", "at", "by", "an", "as", "is", "be", "to", "we",
    "it", "or", "if", "so", "no", "do", "does", "than", "most", "more", "less", "each", "both",
    "time", "date", "value", "values", "data", "dataset", "example", "figure", "table",
];

fn ascii_view(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| {
            if (0x20..0x7f).contains(b) {
                *b as char
            } else {
                ' '
            }
        })
        .collect()
}

fn word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn word_token(text: &str, key: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut from = 0usize;
    while let Some(pos) = text[from..].find(key) {
        let start = from + pos;
        let end = start + key.len();
        let before_ok = start == 0 || !word_byte(bytes[start - 1]);
        let after_ok = end >= bytes.len() || !word_byte(bytes[end]);
        if before_ok && after_ok {
            let rest = text[end..].trim_start();
            let rest = rest
                .strip_prefix('=')
                .or_else(|| rest.strip_prefix(':'))
                .unwrap_or(rest)
                .trim_start();
            let token: String = rest
                .chars()
                .take_while(|c| !c.is_whitespace() && !matches!(c, ',' | ';' | ')' | ']'))
                .collect();
            let token = token.trim_matches(['"', '\'']).to_string();
            if !token.is_empty() {
                return Some(token);
            }
        }
        from = end;
    }
    None
}

fn numeric_token(text: &str, key: &str) -> Option<f64> {
    let token = word_token(text, key)?;
    let clean = token.trim_end_matches(['f', 'F', 'd', 'D']);
    clean.parse::<f64>().ok().filter(|v| v.is_finite())
}

const HDF5_MAGIC: &[u8] = b"\x89HDF\r\n\x1a\n";

enum Hdf5Number {
    Int(i128),
    F32(f32),
    F64(f64),
}

impl Hdf5Number {
    fn text(&self) -> String {
        match self {
            Hdf5Number::Int(v) => v.to_string(),
            Hdf5Number::F32(v) => format!("{v}"),
            Hdf5Number::F64(v) => format!("{v}"),
        }
    }

    fn finite(&self) -> bool {
        match self {
            Hdf5Number::Int(_) => true,
            Hdf5Number::F32(v) => v.is_finite(),
            Hdf5Number::F64(v) => v.is_finite(),
        }
    }

    fn is_one(&self) -> bool {
        match self {
            Hdf5Number::Int(v) => *v == 1,
            Hdf5Number::F32(v) => *v == 1.0,
            Hdf5Number::F64(v) => *v == 1.0,
        }
    }
}

fn hdf5_number(
    datatype: &omegaflow::archivar::hdf5::Hdf5Datatype,
    data: &[u8],
) -> Option<Hdf5Number> {
    use omegaflow::archivar::hdf5::Endian;
    match datatype.class {
        0 => {
            let b = data.get(..datatype.size)?;
            let unsigned = match datatype.size {
                1 => b[0] as u128,
                2 => {
                    let a: [u8; 2] = b.try_into().ok()?;
                    match datatype.endian {
                        Endian::Le => u16::from_le_bytes(a) as u128,
                        Endian::Be => u16::from_be_bytes(a) as u128,
                    }
                }
                4 => {
                    let a: [u8; 4] = b.try_into().ok()?;
                    match datatype.endian {
                        Endian::Le => u32::from_le_bytes(a) as u128,
                        Endian::Be => u32::from_be_bytes(a) as u128,
                    }
                }
                8 => {
                    let a: [u8; 8] = b.try_into().ok()?;
                    match datatype.endian {
                        Endian::Le => u64::from_le_bytes(a) as u128,
                        Endian::Be => u64::from_be_bytes(a) as u128,
                    }
                }
                _ => return None,
            };
            let value = if datatype.signed {
                match datatype.size {
                    1 => unsigned as u8 as i8 as i128,
                    2 => unsigned as u16 as i16 as i128,
                    4 => unsigned as u32 as i32 as i128,
                    8 => unsigned as u64 as i64 as i128,
                    _ => return None,
                }
            } else {
                unsigned as i128
            };
            Some(Hdf5Number::Int(value))
        }
        1 => {
            match datatype.size {
                4 => omegaflow::archivar::hdf5::decode_f32(data, 0, datatype.endian)
                    .map(Hdf5Number::F32),
                8 => omegaflow::archivar::hdf5::decode_f64(data, 0, datatype.endian)
                    .map(Hdf5Number::F64),
                _ => None,
            }
        }
        _ => None,
    }
}

fn hdf5_text(attr: &omegaflow::archivar::hdf5::Hdf5Attribute) -> Option<String> {
    if !(attr.datatype.class == 3 || (attr.datatype.class == 9 && attr.datatype.vlen_is_string)) {
        return None;
    }
    let mut bytes = attr.data.clone();
    while bytes.last().is_some_and(|b| *b == 0 || *b == b' ') {
        bytes.pop();
    }
    let text = String::from_utf8(bytes).ok()?;
    let text = text.trim().to_string();
    if text.is_empty() { None } else { Some(text) }
}

fn hdf5_attr_number(attr: &omegaflow::archivar::hdf5::Hdf5Attribute) -> Option<Hdf5Number> {
    hdf5_number(&attr.datatype, &attr.data).filter(Hdf5Number::finite)
}

fn netcdf4_signal_lines(file: &omegaflow::archivar::hdf5::Hdf5File) -> Vec<String> {
    let root = match file.root() {
        Ok(root) => root,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    let mut unique: Vec<(&str, Vec<String>)> = vec![
        ("units", Vec::new()),
        ("scale_factor", Vec::new()),
        ("add_offset", Vec::new()),
        ("_FillValue", Vec::new()),
    ];
    let mut decoded = false;
    for link in &root.links {
        if link.addr == u64::MAX {
            continue;
        }
        let object = match file.resolve(&link.name) {
            Ok(object) => object,
            Err(_) => continue,
        };
        if object.dataspace.is_none() || object.datatype.is_none() {
            continue;
        }
        let mut parts = Vec::new();
        for key in ["units", "scale_factor", "add_offset", "_FillValue"] {
            let attr = match object.attrs.iter().find(|a| a.name == key) {
                Some(attr) => attr,
                None => continue,
            };
            let value = match key {
                "units" => hdf5_text(attr),
                _ => hdf5_attr_number(attr).map(|n| n.text()),
            };
            let Some(value) = value else {
                continue;
            };
            if key == "scale_factor" {
                if let Some(number) = hdf5_attr_number(attr)
                    && !number.is_one()
                {
                    decoded = true;
                }
            }
            if let Some((_, seen)) = unique.iter_mut().find(|(name, _)| *name == key)
                && !seen.contains(&value)
            {
                seen.push(value.clone());
            }
            parts.push(format!("{key}={value}"));
        }
        if !parts.is_empty() {
            out.push(format!("netcdf {} {}", link.name, parts.join(" ")));
        }
    }
    for (key, values) in unique {
        if values.len() == 1 {
            out.push(format!("netcdf {key}={}", values[0]));
        }
    }
    if decoded {
        out.push("netcdf decoded_value=raw*scale+offset".to_string());
    }
    out
}

const NETCDF_VALUE_LIMIT: usize = 10;

fn hdf5_type_text(datatype: &omegaflow::archivar::hdf5::Hdf5Datatype) -> String {
    match datatype.class {
        0 => format!(
            "{}{}",
            if datatype.signed { "int" } else { "uint" },
            datatype.size * 8
        ),
        1 => match datatype.size {
            4 => "float32".to_string(),
            8 => "float64".to_string(),
            other => format!("float{}", other * 8),
        },
        other => format!("class{}/{}", other, datatype.size),
    }
}

fn hdf5_number_f64(number: &Hdf5Number) -> Option<f64> {
    let value = match number {
        Hdf5Number::Int(v) => *v as f64,
        Hdf5Number::F32(v) => *v as f64,
        Hdf5Number::F64(v) => *v,
    };
    value.is_finite().then_some(value)
}

fn hdf5_numbers_equal(a: &Hdf5Number, b: &Hdf5Number) -> bool {
    match (a, b) {
        (Hdf5Number::Int(x), Hdf5Number::Int(y)) => x == y,
        _ => match (hdf5_number_f64(a), hdf5_number_f64(b)) {
            (Some(x), Some(y)) => x == y,
            _ => false,
        },
    }
}

fn hdf5_option_text(number: Option<&Hdf5Number>) -> String {
    match number {
        Some(value) => value.text(),
        None => "absent".to_string(),
    }
}

fn netcdf_decoded_value(
    raw: &Hdf5Number,
    scale: Option<&Hdf5Number>,
    offset: Option<&Hdf5Number>,
) -> Option<f64> {
    let value = hdf5_number_f64(raw)?;
    let factor = match scale {
        Some(number) => hdf5_number_f64(number)?,
        None => 1.0,
    };
    let scaled = value * factor;
    let decoded = match offset {
        Some(number) => scaled + hdf5_number_f64(number)?,
        None => scaled,
    };
    decoded.is_finite().then_some(decoded)
}

fn netcdf_attribute_number<'a>(
    object: &'a omegaflow::archivar::hdf5::Hdf5Object,
    name: &str,
) -> Option<Hdf5Number> {
    object
        .attrs
        .iter()
        .find(|a| a.name == name)
        .and_then(hdf5_attr_number)
}

fn netcdf_unsigned(object: &omegaflow::archivar::hdf5::Hdf5Object) -> bool {
    object
        .attrs
        .iter()
        .find(|a| a.name == "_Unsigned")
        .and_then(hdf5_text)
        .is_some_and(|text| text.eq_ignore_ascii_case("true"))
}

fn netcdf4_value_lines(
    file: &omegaflow::archivar::hdf5::Hdf5File,
    variable: &str,
    limit: usize,
) -> Vec<String> {
    let (object, dataspace, datatype) = match file.dataset(variable) {
        Ok(parts) => parts,
        Err(_) => {
            return vec![format!(
                "absent — the netcdf file carries no dataset {variable}"
            )];
        }
    };
    let raw = match file.read_dataset(variable) {
        Ok(bytes) => bytes,
        Err(_) => {
            return vec![format!(
                "absent — the dataset {variable} payload stays unread"
            )];
        }
    };
    if datatype.size == 0 {
        return vec![format!(
            "absent — the dataset {variable} carries no readable element size"
        )];
    }
    let mut effective = datatype.clone();
    if netcdf_unsigned(object) && effective.class == 0 {
        effective.signed = false;
    }
    let scale = netcdf_attribute_number(object, "scale_factor");
    let offset = netcdf_attribute_number(object, "add_offset");
    let fill = object
        .attrs
        .iter()
        .find(|a| a.name == "_FillValue")
        .and_then(|attr| {
            if attr.datatype.class == 0 {
                hdf5_number(&effective, &attr.data).filter(Hdf5Number::finite)
            } else {
                hdf5_attr_number(attr)
            }
        });
    let total: usize = dataspace
        .dims
        .iter()
        .fold(1usize, |acc, dim| acc.saturating_mul(*dim as usize));
    let dims = if dataspace.dims.is_empty() {
        "scalar".to_string()
    } else {
        dataspace
            .dims
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join("x")
    };
    let mut out = vec![
        format!(
            "netcdf values variable={} dims={} type={} order=row-major",
            variable,
            dims,
            hdf5_type_text(&effective)
        ),
        format!(
            "netcdf values scale={} offset={} fill={}",
            hdf5_option_text(scale.as_ref()),
            hdf5_option_text(offset.as_ref()),
            hdf5_option_text(fill.as_ref())
        ),
    ];
    let mut shown = 0usize;
    let mut decoded = 0usize;
    let mut skipped = 0usize;
    for index in 0..total {
        let start = index.saturating_mul(effective.size);
        let Some(element) = raw.get(start..start + effective.size) else {
            out.push(format!(
                "absent — element {} lies beyond the {}-byte payload",
                index,
                raw.len()
            ));
            break;
        };
        let Some(number) = hdf5_number(&effective, element) else {
            out.push(format!(
                "absent — element {} carries no numeric {}",
                index,
                hdf5_type_text(&effective)
            ));
            break;
        };
        if let Some(fill_number) = fill.as_ref()
            && hdf5_numbers_equal(&number, fill_number)
        {
            skipped += 1;
            continue;
        }
        let Some(value) = netcdf_decoded_value(&number, scale.as_ref(), offset.as_ref()) else {
            out.push(format!(
                "absent — element {} decodes to no finite value",
                index
            ));
            continue;
        };
        decoded += 1;
        if shown < limit {
            out.push(format!(
                "netcdf value {} raw={} scale={} offset={} decoded={}",
                index,
                number.text(),
                hdf5_option_text(scale.as_ref()),
                hdf5_option_text(offset.as_ref()),
                value
            ));
            shown += 1;
        }
    }
    out.push(format!(
        "netcdf values shown={} decoded={} skipped_fill={} total={}",
        shown, decoded, skipped, total
    ));
    out
}

fn netcdf_value_lines(bytes: &[u8], variable: &str, limit: usize) -> Vec<String> {
    if !bytes.starts_with(HDF5_MAGIC) {
        return vec![format!(
            "absent — {variable}: the bytes are not an HDF5/NetCDF4 container"
        )];
    }
    match omegaflow::archivar::hdf5::Hdf5File::parse(bytes) {
        Ok(file) => netcdf4_value_lines(&file, variable, limit),
        Err(_) => vec![format!(
            "absent — {variable}: the HDF5 container stays unread"
        )],
    }
}

fn netcdf_signal_lines(bytes: &[u8], text: &str) -> Vec<String> {
    if bytes.starts_with(HDF5_MAGIC) {
        if let Ok(file) = omegaflow::archivar::hdf5::Hdf5File::parse(bytes) {
            return netcdf4_signal_lines(&file);
        }
    }
    let magic = bytes.starts_with(b"CDF\x01") || bytes.starts_with(b"CDF\x02");
    let numeric = ["scale_factor", "add_offset", "_FillValue"]
        .iter()
        .filter_map(|key| numeric_token(text, key).map(|v| (*key, v)))
        .collect::<Vec<_>>();
    if !magic && numeric.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for (key, value) in &numeric {
        out.push(format!("netcdf {key}={value}"));
    }
    if let Some(units) = word_token(text, "units").filter(|u| u.len() <= 32) {
        out.push(format!("netcdf units={units}"));
    }
    if let Some((_, scale)) = numeric.iter().find(|(key, _)| *key == "scale_factor")
        && *scale != 1.0
    {
        out.push("netcdf decoded_value=raw*scale+offset".to_string());
    }
    if magic && let Ok(f) = omegaflow::archivar::netcdf::NetcdfFile::parse(bytes) {
        let mut units: Vec<String> = Vec::new();
        for a in f
            .gattrs
            .iter()
            .chain(f.vars.iter().flat_map(|v| v.attrs.iter()))
        {
            if a.name.eq_ignore_ascii_case("units") {
                let s = String::from_utf8_lossy(&a.raw)
                    .trim_matches('\0')
                    .trim()
                    .to_string();
                if !s.is_empty() && s.len() <= 32 && !units.contains(&s) {
                    units.push(s);
                }
            }
        }
        if !units.is_empty() && !out.iter().any(|l| l.starts_with("netcdf units=")) {
            out.push(format!("netcdf units={}", units.join(",")));
        }
        out.push(format!(
            "netcdf dims={} vars={}",
            f.dims.len(),
            f.vars.len()
        ));
    }
    out
}

fn fits_like(bytes: &[u8]) -> bool {
    bytes.starts_with(b"SIMPLE  =")
        || bytes.starts_with(b"XTENSION")
        || bytes.starts_with(b"SIMPLE =")
}

fn fits_cards(bytes: &[u8]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if !fits_like(bytes) {
        return out;
    }
    let limit = bytes.len().min(262_144);
    let mut i = 0usize;
    while i + 80 <= limit {
        let card = &bytes[i..i + 80];
        let key = String::from_utf8_lossy(&card[..8]).trim().to_string();
        if key == "END" {
            break;
        }
        let raw = String::from_utf8_lossy(&card[10..]).trim().to_string();
        let value = match raw.strip_prefix('\'') {
            Some(rest) => rest.split('\'').next().unwrap_or("").trim().to_string(),
            None => raw,
        };
        if !key.is_empty() && !value.is_empty() {
            out.push((key, value));
        }
        i += 80;
    }
    out
}

fn fits_signal_lines(bytes: &[u8]) -> Vec<String> {
    let cards = fits_cards(bytes);
    if cards.is_empty() {
        return Vec::new();
    }
    let mut parts = Vec::new();
    for key in [
        "TELESCOP", "INSTRUME", "DETECTOR", "FILTER", "BAND", "EXPTIME", "BUNIT", "BSCALE",
        "BZERO", "NAXIS",
    ] {
        if let Some((_, value)) = cards.iter().find(|(k, _)| k == key) {
            parts.push(format!("{key}={value}"));
        }
    }
    let mut axis: Vec<&(String, String)> = cards
        .iter()
        .filter(|(k, _)| {
            k.len() > 5 && k.starts_with("NAXIS") && k[5..].chars().all(|c| c.is_ascii_digit())
        })
        .collect();
    axis.sort_by_key(|(k, _)| match k[5..].parse::<usize>() {
        Ok(n) => n,
        Err(_) => usize::MAX,
    });
    for (key, value) in axis {
        parts.push(format!("{key}={value}"));
    }
    if parts.is_empty() {
        return Vec::new();
    }
    vec![format!("fits {}", parts.join(" "))]
}

fn healpix_signal_lines(text: &str) -> Vec<String> {
    let upper = text.to_ascii_uppercase();
    let ordering =
        word_token(&upper, "ORDERING").filter(|v| matches!(v.as_str(), "NESTED" | "RING"));
    let nside = word_token(&upper, "NSIDE").and_then(|v| v.parse::<u64>().ok());
    match (ordering, nside) {
        (None, None) => vec!["healpix absent".to_string()],
        (ordering, nside) => {
            let mut line = "healpix".to_string();
            if let Some(ordering) = ordering {
                line.push_str(&format!(" ordering={ordering}"));
            }
            if let Some(nside) = nside {
                line.push_str(&format!(" nside={nside}"));
            }
            vec![line]
        }
    }
}

fn format_signal_lines(bytes: &[u8]) -> Vec<String> {
    let sample = &bytes[..bytes.len().min(262_144)];
    let text = ascii_view(sample);
    let mut out = netcdf_signal_lines(sample, &text);
    out.extend(fits_signal_lines(sample));
    out.extend(healpix_signal_lines(&text));
    out
}

fn chunk_hash_lines(bytes: &[u8], pattern: Option<&str>) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut push_chunk = |start: usize, end: usize, out: &mut Vec<String>| {
        if start >= end || end > bytes.len() || !seen.insert((start, end)) {
            return;
        }
        out.push(format!(
            "chunk {}..{} sha256 {}",
            start,
            end,
            omegaflow::sha256::sha256_hex(&bytes[start..end])
        ));
    };
    push_chunk(0, bytes.len().min(CHUNK_BYTES), &mut out);
    let tail = bytes.len().saturating_sub(CHUNK_BYTES);
    if tail > 0 {
        push_chunk(tail, bytes.len(), &mut out);
    }
    if let Some(hex) = pattern
        && let Some(needle) = hex_bytes(hex)
        && !needle.is_empty()
    {
        for hit in omegaflow::archivar::sniff::find_pattern(bytes, &needle, 4) {
            let start = hit.saturating_sub(CHUNK_BYTES / 2);
            let end = (hit + needle.len() + CHUNK_BYTES / 2).min(bytes.len());
            push_chunk(start, end, &mut out);
        }
    }
    out
}

fn bait_text_lines(bytes: &[u8]) -> Vec<String> {
    let sample = &bytes[..bytes.len().min(65_536)];
    let Ok(text) = std::str::from_utf8(sample) else {
        return Vec::new();
    };
    if text.contains('\0') || !text.chars().any(|c| c.is_alphabetic()) {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    for (i, raw) in words.iter().enumerate() {
        let word = raw.trim_matches(|c: char| {
            !c.is_alphanumeric()
                && !matches!(c, '_' | '[' | ']' | '/' | '^' | '*' | '·' | '°' | '%' | '-')
        });
        if word.is_empty() {
            continue;
        }
        let lower = word.to_ascii_lowercase();
        let identifier = word.len() >= 3
            && word.len() <= 64
            && word.contains('_')
            && word.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
            && word.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        if identifier && !BAIT_STOP_WORDS.contains(&lower.as_str()) && seen.insert(word.to_string())
        {
            out.push(format!("bait text \"{word}\""));
        }
        if let Some(next) = words.get(i + 1) {
            let unit = next.trim_matches(|c: char| {
                matches!(c, ',' | ';' | ':' | '.' | '"' | '\'' | '(' | ')')
            });
            let unit_ok = unit.starts_with('[') && unit.ends_with(']') && unit.len() > 2;
            let word_ok = word.chars().any(|c| c.is_alphabetic())
                && !BAIT_STOP_WORDS.contains(&lower.as_str());
            if unit_ok && word_ok {
                let token = format!("{word} {unit}");
                if seen.insert(token.clone()) {
                    out.push(format!("bait text \"{token}\""));
                }
            }
        }
        if out.len() >= BAIT_TEXT_MAX {
            break;
        }
    }
    out
}

fn bait_lines(bytes: &[u8]) -> Vec<String> {
    let s = omegaflow::archivar::sniff::sniff(bytes);
    let Some(header) = s.header.clone() else {
        let text = bait_text_lines(bytes);
        return if text.is_empty() {
            vec!["bait absent — the prefix carries no header line to fish with".to_string()]
        } else {
            text
        };
    };
    if header.is_empty()
        || header
            .iter()
            .all(|c| !c.chars().any(|ch| ch.is_alphanumeric()))
    {
        let text = bait_text_lines(bytes);
        return if text.is_empty() {
            vec!["bait absent — the prefix carries no header line to fish with".to_string()]
        } else {
            text
        };
    }
    let prose = header.len() > 12
        || (s.delimiter.is_none() && header.len() > 3)
        || header.iter().any(|c| {
            c.contains(' ')
                || c.ends_with('.')
                || c.ends_with(':')
                || c.ends_with(';')
                || c.ends_with('!')
                || c.ends_with('?')
        });
    if prose {
        return bait_text_lines(bytes);
    }
    let generic = [
        "time",
        "date",
        "value",
        "id",
        "lat",
        "lon",
        "latitude",
        "longitude",
        "year",
    ];
    let mut rare: Vec<String> = header
        .iter()
        .filter(|c| !generic.contains(&c.to_ascii_lowercase().as_str()))
        .take(3)
        .cloned()
        .collect();
    if rare.is_empty() {
        rare = header.iter().take(3).cloned().collect();
    }
    let full = header.join(",");
    let phrases = rare
        .iter()
        .map(|c| format!("\"{c}\""))
        .collect::<Vec<_>>()
        .join(" ");
    let mut out = vec![
        format!("bait header {full}"),
        format!("bait phrase {phrases}"),
        format!("bait grepapp {full}"),
        format!("bait searxng {phrases}"),
    ];
    if let Some(u) = s.unit.clone() {
        out.push(format!("bait unit \"{u}\""));
    }
    out
}

fn sniff_struct_lines(bytes: &[u8], pattern: Option<&str>) -> Vec<String> {
    let s = omegaflow::archivar::sniff::sniff(bytes);
    let mut out = vec![
        format!("format {}", s.format),
        format!(
            "delimiter {}",
            match s.delimiter {
                Some(c) => c.to_string(),
                None => String::from("absent"),
            }
        ),
        format!(
            "header {}",
            match s.header {
                Some(h) => h.join(","),
                None => String::from("absent"),
            }
        ),
        format!("columns {}", s.columns.len()),
        format!(
            "time_column {}",
            match s.time_column {
                Some(i) => i.to_string(),
                None => String::from("absent"),
            }
        ),
        format!(
            "unit {}",
            match s.unit {
                Some(u) => u,
                None => String::from("absent"),
            }
        ),
        format!(
            "magic {}",
            match s.magic {
                Some(m) => m,
                None => String::from("absent"),
            }
        ),
    ];
    out.extend(format_signal_lines(bytes));
    if let Some(hex) = pattern {
        match hex_bytes(hex) {
            Some(needle) if !needle.is_empty() => {
                let hits = omegaflow::archivar::sniff::find_pattern(bytes, &needle, 16);
                out.push(format!(
                    "pattern {} hits {}",
                    hex,
                    hits.iter()
                        .map(|o| o.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ));
            }
            _ => out.push(format!("pattern {hex} absent — not hex")),
        }
    }
    out.extend(chunk_hash_lines(bytes, pattern));
    out
}

fn sniff_lines_from(f: &Fetch, url: &str, pattern: Option<&str>) -> Vec<String> {
    let bytes = f.raw.as_slice();
    let mut out = vec![
        format!("url {}", url),
        format!("status {}", f.status_text()),
        format!("bytes {}", bytes.len()),
    ];
    out.extend(sniff_struct_lines(bytes, pattern));
    out.push(if f.complete {
        format!("sha256 {}", omegaflow::sha256::sha256_hex(bytes))
    } else {
        format!(
            "sha256 partial {} (download incomplete, {} bytes — fields read from the prefix)",
            omegaflow::sha256::sha256_hex(bytes),
            bytes.len()
        )
    });
    out
}

struct SniffRequest<'a> {
    target: &'a str,
    pattern: Option<&'a str>,
    bait: bool,
    variable: Option<&'a str>,
}

fn sniff_request(query: &str) -> Option<SniffRequest<'_>> {
    let mut target = None;
    let mut pattern = None;
    let mut bait = false;
    let mut variable = None;
    let mut tokens = query.split_whitespace().peekable();
    while let Some(token) = tokens.next() {
        match token {
            "bait" => bait = true,
            "values" | "--values" => {
                if let Some(name) = tokens.peek()
                    && *name != "bait"
                    && !name.starts_with("pattern=")
                    && !name.starts_with("--")
                {
                    variable = Some(*name);
                    tokens.next();
                }
            }
            _ => {
                if let Some(name) = token.strip_prefix("--values=") {
                    variable = Some(name);
                } else if let Some(name) = token.strip_prefix("pattern=") {
                    pattern = Some(name);
                } else if target.is_none() {
                    target = Some(token);
                }
            }
        }
    }
    Some(SniffRequest {
        target: target?,
        pattern,
        bait,
        variable,
    })
}

pub fn sniff_lines(query: &str) -> Vec<String> {
    let Some(request) = sniff_request(query) else {
        return vec!["absent — --sniff needs a path or URL".to_string()];
    };
    let target = request.target;
    if !target.starts_with("http://")
        && !target.starts_with("https://")
        && !target.starts_with("s3://")
    {
        let mut out = local_sniff_lines(target, request.pattern);
        if let Ok(bytes) = std::fs::read(target) {
            if request.bait {
                out.extend(bait_lines(&bytes));
                if query.split_whitespace().any(|p| p == "values") {
                    out.extend(value_bait_lines(&bytes));
                }
            }
            if let Some(variable) = request.variable {
                out.extend(netcdf_value_lines(&bytes, variable, NETCDF_VALUE_LIMIT));
            }
        }
        return out;
    }
    match get(target, &[], "40") {
        Some(f) => {
            let mut out = sniff_lines_from(&f, target, request.pattern);
            if request.bait {
                out.extend(bait_lines(f.raw.as_slice()));
                if query.split_whitespace().any(|p| p == "values") {
                    out.extend(value_bait_lines(f.raw.as_slice()));
                }
            }
            if let Some(variable) = request.variable {
                out.extend(netcdf_value_lines(
                    f.raw.as_slice(),
                    variable,
                    NETCDF_VALUE_LIMIT,
                ));
            }
            out
        }
        None => vec!["pending — no network".to_string()],
    }
}

fn local_sniff_lines(path: &str, pattern: Option<&str>) -> Vec<String> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => return vec![format!("absent — {path} unreadable: {e}")],
    };
    let mut out = vec![format!("path {path}"), format!("bytes {}", bytes.len())];
    out.extend(sniff_struct_lines(&bytes, pattern));
    out
}

fn hex_bytes(hex: &str) -> Option<Vec<u8>> {
    let clean: String = hex.chars().filter(|c| !c.is_whitespace()).collect();
    if clean.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(clean.len() / 2);
    let chars: Vec<char> = clean.chars().collect();
    for pair in chars.chunks(2) {
        let s: String = pair.iter().collect();
        out.push(u8::from_str_radix(&s, 16).ok()?);
    }
    Some(out)
}

const HAPI_CATALOG: &str = "https://cdaweb.gsfc.nasa.gov/hapi/catalog";
const HAPI_INFO: &str = "https://cdaweb.gsfc.nasa.gov/hapi/info?id=";
const HAPI_DATA: &str = "https://cdaweb.gsfc.nasa.gov/hapi/data?id=";
const HAPI_UNIT_CAP: usize = 6;

struct HapiEntry {
    id: String,
    title: Option<String>,
    start: Option<String>,
    stop: Option<String>,
    params: Option<usize>,
    units: Vec<String>,
}

fn hapi_text(v: &Json, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn hapi_units(parameters: &Json) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let Some(arr) = parameters.as_arr() else {
        return out;
    };
    for p in arr {
        let Some(units) = p.get("units").and_then(|u| u.as_str()) else {
            continue;
        };
        let units = units.trim();
        if units.is_empty() || out.iter().any(|u| u == units) {
            continue;
        }
        out.push(units.to_string());
        if out.len() >= HAPI_UNIT_CAP {
            break;
        }
    }
    out
}

fn hapi_entry(v: &Json) -> Option<HapiEntry> {
    let id = hapi_text(v, "id")?;
    let params = v.get("parameters").and_then(|p| p.as_arr());
    let units = match v.get("parameters") {
        Some(p) => hapi_units(p),
        None => Vec::new(),
    };
    Some(HapiEntry {
        id,
        title: hapi_text(v, "title"),
        start: hapi_text(v, "startDate"),
        stop: hapi_text(v, "stopDate"),
        params: params.map(|p| p.len()),
        units,
    })
}

fn hapi_matches(catalog: &Json, term: &str, max: usize) -> Vec<HapiEntry> {
    let needle = term.to_lowercase();
    let mut out = Vec::new();
    let Some(rows) = catalog.get("catalog").and_then(|c| c.as_arr()) else {
        return out;
    };
    for row in rows {
        let Some(entry) = hapi_entry(row) else {
            continue;
        };
        let id = entry.id.to_lowercase();
        let title = entry.title.as_deref().unwrap_or("").to_lowercase();
        if id.contains(&needle) || title.contains(&needle) {
            out.push(entry);
            if out.len() >= max {
                break;
            }
        }
    }
    out
}

fn hapi_merge_info(entry: &mut HapiEntry, info: &Json) {
    if entry.title.is_none() {
        entry.title = hapi_text(info, "title");
    }
    if entry.start.is_none() {
        entry.start = hapi_text(info, "startDate");
    }
    if entry.stop.is_none() {
        entry.stop = hapi_text(info, "stopDate");
    }
    if entry.params.is_none() {
        if let Some(params) = info.get("parameters").and_then(|p| p.as_arr()) {
            entry.params = Some(params.len());
        }
    }
    if entry.units.is_empty() {
        entry.units = match info.get("parameters") {
            Some(p) => hapi_units(p),
            None => Vec::new(),
        };
    }
}

fn hapi_line(entry: &HapiEntry) -> String {
    let start = entry.start.as_deref().unwrap_or("");
    let stop = entry.stop.as_deref().unwrap_or("");
    let params = match entry.params {
        Some(n) => n.to_string(),
        None => "absent".to_string(),
    };
    format!(
        "url {}{}&time.min={}&time.max={}\ttitle: {}\tparams: {}\tstart: {}\tstop: {}\tunits: {}",
        HAPI_DATA,
        entry.id,
        start,
        stop,
        entry.title.as_deref().unwrap_or(""),
        params,
        start,
        stop,
        entry.units.join(", ")
    )
}

fn hapi_lines_from<F: FnMut(&str) -> Option<Json>>(
    body: &str,
    query: &str,
    max: usize,
    mut info: F,
) -> Vec<String> {
    let Some(v) = json::parse(body) else {
        return vec!["pending — the hapi response carries no JSON".to_string()];
    };
    let mut entries = hapi_matches(&v, query, max);
    if entries.is_empty() {
        return vec![format!("absent — hapi carries no entry: {}", query)];
    }
    for entry in &mut entries {
        if entry.title.is_none()
            || entry.start.is_none()
            || entry.stop.is_none()
            || entry.params.is_none()
            || entry.units.is_empty()
        {
            if let Some(info) = info(&entry.id) {
                hapi_merge_info(entry, &info);
            }
        }
    }
    entries.iter().map(hapi_line).collect()
}

pub fn hapi_lines(query: &str, max: usize) -> Vec<String> {
    match get(HAPI_CATALOG, &[], "60") {
        Some(f) if f.status == Some(200) => hapi_lines_from(&f.body, query, max, |id| {
            let url = format!("{}{}", HAPI_INFO, id);
            match get(&url, &[], "30") {
                Some(info) if info.status == Some(200) => json::parse(&info.body),
                _ => None,
            }
        }),
        Some(f) => vec![format!("pending — hapi HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

const QUERY_MODES: &[&str] = &[
    "openalex",
    "base",
    "arxiv",
    "crossref",
    "ads",
    "ntrs",
    "wiki",
    "duckduckgo-instant",
    "hn",
    "openverse",
    "nasa-images",
    "artic",
    "github",
    "crates",
    "mwmbl",
    "marginalia",
    "searxng",
    "tavily",
    "exa",
    "linkup",
    "serper",
    "firecrawl",
    "searchapi",
    "serpapi",
    "oeis",
    "hal",
    "wiby",
    "ia-search",
    "ngmdb",
    "rss-bridge",
    "kiwix",
    "scrape",
    "shodan",
    "opencellid",
    "gfw",
    "oapen",
    "regtap",
    "apis",
    "datacite",
    "zenodo",
    "hapi",
    "wayback",
    "cc",
    "pubmed",
    "europepmc",
    "europeana",
    "smithsonian",
    "psychporta",
    "awmf",
    "cochrane",
    "core",
    "materialsproject",
    "semanticscholar",
    "clinicaltrials",
    "openfda",
    "pubchem",
    "uniprot",
    "pdb",
    "chembl",
    "ensembl",
    "doaj",
    "osf",
    "openlibrary",
    "stackexchange",
    "sourcegraph",
    "go",
    "unpaywall",
    "reactome",
    "interpro",
    "alphafold",
    "alphaxiv",
    "alphaxiv-researchers",
    "consensus",
    "perplexity",
    "gemini",
    "wikidata",
    "googlecse",
    "ckan",
    "cmr",
    "dataeuropa",
    "opensn",
    "dryad",
    "frbcat",
    "tap",
    "pangaea",
    "re3data",
    "seanoe",
    "arcgis",
    "dataverse",
    "sparql",
    "copernicus",
    "vires",
    "oai-pmh",
    "stac",
    "erddap",
    "openaire",
    "figshare",
    "simbad",
    "vizier",
    "exoplanet-archive",
    "ned",
    "mast",
    "gaia",
    "horizons",
    "overpass",
    "nominatim",
    "open-meteo",
    "gbif",
    "fdsn",
    "dataone",
    "huggingface",
    "dblp",
    "orcid",
    "ror",
    "opencitations",
    "grepapp",
    "scholexplorer",
    "mendeley",
    "usgs",
    "scopus",
    "geonames",
];

const PARAMETER_MODES: &[&str] = &["tap", "sparql", "oai-pmh", "stac", "erddap"];

const BROWSE_MODES: &[&str] = &["re3data", "copernicus", "vires", "nssdc-auth", "ssdc-auth"];

pub fn is_auth_mode(mode: &str) -> bool {
    matches!(mode, "nssdc-auth" | "ssdc-auth")
}

pub fn keyword_mode_count() -> usize {
    QUERY_MODES.len() - PARAMETER_MODES.len()
}

const DEFAULT_ALL_STOP: usize = 20;
static ALL_STOP: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub fn set_all_stop(n: usize) {
    ALL_STOP.store(n, std::sync::atomic::Ordering::Relaxed);
}

fn all_stop_override() -> Option<usize> {
    match ALL_STOP.load(std::sync::atomic::Ordering::Relaxed) {
        0 => None,
        n => Some(n),
    }
}

fn resolve_stop_threshold(cli: Option<usize>, env_value: Option<&str>) -> usize {
    cli.or_else(|| env_value.and_then(|s| s.trim().parse().ok()))
        .unwrap_or(DEFAULT_ALL_STOP)
}

const UNRANKED_ARM: f64 = 0.05;

fn load_logged_yields() -> HashMap<String, f64> {
    let mut map = HashMap::new();
    let repo = match std::env::var_os("OMEGAFLOW_REPO") {
        Some(r) => r,
        None => return map,
    };
    let body = match std::fs::read_to_string(std::path::PathBuf::from(repo).join("queries.φ")) {
        Ok(b) => b,
        Err(_) => return map,
    };
    let mut runs: HashMap<String, (u64, u64, bool)> = HashMap::new();
    for line in body.lines() {
        let f: Vec<&str> = line.splitn(7, " | ").collect();
        if f.len() < 6 {
            continue;
        }
        let h: u64 = match f[4].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let pending = f.len() >= 7 && f[5].trim() == "pending";
        let e = runs.entry(f[2].trim().to_string()).or_insert((0, 0, false));
        e.0 += 1;
        e.1 += h;
        e.2 = pending;
    }
    for (m, (r, h, pending)) in runs {
        let y = if pending { 0.0 } else { h as f64 / r as f64 };
        map.insert(m, y);
    }
    map
}

fn builtin_yield(mode: &str) -> f64 {
    match mode {
        "zenodo" => 169.0,
        "openalex" => 64.0,
        "linkup" => 44.0,
        "exa" => 36.0,
        "psychporta" => 24.0,
        "semanticscholar" => 20.0,
        "crossref" => 18.0,
        "firecrawl" => 13.0,
        "core" => 13.0,
        "reactome" => 8.5,
        "serper" => 8.1,
        "datacite" => 7.6,
        "re3data" => 7.5,
        "go" => 7.5,
        "arxiv" => 7.2,
        "pangaea" => 6.8,
        "chembl" => 6.7,
        "dataeuropa" => 6.6,
        "dataverse" => 5.9,
        "seanoe" => 5.3,
        "ads" => 4.9,
        "marginalia" => 3.0,
        "sourcegraph" => 2.3,
        "ckan" => 2.2,
        "apis" => 1.8,
        "europepmc" => 1.7,
        "osf" => 1.5,
        "mwmbl" => 1.5,
        "searxng" => 1.4,
        "wiki" => 1.0,
        "stackexchange" => 1.0,
        "awmf" => 1.0,
        "github" => 0.7,
        "scrape" => 0.4,
        "ia-search" => 0.4,
        "doaj" => 0.4,
        "pubmed" => 0.1,
        _ => UNRANKED_ARM,
    }
}

fn all_lines(query: &str, env: &HashMap<String, String>, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut full = Vec::new();
    let stop_threshold = resolve_stop_threshold(
        all_stop_override(),
        std::env::var("OMEGAFLOW_ALL_STOP").ok().as_deref(),
    );
    let min_arms = 6usize;
    let arm_cap = match stop_threshold / min_arms {
        0 => stop_threshold,
        c => c,
    };

    let logged = load_logged_yields();
    let rank = |m: &str| match logged.get(m) {
        Some(y) => *y,
        None => builtin_yield(m),
    };
    let mut modes: Vec<&str> = QUERY_MODES
        .iter()
        .copied()
        .filter(|m| !PARAMETER_MODES.contains(m))
        .collect();
    modes.sort_by(|a, b| {
        rank(b)
            .partial_cmp(&rank(a))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    out.push(format!(
        "=== coverage: {} keyword modes ranked by measured yield; {} parameter-arm(s) held (need endpoint=/server=): {} — stop at {stop_threshold} result lines (--stop <n> raises it; OMEGAFLOW_ALL_STOP overrides) ===",
        modes.len(),
        PARAMETER_MODES.len(),
        PARAMETER_MODES.join(" ")
    ));

    let mut accumulated = 0usize;
    let mut ran = 0usize;
    let mut stopped = false;
    for mode in modes {
        if ran >= min_arms && accumulated >= stop_threshold {
            stopped = true;
            break;
        }
        let lines = run_lines_max(mode, query, env, None, max);
        let n = lines.len();
        accumulated += lines
            .iter()
            .filter(|l| l.contains("] url "))
            .count()
            .min(arm_cap);
        ran += 1;
        full.push(format!("=== {} ({}) ===", mode, n));
        full.extend(lines.iter().cloned());
        out.push(format!(
            "=== {} ({} lines; preview {}) ===",
            mode,
            n,
            n.min(5)
        ));
        out.extend(lines.into_iter().take(5));
    }
    if stopped {
        out.push(format!(
            "=== stop: {accumulated} result line(s) ≥ {stop_threshold} after {ran} arm(s) — the remaining arms are held (pass --stop <n> or raise OMEGAFLOW_ALL_STOP to widen) ==="
        ));
    }
    if let Some(path) = write_full(query, &full) {
        out.push(format!(
            "full: {} lines -> {}  [read the full file to its end — the previewed 5 per source are not the result]",
            full.len(),
            path
        ));
    }
    out
}

fn spill_base(env_dir: Option<std::ffi::OsString>) -> std::path::PathBuf {
    match env_dir {
        Some(dir) => std::path::PathBuf::from(dir),
        None => std::env::temp_dir(),
    }
}

pub(crate) fn spill_dir() -> std::path::PathBuf {
    let env_dir = std::env::var_os("OMEGAFLOW_SPILL_DIR");
    let dir = spill_base(env_dir.clone());
    if env_dir.is_some() {
        let _ = std::fs::create_dir_all(&dir);
    }
    dir
}

fn write_full(query: &str, lines: &[String]) -> Option<String> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let slug: String = query
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .take(40)
        .collect();
    let path = spill_dir().join(format!("omegaflow_all_{stamp}_{slug}.txt"));
    std::fs::write(&path, lines.join("\n")).ok()?;
    Some(path.to_string_lossy().into_owned())
}

fn token_key(top: &str, marker: Option<String>) -> String {
    match marker {
        Some(m) => format!("{} marker {{{m}}}", top),
        None => top.to_string(),
    }
}

pub const DEFAULT_MAX: usize = 25;

pub fn run_lines(
    mode: &str,
    query: &str,
    env: &HashMap<String, String>,
    out: Option<&str>,
) -> Vec<String> {
    run_lines_max(mode, query, env, out, DEFAULT_MAX)
}

pub fn run_lines_max(
    mode: &str,
    query: &str,
    env: &HashMap<String, String>,
    out: Option<&str>,
    max: usize,
) -> Vec<String> {
    let mut lines = run_lines_dispatch(mode, query, env, out, max);
    annotate_declared(&mut lines, env);
    if mode != "all" {
        log_query(mode, query, &lines, max);
    }
    lines
}

fn annotate_declared(lines: &mut [String], env: &HashMap<String, String>) {
    for line in lines.iter_mut() {
        if !line.contains("absent from .secrets.local") {
            continue;
        }
        if line.contains("the gate is covered") {
            continue;
        }
        let Some(rest) = line.split("pending — ").nth(1) else {
            continue;
        };
        let Some(names) = rest.split(" absent").next() else {
            continue;
        };
        let declared: Vec<&str> = names
            .split('/')
            .map(str::trim)
            .filter(|name| env.get(*name).is_some_and(|value| value.is_empty()))
            .collect();
        if !declared.is_empty() {
            line.push_str(&format!(
                " — declared on the host ({}): the gate is covered, do not ask for the key (bin/secrets_keys lists all)",
                declared.join("/")
            ));
        } else {
            line.push_str(
                " — bin/secrets_keys lists the gates that carry a value; a missing name is the only open gate",
            );
        }
    }
}

fn log_query(mode: &str, query: &str, lines: &[String], max: usize) {
    let Some(dir) = log_dir() else {
        return;
    };
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let stamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => return,
    };
    let session = match std::env::var("OMEGAFLOW_SESSION") {
        Ok(v) => v,
        Err(_) => "-".to_string(),
    };
    let url_lines = lines
        .iter()
        .filter(|l| l.starts_with(&format!("[{mode}] url ")))
        .count();
    let hits = url_lines.min(max);
    let pending = url_lines == 0 && lines.iter().any(|l| l.contains("] pending"));
    let clean: String = query
        .chars()
        .map(|c| {
            if c == '|' || c == '\n' || c == '\r' || c == '\t' {
                ' '
            } else {
                c
            }
        })
        .collect();
    let state = if pending { "pending" } else { "ok" };
    let line = format!(
        "{stamp} | {session} | {mode} | {} | {hits} | {state} | {clean}\n",
        lines.len()
    );
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("queries.φ"))
    {
        use std::io::Write;
        let _ = f.write_all(line.as_bytes());
    }
}

fn run_lines_dispatch(
    mode: &str,
    query: &str,
    env: &HashMap<String, String>,
    out: Option<&str>,
    max: usize,
) -> Vec<String> {
    crate::token::set_secrets(env.clone());
    if mode == "all" {
        if query.trim().is_empty() {
            return vec!["usage — archive_search --all [--stop <n>] <query>".to_string()];
        }
        return all_lines(query, env, max);
    }
    if !BROWSE_MODES.contains(&mode) && query.trim().is_empty() {
        return vec![format!("usage — archive_search --{mode} <query>")];
    }
    let lines = match mode {
        "arxiv" => arxiv_lines(query, max),
        "ads" => {
            let token = resolve_key(
                env.get("NASA_ADS_TOKEN").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => ads_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("NASA_ADS_TOKEN", marker)
                )],
            }
        }
        "ntrs" => ntrs_lines(query, max),
        "wayback" => wayback_lines(query, max),
        "cc" => cc_lines(query, max),
        "crossref" => crossref_lines(query, max),
        "wiki" => wiki_lines(query, max),
        "duckduckgo-instant" => duckduckgo_instant_lines(query, max),
        "github" => {
            let token = resolve_key(
                env.get("GH_SEARCH_TOKEN").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => github_lines(query, &t, max),
                Secret::Absent(marker) => {
                    let mut lines = vec![format!(
                        "absent — {}; the github search runs anonymous",
                        token_key("GH_SEARCH_TOKEN", marker)
                    )];
                    lines.extend(github_lines(query, "", max));
                    lines
                }
            }
        }
        "crates" => crates_lines(query, max),
        "brave" => {
            let token = resolve_key(
                env.get("BRAVE_API_KEY").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => brave_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("BRAVE_API_KEY", marker)
                )],
            }
        }
        "mwmbl" => mwmbl_lines(query, max),
        "marginalia" => marginalia_lines(query, max),
        "searxng" => searxng_lines(query, max),
        "tavily" => {
            let token = resolve_key(
                env.get("TAVILY_API_KEY").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => tavily_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("TAVILY_API_KEY", marker)
                )],
            }
        }
        "exa" => {
            let token = resolve_key(
                env.get("EXA_API_KEY").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => exa_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("EXA_API_KEY", marker)
                )],
            }
        }
        "serper" => {
            let token = resolve_key(
                env.get("SERPER_API_KEY").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => serper_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("SERPER_API_KEY", marker)
                )],
            }
        }
        "firecrawl" => {
            let token = resolve_key(
                env.get("FIRECRAWL_API_KEY")
                    .map(String::as_str)
                    .unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => firecrawl_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("FIRECRAWL_API_KEY", marker)
                )],
            }
        }
        "searchapi" => {
            let token = resolve_key(
                env.get("SEARCHAPI_API_KEY")
                    .map(String::as_str)
                    .unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => searchapi_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("SEARCHAPI_API_KEY", marker)
                )],
            }
        }
        "serpapi" => {
            let token = resolve_key(
                env.get("SERPAPI_API_KEY").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => serpapi_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("SERPAPI_API_KEY", marker)
                )],
            }
        }
        "oeis" => oeis_lines(query, max),
        "hal" => hal_lines(query, max),
        "wiby" => wiby_lines(query, max),
        "ia-search" => ia_search_lines(query, max),
        "ngmdb" => ngmdb_lines(query, max),
        "rss-bridge" => rss_bridge_lines(query, max),
        "kiwix" => kiwix_lines(query, max),
        "scrape" => scrape_lines(query, max),
        "shodan" => {
            let token = resolve_key(
                env.get("SHODAN_API_KEY").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => shodan_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("SHODAN_API_KEY", marker)
                )],
            }
        }
        "opencellid" => {
            let token = resolve_key(
                env.get("OPENCELLID_API_KEY")
                    .map(String::as_str)
                    .unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => opencellid_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("OPENCELLID_API_KEY", marker)
                )],
            }
        }
        "gfw" => {
            let raw = env
                .get("GFW_PI_KEY")
                .or_else(|| env.get("GFW_API_KEY"))
                .map(String::as_str)
                .unwrap_or("");
            let token = resolve_key(raw, env);
            match token {
                Secret::Value(t) => gfw_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("GFW_API_KEY", marker)
                )],
            }
        }
        "oapen" => oapen_lines(query, max),
        "regtap" => regtap_lines(query, max),
        "apis" => apis_lines(query, max),
        "linkup" => {
            let token = resolve_key(
                env.get("LINKUP_API_KEY").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => linkup_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("LINKUP_API_KEY", marker)
                )],
            }
        }
        "alphaxiv" => {
            let token = resolve_key(
                env.get("ALPHAXIV_API_KEY")
                    .map(String::as_str)
                    .unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => crate::alphaxiv::alphaxiv_lines(query, &t, 5),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("ALPHAXIV_API_KEY", marker)
                )],
            }
        }
        "alphaxiv-researchers" => {
            let token = resolve_key(
                env.get("ALPHAXIV_API_KEY")
                    .map(String::as_str)
                    .unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => crate::alphaxiv::alphaxiv_researchers_lines(query, &t, 5),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("ALPHAXIV_API_KEY", marker)
                )],
            }
        }
        "alphaxiv-tools" => {
            let token = resolve_key(
                env.get("ALPHAXIV_API_KEY")
                    .map(String::as_str)
                    .unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => crate::alphaxiv::alphaxiv_tools_lines(&t),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("ALPHAXIV_API_KEY", marker)
                )],
            }
        }
        "alphaxiv-call" => {
            let token = resolve_key(
                env.get("ALPHAXIV_API_KEY")
                    .map(String::as_str)
                    .unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => match out {
                    Some(dir) => match crate::alphaxiv::alphaxiv_call_text(query, &t) {
                        Ok((tool, text)) => {
                            let _ = std::fs::create_dir_all(dir);
                            let epoch = match std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                            {
                                Ok(d) => d.as_secs().to_string(),
                                Err(_) => "unknown".to_string(),
                            };
                            let path = std::path::PathBuf::from(format!(
                                "{dir}/alphaxiv-{tool}-{epoch}.txt"
                            ));
                            match crate::write_fetched(
                                &path,
                                text.as_bytes(),
                                Some(crate::alphaxiv::MCP_URL),
                            ) {
                                Ok(()) => {
                                    vec![format!("alphaxiv-call: wrote {}", path.display())]
                                }
                                Err(e) => vec![format!(
                                    "pending — could not write {}: {e}",
                                    path.display()
                                )],
                            }
                        }
                        Err(e) => e,
                    },
                    None => crate::alphaxiv::alphaxiv_call_lines(query, &t, 200),
                },
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("ALPHAXIV_API_KEY", marker)
                )],
            }
        }
        "datacite" => crate::datacite::datacite_lines(query, max),
        "zenodo" => crate::zenodo::zenodo_lines(query, max),
        "hapi" => hapi_lines(query, max),
        "isc" => crate::isc::isc_lines(query, max),
        "base" => {
            let token = resolve_key(
                env.get("BASE_API_KEY").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => crate::base::base_lines(query, max, &t),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("BASE_API_KEY", marker)
                )],
            }
        }
        "openalex" => crate::openalex::openalex_lines(query, max),
        "pubmed" => crate::pubmed::pubmed_lines(query, max),
        "europepmc" => crate::europepmc::europepmc_lines(query, max),
        "europeana" => {
            let token = resolve_key(
                env.get("EUROPEANA_API_KEY").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => crate::europeana::europeana_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("EUROPEANA_API_KEY", marker)
                )],
            }
        }
        "smithsonian" => {
            let token = resolve_key(
                env.get("USGOV_API_KEY").map(String::as_str).unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => crate::smithsonian::smithsonian_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("USGOV_API_KEY", marker)
                )],
            }
        }
        "psychporta" => crate::psychporta::psychporta_lines(query, max),
        "awmf" => crate::awmf::awmf_lines(query, max),
        "cochrane" => crate::cochrane::cochrane_lines(query, max),
        "cod" => crate::cod::cod_lines(query, max),
        "biomodels" => crate::biomodels::biomodels_lines(query, max),
        "core" => crate::core_api::core_lines(query, max),
        "materialsproject" => crate::materialsproject::materialsproject_lines(query, max),
        "semanticscholar" => crate::semanticscholar::semanticscholar_lines(query, max),
        "clinicaltrials" => crate::clinicaltrials::clinicaltrials_lines(query, max),
        "openfda" => crate::openfda::openfda_lines(query, max),
        "pubchem" => crate::pubchem::pubchem_lines(query),
        "uniprot" => crate::uniprot::uniprot_lines(query, max),
        "pdb" => crate::pdb::pdb_lines(query, max),
        "chembl" => crate::chembl::chembl_lines(query, max),
        "ensembl" => crate::ensembl::ensembl_lines(query, max),
        "entrez" => crate::entrez::entrez_lines(query, max),
        "ena" => crate::ena::ena_lines(query, max),
        "doaj" => crate::doaj::doaj_lines(query, max),
        "osf" => crate::osf::osf_lines(query, max),
        "openlibrary" => crate::openlibrary::openlibrary_lines(query, max),
        "stackexchange" => crate::stackexchange::stackexchange_lines(query, max),
        "sourcegraph" => crate::sourcegraph::sourcegraph_lines(query, max),
        "go" => crate::go::go_lines(query, max),
        "hn" => crate::hn::hn_lines(query, max),
        "openverse" => crate::openverse::openverse_lines(query, max),
        "nasa-images" => crate::nasa_images::nasa_images_lines(query, max),
        "artic" => crate::artic::artic_lines(query, max),
        "unpaywall" => crate::unpaywall::unpaywall_lines(query),
        "reactome" => crate::reactome::reactome_lines(query, max),
        "interpro" => crate::interpro::interpro_lines(query, max),
        "alphafold" => crate::alphafold::alphafold_lines(query, max),
        "supermag" => {
            let user = match resolve_key(
                env.get("SUPERMAG_USER").map(String::as_str).unwrap_or(""),
                env,
            ) {
                Secret::Value(u) if !u.is_empty() => Some(u),
                _ => None,
            };
            crate::supermag::supermag_lines(query, max, user.as_deref())
        }
        "heasarc" => crate::heasarc::heasarc_lines(query, max),
        "nssdc" => crate::nssdc::nssdc_lines(query, max),
        "nssdca" => crate::nssdc::nssdca_lines(query, max),
        "nssdc-auth" => crate::cas_auth::nssdc_auth_lines(env),
        "ssdc-auth" => crate::cas_auth::ssdc_auth_lines(query, env),
        "consensus" => {
            let token = resolve_key(
                env.get("CONSENSUS_API_KEY")
                    .map(String::as_str)
                    .unwrap_or(""),
                env,
            );
            match token {
                Secret::Value(t) => crate::consensus::consensus_lines(query, &t, max),
                Secret::Absent(marker) => vec![format!(
                    "pending — {} absent from .secrets.local/.env",
                    token_key("CONSENSUS_API_KEY", marker)
                )],
            }
        }
        "perplexity" => crate::perplexity::perplexity_lines(query),
        "gemini" => crate::gemini::gemini_lines(query),
        "wikidata" => crate::wikidata::wikidata_lines(query, max),
        "googlecse" => crate::googlecse::googlecse_lines(query, max),
        "ckan" => crate::ckan::ckan_lines(query, max),
        "cmr" => crate::cmr::cmr_lines(query, max),
        "dataeuropa" => crate::dataeuropa::dataeuropa_lines(query, max),
        "opensn" => {
            let token = match resolve_key(
                env.get("GH_SEARCH_TOKEN").map(String::as_str).unwrap_or(""),
                env,
            ) {
                Secret::Value(t) => t,
                Secret::Absent(_) => String::new(),
            };
            crate::opensn::opensn_lines(query, &token, max)
        }
        "dryad" => crate::dryad::dryad_lines(query, max),
        "frbcat" => crate::frbcat::frbcat_lines(query, max),
        "tap" => crate::tap::tap_lines(query, max),
        "pangaea" => crate::pangaea::pangaea_lines(query, max),
        "re3data" => crate::re3data::re3data_lines(query, max),
        "seanoe" => crate::seanoe::seanoe_lines(query, max),
        "arcgis" => crate::arcgis::arcgis_lines(query, max),
        "dataverse" => crate::dataverse::dataverse_lines(query, max),
        "sparql" => crate::sparql::sparql_lines(query, max),
        "copernicus" => crate::copernicus::copernicus_lines(query, max),
        "vires" => crate::vires::vires_lines(query, max),
        "oai-pmh" => crate::oai_pmh::oai_pmh_lines(query, max),
        "stac" => crate::stac::stac_lines(query, max),
        "erddap" => crate::erddap::erddap_lines(query, max),
        "openaire" => crate::openaire::openaire_lines(query, max),
        "figshare" => crate::figshare::figshare_lines(query, max),
        "simbad" => crate::simbad::simbad_lines(query, max),
        "vizier" => crate::vizier::vizier_lines(query, max),
        "exoplanet-archive" => crate::exoplanet_archive::exoplanet_archive_lines(query, max),
        "ned" => crate::ned::ned_lines(query, max),
        "mast" => crate::mast::mast_lines(query, max),
        "gaia" => crate::gaia::gaia_lines(query, max),
        "horizons" => crate::horizons::horizons_lines(query, max),
        "overpass" => crate::overpass::overpass_lines(query, max),
        "nominatim" => crate::nominatim::nominatim_lines(query, max),
        "open-meteo" => crate::open_meteo::open_meteo_lines(query, max),
        "gbif" => crate::gbif::gbif_lines(query, max),
        "fdsn" => crate::fdsn::fdsn_lines(query, max),
        "dataone" => crate::dataone::dataone_lines(query, max),
        "huggingface" => crate::huggingface::huggingface_lines(query, max),
        "dblp" => crate::dblp::dblp_lines(query, max),
        "orcid" => crate::orcid::orcid_lines(query, max),
        "ror" => crate::ror::ror_lines(query, max),
        "opencitations" => crate::opencitations::opencitations_lines(query, max),
        "grepapp" => crate::grepapp::grepapp_lines(query, max),
        "scholexplorer" => crate::scholexplorer::scholexplorer_lines(query, max),
        "mendeley" => crate::mendeley::mendeley_lines(query, max),
        "usgs" => crate::usgs::usgs_lines(query, max),
        "scopus" => crate::scopus::scopus_lines(query, max),
        "geonames" => crate::geonames::geonames_lines(query, max),
        "sniff" => sniff_lines(query),
        "verdict" => verdict_lines(query),
        "wayback-available" => wayback_available_lines(query),
        "wayback-timemap" => wayback_timemap_lines(query),
        other => vec![format!("absent — no mode named {}", other)],
    };
    witnessed(mode, lines)
}

fn witnessed(mode: &str, lines: Vec<String>) -> Vec<String> {
    lines
        .into_iter()
        .map(|line| format!("[{mode}] {line}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn witnessed_tags_each_line_with_its_mode() {
        let lines = vec!["one".to_string(), "two".to_string()];
        assert_eq!(
            witnessed("arxiv", lines),
            vec!["[arxiv] one".to_string(), "[arxiv] two".to_string()]
        );
    }

    #[test]
    fn urlencode_escapes_reserved_bytes() {
        assert_eq!(urlencode("a b/c?d=e"), "a%20b%2Fc%3Fd%3De");
        assert_eq!(urlencode("safe-._~"), "safe-._~");
    }

    #[test]
    fn spill_base_honors_the_override_else_temp_dir() {
        assert_eq!(
            spill_base(Some(std::ffi::OsString::from("/canonical/spill"))),
            std::path::PathBuf::from("/canonical/spill")
        );
        assert_eq!(spill_base(None), std::env::temp_dir());
    }

    #[test]
    fn ngmdb_url_without_a_refine_carries_the_current_url() {
        assert_eq!(
            ngmdb_url("Yosemite", &[], 10),
            "https://ngmdb.usgs.gov/arcgis/rest/services/topoview/ustOverlay/MapServer/0/query?where=map_name+LIKE+%27%25Yosemite%27&outFields=map_name,primary_state,imprint_year,scan_id&f=json&resultRecordCount=10"
        );
    }

    #[test]
    fn ngmdb_url_with_a_state_refine_carries_both_conditions() {
        let refine = vec![("state".to_string(), "CA".to_string())];
        let url = ngmdb_url("California", &refine, 10);
        assert!(url.contains("map_name+LIKE+%27%25California%27"), "{url}");
        assert!(url.contains("+AND+primary_state%3D%27CA%27"), "{url}");
    }

    #[test]
    fn ngmdb_url_uppercases_and_trims_the_state() {
        let refine = vec![("state".to_string(), " ca ".to_string())];
        let url = ngmdb_url("California", &refine, 10);
        assert!(url.contains("primary_state%3D%27CA%27"), "{url}");
    }

    #[test]
    fn ngmdb_url_with_empty_text_and_state_carries_only_the_state() {
        let refine = vec![("state".to_string(), "CA".to_string())];
        let url = ngmdb_url("", &refine, 10);
        assert!(!url.contains("map_name+LIKE"), "{url}");
        assert!(
            url.contains("?where=primary_state%3D%27CA%27&outFields="),
            "{url}"
        );
    }

    #[test]
    fn duckduckgo_instant_answer_abstract_and_related() {
        let body = r#"{"Heading":"Transfer entropy","AbstractText":"abstract body","AbstractURL":"https://example.org/a","Answer":"42","RelatedTopics":[{"Text":"first","FirstURL":"https://example.org/1"},{"Name":"more","Topics":[{"Text":"nested","FirstURL":"https://example.org/2"}]}]}"#;
        let v = json::parse(body).expect("json");
        assert_eq!(
            duckduckgo_instant_entries(&v, 10),
            vec![
                "answer: 42".to_string(),
                "abstract: abstract body\thttps://example.org/a".to_string(),
                "url https://example.org/1\tfirst".to_string(),
                "url https://example.org/2\tnested".to_string(),
            ]
        );
    }

    #[test]
    fn duckduckgo_instant_only_related() {
        let body = r#"{"Heading":"T","AbstractText":"","Answer":"","RelatedTopics":[{"Text":"first","FirstURL":"https://example.org/1"}]}"#;
        let v = json::parse(body).expect("json");
        assert_eq!(
            duckduckgo_instant_entries(&v, 10),
            vec!["url https://example.org/1\tfirst".to_string()]
        );
    }

    #[test]
    fn duckduckgo_instant_empty_carries_no_entry() {
        let body = r#"{"Answer":"","AbstractText":"","RelatedTopics":[]}"#;
        let v = json::parse(body).expect("json");
        assert!(duckduckgo_instant_entries(&v, 10).is_empty());
    }

    #[test]
    fn duckduckgo_instant_missing_fields_are_omitted() {
        let v = json::parse(r#"{"Heading":"T","Answer":"only answer"}"#).expect("json");
        assert_eq!(
            duckduckgo_instant_entries(&v, 10),
            vec!["answer: only answer".to_string()]
        );
    }

    #[test]
    fn wiki_hit_carries_title_and_tag_stripped_snippet() {
        let hit = json::parse(
            r#"{"title":"Transfer entropy","snippet":"the <span class=\"searchmatch\">transfer</span> of\n  information"}"#,
        )
        .expect("json");
        assert_eq!(
            wiki_hit_line(&hit),
            Some(
                "url https://en.wikipedia.org/wiki/Transfer_entropy\ttitle: Transfer entropy\tsnippet: the transfer of information"
                    .to_string()
            )
        );
    }

    #[test]
    fn wiki_hit_omits_absent_snippet() {
        let hit = json::parse(r#"{"title":"Transfer entropy"}"#).expect("json");
        assert_eq!(
            wiki_hit_line(&hit),
            Some(
                "url https://en.wikipedia.org/wiki/Transfer_entropy\ttitle: Transfer entropy"
                    .to_string()
            )
        );
    }

    #[test]
    fn wiki_hit_omits_snippet_that_strips_to_nothing() {
        let hit = json::parse(r#"{"title":"A","snippet":"<span></span>"}"#).expect("json");
        assert_eq!(
            wiki_hit_line(&hit),
            Some("url https://en.wikipedia.org/wiki/A\ttitle: A".to_string())
        );
    }

    #[test]
    fn wiki_hit_without_title_carries_no_line() {
        let hit = json::parse(r#"{"snippet":"body"}"#).expect("json");
        assert_eq!(wiki_hit_line(&hit), None);
    }

    #[test]
    fn query_mode_list_is_the_all_fan_out_set() {
        let mut expected = vec![
            "openalex",
            "base",
            "arxiv",
            "artic",
            "hn",
            "nasa-images",
            "openverse",
            "crossref",
            "ads",
            "ntrs",
            "wiki",
            "duckduckgo-instant",
            "github",
            "crates",
            "mwmbl",
            "marginalia",
            "searxng",
            "tavily",
            "exa",
            "linkup",
            "serper",
            "firecrawl",
            "searchapi",
            "serpapi",
            "oeis",
            "hal",
            "wiby",
            "ia-search",
            "ngmdb",
            "rss-bridge",
            "kiwix",
            "scrape",
            "shodan",
            "opencellid",
            "gfw",
            "oapen",
            "regtap",
            "apis",
            "datacite",
            "zenodo",
            "hapi",
            "wayback",
            "cc",
            "pubmed",
            "europepmc",
            "europeana",
            "smithsonian",
            "psychporta",
            "awmf",
            "cochrane",
            "core",
            "materialsproject",
            "semanticscholar",
            "clinicaltrials",
            "openfda",
            "pubchem",
            "uniprot",
            "pdb",
            "chembl",
            "ensembl",
            "doaj",
            "osf",
            "openlibrary",
            "stackexchange",
            "sourcegraph",
            "go",
            "unpaywall",
            "reactome",
            "interpro",
            "alphafold",
            "alphaxiv",
            "alphaxiv-researchers",
            "consensus",
            "perplexity",
            "gemini",
            "wikidata",
            "googlecse",
            "ckan",
            "cmr",
            "dataeuropa",
            "opensn",
            "dryad",
            "frbcat",
            "tap",
            "pangaea",
            "re3data",
            "seanoe",
            "arcgis",
            "dataverse",
            "sparql",
            "copernicus",
            "vires",
            "oai-pmh",
            "stac",
            "erddap",
            "openaire",
            "figshare",
            "simbad",
            "vizier",
            "exoplanet-archive",
            "ned",
            "mast",
            "gaia",
            "horizons",
            "overpass",
            "nominatim",
            "open-meteo",
            "gbif",
            "fdsn",
            "dataone",
            "huggingface",
            "dblp",
            "orcid",
            "ror",
            "opencitations",
            "grepapp",
            "scholexplorer",
            "mendeley",
            "usgs",
            "scopus",
            "geonames",
        ];
        expected.sort_unstable();
        let mut actual = QUERY_MODES.to_vec();
        actual.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hapi_catalog_filters_id_and_title() {
        let body = r#"{"HAPI":"2.0","catalog":[{"id":"OMNI_HRO_1MIN","title":"OMNI 1-min HRO"},{"id":"AC_H0_MFI","title":"ACE Magnetic Field"}]}"#;
        let v = json::parse(body).expect("json");
        let by_id = hapi_matches(&v, "omni_hro", 10);
        assert_eq!(by_id.len(), 1);
        assert_eq!(by_id[0].id, "OMNI_HRO_1MIN");
        let by_title = hapi_matches(&v, "magnetic", 10);
        assert_eq!(by_title.len(), 1);
        assert_eq!(by_title[0].id, "AC_H0_MFI");
    }

    #[test]
    fn hapi_zero_hits_carries_absent() {
        let body = r#"{"HAPI":"2.0","catalog":[{"id":"OMNI_HRO_1MIN"}]}"#;
        let lines = hapi_lines_from(body, "no-such-dataset", 10, |_| None);
        assert_eq!(
            lines,
            vec!["absent — hapi carries no entry: no-such-dataset".to_string()]
        );
    }

    #[test]
    fn hapi_line_carries_url_window_title_params_and_units() {
        let body = r#"{"HAPI":"2.0","catalog":[{"id":"OMNI_HRO_1MIN","title":"OMNI 1-min HRO"}]}"#;
        let info = r#"{"startDate":"1981-01-01T00:00:00Z","stopDate":"2026-09-30T23:59:00Z","parameters":[{"name":"Time","units":"UTC"},{"name":"BX","units":"nT"},{"name":"BY","units":"nT"},{"name":"V","units":"km/s"},{"name":"A","units":"eV"},{"name":"B","units":"keV"},{"name":"C","units":"MeV"},{"name":"D","units":"pfu"}]}"#;
        let lines = hapi_lines_from(body, "omni", 10, |_| json::parse(info));
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            "url https://cdaweb.gsfc.nasa.gov/hapi/data?id=OMNI_HRO_1MIN&time.min=1981-01-01T00:00:00Z&time.max=2026-09-30T23:59:00Z\ttitle: OMNI 1-min HRO\tparams: 8\tstart: 1981-01-01T00:00:00Z\tstop: 2026-09-30T23:59:00Z\tunits: UTC, nT, km/s, eV, keV, MeV"
        );
    }

    #[test]
    fn parameter_modes_are_a_subset_of_query_modes() {
        for mode in PARAMETER_MODES {
            assert!(
                QUERY_MODES.contains(mode),
                "parameter mode {mode} is not a query mode"
            );
        }
        assert_eq!(
            keyword_mode_count(),
            QUERY_MODES.len() - PARAMETER_MODES.len()
        );
    }

    #[test]
    fn cc_json_lines_carry_url_timestamp_status_mime() {
        let body = concat!(
            r#"{"urlkey":"org,example)/a","timestamp":"20241215000000","url":"https://example.org/a","mime":"text/html","status":"200"}"#,
            "\n",
            r#"{"urlkey":"org,example)/b","timestamp":"20241216000000","url":"https://example.org/b","mime":"application/pdf","status":200}"#,
            "\n",
            "",
        );
        assert_eq!(
            cc_record_lines(body, 10),
            vec![
                "url https://example.org/a\ttimestamp: 20241215000000\tstatus: 200\tmime: text/html"
                    .to_string(),
                "url https://example.org/b\ttimestamp: 20241216000000\tstatus: 200\tmime: application/pdf"
                    .to_string(),
            ]
        );
    }

    #[test]
    fn cc_json_lines_skip_unparsable_records_and_stop_at_max() {
        let body = concat!(
            "not json\n",
            "{\"mime\":\"text/html\"}\n",
            r#"{"url":"https://example.org/a"}"#,
            "\n",
            r#"{"url":"https://example.org/b"}"#,
            "\n",
        );
        let out = cc_record_lines(body, 1);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0], "url https://example.org/a");
    }

    #[test]
    fn cc_index_token_overrides_the_default_and_leaves_the_target() {
        assert_eq!(cc_index("example.org/*"), None);
        assert_eq!(
            cc_index("example.org/* index=CC-MAIN-2023-06"),
            Some("CC-MAIN-2023-06".to_string())
        );
        assert_eq!(cc_index("example.org/* index="), None);
        assert_eq!(
            cc_target("example.org/* index=CC-MAIN-2023-06"),
            "example.org/*"
        );
        assert_eq!(cc_target("example.org/*"), "example.org/*");
        assert_eq!(cc_target("url=example.org/*"), "example.org/*");
    }

    #[test]
    fn cc_target_normalizes_bare_hosts_to_the_subdomain_pattern() {
        assert_eq!(cc_target("example.org"), "*.example.org");
        assert_eq!(cc_target("url=example.org"), "*.example.org");
        assert_eq!(
            cc_target("example.org index=CC-MAIN-2023-06"),
            "*.example.org"
        );
        assert_eq!(cc_target("example.org/path"), "example.org/path");
        assert_eq!(cc_target("url=example.org/path"), "example.org/path");
        assert_eq!(cc_target("*.example.org"), "*.example.org");
    }

    #[test]
    fn mwmbl_results_carry_url_title_source_extract() {
        let body = r#"[
            {"url":"https://en.wikipedia.org/wiki/Interplanetary_scintillation",
             "title":[{"value":"Interplanetary","is_bold":true},{"value":" scintillation","is_bold":false}],
             "extract":[{"value":"In astronomy, "},{"value":"interplanetary scintillation"}],
             "source":"wikipedia"},
            {"url":"","title":[{"value":"no url"}],"extract":[],"source":"x"},
            {"url":"https://example.org/b","title":[{"value":"B"}],"extract":[{"value":"text"}],"source":""}
        ]"#;
        let v = json::parse(body).expect("json");
        let lines = mwmbl_results(&v, 10);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "url https://en.wikipedia.org/wiki/Interplanetary_scintillation\ttitle: Interplanetary scintillation\tsource: wikipedia\tdescription: In astronomy, interplanetary scintillation"
        );
        assert_eq!(
            lines[1],
            "url https://example.org/b\ttitle: B\tdescription: text"
        );
    }

    #[test]
    fn marginalia_html_cards_carry_url_title_and_description() {
        let body = r#"<section data-ms-rank="5" class="card search-result" >
<div class="url"><a rel="nofollow external" href="https://example.org/a">https://example.org/a</a></div>
<h2> <a tabindex="-1" class="title" rel="nofollow external" href="https://example.org/a">A &amp; B</a> </h2>
<p class="description">Some <b>text</b>.</p>
</section>
<section class="card search-result">
<div class="url"><a href="/site/example.org">/site</a></div>
<h2><a class="title" href="/site">Local</a></h2>
</section>"#;
        let lines = marginalia_html_results(body, 10);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "url https://example.org/a\ttitle: A & B\tdescription: Some text."
        );
        assert_eq!(
            lines[1],
            "url https://old-search.marginalia.nu/site/example.org\ttitle: Local"
        );
    }

    #[test]
    fn searxng_results_carry_url_title_content_engine() {
        let body = r#"{
            "query":"interplanetary scintillation",
            "number_of_results":2,
            "results":[
                {"url":"https://en.wikipedia.org/wiki/Interplanetary_scintillation",
                 "title":"Interplanetary scintillation",
                 "content":"In astronomy, interplanetary scintillation",
                 "engine":"wikipedia"},
                {"url":"","title":"no url","content":"x","engine":"duckduckgo"},
                {"url":"https://example.org/b","title":"B","content":"text"}
            ]
        }"#;
        let v = json::parse(body).expect("json");
        let lines = searxng_results(&v, 10);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "url https://en.wikipedia.org/wiki/Interplanetary_scintillation\ttitle: Interplanetary scintillation\tengine: wikipedia\tcontent: In astronomy, interplanetary scintillation"
        );
        assert_eq!(
            lines[1],
            "url https://example.org/b\ttitle: B\tcontent: text"
        );
    }

    #[test]
    fn tavily_results_carry_url_title_score_description() {
        let body = r#"{
            "query":"interplanetary scintillation",
            "results":[
                {"title":"Interplanetary scintillation",
                 "url":"https://en.wikipedia.org/wiki/Interplanetary_scintillation",
                 "content":"In astronomy, interplanetary scintillation",
                 "score":0.75},
                {"title":"no url","url":"","content":"x","score":0.1},
                {"title":"B","url":"https://example.org/b","content":"text","score":"high"}
            ]
        }"#;
        let v = json::parse(body).expect("json");
        let lines = tavily_results(&v, 10);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "url https://en.wikipedia.org/wiki/Interplanetary_scintillation\ttitle: Interplanetary scintillation\tscore: 0.75\tdescription: In astronomy, interplanetary scintillation"
        );
        assert_eq!(
            lines[1],
            "url https://example.org/b\ttitle: B\tscore: high\tdescription: text"
        );
    }

    #[test]
    fn exa_results_carry_url_title_author_published_description() {
        let body = r#"{
            "requestId":"abc",
            "results":[
                {"title":"Interplanetary scintillation",
                 "url":"https://en.wikipedia.org/wiki/Interplanetary_scintillation",
                 "author":"A. Reader",
                 "publishedDate":"2024-01-03",
                 "text":"In astronomy, interplanetary scintillation"},
                {"title":"no url","url":"","author":"x","text":"y"},
                {"title":"B","url":"https://example.org/b","text":""}
            ]
        }"#;
        let v = json::parse(body).expect("json");
        let lines = exa_results(&v, 10);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "url https://en.wikipedia.org/wiki/Interplanetary_scintillation\ttitle: Interplanetary scintillation\tauthor: A. Reader\tpublished: 2024-01-03\tdescription: In astronomy, interplanetary scintillation"
        );
        assert_eq!(lines[1], "url https://example.org/b\ttitle: B");
    }

    #[test]
    fn linkup_results_carry_url_name_content() {
        let body = r#"{
            "results":[
                {"name":"Interplanetary scintillation",
                 "url":"https://en.wikipedia.org/wiki/Interplanetary_scintillation",
                 "content":"In astronomy, interplanetary scintillation"},
                {"name":"no url","url":"","content":"x"},
                {"name":"B","url":"https://example.org/b","content":""}
            ]
        }"#;
        let v = json::parse(body).expect("json");
        let lines = linkup_results(&v, 10);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "url https://en.wikipedia.org/wiki/Interplanetary_scintillation\ttitle: Interplanetary scintillation\tdescription: In astronomy, interplanetary scintillation"
        );
        assert_eq!(lines[1], "url https://example.org/b\ttitle: B");
    }

    #[test]
    fn tavily_content_modes_name_the_answer_and_raw_body_fields() {
        let refined = vec![
            ("answer".to_string(), "advanced".to_string()),
            ("raw".to_string(), "1".to_string()),
        ];
        assert_eq!(
            tavily_body("transfer entropy", 5, "tok", &refined),
            "{\"query\":\"transfer entropy\",\"max_results\":5,\"api_key\":\"tok\",\"include_answer\":\"advanced\",\"include_raw_content\":true}"
        );
    }

    #[test]
    fn a_plain_tavily_body_stays_unchanged() {
        assert_eq!(
            tavily_body("transfer entropy", 5, "tok", &[]),
            "{\"query\":\"transfer entropy\",\"max_results\":5,\"api_key\":\"tok\"}"
        );
    }

    #[test]
    fn tavily_results_carry_raw_content() {
        let body = r#"{"answer":"It holds.","results":[{"title":"B","url":"https://example.org/b","raw_content":"full  text"}]}"#;
        let v = json::parse(body).expect("json");
        assert_eq!(
            tavily_results(&v, 10),
            vec!["url https://example.org/b\ttitle: B\tcontent: full text".to_string()]
        );
    }

    #[test]
    fn exa_content_modes_name_the_contents_object() {
        assert_eq!(
            exa_body("transfer entropy", 5, None),
            "{\"query\":\"transfer entropy\",\"numResults\":5}"
        );
        assert_eq!(
            exa_body("transfer entropy", 5, Some("text")),
            "{\"query\":\"transfer entropy\",\"numResults\":5,\"contents\":{\"text\":true}}"
        );
        assert_eq!(
            exa_body("transfer entropy", 5, Some("highlights")),
            "{\"query\":\"transfer entropy\",\"numResults\":5,\"contents\":{\"highlights\":true}}"
        );
    }

    #[test]
    fn exa_results_carry_highlights_and_summary() {
        let body = r#"{"results":[{"title":"B","url":"https://example.org/b","highlights":["a point"],"summary":"a summary"}]}"#;
        let v = json::parse(body).expect("json");
        assert_eq!(
            exa_results(&v, 10),
            vec![
                "url https://example.org/b\ttitle: B\thighlight: a point\tsummary: a summary"
                    .to_string()
            ]
        );
    }

    #[test]
    fn linkup_output_modes_name_the_output_type() {
        assert_eq!(
            linkup_body("transfer entropy", "searchResults", "standard"),
            "{\"q\":\"transfer entropy\",\"depth\":\"standard\",\"outputType\":\"searchResults\"}"
        );
        assert_eq!(
            linkup_body("transfer entropy", "sourcedAnswer", "deep"),
            "{\"q\":\"transfer entropy\",\"depth\":\"deep\",\"outputType\":\"sourcedAnswer\"}"
        );
        assert_eq!(
            linkup_body("transfer entropy", "structured", "standard"),
            "{\"q\":\"transfer entropy\",\"depth\":\"standard\",\"outputType\":\"searchResults\"}"
        );
    }

    #[test]
    fn linkup_sourced_answer_carries_the_answer_and_sources() {
        let body = r#"{"answer":"It holds.","sources":[{"name":"B","url":"https://example.org/b","snippet":"a snippet"}]}"#;
        let v = json::parse(body).expect("json");
        assert_eq!(
            linkup_output(&v, 10),
            vec![
                "answer: It holds.".to_string(),
                "url https://example.org/b\ttitle: B\tdescription: a snippet".to_string(),
            ]
        );
    }

    #[test]
    fn json_escape_quotes_and_controls() {
        assert_eq!(json_escape("a\"b\\c\nd"), "a\\\"b\\\\c\\nd");
    }

    #[test]
    fn civil_calendar_reads_the_unix_epoch() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(19723), (2024, 1, 1));
    }

    #[test]
    fn atom_parser_carries_the_entry_fields() {
        let xml = "<feed><entry><id>http://arxiv.org/abs/2401.01234v1</id>\
            <title>Transmission spectrum of a warm sub-Neptune</title>\
            <summary>A measured atmosphere with haze.</summary>\
            <author><name>Ada Miller</name></author>\
            <author><name>Bo Chen</name></author>\
            <published>2024-01-03T00:00:00Z</published>\
            <link href=\"http://arxiv.org/pdf/2401.01234v1\" rel=\"related\" title=\"pdf\"/>\
            </entry></feed>";
        let parsed = parse_atom_entries(xml);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].id, "http://arxiv.org/abs/2401.01234v1");
        assert_eq!(parsed[0].authors.as_deref(), Some("Ada Miller; Bo Chen"));
        assert_eq!(
            parsed[0].pdf.as_deref(),
            Some("http://arxiv.org/pdf/2401.01234v1")
        );
    }

    #[test]
    fn arxiv_html_fallback_reads_result_cards() {
        let body = r#"<ol class="results">
<li class="arxiv-result">
  <p class="list-title is-inline-block"><a href="https://arxiv.org/abs/2610.28274">arXiv:2610.28274</a></p>
  <div class="tags"><span class="tag">astro-ph.SR</span></div>
  <p class="title is-5 mathjax">The Effect of Field Line <span class="search-hit">Solar</span> Wind</p>
  <p class="authors"><span>Authors:</span> <a href="/x">Satabdwa Majumdar</a>, <a href="/y">Pete Riley</a></p>
</li>
<li class="arxiv-result">
  <p class="list-title is-inline-block"><a href="https://arxiv.org/abs/2601.00001">arXiv:2601.00001</a></p>
  <p class="title is-5 mathjax">Second</p>
</li>
</ol>"#;
        let lines = parse_arxiv_html(body, 10);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "url https://arxiv.org/abs/2610.28274\ttitle: The Effect of Field Line Solar Wind\tauthors: Satabdwa Majumdar, Pete Riley\tvia: arxiv-html (api 429)"
        );
        assert_eq!(
            lines[1],
            "url https://arxiv.org/abs/2601.00001\ttitle: Second\tvia: arxiv-html (api 429)"
        );
    }

    #[test]
    fn arxiv_window_stays_under_the_edge_cap() {
        assert_eq!(arxiv_window(0), 1);
        assert_eq!(arxiv_window(1), 1);
        assert_eq!(arxiv_window(2), 2);
        assert_eq!(arxiv_window(20), ARXIV_QUERY_WINDOW);
        assert_eq!(arxiv_window(200), ARXIV_QUERY_WINDOW);
    }

    #[test]
    fn cdx_parser_reads_the_first_snapshot() {
        let body = r#"[["timestamp","original","statuscode"],["20200101000000","https://example.com/","200"]]"#;
        assert_eq!(
            first_snapshot(body).as_deref(),
            Some("https://web.archive.org/web/20200101000000/https://example.com/")
        );
        assert!(first_snapshot(r#"[["timestamp"]]"#).is_none());
        let default_cols = r#"[["urlkey","timestamp","original","mimetype","statuscode","digest","length"],["com,example)/","20020120142510","http://example.com:80/","text/html","200","X","0"]]"#;
        assert_eq!(
            first_snapshot(default_cols).as_deref(),
            Some("https://web.archive.org/web/20020120142510/http://example.com:80/")
        );
    }

    #[test]
    fn availability_lines_read_the_closest_snapshot() {
        let body = r#"{"url":"http://example.com","archived_snapshots":{"closest":{"status":"200","available":true,"url":"http://web.archive.org/web/20260925031642/https://example.com/","timestamp":"20260925031642"}}}"#;
        assert_eq!(
            availability_lines_from(body, "http://example.com"),
            vec!["url http://web.archive.org/web/20260925031642/https://example.com/\ttimestamp: 20260925031642\tstatus: 200".to_string()]
        );
    }

    #[test]
    fn availability_lines_report_absent_when_closest_is_empty() {
        let body = r#"{"url":"example.com","archived_snapshots":{}}"#;
        assert_eq!(
            availability_lines_from(body, "example.com"),
            vec!["absent — the availability register carries no snapshot: example.com".to_string()]
        );
    }

    #[test]
    fn timemap_lines_carry_each_memento() {
        let body = r#"[["urlkey","timestamp","original","mimetype","statuscode","digest","length"],["com,example)/","20020120142510","http://example.com:80/","text/html","200","X","1792"],["com,example)/","20020328012821","http://www.example.com:80/","text/html","301","Y","481"]]"#;
        assert_eq!(
            timemap_lines_from(body, "http://example.com"),
            vec![
                "url https://web.archive.org/web/20020120142510/http://example.com:80/ (status 200)".to_string(),
                "url https://web.archive.org/web/20020328012821/http://www.example.com:80/ (status 301)".to_string(),
            ]
        );
    }

    #[test]
    fn socks_bind_reads_the_wireproxy_inbound() {
        assert_eq!(
            socks_bind("[Socks5]\nBindAddress = 127.0.0.1:25344\n").as_deref(),
            Some("127.0.0.1:25344")
        );
        assert!(socks_bind("[Interface]\nPrivateKey = x\n").is_none());
    }

    fn answer(status: Option<i32>) -> Option<Fetch> {
        Some(Fetch {
            status,
            body: String::new(),
            raw: Vec::new(),
            retry_after: None,
            complete: true,
        })
    }

    #[test]
    fn is_block_names_the_walls_not_the_end() {
        assert!(is_block(None));
        assert!(is_block(Some(0)));
        assert!(is_block(Some(403)));
        assert!(is_block(Some(429)));
        assert!(!is_block(Some(404)));
        assert!(!is_block(Some(200)));
        assert!(!is_block(Some(500)));
    }

    #[test]
    fn ladder_rolls_on_block_and_stops_at_the_first_answer() {
        let ladder = vec![
            Exit::Direct,
            Exit::Iface("proton0".to_string()),
            Exit::Socks("socks5h://127.0.0.1:25344".to_string()),
        ];
        let seen = std::cell::RefCell::new(Vec::new());
        let result = first_answer(&ladder, |exit| {
            seen.borrow_mut().push(exit.label());
            match exit.label().as_str() {
                "direct" => answer(Some(403)),
                "proton0" => answer(Some(429)),
                _ => answer(Some(200)),
            }
        });
        assert_eq!(result.unwrap().status, Some(200));
        assert_eq!(
            *seen.borrow(),
            vec![
                "direct".to_string(),
                "proton0".to_string(),
                "socks5h://127.0.0.1:25344".to_string()
            ]
        );
    }

    #[test]
    fn ladder_exhausted_returns_the_last_block_truth() {
        let ladder = vec![Exit::Direct, Exit::Iface("proton0".to_string())];
        let result = first_answer(&ladder, |_| answer(Some(403)));
        assert_eq!(result.unwrap().status, Some(403));
    }

    #[test]
    fn ladder_stops_on_a_plain_absent() {
        let ladder = vec![Exit::Direct, Exit::Iface("proton0".to_string())];
        let calls = std::cell::Cell::new(0usize);
        let result = first_answer(&ladder, |_| {
            calls.set(calls.get() + 1);
            answer(Some(404))
        });
        assert_eq!(result.unwrap().status, Some(404));
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn url_host_reads_the_authority() {
        assert_eq!(url_host("https://example.com/path?q=1"), "example.com");
        assert_eq!(
            url_host("http://api.adsabs.harvard.edu/v1/x"),
            "api.adsabs.harvard.edu"
        );
        assert_eq!(url_host("s3://bucket/key"), "bucket");
    }

    #[test]
    fn rate_gate_backs_off_one_exit_not_the_whole_host() {
        let mut gate = RateGate {
            next_allowed: HashMap::new(),
            blocked_until: HashMap::new(),
            interval: Duration::from_millis(0),
        };
        gate.back_off("api.openalex.org|direct", 3600);
        assert!(gate.blocked("api.openalex.org|direct"));
        assert!(!gate.blocked("api.openalex.org|socks5h://127.0.0.1:25344"));
    }

    #[test]
    fn split_curl_stdout_keeps_raw_body_bytes() {
        let body = [0xFFu8, 0xFE, 0x00, b'a'];
        let mut stdout = body.to_vec();
        stdout.extend_from_slice(b"\n200\n");
        let (raw, code, retry) = split_curl_stdout(&stdout).unwrap();
        assert_eq!(raw, body.as_slice());
        assert_eq!(raw.len(), 4);
        assert_eq!(code, 200);
        assert_eq!(retry, None);
    }

    fn lossy_fetch() -> Fetch {
        let raw = vec![0xFFu8, 0xFE, 0x00, b'a'];
        Fetch {
            status: Some(200),
            body: String::from_utf8_lossy(&raw).to_string(),
            raw,
            retry_after: None,
            complete: true,
        }
    }

    fn partial_fetch() -> Fetch {
        Fetch {
            complete: false,
            ..lossy_fetch()
        }
    }

    #[test]
    fn sniff_lines_marks_a_partial_download() {
        let complete = lossy_fetch();
        let lines = sniff_lines_from(&complete, "https://example.com/blob", None);
        let raw_sha = omegaflow::sha256::sha256_hex(&complete.raw);
        assert!(
            lines.iter().any(|l| *l == format!("sha256 {}", raw_sha)),
            "{lines:?}"
        );

        let partial = partial_fetch();
        let partial_lines = sniff_lines_from(&partial, "https://example.com/blob", None);
        assert!(
            partial_lines
                .iter()
                .any(|l| l.starts_with("sha256 partial ")),
            "{partial_lines:?}"
        );
        assert!(
            partial_lines.iter().any(|l| l.contains(&raw_sha)),
            "{partial_lines:?}"
        );
        assert!(
            partial_lines.iter().any(|l| l == "bytes 4"),
            "{partial_lines:?}"
        );
    }

    #[test]
    fn local_sniff_reads_header_columns_and_time() {
        let path = std::env::temp_dir().join(format!("sniff-local-{}", std::process::id()));
        std::fs::write(
            &path,
            "time;value [s]\n2026-01-01T00:00:00;1.5\n2026-01-01T00:00:01;1.6\n",
        )
        .unwrap();
        let lines = sniff_lines(&path.to_string_lossy());
        assert!(lines.iter().any(|l| l == "format csv"), "{lines:?}");
        assert!(lines.iter().any(|l| l == "delimiter ;"), "{lines:?}");
        assert!(lines.iter().any(|l| l == "columns 2"), "{lines:?}");
        assert!(lines.iter().any(|l| l == "time_column 0"), "{lines:?}");
        assert!(lines.iter().any(|l| l == "unit s"), "{lines:?}");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn bait_lines_name_header_and_rare_columns() {
        let path = std::env::temp_dir().join(format!("sniff-bait-{}", std::process::id()));
        std::fs::write(
            &path,
            "time,station_id,discharge_m3s\n2026-01-01T00:00:00,1,2.5\n",
        )
        .unwrap();
        let lines = sniff_lines(&format!("{} bait", path.display()));
        assert!(
            lines
                .iter()
                .any(|l| l == "bait header time,station_id,discharge_m3s"),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "bait grepapp time,station_id,discharge_m3s"),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| l.contains("\"station_id\" \"discharge_m3s\"")),
            "{lines:?}"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn value_bait_carries_numbers_and_float_bytes() {
        let path = std::env::temp_dir().join(format!("sniff-values-{}", std::process::id()));
        std::fs::write(&path, "t v\n0 55041.35\n1 3061447552\n2 0.0849999\n").unwrap();
        let lines = sniff_lines(&format!("{} bait values", path.display()));
        assert!(
            lines.iter().any(|l| l == "bait value 55041.35"),
            "{lines:?}"
        );
        assert!(
            lines.iter().any(|l| l.starts_with("bait hex-f64 ")),
            "{lines:?}"
        );
        assert!(
            lines.iter().any(|l| l == "bait value 3061447552"),
            "{lines:?}"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn local_sniff_finds_a_pattern() {
        let path = std::env::temp_dir().join(format!("sniff-pattern-{}", std::process::id()));
        std::fs::write(&path, b"HEADabcTAILabc").unwrap();
        let query = format!("{} pattern=616263", path.display());
        let lines = sniff_lines(&query);
        assert!(
            lines.iter().any(|l| l == "pattern 616263 hits 4,11"),
            "{lines:?}"
        );
        let _ = std::fs::remove_file(&path);
    }

    fn fetch_of(raw: Vec<u8>) -> Fetch {
        Fetch {
            status: Some(200),
            body: String::from_utf8_lossy(&raw).to_string(),
            raw,
            retry_after: None,
            complete: true,
        }
    }

    fn fits_card(key: &str, value: &str) -> Vec<u8> {
        let mut card = vec![b' '; 80];
        card[..key.len()].copy_from_slice(key.as_bytes());
        card[8] = b'=';
        card[10..10 + value.len()].copy_from_slice(value.as_bytes());
        card
    }

    #[test]
    fn sniff_lines_carry_head_and_tail_chunk_hashes() {
        let raw = vec![b'a'; 10_000];
        let f = fetch_of(raw.clone());
        let lines = sniff_lines_from(&f, "https://example.com/blob", None);
        let head = omegaflow::sha256::sha256_hex(&raw[..4096]);
        let tail = omegaflow::sha256::sha256_hex(&raw[5904..]);
        assert!(
            lines
                .iter()
                .any(|l| *l == format!("chunk 0..4096 sha256 {}", head)),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| *l == format!("chunk 5904..10000 sha256 {}", tail)),
            "{lines:?}"
        );
        assert!(
            lines.iter().filter(|l| l.starts_with("chunk ")).count() == 2,
            "{lines:?}"
        );
    }

    #[test]
    fn sniff_lines_hash_the_chunk_around_a_pattern_hit() {
        let mut raw = vec![b'x'; 12_000];
        raw[6000..6004].copy_from_slice(b"NEED");
        let f = fetch_of(raw.clone());
        let lines = sniff_lines_from(&f, "https://example.com/blob", Some("4e454544"));
        let around = omegaflow::sha256::sha256_hex(&raw[3952..8052]);
        assert!(
            lines
                .iter()
                .any(|l| *l == format!("chunk 3952..8052 sha256 {}", around)),
            "{lines:?}"
        );
    }

    #[test]
    fn netcdf_scale_factor_names_the_decode() {
        let raw =
            b"CDF\x01\x00scale_factor = 0.01\nadd_offset = -5\n_FillValue = -9999\nunits = K\n"
                .to_vec();
        let f = fetch_of(raw);
        let lines = sniff_lines_from(&f, "https://example.com/blob", None);
        assert!(
            lines.iter().any(|l| l == "netcdf scale_factor=0.01"),
            "{lines:?}"
        );
        assert!(
            lines.iter().any(|l| l == "netcdf add_offset=-5"),
            "{lines:?}"
        );
        assert!(
            lines.iter().any(|l| l == "netcdf _FillValue=-9999"),
            "{lines:?}"
        );
        assert!(lines.iter().any(|l| l == "netcdf units=K"), "{lines:?}");
        assert!(
            lines
                .iter()
                .any(|l| l == "netcdf decoded_value=raw*scale+offset"),
            "{lines:?}"
        );
    }

    fn hdf5_checksum(buf: &[u8]) -> u32 {
        fn word(buf: &[u8], off: usize) -> u32 {
            u32::from_le_bytes(buf[off..off + 4].try_into().expect("word"))
        }
        let mut a: u32 = 0xdeadbeefu32.wrapping_add(buf.len() as u32);
        let mut b: u32 = a;
        let mut c: u32 = a;
        let mut off = 0usize;
        while buf.len() - off > 12 {
            a = a.wrapping_add(word(buf, off));
            b = b.wrapping_add(word(buf, off + 4));
            c = c.wrapping_add(word(buf, off + 8));
            a = a.wrapping_sub(c) ^ c.rotate_left(4);
            c = c.wrapping_add(b);
            b = b.wrapping_sub(a) ^ a.rotate_left(6);
            a = a.wrapping_add(c);
            c = c.wrapping_sub(b) ^ b.rotate_left(8);
            b = b.wrapping_add(a);
            a = a.wrapping_sub(c) ^ c.rotate_left(16);
            c = c.wrapping_add(b);
            b = b.wrapping_sub(a) ^ a.rotate_left(19);
            a = a.wrapping_add(c);
            c = c.wrapping_sub(b) ^ b.rotate_left(4);
            b = b.wrapping_add(a);
            off += 12;
        }
        let rest = &buf[off..];
        let n = rest.len();
        if n >= 12 {
            c = c.wrapping_add((rest[11] as u32) << 24);
        }
        if n >= 11 {
            c = c.wrapping_add((rest[10] as u32) << 16);
        }
        if n >= 10 {
            c = c.wrapping_add((rest[9] as u32) << 8);
        }
        if n >= 9 {
            c = c.wrapping_add(rest[8] as u32);
        }
        if n >= 8 {
            b = b.wrapping_add((rest[7] as u32) << 24);
        }
        if n >= 7 {
            b = b.wrapping_add((rest[6] as u32) << 16);
        }
        if n >= 6 {
            b = b.wrapping_add((rest[5] as u32) << 8);
        }
        if n >= 5 {
            b = b.wrapping_add(rest[4] as u32);
        }
        if n >= 4 {
            a = a.wrapping_add((rest[3] as u32) << 24);
        }
        if n >= 3 {
            a = a.wrapping_add((rest[2] as u32) << 16);
        }
        if n >= 2 {
            a = a.wrapping_add((rest[1] as u32) << 8);
        }
        if n >= 1 {
            a = a.wrapping_add(rest[0] as u32);
        }
        c ^= b;
        c = c.wrapping_sub(b.rotate_left(14));
        a ^= c;
        a = a.wrapping_sub(c.rotate_left(11));
        b ^= a;
        b = b.wrapping_sub(a.rotate_left(25));
        c ^= b;
        c = c.wrapping_sub(b.rotate_left(16));
        a ^= c;
        a = a.wrapping_sub(c.rotate_left(4));
        b ^= a;
        b = b.wrapping_sub(a.rotate_left(14));
        c ^= b;
        c = c.wrapping_sub(b.rotate_left(24));
        c
    }

    fn hdf5_message(typ: u8, data: Vec<u8>) -> Vec<u8> {
        let mut m = vec![typ, data.len() as u8, (data.len() >> 8) as u8, 0];
        m.extend_from_slice(&data);
        m
    }

    fn hdf5_header(messages: Vec<Vec<u8>>) -> Vec<u8> {
        let len: usize = messages.iter().map(|m| m.len()).sum();
        let size_len = if len <= u8::MAX as usize {
            1usize
        } else if len <= u16::MAX as usize {
            2usize
        } else {
            4usize
        };
        let flags = match size_len {
            1 => 0u8,
            2 => 1u8,
            _ => 2u8,
        };
        let mut body = vec![b'O', b'H', b'D', b'R', 2, flags];
        body.extend_from_slice(&len.to_le_bytes()[..size_len]);
        for message in messages {
            body.extend_from_slice(&message);
        }
        let ck = hdf5_checksum(&body);
        body.extend_from_slice(&ck.to_le_bytes());
        body
    }

    fn hdf5_dataspace(dims: &[u64]) -> Vec<u8> {
        let mut d = vec![2u8, dims.len() as u8, 0, 0];
        for dim in dims {
            d.extend_from_slice(&dim.to_le_bytes());
        }
        d
    }

    fn hdf5_datatype(class: u8, flags: u8, size: u32) -> Vec<u8> {
        let mut d = vec![
            0x10 | class,
            flags,
            0,
            0,
            size as u8,
            (size >> 8) as u8,
            (size >> 16) as u8,
            (size >> 24) as u8,
        ];
        match class {
            0 => {
                d.extend_from_slice(&0u16.to_le_bytes());
                d.extend_from_slice(&((size * 8) as u16).to_le_bytes());
            }
            1 => {
                d.extend_from_slice(&0u16.to_le_bytes());
                d.extend_from_slice(&((size * 8) as u16).to_le_bytes());
                d.extend_from_slice(&[0u8; 8]);
            }
            _ => {}
        }
        d
    }

    fn hdf5_attribute(name: &str, dt: &[u8], ds: &[u8], data: &[u8]) -> Vec<u8> {
        let mut d = vec![1u8, 0];
        d.extend_from_slice(&(name.len() as u16).to_le_bytes());
        d.extend_from_slice(&(dt.len() as u16).to_le_bytes());
        d.extend_from_slice(&(ds.len() as u16).to_le_bytes());
        d.extend_from_slice(name.as_bytes());
        d.resize(8 + name.len().div_ceil(8) * 8, 0);
        d.extend_from_slice(dt);
        d.resize(d.len().div_ceil(8) * 8, 0);
        d.extend_from_slice(ds);
        d.resize(d.len().div_ceil(8) * 8, 0);
        d.extend_from_slice(data);
        d
    }

    fn hdf5_link(name: &str, addr: u64) -> Vec<u8> {
        let mut d = vec![0, 0, name.len() as u8];
        d.extend_from_slice(name.as_bytes());
        d.extend_from_slice(&addr.to_le_bytes());
        d
    }

    fn hdf5_glm_fixture() -> Vec<u8> {
        let string = |size: u32| hdf5_datatype(3, 0, size);
        let f32dt = || hdf5_datatype(1, 0, 4);
        let i16dt = || hdf5_datatype(0, 0x08, 2);
        let scalar = hdf5_dataspace(&[]);
        let vector = hdf5_dataspace(&[4]);
        let energy_attrs = vec![
            hdf5_message(0x0c, hdf5_attribute("units", &string(1), &scalar, b"J")),
            hdf5_message(
                0x0c,
                hdf5_attribute("scale_factor", &f32dt(), &scalar, &0.5f32.to_le_bytes()),
            ),
            hdf5_message(
                0x0c,
                hdf5_attribute("add_offset", &f32dt(), &scalar, &(-1.25f32).to_le_bytes()),
            ),
            hdf5_message(
                0x0c,
                hdf5_attribute("_FillValue", &i16dt(), &scalar, &(-1i16).to_le_bytes()),
            ),
        ];
        let lat_attrs = vec![
            hdf5_message(
                0x0c,
                hdf5_attribute("units", &string(13), &scalar, b"degrees_north"),
            ),
            hdf5_message(
                0x0c,
                hdf5_attribute("scale_factor", &f32dt(), &scalar, &0.25f32.to_le_bytes()),
            ),
        ];
        let mut energy_messages = vec![
            hdf5_message(0x01, vector.clone()),
            hdf5_message(0x03, f32dt()),
        ];
        energy_messages.extend(energy_attrs);
        let mut lat_messages = vec![hdf5_message(0x01, vector), hdf5_message(0x03, f32dt())];
        lat_messages.extend(lat_attrs);
        let energy_header = hdf5_header(energy_messages);
        let lat_header = hdf5_header(lat_messages);
        let root_addr = 48u64;
        let root_len = hdf5_header(vec![
            hdf5_message(0x06, hdf5_link("event_energy", 0)),
            hdf5_message(0x06, hdf5_link("event_lat", 0)),
        ])
        .len() as u64;
        let energy_addr = root_addr + root_len;
        let lat_addr = energy_addr + energy_header.len() as u64;
        let root_header = hdf5_header(vec![
            hdf5_message(0x06, hdf5_link("event_energy", energy_addr)),
            hdf5_message(0x06, hdf5_link("event_lat", lat_addr)),
        ]);
        let mut buf = Vec::new();
        buf.extend_from_slice(b"\x89HDF\r\n\x1a\n");
        buf.push(2);
        buf.push(8);
        buf.push(8);
        buf.push(0);
        buf.resize(36, 0);
        buf.extend_from_slice(&root_addr.to_le_bytes());
        buf.extend_from_slice(&[0, 0, 0, 0]);
        let ck = hdf5_checksum(&buf[..44]);
        buf[44..48].copy_from_slice(&ck.to_le_bytes());
        buf.extend_from_slice(&root_header);
        buf.extend_from_slice(&energy_header);
        buf.extend_from_slice(&lat_header);
        buf
    }

    fn hdf5_single_dataset_fixture(
        name: &str,
        datatype: Vec<u8>,
        dims: &[u64],
        attrs: Vec<Vec<u8>>,
        payload: &[u8],
    ) -> Vec<u8> {
        let root_addr = 48u64;
        let root_len = hdf5_header(vec![hdf5_message(0x06, hdf5_link(name, 0))]).len() as u64;
        let dataset_addr = root_addr + root_len;
        let layout = |addr: u64| {
            let mut data = vec![3u8, 1u8];
            data.extend_from_slice(&addr.to_le_bytes());
            data.extend_from_slice(&(payload.len() as u64).to_le_bytes());
            hdf5_message(0x08, data)
        };
        let mut messages = vec![
            hdf5_message(0x01, hdf5_dataspace(dims)),
            hdf5_message(0x03, datatype.clone()),
            layout(0),
        ];
        messages.extend(attrs.clone());
        let header_len = hdf5_header(messages.clone()).len() as u64;
        let data_addr = dataset_addr + header_len;
        let mut typed: Vec<Vec<u8>> = vec![
            hdf5_message(0x01, hdf5_dataspace(dims)),
            hdf5_message(0x03, datatype),
        ];
        typed.push(layout(data_addr));
        typed.extend(attrs);
        let dataset_header = hdf5_header(typed);
        assert_eq!(dataset_header.len() as u64, header_len);
        let root_header = hdf5_header(vec![hdf5_message(0x06, hdf5_link(name, dataset_addr))]);
        let mut buf = Vec::new();
        buf.extend_from_slice(b"\x89HDF\r\n\x1a\n");
        buf.push(2);
        buf.push(8);
        buf.push(8);
        buf.push(0);
        buf.resize(36, 0);
        buf.extend_from_slice(&root_addr.to_le_bytes());
        buf.extend_from_slice(&[0, 0, 0, 0]);
        let ck = hdf5_checksum(&buf[..44]);
        buf[44..48].copy_from_slice(&ck.to_le_bytes());
        buf.extend_from_slice(&root_header);
        buf.extend_from_slice(&dataset_header);
        buf.extend_from_slice(payload);
        buf
    }

    fn hdf5_packed_fixture() -> Vec<u8> {
        let string = |size: u32| hdf5_datatype(3, 0, size);
        let f32dt = || hdf5_datatype(1, 0, 4);
        let i16dt = hdf5_datatype(0, 0x08, 2);
        let scalar = hdf5_dataspace(&[]);
        let attrs = vec![
            hdf5_message(0x0c, hdf5_attribute("units", &string(1), &scalar, b"1")),
            hdf5_message(
                0x0c,
                hdf5_attribute("scale_factor", &f32dt(), &scalar, &10.0f32.to_le_bytes()),
            ),
            hdf5_message(
                0x0c,
                hdf5_attribute("add_offset", &f32dt(), &scalar, &(-3.0f32).to_le_bytes()),
            ),
            hdf5_message(
                0x0c,
                hdf5_attribute("_FillValue", &i16dt, &scalar, &(-1i16).to_le_bytes()),
            ),
        ];
        let payload: Vec<u8> = [-1i16, 2, 3, -1, 5]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        hdf5_single_dataset_fixture("event_code", i16dt, &[5], attrs, &payload)
    }

    #[test]
    fn netcdf_values_decode_packed_integers_with_scale_and_offset() {
        let lines = netcdf_value_lines(&hdf5_packed_fixture(), "event_code", 10);
        assert!(
            lines
                .iter()
                .any(|l| l == "netcdf values variable=event_code dims=5 type=int16 order=row-major"),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "netcdf value 1 raw=2 scale=10 offset=-3 decoded=17"),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "netcdf value 2 raw=3 scale=10 offset=-3 decoded=27"),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "netcdf value 4 raw=5 scale=10 offset=-3 decoded=47"),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "netcdf values scale=10 offset=-3 fill=-1"),
            "{lines:?}"
        );
    }

    #[test]
    fn netcdf_values_drop_fill_rows_and_count_them() {
        let lines = netcdf_value_lines(&hdf5_packed_fixture(), "event_code", 10);
        assert!(
            !lines.iter().any(|l| l.starts_with("netcdf value 0 ")),
            "{lines:?}"
        );
        assert!(
            !lines.iter().any(|l| l.starts_with("netcdf value 3 ")),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "netcdf values shown=3 decoded=3 skipped_fill=2 total=5"),
            "{lines:?}"
        );
        let capped = netcdf_value_lines(&hdf5_packed_fixture(), "event_code", 2);
        assert!(
            capped
                .iter()
                .any(|l| l == "netcdf values shown=2 decoded=3 skipped_fill=2 total=5"),
            "{capped:?}"
        );
    }

    fn hdf5_unsigned_fixture() -> Vec<u8> {
        let string = |size: u32| hdf5_datatype(3, 0, size);
        let f32dt = || hdf5_datatype(1, 0, 4);
        let i16dt = hdf5_datatype(0, 0x08, 2);
        let scalar = hdf5_dataspace(&[]);
        let attrs = vec![
            hdf5_message(
                0x0c,
                hdf5_attribute("_Unsigned", &string(4), &scalar, b"true"),
            ),
            hdf5_message(
                0x0c,
                hdf5_attribute("scale_factor", &f32dt(), &scalar, &0.5f32.to_le_bytes()),
            ),
            hdf5_message(
                0x0c,
                hdf5_attribute("add_offset", &f32dt(), &scalar, &(-100.0f32).to_le_bytes()),
            ),
            hdf5_message(
                0x0c,
                hdf5_attribute("_FillValue", &i16dt, &scalar, &(-1i16).to_le_bytes()),
            ),
        ];
        let payload: Vec<u8> = [-1i16, 10].iter().flat_map(|v| v.to_le_bytes()).collect();
        hdf5_single_dataset_fixture("packed_lat", i16dt, &[2], attrs, &payload)
    }

    #[test]
    fn netcdf_values_read_unsigned_int16_with_the_signed_fill() {
        let lines = netcdf_value_lines(&hdf5_unsigned_fixture(), "packed_lat", 10);
        assert!(
            lines.iter().any(
                |l| l == "netcdf values variable=packed_lat dims=2 type=uint16 order=row-major"
            ),
            "{lines:?}"
        );
        assert!(
            !lines.iter().any(|l| l.starts_with("netcdf value 0 ")),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "netcdf value 1 raw=10 scale=0.5 offset=-100 decoded=-95"),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "netcdf values shown=1 decoded=1 skipped_fill=1 total=2"),
            "{lines:?}"
        );
    }

    #[test]
    fn netcdf4_attributes_are_read_from_the_hdf5_root_group() {
        let lines = netcdf_signal_lines(&hdf5_glm_fixture(), "");
        assert!(
            lines.iter().any(|l| l
                == "netcdf event_energy units=J scale_factor=0.5 add_offset=-1.25 _FillValue=-1"),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "netcdf event_lat units=degrees_north scale_factor=0.25"),
            "{lines:?}"
        );
        assert!(
            lines.iter().any(|l| l == "netcdf add_offset=-1.25"),
            "{lines:?}"
        );
        assert!(
            !lines
                .iter()
                .any(|l| l
                    == "netcdf event_lat units=degrees_north scale_factor=0.25 add_offset=-1.25"),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "netcdf decoded_value=raw*scale+offset"),
            "{lines:?}"
        );
    }

    #[test]
    fn netcdf4_absent_attributes_stay_absent() {
        let lines = netcdf_signal_lines(&hdf5_glm_fixture(), "");
        assert!(
            !lines.iter().any(|l| l == "netcdf event_lat _FillValue=0"),
            "{lines:?}"
        );
    }

    #[test]
    fn netcdf_unit_scale_skips_the_decode_hint() {
        let raw = b"CDF\x01 scale_factor = 1 add_offset = 0 units = 1\n".to_vec();
        let f = fetch_of(raw);
        let lines = sniff_lines_from(&f, "https://example.com/blob", None);
        assert!(
            lines.iter().any(|l| l == "netcdf scale_factor=1"),
            "{lines:?}"
        );
        assert!(
            !lines
                .iter()
                .any(|l| l == "netcdf decoded_value=raw*scale+offset"),
            "{lines:?}"
        );
    }

    #[test]
    fn fits_cards_name_bunit_and_axes() {
        let mut raw = fits_card("SIMPLE", "T");
        raw.extend(fits_card("BUNIT", "'Jy'"));
        raw.extend(fits_card("BSCALE", "1.0"));
        raw.extend(fits_card("BZERO", "0.0"));
        raw.extend(fits_card("NAXIS", "2"));
        raw.extend(fits_card("NAXIS1", "1024"));
        raw.extend(fits_card("NAXIS2", "512"));
        raw.extend(fits_card("END", ""));
        let f = fetch_of(raw);
        let lines = sniff_lines_from(&f, "https://example.com/blob", None);
        assert!(
            lines
                .iter()
                .any(|l| l == "fits BUNIT=Jy BSCALE=1.0 BZERO=0.0 NAXIS=2 NAXIS1=1024 NAXIS2=512"),
            "{lines:?}"
        );
    }

    #[test]
    fn healpix_names_ordering_and_nside() {
        let raw = b"ORDERING= NESTED\nNSIDE   = 512\n".to_vec();
        let f = fetch_of(raw);
        let lines = sniff_lines_from(&f, "https://example.com/blob", None);
        assert!(
            lines
                .iter()
                .any(|l| l == "healpix ordering=NESTED nside=512"),
            "{lines:?}"
        );
    }

    #[test]
    fn healpix_is_absent_without_ordering() {
        let raw = b"time,value\n2026-01-01T00:00:00,1.5\n".to_vec();
        let f = fetch_of(raw);
        let lines = sniff_lines_from(&f, "https://example.com/blob", None);
        assert!(lines.iter().any(|l| l == "healpix absent"), "{lines:?}");
    }

    #[test]
    fn bait_text_fishes_identifiers_units_and_stops() {
        let path = std::env::temp_dir().join(format!("sniff-bait-text-{}", std::process::id()));
        std::fs::write(
            &path,
            "We measured station_id and discharge [m3/s] at the gauge.\n\
             the [m] noise was removed by the pipeline before analysis.\n\
             alpha_beta gamma_delta delta_epsilon zeta_eta theta_iota kappa_lambda mu_nu xi_omicron.\n",
        )
        .unwrap();
        let lines = sniff_lines(&format!("{} bait", path.display()));
        assert!(
            lines.iter().any(|l| l == "bait text \"station_id\""),
            "{lines:?}"
        );
        assert!(
            lines.iter().any(|l| l == "bait text \"discharge [m3/s]\""),
            "{lines:?}"
        );
        assert!(
            lines.iter().all(|l| l != "bait text \"the [m]\""),
            "{lines:?}"
        );
        assert!(
            lines.iter().filter(|l| l.starts_with("bait text ")).count() == 8,
            "{lines:?}"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn sniff_lines_hash_and_size_read_the_raw_bytes() {
        let f = lossy_fetch();
        let lines = sniff_lines_from(&f, "https://example.com/blob", None);
        let raw_sha = omegaflow::sha256::sha256_hex(&f.raw);
        let body_sha = omegaflow::sha256::sha256_hex(f.body.as_bytes());
        assert!(lines.iter().any(|l| l == "bytes 4"), "{lines:?}");
        assert!(
            lines.iter().any(|l| *l == format!("sha256 {}", raw_sha)),
            "{lines:?}"
        );
        assert_ne!(lines[4], format!("sha256 {}", body_sha));
    }

    #[test]
    fn stage_result_reports_the_raw_byte_count() {
        let f = lossy_fetch();
        assert_ne!(f.raw.len(), f.body.len());
        let mut lines = Vec::new();
        stage_result(&mut lines, 1, "direct", "https://example.com/blob", f);
        assert_eq!(lines[0], "  stage 1 direct: HTTP 200 (4 bytes) — found");
    }

    #[test]
    fn curl_args_keep_the_trailing_slash_and_follow_redirects() {
        let url = "https://api.alerce.online/alerts/v1/objects/";
        let args = curl_args(url, &[], "30", &[]);
        assert_eq!(args.last().map(String::as_str), Some(url));
        assert!(
            args.iter().any(|a| a == "-sL"),
            "the probe is GET and follows redirects"
        );
        assert!(
            !args.iter().any(|a| a == "-I" || a == "--head"),
            "the probe is no HEAD request"
        );
    }

    #[test]
    fn post_args_prefix_every_header_with_dash_h() {
        let args = post_args("{}", &["x-api-key: K", "Authorization: Bearer T"]);
        for header in ["x-api-key: K", "Authorization: Bearer T"] {
            let at = match args.iter().position(|a| a == header) {
                Some(i) => i,
                None => panic!("{header} missing from the post args"),
            };
            assert_eq!(
                at.checked_sub(1).map(|i| args[i].as_str()),
                Some("-H"),
                "{header} must pass through -H, never land as a bare URL argument"
            );
        }
        assert_eq!(args.last().map(String::as_str), Some("{}"));
        assert!(args.iter().any(|a| a == "--data"));
    }

    #[test]
    fn retry_transient_reprobes_a_zero_status_then_returns_the_answer() {
        let calls = std::cell::Cell::new(0usize);
        let result = retry_transient(|| {
            calls.set(calls.get() + 1);
            if calls.get() < 2 {
                answer(Some(0))
            } else {
                answer(Some(200))
            }
        });
        assert_eq!(result.unwrap().status, Some(200));
        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn retry_transient_reports_pending_when_no_status_is_measured() {
        let calls = std::cell::Cell::new(0usize);
        let result = retry_transient(|| {
            calls.set(calls.get() + 1);
            answer(None)
        });
        assert!(result.is_none());
        assert_eq!(calls.get(), VERDICT_ATTEMPTS);
    }

    #[test]
    fn verdict_probe_reads_only_the_first_byte() {
        let args = curl_args("https://example.com/asset.bin", VERDICT_RANGE, "30", &[]);
        assert!(
            args.iter().any(|a| a == "-sL"),
            "the verdict probe is a follow-redirect GET"
        );
        assert!(
            !args.iter().any(|a| a == "-I" || a == "--head"),
            "the probe is no HEAD request"
        );
        let at = args
            .iter()
            .position(|a| a == "--range")
            .expect("the verdict probe carries --range");
        assert_eq!(
            args.get(at + 1).map(String::as_str),
            Some("0-0"),
            "one byte, never the asset body"
        );
    }

    #[test]
    fn resolved_status_is_not_transient() {
        assert!(!is_transient(&Fetch {
            complete: false,
            status: Some(200),
            ..partial_fetch()
        }));
        assert!(is_transient(&Fetch {
            status: Some(0),
            ..partial_fetch()
        }));
        assert!(is_transient(&Fetch {
            status: None,
            ..partial_fetch()
        }));
    }

    #[test]
    fn is_found_reads_2xx_including_range_partials() {
        for status in [200, 204, 206] {
            assert!(
                is_found(&Fetch {
                    status: Some(status),
                    ..lossy_fetch()
                }),
                "{status} is a reachable answer"
            );
        }
        assert!(!is_found(&Fetch {
            status: Some(404),
            ..lossy_fetch()
        }));
        assert!(!is_found(&Fetch {
            status: Some(302),
            ..lossy_fetch()
        }));
        assert!(!is_found(&Fetch {
            status: None,
            ..lossy_fetch()
        }));
    }

    #[test]
    fn stage_result_reads_a_range_partial_as_found() {
        let mut lines = Vec::new();
        stage_result(
            &mut lines,
            1,
            "direct",
            "https://example.com/asset.bin",
            Fetch {
                status: Some(206),
                ..lossy_fetch()
            },
        );
        assert_eq!(lines[0], "  stage 1 direct: HTTP 206 (4 bytes) — found");

        let mut absent = Vec::new();
        stage_result(
            &mut absent,
            1,
            "direct",
            "https://example.com/asset.bin",
            Fetch {
                status: Some(404),
                ..lossy_fetch()
            },
        );
        assert!(absent[0].ends_with("— absent"), "{}", absent[0]);
    }

    #[test]
    fn crossref_url_carries_the_default_select_and_the_plain_query() {
        assert_eq!(
            crossref_url("gravitational waves", &[], 10, None),
            "https://api.crossref.org/works?rows=10&query=gravitational%20waves&select=DOI%2Ctitle%2Cissued%2Cpublished%2Cauthor%2Cis-referenced-by-count%2Cabstract"
        );
    }

    fn crossref_item(body: &str) -> Json {
        json::parse(body).expect("test item parses")
    }

    #[test]
    fn crossref_line_carries_authors_cites_and_abstract() {
        let item = crossref_item(
            r#"{"DOI":"10.1234/x","title":["A study"],"author":[{"given":"Ada","family":"Lovelace"},{"given":"Alan","family":"Turing"}],"is-referenced-by-count":42,"abstract":"<jats:p>Hello   <jats:italic>world</jats:italic></jats:p>"}"#,
        );
        assert_eq!(
            crossref_line(&item),
            Some(
                "url https://doi.org/10.1234/x\ttitle: A study\tauthors: Ada Lovelace, Alan Turing\tcites: 42\tabstract: Hello world"
                    .to_string()
            )
        );
    }

    #[test]
    fn crossref_line_carries_the_issued_year() {
        let item = crossref_item(
            r#"{"DOI":"10.1234/y","title":["Dated"],"issued":{"date-parts":[[2001,1,29]]}}"#,
        );
        assert_eq!(
            crossref_line(&item),
            Some("url https://doi.org/10.1234/y\ttitle: Dated\tyear: 2001".to_string())
        );
    }

    #[test]
    fn crossref_line_falls_back_past_a_null_issued_year() {
        let item = crossref_item(
            r#"{"DOI":"10.1234/z","title":["Undated"],"issued":{"date-parts":[[null]]},"published":{"date-parts":[[2019]]}}"#,
        );
        assert_eq!(
            crossref_line(&item),
            Some("url https://doi.org/10.1234/z\ttitle: Undated\tyear: 2019".to_string())
        );
    }

    #[test]
    fn crossref_line_omits_absent_author_cites_and_abstract() {
        let item = crossref_item(r#"{"DOI":"10.1234/y","title":["Bare"]}"#);
        assert_eq!(
            crossref_line(&item),
            Some("url https://doi.org/10.1234/y\ttitle: Bare".to_string())
        );
    }

    #[test]
    fn crossref_line_is_none_without_a_doi() {
        let item = crossref_item(r#"{"title":["No doi"]}"#);
        assert_eq!(crossref_line(&item), None);
    }

    #[test]
    fn crossref_line_caps_authors_at_eight_with_et_al() {
        let authors: Vec<String> = (0..9)
            .map(|i| format!(r#"{{"given":"G{i}","family":"F{i}"}}"#))
            .collect();
        let body = format!(
            r#"{{"DOI":"10.1234/z","title":["Many"],"author":[{}]}}"#,
            authors.join(",")
        );
        let item = crossref_item(&body);
        let line = crossref_line(&item).expect("line builds");
        assert!(
            line.contains(
                "\tauthors: G0 F0, G1 F1, G2 F2, G3 F3, G4 F4, G5 F5, G6 F6, G7 F7, et al."
            )
        );
    }

    #[test]
    fn crossref_url_lifts_filter_sort_and_a_custom_select() {
        let opts = vec![
            ("filter".to_string(), "type:journal-article".to_string()),
            ("sort".to_string(), "published".to_string()),
            ("order".to_string(), "desc".to_string()),
            ("select".to_string(), "DOI,title".to_string()),
        ];
        assert_eq!(
            crossref_url("p53", &opts, 5, Some("abc")),
            "https://api.crossref.org/works?rows=5&query=p53&select=DOI%2Ctitle&filter=type%3Ajournal-article&sort=published&order=desc&cursor=abc"
        );
    }

    #[test]
    fn crossref_url_omits_the_query_when_only_a_filter_is_given() {
        let opts = vec![("filter".to_string(), "type:journal-article".to_string())];
        assert_eq!(
            crossref_url("", &opts, 10, None),
            "https://api.crossref.org/works?rows=10&select=DOI%2Ctitle%2Cissued%2Cpublished%2Cauthor%2Cis-referenced-by-count%2Cabstract&filter=type%3Ajournal-article"
        );
    }

    #[test]
    fn hal_url_carries_the_default_fl_and_omits_a_zero_start() {
        assert_eq!(
            hal_url("gravitational waves", &[], 10, 0),
            "https://api.archives-ouvertes.fr/search/?q=gravitational%20waves&wt=json&rows=10&fl=title_s%2Curi_s%2CdoiId_s%2CpublicationDate_s"
        );
    }

    #[test]
    fn hal_url_lifts_fq_fl_sort_and_the_start_offset() {
        let opts = vec![
            ("fq".to_string(), "docType_s:ART".to_string()),
            ("fl".to_string(), "title_s".to_string()),
            ("sort".to_string(), "publicationDate_s desc".to_string()),
        ];
        assert_eq!(
            hal_url("p53", &opts, 10, 20),
            "https://api.archives-ouvertes.fr/search/?q=p53&wt=json&rows=10&fl=title_s&start=20&fq=docType_s%3AART&sort=publicationDate_s%20desc"
        );
    }

    #[test]
    fn hal_results_caps_authors_at_eight_and_appends_et_al() {
        let body = r#"{"response":{"docs":[{
            "uri_s":"https://hal.science/hal-1",
            "title_s":["A study"],
            "authFullName_s":["A1","A2","A3","A4","A5","A6","A7","A8","A9","A10"]
        }]}}"#;
        let v = json::parse(body).expect("json");
        let lines = hal_results(&v, 10);
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            "url https://hal.science/hal-1\ttitle: A study\tauthors: A1, A2, A3, A4, A5, A6, A7, A8, et al."
        );
    }

    #[test]
    fn hal_results_carries_the_authors_and_the_abstract() {
        let body = r#"{"response":{"docs":[{
            "uri_s":"https://hal.science/hal-2",
            "title_s":["Another study"],
            "doiId_s":"10.1/x",
            "publicationDate_s":"2020",
            "authFullName_s":["B1","B2"],
            "abstract_s":["The first abstract text."]
        }]}}"#;
        let v = json::parse(body).expect("json");
        let lines = hal_results(&v, 10);
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            "url https://hal.science/hal-2\ttitle: Another study\tdoi: 10.1/x\tpublished: 2020\tauthors: B1, B2\tabstract: The first abstract text."
        );
    }

    #[test]
    fn hal_results_omits_absent_authors_and_abstract() {
        let body = r#"{"response":{"docs":[{
            "uri_s":"https://hal.science/hal-3",
            "title_s":["No extras"],
            "authFullName_s":[],
            "abstract_s":[]
        }]}}"#;
        let v = json::parse(body).expect("json");
        let lines = hal_results(&v, 10);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "url https://hal.science/hal-3\ttitle: No extras");
    }

    #[test]
    fn ads_url_carries_the_default_fl_and_omits_a_zero_start() {
        assert_eq!(
            ads_url("gravitational waves", &[], 10, 0),
            "https://api.adsabs.harvard.edu/v1/search/query?q=gravitational%20waves&fl=title%2Cbibcode%2Cauthor%2Cyear%2Cabstract&rows=10"
        );
    }

    #[test]
    fn ads_url_lifts_fq_sort_and_the_start_offset() {
        let opts = vec![
            ("fq".to_string(), "{!type=aqp} SUPERNOVA".to_string()),
            ("sort".to_string(), "date desc".to_string()),
        ];
        assert_eq!(
            ads_url("p53", &opts, 10, 30),
            "https://api.adsabs.harvard.edu/v1/search/query?q=p53&fl=title%2Cbibcode%2Cauthor%2Cyear%2Cabstract&rows=10&start=30&fq=%7B%21type%3Daqp%7D%20SUPERNOVA&sort=date%20desc"
        );
    }

    #[test]
    fn ads_url_lets_a_caller_supplied_fl_override_the_default() {
        let opts = vec![("fl".to_string(), "title".to_string())];
        assert_eq!(
            ads_url("p53", &opts, 10, 0),
            "https://api.adsabs.harvard.edu/v1/search/query?q=p53&fl=title&rows=10"
        );
    }

    #[test]
    fn ads_doc_line_carries_authors_year_and_abstract() {
        let body = r#"{"bibcode":"2020ApJ...900....1A","title":["A great paper"],
            "author":["A1","A2","A3","A4","A5","A6","A7","A8","A9"],
            "year":2020,"abstract":"The abstract."}"#;
        let doc = json::parse(body).expect("json");
        assert_eq!(
            ads_doc_line(&doc).expect("line"),
            "url https://ui.adsabs.harvard.edu/abs/2020ApJ...900....1A\ttitle: A great paper\tauthors: A1, A2, A3, A4, A5, A6, A7, A8, et al.\tyear: 2020\tabstract: The abstract."
        );
    }

    #[test]
    fn ads_doc_line_omits_absent_authors_year_and_abstract() {
        let body = r#"{"bibcode":"2021ApJ...901....2B","title":["Bare paper"]}"#;
        let doc = json::parse(body).expect("json");
        assert_eq!(
            ads_doc_line(&doc).expect("line"),
            "url https://ui.adsabs.harvard.edu/abs/2021ApJ...901....2B\ttitle: Bare paper"
        );
    }

    #[test]
    fn ia_search_url_carries_the_default_fl_and_omits_the_first_page() {
        assert_eq!(
            ia_search_url("gravitational waves", &[], 10, 1),
            "https://archive.org/advancedsearch.php?q=gravitational%20waves&rows=10&output=json&fl%5B%5D=identifier&fl%5B%5D=title&fl%5B%5D=mediatype&fl%5B%5D=creator&fl%5B%5D=year"
        );
    }

    #[test]
    fn ia_search_results_carry_creator_string_and_year() {
        let body = r#"{"response":{"docs":[{"identifier":"abc","title":"A work","mediatype":"texts","creator":"Ada Lovelace","year":"1843"}]}}"#;
        let v = json::parse(body).expect("json");
        assert_eq!(
            ia_search_results(&v, 10),
            vec![
                "url https://archive.org/details/abc\ttitle: A work\tmediatype: texts\tcreator: Ada Lovelace\tyear: 1843"
                    .to_string()
            ]
        );
    }

    #[test]
    fn ia_search_results_carry_creator_array_and_year_from_date() {
        let body = r#"{"response":{"docs":[{"identifier":"abc","title":"A work","creator":["Ada Lovelace","Alan Turing"],"date":"1950-06-01"}]}}"#;
        let v = json::parse(body).expect("json");
        assert_eq!(
            ia_search_results(&v, 10),
            vec![
                "url https://archive.org/details/abc\ttitle: A work\tcreator: Ada Lovelace, Alan Turing\tyear: 1950"
                    .to_string()
            ]
        );
    }

    #[test]
    fn ia_search_results_cap_the_creator_list_at_eight() {
        let body = r#"{"response":{"docs":[{"identifier":"abc","title":"A work","creator":["A1","A2","A3","A4","A5","A6","A7","A8","A9"]}]}}"#;
        let v = json::parse(body).expect("json");
        assert_eq!(
            ia_search_results(&v, 10),
            vec![
                "url https://archive.org/details/abc\ttitle: A work\tcreator: A1, A2, A3, A4, A5, A6, A7, A8, et al."
                    .to_string()
            ]
        );
    }

    #[test]
    fn ia_search_results_omit_absent_creator_and_year() {
        let body = r#"{"response":{"docs":[{"identifier":"abc","title":"A work"}]}}"#;
        let v = json::parse(body).expect("json");
        assert_eq!(
            ia_search_results(&v, 10),
            vec!["url https://archive.org/details/abc\ttitle: A work".to_string()]
        );
    }

    #[test]
    fn ia_search_url_lifts_a_fl_list_a_sort_and_the_page() {
        let opts = vec![
            ("fl".to_string(), "identifier,title".to_string()),
            ("sort".to_string(), "downloads desc".to_string()),
        ];
        assert_eq!(
            ia_search_url("p53", &opts, 25, 3),
            "https://archive.org/advancedsearch.php?q=p53&rows=25&output=json&fl%5B%5D=identifier&fl%5B%5D=title&page=3&sort%5B%5D=downloads%20desc"
        );
    }

    #[test]
    fn arxiv_query_prefixes_the_all_field_by_default() {
        assert_eq!(
            arxiv_query("gravitational waves"),
            "all:gravitational%20waves"
        );
    }

    #[test]
    fn arxiv_query_passes_a_recognized_field_prefix_through() {
        assert_eq!(
            arxiv_query("ti:gravitational waves"),
            "ti:gravitational%20waves"
        );
        assert_eq!(arxiv_query("cat:astro-ph"), "cat:astro-ph");
    }

    #[test]
    fn arxiv_url_carries_the_field_query_the_start_and_the_sort() {
        let opts = vec![
            ("sortBy".to_string(), "submittedDate".to_string()),
            ("sortOrder".to_string(), "descending".to_string()),
        ];
        assert_eq!(
            arxiv_url("au:Einstein", &opts, 10, 2),
            "https://export.arxiv.org/api/query?search_query=au:Einstein&start=2&max_results=2&sortBy=submittedDate&sortOrder=descending"
        );
    }

    #[test]
    fn ntrs_id_line_carries_abstract_date_authors() {
        let body = r#"{"id":"20210005208","title":"Transfer entropy in the field","abstract":"the abstract body","publicationDate":"2021-03-15T00:00:00.000Z","authorAffiliations":["A. Author","B. Author"]}"#;
        let v = json::parse(body).expect("json");
        let rid = v.get("id").and_then(|i| i.as_scalar_string()).expect("id");
        assert_eq!(
            ntrs_doc_line(&v, &rid),
            "url https://ntrs.nasa.gov/citations/20210005208\ttitle: Transfer entropy in the field\tabstract: the abstract body\tpublished: 2021-03-15\tauthors: A. Author, B. Author"
                .to_string()
        );
    }

    #[test]
    fn ntrs_search_line_omits_absent_fields() {
        let body = r#"{"id":"20210005208","title":"Transfer entropy in the field"}"#;
        let v = json::parse(body).expect("json");
        assert_eq!(
            ntrs_doc_line(&v, "20210005208"),
            "url https://ntrs.nasa.gov/citations/20210005208\ttitle: Transfer entropy in the field"
                .to_string()
        );
    }

    #[test]
    fn ntrs_authors_fall_back_to_name_objects_and_cap_at_eight() {
        let body = r#"{"authors":[{"name":"A"},{"name":"B"},{"name":"C"},{"name":"D"},{"name":"E"},{"name":"F"},{"name":"G"},{"name":"H"},{"name":"I"}]}"#;
        let v = json::parse(body).expect("json");
        assert_eq!(
            ntrs_authors(&v),
            Some("A, B, C, D, E, F, G, H, et al.".to_string())
        );
    }

    #[test]
    fn all_stop_is_the_one_width_control() {
        assert_eq!(resolve_stop_threshold(None, None), DEFAULT_ALL_STOP);
        assert_eq!(DEFAULT_ALL_STOP, 20);
        assert_eq!(resolve_stop_threshold(Some(400), None), 400);
        assert_eq!(resolve_stop_threshold(Some(400), Some("50")), 400);
        assert_eq!(resolve_stop_threshold(None, Some("50")), 50);
        assert_eq!(resolve_stop_threshold(None, Some("not-a-number")), 20);
    }

    #[test]
    fn a_declared_but_empty_key_says_the_gate_is_covered() {
        let mut env = HashMap::new();
        env.insert("BASE_API_KEY".to_string(), String::new());
        let mut lines = vec!["pending — BASE_API_KEY absent from .secrets.local/.env".to_string()];
        annotate_declared(&mut lines, &env);
        assert!(lines[0].contains("gate is covered"), "{}", lines[0]);

        let mut unknown =
            vec!["pending — NASA_ADS_TOKEN absent from .secrets.local/.env".to_string()];
        annotate_declared(&mut unknown, &env);
        assert!(!unknown[0].contains("gate is covered"), "{}", unknown[0]);
    }

    #[test]
    fn annotating_twice_does_not_grow_the_gate_tail() {
        let mut env = HashMap::new();
        env.insert("BASE_API_KEY".to_string(), String::new());
        let mut lines = vec!["pending — BASE_API_KEY absent from .secrets.local/.env".to_string()];
        annotate_declared(&mut lines, &env);
        let once = lines[0].clone();
        annotate_declared(&mut lines, &env);
        assert_eq!(
            lines[0], once,
            "the --all aggregate pass duplicated the tail"
        );
    }
}
