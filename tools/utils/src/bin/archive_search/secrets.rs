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
}
