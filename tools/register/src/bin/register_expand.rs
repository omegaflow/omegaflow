use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn effective_lines(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for raw in text.lines() {
        let cols: Vec<&str> = raw.split('\t').collect();
        let first = match cols.get(1) {
            Some(correction) if !correction.trim().is_empty() => correction.trim(),
            _ => cols.first().copied().unwrap_or(""),
        };
        if first.starts_with('#') || first.trim().is_empty() || !first.contains(' ') {
            continue;
        }
        out.push(first.to_string());
    }
    out
}

fn blocks_of(lines: &[String]) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for line in lines {
        if line.starts_with("url ") && !current.is_empty() {
            out.push(std::mem::take(&mut current));
        }
        current.push(line.clone());
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

fn tokens(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    for ch in line.chars() {
        if ch == '"' {
            quoted = !quoted;
            current.push(ch);
        } else if ch.is_whitespace() && !quoted {
            if !current.is_empty() {
                out.push(std::mem::take(&mut current));
            }
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

fn value_of(block: &[String], key: &str) -> Option<String> {
    let prefix = format!("{key} ");
    block
        .iter()
        .find(|line| line.starts_with(&prefix))
        .map(|line| line[prefix.len()..].trim().to_string())
}

fn or_blank(value: Option<String>) -> String {
    match value {
        Some(v) => v,
        None => String::new(),
    }
}

fn field_shape(line: &str) -> String {
    let body = match line.strip_prefix("field ") {
        Some(rest) => rest,
        None => line,
    };
    let parts = tokens(body);
    let key = or_blank(parts.first().cloned());
    let name = or_blank(parts.get(1).cloned());
    if parts.len() >= 6 {
        let kernel = or_blank(parts.get(2).cloned());
        let force = or_blank(parts.get(3).cloned());
        let unit = or_blank(parts.get(4).cloned());
        return format!("field {key} {name} {kernel} {force} {unit}");
    }
    let unit = or_blank(parts.get(2).cloned());
    format!("field {key} {name} {unit}")
}

fn interval_of(line: &str) -> String {
    let body = match line.strip_prefix("field ") {
        Some(rest) => rest,
        None => line,
    };
    let parts = tokens(body);
    if parts.len() >= 6 {
        or_blank(parts.get(5).cloned())
    } else {
        or_blank(parts.get(3).cloned())
    }
}

fn rate_of(origin: &str, fallback: &str) -> String {
    let marker = match origin.find("/ascii/") {
        Some(i) => i + "/ascii/".len(),
        None => return fallback.to_string(),
    };
    let rest = &origin[marker..];
    let token = match rest.split('/').next() {
        Some(t) => t,
        None => return fallback.to_string(),
    };
    let split = token.find(|c: char| !c.is_ascii_digit() && c != '.');
    let (number, unit) = match split {
        Some(i) => (&token[..i], &token[i..]),
        None => (token, ""),
    };
    let value: f64 = match number.parse() {
        Ok(v) => v,
        Err(_) => return fallback.to_string(),
    };
    let seconds = match unit {
        "ms" => value / 1000.0,
        "s" => value,
        "m" => value * 60.0,
        "h" => value * 3600.0,
        _ => return fallback.to_string(),
    };
    format!("{seconds}")
}

fn coverage_of(origin: &str) -> String {
    let parts: Vec<&str> = origin.split('/').collect();
    let file = match parts.last() {
        Some(f) => f,
        None => return "pending".to_string(),
    };
    let stem = file.trim_end_matches(".tab");
    let year = parts
        .iter()
        .rev()
        .nth(1)
        .filter(|y| y.len() == 4 && y.chars().all(|c| c.is_ascii_digit()));
    let year = match year {
        Some(y) => y.to_string(),
        None => return "pending".to_string(),
    };
    if stem.len() != 4 || !stem.chars().all(|c| c.is_ascii_digit()) {
        return "pending".to_string();
    }
    let month: u32 = match stem[0..2].parse() {
        Ok(m) => m,
        Err(_) => return "pending".to_string(),
    };
    let day: u32 = match stem[2..4].parse() {
        Ok(d) => d,
        Err(_) => return "pending".to_string(),
    };
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return "pending".to_string();
    }
    format!("{year}-{month:02}-{day:02}")
}

fn definition_key(block: &[String]) -> String {
    let mut parts: Vec<String> = Vec::new();
    for key in ["instrument", "terms", "format", "compiler"] {
        if let Some(v) = value_of(block, key) {
            parts.push(format!("{key} {v}"));
        }
    }
    if let Some(anchor) = block.iter().find(|line| {
        [
            "on ",
            "at ",
            "query ",
            "global ",
            "coverage ",
            "orbit ",
            "track ",
            "sky ",
        ]
        .iter()
        .any(|p| line.starts_with(p))
    }) {
        parts.push(anchor.trim().to_string());
    }
    let mut fields: Vec<String> = block
        .iter()
        .filter(|line| line.starts_with("field "))
        .map(|line| field_shape(line))
        .collect();
    fields.sort();
    parts.extend(fields);
    parts.join("\n")
}

fn compact(text: &str, select: &str) -> String {
    let blocks = blocks_of(&effective_lines(text));
    let mut groups: BTreeMap<String, Vec<&Vec<String>>> = BTreeMap::new();
    for block in &blocks {
        let key = definition_key(block);
        if !key.contains(select) {
            continue;
        }
        groups.entry(key).or_default().push(block);
    }
    let mut out = String::new();
    for (definition, members) in groups {
        out.push_str(&definition);
        out.push('\n');
        for block in members {
            let url = or_blank(value_of(block, "url"));
            let sha = or_blank(value_of(block, "sha256"));
            let origin = or_blank(value_of(block, "origin"));
            let declared = block
                .iter()
                .find(|line| line.starts_with("field "))
                .map(|line| interval_of(line));
            let declared = or_blank(declared);
            let interval = rate_of(&origin, &declared);
            let coverage = coverage_of(&origin);
            out.push_str(&format!(
                "artifact {url} {sha} {interval} {coverage} {origin}\n"
            ));
        }
        out.push('\n');
    }
    out
}

fn flat(path: &Path) -> String {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            println!("expand: {} unreadable: {e}", path.display());
            return String::new();
        }
    };
    let mut shared: Vec<String> = Vec::new();
    let mut artifacts: Vec<(String, String, String, String, String)> = Vec::new();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("artifact ") {
            let parts = tokens(rest);
            if parts.len() < 4 {
                continue;
            }
            artifacts.push((
                parts[0].clone(),
                parts[1].clone(),
                parts[2].clone(),
                parts[3].clone(),
                or_blank(parts.get(4).cloned()),
            ));
        } else if !line.trim().is_empty() && !line.starts_with('#') {
            shared.push(line.to_string());
        }
    }
    let mut out = String::new();
    for (url, sha, interval, coverage, origin) in artifacts {
        out.push_str(&format!("url {url}\n"));
        out.push_str(&format!("sha256 {sha}\n"));
        out.push_str(&format!("coverage {coverage}\n"));
        if !origin.is_empty() {
            out.push_str(&format!("origin {origin}\n"));
        }
        for line in &shared {
            if line.starts_with("field ") {
                out.push_str(&format!("{line} {interval} 0.0 0.0\n"));
            } else {
                out.push_str(line);
                out.push('\n');
            }
        }
        out.push('\n');
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("");
    if mode == "compact" {
        let sources = match arg_value(&args, "--sources") {
            Some(v) => v,
            None => "phi/sources.tsv".to_string(),
        };
        let select = match arg_value(&args, "--select") {
            Some(v) => v,
            None => "fluxgate".to_string(),
        };
        let text = match fs::read_to_string(&sources) {
            Ok(t) => t,
            Err(e) => {
                println!("expand: {sources} unreadable: {e}");
                return;
            }
        };
        print!("{}", compact(&text, &select));
    } else if mode == "flat" {
        let path = match arg_value(&args, "--definition") {
            Some(v) => v,
            None => {
                println!("expand: --definition missing");
                return;
            }
        };
        print!("{}", flat(Path::new(&path)));
    } else {
        println!("usage: register_expand compact --sources <tsv> --select <text>");
        println!("       register_expand flat --definition <phi>");
    }
}
