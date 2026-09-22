#![cfg(test)]

use crate::mathematikerin::machines::TE_KSG_K_PROD;
use crate::mathematikerin::te::{
    TE_KSG_K, embed_series, find_mi_lag, phase_randomized_surrogate, topological_te_phase,
    transfer_entropy_embedded_ksg,
};

const K_VARIANCE_FLOOR: usize = 4;

const K_SWEEP_MAX: usize = 12;

const K_FPR_TOLERANCE_PCT: f64 = 8.0;

const K_FNR_TOLERANCE_PCT: f64 = 50.0;

fn k_gate_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn k_gate_ar1(n: usize, phi: f64, rng: &mut u64) -> Vec<f32> {
    let mut v = Vec::with_capacity(n);
    let mut x = 0.0f64;
    for _ in 0..n {
        x = phi * x + k_gate_rng(rng) * 2.0 - 1.0;
        v.push(x as f32);
    }
    v
}

fn k_gate_coupled_ar1(n: usize, rng: &mut u64) -> (Vec<f32>, Vec<f32>) {
    let a = k_gate_ar1(n, 0.5, rng);
    let b: Vec<f32> = (0..a.len())
        .map(|i| {
            if i == 0 {
                k_gate_rng(rng) as f32
            } else {
                (0.9 * a[i - 1] as f64 + (k_gate_rng(rng) * 0.2 - 0.1)) as f32
            }
        })
        .collect();
    (a, b)
}

fn formula_k(n: usize, d: usize, tau_x: usize, tau_y: usize) -> Option<(usize, usize, usize)> {
    if n < 8 || d < 2 || tau_x == 0 || tau_y == 0 {
        return None;
    }
    let back = (d - 1) * tau_x.max(tau_y);
    let t_high = n.checked_sub(tau_x + 1)?;
    if back > t_high {
        return None;
    }
    let m = t_high - back + 1;
    if m <= K_VARIANCE_FLOOR {
        return None;
    }
    let jd = 1 + 2 * d;
    let k_fit = (m as f64).powf(4.0 / (4 + jd) as f64).round() as usize;
    Some((m, jd, k_fit.clamp(K_VARIANCE_FLOOR, m - 1)))
}

fn ksg_te_phase_null(
    driver: &[f32],
    target: &[f32],
    dim: usize,
    k: usize,
    seed: u64,
) -> Option<(f64, f64)> {
    let n = driver.len();
    if n < 8 || target.len() != n || dim < 2 {
        return None;
    }
    let xf: Vec<f64> = driver.iter().map(|&v| v as f64).collect();
    let yf: Vec<f64> = target.iter().map(|&v| v as f64).collect();
    if xf.iter().chain(yf.iter()).any(|v| !v.is_finite()) {
        return None;
    }
    let tau_x = find_mi_lag(&xf)?;
    let tau_y = find_mi_lag(&yf)?;
    let emb_x = embed_series(&xf, tau_x, dim);
    let emb_y = embed_series(&yf, tau_y, dim);
    let te = transfer_entropy_embedded_ksg(&xf, &emb_x, &emb_y, tau_x, tau_y, k)?;
    let mut vals: Vec<f64> = Vec::with_capacity(10);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..10 {
        let ys = phase_randomized_surrogate(target, &mut rng);
        if ys.len() != n {
            continue;
        }
        let ysf: Vec<f64> = ys.iter().map(|&v| v as f64).collect();
        if ysf.iter().any(|v| !v.is_finite()) {
            continue;
        }
        let tau_s = match find_mi_lag(&ysf) {
            Some(v) => v,
            None => continue,
        };
        let emb_s = embed_series(&ysf, tau_s, dim);
        if emb_s.is_empty() {
            continue;
        }
        if let Some(te_s) = transfer_entropy_embedded_ksg(&xf, &emb_x, &emb_s, tau_x, tau_s, k) {
            vals.push(te_s);
        }
    }
    if vals.len() < 2 {
        return None;
    }
    let mean = vals.iter().sum::<f64>() / vals.len() as f64;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / vals.len() as f64;
    Some((te, mean + 2.0 * var.sqrt()))
}

struct KSweepCell {
    k: usize,
    fp_meas: usize,
    fp: usize,
    fn_meas: usize,
    found: usize,
}

impl KSweepCell {
    fn fpr(&self) -> Option<f64> {
        (self.fp_meas > 0).then(|| 100.0 * self.fp as f64 / self.fp_meas as f64)
    }

    fn fnr(&self) -> Option<f64> {
        (self.fn_meas > 0).then(|| 100.0 * (self.fn_meas - self.found) as f64 / self.fn_meas as f64)
    }

    fn holds(&self) -> bool {
        matches!(
            (self.fpr(), self.fnr()),
            (Some(fpr), Some(fnr)) if fpr <= K_FPR_TOLERANCE_PCT && fnr < K_FNR_TOLERANCE_PCT
        )
    }
}

fn run_ksg_k_sweep(n: usize, dim: usize, fp_trials: usize, fn_trials: usize) -> Vec<KSweepCell> {
    let mut cells = Vec::with_capacity(K_SWEEP_MAX);
    for k in 1..=K_SWEEP_MAX {
        let mut fp = 0usize;
        let mut fp_meas = 0usize;
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        for t in 0..fp_trials {
            let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let a = k_gate_ar1(n, 0.7, &mut rng);
            let b = k_gate_ar1(n, 0.7, &mut rng);
            if let Some((te, thr)) = ksg_te_phase_null(&a, &b, dim, k, seed) {
                fp_meas += 1;
                if te > thr {
                    fp += 1;
                }
            }
        }
        let mut found = 0usize;
        let mut fn_meas = 0usize;
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        for t in 0..fn_trials {
            let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let (a, b) = k_gate_coupled_ar1(n, &mut rng);
            if let Some((te, thr)) = ksg_te_phase_null(&a, &b, dim, k, seed) {
                fn_meas += 1;
                if te > thr {
                    found += 1;
                }
            }
        }
        cells.push(KSweepCell {
            k,
            fp_meas,
            fp,
            fn_meas,
            found,
        });
    }
    cells
}

fn sweep_k(cells: &[KSweepCell]) -> Option<usize> {
    cells.iter().filter(|c| c.holds()).map(|c| c.k).min()
}

#[test]
fn gate_ksg_k_formula_geometry_pins() {
    assert_eq!(formula_k(300, 3, 3, 3), Some((291, 7, 8)));

    assert_eq!(formula_k(1000, 3, 3, 3), Some((991, 7, 12)));

    assert_eq!(formula_k(43, 3, 3, 3), Some((34, 7, 4)));

    assert_eq!(formula_k(12, 3, 3, 3), None);

    let mut prev = usize::MAX;
    for d in 2..=6 {
        let (_, _, k) = formula_k(300, d, 3, 3).expect("d=2..=6 measurable");
        assert!(
            k <= prev,
            "the k-nn rule must not grow with the joint dimension (d={d} k={k})"
        );
        prev = k;
    }
}

#[test]
fn gate_ksg_k_formula_monotone_in_n() {
    let mut prev = 0usize;
    for n in (30..=1000).step_by(10) {
        let (_, _, k) = formula_k(n, 3, 3, 3).expect("n=30..=1000 measurable");
        assert!(
            k >= prev,
            "the k-nn rule must not shrink with more pairs (n={n})"
        );
        prev = k;
    }
    assert_eq!(prev, 12, "the rule reaches 12 at n=1000");
}

#[test]
fn gate_ksg_k_selector_picks_smallest_valid() {
    let cells = vec![
        KSweepCell {
            k: 1,
            fp_meas: 30,
            fp: 3,
            fn_meas: 20,
            found: 20,
        },
        KSweepCell {
            k: 2,
            fp_meas: 30,
            fp: 2,
            fn_meas: 20,
            found: 18,
        },
        KSweepCell {
            k: 3,
            fp_meas: 30,
            fp: 1,
            fn_meas: 20,
            found: 19,
        },
        KSweepCell {
            k: 4,
            fp_meas: 30,
            fp: 4,
            fn_meas: 20,
            found: 5,
        },
    ];
    assert_eq!(sweep_k(&cells), Some(2), "the smallest holding k wins");

    let void = vec![KSweepCell {
        k: 1,
        fp_meas: 0,
        fp: 0,
        fn_meas: 20,
        found: 20,
    }];
    assert_eq!(sweep_k(&void), None, "unmeasured is never a passing zero");
}

#[test]
fn gate_ksg_k_sweep_harness_byte_equals_kalibrier_at_reference_k() {
    let mut rng = 0x2722_0A95_517C_C1B7u64;
    let seed = 0x9E37_79B9_7F4A_7C15;
    for _ in 0..8 {
        let a = k_gate_ar1(300, 0.7, &mut rng);
        let b = k_gate_ar1(300, 0.7, &mut rng);
        let Some(v) = topological_te_phase(&a, &b, 3, 3, seed) else {
            continue;
        };
        let (te, thr) = ksg_te_phase_null(&a, &b, 3, TE_KSG_K, seed)
            .expect("the sweep harness must measure the pair the Kalibrier gate measured");
        assert_eq!(
            te, v.te,
            "the sweep harness te must be byte-equal to the Kalibrier verdict at the reference k"
        );
        assert_eq!(
            thr, v.threshold,
            "the sweep harness threshold must be byte-equal to the Kalibrier verdict"
        );
        return;
    }
    panic!("Kalibrier parity: no measurable pair in 8 draws — the harness stays unpinned");
}

#[test]
#[ignore = "the KSG-k gate — formula vs sweep at n=300, heavy, runs in te-gate.yml"]
fn ksg_k_gate_sweep_and_formula() {
    let (n, dim) = (300usize, 3usize);
    let cells = run_ksg_k_sweep(n, dim, 60, 40);

    let mut rng = 0x9E37_79B9_7F4A_7C15u64;
    let a = k_gate_ar1(n, 0.7, &mut rng);
    let b = k_gate_ar1(n, 0.7, &mut rng);
    let xf: Vec<f64> = a.iter().map(|&v| v as f64).collect();
    let yf: Vec<f64> = b.iter().map(|&v| v as f64).collect();
    let tau_xy = match (find_mi_lag(&xf), find_mi_lag(&yf)) {
        (Some(tx), Some(ty)) => Some((tx, ty)),
        _ => None,
    };
    let k_formula = tau_xy.and_then(|(tx, ty)| formula_k(n, dim, tx, ty));
    for c in &cells {
        println!(
            "ksg-k sweep k={:2}: fpr={} fp_meas={} fp={} | fnr={} fn_meas={} found={}",
            c.k,
            match c.fpr() {
                Some(v) => format!("{v:.1}%"),
                None => "unmeasured".to_string(),
            },
            c.fp_meas,
            c.fp,
            match c.fnr() {
                Some(v) => format!("{v:.1}%"),
                None => "unmeasured".to_string(),
            },
            c.fn_meas,
            c.found,
        );
    }
    match (tau_xy, k_formula) {
        (Some((tx, ty)), Some((m, jd, kf))) => {
            println!("ksg-k formula: n={n} d={dim} tau=({tx},{ty}) m={m} jd={jd} -> k={kf}");
        }
        _ => println!("ksg-k formula: void at the first FP pair's geometry"),
    }
    let k_sweep = sweep_k(&cells);
    println!(
        "ksg-k gate: formula={} sweep={} reference={TE_KSG_K} production={TE_KSG_K_PROD}",
        match k_formula {
            Some((_, _, k)) => k.to_string(),
            None => "void".to_string(),
        },
        match k_sweep {
            Some(k) => k.to_string(),
            None => "void".to_string(),
        },
    );
    for c in &cells {
        assert!(
            c.fp_meas > 0 && c.fn_meas > 0,
            "k={}: both cells measured (unmeasured is never a passing zero)",
            c.k
        );
        assert!(
            c.fp_meas >= 40,
            "k={}: the FP cell stays measurable in {} of 60 trials — below the Kalibrier 2/3 floor",
            c.k,
            c.fp_meas
        );
        assert!(
            c.fn_meas >= 20,
            "k={}: the FN cell stays measurable in {} of 40 trials",
            c.k,
            c.fn_meas
        );
    }
    match (k_formula, k_sweep) {
        (Some((_, _, f)), Some(s)) if f == s => {
            println!("ksg-k verdict: agree on k={f} — the flip candidate is {f}");
        }
        (Some((_, _, f)), Some(s)) => {
            println!("ksg-k verdict: riss — formula {f} vs sweep {s}; the CI log decides");
        }
        (Some((_, _, f)), None) => {
            println!(
                "ksg-k verdict: sweep void — no k holds both tolerances; formula says {f}; K stays 0"
            );
        }
        (None, Some(s)) => {
            println!("ksg-k verdict: formula void at this geometry; sweep says {s}");
        }
        (None, None) => println!("ksg-k verdict: both paths void — K stays 0"),
    }
}
