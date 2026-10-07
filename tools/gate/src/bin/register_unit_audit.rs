use omegaflow::commit_gate::register_field_unit_issues;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut path = "phi/sources.φ".to_string();
    let mut pairs = false;
    let mut lines = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                if let Some(p) = args.get(i) {
                    path = p.clone();
                }
            }
            "--pairs" => pairs = true,
            "--lines" => lines = true,
            _ => {}
        }
        i += 1;
    }
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("register_unit_audit: {path} is not a readable file");
            std::process::exit(2);
        }
    };
    let issues = register_field_unit_issues(&content);
    if pairs {
        let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
        for it in &issues {
            seen.insert((it.force.clone(), it.unit.clone()));
        }
        for (force, unit) in seen {
            println!("{force} {unit}");
        }
        return;
    }
    if lines {
        for it in &issues {
            println!("{path}:{}: {} {} ({})", it.line, it.force, it.unit, it.kind);
        }
        return;
    }
    let mut counts: BTreeMap<(String, String, &'static str), usize> = BTreeMap::new();
    for it in &issues {
        *counts
            .entry((it.force.clone(), it.unit.clone(), it.kind))
            .or_insert(0) += 1;
    }
    println!(
        "register_unit_audit: {path}: {} non-canonical field lines, {} distinct pairs",
        issues.len(),
        counts.len()
    );
    for ((force, unit, kind), n) in &counts {
        println!("{n:5}  {force:14} {unit:18} {kind}");
    }
}
