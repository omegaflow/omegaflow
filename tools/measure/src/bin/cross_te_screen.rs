use std::env;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use omegaflow::te::{
    conditional_te_stats, surrogate_threshold_lag, transfer_entropy_conditional,
    transfer_entropy_lag,
};

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

fn physical_cores() -> usize {
    let mut cores: std::collections::HashSet<usize> = std::collections::HashSet::new();
    if let Ok(entries) = std::fs::read_dir("/sys/devices/system/cpu") {
        for e in entries.flatten() {
            let name = e.file_name();
            let name = name.to_string_lossy();
            if !(name.starts_with("cpu") && name[3..].chars().all(|c| c.is_ascii_digit())) {
                continue;
            }
            let core_path = e.path().join("topology").join("core_id");
            if let Ok(id) = std::fs::read_to_string(&core_path) {
                if let Ok(id) = id.trim().parse::<usize>() {
                    cores.insert(id);
                }
            }
        }
    }
    if !cores.is_empty() {
        return cores.len();
    }
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .max(1)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let dir = args
        .iter()
        .position(|a| a == "--dir")
        .and_then(|i| args.get(i + 1));
    let lags: Vec<usize> = args
        .iter()
        .position(|a| a == "--lags")
        .and_then(|i| args.get(i + 1))
        .map(|v| v.split(',').filter_map(|s| s.parse().ok()).collect())
        .unwrap_or_else(|| vec![1, 3, 6, 12, 24]);
    let n_surr: u64 = args
        .iter()
        .position(|a| a == "--surrogate")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    let min_n: usize = args
        .iter()
        .position(|a| a == "--min-n")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(30);
    let cond_name: Option<String> = args
        .iter()
        .position(|a| a == "--cond")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string());
    let jobs: usize = args
        .iter()
        .position(|a| a == "--jobs")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(physical_cores);
    let filters: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(_, a)| a.as_str() == "--filter")
        .filter_map(|(i, _)| args.get(i + 1))
        .map(|s| s.to_string())
        .collect();

    let Some(dir) = dir else {
        eprintln!("--dir <folder with series> absent");
        std::process::exit(2);
    };

    let mut series: Vec<Series> = Vec::new();
    let mut entries: Vec<_> = match std::fs::read_dir(dir) {
        Ok(e) => e.filter_map(|e| e.ok()).map(|e| e.path()).collect(),
        Err(e) => {
            eprintln!("folder not readable: {e}");
            std::process::exit(2);
        }
    };
    entries.sort();
    for p in &entries {
        if !p.is_file() {
            continue;
        }
        match p.extension().and_then(|s| s.to_str()) {
            Some("csv") | Some("txt") | Some("json") => {}
            _ => continue,
        }
        let vals = read_series(p);
        if vals.len() < min_n {
            eprintln!(
                "{}: n = {} < {min_n} -> skipped (underdetermined, no finding)",
                stem(p),
                vals.len()
            );
            continue;
        }
        if !filters.is_empty()
            && cond_name
                .as_ref()
                .map(|c| stem(p) != c.as_str())
                .unwrap_or(true)
            && !filters.iter().any(|f| stem(p).contains(f.as_str()))
        {
            continue;
        }
        series.push(Series {
            name: stem(p),
            vals,
        });
    }

    if series.len() < 2 {
        eprintln!("< 2 usable series in the folder -> no screening (no fabrication)");
        std::process::exit(0);
    }

    let cond_series: Option<Series> = cond_name.as_ref().map(|name| {
        series
            .iter()
            .find(|s| &s.name == name)
            .cloned()
            .unwrap_or_else(|| {
                eprintln!("--cond series '{name}' not found among loaded series");
                std::process::exit(2);
            })
    });

    println!(
        "=== cross-screening: {} series, {} pairs, lags = {:?}, surrogate = {}, min-n = {min_n}, jobs = {jobs} ===",
        series.len(),
        series.len() * (series.len() - 1) / 2,
        lags,
        n_surr
    );
    println!(
        "Series: {}",
        series
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    match &cond_series {
        Some(c) => println!(
            "Conditioning on shared driver: {} (confounder, excluded from arrows)",
            c.name
        ),
        None => {
            println!("No conditioning series set -> arrows may include the shared diurnal driver")
        }
    }
    println!();

    let seed = 0x9E37_79B9_7F4A_7C15u64;
    let findings: Mutex<Vec<(usize, usize, usize, f64, f64)>> = Mutex::new(Vec::new());
    let findings_cond: Mutex<Vec<(usize, usize, usize, f64, f64)>> = Mutex::new(Vec::new());
    let out: Mutex<Vec<String>> = Mutex::new(Vec::new());

    let mut tasks: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..series.len() {
        for j in (i + 1)..series.len() {
            if let Some(c) = &cond_series {
                if series[i].name == c.name || series[j].name == c.name {
                    continue;
                }
            }
            for li in 0..lags.len() {
                tasks.push((i, j, li));
            }
        }
    }

    let counter = AtomicUsize::new(0);

    std::thread::scope(|s| {
        for _ in 0..jobs.max(1) {
            s.spawn(|| {
                let mut local_out: Vec<String> = Vec::new();
                let mut local_findings: Vec<(usize, usize, usize, f64, f64)> = Vec::new();
                let mut local_findings_cond: Vec<(usize, usize, usize, f64, f64)> = Vec::new();
                loop {
                    let idx = counter.fetch_add(1, Ordering::Relaxed);
                    let Some(&(i, j, li)) = tasks.get(idx) else {
                        break;
                    };
                    let a = &series[i];
                    let b = &series[j];
                    let n = a.vals.len().min(b.vals.len());
                    let av = &a.vals[..n];
                    let bv = &b.vals[..n];
                    let lag = lags[li];
                    let seed_ab = seed ^ (i as u64).rotate_left(16) ^ (j as u64);
                    let seed_ba = seed ^ (j as u64).rotate_left(16) ^ (i as u64);
                    if li == 0 {
                        local_out.push(format!("=== pair {} <-> {} (n = {n}) ===", a.name, b.name));
                        local_out.push(format!(
                            "{:>4} | {:>12} | {:>12} | {:>12} | {:>12} | {:>8}",
                            "lag", "TE(a->b)", "thr", "TE(b->a)", "thr", "finding"
                        ));
                    }
                    let te_a_to_b = transfer_entropy_lag(bv, av, lag);
                    let thr_a_to_b = surrogate_threshold_lag(bv, av, lag, seed_ab ^ lag as u64);
                    let te_b_to_a = transfer_entropy_lag(av, bv, lag);
                    let thr_b_to_a = surrogate_threshold_lag(av, bv, lag, seed_ba ^ lag as u64);
                    let (Some(te_a_to_b), Some(thr_a_to_b), Some(te_b_to_a), Some(thr_b_to_a)) =
                        (te_a_to_b, thr_a_to_b, te_b_to_a, thr_b_to_a)
                    else {
                        local_out.push(format!("{:>4} | too few data", lag));
                        continue;
                    };
                    let sig_ab = te_a_to_b > thr_a_to_b;
                    let sig_ba = te_b_to_a > thr_b_to_a;
                    let finding = match (sig_ab, sig_ba) {
                        (true, true) => "both".to_string(),
                        (true, false) => format!("{} -> {}", a.name, b.name),
                        (false, true) => format!("{} -> {}", b.name, a.name),
                        _ => "no finding".to_string(),
                    };
                    local_out.push(format!(
                        "{:>4} | {:>12.5e} | {:>12.5e} | {:>12.5e} | {:>12.5e} | {}",
                        lag, te_a_to_b, thr_a_to_b, te_b_to_a, thr_b_to_a, finding
                    ));
                    if sig_ab {
                        local_findings.push((i, j, lag, te_a_to_b, thr_a_to_b));
                    }
                    if sig_ba {
                        local_findings.push((j, i, lag, te_b_to_a, thr_b_to_a));
                    }

                    if let Some(c) = &cond_series {
                        let cn = c.vals.len().min(n);
                        let cv = &c.vals[..cn];
                        let te_ab_c = transfer_entropy_conditional(bv, av, cv, lag);
                        let thr_ab_c = conditional_te_stats(
                            bv,
                            av,
                            cv,
                            lag,
                            seed_ab ^ lag as u64,
                            n_surr as usize,
                        )
                        .map(|(_, _, t)| t);
                        let te_ba_c = transfer_entropy_conditional(av, bv, cv, lag);
                        let thr_ba_c = conditional_te_stats(
                            av,
                            bv,
                            cv,
                            lag,
                            seed_ba ^ lag as u64,
                            n_surr as usize,
                        )
                        .map(|(_, _, t)| t);
                        if let (Some(te_ab_c), Some(thr_ab_c), Some(te_ba_c), Some(thr_ba_c)) =
                            (te_ab_c, thr_ab_c, te_ba_c, thr_ba_c)
                        {
                            if te_ab_c > thr_ab_c {
                                local_findings_cond.push((i, j, lag, te_ab_c, thr_ab_c));
                            }
                            if te_ba_c > thr_ba_c {
                                local_findings_cond.push((j, i, lag, te_ba_c, thr_ba_c));
                            }
                        }
                    }
                    if li == lags.len() - 1 {
                        local_out.push(String::new());
                    }
                }
                let mut g = findings.lock().unwrap();
                g.append(&mut local_findings);
                drop(g);
                let mut g = findings_cond.lock().unwrap();
                g.append(&mut local_findings_cond);
                drop(g);
                let mut g = out.lock().unwrap();
                g.append(&mut local_out);
            });
        }
    });

    let out = out.into_inner().unwrap();
    for line in out {
        println!("{}", line);
    }
    let mut findings = findings.into_inner().unwrap();
    let mut findings_cond = findings_cond.into_inner().unwrap();

    println!("=== Finding: significant arrows (TE > mean+2sigma phase-randomized) ===");
    if findings.is_empty() {
        println!("no significant arrows -> no finding (measured, not fabricated)");
    } else {
        findings.sort_by(|x, y| y.3.total_cmp(&x.3));
        for (src, dst, lag, te, thr) in findings {
            println!(
                "{} -> {}  lag {:<3}  TE {:.4} > threshold {:.4}",
                series[src].name, series[dst].name, lag, te, thr
            );
        }
    }

    if let Some(c) = &cond_series {
        println!();
        println!(
            "=== Finding conditioned on {} (TE(a->b | c) > surrogate thr) ===",
            c.name
        );
        if findings_cond.is_empty() {
            println!(
                "no arrow survives conditioning -> shared driver alone, no direct coupling (measured)"
            );
        } else {
            findings_cond.sort_by(|x, y| y.3.total_cmp(&x.3));
            for (src, dst, lag, te, thr) in findings_cond {
                println!(
                    "{} -> {}  lag {:<3}  cTE {:.4} > threshold {:.4}",
                    series[src].name, series[dst].name, lag, te, thr
                );
            }
        }
    }
}
