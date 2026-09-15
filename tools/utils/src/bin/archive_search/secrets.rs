use std::collections::HashMap;

pub fn load_env(root: &std::path::Path) -> HashMap<String, String> {
    let mut env: HashMap<String, String> = HashMap::new();
    for name in [".env", ".secrets.local"] {
        if let Ok(text) = std::fs::read_to_string(root.join(name)) {
            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some(eq) = line.find('=') {
                    let key = line[..eq].trim().to_string();
                    let mut val = line[eq + 1..].trim().to_string();
                    let quoted = val.len() >= 2
                        && ((val.starts_with('"') && val.ends_with('"'))
                            || (val.starts_with('\'') && val.ends_with('\'')));
                    if quoted {
                        val = val[1..val.len() - 1].to_string();
                    }
                    if !key.is_empty() {
                        env.insert(key, val);
                    }
                }
            }
        }
    }
    for (k, v) in std::env::vars() {
        env.insert(k, v);
    }
    env
}

pub fn resolve_secret(template: &str, env: &HashMap<String, String>) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        out.push_str(&rest[..start]);
        rest = &rest[start..];
        let (open, close) = if rest.starts_with("{{") {
            ("{{", "}}")
        } else {
            ("{", "}")
        };
        if let Some(end) = rest[open.len()..].find(close) {
            let key = &rest[open.len()..open.len() + end];
            let upper = key.to_uppercase();
            if let Some(val) = env.get(key).or_else(|| env.get(&upper)) {
                out.push_str(val);
            }
            rest = &rest[open.len() + end + close.len()..];
        } else {
            out.push_str(&rest[..1]);
            rest = &rest[1..];
        }
    }
    out.push_str(rest);
    out
}

pub fn unresolved_key(template: &str, env: &HashMap<String, String>) -> Option<String> {
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        rest = &rest[start..];
        let (open, close) = if rest.starts_with("{{") {
            ("{{", "}}")
        } else {
            ("{", "}")
        };
        let Some(end) = rest[open.len()..].find(close) else {
            return None;
        };
        let key = &rest[open.len()..open.len() + end];
        let upper = key.to_uppercase();
        match env.get(key).or_else(|| env.get(&upper)) {
            Some(v) if !v.is_empty() => {}
            _ => return Some(key.to_string()),
        }
        rest = &rest[open.len() + end + close.len()..];
    }
    None
}

pub enum Secret {
    Value(String),
    Absent(Option<String>),
}

pub fn resolve_key(template: &str, env: &HashMap<String, String>) -> Secret {
    if let Some(marker) = unresolved_key(template, env) {
        return Secret::Absent(Some(marker));
    }
    let value = resolve_secret(template, env);
    if value.is_empty() {
        Secret::Absent(None)
    } else {
        Secret::Value(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_single_and_double_braces() {
        let mut env = HashMap::new();
        env.insert("TOKEN".to_string(), "abc".to_string());
        assert_eq!(resolve_secret("Bearer {TOKEN}", &env), "Bearer abc");
        assert_eq!(resolve_secret("Bearer {{TOKEN}}", &env), "Bearer abc");
        assert_eq!(resolve_secret("{{token}}", &env), "abc");
    }

    #[test]
    fn absent_marker_substitutes_void() {
        let env = HashMap::new();
        assert_eq!(resolve_secret("x{MISSING}y", &env), "xy");
    }

    #[test]
    fn unresolved_key_names_the_missing_marker() {
        let mut env = HashMap::new();
        env.insert("ADS_RAW".to_string(), "abc".to_string());
        assert_eq!(unresolved_key("Bearer {ADS_RAW}", &env), None);
        assert_eq!(
            unresolved_key("Bearer {MISSING}", &env),
            Some("MISSING".to_string())
        );
        assert_eq!(unresolved_key("plain", &env), None);
    }

    #[test]
    fn resolve_key_is_absent_not_a_partial_token() {
        let env = HashMap::new();
        assert!(matches!(
            resolve_key("Bearer {MISSING}", &env),
            Secret::Absent(Some(m)) if m == "MISSING"
        ));
        assert!(matches!(resolve_key("", &env), Secret::Absent(None)));
        let mut env2 = HashMap::new();
        env2.insert("ADS_RAW".to_string(), "abc".to_string());
        assert!(matches!(
            resolve_key("Bearer {ADS_RAW}", &env2),
            Secret::Value(v) if v == "Bearer abc"
        ));
    }
}
