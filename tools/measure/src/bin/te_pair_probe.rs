use std::env;
use std::process::exit;

use omegaflow::te::{surrogate_threshold_lag, transfer_entropy_lag};

fn read_series(path: &str) -> Vec<f32> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.trim().parse::<f32>().ok())
        .collect()
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
    let name_a = args
        .iter()
        .position(|a| a == "--name-a")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "A".to_string());
    let name_b = args
        .iter()
        .position(|a| a == "--name-b")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "B".to_string());
    let lags: Vec<usize> = args
        .iter()
        .position(|a| a == "--lags")
        .and_then(|i| args.get(i + 1))
        .map(|v| v.split(',').filter_map(|s| s.parse().ok()).collect())
        .unwrap_or_else(|| vec![1, 3, 6, 12, 24]);
    let n_surr: u64 = args
        .iter()
        .position(|a| a == "--surrogat")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);

    let (Some(pa), Some(pb)) = (path_a, path_b) else {
        eprintln!("--a <csv> --b <csv> fehlen");
        exit(2);
    };
    let a = read_series(pa);
    let b = read_series(pb);
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
        "paar: {} <-> {} | n = {} | lags = {:?} | surrogat = {}",
        name_a, name_b, n, lags, n_surr
    );
    println!();
    println!(
        "{:>4} | {:>12} | {:>12} | {:>12} | {:>8} | {:>8}",
        "lag", "TE(a->b)", "schwelle", "TE(b->a)", "schwelle", "befund"
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
        let befund = match (sig_ab, sig_ba) {
            (true, true) => "beide".to_string(),
            (true, false) => format!("{} -> {}", name_a, name_b),
            (false, true) => format!("{} -> {}", name_b, name_a),
            _ => "no finding".to_string(),
        };
        println!(
            "{:>4} | {:>12.5e} | {:>12.5e} | {:>12.5e} | {:>8.4e} | {}",
            lag, te_ab, thr_ab, te_ba, thr_ba, befund
        );
    }
    println!();
    println!(
        "TE > Schwelle (mean+2sigma phasenrandomisiert) = signifikanter Pfeil; sonst no finding."
    );
}
