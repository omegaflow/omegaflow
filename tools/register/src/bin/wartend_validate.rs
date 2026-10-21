use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: wartend_validate <state/wartend.φ>");
            std::process::exit(2);
        }
    };
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("wartend_validate: {path}: {e}");
            std::process::exit(2);
        }
    };

    let mut hard = 0usize;
    let mut soft = 0usize;
    let mut keys: HashSet<String> = HashSet::new();
    let mut findings: Vec<(String, String)> = Vec::new();

    for block in text.split("\n\n") {
        let mut f: HashMap<String, String> = HashMap::new();
        for line in block.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut it = line.splitn(2, ' ');
            let k = match it.next() {
                Some(x) => x.trim(),
                None => continue,
            };
            let v = match it.next() {
                Some(x) => x.trim(),
                None => "",
            };
            if !k.is_empty() {
                f.insert(k.to_string(), v.to_string());
            }
        }
        if f.is_empty() {
            continue;
        }

        let key = f.get("key").cloned();
        let key_display = match &key {
            Some(k) => k.as_str(),
            None => "?",
        };
        for req in ["key", "trigger", "aufnehmer", "zustand"] {
            let absent = match f.get(req) {
                Some(s) => s.is_empty(),
                None => true,
            };
            if absent {
                findings.push((format!("missing_{req}"), key_display.to_string()));
                hard += 1;
            }
        }
        if let Some(k) = &key {
            if !keys.insert(k.clone()) {
                findings.push(("duplicate_key".to_string(), k.clone()));
                hard += 1;
            }
        }
        if let Some(z) = f.get("zustand") {
            if !z.is_empty()
                && !["pending", "gemessen", "obergrenze", "null-echt", "absent", "riss"]
                    .contains(&z.as_str())
            {
                findings.push(("zustand_unbekannt".to_string(), z.clone()));
                hard += 1;
            }
        }
        if let Some(l) = f.get("lebenszyklus") {
            if !l.is_empty() && !["offen", "wartend", "declined"].contains(&l.as_str()) {
                findings.push(("lebenszyklus_unbekannt".to_string(), l.clone()));
                hard += 1;
            }
        }
        if let Some(g) = f.get("zugang") {
            if !g.is_empty() && !g.starts_with("frei") && !g.starts_with("LOCK") {
                findings.push(("zugang_unbekannt".to_string(), g.clone()));
                soft += 1;
            }
        }
    }

    for (rule, key) in &findings {
        println!("{rule}\t{key}");
    }
    eprintln!(
        "wartend_validate: {} findings ({} hard, {} soft)",
        hard + soft,
        hard,
        soft
    );
    if hard > 0 {
        std::process::exit(2);
    }
}
