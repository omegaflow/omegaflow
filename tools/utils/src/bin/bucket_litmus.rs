use std::collections::BTreeMap;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("bucket_litmus <decline_lens.φ> <inventory.φ> [--calibrate <disposition.φ>]");
        std::process::exit(1);
    }
    let lens = match std::fs::read_to_string(&args[1]) {
        Ok(c) => parse_lens(&c),
        Err(_) => {
            eprintln!("decline_lens unreadable: {}", args[1]);
            std::process::exit(1);
        }
    };
    let inventory = match std::fs::read_to_string(&args[2]) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("inventory unreadable: {}", args[2]);
            std::process::exit(1);
        }
    };

    let mut disp_path: Option<&String> = None;
    if args.len() >= 5 && args[3] == "--calibrate" {
        disp_path = Some(&args[4]);
    }

    let mut verdicts: BTreeMap<String, String> = BTreeMap::new();
    if let Some(disp) = disp_path {
        if let Ok(c) = std::fs::read_to_string(disp) {
            for line in c.lines() {
                let t = line.trim();
                if t.is_empty() || t.starts_with('#') || t.starts_with("note ") {
                    continue;
                }
                if let Some((word, id)) = t.split_once(' ') {
                    match word {
                        "compiler-lease" | "descoped" | "pending" | "konsument" => {
                            verdicts.insert(id.trim().to_string(), word.to_string());
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let mut lines: Vec<String> = Vec::new();
    lines.push(
        "# bucket_litmus: question 1 (oscillator gate) — pre-verdict, no judgment.".to_string(),
    );
    lines.push("# questions 2–4 stay an unanswered review checklist:".to_string());
    lines.push(
        "#   question 2 form — live scalar (url-line) vs bulk archive (compiler-lease)".to_string(),
    );
    lines.push(
        "#   question 3 manifestation — url-line/CDN vs compiler asset/FORM (C1 doctrine)"
            .to_string(),
    );
    lines.push("#   question 4 overflow — named consumer vs pending/descoped".to_string());
    lines.push(format!("# lens: {}", args[1]));

    let mut seen: BTreeMap<String, ()> = BTreeMap::new();
    let mut fp: Vec<(String, String)> = Vec::new();
    let mut fn_: Vec<(String, String)> = Vec::new();
    let mut n_lease = 0usize;
    let mut n_descoped = 0usize;
    let mut n_pending = 0usize;

    for line in inventory.lines() {
        let t = line.trim();
        if !t.starts_with("catalog ") {
            continue;
        }
        let Some((title, id)) = split_title_id(t) else {
            continue;
        };
        if seen.insert(id.clone(), ()).is_some() {
            continue;
        }
        let title_lc = title.to_lowercase().replace('-', " ");
        let class = first_match(&title_lc, &lens.classes);
        let family = family_of(&title_lc, &lens.families);

        match verdicts.get(&id).map(|s| s.as_str()) {
            Some("descoped") => {
                n_descoped += 1;
                if !is_flagged(&class, &family) {
                    fn_.push((id.clone(), title.clone()));
                }
            }
            Some("compiler-lease") => {
                n_lease += 1;
                if is_flagged(&class, &family) {
                    fp.push((id.clone(), title.clone()));
                }
            }
            Some("pending") => {
                n_pending += 1;
            }
            _ => {}
        }

        let class_s = match &class {
            Some((name, tok)) => format!("lens-match {} @ {}", name, tok),
            None => "lens-no-match".to_string(),
        };
        let family_s = match &family {
            Some((name, tok)) => format!("family {} @ {}", name, tok),
            None => "family ?".to_string(),
        };
        lines.push(format!("{} | {} | {}", id, class_s, family_s));
    }

    if disp_path.is_some() {
        lines.push("#".to_string());
        lines.push(
            "# calibration gate against the recorded verdicts (noaa_nodd_disposition.φ):"
                .to_string(),
        );
        lines.push(format!(
            "#   compiler-lease: {}  descoped: {}  pending: {}",
            n_lease, n_descoped, n_pending
        ));
        lines.push(format!(
            "#   FP (lease flagged as lens-match): {}",
            fp.len()
        ));
        for (id, title) in &fp {
            lines.push(format!("#   FP  {}  —  {}", id, title));
        }
        lines.push(format!(
            "#   FN (descoped without lens-match): {}",
            fn_.len()
        ));
        for (id, title) in &fn_ {
            lines.push(format!("#   FN  {}  —  {}", id, title));
        }
    }

    let mut out = String::new();
    for l in &lines {
        out.push_str(l);
        out.push('\n');
    }
    match std::io::stdout().write_all(out.as_bytes()) {
        Ok(()) => {}
        Err(_) => std::process::exit(1),
    }
}

struct Lens {
    classes: Vec<(String, Vec<String>)>,
    families: Vec<(String, Vec<String>)>,
}

fn parse_lens(content: &str) -> Lens {
    let mut classes = Vec::new();
    let mut families = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let Some((head, tokens)) = t.split_once('@') else {
            continue;
        };
        let mut head_it = head.split_whitespace();
        let Some(kind) = head_it.next() else {
            continue;
        };
        let Some(name) = head_it.next() else {
            continue;
        };
        let toks: Vec<String> = tokens
            .split_whitespace()
            .map(|s| s.to_lowercase().replace('-', " "))
            .collect();
        match kind {
            "class" => classes.push((name.to_string(), toks)),
            "family" => families.push((name.to_string(), toks)),
            _ => {}
        }
    }
    Lens { classes, families }
}

fn split_title_id(line: &str) -> Option<(String, String)> {
    let body = line.trim_start_matches("catalog ").trim();
    let parts: Vec<&str> = body.split('|').map(|s| s.trim()).collect();
    if parts.len() < 2 {
        return None;
    }
    let title = parts[1].to_string();
    let id = parts.last()?.to_string();
    Some((title, id))
}

fn first_match(title_lc: &str, classes: &[(String, Vec<String>)]) -> Option<(String, String)> {
    for (name, toks) in classes {
        for tok in toks {
            if !tok.is_empty() && title_lc.contains(tok.as_str()) {
                return Some((name.clone(), tok.clone()));
            }
        }
    }
    None
}

fn family_of(title_lc: &str, families: &[(String, Vec<String>)]) -> Option<(String, String)> {
    for want in ["a", "b", "c"] {
        for (name, toks) in families {
            if name != want {
                continue;
            }
            for tok in toks {
                if !tok.is_empty() && title_lc.contains(tok.as_str()) {
                    return Some((name.clone(), tok.clone()));
                }
            }
        }
    }
    None
}

fn is_flagged(class: &Option<(String, String)>, family: &Option<(String, String)>) -> bool {
    if class.is_some() {
        return true;
    }
    match family {
        Some((name, _)) => name == "a" || name == "b",
        None => false,
    }
}
