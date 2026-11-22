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
            rows += 1;
            lines.push(line.to_string());
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

fn block_key(block: &str) -> String {
    if let Some(u) = block.lines().find_map(|line| line.strip_prefix("url ")) {
        return u.split_whitespace().next().unwrap_or("").to_string();
    }
    block
        .lines()
        .filter(|line| {
            !line.starts_with("seen ")
                && !line.starts_with("url ")
                && !line.starts_with("origin ")
                && !line.starts_with("sha256 ")
                && !line.starts_with("coverage ")
                && !line.trim().is_empty()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn blocks(text: &str) -> Vec<(String, String)> {
    text.split("\n\n")
        .filter(|b| !b.trim().is_empty())
        .map(|b| (block_key(b), b.to_string()))
        .collect()
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

fn licence_admitted(token: &str) -> bool {
    static TABLE: std::sync::OnceLock<std::collections::HashMap<&'static str, &'static str>> =
        std::sync::OnceLock::new();
    let table = TABLE.get_or_init(|| {
        let mut m = std::collections::HashMap::new();
        for line in include_str!("../../../../phi/licences.tsv").lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let p: Vec<&str> = line.split('\t').collect();
            if p.len() >= 2 {
                m.entry(p[0]).or_insert(p[1]);
            }
        }
        m
    });
    table.get(token).copied() == Some("admitted")
}

fn delivery_index() -> &'static std::collections::HashMap<String, std::collections::BTreeMap<String, String>> {
    static TABLE: std::sync::OnceLock<
        std::collections::HashMap<String, std::collections::BTreeMap<String, String>>,
    > = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        let mut m = std::collections::HashMap::new();
        let path = work_root().join("phi/delivery.\u{03c6}");
        if let Ok(text) = std::fs::read_to_string(path) {
            for block in text.split("\n\n") {
                let Some(key) = block
                    .lines()
                    .find_map(|l| l.strip_prefix("artifact ").map(|r| r.trim().to_string()))
                else {
                    continue;
                };
                let mut kv = std::collections::BTreeMap::new();
                for line in block.lines() {
                    let line = line.trim();
                    for prefix in ["url ", "origin ", "sha256 ", "coverage "] {
                        if let Some(rest) = line.strip_prefix(prefix) {
                            let value = rest.split_whitespace().next().unwrap_or("");
                            kv.insert(prefix.trim().to_string(), value.to_string());
                        }
                    }
                }
                m.insert(key, kv);
            }
        }
        m
    })
}

fn block_is_evidenced(block: &str) -> bool {
    let has_field = block.lines().any(|line| {
        line.starts_with("field ")
            || line.starts_with("quantity ")
            || line.starts_with("first ")
            || line.starts_with("last ")
            || line.starts_with("lastrow ")
    });
    if !has_field {
        return false;
    }
    let Some(terms) = block
        .lines()
        .find(|line| line.starts_with("terms "))
        .and_then(|line| line.split_whitespace().nth(1))
    else {
        return false;
    };
    if !licence_admitted(terms) {
        return false;
    }
    let has_sha = block.lines().any(|line| line.starts_with("sha256 "));
    let has_artifact_sha = block
        .lines()
        .any(|line| line.starts_with("artifact ") && line.contains("sha256="));
    let artifact_sha_in_delivery = block.lines().any(|line| {
        let Some(key) = line.strip_prefix("artifact ") else {
            return false;
        };
        if key.contains('=') {
            return false;
        }
        delivery_index()
            .get(key.trim())
            .map(|kv| kv.contains_key("sha256"))
            .unwrap_or(false)
    });
    let live = ["schema ", "sample ", "ts "]
        .iter()
        .all(|prefix| block.lines().any(|line| line.starts_with(prefix)));
    has_sha || has_artifact_sha || artifact_sha_in_delivery || live
}

fn expand_artifacts(text: &str) -> String {
    let mut out = String::new();
    for block in text.split("\n\n") {
        if block.trim().is_empty() {
            continue;
        }
        if !block_is_evidenced(block) {
            continue;
        }
        let mut body: Vec<String> = Vec::new();
        let mut artifacts: Vec<std::collections::BTreeMap<String, String>> = Vec::new();
        for line in block.lines() {
            if let Some(rest) = line.strip_prefix("artifact ") {
                let mut kv = std::collections::BTreeMap::new();
                for token in rest.split_whitespace() {
                    if let Some((key, value)) = token.split_once('=') {
                        kv.insert(key.to_string(), value.to_string());
                    }
                }
                if kv.is_empty() {
                    if let Some(d) = delivery_index().get(rest.trim()) {
                        kv = d.clone();
                    }
                }
                artifacts.push(kv);
            } else if !line.starts_with("url ")
                && !line.starts_with("sha256 ")
                && !line.starts_with("coverage ")
                && !line.starts_with("origin ")
            {
                body.push(line.to_string());
            }
        }
        for line in &body {
            out.push_str(line);
            out.push('\n');
        }
        for artifact in &artifacts {
            if let Some(url) = artifact.get("url") {
                out.push_str(&format!("url {url}\n"));
            }
            if let Some(sha) = artifact.get("sha256") {
                out.push_str(&format!("sha256 {sha}\n"));
            }
            if let Some(origin) = artifact.get("origin") {
                out.push_str(&format!("origin {origin}\n"));
            }
            if let Some(coverage) = artifact.get("coverage") {
                out.push_str(&format!("coverage {coverage}\n"));
            }
        }
        out.push('\n');
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

    let work = work_root().join("phi/measurements.tsv");
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
        code_root().join("phi/measurements.\u{03c6}")
    } else {
        match explicit {
            Some(p) => p,
            None => work_root().join("build/measurements.\u{03c6}"),
        }
    };

    if check {
        let published = code_root().join("phi/measurements.\u{03c6}");
        match std::fs::read_to_string(&published) {
            Ok(p) => report(&body, &p, "published"),
            Err(_) => println!("register_release --check: {} absent", published.display()),
        }
        let previous = work_root().join("build/measurements.\u{03c6}");
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
