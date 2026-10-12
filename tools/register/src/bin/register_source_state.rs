use std::env;
use std::fs;

const DEFAULT_REGISTER: &str = "phi/sources.φ";

fn state_of(text: &str, netloc: &str) -> &'static str {
    let mut live = false;
    let mut declined = false;
    for line in text.lines() {
        if let Some(rest) = line
            .strip_prefix("url ")
            .or_else(|| line.strip_prefix("origin "))
        {
            if rest.contains(netloc) {
                live = true;
            }
        } else if let Some(rest) = line.strip_prefix("decline ") {
            if rest.contains(netloc) {
                declined = true;
            }
        }
    }
    if live {
        "live"
    } else if declined {
        "declined"
    } else {
        "absent"
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut netloc: Option<String> = None;
    let mut path = DEFAULT_REGISTER.to_string();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--path" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("register_source_state: --path needs a value");
                    std::process::exit(2);
                }
                path = args[i].clone();
            }
            other if other.starts_with('-') => {
                eprintln!("register_source_state: unknown flag '{other}'");
                std::process::exit(2);
            }
            other => {
                if netloc.is_none() {
                    netloc = Some(other.to_string());
                } else {
                    eprintln!("register_source_state: multiple netlocs");
                    std::process::exit(2);
                }
            }
        }
        i += 1;
    }
    let netloc = match netloc {
        Some(n) => n,
        None => {
            eprintln!("usage: register_source_state <netloc> [--path <register>]");
            eprintln!(
                "  prints live | declined | absent — the admission guard for a *-cdn workflow"
            );
            std::process::exit(2);
        }
    };
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("register_source_state: read {path}: {e}");
            std::process::exit(2);
        }
    };
    println!("{}", state_of(&text, &netloc));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
url https://example.org/live.bin
origin https://example.org/src
decline commercial https://gone.example.net/x
";

    #[test]
    fn live_wins_over_declined_and_absent_is_empty() {
        assert_eq!(state_of(SAMPLE, "example.org"), "live");
        assert_eq!(state_of(SAMPLE, "gone.example.net"), "declined");
        assert_eq!(state_of(SAMPLE, "never.example.com"), "absent");
    }
}
