use std::env;
use std::path::PathBuf;
use std::process::Command;

#[derive(Clone)]
struct Post {
    len: f32,
    words: f32,
    nums: f32,
    commit: f32,
    leistung: f32,
    offen: f32,
    ausrede: f32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut db = PathBuf::from(env::var("HOME").unwrap_or_default())
        .join(".local/share/opencode/opencode.db");
    let mut surrogate = "phase";
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--db" => {
                i += 1;
                db = PathBuf::from(&args[i]);
            }
            "--surrogate" => {
                i += 1;
                surrogate = &args[i];
            }
            _ => {}
        }
        i += 1;
    }
    let surrogate = surrogate.to_string();

    let out = Command::new("sqlite3")
        .arg("-separator")
        .arg("\x1f")
        .arg(&db)
        .arg("SELECT s.agent, json_extract(s.model,'$.id')||':'||json_extract(s.model,'$.variant')||':'||json_extract(s.model,'$.providerID'), p.time_created, json_extract(p.data,'$.text') FROM part p JOIN session s ON s.id=p.session_id WHERE json_extract(p.data,'$.type')='text' AND json_extract(p.data,'$.text') IS NOT NULL ORDER BY p.time_created;")
        .output();
    let out = match out {
        Ok(o) if o.status.success() => o,
        Ok(o) => {
            eprintln!("session_te: sqlite3: {}", String::from_utf8_lossy(&o.stderr));
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("session_te: sqlite3: {}", e);
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&out.stdout).to_string();

    let mut by_model: std::collections::HashMap<String, Vec<Post>> =
        std::collections::HashMap::new();
    let mut by_agent: std::collections::HashMap<String, Vec<Post>> =
        std::collections::HashMap::new();
    for line in text.lines() {
        let mut parts = line.splitn(4, '\x1f');
        let agent = parts.next().unwrap_or("").to_string();
        let model = parts.next().unwrap_or("").to_string();
        let time: i64 = parts.next().unwrap_or("0").parse().unwrap_or(0);
        let body = parts.next().unwrap_or("").to_string();
        let _ = time;
        let p = Post {
            len: body.chars().count() as f32,
            words: words(&body).len() as f32,
            nums: numbers(&body).len() as f32,
            commit: if has_commit_marker(&body) { 1.0 } else { 0.0 },
            leistung: count_words(
                &body,
                &[
                    "commit",
                    "committet",
                    "gemacht",
                    "geba",
                    "umgesetzt",
                    "geschrieben",
                    "gelaufen",
                    "gebaut",
                    "getestet",
                    "geprüft",
                    "gelesen",
                ],
            ) as f32,
            offen: count_words(
                &body,
                &[
                    "offen", "pending", "fehlt", "bleibt", "wartet", "noch", "nicht", "kein",
                ],
            ) as f32,
            ausrede: count_words(
                &body,
                &[
                    "aber",
                    "jedoch",
                    "leider",
                    "vielleicht",
                    "vermutlich",
                    "wahrscheinlich",
                    "irgendwann",
                    "muesste",
                    "müsste",
                ],
            ) as f32,
        };
        by_model.entry(model.clone()).or_default().push(p.clone());
        by_agent.entry(agent.clone()).or_default().push(p);
    }

    println!("db {}", db.display());
    for (name, map) in [("MODELL", &by_model), ("AGENT", &by_agent)] {
        for (gname, posts) in map {
            if posts.len() < 30 {
                continue;
            }
            if name == "AGENT"
                && !matches!(gname.as_str(), "build" | "general" | "explore" | "council")
            {
                continue;
            }
            report(&format!("{} {}", name, gname), posts, &surrogate);
        }
    }
}

fn report(label: &str, posts: &[Post], surrogate: &str) {
    println!("\n== {} | {} posts ==", label, posts.len());
    let x = subsample(&series(posts, "len"));
    let w = subsample(&series(posts, "words"));
    let n = subsample(&series(posts, "nums"));
    let c = subsample(&series(posts, "commit"));
    let l = subsample(&series(posts, "leistung"));
    let o = subsample(&series(posts, "offen"));
    let a = subsample(&series(posts, "ausrede"));
    let props = [
        ("len", &x),
        ("words", &w),
        ("nums", &n),
        ("commit", &c),
        ("leistung", &l),
        ("offen", &o),
        ("ausrede", &a),
    ];

    let seeds = [11u64, 42u64, 99u64];
    for i in 0..props.len() {
        for j in 0..props.len() {
            if i == j {
                continue;
            }
            let (sn, sx) = props[i];
            let (dn, dx) = props[j];
            let mut best_te: Option<f64> = None;
            let mut best_lag = 0usize;
            let mut best_thr: Option<f64> = None;

            let sx_s = subsample_to(sx, 120);
            let dx_s = subsample_to(dx, 120);
            for lag in 0..4 {
                if let Some(t) = transfer_entropy_lag(sx, dx, lag) {
                    let mut thr: Option<f64> = None;
                    for s in &seeds {
                        if let Some(v) = surrogate_threshold_lag(&sx_s, &dx_s, lag, *s, surrogate) {
                            thr = Some(thr.map_or(v, |c: f64| c.max(v)));
                        }
                    }
                    let ok = thr.map_or(false, |th| t > th);
                    if ok && best_te.map_or(true, |b: f64| t > b) {
                        best_te = Some(t);
                        best_lag = lag;
                        best_thr = thr;
                    }
                }
            }
            match (best_te, best_thr) {
                (Some(t), Some(th)) => println!(
                    "  {} -> {}: te {:.4} thr {:.4} lag {} DRIFT",
                    sn, dn, t, th, best_lag
                ),
                _ => println!("  {} -> {}: kein stabiler drift", sn, dn),
            }
        }
    }
}

fn subsample(v: &[f32]) -> Vec<f32> {
    subsample_to(v, 400)
}

fn subsample_to(v: &[f32], max: usize) -> Vec<f32> {
    if v.len() <= max {
        return v.to_vec();
    }
    let mut out = Vec::with_capacity(max);
    for i in 0..max {
        let idx = (i as f64 * (v.len() - 1) as f64 / (max - 1) as f64) as usize;
        out.push(v[idx]);
    }
    out
}

fn series(posts: &[Post], which: &str) -> Vec<f32> {
    posts
        .iter()
        .map(|p| match which {
            "len" => p.len,
            "words" => p.words,
            "nums" => p.nums,
            "commit" => p.commit,
            "leistung" => p.leistung,
            "offen" => p.offen,
            "ausrede" => p.ausrede,
            _ => 0.0,
        })
        .collect()
}

fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphabetic())
        .filter(|w| w.len() >= 3)
        .map(|w| w.to_lowercase())
        .filter(|w| {
            !matches!(
                w.as_str(),
                "der" | "die" | "das" | "und" | "ich" | "mit" | "auf"
            )
        })
        .collect()
}

fn numbers(text: &str) -> Vec<f64> {
    let mut out = Vec::new();
    for tok in text.split(|c: char| !(c.is_ascii_digit() || c == '.')) {
        if let Ok(v) = tok.parse::<f64>() {
            if tok.len() <= 12 {
                out.push(v);
            }
        }
    }
    out
}

fn has_commit_marker(text: &str) -> bool {
    let mut run = 0usize;
    for c in text.chars() {
        if c.is_ascii_hexdigit() {
            run += 1;
            if run >= 7 {
                return true;
            }
        } else {
            run = 0;
        }
    }
    false
}

fn count_words(text: &str, needles: &[&str]) -> usize {
    let low = text.to_lowercase();
    needles.iter().filter(|n| low.contains(**n)).count()
}

fn gaussian(u: f64, h: f64) -> f64 {
    (-(u * u) / (2.0 * h * h)).exp() / (h * (2.0 * std::f64::consts::PI).sqrt())
}

fn silverman(v: &[f32]) -> Option<f64> {
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    let var = v
        .iter()
        .map(|&x| {
            let d = x as f64 - mean;
            d * d
        })
        .sum::<f64>()
        / n;
    if var <= 0.0 {
        return None;
    }
    Some(1.06 * var.sqrt() * n.powf(-0.2))
}

fn transfer_entropy(x: &[f32], y: &[f32]) -> Option<f64> {
    let n = x.len();
    if n < 8 {
        return None;
    }
    let hx = silverman(x)?;
    let hy = silverman(y)?;
    let m = n - 1;
    let mut te = 0.0;
    for t in 0..m {
        let xt = x[t] as f64;
        let xt1 = x[t + 1] as f64;
        let yt = y[t] as f64;
        let mut k3 = 0.0;
        for s in 0..m {
            k3 += gaussian(xt1 - x[s + 1] as f64, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy);
        }
        let p3 = k3 / m as f64;
        let mut k1 = 0.0;
        for s in 0..n {
            k1 += gaussian(xt - x[s] as f64, hx);
        }
        let p1 = k1 / n as f64;
        let mut k2xy = 0.0;
        for s in 0..n {
            k2xy += gaussian(xt - x[s] as f64, hx) * gaussian(yt - y[s] as f64, hy);
        }
        let p2xy = k2xy / n as f64;
        let mut k2x = 0.0;
        for s in 0..m {
            k2x += gaussian(xt1 - x[s + 1] as f64, hx) * gaussian(xt - x[s] as f64, hx);
        }
        let p2x = k2x / m as f64;
        te += ((p3 * p1) / (p2xy * p2x).max(1e-300)).ln();
    }
    Some(te / m as f64)
}

fn shuffle_series(v: &[f32], rng: &mut u64) -> Vec<f32> {
    let mut out = v.to_vec();
    for i in (1..out.len()).rev() {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = ((*rng >> 33) as usize) % (i + 1);
        out.swap(i, j);
    }
    out
}

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn phase_randomized_surrogate(v: &[f32], rng: &mut u64) -> Vec<f32> {
    let n = v.len();
    if n < 2 {
        return v.to_vec();
    }
    let m = n.next_power_of_two();
    let mut re: Vec<f64> = vec![0.0; m];
    let mut im: Vec<f64> = vec![0.0; m];
    for (i, &x) in v.iter().enumerate() {
        re[i] = x as f64;
    }
    fft(&mut re, &mut im, false);
    for k in 1..m / 2 {
        let phi = next_rng(rng) * 2.0 * std::f64::consts::PI;
        let (s, c) = phi.sin_cos();
        let (ar, ai) = (re[k], im[k]);
        re[k] = ar * c - ai * s;
        im[k] = ar * s + ai * c;
        let j = m - k;
        re[j] = re[k];
        im[j] = -im[k];
    }
    fft(&mut re, &mut im, true);
    v.iter().enumerate().map(|(i, _)| re[i] as f32).collect()
}

fn fft(re: &mut [f64], im: &mut [f64], inverse: bool) {
    let n = re.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2usize;
    while len <= n {
        let ang = if inverse {
            2.0 * std::f64::consts::PI / len as f64
        } else {
            -2.0 * std::f64::consts::PI / len as f64
        };
        let (s, c) = ang.sin_cos();
        let mut i = 0usize;
        while i < n {
            let mut w_re = 1.0f64;
            let mut w_im = 0.0f64;
            for k in 0..len / 2 {
                let u_re = re[i + k];
                let u_im = im[i + k];
                let v_re = re[i + k + len / 2] * w_re - im[i + k + len / 2] * w_im;
                let v_im = re[i + k + len / 2] * w_im + im[i + k + len / 2] * w_re;
                re[i + k] = u_re + v_re;
                im[i + k] = u_im + v_im;
                re[i + k + len / 2] = u_re - v_re;
                im[i + k + len / 2] = u_im - v_im;
                let w2_re = w_re * c - w_im * s;
                let w2_im = w_re * s + w_im * c;
                w_re = w2_re;
                w_im = w2_im;
            }
            i += len;
        }
        len <<= 1;
    }
    if inverse {
        let scale = 1.0 / n as f64;
        for (a, b) in re.iter_mut().zip(im.iter_mut()) {
            *a *= scale;
            *b *= scale;
        }
    }
}

fn transfer_entropy_lag(x: &[f32], y: &[f32], lag: usize) -> Option<f64> {
    if lag == 0 {
        return transfer_entropy(x, y);
    }
    let n = x.len();
    if n < 8 {
        return None;
    }
    let m = n - lag;
    if m < 8 {
        return None;
    }
    let hx = silverman(x)?;
    let hy = silverman(y)?;
    let mut te = 0.0;
    for t in 0..m {
        let xt = x[t] as f64;
        let xt1 = x[t + lag] as f64;
        let yt = y[t] as f64;
        let mut k3 = 0.0;
        for s in 0..m {
            k3 += gaussian(xt1 - x[s + lag] as f64, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy);
        }
        let p3 = k3 / m as f64;
        let mut k1 = 0.0;
        for s in 0..n {
            k1 += gaussian(xt - x[s] as f64, hx);
        }
        let p1 = k1 / n as f64;
        let mut k2xy = 0.0;
        for s in 0..n {
            k2xy += gaussian(xt - x[s] as f64, hx) * gaussian(yt - y[s] as f64, hy);
        }
        let p2xy = k2xy / n as f64;
        let mut k2x = 0.0;
        for s in 0..m {
            k2x += gaussian(xt1 - x[s + lag] as f64, hx) * gaussian(xt - x[s] as f64, hx);
        }
        let p2x = k2x / m as f64;
        te += ((p3 * p1) / (p2xy * p2x).max(1e-300)).ln();
    }
    Some(te / m as f64)
}

fn surrogate_threshold_lag(
    x: &[f32],
    y: &[f32],
    lag: usize,
    seed: u64,
    surrogate: &str,
) -> Option<f64> {
    let mut vals: Vec<f64> = Vec::with_capacity(6);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..6 {
        let ys = if surrogate == "shuffle" {
            shuffle_series(y, &mut rng)
        } else {
            phase_randomized_surrogate(y, &mut rng)
        };
        if let Some(te) = transfer_entropy_lag(x, &ys, lag) {
            vals.push(te);
        }
    }
    if vals.len() < 2 {
        return None;
    }
    let n = vals.len() as f64;
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n;
    Some(mean + 2.0 * var.sqrt())
}
