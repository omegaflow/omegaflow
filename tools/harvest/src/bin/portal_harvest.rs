use omegaflow::archivar::json::{JsonVal, jpath_val, jstr, parse_json};
use std::env;
use std::fs;
use std::process::Command;

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        None
    }
}

fn urlencode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Class {
    Redistributable,
    Unknown,
    Blocked,
}

fn classify(token: &str) -> Class {
    let t = token.trim().to_ascii_lowercase();
    if t.is_empty()
        || matches!(
            t.as_str(),
            "notspecified" | "unspecified" | "unknown" | "none" | "no-license"
        )
    {
        return Class::Unknown;
    }
    if t.split('-').any(|seg| seg == "nd")
        || matches!(
            t.as_str(),
            "other-closed" | "proprietary" | "all-rights-reserved"
        )
    {
        return Class::Blocked;
    }
    let redistributable = t.contains("cc0")
        || t.contains("cc-by")
        || t.contains("publicdomain")
        || t.contains("public-domain")
        || t.contains("cc-pd")
        || t.contains("other-pd")
        || t.contains("odbl")
        || t.contains("odc-by")
        || t.contains("ogl")
        || t.contains("pddl")
        || t.contains("sprep-public")
        || t.contains("dl-de");
    if redistributable {
        Class::Redistributable
    } else {
        Class::Blocked
    }
}

fn class_word(c: Class) -> &'static str {
    match c {
        Class::Redistributable => "redistributable",
        Class::Unknown => "terms-unknown",
        Class::Blocked => "blocked",
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut base: Option<String> = None;
    let mut query = String::from("*:*");
    let mut rows: usize = 100;
    let mut start: usize = 0;
    let mut out: Option<String> = None;
    let mut show = false;
    let mut all = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--base" => {
                base = args.get(i + 1).cloned();
                i += 1;
            }
            "--q" => {
                if let Some(v) = args.get(i + 1) {
                    query = v.clone();
                }
                i += 1;
            }
            "--rows" => {
                rows = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(100);
                i += 1;
            }
            "--start" => {
                if let Some(v) = args.get(i + 1).and_then(|s| s.parse().ok()) {
                    start = v;
                }
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--package-show" => show = true,
            "--all" => all = true,
            _ => {}
        }
        i += 1;
    }
    let Some(base) = base else {
        eprintln!(
            "usage: portal_harvest --base <ckan-url> [--q <query>] [--rows N] [--start N] [--package-show] [--all] [--out path]"
        );
        eprintln!("  queries {{base}}/api/3/action/package_search and classifies license_id");
        eprintln!(
            "  (redistributable = PD/CC0/CC-BY[-SA/-NC/-NC-SA]/ODbL/OGL/…; blocked = ND/proprietary/other-closed;"
        );
        eprintln!("   notspecified/unknown = terms-unknown, a measurement — never a silent drop).");
        eprintln!(
            "  A Cloudflare interstitial is a non-browser curl limit, not a portal verdict: the browser bridge is the CF path."
        );
        std::process::exit(2);
    };
    let base = base.trim_end_matches('/').to_string();
    let url = format!(
        "{}/api/3/action/package_search?q={}&rows={}&start={}",
        base,
        urlencode(&query),
        rows,
        start
    );
    let Some(body) = curl(&url) else {
        eprintln!("package_search returned void: {url}");
        std::process::exit(1);
    };
    let Some(json) = parse_json(&body) else {
        eprintln!("package_search returned no JSON (Cloudflare interstitial?): {url}");
        std::process::exit(1);
    };
    let Some(JsonVal::Arr(results)) = jpath_val(&json, "result.results") else {
        eprintln!("package_search carries no result.results array: {url}");
        std::process::exit(1);
    };
    let count = jpath_val(&json, "result.count")
        .and_then(|v| match v {
            JsonVal::Num(n) => Some(*n as usize),
            _ => None,
        })
        .unwrap_or(results.len());

    let mut buf = String::new();
    let mut emitted = 0usize;
    let mut unknown = 0usize;
    let mut blocked = 0usize;
    for d in results {
        let title = match jstr(d, "title") {
            Some(s) => s,
            None => String::new(),
        };
        let name = match jstr(d, "name") {
            Some(s) => s,
            None => String::new(),
        };
        let license = match jstr(d, "license_id") {
            Some(s) => s,
            None => String::new(),
        };
        let class = classify(&license);
        match class {
            Class::Redistributable => emitted += 1,
            Class::Unknown => unknown += 1,
            Class::Blocked => blocked += 1,
        }
        if !all && class == Class::Blocked {
            continue;
        }
        let dataset_url = format!("{base}/dataset/{name}");
        let license_word = if license.is_empty() {
            "none"
        } else {
            license.as_str()
        };
        let mut line = format!(
            "{} | {} | {} | {}",
            class_word(class),
            license_word,
            title,
            dataset_url
        );
        if show {
            let show_url = format!("{}/api/3/action/package_show?id={}", base, urlencode(&name));
            if let Some(sb) = curl(&show_url) {
                if let Some(sj) = parse_json(&sb) {
                    if let Some(JsonVal::Arr(res)) = jpath_val(&sj, "result.resources") {
                        for r in res {
                            if let Some(u) = jstr(r, "url") {
                                if !u.is_empty() {
                                    line.push_str(" | ");
                                    line.push_str(&u);
                                }
                            }
                        }
                    }
                }
            }
        }
        buf.push_str(&line);
        buf.push('\n');
    }
    if let Some(path) = out {
        let _ = fs::write(&path, &buf);
        eprintln!(
            "portal_harvest: {count} datasets | {emitted} redistributable, {unknown} terms-unknown, {blocked} blocked → {path}"
        );
    } else {
        print!("{buf}");
        eprintln!(
            "portal_harvest: {count} datasets | {emitted} redistributable, {unknown} terms-unknown, {blocked} blocked"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_splits_the_license_vocabulary() {
        assert_eq!(classify("CC-BY-4.0"), Class::Redistributable);
        assert_eq!(classify("cc0"), Class::Redistributable);
        assert_eq!(classify("sprep-public-license"), Class::Redistributable);
        assert_eq!(classify("other-pd"), Class::Redistributable);
        assert_eq!(classify("CC-BY-NC-SA-4.0"), Class::Redistributable);
        assert_eq!(classify("CC-BY-NC-3.0-IGO"), Class::Redistributable);
        assert_eq!(classify("CC-BY-NC-ND-4.0"), Class::Blocked);
        assert_eq!(classify("CC-BY-ND-4.0"), Class::Blocked);
        assert_eq!(classify("other-closed"), Class::Blocked);
        assert_eq!(classify("notspecified"), Class::Unknown);
        assert_eq!(classify(""), Class::Unknown);
    }
}
