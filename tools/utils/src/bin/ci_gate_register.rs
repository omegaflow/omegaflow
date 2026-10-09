use omegaflow::archivar::{RetryPolicy, fetch_raw_with, load_env};
use omegaflow::json::{JsonVal, jpath_val, jstr, parse_json};
use std::fs;
use std::path::PathBuf;

const API: &str = "https://api.github.com";
const REPO: &str = "omegaflow/omegaflow";
const REGISTER: &str = "state/zustand/ci-gate.φ";
const REQUIRED_CHECK: &str = "subset";

fn token(keys: &[&str]) -> Option<String> {
    let env = load_env();
    for key in keys {
        match env.get(*key) {
            Some(v) if !v.is_empty() => return Some(v.clone()),
            _ => {}
        }
    }
    None
}

fn headers(tok: &str) -> Vec<(String, String)> {
    vec![
        ("Authorization".to_string(), format!("Bearer {tok}")),
        (
            "Accept".to_string(),
            "application/vnd.github+json".to_string(),
        ),
        ("X-GitHub-Api-Version".to_string(), "2022-11-28".to_string()),
    ]
}

fn read_headers() -> Option<Vec<(String, String)>> {
    token(&[
        "GH_SEARCH_TOKEN",
        "GH_TOKEN",
        "OMEGAFLOW_TOKEN",
        "GITHUB_TOKEN",
    ])
    .map(|t| headers(&t))
}

fn read_call(url: &str) -> Option<String> {
    let h = read_headers()?;
    fetch_raw_with(url, None, &h, RetryPolicy::Transient, 30)
}

fn repo_root() -> PathBuf {
    match std::env::var("OMEGAFLOW_REPO") {
        Ok(v) if !v.is_empty() => PathBuf::from(v),
        _ => PathBuf::from("."),
    }
}

fn git_head() -> Option<String> {
    let root = repo_root();
    let mut git = root.join(".git");
    if let Ok(text) = fs::read_to_string(&git) {
        if let Some(rest) = text.strip_prefix("gitdir: ") {
            git = PathBuf::from(rest.trim());
        }
    }
    let head = fs::read_to_string(git.join("HEAD")).ok()?;
    let head = head.trim();
    match head.strip_prefix("ref: ") {
        Some(reference) => fs::read_to_string(git.join(reference))
            .ok()
            .map(|v| v.trim().to_string()),
        None => Some(head.to_string()),
    }
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn utc_now() -> String {
    let secs = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(_) => return "clock-absent".to_string(),
    };
    let (y, m, d) = civil_from_days(secs.div_euclid(86400));
    let sod = secs.rem_euclid(86400);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}Z",
        y,
        m,
        d,
        sod / 3600,
        (sod % 3600) / 60
    )
}

fn check_state(check: &JsonVal) -> (String, String) {
    let name = match jstr(check, "name") {
        Some(v) => v,
        None => "?".to_string(),
    };
    let status = match jstr(check, "status") {
        Some(v) => v,
        None => "?".to_string(),
    };
    let state = if status == "completed" {
        match jstr(check, "conclusion") {
            Some(c) => c,
            None => "pending".to_string(),
        }
    } else {
        status
    };
    (name, state)
}

fn verdict_of_check(check_runs: &[JsonVal], required: &str) -> (String, Vec<(String, String)>) {
    let mut states: Vec<(String, String)> = check_runs.iter().map(check_state).collect();
    let decisive = states.iter().find(|(name, _)| name == required).cloned();
    let verdict = match decisive {
        None => "pending".to_string(),
        Some((_, state)) => match state.as_str() {
            "success" | "neutral" | "skipped" => "green".to_string(),
            "queued" | "in_progress" | "waiting" | "requested" | "pending" => "pending".to_string(),
            _ => "red".to_string(),
        },
    };
    states.sort();
    (verdict, states)
}

fn render_line(sha: &str, verdict: &str, states: &[(String, String)]) -> String {
    let shown: Vec<String> = states.iter().map(|(n, s)| format!("{n}={s}")).collect();
    let evidence = if shown.is_empty() {
        "no check-runs".to_string()
    } else {
        shown.join(", ")
    };
    let due = if verdict == "pending" {
        "run-end"
    } else {
        "head-change"
    };
    format!(
        "{sha} | {verdict} | {} ({evidence}) | {due} | ci_gate_register --sha {sha}",
        utc_now()
    )
}

fn update_register(sha: &str, line: &str) -> Result<(), String> {
    let path = repo_root().join(REGISTER);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let mut kept: Vec<String> = Vec::new();
    if let Ok(text) = fs::read_to_string(&path) {
        for existing in text.lines() {
            if existing.trim().is_empty() {
                continue;
            }
            let first = match existing.split('|').next() {
                Some(v) => v.trim(),
                None => "",
            };
            if first != sha {
                kept.push(existing.to_string());
            }
        }
    }
    let mut out = String::new();
    out.push_str(line);
    out.push('\n');
    for existing in kept {
        out.push_str(&existing);
        out.push('\n');
    }
    fs::write(&path, out).map_err(|e| format!("write {}: {e}", path.display()))
}

fn measure(sha: &str) -> Result<(String, Vec<(String, String)>), String> {
    let url = format!("{API}/repos/{REPO}/commits/{sha}/check-runs");
    let body = read_call(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let json = parse_json(&body).ok_or_else(|| format!("{url}: json unreadable"))?;
    let check_runs: Vec<JsonVal> = match jpath_val(&json, "check_runs") {
        Some(JsonVal::Arr(a)) => a.clone(),
        _ => Vec::new(),
    };
    Ok(verdict_of_check(&check_runs, REQUIRED_CHECK))
}

fn usage() {
    eprintln!(
        "ci_gate_register — the local per-SHA ci-gate verdict register (SHA -> green|red|pending)\n\
         usage:\n\
         \x20 ci_gate_register [--sha <sha>]\n\
         \x20 ci_gate_register --list"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--list") {
        match fs::read_to_string(repo_root().join(REGISTER)) {
            Ok(text) => print!("{text}"),
            Err(_) => println!("{REGISTER}: pending — no measurement stands"),
        }
        return;
    }
    let sha = match args.iter().position(|a| a == "--sha") {
        Some(i) => match args.get(i + 1) {
            Some(v) => v.clone(),
            None => {
                usage();
                std::process::exit(2);
            }
        },
        None => match git_head() {
            Some(v) => v,
            None => {
                eprintln!("no --sha given and .git/HEAD is unreadable");
                std::process::exit(2);
            }
        },
    };
    match measure(&sha) {
        Ok((verdict, states)) => {
            let line = render_line(&sha, &verdict, &states);
            if let Err(e) = update_register(&sha, &line) {
                eprintln!("ci_gate_register: {e}");
                std::process::exit(2);
            }
            println!("{line}");
        }
        Err(e) => {
            eprintln!("ci_gate_register: {e}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn runs(body: &str) -> Vec<JsonVal> {
        let json = parse_json(body).unwrap();
        match jpath_val(&json, "check_runs") {
            Some(JsonVal::Arr(a)) => a.clone(),
            _ => Vec::new(),
        }
    }

    #[test]
    fn verdict_reads_the_required_check() {
        let v = runs(
            r#"{"check_runs":[
                {"name":"subset","status":"completed","conclusion":"success"},
                {"name":"dropped-gate","status":"completed","conclusion":"success"}
            ]}"#,
        );
        assert_eq!(verdict_of_check(&v, "subset").0, "green");
    }

    #[test]
    fn verdict_is_pending_while_the_required_check_runs() {
        let v =
            runs(r#"{"check_runs":[{"name":"subset","status":"in_progress","conclusion":null}]}"#);
        assert_eq!(verdict_of_check(&v, "subset").0, "pending");
    }

    #[test]
    fn verdict_is_rot_on_a_failed_required_check() {
        let v = runs(
            r#"{"check_runs":[{"name":"subset","status":"completed","conclusion":"failure"}]}"#,
        );
        assert_eq!(verdict_of_check(&v, "subset").0, "red");
    }

    #[test]
    fn absent_required_check_reads_pending_never_green() {
        let v = runs(
            r#"{"check_runs":[{"name":"other","status":"completed","conclusion":"success"}]}"#,
        );
        assert_eq!(verdict_of_check(&v, "subset").0, "pending");
    }

    #[test]
    fn empty_check_runs_read_pending() {
        let v = runs(r#"{"total_count":0,"check_runs":[]}"#);
        assert_eq!(verdict_of_check(&v, "subset").0, "pending");
    }
}
