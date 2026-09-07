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
    eprintln!("  reports ttl-order violations (url + ttl), exit 0 when ttl-ascending");
    eprintln!("  --write re-orders blocks by (ttl asc, url asc) into the file");
}

struct Block {
    lines: Vec<String>,
    url: String,
    ttl: u64,
}

fn split_blocks(content: &str) -> Result<Vec<Block>, String> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                blocks.push(build_block(&current)?);
                current.clear();
            }
        } else {
            current.push(line.to_string());
        }
    }
    if !current.is_empty() {
        blocks.push(build_block(&current)?);
    }
    Ok(blocks)
}

fn build_block(lines: &[String]) -> Result<Block, String> {
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
            url,
            ttl,
        }),
        None => Err(format!("block has no ttl line (opening '{}')", lines[0])),
    }
}

fn report(path: &str, blocks: &[Block]) {
    let n = blocks.len();
    let violations = ttl_violations(blocks);
    let notes = url_notes(blocks);
    for &p in &violations {
        println!(
            "ttl-order violation: ttl {} at {} placed after ttl {}",
            blocks[p].ttl,
            blocks[p].url,
            blocks[p - 1].ttl
        );
    }
    for &p in &notes {
        println!(
            "url-order note within ttl {}: {}",
            blocks[p].ttl, blocks[p].url
        );
    }
    if violations.is_empty() {
        println!(
            "register {} is ttl-ascending across {} blocks ({} url-order note(s) within equal ttl)",
            path,
            n,
            notes.len()
        );
        std::process::exit(0);
    }
    println!(
        "register {} holds {} ttl-order violation(s) across {} blocks",
        path,
        violations.len(),
        n
    );
    std::process::exit(1);
}

fn ttl_violations(blocks: &[Block]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    for i in 1..blocks.len() {
        if blocks[i].ttl < blocks[i - 1].ttl {
            out.push(i);
        }
    }
    out
}

fn url_notes(blocks: &[Block]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < blocks.len() {
        let ttl = blocks[i].ttl;
        let mut j = i;
        while j + 1 < blocks.len() && blocks[j + 1].ttl == ttl {
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

fn sorted_order(blocks: &[Block]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..blocks.len()).collect();
    order.sort_by(|a, b| {
        blocks[*a]
            .ttl
            .cmp(&blocks[*b].ttl)
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
    let order = sorted_order(blocks);
    let identity: Vec<usize> = (0..blocks.len()).collect();
    let out = render(blocks, &order);
    if order == identity && out == content {
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
        "register {} rewritten sorted by (ttl asc, url asc): {} block(s) changed position",
        path, moved
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

    #[test]
    fn parse_keeps_block_line_order() {
        let blocks = split_blocks(&sample()).expect("sample parses");
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[1].url, "https://a.example/data");
        assert_eq!(blocks[1].ttl, 5);
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
        assert_eq!(blocks[v[0]].ttl, 5);
    }

    #[test]
    fn equal_ttl_url_note_detected() {
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
        let notes = url_notes(&blocks);
        assert_eq!(notes, vec![1]);
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
}
