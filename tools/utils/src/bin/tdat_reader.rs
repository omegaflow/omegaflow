use omegaflow::inflate::gunzip;
use omegaflow::tdat::{TdatColumn, parse_tdat};
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: tdat_reader <file.tdat[.gz]>");
        eprintln!("       tdat_reader --url <url>");
        eprintln!("       tdat_reader <file> --head <n>");
        return;
    }

    let bytes: Vec<u8> = if args[0] == "--url" {
        let url = args.get(1).map(|s| s.as_str()).unwrap_or("");
        match fetch(url) {
            Some(b) => b,
            None => {
                println!("curl fetch without answer: {}", url);
                return;
            }
        }
    } else {
        match std::fs::read(&args[0]) {
            Ok(b) => b,
            Err(_) => {
                println!("file does not open: {}", args[0]);
                return;
            }
        }
    };

    let bytes = if bytes.starts_with(&[0x1f, 0x8b]) {
        match gunzip(&bytes) {
            Some(b) => b,
            None => {
                println!("gzip stream stays unreadable");
                return;
            }
        }
    } else {
        bytes
    };

    let table = match parse_tdat(&bytes) {
        Some(t) => t,
        None => {
            println!(
                "parse void — {} B carry no <HEADER>/<DATA> table (0 honored)",
                bytes.len()
            );
            return;
        }
    };

    struktur(&table);

    match arg_value(&args, "--head") {
        Some(n) => {
            let n = match n.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    println!("--head needs a whole count, has: {}", n);
                    return;
                }
            };
            dump_rows(&table, n);
        }
        None => {}
    }
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl").arg("-sSf").arg(url).output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn struktur(table: &omegaflow::tdat::TdatTable) {
    match &table.table_name {
        Some(n) => println!("table_name: {}", n),
        None => println!("table_name: absent"),
    }
    match &table.description {
        Some(d) => println!("description: {}", d),
        None => println!("description: absent"),
    }
    println!("columns: {}", table.columns.len());
    for c in &table.columns {
        println!("  {}", column_text(c));
    }
    println!(
        "row_order ({}): {}",
        table.row_order.len(),
        table.row_order.join(" ")
    );
    println!("rows: {}", table.rows.len());
}

fn column_text(c: &TdatColumn) -> String {
    let ucd = match &c.ucd {
        Some(u) => format!(" [{}]", u),
        None => String::new(),
    };
    let index = if c.indexed { " (index)" } else { "" };
    let desc = match &c.description {
        Some(d) => format!(" // {}", d),
        None => String::new(),
    };
    format!("{} = {}{}{}{}", c.name, c.ttype, ucd, index, desc)
}

fn dump_rows(table: &omegaflow::tdat::TdatTable, n: usize) {
    let bound = n.min(table.rows.len());
    for i in 0..bound {
        let mut cells = Vec::new();
        for name in &table.row_order {
            let cell = match table.cell(i, name) {
                Some(Some(v)) => v.to_string(),
                _ => "-".to_string(),
            };
            cells.push(format!("{}={}", name, cell));
        }
        println!("  [{}] {}", i, cells.join(" "));
    }
}
