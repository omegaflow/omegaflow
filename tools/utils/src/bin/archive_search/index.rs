#[derive(Clone)]
pub struct Entry {
    pub parent: u32,
    pub is_dir: bool,
    pub size: u64,
    pub mtime: Option<i64>,
    pub name: String,
}

pub struct Index {
    pub entries: Vec<Entry>,
    pub labels: Vec<String>,
    pub scanned_at: i64,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    Any,
    File,
    Dir,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Sort {
    Name,
    Size,
    Mtime,
}

pub struct Query {
    pub text: String,
    pub ci: bool,
    pub path: bool,
    pub kind: Kind,
    pub sort: Sort,
    pub limit: usize,
}

pub struct Hit {
    pub path: String,
    pub size: u64,
    pub mtime: Option<i64>,
}

impl Index {
    pub fn resolve_path(&self, index: u32) -> String {
        let mut parts: Vec<&str> = Vec::new();
        let mut cur = index;
        let mut guard = 0u32;
        while (cur as usize) < self.entries.len() {
            let entry = &self.entries[cur as usize];
            parts.push(entry.name.as_str());
            if entry.parent == u32::MAX {
                break;
            }
            cur = entry.parent;
            guard += 1;
            if guard > 8192 {
                break;
            }
        }
        let mut out = String::new();
        for (i, part) in parts.iter().rev().enumerate() {
            if i > 0 {
                out.push('/');
            }
            out.push_str(part);
        }
        out
    }

    pub fn search(&self, q: &Query) -> Vec<Hit> {
        let wildcard = q.text.contains('*') || q.text.contains('?');
        let needle = if q.ci {
            q.text.to_lowercase()
        } else {
            q.text.clone()
        };
        let mut matched: Vec<u32> = Vec::new();
        for (i, entry) in self.entries.iter().enumerate() {
            match q.kind {
                Kind::File if entry.is_dir => continue,
                Kind::Dir if !entry.is_dir => continue,
                _ => {}
            }
            if q.path {
                let path = self.resolve_path(i as u32);
                if !pattern_match(&path, &needle, wildcard, q.ci) {
                    continue;
                }
            } else if !pattern_match(&entry.name, &needle, wildcard, q.ci) {
                continue;
            }
            matched.push(i as u32);
        }
        match q.sort {
            Sort::Name => matched.sort_by(|a, b| {
                self.entries[*a as usize]
                    .name
                    .to_lowercase()
                    .cmp(&self.entries[*b as usize].name.to_lowercase())
            }),
            Sort::Size => matched.sort_by(|a, b| {
                self.entries[*b as usize]
                    .size
                    .cmp(&self.entries[*a as usize].size)
            }),
            Sort::Mtime => matched.sort_by(|a, b| {
                self.entries[*b as usize]
                    .mtime
                    .cmp(&self.entries[*a as usize].mtime)
            }),
        }
        matched.truncate(q.limit);
        matched
            .iter()
            .map(|&i| {
                let entry = &self.entries[i as usize];
                Hit {
                    path: self.resolve_path(i),
                    size: entry.size,
                    mtime: entry.mtime,
                }
            })
            .collect()
    }
}

fn pattern_match(hay: &str, needle: &str, wildcard: bool, ci: bool) -> bool {
    if needle.is_empty() {
        return true;
    }
    let text = if ci {
        hay.to_lowercase()
    } else {
        hay.to_string()
    };
    if wildcard {
        wildcard_match(&text, needle)
    } else {
        text.contains(needle)
    }
}

fn wildcard_match(text: &str, pattern: &str) -> bool {
    let t: Vec<char> = text.chars().collect();
    let p: Vec<char> = pattern.chars().collect();
    let mut ti = 0usize;
    let mut pi = 0usize;
    let mut star_p = usize::MAX;
    let mut star_t = 0usize;
    while ti < t.len() {
        if pi < p.len() && (p[pi] == '?' || p[pi] == t[ti]) {
            ti += 1;
            pi += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star_p = pi;
            star_t = ti;
            pi += 1;
        } else if star_p != usize::MAX {
            pi = star_p + 1;
            star_t += 1;
            ti = star_t;
        } else {
            return false;
        }
    }
    while pi < p.len() && p[pi] == '*' {
        pi += 1;
    }
    pi == p.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_matches_star_and_question() {
        assert!(wildcard_match("report_final.pdf", "*.pdf"));
        assert!(wildcard_match("report_final.pdf", "report*"));
        assert!(wildcard_match("a1c", "a?c"));
        assert!(!wildcard_match("a12c", "a?c"));
        assert!(wildcard_match("abc", "*"));
    }

    #[test]
    fn search_filters_by_kind_and_text() {
        let index = Index {
            entries: vec![
                Entry {
                    parent: u32::MAX,
                    is_dir: true,
                    size: 0,
                    mtime: Some(0),
                    name: "root".into(),
                },
                Entry {
                    parent: 0,
                    is_dir: false,
                    size: 10,
                    mtime: Some(5),
                    name: "alpha.txt".into(),
                },
                Entry {
                    parent: 0,
                    is_dir: true,
                    size: 0,
                    mtime: Some(0),
                    name: "alpine".into(),
                },
            ],
            labels: vec!["root".into()],
            scanned_at: 0,
        };
        let files = index.search(&Query {
            text: "alp".into(),
            ci: true,
            path: false,
            kind: Kind::File,
            sort: Sort::Name,
            limit: 10,
        });
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "root/alpha.txt");
    }
}
