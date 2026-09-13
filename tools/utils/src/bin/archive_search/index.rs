use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

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

const CACHE_MAGIC: &[u8; 8] = b"ARCHIVE1";

pub fn save_cache(path: &Path, index: &Index) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let file =
        File::create(path).map_err(|e| format!("the cache {} writes not: {e}", path.display()))?;
    let mut w = BufWriter::new(file);
    w.write_all(CACHE_MAGIC).map_err(|e| e.to_string())?;
    w.write_all(&index.scanned_at.to_le_bytes())
        .map_err(|e| e.to_string())?;
    w.write_all(&(index.labels.len() as u32).to_le_bytes())
        .map_err(|e| e.to_string())?;
    for label in &index.labels {
        write_str(&mut w, label)?;
    }
    w.write_all(&(index.entries.len() as u64).to_le_bytes())
        .map_err(|e| e.to_string())?;
    for entry in &index.entries {
        w.write_all(&entry.parent.to_le_bytes())
            .map_err(|e| e.to_string())?;
        w.write_all(&[entry.is_dir as u8])
            .map_err(|e| e.to_string())?;
        w.write_all(&entry.size.to_le_bytes())
            .map_err(|e| e.to_string())?;
        match entry.mtime {
            Some(v) => {
                w.write_all(&[1u8]).map_err(|e| e.to_string())?;
                w.write_all(&v.to_le_bytes()).map_err(|e| e.to_string())?;
            }
            None => {
                w.write_all(&[0u8]).map_err(|e| e.to_string())?;
            }
        }
        write_str(&mut w, &entry.name)?;
    }
    w.flush().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_cache(path: &Path) -> Result<Index, String> {
    let file =
        File::open(path).map_err(|e| format!("the cache {} reads not: {e}", path.display()))?;
    let mut r = BufReader::new(file);
    let mut magic = [0u8; 8];
    r.read_exact(&mut magic).map_err(|e| e.to_string())?;
    if &magic != CACHE_MAGIC {
        return Err("the cache carries a foreign signature".to_string());
    }
    let scanned_at = read_i64(&mut r)?;
    let label_count = read_u32(&mut r)?;
    let mut labels = Vec::with_capacity(label_count as usize);
    for _ in 0..label_count {
        labels.push(read_str(&mut r)?);
    }
    let count = read_u64(&mut r)?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let parent = read_u32(&mut r)?;
        let is_dir = read_u8(&mut r)? != 0;
        let size = read_u64(&mut r)?;
        let mtime = match read_u8(&mut r)? {
            0 => None,
            _ => Some(read_i64(&mut r)?),
        };
        let name = read_str(&mut r)?;
        entries.push(Entry {
            parent,
            is_dir,
            size,
            mtime,
            name,
        });
    }
    Ok(Index {
        entries,
        labels,
        scanned_at,
    })
}

fn write_str(w: &mut BufWriter<File>, s: &str) -> Result<(), String> {
    w.write_all(&(s.len() as u32).to_le_bytes())
        .map_err(|e| e.to_string())?;
    w.write_all(s.as_bytes()).map_err(|e| e.to_string())
}

fn read_u8(r: &mut BufReader<File>) -> Result<u8, String> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b).map_err(|e| e.to_string())?;
    Ok(b[0])
}

fn read_u32(r: &mut BufReader<File>) -> Result<u32, String> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b).map_err(|e| e.to_string())?;
    Ok(u32::from_le_bytes(b))
}

fn read_u64(r: &mut BufReader<File>) -> Result<u64, String> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b).map_err(|e| e.to_string())?;
    Ok(u64::from_le_bytes(b))
}

fn read_i64(r: &mut BufReader<File>) -> Result<i64, String> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b).map_err(|e| e.to_string())?;
    Ok(i64::from_le_bytes(b))
}

fn read_str(r: &mut BufReader<File>) -> Result<String, String> {
    let len = read_u32(r)? as usize;
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf).map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&buf).to_string())
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

    #[test]
    fn cache_round_trips() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("archive_index_test_{}.cache", std::process::id()));
        let index = Index {
            entries: vec![Entry {
                parent: u32::MAX,
                is_dir: false,
                size: 3,
                mtime: Some(7),
                name: "one.txt".into(),
            }],
            labels: vec!["tmp".into()],
            scanned_at: 42,
        };
        save_cache(&path, &index).unwrap();
        let back = load_cache(&path).unwrap();
        assert_eq!(back.scanned_at, 42);
        assert_eq!(back.labels, vec!["tmp".to_string()]);
        assert_eq!(back.entries.len(), 1);
        assert_eq!(back.entries[0].name, "one.txt");
        let _ = std::fs::remove_file(&path);
    }
}
