use std::path::PathBuf;

fn work_root() -> PathBuf {
    match std::env::var_os("OMEGAFLOW_WORK") {
        Some(w) => PathBuf::from(w),
        None => match std::env::var_os("HOME") {
            Some(h) => PathBuf::from(h).join("projects/omegaflow-work"),
            None => PathBuf::new(),
        },
    }
}

fn code_root() -> PathBuf {
    match std::env::var_os("OMEGAFLOW_CODE") {
        Some(w) => PathBuf::from(w),
        None => match std::env::var_os("HOME") {
            Some(h) => PathBuf::from(h).join("projects/omegaflow"),
            None => PathBuf::new(),
        },
    }
}

fn build(text: &str) -> (String, usize, usize, usize) {
    let mut lines: Vec<String> = Vec::new();
    let mut applied = 0usize;
    let mut rows = 0usize;
    let mut notes = 0usize;
    for line in text.lines() {
        if line.starts_with('#') {
            continue;
        }
        if line.trim().is_empty() {
            lines.push(String::new());
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 2 {
            continue;
        }
        let original = cols.first().copied().unwrap_or("");
        if original == "original" {
            continue;
        }
        if original.trim().is_empty() && cols.get(1).map(|c| c.trim()).unwrap_or("").is_empty() {
            lines.push(String::new());
            continue;
        }
        rows += 1;
        let correction = cols.get(1).map(|c| c.trim()).unwrap_or("");
        let chosen = if correction.is_empty() {
            original
        } else {
            applied += 1;
            correction
        };
        if chosen == "note" || chosen.starts_with("note ") {
            notes += 1;
            continue;
        }
        lines.push(chosen.to_string());
    }
    (lines.join("\n") + "\n", rows, applied, notes)
}

fn blocks(text: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut cur: Option<(String, Vec<&str>)> = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("url ") {
            if let Some((u, body)) = cur.take() {
                out.push((u, body.join("\n")));
            }
            cur = Some((rest.trim().to_string(), vec![line]));
        } else if let Some((_, body)) = cur.as_mut()
            && !line.trim().is_empty()
        {
            body.push(line);
        }
    }
    if let Some((u, body)) = cur.take() {
        out.push((u, body.join("\n")));
    }
    out
}

fn report(release: &str, published: &str, label: &str) {
    let r = blocks(release);
    let p = blocks(published);
    let rmap: std::collections::BTreeMap<&str, &str> =
        r.iter().map(|(u, b)| (u.as_str(), b.as_str())).collect();
    let pmap: std::collections::BTreeMap<&str, &str> =
        p.iter().map(|(u, b)| (u.as_str(), b.as_str())).collect();
    let mut same = 0usize;
    let mut differs = 0usize;
    for (u, b) in &rmap {
        match pmap.get(u) {
            Some(pb) if pb == b => same += 1,
            Some(_) => differs += 1,
            None => {}
        }
    }
    let only_release = rmap.keys().filter(|u| !pmap.contains_key(*u)).count();
    let only_published = pmap.keys().filter(|u| !rmap.contains_key(*u)).count();
    println!(
        "register_release --check {label}: release {} blocks, published {} — {same} identical, {differs} same url newer text, {only_release} only in release, {only_published} only published",
        rmap.len(),
        pmap.len()
    );
}

fn expand_artifacts(text: &str) -> String {
    if !text.lines().any(|line| line.starts_with("artifact ")) {
        return text.to_string();
    }
    let mut out = String::new();
    for block in text.split("\n\n") {
        if block.trim().is_empty() {
            continue;
        }
        let mut shared: Vec<&str> = Vec::new();
        let mut artifacts: Vec<Vec<&str>> = Vec::new();
        for line in block.lines() {
            if let Some(rest) = line.strip_prefix("artifact ") {
                artifacts.push(rest.split_whitespace().collect());
            } else if !line.starts_with("url ")
                && !line.starts_with("sha256 ")
                && !line.starts_with("coverage ")
            {
                shared.push(line);
            }
        }
        if artifacts.is_empty() {
            out.push_str(block.trim_end());
            out.push_str("\n\n");
            continue;
        }
        for artifact in &artifacts {
            let url = artifact.first().copied().unwrap_or("");
            let sha = artifact.get(1).copied().unwrap_or("");
            let interval = artifact.get(2).copied().unwrap_or("");
            let coverage = artifact.get(3).copied().unwrap_or("");
            let origin = artifact.get(4).copied().unwrap_or("");
            out.push_str(&format!("url {url}\n"));
            if !sha.is_empty() {
                out.push_str(&format!("sha256 {sha}\n"));
            }
            if !origin.is_empty() {
                out.push_str(&format!("origin {origin}\n"));
            }
            if !coverage.is_empty() {
                out.push_str(&format!("coverage {coverage}\n"));
            }
            for line in &shared {
                if line.starts_with("field ") && !interval.is_empty() {
                    out.push_str(&format!("{line} {interval} 0.0 0.0\n"));
                } else {
                    out.push_str(line);
                    out.push('\n');
                }
            }
            out.push('\n');
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let publish = args.iter().any(|a| a == "--publish");
    let check = args.iter().any(|a| a == "--check");
    let explicit: Option<PathBuf> = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(PathBuf::from);

    let work = work_root().join("phi/sources.tsv");
    let text = match std::fs::read_to_string(&work) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("register_release: read {}: {e}", work.display());
            std::process::exit(2);
        }
    };
    let (built, rows, applied, notes) = build(&text);
    let body = expand_artifacts(&built);

    let target = if publish {
        if std::env::var_os("OMEGAFLOW_REGISTER_RELEASE").is_none() {
            eprintln!(
                "register_release: --publish needs OMEGAFLOW_REGISTER_RELEASE — the register is written by the release step alone"
            );
            std::process::exit(2);
        }
        code_root().join("phi/sources.\u{03c6}")
    } else {
        match explicit {
            Some(p) => p,
            None => work_root().join("build/sources.\u{03c6}"),
        }
    };

    if check {
        let published = code_root().join("phi/sources.\u{03c6}");
        match std::fs::read_to_string(&published) {
            Ok(p) => report(&body, &p, "published"),
            Err(_) => println!("register_release --check: {} absent", published.display()),
        }
        let previous = work_root().join("build/sources.\u{03c6}");
        if let Ok(p) = std::fs::read_to_string(&previous) {
            report(&body, &p, "build");
        }
        if !publish {
            return;
        }
    }

    if let Some(parent) = target.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match std::fs::write(&target, &body) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("register_release: write {}: {e}", target.display());
            std::process::exit(2);
        }
    }
    println!(
        "register_release: {rows} rows, {applied} corrections applied, {notes} notes dropped -> {}",
        target.display()
    );
}
