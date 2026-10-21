use crate::json;
use crate::net::{get, urlencode};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn base_gate_dir() -> Option<String> {
    if let Ok(d) = std::env::var("OMEGAFLOW_TMP") {
        if !d.trim().is_empty() {
            let _ = fs::create_dir_all(&d);
            return Some(d);
        }
    }
    let d = std::env::temp_dir().join("omegaflow");
    fs::create_dir_all(&d).ok()?;
    Some(d.to_string_lossy().to_string())
}
fn base_gate_lock() -> Option<String> {
    base_gate_dir().map(|d| format!("{d}/archive_search_base.lock"))
}
fn base_gate_stamp() -> Option<String> {
    base_gate_dir().map(|d| format!("{d}/archive_search_base.ts"))
}
const BASE_GATE_SPACING_SECS: f64 = 1.1;
const BASE_GATE_WAIT_SECS: f64 = 5.0;
const BASE_GATE_STALE_SECS: f64 = 10.0;
const BASE_GATE_POLL: Duration = Duration::from_millis(50);

fn unix_now() -> Option<f64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs_f64())
}

fn read_stamp() -> Option<f64> {
    fs::read_to_string(base_gate_stamp()?)
        .ok()?
        .trim()
        .parse::<f64>()
        .ok()
}

fn write_stamp() {
    let Some(stamp) = base_gate_stamp() else {
        return;
    };
    if let Some(now) = unix_now() {
        if let Ok(mut f) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(stamp)
        {
            let _ = f.write_all(format!("{now}\n").as_bytes());
        }
    }
}

fn space_requests() {
    if let Some(prior) = read_stamp() {
        if let Some(now) = unix_now() {
            let wait = BASE_GATE_SPACING_SECS - (now - prior);
            if wait > 0.0 {
                thread::sleep(Duration::from_secs_f64(wait));
            }
        }
    }
}

fn release_if_stale() {
    let stale = match read_stamp() {
        Some(ts) => match unix_now() {
            Some(now) => now - ts > BASE_GATE_STALE_SECS,
            None => false,
        },
        None => true,
    };
    if stale {
        if let Some(lock) = base_gate_lock() {
            let _ = fs::remove_file(lock);
        }
    }
}

fn base_gate() {
    let Some(lock) = base_gate_lock() else {
        return;
    };
    let Some(dir) = base_gate_dir() else { return };
    if fs::create_dir_all(&dir).is_err() {
        return;
    }
    let entered = unix_now();
    loop {
        match OpenOptions::new().write(true).create_new(true).open(&lock) {
            Ok(mut held) => {
                space_requests();
                write_stamp();
                if let Some(now) = unix_now() {
                    let _ = held.write_all(format!("{now}\n").as_bytes());
                }
                drop(held);
                let _ = fs::remove_file(&lock);
                return;
            }
            Err(_) => {
                let waited = match (entered, unix_now()) {
                    (Some(a), Some(b)) => b - a,
                    _ => BASE_GATE_WAIT_SECS,
                };
                if waited >= BASE_GATE_WAIT_SECS {
                    release_if_stale();
                    space_requests();
                    write_stamp();
                    eprintln!(
                        "base gate: lock held {waited:.1}s — spacing enforced without the lock"
                    );
                    return;
                }
                thread::sleep(BASE_GATE_POLL);
            }
        }
    }
}

pub fn base_lines(query: &str, max: usize, key: &str) -> Vec<String> {
    base_gate();
    let url = format!(
        "https://api.base-search.net/cgi-bin/BaseHttpSearchInterface.fcgi?func=PerformSearch&query={}&format=json&hits={}&apikey={}",
        urlencode(query),
        max,
        urlencode(key)
    );
    match get(&url, &["-H", "User-Agent: omegaflow-archive-search"], "40") {
        Some(f) if f.status == Some(200) => match json::parse(&f.body) {
            Some(v) => {
                if let Some(denial) = v.get("error").and_then(|e| e.as_str()) {
                    return vec![format!("pending — base: {}", denial)];
                }
                let out = parse_base(&f.body);
                if out.is_empty() {
                    vec![format!("absent — base carries no entry: {}", query)]
                } else {
                    out
                }
            }
            None => vec!["pending — the base response carries no JSON".to_string()],
        },
        Some(f) if f.status == Some(401) || f.status == Some(403) => vec![format!(
            "pending — base HTTP {} (access denied)",
            f.status_text()
        )],
        Some(f) => vec![format!("pending — base HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn scalar(doc: &json::Json, key: &str) -> Option<String> {
    let v = doc.get(key)?;
    if let Some(s) = v.as_str() {
        return (!s.is_empty()).then(|| s.to_string());
    }
    if let Some(arr) = v.as_arr() {
        for item in arr {
            if let Some(s) = item.as_str() {
                if !s.is_empty() {
                    return Some(s.to_string());
                }
            }
        }
    }
    v.as_scalar_string().filter(|s| !s.is_empty())
}

fn http_identifier(doc: &json::Json) -> Option<String> {
    let ids = doc.get("dcidentifier").and_then(|i| i.as_arr())?;
    for id in ids {
        if let Some(s) = id.as_str() {
            if s.starts_with("http") {
                return Some(s.to_string());
            }
        }
    }
    None
}

fn doi_line(doc: &json::Json) -> Option<String> {
    if let Some(doi) = scalar(doc, "dcdoi") {
        return Some(doi);
    }
    let ids = doc.get("dcidentifier").and_then(|i| i.as_arr())?;
    for id in ids {
        if let Some(s) = id.as_str() {
            if let Some(rest) = s.strip_prefix("info:doi/") {
                if !rest.is_empty() {
                    return Some(rest.to_string());
                }
            }
        }
    }
    None
}

fn authors_line(doc: &json::Json) -> Option<String> {
    let raw = doc.get("dccreator")?;
    let mut names: Vec<String> = Vec::new();
    if let Some(items) = raw.as_arr() {
        for item in items {
            if let Some(s) = item.as_str() {
                if !s.is_empty() {
                    names.push(s.to_string());
                }
            }
        }
    } else if let Some(s) = raw.as_str() {
        if !s.is_empty() {
            names.push(s.to_string());
        }
    }
    if names.is_empty() {
        return None;
    }
    if names.len() > 8 {
        let mut capped = names[..8].join(", ");
        capped.push_str(", et al.");
        Some(capped)
    } else {
        Some(names.join(", "))
    }
}

fn parse_base(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(v) = json::parse(body) else {
        return out;
    };
    let Some(docs) = v
        .get("response")
        .and_then(|r| r.get("docs"))
        .and_then(|d| d.as_arr())
    else {
        return out;
    };
    for doc in docs {
        let Some(url) = http_identifier(doc) else {
            continue;
        };
        let mut line = format!("url {}", url);
        if let Some(title) = scalar(doc, "dctitle") {
            line.push_str(&format!("\ttitle: {}", title));
        }
        if let Some(authors) = authors_line(doc) {
            line.push_str(&format!("\tauthors: {}", authors));
        }
        if let Some(year) = scalar(doc, "dcyear") {
            line.push_str(&format!("\tyear: {}", year));
        }
        if let Some(doi) = doi_line(doc) {
            line.push_str(&format!("\tdoi: {}", doi));
        }
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_doc_fields() {
        let body = r#"{"response":{"numFound":1,"docs":[{"dctitle":"A measured field","dccreator":["Ada Lovelace","Alan Turing"],"dcidentifier":["info:eu-repo/semantics/openAccess","https://example.org/a.pdf"],"dcyear":"2019","dcdoi":"10.1234/abc","dcdescription":"body"}]}}"#;
        assert_eq!(
            parse_base(body),
            vec!["url https://example.org/a.pdf\ttitle: A measured field\tauthors: Ada Lovelace, Alan Turing\tyear: 2019\tdoi: 10.1234/abc".to_string()]
        );
    }

    #[test]
    fn doc_without_identifier_is_skipped() {
        let body = r#"{"response":{"numFound":1,"docs":[{"dctitle":"Solo","dcyear":"2020"}]}}"#;
        assert!(parse_base(body).is_empty());
    }

    #[test]
    fn caps_the_authors_to_eight_with_et_al() {
        let names = (1..=9)
            .map(|n| format!("\"Author {n}\""))
            .collect::<Vec<_>>()
            .join(",");
        let body = format!(
            r#"{{"response":{{"docs":[{{"dctitle":"Many","dccreator":[{names}],"dcidentifier":["https://example.org/many.pdf"]}}]}}}}"#
        );
        assert_eq!(
            parse_base(&body),
            vec!["url https://example.org/many.pdf\ttitle: Many\tauthors: Author 1, Author 2, Author 3, Author 4, Author 5, Author 6, Author 7, Author 8, et al.".to_string()]
        );
    }

    #[test]
    fn empty_docs_carry_nothing() {
        assert!(parse_base(r#"{"response":{"numFound":0,"docs":[]}}"#).is_empty());
    }
}
