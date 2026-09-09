use std::env;

use vo_tap::{query_sync, submit_async, tables, Format};

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn format_of(args: &[String]) -> Format {
    match arg(args, "--format").and_then(|f| Format::parse(&f)) {
        Some(f) => f,
        None => Format::Csv,
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(cmd) = args.first().cloned() else {
        eprintln!("usage: vo-tap <sync|async|tables> <root> [adql] [--format csv|json|text|votable|votable/td] [--poll N]");
        std::process::exit(1);
    };
    match cmd.as_str() {
        "tables" => {
            let Some(root) = arg(&args, "--root").or_else(|| args.get(1).cloned()) else {
                eprintln!("--root absent");
                std::process::exit(1);
            };
            match tables(&root) {
                Some(ts) => {
                    for (name, typ, schema) in ts {
                        let t = match typ {
                            Some(v) => v,
                            None => String::from("absent"),
                        };
                        let s = match schema {
                            Some(v) => v,
                            None => String::from("absent"),
                        };
                        println!("{s}\t{name}\t{t}");
                    }
                }
                None => {
                    eprintln!("tables returned void");
                    std::process::exit(1);
                }
            }
        }
        "sync" => {
            let Some(root) = args.get(1).cloned() else {
                eprintln!("sync needs <root> <adql>");
                std::process::exit(1);
            };
            let Some(adql) = args.get(2).cloned() else {
                eprintln!("sync needs <root> <adql>");
                std::process::exit(1);
            };
            match query_sync(&root, &adql, format_of(&args)) {
                Some(body) => print!("{}", body),
                None => {
                    eprintln!("sync returned void");
                    std::process::exit(1);
                }
            }
        }
        "async" => {
            let Some(root) = args.get(1).cloned() else {
                eprintln!("async needs <root> <adql>");
                std::process::exit(1);
            };
            let Some(adql) = args.get(2).cloned() else {
                eprintln!("async needs <root> <adql>");
                std::process::exit(1);
            };
            let poll: u64 = match arg(&args, "--poll").and_then(|s| s.parse().ok()) {
                Some(v) => v,
                None => 10,
            };
            let timeout: u64 = match arg(&args, "--timeout").and_then(|s| s.parse().ok()) {
                Some(v) => v,
                None => 600,
            };
            let job = match submit_async(&root, &adql, format_of(&args)) {
                Some(j) => j,
                None => {
                    eprintln!("async returned void");
                    std::process::exit(1);
                }
            };
            eprintln!("uws job: {}", job.url);
            match job.wait(timeout, poll) {
                Some(phase) if phase == "COMPLETED" => match job.result() {
                    Some(body) => print!("{}", body),
                    None => {
                        eprintln!("result returned void");
                        std::process::exit(1);
                    }
                },
                Some(phase) => {
                    eprintln!("uws phase: {} — the query stays unharvested", phase);
                    std::process::exit(1);
                }
                None => {
                    eprintln!("uws poll returned void");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: vo-tap <sync|async|tables> <root> [adql] [--format csv|json|text|votable|votable/td] [--poll N]");
            std::process::exit(1);
        }
    }
}
