use std::env;
use std::process::exit;

use omegaflow_measure::eeglab::{channel_series, open_set_bin, resolve_channel};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut positional: Vec<String> = Vec::new();
    let mut channel_sel = "1".to_string();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--channel" {
            match args.get(i + 1) {
                Some(v) => channel_sel = v.clone(),
                None => {
                    eprintln!("openneuro_channel_dump: --channel needs a selector (n|label)");
                    exit(2);
                }
            }
            i += 2;
        } else {
            positional.push(args[i].clone());
            i += 1;
        }
    }

    let (Some(in_path), Some(out_path)) = (positional.first(), positional.get(1)) else {
        eprintln!("openneuro_channel_dump <in.bin> <out.txt> [--channel <n|label>]");
        exit(2);
    };

    let Some((set, samples)) = open_set_bin(in_path) else {
        eprintln!("openneuro_channel_dump: {in_path} reads absent (not an EEGB bin)");
        exit(2);
    };
    let Some(ch) = resolve_channel(&set, &channel_sel) else {
        eprintln!(
            "openneuro_channel_dump: channel [{channel_sel}] absent among {} channels",
            set.nbchan
        );
        exit(2);
    };
    let Some(series) = channel_series(&samples, &set, ch) else {
        eprintln!("openneuro_channel_dump: channel {ch} does not resolve against the sample block");
        exit(2);
    };

    let label: &str = match set.labels.get(ch) {
        Some(l) if !l.is_empty() => l.as_str(),
        _ => "absent",
    };
    let mut out = String::new();
    for v in &series {
        if !v.is_finite() {
            eprintln!("openneuro_channel_dump: sample [{v}] is not finite — absent, not written");
            exit(2);
        }
        out.push_str(&format!("{v}\n"));
    }
    if let Err(e) = std::fs::write(out_path, &out) {
        eprintln!("openneuro_channel_dump: {out_path} unreadable: {e}");
        exit(2);
    }

    println!(
        "openneuro_channel_dump: {in_path} -> {out_path} | channel {ch} ({label}) | \
         nbchan = {} | pnts = {} | trials = {} | n = {}",
        set.nbchan,
        set.pnts,
        set.trials,
        series.len()
    );
}
