use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn allowed(token: &str) -> bool {
    let t = token.trim().to_ascii_uppercase();
    !t.is_empty()
        && (t == "PD"
            || t.starts_with("PD-")
            || t.starts_with("CC0")
            || t.starts_with("CC-BY")
            || t.starts_with("OGL")
            || t.starts_with("ETALAB")
            || t.starts_with("PDL")
            || t == "JP"
            || t.starts_with("JP-")
            || t.starts_with("OWN-WORK")
            || t.starts_with("REVIEWED-"))
}

fn host_of(url: &str) -> Option<String> {
    let rest = url.split_once("://")?.1;
    let host = rest.split(['/', '?', '#']).next()?;
    if host.is_empty() {
        None
    } else {
        Some(host.trim_start_matches("www.").to_string())
    }
}

fn netloc_of(url: &str, origin: &str) -> Option<String> {
    if let Some(rest) = url.split_once("/releases/download/") {
        let tag = rest.1.split('/').next()?;
        if !tag.is_empty() {
            return Some(tag.to_string());
        }
    }
    host_of(url).or_else(|| host_of(origin))
}

struct Block {
    url: String,
    origin: String,
    terms: String,
    terms_ref: String,
}

fn parse_blocks(text: &str) -> Vec<Block> {
    let mut out = Vec::new();
    for raw in text.split("\n\n") {
        let mut b = Block {
            url: String::new(),
            origin: String::new(),
            terms: String::new(),
            terms_ref: String::new(),
        };
        for line in raw.lines() {
            if let Some(v) = line.strip_prefix("url ") {
                b.url = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("origin ") {
                b.origin = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("terms ") {
                b.terms = v.split_whitespace().next().unwrap_or("").to_string();
                b.terms_ref = v.trim().to_string();
            }
        }
        if !b.url.is_empty() {
            out.push(b);
        }
    }
    out
}

fn local_dirs(root: &Path) -> Vec<(String, PathBuf)> {
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for e in entries.flatten() {
        if !e.path().is_dir() {
            continue;
        }
        let name = e.file_name().to_string_lossy().to_string();
        if name.starts_with('_') || name == "content" {
            continue;
        }
        out.push((name, e.path()));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let work = match arg_value(&args, "--work") {
        Some(v) => v,
        None => match std::env::var("OMEGAFLOW_WORK") {
            Ok(v) => v,
            Err(_) => {
                let home = match std::env::var("HOME") {
                    Ok(v) => v,
                    Err(_) => String::new(),
                };
                format!("{home}/projects/omegaflow-work")
            }
        },
    };
    let work = PathBuf::from(work);
    let sources = work.join("sources");
    let register = match arg_value(&args, "--register") {
        Some(v) => v,
        None => "phi/sources.φ".to_string(),
    };
    let fail = args.iter().any(|a| a == "--fail");
    let write_flags = args.iter().any(|a| a == "--write-flags");
    let text = match fs::read_to_string(&register) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{register}: {e}");
            std::process::exit(2);
        }
    };
    let mut terms: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut terms_ref: BTreeMap<String, String> = BTreeMap::new();
    for b in parse_blocks(&text) {
        let Some(netloc) = netloc_of(&b.url, &b.origin) else {
            continue;
        };
        if b.terms.is_empty() {
            continue;
        }
        terms.entry(netloc.clone()).or_default().insert(b.terms);
        terms_ref
            .entry(netloc)
            .or_insert_with(|| b.terms_ref.clone());
    }

    let mut unregistered = 0usize;
    let mut unflagged = 0usize;
    let mut flagged = 0usize;
    let mut ok = 0usize;
    for (netloc, dir) in local_dirs(&sources) {
        let has_flag = dir.join("TERMS.md").is_file()
            || dir.join("PROVENANCE.md").is_file()
            || dir.join("UNREGISTERED.md").is_file();
        let Some(tokens) = terms.get(&netloc) else {
            if has_flag {
                flagged += 1;
                println!("FLAGGED-UNREGISTERED\t{netloc}\t{}", dir.display());
            } else {
                unregistered += 1;
                if write_flags {
                    let body = format!(
                        "# {netloc} — local copy without a register block\n\n- status: unregistered local copy; no CDN upload; owner/review pending\n- generated: 2026-10-20\n"
                    );
                    let _ = fs::write(dir.join("UNREGISTERED.md"), body);
                }
                println!("UNREGISTERED\t{netloc}\t{}", dir.display());
            }
            continue;
        };
        let bad: Vec<&String> = tokens.iter().filter(|t| !allowed(t)).collect();
        if bad.is_empty() {
            ok += 1;
        } else if has_flag {
            flagged += 1;
            println!(
                "FLAGGED\t{netloc}\t{}",
                bad.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(",")
            );
        } else {
            unflagged += 1;
            if write_flags {
                let bad_text = bad
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                let reference = match terms_ref.get(&netloc).cloned() {
                    Some(v) => v,
                    None => String::new(),
                };
                let body = format!(
                    "# {netloc} — local copy\n\n- terms: `{bad_text}`\n- evidence: {reference}\n- status: local; no CDN upload before review\n- generated: 2026-10-20\n"
                );
                let _ = fs::write(dir.join("TERMS.md"), body);
            }
            println!(
                "UNFLAGGED\t{netloc}\t{}",
                bad.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(",")
            );
        }
    }
    eprintln!(
        "sources_local_audit: ok {ok} · flagged {flagged} · unflagged {unflagged} · unregistered {unregistered}"
    );
    if fail && (unflagged > 0 || unregistered > 0) {
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn netloc_from_github_release() {
        assert_eq!(
            netloc_of(
                "https://github.com/omegaflow/sources/releases/download/wso.stanford.edu/wso_polar.bin",
                ""
            )
            .as_deref(),
            Some("wso.stanford.edu")
        );
    }

    #[test]
    fn netloc_from_origin_host() {
        assert_eq!(
            netloc_of("https://example.org/file.bin", "https://data.example.org/x").as_deref(),
            Some("example.org")
        );
    }

    #[test]
    fn allowlist_is_the_house_rule() {
        assert!(allowed("CC-BY-NC-4.0"));
        assert!(allowed("reviewed-2026-10-20"));
        assert!(!allowed("free-open"));
        assert!(!allowed("other-attribution"));
    }
}
