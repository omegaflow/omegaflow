use std::fs;

use omegaflow::archivar::parse::parse_sources;
use omegaflow::archivar::types::{Extract, TauSource};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn field_of(extract: &Extract) -> Option<&omegaflow::archivar::types::FieldConfig> {
    match extract {
        Extract::Field(fc)
        | Extract::First(fc, _)
        | Extract::Last(fc, _)
        | Extract::Count(fc)
        | Extract::LastRow(fc) => Some(fc),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let register = match arg_value(&args, "--register") {
        Some(v) => v,
        None => {
            println!("usage: source_tau --register <sources.φ>");
            return;
        }
    };
    let text = match fs::read_to_string(&register) {
        Ok(t) => t,
        Err(e) => {
            println!("source_tau: {register}: {e}");
            return;
        }
    };
    let sources = parse_sources(&text);
    let mut measured = 0usize;
    let mut pending = 0usize;
    for src in &sources {
        for extract in &src.extracts {
            let Some(fc) = field_of(extract) else {
                continue;
            };
            let state = match fc.tau_src {
                TauSource::Measured => {
                    measured += 1;
                    "measured"
                }
                TauSource::Derived => "derived",
                TauSource::Pending => {
                    pending += 1;
                    "pending"
                }
            };
            println!("{}\t{}\t{}\t{}", src.url, fc.key, fc.tau, state);
        }
    }
    eprintln!("source_tau: {measured} measured, {pending} pending");
}
