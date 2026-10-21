use std::collections::HashMap;
use std::process::Command;
use std::sync::{Mutex, OnceLock};

const PREFIX: &str = "OMEGAFLOW_";
const SUFFIX: &str = "_CMD";

static CACHE: OnceLock<Mutex<HashMap<String, Option<String>>>> = OnceLock::new();

pub(crate) fn env_name(key: &str) -> String {
    format!("{PREFIX}{key}{SUFFIX}")
}

pub(crate) fn configured(key: &str) -> Option<String> {
    std::env::var(env_name(key))
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub(crate) fn cached(key: &str) -> Option<String> {
    let cmd = configured(key)?;
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    cached_with(cache, &env_name(key), &cmd)
}

pub(crate) fn refresh_env(env: &mut HashMap<String, String>) {
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let keys: Vec<String> = env
        .keys()
        .filter_map(|name| {
            name.strip_prefix(PREFIX)
                .and_then(|rest| rest.strip_suffix(SUFFIX))
                .map(str::to_string)
        })
        .filter(|key| !key.is_empty())
        .collect();
    for key in keys {
        let name = env_name(&key);
        let cmd = match env.get(&name).map(|value| value.trim().to_string()) {
            Some(cmd) if !cmd.is_empty() => cmd,
            _ => continue,
        };
        match cached_with(cache, &name, &cmd) {
            Some(value) => {
                env.insert(key, value);
            }
            None => {
                env.remove(&key);
            }
        }
    }
}

fn cached_with(
    cache: &Mutex<HashMap<String, Option<String>>>,
    label: &str,
    cmd: &str,
) -> Option<String> {
    if let Ok(map) = cache.lock() {
        if let Some(value) = map.get(label) {
            return value.clone();
        }
    }
    let value = run(label, cmd);
    if let Ok(mut map) = cache.lock() {
        map.insert(label.to_string(), value.clone());
    }
    value
}

fn run(label: &str, cmd: &str) -> Option<String> {
    match Command::new("sh").arg("-c").arg(cmd).output() {
        Ok(out) if out.status.success() => {
            let token = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if token.is_empty() {
                eprintln!("token: {label} writes no token");
                None
            } else {
                Some(token)
            }
        }
        Ok(out) => {
            eprintln!("{}", failure_line(label, out.status.code()));
            None
        }
        Err(err) => {
            eprintln!("token: {label} does not start: {}", err.kind());
            None
        }
    }
}

fn failure_line(label: &str, code: Option<i32>) -> String {
    match code {
        Some(code) => format!("token: {label} exits {code}"),
        None => format!("token: {label} ends on a signal"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cache() -> Mutex<HashMap<String, Option<String>>> {
        Mutex::new(HashMap::new())
    }

    #[test]
    fn env_name_is_derived_from_the_key() {
        assert_eq!(env_name("CORE_API_KEY"), "OMEGAFLOW_CORE_API_KEY_CMD");
        assert_eq!(env_name("NASA_ADS_TOKEN"), "OMEGAFLOW_NASA_ADS_TOKEN_CMD");
    }

    #[test]
    fn command_stdout_is_trimmed_to_the_token() {
        let cache = cache();
        assert_eq!(
            cached_with(&cache, "OMEGAFLOW_TRIM_CMD", "printf '  test-token \\n\\n'").as_deref(),
            Some("test-token")
        );
    }

    #[test]
    fn an_empty_command_result_is_absent_and_not_a_silent_token() {
        let cache = cache();
        assert!(cached_with(&cache, "OMEGAFLOW_EMPTY_CMD", "printf '   \\n'").is_none());
    }

    #[test]
    fn the_result_is_cached_for_the_process() {
        let path = std::env::temp_dir().join(format!(
            "omegaflow-token-cmd-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let cmd = format!("echo run >> {}; printf cached-token", path.display());
        let cache = cache();
        assert_eq!(
            cached_with(&cache, "OMEGAFLOW_CACHE_CMD", &cmd).as_deref(),
            Some("cached-token")
        );
        assert_eq!(
            cached_with(&cache, "OMEGAFLOW_CACHE_CMD", &cmd).as_deref(),
            Some("cached-token")
        );
        let runs = std::fs::read_to_string(&path).unwrap_or_default();
        let _ = std::fs::remove_file(&path);
        assert_eq!(runs.lines().count(), 1);
    }

    #[test]
    fn refresh_env_fills_the_key_and_failure_removes_a_stale_value() {
        let mut env = HashMap::new();
        env.insert(
            "OMEGAFLOW_G_TEST_TOKEN_CMD".to_string(),
            "printf refreshed-token".to_string(),
        );
        refresh_env(&mut env);
        assert_eq!(
            env.get("G_TEST_TOKEN").map(String::as_str),
            Some("refreshed-token")
        );

        let mut stale = HashMap::new();
        stale.insert("G_FAILED_TOKEN".to_string(), "stale-token".to_string());
        stale.insert(
            "OMEGAFLOW_G_FAILED_TOKEN_CMD".to_string(),
            "printf leaked-token; exit 7".to_string(),
        );
        refresh_env(&mut stale);
        assert!(!stale.contains_key("G_FAILED_TOKEN"));
    }

    #[test]
    fn failure_messages_name_the_command_and_never_the_output() {
        let line = failure_line("OMEGAFLOW_CORE_API_KEY_CMD", Some(7));
        assert!(line.contains("OMEGAFLOW_CORE_API_KEY_CMD"));
        assert!(line.contains("7"));
        assert!(!line.contains("test-token"));
    }
}
