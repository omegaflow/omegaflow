// TE-Referenz-Validierung — Schreiber 2000 („Measuring Information
// Transfer", PRL 85, 461): zwei unidirektional gekoppelte Hénon-Maps mit
// bekannter Richtung. X treibt Y bei c > 0; bei c = 0 sind beide
// unabhängig. Die Implementierung (der öffentliche skalare Pfad
// `transfer_entropy_lag` + die phasenrandomisierte Schwelle
// `surrogate_stats_phase`) muss die bekannte Richtung rekonstruieren:
// TE(X→Y) schlägt seine Schwelle, TE(Y→X) bleibt still, und die
// c=0-Kontrolle bleibt in beide Richtungen still. `src/te.rs` bleibt
// unberührt — dies ist eine Referenz-Probe über die öffentliche API.
//
// System:
//   x_{n+1} = 1.4 − x_n² + 0.3 x_{n−1}
//   y_{n+1} = 1.4 − (c·x_n·y_n + (1−c)·y_n²) + 0.3 y_{n−1}
// lag = 1 (die Kopplung x_n → y_{n+1} wirkt eine Zeitschritt später).

use omegaflow::te::{surrogate_stats_phase, transfer_entropy_lag};

const N: usize = 10000;
const TRANSIENT: usize = 1000;
const COUPLING: f64 = 0.2;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

fn coupled_henon(n: usize, transient: usize, c: f64) -> (Vec<f32>, Vec<f32>) {
    let total = n + transient;
    let mut xs = vec![0.0f64; total];
    let mut ys = vec![0.0f64; total];
    xs[0] = 0.1;
    xs[1] = 0.0;
    ys[0] = 0.2;
    ys[1] = 0.1;
    for t in 1..total - 1 {
        xs[t + 1] = 1.4 - xs[t] * xs[t] + 0.3 * xs[t - 1];
        ys[t + 1] = 1.4 - (c * xs[t] * ys[t] + (1.0 - c) * ys[t] * ys[t]) + 0.3 * ys[t - 1];
    }
    let to_f32 = |v: &[f64]| {
        v[transient..]
            .iter()
            .map(|&x| x as f32)
            .collect::<Vec<f32>>()
    };
    (to_f32(&xs), to_f32(&ys))
}

fn direction(target: &[f32], source: &[f32], label: &str) -> (f64, f64, bool) {
    match (
        transfer_entropy_lag(target, source, 1),
        surrogate_stats_phase(target, source, 1, SEED),
    ) {
        (Some(te), Some((_, _, thr))) => {
            let arrow = te > thr;
            println!(
                "  {:<12} | TE {:>10.4e} | thr {:>10.4e} | {}",
                label,
                te,
                thr,
                if arrow { "arrow" } else { "still" }
            );
            (te, thr, arrow)
        }
        _ => {
            println!("  {:<12} | TE missing | thr missing | still", label);
            (f64::NAN, f64::NAN, false)
        }
    }
}

fn main() {
    println!(
        "=== TE-Referenz-Validierung — Schreiber 2000 (unidirektional gekoppelte Hénon-Maps) ==="
    );
    println!("System: x_{{n+1}} = 1.4 − x_n² + 0.3 x_{{n−1}};  y_{{n+1}} = 1.4 − (c·x_n·y_n + (1−c)·y_n²) + 0.3 y_{{n−1}}");
    println!("n = {} (Transient {} verworfen), lag = 1, Schwelle = phasenrandomisierte Surrogate (10, mean + 2σ)", N, TRANSIENT);

    let (xc, yc) = coupled_henon(N, TRANSIENT, COUPLING);
    println!();
    println!("Kopplung c = {:.2} (bekannte Richtung: X → Y):", COUPLING);
    let (te_xy, thr_xy, arrow_xy) = direction(&yc, &xc, "TE(X→Y)");
    let (te_yx, thr_yx, arrow_yx) = direction(&xc, &yc, "TE(Y→X)");

    let (xi, yi) = coupled_henon(N, TRANSIENT, 0.0);
    println!();
    println!("Kontrolle c = 0.00 (unabhängig):");
    let (_, _, arrow_xy0) = direction(&yi, &xi, "TE(X→Y)");
    let (_, _, arrow_yx0) = direction(&xi, &yi, "TE(Y→X)");

    println!();
    let pass = arrow_xy && !arrow_yx && !arrow_xy0 && !arrow_yx0;
    println!(
        "Verdikt: {}",
        if pass {
            "PASS — die Implementierung rekonstruiert die bekannte Richtung (TE(X→Y) schlägt, TE(Y→X) still, Kontrolle still)."
        } else {
            "FAIL — die Richtung wird nicht sauber rekonstruiert; die Werte oben benennen, welcher Ast abweicht."
        }
    );
    if pass {
        println!(
            "TE(X→Y) = {:.4e} über Schwelle {:.4e}; TE(Y→X) = {:.4e} unter {:.4e}.",
            te_xy, thr_xy, te_yx, thr_yx
        );
    }
}
