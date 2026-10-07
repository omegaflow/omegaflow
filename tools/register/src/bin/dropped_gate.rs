use std::collections::BTreeSet;

const DEFAULT_PIN: &str = "docs/zustand/dropped-roster-baseline.txt";
const HEADER_PREFIX: &str = "# dropped-events v1 pin=";

#[derive(Debug)]
enum Event {
    Minted,
    Carried,
    Resolved,
    Dropped,
}

struct Outcome {
    keys: BTreeSet<String>,
    minted: usize,
    carried: usize,
    resolved: usize,
    dropped: usize,
    dropped_keys: Vec<String>,
    refusal: Option<String>,
}

fn refusal(msg: impl Into<String>) -> Outcome {
    Outcome {
        keys: BTreeSet::new(),
        minted: 0,
        carried: 0,
        resolved: 0,
        dropped: 0,
        dropped_keys: Vec::new(),
        refusal: Some(msg.into()),
    }
}

fn parse_event(word: &str) -> Result<Event, String> {
    match word {
        "minted" => Ok(Event::Minted),
        "carried" => Ok(Event::Carried),
        "resolved" => Ok(Event::Resolved),
        "dropped" => Ok(Event::Dropped),
        other => Err(format!("unknown event word `{other}`")),
    }
}

fn is_proof(witness: &str) -> bool {
    match witness.strip_prefix("commit:") {
        Some(rest) => !rest.is_empty(),
        None => match witness.strip_prefix("artifact:") {
            Some(rest) => !rest.is_empty(),
            None => false,
        },
    }
}

fn fold_events(log: &str, pin: &BTreeSet<String>) -> Outcome {
    let mut keys = pin.clone();
    let mut minted = 0usize;
    let mut carried = 0usize;
    let mut resolved = 0usize;
    let mut dropped = 0usize;
    let mut dropped_keys: Vec<String> = Vec::new();

    let mut lines = log.lines();
    let header = match lines.next() {
        Some(h) => h,
        None => return refusal("empty event log: missing header"),
    };
    if !header.starts_with(HEADER_PREFIX) {
        return refusal(format!("bad header `{header}`"));
    }

    for (idx, line) in lines.enumerate() {
        let lineno = idx + 2;
        if line.is_empty() {
            return refusal(format!("line {lineno}: empty line"));
        }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 4 {
            return refusal(format!(
                "line {lineno}: expected 4 tab-separated fields, found {}",
                parts.len()
            ));
        }
        let event = match parse_event(parts[0]) {
            Ok(e) => e,
            Err(e) => return refusal(format!("line {lineno}: {e}")),
        };
        let key = parts[1];
        let witness = parts[2];
        let date = parts[3];
        if key.is_empty() {
            return refusal(format!("line {lineno}: empty key"));
        }
        if date.is_empty() {
            return refusal(format!("line {lineno}: empty date"));
        }

        match event {
            Event::Minted => {
                if keys.contains(key) {
                    return refusal(format!("line {lineno}: minted key `{key}` already present"));
                }
                keys.insert(key.to_string());
                minted += 1;
            }
            Event::Carried => {
                if !keys.contains(key) {
                    return refusal(format!("line {lineno}: carried key `{key}` not present"));
                }
                carried += 1;
            }
            Event::Resolved => {
                if !keys.contains(key) {
                    return refusal(format!("line {lineno}: resolved key `{key}` not present"));
                }
                if !is_proof(witness) {
                    return refusal(format!(
                        "line {lineno}: resolved key `{key}` lacks proof token (commit:<sha>|artifact:<path>), witness `{witness}`"
                    ));
                }
                keys.remove(key);
                resolved += 1;
            }
            Event::Dropped => {
                if !keys.contains(key) {
                    return refusal(format!("line {lineno}: dropped key `{key}` not present"));
                }
                if witness.is_empty() {
                    return refusal(format!(
                        "line {lineno}: dropped key `{key}` lacks the second witness"
                    ));
                }
                keys.remove(key);
                dropped_keys.push(key.to_string());
                dropped += 1;
            }
        }
    }

    Outcome {
        keys,
        minted,
        carried,
        resolved,
        dropped,
        dropped_keys,
        refusal: None,
    }
}

fn read_pin(path: &str) -> Result<BTreeSet<String>, String> {
    let text =
        std::fs::read_to_string(path).map_err(|e| format!("pin `{path}` unreadable: {e}"))?;
    let mut set = BTreeSet::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        set.insert(line.to_string());
    }
    Ok(set)
}

fn carried_log<'a, I: Iterator<Item = &'a String>>(keys: I, alias: usize) -> String {
    let mut log = String::from(HEADER_PREFIX);
    log.push_str("shadow\n");
    for (idx, key) in keys.enumerate() {
        let witness = if idx < alias { "alias:shadow-old" } else { "" };
        log.push_str("carried\t");
        log.push_str(key);
        log.push('\t');
        log.push_str(witness);
        log.push_str("\t2026-10-07\n");
    }
    log
}

fn compare_log_to_pin(log: &str, pin: &BTreeSet<String>) -> (usize, usize) {
    let outcome = fold_events(log, pin);
    if outcome.refusal.is_some() {
        return (1, 1);
    }
    let mut mentioned: BTreeSet<String> = BTreeSet::new();
    let mut minted: BTreeSet<String> = BTreeSet::new();
    for line in log.lines().skip(1) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 4 {
            continue;
        }
        mentioned.insert(parts[1].to_string());
        if parts[0] == "minted" {
            minted.insert(parts[1].to_string());
        }
    }
    let false_green = pin.iter().filter(|k| !mentioned.contains(*k)).count();
    let false_red = outcome
        .keys
        .iter()
        .filter(|k| !pin.contains(*k) && !minted.contains(*k))
        .count();
    (false_red, false_green)
}

fn shadow_null_control(pin: &BTreeSet<String>) -> (usize, usize) {
    let k = pin.len().min(3);
    let mut false_red = 0usize;
    let mut false_green = 0usize;

    let all_carried = carried_log(pin.iter(), 0);
    let (r, g) = compare_log_to_pin(&all_carried, pin);
    false_red += r;
    false_green += g;

    let reduced = carried_log(pin.iter().skip(k), 0);
    let (r, g) = compare_log_to_pin(&reduced, pin);
    false_red += r;
    false_green += g.abs_diff(k);

    let renamed = carried_log(pin.iter(), k);
    let (r, g) = compare_log_to_pin(&renamed, pin);
    false_red += r;
    false_green += g;

    (false_red, false_green)
}

fn run_selftest() -> Result<(), String> {
    let pin: BTreeSet<String> = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();
    let log = concat!(
        "# dropped-events v1 pin=deadbeef\n",
        "minted\tnew\t\t2026-10-07\n",
        "carried\ta\talias:old\t2026-10-07\n",
        "resolved\tb\tcommit:abc123\t2026-10-07\n",
        "dropped\tc\treason text\t2026-10-07\n",
    );
    let o = fold_events(log, &pin);
    if let Some(r) = &o.refusal {
        return Err(r.clone());
    }
    if !o.keys.contains("new") {
        return Err("minted did not add `new`".to_string());
    }
    if !o.keys.contains("a") {
        return Err("carried did not keep `a`".to_string());
    }
    if o.keys.contains("b") {
        return Err("resolved did not remove `b`".to_string());
    }
    if o.keys.contains("c") {
        return Err("dropped did not remove `c`".to_string());
    }
    if o.dropped_keys != vec!["c".to_string()] {
        return Err("dropped key not recorded".to_string());
    }
    let proof_less = "# dropped-events v1 pin=x\nresolved\tb\t\tnope\n";
    if fold_events(proof_less, &pin).refusal.is_none() {
        return Err("proof-less resolved not refused".to_string());
    }
    let absent_carried = "# dropped-events v1 pin=x\ncarried\tz\t\t2026-10-07\n";
    if fold_events(absent_carried, &pin).refusal.is_none() {
        return Err("carried of absent target not refused".to_string());
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut log_path: Option<String> = None;
    let mut pin_path = DEFAULT_PIN.to_string();
    let mut selftest = false;
    let mut shadow = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--log" => {
                i += 1;
                match args.get(i) {
                    Some(v) => log_path = Some(v.clone()),
                    None => {
                        eprintln!("dropped_gate: --log needs a path");
                        std::process::exit(1);
                    }
                }
            }
            "--pin" => {
                i += 1;
                match args.get(i) {
                    Some(v) => pin_path = v.clone(),
                    None => {
                        eprintln!("dropped_gate: --pin needs a path");
                        std::process::exit(1);
                    }
                }
            }
            "--selftest" => selftest = true,
            "--shadow" => shadow = true,
            other => {
                eprintln!("dropped_gate: unknown argument `{other}`");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if selftest {
        match run_selftest() {
            Ok(()) => println!("dropped_gate: selftest passes"),
            Err(e) => {
                eprintln!("dropped_gate: selftest refusal: {e}");
                std::process::exit(1);
            }
        }
        return;
    }

    if shadow {
        let pin = match read_pin(&pin_path) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("dropped_gate: {e}");
                std::process::exit(1);
            }
        };
        let (false_red, false_green) = shadow_null_control(&pin);
        let sharp = false_red == 0 && false_green == 0;
        println!(
            "dropped_gate shadow: false_red={false_red} false_green={false_green} sharp={sharp}"
        );
        std::process::exit(if sharp { 0 } else { 1 });
    }

    let log_path = match log_path {
        Some(p) => p,
        None => {
            eprintln!("dropped_gate: --log <path> is required");
            std::process::exit(1);
        }
    };
    let log = match std::fs::read_to_string(&log_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dropped_gate: log `{log_path}` unreadable: {e}");
            std::process::exit(1);
        }
    };
    let pin = match read_pin(&pin_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("dropped_gate: {e}");
            std::process::exit(1);
        }
    };

    let outcome = fold_events(&log, &pin);
    if let Some(r) = &outcome.refusal {
        eprintln!("dropped_gate: refusal: {r}");
        std::process::exit(1);
    }

    println!(
        "dropped_gate: minted={} carried={} resolved={} dropped={}",
        outcome.minted, outcome.carried, outcome.resolved, outcome.dropped
    );
    for k in &outcome.dropped_keys {
        println!("dropped: {k}");
    }
    if outcome.dropped > 0 {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold_applies_the_four_events() {
        let pin: BTreeSet<String> = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();
        let log = concat!(
            "# dropped-events v1 pin=deadbeef\n",
            "minted\tnew\t\t2026-10-07\n",
            "carried\ta\talias:old\t2026-10-07\n",
            "resolved\tb\tcommit:abc123\t2026-10-07\n",
            "dropped\tc\treason text\t2026-10-07\n",
        );
        let outcome = fold_events(log, &pin);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        assert!(outcome.keys.contains("new"));
        assert!(outcome.keys.contains("a"));
        assert!(!outcome.keys.contains("b"));
        assert!(!outcome.keys.contains("c"));
        assert_eq!(outcome.minted, 1);
        assert_eq!(outcome.carried, 1);
        assert_eq!(outcome.resolved, 1);
        assert_eq!(outcome.dropped, 1);
        assert_eq!(outcome.dropped_keys, vec!["c".to_string()]);

        let proof_less = "# dropped-events v1 pin=x\nresolved\tb\t\tnope\n";
        assert!(fold_events(proof_less, &pin).refusal.is_some());

        let absent_carried = "# dropped-events v1 pin=x\ncarried\tz\t\t2026-10-07\n";
        assert!(fold_events(absent_carried, &pin).refusal.is_some());
    }

    #[test]
    fn shadow_control_is_sharp_on_synthetic_pin() {
        let pin: BTreeSet<String> = ["a", "b", "c", "d"].iter().map(|s| s.to_string()).collect();
        let k = pin.len().min(3);

        let all_carried = carried_log(pin.iter(), 0);
        assert_eq!(compare_log_to_pin(&all_carried, &pin), (0, 0));

        let reduced = carried_log(pin.iter().skip(k), 0);
        let (false_red, false_green) = compare_log_to_pin(&reduced, &pin);
        assert_eq!(false_red, 0);
        assert_eq!(false_green, k);

        let renamed = carried_log(pin.iter(), k);
        assert_eq!(compare_log_to_pin(&renamed, &pin), (0, 0));

        assert_eq!(shadow_null_control(&pin), (0, 0));
    }
}
