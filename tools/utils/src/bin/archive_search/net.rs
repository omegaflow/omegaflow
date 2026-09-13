use crate::json::{self, Json};
use crate::secrets::resolve_secret;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

pub struct Fetch {
    pub status: Option<i32>,
    pub body: String,
    pub retry_after: Option<u64>,
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

fn curl_fetch(url: &str, extra: &[&str], timeout: &str, transport: &[String]) -> Option<Fetch> {
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
    let out = Command::new("curl").args(&args).output().ok()?;
    let raw = String::from_utf8_lossy(&out.stdout).to_string();
    let (rest, retry) = raw.rsplit_once('\n')?;
    let (body, code) = rest.rsplit_once('\n')?;
    Some(Fetch {
        status: code.trim().parse::<i32>().ok(),
        body: body.to_string(),
        retry_after: retry.trim().parse::<u64>().ok(),
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

fn proton_socks() -> Option<String> {
    let runtime = match std::env::var("XDG_RUNTIME_DIR") {
        Ok(dir) => dir,
        Err(_) => "/tmp".to_string(),
    };
    let dir = std::path::Path::new(&runtime).join("proton-wg");
    let pid = std::fs::read_to_string(dir.join("wireproxy.pid")).ok()?;
    let pid = pid.trim();
    if pid.is_empty() || !std::path::Path::new(&format!("/proc/{pid}")).exists() {
        return None;
    }
    let conf = std::fs::read_to_string(dir.join("active.conf")).ok()?;
    let addr = socks_bind(&conf)?;
    let sock: SocketAddr = addr.parse().ok()?;
    TcpStream::connect_timeout(&sock, Duration::from_millis(500)).ok()?;
    Some(format!("socks5h://{addr}"))
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

struct RateGate {
    next_allowed: HashMap<String, Instant>,
    interval: Duration,
}

impl RateGate {
    fn new() -> Self {
        let ms = std::env::var("ARCHIVE_SEARCH_MIN_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(DEFAULT_MIN_INTERVAL_MS);
        RateGate {
            next_allowed: HashMap::new(),
            interval: Duration::from_millis(ms),
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
            .insert(host.to_string(), Instant::now() + self.interval);
    }

    fn back_off(&mut self, host: &str, secs: u64) {
        self.next_allowed
            .insert(host.to_string(), Instant::now() + Duration::from_secs(secs));
    }
}

static RATE_GATE: OnceLock<Mutex<RateGate>> = OnceLock::new();

pub(crate) fn get(url: &str, extra: &[&str], timeout: &str) -> Option<Fetch> {
    let host = url_host(url);
    let gate = RATE_GATE.get_or_init(|| Mutex::new(RateGate::new()));
    let ladder = exits();
    let result = first_answer(&ladder, |exit| {
        if let Ok(mut g) = gate.lock() {
            g.wait(&host);
        }
        let result = get_once(url, extra, timeout, exit);
        if let Some(f) = &result {
            if f.status == Some(429) {
                if let Ok(mut g) = gate.lock() {
                    g.back_off(&host, f.retry_after.unwrap_or(DEFAULT_RETRY_AFTER_SECS));
                }
            }
        }
        result
    });
    if let Some(f) = &result {
        if crate::token::is_unauthorized(f.status) && crate::token::is_earthdata_host(&host) {
            if let Some(token) = crate::token::earthdata_token() {
                let auth = format!("Authorization: Bearer {}", token);
                let mut merged: Vec<&str> = extra.to_vec();
                merged.push("-H");
                merged.push(auth.as_str());
                if let Ok(mut g) = gate.lock() {
                    g.wait(&host);
                }
                if let Some(retried) = get_once(url, &merged, timeout, &Exit::Direct) {
                    return Some(retried);
                }
            }
        }
    }
    result
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

fn stage(lines: &mut Vec<String>, n: u8, name: &str, url: &str, r: Option<Fetch>) {
    match r {
        Some(f) => stage_result(lines, n, name, url, f),
        None => lines.push(format!("  stage {} {}: pending — no response", n, name)),
    }
}

fn stage_result(lines: &mut Vec<String>, n: u8, name: &str, url: &str, f: Fetch) {
    if f.status == Some(200) && !f.body.trim().is_empty() {
        lines.push(format!(
            "  stage {} {}: HTTP 200 ({} bytes) — found",
            n,
            name,
            f.body.len()
        ));
        lines.push(format!("url {}", url));
    } else {
        lines.push(format!(
            "  stage {} {}: HTTP {} ({} bytes) — absent",
            n,
            name,
            f.status_text(),
            f.body.len()
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

pub fn verdict_lines(url: &str) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("verdict {} — three-stage ladder", url));
    stage(
        &mut lines,
        1,
        "direct",
        url,
        get_once(url, &[], "30", &Exit::Direct),
    );
    let proxies: Vec<Exit> = exits()
        .into_iter()
        .filter(|exit| !matches!(exit, Exit::Direct))
        .collect();
    if proxies.is_empty() {
        lines.push("  stage 2 proton: absent — no proton transport is up".to_string());
    } else {
        for exit in &proxies {
            let label = exit.label();
            match get_once(url, &[], "30", exit) {
                Some(f) => stage_result(&mut lines, 2, &label, url, f),
                None => lines.push(format!("  stage 2 {}: pending — no response", label)),
            }
        }
    }
    let cdx = format!(
        "https://web.archive.org/cdx/search/cdx?url={}&output=json&limit=1",
        urlencode(url)
    );
    match get(&cdx, &[], "40") {
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

pub fn arxiv_lines(query: &str, max: usize) -> Vec<String> {
    let url = format!(
        "http://export.arxiv.org/api/query?search_query=all:{}&max_results={}",
        urlencode(query),
        max
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let entries = parse_atom_entries(&f.body);
            if entries.is_empty() {
                vec![format!(
                    "absent — the arxiv register carries no entry: {}",
                    query
                )]
            } else {
                entries
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
                    .collect()
            }
        }
        Some(f) => vec![format!("pending — arxiv HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn doc_title(doc: &Json) -> &str {
    doc.get("title")
        .and_then(|t| t.as_arr())
        .and_then(|a| a.first())
        .and_then(|s| s.as_str())
        .unwrap_or("")
}

pub fn ads_lines(query: &str, token: &str, max: usize) -> Vec<String> {
    if token.is_empty() {
        return vec!["pending — NASA_ADS_TOKEN absent from .secrets.local/.env".to_string()];
    }
    let url = format!(
        "https://api.adsabs.harvard.edu/v1/search/query?q={}&fl=title,bibcode&rows={}",
        urlencode(query),
        max
    );
    let auth = format!("Authorization: Bearer {}", token);
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
                        let bib = doc.get("bibcode").and_then(|b| b.as_str()).unwrap_or("");
                        if !bib.is_empty() {
                            out.push(format!(
                                "url https://ui.adsabs.harvard.edu/abs/{}\ttitle: {}",
                                bib,
                                doc_title(doc)
                            ));
                        }
                    }
                }
                if out.is_empty() {
                    vec![format!(
                        "absent — the ADS register carries no entry: {}",
                        query
                    )]
                } else {
                    out
                }
            }
            None => vec!["pending — the ADS response carries no JSON".to_string()],
        },
        Some(f) => vec![format!("pending — ADS HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn ntrs_id_form(query: &str) -> bool {
    let t = query.trim();
    t.len() >= 6 && t.chars().all(|c| c.is_ascii_digit())
}

pub fn ntrs_lines(query: &str, max: usize) -> Vec<String> {
    if ntrs_id_form(query) {
        let id = query.trim();
        let url = format!("https://ntrs.nasa.gov/api/citations/{}", id);
        return match get(&url, &[], "40") {
            Some(f) if f.status == Some(200) => match json::parse(&f.body) {
                Some(v) => {
                    let title = v.get("title").and_then(|t| t.as_str()).unwrap_or("");
                    let rid = match v.get("id").and_then(|i| i.as_scalar_string()) {
                        Some(s) => s,
                        None => id.to_string(),
                    };
                    vec![format!(
                        "url https://ntrs.nasa.gov/citations/{}\ttitle: {}",
                        rid, title
                    )]
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
                            let title = doc.get("title").and_then(|t| t.as_str()).unwrap_or("");
                            out.push(format!(
                                "url https://ntrs.nasa.gov/citations/{}\ttitle: {}",
                                id, title
                            ));
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
                let mut out = Vec::new();
                if let Some(rows) = v.as_arr() {
                    let header = rows.first().and_then(|r| r.as_arr());
                    let (ti, oi, si) = match header {
                        Some(h) => cdx_indices(h),
                        None => (0, 1, 2),
                    };
                    let start = match header {
                        Some(h) if cdx_is_header(h) => 1,
                        Some(_) | None => 0,
                    };
                    for row in rows.iter().skip(start) {
                        let cells = match row.as_arr() {
                            Some(c) => c,
                            None => continue,
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
                }
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

pub fn crossref_lines(query: &str, max: usize) -> Vec<String> {
    let mut cursor: Option<String> = None;
    let (mut lines, stop) = crate::paged::follow_pages(crate::paged::DEFAULT_PAGE_BUDGET, |_| {
        let mut url = format!(
            "https://api.crossref.org/works?query={}&rows={}&select=DOI,title,issued",
            urlencode(query),
            max
        );
        if let Some(c) = &cursor {
            url.push_str("&cursor=");
            url.push_str(&urlencode(c));
        }
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
                            let doi = item.get("DOI").and_then(|d| d.as_str()).unwrap_or("");
                            if !doi.is_empty() {
                                out.push(format!(
                                    "url https://doi.org/{}\ttitle: {}",
                                    doi,
                                    doc_title(item)
                                ));
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
                        let title = hit.get("title").and_then(|t| t.as_str()).unwrap_or("");
                        if !title.is_empty() {
                            out.push(format!(
                                "url https://en.wikipedia.org/wiki/{}",
                                title.replace(' ', "_")
                            ));
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

fn extract_hrefs(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    let lower = html.to_lowercase();
    let mut pos = 0;
    while let Some(rel) = lower[pos..].find("href=\"") {
        let idx = pos + rel + 6;
        let rest = &html[idx..];
        let Some(end) = rest.find('"') else { break };
        let href = rest[..end].to_string();
        if !href.is_empty() && !out.contains(&href) {
            out.push(href);
        }
        pos = idx + end;
    }
    out
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
        Some(f) => vec![format!("pending — Brave HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn librs_lines(query: &str) -> Vec<String> {
    let url = format!("https://lib.rs/search?q={}", urlencode(query));
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let mut out = Vec::new();
            for href in extract_hrefs(&f.body) {
                if href.contains("/crates/") {
                    let full = if href.starts_with("http") {
                        href
                    } else {
                        format!("https://lib.rs{}", href)
                    };
                    if !out.contains(&full) {
                        out.push(format!("url {}", full));
                    }
                }
            }
            out.truncate(20);
            if out.is_empty() {
                vec![format!("absent — lib.rs carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => vec![format!("pending — lib.rs HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn magic_label(magic: crate::magic::Magic) -> &'static str {
    match magic {
        crate::magic::Magic::Pdf => "pdf",
        crate::magic::Magic::Zip => "zip",
        crate::magic::Magic::Fits => "fits",
        crate::magic::Magic::Png => "png",
        crate::magic::Magic::Gzip => "gzip",
        crate::magic::Magic::Hdf5 => "hdf5",
        crate::magic::Magic::NetCdf => "netcdf",
        crate::magic::Magic::Tiff => "tiff",
        crate::magic::Magic::Unrecognized => "unrecognized",
    }
}

pub fn sniff_lines(url: &str) -> Vec<String> {
    match get(url, &[], "40") {
        Some(f) => {
            let bytes = f.body.as_bytes();
            vec![
                format!("url {}", url),
                format!("status {}", f.status_text()),
                format!("bytes {}", bytes.len()),
                format!("magic {}", magic_label(crate::magic::magic_identity(bytes))),
                format!("sha256 {}", omegaflow::sha256::sha256_hex(bytes)),
            ]
        }
        None => vec!["pending — no network".to_string()],
    }
}

pub fn run_lines(mode: &str, query: &str, env: &HashMap<String, String>) -> Vec<String> {
    let max = 10usize;
    match mode {
        "arxiv" => arxiv_lines(query, max),
        "ads" => {
            let token = resolve_secret(
                env.get("NASA_ADS_TOKEN").map(String::as_str).unwrap_or(""),
                env,
            );
            ads_lines(query, &token, max)
        }
        "ntrs" => ntrs_lines(query, max),
        "wayback" => wayback_lines(query, max),
        "crossref" => crossref_lines(query, max),
        "wiki" => wiki_lines(query, max),
        "github" => {
            let token = resolve_secret(
                env.get("OMEGAFLOW_TOKEN").map(String::as_str).unwrap_or(""),
                env,
            );
            github_lines(query, &token, max)
        }
        "crates" => crates_lines(query, max),
        "librs" => librs_lines(query),
        "brave" => {
            let token = resolve_secret(
                env.get("BRAVE_API_KEY").map(String::as_str).unwrap_or(""),
                env,
            );
            brave_lines(query, &token, max)
        }
        "datacite" => crate::datacite::datacite_lines(query, max),
        "zenodo" => crate::zenodo::zenodo_lines(query, max),
        "isc" => crate::isc::isc_lines(query, max),
        "openalex" => crate::openalex::openalex_lines(query, max),
        "sniff" => sniff_lines(query),
        "verdict" => verdict_lines(query),
        other => vec![format!("absent — no mode named {}", other)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urlencode_escapes_reserved_bytes() {
        assert_eq!(urlencode("a b/c?d=e"), "a%20b%2Fc%3Fd%3De");
        assert_eq!(urlencode("safe-._~"), "safe-._~");
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
    fn href_extraction_dedups() {
        let html = "<a href=\"/crates/serde\">x</a><a href=\"/crates/serde\">y</a>";
        assert_eq!(extract_hrefs(html), vec!["/crates/serde".to_string()]);
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
            retry_after: None,
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
    fn rate_gate_backs_off_the_host() {
        let mut gate = RateGate {
            next_allowed: HashMap::new(),
            interval: Duration::from_millis(0),
        };
        gate.back_off("example.com", 5);
        let until = gate.next_allowed.get("example.com").copied();
        assert!(until.is_some());
        assert!(until.unwrap() > Instant::now());
    }
}
