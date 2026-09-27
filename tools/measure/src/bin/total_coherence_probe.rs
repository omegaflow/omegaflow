const PERM_GROUND: f32 = f32::EPSILON;

const FORCE_NAME: [&str; 9] = [
    "em",
    "gravity",
    "acoustic",
    "seismic-body",
    "seismic-surface",
    "thermal",
    "diffusion",
    "advective",
    "electric",
];

fn perm_target(g: f32, v_c: f32) -> Option<f32> {
    let denom = g + PERM_GROUND;
    if !denom.is_finite() || denom <= 0.0 {
        return None;
    }
    let ratio = v_c / denom;
    if !ratio.is_finite() {
        return None;
    }
    let t = ratio.tanh();
    if !t.is_finite() {
        return None;
    }
    Some(t)
}

fn total_coherence_integral(now: &[f32], prev: &[f32]) -> Option<f32> {
    if now.len() != prev.len() || now.is_empty() {
        return None;
    }
    let mut acc = 0.0f32;
    for i in 0..now.len() {
        if !now[i].is_finite() || !prev[i].is_finite() {
            return None;
        }
        let d = now[i] - prev[i];
        if !d.is_finite() {
            return None;
        }
        let Some(c) = perm_target(now[i].abs(), d.abs()) else {
            return None;
        };
        acc += c;
    }
    if !acc.is_finite() {
        return None;
    }
    Some(acc)
}

fn global_permeability_target(now: &[f32], prev: &[f32]) -> Option<f32> {
    if now.len() != prev.len() || now.is_empty() {
        return None;
    }
    let mut sum_now = 0.0f32;
    let mut sum_delta = 0.0f32;
    for i in 0..now.len() {
        if !now[i].is_finite() || !prev[i].is_finite() {
            return None;
        }
        sum_now += now[i];
        sum_delta += now[i] - prev[i];
    }
    if !sum_now.is_finite() || !sum_delta.is_finite() {
        return None;
    }
    perm_target(sum_now.abs(), sum_delta.abs())
}

fn main() {
    let now: [f32; 9] = [0.00, 0.00, 0.50, 0.00, 0.00, 0.50, 0.00, 0.00, 0.00];
    let prev: [f32; 9] = [0.00, 0.00, 0.25, 0.00, 0.00, 0.75, 0.00, 0.00, 0.00];
    let empty: [f32; 9] = [0.0; 9];

    println!("=== total-coherence-integration (per-oscillator permeability integral) ===");
    println!(
        "formula: total coherence = Σ_i perm_target(|ω_i|, |Δω_i|) = Σ_i tanh(|Δω_i| / (|ω_i| + ε))"
    );
    println!(
        "         ε = f32::EPSILON (PERM_GROUND, omega.rs:13); perm_target mirrors omega.rs:15"
    );
    println!(
        "built  : breath branch drives one scalar tanh(|Σ Δω_i| / (|Σ ω_i| + ε)) (omega.rs:1674-1676)"
    );
    println!(
        "scope  : breath-branch term only; the TE term (live) and a complexity term are not built per oscillator and are not fabricated"
    );
    println!();

    for (i, name) in FORCE_NAME.iter().enumerate() {
        let d = now[i] - prev[i];
        let c = perm_target(now[i].abs(), d.abs());
        let ctxt = match c {
            Some(v) => format!("{v:.6}"),
            None => "absent".to_string(),
        };
        println!(
            "osc[{i}] {name:<14} ω {:.2}  Δω {:.2}  contribution {ctxt}",
            now[i], d
        );
    }
    println!();

    let integral = total_coherence_integral(&now, &prev);
    let null = total_coherence_integral(&empty, &empty);
    let global = global_permeability_target(&now, &prev);

    match (integral, null, global) {
        (Some(v), Some(n), Some(g)) => {
            println!("total-coherence-integral = {v:.6}");
            println!("null (empty field)       = {n:.6}");
            println!("global permeability      = {g:.6}  (built scalar on the same fixture)");
        }
        _ => {
            eprintln!("fixture carries a non-finite value — the record is skipped (0 honored)");
            std::process::exit(2);
        }
    }
}
