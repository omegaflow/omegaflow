use crate::json;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const CACHE_VERSION: &str = "v2";
const OK_TTL_SECS: u64 = 3600;
const ABSENT_TTL_SECS: u64 = 3600;
const NEGATIVE_TTL_SECS: u64 = 300;
const BREAKER_FAILURES: u64 = 3;
const BREAKER_SECS: u64 = 600;
const NO_COUNT: u64 = 0;
const LOCK_STALE_SECS: u64 = 30;
const LOCK_ATTEMPTS: usize = 500;
const LOCK_SLEEP_MS: u64 = 10;
const NEGATIVE_STATUSES: &[i32] = &[400, 402, 403, 429, 432];
const BREAKER_SUBDIR: &str = "breaker";

static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);
static DISABLED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Ok,
    Absent,
    Negative,
}

impl Kind {
    fn as_str(self) -> &'static str {
        match self {
            Kind::Ok => "ok",
            Kind::Absent => "absent",
            Kind::Negative => "negative",
        }
    }

    fn from_str(text: &str) -> Option<Self> {
        match text {
            "ok" => Some(Kind::Ok),
            "absent" => Some(Kind::Absent),
            "negative" => Some(Kind::Negative),
            _ => None,
        }
    }

    fn ttl(self) -> u64 {
        match self {
            Kind::Negative => NEGATIVE_TTL_SECS,
            Kind::Absent => env_ttl(ABSENT_TTL_SECS),
            Kind::Ok => env_ttl(OK_TTL_SECS),
        }
    }
}

fn env_ttl(fallback: u64) -> u64 {
    std::env::var("OMEGAFLOW_CACHE_TTL")
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(fallback)
}

pub fn disabled() -> bool {
    DISABLED.load(Ordering::Relaxed)
        || std::env::var("OMEGAFLOW_NO_CACHE")
            .map(|v| !v.trim().is_empty() && v.trim() != "0")
            .unwrap_or(false)
}

pub fn disable() {
    DISABLED.store(true, Ordering::Relaxed);
}

fn cache_dir() -> Option<PathBuf> {
    let dir = match std::env::var("OMEGAFLOW_CACHE_DIR") {
        Ok(v) if !v.trim().is_empty() => PathBuf::from(v),
        _ => {
            let home = std::env::var("HOME").ok()?;
            PathBuf::from(home).join(".cache/omegaflow/archive_search")
        }
    };
    fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

fn now() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs())
}

fn fnv1a(text: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in text.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn body_of(line: &str) -> &str {
    if line.starts_with('[') {
        if let Some((_, body)) = line.split_once("] ") {
            return body;
        }
    }
    line
}

fn pending_http(body: &str) -> Option<i32> {
    if !body.starts_with("pending") {
        return None;
    }
    let rest = body.split("HTTP ").nth(1)?;
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn negative_status(status: i32) -> bool {
    NEGATIVE_STATUSES.contains(&status)
}

fn classify(lines: &[String]) -> Option<Kind> {
    if lines.is_empty() {
        return None;
    }
    let mut result = false;
    let mut absent = false;
    let mut pending = false;
    let mut negative = false;
    let mut held = false;
    for line in lines {
        let body = body_of(line);
        if body.starts_with("url ") || body.starts_with("identifier ") {
            result = true;
        } else if let Some(status) = pending_http(body) {
            pending = true;
            if negative_status(status) {
                negative = true;
            }
        } else if body.starts_with("pending") {
            pending = true;
        } else if body.starts_with("absent") {
            absent = true;
        } else if body.starts_with("usage") || body.starts_with("note:") {
            held = true;
        }
    }
    if result {
        Some(Kind::Ok)
    } else if negative {
        Some(Kind::Negative)
    } else if pending || held {
        None
    } else if absent {
        Some(Kind::Absent)
    } else {
        Some(Kind::Ok)
    }
}

fn entry_path(dir: &Path, mode: &str, query: &str, max: usize) -> PathBuf {
    let key = format!("{CACHE_VERSION}|{mode}|{query}|{max}");
    dir.join(format!("{mode}-{:016x}.json", fnv1a(&key)))
}

fn breaker_path(dir: &Path, host: &str) -> PathBuf {
    dir.join(BREAKER_SUBDIR)
        .join(format!("host-{:016x}.json", fnv1a(host)))
}

fn read_json(path: &Path) -> Option<json::Json> {
    let text = fs::read_to_string(path).ok()?;
    json::parse(&text)
}

fn read_number(v: &json::Json, key: &str) -> Option<u64> {
    v.get(key)?.as_scalar_string()?.parse().ok()
}

fn read_lines(v: &json::Json) -> Option<Vec<String>> {
    let lines = v
        .get("lines")?
        .as_arr()?
        .iter()
        .filter_map(|l| l.as_str().map(str::to_string))
        .collect::<Vec<_>>();
    if lines.is_empty() {
        None
    } else {
        Some(lines)
    }
}

fn load_at(dir: &Path, mode: &str, query: &str, max: usize, now: u64) -> Option<Vec<String>> {
    let path = entry_path(dir, mode, query, max);
    let v = read_json(&path)?;
    let ts = read_number(&v, "ts")?;
    let kind = v
        .get("kind")
        .and_then(|k| k.as_str())
        .and_then(Kind::from_str)?;
    if now.saturating_sub(ts) > kind.ttl() {
        let _ = fs::remove_file(&path);
        return None;
    }
    read_lines(&v)
}

pub fn load(mode: &str, query: &str, max: usize) -> Option<Vec<String>> {
    if disabled() {
        return None;
    }
    let dir = cache_dir()?;
    let ts = now()?;
    let hit = load_at(&dir, mode, query, max, ts);
    record_stat(&dir, hit.is_some());
    hit
}

fn atomic_write(path: &Path, body: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let n = TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let tmp = path.with_extension(format!("tmp-{}-{n}", std::process::id()));
    {
        let mut file = fs::File::create(&tmp)?;
        file.write_all(body.as_bytes())?;
        file.sync_all()?;
    }
    fs::rename(&tmp, path).map_err(|e| {
        let _ = fs::remove_file(&tmp);
        e
    })
}

struct Lock {
    path: PathBuf,
}

impl Drop for Lock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn lock_path(dir: &Path) -> PathBuf {
    dir.join(".lock")
}

fn acquire_lock(dir: &Path) -> Option<Lock> {
    fs::create_dir_all(dir).ok()?;
    let path = lock_path(dir);
    for _ in 0..LOCK_ATTEMPTS {
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut file) => {
                let _ = writeln!(file, "{}", std::process::id());
                return Some(Lock { path });
            }
            Err(_) => {
                if let Ok(meta) = fs::metadata(&path) {
                    if let Ok(modified) = meta.modified() {
                        if modified
                            .elapsed()
                            .map(|age| age > Duration::from_secs(LOCK_STALE_SECS))
                            .unwrap_or(false)
                        {
                            let _ = fs::remove_file(&path);
                            continue;
                        }
                    }
                }
                std::thread::sleep(Duration::from_millis(LOCK_SLEEP_MS));
            }
        }
    }
    None
}

fn store_at(dir: &Path, mode: &str, query: &str, max: usize, lines: &[String], now: u64) -> bool {
    let Some(kind) = classify(lines) else {
        return false;
    };
    let Some(_lock) = acquire_lock(dir) else {
        return false;
    };
    let path = entry_path(dir, mode, query, max);
    let body = format!(
        "{{\"ts\":{},\"kind\":\"{}\",\"lines\":[{}]}}",
        now,
        kind.as_str(),
        lines
            .iter()
            .map(|l| format!("\"{}\"", escape(l)))
            .collect::<Vec<_>>()
            .join(",")
    );
    atomic_write(&path, &body).is_ok()
}

pub fn store(mode: &str, query: &str, max: usize, lines: &[String]) {
    if disabled() {
        return;
    }
    let Some(dir) = cache_dir() else {
        return;
    };
    let Some(ts) = now() else {
        return;
    };
    store_at(&dir, mode, query, max, lines, ts);
}

fn stats_path(dir: &Path) -> PathBuf {
    dir.join("stats.json")
}

fn read_stats_in(dir: &Path) -> (u64, u64) {
    let Some(v) = read_json(&stats_path(dir)) else {
        return (0, 0);
    };
    (
        read_number(&v, "hits").unwrap_or(NO_COUNT),
        read_number(&v, "misses").unwrap_or(NO_COUNT),
    )
}

fn write_stats_in(dir: &Path, hits: u64, misses: u64) {
    let body = format!("{{\"hits\":{hits},\"misses\":{misses}}}");
    let _ = atomic_write(&stats_path(dir), &body);
}

fn record_stat(dir: &Path, hit: bool) {
    let Some(_lock) = acquire_lock(dir) else {
        return;
    };
    let (hits, misses) = read_stats_in(dir);
    if hit {
        write_stats_in(dir, hits.saturating_add(1), misses);
    } else {
        write_stats_in(dir, hits, misses.saturating_add(1));
    }
}

pub struct Stats {
    pub dir: PathBuf,
    pub hits: u64,
    pub misses: u64,
    pub bytes: u64,
    pub blocked: Vec<(String, u64)>,
}

fn dir_bytes(dir: &Path) -> u64 {
    let mut total = 0u64;
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return total,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        match entry.metadata() {
            Ok(meta) if meta.is_dir() => total = total.saturating_add(dir_bytes(&path)),
            Ok(meta) if meta.is_file() => total = total.saturating_add(meta.len()),
            _ => {}
        }
    }
    total
}

fn blocked_in(dir: &Path, now: u64) -> Vec<(String, u64)> {
    let path = dir.join(BREAKER_SUBDIR);
    let mut out = Vec::new();
    let entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(_) => return out,
    };
    for entry in entries.flatten() {
        let Some(v) = read_json(&entry.path()) else {
            continue;
        };
        let host = match v.get("host").and_then(|h| h.as_str()) {
            Some(host) => host.to_string(),
            None => continue,
        };
        let until = read_number(&v, "until").unwrap_or(NO_COUNT);
        if until > now {
            out.push((host, until - now));
        }
    }
    out.sort();
    out
}

pub fn stats() -> Option<Stats> {
    let dir = cache_dir()?;
    let ts = now()?;
    let (hits, misses) = read_stats_in(&dir);
    Some(Stats {
        dir: dir.clone(),
        hits,
        misses,
        bytes: dir_bytes(&dir),
        blocked: blocked_in(&dir, ts),
    })
}

pub fn clear() -> Option<(PathBuf, usize, u64)> {
    let dir = cache_dir()?;
    let _lock = acquire_lock(&dir);
    let mut files = 0usize;
    let mut bytes = 0u64;
    let entries = fs::read_dir(&dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path == lock_path(&dir) {
            continue;
        }
        let meta = entry.metadata().ok();
        let is_dir = meta.as_ref().is_some_and(|m| m.is_dir());
        if is_dir {
            bytes = bytes.saturating_add(dir_bytes(&path));
            files = files.saturating_add(count_files(&path));
        } else if let Some(meta) = &meta {
            bytes = bytes.saturating_add(meta.len());
            files = files.saturating_add(1);
        }
        if is_dir {
            let _ = fs::remove_dir_all(&path);
        } else {
            let _ = fs::remove_file(&path);
        }
    }
    Some((dir, files, bytes))
}

fn count_files(dir: &Path) -> usize {
    let mut count = 0usize;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            match entry.metadata() {
                Ok(meta) if meta.is_dir() => count += count_files(&entry.path()),
                Ok(meta) if meta.is_file() => count += 1,
                _ => {}
            }
        }
    }
    count
}

fn breaker_failure_status(status: Option<i32>) -> bool {
    match status {
        None => true,
        Some(0) => true,
        Some(status) => negative_status(status) || status >= 500,
    }
}

pub fn breaker_open(host: &str) -> Option<u64> {
    if disabled() {
        return None;
    }
    let dir = cache_dir()?;
    breaker_remaining(&dir, host, now()?)
}

fn breaker_remaining(dir: &Path, host: &str, now: u64) -> Option<u64> {
    let path = breaker_path(dir, host);
    let v = read_json(&path)?;
    let until = read_number(&v, "until").unwrap_or(NO_COUNT);
    if until > now {
        Some(until - now)
    } else {
        if until != 0 {
            let _ = fs::remove_file(&path);
        }
        None
    }
}

pub fn breaker_record(host: &str, status: Option<i32>) {
    if disabled() {
        return;
    }
    let Some(dir) = cache_dir() else {
        return;
    };
    let Some(ts) = now() else {
        return;
    };
    breaker_record_in(&dir, host, status, ts);
}

fn breaker_record_in(dir: &Path, host: &str, status: Option<i32>, now: u64) {
    if host.is_empty() {
        return;
    }
    let Some(_lock) = acquire_lock(dir) else {
        return;
    };
    let path = breaker_path(dir, host);
    let mut failures = read_json(&path)
        .and_then(|v| read_number(&v, "failures"))
        .unwrap_or(NO_COUNT);
    let mut until = read_json(&path)
        .and_then(|v| read_number(&v, "until"))
        .unwrap_or(NO_COUNT);
    if until > now {
        return;
    }
    if breaker_failure_status(status) {
        failures = failures.saturating_add(1);
        if failures >= BREAKER_FAILURES {
            failures = 0;
            until = now.saturating_add(BREAKER_SECS);
        }
    } else {
        failures = 0;
        until = 0;
    }
    if failures == 0 && until == 0 {
        let _ = fs::remove_file(&path);
        return;
    }
    let body = format!(
        "{{\"host\":\"{}\",\"failures\":{},\"until\":{}}}",
        escape(host),
        failures,
        until
    );
    let _ = atomic_write(&path, &body);
}

pub fn store_http_error(url: &str, status: i32, retry_after: Option<u64>) {
    if disabled() || !negative_status(status) {
        return;
    }
    let Some(dir) = cache_dir() else {
        return;
    };
    let Some(ts) = now() else {
        return;
    };
    let _lock = acquire_lock(&dir);
    store_http_in(&dir, url, status, retry_after, ts);
}

pub fn load_http_error(url: &str) -> Option<(i32, Option<u64>)> {
    if disabled() {
        return None;
    }
    let dir = cache_dir()?;
    let ts = now()?;
    let hit = load_http_in(&dir, url, ts);
    record_stat(&dir, hit.is_some());
    hit
}

fn http_key(url: &str) -> String {
    format!("{CACHE_VERSION}|http|{url}")
}

fn http_path(dir: &Path, url: &str) -> PathBuf {
    dir.join(format!("http-{:016x}.json", fnv1a(&http_key(url))))
}

fn store_http_in(
    dir: &Path,
    url: &str,
    status: i32,
    retry_after: Option<u64>,
    now: u64,
) {
    let retry = match retry_after {
        Some(r) => r.to_string(),
        None => "null".to_string(),
    };
    let body = format!(
        "{{\"ts\":{},\"status\":{},\"retry_after\":{}}}",
        now, status, retry
    );
    let _ = atomic_write(&http_path(dir, url), &body);
}

fn load_http_in(dir: &Path, url: &str, now: u64) -> Option<(i32, Option<u64>)> {
    let path = http_path(dir, url);
    let v = read_json(&path)?;
    let ts = read_number(&v, "ts")?;
    if now.saturating_sub(ts) > NEGATIVE_TTL_SECS {
        let _ = fs::remove_file(&path);
        return None;
    }
    let status = read_number(&v, "status")? as i32;
    if !negative_status(status) {
        let _ = fs::remove_file(&path);
        return None;
    }
    let retry_after = v
        .get("retry_after")
        .and_then(|r| r.as_scalar_string())
        .and_then(|r| r.parse().ok());
    Some((status, retry_after))
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_dir() -> PathBuf {
        let n = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "omegaflow-cache-test-{}-{}",
            std::process::id(),
            n
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn lines(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn escapes_quotes_and_tabs() {
        assert_eq!(escape("a\t\"b\"\\c\n"), "a\\t\\\"b\\\"\\\\c\\n");
    }

    #[test]
    fn hashes_are_stable_and_distinct() {
        assert_eq!(fnv1a("abc"), fnv1a("abc"));
        assert_ne!(fnv1a("abc"), fnv1a("abd"));
    }

    #[test]
    fn negative_ttl_is_300_and_absent_is_3600() {
        assert_eq!(Kind::Negative.ttl(), 300);
        assert_eq!(Kind::Absent.ttl(), 3600);
        assert_eq!(Kind::Ok.ttl(), 3600);
    }

    #[test]
    fn blocking_statuses_are_negative() {
        for status in [400, 402, 403, 429, 432] {
            let line = format!("[brave] pending — Brave HTTP {status} (quota)");
            assert_eq!(classify(&[line]), Some(Kind::Negative));
        }
        assert_eq!(
            classify(&["pending — Consensus HTTP 429".to_string()]),
            Some(Kind::Negative)
        );
    }

    #[test]
    fn other_pending_and_usage_are_not_cached() {
        assert_eq!(classify(&["pending — no network".to_string()]), None);
        assert_eq!(
            classify(&["pending — Brave HTTP 500".to_string()]),
            None
        );
        assert_eq!(
            classify(&["pending — Brave HTTP 404".to_string()]),
            None
        );
        assert_eq!(classify(&["usage — archive_search --brave <query>".to_string()]), None);
        assert_eq!(classify(&["note: filter unsupported".to_string()]), None);
    }

    #[test]
    fn absent_and_results_are_cached() {
        assert_eq!(classify(&["absent — crossref carries no entry: x".to_string()]), Some(Kind::Absent));
        assert_eq!(
            classify(&["[brave] url https://x\ttitle: x".to_string()]),
            Some(Kind::Ok)
        );
        assert_eq!(
            classify(&[
                "absent — GH_SEARCH_TOKEN missing".to_string(),
                "url https://github.com/x\ttitle: x".to_string()
            ]),
            Some(Kind::Ok)
        );
    }

    #[test]
    fn store_is_atomic_and_leaves_no_tmp() {
        let dir = temp_dir();
        let body = lines(&["url https://example.test/x\ttitle: x"]);
        assert!(store_at(&dir, "brave", "solar wind", 10, &body, 1000));
        let path = entry_path(&dir, "brave", "solar wind", 10);
        assert!(path.is_file());
        let names: Vec<String> = fs::read_dir(&dir)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        assert_eq!(names, vec![path.file_name().unwrap().to_string_lossy().to_string()]);
        assert_eq!(load_at(&dir, "brave", "solar wind", 10, 1000), Some(body));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn negative_entry_expires_after_300_seconds() {
        let dir = temp_dir();
        let body = lines(&["[brave] pending — Brave HTTP 402 (quota)"]);
        assert!(store_at(&dir, "brave", "x", 10, &body, 1000));
        assert_eq!(load_at(&dir, "brave", "x", 10, 1300), Some(body.clone()));
        assert_eq!(load_at(&dir, "brave", "x", 10, 1301), None);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn absent_entry_expires_after_3600_seconds() {
        let dir = temp_dir();
        let body = lines(&["absent — crossref carries no entry: x"]);
        assert!(store_at(&dir, "crossref", "x", 10, &body, 1000));
        assert_eq!(load_at(&dir, "crossref", "x", 10, 4600), Some(body.clone()));
        assert_eq!(load_at(&dir, "crossref", "x", 10, 4601), None);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn stats_count_hits_misses_and_bytes() {
        let dir = temp_dir();
        let body = lines(&["url https://example.test/x"]);
        assert!(store_at(&dir, "brave", "x", 10, &body, 1000));
        record_stat(&dir, true);
        record_stat(&dir, false);
        record_stat(&dir, false);
        let (hits, misses) = read_stats_in(&dir);
        assert_eq!((hits, misses), (1, 2));
        assert!(dir_bytes(&dir) > 0);
        let stats = Stats {
            dir: dir.clone(),
            hits,
            misses,
            bytes: dir_bytes(&dir),
            blocked: blocked_in(&dir, 1000),
        };
        assert_eq!(stats.blocked.len(), 0);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn circuit_breaker_opens_after_three_failures_and_expires() {
        let dir = temp_dir();
        let host = "api.search.brave.com";
        assert_eq!(breaker_remaining(&dir, host, 1000), None);
        breaker_record_in(&dir, host, Some(429), 1000);
        breaker_record_in(&dir, host, Some(429), 1001);
        assert_eq!(breaker_remaining(&dir, host, 1002), None);
        breaker_record_in(&dir, host, Some(429), 1002);
        assert_eq!(breaker_remaining(&dir, host, 1002), Some(600));
        assert_eq!(breaker_remaining(&dir, host, 1601), Some(1));
        assert_eq!(breaker_remaining(&dir, host, 1602), None);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn circuit_breaker_resets_on_success() {
        let dir = temp_dir();
        let host = "example.test";
        breaker_record_in(&dir, host, None, 1000);
        breaker_record_in(&dir, host, Some(503), 1001);
        breaker_record_in(&dir, host, Some(200), 1002);
        breaker_record_in(&dir, host, Some(429), 1003);
        breaker_record_in(&dir, host, Some(429), 1004);
        assert_eq!(breaker_remaining(&dir, host, 1005), None);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn http_error_cache_hits_negative_status_only() {
        let dir = temp_dir();
        store_http_in(&dir, "https://x.test/q", 402, Some(5), 1000);
        assert_eq!(load_http_in(&dir, "https://x.test/q", 1000), Some((402, Some(5))));
        assert_eq!(load_http_in(&dir, "https://x.test/q", 1301), None);
        store_http_in(&dir, "https://x.test/q", 500, None, 1000);
        assert_eq!(load_http_in(&dir, "https://x.test/q", 1000), None);
        let _ = fs::remove_dir_all(&dir);
    }
}
