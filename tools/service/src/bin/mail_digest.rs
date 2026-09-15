use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut path = match env_str("OMEGAFLOW_MAIL_LEDGER") {
        Some(p) => p,
        None => "state/mail/mail_ledger.φ".to_string(),
    };
    let mut from_filter: Option<String> = None;
    let mut to_filter: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--ledger" => {
                i += 1;
                if let Some(p) = args.get(i) {
                    path = p.clone();
                }
            }
            "--from" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    from_filter = Some(v.clone());
                }
            }
            "--to" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    to_filter = Some(v.clone());
                }
            }
            _ => {}
        }
        i += 1;
    }
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("mail_digest: ledger absent: {path}");
            std::process::exit(2);
        }
    };
    let records = records(&text);
    let mut shown = 0usize;
    println!("mail_digest | {} records | {path}", records.len());
    for r in &records {
        if let Some(f) = &from_filter {
            if !r.from.contains(f.as_str()) {
                continue;
            }
        }
        if let Some(t) = &to_filter {
            if !r.to.contains(t.as_str()) {
                continue;
            }
        }
        shown += 1;
        println!(
            "  {} | {:<40} -> {:<22} | {}",
            date(r.epoch),
            short(&r.from, 40),
            short(&r.to, 22),
            short(&r.subject, 60)
        );
    }
    println!("  shown {shown}");
}

struct Mail {
    epoch: i64,
    from: String,
    to: String,
    subject: String,
}

fn records(text: &str) -> Vec<Mail> {
    let mut out = Vec::new();
    for line in text.lines() {
        if !line.starts_with("mail\t") {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 5 {
            continue;
        }
        let Ok(epoch) = f[1].parse::<i64>() else {
            continue;
        };
        out.push(Mail {
            epoch,
            from: decode_mime(f[2]),
            to: decode_mime(f[3]),
            subject: decode_mime(f[4]),
        });
    }
    out
}

fn decode_mime(s: &str) -> String {
    let s = s.trim();
    let Some(rest) = s.strip_prefix("=?utf-8?Q?") else {
        return s.to_string();
    };
    let Some(end) = rest.find("?=") else {
        return s.to_string();
    };
    let body = &rest[..end];
    let raw = body.as_bytes();
    let mut bytes: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'=' && i + 2 < raw.len() {
            if let Ok(v) = u8::from_str_radix(&body[i + 1..i + 3], 16) {
                bytes.push(v);
                i += 3;
                continue;
            }
        }
        bytes.push(if raw[i] == b'_' { b' ' } else { raw[i] });
        i += 1;
    }
    String::from_utf8_lossy(&bytes).to_string()
}

fn date(epoch: i64) -> String {
    let days = epoch.div_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn short(s: &str, n: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= n {
        return s.to_string();
    }
    chars[..n.saturating_sub(1)].iter().collect::<String>() + "…"
}

fn env_str(name: &str) -> Option<String> {
    env::var(name).ok().filter(|v| !v.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_starts_at_mail_tab() {
        let t = "mail\t100\ta@x\ts@y\tsubj\tbody line\ncontinuation\nmail\t200\tb@x\ts@y\ts2\t\n";
        let r = records(t);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].from, "a@x");
        assert_eq!(r[0].subject, "subj");
        assert_eq!(r[1].epoch, 200);
    }

    #[test]
    fn civil_date_epoch_zero() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(date(1787631476), "2026-08-25");
    }
}
