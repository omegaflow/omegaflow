use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::spectral::civil_from_days;

const TEMPLATE: &str = include_str!("handover_template.md");

pub fn today_civil() -> Option<(u32, u32, u32)> {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
    civil_from_days(secs / 86400)
}

pub fn date_str() -> String {
    match today_civil() {
        Some((y, m, d)) => format!("{:04}-{:02}-{:02}", y, m, d),
        None => String::new(),
    }
}

pub fn handover_markdown(reason: &str, state: &str, git_log: &str, git_status: &str) -> String {
    let log = if git_log.trim().is_empty() {
        "none".to_string()
    } else {
        git_log.to_string()
    };
    let status = if git_status.trim().is_empty() {
        "tree clean".to_string()
    } else {
        git_status.to_string()
    };
    TEMPLATE
        .replace("{date}", &date_str())
        .replace("{reason}", reason)
        .replace("{state}", state)
        .replace("{git_log}", &log)
        .replace("{git_status}", &status)
}

pub fn write_handover(root: &str, content: &str) -> std::io::Result<String> {
    let dir = format!("{}/docs/handover", root);
    std::fs::create_dir_all(&dir)?;
    let path = format!("{}/handover-{}-drift.md", dir, date_str());
    let mut f = std::fs::File::create(&path)?;
    f.write_all(content.as_bytes())?;
    Ok(path)
}

pub fn consent_word(text: &str) -> bool {
    let t = text.trim().to_lowercase();
    t == "go" || t == "go ahead"
}

pub fn spawn_command(content: &str, model: &str, root: &str) -> Vec<String> {
    vec![
        "opencode".to_string(),
        "run".to_string(),
        content.to_string(),
        "--title".to_string(),
        "nachfolge".to_string(),
        "-m".to_string(),
        model.to_string(),
        "--dir".to_string(),
        root.to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_carries_the_facts() {
        let md = handover_markdown(
            "context-fill threshold reached",
            "context_fill 0.92 ≥ 0.875 (model deepseek-v4-pro)",
            "c3e4220 gate\nf1e81b7 s03",
            " M src/friction.rs",
        );
        assert!(md.contains("context-fill threshold reached"));
        assert!(md.contains("context_fill 0.92"));
        assert!(md.contains("c3e4220"));
        assert!(md.contains("M src/friction.rs"));
        assert!(md.contains("class: handover"));
        assert!(md.contains("status: live"));
        assert!(md.contains("TODO.md"));
    }

    #[test]
    fn empty_git_names_absence() {
        let md = handover_markdown("r", "s", "", "");
        assert!(md.contains("none"));
        assert!(md.contains("tree clean"));
    }

    #[test]
    fn consent_is_the_word_go() {
        assert!(consent_word("go"));
        assert!(consent_word("Go"));
        assert!(consent_word("go ahead"));
        assert!(!consent_word("hello"));
        assert!(!consent_word(""));
        assert!(!consent_word("ja bitte"));
        assert!(!consent_word("logo"));
    }

    #[test]
    fn spawn_command_is_opencode_run() {
        let argv = spawn_command("the content", "omegaflow/deepseek-v4-pro", "/root");
        assert_eq!(argv[0], "opencode");
        assert_eq!(argv[1], "run");
        assert_eq!(argv[2], "the content");
        assert!(argv.contains(&"omegaflow/deepseek-v4-pro".to_string()));
        assert!(argv.contains(&"/root".to_string()));
    }

    #[test]
    fn write_handover_creates_the_file() {
        let root =
            std::env::temp_dir().join(format!("omegaflow_handover_test_{}", std::process::id()));
        let root = root.to_string_lossy().to_string();
        let path = write_handover(&root, "test content").unwrap();
        assert!(path.ends_with("-drift.md"));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "test content");
        let _ = std::fs::remove_dir_all(&root);
    }
}
