use std::env;
use std::fs;

use omegaflow::archivar::gpkg::{SqliteDb, SqliteValue};

const SESSION_COLUMNS: &[&str] = &[
    "id",
    "project_id",
    "workspace_id",
    "parent_id",
    "slug",
    "directory",
    "path",
    "title",
    "version",
    "share_url",
    "summary_additions",
    "summary_deletions",
    "summary_files",
    "summary_diffs",
    "metadata",
    "cost",
    "tokens_input",
    "tokens_output",
    "tokens_reasoning",
    "tokens_cache_read",
    "tokens_cache_write",
    "revert",
    "permission",
    "agent",
    "model",
    "time_created",
    "time_updated",
    "time_compacting",
    "time_archived",
];

struct Burn {
    agent: String,
    model: String,
    title: String,
    cost: f64,
    input: i64,
    output: i64,
    reasoning: i64,
    cache_read: i64,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut db_path = match default_db() {
        Some(p) => p,
        None => {
            eprintln!("session_burn: HOME absent — pass --db <path>");
            std::process::exit(2);
        }
    };
    let mut top = 10usize;
    let mut dir_filter: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--db" => {
                i += 1;
                if let Some(p) = args.get(i) {
                    db_path = p.clone();
                }
            }
            "--top" => {
                i += 1;
                if let Some(n) = args.get(i).and_then(|s| s.parse().ok()) {
                    top = n;
                }
            }
            "--dir" => {
                i += 1;
                if let Some(d) = args.get(i) {
                    dir_filter = Some(d.clone());
                }
            }
            _ => {}
        }
        i += 1;
    }
    let bytes = match fs::read(&db_path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("session_burn: db absent: {db_path}");
            std::process::exit(2);
        }
    };
    let Some(db) = SqliteDb::from_bytes(bytes) else {
        eprintln!("session_burn: {db_path} is not a SQLite database");
        std::process::exit(2);
    };
    let Some(rows) = db.read_table("session") else {
        eprintln!("session_burn: no session table");
        std::process::exit(2);
    };
    let mut burns: Vec<Burn> = Vec::new();
    for row in &rows {
        if row.len() != SESSION_COLUMNS.len() {
            eprintln!(
                "session_burn: session row has {} columns, the schema names {}",
                row.len(),
                SESSION_COLUMNS.len()
            );
            std::process::exit(2);
        }
        let dir = col(row, "directory");
        if let Some(f) = &dir_filter {
            if !dir.contains(f.as_str()) {
                continue;
            }
        }
        let (Some(cost), Some(input), Some(output), Some(reasoning), Some(cache_read)) = (
            num(row, "cost"),
            int(row, "tokens_input"),
            int(row, "tokens_output"),
            int(row, "tokens_reasoning"),
            int(row, "tokens_cache_read"),
        ) else {
            eprintln!("session_burn: a session row carries no numeric burn — skipped");
            continue;
        };
        burns.push(Burn {
            agent: col(row, "agent"),
            model: json_id(&col(row, "model")),
            title: col(row, "title"),
            cost,
            input,
            output,
            reasoning,
            cache_read,
        });
    }
    if burns.is_empty() {
        println!("session_burn: no sessions for dir filter {:?}", dir_filter);
        return;
    }
    let total: f64 = burns.iter().map(|b| b.cost).sum();
    println!(
        "session_burn | {} sessions | total ${:.4}",
        burns.len(),
        total
    );
    print_by(&burns, "agent", top);
    print_by(&burns, "model", top);
    println!("\ntop sessions by cost:");
    let mut sorted = burns;
    sorted.sort_by(|a, b| b.cost.total_cmp(&a.cost));
    for b in sorted.iter().take(top) {
        println!(
            "  ${:>8.4}  {:>14}  {:>34}  {}",
            b.cost,
            b.agent,
            b.model,
            short(&b.title, 46)
        );
    }
}

fn print_by(burns: &[Burn], key: &str, top: usize) {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, (usize, f64, i64, i64, i64, i64)> = BTreeMap::new();
    for b in burns {
        let k = if key == "agent" {
            b.agent.clone()
        } else {
            b.model.clone()
        };
        let e = map.entry(k).or_insert((0, 0.0, 0, 0, 0, 0));
        e.0 += 1;
        e.1 += b.cost;
        e.2 += b.input;
        e.3 += b.output;
        e.4 += b.reasoning;
        e.5 += b.cache_read;
    }
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|a, b| b.1 .1.total_cmp(&a.1 .1));
    println!("\nby {key}:");
    println!(
        "  {:>16}  {:>4}  {:>10}  {:>8}  {:>9}  {:>11}  {:>9}",
        key, "n", "input", "output", "reasoning", "cache_read", "cost"
    );
    for (k, (n, cost, input, output, reason, cache)) in v.iter().take(top) {
        println!(
            "  {:>16}  {:>4}  {:>10}  {:>8}  {:>9}  {:>11}  ${:>8.4}",
            short(k, 16),
            n,
            input,
            output,
            reason,
            cache,
            cost
        );
    }
}

fn col(row: &[SqliteValue], name: &str) -> String {
    let Some(i) = SESSION_COLUMNS.iter().position(|c| *c == name) else {
        return String::new();
    };
    match &row[i] {
        SqliteValue::Text(t) => t.clone(),
        SqliteValue::Int(n) => n.to_string(),
        SqliteValue::Real(f) => f.to_string(),
        _ => String::new(),
    }
}

fn num(row: &[SqliteValue], name: &str) -> Option<f64> {
    col(row, name).parse().ok()
}

fn int(row: &[SqliteValue], name: &str) -> Option<i64> {
    col(row, name).parse().ok()
}

fn json_id(s: &str) -> String {
    let Some(k) = s.find("\"id\"") else {
        return s.to_string();
    };
    let rest = &s[k + 4..];
    let Some(colon) = rest.find(':') else {
        return s.to_string();
    };
    let rest = &rest[colon + 1..];
    let Some(q1) = rest.find('"') else {
        return s.to_string();
    };
    let rest = &rest[q1 + 1..];
    match rest.find('"') {
        Some(q2) => rest[..q2].to_string(),
        None => s.to_string(),
    }
}

fn short(s: &str, n: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= n {
        return s.to_string();
    }
    chars[..n.saturating_sub(1)].iter().collect::<String>() + "…"
}

fn default_db() -> Option<String> {
    let home = env::var("HOME").ok()?;
    Some(format!("{home}/.local/share/opencode/opencode.db"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_id_extracts_model() {
        assert_eq!(
            json_id("{\"id\":\"deepseek-v4-flash\",\"providerID\":\"deepseek\"}"),
            "deepseek-v4-flash"
        );
        assert_eq!(json_id("plain"), "plain");
    }

    #[test]
    fn short_truncates_on_char_boundary() {
        assert_eq!(short("abcdef", 4), "abc…");
        assert_eq!(short("ab", 4), "ab");
    }
}
