use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;

struct Post {
    text: String,
    has_commit: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut db = PathBuf::from(env::var("HOME").unwrap_or_default())
        .join(".local/share/opencode/opencode.db");
    let mut top: usize = 20;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--db" => {
                i += 1;
                db = PathBuf::from(&args[i]);
            }
            "--top" => {
                i += 1;
                top = args[i].parse().unwrap_or(20);
            }
            _ => {}
        }
        i += 1;
    }

    let posts = read_posts(&db);
    let (commit_posts, no_commit_posts): (Vec<&Post>, Vec<&Post>) =
        posts.iter().partition(|p| p.has_commit);
    println!(
        "db: {} posts | {} with commit | {} without commit",
        posts.len(),
        commit_posts.len(),
        no_commit_posts.len()
    );

    let mut freq: HashMap<String, usize> = HashMap::new();
    for p in &posts {
        for w in words(&p.text) {
            *freq.entry(w).or_insert(0) += 1;
        }
    }
    let mut sorted: Vec<(&String, usize)> = freq.iter().map(|(w, c)| (w, *c)).collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    println!("\ntop {top} words (all posts):");
    for (w, c) in sorted.iter().take(top) {
        println!("  {:>6}  {}", c, w);
    }

    let numbers: Vec<usize> = posts.iter().map(|p| numbers(&p.text).len()).collect();
    let nsum: usize = numbers.iter().sum();
    println!(
        "\nnumbers: {} tokens total, mean {:.2} per post",
        nsum,
        if posts.is_empty() {
            0.0
        } else {
            nsum as f64 / posts.len() as f64
        }
    );

    println!(
        "\ntransfer entropy (word -> number / number -> word), per class, lag 1..3, top words:"
    );
    let top_words: Vec<String> = sorted.iter().take(top).map(|(w, _)| (*w).clone()).collect();
    for (name, class) in [("commit", &commit_posts), ("no-commit", &no_commit_posts)] {
        if class.len() < 30 {
            println!("  {}: too few posts ({}), no TE", name, class.len());
            continue;
        }
        let nnum = number_series(class);
        println!("  --- {} ({} posts) ---", name, class.len());
        for w in &top_words {
            let nword = word_series(class, w);

            let mut best_fwd: f64 = 0.0;
            let mut best_rev: f64 = 0.0;
            for lag in 1..4 {
                if let Some(f) = te(&nword, &nnum, lag) {
                    best_fwd = best_fwd.max(f);
                }
                if let Some(r) = te(&nnum, &nword, lag) {
                    best_rev = best_rev.max(r);
                }
            }
            if best_fwd > 0.01 || best_rev > 0.01 {
                println!(
                    "    {:>10}: wort->zahl {:.4}  zahl->wort {:.4}",
                    w, best_fwd, best_rev
                );
            }
        }
    }
}

fn read_posts(db: &PathBuf) -> Vec<Post> {
    let out = Command::new("sqlite3")
        .arg("-separator")
        .arg("\x1f")
        .arg(db)
        .arg("SELECT p.time_created, json_extract(p.data,'$.text') FROM part p WHERE json_extract(p.data,'$.type')='text' AND json_extract(p.data,'$.text') IS NOT NULL ORDER BY p.time_created;")
        .output();
    let out = match out {
        Ok(o) if o.status.success() => o,
        Ok(o) => {
            eprintln!(
                "session_token_te: sqlite3: {}",
                String::from_utf8_lossy(&o.stderr)
            );
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("session_token_te: sqlite3: {}", e);
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    let mut posts = Vec::new();
    for line in text.lines() {
        let mut parts = line.splitn(2, '\x1f');
        let _time = parts.next().unwrap_or("").to_string();
        let body = parts.next().unwrap_or("").to_string();
        let has_commit = has_commit_marker(&body);
        posts.push(Post {
            text: body,
            has_commit,
        });
    }
    posts
}

fn has_commit_marker(text: &str) -> bool {
    let lower = text.to_lowercase();
    for w in lower.split(|c: char| !c.is_ascii_alphanumeric()) {
        if w.len() >= 7 && w.chars().all(|c| c.is_ascii_hexdigit()) {
            return true;
        }
    }
    false
}

fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphabetic())
        .filter(|w| w.len() >= 3)
        .map(|w| w.to_lowercase())
        .filter(|w| !matches!(w.as_str(), "der" | "die" | "das" | "und" | "ich" | "mit"))
        .collect()
}

fn numbers(text: &str) -> Vec<f64> {
    let mut out = Vec::new();
    for tok in text.split(|c: char| !(c.is_ascii_digit() || c == '.')) {
        if tok.parse::<f64>().is_ok() && tok.len() <= 12 {
            out.push(tok.parse().unwrap());
        }
    }
    out
}

fn word_series(posts: &[&Post], word: &str) -> Vec<f32> {
    posts
        .iter()
        .map(|p| {
            if p.text.to_lowercase().contains(word) {
                1.0
            } else {
                0.0
            }
        })
        .collect()
}

fn number_series(posts: &[&Post]) -> Vec<f32> {
    posts
        .iter()
        .map(|p| numbers(&p.text).len() as f32)
        .collect()
}

fn te(x: &[f32], y: &[f32], lag: usize) -> Option<f64> {
    if x.len() < 40 || y.len() < 40 {
        return None;
    }

    let nb = 8;
    let (xb, yb) = (bin(x, nb), bin(y, nb));
    let n = xb.len();
    let mut cnt_prev = [[0f64; 8]; 8];
    let mut cnt_cond = [[[0f64; 8]; 8]; 8];
    for t in 1..n {
        let yv = yb[t];
        let yp = yb[t - 1];
        cnt_prev[yp][yv] += 1.0;
        if t >= lag {
            let xp = xb[t - lag];
            cnt_cond[xp][yp][yv] += 1.0;
        }
    }
    let mut h_prev = 0.0;
    for j in 0..8 {
        let tot: f64 = (0..8).map(|i| cnt_prev[j][i]).sum();
        if tot > 0.0 {
            for i in 0..8 {
                if cnt_prev[j][i] > 0.0 {
                    let p = cnt_prev[j][i] / tot;
                    h_prev -= p * p.ln();
                }
            }
        }
    }
    let mut h_cond = 0.0;
    for xv in 0..8 {
        for j in 0..8 {
            let tot: f64 = (0..8).map(|i| cnt_cond[xv][j][i]).sum();
            if tot > 0.0 {
                for i in 0..8 {
                    if cnt_cond[xv][j][i] > 0.0 {
                        let p = cnt_cond[xv][j][i] / tot;
                        h_cond -= p * p.ln();
                    }
                }
            }
        }
    }
    let te = h_prev - h_cond;
    if te.is_finite() && te > 0.0 {
        Some(te)
    } else {
        Some(0.0)
    }
}

fn bin(vals: &[f32], nb: usize) -> Vec<usize> {
    let lo = vals.iter().cloned().fold(f32::INFINITY, f32::min);
    let hi = vals.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let span = (hi - lo).max(1.0);
    vals.iter()
        .map(|&v| ((v - lo) / span * nb as f32) as usize % nb)
        .collect()
}
