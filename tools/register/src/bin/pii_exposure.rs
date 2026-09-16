use std::env;
use std::fs;
use std::process::Command;

const REPO: &str = "omegaflow/omegaflow";

const COMMITS: [&str; 15] = [
    "317418d", "afae680", "69813f1", "0d76458f", "88dda694", "ea696f62", "035a9191", "afa96459",
    "d9d6e800", "b0bebc1c", "d2ab19b1", "10af67bd", "1ecb8e77", "1258eb69", "1db52c0e",
];

const SENSITIVE_PATHS: [&str; 10] = [
    "docs/auftrag/auftrag-adoption-mails.md",
    "docs/auftrag/auftrag-rubin-data-rights-antrag.md",
    "docs/auftrag/auftrag-igets-sftp-passwort-antrag.md",
    "docs/auftrag/auftrag-lisa-pathfinder-psd-antrag.md",
    "docs/auftrag/auftrag-flyby-doppler-rohdaten.md",
    "docs/auftrag/auftrag-sonden-rohdaten-anfragen.md",
    "docs/auftrag/gavo-dc-account-anfrage.md",
    "docs/reference/antares-konto-2026-09-05.md",
    "docs/reference/fink-konto-2026-09-05.md",
    "docs/handover/archiv/fink-konto-2026-09-05.md",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Exposure {
    Exposed,
    Absent,
    Unknown,
}

fn classify(code: Option<u16>) -> Exposure {
    match code {
        Some(200) => Exposure::Exposed,
        Some(404) => Exposure::Absent,
        _ => Exposure::Unknown,
    }
}

fn secret_key(key: &str) -> Option<String> {
    if let Ok(v) = env::var(key) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = fs::read_to_string(".secrets.local").ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn http_code(url: &str, token: Option<&str>) -> Option<u16> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-o")
        .arg("/dev/null")
        .arg("-w")
        .arg("%{http_code}")
        .arg("--max-time")
        .arg("20")
        .arg("-H")
        .arg("Accept: application/vnd.github+json");
    if let Some(t) = token {
        cmd.arg("-H").arg(format!("Authorization: Bearer {}", t));
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    String::from_utf8_lossy(&out.stdout).trim().parse().ok()
}

fn main() {
    let token = secret_key("GH_TOKEN");
    if token.is_none() {
        println!(
            "note: GH_TOKEN absent (env or .secrets.local) — running unauthenticated, the GitHub REST API carries a 60 requests/hour limit per source address"
        );
    }

    let mut incomplete = false;

    let mut reachable = 0usize;
    for sha in COMMITS {
        let url = format!("https://api.github.com/repos/{}/commits/{}", REPO, sha);
        match classify(http_code(&url, token.as_deref())) {
            Exposure::Exposed => reachable += 1,
            Exposure::Absent => {}
            Exposure::Unknown => incomplete = true,
        }
    }
    println!("commits reachable: {}/{}", reachable, COMMITS.len());

    let mut exposed_total = 0usize;
    for path in SENSITIVE_PATHS {
        let mut refs: Vec<&str> = Vec::new();
        for sha in COMMITS {
            let url = format!(
                "https://api.github.com/repos/{}/contents/{}?ref={}",
                REPO, path, sha
            );
            match classify(http_code(&url, token.as_deref())) {
                Exposure::Exposed => refs.push(sha),
                Exposure::Absent => {}
                Exposure::Unknown => incomplete = true,
            }
        }
        if refs.is_empty() {
            println!("{}\tabsent at all refs", path);
        } else {
            exposed_total += refs.len();
            println!("{}\t exposed_at={} ({})", path, refs.join(","), refs.len());
        }
    }

    let retrievable = exposed_total > 0;
    println!(
        "PII retrievable: {}",
        if retrievable { "yes" } else { "no" }
    );
    if retrievable {
        println!("exposed combinations: {}", exposed_total);
    }

    if incomplete {
        std::process::exit(1);
    }
    if retrievable {
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_classification() {
        assert_eq!(classify(Some(200)), Exposure::Exposed);
        assert_eq!(classify(Some(404)), Exposure::Absent);
        assert_eq!(classify(Some(403)), Exposure::Unknown);
        assert_eq!(classify(Some(500)), Exposure::Unknown);
        assert_eq!(classify(None), Exposure::Unknown);
    }

    #[test]
    fn constant_lists() {
        assert_eq!(COMMITS.len(), 15);
        assert_eq!(SENSITIVE_PATHS.len(), 10);
    }

    #[test]
    fn commit_shas_are_unique() {
        for (i, a) in COMMITS.iter().enumerate() {
            for b in &COMMITS[i + 1..] {
                assert_ne!(a, b);
            }
        }
    }

    #[test]
    fn sensitive_paths_are_unique() {
        for (i, a) in SENSITIVE_PATHS.iter().enumerate() {
            for b in &SENSITIVE_PATHS[i + 1..] {
                assert_ne!(a, b);
            }
        }
    }
}
