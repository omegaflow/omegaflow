use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;

use vo_tap::{
    census, census_with, gewogen_note, host_of, known_hosts_and_urls, order_fruchtfolge,
    query_sync, read_kandidat, regtap_count, regtap_services, rewrite_ledger_notes, submit_async,
    tables, today_ymd, Format,
};

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
        eprintln!("usage: vo-tap <sync|async|tables|census|wave|import> <root> [adql] [--format csv|json|text|votable|votable/td] [--poll N] [--regtap <root>] [--ledger <path>]");
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
        "census" => {
            let urls: Vec<String> = args.iter().skip(1).cloned().collect();
            if urls.is_empty() {
                eprintln!("census needs <url> [<url> ...]");
                std::process::exit(1);
            }
            println!("url\thttp_code\ttime_s\tfinal_url\tprobe\tdate");
            for u in &urls {
                let l = census(u);
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{}",
                    l.url, l.http_code, l.time_s, l.final_url, l.probe, l.date
                );
            }
        }
        "wave" => {
            let Some(ledger_path) = arg(&args, "--ledger") else {
                eprintln!("wave needs --ledger <path>");
                std::process::exit(1);
            };
            let probe_timeout: u64 = arg(&args, "--probe-timeout")
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);
            let pause_s: u64 = arg(&args, "--pause")
                .and_then(|s| s.parse().ok())
                .unwrap_or(2);
            let entries: Vec<(String, String)> = read_kandidat(&ledger_path)
                .into_iter()
                .filter(|(_, n)| n.contains("ungewogen"))
                .collect();
            let ordered = order_fruchtfolge(entries);
            println!("url\thttp_code\ttime_s\tfinal_url\tprobe\tdate");
            let mut weighed: HashMap<String, String> = HashMap::new();
            let total = ordered.len();
            for (i, (url, note)) in ordered.iter().enumerate() {
                let l = census_with(url, probe_timeout);
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{}",
                    l.url, l.http_code, l.time_s, l.final_url, l.probe, l.date
                );
                weighed.insert(url.clone(), gewogen_note(note, &l));
                if i + 1 < total {
                    std::thread::sleep(std::time::Duration::from_secs(pause_s));
                }
            }
            match rewrite_ledger_notes(&ledger_path, &weighed) {
                Ok(n) => eprintln!("wave: {} weighed, {} ledger notes rewritten", total, n),
                Err(_) => eprintln!("wave: {} weighed, ledger rewrite returned void", total),
            }
        }
        "import" => {
            let Some(root) = arg(&args, "--regtap") else {
                eprintln!("import needs --regtap <root>");
                std::process::exit(1);
            };
            let ledger_path = arg(&args, "--ledger");
            let mut paths = vec![
                "phi/sources.φ".to_string(),
                "phi/dead_sources.φ".to_string(),
                "phi/blocked_sources.φ".to_string(),
                "phi/witnesses.φ".to_string(),
                "phi/footprints.φ".to_string(),
                "phi/pipeline/ledger.φ".to_string(),
            ];
            if let Some(p) = &ledger_path {
                if !paths.iter().any(|x| x == p) {
                    paths.push(p.clone());
                }
            }
            let (hosts, urls) = known_hosts_and_urls(&paths);
            let (count, rows) = (regtap_count(&root), regtap_services(&root));
            match (count, rows) {
                (Some(n), Some(rows)) => {
                    eprintln!("regtap: {} rows, COUNT(*) = {}", rows.len(), n);
                    let mut block = String::new();
                    let mut emitted = 0usize;
                    let mut artifacts = 0usize;
                    let mut duplicates = 0usize;
                    let mut seen: HashSet<String> = HashSet::new();
                    for (ivoid, url) in &rows {
                        if urls.contains(url) {
                            continue;
                        }
                        let Some(h) = host_of(url) else {
                            artifacts += 1;
                            continue;
                        };
                        if hosts.contains(&h) {
                            continue;
                        }
                        if !seen.insert(url.clone()) {
                            duplicates += 1;
                            continue;
                        }
                        let note = if ivoid.is_empty() {
                            format!("RegTAP-entdeckt, ungewogen ({})", today_ymd())
                        } else {
                            format!("{} — RegTAP-entdeckt, ungewogen ({})", ivoid, today_ymd())
                        };
                        block.push_str(&format!("ausstehend\nkandidat {}\nnote {}\n\n", url, note));
                        emitted += 1;
                    }
                    print!("{}", block);
                    eprintln!(
                        "regtap: {} candidates after Bestand-Dedupe ({} relative access_url, {} batch duplicates skipped)",
                        emitted, artifacts, duplicates
                    );
                    if let Some(p) = ledger_path {
                        let needs_sep = fs::read_to_string(&p)
                            .map(|c| !c.is_empty() && !c.ends_with("\n\n"))
                            .unwrap_or(false);
                        match OpenOptions::new().create(true).append(true).open(&p) {
                            Ok(mut f) => {
                                let sep = if needs_sep { "\n" } else { "" };
                                if f.write_all(sep.as_bytes()).is_ok()
                                    && f.write_all(block.as_bytes()).is_ok()
                                {
                                    eprintln!("ledger: appended to {}", p);
                                } else {
                                    eprintln!("ledger: write to {} returned void", p);
                                }
                            }
                            Err(_) => eprintln!("ledger: {} not writable", p),
                        }
                    }
                }
                _ => {
                    eprintln!("regtap returned void");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: vo-tap <sync|async|tables|census|wave|import> <root> [adql] [--format csv|json|text|votable|votable/td] [--poll N] [--regtap <root>] [--ledger <path>]");
            std::process::exit(1);
        }
    }
}
