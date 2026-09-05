use std::env;
use std::path::Path;

use omegaflow::te::{gaussian, shuffle_series, silverman};

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const MIN_N: usize = 30;
const FACTORS: [f64; 2] = [1.0, 2.0];

fn read_series(path: &Path) -> Vec<f32> {
    let body = std::fs::read_to_string(path).unwrap_or_default();
    if path.extension().and_then(|s| s.to_str()) == Some("json") {
        return json_values(&body);
    }
    body.lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() || l.starts_with('#') {
                return None;
            }
            let v = if let Some((_, rhs)) = l.split_once(',') {
                rhs.trim()
            } else {
                l
            };
            v.parse::<f32>().ok()
        })
        .collect()
}

fn json_values(body: &str) -> Vec<f32> {
    let mut out = Vec::new();
    let pat = "\"v\":";
    let mut idx = 0;
    while let Some(rel) = body[idx..].find(pat) {
        let start = idx + rel + pat.len();
        let rest = &body[start..];
        let mut num = String::new();
        for ch in rest.chars() {
            if num.is_empty() && (ch == ' ' || ch == '\t') {
                continue;
            }
            if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == 'e' || ch == 'E' || ch == '+'
            {
                num.push(ch);
            } else {
                break;
            }
        }
        if num == "null" {
            idx = start + 1;
            continue;
        }
        if let Ok(v) = num.parse::<f32>() {
            out.push(v);
        }
        idx = start + 1;
    }
    out
}

fn stem(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("?")
        .to_string()
}

#[derive(Clone)]
struct Series {
    name: String,
    vals: Vec<f32>,
}

fn ols_fit(y: &[f32], c: &[f32]) -> Option<(f64, f64)> {
    if y.len() < 2 || y.len() != c.len() {
        return None;
    }
    let n = y.len();
    let yf: Vec<f64> = y.iter().map(|&v| v as f64).collect();
    let cf: Vec<f64> = c.iter().map(|&v| v as f64).collect();
    let mut sx = 0.0;
    let mut sy = 0.0;
    for i in 0..n {
        sx += cf[i];
        sy += yf[i];
    }
    let xm = sx / n as f64;
    let ym = sy / n as f64;
    let mut num = 0.0;
    let mut den = 0.0;
    let mut ssc = 0.0;
    for i in 0..n {
        num += (cf[i] - xm) * (yf[i] - ym);
        den += (cf[i] - xm) * (cf[i] - xm);
        ssc += cf[i] * cf[i];
    }
    if den <= 1e-12 * ssc {
        return None;
    }
    let beta1 = num / den;
    Some((beta1, ym - beta1 * xm))
}

fn residual_surrogate_conditional(y: &[f32], c: &[f32], rng: &mut u64) -> Vec<f32> {
    let n = y.len();
    match ols_fit(y, c) {
        Some((beta1, beta0)) => {
            let yf: Vec<f64> = y.iter().map(|&v| v as f64).collect();
            let cf: Vec<f64> = c.iter().map(|&v| v as f64).collect();
            let mut resid: Vec<f64> = (0..n).map(|i| yf[i] - (beta0 + beta1 * cf[i])).collect();
            for i in (1..n).rev() {
                *rng = rng
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let j = ((*rng >> 33) as usize) % (i + 1);
                resid.swap(i, j);
            }
            (0..n)
                .map(|i| (beta0 + beta1 * cf[i] + resid[i]) as f32)
                .collect()
        }
        None => shuffle_series(y, rng),
    }
}

fn conditional_te_h(x: &[f32], y: &[f32], c: &[f32], lag: usize, factor: f64) -> Option<f64> {
    let n = x.len();
    if n < 8 || y.len() < n || c.len() < n {
        return None;
    }
    if lag == 0 {
        return conditional_te_h(x, y, c, 1, factor);
    }
    let m = n - lag;
    if m < 8 {
        return None;
    }
    let hx = silverman(x)? * factor;
    let hy = silverman(y)? * factor;
    let hz = silverman(c)? * factor;
    let mut te = 0.0;
    for t in 0..m {
        let xt = x[t] as f64;
        let xk = x[t + lag] as f64;
        let yt = y[t] as f64;
        let zt = c[t] as f64;
        let mut k4 = 0.0;
        for s in 0..m {
            k4 += gaussian(xk - x[s + lag] as f64, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy)
                * gaussian(zt - c[s] as f64, hz);
        }
        let p4 = k4 / m as f64;
        let mut k2 = 0.0;
        for s in 0..n {
            k2 += gaussian(xt - x[s] as f64, hx) * gaussian(zt - c[s] as f64, hz);
        }
        let p2 = k2 / n as f64;
        let mut k3a = 0.0;
        for s in 0..n {
            k3a += gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy)
                * gaussian(zt - c[s] as f64, hz);
        }
        let p3a = k3a / n as f64;
        let mut k3b = 0.0;
        for s in 0..m {
            k3b += gaussian(xk - x[s + lag] as f64, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(zt - c[s] as f64, hz);
        }
        let p3b = k3b / m as f64;
        te += ((p4 * p2) / (p3a * p3b).max(1e-300)).ln();
    }
    Some(te / m as f64)
}

fn conditional_null_stats_h(
    x: &[f32],
    y: &[f32],
    c: &[f32],
    lag: usize,
    seed: u64,
    n_surr: usize,
    factor: f64,
) -> Option<(f64, f64, f64)> {
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..n_surr {
        let ys = residual_surrogate_conditional(y, c, &mut rng);
        if let Some(te) = conditional_te_h(x, &ys, c, lag, factor) {
            vals.push(te);
        }
    }
    if vals.len() < 2 {
        return None;
    }
    let n = vals.len() as f64;
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n;
    let sd = var.sqrt();
    Some((mean, sd, mean + 2.0 * sd))
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn print_cell(
    label: &str,
    te: Option<f64>,
    null: Option<(f64, f64, f64)>,
) -> String {
    match (te, null) {
        (Some(te), Some((_, sd, thr))) => {
            let verdict = if te > thr { "survives" } else { "falls" };
            format!(
                "{label:>24} | cTE {te:>10.6} | null mean+2σ {thr:>10.6} | sd {sd:>9.6} | {verdict}"
            )
        }
        _ => format!("{label:>24} | cTE       void | null mean+2σ        void | {:<9} | void", "void"),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(dir) = arg_value(&args, "--dir") else {
        eprintln!("--dir <folder with series> absent");
        std::process::exit(2);
    };
    let Some(cond_name) = arg_value(&args, "--cond") else {
        eprintln!("--cond <conditioning series name> absent");
        std::process::exit(2);
    };
    let lags: Vec<usize> = arg_value(&args, "--lags")
        .map(|v| v.split(',').filter_map(|s| s.parse().ok()).collect())
        .unwrap_or_else(|| vec![24, 48]);
    let n_surr: usize = arg_value(&args, "--surrogate")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20);

    let mut entries: Vec<_> = match std::fs::read_dir(&dir) {
        Ok(e) => e.filter_map(|e| e.ok()).map(|e| e.path()).collect(),
        Err(e) => {
            eprintln!("folder not readable: {e}");
            std::process::exit(2);
        }
    };
    entries.sort();
    let mut series: Vec<Series> = Vec::new();
    for p in &entries {
        if !p.is_file() {
            continue;
        }
        match p.extension().and_then(|s| s.to_str()) {
            Some("csv") | Some("txt") | Some("json") => {}
            _ => continue,
        }
        let vals = read_series(p);
        if vals.len() < MIN_N {
            eprintln!(
                "{}: n = {} < {MIN_N} -> skipped (underdetermined, no finding)",
                stem(p),
                vals.len()
            );
            continue;
        }
        series.push(Series {
            name: stem(p),
            vals,
        });
    }
    let cond = series
        .iter()
        .find(|s| s.name == cond_name)
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("--cond series '{cond_name}' not found among loaded series");
            std::process::exit(2);
        });

    println!(
        "=== Trishuli conditioned TE, Silverman bandwidth × factor (residual-surrogate null mean + 2σ, n_surr = {n_surr}) ==="
    );
    println!(
        "Series: {} | conditioning on {} (shared synoptic driver)",
        series
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        cond.name
    );
    println!(
        "Orientation (screen convention): 'a -> b' carries the a-side as the TE source (second estimator argument)."
    );
    println!();

    let mut tasks: Vec<(usize, usize)> = Vec::new();
    for i in 0..series.len() {
        for j in (i + 1)..series.len() {
            if series[i].name == cond.name || series[j].name == cond.name {
                continue;
            }
            tasks.push((i, j));
        }
    }
    if tasks.is_empty() {
        eprintln!("no non-conditioning pair in the folder -> no measurement (0 honored)");
        std::process::exit(0);
    }

    for &(i, j) in &tasks {
        let a = &series[i];
        let b = &series[j];
        let n = a.vals.len().min(b.vals.len());
        let av = &a.vals[..n];
        let bv = &b.vals[..n];
        let cn = cond.vals.len().min(n);
        let cv = &cond.vals[..cn];
        let seed_ab = SEED ^ (i as u64).rotate_left(16) ^ (j as u64);
        let seed_ba = SEED ^ (j as u64).rotate_left(16) ^ (i as u64);
        println!(
            "=== pair {} <-> {} (n = {n}), conditioned on {} ===",
            a.name, b.name, cond.name
        );
        for &lag in &lags {
            if n.saturating_sub(lag) < 8 {
                println!("lag {lag}: too few data");
                continue;
            }
            println!("--- lag {lag} h ---");
            for &factor in &FACTORS {
                let te_ab = conditional_te_h(bv, av, cv, lag, factor);
                let null_ab = conditional_null_stats_h(
                    bv,
                    av,
                    cv,
                    lag,
                    seed_ab ^ lag as u64,
                    n_surr,
                    factor,
                );
                let te_ba = conditional_te_h(av, bv, cv, lag, factor);
                let null_ba = conditional_null_stats_h(
                    av,
                    bv,
                    cv,
                    lag,
                    seed_ba ^ lag as u64,
                    n_surr,
                    factor,
                );
                let arrow_ab = format!("{} -> {}", a.name, b.name);
                let arrow_ba = format!("{} -> {}", b.name, a.name);
                println!("h × {factor:.1}:");
                println!("  {}", print_cell(&arrow_ab, te_ab, null_ab));
                println!("  {}", print_cell(&arrow_ba, te_ba, null_ba));
            }
            println!();
        }
    }
}
