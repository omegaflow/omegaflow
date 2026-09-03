use std::collections::HashMap;
use std::fs;

use crate::json::JsonVal;

pub struct State {
    pub session: String,
    pub model: String,
    pub head: String,
    pub turns: u64,
    pub violations: u64,
    pub fill: Option<f64>,
}

impl State {
    pub fn fresh(session: &str) -> State {
        State {
            session: session.to_string(),
            model: String::new(),
            head: String::new(),
            turns: 0,
            violations: 0,
            fill: None,
        }
    }
}

pub fn read(path: &str) -> Option<State> {
    let text = fs::read_to_string(path).ok()?;
    let mut s = State::fresh("");
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let Some((key, val)) = t.split_once(' ') else {
            continue;
        };
        match key {
            "session" => s.session = val.to_string(),
            "model" => s.model = val.to_string(),
            "head" => s.head = val.to_string(),
            "turns" => {
                let Ok(v) = val.parse::<u64>() else { continue };
                s.turns = v;
            }
            "violations" => {
                let Ok(v) = val.parse::<u64>() else { continue };
                s.violations = v;
            }
            "fill" => {
                s.fill = val
                    .parse::<f64>()
                    .ok()
                    .filter(|f| f.is_finite() && *f >= 0.0)
            }
            _ => {}
        }
    }
    Some(s)
}

pub fn write(path: &str, s: &State) -> std::io::Result<()> {
    let fill = match s.fill {
        Some(f) => format!("{:.4}", f),
        None => "pending".to_string(),
    };
    let content = format!(
        "session {}\nmodel {}\nhead {}\nturns {}\nviolations {}\nfill {}\n",
        s.session, s.model, s.head, s.turns, s.violations, fill
    );
    if let Some(dir) = std::path::Path::new(path).parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(path, content)
}

fn shown(value: &str) -> String {
    if value.is_empty() {
        "pending".to_string()
    } else {
        value.to_string()
    }
}

pub fn system_message(s: &State) -> JsonVal {
    let fill = match s.fill {
        Some(f) => format!("{:.3}", f),
        None => "pending".to_string(),
    };
    let content = format!(
        "Zustand — vom Granit gelesen, nie vom Verlauf:\nsession: {}\nmodel: {}\nhead: {}\nturns: {}\nviolations: {}\nfill: {}",
        shown(&s.session),
        shown(&s.model),
        shown(&s.head),
        s.turns,
        s.violations,
        fill,
    );
    let mut m = HashMap::new();
    m.insert("role".to_string(), JsonVal::Str("system".to_string()));
    m.insert("content".to_string(), JsonVal::Str(content));
    JsonVal::Obj(m)
}

pub fn inject_after_axioms(parsed: &mut JsonVal, s: &State) {
    let JsonVal::Obj(map) = parsed else {
        return;
    };
    let Some(JsonVal::Arr(msgs)) = map.get_mut("messages") else {
        return;
    };
    let idx = msgs.len().min(1);
    msgs.insert(idx, system_message(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_state_is_empty() {
        let s = State::fresh("operator");
        assert_eq!(s.session, "operator");
        assert_eq!(s.turns, 0);
        assert_eq!(s.violations, 0);
        assert!(s.fill.is_none());
        assert!(s.model.is_empty());
    }

    #[test]
    fn write_and_read_roundtrip() {
        let root =
            std::env::temp_dir().join(format!("omegaflow_state_test_{}", std::process::id()));
        let path = root.join("phi").join("reports").join("gate_state.φ");
        let path = path.to_string_lossy().to_string();
        let s = State {
            session: "operator".to_string(),
            model: "omegaflow/deepseek-v4-pro".to_string(),
            head: "d33d358".to_string(),
            turns: 42,
            violations: 7,
            fill: Some(0.42),
        };
        write(&path, &s).unwrap();
        let back = read(&path).unwrap();
        assert_eq!(back.session, "operator");
        assert_eq!(back.model, "omegaflow/deepseek-v4-pro");
        assert_eq!(back.head, "d33d358");
        assert_eq!(back.turns, 42);
        assert_eq!(back.violations, 7);
        assert_eq!(back.fill, Some(0.42));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn missing_file_is_none() {
        let path =
            std::env::temp_dir().join(format!("omegaflow_state_missing_{}", std::process::id()));
        assert!(read(&path.to_string_lossy()).is_none());
    }

    #[test]
    fn absent_fill_is_pending_not_zero() {
        let root =
            std::env::temp_dir().join(format!("omegaflow_state_fill_{}", std::process::id()));
        let path = root.to_string_lossy().to_string();
        let s = State::fresh("operator");
        write(&path, &s).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.contains("fill pending"));
        let back = read(&path).unwrap();
        assert!(back.fill.is_none());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn system_message_carries_the_facts() {
        let s = State {
            session: "operator".to_string(),
            model: "omegaflow/deepseek-v4-pro".to_string(),
            head: "d33d358".to_string(),
            turns: 42,
            violations: 7,
            fill: Some(0.42),
        };
        let JsonVal::Obj(m) = system_message(&s) else {
            panic!("not an object");
        };
        let role = m.get("role").and_then(|v| match v {
            JsonVal::Str(s) => Some(s.as_str()),
            _ => None,
        });
        assert_eq!(role, Some("system"));
        let content = match m.get("content") {
            Some(JsonVal::Str(c)) => c.as_str(),
            _ => panic!("content absent"),
        };
        assert!(content.contains("operator"));
        assert!(content.contains("d33d358"));
        assert!(content.contains("turns: 42"));
        assert!(content.contains("fill: 0.420"));
    }

    #[test]
    fn malformed_counter_is_fehlt_not_zero() {
        let root =
            std::env::temp_dir().join(format!("omegaflow_state_badcount_{}", std::process::id()));
        let path = root.join("phi").join("reports").join("gate_state.φ");
        let path = path.to_string_lossy().to_string();
        std::fs::create_dir_all(std::path::Path::new(&path).parent().unwrap()).unwrap();
        std::fs::write(
            &path,
            "session x\nmodel y\nhead h\nturns 42\nturns nope\nviolations 7\n",
        )
        .unwrap();
        let s = read(&path).unwrap();
        assert_eq!(s.turns, 42);
        assert_eq!(s.violations, 7);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn absent_state_reads_pending() {
        let s = State::fresh("operator");
        let JsonVal::Obj(m) = system_message(&s) else {
            panic!("not an object");
        };
        let content = match m.get("content") {
            Some(JsonVal::Str(c)) => c.as_str(),
            _ => panic!("content absent"),
        };
        assert!(content.contains("model: pending"));
        assert!(content.contains("fill: pending"));
    }

    #[test]
    fn inject_places_state_after_axioms() {
        let mut body = crate::json::parse_json(
            r#"{"model":"x","messages":[{"role":"system","content":"axiom"}]}"#,
        )
        .unwrap();
        inject_after_axioms(&mut body, &State::fresh("operator"));
        let JsonVal::Obj(map) = &body else {
            panic!("not an object");
        };
        let JsonVal::Arr(msgs) = map.get("messages").unwrap() else {
            panic!("messages absent");
        };
        assert_eq!(msgs.len(), 2);
        let JsonVal::Obj(m1) = &msgs[1] else {
            panic!("second message absent");
        };
        let content = match m1.get("content") {
            Some(JsonVal::Str(c)) => c.as_str(),
            _ => panic!("content absent"),
        };
        assert!(content.contains("Zustand"));
    }
}
