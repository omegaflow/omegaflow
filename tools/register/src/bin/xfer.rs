use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

const TERMINALS: [&str; 3] = ["REJECTED", "SUPERSEDED", "DROPPED"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Derived {
    Sent,
    Folded,
    Closed,
    Rejected,
    Superseded,
    Dropped,
    Contested,
}

impl Derived {
    fn name(self) -> &'static str {
        match self {
            Derived::Sent => "SENT",
            Derived::Folded => "FOLDED",
            Derived::Closed => "CLOSED",
            Derived::Rejected => "REJECTED",
            Derived::Superseded => "SUPERSEDED",
            Derived::Dropped => "DROPPED",
            Derived::Contested => "CONTESTED",
        }
    }

    fn disposed(self) -> bool {
        !matches!(self, Derived::Sent | Derived::Contested)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Token {
    line: String,
    sess: String,
    n: u64,
    h8: String,
}

fn usage() {
    println!(
        "usage: xfer <subcommand> [options]\n\
         \n\
         xfer mark --line <L> [--to <line|self>] <text...>   mint the next token, append SENT, print it\n\
         xfer list --line <L> [--open]                        one line per token and its derived state\n\
         xfer fold <tok> --line <L> --into <target>           append FOLDED (the folding session)\n\
         xfer close <tok> --line <L> --ref <commit|path>      append CLOSED with its reference\n\
         xfer coverage --line <L>                             emitted vs disposed; exit 2 while any token is open\n\
         \n\
         --line falls back to OMEGAFLOW_LINE; the session id to OMEGAFLOW_SESSION, then\n\
         state/reports/active_session.phi. A missing line or session exits 2, never a fabricated id.\n\
         ledger: state/zustand/ledger/<line>/<sess>.log  (gitignored, append-only)"
    );
}

fn fail(msg: &str) -> ! {
    eprintln!("xfer: {msg}");
    exit(2);
}

fn state_dir() -> PathBuf {
    match env::var("OMEGAFLOW_STATE") {
        Ok(d) if !d.trim().is_empty() => PathBuf::from(d),
        _ => PathBuf::from("state"),
    }
}

fn ledger_dir() -> PathBuf {
    state_dir().join("zustand/ledger")
}

fn valid_component(s: &str) -> bool {
    !s.is_empty() && !s.chars().any(|c| c == '/' || c == '~' || c.is_whitespace())
}

fn resolve_line(explicit: Option<String>) -> Result<String, String> {
    if let Some(l) = explicit {
        let t = l.trim();
        if !t.is_empty() {
            return Ok(t.to_string());
        }
    }
    match env::var("OMEGAFLOW_LINE") {
        Ok(l) if !l.trim().is_empty() => Ok(l.trim().to_string()),
        _ => Err("no line: --line absent and OMEGAFLOW_LINE unset".to_string()),
    }
}

fn resolve_sess_from(
    env_val: Option<String>,
    file_contents: Option<String>,
) -> Result<String, String> {
    if let Some(v) = env_val {
        let t = v.trim();
        if !t.is_empty() {
            return Ok(t.to_string());
        }
    }
    match file_contents {
        Some(s) => {
            let t = s.trim();
            if t.is_empty() {
                Err("active session file carries no id".to_string())
            } else {
                Ok(t.to_string())
            }
        }
        None => Err(
            "no session: OMEGAFLOW_SESSION unset and state/reports/active_session.phi absent"
                .to_string(),
        ),
    }
}

fn resolve_sess() -> Result<String, String> {
    let env_val = env::var("OMEGAFLOW_SESSION").ok();
    let file = state_dir().join("reports/active_session.φ");
    let file_contents = fs::read_to_string(&file).ok();
    resolve_sess_from(env_val, file_contents)
}

fn canonicalize(s: &str) -> String {
    let unified = s.replace("\r\n", "\n").replace('\r', "\n");
    unified
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_token(tok: &str) -> Option<Token> {
    let rest = tok.strip_prefix("xfer:")?;
    let (left, h8) = rest.split_once('~')?;
    if h8.len() != 8 || !h8.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let (head, n) = left.rsplit_once('.')?;
    let n: u64 = n.parse().ok()?;
    let (line, sess) = head.rsplit_once('.')?;
    if line.is_empty() || sess.is_empty() {
        return None;
    }
    Some(Token {
        line: line.to_string(),
        sess: sess.to_string(),
        n,
        h8: h8.to_string(),
    })
}

fn next_index(tokens: &[Token]) -> u64 {
    match tokens.iter().map(|t| t.n).max() {
        Some(m) => m + 1,
        None => 1,
    }
}

fn rank(state: &str) -> Option<u8> {
    match state {
        "SENT" => Some(0),
        "FOLDED" => Some(1),
        "CLOSED" => Some(2),
        _ => None,
    }
}

fn derived_rank(d: Derived) -> u8 {
    match d {
        Derived::Sent => 0,
        Derived::Folded => 1,
        Derived::Closed => 2,
        _ => 3,
    }
}

fn derive_state(states: &[&str]) -> Option<Derived> {
    let mut terminals: Vec<&str> = states
        .iter()
        .copied()
        .filter(|s| TERMINALS.contains(s))
        .collect();
    terminals.sort_unstable();
    terminals.dedup();
    match terminals.len() {
        1 => {
            return Some(match terminals[0] {
                "REJECTED" => Derived::Rejected,
                "SUPERSEDED" => Derived::Superseded,
                _ => Derived::Dropped,
            });
        }
        n if n > 1 => return Some(Derived::Contested),
        _ => {}
    }
    let mut best: Option<Derived> = None;
    for s in states {
        if let Some(r) = rank(s) {
            let cand = match r {
                0 => Derived::Sent,
                1 => Derived::Folded,
                _ => Derived::Closed,
            };
            best = Some(match best {
                Some(b) if derived_rank(b) >= r => b,
                _ => cand,
            });
        }
    }
    best
}

fn coverage_exit(states: &[Derived]) -> i32 {
    if states.iter().any(|s| !s.disposed()) {
        2
    } else {
        0
    }
}

fn parse_event(line: &str) -> Option<(String, String)> {
    let mut it = line.split_whitespace();
    let _utc = it.next()?;
    let state = it.next()?;
    let tok = it.next()?;
    if !tok.starts_with("xfer:") {
        return None;
    }
    Some((state.to_string(), tok.to_string()))
}

fn append_ledger(path: &Path, line: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    writeln!(f, "{line}").map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(())
}

fn read_file_events(file: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if let Ok(s) = fs::read_to_string(file) {
        for l in s.lines() {
            if let Some(ev) = parse_event(l) {
                out.push(ev);
            }
        }
    }
    out
}

fn read_dir_events(dir: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    paths.sort();
    for p in paths {
        out.extend(read_file_events(&p));
    }
    out
}

fn aggregate(events: &[(String, String)]) -> BTreeMap<String, Vec<String>> {
    let mut m: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (st, tok) in events {
        m.entry(tok.clone()).or_default().push(st.clone());
    }
    m
}

fn derived_of(states: &[String]) -> Option<Derived> {
    let refs: Vec<&str> = states.iter().map(|s| s.as_str()).collect();
    derive_state(&refs)
}

fn parse_flag<'a>(args: &'a [String], i: &mut usize, name: &str) -> Option<&'a String> {
    if args.get(*i).map(|s| s.as_str()) == Some(name) {
        *i += 1;
        args.get(*i)
    } else {
        None
    }
}

fn cmd_mark(args: &[String]) -> i32 {
    let mut line_opt: Option<String> = None;
    let mut to = "self".to_string();
    let mut text: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if let Some(v) = parse_flag(args, &mut i, "--line") {
            line_opt = Some(v.clone());
        } else if let Some(v) = parse_flag(args, &mut i, "--to") {
            to = v.clone();
        } else {
            text.push(args[i].clone());
        }
        i += 1;
    }
    if text.is_empty() {
        fail("mark: empty text");
    }
    let line = match resolve_line(line_opt) {
        Ok(l) => l,
        Err(e) => fail(&e),
    };
    if !valid_component(&line) {
        fail("mark: --line carries a forbidden character");
    }
    if to != "self" && !valid_component(&to) {
        fail("mark: --to carries a forbidden character");
    }
    let sess = match resolve_sess() {
        Ok(s) => s,
        Err(e) => fail(&e),
    };
    if !valid_component(&sess) {
        fail("mark: session id carries a forbidden character");
    }

    let file = ledger_dir().join(&line).join(format!("{sess}.log"));
    let existing: Vec<Token> = read_file_events(&file)
        .iter()
        .filter_map(|(_, t)| parse_token(t))
        .filter(|t| t.sess == sess)
        .collect();
    let n = next_index(&existing);
    let h8 = sha256_hex8(&canonicalize(&text.join(" ")));
    let token = format!("xfer:{line}.{sess}.{n}~{h8}");
    let rec = format!(
        "{} SENT {} by={}.{} to={}",
        utc_now(),
        token,
        line,
        sess,
        to
    );
    if let Err(e) = append_ledger(&file, &rec) {
        fail(&e);
    }
    println!("{token}");
    0
}

fn cmd_list(args: &[String]) -> i32 {
    let mut line_opt: Option<String> = None;
    let mut open = false;
    let mut i = 0;
    while i < args.len() {
        if let Some(v) = parse_flag(args, &mut i, "--line") {
            line_opt = Some(v.clone());
        } else if args[i] == "--open" {
            open = true;
        }
        i += 1;
    }
    let line = match resolve_line(line_opt) {
        Ok(l) => l,
        Err(e) => fail(&e),
    };
    if !valid_component(&line) {
        fail("list: --line carries a forbidden character");
    }
    let agg = aggregate(&read_dir_events(&ledger_dir().join(&line)));
    for (tok, states) in &agg {
        if let Some(st) = derived_of(states) {
            if open && st.disposed() {
                continue;
            }
            println!("{tok}\t{}", st.name());
        }
    }
    0
}

fn cmd_transition(args: &[String], state: &str) -> i32 {
    let mut tok: Option<String> = None;
    let mut line_opt: Option<String> = None;
    let mut extra: Option<String> = None;
    let extra_flag = if state == "FOLDED" { "--into" } else { "--ref" };
    let mut i = 0;
    while i < args.len() {
        if let Some(v) = parse_flag(args, &mut i, "--line") {
            line_opt = Some(v.clone());
        } else if let Some(v) = parse_flag(args, &mut i, extra_flag) {
            extra = Some(v.clone());
        } else if tok.is_none() {
            tok = Some(args[i].clone());
        }
        i += 1;
    }
    let tok = match tok {
        Some(t) => t,
        None => fail(&format!("{}: token absent", state.to_lowercase())),
    };
    let parsed = match parse_token(&tok) {
        Some(t) => t,
        None => fail(&format!("{}: token malformed: {tok}", state.to_lowercase())),
    };
    let line = match resolve_line(line_opt) {
        Ok(l) => l,
        Err(e) => fail(&e),
    };
    if line != parsed.line {
        fail(&format!(
            "{}: --line {line} differs from the token line {}",
            state.to_lowercase(),
            parsed.line
        ));
    }
    let extra = match extra {
        Some(e) => e,
        None => fail(&format!("{}: {extra_flag} absent", state.to_lowercase())),
    };
    let sess = match resolve_sess() {
        Ok(s) => s,
        Err(e) => fail(&e),
    };
    let key = if state == "FOLDED" { "into" } else { "ref" };
    let file = ledger_dir()
        .join(&line)
        .join(format!("{}.log", parsed.sess));
    let rec = format!(
        "{} {} {} by={}.{} {}={}",
        utc_now(),
        state,
        tok,
        line,
        sess,
        key,
        extra
    );
    if let Err(e) = append_ledger(&file, &rec) {
        fail(&e);
    }
    0
}

fn cmd_coverage(args: &[String]) -> i32 {
    let mut line_opt: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        if let Some(v) = parse_flag(args, &mut i, "--line") {
            line_opt = Some(v.clone());
        }
        i += 1;
    }
    let line = match resolve_line(line_opt) {
        Ok(l) => l,
        Err(e) => fail(&e),
    };
    if !valid_component(&line) {
        fail("coverage: --line carries a forbidden character");
    }
    let agg = aggregate(&read_dir_events(&ledger_dir().join(&line)));
    let mut emitted = 0usize;
    let mut disposed = 0usize;
    let mut open: Vec<(String, Derived)> = Vec::new();
    let mut states: Vec<Derived> = Vec::new();
    for (tok, evs) in &agg {
        let Some(st) = derived_of(evs) else {
            continue;
        };
        emitted += 1;
        states.push(st);
        if st.disposed() {
            disposed += 1;
        } else {
            open.push((tok.clone(), st));
        }
    }
    println!("xfer coverage — line {line}");
    println!("emitted  {emitted}");
    println!("disposed {disposed}");
    println!("open     {}", emitted - disposed);
    for (tok, st) in &open {
        println!("{tok}\t{}", st.name());
    }
    coverage_exit(&states)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let cmd = match args.first() {
        Some(c) => c.as_str(),
        None => {
            usage();
            exit(2);
        }
    };
    let rest = &args[1..];
    let code = match cmd {
        "mark" => cmd_mark(rest),
        "list" => cmd_list(rest),
        "fold" => cmd_transition(rest, "FOLDED"),
        "close" => cmd_transition(rest, "CLOSED"),
        "coverage" => cmd_coverage(rest),
        "-h" | "--help" => {
            usage();
            0
        }
        other => fail(&format!("unknown subcommand: {other}")),
    };
    exit(code);
}

fn utc_now() -> String {
    let secs = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(_) => fail("clock reads before the Unix epoch"),
    };
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (h, mi, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}T{h:02}:{mi:02}:{s:02}Z")
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn sha256(data: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut msg = data.to_vec();
    let bitlen = (data.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bitlen.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut v = h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ ((!v[4]) & v[6]);
            let t1 = v[7]
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[i])
                .wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v[7] = v[6];
            v[6] = v[5];
            v[5] = v[4];
            v[4] = v[3].wrapping_add(t1);
            v[3] = v[2];
            v[2] = v[1];
            v[1] = v[0];
            v[0] = t1.wrapping_add(t2);
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    let mut out = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

fn sha256_hex8(body: &str) -> String {
    let digest = sha256(body.as_bytes());
    let mut s = String::with_capacity(8);
    for b in digest.iter().take(4) {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tok(line: &str, sess: &str, n: u64) -> Token {
        Token {
            line: line.to_string(),
            sess: sess.to_string(),
            n,
            h8: "deadbeef".to_string(),
        }
    }

    #[test]
    fn mint_index_increments() {
        assert_eq!(next_index(&[]), 1);
        assert_eq!(next_index(&[tok("mountain", "s1", 1)]), 2);
        assert_eq!(
            next_index(&[tok("mountain", "s1", 1), tok("mountain", "s1", 7)]),
            8
        );
    }

    #[test]
    fn state_is_max_over_events() {
        assert_eq!(derive_state(&["SENT"]), Some(Derived::Sent));
        assert_eq!(derive_state(&["SENT", "FOLDED"]), Some(Derived::Folded));
        assert_eq!(
            derive_state(&["FOLDED", "SENT", "CLOSED"]),
            Some(Derived::Closed)
        );
        assert_eq!(derive_state(&[]), None);
    }

    #[test]
    fn terminal_wins_and_conflicts_are_contested() {
        assert_eq!(
            derive_state(&["SENT", "CLOSED", "REJECTED"]),
            Some(Derived::Rejected)
        );
        assert_eq!(
            derive_state(&["REJECTED", "SUPERSEDED"]),
            Some(Derived::Contested)
        );
    }

    #[test]
    fn coverage_is_open_while_a_token_is_sent() {
        assert_eq!(coverage_exit(&[Derived::Folded, Derived::Closed]), 0);
        assert_eq!(coverage_exit(&[Derived::Closed, Derived::Sent]), 2);
        assert_eq!(coverage_exit(&[Derived::Contested]), 2);
    }

    #[test]
    fn missing_session_is_an_error_not_a_token() {
        assert!(resolve_sess_from(None, None).is_err());
        assert!(resolve_sess_from(Some("   ".to_string()), None).is_err());
        assert_eq!(
            resolve_sess_from(Some("ses_abc".to_string()), None).ok(),
            Some("ses_abc".to_string())
        );
        assert_eq!(
            resolve_sess_from(None, Some("ses_file\n".to_string())).ok(),
            Some("ses_file".to_string())
        );
    }

    #[test]
    fn token_roundtrip() {
        let t = parse_token("xfer:mountain.ses_1.3~deadbeef").unwrap();
        assert_eq!(t.line, "mountain");
        assert_eq!(t.sess, "ses_1");
        assert_eq!(t.n, 3);
        assert_eq!(t.h8, "deadbeef");
        assert!(parse_token("xfer:mountain.ses_1~deadbeef").is_none());
        assert!(parse_token("nope").is_none());
    }

    #[test]
    fn canonicalize_strips_trailing_space_and_normalizes_line_endings() {
        assert_eq!(canonicalize("a  \r\nb\t\r\n"), "a\nb");
        assert_eq!(sha256_hex8("abc").len(), 8);
    }
}
