pub struct RefusalLedger {
    pub path: std::path::PathBuf,
    pub seen: std::collections::HashSet<String>,
}

impl RefusalLedger {
    pub fn new(path: &str) -> RefusalLedger {
        let mut seen = std::collections::HashSet::new();
        if let Ok(content) = std::fs::read_to_string(path) {
            for line in content.lines() {
                let mut parts = line.splitn(4, ' ');
                if parts.next() == Some("refused") {
                    let _unix = parts.next();
                    let class = parts.next().unwrap_or("");
                    let url = parts.next().unwrap_or("");
                    if !class.is_empty() && !url.is_empty() {
                        seen.insert(format!("{}|{}", class, url));
                    }
                }
            }
        }
        RefusalLedger {
            path: std::path::PathBuf::from(path),
            seen,
        }
    }

    pub fn register(&mut self, url: &str, class: &str) {
        let key = format!("{}|{}", class, url);
        let unix = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => d.as_secs(),
            Err(_) => return,
        };
        if !self.seen.insert(key) {
            return;
        }
        let line = format!("refused {} {} {}\n", unix, class, url);
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
        {
            use std::io::Write;
            let _ = f.write_all(line.as_bytes());
        }
    }
}
