use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut db = PathBuf::from(env::var("HOME").unwrap_or_default())
        .join(".local/share/opencode/opencode.db");
    let mut top = 40usize;
    let mut context = 1usize;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--db" => {
                i += 1;
                db = PathBuf::from(&args[i]);
            }
            "--top" => {
                i += 1;
                top = args[i].parse().unwrap_or(40);
            }
            "--context" => {
                i += 1;
                context = args[i].parse().unwrap_or(1);
            }
            _ => {}
        }
        i += 1;
    }

    let out = Command::new("sqlite3")
        .arg("-separator")
        .arg("\x1f")
        .arg(&db)
        .arg("SELECT json_extract(data,'$.text') FROM part WHERE json_extract(data,'$.type')='text' AND json_extract(data,'$.text') IS NOT NULL;")
        .output();
    let out = match out {
        Ok(o) if o.status.success() => o,
        Ok(o) => {
            eprintln!(
                "commit_words: sqlite3: {}",
                String::from_utf8_lossy(&o.stderr)
            );
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("commit_words: sqlite3: {}", e);
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&out.stdout).to_string();

    let mut freq: HashMap<String, usize> = HashMap::new();
    let mut blocks = 0usize;
    let mut with_commit_word = 0usize;
    for post in text.split('\x1f') {
        let lines: Vec<&str> = post.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if !has_commit_marker(line) {
                continue;
            }
            blocks += 1;
            let lo = idx.saturating_sub(context);
            let hi = (idx + context + 1).min(lines.len());
            for ctx in &lines[lo..hi] {
                if lower(ctx).contains("commit") {
                    with_commit_word += 1;
                }
                for w in words(ctx) {
                    *freq.entry(w).or_insert(0) += 1;
                }
            }
        }
    }

    let mut sorted: Vec<(&String, usize)> = freq.iter().map(|(w, c)| (w, *c)).collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    println!(
        "BLOCKS {} | commit-kontext {} | db {}",
        blocks,
        with_commit_word,
        db.display()
    );
    println!("\ntop {} words in commit blocks:", top);
    let mut leistung_n = 0usize;
    let mut offen_n = 0usize;
    let mut ausrede_n = 0usize;
    let mut messung_n = 0usize;
    for (w, c) in sorted.iter().take(top) {
        let cls = class(w);
        println!("  {:>5}  {:>10}  {}", c, cls, w);
        match cls {
            "leistung" => leistung_n += c,
            "offen" => offen_n += c,
            "ausrede" => ausrede_n += c,
            "messung" => messung_n += c,
            _ => {}
        }
    }
    let total = leistung_n + offen_n + ausrede_n + messung_n;
    if total > 0 {
        println!("\nklassen (top-{}):", top);
        for (name, n) in [
            ("leistung (gemachte)", leistung_n),
            ("offen    (offene)  ", offen_n),
            ("ausrede  (faule)   ", ausrede_n),
            ("messung  (gemessen)", messung_n),
        ] {
            println!(
                "  {} {:>5}  {:.1}%",
                name,
                n,
                100.0 * n as f64 / total as f64
            );
        }
    }
}

fn has_commit_marker(text: &str) -> bool {
    let mut run = 0usize;
    for c in text.chars() {
        if c.is_ascii_hexdigit() {
            run += 1;
            if run >= 7 {
                return true;
            }
        } else {
            run = 0;
        }
    }
    false
}

fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphabetic())
        .filter(|w| w.len() >= 4)
        .map(|w| w.to_lowercase())
        .filter(|w| {
            !matches!(
                w.as_str(),
                "der" | "die" | "das" | "und" | "ich" | "mit" | "auf" | "eine"
            )
        })
        .collect()
}

fn lower(s: &str) -> String {
    s.to_lowercase()
}

fn class(w: &str) -> &'static str {
    let leistung = [
        "commit",
        "committet",
        "gemacht",
        "geba",
        "umgesetzt",
        "implementiert",
        "geschrieben",
        "gelaufen",
        "gebaut",
        "geprueft",
        "geprüft",
        "getestet",
        "gelesen",
        "extrahiert",
        "geladen",
        "erledigt",
        "gecheckt",
    ];
    let messung = [
        "git",
        "grep",
        "gemessen",
        "zaehlt",
        "zählt",
        "sqlite",
        "berechnet",
        "lauf",
    ];
    let offen = [
        "offen", "pending", "fehlt", "bleibt", "wartet", "noch", "nicht", "kein",
    ];
    let ausrede = [
        "aber",
        "jedoch",
        "leider",
        "schade",
        "vermutlich",
        "wahrscheinlich",
        "irgendwann",
        "später",
        "spaeter",
        "muesste",
        "müsste",
        "vielleicht",
        "irgendwie",
        "anscheinend",
        "scheinbar",
    ];
    if leistung.contains(&w) {
        "leistung"
    } else if messung.contains(&w) {
        "messung"
    } else if offen.contains(&w) {
        "offen"
    } else if ausrede.contains(&w) {
        "ausrede"
    } else {
        "zeichen"
    }
}
