use std::env;
use std::path::PathBuf;
use std::process::Command;

struct Entry {
    kind: String,
    session: String,
    time: String,
    text: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let home = match env::var("HOME") {
        Ok(h) => h,
        Err(_) => {
            eprintln!("HOME is absent — the opencode database path does not resolve");
            std::process::exit(2);
        }
    };
    let mut db = PathBuf::from(home).join(".local/share/opencode/opencode.db");
    let mut since: i64 = 0;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--db" => {
                i += 1;
                db = PathBuf::from(&args[i]);
            }
            "--since" => {
                i += 1;
                since = match args[i].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("--since: not a number");
                        std::process::exit(2);
                    }
                };
            }
            _ => {}
        }
        i += 1;
    }

    let out = Command::new("sqlite3")
        .arg(&db)
        .arg("-separator")
        .arg("\x1f") 
        .arg("SELECT s.title, p.time_created, json_extract(p.data,'$.text') FROM part p JOIN session s ON s.id=p.session_id WHERE json_extract(p.data,'$.type')='text' ORDER BY p.time_created;")
        .output();
    let out = match out {
        Ok(o) => o,
        Err(e) => {
            eprintln!("pending_extract: sqlite3 failed: {}", e);
            std::process::exit(1);
        }
    };
    if !out.status.success() {
        eprintln!(
            "pending_extract: sqlite3 error: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        std::process::exit(1);
    }
    let text = String::from_utf8_lossy(&out.stdout).to_string();

    let mut entries: Vec<Entry> = Vec::new();
    for line in text.lines() {
        let mut parts = line.splitn(3, '\x1f');
        let session = parts.next().unwrap_or("").to_string();
        let time = parts.next().unwrap_or("").to_string();
        let body = parts.next().unwrap_or("").to_string();

        let all: Vec<&str> = body.lines().filter(|l| !l.trim().is_empty()).collect();
        let start = all.len().saturating_sub(15);
        for bl in all.into_iter().skip(start) {
            let trimmed = bl.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(kind) = classify(trimmed) {
                let short = truncate(&trimmed, 120);
                let Ok(epoch) = time.parse::<i64>() else {
                    continue;
                };
                if epoch < since {
                    continue;
                }

                if trimmed == "text" {
                    continue;
                }
                entries.push(Entry {
                    kind: kind.to_string(),
                    session: session.clone(),
                    time: time.clone(),
                    text: short,
                });
            }
        }
    }

    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    entries.retain(|e| seen.insert(format!("{}|{}|{}", e.session, e.text, e.kind)));
    entries.sort_by(|a, b| a.session.cmp(&b.session).then(a.time.cmp(&b.time)));

    for e in &entries {
        println!("{} {} {} {} {}", e.kind, e.session, e.time, e.text, "");
    }
    println!("COUNT {} {}", entries.len(), db.display());
}

fn classify(trimmed: &str) -> Option<&'static str> {
    let raw = trimmed.trim();
    if raw.len() > 150 {
        return None;
    }

    let mut t = raw.trim_start_matches('*').trim_start();
    t = t.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ' ');
    t = t.trim_start_matches(['-', '*', '•', ' ']);
    t = t.trim();
    if t.is_empty() {
        return None;
    }
    let low = t.to_lowercase();
    if !listed(OPEN_MARKERS, &low) {
        return None;
    }
    if listed(PENDING_MARKERS, &low) {
        Some("PENDING")
    } else if listed(WAIT_MARKERS, &low) {
        Some("WAIT")
    } else {
        Some("OPEN")
    }
}

const OPEN_MARKERS: &str = include_str!("../../data/open_marker.txt");
const PENDING_MARKERS: &str = include_str!("../../data/pending_marker.txt");
const WAIT_MARKERS: &str = include_str!("../../data/wait_marker.txt");

fn listed(list: &str, text: &str) -> bool {
    list.lines()
        .any(|w| !w.trim().is_empty() && text.contains(w.trim()))
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut r = s.chars().take(n).collect::<String>();
        r.push('…');
        r
    }
}
