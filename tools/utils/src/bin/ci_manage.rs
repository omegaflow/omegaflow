use omegaflow::archivar::{RetryPolicy, fetch_raw_with, load_env};
use omegaflow::json::{JsonVal, jnum, jpath_val, jstr, parse_json};

const API: &str = "https://api.github.com";
const REPO: &str = "omegaflow/omegaflow";

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

fn write_headers() -> Option<Vec<(String, String)>> {
    token(&["GH_TOKEN", "OMEGAFLOW_TOKEN", "GITHUB_TOKEN"]).map(|t| headers(&t))
}

fn call(url: &str, body: Option<&str>, h: &[(String, String)]) -> Option<String> {
    fetch_raw_with(url, body, h, RetryPolicy::Transient, 30)
}

fn read_call(url: &str) -> Option<String> {
    if let Some(h) = read_headers() {
        if let Some(body) = call(url, None, &h) {
            return Some(body);
        }
    }
    let h = write_headers()?;
    call(url, None, &h)
}

fn field(r: &JsonVal, key: &str) -> String {
    match jstr(r, key) {
        Some(v) => v,
        None => "?".to_string(),
    }
}

fn num_field(r: &JsonVal, key: &str) -> String {
    match jnum(r, key) {
        Some(v) => format!("{}", v as i64),
        None => "?".to_string(),
    }
}

fn list(limit: usize) -> Option<()> {
    let url = format!("{API}/repos/{REPO}/actions/runs?per_page={limit}");
    let body = read_call(&url)?;
    let json = parse_json(&body)?;
    let runs = jpath_val(&json, "workflow_runs")?;
    let arr = match runs {
        JsonVal::Arr(a) => a,
        _ => return None,
    };
    for r in arr {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            num_field(r, "id"),
            field(r, "status"),
            field(r, "conclusion"),
            num_field(r, "run_attempt"),
            field(r, "name"),
            field(r, "run_started_at"),
            field(r, "updated_at")
        );
    }
    Some(())
}

fn view(id: &str) -> Option<()> {
    let url = format!("{API}/repos/{REPO}/actions/runs/{id}");
    let body = read_call(&url)?;
    let json = parse_json(&body)?;
    println!("id: {}", num_field(&json, "id"));
    println!("name: {}", field(&json, "name"));
    println!("status: {}", field(&json, "status"));
    println!("conclusion: {}", field(&json, "conclusion"));
    println!("attempt: {}", num_field(&json, "run_attempt"));
    println!("head_sha: {}", field(&json, "head_sha"));
    println!("head_branch: {}", field(&json, "head_branch"));
    println!("created_at: {}", field(&json, "created_at"));
    println!("run_started_at: {}", field(&json, "run_started_at"));
    println!("updated_at: {}", field(&json, "updated_at"));
    println!("html_url: {}", field(&json, "html_url"));
    Some(())
}

fn jobs_of(json: &JsonVal) -> Vec<(String, String, String)> {
    let Some(JsonVal::Arr(jobs)) = jpath_val(json, "jobs") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for j in jobs {
        let Some(id) = jnum(j, "id") else {
            continue;
        };
        out.push((
            format!("{}", id as i64),
            field(j, "name"),
            field(j, "conclusion"),
        ));
    }
    out
}

fn red_jobs(jobs: &[(String, String, String)], all: bool) -> Vec<(String, String, String)> {
    if all {
        return jobs.to_vec();
    }
    jobs.iter()
        .filter(|(_, _, conclusion)| {
            matches!(conclusion.as_str(), "failure" | "cancelled" | "timed_out")
        })
        .cloned()
        .collect()
}

fn log(id: &str, all: bool) -> Option<()> {
    let url = format!("{API}/repos/{REPO}/actions/runs/{id}/jobs?per_page=100");
    let body = read_call(&url)?;
    let json = parse_json(&body)?;
    let jobs = jobs_of(&json);
    let chosen = red_jobs(&jobs, all);
    if chosen.is_empty() {
        println!(
            "run {id}: no red job to log ({} jobs, all green) — --all prints every job",
            jobs.len()
        );
        return Some(());
    }
    for (job_id, name, conclusion) in chosen {
        println!("=== job {job_id} · {name} · {conclusion} ===");
        let log_url = format!("{API}/repos/{REPO}/actions/jobs/{job_id}/logs");
        match read_call(&log_url) {
            Some(text) => print!("{text}"),
            None => println!("(no log returned for job {job_id})"),
        }
    }
    Some(())
}

fn step_state(j: &JsonVal) -> String {
    let Some(JsonVal::Arr(steps)) = jpath_val(j, "steps") else {
        return "—".to_string();
    };
    let mut last_done: Option<String> = None;
    let mut first_pending: Option<String> = None;
    for s in steps {
        let Some(name) = jstr(s, "name") else {
            continue;
        };
        let Some(status) = jstr(s, "status") else {
            continue;
        };
        match status.as_str() {
            "in_progress" => return format!("→ {name}"),
            "completed" => last_done = Some(name),
            "pending" | "queued" => {
                if first_pending.is_none() {
                    first_pending = Some(name);
                }
            }
            _ => {}
        }
    }
    match (last_done, first_pending) {
        (Some(n), _) => format!("✓ {n}"),
        (None, Some(n)) => format!("· {n}"),
        (None, None) => "—".to_string(),
    }
}

fn jobs(id: &str) -> Option<()> {
    let url = format!("{API}/repos/{REPO}/actions/runs/{id}/jobs?per_page=100");
    let body = read_call(&url)?;
    let json = parse_json(&body)?;
    let Some(JsonVal::Arr(arr)) = jpath_val(&json, "jobs") else {
        return None;
    };
    println!("run {id}");
    for j in arr {
        println!(
            "{}\t{}\t{}\t{}",
            field(j, "name"),
            field(j, "status"),
            field(j, "conclusion"),
            step_state(j)
        );
    }
    Some(())
}

fn current_step(id: &str) -> String {
    let url = format!("{API}/repos/{REPO}/actions/runs/{id}/jobs?per_page=100");
    let Some(body) = read_call(&url) else {
        return "?".to_string();
    };
    let Some(json) = parse_json(&body) else {
        return "?".to_string();
    };
    let Some(JsonVal::Arr(arr)) = jpath_val(&json, "jobs") else {
        return "?".to_string();
    };
    for j in arr {
        if jstr(j, "status").as_deref() == Some("in_progress") {
            return format!("{} {}", field(j, "name"), step_state(j));
        }
    }
    "—".to_string()
}

fn status(limit: usize) -> Option<()> {
    let url = format!("{API}/repos/{REPO}/actions/runs?per_page={limit}");
    let body = read_call(&url)?;
    let json = parse_json(&body)?;
    let Some(JsonVal::Arr(runs)) = jpath_val(&json, "workflow_runs") else {
        return None;
    };
    for r in runs {
        let id = num_field(r, "id");
        let st = field(r, "status");
        let step = if st == "completed" {
            "—".to_string()
        } else {
            current_step(&id)
        };
        println!(
            "{id}\t{}\t{st}\t{}\t{}\t{step}",
            field(r, "name"),
            field(r, "conclusion"),
            field(r, "updated_at")
        );
    }
    Some(())
}

fn act(id: &str, action: &str) -> Option<()> {
    let url = format!("{API}/repos/{REPO}/actions/runs/{id}/{action}");
    let h = write_headers()?;
    call(&url, Some(""), &h)?;
    println!("{action} requested for run {id}");
    Some(())
}

fn usage() {
    eprintln!(
        "ci_manage — GitHub Actions runs (repo {REPO})\n\
         usage: ci_manage list [--limit N]   id/status/conclusion/attempt/workflow/started/updated\n\
         \x20      ci_manage status [--limit N] overview + current step of every running run\n\
         \x20      ci_manage view <run-id>    status/conclusion/attempt/sha/branch/times/url\n\
         \x20      ci_manage jobs <run-id>... job status + current step (what is happening now)\n\
         \x20      ci_manage log <run-id> [--all]  job logs (default: the red jobs)\n\
         \x20      ci_manage cancel <run-id>  request cancellation (keeps the log)\n\
         \x20      ci_manage rerun <run-id>   request a rerun (attempt +1)"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = match args.first() {
        Some(c) => c.as_str(),
        None => {
            usage();
            std::process::exit(2);
        }
    };
    if read_headers().is_none() && write_headers().is_none() {
        eprintln!("ci_manage: no GH_TOKEN/GH_SEARCH_TOKEN in env/.secrets.local");
        std::process::exit(2);
    }
    let done = match cmd {
        "list" => {
            let mut limit = 20usize;
            if let Some(pos) = args.iter().position(|a| a == "--limit") {
                if let Some(v) = args.get(pos + 1).and_then(|s| s.parse::<usize>().ok()) {
                    limit = v;
                }
            }
            list(limit).is_some()
        }
        "status" => {
            let mut limit = 20usize;
            if let Some(pos) = args.iter().position(|a| a == "--limit") {
                if let Some(v) = args.get(pos + 1).and_then(|s| s.parse::<usize>().ok()) {
                    limit = v;
                }
            }
            status(limit).is_some()
        }
        "view" => match args.get(1) {
            Some(id) => view(id).is_some(),
            None => {
                usage();
                false
            }
        },
        "log" => match args.get(1) {
            Some(id) => {
                let all = args.iter().any(|a| a == "--all");
                log(id, all).is_some()
            }
            None => {
                usage();
                false
            }
        },
        "jobs" => {
            let ids: Vec<String> = args.iter().skip(1).cloned().collect();
            if ids.is_empty() {
                usage();
                false
            } else {
                let mut ok = true;
                for id in &ids {
                    ok &= jobs(id).is_some();
                }
                ok
            }
        }
        "cancel" => match args.get(1) {
            Some(id) => act(id, "cancel").is_some(),
            None => {
                usage();
                false
            }
        },
        "rerun" => match args.get(1) {
            Some(id) => act(id, "rerun").is_some(),
            None => {
                usage();
                false
            }
        },
        _ => {
            usage();
            false
        }
    };
    if !done {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JOBS: &str = r#"{"total_count":3,"jobs":[
        {"id":11,"name":"check","conclusion":"success"},
        {"id":22,"name":"test","conclusion":"failure"},
        {"id":33,"name":"build","conclusion":null}
    ]}"#;

    #[test]
    fn red_jobs_selects_only_red_conclusions() {
        let json = parse_json(JOBS).unwrap();
        let jobs = jobs_of(&json);
        assert_eq!(jobs.len(), 3);
        let chosen = red_jobs(&jobs, false);
        assert_eq!(chosen.len(), 1);
        assert_eq!(chosen[0].0, "22");
        assert_eq!(chosen[0].1, "test");
    }

    #[test]
    fn red_jobs_all_returns_every_job() {
        let json = parse_json(JOBS).unwrap();
        let jobs = jobs_of(&json);
        assert_eq!(red_jobs(&jobs, true).len(), 3);
    }

    #[test]
    fn jobs_of_skips_a_job_without_id() {
        let json = parse_json(r#"{"jobs":[{"name":"x","conclusion":"failure"}]}"#).unwrap();
        assert!(jobs_of(&json).is_empty());
    }

    #[test]
    fn step_state_names_the_current_position() {
        let json = parse_json(
            r#"{"jobs":[
                {"name":"a","steps":[{"name":"s1","status":"completed"},{"name":"s2","status":"in_progress"},{"name":"s3","status":"pending"}]},
                {"name":"b","steps":[{"name":"t1","status":"completed"},{"name":"t2","status":"completed"}]},
                {"name":"c","steps":[{"name":"u1","status":"pending"}]}
            ]}"#,
        )
        .unwrap();
        let JsonVal::Arr(arr) = jpath_val(&json, "jobs").unwrap() else {
            panic!("jobs array");
        };
        assert_eq!(step_state(&arr[0]), "→ s2");
        assert_eq!(step_state(&arr[1]), "✓ t2");
        assert_eq!(step_state(&arr[2]), "· u1");
    }
}
