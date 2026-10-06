use super::*;

pub fn derive_frame(parsed: &JsonVal, coords: &str) -> (String, String) {
    let _ = (parsed, coords);
    ("".to_string(), "frame pending".to_string())
}

pub fn draft_frame_guess(
    url: &str,
    context: &str,
    registry: &HashMap<String, String>,
) -> (String, String) {
    for key in route_prefix_keys(url) {
        if let Some(f) = registry.get(&key) {
            return (format!("{}\n", f), format!("route-registry: {}", f));
        }
    }
    let _ = context;
    ("".to_string(), "frame pending".to_string())
}

pub fn build_frame_registry() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for path in [
        "phi/sources.φ",
        "phi/dead_sources.φ",
        "phi/declined_sources.φ",
        "phi/blocked_sources.φ",
        "phi/witnesses.φ",
    ] {
        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };
        let mut cur_url: Option<String> = None;
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ") {
                cur_url = Some(rest.trim().to_string());
            } else if let Some(url) = &cur_url {
                if let Some(rest) = t.strip_prefix("on ") {
                    let body = rest.split_whitespace().next();
                    if let (Some(rk), Some(body)) = (route_key(url), body) {
                        map.entry(rk).or_insert_with(|| format!("on {}", body));
                    }
                } else if let Some(rest) = t.strip_prefix("at ") {
                    let body = rest.split_whitespace().next();
                    if let (Some(rk), Some(body)) = (route_key(url), body) {
                        map.entry(rk).or_insert_with(|| format!("at {}", body));
                    }
                }
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("phi/pipeline/frame_learned.φ") {
        for line in content.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('#') {
                continue;
            }
            if let Some((nl, frame)) = t.split_once('|') {
                let nl = nl.trim();
                let frame = frame.trim();
                if !nl.is_empty() && !frame.is_empty() {
                    map.entry(nl.to_string())
                        .or_insert_with(|| frame.to_string());
                }
            }
        }
    }
    map
}

pub fn learn_frames(new: &HashMap<String, String>) {
    let mut map: HashMap<String, String> = HashMap::new();
    if let Ok(content) = std::fs::read_to_string("phi/pipeline/frame_learned.φ") {
        for line in content.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('#') {
                continue;
            }
            if let Some((nl, frame)) = t.split_once('|') {
                map.insert(nl.trim().to_string(), frame.trim().to_string());
            }
        }
    }
    for (nl, frame) in new {
        map.entry(nl.to_string())
            .or_insert_with(|| frame.to_string());
    }
    let mut out = String::from(
        "# frame-learned — route (host/path, query stripped) → frame, self-learning from probe responses (--draft)\n",
    );
    let mut keys: Vec<(&String, &String)> = map.iter().collect();
    keys.sort();
    for (nl, frame) in keys {
        out.push_str(&format!("{} | {}\n", nl, frame));
    }
    std::fs::create_dir_all("phi/pipeline").ok();
    if std::fs::write("phi/pipeline/frame_learned.φ", out).is_err() {
        eprintln!("write phi/pipeline/frame_learned.φ: the register does not remember");
    }
}
