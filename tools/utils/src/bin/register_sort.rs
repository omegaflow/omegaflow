use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

const DEFAULT_REGISTER: &str = "phi/sources.φ";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = String::from(DEFAULT_REGISTER);
    let mut write = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--write" => write = true,
            other if other.starts_with('-') => {
                eprintln!("register_sort: unknown flag '{}'", other);
                usage();
                std::process::exit(2);
            }
            other => {
                if path == DEFAULT_REGISTER {
                    path = other.to_string();
                } else {
                    eprintln!("register_sort: multiple paths '{}' '{}'", path, other);
                    usage();
                    std::process::exit(2);
                }
            }
        }
        i += 1;
    }
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("register_sort: read {}: {}", path, e);
            std::process::exit(2);
        }
    };
    let blocks = match split_blocks(&content) {
        Ok(b) => b,
        Err(why) => {
            eprintln!("register_sort: {}: {}", path, why);
            std::process::exit(2);
        }
    };
    if write {
        match rewrite(&path, &content, &blocks) {
            Ok(()) => std::process::exit(0),
            Err(why) => {
                eprintln!("register_sort: {}: {}", path, why);
                std::process::exit(2);
            }
        }
    }
    report(&path, &blocks);
}

fn usage() {
    eprintln!("usage: register_sort [path] [--write]");
    eprintln!("  sources register (opener 'url', ttl mandatory): sorts (ttl asc, url asc)");
    eprintln!("  disposition register (opener 'decline'/'dead'/'key-needed'/'parser-def'/'pending'/'descoped',");
    eprintln!("  sort key url, ttl optional): sorts (ttl asc, url asc), dedupes exact blocks");
    eprintln!("  reports order violations; exit 0 only when canonical");
    eprintln!("  --write re-orders and dedupes exact duplicate blocks into the file");
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RegisterKind {
    Sources,
    Disposition,
}

#[derive(Clone)]
struct Block {
    lines: Vec<String>,
    opener: String,
    url: String,
    ttl: Option<u64>,
}

impl Block {
    fn ttl_key(&self) -> Option<u64> {
        self.ttl
    }

    fn ttl_label(&self) -> String {
        match self.ttl {
            Some(ttl) => ttl.to_string(),
            None => "none".to_string(),
        }
    }

    fn identity(&self) -> String {
        self.lines.join("\n")
    }
}

fn first_token(line: &str) -> &str {
    line.trim_start().split_whitespace().next().unwrap_or("")
}

fn register_kind(first_line: &str) -> Result<RegisterKind, String> {
    match first_token(first_line) {
        "url" => Ok(RegisterKind::Sources),
        "decline" | "dead" | "key-needed" | "parser-def" | "pending" | "descoped" => {
            Ok(RegisterKind::Disposition)
        }
        other => Err(format!(
            "unknown block opener '{}' (register kind undetermined)",
            other
        )),
    }
}

fn split_blocks(content: &str) -> Result<Vec<Block>, String> {
    let kind = match content.lines().find(|l| !l.trim().is_empty()) {
        Some(first) => register_kind(first)?,
        None => return Ok(Vec::new()),
    };
    let mut blocks: Vec<Block> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                blocks.push(build_block(&current, kind)?);
                current.clear();
            }
        } else {
            current.push(line.to_string());
        }
    }
    if !current.is_empty() {
        blocks.push(build_block(&current, kind)?);
    }
    Ok(blocks)
}

fn build_block(lines: &[String], kind: RegisterKind) -> Result<Block, String> {
    match kind {
        RegisterKind::Sources => build_sources_block(lines),
        RegisterKind::Disposition => build_disposition_block(lines),
    }
}

fn build_sources_block(lines: &[String]) -> Result<Block, String> {
    let first = &lines[0];
    let trimmed_first = first.trim_start();
    let url = match trimmed_first.split_once(char::is_whitespace) {
        Some(("url", rest)) => rest.trim().to_string(),
        _ => return Err(format!("block does not open with 'url': '{}'", lines[0])),
    };
    let mut ttl: Option<u64> = None;
    for line in lines.iter().skip(1) {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("ttl ") {
            let value = rest.trim();
            match value.parse::<u64>() {
                Ok(v) => {
                    ttl = Some(v);
                    break;
                }
                Err(_) => return Err(format!("block ttl is not an integer: '{}'", line.trim())),
            }
        }
    }
    match ttl {
        Some(ttl) => Ok(Block {
            lines: lines.to_vec(),
            opener: "url".to_string(),
            url,
            ttl: Some(ttl),
        }),
        None => Err(format!("block has no ttl line (opening '{}')", lines[0])),
    }
}

fn build_disposition_block(lines: &[String]) -> Result<Block, String> {
    let opener = first_token(&lines[0]).to_string();
    let mut url: Option<String> = None;
    let mut ttl: Option<u64> = None;
    for line in lines {
        let trimmed = line.trim();
        if url.is_none() {
            if let Some(rest) = trimmed.strip_prefix("url ") {
                url = Some(rest.trim().to_string());
            }
        }
        if let Some(rest) = trimmed.strip_prefix("ttl ") {
            match rest.trim().parse::<u64>() {
                Ok(v) => ttl = Some(v),
                Err(_) => return Err(format!("block ttl is not an integer: '{}'", trimmed)),
            }
        }
    }
    match url {
        Some(url) => Ok(Block {
            lines: lines.to_vec(),
            opener,
            url,
            ttl,
        }),
        None => Err(format!(
            "disposition block has no url line (opening '{}')",
            lines[0]
        )),
    }
}

fn report(path: &str, blocks: &[Block]) {
    let n = blocks.len();
    let ttl_bad = ttl_violations(blocks);
    let url_bad = url_violations(blocks);
    for &p in &ttl_bad {
        println!(
            "ttl-order violation: ttl {} at {} placed after ttl {}",
            blocks[p].ttl_label(),
            blocks[p].url,
            blocks[p - 1].ttl_label()
        );
    }
    for &p in &url_bad {
        println!(
            "url-order violation within ttl {}: {} placed after {}",
            blocks[p].ttl_label(),
            blocks[p].url,
            blocks[p - 1].url
        );
    }
    let exact = exact_duplicate_indices(blocks);
    for &i in &exact {
        println!(
            "exact duplicate block (same opener '{}', url, note): {}",
            blocks[i].opener, blocks[i].url
        );
    }
    for (url, openers) in distinct_duplicate_groups(blocks) {
        println!(
            "duplicate url (distinct verdict, kept): {} -> {}",
            url,
            openers.join(" | ")
        );
    }
    if ttl_bad.is_empty() && url_bad.is_empty() && exact.is_empty() {
        println!(
            "register {} is canonical (ttl asc, url asc, no exact duplicates) across {} blocks",
            path, n
        );
        std::process::exit(0);
    }
    println!(
        "register {} holds {} ttl-order and {} url-order violation(s), {} exact duplicate(s) across {} blocks",
        path,
        ttl_bad.len(),
        url_bad.len(),
        exact.len(),
        n
    );
    std::process::exit(1);
}

fn ttl_violations(blocks: &[Block]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    for i in 1..blocks.len() {
        if blocks[i].ttl_key() < blocks[i - 1].ttl_key() {
            out.push(i);
        }
    }
    out
}

fn url_violations(blocks: &[Block]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < blocks.len() {
        let ttl = blocks[i].ttl_key();
        let mut j = i;
        while j + 1 < blocks.len() && blocks[j + 1].ttl_key() == ttl {
            j += 1;
        }
        if j > i {
            let mut best = &blocks[i].url;
            for k in (i + 1)..=j {
                if blocks[k].url < *best {
                    out.push(k);
                } else {
                    best = &blocks[k].url;
                }
            }
        }
        i = j + 1;
    }
    out
}

fn duplicate_url_groups(blocks: &[Block]) -> Vec<(String, Vec<usize>)> {
    let mut order: Vec<String> = Vec::new();
    let mut map: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, b) in blocks.iter().enumerate() {
        if !map.contains_key(&b.url) {
            order.push(b.url.clone());
        }
        map.entry(b.url.clone()).or_default().push(i);
    }
    let mut out: Vec<(String, Vec<usize>)> = Vec::new();
    for url in order {
        if let Some(idxs) = map.remove(&url) {
            if idxs.len() > 1 {
                out.push((url, idxs));
            }
        }
    }
    out
}

fn exact_duplicate_indices(blocks: &[Block]) -> Vec<usize> {
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut out: Vec<usize> = Vec::new();
    for (i, b) in blocks.iter().enumerate() {
        let key = b.identity();
        if seen.contains_key(&key) {
            out.push(i);
        } else {
            seen.insert(key, i);
        }
    }
    out
}

fn distinct_duplicate_groups(blocks: &[Block]) -> Vec<(String, Vec<String>)> {
    let mut out: Vec<(String, Vec<String>)> = Vec::new();
    for (url, idxs) in duplicate_url_groups(blocks) {
        let first = blocks[idxs[0]].identity();
        if idxs.iter().any(|&i| blocks[i].identity() != first) {
            let openers: Vec<String> = idxs.iter().map(|&i| blocks[i].opener.clone()).collect();
            out.push((url, openers));
        }
    }
    out
}

fn sorted_order(blocks: &[Block]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..blocks.len()).collect();
    order.sort_by(|a, b| {
        blocks[*a]
            .ttl_key()
            .cmp(&blocks[*b].ttl_key())
            .then_with(|| blocks[*a].url.cmp(&blocks[*b].url))
    });
    order
}

fn render(blocks: &[Block], order: &[usize]) -> String {
    let mut out: Vec<String> = Vec::new();
    for (pos, &idx) in order.iter().enumerate() {
        if pos > 0 {
            out.push(String::new());
        }
        out.extend(blocks[idx].lines.iter().cloned());
    }
    out.join("\n") + "\n"
}

fn rewrite(path: &str, content: &str, blocks: &[Block]) -> Result<(), String> {
    let drop: HashSet<usize> = exact_duplicate_indices(blocks).into_iter().collect();
    let removed = drop.len();
    let kept: Vec<Block> = blocks
        .iter()
        .enumerate()
        .filter(|(i, _)| !drop.contains(i))
        .map(|(_, b)| b.clone())
        .collect();
    let order = sorted_order(&kept);
    let identity: Vec<usize> = (0..kept.len()).collect();
    let out = render(&kept, &order);
    if removed == 0 && order == identity && out == content {
        println!(
            "register {} already sorted by (ttl, url); file untouched",
            path
        );
        return Ok(());
    }
    let moved = identity
        .iter()
        .zip(order.iter())
        .filter(|(a, b)| a != b)
        .count();
    fs::write(path, &out).map_err(|e| format!("write {}: {}", path, e))?;
    println!(
        "register {} rewritten sorted by (ttl asc, url asc): {} block(s) changed position, {} exact duplicate(s) removed",
        path, moved, removed
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> String {
        [
            "url https://b.example/data",
            "ttl 10",
            "at earth",
            "field b b inverse-square em V 10 0.0 0.0",
            "",
            "url https://a.example/data",
            "ttl 5",
            "at earth",
            "field a a inverse-square em V 5 0.0 0.0",
            "",
            "url https://c.example/data",
            "ttl 10",
            "at earth",
            "field c c inverse-square em V 10 0.0 0.0",
        ]
        .join("\n")
            + "\n"
    }

    fn disposition_sample() -> String {
        [
            "decline model",
            "url https://b.example/x",
            "note b",
            "",
            "decline model",
            "url https://a.example/x",
            "note a",
            "",
            "decline model",
            "url https://a.example/x",
            "note a",
        ]
        .join("\n")
            + "\n"
    }

    #[test]
    fn parse_keeps_block_line_order() {
        let blocks = split_blocks(&sample()).expect("sample parses");
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[1].url, "https://a.example/data");
        assert_eq!(blocks[1].ttl, Some(5));
        assert_eq!(blocks[1].lines[2], "at earth");
    }

    #[test]
    fn ttl_desc_detected() {
        let text = [
            "url https://b.example/data",
            "ttl 10",
            "field b b inverse-square em V 10 0.0 0.0",
            "",
            "url https://a.example/data",
            "ttl 5",
            "field a a inverse-square em V 5 0.0 0.0",
        ]
        .join("\n")
            + "\n";
        let blocks = split_blocks(&text).expect("sample parses");
        let v = ttl_violations(&blocks);
        assert_eq!(v, vec![1]);
        assert_eq!(blocks[v[0]].ttl_key(), Some(5));
    }

    #[test]
    fn equal_ttl_url_violation_detected() {
        let text = [
            "url https://z.example/data",
            "ttl 10",
            "field z z inverse-square em V 10 0.0 0.0",
            "",
            "url https://a.example/data",
            "ttl 10",
            "field a a inverse-square em V 10 0.0 0.0",
        ]
        .join("\n")
            + "\n";
        let blocks = split_blocks(&text).expect("sample parses");
        let bad = url_violations(&blocks);
        assert_eq!(bad, vec![1]);
    }

    #[test]
    fn render_separates_blocks_by_one_blank_line() {
        let blocks = split_blocks(&sample()).expect("sample parses");
        let order: Vec<usize> = (0..blocks.len()).collect();
        assert_eq!(render(&blocks, &order), sample());
    }

    #[test]
    fn sort_keys_ttl_then_url() {
        let blocks = split_blocks(&sample()).expect("sample parses");
        let order = sorted_order(&blocks);
        let urls: Vec<&str> = order.iter().map(|&i| blocks[i].url.as_str()).collect();
        assert_eq!(
            urls,
            vec![
                "https://a.example/data",
                "https://b.example/data",
                "https://c.example/data"
            ]
        );
        let text = render(&blocks, &order);
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines[0], "url https://a.example/data");
        assert_eq!(lines[5], "url https://b.example/data");
        assert_eq!(lines[10], "url https://c.example/data");
    }

    #[test]
    fn stable_sort_keeps_identical_keys_in_place() {
        let text = [
            "url https://x.example/1",
            "ttl 7",
            "field p p inverse-square em V 7 0.0 0.0",
            "",
            "url https://x.example/1",
            "ttl 7",
            "field q q inverse-square em V 7 0.0 0.0",
        ]
        .join("\n")
            + "\n";
        let blocks = split_blocks(&text).expect("parses");
        let order = sorted_order(&blocks);
        assert_eq!(order, vec![0, 1]);
    }

    #[test]
    fn disposition_register_sorted_and_duplicate_detected() {
        let blocks = split_blocks(&disposition_sample()).expect("disposition parses");
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].opener, "decline");
        assert_eq!(blocks[1].url, "https://a.example/x");
        assert_eq!(blocks[1].ttl, None);

        let bad = url_violations(&blocks);
        assert_eq!(bad, vec![1]);

        let exact = exact_duplicate_indices(&blocks);
        assert_eq!(exact, vec![2]);

        let order = sorted_order(&blocks);
        let urls: Vec<&str> = order.iter().map(|&i| blocks[i].url.as_str()).collect();
        assert_eq!(
            urls,
            vec![
                "https://a.example/x",
                "https://a.example/x",
                "https://b.example/x"
            ]
        );
    }

    #[test]
    fn disposition_distinct_verdict_duplicate_is_reported_not_deduped() {
        let text = [
            "decline analysis",
            "url https://a.example/x",
            "note one",
            "",
            "decline count",
            "url https://a.example/x",
            "note two",
        ]
        .join("\n")
            + "\n";
        let blocks = split_blocks(&text).expect("disposition parses");
        assert!(exact_duplicate_indices(&blocks).is_empty());
        let groups = distinct_duplicate_groups(&blocks);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].0, "https://a.example/x");
        assert_eq!(groups[0].1, vec!["decline", "decline"]);
    }

    #[test]
    fn disposition_kind_and_optional_ttl_parse() {
        let text = [
            "parser-def",
            "url https://c.example/x",
            "ttl 3",
            "note c",
        ]
        .join("\n")
            + "\n";
        let blocks = split_blocks(&text).expect("disposition parses");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].opener, "parser-def");
        assert_eq!(blocks[0].ttl_key(), Some(3));
    }
}
