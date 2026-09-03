use omegaflow::force::{force_name_of, gate_weigh, parse_library};
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("source_scanner <library.φ> <katalog.φ> [ausgabe.txt]");
        std::process::exit(1);
    }
    let library = match std::fs::read_to_string(&args[1]) {
        Ok(c) => parse_library(&c),
        Err(_) => {
            eprintln!("library unreadable: {}", args[1]);
            std::process::exit(1);
        }
    };
    let content = match std::fs::read_to_string(&args[2]) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("catalog unreadable: {}", args[2]);
            std::process::exit(1);
        }
    };
    let base = args[2]
        .rsplit('/')
        .next()
        .unwrap_or("catalog")
        .trim_end_matches(".φ")
        .to_string();
    let out_path = match args.get(3) {
        Some(p) => p.clone(),
        None => format!("phi/pipeline/weights_{}.txt", base),
    };
    let mut out = String::new();
    for line in content.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') && !t.starts_with("# dataset ") {
            continue;
        }
        let text = if t.starts_with("catalog ") {
            t.trim_start_matches("catalog ").to_string()
        } else if t.starts_with("candidate ") {
            t.trim_start_matches("candidate ").to_string()
        } else if t.starts_with("url ") {
            t.trim_start_matches("url ").to_string()
        } else if t.starts_with("# dataset ") {
            t.trim_start_matches("# dataset ").to_string()
        } else {
            t.to_string()
        };
        let g = gate_weigh(&text, &library);
        let force_name = g.force.and_then(force_name_of).unwrap_or("-");
        out.push_str(&format!("{} | {} | {}\n", g.weight, force_name, text));
    }
    match std::fs::File::create(&out_path).and_then(|mut f| {
        f.write_all(format!("# {} tags in the library\n", library.len()).as_bytes())
            .and(f.write_all(out.as_bytes()))
    }) {
        Ok(()) => {
            eprintln!(
                "source_scanner: {} records → {}",
                count_records(&content),
                out_path
            );
        }
        Err(_) => {
            eprintln!("source_scanner: output unwritable: {}", out_path);
            std::process::exit(1);
        }
    }
}

fn count_records(content: &str) -> usize {
    content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#')
        })
        .count()
}
