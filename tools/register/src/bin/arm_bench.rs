use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

fn query_of(row: &[&str]) -> String {
    let field = row.get(5).copied().unwrap_or("");
    let unit = row.get(6).copied().unwrap_or("");
    let inst = row.get(3).copied().unwrap_or("");
    let mut q = String::new();
    for part in [
        field.replace('_', " "),
        unit.to_string(),
        inst.replace('-', " "),
    ] {
        let p = part.trim();
        if !p.is_empty() && p != "-" {
            q.push_str(p);
            q.push(' ');
        }
    }
    q.trim().to_string()
}

fn run_arm(arm: &str, query: &str) -> Vec<String> {
    let out = Command::new("bin/archive_search")
        .arg(format!("--{arm}"))
        .arg(query)
        .output();
    let Ok(out) = out else { return Vec::new() };
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    text.lines()
        .filter_map(|l| {
            let rest = l.strip_prefix(&format!("[{arm}] url "))?;
            Some(rest.split_whitespace().next()?.to_string())
        })
        .collect()
}

fn read_ok(url: &str, needles: &[String]) -> bool {
    let out = Command::new("bin/archive_search")
        .args(["--scrape", url])
        .output();
    let Ok(out) = out else { return false };
    let text = String::from_utf8_lossy(&out.stdout).to_lowercase();
    let useful = text.len() > 200 && !text.contains("absent — the scraped page");
    useful
        && needles
            .iter()
            .any(|n| !n.is_empty() && text.contains(&n.to_lowercase()))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rows_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("scratch/open.tsv");
    let limit: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(5);
    let arms: Vec<String> = match args.get(3) {
        Some(s) => s.split(',').map(str::to_string).collect(),
        None => vec!["searxng".into(), "tavily".into(), "youcom".into()],
    };
    let text = fs::read_to_string(rows_path).expect("rows");
    let mut lines = text.lines();
    lines.next();
    let mut stats: BTreeMap<String, (usize, usize, usize, usize)> = BTreeMap::new();
    println!("line\tarm\turl\texclusive\ttrue");
    for line in lines.take(limit) {
        let row: Vec<&str> = line.split('\t').collect();
        let q = query_of(&row);
        if q.is_empty() {
            continue;
        }
        let line_no = row.first().copied().unwrap_or("-");
        let mut per: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for arm in &arms {
            per.insert(arm.clone(), run_arm(arm, &q));
        }
        let mut seen: BTreeMap<String, usize> = BTreeMap::new();
        for urls in per.values() {
            for u in urls {
                *seen.entry(u.clone()).or_insert(0) += 1;
            }
        }
        let needles: Vec<String> = vec![
            row.get(6).copied().unwrap_or("").to_string(),
            row.get(3).copied().unwrap_or("").to_string(),
        ];
        for arm in &arms {
            let Some(urls) = per.get(arm) else { continue };
            for u in urls {
                let exclusive = seen.get(u).copied() == Some(1);
                let truth = exclusive && read_ok(u, &needles);
                println!(
                    "{line_no}\t{arm}\t{u}\t{}\t{}",
                    exclusive as u8, truth as u8
                );
                let e = stats.entry(arm.clone()).or_insert((0, 0, 0, 0));
                e.0 += 1;
                if exclusive {
                    e.1 += 1;
                }
                if truth {
                    e.2 += 1;
                }
                e.3 += 1;
            }
        }
    }
    println!("\narm\thits\texclusive\ttrue-exclusive");
    for (arm, (hits, excl, truth, _)) in stats {
        println!("{arm}\t{hits}\t{excl}\t{truth}");
    }
}
