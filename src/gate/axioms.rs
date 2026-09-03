use std::collections::HashMap;

use crate::json::JsonVal;

pub fn load(root: &str) -> String {
    std::fs::read_to_string(format!("{root}/docs/granit.md")).unwrap_or_default()
}

pub fn system_message(axioms: &str) -> JsonVal {
    let mut msg = HashMap::new();
    msg.insert("role".to_string(), JsonVal::Str("system".to_string()));
    msg.insert(
        "content".to_string(),
        JsonVal::Str(axioms.trim().to_string()),
    );
    JsonVal::Obj(msg)
}

pub fn inject(parsed: &mut JsonVal, root: &str) {
    let JsonVal::Obj(map) = parsed else {
        return;
    };
    let Some(JsonVal::Arr(msgs)) = map.get_mut("messages") else {
        return;
    };
    msgs.insert(0, system_message(&load(root)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axioms_are_granite() {
        let g = load(".");
        assert!(g.contains("A = A"));
        assert!(g.contains("0 honored"));
        assert!(g.contains("force_type"));
        assert!(g.contains("ICRS & TDB"));
        assert!(g.contains("pending"));
    }

    #[test]
    fn inject_prepends_a_system_message() {
        let mut body =
            crate::json::parse_json(r#"{"model":"x","messages":[{"role":"user","content":"hi"}]}"#)
                .unwrap();
        inject(&mut body, ".");
        let JsonVal::Obj(map) = &body else {
            panic!("body is not an object");
        };
        let JsonVal::Arr(msgs) = map.get("messages").unwrap() else {
            panic!("messages absent");
        };
        assert_eq!(msgs.len(), 2);
        let JsonVal::Obj(m0) = &msgs[0] else {
            panic!("first message is not an object");
        };
        let role = m0.get("role").and_then(|v| match v {
            JsonVal::Str(s) => Some(s.as_str()),
            _ => None,
        });
        assert_eq!(role, Some("system"));
        let content = m0.get("content").and_then(|v| match v {
            JsonVal::Str(s) => Some(s.as_str()),
            _ => None,
        });
        assert!(content.unwrap().contains("A = A"));
    }

    #[test]
    fn inject_without_messages_stays_quiet() {
        let mut body = crate::json::parse_json(r#"{"model":"x"}"#).unwrap();
        inject(&mut body, ".");
        let JsonVal::Obj(map) = &body else {
            panic!("body is not an object");
        };
        assert!(map.get("messages").is_none());
    }
}
