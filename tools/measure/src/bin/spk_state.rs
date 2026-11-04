use std::fs;

use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::archivar::parse_iso_unix;
use omegaflow::bsp_reader::spk::SpkFile;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_ids(args: &[String], name: &str) -> Vec<i32> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < args.len() {
        if args[i] == name
            && let Ok(v) = args[i + 1].parse::<i32>()
        {
            out.push(v);
            i += 1;
        }
        i += 1;
    }
    out
}

fn times(args: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < args.len() {
        if args[i] == "--utc" {
            out.push(args[i + 1].clone());
            i += 1;
        }
        i += 1;
    }
    if let Some(path) = arg_value(args, "--from-file")
        && let Ok(text) = fs::read_to_string(&path)
    {
        for line in text.lines() {
            let t = line.trim();
            if !t.is_empty() && !t.starts_with('#') {
                out.push(t.to_string());
            }
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let kernel = match arg_value(&args, "--kernel") {
        Some(v) => v,
        None => {
            println!(
                "usage: spk_state --kernel <file.bsp> --target <id> --center <id> --utc <iso> [--against <id>] [--from-file <path>]"
            );
            return;
        }
    };
    let targets = arg_ids(&args, "--target");
    let centers = arg_ids(&args, "--center");
    let against = arg_ids(&args, "--against");
    let target = match targets.first() {
        Some(v) => *v,
        None => {
            println!("spk_state: --target missing");
            return;
        }
    };
    let center = match centers.first() {
        Some(v) => *v,
        None => {
            println!("spk_state: --center missing");
            return;
        }
    };
    let spac = match SpkFile::open(&kernel) {
        Ok(s) => s,
        Err(e) => {
            println!("spk_state: {kernel}: {e}");
            return;
        }
    };
    if args.iter().any(|a| a == "--segments") {
        for seg in spac.segments() {
            println!(
                "segment target {} center {} type {} et {:.3}..{:.3} {}",
                seg.target, seg.center, seg.data_type, seg.start_et, seg.end_et, seg.name
            );
        }
        return;
    }
    let lsk = match embedded_lsk() {
        Some(v) => v,
        None => {
            println!("spk_state: the embedded leap second table is absent");
            return;
        }
    };
    let stamps = times(&args);
    if stamps.is_empty() {
        println!("spk_state: no times given");
        return;
    }
    for stamp in stamps {
        let unix = match parse_iso_unix(&stamp) {
            Some(v) => v,
            None => {
                println!("spk_state: {stamp} unreadable");
                continue;
            }
        };
        let tdb = match lsk.unix_to_tdb(unix) {
            Some(v) => v,
            None => {
                println!("spk_state: {stamp} outside the leap second table");
                continue;
            }
        };
        let et = tdb;
        let state = match spac.state(target, center, et) {
            Ok(s) => s,
            Err(e) => {
                println!("spk_state: {stamp} state: {e}");
                continue;
            }
        };
        let r = (state[0] * state[0] + state[1] * state[1] + state[2] * state[2]).sqrt();
        let v = (state[3] * state[3] + state[4] * state[4] + state[5] * state[5]).sqrt();
        let mut line = format!("{stamp}  et {:.3}  r {:.1} km  v {:.3} km/s", et, r, v);
        if let Some(other) = against.first()
            && let Ok(o) = spac.state(*other, center, et)
        {
            let d = [state[0] - o[0], state[1] - o[1], state[2] - o[2]];
            let distance = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            line.push_str(&format!("  to {other}: {distance:.1} km"));
        }
        println!("{line}");
    }
}
