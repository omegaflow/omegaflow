use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

const FORCES: [&str; 9] = [
    "em",
    "gravity",
    "acoustic",
    "seismic-body",
    "seismic-surface",
    "thermal",
    "diffusion",
    "advective",
    "electric",
];

const GEOMETRY_HINTS: [&str; 15] = [
    "depth",
    "distance",
    "height",
    "magnitude",
    "latitude",
    "longitude",
    "lat",
    "lon",
    "area",
    "altitude",
    "azimuth",
    "angle",
    "index",
    "ratio",
    "coordinate",
];

const REGISTERS: [&str; 6] = [
    "phi/sources.φ",
    "phi/declined_sources.φ",
    "phi/pipeline/ledger.φ",
    "tools/utils/data/library.φ",
    "phi/witnesses.φ",
    "tools/register/data/bands.φ",
];

fn is_force(token: &str) -> bool {
    FORCES.contains(&token)
}

fn geometry_candidate(line: &str) -> bool {
    let lower = line.to_lowercase();
    GEOMETRY_HINTS.iter().any(|h| lower.contains(h))
}

struct FileReport {
    field_lines: usize,
    quantity_lines: usize,
    groups: BTreeMap<(String, String), (usize, Vec<String>)>,
    geometry: usize,
}

fn analyze(content: &str) -> FileReport {
    let mut report = FileReport {
        field_lines: 0,
        quantity_lines: 0,
        groups: BTreeMap::new(),
        geometry: 0,
    };
    for line in content.lines() {
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("field") => {
                report.field_lines += 1;
                let all: Vec<&str> = line.split_whitespace().collect();
                if let Some(idx) = all.iter().position(|t| is_force(t)) {
                    let force = all[idx];
                    let kernel = if idx > 0 { all[idx - 1] } else { "?" };
                    let entry = report
                        .groups
                        .entry((force.to_string(), kernel.to_string()))
                        .or_insert((0, Vec::new()));
                    entry.0 += 1;
                    if entry.1.len() < 2 {
                        entry.1.push(line.trim().to_string());
                    }
                }
                if geometry_candidate(line) {
                    report.geometry += 1;
                }
            }
            Some("quantity") => report.quantity_lines += 1,
            _ => {}
        }
    }
    report
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!(
            "usage: source_physics_lint [--root <dir>]  — read-only: groups phi/*.φ field lines by force × kernel"
        );
        return;
    }
    let mut root = PathBuf::from(".");
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--root" {
            i += 1;
            if let Some(r) = args.get(i) {
                root = PathBuf::from(r);
            }
        }
        i += 1;
    }

    for rel in REGISTERS {
        let path = root.join(rel);
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let report = analyze(&content);
        if report.field_lines == 0 && report.quantity_lines == 0 {
            continue;
        }
        println!(
            "== {} == field {} · quantity {} · geometry-candidate {}",
            rel, report.field_lines, report.quantity_lines, report.geometry
        );
        let mut groups: Vec<_> = report.groups.iter().collect();
        groups.sort_by(|a, b| b.1.0.cmp(&a.1.0));
        for ((force, kernel), (count, examples)) in groups {
            println!("  {force} × {kernel}: {count}");
            for ex in examples {
                println!("      {ex}");
            }
        }
        println!();
    }
}
