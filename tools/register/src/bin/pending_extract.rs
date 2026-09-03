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
    let mut db = PathBuf::from(env::var("HOME").unwrap_or_default())
        .join(".local/share/opencode/opencode.db");
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
                since = args[i].parse().unwrap_or(0);
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
                let epoch: i64 = time.parse().unwrap_or(0);
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
    let has_marker = low.contains("offen:")
        || low.contains("pending")
        || low.contains("wartet auf")
        || low.contains("bleibt offen")
        || low.contains("nächster schritt")
        || low.contains("naechster schritt")
        || low.contains("ausstehend")
        || low.starts_with("später")
        || low.starts_with("spaeter")
        || low.starts_with("offen");
    if !has_marker {
        return None;
    }
    if low.contains("pending") || low.contains("ausstehend") {
        Some("PENDING")
    } else if low.contains("nächster schritt")
        || low.contains("naechster schritt")
        || low.contains("wartet auf")
    {
        Some("WAIT")
    } else {
        Some("OPEN")
    }
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
