use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn work_root() -> PathBuf {
    if let Some(dir) = env::var_os("OMEGAFLOW_WORK").filter(|v| !v.is_empty()) {
        return PathBuf::from(dir);
    }
    if let Some(home) = env::var_os("HOME").filter(|v| !v.is_empty()) {
        return PathBuf::from(home).join("projects/omegaflow-work");
    }
    PathBuf::from("omegaflow-work")
}

fn today() -> i64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => (d.as_secs() as i64).div_euclid(86_400),
        Err(e) => {
            eprintln!("waiting_check: the clock reads before 1970: {e}");
            std::process::exit(2);
        }
    }
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn dates_in(text: &str) -> Vec<i64> {
    let bytes = text.as_bytes();
    let number = |from: usize, to: usize| -> i64 {
        let mut value = 0i64;
        for byte in &bytes[from..to] {
            value = value * 10 + (byte - b'0') as i64;
        }
        value
    };
    let mut out = Vec::new();
    let mut i = 0;
    while i + 10 <= bytes.len() {
        let shape = bytes[i + 4] == b'-'
            && bytes[i + 7] == b'-'
            && bytes[i..i + 4].iter().all(|b| b.is_ascii_digit())
            && bytes[i + 5..i + 7].iter().all(|b| b.is_ascii_digit())
            && bytes[i + 8..i + 10].iter().all(|b| b.is_ascii_digit());
        if shape {
            let (y, m, d) = (number(i, i + 4), number(i + 5, i + 7), number(i + 8, i + 10));
            if (1..=12).contains(&m) && (1..=31).contains(&d) {
                out.push(days_from_civil(y, m, d));
            }
            i += 10;
        } else {
            i += 1;
        }
    }
    out
}

fn field<'a>(block: &'a str, name: &str) -> Option<&'a str> {
    block
        .lines()
        .find_map(|line| line.strip_prefix(name).and_then(|rest| rest.strip_prefix(' ')))
        .map(|v| v.trim())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stale_after: i64 = 14;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--stale-after" => {
                i += 1;
                match args.get(i).and_then(|v| v.parse::<i64>().ok()) {
                    Some(n) => stale_after = n,
                    None => {
                        eprintln!("waiting_check: --stale-after needs a number of days");
                        std::process::exit(2);
                    }
                }
            }
            "-h" | "--help" => {
                println!(
                    "usage: waiting_check [--stale-after <tage>]\n\
                     \n\
                     Reads state/pending.phi and prints two findings: a date in `trigger`\n\
                     that lies in the past (EXPIRED), and a block whose newest measurement\n\
                     is older than the limit (STALE). Exit 1 when an entry is expired."
                );
                return;
            }
            other => {
                eprintln!("waiting_check: unknown argument {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let path = work_root().join("state/pending.\u{03c6}");
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("waiting_check: {} reads not: {}", path.display(), e);
            std::process::exit(2);
        }
    };

    let now = today();
    let blocks: Vec<&str> = text
        .split("\n\n")
        .filter(|b| !b.trim().is_empty() && !b.trim_start().starts_with('#'))
        .collect();

    let mut open = 0usize;
    let mut dated = 0usize;
    let mut expired = 0usize;
    let mut stale = 0usize;

    for block in &blocks {
        let Some(key) = field(block, "key") else {
            continue;
        };
        open += 1;
        let trigger = field(block, "trigger").unwrap_or("");
        let lage = field(block, "lage").unwrap_or("");

        let term: Vec<i64> = dates_in(trigger);
        if !term.is_empty() {
            dated += 1;
        }
        let past: Vec<i64> = term.iter().copied().filter(|d| *d < now).collect();
        if let Some(day) = past.iter().max() {
            expired += 1;
            println!("EXPIRED {key} termin={} tage={}", show(*day), now - day);
        }

        let newest = dates_in(lage).into_iter().max();
        match newest {
            Some(day) if now - day <= stale_after => {}
            Some(day) => {
                stale += 1;
                println!("STALE   {key} messung={} tage={}", show(day), now - day);
            }
            None => {
                stale += 1;
                println!("STALE   {key} messung=fehlt");
            }
        }
    }

    println!(
        "waiting_check — {open} offen · {dated} mit Termin · {expired} Termin verstrichen · {stale} ohne frische Messung (>{stale_after} Tage)"
    );
    if expired > 0 {
        std::process::exit(1);
    }
}

fn show(day: i64) -> String {
    let z = day + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_round_trip() {
        let day = days_from_civil(2026, 10, 31);
        assert_eq!(show(day), "2026-10-31");
    }

    #[test]
    fn dates_are_found_in_text() {
        let found = dates_in("Termin 2026-12-02, vorher 2026-10-11");
        assert_eq!(found.len(), 2);
        assert_eq!(show(found[1]), "2026-12-02");
    }
}
