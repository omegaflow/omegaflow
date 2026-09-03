use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut db = PathBuf::from(env::var("HOME").unwrap_or_default())
        .join(".local/share/opencode/opencode.db");
    let mut since: i64 = 0;
    let mut full = false;
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
            "--full" => full = true,
            _ => {}
        }
        i += 1;
    }

    let out = Command::new("sqlite3")
        .arg("-separator")
        .arg("\x1f")
        .arg(&db)
        .arg("SELECT s.title, p.time_created, json_extract(p.data,'$.text') FROM part p JOIN session s ON s.id=p.session_id WHERE json_extract(p.data,'$.type')='text' AND json_extract(p.data,'$.text') IS NOT NULL ORDER BY p.time_created;")
        .output();
    let out = match out {
        Ok(o) if o.status.success() => o,
        Ok(o) => {
            eprintln!(
                "claim_reader: sqlite3: {}",
                String::from_utf8_lossy(&o.stderr)
            );
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("claim_reader: sqlite3: {}", e);
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&out.stdout).to_string();

    let mut count_abs = 0usize;
    let mut count_widerspruch = 0usize;
    for line in text.lines() {
        let mut parts = line.splitn(3, '\x1f');
        let session = parts.next().unwrap_or("").to_string();
        let time = parts.next().unwrap_or("").to_string();
        let body = parts.next().unwrap_or("").to_string();
        let epoch: i64 = time.parse().unwrap_or(0);
        if epoch < since {
            continue;
        }
        if !is_completion(&body) {
            continue;
        }
        count_abs += 1;
        let has_commit = has_commit_marker(&body);
        let claim = claim_word(&body);
        println!(
            "ABSCHLUSS {} {} commit={} behauptung={}",
            time, session, has_commit, claim
        );
        if full {
            println!("{}", body);
            println!("----");
            continue;
        }

        for bl in body.lines() {
            let t = bl.trim();
            if t.is_empty() {
                continue;
            }
            if t.starts_with("## ") || t.starts_with("### ") {
                let title = t.trim_start_matches('#').trim();
                println!("  ABSCHNITT {}", title);
            } else if let Some(status) = status_word(t) {
                println!("    {} {}", status, truncate(t, 130));
            }
        }

        let has_open = contains_open_word(&body);
        if has_open {
            count_widerspruch += 1;
            println!(
                "  WIDERSPRUCH: post behauptet Abschluss, enthaelt aber offene Teile (offen/pending/fehlt/bleibt/ausstehend)"
            );
        }
    }
    println!(
        "COUNT abschluss={} widerspruch={} db={}",
        count_abs,
        count_widerspruch,
        db.display()
    );
}

fn is_completion(text: &str) -> bool {
    let low = text.to_lowercase();
    low.contains("fertig")
        || low.contains("befund")
        || low.contains("erledigt")
        || low.contains("gelaufen")
        || low.contains("abgeschlossen")
        || low.contains("complete")
        || low.contains("verdict")
}

fn claim_word(text: &str) -> &'static str {
    let low = text.to_lowercase();
    let measured = [
        "git", "grep", "sqlite", "gemessen", "gemeldet", "zählt", "läuft",
    ]
    .iter()
    .any(|m| low.contains(m));
    let acted = [
        "geba",
        "umgesetzt",
        "committet",
        "gemacht",
        "implementiert",
        "geprüft",
        "geprueft",
        "getestet",
        "verifiziert",
        "geschrieben",
        "gelesen",
        "gelaufen",
        "extrahiert",
        "geladen",
        "gebaut",
    ]
    .iter()
    .any(|a| low.contains(a));
    if measured {
        "gemessen"
    } else if acted {
        "leistung"
    } else {
        "leer"
    }
}

fn contains_open_word(text: &str) -> bool {
    let low = text.to_lowercase();
    low.contains("offen:")
        || low.contains("pending")
        || low.contains("fehlt")
        || low.contains("bleibt")
        || low.contains("ausstehend")
        || low.contains("wartet")
        || low.contains("offen bleibt")
        || low.contains("nicht gebaut")
        || low.contains("ungebaut")
}

fn status_word(t: &str) -> Option<&'static str> {
    let low = t.to_lowercase();
    if low.contains("erledigt") || low.contains("gelaufen") || low.contains("abgeschlossen") {
        Some("ERLEDIGT")
    } else if low.contains("pending") || low.contains("ausstehend") {
        Some("PENDING")
    } else if low.contains("offen") {
        Some("OFFEN")
    } else if low.contains("fehlt") {
        Some("FEHLT")
    } else if low.contains("wartet") {
        Some("WARTET")
    } else if low.contains("bleibt") {
        Some("BLEIBT")
    } else {
        None
    }
}

fn has_commit_marker(text: &str) -> bool {
    let lower = text.to_lowercase();
    for w in lower.split(|c: char| !c.is_ascii_alphanumeric()) {
        if w.len() >= 7 && w.chars().all(|c| c.is_ascii_hexdigit()) {
            return true;
        }
    }
    false
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
