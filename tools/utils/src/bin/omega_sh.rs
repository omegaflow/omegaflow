use omegaflow::json::{JsonVal, jpath_val, parse_json};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        return;
    }
    match args[1].as_str() {
        "reports" => reports(),
        "status" => status(),
        "search" => run_sibling("sgrep", &args[2..]),
        "fetch" => run_sibling("sfetch", &args[2..]),
        "jwst" => jwst(),
        "sha" => match args.get(2) {
            Some(path) => sha(path),
            None => usage(),
        },
        "check" => check(),
        "perms" => perms(args.get(2).map(|s| s.as_str())),
        "help" | "-h" | "--help" => usage(),
        other => {
            eprintln!("omega_sh: unknown subcommand '{}'", other);
            usage();
        }
    }
}

fn usage() {
    eprintln!("usage: omega_sh <reports|status|search|fetch|jwst|sha|check> [args]");
    eprintln!("  reports  concat <state>/reports/*.φ (the watchdog lines)");
    eprintln!("  status   git status --short");
    eprintln!("  search   sgrep <args>");
    eprintln!("  fetch    sfetch <args>");
    eprintln!("  jwst     the jwst_spectra.bin CDN watch");
    eprintln!("  sha <f>  sha256 over the body without the <!-- … --> header");
    eprintln!("  check    cargo check, summarised as error/warning counts");
    eprintln!("  perms [<agent>]  forbidden leading form → canonical replacement (from opencode.json)");
}

fn sha(path: &str) {
    let data = match fs::read(path) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("omega_sh: read void: {}", path);
            std::process::exit(2);
        }
    };
    println!("{}", hex(&sha256(body_after_header(&data))));
}

fn body_after_header(data: &[u8]) -> &[u8] {
    let mut open = false;
    let mut i = 0usize;
    while i < data.len() {
        let end = data[i..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|p| i + p)
            .unwrap_or(data.len());
        let line = &data[i..end];
        if !open && line.starts_with(b"<!--") {
            open = true;
        } else if open && line.starts_with(b"-->") {
            return if end < data.len() {
                &data[end + 1..]
            } else {
                &data[data.len()..]
            };
        }
        i = if end < data.len() {
            end + 1
        } else {
            data.len()
        };
    }
    if open { &data[data.len()..] } else { data }
}

fn check() {
    let out = match Command::new("cargo").arg("check").output() {
        Ok(o) => o,
        Err(_) => {
            eprintln!("omega_sh: cargo not available");
            return;
        }
    };
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    let (errors, warnings) = check_counts(&text);
    println!("cargo check: {} errors, {} warnings", errors, warnings);
    if errors > 0 {
        eprint!("{}", text);
    }
    if !out.status.success() {
        std::process::exit(1);
    }
}

fn check_counts(text: &str) -> (usize, usize) {
    let mut errors = 0usize;
    let mut warnings = 0usize;
    for line in text.lines() {
        let t = line.trim_start();
        if t.starts_with("error")
            && !t.contains("could not compile")
            && !t.contains("previous error")
        {
            errors += 1;
        }
        if t.starts_with("warning") && !t.contains("generated") {
            warnings += 1;
        }
    }
    (errors, warnings)
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push(DIGITS[(b >> 4) as usize] as char);
        s.push(DIGITS[(b & 0x0f) as usize] as char);
    }
    s
}

const SHA_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const SHA_H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

fn sha256(data: &[u8]) -> [u8; 32] {
    let mut h = SHA_H0;
    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[4 * i],
                chunk[4 * i + 1],
                chunk[4 * i + 2],
                chunk[4 * i + 3],
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
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA_K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    let mut out = [0u8; 32];
    for i in 0..8 {
        out[4 * i..4 * i + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    out
}

fn reports() {
    let entries = match fs::read_dir(state_dir().join("reports")) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("omega_sh: no <state>/reports dir");
            return;
        }
    };
    let mut names: Vec<String> = entries
        .flatten()
        .filter(|e| e.path().is_file())
        .filter(|e| {
            e.path()
                .file_name()
                .map(|f| f.to_string_lossy().ends_with(".φ"))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();
    names.sort();
    for n in names {
        match fs::read_to_string(&n) {
            Ok(t) => print!("{}", t),
            Err(_) => eprintln!("omega_sh: read void: {}", n),
        }
    }
}

fn status() {
    match Command::new("git").args(["status", "--short"]).output() {
        Ok(o) => {
            print!("{}", String::from_utf8_lossy(&o.stdout));
            eprint!("{}", String::from_utf8_lossy(&o.stderr));
        }
        Err(_) => eprintln!("omega_sh: git not available"),
    }
}

fn run_sibling(name: &str, args: &[String]) {
    let exe = match env::current_exe() {
        Ok(e) => e,
        Err(_) => {
            eprintln!("omega_sh: current_exe void");
            return;
        }
    };
    let dir = match exe.parent() {
        Some(p) => p.to_path_buf(),
        None => Path::new(".").to_path_buf(),
    };
    let sibling = dir.join(name);
    match Command::new(&sibling).args(args).output() {
        Ok(o) => {
            print!("{}", String::from_utf8_lossy(&o.stdout));
            eprint!("{}", String::from_utf8_lossy(&o.stderr));
        }
        Err(_) => eprintln!("omega_sh: {} not found next to {}", name, exe.display()),
    }
}

fn jwst() {
    let asset =
        "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/jwst_spectra.bin";
    let out = Command::new("curl")
        .args([
            "-sL",
            "-o",
            "/dev/null",
            "-w",
            "%{http_code}",
            "--max-time",
            "30",
            asset,
        ])
        .output();
    let code = match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => String::from("void"),
    };
    println!(
        "jwst_spectra.bin: {} ({}) — harvest complete",
        jwst_verdict(&code),
        code
    );
    if let Ok(t) = fs::read_to_string("phi/jwst_harvest/ledger.tsv") {
        let n = t.lines().count();
        println!("local ledger: {} obs_id finished", n);
    }
}

fn state_dir() -> PathBuf {
    if let Ok(dir) = env::var("OMEGAFLOW_STATE") {
        return PathBuf::from(dir);
    }
    PathBuf::from("state")
}

fn jwst_verdict(status: &str) -> &'static str {
    if status == "200" { "DA" } else { "absent" }
}

fn repo_root() -> PathBuf {
    if let Ok(r) = env::var("OMEGAFLOW_REPO") {
        if !r.is_empty() {
            return PathBuf::from(r);
        }
    }
    let mut dir = match env::current_dir() {
        Ok(d) => d,
        Err(_) => return PathBuf::from("."),
    };
    loop {
        if dir.join("opencode.json").is_file() {
            return dir;
        }
        match dir.parent() {
            Some(p) => dir = p.to_path_buf(),
            None => return PathBuf::from("."),
        }
    }
}

fn load_config() -> Option<JsonVal> {
    let text = fs::read_to_string(repo_root().join("opencode.json")).ok()?;
    parse_json(&text)
}

fn norm_pattern(p: &str) -> String {
    let s: String = p.chars().filter(|c| !c.is_whitespace() && *c != '`').collect();
    let s = match s.strip_prefix("*/") {
        Some(rest) => rest.to_string(),
        None => s.trim_start_matches('*').to_string(),
    };
    s.chars().filter(|c| *c != '*').collect()
}

const DENY_REPLACEMENTS: &[(&str, &str)] = &[
    ("ls", "glob <pattern>"),
    ("grep", "sgrep [-i] [-l] [-c] <pattern> [dir] | archive_search <kw> --root <dir>"),
    ("rg", "sgrep"),
    ("cat", "sread <file> [--offset N --limit M] | the read tool"),
    ("cd", "the bash workdir parameter"),
    ("python", "Rust (no Python in this repo)"),
    ("python3", "Rust (no Python in this repo)"),
    ("sed-i", "the edit tool"),
    ("rustc", "build via cargo"),
    ("cargo", "cargo check | cargo fmt -- <path> | cargo build -p <crate> --bin <name> | cargo run -p <crate> --bin <name>"),
    ("ghrunlist", "ci_manage list"),
    ("ghrunview", "ci_manage view <id> | ci_manage log <id>"),
    ("ghrundelete", "never — a deleted run takes the measurement series"),
    ("ghrunwatch", "no polling — ci_manage view <id> once"),
    ("gitreset", "never (destructive)"),
    ("gitcheckout", "never (destructive)"),
    ("gitclean", "never (destructive)"),
    ("gitrebase", "never (destructive)"),
    ("gitstash", "never (destructive)"),
    ("gitrestore", "never (destructive)"),
    ("gitswitch", "never (destructive)"),
    ("gitpush--force", "never (force)"),
    ("gitpush-f", "never (force)"),
    ("watch", "no polling"),
    ("sleep", "no polling"),
    ("inotifywait", "no polling"),
    ("entr", "no polling"),
    ("while", "no polling"),
    ("until", "no polling"),
    ("tail-f", "no polling — tail without -f is read-only"),
    ("journalctl-f", "no polling"),
    ("--watch", "no polling"),
    ("smail--send", "never from the machine — the send is the operator's hand"),
];

fn replacement_for(pattern: &str) -> &'static str {
    let n = norm_pattern(pattern);
    let mut best: Option<(&'static str, usize)> = None;
    for (tok, repl) in DENY_REPLACEMENTS {
        let t = norm_pattern(tok);
        if n == t || (n.starts_with(&t) && t.len() > 1) {
            if best.map(|(_, l)| t.len() > l).unwrap_or(true) {
                best = Some((repl, t.len()));
            }
        }
    }
    best.map(|(r, _)| r).unwrap_or("—")
}

fn add_bash_deny(v: Option<&JsonVal>, out: &mut BTreeSet<String>) {
    if let Some(JsonVal::Obj(map)) = v {
        for (pat, act) in map {
            if matches!(act, JsonVal::Str(s) if s == "deny") {
                out.insert(pat.clone());
            }
        }
    }
}

fn all_deny(cfg: &JsonVal) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    add_bash_deny(jpath_val(cfg, "permission.bash"), &mut out);
    if let Some(JsonVal::Obj(agents)) = jpath_val(cfg, "agent") {
        for (_name, a) in agents {
            add_bash_deny(jpath_val(a, "permission.bash"), &mut out);
        }
    }
    out
}

fn print_bash_layer(name: &str, v: Option<&JsonVal>) {
    let mut pats: Vec<String> = Vec::new();
    if let Some(JsonVal::Obj(map)) = v {
        for (pat, act) in map {
            if matches!(act, JsonVal::Str(s) if s == "deny") {
                pats.push(pat.clone());
            }
        }
    }
    pats.sort();
    println!("\n## {}  ({} deny)", name, pats.len());
    for p in &pats {
        println!("  {:<22} → {}", p, replacement_for(p));
    }
}

fn perms(agent: Option<&str>) {
    let cfg = match load_config() {
        Some(c) => c,
        None => {
            eprintln!("omega_sh: opencode.json absent — pending (no silent zero)");
            std::process::exit(2);
        }
    };
    let union = all_deny(&cfg);
    let scope = match agent {
        Some(a) => format!("agent {}", a),
        None => String::from("global"),
    };
    println!("opencode.json — forbidden leading form → canonical replacement ({scope})");
    println!("distinct deny patterns (global + all agents): {}", union.len());
    print_bash_layer("global permission.bash", jpath_val(&cfg, "permission.bash"));
    if let Some(a) = agent {
        let path = format!("agent.{}.permission.bash", a);
        print_bash_layer(&format!("agent {} permission.bash", a), jpath_val(&cfg, &path));
    }
    println!("\n## Ersatz-Karte");
    for (tok, repl) in DENY_REPLACEMENTS {
        println!("  {:<16} → {}", tok, repl);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verdict_da_on_200() {
        assert_eq!(jwst_verdict("200"), "DA");
    }

    #[test]
    fn verdict_fehlt_otherwise() {
        assert_eq!(jwst_verdict("404"), "absent");
        assert_eq!(jwst_verdict("void"), "absent");
    }

    #[test]
    fn sha256_matches_nist_vectors() {
        assert_eq!(
            hex(&sha256(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn body_starts_after_the_header() {
        let doc = b"<!--\n  title: t\n  sha256: x\n-->\nbody\n";
        assert_eq!(body_after_header(doc), b"body\n");
        let plain = b"body without header\n";
        assert_eq!(body_after_header(plain), plain);
    }

    #[test]
    fn check_counts_ignore_the_summary_lines() {
        let text = "error[E0432]: unresolved import\n --> src/x.rs\nerror: could not compile `c` due to 1 previous error\n";
        assert_eq!(check_counts(text), (1, 0));
        let text =
            "warning: unused import\n --> src/x.rs\nwarning: `c` (bin \"x\") generated 1 warning\n";
        assert_eq!(check_counts(&text), (0, 1));
    }

    #[test]
    fn gate_perms_matches_config() {
        let cfg = load_config().expect("opencode.json parses");
        let denied = all_deny(&cfg);
        for tok in [
            "ls", "grep", "cat", "rg", "cd", "python", "python3", "sed -i", "rustc", "cargo",
            "gh run list", "gh run view", "git reset", "watch", "sleep",
        ] {
            let t = norm_pattern(tok);
            assert!(
                denied.iter().any(|p| norm_pattern(p) == t),
                "opencode.json must deny `{}`",
                tok
            );
        }
        let doc = fs::read_to_string(repo_root().join("docs/concepts/tool-forms.md"))
            .expect("docs/concepts/tool-forms.md");
        for t in doc_forbidden_tokens(&doc) {
            assert!(
                denied.iter().any(|p| norm_pattern(p) == t),
                "tool-forms.md token `{}` is not denied in opencode.json",
                t
            );
        }
    }

    fn doc_forbidden_tokens(doc: &str) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        for line in doc.lines() {
            let line = line.trim();
            if !line.starts_with("| `") {
                continue;
            }
            let cell = line.trim_matches('|').split('|').next().unwrap_or("");
            let mut rest = cell;
            while let Some(a) = rest.find('`') {
                let after = &rest[a + 1..];
                match after.find('`') {
                    Some(b) => {
                        out.insert(norm_pattern(&after[..b]));
                        rest = &after[b + 1..];
                    }
                    None => break,
                }
            }
        }
        out
    }
}
