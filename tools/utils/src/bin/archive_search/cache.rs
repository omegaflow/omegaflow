use crate::json;
use std::fs;
use std::path::PathBuf;

const CACHE_VERSION: &str = "v1";

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

fn ttl_secs() -> u64 {
    std::env::var("OMEGAFLOW_CACHE_TTL")
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(3600)
}

fn disabled() -> bool {
    std::env::var("OMEGAFLOW_NO_CACHE")
        .map(|v| !v.trim().is_empty() && v.trim() != "0")
        .unwrap_or(false)
}

fn fnv1a(text: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in text.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn path_for(mode: &str, query: &str, max: usize) -> Option<PathBuf> {
    let key = format!("{CACHE_VERSION}|{mode}|{query}|{max}");
    Some(cache_dir()?.join(format!("{mode}-{:016x}.json", fnv1a(&key))))
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn load(mode: &str, query: &str, max: usize) -> Option<Vec<String>> {
    if disabled() {
        return None;
    }
    let path = path_for(mode, query, max)?;
    let text = fs::read_to_string(&path).ok()?;
    let v = json::parse(&text)?;
    let ts = v.get("ts")?.as_scalar_string()?.parse::<u64>().ok()?;
    if now().saturating_sub(ts) > ttl_secs() {
        let _ = fs::remove_file(&path);
        return None;
    }
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

pub fn store(mode: &str, query: &str, max: usize, lines: &[String]) {
    if disabled() || !cacheable(lines) {
        return;
    }
    let Some(path) = path_for(mode, query, max) else {
        return;
    };
    let body = format!(
        "{{\"ts\":{},\"lines\":[{}]}}",
        now(),
        lines
            .iter()
            .map(|l| format!("\"{}\"", escape(l)))
            .collect::<Vec<_>>()
            .join(",")
    );
    let _ = fs::write(path, body);
}

fn cacheable(lines: &[String]) -> bool {
    let Some(first) = lines.first() else {
        return false;
    };
    let body = first.split_once("] ").map(|(_, body)| body).unwrap_or(first);
    !body.starts_with("pending")
        && !body.starts_with("absent")
        && !body.starts_with("usage")
        && !body.starts_with("note:")
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
    fn pending_and_absent_are_not_cached() {
        assert!(!cacheable(&["pending — x".to_string()]));
        assert!(!cacheable(&["absent — x".to_string()]));
        assert!(!cacheable(&["[brave] pending — x".to_string()]));
        assert!(cacheable(&["url https://x\ttitle: x".to_string()]));
        assert!(cacheable(&["[brave] url https://x\ttitle: x".to_string()]));
    }
}
