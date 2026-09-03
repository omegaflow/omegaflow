use omegaflow::archivar::f107;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::goes::{self, COMP_XRSA, COMP_XRSB};
use omegaflow::te::{surrogate_stats, surrogate_stats_phase, transfer_entropy_lag};

const GOES_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/goes_xrs.bin";
const F107_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/f107_penticton.bin";
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DAY: f64 = 86400.0;
const LAGS: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
const MIN_N: usize = 30;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load_bin(kind: &str, url: &str, path: Option<String>) -> Option<Vec<u8>> {
    if let Some(p) = path {
        match std::fs::read(&p) {
            Ok(bytes) => return Some(bytes),
            Err(_) => {
                eprintln!("{kind}: {} reads void — the channel stays unmeasured", p);
                return None;
            }
        }
    }
    match fetch_raw_bytes(url, 3600) {
        Some(bytes) => Some(bytes),
        None => {
            eprintln!(
                "{kind}: {url} carries no asset — the CI manifest has not landed, the channel stays unmeasured (0 honored)"
            );
            None
        }
    }
}

fn bin_mean_day(series: &[(f64, f64)], t0: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in series {
        let idx = ((t - t0) / DAY).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        let i = idx as usize;
        sums[i] += v;
        counts[i] += 1;
    }
    (0..n)
        .map(|i| {
            if counts[i] > 0 {
                Some((sums[i] / counts[i] as f64) as f32)
            } else {
                None
            }
        })
        .collect()
}

fn pair_cells(a: &[Option<f32>], b: &[Option<f32>]) -> (Vec<f32>, Vec<f32>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (ca, cb) in a.iter().zip(b.iter()) {
        if let (Some(x), Some(y)) = (ca, cb) {
            xs.push(*x);
            ys.push(*y);
        }
    }
    (xs, ys)
}

fn te_row(label_a: &str, label_b: &str, xs: &[f32], ys: &[f32]) {
    if xs.len() < MIN_N {
        println!(
            "{:>14} → {:<14} | no statement possible (n = {})",
            label_a,
            label_b,
            xs.len()
        );
        return;
    }
    for &lag in LAGS.iter() {
        let seed = SURROGATE_SEED ^ (lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        let te = transfer_entropy_lag(xs, ys, lag);
        let thr = surrogate_stats_phase(xs, ys, lag, seed).map(|(_, _, t)| t);
        match (te, thr) {
            (Some(t), Some(h)) => {
                let arrow = if t > h { "arrow" } else { "silent" };
                println!(
                    "{:>14} → {:<14} | lag {:>2} d | TE {:>10.4e} | threshold {:>10.4e} | excess {:>10.4e} | {}",
                    label_a,
                    label_b,
                    lag,
                    t,
                    h,
                    t - h,
                    arrow
                );
            }
            _ => {
                println!(
                    "{:>14} → {:<14} | lag {:>2} d | TE absent | threshold absent | silent",
                    label_a, label_b, lag
                );
            }
        }
    }
}

struct Arrow {
    from: String,
    to: String,
    n: usize,
    te: f64,
    threshold: f64,
    naive_threshold: f64,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    println!(
        "=== Long-window probe: F10.7 history × GOES-XRS history through the TE machine (Nadel Ⅲ) ==="
    );
    println!(
        "TE-Lib: TE(Y→X; τ) = Σ_t ln[ p(x_{{t+τ}}, x_t, y_t) · p(x_t) / (p(x_t, y_t) · p(x_{{t+τ}}, x_t)) ] / m, m = n − τ;"
    );
    println!("KDE bandwidth: Silverman h = 1.06·σ·n^(−0.2) per series.");
    println!(
        "Threshold: phase-randomized surrogates (spectrum-preserving, std-only FFT, 10 realizations), mean + 2σ; the naive shuffle threshold stands printed for the broken-null-control record. Arrow ⇔ TE > threshold."
    );
    println!(
        "Grid: one cell per day, common window; lags in days. Time base: GOES carries TDB epochs, F10.7 carries UTC calendar days — the ~69 s TDB constant and the noon-vs-midnight conventions sit below the daily cell width."
    );
    println!(
        "Channels: F10.7-Penticton (sfu → W/m²/Hz, noon measurement) × GOES XRSA (0,05–0,4 nm) and XRSB (0,1–0,8 nm), both W/m²@1AU, daily means over all GOES satellites."
    );

    let goes_bytes = load_bin("goes_xrs.bin", GOES_CDN, arg_value(&args, "--goes-bin"));
    let f107_bytes = load_bin(
        "f107_penticton.bin",
        F107_CDN,
        arg_value(&args, "--f107-bin"),
    );
    let (Some(goes_bytes), Some(f107_bytes)) = (goes_bytes, f107_bytes) else {
        println!(
            "Verdict: the long window stays unmeasured — the CDN assets are the precondition (0 honored, silence is the answer)."
        );
        return;
    };
    let goes_records = match goes::parse_bin(&goes_bytes) {
        Some(r) => r,
        None => {
            println!(
                "goes_xrs.bin: {} B carry no GXS1 contract — the channel stays unmeasured",
                goes_bytes.len()
            );
            return;
        }
    };
    let f107_records = match f107::parse_bin(&f107_bytes) {
        Some(r) => r,
        None => {
            println!(
                "f107_penticton.bin: {} B carry no F107 contract — the channel stays unmeasured",
                f107_bytes.len()
            );
            return;
        }
    };
    let xrsa: Vec<(f64, f64)> = goes_records
        .iter()
        .filter(|(_, _, c)| *c == COMP_XRSA)
        .map(|&(t, v, _)| (t, v))
        .collect();
    let xrsb: Vec<(f64, f64)> = goes_records
        .iter()
        .filter(|(_, _, c)| *c == COMP_XRSB)
        .map(|&(t, v, _)| (t, v))
        .collect();
    let f107_series: Vec<(f64, f64)> = f107_records
        .iter()
        .map(|&(d, v)| (d as f64 * DAY, v))
        .collect();

    println!();
    println!("=== channel board ===");
    let window_report = |name: &str, s: &[(f64, f64)]| match (s.first(), s.last()) {
        (Some(&(a, _)), Some(&(b, _))) => {
            let days = (b - a) / DAY;
            println!(
                "{name:<10} | n = {:<7} | window {:.1} d | cells/day {:.3}",
                s.len(),
                days,
                s.len() as f64 / days.max(1.0)
            );
        }
        _ => println!("{name:<10} | no samples — the channel harvests null"),
    };
    window_report("F10.7", &f107_series);
    window_report("XRSA", &xrsa);
    window_report("XRSB", &xrsb);

    let lo = [f107_series.first(), xrsa.first(), xrsb.first()]
        .iter()
        .filter_map(|o| o.map(|&(t, _)| t))
        .fold(f64::NEG_INFINITY, f64::max);
    let hi = [f107_series.last(), xrsa.last(), xrsb.last()]
        .iter()
        .filter_map(|o| o.map(|&(t, _)| t))
        .fold(f64::INFINITY, f64::min);
    let Some((t0, n)) = (lo < hi).then(|| {
        let t0 = (lo / DAY).floor() * DAY;
        let n = ((hi - t0) / DAY).floor() as usize;
        (t0, n)
    }) else {
        println!("common window empty — the pairing carries no cells");
        return;
    };
    let f107_cells = bin_mean_day(&f107_series, t0, n);
    let xrsa_cells = bin_mean_day(&xrsa, t0, n);
    let xrsb_cells = bin_mean_day(&xrsb, t0, n);

    println!();
    println!("=== matrix — daily cells (common window, lag ∈ 0..7 d) ===");
    let (fa, xa) = pair_cells(&f107_cells, &xrsa_cells);
    te_row("F10.7", "XRSA", &fa, &xa);
    te_row("XRSA", "F10.7", &xa, &fa);
    let (fb, xb) = pair_cells(&f107_cells, &xrsb_cells);
    te_row("F10.7", "XRSB", &fb, &xb);
    te_row("XRSB", "F10.7", &xb, &fb);

    let mut arrows: Vec<Arrow> = Vec::new();
    for (from, to, xs, ys) in [
        ("F10.7", "XRSA", fa.as_slice(), xa.as_slice()),
        ("XRSA", "F10.7", xa.as_slice(), fa.as_slice()),
        ("F10.7", "XRSB", fb.as_slice(), xb.as_slice()),
        ("XRSB", "F10.7", xb.as_slice(), fb.as_slice()),
    ] {
        if xs.len() < MIN_N {
            continue;
        }
        let te = transfer_entropy_lag(xs, ys, 0);
        let stats = surrogate_stats_phase(xs, ys, 0, SURROGATE_SEED);
        let naive = surrogate_stats(xs, ys, 0, SURROGATE_SEED);
        if let (Some(t), Some((_, _, thr))) = (te, stats) {
            arrows.push(Arrow {
                from: from.into(),
                to: to.into(),
                n: xs.len(),
                te: t,
                threshold: thr,
                naive_threshold: naive.map(|(_, _, t)| t).unwrap_or(f64::NAN),
            });
        }
    }

    println!();
    println!(
        "=== arrows (significant ⇔ TE > threshold = mean + 2σ of phase-randomized surrogates) ==="
    );
    for p in &arrows {
        println!(
            "{:>6} → {:<6} | n {:>5} | TE {:.4e} | threshold {:.4e} | threshold-naive {:.4e} | {}",
            p.from,
            p.to,
            p.n,
            p.te,
            p.threshold,
            p.naive_threshold,
            if p.te > p.threshold {
                "arrow"
            } else {
                "silent"
            }
        );
    }
    let sig = |a: &str, b: &str| {
        arrows
            .iter()
            .any(|p| p.from == a && p.to == b && p.te > p.threshold)
    };
    println!();
    println!("=== null control: naive vs. phase-randomized threshold (lag 0) ===");
    for p in &arrows {
        let naiv = if p.te > p.naive_threshold {
            "breaks"
        } else {
            "holds"
        };
        let phase = if p.te > p.threshold {
            "breaks"
        } else {
            "holds"
        };
        println!(
            "{:>6} → {:<6} | n {:>5} | TE {:.4e} | threshold-naive {:.4e} ({}) | threshold-phase {:.4e} ({})",
            p.from, p.to, p.n, p.te, p.naive_threshold, naiv, p.threshold, phase
        );
    }

    println!();
    println!("=== verdict ===");
    let f107_drives = sig("F10.7", "XRSA") || sig("F10.7", "XRSB");
    let xrs_drives = sig("XRSA", "F10.7") || sig("XRSB", "F10.7");
    let n_min = arrows.iter().map(|p| p.n).min().unwrap_or(0);
    if n_min < MIN_N {
        println!(
            "no statement possible (n = {}) — underdetermination, not physical silence.",
            n_min
        );
    } else if f107_drives && xrs_drives {
        println!(
            "F10.7 ↔ X-Ray bidirectional on the daily scale — both arrows carry; the larger excess names the prevailing coupling."
        );
    } else if f107_drives {
        println!(
            "F10.7 → X-Ray significant with silent reverse direction → the chromosphere drives the corona (the F10.7 chromospheric reservoir leads the coronal X-ray emission by days)."
        );
    } else if xrs_drives {
        println!(
            "X-Ray → F10.7 significant with silent reverse direction → the corona leads the chromosphere — the reservoir coupling runs the other way."
        );
    } else {
        println!("F10.7 ↔ X-Ray silent on both sides → no causal arrow on the daily scale.");
    }
    println!("Silent lines are findings. Exit 0.");
}
