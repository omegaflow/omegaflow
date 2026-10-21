const BLOCKED: &[&str] = &[
    "brave",
    "exa",
    "tavily",
    "searchapi",
    "serpapi",
    "consensus",
    "gemini",
    "googlecse",
];

fn block_status(mode: &str, lines: &[String]) -> Option<String> {
    if !BLOCKED.contains(&mode) {
        return None;
    }
    let first = lines.first()?;
    let body = first.split_once("] ").map(|(_, body)| body).unwrap_or(first);
    if !body.starts_with("pending") {
        return None;
    }
    for status in ["400", "402", "403", "429", "432"] {
        if body.contains(&format!("HTTP {status}")) {
            return Some(status.to_string());
        }
    }
    None
}

pub fn apply(mode: &str, query: &str, lines: Vec<String>, max: usize) -> Vec<String> {
    let Some(status) = block_status(mode, &lines) else {
        return lines;
    };
    let alt = crate::four_get_lines(query, max);
    if !alt.iter().any(|l| l.starts_with("url ")) {
        return lines;
    }
    let label = format!("via: 4get ({mode} {status})");
    alt.into_iter()
        .map(|line| {
            if line.starts_with("url ") {
                format!("[{mode}] {line}\t{label}")
            } else {
                format!("[{mode}] {line}")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_a_blocked_status() {
        let lines = vec!["pending — brave HTTP 402 (billing)".to_string()];
        assert_eq!(block_status("brave", &lines), Some("402".to_string()));
        let witnessed = vec!["[brave] pending — Brave HTTP 402".to_string()];
        assert_eq!(block_status("brave", &witnessed), Some("402".to_string()));
    }

    #[test]
    fn ignores_a_working_arm() {
        let lines = vec!["url https://x\ttitle: x".to_string()];
        assert_eq!(block_status("brave", &lines), None);
    }

    #[test]
    fn ignores_a_mode_outside_the_map() {
        let lines = vec!["pending — openalex HTTP 429".to_string()];
        assert_eq!(block_status("openalex", &lines), None);
    }
}
