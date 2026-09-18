use std::collections::HashSet;
use std::env;
use std::fs;
use std::process::exit;

const REGISTER: &str = "phi/harvest.φ";

struct Block {
    start: usize,
    lines: Vec<String>,
}

impl Block {
    fn field(&self, key: &str) -> Option<&str> {
        self.lines
            .iter()
            .filter_map(|l| parse_field(l))
            .find_map(|(k, v)| (k == key).then_some(v))
    }

    fn asset(&self) -> Option<&str> {
        self.lines
            .first()
            .and_then(|l| parse_field(l))
            .and_then(|(k, v)| (k == "asset").then_some(v))
    }
}

fn parse_field(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(' ')?;
    if key.is_empty() || value.is_empty() {
        return None;
    }
    Some((key, value))
}

fn blocks(text: &str) -> Vec<Block> {
    let mut out = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    let mut start = 0usize;
    for (idx, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            if !lines.is_empty() {
                out.push(Block { start, lines });
                lines = Vec::new();
            }
            continue;
        }
        if lines.is_empty() {
            start = idx + 1;
        }
        lines.push(line.to_string());
    }
    if !lines.is_empty() {
        out.push(Block { start, lines });
    }
    out
}

fn read() -> String {
    match fs::read_to_string(REGISTER) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("harvest_reg: read {REGISTER}: {e}");
            exit(2)
        }
    }
}

fn lookup(fmt: &str) {
    let text = read();
    for block in &blocks(&text) {
        if block.field("format") != Some(fmt) {
            continue;
        }
        for line in &block.lines {
            println!("{line}");
        }
        match block.asset() {
            Some("present") | Some("fehlt") => return,
            _ => {
                eprintln!(
                    "harvest_reg: format '{fmt}' is registered but unmeasured (no asset line) — register duty"
                );
                exit(2)
            }
        }
    }
    eprintln!("harvest_reg: no block for format '{fmt}' in {REGISTER}");
    exit(2)
}

fn arm_search(stem: &str) {
    let text = read();
    for block in &blocks(&text) {
        if block.field("arm") == Some(stem) {
            if let Some(fmt) = block.field("format") {
                println!("format {fmt}");
            }
        }
    }
}

fn check() {
    let text = read();
    let blocks = blocks(&text);
    let mut bad = 0usize;
    let mut formats: Vec<&str> = Vec::new();
    let mut seen = HashSet::new();
    for block in &blocks {
        match block.asset() {
            Some("present") | Some("fehlt") => {}
            Some(other) => {
                eprintln!(
                    "harvest_reg: block at line {}: asset '{other}' is neither present nor fehlt",
                    block.start
                );
                bad += 1;
            }
            None => {
                eprintln!(
                    "harvest_reg: block at line {}: unmeasured (no asset line) — register duty",
                    block.start
                );
                bad += 1;
            }
        }
        match block.field("format") {
            Some(f) => {
                if !seen.insert(f) {
                    eprintln!("harvest_reg: format '{f}' appears in more than one block");
                    bad += 1;
                }
                formats.push(f);
            }
            None => {
                eprintln!(
                    "harvest_reg: block at line {}: no format field",
                    block.start
                );
                bad += 1;
            }
        }
        for key in ["tag", "arm", "pattern", "idempotent"] {
            if block.field(key).is_none() {
                eprintln!("harvest_reg: block at line {}: no {key} field", block.start);
                bad += 1;
            }
        }
        if let Some(shard) = block.field("shard") {
            match shard.parse::<usize>() {
                Ok(n) if n > 0 => {}
                _ => {
                    eprintln!(
                        "harvest_reg: block at line {}: shard '{shard}' is not a positive count",
                        block.start
                    );
                    bad += 1;
                }
            }
        }
        if let Some(timeout) = block.field("timeout") {
            match timeout.parse::<usize>() {
                Ok(n) if n > 0 => {}
                _ => {
                    eprintln!(
                        "harvest_reg: block at line {}: timeout '{timeout}' is not a positive count",
                        block.start
                    );
                    bad += 1;
                }
            }
        }
        if let Some(idem) = block.field("idempotent") {
            if idem != "true" && idem != "false" {
                eprintln!(
                    "harvest_reg: block at line {}: idempotent '{idem}' is neither true nor false",
                    block.start
                );
                bad += 1;
            }
        }
        let mut keys = HashSet::new();
        for line in &block.lines {
            if let Some((key, _)) = parse_field(line) {
                if !keys.insert(key) {
                    eprintln!(
                        "harvest_reg: block at line {}: duplicate field '{key}'",
                        block.start
                    );
                    bad += 1;
                }
            }
        }
        for line in &block.lines {
            match parse_field(line) {
                Some((key, _))
                    if matches!(
                        key,
                        "asset"
                            | "format"
                            | "tag"
                            | "arm"
                            | "args"
                            | "pattern"
                            | "shard"
                            | "timeout"
                            | "idempotent"
                            | "workflow"
                            | "note"
                    ) => {}
                Some((key, _)) => {
                    eprintln!(
                        "harvest_reg: block at line {}: unknown field '{key}'",
                        block.start
                    );
                    bad += 1;
                }
                None => {
                    eprintln!(
                        "harvest_reg: block at line {}: line without key and value: '{line}'",
                        block.start
                    );
                    bad += 1;
                }
            }
        }
    }
    for pair in formats.windows(2) {
        if pair[0] >= pair[1] {
            eprintln!(
                "harvest_reg: blocks out of order: '{}' before '{}'",
                pair[0], pair[1]
            );
            bad += 1;
        }
    }
    if bad > 0 {
        eprintln!("harvest_reg: {bad} violation(s) — no dispatch on an unmeasured register");
        exit(2)
    }
    eprintln!(
        "harvest_reg: {} block(s), all measured and in order",
        formats.len()
    );
}

fn usage() {
    eprintln!("usage: harvest_reg <--lookup <format> | --arm <stem> | --check>");
    eprintln!("  --lookup <format>  print the register block; exit 2 when unknown or unmeasured");
    eprintln!(
        "  --arm <stem>       print 'format <name>' for every block whose arm field is <stem>"
    );
    eprintln!("  --check            gate test: unmeasured or malformed block -> exit 2");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(mode) = args.first().map(|s| s.as_str()) else {
        usage();
        exit(2)
    };
    match mode {
        "--check" => check(),
        "--lookup" => match args.get(1) {
            Some(fmt) => lookup(fmt),
            None => {
                usage();
                exit(2)
            }
        },
        "--arm" => match args.get(1) {
            Some(stem) => arm_search(stem),
            None => {
                usage();
                exit(2)
            }
        },
        other => {
            eprintln!("harvest_reg: unknown mode '{other}'");
            usage();
            exit(2)
        }
    }
}
