use std::collections::HashMap;
use std::fs;

const DENY_MARKERS: [&str; 8] = [
    "denied",
    "not allowed",
    "auto-rejected",
    "auto-reject",
    "permission required",
    "permission denied",
    "refused",
    "schema error",
];

pub const ALLOW_AFTER: u64 = 3;
pub const DENY_AFTER: u64 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ToolState {
    Allowed,
    Denied,
    Unlock,
}

impl ToolState {
    pub fn word(self) -> &'static str {
        match self {
            ToolState::Allowed => "allowed",
            ToolState::Denied => "denied",
            ToolState::Unlock => "unlock",
        }
    }

    fn parse(s: &str) -> Option<ToolState> {
        match s {
            "allowed" => Some(ToolState::Allowed),
            "denied" => Some(ToolState::Denied),
            "unlock" => Some(ToolState::Unlock),
            _ => None,
        }
    }
}

pub struct Entry {
    pub state: ToolState,
    pub allow: u64,
    pub deny: u64,
    pub signal: String,
    pub written: bool,
}

pub struct ToolPerm {
    pub path: String,
    pub tools: HashMap<String, Entry>,
}

impl ToolPerm {
    pub fn load(path: &str) -> ToolPerm {
        let mut perm = ToolPerm {
            path: path.to_string(),
            tools: HashMap::new(),
        };
        if let Ok(text) = fs::read_to_string(path) {
            for line in text.lines() {
                let t = line.trim();
                if t.is_empty() || t.starts_with('#') {
                    continue;
                }
                let tokens: Vec<&str> = t.split_whitespace().collect();
                if tokens.len() < 5 || tokens[0] != "tool" {
                    continue;
                }
                let Some(state) = ToolState::parse(tokens[2]) else {
                    continue;
                };
                let Ok(allow) = tokens[3].parse::<u64>() else {
                    continue;
                };
                let Ok(deny) = tokens[4].parse::<u64>() else {
                    continue;
                };
                let signal = tokens.get(5..).unwrap_or(&[]).join(" ");
                perm.tools.insert(
                    tokens[1].to_string(),
                    Entry {
                        state,
                        allow,
                        deny,
                        signal,
                        written: true,
                    },
                );
            }
        }
        perm
    }

    pub fn observe(&mut self, name: &str, result: &str) {
        let e = self.tools.entry(name.to_string()).or_insert(Entry {
            state: ToolState::Unlock,
            allow: 0,
            deny: 0,
            signal: String::new(),
            written: false,
        });
        if e.written {
            return;
        }
        let lower = result.to_lowercase();
        let denied = DENY_MARKERS.iter().any(|m| lower.contains(m));
        if denied {
            e.deny += 1;
            e.signal = "the result carries the refusal".to_string();
        } else {
            e.allow += 1;
            e.signal = "the result carries a clean run".to_string();
        }
        if e.deny >= DENY_AFTER {
            e.state = ToolState::Denied;
        } else if e.allow >= ALLOW_AFTER {
            e.state = ToolState::Allowed;
        }
    }

    pub fn state_of(&self, name: &str) -> ToolState {
        self.tools
            .get(name)
            .map(|e| e.state)
            .unwrap_or(ToolState::Unlock)
    }

    pub fn system_message(&self) -> Option<String> {
        let mut lines: Vec<String> = Vec::new();
        let mut tools: Vec<(&String, &Entry)> = self.tools.iter().collect();
        tools.sort_by(|a, b| a.0.cmp(b.0));
        for (name, e) in tools {
            match e.state {
                ToolState::Denied => lines.push(format!(
                    "{}: denied — the machine does not call this tool",
                    name
                )),
                ToolState::Unlock => {
                    lines.push(format!("{}: unlock — the operator decides first", name))
                }
                ToolState::Allowed => {}
            }
        }
        if lines.is_empty() {
            return None;
        }
        let mut content =
            String::from("Tool-Erlaubnis — Zustand aus dem Ledger, nie aus dem Verlauf:\n");
        content.push_str(&lines.join("\n"));
        Some(content)
    }

    pub fn save(&self) {
        let mut lines: Vec<String> = Vec::new();
        if let Ok(text) = fs::read_to_string(&self.path) {
            for line in text.lines() {
                if line.trim_start().starts_with('#') {
                    lines.push(line.to_string());
                }
            }
        }
        let mut tools: Vec<(&String, &Entry)> = self.tools.iter().collect();
        tools.sort_by(|a, b| a.0.cmp(b.0));
        for (name, e) in tools {
            lines.push(format!(
                "tool {} {} {} {} {}",
                name,
                e.state.word(),
                e.allow,
                e.deny,
                e.signal
            ));
        }
        let content = lines.join("\n") + "\n";
        if let Some(dir) = std::path::Path::new(&self.path).parent() {
            fs::create_dir_all(dir).ok();
        }
        let _ = fs::write(&self.path, content);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(tag: &str) -> String {
        std::env::temp_dir()
            .join(format!("llm_tool_perm_{}_{}", std::process::id(), tag))
            .to_string_lossy()
            .to_string()
    }

    #[test]
    fn load_empty_is_unlock() {
        let path = temp_path("empty");
        let _ = fs::remove_file(&path);
        let perm = ToolPerm::load(&path);
        assert_eq!(perm.state_of("git"), ToolState::Unlock);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn repeated_clean_runs_allow() {
        let mut perm = ToolPerm::load(&temp_path("allow"));
        for _ in 0..ALLOW_AFTER {
            perm.observe("bash", "the command finished cleanly");
        }
        assert_eq!(perm.state_of("bash"), ToolState::Allowed);
    }

    #[test]
    fn repeated_denials_deny() {
        let mut perm = ToolPerm::load(&temp_path("deny"));
        for _ in 0..DENY_AFTER {
            perm.observe("git", "the tool call was auto-rejected");
        }
        assert_eq!(perm.state_of("git"), ToolState::Denied);
    }

    #[test]
    fn single_denial_stays_unlock() {
        let mut perm = ToolPerm::load(&temp_path("single"));
        perm.observe("git", "the tool call was auto-rejected");
        assert_eq!(perm.state_of("git"), ToolState::Unlock);
    }

    #[test]
    fn denial_dominates_grants() {
        let mut perm = ToolPerm::load(&temp_path("dom"));
        for _ in 0..ALLOW_AFTER {
            perm.observe("git", "the command finished cleanly");
        }
        assert_eq!(perm.state_of("git"), ToolState::Allowed);
        for _ in 0..DENY_AFTER {
            perm.observe("git", "the tool call was denied");
        }
        assert_eq!(perm.state_of("git"), ToolState::Denied);
    }

    #[test]
    fn hand_written_state_is_pinned() {
        let path = temp_path("pinned");
        std::fs::write(&path, "tool weave denied 0 0 the operator said no\n").unwrap();
        let mut perm = ToolPerm::load(&path);
        for _ in 0..ALLOW_AFTER {
            perm.observe("weave", "the command finished cleanly");
        }
        assert_eq!(perm.state_of("weave"), ToolState::Denied);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn system_message_lists_denied_and_unlock_only() {
        let path = temp_path("msg");
        std::fs::write(
            &path,
            "tool bash allowed 3 0 clean\n\
             tool git denied 0 2 refused\n\
             tool weave unlock 1 0 one clean run\n",
        )
        .unwrap();
        let perm = ToolPerm::load(&path);
        let msg = perm.system_message().unwrap();
        assert!(msg.contains("git: denied"));
        assert!(msg.contains("weave: unlock"));
        assert!(!msg.contains("bash"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn clean_perm_has_no_message() {
        let path = temp_path("quiet");
        let _ = fs::remove_file(&path);
        let perm = ToolPerm::load(&path);
        assert!(perm.system_message().is_none());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn save_and_load_roundtrip() {
        let path = temp_path("roundtrip");
        let _ = fs::remove_file(&path);
        let mut perm = ToolPerm::load(&path);
        for _ in 0..DENY_AFTER {
            perm.observe("git", "the tool call was auto-rejected");
        }
        perm.save();
        let back = ToolPerm::load(&path);
        assert_eq!(back.state_of("git"), ToolState::Denied);
        assert_eq!(back.tools.get("git").unwrap().deny, DENY_AFTER);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn save_preserves_operator_comments() {
        let path = temp_path("comments_save");
        std::fs::write(
            &path,
            "# the operator's word rules\n\
             tool bash allowed 3 0 clean\n",
        )
        .unwrap();
        let mut perm = ToolPerm::load(&path);
        for _ in 0..DENY_AFTER {
            perm.observe("git", "the tool call was auto-rejected");
        }
        perm.save();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("# the operator's word rules"));
        assert!(text.contains("tool git denied"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn malformed_count_is_fehlt_not_zero() {
        let path = temp_path("malformed");
        std::fs::write(&path, "tool bash allowed nope 0 clean\n").unwrap();
        let perm = ToolPerm::load(&path);
        assert!(!perm.tools.contains_key("bash"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn ledger_skips_comments() {
        let path = temp_path("comments");
        std::fs::write(
            &path,
            "# the operator's word rules\n\
             tool bash allowed 3 0 clean\n",
        )
        .unwrap();
        let perm = ToolPerm::load(&path);
        assert_eq!(perm.state_of("bash"), ToolState::Allowed);
        assert_eq!(perm.tools.len(), 1);
        let _ = fs::remove_file(&path);
    }
}
