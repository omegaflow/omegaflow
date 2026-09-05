use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Claim {
    time: String,
    session: String,
    claim: String,
    claim_word: String,
    paths: Vec<PathCheck>,
    commits: Vec<String>,
    commit_verified: Option<bool>,
}

struct PathCheck {
    path: String,
    kind: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut db: Option<PathBuf> = None;
    let mut root = PathBuf::from(".");
    let mut session_arg: Option<String> = None;
    let mut write = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--db" => {
                i += 1;
                db = Some(PathBuf::from(&args[i]));
            }
            "--root" => {
                i += 1;
                root = PathBuf::from(&args[i]);
            }
            "--session" => {
                i += 1;
                session_arg = Some(args[i].clone());
            }
            "--write" => write = true,
            _ => {}
        }
        i += 1;
    }

    let db = match db {
        Some(p) => p,
        None => match env::var("HOME") {
            Ok(h) => PathBuf::from(h).join(".local/share/opencode/opencode.db"),
            Err(_) => {
                eprintln!("claim_verify: HOME absent — pass --db <path>");
                std::process::exit(1);
            }
        },
    };

    let session = match session_arg {
        Some(s) => s,
        None => newest_session(&db),
    };
    let claims = read_claims(&db, &session);
    let mut unbacked: Vec<&Claim> = Vec::new();
    for c in &claims {
        if !backed(&c.paths) {
            unbacked.push(c);
        }
    }

    let root_s = root.display().to_string();
    for c in &unbacked {
        let paths = c
            .paths
            .iter()
            .map(|p| format!("{}={}", p.kind, p.path))
            .collect::<Vec<_>>()
            .join(" ");
        let paths_display = if c.paths.is_empty() {
            "(no path)".to_string()
        } else {
            paths
        };
        let commits_display = match &c.commit_verified {
            Some(true) => format!("commits={} true", c.commits.join(",")),
            Some(false) => format!("commits={} FALSE", c.commits.join(",")),
            None => String::new(),
        };
        let extra = if commits_display.is_empty() {
            String::new()
        } else {
            format!(" [{}]", commits_display)
        };
        println!(
            "CLAIM {} {} {} [{}] {}{}",
            c.time,
            c.session,
            c.claim_word,
            paths_display,
            truncate(&c.claim, 80),
            extra
        );
        if write {
            let line = register_line(c);
            println!("  -> would register: {}", line);
        }
    }

    if write {
        if unbacked.is_empty() {
            println!(
                "claim_verify: no unbacked claims in {} — nothing to register",
                session
            );
        } else {
            append_register(&root, &session, &unbacked);
            println!(
                "claim_verify: registered {} unbacked claims in TODO.md (session {})",
                unbacked.len(),
                session
            );
        }
    } else {
        if unbacked.is_empty() {
            println!(
                "claim_verify: {} — every claim is backed (names an existing path). Root {}",
                session, root_s
            );
        } else {
            println!(
                "claim_verify: measured {} unbacked claims in {} (Root {})",
                unbacked.len(),
                session,
                root_s
            );
            std::process::exit(1);
        }
    }
}

fn newest_session(db: &Path) -> String {
    let out = Command::new("sqlite3")
        .arg("-separator")
        .arg("\x1f")
        .arg(db)
        .arg("SELECT s.title FROM part p JOIN session s ON s.id=p.session_id WHERE json_extract(p.data,'$.type')='text' ORDER BY p.time_created DESC LIMIT 1;")
        .output()
        .expect("claim_verify: sqlite3");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn read_claims(db: &Path, session: &str) -> Vec<Claim> {
    let out = Command::new("sqlite3")
        .arg("-separator")
        .arg("\x1f")
        .arg(db)
        .arg(format!(
            "SELECT p.time_created, s.title, json_extract(p.data,'$.text') FROM part p JOIN session s ON s.id=p.session_id WHERE s.title='{}' AND json_extract(p.data,'$.type')='text' AND json_extract(p.data,'$.text') IS NOT NULL ORDER BY p.time_created;",
            session.replace('\'', "''")
        ))
        .output();
    let out = match out {
        Ok(o) if o.status.success() => o,
        Ok(o) => {
            eprintln!(
                "claim_verify: sqlite3: {}",
                String::from_utf8_lossy(&o.stderr)
            );
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("claim_verify: sqlite3: {}", e);
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    let mut claims = Vec::new();
    for line in text.lines() {
        let mut parts = line.splitn(3, '\x1f');
        let time = parts.next().unwrap_or("").to_string();
        let st = parts.next().unwrap_or("").to_string();
        let body = parts.next().unwrap_or("").to_string();
        if !is_completion(&body) {
            continue;
        }
        let paths = extract_paths(&body)
            .into_iter()
            .map(|p| PathCheck {
                kind: check_kind(&PathBuf::from("."), &p),
                path: p,
            })
            .collect();
        let claim_word = claim_word(&body).to_string();
        let commits = extract_commits(&body);
        let commit_verified = verify_commits(&commits);
        claims.push(Claim {
            time,
            session: st,
            claim: body,
            claim_word,
            paths,
            commits,
            commit_verified,
        });
    }
    claims
}

fn backed(paths: &[PathCheck]) -> bool {
    paths.iter().any(|p| p.kind == "EXIST")
}

fn extract_commits(text: &str) -> Vec<String> {
    let mut commits = Vec::new();
    let mut run = String::new();
    for c in text.chars() {
        if c.is_ascii_hexdigit() {
            run.push(c);
        } else {
            if run.len() >= 7 && !is_epoch(&run) {
                commits.push(run.clone());
            }
            run.clear();
        }
    }
    if run.len() >= 7 && !is_epoch(&run) {
        commits.push(run);
    }
    commits
}

fn is_epoch(s: &str) -> bool {
    s.len() == 13 && s.chars().all(|c| c.is_ascii_digit())
}

fn verify_commits(commits: &[String]) -> Option<bool> {
    if commits.is_empty() {
        return None;
    }

    let mut all = true;
    for c in commits {
        let ok = Command::new("git")
            .args(["cat-file", "-e", &format!("{}", c)])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        all = all && ok;
    }
    Some(all)
}

fn register_line(c: &Claim) -> String {
    let why = match c.claim_word.as_str() {
        "leer" => "empty — no anchor, no measurement, just a 'done'",
        "leistung" => {
            if c.paths.iter().any(|p| p.kind == "MISSING") {
                "names a path that does not exist (MISSING)"
            } else if c.paths.iter().any(|p| p.kind == "DRIFT") {
                "names a path that lies elsewhere (DRIFT)"
            } else {
                "action without a resolving path — assertion, not a measurement"
            }
        }
        _ => "not verifiable",
    };
    format!(
        "- Claim {} ({}, {}): {} — {}, registered as `unverified` ({}).",
        c.time,
        c.session,
        c.claim_word,
        why,
        truncate(&c.claim, 60),
        "the thread stays"
    )
}

fn append_register(root: &Path, session: &str, unbacked: &[&Claim]) {
    let todo = root.join("docs").join("TODO.md");
    let now = date_today();
    let mut block = format!("\n## {} ({}, claim_verify)\n", session, now);

    let topics = distill(unbacked);
    block.push_str("### Open State (distilled)\n");
    for (topic, claims) in &topics {
        block.push_str(&format!(
            "- **{}** ({}): {} — {}\n",
            topic,
            claims.len(),
            topic_status(claims),
            topic_points(topic, claims)
        ));
    }
    block.push('\n');

    block.push_str("### Thread (all claims)\n");
    for c in unbacked {
        block.push_str(&register_line(c));
        block.push('\n');
    }
    block.push_str(
        "Registered so the claim is not lost; the truth of the skill stays with the operator.\n",
    );
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .open(&todo)
        .expect("claim_verify: TODO.md open");
    f.write_all(block.as_bytes())
        .expect("claim_verify: TODO.md write");
}

fn distill<'a>(unbacked: &[&'a Claim]) -> Vec<(String, Vec<&'a Claim>)> {
    let topics: Vec<&str> = vec![
        "akteure",
        "boden",
        "bz",
        "cses",
        "demeter",
        "enso",
        "ephemeriden",
        "farbe",
        "fenster",
        "generation",
        "himmel",
        "kataloge",
        "koerper",
        "livefeed",
        "miniseed",
        "nebra",
        "pioneer",
        "quanten",
        "quelle",
        "sonne",
        "spk",
        "ztf",
        "paper",
        "survey",
        "register",
        "messreihe",
        "handover",
        "main",
        "doku",
        "gate",
        "friction",
        "lem",
        "blatt",
        "datenbank",
        "vacuum",
        "ledger",
        "rebase",
        "push",
        "axiom",
        "cdn",
        "ernte",
        "message",
    ];
    let mut out: Vec<(String, Vec<&Claim>)> = Vec::new();
    for c in unbacked {
        let lower = c.claim.to_lowercase();
        let mut assigned = false;
        for t in &topics {
            if lower.contains(t) {
                let key = t.to_string();
                if let Some(e) = out.iter_mut().find(|(k, _)| k == &key) {
                    e.1.push(c);
                } else {
                    out.push((key.clone(), vec![c]));
                }
                assigned = true;
                break;
            }
        }
        if !assigned {
            let key = "(vague — no anchor)".to_string();
            if let Some(e) = out.iter_mut().find(|(k, _)| k == &key) {
                e.1.push(c);
            } else {
                out.push((key, vec![c]));
            }
        }
    }
    out.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    out
}

fn topic_status(claims: &[&Claim]) -> String {
    if claims
        .iter()
        .any(|c| c.paths.iter().any(|p| p.kind.starts_with("MISSING")))
    {
        return "OPEN (path absent)".to_string();
    }

    if !claims.is_empty() && claims.iter().all(|c| c.commit_verified == Some(true)) {
        return "DONE (commits verified)".to_string();
    }
    if claims.iter().any(|c| c.claim_word == "leistung") {
        return "OPEN (claim without path)".to_string();
    }
    "OPEN (unverified)".to_string()
}

fn topic_points(topic: &str, claims: &[&Claim]) -> String {
    let mut points = Vec::new();
    for c in claims.iter().take(4) {
        let text = c.claim.trim_start_matches("# ").trim();
        let t = truncate(text, 70);
        if !points.contains(&t) {
            points.push(t);
        }
    }
    if points.is_empty() {
        format!("{}: nothing concrete named", topic)
    } else {
        points.join(" | ")
    }
}

fn date_today() -> String {
    let out = Command::new("date")
        .arg("+%Y-%m-%d")
        .output()
        .expect("claim_verify: date");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
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
    let low = fold_umlauts(&text.to_lowercase());
    let measured = [
        "git",
        "grep",
        "sqlite",
        "gemessen",
        "laeuft",
        "messbar",
        "messung",
        "gemeldet",
        "berechnet",
        "verifiziert",
        "belegt",
        "quanti",
        "zaehlt",
        "ausgabe",
        "ergebnis",
        "exit",
        "pid",
        "mb",
        "gb",
        "bytes",
    ]
    .iter()
    .any(|m| low.contains(m));
    let acted = [
        "geba",
        "umgesetzt",
        "committet",
        "gemacht",
        "implementiert",
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

fn fold_umlauts(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\u{e4}' | '\u{c4}' => out.push_str("ae"),
            '\u{f6}' | '\u{d6}' => out.push_str("oe"),
            '\u{fc}' | '\u{dc}' => out.push_str("ue"),
            '\u{df}' => out.push_str("ss"),
            _ => out.push(c),
        }
    }
    out
}

fn extract_paths(line: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0;
    let candidates = ["src/", "tools/src/bin/", "docs/", "archive/", "phi/"];
    while i < bytes.len() {
        for c in &candidates {
            if bytes[i..].starts_with(c.as_bytes()) {
                let start = i;
                let mut j = i + c.len();
                while j < bytes.len() && is_path_char(bytes[j]) {
                    j += 1;
                }
                let p = &line[start..j];
                if is_path(p) {
                    paths.push(p.to_string());
                }
                i = j;
                break;
            }
        }
        i += 1;
    }
    paths
}

fn is_path_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'/' | b'.' | b'_' | b'-' | b'~')
}

fn is_path(p: &str) -> bool {
    p.ends_with(".rs")
        || p.ends_with(".md")
        || p.ends_with(".φ")
        || p.ends_with(".bin")
        || p.ends_with(".json")
        || p.ends_with(".txt")
}

fn check_kind(root: &Path, path: &str) -> String {
    let full = root.join(path);
    if full.exists() {
        return "EXIST".to_string();
    }
    if path.starts_with("src/") {
        if let Some(actual) = find_by_basename(root, path) {
            return format!("DRIFT->{}", actual.display());
        }
    }
    if is_gate_path(path) {
        return "GATE".to_string();
    }
    "MISSING".to_string()
}

fn find_by_basename(root: &Path, path: &str) -> Option<PathBuf> {
    let name = Path::new(path).file_name()?;
    let mut stack = vec![root.join("src")];
    while let Some(dir) = stack.pop() {
        let entries = std::fs::read_dir(&dir).ok()?;
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p.clone());
            } else if p.file_name() == Some(name) {
                return Some(p);
            }
        }
    }
    None
}

fn is_gate_path(path: &str) -> bool {
    let name = match Path::new(path).file_name() {
        Some(n) => n.to_string_lossy(),
        None => return false,
    };
    matches!(
        name.as_ref(),
        "commit_gate.rs" | "axioms.rs" | "state.rs" | "friction.rs"
    )
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
