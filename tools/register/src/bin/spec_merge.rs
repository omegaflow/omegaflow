use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

const REG: &str = "tools/register/data/instruments.φ";
const PHI: &str = "phi/sources.φ";

fn blocks(text: &str) -> Vec<(usize, Vec<String>)> {
    let mut out = Vec::new();
    let mut cur: Vec<String> = Vec::new();
    let mut start = 0usize;
    for (idx, line) in text.lines().enumerate() {
        let begins = line.starts_with("host ") || line.starts_with("zustand ");
        if begins && !cur.is_empty() {
            out.push((start, std::mem::take(&mut cur)));
        }
        if line.trim().is_empty() {
            if !cur.is_empty() {
                out.push((start, std::mem::take(&mut cur)));
            }
            continue;
        }
        if cur.is_empty() {
            start = idx + 1;
        }
        cur.push(line.to_string());
    }
    if !cur.is_empty() {
        out.push((start, cur));
    }
    out
}

fn field(block: &[String], name: &str) -> Option<String> {
    block.iter().find_map(|l| {
        let p: Vec<&str> = l.split_whitespace().collect();
        (p.first() == Some(&name)).then(|| p[1..].join(" "))
    })
}

fn host_of(block: &[String]) -> Option<String> {
    let url = field(block, "applies").or_else(|| field(block, "probe"))?;
    let rest = url.split_once("://")?.1;
    let host = rest.split(['/', '?', ':']).next()?;
    Some(host.trim_start_matches("www.").to_string())
}

fn product_of(block: &[String]) -> Option<String> {
    let applies = field(block, "applies")?;
    if let Some(i) = applies.find("table=") {
        return Some(applies[i + 6..].split_whitespace().next()?.to_string());
    }
    if let Some(i) = applies.find("source=") {
        return Some(applies[i + 7..].split('&').next()?.to_string());
    }
    let probe = field(block, "probe")?;
    Some(probe.split_whitespace().last()?.to_string())
}

fn cites(block: &[String]) -> usize {
    block
        .iter()
        .filter(|l| {
            let p: Vec<&str> = l.split_whitespace().collect();
            matches!(
                p.first(),
                Some(&"api_cite") | Some(&"instrument_cite") | Some(&"host_cite")
            )
        })
        .count()
}

fn normalize(block: &mut [String]) {
    for line in block.iter_mut() {
        if let Some(rest) = line.strip_prefix("  cadence ") {
            *line = format!("  interval {rest}");
        }
        if line.starts_with("  probe probe ") {
            *line = line.replacen("  probe probe ", "  probe ", 1);
        }
    }
}

fn merge_rows(out_path: &str, state_path: &str) {
    let text = fs::read_to_string(out_path).expect("out file");
    let mut reg_text = fs::read_to_string(REG).expect("register");
    if !reg_text.ends_with('\n') {
        reg_text.push('\n');
    }
    let mut rows = blocks(&reg_text);
    rows.retain(|(_, b)| b.first().is_some_and(|l| l.starts_with("host ")));
    let mut seen: BTreeMap<(String, String), usize> = BTreeMap::new();
    for (i, (_, b)) in rows.iter().enumerate() {
        if let (Some(h), Some(p)) = (host_of(b), product_of(b)) {
            seen.entry((h, p)).or_insert(i);
        }
    }
    let mut states = String::new();
    let mut added = 0usize;
    let mut replaced = 0usize;
    for (_, mut b) in blocks(&text) {
        if b.first().is_some_and(|l| l.starts_with("zustand ")) {
            states.push_str(&b.join("\n"));
            states.push_str("\n\n");
            continue;
        }
        normalize(&mut b);
        if let (Some(h), Some(p)) = (host_of(&b), product_of(&b)) {
            if let Some(&i) = seen.get(&(h.clone(), p.clone())) {
                if cites(&b) > cites(&rows[i].1) {
                    rows[i].1 = b;
                    replaced += 1;
                }
                continue;
            }
            seen.insert((h, p), rows.len());
            rows.push((0, b));
            added += 1;
        }
    }
    let mut out = String::new();
    for (_, b) in &rows {
        out.push_str(&b.join("\n"));
        out.push_str("\n\n");
    }
    fs::write(REG, out).expect("register write");
    if !states.is_empty() {
        let mut old = match fs::read_to_string(state_path) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        if !old.is_empty() && !old.ends_with("\n\n") {
            old.push_str("\n\n");
        }
        old.push_str(&states);
        fs::write(state_path, old).expect("states write");
    }
    println!("rows: +{added} neu, {replaced} ersetzt, {} gesamt", rows.len());
}

fn apply_instruments(tsv_path: &str) {
    let tsv = fs::read_to_string(tsv_path).expect("tsv");
    let mut map: BTreeMap<usize, String> = BTreeMap::new();
    for line in tsv.lines() {
        let mut it = line.split('\t');
        let (Some(n), Some(inst)) = (it.next(), it.next()) else {
            continue;
        };
        let Ok(n) = n.trim().parse::<usize>() else {
            continue;
        };
        if inst.trim().eq_ignore_ascii_case("pending") {
            continue;
        }
        map.insert(n, inst.trim().to_string());
    }
    let text = fs::read_to_string(PHI).expect("phi");
    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    let starts: Vec<usize> = blocks(&text).into_iter().map(|(s, _)| s).collect();
    let mut added = 0usize;
    for (idx, start) in starts.iter().enumerate() {
        let Some(inst) = map.get(start) else {
            continue;
        };
        let end = starts.get(idx + 1).map(|s| s - 1).unwrap_or(lines.len());
        let has = lines[start - 1..end.min(lines.len())]
            .iter()
            .any(|l| l.starts_with("instrument "));
        if has {
            continue;
        }
        if let Some(pos) = lines[start - 1..end.min(lines.len())]
            .iter()
            .position(|l| l.starts_with("origin "))
        {
            lines.insert(start + pos, format!("instrument {inst}"));
            added += 1;
        }
    }
    fs::write(PHI, lines.join("\n") + "\n").expect("phi write");
    println!("instrument: {added} eingesetzt");
}

fn host_docs() {
    let text = fs::read_to_string(REG).expect("register");
    let mut out = String::new();
    let mut docs = 0usize;
    let mut failed = 0usize;
    let mut rows = 0usize;
    let mut with_probe = 0usize;
    for (_, mut b) in blocks(&text) {
        rows += 1;
        if field(&b, "probe").is_some() {
            with_probe += 1;
        }
        if b.iter().any(|l| l.starts_with("  host_cite ")) {
            out.push_str(&b.join("\n"));
            out.push_str("\n\n");
            continue;
        }
        let Some(probe) = field(&b, "probe").and_then(|p| p.split_whitespace().last().map(str::to_string)) else {
            out.push_str(&b.join("\n"));
            out.push_str("\n\n");
            continue;
        };
        let mut headers = String::new();
        for args in [vec!["-sSI"], vec!["-sS", "-r", "0-0"]] {
            let mut cmd = Command::new("curl");
            cmd.args(["--max-time", "15"]);
            cmd.args(&args);
            cmd.arg(&probe);
            let Ok(res) = cmd.output() else { continue };
            headers = String::from_utf8_lossy(&res.stdout).to_string();
            if headers.starts_with("HTTP/") {
                break;
            }
            headers.clear();
        }
        let mut quote = String::new();
        for line in headers.lines() {
            let low = line.to_ascii_lowercase();
            if low.starts_with("cache-control:") || low.starts_with("last-modified:") || low.starts_with("etag:") {
                quote = line.trim().to_string();
                if low.starts_with("cache-control:") {
                    break;
                }
            }
        }
        if quote.is_empty() {
            failed += 1;
        } else {
            let at = b
                .iter()
                .position(|l| {
                    l.starts_with("  probe ")
                        || l.starts_with("  applies ")
                        || l.starts_with("  note ")
                })
                .unwrap_or(b.len());
            b.insert(at, format!("  host_cite {probe}"));
            b.insert(at + 1, format!("  host_quote \"{quote}\""));
            docs += 1;
        }
        out.push_str(&b.join("\n"));
        out.push_str("\n\n");
    }
    fs::write(REG, out).expect("register write");
    println!("rows {rows}, mit probe {with_probe}, host_cite: {docs} ergaenzt, {failed} ohne Header");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("rows") if args.len() >= 4 => merge_rows(&args[2], &args[3]),
        Some("instruments") if args.len() >= 3 => apply_instruments(&args[2]),
        Some("hostdocs") => host_docs(),
        _ => {
            eprintln!("spec_merge rows <out.φ> <states.φ> | instruments <tsv> | hostdocs");
            std::process::exit(2);
        }
    }
}
