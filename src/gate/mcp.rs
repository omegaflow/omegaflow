use std::collections::HashMap;

use crate::json::{JsonVal, parse_json};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct McpLeak {
    pub rule: &'static str,
    pub server: String,
    pub detail: String,
}

const WILDCARD_EGRESS: [&str; 5] = ["*", "https://*", "http://*", "*://*", "https*://*"];

pub fn mcp_leak_violations(cfg_json: &str) -> Vec<McpLeak> {
    let mut out = Vec::new();
    let Some(JsonVal::Obj(root)) = parse_json(cfg_json) else {
        return out;
    };
    let Some(JsonVal::Obj(servers)) = root.get("mcp") else {
        return out;
    };
    for (name, entry) in servers {
        let JsonVal::Obj(entry) = entry else {
            continue;
        };
        let kind = match entry.get("type") {
            Some(JsonVal::Str(s)) => s.as_str(),
            _ => "",
        };
        let argv = command_argv(entry);
        let has = |flag: &str| argv.iter().any(|a| a == flag);

        if argv.first().map(String::as_str) == Some("npx") {
            out.push(McpLeak {
                rule: "mcp-launch-unpinned",
                server: name.clone(),
                detail: format!("command[0]=npx launches {name} as an unpinned network fetch"),
            });
        }

        if let Some(pattern) = egress_wildcard(&argv) {
            out.push(McpLeak {
                rule: "mcp-egress-unbounded",
                server: name.clone(),
                detail: format!("{pattern} leaves the egress of {name} unbounded"),
            });
        }

        let fs_key = matches!(
            entry.get("allowUnrestrictedPaths"),
            Some(JsonVal::Bool(true))
        );
        if has("--allowUnrestrictedPaths") || fs_key {
            out.push(McpLeak {
                rule: "mcp-fs-unrestricted",
                server: name.clone(),
                detail: format!("{name} exposes unrestricted filesystem paths"),
            });
        }

        let conflict = (has("--autoConnect") && (has("--isolated") || has("--executablePath")))
            || (has("--executablePath") && (has("--browserUrl") || has("--wsEndpoint")));
        if conflict {
            out.push(McpLeak {
                rule: "mcp-yargs-conflict",
                server: name.clone(),
                detail: format!("{name} carries mutually exclusive yargs flags"),
            });
        }

        if let Some(path) = value_after(&argv, "--executablePath") {
            if !path.starts_with('/') {
                out.push(McpLeak {
                    rule: "mcp-executable-not-absolute",
                    server: name.clone(),
                    detail: format!("--executablePath {path} of {name} is not absolute"),
                });
            }
        }

        if kind == "local" && !entry.contains_key("cwd") {
            out.push(McpLeak {
                rule: "mcp-cwd-missing",
                server: name.clone(),
                detail: format!("local server {name} declares no cwd"),
            });
        }
    }
    out
}

fn command_argv(entry: &HashMap<String, JsonVal>) -> Vec<String> {
    match entry.get("command") {
        Some(JsonVal::Arr(items)) => items
            .iter()
            .filter_map(|v| match v {
                JsonVal::Str(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn value_after<'a>(argv: &'a [String], flag: &str) -> Option<&'a str> {
    let idx = argv.iter().position(|a| a == flag)?;
    argv.get(idx + 1).map(String::as_str)
}

fn egress_wildcard(argv: &[String]) -> Option<&str> {
    ["--allowedUrlPattern", "--blockedUrlPattern"]
        .into_iter()
        .filter_map(|flag| value_after(argv, flag))
        .find(|v| WILDCARD_EGRESS.contains(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CURRENT: &str = r#"{
      "mcp": {
        "chrome-devtools": {
          "type": "local",
          "command": ["npx", "-y", "chrome-devtools-mcp@1.9.0", "--no-usage-statistics", "--no-performance-crux", "--autoConnect"],
          "enabled": true,
          "timeout": 180000
        },
        "chrome-devtools-voices": {
          "type": "local",
          "command": ["npx", "-y", "chrome-devtools-mcp@1.9.0", "--headless", "--isolated", "--executablePath", "/usr/bin/google-chrome", "--allowedUrlPattern", "https://*", "--no-usage-statistics", "--no-performance-crux"],
          "enabled": true,
          "timeout": 180000
        }
      }
    }"#;

    const CONFORMANT: &str = r#"{
      "mcp": {
        "local-ok": {
          "type": "local",
          "command": ["/usr/local/bin/some-mcp", "--allowedUrlPattern", "https://example.com", "--executablePath", "/usr/bin/google-chrome"],
          "cwd": "/srv/workspace",
          "enabled": true
        },
        "remote-ok": {
          "type": "remote",
          "url": "https://example.com/mcp",
          "enabled": true
        }
      }
    }"#;

    fn pairs(leaks: &[McpLeak]) -> Vec<(String, &'static str)> {
        let mut got: Vec<(String, &'static str)> =
            leaks.iter().map(|l| (l.server.clone(), l.rule)).collect();
        got.sort();
        got
    }

    #[test]
    fn current_config_yields_the_measured_leak_set() {
        let got = pairs(&mcp_leak_violations(CURRENT));
        let mut want = vec![
            ("chrome-devtools".to_string(), "mcp-cwd-missing"),
            ("chrome-devtools".to_string(), "mcp-launch-unpinned"),
            ("chrome-devtools-voices".to_string(), "mcp-cwd-missing"),
            ("chrome-devtools-voices".to_string(), "mcp-egress-unbounded"),
            ("chrome-devtools-voices".to_string(), "mcp-launch-unpinned"),
        ];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn conformant_config_yields_no_leak() {
        assert!(mcp_leak_violations(CONFORMANT).is_empty());
    }

    #[test]
    fn absent_mcp_key_yields_no_leak() {
        assert!(mcp_leak_violations("{}").is_empty());
    }
}
