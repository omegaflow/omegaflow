use std::collections::BTreeMap;
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
    generations: BTreeMap<String, u32>,
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
        generations: BTreeMap::new(),
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

fn is_revive(witness: &str) -> bool {
    match witness.strip_prefix("revive") {
        Some(rest) => rest.is_empty() || rest.starts_with(':'),
        None => false,
    }
}

fn fold_events(log: &str, pin: &BTreeSet<String>) -> Outcome {
    let mut keys = pin.clone();
    let mut minted = 0usize;
    let mut carried = 0usize;
    let mut resolved = 0usize;
    let mut dropped = 0usize;
    let mut dropped_keys: Vec<String> = Vec::new();
    let mut generations: BTreeMap<String, u32> = BTreeMap::new();
    let mut tombstones: BTreeSet<String> = BTreeSet::new();

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
                if tombstones.contains(key) {
                    if !is_revive(witness) {
                        return refusal(format!(
                            "line {lineno}: minted key `{key}` carries a tombstone and lacks a revive witness (revival hole)"
                        ));
                    }
                    tombstones.remove(key);
                    *generations.entry(key.to_string()).or_insert(0) += 1;
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
                tombstones.insert(key.to_string());
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
        generations,
        refusal: None,
    }
}

fn read_keys(path: &str, what: &str) -> Result<BTreeSet<String>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("{what} `{path}` unreadable: {e}"))?;
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

fn compare_roster(
    roster: &BTreeSet<String>,
    pin: &BTreeSet<String>,
    log: &str,
) -> Result<(usize, usize), String> {
    let mut minted: BTreeSet<String> = BTreeSet::new();
    let mut removed: BTreeSet<String> = BTreeSet::new();
    let mut alias_new_to_old: BTreeMap<String, String> = BTreeMap::new();

    let mut lines = log.lines();
    let header = match lines.next() {
        Some(h) => h,
        None => return Err("empty event log: missing header".to_string()),
    };
    if !header.starts_with(HEADER_PREFIX) {
        return Err(format!("bad header `{header}`"));
    }

    for (idx, line) in lines.enumerate() {
        let lineno = idx + 2;
        if line.is_empty() {
            return Err(format!("line {lineno}: empty line"));
        }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 4 {
            return Err(format!(
                "line {lineno}: expected 4 tab-separated fields, found {}",
                parts.len()
            ));
        }
        let event = parse_event(parts[0]).map_err(|e| format!("line {lineno}: {e}"))?;
        let key = parts[1];
        let witness = parts[2];
        if key.is_empty() {
            return Err(format!("line {lineno}: empty key"));
        }
        if parts[3].is_empty() {
            return Err(format!("line {lineno}: empty date"));
        }

        if let Some(origin) = witness.strip_prefix("alias:") {
            if !origin.is_empty() && pin.contains(origin) {
                alias_new_to_old.insert(key.to_string(), origin.to_string());
            }
        }

        match event {
            Event::Minted => {
                minted.insert(key.to_string());
            }
            Event::Resolved | Event::Dropped => {
                removed.insert(key.to_string());
            }
            Event::Carried => {}
        }
    }

    let mut accounted_origins: BTreeSet<String> = BTreeSet::new();
    for (new, old) in &alias_new_to_old {
        if roster.contains(new) {
            accounted_origins.insert(old.to_string());
        }
    }

    let false_green = pin
        .iter()
        .filter(|k| {
            !roster.contains(*k) && !removed.contains(*k) && !accounted_origins.contains(*k)
        })
        .count();
    let false_red = roster
        .iter()
        .filter(|k| {
            !pin.contains(*k) && !minted.contains(*k) && !alias_new_to_old.contains_key(*k)
        })
        .count();
    Ok((false_red, false_green))
}

fn shadow_null_control(pin: &BTreeSet<String>) -> (usize, usize) {
    let k = pin.len().min(3);
    let empty_log = format!("{HEADER_PREFIX}shadow\n");
    let mut dev_red = 0usize;
    let mut dev_green = 0usize;

    let all_carried = carried_log(pin.iter(), 0);
    match compare_roster(pin, pin, &all_carried) {
        Ok((r, g)) => {
            dev_red += r;
            dev_green += g;
        }
        Err(_) => {
            dev_red += 1;
            dev_green += 1;
        }
    }

    let reduced: BTreeSet<String> = pin.iter().skip(k).cloned().collect();
    match compare_roster(&reduced, pin, &empty_log) {
        Ok((r, g)) => {
            dev_red += r;
            dev_green += g.abs_diff(k);
        }
        Err(_) => {
            dev_red += 1;
            dev_green += 1;
        }
    }

    if let Some(first) = pin.iter().next() {
        let renamed = format!("{first}__alias");
        let mut roster = pin.clone();
        roster.remove(first);
        roster.insert(renamed.clone());
        let log =
            format!("{HEADER_PREFIX}shadow\ncarried\t{renamed}\talias:{first}\t2026-10-07\n");
        match compare_roster(&roster, pin, &log) {
            Ok((r, g)) => {
                dev_red += r;
                dev_green += g;
            }
            Err(_) => {
                dev_red += 1;
                dev_green += 1;
            }
        }
    }

    let mut grown = pin.clone();
    for i in 0..k {
        grown.insert(format!("__new{i}"));
    }
    match compare_roster(&grown, pin, &empty_log) {
        Ok((r, g)) => {
            dev_red += r.abs_diff(k);
            dev_green += g;
        }
        Err(_) => {
            dev_red += 1;
            dev_green += 1;
        }
    }

    (dev_red, dev_green)
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
    let revival_hole =
        "# dropped-events v1 pin=x\ndropped\tc\treason\t2026-10-07\nminted\tc\t\t2026-10-07\n";
    if fold_events(revival_hole, &pin).refusal.is_none() {
        return Err("tombstoned mint without revive not refused".to_string());
    }
    let revived_log =
        "# dropped-events v1 pin=x\ndropped\tc\treason\t2026-10-07\nminted\tc\trevive\t2026-10-07\n";
    let revived = fold_events(revived_log, &pin);
    if let Some(r) = &revived.refusal {
        return Err(r.clone());
    }
    if !revived.keys.contains("c") {
        return Err("revive did not restore `c`".to_string());
    }
    if revived.generations.get("c") != Some(&1) {
        return Err("revive did not increment generation".to_string());
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut log_path: Option<String> = None;
    let mut pin_path = DEFAULT_PIN.to_string();
    let mut roster_path: Option<String> = None;
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
            "--roster" => {
                i += 1;
                match args.get(i) {
                    Some(v) => roster_path = Some(v.clone()),
                    None => {
                        eprintln!("dropped_gate: --roster needs a path");
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
        let pin = match read_keys(&pin_path, "pin") {
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
    let pin = match read_keys(&pin_path, "pin") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("dropped_gate: {e}");
            std::process::exit(1);
        }
    };

    if let Some(roster_path) = roster_path {
        let roster = match read_keys(&roster_path, "roster") {
            Ok(r) => r,
            Err(e) => {
                eprintln!("dropped_gate: {e}");
                std::process::exit(1);
            }
        };
        let (false_red, false_green) = match compare_roster(&roster, &pin, &log) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("dropped_gate: refusal: {e}");
                std::process::exit(1);
            }
        };
        let sharp = false_red == 0 && false_green == 0;
        println!(
            "dropped_gate gate: false_red={false_red} false_green={false_green} sharp={sharp}"
        );
        std::process::exit(if sharp { 0 } else { 1 });
    }

    let outcome = fold_events(&log, &pin);
    if let Some(r) = &outcome.refusal {
        eprintln!("dropped_gate: refusal: {r}");
        std::process::exit(1);
    }

    let revived = outcome.generations.values().filter(|g| **g > 0).count();
    println!(
        "dropped_gate: minted={} carried={} resolved={} dropped={} revived={}",
        outcome.minted, outcome.carried, outcome.resolved, outcome.dropped, revived
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

    fn set(items: &[&str]) -> BTreeSet<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn fold_applies_the_four_events() {
        let pin = set(&["a", "b", "c"]);
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
    fn tombstone_refuses_revival_without_witness() {
        let pin = set(&["a", "b", "c"]);
        let hole =
            "# dropped-events v1 pin=x\ndropped\tc\treason\t2026-10-07\nminted\tc\t\t2026-10-07\n";
        assert!(fold_events(hole, &pin).refusal.is_some());
        let revived =
            "# dropped-events v1 pin=x\ndropped\tc\treason\t2026-10-07\nminted\tc\trevive\t2026-10-07\n";
        let o = fold_events(revived, &pin);
        assert!(o.refusal.is_none(), "{:?}", o.refusal);
        assert!(o.keys.contains("c"));
        assert_eq!(o.generations.get("c"), Some(&1));
    }

    #[test]
    fn gate_detects_silent_loss_and_unproven_appearance() {
        let pin = set(&["a", "b", "c", "d"]);
        let empty = format!("{HEADER_PREFIX}shadow\n");

        let reduced = set(&["a", "b", "c"]);
        assert_eq!(compare_roster(&reduced, &pin, &empty).unwrap(), (0, 1));

        let added = set(&["a", "b", "c", "d", "x", "y"]);
        assert_eq!(compare_roster(&added, &pin, &empty).unwrap(), (2, 0));

        let proven = set(&["a", "b", "c", "d", "x"]);
        let minted = format!("{HEADER_PREFIX}shadow\nminted\tx\t\t2026-10-07\n");
        assert_eq!(compare_roster(&proven, &pin, &minted).unwrap(), (0, 0));

        let renamed = set(&["a__alias", "b", "c", "d"]);
        let alias = format!(
            "{HEADER_PREFIX}shadow\ncarried\ta__alias\talias:a\t2026-10-07\n"
        );
        assert_eq!(compare_roster(&renamed, &pin, &alias).unwrap(), (0, 0));
    }

    #[test]
    fn shadow_control_is_sharp_on_synthetic_pin() {
        let pin = set(&["a", "b", "c", "d"]);
        assert_eq!(shadow_null_control(&pin), (0, 0));
        assert_eq!(shadow_null_control(&BTreeSet::new()), (0, 0));
    }
}
