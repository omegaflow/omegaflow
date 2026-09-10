use std::env;
use std::process::exit;

use omegaflow::te::{surrogate_threshold_lag, transfer_entropy_lag};

fn read_series(path: &str) -> Option<Vec<f32>> {
    let text = std::fs::read_to_string(path).ok()?;
    let series: Vec<f32> = text
        .lines()
        .filter_map(|l| l.trim().parse::<f32>().ok())
        .collect();
    Some(series)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let path_a = args
        .iter()
        .position(|a| a == "--a")
        .and_then(|i| args.get(i + 1));
    let path_b = args
        .iter()
        .position(|a| a == "--b")
        .and_then(|i| args.get(i + 1));
    let name_a = match args
        .iter()
        .position(|a| a == "--name-a")
        .and_then(|i| args.get(i + 1))
    {
        Some(v) => v.clone(),
        None => "A".to_string(),
    };
    let name_b = match args
        .iter()
        .position(|a| a == "--name-b")
        .and_then(|i| args.get(i + 1))
    {
        Some(v) => v.clone(),
        None => "B".to_string(),
    };
    let lags: Vec<usize> = match args
        .iter()
        .position(|a| a == "--lags")
        .and_then(|i| args.get(i + 1))
    {
        Some(v) => v.split(',').filter_map(|s| s.parse().ok()).collect(),
        None => vec![1, 3, 6, 12, 24],
    };
    let n_surr: u64 = match args
        .iter()
        .position(|a| a == "--surrogat")
        .and_then(|i| args.get(i + 1))
    {
        Some(v) => match v.parse().ok() {
            Some(n) => n,
            None => 10,
        },
        None => 10,
    };

    let (Some(pa), Some(pb)) = (path_a, path_b) else {
        eprintln!("--a <csv> --b <csv> required");
        exit(2);
    };
    let (Some(a), Some(b)) = (read_series(pa), read_series(pb)) else {
        eprintln!("--a/--b files unreadable");
        exit(2);
    };
    let n = a.len().min(b.len());
    if n < 30 {
        println!(
            "n = {} < 30 -> no finding (underdetermination, no fabrication)",
            n
        );
        return;
    }
    let a = &a[..n];
    let b = &b[..n];
    println!(
        "pair: {} <-> {} | n = {} | lags = {:?} | surrogates = {}",
        name_a, name_b, n, lags, n_surr
    );
    println!();
    println!(
        "{:>4} | {:>12} | {:>12} | {:>12} | {:>8} | {:>8}",
        "lag", "TE(a->b)", "threshold", "TE(b->a)", "threshold", "verdict"
    );
    for &lag in &lags {
        let te_ab = transfer_entropy_lag(b, a, lag);
        let thr_ab = surrogate_threshold_lag(b, a, lag, 0x9E37_79B9_7F4A_7C15);
        let te_ba = transfer_entropy_lag(a, b, lag);
        let thr_ba = surrogate_threshold_lag(a, b, lag, 0x9E37_79B9_7F4A_7C15);
        let (Some(te_ab), Some(thr_ab), Some(te_ba), Some(thr_ba)) = (te_ab, thr_ab, te_ba, thr_ba)
        else {
            println!("{:>4} | too few data", lag);
            continue;
        };
        let sig_ab = te_ab > thr_ab;
        let sig_ba = te_ba > thr_ba;
        let verdict = match (sig_ab, sig_ba) {
            (true, true) => "both".to_string(),
            (true, false) => format!("{} -> {}", name_a, name_b),
            (false, true) => format!("{} -> {}", name_b, name_a),
            _ => "no finding".to_string(),
        };
        println!(
            "{:>4} | {:>12.5e} | {:>12.5e} | {:>12.5e} | {:>8.4e} | {}",
            lag, te_ab, thr_ab, te_ba, thr_ba, verdict
        );
    }
    println!();
    println!("TE > threshold (mean+2sigma phase-randomized) = significant arrow; else no finding.");
}
