use std::process::Command;

fn main() {
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => {
            eprintln!("notes-notify: HOME is absent");
            return;
        }
    };
    let root = format!("{}/projects/omegaflow", home);
    let notes_file = format!("{}/docs/status/notizen.md", root);
    let notes = match std::fs::read_to_string(&notes_file) {
        Ok(s) => s,
        Err(_) => return,
    };
    if !has_unprocessed_notes(&notes) {
        return;
    }
    let state = state_dir();
    let sid_file = state.join("reports/active_session.φ");
    let sid = match std::fs::read_to_string(&sid_file) {
        Ok(s) => s.trim().to_string(),
        Err(_) => String::new(),
    };
    if sid.is_empty() {
        eprintln!(
            "notes-notify: no session registered in {}",
            sid_file.display()
        );
        return;
    }
    let _ = Command::new("opencode")
        .arg("run")
        .arg("Notizen ernten")
        .arg("-s")
        .arg(&sid)
        .arg("-m")
        .arg("omegaflow/deepseek-v4-pro")
        .arg("--dir")
        .arg(&root)
        .spawn();
}

fn state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    std::path::PathBuf::from(".")
}

fn has_unprocessed_notes(content: &str) -> bool {
    let mut in_eingang = false;
    for line in content.lines() {
        let t = line.trim();
        if t == "## Eingang" {
            in_eingang = true;
            continue;
        }
        if in_eingang && !t.is_empty() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_eingang_is_idle() {
        assert!(!has_unprocessed_notes("# Notizen\n\n## Eingang\n"));
    }

    #[test]
    fn a_note_line_is_dirty() {
        assert!(has_unprocessed_notes(
            "# Notizen\n\n## Eingang\n\n- eine notiz\n"
        ));
    }

    #[test]
    fn notes_before_eingang_do_not_count() {
        assert!(!has_unprocessed_notes("- alte notiz\n\n## Eingang\n"));
    }
}
