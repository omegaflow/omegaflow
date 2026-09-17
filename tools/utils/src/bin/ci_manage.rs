use omegaflow::archivar::{RetryPolicy, fetch_raw_with, load_env};
use omegaflow::json::{JsonVal, jnum, jpath_val, jstr, parse_json};

const API: &str = "https://api.github.com";
const REPO: &str = "omegaflow/omegaflow";

fn token() -> Option<String> {
    let env = load_env();
    for key in ["GH_TOKEN", "OMEGAFLOW_TOKEN", "GITHUB_TOKEN"] {
        match env.get(key) {
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

fn call(url: &str, body: Option<&str>, h: &[(String, String)]) -> Option<String> {
    fetch_raw_with(url, body, h, 60, RetryPolicy::Transient, 30)
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

fn list(limit: usize, h: &[(String, String)]) -> Option<()> {
    let url = format!("{API}/repos/{REPO}/actions/runs?per_page={limit}");
    let body = call(&url, None, h)?;
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

fn view(id: &str, h: &[(String, String)]) -> Option<()> {
    let url = format!("{API}/repos/{REPO}/actions/runs/{id}");
    let body = call(&url, None, h)?;
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

fn act(id: &str, action: &str, h: &[(String, String)]) -> Option<()> {
    let url = format!("{API}/repos/{REPO}/actions/runs/{id}/{action}");
    call(&url, Some(""), h)?;
    println!("{action} requested for run {id}");
    Some(())
}

fn usage() {
    eprintln!(
        "ci_manage — GitHub Actions runs (repo {REPO})\n\
         usage: ci_manage list [--limit N]   id/status/conclusion/attempt/workflow/started/updated\n\
         \x20      ci_manage view <run-id>    status/conclusion/attempt/sha/branch/times/url\n\
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
    let Some(tok) = token() else {
        eprintln!("ci_manage: GH_TOKEN absent in env/.secrets.local");
        std::process::exit(2);
    };
    let h = headers(&tok);
    let done = match cmd {
        "list" => {
            let mut limit = 20usize;
            if let Some(pos) = args.iter().position(|a| a == "--limit") {
                if let Some(v) = args.get(pos + 1).and_then(|s| s.parse::<usize>().ok()) {
                    limit = v;
                }
            }
            list(limit, &h).is_some()
        }
        "view" => match args.get(1) {
            Some(id) => view(id, &h).is_some(),
            None => {
                usage();
                false
            }
        },
        "cancel" => match args.get(1) {
            Some(id) => act(id, "cancel", &h).is_some(),
            None => {
                usage();
                false
            }
        },
        "rerun" => match args.get(1) {
            Some(id) => act(id, "rerun", &h).is_some(),
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
