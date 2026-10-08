use std::path::{Path, PathBuf};

const AGGREGATORS: [&str; 5] = [
    "dryad_catalog.φ",
    "pangaea_catalog.φ",
    "re3data_catalog.φ",
    "seanoe_catalog.φ",
    "zenodo_catalog.φ",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexEntry {
    source: String,
    root: String,
    schema: Option<String>,
    table: String,
    kind: Option<String>,
    extra: Option<String>,
}

fn parse_tap(source: &str, text: &str) -> Vec<IndexEntry> {
    let mut entries = Vec::new();
    let mut root = String::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("inventar ") {
            root = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("catalog ") {
            let mut fields = rest.split_whitespace();
            let (Some(table), Some(schema), Some(kind)) =
                (fields.next(), fields.next(), fields.next())
            else {
                continue;
            };
            entries.push(IndexEntry {
                source: source.to_string(),
                root: root.clone(),
                schema: Some(schema.to_string()),
                table: table.to_string(),
                kind: Some(kind.to_string()),
                extra: None,
            });
        }
    }
    entries
}

fn parse_aggregator(source: &str, text: &str) -> Vec<IndexEntry> {
    let mut entries = Vec::new();
    let root = source.to_string();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, extra) = match line.split_once('|') {
            Some((key, title)) => (key.trim(), Some(title.trim().to_string())),
            None => (line, None),
        };
        if key.is_empty() {
            continue;
        }
        entries.push(IndexEntry {
            source: source.to_string(),
            root: root.clone(),
            schema: None,
            table: key.to_string(),
            kind: None,
            extra,
        });
    }
    entries
}

fn is_tap_name(name: &str) -> bool {
    name.starts_with("tap_index_") && name.ends_with(".φ")
}

fn collect_entries(dir: &Path) -> Vec<IndexEntry> {
    let mut tap_paths: Vec<(String, PathBuf)> = Vec::new();
    if let Ok(read) = std::fs::read_dir(dir) {
        for entry in read.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if is_tap_name(name) {
                tap_paths.push((name.to_string(), path));
            }
        }
    }
    tap_paths.sort_by(|a, b| a.0.cmp(&b.0));

    let mut entries = Vec::new();
    for (name, path) in tap_paths {
        if let Ok(text) = std::fs::read_to_string(&path) {
            entries.extend(parse_tap(&name, &text));
        }
    }
    for name in AGGREGATORS {
        let path = dir.join(name);
        if let Ok(text) = std::fs::read_to_string(&path) {
            entries.extend(parse_aggregator(name, &text));
        }
    }
    entries
}

fn field(value: &Option<String>) -> &str {
    match value {
        Some(text) => text.as_str(),
        None => "",
    }
}

fn render_line(entry: &IndexEntry) -> String {
    let mut line = String::new();
    line.push_str(&entry.source);
    line.push('\t');
    line.push_str(&entry.root);
    line.push('\t');
    line.push_str(field(&entry.schema));
    line.push('\t');
    line.push_str(&entry.table);
    line.push('\t');
    line.push_str(field(&entry.kind));
    line.push('\t');
    line.push_str(field(&entry.extra));
    line
}

fn render(entries: &[IndexEntry]) -> String {
    let mut out = String::new();
    for entry in entries {
        out.push_str(&render_line(entry));
        out.push('\n');
    }
    out
}

fn print_help() {
    println!("tap_index_merge — static VO catalogue index (arm 1)");
    println!();
    println!("reads every phi/pipeline/catalog/tap_index_*.φ plus the five");
    println!("aggregator catalogues (dryad, pangaea, re3data, seanoe, zenodo)");
    println!("and writes one tab-separated index:");
    println!("  source\\troot\\tschema\\ttable\\ttype\\textra");
    println!();
    println!("options:");
    println!("  --out <path>    write the index to <path> (default stdout)");
    println!("  --query <text>  keep entries whose rendered line contains <text>");
    println!("  --count         print only the number of entries");
    println!("  --help          this text");
}

fn main() {
    let mut out_path: Option<String> = None;
    let mut query: Option<String> = None;
    let mut count = false;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => match args.next() {
                Some(value) => out_path = Some(value),
                None => {
                    eprintln!("--out needs a path");
                    std::process::exit(2);
                }
            },
            "--query" => match args.next() {
                Some(value) => query = Some(value),
                None => {
                    eprintln!("--query needs a substring");
                    std::process::exit(2);
                }
            },
            "--count" => count = true,
            "--help" | "-h" => {
                print_help();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                std::process::exit(2);
            }
        }
    }

    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("phi")
        .join("pipeline")
        .join("catalog");
    let entries = collect_entries(&dir);

    let selected: Vec<IndexEntry> = match &query {
        Some(needle) => {
            let lower = needle.to_lowercase();
            entries
                .into_iter()
                .filter(|entry| render_line(entry).to_lowercase().contains(&lower))
                .collect()
        }
        None => entries,
    };

    if count {
        println!("{}", selected.len());
        return;
    }

    let text = render(&selected);
    match out_path {
        Some(path) => {
            if let Err(err) = std::fs::write(&path, text) {
                eprintln!("write {path}: {err}");
                std::process::exit(1);
            }
        }
        None => print!("{text}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TAP_FIXTURE: &str =
        "inventar https://example.org/tap\ncatalog tbl_a scm table\ncatalog scm.tbl_b scm table\n";
    const AGG_FIXTURE: &str = "# fixture\nid1 | First title\nid2 | Second | with pipe\n";

    #[test]
    fn tap_fixture_yields_two_tables() {
        let entries = parse_tap("tap_index_fixture.φ", TAP_FIXTURE);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].root, "https://example.org/tap");
        assert_eq!(entries[0].schema.as_deref(), Some("scm"));
        assert_eq!(entries[0].table, "tbl_a");
        assert_eq!(entries[0].kind.as_deref(), Some("table"));
        assert!(entries[0].extra.is_none());
        assert_eq!(entries[1].table, "scm.tbl_b");
        assert_eq!(entries[1].root, "https://example.org/tap");
    }

    #[test]
    fn aggregator_fixture_yields_two_keys_and_keeps_pipe_title() {
        let entries = parse_aggregator("re3data_catalog.φ", AGG_FIXTURE);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].table, "id1");
        assert_eq!(entries[0].extra.as_deref(), Some("First title"));
        assert!(entries[0].schema.is_none());
        assert!(entries[0].kind.is_none());
        assert_eq!(entries[1].extra.as_deref(), Some("Second | with pipe"));
    }

    #[test]
    fn collect_entries_reads_two_fixture_files() {
        let dir = std::env::temp_dir().join(format!("tap_index_merge_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("create fixture dir");
        std::fs::write(dir.join("tap_index_fixture.φ"), TAP_FIXTURE).expect("write tap fixture");
        std::fs::write(dir.join("re3data_catalog.φ"), AGG_FIXTURE)
            .expect("write aggregator fixture");

        let entries = collect_entries(&dir);
        assert_eq!(entries.len(), 4);

        let rendered = render(&entries);
        assert_eq!(rendered.lines().count(), 4);

        std::fs::remove_dir_all(&dir).expect("remove fixture dir");
    }
}
