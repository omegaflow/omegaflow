use std::fs;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let register = match arg_value(&args, "--register") {
        Some(v) => v,
        None => {
            println!("usage: entry_check --register <sources.φ>");
            return;
        }
    };
    let text = match fs::read_to_string(&register) {
        Ok(t) => t,
        Err(e) => {
            println!("entry_check: {register}: {e}");
            std::process::exit(2);
        }
    };

    let mut blocks = 0usize;
    let mut missing = [0usize; 7];
    let mut coverage_missing = 0usize;
    let mut compiler_missing = 0usize;

    for block in text.split("\n\n") {
        if block.trim().is_empty() {
            continue;
        }
        blocks += 1;
        let mut has = [false; 9];
        for line in block.lines() {
            let line = line.trim();
            let artifact = line.starts_with("artifact ");
            if line.starts_with("sha256 ") || (artifact && line.contains("sha256=")) {
                has[0] = true;
            }
            if line.starts_with("terms ") {
                has[1] = true;
            }
            if line.starts_with("origin ") || (artifact && line.contains("origin=")) {
                has[2] = true;
            }
            if line.starts_with("instrument ") || line.starts_with("network ") {
                has[3] = true;
            }
            if line.starts_with("at ")
                || line.starts_with("on ")
                || line.starts_with("query ")
                || line.starts_with("track ")
                || line.starts_with("sky ")
                || line.starts_with("global ")
            {
                has[4] = true;
            }
            if line.starts_with("field ")
                || line.starts_with("quantity ")
                || line.starts_with("first ")
                || line.starts_with("last ")
                || line.starts_with("lastrow ")
            {
                has[5] = true;
            }
            if line.starts_with("format ") || line.starts_with("hapi ") {
                has[6] = true;
            }
            if line.starts_with("coverage ") || (artifact && line.contains("coverage=")) {
                has[7] = true;
            }
            if line.starts_with("compiler ") {
                has[8] = true;
            }
        }
        for (i, present) in has.iter().take(7).enumerate() {
            if !present {
                missing[i] += 1;
            }
        }
        if !has[7] {
            coverage_missing += 1;
        }
        if !has[8] {
            compiler_missing += 1;
        }
    }

    let names = [
        "sha256",
        "terms",
        "origin",
        "instrument",
        "anchor",
        "fields",
        "format",
    ];
    println!("entry_check: {blocks} blocks");
    for (i, name) in names.iter().enumerate() {
        println!("  {name:<10} missing {}", missing[i]);
    }
    println!("  coverage   missing {coverage_missing}");
    println!("  compiler   missing {compiler_missing}");

    let required: usize = missing.iter().sum();
    if required > 0 {
        std::process::exit(1);
    }
}
